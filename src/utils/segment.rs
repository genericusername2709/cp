use std::fmt::Debug;

#[derive(Default)]
pub struct LazySegmentTree<T, U, F, M, C> {
    n: usize,
    tree: Vec<T>,
    lazy: Vec<U>,
    identity_t: T,
    identity_u: U,
    op: F,
    mapping: M,
    composition: C,
}

#[allow(dead_code)]
impl<T, U, F, M, C> LazySegmentTree<T, U, F, M, C>
where
    T: Clone + Default + Debug,
    U: Clone + PartialEq,
    F: Fn(&T, &T) -> T,
    M: Fn(&T, &U, usize) -> T,
    C: Fn(&U, &U) -> U,
{
    pub fn new(arr: &[T], identity_t: T, identity_u: U, op: F, mapping: M, composition: C) -> Self {
        let n: usize = arr.len().next_power_of_two();
        let tree = vec![identity_t.clone(); n << 1];
        let lazy = vec![identity_u.clone(); n << 1];

        let mut st = Self {
            n,
            tree,
            lazy,
            identity_t,
            identity_u,
            op,
            mapping,
            composition,
        };

        if n > 0 {
            st.build(arr, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, arr: &[T], node: usize, l: usize, r: usize) {
        if l == r {
            self.tree[node] = if l < arr.len() {
                arr[l].clone()
            } else {
                T::default()
            };
            return;
        }
        let mid = l + (r - l) / 2;
        self.build(arr, 2 * node, l, mid);
        self.build(arr, 2 * node + 1, mid + 1, r);
        self.tree[node] = (self.op)(&self.tree[2 * node], &self.tree[2 * node + 1]);
    }

    fn push(&mut self, node: usize, l: usize, r: usize) {
        if l != r && self.lazy[node] != self.identity_u {
            let mid = l + (r - l) / 2;
            let left_len = mid - l + 1;
            let right_len = r - mid;
            let tag = self.lazy[node].clone();

            // Apply tag to left child
            self.tree[2 * node] = (self.mapping)(&self.tree[2 * node], &tag, left_len);
            self.lazy[2 * node] = (self.composition)(&self.lazy[2 * node], &tag);

            // Apply tag to right child
            self.tree[2 * node + 1] = (self.mapping)(&self.tree[2 * node + 1], &tag, right_len);
            self.lazy[2 * node + 1] = (self.composition)(&self.lazy[2 * node + 1], &tag);

            // Clear current lazy tag
            self.lazy[node] = self.identity_u.clone();
        }
    }

    /// Updates range [ql, qr] with value `val` in O(log N)
    pub fn update_range(&mut self, ql: usize, qr: usize, val: U) {
        if self.n > 0 && ql <= qr && ql < self.n {
            let qr = qr.min(self.n - 1);
            self.update_range_rec(1, 0, self.n - 1, ql, qr, val);
        }
    }

    fn update_range_rec(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, val: U) {
        if ql <= l && r <= qr {
            self.tree[node] = (self.mapping)(&self.tree[node], &val, r - l + 1);
            self.lazy[node] = (self.composition)(&self.lazy[node], &val);
            return;
        }

        self.push(node, l, r);
        let mid = l + (r - l) / 2;

        if ql <= mid {
            self.update_range_rec(2 * node, l, mid, ql, qr, val.clone());
        }
        if qr > mid {
            self.update_range_rec(2 * node + 1, mid + 1, r, ql, qr, val);
        }

        self.tree[node] = (self.op)(&self.tree[2 * node], &self.tree[2 * node + 1]);
    }

    /// Queries range [ql, qr] in O(log N)
    pub fn query(&mut self, ql: usize, qr: usize) -> T {
        if self.n == 0 || ql > qr || ql >= self.n {
            return self.identity_t.clone();
        }
        let qr = qr.min(self.n - 1);
        self.query_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_rec(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> T {
        if ql <= l && r <= qr {
            return self.tree[node].clone();
        }

        self.push(node, l, r);
        let mid = l + (r - l) / 2;

        if qr <= mid {
            self.query_rec(2 * node, l, mid, ql, qr)
        } else if ql > mid {
            self.query_rec(2 * node + 1, mid + 1, r, ql, qr)
        } else {
            let left = self.query_rec(2 * node, l, mid, ql, qr);
            let right = self.query_rec(2 * node + 1, mid + 1, r, ql, qr);
            (self.op)(&left, &right)
        }
    }
}
