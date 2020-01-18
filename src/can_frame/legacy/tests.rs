use super::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

macro_rules! impl_traits {
    ($name:ident) => {
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.id() == other.id() && self.data() == other.data()
            }
        }

        impl Distribution<$name> for Standard {
            fn sample<R>(&self, rng: &mut R) -> $name
            where
                R: Rng + ?Sized,
            {
                let id = rng.gen_range(0, (1 << $name::ID_BITS) - 1);
                let data = (0..rng.gen_range(0, $name::MAX_DLEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>();
                $name::new(id, &data)
            }
        }
    };
}
impl_traits!(CanStandardFrame);
impl_traits!(CanExtendedFrame);

#[test]
fn test_standard() {
    let data = rand::random::<[_; 8]>();
    let frame = CanStandardFrame::new(0x42, &data);
    assert_eq!(frame.id(), 0x42);
    assert_eq!(frame.data(), &data);
}

#[test]
#[should_panic]
fn test_standard_id_exceed() {
    CanStandardFrame::new(0x800, &[]);
}

#[test]
#[should_panic]
fn test_standard_data_exceed() {
    CanStandardFrame::new(0x42, &[0; 12]);
}

#[test]
fn test_extended() {
    let data = rand::random::<[_; 8]>();
    let frame = CanExtendedFrame::new(0x4242, &data);
    assert_eq!(frame.id(), 0x4242);
    assert_eq!(frame.data(), &data);
}

#[test]
#[should_panic]
fn test_extended_id_exceed() {
    CanExtendedFrame::new(0x20000000, &[]);
}

#[test]
#[should_panic]
fn test_extended_data_exceed() {
    CanExtendedFrame::new(0x4242, &[0; 12]);
}
