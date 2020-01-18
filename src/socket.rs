use crate::{sys, Frame};
use std::ffi::CStr;
use std::io::{Error, Result};
use std::mem::{self, size_of, size_of_val, MaybeUninit};
use std::os::raw::c_int;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

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
        let opt = nonblocking as c_int;
        if unsafe { libc::ioctl(self.as_raw_fd(), libc::FIONBIO, &opt) } != 0 {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn set_recv_own_msgs(&self, enable: bool) -> Result<()> {
        let opt = enable as c_int;
        if unsafe {
            libc::setsockopt(
                self.as_raw_fd(),
                sys::SOL_CAN_RAW as _,
                sys::CAN_RAW_RECV_OWN_MSGS as _,
                &opt as *const _ as _,
                size_of_val(&opt) as _,
            )
        } != 0
        {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn set_fd_frames(&self, enable: bool) -> Result<()> {
        let opt = enable as c_int;
        if unsafe {
            libc::setsockopt(
                self.as_raw_fd(),
                sys::SOL_CAN_RAW as _,
                sys::CAN_RAW_FD_FRAMES as _,
                &opt as *const _ as _,
                size_of_val(&opt) as _,
            )
        } != 0
        {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn recv(&self) -> Result<Frame> {
        #[repr(C)]
        union Inner {
            can: sys::can_frame,
            canfd: sys::canfd_frame,
        }
        let mut inner = MaybeUninit::<Inner>::uninit();
        let size = unsafe {
            libc::read(
                self.as_raw_fd(),
                inner.as_mut_ptr() as _,
                size_of::<Inner>(),
            )
        } as usize;
        if size == size_of::<sys::can_frame>() {
            Ok(unsafe { inner.assume_init().can }.into())
        } else if size == size_of::<sys::canfd_frame>() {
            Ok(unsafe { inner.assume_init().canfd }.into())
        } else {
            Err(Error::last_os_error())
        }
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
mod tests;
