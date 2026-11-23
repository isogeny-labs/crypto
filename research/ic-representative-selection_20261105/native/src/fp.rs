//! Counted arithmetic in `F_p` (`p < 2^62`), polynomials modulo a short
//! Weierstrass cubic, and affine curve arithmetic for verification.
//!
//! Every multiplication and every inversion goes through a [`Tally`], so a
//! procedure's native work is a count, not an estimate. Additions,
//! subtractions and multiplications by small integer constants (2, 3, 4, 5,
//! 7, 12) are done with additions and are not counted. The tally that charges
//! a procedure is kept separate from the tally of its uncharged verification.

/// Native field operations spent by a procedure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub muls: u64,
    pub invs: u64,
}

impl Tally {
    /// Native operations, each counted once (AGENTS.md §5a prices them at
    /// one weight `w` per operation).
    pub fn ops(&self) -> u64 {
        self.muls + self.invs
    }
    pub fn absorb(&mut self, o: &Tally) {
        self.muls += o.muls;
        self.invs += o.invs;
    }
}

/// The prime field `F_p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp {
    pub p: u64,
}

impl Fp {
    pub fn new(p: u64) -> Fp {
        assert!(p > 3 && p < (1 << 62), "p must be an odd prime below 2^62");
        Fp { p }
    }
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    /// `k·a` for a small constant `k`, by additions (not counted).
    pub fn small(&self, k: u64, a: u64) -> u64 {
        let mut r = 0;
        for _ in 0..k {
            r = self.add(r, a);
        }
        r
    }
    pub fn mul(&self, a: u64, b: u64, t: &mut Tally) -> u64 {
        t.muls += 1;
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    pub fn inv(&self, a: u64, t: &mut Tally) -> u64 {
        assert!(!a.is_multiple_of(self.p), "inverse of zero");
        t.invs += 1;
        let (mut r0, mut r1) = (self.p as i128, (a % self.p) as i128);
        let (mut s0, mut s1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (s0, s1) = (s1, s0 - q * s1);
        }
        s0.rem_euclid(self.p as i128) as u64
    }
    pub fn pow(&self, mut a: u64, mut e: u64, t: &mut Tally) -> u64 {
        let mut r = 1 % self.p;
        let mut first = true;
        while e > 0 {
            if e & 1 == 1 {
                r = if first { a } else { self.mul(r, a, t) };
                first = false;
            }
            e >>= 1;
            if e > 0 {
                a = self.mul(a, a, t);
            }
        }
        r
    }
    /// 1 for a nonzero square, `p − 1` for a non-square, 0 for 0.
    pub fn legendre(&self, a: u64, t: &mut Tally) -> u64 {
        self.pow(a, (self.p - 1) / 2, t)
    }
    pub fn is_square(&self, a: u64, t: &mut Tally) -> bool {
        a == 0 || self.legendre(a, t) == 1
    }
    /// Tonelli–Shanks; `None` for a non-square. The non-residue search is
    /// charged like the rest (one Legendre symbol per candidate).
    pub fn sqrt(&self, a: u64, t: &mut Tally) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a, t) != 1 {
            return None;
        }
        if self.p % 4 == 3 {
            return Some(self.pow(a, (self.p + 1) / 4, t));
        }
        let s = (self.p - 1).trailing_zeros();
        let q = (self.p - 1) >> s;
        let mut z = 2;
        while self.legendre(z, t) != self.p - 1 {
            z += 1;
        }
        let mut m = s;
        let mut c = self.pow(z, q, t);
        let mut tt = self.pow(a, q, t);
        let mut r = self.pow(a, q.div_ceil(2), t);
        while tt != 1 {
            let mut i = 0;
            let mut t2 = tt;
            while t2 != 1 {
                t2 = self.mul(t2, t2, t);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b, t);
            }
            m = i;
            c = self.mul(b, b, t);
            tt = self.mul(tt, c, t);
            r = self.mul(r, b, t);
        }
        Some(r)
    }
}

/// `y² = x³ + ax + b` over `F_p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    pub a: u64,
    pub b: u64,
}

/// An affine point; `None` is the point at infinity.
pub type Point = Option<(u64, u64)>;

