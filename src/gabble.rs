use crate::generator::generate;
use crate::symbol::Symbol::{self, *};
pub struct Gabble {
    start: Symbol,
    end: Symbol,
    length: Option<usize>,
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

    pub fn starts_with(mut self, symbol: Symbol) -> Self {
        self.start = symbol;
        self
    }

    pub fn ends_with(mut self, symbol: Symbol) -> Self {
        self.end = symbol;
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
        use crate::Symbol::*;
        let mut rng = thread_rng();
        let gib = Gabble::new()
            .with_length(6)
            .starts_with(Consonant)
            .ends_with(Number(10..100));
        let word = gib.generate(&mut rng);
        assert!(word.len() > 0);
        println!("gabble {}", word);
    }
}
