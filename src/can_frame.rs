mod fd;
mod legacy;

use crate::sys;
pub use fd::*;
pub use legacy::*;
use std::mem::size_of_val;
use std::os::raw::c_void;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum CanFrame {
    Standard(CanStandardFrame),
    Extended(CanExtendedFrame),
    FdStandard(CanFdStandardFrame),
    FdExtended(CanFdExtendedFrame),
}

impl From<sys::can_frame> for CanFrame {
    fn from(inner: sys::can_frame) -> Self {
        if inner.can_id & sys::CAN_EFF_FLAG == 0 {
            Self::Standard(CanStandardFrame(inner))
        } else {
            Self::Extended(CanExtendedFrame(inner))
        }
    }
}

impl From<sys::canfd_frame> for CanFrame {
    fn from(inner: sys::canfd_frame) -> Self {
        if inner.can_id & sys::CAN_EFF_FLAG == 0 {
            Self::FdStandard(CanFdStandardFrame(inner))
        } else {
            Self::FdExtended(CanFdExtendedFrame(inner))
        }
    }
}

impl CanFrame {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        match self {
            Self::Standard(CanStandardFrame(inner)) | Self::Extended(CanExtendedFrame(inner)) => {
                inner as *const _ as _
            }
            Self::FdStandard(CanFdStandardFrame(inner))
            | Self::FdExtended(CanFdExtendedFrame(inner)) => inner as *const _ as _,
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Standard(CanStandardFrame(inner)) | Self::Extended(CanExtendedFrame(inner)) => {
                size_of_val(inner)
            }
            Self::FdStandard(CanFdStandardFrame(inner))
            | Self::FdExtended(CanFdExtendedFrame(inner)) => size_of_val(inner),
        }
    }
}
