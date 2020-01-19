use super::DataFrame;
use crate::Id;

impl PartialEq for DataFrame {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.data() == other.data()
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
