fn max_points((x, column): (usize, &Vec<u64>)) -> Vec<(usize, usize, u64)> {
    column
        .iter()
        .copied()
        .enumerate()
        .fold(vec![], |mut a, (y, value)| {
            if a.is_empty() || value == a[0].2 {
                a.push((x, y, value));
                a
            } else if value > a[0].2 {
                vec![(x, y, value)]
            } else {
                a
            }
        })
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let column_high_points = input
        .iter()
        .enumerate()
        .map(max_points)
        .flatten()
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    for (x, y, value) in column_high_points.iter().copied() {
        let mut is_saddle_point = true;
        for (x2, list) in input.iter().enumerate() {
            if x == x2 {
                continue;
            }
            if value > list[y] {
                is_saddle_point = false;
                break;
            }
        }
        if !is_saddle_point {
            continue;
        }
        result.push((x, y));
    }

    result
}
