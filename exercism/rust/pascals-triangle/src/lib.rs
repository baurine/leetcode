pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = vec![];

        if row_count > 0 {
            let mut row: Vec<u32>;
            for i in 1..=row_count {
                match i {
                    1 => row = vec![1],
                    2 => row = vec![1, 1],
                    _ => {
                        let pre_low = &rows[i as usize - 2];
                        row = vec![];
                        row.push(1);
                        for j in 1..=i - 2 {
                            row.push(pre_low[j as usize - 1] + pre_low[j as usize]);
                        }
                        row.push(1);
                    }
                }
                rows.push(row);
            }
        }

        PascalsTriangle(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
