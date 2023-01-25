use num_traits::{
    identities::{One, Zero},
    Num,
};
use std::convert::{From, Into};
use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModInt<const MOD: usize> {
    val: usize,
}

impl<const MOD: usize> Num for ModInt<MOD> {
    type FromStrRadixErr = <usize as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let val = usize::from_str_radix(str, radix)?;
        Ok(Self { val })
    }
}

impl<const MOD: usize> fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const MOD: usize, I> From<I> for ModInt<MOD>
where
    I: Into<usize>,
{
    fn from(n: I) -> Self {
        Self {
            val: n.into() % MOD,
        }
    }
}

impl<const MOD: usize> ModInt<MOD> {
    pub fn new(n: usize) -> Self {
        Self { val: n % MOD }
    }

    pub fn val(&self) -> usize {
        // 念のためMOD演算
        self.val % MOD
    }

    pub fn _set_val(&mut self, val: usize) {
        self.val = val % MOD;
    }

    pub fn pow_u(&self, mut n: usize) -> Self {
        let mut val = self.val;
        let mut res: usize = 1;
        while n > 0 {
            if n % 2 == 1 {
                res = (res * val) % MOD;
            }
            val = (val * val) % MOD;
            n /= 2;
        }

        Self { val: res }
    }

    pub fn pow(&self, other: Self) -> Self {
        self.pow_u(other.val)
    }

    pub fn inv(&self) -> Self {
        self.pow_u(MOD - 2)
    }
}

impl<const MOD: usize> ops::Add for ModInt<MOD> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: (self.val + other.val) % MOD,
        }
    }
}

impl<const MOD: usize> ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            val: (self.val + other.val) % MOD,
        };
    }
}

impl<const MOD: usize> ops::Mul for ModInt<MOD> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            val: (self.val * other.val) % MOD,
        }
    }
}

impl<const MOD: usize> ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            val: (self.val * other.val) % MOD,
        };
    }
}

impl<const MOD: usize> ops::Sub for ModInt<MOD> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        if self.val < other.val {
            self.val += MOD;
        }
        Self {
            val: self.val - other.val % MOD,
        }
    }
}

impl<const MOD: usize> ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, other: Self) {
        if self.val < other.val {
            self.val += MOD;
        }
        *self = Self {
            val: (self.val - other.val) % MOD,
        };
    }
}

impl<const MOD: usize> ops::Div for ModInt<MOD> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.val == 0 {
            panic!("0 division occured.");
        }

        self * other.inv()
    }
}

impl<const MOD: usize> ops::DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, other: Self) {
        if other.val == 0 {
            panic!("0 division occured.");
        }

        *self *= other.inv();
    }
}

impl<const MOD: usize> ops::Rem for ModInt<MOD> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            val: (self.val % other.val) % MOD, // 念のためMOD演算
        }
    }
}

impl<const MOD: usize> ops::RemAssign for ModInt<MOD> {
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            val: (self.val % other.val) % MOD, // 念のためMOD演算
        };
    }
}

impl<const MOD: usize> Zero for ModInt<MOD> {
    fn zero() -> Self {
        Self { val: 0 }
    }

    fn is_zero(&self) -> bool {
        self.val == 0
    }

    fn set_zero(&mut self) {
        self.val = 0;
    }
}

impl<const MOD: usize> One for ModInt<MOD> {
    fn one() -> Self {
        Self { val: 1 }
    }

    fn is_one(&self) -> bool {
        self.val == 1
    }

    fn set_one(&mut self) {
        self.val = 1;
    }
}

pub struct ModCom<const MOD: usize> {
    fac: Vec<usize>,
    finv: Vec<usize>,
}

impl<const MOD: usize> ModCom<MOD> {
    pub fn new(cap: usize) -> Self {
        let mut fac = vec![0; cap];
        let mut finv = vec![0; cap];
        let mut inv = vec![0; cap];
        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..cap {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        Self { fac, finv }
    }

    pub fn com(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % MOD) % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type MINT = ModInt<1_000_000_007>;

    #[test]
    fn test1() {
        let a = MINT::new(111);
        let b = MINT::new(222);
        let c = MINT::new(333);
        let d = MINT::new(444);

        let res = a * b + c - d;
        assert_eq!(res.val(), 24531);
    }

    #[test]
    fn test2() {
        let a = MINT::new(111111111);
        let b = MINT::new(222222222);
        let c = MINT::new(333333333);
        let d = MINT::new(444444444);

        let res = a * b + c - d;
        assert_eq!(res.val(), 691358032);
    }
}
