#![allow(unused_imports)]

mod reader {
    use std::io::{stdin, BufRead, BufReader, Stdin};

    #[derive(Default)]
    pub struct Reader<R> {
        reader: R,
        buffer: Vec<u8>,
        cursor: usize,
    }

    impl Reader<BufReader<Stdin>> {
        pub fn new() -> Self {
            Self {
                reader: BufReader::with_capacity(1 << 20, stdin()), // 1MB buffer
                buffer: Vec::new(),
                cursor: 0,
            }
        }
    }

    #[allow(unused)]
    impl<R: BufRead> Reader<R> {
        #[inline]
        fn next_token_slice(&mut self) -> Option<(usize, usize)> {
            loop {
                while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
                    self.cursor += 1;
                }

                if self.cursor < self.buffer.len() {
                    let start = self.cursor;
                    while self.cursor < self.buffer.len() && self.buffer[self.cursor] > b' ' {
                        self.cursor += 1;
                    }
                    return Some((start, self.cursor));
                }

                self.buffer.clear();
                self.cursor = 0;
                if self.reader.read_until(b'\n', &mut self.buffer).unwrap() == 0 {
                    return None;
                }
            }
        }

        /// Reads the next single token
        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            let (start, end) = self.next_token_slice().expect("Unexpected end of input");
            let token_str = std::str::from_utf8(&self.buffer[start..end]).unwrap();
            token_str.parse().ok().expect("Failed to parse!")
        }

        pub fn read_chars(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        /// Reads the next integer token
        pub fn next_int<T: FastParsable>(&mut self) -> T {
            let (start, end) = self.next_token_slice().expect("Unexpected end of input");
            FastParsable::parse_bytes(&self.buffer[start..end]).expect("Failed to parse integer!")
        }

        #[inline]
        fn fill_line_if_empty(&mut self) -> bool {
            while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
                self.cursor += 1;
            }

            if self.cursor >= self.buffer.len() {
                self.buffer.clear();
                self.cursor = 0;
                if self.reader.read_until(b'\n', &mut self.buffer).unwrap() == 0 {
                    return false; // EOF
                }

                while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
                    self.cursor += 1;
                }
            }
            self.cursor < self.buffer.len()
        }

        #[inline]
        fn next_line_token_slice(&mut self) -> Option<(usize, usize)> {
            // Skip whitespace on the current line
            while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
                self.cursor += 1;
            }

            if self.cursor >= self.buffer.len() {
                return None;
            }

            let start = self.cursor;
            while self.cursor < self.buffer.len() && self.buffer[self.cursor] > b' ' {
                self.cursor += 1;
            }

            Some((start, self.cursor))
        }

        /// Fast reading of a full array line
        pub fn read_arr<T: std::str::FromStr>(&mut self) -> Vec<T> {
            let mut arr = Vec::new();
            if !self.fill_line_if_empty() {
                return arr;
            }

            while let Some((start, end)) = self.next_line_token_slice() {
                let token_str = std::str::from_utf8(&self.buffer[start..end]).unwrap();
                arr.push(token_str.parse().ok().expect("Failed to parse!"));
            }

            arr
        }

        /// Fast reading of a full array line
        pub fn read_vec<T: std::str::FromStr>(&mut self, len: usize) -> Vec<T> {
            let mut arr = Vec::with_capacity(len);
            if !self.fill_line_if_empty() {
                return arr;
            }

            while let Some((start, end)) = self.next_line_token_slice() {
                let token_str = std::str::from_utf8(&self.buffer[start..end]).unwrap();
                arr.push(token_str.parse().ok().expect("Failed to parse!"));
            }

            arr
        }

        /// Fast reading of a full array line
        pub fn read_arr_int<T: FastParsable>(&mut self) -> Vec<T> {
            let mut arr = Vec::new();
            if !self.fill_line_if_empty() {
                return arr;
            }

            while let Some((start, end)) = self.next_line_token_slice() {
                let val = FastParsable::parse_bytes(&self.buffer[start..end])
                    .expect("Failed to parse integer!");
                arr.push(val);
            }

            arr
        }

        /// Fast reading of a full array line
        pub fn read_vec_int<T: FastParsable>(&mut self, len: usize) -> Vec<T> {
            let mut arr = Vec::with_capacity(len);
            if !self.fill_line_if_empty() {
                return arr;
            }

            while let Some((start, end)) = self.next_line_token_slice() {
                let val = FastParsable::parse_bytes(&self.buffer[start..end])
                    .expect("Failed to parse integer!");
                arr.push(val);
            }

            arr
        }
    }

    pub trait FastParsable: Sized {
        fn parse_bytes(bytes: &[u8]) -> Option<Self>;
    }

    // Single macro generating the byte-parsing loop for any unsigned type
    macro_rules! impl_fast_parse_unsigned {
        ($($t:ty),*) => {
            $(
                impl FastParsable for $t {
                    #[inline]
                    fn parse_bytes(bytes: &[u8]) -> Option<Self> {
                        if bytes.is_empty() { return None; }
                        let mut val: $t = 0;
                        for &b in bytes {
                            if b < b'0' || b > b'9' { return None; }
                            val = val * 10 + (b - b'0') as $t;
                        }
                        Some(val)
                    }
                }
            )*
        };
    }

    // Single macro generating the byte-parsing loop for any signed type
    macro_rules! impl_fast_parse_signed {
        ($($t:ty),*) => {
            $(
                impl FastParsable for $t {
                    #[inline]
                    fn parse_bytes(bytes: &[u8]) -> Option<Self> {
                        if bytes.is_empty() { return None; }
                        let (neg, rest) = if bytes[0] == b'-' {
                            (true, &bytes[1..])
                        } else {
                            (false, bytes)
                        };
                        let mut val: $t = 0;
                        for &b in rest {
                            if b < b'0' || b > b'9' { return None; }
                            val = val * 10 + (b - b'0') as $t;
                        }
                        Some(if neg { -val } else { val })
                    }
                }
            )*
        };
    }

    // Apply the implementation to all desired types in a single line each
    impl_fast_parse_unsigned!(usize, u8, u16, u32, u64, u128);
    impl_fast_parse_signed!(isize, i8, i16, i32, i64, i128);

    #[macro_export]
    macro_rules! parr {
        ($writer: expr, $arr: expr) => {
            for i in 0..$arr.len() {
                write!(
                    $writer,
                    "{}{}",
                    $arr[i],
                    if i == $arr.len() { " " } else { "\n" }
                );
            }
        };
    }

    #[macro_export]
    macro_rules! pen {
        ($writer: expr, $elem: expr) => {
            write!($writer, "{}\n", $elem);
        };
    }

    #[macro_export]
    macro_rules! pes {
        ($writer: expr, $elem: expr) => {
            write!($writer, "{} ", $elem);
        };
    }
}
mod utils {
    use std::ops::{Div, Mul, Rem};

