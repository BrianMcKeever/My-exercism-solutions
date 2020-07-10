pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();
    let openings = ['{', '[', '('];
    let closings = ['}', ']', ')'];
    for character in string.chars() {
        if openings.contains(&character) {
            brackets.push(character);
        } else if closings.contains(&character) {
            let left = brackets.pop().unwrap_or('!');
            //the ! will return false
            match (left, character) {
                ('{', '}') => continue,
                ('[', ']') => continue,
                ('(', ')') => continue,
                _ => return false,
            }
        }
    }
    brackets.is_empty()
}
