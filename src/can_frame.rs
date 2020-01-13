use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub struct CanFrame(pub(crate) sys::can_frame);

impl CanFrame {
    pub fn new(can_id: u32, data: &[u8]) -> Self {
        assert!(data.len() <= sys::CAN_MAX_DLEN as _);
        let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
        unsafe {
            (*inner.as_mut_ptr()).can_id = can_id;
            (*inner.as_mut_ptr()).can_dlc = data.len() as _;
            (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
            Self(inner.assume_init())
        }
    }

    pub fn can_id(&self) -> u32 {
        self.0.can_id
    }

    pub fn data(&self) -> &[u8] {
        &self.0.data[..self.0.can_dlc as _]
    }
}

impl fmt::Debug for CanFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CanFrame {{ can_id: {:?}, data: {:?} }}",
            self.can_id(),
            self.data()
        )
    }
}
