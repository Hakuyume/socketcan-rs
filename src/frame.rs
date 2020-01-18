mod fd;
mod legacy;

use crate::sys;
pub use fd::*;
pub use legacy::*;
use std::mem::size_of_val;
use std::os::raw::c_void;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[cfg_attr(test, derive(PartialEq))]
pub enum Frame {
    Standard(StandardFrame),
    Extended(ExtendedFrame),
    FdStandard(FdStandardFrame),
    FdExtended(FdExtendedFrame),
}

impl From<sys::can_frame> for Frame {
    fn from(inner: sys::can_frame) -> Self {
        assert_eq!(inner.can_id & (sys::CAN_RTR_FLAG | sys::CAN_ERR_FLAG), 0);
        if inner.can_id & sys::CAN_EFF_FLAG == 0 {
            Self::Standard(StandardFrame(inner))
        } else {
            Self::Extended(ExtendedFrame(inner))
        }
    }
}

impl From<sys::canfd_frame> for Frame {
    fn from(inner: sys::canfd_frame) -> Self {
        assert_eq!(inner.can_id & (sys::CAN_RTR_FLAG | sys::CAN_ERR_FLAG), 0);
        if inner.can_id & sys::CAN_EFF_FLAG == 0 {
            Self::FdStandard(FdStandardFrame(inner))
        } else {
            Self::FdExtended(FdExtendedFrame(inner))
        }
    }
}

impl Frame {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        match self {
            Self::Standard(StandardFrame(inner)) | Self::Extended(ExtendedFrame(inner)) => {
                inner as *const _ as _
            }
            Self::FdStandard(FdStandardFrame(inner)) | Self::FdExtended(FdExtendedFrame(inner)) => {
                inner as *const _ as _
            }
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Standard(StandardFrame(inner)) | Self::Extended(ExtendedFrame(inner)) => {
                size_of_val(inner)
            }
            Self::FdStandard(FdStandardFrame(inner)) | Self::FdExtended(FdExtendedFrame(inner)) => {
                size_of_val(inner)
            }
        }
    }
}

#[cfg(test)]
mod tests;
