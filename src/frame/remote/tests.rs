use super::*;

impl PartialEq for RemoteFrame {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[test]
fn test_remote() {
    let frame = RemoteFrame::new(Id::Standard(42));
    assert_eq!(frame.id(), Id::Standard(42));
}
