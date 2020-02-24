use std::mem::{align_of, size_of};

#[non_exhaustive]
pub enum Cmsg<'a> {
    Timestamping(&'a [libc::timespec; 3]),
    #[doc(hidden)]
    Other(&'a libc::cmsghdr),
}

impl<'a> Cmsg<'a> {
    pub(crate) unsafe fn from_raw(cmsg: &'a libc::cmsghdr) -> Self {
        match (cmsg.cmsg_level, cmsg.cmsg_type) {
            (libc::SOL_SOCKET, libc::SO_TIMESTAMPING) => Self::Timestamping(cmsg_data(cmsg)),
            _ => Self::Other(cmsg),
        }
    }
}

unsafe fn cmsg_data<T>(cmsg: &libc::cmsghdr) -> &T {
    assert_eq!(cmsg.cmsg_len, libc::CMSG_LEN(size_of::<T>() as _) as _);
    let data = libc::CMSG_DATA(cmsg);
    assert_eq!(data.align_offset(align_of::<T>()), 0);
    &*(data as *const T)
}
