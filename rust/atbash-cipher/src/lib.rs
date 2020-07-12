use std::convert::TryFrom;

fn encode_char(l: char) -> Option<char> {
    if l.is_ascii_alphabetic() {
        //formula is
        //new_ascii_index = 25 + 97 - (l as usize - 97)
        //97 = a as usize
        Some(u8::try_from(219 - (l as u8)).unwrap() as char)
    } else if l.is_ascii_digit() {
        Some(l)
    } else {
        None
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_string()
        .to_lowercase()
        .chars()
        .map(encode_char)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .to_string()
        .to_lowercase()
        .chars()
        .map(encode_char)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}
