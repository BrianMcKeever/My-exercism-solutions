use rand::{thread_rng, Rng};
use std::convert::TryFrom;
use std::iter;

fn shift_within_range(letter: char, key: char, positive: bool) -> char {
    let key_index = key as i16 - 'a' as i16;
    let letter_index = if positive {
        letter as i16 + key_index
    } else {
        letter as i16 - key_index
    };
    let mut shifted = u8::try_from(letter_index).unwrap();
    let alphabet_length = 26;
    if shifted < 'a' as u8 {
        shifted += alphabet_length;
    } else if shifted >= 'a' as u8 + alphabet_length {
        shifted -= alphabet_length;
    }
    shifted as char
}

fn shift(key: &str, input: &str, positive: bool) -> String {
    input
        .chars()
        .zip(key.chars().cycle())
        .map(|(l, k)| shift_within_range(l, k, positive))
        .collect()
}

fn is_valid_key(key: &str) -> bool {
    if key.len() == 0 {
        return false;
    }
    if !key.chars().all(|x| x.is_alphabetic() && x.is_lowercase()) {
        return false;
    }
    return true;
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    Some(shift(key, s, true))
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    Some(shift(key, s, false))
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key = iter::repeat_with(|| rng.gen_range('a' as u8, 'z' as u8 + 1) as char)
        .take(100)
        .collect::<String>();
    (key.clone(), shift(&key, s, true))
}
