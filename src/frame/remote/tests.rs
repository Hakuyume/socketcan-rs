use super::RemoteFrame;
use crate::Id;

impl PartialEq for RemoteFrame {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.dlc() == other.dlc()
    }
}

#[test]
fn test_remote() {
    let frame = RemoteFrame::new(Id::Standard(42), 3);
    assert_eq!(frame.id(), Id::Standard(42));
    assert_eq!(frame.dlc(), 3);
}

#[test]
#[should_panic]
fn test_remote_exceed() {
    RemoteFrame::new(Id::Standard(42), 12);
}
