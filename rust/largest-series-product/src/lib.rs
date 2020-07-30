#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    match string_digits.chars().find(|c| !c.is_digit(10)) {
        None => (),
        Some(c) => return Err(Error::InvalidDigit(c)),
    }
    let digits: Vec<u64> = string_digits.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
    Ok(digits.windows(span).map(|w| w.iter().product()).max().unwrap())
 
}
