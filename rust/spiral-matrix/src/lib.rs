enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Right;
    for i in 1..=(size.pow(2)) {
        matrix[y][x] = i;
        match direction {
            Direction::Right => {
                if x + 1 < size as usize && matrix[y][x + 1] == 0 {
                    x += 1;
                } else {
                    y += 1;
                    direction = Direction::Down;
                }
            }
            Direction::Down => {
                if y + 1 < size as usize && matrix[y + 1][x] == 0 {
                    y += 1;
                } else {
                    x -= 1;
                    direction = Direction::Left;
                }
            }
            Direction::Left => {
                if x != 0 && matrix[y][x - 1] == 0 {
                    x -= 1;
                } else {
                    y -= 1;
                    direction = Direction::Up;
                }
            }
            Direction::Up => {
                if y != 0 && matrix[y - 1][x] == 0 {
                    y -= 1;
                } else {
                    x += 1;
                    direction = Direction::Right;
                }
            }
        }
    }
    matrix
}