impl Curve {
    pub fn on(&self, f: &Fp, pt: Point) -> bool {
        let mut t = Tally::default();
        match pt {
            None => true,
            Some((x, y)) => {
                let x3 = f.mul(f.mul(x, x, &mut t), x, &mut t);
                let rhs = f.add(f.add(x3, f.mul(self.a, x, &mut t)), self.b);
                f.mul(y, y, &mut t) == rhs
            }
        }
    }
    /// `j = 1728 · 4a³ / (4a³ + 27b²)` (uncounted; identifies vertices).
    pub fn j(&self, f: &Fp) -> u64 {
        let mut t = Tally::default();
        let a3 = f.mul(f.mul(self.a, self.a, &mut t), self.a, &mut t);
        let four_a3 = f.small(4, a3);
        let b2 = f.mul(self.b, self.b, &mut t);
        let den = f.add(four_a3, f.small(27, b2));
        let num = f.mul(1728 % f.p, four_a3, &mut t);
        f.mul(num, f.inv(den, &mut t), &mut t)
    }
    pub fn add(&self, f: &Fp, p1: Point, p2: Point) -> Point {
        let mut t = Tally::default();
        let (x1, y1) = match p1 {
            None => return p2,
            Some(v) => v,
        };
        let (x2, y2) = match p2 {
            None => return p1,
            Some(v) => v,
        };
        let lam = if x1 == x2 {
            if f.add(y1, y2) == 0 {
                return None;
            }
            let num = f.add(f.small(3, f.mul(x1, x1, &mut t)), self.a);
            f.mul(num, f.inv(f.small(2, y1), &mut t), &mut t)
        } else {
            f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1), &mut t), &mut t)
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam, &mut t), x1), x2);
        let y3 = f.sub(f.mul(lam, f.sub(x1, x3), &mut t), y1);
        Some((x3, y3))
    }
    pub fn mul(&self, f: &Fp, pt: Point, mut k: u64) -> Point {
        let mut acc = None;
        let mut base = pt;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(f, acc, base);
            }
            base = self.add(f, base, base);
            k >>= 1;
        }
        acc
    }
}

/// Polynomials over `F_p` as ascending coefficient vectors, counted.
pub mod poly {
    use super::{Fp, Tally};

    pub fn trim(v: &mut Vec<u64>) {
        while v.len() > 1 && *v.last().unwrap() == 0 {
            v.pop();
        }
        if v.is_empty() {
            v.push(0);
        }
    }

    pub fn is_zero(v: &[u64]) -> bool {
        v.iter().all(|&c| c == 0)
    }

    pub fn deg(v: &[u64]) -> usize {
        let mut w = v.to_vec();
        trim(&mut w);
        w.len() - 1
    }

    pub fn mul(f: &Fp, a: &[u64], b: &[u64], t: &mut Tally) -> Vec<u64> {
        let mut out = vec![0u64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            if x == 0 {
                continue;
            }
            for (j, &y) in b.iter().enumerate() {
                if y == 0 {
                    continue;
                }
                out[i + j] = f.add(out[i + j], f.mul(x, y, t));
            }
        }
        trim(&mut out);
        out
    }

    /// `a mod m` for a nonzero `m`; one inversion unless `m` is monic.
    pub fn rem(f: &Fp, a: &[u64], m: &[u64], t: &mut Tally) -> Vec<u64> {
        let mut m = m.to_vec();
        trim(&mut m);
        let mut r = a.to_vec();
        trim(&mut r);
        let dm = m.len() - 1;
        let lead = *m.last().unwrap();
        assert!(lead != 0, "division by the zero polynomial");
        let li = if lead == 1 { 1 } else { f.inv(lead, t) };
        while r.len() > dm && !(r.len() == 1 && r[0] == 0) {
            let c = *r.last().unwrap();
            if c != 0 {
                let q = if li == 1 { c } else { f.mul(c, li, t) };
                let shift = r.len() - 1 - dm;
                for (i, &mi) in m.iter().enumerate() {
                    if mi != 0 {
                        r[shift + i] = f.sub(r[shift + i], f.mul(q, mi, t));
                    }
                }
            }
            r.pop();
            if r.is_empty() {
                r.push(0);
            }
        }
        trim(&mut r);
        r
    }

    pub fn mulmod(f: &Fp, a: &[u64], b: &[u64], m: &[u64], t: &mut Tally) -> Vec<u64> {
        let pr = mul(f, a, b, t);
        rem(f, &pr, m, t)
    }

    /// `base^e mod m` by left-to-right square-and-multiply.
    pub fn powmod(f: &Fp, base: &[u64], e: u64, m: &[u64], t: &mut Tally) -> Vec<u64> {
        if e == 0 {
            return vec![1];
        }
        let base = rem(f, base, m, t);
        let mut r = base.clone();
        let top = 63 - e.leading_zeros();
        for bit in (0..top).rev() {
            r = mulmod(f, &r, &r, m, t);
            if (e >> bit) & 1 == 1 {
                r = mulmod(f, &r, &base, m, t);
            }
        }
        r
    }

    /// Monic gcd.
    pub fn gcd(f: &Fp, a: &[u64], b: &[u64], t: &mut Tally) -> Vec<u64> {
        let (mut x, mut y) = (a.to_vec(), b.to_vec());
        trim(&mut x);
        trim(&mut y);
        while !is_zero(&y) {
            let r = rem(f, &x, &y, t);
            x = y;
            y = r;
        }
        let lead = *x.last().unwrap();
        if lead != 1 && lead != 0 {
            let li = f.inv(lead, t);
            for c in x.iter_mut() {
                *c = f.mul(*c, li, t);
            }
        }
        x
    }

