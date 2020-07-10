use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    for i in 1..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            factors.push(i);
            let b = num/i;
            if b != num  && b != i {
                factors.push(b);
            }
        }
    }
    factors
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }
    let sum : u64 = factors(num).iter().sum();
    Some(match sum.cmp(&num){
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    })
}
