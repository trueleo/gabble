use crate::Syllable;
use crate::{FINALS, INITIALS, VOWELS};
use rand::seq::IndexedRandom;

pub fn generate<R: rand::Rng + ?Sized>(
    rng: &mut R,
    start: Syllable,
    end: Syllable,
    length: Option<usize>,
) -> String {
    let mut curr_length = 0;
    let mut syllables = Vec::new();
    let approx_length = length.unwrap_or_else(|| rng.random_range(7..9));
    let mut final_string = String::with_capacity(approx_length + 5);

    while curr_length < approx_length {
        let v = *VOWELS.choose(rng).unwrap();
        curr_length += v.len();
        syllables.push(v);
        let f = *FINALS.choose(rng).unwrap();
        curr_length += f.len();
        syllables.push(f);
    }

    match start {
        Syllable::Consonant => final_string += INITIALS.choose(rng).unwrap(),
        Syllable::Alphabet => {
            if rng.random_bool(0.65) {
                final_string += INITIALS.choose(rng).unwrap()
            }
        }
        Syllable::Vowel => (),
    }

    syllables.into_iter().for_each(|s| final_string += s);

    match end {
        Syllable::Consonant => (),
        Syllable::Alphabet => {
            if rng.random_bool(0.35) {
                final_string += VOWELS.choose(rng).unwrap()
            }
        }
        Syllable::Vowel => final_string += VOWELS.choose(rng).unwrap(),
    }

    final_string
}