    pub fn sub(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
        let n = a.len().max(b.len());
        let mut out = vec![0u64; n];
        for (i, o) in out.iter_mut().enumerate() {
            *o = f.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
        }
        trim(&mut out);
        out
    }
}

/// The cubic `x³ + ax + b` as an ascending coefficient vector.
pub fn cubic(c: &Curve) -> Vec<u64> {
    vec![c.b, c.a, 0, 1]
}

/// `gcd(x^p − x, x³ + ax + b)`: its degree is the number of rational roots.
pub fn rational_part(f: &Fp, c: &Curve, t: &mut Tally) -> Vec<u64> {
    let m = cubic(c);
    let xp = poly::powmod(f, &[0, 1], f.p, &m, t);
    let g = poly::sub(f, &xp, &[0, 1]);
    poly::gcd(f, &m, &g, t)
}

/// Every rational root of `x³ + ax + b`, ascending (uncharged callers pass
/// a scratch tally). Equal-degree splitting with `(x + δ)^((p−1)/2) − 1`.
pub fn all_roots(f: &Fp, c: &Curve, t: &mut Tally) -> Vec<u64> {
    let g = rational_part(f, c, t);
    let mut out = Vec::new();
    split(f, g, 0, &mut out, t);
    out.sort_unstable();
    out
}

fn split(f: &Fp, g: Vec<u64>, mut delta: u64, out: &mut Vec<u64>, t: &mut Tally) {
    let d = poly::deg(&g);
    if d == 0 {
        return;
    }
    if d == 1 {
        out.push(f.neg(g[0]));
        return;
    }
    loop {
        let h = poly::powmod(f, &[delta % f.p, 1], (f.p - 1) / 2, &g, t);
        let h1 = poly::sub(f, &h, &[1]);
        let k = poly::gcd(f, &g, &h1, t);
        let dk = poly::deg(&k);
        if dk > 0 && dk < d {
            let rest = div_exact(f, &g, &k, t);
            split(f, k, delta + 1, out, t);
            split(f, rest, delta + 1, out, t);
            return;
        }
        delta += 1;
        assert!(delta < 10_000, "root splitting failed");
    }
}

/// `a / b` for a monic `b` that divides `a`.
fn div_exact(f: &Fp, a: &[u64], b: &[u64], t: &mut Tally) -> Vec<u64> {
    let mut r = a.to_vec();
    poly::trim(&mut r);
    let db = b.len() - 1;
    let mut q = vec![0u64; r.len() - db];
    while r.len() > db && poly::deg(&r) >= db && !poly::is_zero(&r) {
        let c = *r.last().unwrap();
        let shift = r.len() - 1 - db;
        q[shift] = c;
        for (i, &bi) in b.iter().enumerate() {
            r[shift + i] = f.sub(r[shift + i], f.mul(c, bi, t));
        }
        r.pop();
    }
    poly::trim(&mut q);
    q
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_and_legendre_agree() {
        let f = Fp::new(15808601); // ≡ 1 mod 8: the general Tonelli–Shanks branch
        let mut t = Tally::default();
        for a in 1..200u64 {
            let sq = f.mul(a, a, &mut t);
            let r = f.sqrt(sq, &mut t).expect("a square");
            assert_eq!(f.mul(r, r, &mut t), sq);
        }
        let mut non = 0;
        for a in 2..200u64 {
            if f.sqrt(a, &mut t).is_none() {
                non += 1;
                assert_eq!(f.legendre(a, &mut t), f.p - 1);
            }
        }
        assert!(non > 50);
    }

    #[test]
    fn inverse_and_pow() {
        let f = Fp::new(1_000_003);
        let mut t = Tally::default();
        for a in 1..100u64 {
            assert_eq!(f.mul(a, f.inv(a, &mut t), &mut t), 1);
            assert_eq!(f.pow(a, f.p - 1, &mut t), 1);
        }
        assert_eq!(t.invs, 99);
    }

    #[test]
    fn roots_of_a_split_cubic() {
        // (x − 3)(x − 5)(x + 8) = x³ − 49x + 120 over F_10007.
        let f = Fp::new(10007);
        let c = Curve {
            a: f.neg(49),
            b: 120,
        };
        let mut t = Tally::default();
        assert_eq!(all_roots(&f, &c, &mut t), vec![3, 5, 10007 - 8]);
        assert_eq!(poly::deg(&rational_part(&f, &c, &mut t)), 3);
    }

    #[test]
    fn group_law_has_the_right_order_on_a_fixture_curve() {
        // research/isogeny-volcano-cost-v2_20261017 class l2-s1001, vertex 0.
        let f = Fp::new(15808601);
        let c = Curve {
            a: 2021557,
            b: 15763640,
        };
        let g = Some((15122355, 3611240));
        assert!(c.on(&f, g));
        assert_eq!(c.mul(&f, g, 1317583), None);
        assert!(c.mul(&f, g, 12).is_some());
    }
}
