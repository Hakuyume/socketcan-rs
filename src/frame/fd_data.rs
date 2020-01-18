use super::Id;
use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

const DLC: [u8; sys::CANFD_MAX_DLC as _] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48];

#[derive(Clone, Copy)]
pub struct FdDataFrame(pub(super) sys::canfd_frame);

impl FdDataFrame {
    /// # Panics
    ///
    /// Panics if `id` exceeds its limit or `data` is longer than 64 bytes.
    pub fn new(id: Id, brs: bool, esi: bool, data: &[u8]) -> Self {
        assert!(data.len() <= sys::CANFD_MAX_DLEN as _);
        let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = id.into_can_id();
            (*inner.as_mut_ptr()).len = DLC
                .iter()
                .copied()
                .find(|&dlc| dlc as usize >= data.len())
                .unwrap_or(sys::CANFD_MAX_DLEN as _);
            (*inner.as_mut_ptr()).flags = if brs { sys::CANFD_BRS as _ } else { 0 }
                | if esi { sys::CANFD_ESI as _ } else { 0 };
            (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
            Self(inner.assume_init())
        }
    }

    pub fn id(&self) -> Id {
        Id::from_can_id(self.0.can_id)
    }

    pub fn brs(&self) -> bool {
        self.0.flags & (sys::CANFD_BRS as u8) != 0
    }

    pub fn esi(&self) -> bool {
        self.0.flags & (sys::CANFD_ESI as u8) != 0
    }

    pub fn data(&self) -> &[u8] {
        &self.0.data[..self.0.len as _]
    }
}

impl fmt::Debug for FdDataFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("FdDataFrame")
            .field("id", &self.id())
            .field("brs", &self.brs())
            .field("esi", &self.esi())
            .field("data", &self.data())
            .finish()
    }
}

#[cfg(test)]
mod tests;
