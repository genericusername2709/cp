#![allow(unused_imports)]

mod reader {
    #[derive(Default)]
    pub struct Reader {
        buffer: Vec<String>,
    }

    #[allow(dead_code)]
    impl Reader {
        // Reads next token from stdin - either from the current line - that is present in the #buffer
        // or from reading new a line.
        //
        // It panics if the expected type of token is not present.
        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            loop {
                if let Some(token) = self.buffer.pop() {
                    return token.parse().ok().expect("Failed to parse!");
                }
                self.fill_buff();
            }
        }

        pub fn read_chars(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        // Reads and stores elems into a array, until first non-empty line ends.
        pub fn read_arr<T: std::str::FromStr>(&mut self) -> Vec<T> {
            let mut len = self.buffer.len();
            while len == 0 {
                self.fill_buff();
                len = self.buffer.len();
            }
            self.buffer
                .drain(..)
                .map(|e| e.parse().ok().expect("Failed to parse!"))
                .rev()
                .collect()
        }

        // Reads line, and puts it into the buffer
        fn fill_buff(&mut self) {
            let input = self.read_line();
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }

        pub fn read_line(&self) -> String {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
        }
    }
}
mod utils {
    use std::ops::{Div, Mul, Rem};

    pub mod bittree {
        use crate::ARR_LIMIT;
        use std::ops::{Add, AddAssign, Sub, SubAssign};

        // Fenwick Tree (1-indexed)
        #[derive(Default)]
        pub struct BitTree<T>
        where
            T: Default + Copy + PartialEq + Add + Sub + AddAssign + SubAssign,
        {
            data: Vec<T>,
        }

        #[allow(dead_code)]
        impl<T> BitTree<T>
        where
            T: Default + Copy + PartialEq + Add + Sub + AddAssign + SubAssign,
        {
            pub fn new(size: usize) -> Self {
                assert!(
                    size <= ARR_LIMIT,
                    "Array size too big! Expected max size: {}, got {}",
                    ARR_LIMIT,
                    size
                );

                Self {
                    data: vec![T::default(); size + 1],
                }
            }

            pub fn init(arr: Vec<T>) -> Self {
                let mut tree = Self::new(arr.len());
                for i in 0..arr.len() {
                    tree.update(i + 1, arr[i]);
                }

                tree
            }

            // 1-based indexing
            pub fn update(&mut self, index: usize, val: T) {
                let mut current = index;
                while current < self.data.len() {
                    self.data[current] += val;
                    current += current & current.wrapping_neg();
                }
            }

            // 1-based indexing
            pub fn query(&self, lower: usize, upper: usize) -> T {
                assert!(
                        lower <= upper && upper <= self.data.len(),
                        "Invalid Range or Range out-of-bounds, Got range [{} , {}], for Fenwick tree of size: {}",
                        lower, upper, self.data.len()
                    );

                let mut result: T = T::default();
                let mut current = upper;
                while current > 0 {
                    result += self.data[current];
                    current -= current & current.wrapping_neg();
                }
                current = lower;
                while current > 0 {
                    result -= self.data[current];
                    current -= current & current.wrapping_neg();
                }

                result
            }
        }
    }
    pub mod comb {
        use crate::utils::modulus;
        use crate::ARR_LIMIT;

        #[derive(Default)]
        pub struct Comb<const MOD: i64> {
            factorials: Vec<modulus::Mod<MOD>>,
            inverse_factorials: Vec<modulus::Mod<MOD>>,
        }

        #[allow(dead_code)]
        impl<const MOD: i64> Comb<MOD> {
            pub fn populate_factorials(&mut self, till: usize) {
                assert!(
                    till <= ARR_LIMIT,
                    "Array size too big! Expected max size: {}, got {}",
                    ARR_LIMIT,
                    till
                );

                self.factorials = vec![modulus::Mod::new(1); till];
                self.inverse_factorials = vec![modulus::Mod::new(1); till];

                for i in 2..till {
                    self.factorials[i] = self.factorials[i - 1] * modulus::Mod::new(i as i64);
                    self.inverse_factorials[i] = modulus::Mod::inverse(self.factorials[i]);
                }
            }

