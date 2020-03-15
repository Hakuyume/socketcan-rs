use super::Socket;
use crate::{sys, Cmsg, DataFrame, FdDataFrame, Frame, Id, Timestamping};
use rand::Rng;
use spin::RwLock;
use std::env;
use std::ffi::CString;
use std::io::ErrorKind;
use std::io::Result;
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub(crate) static LOCK: RwLock<()> = RwLock::new(());

pub(crate) fn ifname() -> CString {
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

fn timeout<F, T>(f: F) -> Option<T>
where
    F: 'static + Send + FnOnce() -> T,
    T: 'static + Send,
{
    struct Context(Arc<AtomicBool>);
    impl Drop for Context {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    let is_done = Arc::new(AtomicBool::new(false));
    let cxt = Context(is_done.clone());
    let handle = thread::spawn(move || {
        let _cxt = cxt;
        f()
    });

    thread::sleep(Duration::from_millis(100));
    if is_done.load(Ordering::Relaxed) {
        handle.join().ok()
    } else {
        None
    }
}

fn recv(socket: Socket, query: Option<Frame>) -> Option<Result<()>> {
    timeout(move || loop {
        let frame = socket.recv()?;
        if query.as_ref().map(|query| &frame == query).unwrap_or(true) {
            return Ok(());
        }
    })
}

fn recv_msg(socket: Socket, query: Option<Frame>) -> Option<Result<Option<libc::timespec>>> {
    timeout(move || {
        let mut cmsg_buf = vec![0; Cmsg::space()];
        loop {
            let (frame, cmsgs) = socket.recv_msg(&mut cmsg_buf)?;
            if query.as_ref().map(|query| &frame == query).unwrap_or(true) {
                let timestamp = cmsgs.into_iter().flatten().find_map(|cmsg| match cmsg {
                    Cmsg::Timestamping(ts) => Some(ts[0]),
                    _ => None,
                });
                return Ok(timestamp);
            }
        }
    })
}

pub(crate) fn random_data_standard() -> Frame {
    let mut rng = rand::thread_rng();
    let id = Id::Standard(rng.gen_range(0, sys::CAN_SFF_MASK));
    let data = (0..rng.gen_range(0, sys::CAN_MAX_DLEN))
        .map(|_| rng.gen())
        .collect::<Vec<_>>();
    Frame::Data(DataFrame::new(id, &data))
}

pub(crate) fn random_fd_data_standard() -> Frame {
    let mut rng = rand::thread_rng();
    let id = Id::Standard(rng.gen_range(0, sys::CAN_SFF_MASK));
    let data = (0..rng.gen_range(0, sys::CANFD_MAX_DLEN))
        .map(|_| rng.gen())
        .collect::<Vec<_>>();
    Frame::FdData(FdDataFrame::new(id, false, false, &data))
}

#[test]
#[ignore]
fn test_bind() {
    Socket::bind(ifname()).unwrap();
}

#[test]
fn test_bind_no_device() {
    let ifname = CString::new("NO DEVICE").unwrap();
    assert!(Socket::bind(ifname).is_err());
}

#[test]
#[ignore]
fn test_default_nonblocking_off() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();

    assert!(recv(socket, None).is_none());
}

#[test]
#[ignore]
fn test_set_nonblocking_on() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_nonblocking(true).unwrap();

    assert_eq!(
        recv(socket, None).unwrap().unwrap_err().kind(),
        ErrorKind::WouldBlock
    );
}

#[test]
#[ignore]
fn test_default_timestamping_off() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket_tx.send(&frame).unwrap();
    assert!(recv_msg(socket_rx, Some(frame)).unwrap().unwrap().is_none());
}

#[test]
#[ignore]
fn test_set_timestamping_on() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx0 = Socket::bind(ifname()).unwrap();
    let socket_rx1 = Socket::bind(ifname()).unwrap();
    socket_rx0
        .set_timestamping(Timestamping::RX_SOFTWARE | Timestamping::SOFTWARE)
        .unwrap();
    socket_rx1
        .set_timestamping(Timestamping::RX_SOFTWARE | Timestamping::SOFTWARE)
        .unwrap();

    let frame = random_data_standard();
    socket_tx.send(&frame).unwrap();
    let timestamp0 = recv_msg(socket_rx0, Some(frame)).unwrap().unwrap().unwrap();
    thread::sleep(Duration::from_millis(100));
    let timestamp1 = recv_msg(socket_rx1, Some(frame)).unwrap().unwrap().unwrap();
    assert!(timestamp0.tv_sec != 0);
    assert!(timestamp1.tv_sec != 0);
    assert_eq!(
        (timestamp0.tv_sec, timestamp0.tv_nsec),
        (timestamp1.tv_sec, timestamp1.tv_nsec)
    );
}

#[test]
#[ignore]
fn test_default_loopback_on() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket_tx.send(&frame).unwrap();
    recv(socket_rx, Some(frame)).unwrap().unwrap();
}

#[test]
#[ignore]
fn test_default_recv_own_msgs_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket.send(&frame).unwrap();
    assert!(recv(socket, Some(frame)).is_none());
}

#[test]
#[ignore]
fn test_set_recv_own_msgs_on() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = random_data_standard();
    socket.send(&frame).unwrap();
    recv(socket, Some(frame)).unwrap().unwrap();
}

#[test]
#[ignore]
fn test_default_fd_frames_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = random_fd_data_standard();
    assert_eq!(
        socket.send(&frame).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[test]
#[ignore]
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
