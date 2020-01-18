use super::*;
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
        let data = (0..rng.gen_range(0, DataFrame::MAX_DLEN))
            .map(|_| rng.gen())
            .collect::<Vec<_>>();
        DataFrame::new(id, &data)
    }
}

#[test]
fn test_standard() {
    let data = rand::random::<[_; 8]>();
    let frame = DataFrame::new(Id::Standard(0x42), &data);
    assert_eq!(frame.id(), Id::Standard(0x42));
    assert_eq!(frame.data(), &data);
}

#[test]
#[should_panic]
fn test_standard_id_exceed() {
    DataFrame::new(Id::Standard(0x800), &[]);
}

#[test]
#[should_panic]
fn test_standard_data_exceed() {
    DataFrame::new(Id::Standard(0x42), &[0; 12]);
}

#[test]
fn test_extended() {
    let data = rand::random::<[_; 8]>();
    let frame = DataFrame::new(Id::Extended(0x4242), &data);
    assert_eq!(frame.id(), Id::Extended(0x4242));
    assert_eq!(frame.data(), &data);
}

#[test]
#[should_panic]
fn test_extended_id_exceed() {
    DataFrame::new(Id::Extended(0x2000_0000), &[]);
}

#[test]
#[should_panic]
fn test_extended_data_exceed() {
    DataFrame::new(Id::Extended(0x4242), &[0; 12]);
}
