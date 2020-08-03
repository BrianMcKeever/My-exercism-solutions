use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    for c in sum/3..sum {
        for b in 0..sum - c {
            if c < b {
                continue;
            }
            let a = sum - b - c;
            if b < a {
                continue;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                set.insert([a, b, c]);
            }
        }
    }
    set
}
