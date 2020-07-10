fn big_number(n: u64, divisor: u64, name: &str) -> String {
    let amount = n / divisor;
    let remainder = n % divisor;
    let mut result = encode(amount) + name;
    if remainder != 0 {
        result.push_str(" ");
        result.push_str(&encode(remainder));
    }
    result
}

pub fn encode(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        21..=29 => String::from("twenty-") + &encode(n - 20),
        30 => String::from("thirty"),
        31..=39 => String::from("thirty-") + &encode(n - 30),
        40 => String::from("forty"),
        41..=49 => String::from("forty-") + &encode(n - 40),
        50 => String::from("fifty"),
        51..=59 => String::from("fifty-") + &encode(n - 50),
        60 => String::from("sixty"),
        61..=69 => String::from("sixty-") + &encode(n - 60),
        70 => String::from("seventy"),
        71..=79 => String::from("seventy-") + &encode(n - 70),
        80 => String::from("eighty"),
        81..=89 => String::from("eighty-") + &encode(n - 80),
        90 => String::from("ninety"),
        91..=99 => String::from("ninety-") + &encode(n - 90),
        100..=999 => big_number(n, 100, " hundred"),
        1000..=999_999 => big_number(n, 1000, " thousand"),
        1_000_000..=999_999_999 => big_number(n, 1_000_000, " million"),
        1_000_000_000..=999_999_999_999 => big_number(n, 1_000_000_000, " billion"),
        1_000_000_000_000..=999_999_999_999_999 => big_number(n, 1_000_000_000_000, " trillion"),
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            big_number(n, 1_000_000_000_000_000, " quadrillion")
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            big_number(n, 1_000_000_000_000_000_000, " quintillion")
        }
    }
}
