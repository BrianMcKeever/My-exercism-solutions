pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    //This should really just return 18_446_744_073_709_551_615 not calculate anything.
    (1..=64).map(square).sum()
}
