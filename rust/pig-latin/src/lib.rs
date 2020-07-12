#[macro_use]
extern crate lazy_static;
use regex::Regex;
lazy_static! {
    static ref VOWEL_SOUND_START: Regex = Regex::new(r"^([aeiou]|xr|yt)").unwrap();
    static ref CONSONANT_SOUND_START: Regex = Regex::new(r"^([bcdfghjklmnpqrstvwxyz]+)").unwrap();
    static ref CONSONANT_QU_START: Regex = Regex::new(r"^([bcdfghjklmnopqrstvwxyz]?qu)").unwrap();
    static ref CONSONANT_Y_START: Regex = Regex::new(r"^([bcdfghjklmnopqrstvwxz]+y)").unwrap();
}

fn piggify_word(input: &str) -> String {
    let lower_input = input.to_lowercase();
    let mut result = String::new();
    dbg!(&input);
    let prefix_len;
    if VOWEL_SOUND_START.is_match(&lower_input) {
        prefix_len = 0;
    } else if CONSONANT_QU_START.is_match(&lower_input) {
        prefix_len = CONSONANT_QU_START.captures(&lower_input).unwrap()[0].len();
    } else if CONSONANT_Y_START.is_match(&lower_input) {
        prefix_len = CONSONANT_Y_START.captures(&lower_input).unwrap()[0].len() - 1;
    } else {
        prefix_len = CONSONANT_SOUND_START.captures(&lower_input).unwrap()[0].len();
    }
    result.push_str(&input[prefix_len..]);
    result.push_str(&input[0..prefix_len]);
    result.push_str("ay");
    result
}

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(piggify_word)
        .collect::<Vec<_>>()
        .join(" ")
}
