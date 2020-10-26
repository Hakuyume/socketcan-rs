use super::Socket;
use crate::socket::tests::{ifname, random_data_standard, random_fd_data_standard, LOCK};
use crate::{Cmsg, Frame, Timestamping};
use std::ffi::CString;
use std::io::ErrorKind;
use std::io::Result;
use std::time::Duration;
use tokio::time::{sleep, timeout};

macro_rules! lock {
    (shared) => {
        let _lock = LOCK.read();
    };
    (exclusive) => {
        let _lock = LOCK.write();
    };
}

async fn recv(socket: Socket, query: Option<Frame>) -> Option<Result<()>> {
    timeout(Duration::from_millis(100), async {
        loop {
            let frame = socket.recv().await?;
            if query.as_ref().map(|query| &frame == query).unwrap_or(true) {
                return Ok(());
            }
        }
    })
    .await
    .ok()
}

async fn recv_msg(socket: Socket, query: Option<Frame>) -> Option<Result<Option<libc::timespec>>> {
    timeout(Duration::from_millis(100), async {
        let mut cmsg_buf = vec![0; Cmsg::space()];
        loop {
            let (frame, cmsgs) = socket.recv_msg(&mut cmsg_buf).await?;
            if query.as_ref().map(|query| &frame == query).unwrap_or(true) {
                let timestamp = cmsgs.into_iter().flatten().find_map(|cmsg| match cmsg {
                    Cmsg::Timestamping(ts) => Some(ts[0]),
                    _ => None,
                });
                return Ok(timestamp);
            }
        }
    })
    .await
    .ok()
}

#[tokio::test]
#[ignore]
async fn test_bind() {
    Socket::bind(ifname()).unwrap();
}

#[tokio::test]
async fn test_bind_no_device() {
    let ifname = CString::new("NO DEVICE").unwrap();
    assert!(Socket::bind(ifname).is_err());
}

#[tokio::test]
#[ignore]
async fn test_nonblocking_on() {
    lock!(exclusive);
    let socket = Socket::bind(ifname()).unwrap();

    assert!(recv(socket, None).await.is_none());
}

#[tokio::test]
#[ignore]
async fn test_default_timestamping_off() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket_tx.send(&frame).await.unwrap();
    assert!(recv_msg(socket_rx, Some(frame))
        .await
        .unwrap()
        .unwrap()
        .is_none());
}

#[tokio::test]
#[ignore]
async fn test_set_timestamping_on() {
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
    socket_tx.send(&frame).await.unwrap();
    let timestamp0 = recv_msg(socket_rx0, Some(frame))
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    sleep(Duration::from_millis(100)).await;
    let timestamp1 = recv_msg(socket_rx1, Some(frame))
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(timestamp0.tv_sec != 0);
    assert!(timestamp1.tv_sec != 0);
    assert_eq!(
        (timestamp0.tv_sec, timestamp0.tv_nsec),
        (timestamp1.tv_sec, timestamp1.tv_nsec)
    );
}

#[tokio::test]
#[ignore]
async fn test_default_loopback_on() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket_tx.send(&frame).await.unwrap();
    recv(socket_rx, Some(frame)).await.unwrap().unwrap();
}

#[tokio::test]
#[ignore]
async fn test_default_recv_own_msgs_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = random_data_standard();
    socket.send(&frame).await.unwrap();
    assert!(recv(socket, Some(frame)).await.is_none());
}

#[tokio::test]
#[ignore]
async fn test_set_recv_own_msgs_on() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();
    socket.set_recv_own_msgs(true).unwrap();

    let frame = random_data_standard();
    socket.send(&frame).await.unwrap();
    recv(socket, Some(frame)).await.unwrap().unwrap();
}

#[tokio::test]
#[ignore]
async fn test_default_fd_frames_off() {
    lock!(shared);
    let socket = Socket::bind(ifname()).unwrap();

    let frame = random_fd_data_standard();
    assert_eq!(
        socket.send(&frame).await.unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[tokio::test]
#[ignore]
async fn test_set_fd_frames_on() {
    lock!(shared);
    let socket_tx = Socket::bind(ifname()).unwrap();
    let socket_rx = Socket::bind(ifname()).unwrap();
    socket_tx.set_fd_frames(true).unwrap();
    socket_rx.set_fd_frames(true).unwrap();

    let frame = random_fd_data_standard();
    socket_tx.send(&frame).await.unwrap();
    recv(socket_rx, Some(frame)).await.unwrap().unwrap();
}

#[test]
fn test_marker_traits() {
    fn check<F>(_: F)
    where
        F: Send,
    {
    }

    check(async {
        let ifname = CString::new("NO DEVICE").unwrap();
        let socket = Socket::bind(ifname).unwrap();

        socket.recv().await.unwrap();

        let mut cmsg_buf = Vec::new();
        socket.recv_msg(&mut cmsg_buf).await.unwrap();

        let frame = random_data_standard();
        socket.send(&frame).await.unwrap();
    })
}
