pub fn count(lines: &[&str]) -> u32 {
    let lines: Vec<Vec<char>> = lines.iter().map(|row| row.chars().collect()).collect();
    let mut count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] != '+' {
                continue;
            }
            let mut xs: Vec<usize> = Vec::new();
            for (x1, _) in lines[y].iter().enumerate().skip(x + 1) {
                match lines[y][x1] {
                    '+' => xs.push(x1),
                    '-' => (),
                    ' ' | '|' => break,
                    a => panic!("unknown input {}", a),
                }
            }
            let mut ys: Vec<usize> = Vec::new();
            for (y1, _) in lines.iter().enumerate().skip(y + 1) {
                match lines[y1][x] {
                    '+' => ys.push(y1),
                    '|' => (),
                    ' ' | '-' => break,
                    a => panic!("unknown input {}", a),
                }
            }
            for x1 in xs.iter().copied() {
                for y1 in ys.iter().copied() {
                    if lines[y1][x1] == '+' {
                        let mut valid = true;
                        for line in lines[y1].iter().take(x1).skip(x + 1) {
                            match line {
                                '+' | '-' => (),
                                ' ' | '|' => {
                                    valid = false;
                                    break;
                                }
                                a => panic!("unknown input {}", a),
                            }
                        }
                        for line in lines.iter().take(y1).skip(y + 1) {
                            match line[x1] {
                                '+' | '|' => (),
                                ' ' | '-' => {
                                    valid = false;
                                    break;
                                }
                                a => panic!("unknown input {}", a),
                            }
                        }
                        if valid {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
