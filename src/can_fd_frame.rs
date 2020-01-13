use crate::linux_can::canfd_frame;
use std::mem::MaybeUninit;

#[repr(C)]
pub struct CanFdFrame(canfd_frame);

impl CanFdFrame {
    pub fn new(can_id: u32, flags: u8, data: &[u8]) -> Option<Self> {
        let mut inner = MaybeUninit::<canfd_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = can_id;
            (*inner.as_mut_ptr()).flags = flags;
            (*inner.as_mut_ptr()).len = match data.len() {
                len @ 0..=8 => len as _,
                9..=12 => 12,
                13..=16 => 16,
                17..=20 => 20,
                21..=24 => 24,
                25..=32 => 32,
                33..=48 => 48,
                49..=64 => 64,
                _ => return None,
            };
            (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
            Some(Self(inner.assume_init()))
        }
    }

    pub fn can_id(&self) -> u32 {
        self.0.can_id
    }

    pub fn flags(&self) -> u8 {
        self.0.flags
    }

    pub fn data(&self) -> &[u8] {
        &self.0.data[..self.0.len as _]
    }
}
