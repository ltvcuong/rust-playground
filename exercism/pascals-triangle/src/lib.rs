use std::iter;

pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = vec![];
        match self.row_count {
            0 => {}
            r => {
                rows.push(vec![1]);
                for _ in 2..=r {
                    let last = rows.last().unwrap();
                    let iter1 = iter::once(&0).chain(last.iter());
                    let iter2 = last.iter().chain(iter::once(&0));
                    let row = iter1
                        .zip(iter2)
                        .map(|(i1, i2)| i1 + i2)
                        .collect();
                    rows.push(row);
                }
            }
        }
        rows
    }
}
