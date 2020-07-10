pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    while i < source.len() {
        let c = chars[i];
        i += 1;
        let mut count = 1;
        while i < source.len() && chars[i] == c {
            count += 1;
            i += 1;
        }
        if count == 1 {
            result.push(c);
        } else {
            result.push_str(&count.to_string());
            result.push(c);
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    while i < source.len() {
        if chars[i].is_digit(10) {
            let mut number_string = String::new();
            while chars[i].is_digit(10) {
                number_string.push(chars[i]);
                i += 1;
            }
            for _ in 0..number_string.parse::<usize>().unwrap() {
                result.push(chars[i]);
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }
    result
}
