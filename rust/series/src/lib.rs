pub fn series(digits: &str, len: usize) -> Vec<String> {
    let v: Vec<char> = digits.chars().collect();
    let mut result = Vec::new();

    if len == 0 {
        for _ in 0..=(digits.len()) {
            result.push(String::new());
        }
        return result;
    }
    v.windows(len).for_each(|a| result.push(a.iter().collect()));
    result
}
