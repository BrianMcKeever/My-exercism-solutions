pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for i in 0..(row_count as usize) {
            let mut row = Vec::new();
            for j in 0..=i {
                if j == 0 || i == j {
                    row.push(1);
                    continue;
                }
                row.push(rows[i - 1][j - 1] + rows[i - 1][j]);
            }
            rows.push(row);
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
