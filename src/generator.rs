use crate::Symbol;
use crate::{FINALS, INITIALS, VOWELS};
use rand::seq::SliceRandom;

pub fn generate<R: rand::Rng + ?Sized>(
    rng: &mut R,
    start: Symbol,
    end: Symbol,
    length: Option<usize>,
) -> String {

    let mut curr_length = 0;
    let mut symbols = Vec::new();
    let approx_length = length.unwrap_or_else(|| rng.gen_range(7..14));
    let mut final_string = String::with_capacity(approx_length + 5);

    while curr_length < approx_length {
        let v = *VOWELS.choose(rng).unwrap();
        curr_length += v.len();
        symbols.push(v);
        let f = *FINALS.choose(rng).unwrap();
        curr_length += f.len();
        symbols.push(f);
    }

    match start {
        Symbol::Consonant => final_string += INITIALS.choose(rng).unwrap(),
        Symbol::Alphabet => {
            if rng.gen_bool(0.65) {
                final_string += INITIALS.choose(rng).unwrap()
            }
        }
        Symbol::Number(range) => {
            let n = rng.gen_range(range);
            final_string += n.to_string().as_str();
        }
        Symbol::Vowel => (),
    }

    symbols.into_iter().for_each(|s| final_string += s);

    match end {
        Symbol::Consonant => (),
        Symbol::Alphabet => {
            if rng.gen_bool(0.35) {
                final_string += VOWELS.choose(rng).unwrap()
            }
        }
        Symbol::Number(range) => {
            let n = rng.gen_range(range);
            final_string += n.to_string().as_str();
        }
        Symbol::Vowel => final_string += VOWELS.choose(rng).unwrap(),
    }

    final_string
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_gabble() {
        assert!(true)
    }
}