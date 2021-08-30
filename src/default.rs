use crate::generator::generate;
use crate::Symbol;
use rand::distributions::{Distribution, Standard};
use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct Default(pub String);

impl Distribution<Default> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Default {
        Default(
            generate(rng, Symbol::Alphabet, Symbol::Consonant, Some(6))
        )
    }
}

impl ops::Deref for Default {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Default {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Default {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn default() {
        use crate::Default;
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();
        let gib: Default = rng.gen();
        assert!(gib.len() > 0);
        println!("large {}", gib);
    }
}