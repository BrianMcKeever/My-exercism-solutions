/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut longer_than_1 = false;
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .inspect(|(i, _)| {
            if *i > 0 {
                longer_than_1 = true;
            }
        })
        .map(|(i, c)| {
            c.to_digit(10).map(|mut n| {
                if i % 2 == 1 {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }
                n
            })
        })
        .sum::<Option<u32>>()
        .map(|n| n % 10 == 0 && longer_than_1)
        .unwrap_or(false)
}