            pub fn c(&mut self, n: usize, r: usize) -> modulus::Mod<MOD> {
                assert!(
                    n <= ARR_LIMIT,
                    "Array size too big! Expected max size: {}, got {}",
                    ARR_LIMIT,
                    n
                );

                if n < r {
                    return modulus::Mod::new(0);
                }

                if self.factorials.len() == 0 {
                    self.populate_factorials(ARR_LIMIT);
                }

                self.factorials[n] * self.inverse_factorials[n - r] * self.inverse_factorials[r]
            }
        }
    }
    pub mod dsu {
        use crate::ARR_LIMIT;
        use std::mem::swap;

        #[derive(Default)]
        pub struct DisjointSetUnion {
            data: Vec<(usize, usize)>,
        }

        #[allow(dead_code)]
        impl DisjointSetUnion {
            pub fn new(size: usize) -> Self {
                assert!(
                    size <= ARR_LIMIT,
                    "Array size too big! Expected max size: {}, got {}",
                    ARR_LIMIT,
                    size
                );

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
    }
    pub mod modulus {
        use std::cmp::Ordering;
        use std::mem::swap;
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

        #[allow(dead_code)]
        #[derive(Debug, Clone, Copy)]
        pub struct Mod<const MOD: i64> {
            pub val: i64,
        }

        #[allow(dead_code)]
        impl<const MOD: i64> Mod<MOD> {
            pub fn new(x: i64) -> Self {
                let mut val: i64 = x % MOD;
                if val < 0 {
                    val += MOD;
                }
                Self { val }
            }

            pub fn inverse(self) -> Self {
                Mod::new(1) / self
            }

            pub fn inv_with_diff_mod(v: i64, n: i64) -> i64 {
                let (mut a, mut b, mut c): (i64, i64, i64) = (0, 1, v);
                let mut m: i64 = n;
                while c != 0 {
                    let t: i64 = m / c;
                    m -= t * c;
                    swap(&mut c, &mut m);
                    a -= t * b;
                    swap(&mut a, &mut b);
                }
                a %= n;
                if a < 0 {
                    a += n;
                }
                a % n
            }

            pub fn power(self, pow: i64) -> Self {
                let mut result: Self = Self::new(1);
                let mut base = self;
                let mut power = pow;
                while power > 0 {
                    if power & 1 == 1 {
                        result *= base;
                    }
                    base *= base;
                    power >>= 1;
                }
                result
            }
        }

        impl<const MOD: i64> Add for Mod<MOD> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = Mod::new(self.val);
                lhs += rhs;
                lhs
            }
        }

