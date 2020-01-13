use crate::{sys, CanFdFrame, CanFrame, Frame};
use std::ffi::CStr;
use std::io::{Error, Result};
use std::mem::{self, size_of, size_of_val, MaybeUninit};
use std::os::raw::c_int;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub struct CanSocket(RawFd);

impl CanSocket {
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
        union Buffer {
            can: sys::can_frame,
            can_fd: sys::canfd_frame,
        }
        let mut frame = MaybeUninit::<Buffer>::uninit();
        let len = unsafe {
            libc::read(
                self.as_raw_fd(),
                frame.as_mut_ptr() as _,
                size_of_val(&frame),
            )
        } as usize;
        if len == size_of::<sys::can_frame>() {
            let frame = unsafe { frame.assume_init().can };
            Ok(Frame::Can(CanFrame(frame)))
        } else if len == size_of::<sys::canfd_frame>() {
            let frame = unsafe { frame.assume_init().can_fd };
            Ok(Frame::CanFd(CanFdFrame(frame)))
        } else {
            Err(Error::last_os_error())
        }
    }

    pub fn send(&self, frame: &Frame) -> Result<()> {
        let (frame, len) = match frame {
            Frame::Can(CanFrame(frame)) => (frame as *const _ as _, size_of_val(frame)),
            Frame::CanFd(CanFdFrame(frame)) => (frame as *const _ as _, size_of_val(frame)),
        };
        if unsafe { libc::write(self.as_raw_fd(), frame, len as _) } as usize != len {
            return Err(Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for CanSocket {
    fn drop(&mut self) {
        unsafe { libc::close(self.as_raw_fd()) };
    }
}

impl AsRawFd for CanSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl FromRawFd for CanSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self(fd)
    }
}

impl IntoRawFd for CanSocket {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.0;
        mem::forget(self);
        fd
    }
}
