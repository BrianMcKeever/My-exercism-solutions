pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if command[command.len() - 1..] != *"?" {
        return None;
    }
    let mut split = (*command)[0..command.len() - 1].split(' ');
    if split.next() != Some("What") || split.next() != Some("is") {
        return None;
    }
    let mut total = split.next()?.parse().ok()?;
    loop {
        match split.next() {
            None => return Some(total),
            Some("plus") => match split.next() {
                None => return None,
                Some(n) => {
                    total += n.parse::<i32>().ok()?;
                }
            },
            Some("minus") => match split.next() {
                None => return None,
                Some(n) => {
                    total -= n.parse::<i32>().ok()?;
                }
            },
            Some("multiplied") => {
                if split.next() != Some("by") {
                    return None;
                }
                match split.next() {
                    None => return None,
                    Some(n) => {
                        total *= n.parse::<i32>().ok()?;
                    }
                }
            }
            Some("divided") => {
                if split.next() != Some("by") {
                    return None;
                }
                match split.next() {
                    None => return None,
                    Some(n) => {
                        total /= n.parse::<i32>().ok()?;
                    }
                }
            }
            _ => return None,
        }
    }
}
