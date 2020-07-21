use crate::{sys, CmsgIter, Frame, Timestamping};
use std::ffi::CStr;
use std::io::{Error, Result};
use std::mem::{self, size_of, size_of_val, MaybeUninit};
use std::os::raw::c_int;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::ptr;

pub struct Socket(RawFd);

impl Socket {
    pub fn bind<I>(ifname: I) -> Result<Self>
    where
        I: AsRef<CStr>,
    {
        let ifindex = unsafe { libc::if_nametoindex(ifname.as_ref().as_ptr()) };
        if ifindex == 0 {
            return Err(Error::last_os_error());
        }

        let fd = unsafe { libc::socket(libc::PF_CAN, libc::SOCK_RAW, sys::CAN_RAW as _) };
        if fd == -1 {
            return Err(Error::last_os_error());
        }
        let socket = Self(fd);

        let mut address = MaybeUninit::<sys::sockaddr_can>::zeroed();
        let address = unsafe {
            (*address.as_mut_ptr()).can_family = libc::AF_CAN as _;
            (*address.as_mut_ptr()).can_ifindex = ifindex as _;
            address.assume_init()
        };
        if unsafe {
            libc::bind(
                socket.as_raw_fd(),
                &address as *const _ as _,
                size_of_val(&address) as _,
            ) != 0
        } {
            return Err(Error::last_os_error());
        }
        Ok(socket)
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        if unsafe { libc::ioctl(self.as_raw_fd(), libc::FIONBIO, &(nonblocking as c_int)) } != 0 {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    unsafe fn setsockopt<T>(&self, level: c_int, name: c_int, value: &T) -> Result<()> {
        if libc::setsockopt(
            self.as_raw_fd(),
            level,
            name,
            value as *const _ as _,
            size_of_val(value) as _,
        ) != 0
        {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn set_timestamping(&self, timestamping: Timestamping) -> Result<()> {
        unsafe {
            self.setsockopt(
                libc::SOL_SOCKET,
                libc::SO_TIMESTAMPING,
                &(timestamping.bits() as c_int),
            )
        }
    }

    pub fn set_recv_own_msgs(&self, enable: bool) -> Result<()> {
        unsafe {
            self.setsockopt(
                sys::SOL_CAN_RAW as _,
                sys::CAN_RAW_RECV_OWN_MSGS as _,
                &(enable as c_int),
            )
        }
    }

    pub fn set_fd_frames(&self, enable: bool) -> Result<()> {
        unsafe {
            self.setsockopt(
                sys::SOL_CAN_RAW as _,
                sys::CAN_RAW_FD_FRAMES as _,
                &(enable as c_int),
            )
        }
    }

    pub fn recv(&self) -> Result<Frame> {
        let mut frame = MaybeUninit::<sys::canfd_frame>::uninit();
        unsafe {
            let size = libc::read(
                self.as_raw_fd(),
                frame.as_mut_ptr() as _,
                size_of::<sys::canfd_frame>(),
            );
            Frame::from_raw(frame, size as _)
        }
        .ok_or_else(Error::last_os_error)
    }

    pub(crate) fn _recv_msg<'a>(
        &self,
        cmsg_buf: &'a mut [u8],
    ) -> std::result::Result<(Frame, Option<CmsgIter<'a>>), (Error, &'a mut [u8])> {
        let mut frame = MaybeUninit::<sys::canfd_frame>::uninit();
        let mut iov = MaybeUninit::<libc::iovec>::uninit();
        let mut msg = MaybeUninit::<libc::msghdr>::uninit();
        unsafe {
            (*iov.as_mut_ptr()).iov_base = frame.as_mut_ptr() as _;
            (*iov.as_mut_ptr()).iov_len = size_of::<sys::canfd_frame>();

            (*msg.as_mut_ptr()).msg_name = ptr::null_mut();
            (*msg.as_mut_ptr()).msg_iov = iov.as_mut_ptr();
            (*msg.as_mut_ptr()).msg_iovlen = 1;
            (*msg.as_mut_ptr()).msg_control = cmsg_buf.as_mut_ptr() as _;
            (*msg.as_mut_ptr()).msg_controllen = cmsg_buf.len();

            let size = libc::recvmsg(self.as_raw_fd(), msg.as_mut_ptr(), 0);
            // frame will be moved
            (*iov.as_mut_ptr()).iov_base = ptr::null_mut();
            let frame = Frame::from_raw(frame, size as _)
                .ok_or_else(|| (Error::last_os_error(), cmsg_buf))?;
            let cmsgs = CmsgIter::from_raw(msg.assume_init());
            Ok((frame, cmsgs))
        }
    }

    pub fn recv_msg<'a>(&self, cmsg_buf: &'a mut [u8]) -> Result<(Frame, Option<CmsgIter<'a>>)> {
        self._recv_msg(cmsg_buf).map_err(|(e, _)| e)
    }

    pub fn send(&self, frame: &Frame) -> Result<()> {
        if unsafe { libc::write(self.as_raw_fd(), frame.as_ptr(), frame.size()) } as usize
            != frame.size()
        {
            return Err(Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { libc::close(self.as_raw_fd()) };
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl FromRawFd for Socket {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self(fd)
    }
}

impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.0;
        mem::forget(self);
        fd
    }
}

#[cfg(test)]
pub(crate) mod tests;
