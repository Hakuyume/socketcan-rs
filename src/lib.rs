mod linux_can;

use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};
use std::mem;
use std::os::raw::c_int;
use std::os::unix::io::{AsRawFd, RawFd};

pub struct CanSocket(RawFd);

impl CanSocket {
    fn new() -> Result<Self> {
        let socket = unsafe { libc::socket(libc::PF_CAN, libc::SOCK_RAW, linux_can::CAN_RAW as _) };
        if socket == -1 {
            return Err(Error::last_os_error());
        }
        Ok(Self(socket))
    }

    pub fn bind<I>(ifname: I) -> Result<Self>
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

        let socket = Self::new()?;
        if unsafe {
            libc::bind(
                socket.as_raw_fd(),
                &address as *const _ as _,
                mem::size_of_val(&address) as _,
            ) != 0
        } {
            return Err(Error::last_os_error());
        }
        Ok(socket)
    }

    pub fn set_can_fd_frames(&self, enable: bool) -> Result<()> {
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

pub type CanFdFrame = linux_can::canfd_frame;

impl CanFdFrame {
    pub fn new(can_id: u32, data: &[u8]) -> Result<Self> {
        let mut frame = unsafe { mem::MaybeUninit::<Self>::zeroed().assume_init() };
        frame.can_id = can_id;
        let len = match data.len() {
            len @ 0..=8 => len,
            9..=12 => 12,
            13..=16 => 16,
            17..=20 => 20,
            21..=24 => 24,
            25..=32 => 32,
            33..=48 => 48,
            49..=64 => 64,
            _ => return Err(ErrorKind::InvalidInput.into()),
        };
        frame.data[..data.len()].copy_from_slice(data);
        frame.len = len as _;
        Ok(frame)
    }
}
