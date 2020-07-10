use std::collections::HashSet;
pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut v = Vec::from(sides);
        v.sort();
        if v[0] + v[1] < v[2] || v[0] == 0 || v[1] == 0 || v[2] == 0 {
            None
        } else {
            Some(Triangle { sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.iter().collect::<HashSet<&u64>>().len() == self.sides.len()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
