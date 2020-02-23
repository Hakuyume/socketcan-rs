use super::{Frame, Inner};
use crate::{sys, Id};
use std::mem::{size_of, MaybeUninit};

#[test]
fn test_data_standard() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).can.can_id = 42;
        Frame::from_inner(inner, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Data(frame)) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_data_extended() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).can.can_id = 4242 | sys::CAN_EFF_FLAG;
        Frame::from_inner(inner, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Data(frame)) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_fd_data_standard() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).canfd.can_id = 42;
        Frame::from_inner(inner, size_of::<sys::canfd_frame>())
    };
    match frame {
        Some(Frame::FdData(frame)) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_fd_data_extended() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).canfd.can_id = 4242 | sys::CAN_EFF_FLAG;
        Frame::from_inner(inner, size_of::<sys::canfd_frame>())
    };
    match frame {
        Some(Frame::FdData(frame)) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_remote_standard() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).can.can_id = 42 | sys::CAN_RTR_FLAG;
        Frame::from_inner(inner, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Remote(frame)) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_remote_extended() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).can.can_id = 4242 | sys::CAN_EFF_FLAG | sys::CAN_RTR_FLAG;
        Frame::from_inner(inner, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Remote(frame)) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_error() {
    let mut inner = MaybeUninit::<Inner>::zeroed();
    let frame = unsafe {
        (*inner.as_mut_ptr()).can.can_id = sys::CAN_ERR_FLAG;
        Frame::from_inner(inner, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Error(_)) => (),
        _ => panic!(),
    }
}
