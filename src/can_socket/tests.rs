use super::CanSocket;
use crate::CanFrame;
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::env;
use std::ffi::CString;
use std::io::ErrorKind;
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

fn lock_shared() -> RwLockReadGuard<'static, ()> {
    LOCK.read()
}

fn lock_exclusive() -> RwLockWriteGuard<'static, ()> {
    let lock = LOCK.write();
    thread::sleep(Duration::from_millis(100));
    lock
}

fn recv(socket: CanSocket, frame: Option<CanFrame>) -> Option<Result<CanFrame>> {
    let is_done = Arc::new(AtomicBool::new(false));
    let handle = {
        let is_done = is_done.clone();
        thread::spawn(move || {
            let r = loop {
                let f = socket.recv()?;
                match &frame {
                    Some(frame) => {
                        if &f == frame {
                            break Ok(f);
                        }
                    }
                    None => break Ok(f),
                }
            };
            is_done.store(true, Ordering::Relaxed);
            r
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
fn test_bind() {
    CanSocket::bind(ifname()).unwrap();
}

#[test]
#[should_panic]
fn test_bind_no_device() {
    let ifname = CString::new("NO DEVICE").unwrap();
    CanSocket::bind(ifname).unwrap();
}

#[test]
fn test_blocking() {
    let _ = lock_exclusive();
    let socket = CanSocket::bind(ifname()).unwrap();

    assert!(recv(socket, None).is_none());
}

#[test]
fn test_nonblocking() {
    let _ = lock_exclusive();
    let socket = CanSocket::bind(ifname()).unwrap();
    socket.set_nonblocking(true).unwrap();

    assert_eq!(
        recv(socket, None).unwrap().unwrap_err().kind(),
        ErrorKind::WouldBlock
    );
}

#[test]
fn test_recv_own_msgs() {
    let _ = lock_shared();
    let socket = CanSocket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = rand::random();
    socket.send(&CanFrame::Standard(frame)).unwrap();
    recv(socket, Some(CanFrame::Standard(frame)))
        .unwrap()
        .unwrap();
}

#[test]
fn test_no_recv_own_msgs() {
    let _ = lock_shared();
    let socket = CanSocket::bind(ifname()).unwrap();

    let frame = rand::random();
    socket.send(&CanFrame::Standard(frame)).unwrap();
    assert!(recv(socket, Some(CanFrame::Standard(frame))).is_none());
}
