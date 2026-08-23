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
