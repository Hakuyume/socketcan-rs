use super::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

macro_rules! impl_distribution {
    ($name:ident) => {
        impl Distribution<$name> for Standard {
            fn sample<R>(&self, rng: &mut R) -> $name
            where
                R: Rng + ?Sized,
            {
                let id = rng.gen::<u32>() & (1 << $name::ID_BITS) - 1;
                let data = (0..rng.gen_range(0, $name::MAX_DLEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>();
                $name::new(id, false, false, &data)
            }
        }
    };
}
impl_distribution!(CanFdStandardFrame);
impl_distribution!(CanFdExtendedFrame);

#[test]
fn test_fd_standard() {
    for &brs in &[false, true] {
        for &esi in &[false, true] {
            let data = rand::random::<[_; 8]>();
            let frame = CanFdStandardFrame::new(0x42, brs, esi, &data);
            assert_eq!(frame.id(), 0x42);
            assert_eq!(frame.brs(), brs);
            assert_eq!(frame.esi(), esi);
            assert_eq!(frame.data(), &data);
        }
    }
}

#[test]
fn test_fd_standard_data_padded() {
    let data = rand::random::<[_; 17]>();
    let frame = CanFdStandardFrame::new(0x42, false, false, &data);
    assert_eq!(frame.data().len(), 20);
    assert_eq!(&frame.data()[..17], &data);
}

#[test]
#[should_panic]
fn test_fd_standard_id_exceed() {
    CanFdStandardFrame::new(0x800, false, false, &[]);
}

#[test]
#[should_panic]
fn test_fd_standard_data_exceed() {
    CanFdStandardFrame::new(0x42, false, false, &[0; 72]);
}

#[test]
fn test_fd_extended() {
    for &brs in &[false, true] {
        for &esi in &[false, true] {
            let data = rand::random::<[_; 8]>();
            let frame = CanFdExtendedFrame::new(0x4242, brs, esi, &data);
            assert_eq!(frame.id(), 0x4242);
            assert_eq!(frame.brs(), brs);
            assert_eq!(frame.esi(), esi);
            assert_eq!(frame.data(), &data);
        }
    }
}

#[test]
fn test_fd_extended_data_padded() {
    let data = rand::random::<[_; 17]>();
    let frame = CanFdExtendedFrame::new(0x4242, false, false, &data);
    assert_eq!(frame.data().len(), 20);
    assert_eq!(&frame.data()[..17], &data);
}

#[test]
#[should_panic]
fn test_fd_extended_id_exceed() {
    CanFdExtendedFrame::new(0x20000000, false, false, &[]);
}

#[test]
#[should_panic]
fn test_fd_extended_data_exceed() {
    CanFdExtendedFrame::new(0x4242, false, false, &[0; 72]);
}
