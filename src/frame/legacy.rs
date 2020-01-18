use crate::sys;
use std::fmt;
use std::mem::MaybeUninit;

macro_rules! frame {
    ($name:ident, $flags:expr, $bits:expr) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub(super) sys::can_frame);

        impl $name {
            pub const ID_BITS: u32 = $bits;
            pub const MAX_DLEN: usize = sys::CAN_MAX_DLEN as _;

            /// # Panics
            ///
            /// Panics if `id` is more than `ID_BITS` bits or `data` is longer than `MAX_DLEN` bytes.
            pub fn new(id: u32, data: &[u8]) -> Self {
                assert!(id < 1 << Self::ID_BITS);
                assert!(data.len() <= Self::MAX_DLEN);
                let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
                unsafe {
                    (*inner.as_mut_ptr()).can_id = id | $flags;
                    (*inner.as_mut_ptr()).can_dlc = data.len() as _;
                    (*inner.as_mut_ptr()).data[..data.len()].copy_from_slice(data);
                    Self(inner.assume_init())
                }
            }

            pub fn id(&self) -> u32 {
                self.0.can_id & (1 << Self::ID_BITS) - 1
            }

            pub fn data(&self) -> &[u8] {
                &self.0.data[..self.0.can_dlc as _]
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct(stringify!($name))
                    .field("id", &self.id())
                    .field("data", &self.data())
                    .finish()
            }
        }
    };
}
frame!(StandardFrame, 0, sys::CAN_SFF_ID_BITS);
frame!(ExtendedFrame, sys::CAN_EFF_FLAG, sys::CAN_EFF_ID_BITS);

#[cfg(test)]
mod tests;
