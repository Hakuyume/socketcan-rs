use crate::CanFdFrame;
use futures::future::poll_fn;
use futures::ready;
use mio::event::Evented;
use mio::unix::{EventedFd, UnixReady};
use mio::Ready;
use mio::{PollOpt, Token};
use std::ffi::CStr;
use std::io::{ErrorKind, Result};
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::PollEvented;

pub struct CanSocket(PollEvented<EventedCanSocket>);
pub struct RecvHalf(Arc<CanSocket>);
pub struct SendHalf(Arc<CanSocket>);

impl CanSocket {
    pub fn bind<I>(ifname: I) -> Result<Self>
    where
        I: AsRef<CStr>,
    {
        let socket = crate::CanSocket::bind(ifname)?;
        socket.set_nonblocking(true)?;
        Ok(Self(PollEvented::new(EventedCanSocket(socket))?))
    }

    pub fn set_recv_own_msgs(&self, enable: bool) -> Result<()> {
        self.0.get_ref().0.set_recv_own_msgs(enable)
    }

    pub fn set_fd_frames(&self, enable: bool) -> Result<()> {
        self.0.get_ref().0.set_fd_frames(enable)
    }

    pub async fn recv(&mut self) -> Result<CanFdFrame> {
        poll_fn(|cx| self.poll_recv(cx)).await
    }

    fn poll_recv(&self, cx: &mut Context<'_>) -> Poll<Result<CanFdFrame>> {
        let ready = Ready::readable() | Ready::from(UnixReady::error());
        ready!(self.0.poll_read_ready(cx, ready))?;
        match self.0.get_ref().0.recv() {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                self.0.clear_read_ready(cx, ready)?;
                Poll::Pending
            }
            r => Poll::Ready(r),
        }
    }

    pub async fn send(&mut self, frame: &CanFdFrame) -> Result<()> {
        poll_fn(|cx| self.poll_send(cx, frame)).await
    }

    fn poll_send(&self, cx: &mut Context<'_>, frame: &CanFdFrame) -> Poll<Result<()>> {
        ready!(self.0.poll_write_ready(cx))?;
        match self.0.get_ref().0.send(frame) {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                self.0.clear_write_ready(cx)?;
                Poll::Pending
            }
            r => Poll::Ready(r),
        }
    }

    pub fn split(self) -> (RecvHalf, SendHalf) {
        let socket = Arc::new(self);
        (RecvHalf(socket.clone()), SendHalf(socket))
    }
}

impl RecvHalf {
    pub async fn recv(&mut self) -> Result<CanFdFrame> {
        poll_fn(|cx| self.0.poll_recv(cx)).await
    }
}

impl SendHalf {
    pub async fn send(&mut self, frame: &CanFdFrame) -> Result<()> {
        poll_fn(|cx| self.0.poll_send(cx, frame)).await
    }
}

struct EventedCanSocket(crate::CanSocket);

impl Evented for EventedCanSocket {
    fn register(
        &self,
        poll: &mio::Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> Result<()> {
        EventedFd(&self.0.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &mio::Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> Result<()> {
        EventedFd(&self.0.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> Result<()> {
        EventedFd(&self.0.as_raw_fd()).deregister(poll)
    }
}
