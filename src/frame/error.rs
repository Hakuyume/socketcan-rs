use crate::sys;
use std::fmt;

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ErrorFrame(pub(super) sys::can_frame);

impl fmt::Debug for ErrorFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ErrorFrame").finish()
    }
}

#[cfg(test)]
mod tests;
