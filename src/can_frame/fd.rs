use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

const DLC: [u8; sys::CANFD_MAX_DLC as _] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48];

macro_rules! frame {
    ($name:ident, $flags:expr, $bits:expr) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub(super) sys::canfd_frame);

        impl $name {
            pub const ID_BITS: u32 = $bits;
            pub const MAX_DLEN: usize = sys::CANFD_MAX_DLEN as _;

            /// # Panics
            ///
            /// Panics if `id` is more than `ID_BITS` bits or `data` is longer than `MAX_DLEN` bytes.
            pub fn new(id: u32, brs: bool, esi: bool, data: &[u8]) -> Self {
                assert!(id < 1 << Self::ID_BITS);
                assert!(data.len() <= Self::MAX_DLEN);
                let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
                unsafe {
                    (*inner.as_mut_ptr()).can_id = id | $flags;
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

            pub fn id(&self) -> u32 {
                self.0.can_id & (1 << Self::ID_BITS) - 1
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

        impl fmt::Debug for $name {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct(stringify!($name))
                    .field("id", &self.id())
                    .field("brs", &self.brs())
                    .field("esi", &self.esi())
                    .field("data", &self.data())
                    .finish()
            }
        }
    };
}
frame!(CanFdStandardFrame, 0, sys::CAN_SFF_ID_BITS);
frame!(CanFdExtendedFrame, sys::CAN_EFF_FLAG, sys::CAN_EFF_ID_BITS);

#[cfg(test)]
mod tests;
