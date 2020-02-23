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

impl<'a> From<&'a libc::cmsghdr> for Cmsg<'a> {
    fn from(value: &'a libc::cmsghdr) -> Self {
        match (value.cmsg_level, value.cmsg_type) {
            (libc::SOL_SOCKET, libc::SO_TIMESTAMPING) => {
                let mut ts = MaybeUninit::<[libc::timespec; 3]>::uninit();
                let ts = unsafe {
                    ptr::copy_nonoverlapping(
                        libc::CMSG_DATA(value),
                        ts.as_mut_ptr() as _,
                        size_of::<[libc::timespec; 3]>(),
                    );
                    ts.assume_init()
                };
                Self::Timestamping {
                    software: ts[0],
                    hardware: ts[2],
                }
            }
            _ => Self::Other(value),
        }
    }
}
