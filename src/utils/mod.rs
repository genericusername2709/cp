use std::ops::{Div, Mul, Rem};

pub mod bittree;
pub mod comb;
pub mod dsu;
pub mod modulus;
pub mod primes;
pub mod segment;

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
