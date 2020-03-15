use std::mem::{align_of, size_of};

#[non_exhaustive]
pub enum Cmsg<'a> {
    Timestamping(&'a [libc::timespec; 3]),
    #[doc(hidden)]
    Other(&'a libc::cmsghdr),
}

impl<'a> Cmsg<'a> {
    pub fn space() -> usize {
        [size_of::<[libc::timespec; 3]>()]
            .iter()
            .map(|&size| unsafe { libc::CMSG_SPACE(size as _) })
            .max()
            .unwrap_or_default() as _
    }
}

pub struct CmsgIter<'a> {
    msg: libc::msghdr,
    cmsg: Option<&'a libc::cmsghdr>,
}

impl CmsgIter<'_> {
    pub(crate) unsafe fn from_raw(msg: libc::msghdr) -> Option<Self> {
        if msg.msg_flags & libc::MSG_CTRUNC == 0 {
            Some(Self {
                msg,
                cmsg: libc::CMSG_FIRSTHDR(&msg).as_ref(),
            })
        } else {
            None
        }
    }
}

impl<'a> Iterator for CmsgIter<'a> {
    type Item = Cmsg<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let cmsg = self.cmsg?;
        self.cmsg = unsafe { libc::CMSG_NXTHDR(&self.msg, cmsg).as_ref() };
        Some(match (cmsg.cmsg_level, cmsg.cmsg_type) {
            (libc::SOL_SOCKET, libc::SCM_TIMESTAMPING) => {
                Cmsg::Timestamping(unsafe { cmsg_data(cmsg) })
            }
            _ => Cmsg::Other(cmsg),
        })
    }
}

unsafe fn cmsg_data<T>(cmsg: &libc::cmsghdr) -> &T {
    assert_eq!(cmsg.cmsg_len, libc::CMSG_LEN(size_of::<T>() as _) as _);
    let data = libc::CMSG_DATA(cmsg);
    assert_eq!(data.align_offset(align_of::<T>()), 0);
    &*(data as *const T)
}
