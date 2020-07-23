pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut columns: Vec<Vec<char>> = minefield
        .iter()
        .map(|x| x.chars().collect())
        .collect();
    for y in 0..columns.len() {
        for x in 0..columns[y].len() {
            columns[y][x] = match columns[y][x] {
                '*' => '*',
                _ => {
                    let mut count: u8 = b'0';
                    #[rustfmt::skip]
                    static SURROUNDING: [(i32, i32); 8] = [
                        (-1, -1), (0, -1), (1, -1),
                        (-1,  0),          (1,  0),
                        (-1,  1), (0,  1), (1,  1),
                    ];
                    for (x1, y1) in SURROUNDING.iter().copied() {
                        if y == 0 && y1 == -1 || x == 0 && x1 == -1 {
                            continue;
                        }
                        let y2 = (y as i32 + y1) as usize;
                        let x2 = (x as i32 + x1) as usize;
                        columns
                            .get(y2)
                            .and_then(|o| o.get(x2))
                            .iter()
                            .for_each(|c| {
                                if **c == '*' {
                                    count += 1
                                }
                            });
                    }
                    if count == b'0' {
                        ' '
                    } else {
                        count as char
                    }
                }
            }
        }
    }

    columns.iter_mut().map(|r| r.iter().collect()).collect()
}
