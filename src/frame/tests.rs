use super::Frame;
use crate::{sys, Id};
use std::mem::{align_of, size_of, MaybeUninit};

#[test]
fn test_layout() {
    assert_eq!(align_of::<sys::can_frame>(), align_of::<sys::canfd_frame>());
}

#[test]
fn test_data_standard() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = 42;
        Frame::from_raw(frame, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Data(frame)) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_data_extended() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = 4242 | sys::CAN_EFF_FLAG;
        Frame::from_raw(frame, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Data(frame)) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_fd_data_standard() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = 42;
        Frame::from_raw(frame, size_of::<sys::canfd_frame>())
    };
    match frame {
        Some(Frame::FdData(frame)) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_fd_data_extended() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = 4242 | sys::CAN_EFF_FLAG;
        Frame::from_raw(frame, size_of::<sys::canfd_frame>())
    };
    match frame {
        Some(Frame::FdData(frame)) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_remote_standard() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = 42 | sys::CAN_RTR_FLAG;
        Frame::from_raw(frame, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Remote(frame)) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_remote_extended() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = 4242 | sys::CAN_EFF_FLAG | sys::CAN_RTR_FLAG;
        Frame::from_raw(frame, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Remote(frame)) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_error() {
    let mut frame = MaybeUninit::<sys::canfd_frame>::zeroed();
    let frame = unsafe {
        (*frame.as_mut_ptr()).can_id = sys::CAN_ERR_FLAG;
        Frame::from_raw(frame, size_of::<sys::can_frame>())
    };
    match frame {
        Some(Frame::Error(_)) => (),
        _ => panic!(),
    }
}
