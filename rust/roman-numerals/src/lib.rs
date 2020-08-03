use std::fmt::{Display, Formatter, Result};

pub struct Roman{
    number: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", u32_to_roman_numeral(self.number))
    }
}

fn u32_to_roman_numeral(num: u32) -> String {
    match num {
        0 => String::new(),
        1 => "I".to_string(),
        2 => "II".to_string(),
        3 => "III".to_string(),
        4 => "IV".to_string(),
        5 => "V".to_string(),
        6 => "VI".to_string(),
        7 => "VII".to_string(),
        8 => "VIII".to_string(),
        9 => "IX".to_string(),
        10 => "X".to_string(),
        11..=20 => "X".to_string() + &u32_to_roman_numeral(num - 10),
        21..=30 => "XX".to_string() + &u32_to_roman_numeral(num - 20),
        31..=39 => "XXX".to_string() + &u32_to_roman_numeral(num - 30),
        40..=49 => "XL".to_string() + &u32_to_roman_numeral(num - 40),
        50..=59 => "L".to_string() + &u32_to_roman_numeral(num - 50),
        60..=69 => "LX".to_string() + &u32_to_roman_numeral(num - 60),
        70..=79 => "LXX".to_string() + &u32_to_roman_numeral(num - 70),
        80..=89 => "LXXX".to_string() + &u32_to_roman_numeral(num - 80),
        90..=99 => "XC".to_string() + &u32_to_roman_numeral(num - 90),
        100..=199 => "C".to_string() + &u32_to_roman_numeral(num - 100),
        200..=299 => "CC".to_string() + &u32_to_roman_numeral(num - 200),
        300..=399 => "CCC".to_string() + &u32_to_roman_numeral(num - 300),
        400..=499 => "CD".to_string() + &u32_to_roman_numeral(num - 400),
        500..=899 => "D".to_string() + &u32_to_roman_numeral(num - 500),
        900..=999 => "CM".to_string() + &u32_to_roman_numeral(num - 900),
        1000..=4999 => "M".to_string() + &u32_to_roman_numeral(num - 1000),
        _ => "?".to_string(),
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman { number: num }
    }
}
