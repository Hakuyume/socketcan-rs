use super::Id;
use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub struct RemoteFrame(pub(super) sys::can_frame);

impl RemoteFrame {
    /// # Panics
    ///
    /// Panics if `id` exceeds its limit or `len` is greater than 8.
    pub fn new(id: Id, len: u8) -> Self {
        assert!(len <= sys::CAN_MAX_DLEN as _);
        let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = id.into_can_id();
            (&mut *inner.as_mut_ptr()).set_len(len as _);
            Self(inner.assume_init())
        }
    }

    pub fn id(&self) -> Id {
        Id::from_can_id(self.0.can_id)
    }

    pub fn len(&self) -> u8 {
        self.0.len()
    }
}

impl fmt::Debug for RemoteFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RemoteFrame")
            .field("id", &self.id())
            .field("len", &self.len())
            .finish()
    }
}

#[cfg(test)]
mod tests;
