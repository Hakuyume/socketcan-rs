use super::Frame;
use crate::{sys, Id};
use std::mem::MaybeUninit;

#[test]
fn test_data_standard() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 42;
        inner.assume_init()
    };
    match Frame::from_can_frame(inner) {
        Frame::Data(frame) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_data_extended() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 4242 | sys::CAN_EFF_FLAG;
        inner.assume_init()
    };
    match Frame::from_can_frame(inner) {
        Frame::Data(frame) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_fd_data_standard() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 42;
        inner.assume_init()
    };
    match Frame::from_canfd_frame(inner) {
        Frame::FdData(frame) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_fd_data_extended() {
    let mut inner = MaybeUninit::<sys::canfd_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 4242 | sys::CAN_EFF_FLAG;
        inner.assume_init()
    };
    match Frame::from_canfd_frame(inner) {
        Frame::FdData(frame) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_remote_standard() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 42 | sys::CAN_RTR_FLAG;
        inner.assume_init()
    };
    match Frame::from_can_frame(inner) {
        Frame::Remote(frame) => assert_eq!(frame.id(), Id::Standard(42)),
        _ => panic!(),
    }
}

#[test]
fn test_remote_extended() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = 4242 | sys::CAN_EFF_FLAG | sys::CAN_RTR_FLAG;
        inner.assume_init()
    };
    match Frame::from_can_frame(inner) {
        Frame::Remote(frame) => assert_eq!(frame.id(), Id::Extended(4242)),
        _ => panic!(),
    }
}

#[test]
fn test_error() {
    let mut inner = MaybeUninit::<sys::can_frame>::zeroed();
    let inner = unsafe {
        (*inner.as_mut_ptr()).can_id = sys::CAN_ERR_FLAG;
        inner.assume_init()
    };
    match Frame::from_can_frame(inner) {
        Frame::Error(_) => (),
        _ => panic!(),
    }
}
