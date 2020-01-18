use super::CanSocket;
use crate::CanFrame;
use spin::RwLock;
use std::env;
use std::ffi::CString;
use std::io::Result;
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

static LOCK: RwLock<()> = RwLock::new(());

fn ifname() -> CString {
    let ifname = env::var_os("IFNAME").expect("IFNAME environment variable is not set");
    CString::new(ifname.as_bytes()).unwrap()
}

macro_rules! lock {
    (shared) => {
        let _lock = LOCK.read();
    };
    (exclusive) => {
        let _lock = LOCK.write();
    };
}

fn recv(socket: CanSocket, frame: Option<CanFrame>) -> Option<Result<CanFrame>> {
    struct Context(Arc<AtomicBool>);
    impl Drop for Context {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    let is_done = Arc::new(AtomicBool::new(false));
    let handle = {
        let is_done = is_done.clone();
        thread::spawn(move || {
            let _cxt = Context(is_done);
            loop {
                let f = socket.recv()?;
                match &frame {
                    Some(frame) => {
                        if &f == frame {
                            break Ok(f);
                        }
                    }
                    None => break Ok(f),
                }
            }
        })
    };
    thread::sleep(Duration::from_millis(100));
    if is_done.load(Ordering::Relaxed) {
        handle.join().ok()
    } else {
        None
    }
}

#[test]
#[ignore]
fn test_bind() {
    CanSocket::bind(ifname()).unwrap();
}

#[test]
#[should_panic(expected = "No such device")]
fn test_bind_no_device() {
    let ifname = CString::new("NO DEVICE").unwrap();
    CanSocket::bind(ifname).unwrap();
}

#[test]
#[ignore]
#[should_panic(expected = "WouldBlock")]
fn test_nonblocking() {
    lock!(exclusive);
    let socket = CanSocket::bind(ifname()).unwrap();
    socket.set_nonblocking(true).unwrap();

    recv(socket, None).unwrap().unwrap();
}

#[test]
#[ignore]
#[should_panic(expected = "None")]
fn test_no_nonblocking() {
    lock!(exclusive);
    let socket = CanSocket::bind(ifname()).unwrap();

    let _ = recv(socket, None).unwrap();
}

#[test]
#[ignore]
fn test_recv_own_msgs() {
    lock!(shared);
    let socket = CanSocket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = CanFrame::Standard(rand::random());
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[test]
#[ignore]
#[should_panic(expected = "None")]
fn test_no_recv_own_msgs() {
    lock!(shared);
    let socket = CanSocket::bind(ifname()).unwrap();

    let frame = CanFrame::Standard(rand::random());
    socket.send(&frame).unwrap();
    let _ = recv(socket, Some(frame)).unwrap();
}

#[test]
#[ignore]
fn test_fd_frames() {
    lock!(shared);
    let socket = CanSocket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();
    socket.set_fd_frames(true).unwrap();

    let frame = CanFrame::FdStandard(rand::random());
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[test]
#[ignore]
#[should_panic(expected = "InvalidInput")]
fn test_no_fd_frames() {
    lock!(shared);
    let socket = CanSocket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = CanFrame::FdStandard(rand::random());
    socket.send(&frame).unwrap();
}
