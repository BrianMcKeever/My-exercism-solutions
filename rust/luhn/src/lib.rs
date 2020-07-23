/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    if !code.iter().all(|c| c.is_digit(10)) {
        return false;
    }
    if code.len() <= 1 {
        return false;
    }
    let mut code: Vec<u8> = code.iter().copied().map(|c| c as u8 - b'0').collect();
    for i in (0..code.len()).rev().skip(1).step_by(2) {
        code[i] *= 2;
        if code[i] > 9 {
            code[i] -= 9;
        }
    }
    code.iter().rev().copied().map(|n| n as u64).sum::<u64>() % 10 == 0
}
