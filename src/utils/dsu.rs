use std::mem::swap;

#[derive(Default)]
pub struct DisjointSetUnion {
    data: Vec<(usize, usize)>,
}

#[allow(dead_code)]
impl DisjointSetUnion {
    pub fn new(size: usize) -> Self {
        let mut init_dsu = Self {
            data: vec![(0, 0); size],
        };
        for i in 0..size {
            init_dsu.data[i] = (i, 1);
        }
        init_dsu
    }

    fn find(&self, v: usize) -> usize {
        if v == self.data[v].0 {
            return v;
        }
        self.find(self.data[v].0)
    }

    fn union(&mut self, mut u: usize, mut v: usize) {
        u = self.find(u);
        v = self.find(v);
        if u != v {
            if self.data[u].1 < self.data[v].1 {
                swap(&mut u, &mut v);
            }
            self.data[v].0 = u;
            self.data[u].1 += self.data[v].1;
        }
    }
}
