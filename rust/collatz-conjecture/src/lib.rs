pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    let mut n = n;
    if n == 0 {
        return None;
    }
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    Some(count)
}
