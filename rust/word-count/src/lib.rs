use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    let valid: String = words.to_lowercase().chars().map(|x| {
        if x.is_ascii_punctuation() && x != '-' && x != '\'' {
            ' '
        } else {
            x
        }
    }).collect();
    valid.split(char::is_whitespace).for_each(|word| {
        let word = word.trim_matches('\'');
        if word != "" {
            result.insert(word.to_string(), result.get(word).unwrap_or(&0) + 1 as u32);
        }
    });

    result
}
