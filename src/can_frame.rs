mod fd;
mod legacy;

use crate::sys;
pub use fd::*;
pub use legacy::*;
use std::mem::size_of_val;
use std::os::raw::c_void;

#[derive(Clone, Copy, Debug)]
pub enum CanFrame {
    Base(CanBaseFrame),
    Extended(CanExtendedFrame),
    FdBase(CanFdBaseFrame),
    FdExtended(CanFdExtendedFrame),
}

impl From<sys::can_frame> for CanFrame {
    fn from(frame: sys::can_frame) -> Self {
        if frame.can_id & sys::CAN_EFF_FLAG == 0 {
            Self::Base(CanBaseFrame(frame))
        } else {
            Self::Extended(CanExtendedFrame(frame))
        }
    }
}

impl From<sys::canfd_frame> for CanFrame {
    fn from(frame: sys::canfd_frame) -> Self {
        if frame.can_id & sys::CAN_EFF_FLAG == 0 {
            Self::FdBase(CanFdBaseFrame(frame))
        } else {
            Self::FdExtended(CanFdExtendedFrame(frame))
        }
    }
}

impl CanFrame {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        match self {
            Self::Base(CanBaseFrame(frame)) | Self::Extended(CanExtendedFrame(frame)) => {
                frame as *const _ as _
            }
            Self::FdBase(CanFdBaseFrame(frame)) | Self::FdExtended(CanFdExtendedFrame(frame)) => {
                frame as *const _ as _
            }
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Base(CanBaseFrame(frame)) | Self::Extended(CanExtendedFrame(frame)) => {
                size_of_val(frame)
            }
            Self::FdBase(CanFdBaseFrame(frame)) | Self::FdExtended(CanFdExtendedFrame(frame)) => {
                size_of_val(frame)
            }
        }
    }
}
