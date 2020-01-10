use crate::linux_can;
use std::mem;

pub type CanFdFrame = linux_can::canfd_frame;

impl CanFdFrame {
    pub fn new(can_id: u32, flags: u8, data: &[u8]) -> Option<Self> {
        let mut frame = unsafe { mem::MaybeUninit::<Self>::zeroed().assume_init() };
        frame.can_id = can_id;
        frame.flags = flags;
        let len = match data.len() {
            len @ 0..=8 => len,
            9..=12 => 12,
            13..=16 => 16,
            17..=20 => 20,
            21..=24 => 24,
            25..=32 => 32,
            33..=48 => 48,
            49..=64 => 64,
            _ => return None,
        };
        frame.len = len as _;
        frame.data[..data.len()].copy_from_slice(data);
        Some(frame)
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.len as _]
    }
}
