use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let lower_case = candidate.to_lowercase();
    let letters = lower_case.chars().filter(|c| *c != '-' && *c != ' ');
    let set = letters.clone().collect::<HashSet<_>>();
    set.len() == letters.count()
}
