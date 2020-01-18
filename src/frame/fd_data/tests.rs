use super::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

impl PartialEq for FdDataFrame {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.data() == other.data()
    }
}

impl Distribution<FdDataFrame> for Standard {
    fn sample<R>(&self, rng: &mut R) -> FdDataFrame
    where
        R: Rng + ?Sized,
    {
        let id = rng.gen();
        let data = (0..rng.gen_range(0, FdDataFrame::MAX_DLEN))
            .map(|_| rng.gen())
            .collect::<Vec<_>>();
        FdDataFrame::new(id, false, false, &data)
    }
}

#[test]
fn test_fd_standard() {
    for &brs in &[false, true] {
        for &esi in &[false, true] {
            let data = rand::random::<[_; 8]>();
            let frame = FdDataFrame::new(Id::Standard(0x42), brs, esi, &data);
            assert_eq!(frame.id(), Id::Standard(0x42));
            assert_eq!(frame.brs(), brs);
            assert_eq!(frame.esi(), esi);
            assert_eq!(frame.data(), &data);
        }
    }
}

#[test]
fn test_fd_standard_data_padded() {
    let data = rand::random::<[_; 17]>();
    let frame = FdDataFrame::new(Id::Standard(0x42), false, false, &data);
    assert_eq!(frame.data().len(), 20);
    assert_eq!(&frame.data()[..17], &data);
}

#[test]
#[should_panic]
fn test_fd_standard_id_exceed() {
    FdDataFrame::new(Id::Standard(0x800), false, false, &[]);
}

#[test]
#[should_panic]
fn test_fd_standard_data_exceed() {
    FdDataFrame::new(Id::Standard(0x42), false, false, &[0; 72]);
}

#[test]
fn test_fd_extended() {
    for &brs in &[false, true] {
        for &esi in &[false, true] {
            let data = rand::random::<[_; 8]>();
            let frame = FdDataFrame::new(Id::Extended(0x4242), brs, esi, &data);
            assert_eq!(frame.id(), Id::Extended(0x4242));
            assert_eq!(frame.brs(), brs);
            assert_eq!(frame.esi(), esi);
            assert_eq!(frame.data(), &data);
        }
    }
}

#[test]
fn test_fd_extended_data_padded() {
    let data = rand::random::<[_; 17]>();
    let frame = FdDataFrame::new(Id::Extended(0x4242), false, false, &data);
    assert_eq!(frame.data().len(), 20);
    assert_eq!(&frame.data()[..17], &data);
}

#[test]
#[should_panic]
fn test_fd_extended_id_exceed() {
    FdDataFrame::new(Id::Extended(0x2000_0000), false, false, &[]);
}

#[test]
#[should_panic]
fn test_fd_extended_data_exceed() {
    FdDataFrame::new(Id::Extended(0x4242), false, false, &[0; 72]);
}
