use crate::sys;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Id {
    /// 11 bits identifier
    Standard(u32),
    /// 29 bits identifier
    Extended(u32),
}

impl Id {
    pub(crate) fn from_can_id(can_id: u32) -> Self {
        if can_id & sys::CAN_EFF_FLAG == 0 {
            Id::Standard(can_id & sys::CAN_SFF_MASK)
        } else {
            Id::Extended(can_id & sys::CAN_EFF_MASK)
        }
    }

    pub(crate) fn into_can_id(self) -> u32 {
        match self {
            Self::Standard(id) => {
                assert!(id <= sys::CAN_SFF_MASK);
                id
            }
            Self::Extended(id) => {
                assert!(id <= sys::CAN_EFF_MASK);
                id | sys::CAN_EFF_FLAG
            }
        }
    }
}

#[cfg(test)]
mod tests;
