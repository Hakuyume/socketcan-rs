use std::mem::{size_of, MaybeUninit};
use std::ptr;

#[non_exhaustive]
pub enum Cmsg<'a> {
    Timestamping {
        software: libc::timespec,
        hardware: libc::timespec,
    },
    #[doc(hidden)]
    Other(&'a libc::cmsghdr),
}

impl<'a> Cmsg<'a> {
    pub(crate) unsafe fn from_raw(cmsg: &'a libc::cmsghdr) -> Self {
        match (cmsg.cmsg_level, cmsg.cmsg_type) {
            (libc::SOL_SOCKET, libc::SO_TIMESTAMPING) => {
                let mut ts = MaybeUninit::<[libc::timespec; 3]>::uninit();
                ptr::copy_nonoverlapping(
                    libc::CMSG_DATA(cmsg),
                    ts.as_mut_ptr() as _,
                    size_of::<[libc::timespec; 3]>(),
                );
                let ts = ts.assume_init();
                Self::Timestamping {
                    software: ts[0],
                    hardware: ts[2],
                }
            }
            _ => Self::Other(cmsg),
        }
    }
}
