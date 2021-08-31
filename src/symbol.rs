use std::ops::Range;

///Represent units that makes up a psuedoword
#[derive(Clone, Debug)]
pub enum Symbol {
    /// Vowel Syllable
    Vowel,
    /// Consonant Syllable
    Consonant,
    /// Alphabetical Syllable
    Alphabet,
    /// Number between start..=end
    Number(Range<usize>)
}

pub const INITIALS: [&'static str; 22] = [
    "bl", "br", "cl", "cr", "dr", "fl", "fr", "gl", "gr", "pl", "pr", "sk", "sl", "sm", "sn", "sp",
    "st", "str", "sw", "tr", "ch", "sh",
];
pub const VOWELS: [&'static str; 6] = ["a", "e", "i", "o", "u", "oo"];
pub const FINALS: [&'static str; 14] = [
    "ct", "ft", "mp", "nd", "ng", "nk", "nt", "pt", "sk", "sp", "ss", "st", "ch", "sh",
];
