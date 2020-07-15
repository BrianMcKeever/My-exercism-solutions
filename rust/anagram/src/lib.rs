use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut sorted_word = word_lower.chars().collect::<Vec<_>>();
    sorted_word.sort_unstable();

    let mut result: HashSet<&'a str> = HashSet::new();
    for possible in possible_anagrams {
        let lower_case = possible.to_lowercase();
        if lower_case == word_lower {
            continue;
        }
        let mut sorted_possible = lower_case.chars().collect::<Vec<_>>();
        sorted_possible.sort_unstable();
        if sorted_word == sorted_possible {
            result.insert(*possible);
        }
    }
    result
}
