use super::Socket;
use crate::Frame;
use spin::RwLock;
use std::env;
use std::ffi::CString;
use std::io::Result;
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
static LOCK: RwLock<()> = RwLock::new(());

#[allow(dead_code)]
fn ifname() -> CString {
    let ifname = env::var_os("IFNAME").expect("IFNAME environment variable is not set");
    CString::new(ifname.as_bytes()).unwrap()
}

#[cfg(feature = "test_all")]
macro_rules! lock {
    (shared) => {
        let _lock = LOCK.read();
    };
    (exclusive) => {
        let _lock = LOCK.write();
    };
}

#[allow(dead_code)]
fn recv(socket: Socket, frame: Option<Frame>) -> Option<Result<Frame>> {
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

#[cfg(feature = "test_all")]
#[test]
fn test_bind() {
    Socket::bind(ifname()).unwrap();
}

#[test]
#[should_panic(expected = "No such device")]
fn test_bind_no_device() {
    let ifname = CString::new("NO DEVICE").unwrap();
    Socket::bind(ifname).unwrap();
}

#[cfg(feature = "test_all")]
#[test]
#[should_panic(expected = "WouldBlock")]
fn test_nonblocking() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_nonblocking(true).unwrap();

    recv(socket, None).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
#[should_panic(expected = "None")]
fn test_no_nonblocking() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();

    let _ = recv(socket, None).unwrap();
}

#[cfg(feature = "test_all")]
#[test]
fn test_recv_own_msgs() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = Frame::Standard(rand::random());
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
#[should_panic(expected = "None")]
fn test_no_recv_own_msgs() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = Frame::Standard(rand::random());
    socket.send(&frame).unwrap();
    let _ = recv(socket, Some(frame)).unwrap();
}

#[cfg(feature = "test_all")]
#[test]
fn test_fd_frames() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();
    socket.set_fd_frames(true).unwrap();

    let frame = Frame::FdStandard(rand::random());
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
#[should_panic(expected = "InvalidInput")]
fn test_no_fd_frames() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = Frame::FdStandard(rand::random());
    socket.send(&frame).unwrap();
}
