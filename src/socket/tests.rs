use super::Socket;
use crate::{sys, DataFrame, FdDataFrame, Frame, Id};
use rand::Rng;
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
                if frame.as_ref().map(|frame| &f == frame).unwrap_or(true) {
                    break Ok(f);
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

fn random_data_standard() -> Frame {
    let mut rng = rand::thread_rng();
    let id = Id::Standard(rng.gen_range(0, sys::CAN_SFF_MASK));
    let data = (0..rng.gen_range(0, sys::CAN_MAX_DLEN))
        .map(|_| rng.gen())
        .collect::<Vec<_>>();
    Frame::Data(DataFrame::new(id, &data))
}

fn random_fd_data_standard() -> Frame {
    let mut rng = rand::thread_rng();
    let id = Id::Standard(rng.gen_range(0, sys::CAN_SFF_MASK));
    let data = (0..rng.gen_range(0, sys::CANFD_MAX_DLEN))
        .map(|_| rng.gen())
        .collect::<Vec<_>>();
    Frame::FdData(FdDataFrame::new(id, false, false, &data))
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

    let frame = random_data_standard();
    socket_tx.send(&frame).unwrap();
    recv(socket_rx, Some(frame)).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
fn test_default_recv_own_msgs_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket.send(&frame).unwrap();
    assert!(recv(socket, Some(frame)).is_none());
}

#[cfg(feature = "test_all")]
#[test]
fn test_set_recv_own_msgs_on() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = random_data_standard();
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[cfg(feature = "test_all")]
#[test]
fn test_default_fd_frames_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = random_fd_data_standard();
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

    let frame = random_fd_data_standard();
    socket_tx.send(&frame).unwrap();
    recv(socket_rx, Some(frame)).unwrap().unwrap();
}
