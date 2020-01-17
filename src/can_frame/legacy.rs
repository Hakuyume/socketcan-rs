use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

macro_rules! frame {
    ($name:ident, $flags:expr, $mask:expr) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub(super) sys::can_frame);

        impl $name {
            pub fn new(id: u32, data: &[u8]) -> Self {
                assert!(id <= $mask);
                assert!(data.len() <= sys::CAN_MAX_DLEN as _);
                let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
                unsafe {
                    (*inner.as_mut_ptr()).can_id = id | $flags;
                    (*inner.as_mut_ptr()).can_dlc = data.len() as _;
                    (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
                    Self(inner.assume_init())
                }
            }

            pub fn id(&self) -> u32 {
                self.0.can_id & $mask
            }

            pub fn data(&self) -> &[u8] {
                &self.0.data[..self.0.can_dlc as _]
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    concat!(stringify!($name), " {{ id: {:?}, data: {:?} }}"),
                    self.id(),
                    self.data()
                )
            }
        }
    };
}
frame!(CanBaseFrame, 0, sys::CAN_SFF_MASK);
frame!(CanExtendedFrame, sys::CAN_EFF_FLAG, sys::CAN_EFF_MASK);
