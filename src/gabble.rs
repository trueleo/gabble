use crate::generator::generate;
use crate::Syllable::{self, *};
/// Generator type used for generating custom variant of pseudo-word
/// ## Example
/// ```
/// use gabble::Gabble;
/// use gabble::Syllable::{Alphabet, Consonant};
/// use rand::thread_rng;
/// let mut rng = thread_rng();
/// //Generator configured to generate words
/// //that starts with consonant syllable and ends with a number
/// let gabble = Gabble::new()
///     .with_length(10)
///     .starts_with(Alphabet)
///     .ends_with(Consonant);
/// println!("customized answer to life is {}", gabble.generate(&mut rng));
/// ```
///
pub struct Gabble {
    pub start: Syllable,
    pub end: Syllable,
    pub length: Option<usize>,
}

impl Gabble {
    pub fn new() -> Self {
        Self {
            start: Alphabet,
            end: Consonant,
            length: None,
        }
    }

    pub fn with_length(mut self, n: usize) -> Self {
        if n < 3 {
            panic!("provide appropriate length");
        }
        self.length = Some(n);
        self
    }

    pub fn starts_with(mut self, syllable: Syllable) -> Self {
        self.start = syllable;
        self
    }

    pub fn ends_with(mut self, syllable: Syllable) -> Self {
        self.end = syllable;
        self
    }

    pub fn generate<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> String {
        generate(rng, self.start.clone(), self.end.clone(), self.length)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    pub fn gabble() {
        use crate::Gabble;
        use rand::thread_rng;
        use crate::Syllable::*;
        let mut rng = thread_rng();
        let gib = Gabble::new()
            .with_length(6)
            .starts_with(Alphabet)
            .ends_with(Consonant);
        let word = gib.generate(&mut rng);
        assert!(word.len() > 0);
        println!("gabble {}", word);
    }
}