    pub mod comb {
        use crate::utils::modulus;

        #[derive(Default)]
        pub struct Comb<const MOD: i64> {
            factorials: Vec<modulus::Mod<MOD>>,
            inverse_factorials: Vec<modulus::Mod<MOD>>,
        }

        #[allow(dead_code)]
        impl<const MOD: i64> Comb<MOD> {
            pub fn populate_factorials(&mut self, till: usize) {
                self.factorials = vec![modulus::Mod::new(1); till];
                self.inverse_factorials = vec![modulus::Mod::new(1); till];

                for i in 2..till {
                    self.factorials[i] = self.factorials[i - 1] * modulus::Mod::new(i as i64);
                    self.inverse_factorials[i] = modulus::Mod::inverse(self.factorials[i]);
                }
            }

            pub fn fac(&mut self, n: usize) -> modulus::Mod<MOD> {
                if self.factorials.len() == 0 {
                    self.populate_factorials(n);
                }

                self.factorials[n]
            }

            pub fn c(&mut self, n: usize, r: usize) -> modulus::Mod<MOD> {
                if n < r {
                    return modulus::Mod::new(0);
                }

                if self.factorials.len() == 0 {
                    self.populate_factorials(n);
                }

                self.factorials[n] * self.inverse_factorials[n - r] * self.inverse_factorials[r]
            }
        }
    }
    pub mod dsu {
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
                if num != 1 {
                    prime_factors.push(num);
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
        pub preprocdata: Vec<Vec<i64>>,
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
    // 2. Range Set (Assignment) + Range Maximum
    // =========================================================================

    pub type RangeSetMaxTree = LazySegTreeFn<i64, Option<i64>>;

    #[allow(dead_code)]
    impl RangeSetMaxTree {
        pub fn new_set_max(arr: &[i64]) -> Self {
            Self::new(
                arr,
                i64::MIN,                      // Identity T
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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::convert::TryInto;
use std::io::{stdout, BufReader, BufWriter, Stdin, Stdout, Write};

use utils::dsu::DisjointSetUnion;
use utils::modulus::Mod;
use utils::{
    gcd, lcm, LazySegTreeFn, PointUpdateTree, RangeAddSumTree, RangeGcdTree, RangeMaxTree,
    RangeMinTree, RangeSetMaxTree, RangeSetMinTree, Utils,
};

// const MOD: i64 = 1_000_000_007;
const MOD: i64 = 998_244_353;

// const ARR_LIMIT: usize = 1 << 18;
// const ARR_LIMIT: usize = 1_000_005;
const ARR_LIMIT: usize = 300_005;

#[allow(unused)]
fn solve(
    t: usize,
    r: &mut Reader<BufReader<Stdin>>,
    w: &mut BufWriter<Stdout>,
    utils: &mut Utils<MOD>,
) {
    let n: usize = r.next_int();
    let q: usize = r.next_int();
    let mut a: Vec<u8> = r.read_vec_int(n);
    let nons: Vec<u8> = vec![1, 2, 4, 7, 8, 11, 13, 14];
    let mut c = 0;
    for i in 0..n {
        for j in nons.iter() {
            if a[i] == *j {
                c += 1;
            }
        }
    }
    pes!(w, n - c);
    for i in 0..q {
        let qi = r.next_int::<usize>() - 1;
        let qv: u8 = r.next_int();
        let mut p = 0;
        let mut cr = 0;
        for j in nons.iter() {
            if a[qi] == *j {
                p = 1;
            }
            if qv == *j {
                cr = 1;
            }
        }
        a[qi] = qv;
        c = c - p + cr;

        if i == q - 1 {
            pen!(w, n - c);
        } else {
            pes!(w, n - c);
        }
    }
}

#[allow(unused)]
fn preproc(utils: &mut Utils<MOD>) {
    // utils.comb.populate_factorials(ARR_LIMIT);
    // utils.prime.init_prime_seive(ARR_LIMIT);
    // utils.preprocdata.push(vec![0]);
}

#[allow(dead_code)]
fn main() {
    let mut reader: Reader<BufReader<Stdin>> = Reader::new();
    let writer = &mut BufWriter::new(stdout());
    let mut utils = Utils::default();
    preproc(&mut utils);
    let t: usize = reader.next_int();
    for i in 1..=t {
        solve(i, &mut reader, writer, &mut utils);
    }
}
