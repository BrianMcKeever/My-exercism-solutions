pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut bools: Vec<bool> = vec![true; upper_bound as usize];
    for i in 0..upper_bound {
        if !bools[i as usize] {
            continue;
        }
        let num = i + 2;
        let mut j = num + num;
        while j <= upper_bound {
            bools[j as usize - 2] = false;
            j += num;
        }
    }
    bools
        .iter()
        .zip(2..=upper_bound)
        .filter(|(a, _)| **a)
        .map(|(_, b)| b)
        .collect()
}
