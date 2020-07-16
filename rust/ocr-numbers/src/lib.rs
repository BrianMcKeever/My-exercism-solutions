#[rustfmt::skip]
static DIGITS: [[&'static str; 4]; 10] = [
    [
        " _ ", 
        "| |", 
        "|_|", 
        "   "
    ],
    [
        "   ", 
        "  |", 
        "  |", 
        "   "
    ],
    [
        " _ ", 
        " _|", 
        "|_ ", 
        "   "
    ],
    [
        " _ ", 
        " _|", 
        " _|", 
        "   "
    ],
    [
        "   ", 
        "|_|", 
        "  |", 
        "   "
    ],
    [
        " _ ", 
        "|_ ", 
        " _|", 
        "   "
    ],
    [
        " _ ", 
        "|_ ", 
        "|_|", 
        "   "
    ],
    [
        " _ ", 
        "  |", 
        "  |", 
        "   "
    ],
    [
        " _ ", 
        "|_|", 
        "|_|", 
        "   "
    ],
    [
        " _ ", 
        "|_|", 
        " _|", 
        "   "
    ],
];

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
    InconsistantRowLength,
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(input.len() / 3));
    }
    for line in &lines {
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
        if line.len() != lines[0].len() {
            return Err(Error::InconsistantRowLength);
        }
    }

    let mut result = String::new();
    for row in (0..lines.len()).step_by(4) {
        for i in (0..lines[row].len()).step_by(3) {
            let digit = [
                &lines[row + 0][i..i + 3],
                &lines[row + 1][i..i + 3],
                &lines[row + 2][i..i + 3],
                &lines[row + 3][i..i + 3],
            ];
            match DIGITS.iter().enumerate().find(|(_, d)| *d == &digit) {
                Some((n, _)) => result.push((n as u8 + b'0') as u8 as char),
                None => result.push('?'),
            }
        }
        if row + 4 < lines.len() {
            result.push(',');
        }
    }

    Ok(result)
}
