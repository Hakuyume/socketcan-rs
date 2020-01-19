use super::Id;
use crate::sys;

#[test]
fn test_standard() {
    assert_eq!(Id::Standard(42).into_can_id(), 42);
}

#[test]
#[should_panic]
fn test_standard_exceed() {
    Id::Standard(0x800).into_can_id();
}

#[test]
fn test_extended() {
    assert_eq!(Id::Extended(4242).into_can_id(), 4242 | sys::CAN_EFF_FLAG);
}

#[test]
#[should_panic]
fn test_extended_exceed() {
    Id::Extended(0x2000_0000).into_can_id();
}
