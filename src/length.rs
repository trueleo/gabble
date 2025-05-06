use crate::generator::generate;
use crate::Syllable;
use rand::distr::{Distribution, StandardUniform};
use std::fmt;
use std::ops;

/// Pseudo-word of some specified minimum length N
///
/// Wrapper type around [`String`] which implements [`Distribution`](rand::distributions::Distribution)
#[derive(Debug)]
pub struct GabLength<const N: usize>(pub String);

impl<const N: usize> Distribution<GabLength<{ N }>> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GabLength<{ N }> {
        GabLength(generate(
            rng,
            Syllable::Alphabet,
            Syllable::Alphabet,
            Some(N),
        ))
    }
}

impl<const N: usize> ops::Deref for GabLength<{ N }> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> ops::DerefMut for GabLength<{ N }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> fmt::Display for GabLength<{ N }> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn gablength() {
        use crate::GabLength;
        use rand::rng;
        use rand::Rng;
        let mut rng = rng();
        let gib: GabLength<4> = rng.random();
        assert!(gib.len() > 0);
        println!("length {}", gib);
    }
}