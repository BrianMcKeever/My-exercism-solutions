use lazy_static::lazy_static;
use std::collections::HashMap;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref LETTER_SCORES: HashMap<String, u64> = {
        let mut letter_scores = HashMap::new();
        let triplets = [
            ("A", "a", 1),
            ("B", "b", 3),
            ("C", "c", 3),
            ("D", "d", 2),
            ("E", "e", 1),
            ("F", "f", 4),
            ("G", "g", 2),
            ("H", "h", 4),
            ("I", "i", 1),
            ("J", "j", 8),
            ("K", "k", 5),
            ("L", "l", 1),
            ("M", "m", 3),
            ("N", "n", 1),
            ("O", "o", 1),
            ("P", "p", 3),
            ("Q", "q", 10),
            ("R", "r", 1),
            ("S", "s", 1),
            ("T", "t", 1),
            ("U", "u", 1),
            ("V", "v", 4),
            ("W", "w", 4),
            ("X", "x", 8),
            ("Y", "y", 4),
            ("Z", "z", 10),
        ];
        for (a, b, s) in triplets.iter() {
            letter_scores.insert(a.to_string(), *s);
            letter_scores.insert(b.to_string(), *s);
        }
        letter_scores
    };
}

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.graphemes(true)
        .map(|c| LETTER_SCORES.get(&c.to_string()).unwrap_or(&0))
        .sum()
}
