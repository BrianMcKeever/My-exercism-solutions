pub fn get_diamond(c: char) -> Vec<String> {
    let max_width = (1 + 2 * (c as u8 - b'A')) as usize;
    (b'A'..=c as u8)
        .enumerate()
        .chain((b'A'..c as u8).enumerate().rev())
        .map(|(i, c)| {
            let mut s = String::new();
            let c = c as char;
            if c == 'A' {
                let n = (max_width - 1) / 2;
                for _ in 0..n {
                    s.push(' ');
                }
                s.push('A');
                for _ in 0..n {
                    s.push(' ');
                }
            } else {
                let mid = 2 * (i - 1) + 1;
                let side = (max_width - mid - 2) / 2;
                for _ in 0..side {
                    s.push(' ');
                }
                s.push(c);
                for _ in 0..mid {
                    s.push(' ');
                }
                s.push(c);
                for _ in 0..side {
                    s.push(' ');
                }
            }
            s
        })
        .collect()
}
