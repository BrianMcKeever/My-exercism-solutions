use regex::Regex;
pub fn reply(message: &str) -> &str {
    let white_space = Regex::new(r"^[\s\r]*$").unwrap();
    if white_space.is_match(message) {
        return "Fine. Be that way!";
    }
    let question = Regex::new(r"\?\s*$").unwrap();
    let is_question = question.is_match(message);

    let lower_case = Regex::new(r"[a-z]").unwrap();
    let has_lower_case = lower_case.is_match(message);
    let upper_case = Regex::new(r"[A-Z]").unwrap();
    let has_upper_case = upper_case.is_match(message);
    let is_yelling = !has_lower_case && has_upper_case;

    if is_question {
        if is_yelling {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    } 
    if is_yelling {
        return "Whoa, chill out!";
    }
    return "Whatever."
}