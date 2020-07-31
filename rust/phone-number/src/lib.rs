pub fn number(user_number: &str) -> Option<String> {
    let mut itter = user_number.chars().filter(|c| c.is_digit(10));
    let mut len = itter.clone().count();
    if len == 11 {
        if itter.next().unwrap() != '1' {
            return None;
        }
        len -= 1;
    }
    if len != 10 {
        return None;
    }
    itter
        .enumerate()
        .map(|(i, c)| {
            if (i == 0 || i == 3) && (c == '1' || c == '0') {
                None
            } else {
                Some(c)
            }
        })
        .collect()
}
