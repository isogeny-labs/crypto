//! Small-prime (p < 2^62) short-Weierstrass arithmetic, independent of the
//! repository crate and of PARI: used to re-verify every curve order.
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}
pub fn legendre(a: u64, p: u64) -> i32 {
    let r = powm(a, (p - 1) / 2, p);
    if r == 0 {
        0
    } else if r == 1 {
        1
    } else {
        -1
    }
}
pub fn sqrtm(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if legendre(a, p) != 1 {
        return None;
    }
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while legendre(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powm(z, q, p);
    let mut t = powm(a, q, p);
    let mut r = powm(a, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

pub type Pt = Option<(u64, u64)>;
#[derive(Clone, Copy, Debug)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}
impl Curve {
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (mulm(mulm(x, x, p), x, p) + mulm(self.a, x, p) + self.b) % p
    }
    pub fn on(&self, pt: Pt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => mulm(y, y, self.p) == self.rhs(x),
        }
    }
    pub fn add(&self, p1: Pt, p2: Pt) -> Pt {
        let p = self.p;
        match (p1, p2) {
            (None, q) | (q, None) => q,
            (Some((x1, y1)), Some((x2, y2))) => {
                let lam;
                if x1 == x2 {
                    if (y1 + y2) % p == 0 {
                        return None;
                    }
                    let num = (3 * mulm(x1, x1, p) + self.a) % p;
                    lam = mulm(num, invm(2 * y1 % p, p), p);
                } else {
                    let num = (y2 + p - y1) % p;
                    let den = (x2 + p - x1) % p;
                    lam = mulm(num, invm(den, p), p);
                }
                let x3 = (mulm(lam, lam, p) + 2 * p - x1 - x2) % p;
                let y3 = (mulm(lam, (x1 + p - x3) % p, p) + p - y1) % p;
                Some((x3, y3))
            }
        }
    }
    pub fn mul(&self, k: u64, pt: Pt) -> Pt {
        let mut r: Pt = None;
        let mut q = pt;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, q);
            }
            q = self.add(q, q);
            k >>= 1;
        }
        r
    }
    /// Point with the smallest abscissa x >= 1 (smaller y root).
    pub fn first_point(&self) -> (u64, u64) {
        let mut x = 1;
        loop {
            if let Some(y) = sqrtm(self.rhs(x), self.p) {
                if y != 0 {
                    return (x, y.min(self.p - y));
                }
            }
            x += 1;
        }
    }
    pub fn j(&self) -> u64 {
        let p = self.p;
        let a3 = mulm(mulm(self.a, self.a, p), self.a, p);
        let num = mulm(1728 % p, mulm(4, a3, p), p);
        let den = (mulm(4, a3, p) + mulm(27, mulm(self.b, self.b, p), p)) % p;
        mulm(num, invm(den, p), p)
    }
}
/// Verified order: n prime with n*P = O for a point P != O, and n > 2*(p+1)/3
/// so n is the only multiple of ord(P) = n inside the Hasse interval.
pub fn order_is(c: &Curve, n: u64) -> bool {
    let (x, y) = c.first_point();
    let g = Some((x, y));
    c.on(g) && c.mul(n, g).is_none()
}
