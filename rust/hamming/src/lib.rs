/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    s1.chars().zip(s2.chars()).fold(Some(0), |a, (x, y)| {
        match a {
            None => None,
            Some(d) => {
                if x == y {
                    a
                } else {
                    Some(d+1)
                } 
            },
        }
    })
}
