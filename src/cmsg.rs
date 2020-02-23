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
                let timestamping =
                    unsafe { &*(libc::CMSG_DATA(value) as *const [libc::timespec; 3]) };
                Self::Timestamping {
                    software: timestamping[0],
                    hardware: timestamping[2],
                }
            }
            _ => Self::Other(value),
        }
    }
}
