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
    match Frame::from_can_frame(inner) {
        Frame::Data(frame) => assert_eq!(frame.id(), Id::Standard(0x42)),
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
    match Frame::from_can_frame(inner) {
        Frame::Data(frame) => assert_eq!(frame.id(), Id::Extended(0x4242)),
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
    match Frame::from_canfd_frame(inner) {
        Frame::FdData(frame) => assert_eq!(frame.id(), Id::Standard(0x42)),
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
    match Frame::from_canfd_frame(inner) {
        Frame::FdData(frame) => assert_eq!(frame.id(), Id::Extended(0x4242)),
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
    Frame::from_can_frame(inner);
}

#[test]
#[should_panic]
fn test_error() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_ERR_FLAG;
        inner.assume_init()
    };
    Frame::from_can_frame(inner);
}

#[test]
#[should_panic]
fn test_fd_remote() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_RTR_FLAG;
        inner.assume_init()
    };
    Frame::from_canfd_frame(inner);
}

#[test]
#[should_panic]
fn test_fd_error() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_ERR_FLAG;
        inner.assume_init()
    };
    Frame::from_canfd_frame(inner);
}