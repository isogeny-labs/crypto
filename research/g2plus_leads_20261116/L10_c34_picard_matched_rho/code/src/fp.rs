//! Counted F_p layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cnt {
    pub mul: u64,
    pub inv: u64,
}
impl Cnt {
    pub fn fpm(&self) -> u64 {
        fpm(self)
    }
}
pub fn fpm(c: &Cnt) -> u64 {
    c.mul + 10 * c.inv
}

#[derive(Clone, Copy, Debug)]
pub struct Fp {
    pub p: u64,
}
impl Fp {
    pub fn new(p: u64) -> Fp {
        Fp { p }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64, c: &mut Cnt) -> u64 {
        c.mul += 1;
        a * b % self.p
    }
    #[inline]
    pub fn mul_u(&self, a: u64, b: u64) -> u64 {
        a * b % self.p
    }
    pub fn inv(&self, a: u64, c: &mut Cnt) -> u64 {
        c.inv += 1;
        self.inv_u(a)
    }
    pub fn inv_u(&self, a: u64) -> u64 {
        assert!(a % self.p != 0, "inverse of zero");
        let (mut r0, mut r1) = (self.p as i64, (a % self.p) as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            let r2 = r0 - q * r1;
            r0 = r1;
            r1 = r2;
            let t2 = t0 - q * t1;
            t0 = t1;
            t1 = t2;
        }
        assert_eq!(r0, 1);
        (((t0 % self.p as i64) + self.p as i64) % self.p as i64) as u64
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    pub fn pow_u(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        a %= self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = r * a % self.p;
            }
            a = a * a % self.p;
            e >>= 1;
        }
        r
    }
}
