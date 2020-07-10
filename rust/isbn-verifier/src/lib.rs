/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if !isbn
        .chars()
        .all(|c| c.is_digit(10) || c == '-' || c == 'X' || c == 'x')
    {
        return false;
    }
    let filtered: Vec<char> = isbn.chars().filter(|c| c.is_alphanumeric()).collect();
    if filtered.len() != 10 {
        return false;
    }
    if filtered.iter().copied().any(|c| c == 'X' || c == 'x') {
        if filtered[9] != 'X' && filtered[9] != 'x' {
            return false;
        }
    }
    filtered
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let n = if *c == 'x' || *c == 'X' {
                10
            } else {
                c.to_digit(10).unwrap()
            };
            n * (10 - i as u32)
        })
        .sum::<u32>()
        % 11
        == 0
}
