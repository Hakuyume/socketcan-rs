use super::Id;
use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub struct RemoteFrame(pub(super) sys::can_frame);

impl RemoteFrame {
    /// # Panics
    ///
    /// Panics if `id` exceeds its limit.
    pub fn new(id: Id) -> Self {
        let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = id.into_can_id();
            Self(inner.assume_init())
        }
    }

    pub fn id(&self) -> Id {
        Id::from_can_id(self.0.can_id)
    }
}

impl fmt::Debug for RemoteFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RemoteFrame")
            .field("id", &self.id())
            .finish()
    }
}

#[cfg(test)]
mod tests;
