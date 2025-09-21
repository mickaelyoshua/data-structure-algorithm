// Fenwick Tree (Binary Indexed Tree or BIT)

pub mod fenwick_tree {
    pub struct FenwickTree {
        sums: Vec<i32>,
    }

    impl FenwickTree {
        pub fn new(size: usize) -> Self {
            FenwickTree {
                sums: vec![0;size+1]
            }
        }

        pub fn update(&mut self, index: i32, delta: i32) {
            let mut i = index;
            while (i as usize) < self.sums.len() {
                self.sums[i as usize] += delta;
                i = i + (i & -i);
            }
        }

        pub fn query(&self, index: i32) -> i32 {
            let mut i = index;
            let mut sum = 0;

            while i > 0 {
                sum += self.sums[i as usize];
                i = i - (i & -i);
            }
            sum
        }
    }
}

