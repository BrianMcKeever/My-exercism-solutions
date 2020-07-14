pub fn encrypt(input: &str) -> String {
    let mut normal_input = input
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .collect::<Vec<_>>();
    let input_length = normal_input.len();

    let row_width = (input_length as f32).sqrt().ceil() as usize;
    if row_width == 0 {
        return String::new();
    }

    while normal_input.len() % row_width != 0 {
        normal_input.push(' ');
    }

    let rows = normal_input
        .chunks(row_width)
        .map(|a| a.iter().copied().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut result_unspaced = Vec::new();
    for i in 0..row_width {
        for row in rows.iter() {
            match row.get(i) {
                Some(&' ') => (),
                Some(l) => result_unspaced.push(*l),
                None => result_unspaced.push(' '),
            }
        }
    }
    let output_row_width = (input_length as f32).sqrt().floor() as usize;
    let number_extra_spaces =
        if input_length % output_row_width != 0 {
            output_row_width - input_length % output_row_width
        } else {
            0
        };
    for i in 0..number_extra_spaces {
        result_unspaced.insert(result_unspaced.len() - i * output_row_width, ' ');
    }
    let result_rows = result_unspaced
        .chunks(output_row_width)
        .map(|a| a.iter().copied().collect::<String>())
        .collect::<Vec<_>>();

    result_rows.join(" ")
}