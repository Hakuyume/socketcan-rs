use crate::{linux_can, CanFdFrame};
use std::ffi::CString;
use std::io::{Error, Result};
use std::mem;
use std::os::raw::c_int;
use std::os::unix::io::{AsRawFd, RawFd};

pub struct CanSocket(RawFd);

impl CanSocket {
    pub fn new() -> Result<Self> {
        let socket = unsafe { libc::socket(libc::PF_CAN, libc::SOCK_RAW, linux_can::CAN_RAW as _) };
        if socket == -1 {
            return Err(Error::last_os_error());
        }
        Ok(Self(socket))
    }

    pub fn bind<I>(&self, ifname: I) -> Result<()>
    where
        I: Into<Vec<u8>>,
    {
        let ifname = CString::new(ifname)?;
        let ifindex = unsafe { libc::if_nametoindex(ifname.as_ptr()) };
        if ifindex == 0 {
            return Err(Error::last_os_error());
        }

        let mut address =
            unsafe { mem::MaybeUninit::<linux_can::sockaddr_can>::zeroed().assume_init() };
        address.can_family = libc::AF_CAN as _;
        address.can_ifindex = ifindex as _;

        if unsafe {
            libc::bind(
                self.as_raw_fd(),
                &address as *const _ as _,
                mem::size_of_val(&address) as _,
            ) != 0
        } {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn set_fd_frames(&self, enable: bool) -> Result<()> {
        let opt: c_int = if enable { 1 } else { 0 };
        if unsafe {
            libc::setsockopt(
                self.as_raw_fd(),
                linux_can::SOL_CAN_RAW as _,
                linux_can::CAN_RAW_FD_FRAMES as _,
                &opt as *const _ as _,
                mem::size_of_val(&opt) as _,
            )
        } != 0
        {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn read_frame(&self) -> Result<CanFdFrame> {
        let mut frame = unsafe { mem::MaybeUninit::<CanFdFrame>::zeroed().assume_init() };
        if unsafe {
            libc::read(
                self.as_raw_fd(),
                &mut frame as *mut _ as _,
                mem::size_of_val(&frame),
            )
        } as usize
            != mem::size_of_val(&frame)
        {
            return Err(Error::last_os_error());
        }
        Ok(frame)
    }

    pub fn write_frame(&self, frame: &CanFdFrame) -> Result<()> {
        if unsafe {
            libc::write(
                self.as_raw_fd(),
                frame as *const _ as _,
                mem::size_of_val(frame),
            )
        } as usize
            != mem::size_of_val(frame)
        {
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
