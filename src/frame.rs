use crate::{CanFdFrame, CanFrame};

pub enum Frame {
    Can(CanFrame),
    CanFd(CanFdFrame),
}
