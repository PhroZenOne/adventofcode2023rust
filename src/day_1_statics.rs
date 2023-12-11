use aho_corasick::{AhoCorasick, MatchKind};
use lazy_static::lazy_static;

// So this is some magic that makes the NUMBER_FINDER a static instance...
lazy_static! {
    /// Uses the AhoCorasick algorithm (https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm)
    /// to search for the first instance of the numbers.
    /// The index returned is the index in NUMBERS_TO_INDEX and as it contains the numbers 0-9 three times
    /// in different versions.
    /// This means that we can use index % 10 to get the actual number.
    pub static ref NUMBER_FINDER: AhoCorasick = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(NUMBERS_TO_INDEX)
        .unwrap();
}

// Regular numbers
const ZERO_STR: &'static str = "zero";
const ONE_STR: &'static str = "one";
const TWO_STR: &'static str = "two";
const THREE_STR: &'static str = "three";
const FOUR_STR: &'static str = "four";
const FIVE_STR: &'static str = "five";
const SIX_STR: &'static str = "six";
const SEVEN_STR: &'static str = "seven";
const EIGHT_STR: &'static str = "eight";
const NINE_STR: &'static str = "nine";

// Reverse numbers to use when reading backwards
const ZERO_STR_REV: &'static str = "orez";
const ONE_STR_REV: &'static str = "eno";
const TWO_STR_REV: &'static str = "owt";
const THREE_STR_REV: &'static str = "eerht";
const FOUR_STR_REV: &'static str = "ruof";
const FIVE_STR_REV: &'static str = "evif";
const SIX_STR_REV: &'static str = "xis";
const SEVEN_STR_REV: &'static str = "neves";
const EIGHT_STR_REV: &'static str = "thgie";
const NINE_STR_REV: &'static str = "enin";

// Real numbers
const ZERO: &'static str = "0";
const ONE: &'static str = "1";
const TWO: &'static str = "2";
const THREE: &'static str = "3";
const FOUR: &'static str = "4";
const FIVE: &'static str = "5";
const SIX: &'static str = "6";
const SEVEN: &'static str = "7";
const EIGHT: &'static str = "8";
const NINE: &'static str = "9";

pub(crate) const NUMBERS_TO_INDEX: [&'static str; 30] = [
    ZERO_STR,
    ONE_STR,
    TWO_STR,
    THREE_STR,
    FOUR_STR,
    FIVE_STR,
    SIX_STR,
    SEVEN_STR,
    EIGHT_STR,
    NINE_STR,
    ZERO_STR_REV,
    ONE_STR_REV,
    TWO_STR_REV,
    THREE_STR_REV,
    FOUR_STR_REV,
    FIVE_STR_REV,
    SIX_STR_REV,
    SEVEN_STR_REV,
    EIGHT_STR_REV,
    NINE_STR_REV,
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE
];
