// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    facing: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, facing: d }
    }

    pub fn turn_right(self) -> Self {
        match self.facing {
            Direction::North => Robot {
                facing: Direction::East,
                ..self
            },
            Direction::East => Robot {
                facing: Direction::South,
                ..self
            },
            Direction::South => Robot {
                facing: Direction::West,
                ..self
            },
            Direction::West => Robot {
                facing: Direction::North,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.facing {
            Direction::North => Robot {
                facing: Direction::West,
                ..self
            },
            Direction::East => Robot {
                facing: Direction::North,
                ..self
            },
            Direction::South => Robot {
                facing: Direction::East,
                ..self
            },
            Direction::West => Robot {
                facing: Direction::South,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.facing {
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            match instruction {
                'A' => robot = robot.advance(),
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                _ => panic!("Invalid instruction"),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
