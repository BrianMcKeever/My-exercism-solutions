use std::convert::TryFrom;

fn shift_within_range(letter: char, key: i8, lower_bound: u8, upper_bound: u8)-> char {
    let ascii_index = letter as i32 + key as i32;
    let mut shifted = u8::try_from(ascii_index).unwrap();
    let alphabet_length = 26;
    if shifted < lower_bound {
        shifted += alphabet_length;
    } else if shifted > upper_bound {
        shifted -= alphabet_length;
    }
    shifted as char
}

pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|l| {
        match l {
            'a'..='z' => shift_within_range(l, key, 'a' as u8, 'z' as u8),
            'A'..='Z' => shift_within_range(l, key, 'A' as u8, 'Z' as u8),
            _ => l,
        }
    }).collect()
}
