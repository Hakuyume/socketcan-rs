use super::Id;
use crate::sys;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

impl Distribution<Id> for Standard {
    fn sample<R>(&self, rng: &mut R) -> Id
    where
        R: Rng + ?Sized,
    {
        if rng.gen() {
            Id::Standard(rng.gen_range(0, sys::CAN_SFF_MASK))
        } else {
            Id::Extended(rng.gen_range(0, sys::CAN_EFF_MASK))
        }
    }
}
