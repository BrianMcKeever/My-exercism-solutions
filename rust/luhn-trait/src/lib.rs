pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let data: Option<Vec<u64>> = self
            .to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).map(|n| n as u64))
            .collect();

        let mut longer_than_1 = false;
        let data = match &data {
            None => return false,
            Some(d) => d,
        };
        data.iter()
            .rev()
            .copied()
            .enumerate()
            .inspect(|(i, _)| {
                if *i > 0 {
                    longer_than_1 = true;
                }
            })
            .map(|(i, mut n)| {
                if i % 2 == 1 {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }
                n
            })
            .sum::<u64>()
            % 10
            == 0
            && longer_than_1
    }
}
