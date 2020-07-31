fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return false;
    }

    let max: f64 = n.into();
    let max = max.sqrt().floor() as u32 + 1;
    for i in (3..max).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut number_primes = 1;
    for i in (3..u32::max_value()).step_by(2) {
        if is_prime(i) {
            if n == number_primes {
                return i;
            }
            number_primes += 1;
        }
    }
    panic!("bigger than u32"); //just gonna ignore that this will choke on big enough inputs
}
