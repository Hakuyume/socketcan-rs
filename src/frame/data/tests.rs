use super::DataFrame;
use crate::{sys, Id};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

impl PartialEq for DataFrame {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.data() == other.data()
    }
}

impl Distribution<DataFrame> for Standard {
    fn sample<R>(&self, rng: &mut R) -> DataFrame
    where
        R: Rng + ?Sized,
    {
        let id = rng.gen();
        let data = (0..rng.gen_range(0, sys::CAN_MAX_DLEN))
            .map(|_| rng.gen())
            .collect::<Vec<_>>();
        DataFrame::new(id, &data)
    }
}

#[test]
fn test_data() {
    let data = rand::random::<[_; 8]>();
    let frame = DataFrame::new(Id::Standard(42), &data);
    assert_eq!(frame.id(), Id::Standard(42));
    assert_eq!(frame.data(), &data);
}

#[test]
#[should_panic]
fn test_data_exceed() {
    DataFrame::new(Id::Standard(42), &[0; 12]);
}
