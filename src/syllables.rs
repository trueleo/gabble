///Enum to represent available kind of syllables
#[derive(Clone, Debug)]
pub enum Syllable {
    /// syllable comprised of vowels
    Vowel,
    /// syllable comprised of consonants
    Consonant,
    /// either vowel or consonant syllable
    Alphabet,
}

pub const INITIALS: [&str; 121] = [
    "bl", "br", "cl", "cr", "dr", "fl", "fr", "gl", "gr", "pl", "pr", "sk", "sl", "sm", "sn", "sp",
    "st", "str", "sw", "tr", "ch", "sh", "m", "c", "b", "r", "d", "h", "s", "p", "l", "g", "f",
    "w", "t", "k", "n", "v", "st", "pr", "j", "br", "ch", "gr", "sh", "tr", "cr", "fr", "z", "sp",
    "wh", "cl", "y", "bl", "th", "fl", "sch", "pl", "q", "dr", "str", "sc", "sl", "kr", "sw", "gl",
    "ph", "kl", "sm", "sn", "kn", "sk", "mcc", "scr", "wr", "mc", "chr", "spr", "thr", "tw",
    "schw", "mcg", "gyn", "rh", "sq", "schl", "shr", "schr", "x", "schm", "mcm", "gh", "mcn",
    "hyp", "mq", "schn", "mcd", "hydr", "kh", "ts", "mcl", "by", "spl", "dw", "pf", "bh", "ryn",
    "typ", "cz", "sr", "fry", "gn", "hr", "hy", "syn", "sz", "kw", "dyn", "ll", "pc", "cyr",
];
pub const VOWELS: [&str; 46] = [
    "a", "e", "i", "o", "u", "oo", "e", "a", "i", "o", "u", "ia", "ie", "ee", "io", "au", "ea",
    "ou", "ai", "ue", "ei", "eau", "eu", "oe", "ae", "eo", "oa", "oo", "ao", "ua", "oi", "ui",
    "aa", "ieu", "uo", "oia", "aue", "iu", "aia", "iou", "ii", "aio", "uie", "eia", "uia", "iao",
];
pub const FINALS: [&str; 113] = [
    "ct", "ft", "mp", "nd", "ng", "nk", "nt", "pt", "sk", "sp", "ss", "st", "ch", "sh", "s", "n",
    "r", "d", "ng", "l", "y", "rs", "ns", "t", "m", "ll", "nt", "c", "ck", "st", "k", "ss", "sts",
    "rd", "nd", "ry", "rt", "w", "lly", "tt", "ch", "ts", "ty", "p", "ls", "ld", "nts", "x", "rg",
    "sh", "ly", "th", "ff", "g", "rn", "ngs", "nn", "tz", "sm", "gh", "ms", "z", "cs", "ps", "h",
    "ds", "b", "lt", "nk", "nds", "ys", "rk", "ght", "v", "cks", "f", "ct", "rth", "rry", "lls",
    "ny", "ws", "cts", "wn", "rds", "dy", "bly", "rts", "ft", "hl", "gy", "pp", "rly", "mp",
    "ntly", "sch", "ngly", "sly", "ks", "tch", "ncy", "rm", "gs", "rty", "hn", "fy", "rst", "rr",
    "ntz", "bs", "cy", "dly", "tts",
];
