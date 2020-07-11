use bit_vec::BitVec;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    for value in values {
        let input_bits = BitVec::from_bytes(&value.to_be_bytes());
        let mut no_leading_zeroes: BitVec = input_bits.iter().skip_while(|b| !b).collect();
        let mut bits = BitVec::new();
        //we're building this backwards due to pushing to the end

        let mut is_more = false;
        while !is_more || !no_leading_zeroes.is_empty() {
            for _ in 0..7 {
                match no_leading_zeroes.pop() {
                    None => bits.push(false), //padding the byte with zeroes
                    Some(b) => bits.push(b),
                }
            }
            bits.push(is_more); //signaling more bytes or not
            is_more = true;
        }
        bits = bits.iter().rev().collect();
        result.extend_from_slice(&bits.to_bytes());
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut input_bits = BitVec::from_bytes(&bytes);
    input_bits = input_bits.iter().rev().collect();
    while !input_bits.is_empty() {
        let mut bits = BitVec::new();
        let mut should_continue = true;
        while should_continue {
            should_continue = input_bits.pop().unwrap();
            for _ in 0..7 {
                bits.push(input_bits.pop().unwrap());
            }
            if should_continue && input_bits.is_empty() {
                return Err(Error::IncompleteNumber);
            }
        }
        bits = bits.iter().rev().collect();
        while bits.len() < 32 {
            bits.push(false);
        }
        while bits.len() > 32 {
            if let Some(b) = bits.pop() {
                if b {
                    return Err(Error::Overflow);
                }
            }
        }
        bits = bits.iter().rev().collect();
        let mut array: [u8; 4] = [0; 4];
        array.copy_from_slice(&bits.to_bytes().as_slice());
        result.push(u32::from_be_bytes(array));
    }
    Ok(result)
}
