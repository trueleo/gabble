use crate::generator::generate;
use crate::Symbol;
use rand::distributions::{Distribution, Standard};
use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct Length<const N: usize>(pub String);

impl<const N: usize> Distribution<Length<{ N }>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Length<{ N }> {
        Length(generate(rng, Symbol::Alphabet, Symbol::Alphabet, Some(N)))
    }
}

impl<const N: usize> ops::Deref for Length<{ N }> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> ops::DerefMut for Length<{ N }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> fmt::Display for Length<{ N }> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn length() {
        use crate::Length;
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();
        let gib: Length<4> = rng.gen();
        assert!(gib.len() > 0);
        println!("length {}", gib);
    }
}