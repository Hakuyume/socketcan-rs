use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

const DLC: [u8; sys::CANFD_MAX_DLC as _] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48];

macro_rules! frame {
    ($name:ident, $flags:expr, $mask:expr) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub(super) sys::canfd_frame);

        impl $name {
            pub fn new(id: u32, flags: u8, data: &[u8]) -> Self {
                assert!(id <= $mask);
                assert!(data.len() <= sys::CANFD_MAX_DLEN as _);
                let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
                unsafe {
                    (*inner.as_mut_ptr()).can_id = id | $flags;
                    (*inner.as_mut_ptr()).len = DLC
                        .iter()
                        .copied()
                        .find(|&dlc| dlc as usize >= data.len())
                        .unwrap_or(sys::CANFD_MAX_DLEN as _);
                    (*inner.as_mut_ptr()).flags = flags;
                    (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
                    Self(inner.assume_init())
                }
            }

            pub fn id(&self) -> u32 {
                self.0.can_id & $mask
            }

            pub fn flags(&self) -> u8 {
                self.0.flags
            }

            pub fn data(&self) -> &[u8] {
                &self.0.data[..self.0.len as _]
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    concat!(
                        stringify!($name),
                        " {{ id: {:?}, flags: {:?}, data: {:?} }}"
                    ),
                    self.id(),
                    self.flags(),
                    self.data()
                )
            }
        }
    };
}
frame!(CanFdBaseFrame, 0, sys::CAN_SFF_MASK);
frame!(CanFdExtendedFrame, sys::CAN_EFF_FLAG, sys::CAN_EFF_MASK);
