use super::Socket;
use crate::Frame;
use spin::RwLock;
use std::env;
use std::ffi::CString;
use std::io::{ErrorKind, Result};
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
fn test_bind_no_device() {
    let ifname = CString::new("NO DEVICE").unwrap();
    assert!(Socket::bind(ifname).is_err());
}

#[cfg(feature = "test_all")]
#[test]
fn test_default_nonblocking_off() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();

    assert!(recv(socket, None).is_none());
}

#[cfg(feature = "test_all")]
#[test]
fn test_set_nonblocking_on() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_nonblocking(true).unwrap();

    assert_eq!(
        recv(socket, None).unwrap().unwrap_err().kind(),
        ErrorKind::WouldBlock
    );
}

#[cfg(feature = "test_all")]
#[test]
fn test_default_loopback_on() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();

    let frame = Frame::Data(rand::random());
    socket_tx.send(&frame).unwrap();
    recv(socket_rx, Some(frame)).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
fn test_default_recv_own_msgs_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = Frame::Data(rand::random());
    socket.send(&frame).unwrap();
    assert!(recv(socket, Some(frame)).is_none());
}

#[cfg(feature = "test_all")]
#[test]
fn test_set_recv_own_msgs_on() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = Frame::Data(rand::random());
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
fn test_default_fd_frames_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = Frame::FdData(rand::random());
    assert_eq!(
        socket.send(&frame).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[cfg(feature = "test_all")]
#[test]
fn test_set_fd_frames_on() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();
    socket_tx.set_fd_frames(true).unwrap();
    socket_rx.set_fd_frames(true).unwrap();

    let frame = Frame::FdData(rand::random());
    socket_tx.send(&frame).unwrap();
    recv(socket_rx, Some(frame)).unwrap().unwrap();
}
