pub fn abbreviate(phrase: &str) -> String {
    let mut previous_lowercase = false;
    let mut previous_special_char = true;
    let mut potential_letter;
    let mut result = String::new();
    for c in phrase.chars() {
        potential_letter = (previous_lowercase && c.is_ascii_uppercase()) || previous_special_char;
        if potential_letter && c.is_ascii_alphabetic() {
            result.push(c.to_ascii_uppercase());
        }
        previous_lowercase = c.is_ascii_lowercase();
        previous_special_char = c == ' ' || c == '-' || c == '_'
    }
    result
}
