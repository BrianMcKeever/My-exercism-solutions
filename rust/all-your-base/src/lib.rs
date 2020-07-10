#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn to_base_10(number: &[u32], from_base: u32) -> u32 {
    number
        .iter()
        .zip((0..number.len() as u32).rev())
        .map(|(n, i)| n * from_base.pow(i))
        .sum()
}

fn from_10_to_base(number: u32, to_base: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = number;
    while n > 0 {
        result.push(n % to_base);
        n /= to_base;
    }
    result.iter().copied().rev().collect()
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if number.is_empty() || !number.iter().copied().any(|x| x > 0) {
        return Ok(vec![0]);
    }
    let invalid_option = number.iter().copied().find(|x| *x >= from_base);
    if let Some(invalid_number) = invalid_option {
        return Err(Error::InvalidDigit(invalid_number));
    }
    Ok(from_10_to_base(to_base_10(number, from_base), to_base))
}
