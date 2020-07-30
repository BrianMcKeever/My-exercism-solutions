pub struct Luhn {
    data: Option<Vec<u64>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut longer_than_1 = false;
        let data = match &self.data {
            None => return false,
            Some(d) => d,
        };
        dbg!(data);
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

impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        Luhn::from(input.to_string())
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn {
            data: input
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).map(|n| n as u64))
                .collect(),
        }
    }
}

impl From<u64> for Luhn {
    fn from(input: u64) -> Self {
        Luhn::from(input.to_string())
    }
}

impl From<u8> for Luhn {
    fn from(input: u8) -> Self {
        Luhn::from(input.to_string())
    }
}
impl From<u32> for Luhn {
    fn from(input: u32) -> Self {
        Luhn::from(input.to_string())
    }
}

impl From<usize> for Luhn {
    fn from(input: usize) -> Self {
        Luhn::from(input.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(input: u16) -> Self {
        Luhn::from(input.to_string())
    }
}
