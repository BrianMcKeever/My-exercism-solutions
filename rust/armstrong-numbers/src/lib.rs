pub fn is_armstrong_number(num: u32) -> bool {
    let number_string = num.to_string();
    let number_digits = number_string.len() as u32;
    number_string
        .chars()
        .map(|x| x.to_digit(10))
        .map(|x| x.unwrap())
        .map(|x| x.pow(number_digits))
        .sum::<u32>()
        == num
}
