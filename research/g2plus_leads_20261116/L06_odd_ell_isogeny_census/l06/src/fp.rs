pub struct Fp {
    pub p: u64,
    pub inv: Vec<u32>,
    pub leg: Vec<i8>,
    pub nu: u64,
}

impl Fp {
    pub fn new(p: u64) -> Fp {
        assert!(p >= 2 && p < (1 << 13));
        let mut d = 2;
        while d * d <= p {
            assert!(p % d != 0, "p must be prime");
            d += 1;
        }
        let mut fp = Fp { p, inv: vec![0; p as usize], leg: vec![0; p as usize], nu: 0 };
        for a in 1..p {
            fp.inv[a as usize] = fp.pow(a, p.wrapping_sub(2)) as u32;
        }
        for y in 1..p {
            fp.leg[(y * y % p) as usize] = 1;
        }
        for a in 1..p {
            if fp.leg[a as usize] == 0 {
                fp.leg[a as usize] = -1;
            }
        }
        for a in 2..p {
            if fp.leg[a as usize] == -1 {
                fp.nu = a;
                break;
            }
        }
        fp
    }
    pub fn pow(&self, mut b: u64, mut e: u64) -> u64 {
        let p = self.p;
        let mut r = 1 % p;
        b %= p;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % p;
            }
            b = b * b % p;
            e >>= 1;
        }
        r
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.p
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        a * b % self.p
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        (self.p - a) % self.p
    }
    #[inline]
    pub fn inv(&self, a: u64) -> u64 {
        assert!(a % self.p != 0);
        self.inv[a as usize] as u64
    }
    #[inline]
    pub fn legendre(&self, a: u64) -> i8 {
        self.leg[a as usize]
    }
    pub fn legendre_pow(&self, a: u64) -> i8 {
        if a == 0 {
            return 0;
        }
        if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    /// Tonelli-Shanks with the smallest non-residue as the generator.
    pub fn sqrt(&self, a: u64) -> u64 {
        let p = self.p;
        if a == 0 {
            return 0;
        }
        assert!(p % 2 == 1 && self.leg[a as usize] == 1);
        let mut q = p - 1;
        let mut s = 0;
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut m = s;
        let mut c = self.pow(self.nu, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, (q + 1) / 2);
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = tt * tt % p;
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = b * b % p;
            }
            m = i;
            c = b * b % p;
            t = t * c % p;
            r = r * b % p;
        }
        assert!(r * r % p == a);
        r
    }
}

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn powmod(mut b: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    b %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, n);
        }
        b = mulmod(b, b, n);
        e >>= 1;
    }
    r
}
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let small = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &q in &small {
        if n == q {
            return true;
        }
        if n % q == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for &a in &small {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
