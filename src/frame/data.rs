use super::Id;
use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub struct DataFrame(pub(super) sys::can_frame);

impl DataFrame {
    /// # Panics
    ///
    /// Panics if `id` exceeds its limit or `data` is longer than 8 bytes.
    pub fn new(id: Id, data: &[u8]) -> Self {
        assert!(data.len() <= sys::CAN_MAX_DLEN as _);
        let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = id.into_can_id();
            *sys::can_frame_len_mut(inner.as_mut_ptr()) = data.len() as _;
            (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
            Self(inner.assume_init())
        }
    }

    pub fn id(&self) -> Id {
        Id::from_can_id(self.0.can_id)
    }

    pub fn data(&self) -> &[u8] {
        // SAFETY: call is safe, because &self.0 points to a valid reference
        &self.0.data[..unsafe { sys::can_frame_len(&self.0) } as _]
    }
}

impl fmt::Debug for DataFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("DataFrame")
            .field("id", &self.id())
            .field("data", &self.data())
            .finish()
    }
}

#[cfg(test)]
mod tests;
