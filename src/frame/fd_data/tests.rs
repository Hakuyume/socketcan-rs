use super::FdDataFrame;
use crate::Id;

impl PartialEq for FdDataFrame {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.data() == other.data()
    }
}

#[test]
fn test_fd_data() {
    for &brs in &[false, true] {
        for &esi in &[false, true] {
            let data = rand::random::<[_; 8]>();
            let frame = FdDataFrame::new(Id::Standard(42), brs, esi, &data);
            assert_eq!(frame.id(), Id::Standard(42));
            assert_eq!(frame.brs(), brs);
            assert_eq!(frame.esi(), esi);
            assert_eq!(frame.data(), &data);
        }
    }
}

#[test]
fn test_fd_data_padded() {
    let data = rand::random::<[_; 17]>();
    let frame = FdDataFrame::new(Id::Standard(0x42), false, false, &data);
    assert_eq!(frame.data().len(), 20);
    assert_eq!(&frame.data()[..17], &data);
}

#[test]
#[should_panic]
fn test_fd_data_exceed() {
    FdDataFrame::new(Id::Standard(0x42), false, false, &[0; 72]);
}
