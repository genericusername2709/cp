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