        impl<const MOD: i64> Sub for Mod<MOD> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = Mod::new(self.val);
                lhs -= rhs;
                lhs
            }
        }

        impl<const MOD: i64> Mul for Mod<MOD> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = Mod::new(self.val);
                lhs *= rhs;
                lhs
            }
        }

        impl<const MOD: i64> Div for Mod<MOD> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = Mod::new(self.val);
                lhs /= rhs;
                lhs
            }
        }

        impl<const MOD: i64> AddAssign for Mod<MOD> {
            fn add_assign(&mut self, rhs: Self) {
                self.val += rhs.val;
                if self.val >= MOD {
                    self.val -= MOD;
                }
            }
        }

        impl<const MOD: i64> SubAssign for Mod<MOD> {
            fn sub_assign(&mut self, rhs: Self) {
                self.val -= rhs.val;
                if self.val < 0 {
                    self.val += MOD;
                }
            }
        }

        impl<const MOD: i64> MulAssign for Mod<MOD> {
            fn mul_assign(&mut self, rhs: Self) {
                self.val = (self.val * rhs.val) % MOD;
            }
        }

        impl<const MOD: i64> DivAssign for Mod<MOD> {
            fn div_assign(&mut self, rhs: Self) {
                let (mut a, mut b, mut c): (i64, i64, i64) = (0, 1, rhs.val);
                let mut m: i64 = MOD;
                while c != 0 {
                    let t: i64 = m / c;
                    m -= t * c;
                    swap(&mut c, &mut m);
                    a -= t * b;
                    swap(&mut a, &mut b);
                }
                a %= MOD;
                if a < 0 {
                    a += MOD;
                }
                self.val = (self.val * a) % MOD;
            }
        }

        impl<const MOD: i64> Neg for Mod<MOD> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Mod::new(-self.val)
            }
        }

        impl<const MOD: i64> PartialEq for Mod<MOD> {
            fn eq(&self, other: &Self) -> bool {
                self.val == other.val
            }
        }

        impl<const MOD: i64> PartialOrd for Mod<MOD> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.val.partial_cmp(&other.val)
            }

            fn lt(&self, other: &Self) -> bool {
                self.val.lt(&other.val)
            }

            fn le(&self, other: &Self) -> bool {
                self.val.le(&other.val)
            }

            fn gt(&self, other: &Self) -> bool {
                self.val.gt(&other.val)
            }

            fn ge(&self, other: &Self) -> bool {
                self.val.ge(&other.val)
            }
        }
    }
    pub mod primes {
        use crate::ARR_LIMIT;

        #[derive(Default)]
        pub struct Prime {
            prime_seive: Vec<usize>,
        }

        #[allow(dead_code)]
        impl Prime {
            pub fn init_prime_seive(&mut self, till: usize) {
                assert!(
                    till <= ARR_LIMIT,
                    "Array size too big! Expected max size: {}, got {}",
                    ARR_LIMIT,
                    till
                );
                self.prime_seive = Vec::with_capacity(till + 1);
                self.prime_seive.push(0); // 0
                for _ in 0..till {
                    self.prime_seive.push(1);
                }
                for i in 2..till {
                    if self.prime_seive[i] == 1 {
                        let mut j = i << 1;
                        while j <= till {
                            if self.prime_seive[j] == 1 {
                                self.prime_seive[j] = i;
                            }
                            j += i;
                        }
                    }
                }
            }

            pub fn get_prime_factors(&mut self, n: usize) -> Vec<usize> {
                if self.prime_seive.len() == 0 {
                    self.init_prime_seive(ARR_LIMIT);
                }
                let mut prime_factors: Vec<usize> = Vec::new();
                let mut num = n;
                while self.prime_seive[num] != 1 {
                    prime_factors.push(self.prime_seive[num]);
                    num = num / self.prime_seive[num];
                }

                prime_factors
            }

            pub fn get_prime_factors_without_seive(mut n: i64) -> Vec<i64> {
                let mut factors = Vec::new();
                if n <= 1 {
                    return factors;
                }
                while n % 2 == 0 {
                    factors.push(2);
                    n /= 2;
                }
                let mut d = 3;
                while d * d <= n {
                    if n % d == 0 {
                        while n % d == 0 {
                            factors.push(d);
                            n /= d;
                        }
                    }
                    d += 2;
                }
                if n > 1 {
                    factors.push(n);
                }
                factors
            }

            fn get_uniques<T>(sorted_arr: &Vec<T>) -> Vec<(T, usize)>
            where
                T: Default + Copy + PartialEq,
            {
                let mut curr: T = T::default();
                let mut sorted_result: Vec<(T, usize)> = Vec::new();
                for &e in sorted_arr.iter() {
                    if curr != e {
                        sorted_result.push((e, 1));
                    } else {
                        sorted_result.last_mut().expect("Element not found").1 += 1;
                    }
                    curr = e;
                }
                sorted_result
            }

            pub fn get_unique_prime_factors(&mut self, n: usize) -> Vec<(usize, usize)> {
                let factors = self.get_prime_factors(n);
                Self::get_uniques(&factors)
            }

            pub fn get_unique_prime_factors_without_sieve(n: i64) -> Vec<(i64, usize)> {
                let factors = Self::get_prime_factors_without_seive(n);
                Self::get_uniques(&factors)
            }
        }
    }
    pub mod segment {
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
            T: Clone,
            U: Clone + PartialEq,
            F: Fn(&T, &T) -> T,
            M: Fn(&T, &U, usize) -> T,
            C: Fn(&U, &U) -> U,
        {
            pub fn new(
                arr: &[T],
                identity_t: T,
                identity_u: U,
                op: F,
                mapping: M,
                composition: C,
            ) -> Self {
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
                    self.tree[node] = arr[l].clone();
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
                    self.tree[2 * node + 1] =
                        (self.mapping)(&self.tree[2 * node + 1], &tag, right_len);
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

            fn update_range_rec(
                &mut self,
                node: usize,
                l: usize,
                r: usize,
                ql: usize,
                qr: usize,
                val: U,
            ) {
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
    }

    #[allow(unused)]
    #[derive(Default)]
    pub struct Utils<const MOD: i64> {
        pub prime: primes::Prime,
        pub comb: comb::Comb<MOD>,
    }

    #[allow(dead_code)]
    pub fn gcd<T>(a: T, b: T) -> T
    where
        T: Default + Copy + PartialEq + PartialOrd + Rem<Output = T>,
    {
        return if b == T::default() { a } else { gcd(b, a % b) };
    }

    #[allow(dead_code)]
    pub fn lcm<T>(a: T, b: T) -> T
    where
        T: Default
            + Copy
            + PartialEq
            + PartialOrd
            + Rem<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    {
        (a * b) / gcd(a, b)
    }

    #[allow(dead_code)]
    pub type LazySegTreeFn<T, U> =
        segment::LazySegmentTree<T, U, fn(&T, &T) -> T, fn(&T, &U, usize) -> T, fn(&U, &U) -> U>;

    // =========================================================================
    // 1. Range Add + Range Sum
    // =========================================================================
    pub type RangeAddSumTree = LazySegTreeFn<i64, i64>;

    #[allow(dead_code)]
    impl RangeAddSumTree {
        pub fn new_add_sum(arr: &[i64]) -> Self {
            Self::new(
                arr,
                0,                                  // Identity T
                0,                                  // Identity U
                |a, b| a + b,                       // Op
                |v, tag, len| v + tag * len as i64, // Mapping
                |old, new| old + new,               // Composition
            )
        }
    }

    // =========================================================================
    // 2. Range Set (Assignment) + Range Minimum
    // =========================================================================

    pub type RangeSetMinTree = LazySegTreeFn<i64, Option<i64>>;

    #[allow(dead_code)]
    impl RangeSetMinTree {
        pub fn new_set_min(arr: &[i64]) -> Self {
            Self::new(
                arr,
                i64::MAX,                      // Identity T
                None,                          // Identity U
                |a, b| *a.min(b),              // Op
                |v, tag, _| tag.unwrap_or(*v), // Mapping
                |_, new| *new,                 // Composition
            )
        }
    }

    // =========================================================================
    // 3. Point-Update / Range-Query Variants (Using dummy unit lazy tag `()`)
    // =========================================================================
    pub type PointUpdateTree<T> = LazySegTreeFn<T, ()>;

    // Range Minimum Query (RMQ)
    pub type RangeMinTree = PointUpdateTree<i64>;

    #[allow(dead_code)]
    impl RangeMinTree {
        pub fn new_min(arr: &[i64]) -> Self {
            Self::new(arr, i64::MAX, (), |a, b| *a.min(b), |v, _, _| *v, |_, _| ())
        }
    }

    // Range Maximum Query (RMAQ)
    pub type RangeMaxTree = PointUpdateTree<i64>;

    #[allow(dead_code)]
    impl RangeMaxTree {
        pub fn new_max(arr: &[i64]) -> Self {
            Self::new(arr, i64::MIN, (), |a, b| *a.max(b), |v, _, _| *v, |_, _| ())
        }
    }

    // Range GCD Query
    pub type RangeGcdTree = PointUpdateTree<i64>;

    #[allow(dead_code)]
    impl RangeGcdTree {
        pub fn new_gcd(arr: &[i64]) -> Self {
            Self::new(arr, 0, (), |a, b| gcd(*a, *b), |v, _, _| *v, |_, _| ())
        }
    }
}

use reader::Reader;
use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::io::{stdout, BufWriter, Stdout, Write};
use utils::modulus::Mod;
use utils::Utils;

// const MOD: i64 = 1_000_000_007;
const MOD: i64 = 998_244_353;
// const ARR_LIMIT: usize = 1 << 18;
const ARR_LIMIT: usize = 1_000_005;

#[allow(unused)]
fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>, utils: &mut Utils<MOD>) {}

#[allow(dead_code)]
fn main() {
    let mut reader: Reader = Reader::default();
    let writer = &mut BufWriter::new(stdout());
    let mut utils = Utils::default();
    let t: usize = reader.next(); // Read the number of test case
    for _ in 0..t {
        solve(&mut reader, writer, &mut utils);
    }
}
