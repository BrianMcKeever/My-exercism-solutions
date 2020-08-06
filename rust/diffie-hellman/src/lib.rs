use rand::prelude::*;
use rand::rngs::StdRng;
pub fn private_key(private: u64) -> u64 {
    let mut r = StdRng::from_entropy();
    r.gen_range(2, private)
}

pub fn public_key(p: u64, g: u64, private: u64) -> u64 {
    modular_pow(g, private, p)
}

pub fn secret(p: u64, b_pub: u64, private: u64) -> u64 {
    modular_pow(b_pub, private, p)
}

fn modular_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64{
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 { 
        if exponent % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exponent = exponent >> 1;
        base = ((base as u128 * base as u128) % modulus as u128 ) as u64;
    }
    result
}
