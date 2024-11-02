pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut v = Vec::with_capacity(self.row_count as usize);

        for i in 0..self.row_count {
            v.push(Self::single_row(i));
        }
        v
    }

    /*
     * Pascal's Triange formula for k-th element in the n-th row:
     * C(n,k) = n! / (k! × (n-k)!); n = row_count, k = element_index -> (0..n)
     * using recursion:
     * C(n,k+1) = C(n,k) * (n-k) / (k+1)
     */
    pub fn single_row(n: u32) -> Vec<u32> {
        let mut c = Vec::with_capacity((n + 1) as usize);
        c.push(1);

        for k in 0..n {
            if let Some(last_c) = c.last() {
                c.push(last_c * (n - k) / (k + 1));
            }
        }
        c
    }
}
