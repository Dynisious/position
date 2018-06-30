//! Provides implementations for generating random `Pos` values using the
//! [`rand`](https://crates.io/crates/rand) crate.
//! 
//! Gated by the `pos-rand` feature.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/06/30

#![cfg(feature = "pos-rand")]

use super::*;
use ext_rand::distributions::{self, Distribution,};

impl<T> Pos<T>
    where Standard: Distribution<Pos<T>> {
    #[inline]
    pub fn random() -> Self {
        use ext_rand::Rng;

        ext_rand::thread_rng().sample(Standard)
    }
}

/// A [`Distribution`](https://docs.rs/rand/0.5.3/rand/distributions/trait.Distribution.html)
/// for generating `Pos<T>` values where there is an implementation of
/// [`Standard`](https://docs.rs/rand/0.5.3/rand/distributions/struct.Standard.html) for `T`.
pub struct Standard;

impl<T> Distribution<Pos<T>> for Standard
    where distributions::Standard: Distribution<T> {
    fn sample<R: ::ext_rand::Rng + ?Sized>(&self, rng: &mut R) -> Pos<T> {
        Pos::new(rng.gen(), rng.gen())
    }
}

/// A [`Distribution`](https://docs.rs/rand/0.5.3/rand/distributions/trait.Distribution.html)
/// for generating `Pos<T>` values where there is an implementation of
/// [`StandardNormal`](https://docs.rs/rand/0.5.3/rand/distributions/struct.StandardNormal.html) for `T`.
pub struct StandardNormal;

impl<T> Distribution<Pos<T>> for StandardNormal
    where distributions::StandardNormal: Distribution<T> {
    fn sample<R: ::ext_rand::Rng + ?Sized>(&self, rng: &mut R) -> Pos<T> {
        Pos::new(
            rng.sample(distributions::StandardNormal),
            rng.sample(distributions::StandardNormal)
        )
    }
}
