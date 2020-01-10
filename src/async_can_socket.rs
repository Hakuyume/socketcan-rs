use crate::{CanFdFrame, CanSocket};
use futures::future::poll_fn;
use futures::ready;
use mio::event::Evented;
use mio::unix::EventedFd;
use mio::{PollOpt, Token};
use std::io::{ErrorKind, Result};
use std::os::unix::io::AsRawFd;
use std::task::{Context, Poll};
use tokio::io::PollEvented;

pub struct AsyncCanSocket(PollEvented<CanSocket>);

impl AsyncCanSocket {
    pub fn new() -> Result<Self> {
        let socket = CanSocket::new()?;
        socket.set_nonblocking(true)?;
        Ok(Self(PollEvented::new(socket)?))
    }

    pub fn bind<I>(&self, ifname: I) -> Result<()>
    where
        I: Into<Vec<u8>>,
    {
        self.0.get_ref().bind(ifname)
    }

    pub fn set_recv_own_msgs(&self, enable: bool) -> Result<()> {
        self.0.get_ref().set_recv_own_msgs(enable)
    }

    pub fn set_fd_frames(&self, enable: bool) -> Result<()> {
        self.0.get_ref().set_fd_frames(enable)
    }

    pub async fn recv(&self) -> Result<CanFdFrame> {
        poll_fn(|cx| self.poll_recv(cx)).await
    }

    fn poll_recv(&self, cx: &mut Context<'_>) -> Poll<Result<CanFdFrame>> {
        ready!(self.0.poll_read_ready(cx, mio::Ready::readable()))?;
        match self.0.get_ref().recv() {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                self.0.clear_read_ready(cx, mio::Ready::readable())?;
                Poll::Pending
            }
            r => Poll::Ready(r),
        }
    }

    pub async fn send(&self, frame: &CanFdFrame) -> Result<()> {
        poll_fn(|cx| self.poll_send(cx, frame)).await
    }

    fn poll_send(&self, cx: &mut Context<'_>, frame: &CanFdFrame) -> Poll<Result<()>> {
        ready!(self.0.poll_write_ready(cx))?;
        match self.0.get_ref().send(frame) {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                self.0.clear_write_ready(cx)?;
                Poll::Pending
            }
            r => Poll::Ready(r),
        }
    }
}

impl Evented for CanSocket {
    fn register(
        &self,
        poll: &mio::Poll,
        token: Token,
        interest: mio::Ready,
        opts: PollOpt,
    ) -> Result<()> {
        EventedFd(&self.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &mio::Poll,
        token: Token,
        interest: mio::Ready,
        opts: PollOpt,
    ) -> Result<()> {
        EventedFd(&self.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> Result<()> {
        EventedFd(&self.as_raw_fd()).deregister(poll)
    }
}
