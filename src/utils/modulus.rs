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
