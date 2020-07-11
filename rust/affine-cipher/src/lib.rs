use std::char;
/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ALPHABET_LENGTH: i32 = 26;

fn encode_letter(letter: char, a: i32, b: i32) -> char {
    let alphabet_position = letter as i32 - 97;
    ALPHABET[((a * alphabet_position + b) % ALPHABET_LENGTH) as usize]
}

fn decode_letter(letter: char, a: i32, b: i32) -> char {
    let alphabet_position = letter as i32 - 97;
    let mut mmi = 1;
    while a * mmi % ALPHABET_LENGTH != 1 {
        mmi += 1;
    }
    ALPHABET[(((alphabet_position - b) * mmi).rem_euclid(ALPHABET_LENGTH)) as usize]
}

fn is_coprime(a: i32, b: i32) -> bool {
    (2..=ALPHABET_LENGTH).any(|n| a % n == 0 && b % n == 0)
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if is_coprime(a, ALPHABET_LENGTH) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut result = String::new();
    let mut count = 0;
    for letter in plaintext.to_lowercase().chars() {
        if !letter.is_ascii_alphanumeric() {
            continue;
        }
        if count == 5 {
            result.push(' ');
            count = 0;
        }
        if letter.is_numeric() {
            result.push(letter);
        } else {
            result.push(encode_letter(letter, a, b));
        }
        count += 1
    }
    Ok(result)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if is_coprime(a, ALPHABET_LENGTH) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(ciphertext
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|letter| {
            if letter.is_numeric() {
                letter
            } else {
                decode_letter(letter, a, b)
            }
        })
        .collect())
}
