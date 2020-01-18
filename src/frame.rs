mod data;
mod fd_data;
mod id;

use crate::sys;
pub use data::DataFrame;
pub use fd_data::FdDataFrame;
pub use id::Id;
use std::mem::size_of_val;
use std::os::raw::c_void;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[cfg_attr(test, derive(PartialEq))]
pub enum Frame {
    Data(DataFrame),
    FdData(FdDataFrame),
}

impl Frame {
    pub(crate) fn from_can_frame(inner: sys::can_frame) -> Self {
        assert_eq!(inner.can_id & (sys::CAN_RTR_FLAG | sys::CAN_ERR_FLAG), 0);
        Self::Data(DataFrame(inner))
    }

    pub(crate) fn from_canfd_frame(inner: sys::canfd_frame) -> Self {
        assert_eq!(inner.can_id & (sys::CAN_RTR_FLAG | sys::CAN_ERR_FLAG), 0);
        Self::FdData(FdDataFrame(inner))
    }

    pub(crate) fn as_ptr(&self) -> *const c_void {
        match self {
            Self::Data(DataFrame(inner)) => inner as *const _ as _,
            Self::FdData(FdDataFrame(inner)) => inner as *const _ as _,
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Data(DataFrame(inner)) => size_of_val(inner),
            Self::FdData(FdDataFrame(inner)) => size_of_val(inner),
        }
    }
}

#[cfg(test)]
mod tests;
