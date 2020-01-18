use super::*;
use crate::sys;
use std::mem::MaybeUninit;

#[test]
fn test_standard() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 0x42;
        inner.assume_init()
    };
    match inner.into() {
        Frame::Standard(frame) => assert_eq!(frame.id(), 0x42),
        _ => panic!(),
    }
}

#[test]
fn test_extended() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 0x4242 | sys::CAN_EFF_FLAG;
        inner.assume_init()
    };
    match inner.into() {
        Frame::Extended(frame) => assert_eq!(frame.id(), 0x4242),
        _ => panic!(),
    }
}

#[test]
fn test_fd_standard() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 0x42;
        inner.assume_init()
    };
    match inner.into() {
        Frame::FdStandard(frame) => assert_eq!(frame.id(), 0x42),
        _ => panic!(),
    }
}

#[test]
fn test_fd_extended() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 0x4242 | sys::CAN_EFF_FLAG;
        inner.assume_init()
    };
    match inner.into() {
        Frame::FdExtended(frame) => assert_eq!(frame.id(), 0x4242),
        _ => panic!(),
    }
}

#[test]
#[should_panic]
fn test_remote() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_RTR_FLAG;
        inner.assume_init()
    };
    Frame::from(inner);
}

#[test]
#[should_panic]
fn test_error() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_ERR_FLAG;
        inner.assume_init()
    };
    Frame::from(inner);
}

#[test]
#[should_panic]
fn test_fd_remote() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_RTR_FLAG;
        inner.assume_init()
    };
    Frame::from(inner);
}

#[test]
#[should_panic]
fn test_fd_error() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_ERR_FLAG;
        inner.assume_init()
    };
    Frame::from(inner);
}
