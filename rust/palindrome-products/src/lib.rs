use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let factors = HashSet::new();
        let mut p = Palindrome { factors };
        p.insert(a, b);
        p
    }

    pub fn value(&self) -> u64 {
        let pair = self.factors.iter().next().unwrap();
        pair.1 * pair.0
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        if a < b {
            self.factors.insert((a, b));
        } else {
            self.factors.insert((b, a));
        }
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    if s.len() == 1 {
        return true;
    }
    !s.chars()
        .zip(s.chars().rev())
        .take(s.len() / 2)
        .any(|(a, b)| a != b)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_product = u64::MAX;
    let mut max_product = u64::MIN;
    let mut min_palindrome = None;
    let mut max_palindrome = None;

    for x in min..=max {
        for y in min..=max {
            let product = x * y;
            if product > min_product && product < max_product {
                continue;
            }
            if !is_palindrome(product) {
                continue;
            }
            let update = |mut palindrome: Option<Palindrome>| {
                match palindrome.as_mut() {
                    None => Some(Palindrome::new(x, y)),
                    Some(p) => {
                        if p.value() == x * y {
                            p.insert(x, y);
                            palindrome
                        } else {
                            let mut n = Palindrome::new(x, y);
                            n.insert(x, y);
                            Some(n)
                        }
                    }
                }
            };
            if product <= min_product {
                min_product = product;
                min_palindrome = update(min_palindrome);
            }
            if product >= max_product {
                max_product = product;
                max_palindrome = update(max_palindrome);
            }
        }
    }
    Some((min_palindrome?, max_palindrome?))
}
