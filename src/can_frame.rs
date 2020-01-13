use crate::sys;
use std::mem::MaybeUninit;

pub struct CanFrame(pub(crate) sys::can_frame);

impl CanFrame {
    pub fn new(can_id: u32, data: &[u8]) -> Option<Self> {
        let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = can_id;
            (*inner.as_mut_ptr()).can_dlc = match data.len() {
                len @ 0..=8 => len as _,
                _ => return None,
            };
            (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
            Some(Self(inner.assume_init()))
        }
    }

    pub fn can_id(&self) -> u32 {
        self.0.can_id
    }

    pub fn data(&self) -> &[u8] {
        &self.0.data[..self.0.can_dlc as _]
    }
}
