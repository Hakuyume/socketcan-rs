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
                let mut timestamping = MaybeUninit::<[libc::timespec; 3]>::uninit();
                let timestamping = unsafe {
                    ptr::copy_nonoverlapping(
                        libc::CMSG_DATA(value),
                        timestamping.as_mut_ptr() as _,
                        size_of::<[libc::timespec; 3]>(),
                    );
                    timestamping.assume_init()
                };
                Self::Timestamping {
                    software: timestamping[0],
                    hardware: timestamping[2],
                }
            }
            _ => Self::Other(value),
        }
    }
}
