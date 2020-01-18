mod data;
mod fd_data;
mod id;
mod remote;

use crate::sys;
pub use data::DataFrame;
pub use fd_data::FdDataFrame;
pub use id::Id;
pub use remote::RemoteFrame;
use std::mem::size_of_val;
use std::os::raw::c_void;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[cfg_attr(test, derive(PartialEq))]
pub enum Frame {
    Data(DataFrame),
    FdData(FdDataFrame),
    Remote(RemoteFrame),
}

impl Frame {
    pub(crate) fn from_can_frame(inner: sys::can_frame) -> Self {
        assert_eq!(inner.can_id & sys::CAN_ERR_FLAG, 0);
        if inner.can_id & sys::CAN_RTR_FLAG != 0 {
            Self::Remote(RemoteFrame(inner))
        } else {
            Self::Data(DataFrame(inner))
        }
    }

    pub(crate) fn from_canfd_frame(inner: sys::canfd_frame) -> Self {
        assert_eq!(inner.can_id & (sys::CAN_RTR_FLAG | sys::CAN_ERR_FLAG), 0);
        Self::FdData(FdDataFrame(inner))
    }

    pub(crate) fn as_ptr(&self) -> *const c_void {
        match self {
            Self::Data(DataFrame(inner)) | Self::Remote(RemoteFrame(inner)) => {
                inner as *const _ as _
            }
            Self::FdData(FdDataFrame(inner)) => inner as *const _ as _,
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Data(DataFrame(inner)) | Self::Remote(RemoteFrame(inner)) => size_of_val(inner),
            Self::FdData(FdDataFrame(inner)) => size_of_val(inner),
        }
    }
}

#[cfg(test)]
mod tests;
