use crate::{CanFdFrame, CanFrame};

#[derive(Clone, Copy, Debug)]
pub enum Frame {
    Can(CanFrame),
    CanFd(CanFdFrame),
}
