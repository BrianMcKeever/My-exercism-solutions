use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for score in h.keys() {
        for letter in h
            .get(score)
            .unwrap()
            .iter()
            .collect::<String>()
            .to_lowercase()
            .chars()
        {
            result.insert(letter, *score);
        }
    }
    result
}
