use crate::generator::generate;
use crate::Syllable;
use rand::distributions::{Distribution, Standard};
use std::fmt;
use std::ops;

/// Pseudo-word of moderate length ( 6 to 15 chars )
///
/// Wrapper type around [`String`] which implements [`Distribution`](rand::distributions::Distribution)
#[derive(Debug)]
pub struct Gab(pub String);

impl Distribution<Gab> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Gab {
        Gab(generate(rng, Syllable::Alphabet, Syllable::Consonant, None))
    }
}

impl ops::Deref for Gab {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Gab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Gab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn default() {
        use crate::Gab;
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();
        let gib: Gab = rng.gen();
        assert!(gib.len() > 0);
        println!("gab {}", gib);
    }
}
