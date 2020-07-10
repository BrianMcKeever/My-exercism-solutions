/*
fn calc_prime_factor(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }
    for i in (3_u64..=n).step_by(2) {
        if n % i == 0 {
            return i;
        }
    }
    panic!("Should have returned");
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut x = n;
    while x != 1 {
        let factor = calc_prime_factor(x);
        factors.push(factor);
        x = x / factor;
    }
    factors
}
*/

pub fn factors(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut divisor = 2;
    let mut i = n;
    while i != 1 {
        if i % divisor == 0 {
            res.push(divisor);
            i /= divisor;
        } else {
            divisor += 1;
        }
    }

    res
}
