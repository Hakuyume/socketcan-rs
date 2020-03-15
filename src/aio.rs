use crate::{CmsgIter, Frame, Timestamping};
use futures::future::poll_fn;
use futures::ready;
use mio::event::Evented;
use mio::unix::{EventedFd, UnixReady};
use mio::Ready;
use mio::{PollOpt, Token};
use std::ffi::CStr;
use std::io::{ErrorKind, Result};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr::NonNull;
use std::task::Poll;
use tokio::io::PollEvented;

pub struct Socket(PollEvented<Inner>);

impl Socket {
    pub fn bind<I>(ifname: I) -> Result<Self>
    where
        I: AsRef<CStr>,
    {
        let socket = crate::Socket::bind(ifname)?;
        socket.set_nonblocking(true)?;
        Ok(Self(PollEvented::new(Inner(socket))?))
    }

    pub fn set_timestamping(&self, timestamping: Timestamping) -> Result<()> {
        self.0.get_ref().0.set_timestamping(timestamping)
    }

    pub fn set_recv_own_msgs(&self, enable: bool) -> Result<()> {
        self.0.get_ref().0.set_recv_own_msgs(enable)
    }

    pub fn set_fd_frames(&self, enable: bool) -> Result<()> {
        self.0.get_ref().0.set_fd_frames(enable)
    }

    pub async fn recv(&mut self) -> Result<Frame> {
        let ready = Ready::readable() | Ready::from(UnixReady::error());
        poll_fn(|cx| {
            ready!(self.0.poll_read_ready(cx, ready))?;
            match self.0.get_ref().0.recv() {
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    self.0.clear_read_ready(cx, ready)?;
                    Poll::Pending
                }
                r => Poll::Ready(r),
            }
        })
        .await
    }

    #[allow(clippy::needless_lifetimes)]
    pub async fn recv_msg<'a>(
        &mut self,
        cmsg_buf: &'a mut [u8],
    ) -> Result<(Frame, Option<CmsgIter<'a>>)> {
        let ready = Ready::readable() | Ready::from(UnixReady::error());
        let mut cmsg_buf = NonNull::new(cmsg_buf);
        poll_fn(|cx| {
            ready!(self.0.poll_read_ready(cx, ready))?;
            match self
                .0
                .get_ref()
                .0
                .recv_msg(unsafe { &mut *cmsg_buf.unwrap().as_ptr() })
            {
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    self.0.clear_read_ready(cx, ready)?;
                    Poll::Pending
                }
                r => {
                    cmsg_buf = None;
                    Poll::Ready(r)
                }
            }
        })
        .await
    }

    pub async fn send(&mut self, frame: &Frame) -> Result<()> {
        poll_fn(|cx| {
            ready!(self.0.poll_write_ready(cx))?;
            match self.0.get_ref().0.send(frame) {
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    self.0.clear_write_ready(cx)?;
                    Poll::Pending
                }
                r => Poll::Ready(r),
            }
        })
        .await
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0.get_ref().0.as_raw_fd()
    }
}

struct Inner(crate::Socket);

impl Evented for Inner {
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

#[cfg(test)]
mod tests;
