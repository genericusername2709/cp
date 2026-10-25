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
