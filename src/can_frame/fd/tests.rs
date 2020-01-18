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
