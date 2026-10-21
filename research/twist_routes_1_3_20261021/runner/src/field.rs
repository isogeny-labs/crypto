//! Exact small-field reference arithmetic. No timing or cryptographic-size claim.
use serde::{Deserialize, Serialize};

pub fn pow(mut a: u64, mut n: u128, p: u64) -> u64 {
    let mut b = 1;
    while n != 0 {
        if n & 1 != 0 {
            b = b * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    b
}
pub fn inv(a: u64, p: u64) -> u64 {
    assert_ne!(a % p, 0);
    pow(a, (p - 2) as u128, p)
}
pub fn prime(p: u64) -> bool {
    p >= 2 && (2..).take_while(|x| x * x <= p).all(|x| p % x != 0)
}
pub fn sqrt(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if pow(a, ((p - 1) / 2) as u128, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(pow(a, ((p + 1) / 4) as u128, p));
    }
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let z = (2..p)
        .find(|&z| pow(z, ((p - 1) / 2) as u128, p) == p - 1)
        .unwrap();
    let mut c = pow(z, q as u128, p);
    let mut x = pow(a, ((q + 1) / 2) as u128, p);
    let mut t = pow(a, q as u128, p);
    let mut m = s;
    while t != 1 {
        let mut v = t;
        let mut i = 0;
        while v != 1 {
            v = v * v % p;
            i += 1;
            assert!(i < m);
        }
        let b = pow(c, 1u128 << (m - i - 1), p);
        x = x * b % p;
        c = b * b % p;
        t = t * c % p;
        m = i;
    }
    Some(x)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Q(pub u64, pub u64);
pub type P = Option<(Q, Q)>;
#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub p: u64,
    pub d: u64,
    pub a: u64,
    pub b: u64,
}
impl Quad {
    pub fn new(p: u64, a: u64, b: u64) -> Self {
        assert!(p > 3 && p < 32768 && prime(p));
        assert_ne!((4 * a % p * a % p * a + 27 * b % p * b) % p, 0);
        let d = (2..p)
            .find(|&d| pow(d, ((p - 1) / 2) as u128, p) == p - 1)
            .unwrap();
        Self { p, d, a, b }
    }
    pub fn add(self, x: Q, y: Q) -> Q {
        Q((x.0 + y.0) % self.p, (x.1 + y.1) % self.p)
    }
    pub fn neg(self, x: Q) -> Q {
        Q((self.p - x.0) % self.p, (self.p - x.1) % self.p)
    }
    pub fn sub(self, x: Q, y: Q) -> Q {
        self.add(x, self.neg(y))
    }
    pub fn mul(self, x: Q, y: Q) -> Q {
        Q(
            (x.0 * y.0 + self.d * x.1 % self.p * y.1) % self.p,
            (x.0 * y.1 + x.1 * y.0) % self.p,
        )
    }
    pub fn scale(self, x: Q, n: u64) -> Q {
        Q(x.0 * n % self.p, x.1 * n % self.p)
    }
    pub fn inv(self, x: Q) -> Q {
        let n = (x.0 * x.0 + self.p - self.d * x.1 % self.p * x.1 % self.p) % self.p;
        self.scale(self.neg(Q((self.p - x.0) % self.p, x.1)), inv(n, self.p))
    }
    pub fn sqrt(self, x: Q) -> Option<Q> {
        if x.1 == 0 {
            return sqrt(x.0, self.p)
                .map(|z| Q(z, 0))
                .or_else(|| sqrt(x.0 * inv(self.d, self.p) % self.p, self.p).map(|z| Q(0, z)));
        }
        let norm = (x.0 * x.0 + self.p - self.d * x.1 % self.p * x.1 % self.p) % self.p;
        let s = sqrt(norm, self.p)?;
        let half = inv(2, self.p);
        for n in [(x.0 + s) % self.p, (x.0 + self.p - s) % self.p] {
            if let Some(a) = sqrt(n * half % self.p, self.p) {
                if a != 0 {
                    let y = Q(a, x.1 * inv(2 * a % self.p, self.p) % self.p);
                    assert_eq!(self.mul(y, y), x);
                    return Some(y);
                }
            }
        }
        None
    }
    pub fn rhs(self, x: Q) -> Q {
        self.add(
            self.add(self.mul(self.mul(x, x), x), self.scale(x, self.a)),
            Q(self.b, 0),
        )
    }
    pub fn on(self, x: P) -> bool {
        x.is_none_or(|(x, y)| self.mul(y, y) == self.rhs(x))
    }
    pub fn pn(self, x: P) -> P {
        x.map(|(x, y)| (x, self.neg(y)))
    }
    pub fn frob(self, x: P) -> P {
        x.map(|(x, y)| {
            (
                Q(x.0, (self.p - x.1) % self.p),
                Q(y.0, (self.p - y.1) % self.p),
            )
        })
    }
    pub fn pa(self, x: P, y: P) -> P {
        let (Some((x1, y1)), Some((x2, y2))) = (x, y) else {
            return x.or(y);
        };
        if x1 == x2 && self.add(y1, y2) == Q(0, 0) {
            return None;
        }
        let m = if x1 == x2 {
            self.mul(
                self.add(self.scale(self.mul(x1, x1), 3), Q(self.a, 0)),
                self.inv(self.scale(y1, 2)),
            )
        } else {
            self.mul(self.sub(y2, y1), self.inv(self.sub(x2, x1)))
        };
        let x3 = self.sub(self.sub(self.mul(m, m), x1), x2);
        Some((x3, self.sub(self.mul(m, self.sub(x1, x3)), y1)))
    }
    pub fn pm(self, mut x: P, mut n: u128) -> P {
        let mut z = None;
        while n != 0 {
            if n & 1 != 0 {
                z = self.pa(z, x);
            }
            x = self.pa(x, x);
            n >>= 1;
        }
        z
    }
    pub fn trace(self, x: P) -> P {
        self.pa(x, self.frob(x))
    }
    pub fn rational(self) -> Vec<P> {
        let mut v = vec![None];
        for x in 0..self.p {
            if let Some(y) = sqrt(self.rhs(Q(x, 0)).0, self.p) {
                v.push(Some((Q(x, 0), Q(y, 0))));
                if y != 0 {
                    v.push(Some((Q(x, 0), Q(self.p - y, 0))));
                }
            }
        }
        v.sort();
        v
    }
    pub fn points(self) -> Vec<P> {
        let q = (self.p * self.p) as usize;
        let mut roots = vec![None; q];
        for a in 0..self.p {
            for b in 0..self.p {
                let x = Q(a, b);
                let s = self.mul(x, x);
                roots[(s.0 * self.p + s.1) as usize] = Some(x);
            }
        }
        let mut out = vec![None];
        for a in 0..self.p {
            for b in 0..self.p {
                let x = Q(a, b);
                let r = self.rhs(x);
                if let Some(y) = roots[(r.0 * self.p + r.1) as usize] {
                    out.push(Some((x, y)));
                    if y != Q(0, 0) {
                        out.push(Some((x, self.neg(y))));
                    }
                }
            }
        }
        out.sort();
        out
    }
}

/// F_(p^6)=F_p[alpha,omega]/(alpha^3+A alpha+B,omega^2-d).
/// Requires the cubic to have no F_p root. Arrays are 1,alpha,alpha²,omega,... .
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct S(pub [u64; 6]);
pub const Z: S = S([0; 6]);
pub const ONE: S = S([1, 0, 0, 0, 0, 0]);
pub type SP = Option<(S, S)>;
#[derive(Copy, Clone, Debug)]
pub struct Sext {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub d: u64,
}
impl Sext {
    pub fn new(e: Quad) -> Self {
        assert!((0..e.p).all(|x| e.rhs(Q(x, 0)).0 != 0));
        Self {
            p: e.p,
            a: e.a,
            b: e.b,
            d: e.d,
        }
    }
    pub fn c(self, x: u64) -> S {
        S([x % self.p, 0, 0, 0, 0, 0])
    }
    pub fn add(self, x: S, y: S) -> S {
        let mut r = [0; 6];
        for i in 0..6 {
            r[i] = (x.0[i] + y.0[i]) % self.p;
        }
        S(r)
    }
    pub fn neg(self, x: S) -> S {
        S(x.0.map(|v| (self.p - v) % self.p))
    }
    pub fn sub(self, x: S, y: S) -> S {
        self.add(x, self.neg(y))
    }
    fn m3(self, x: &[u64], y: &[u64]) -> [u64; 3] {
        let mut z = [0; 5];
        for i in 0..3 {
            for j in 0..3 {
                z[i + j] = (z[i + j] + x[i] * y[j]) % self.p;
            }
        }
        for i in (3..5).rev() {
            z[i - 2] = (z[i - 2] + self.p - z[i] * self.a % self.p) % self.p;
            z[i - 3] = (z[i - 3] + self.p - z[i] * self.b % self.p) % self.p;
        }
        [z[0], z[1], z[2]]
    }
    pub fn mul(self, x: S, y: S) -> S {
        let ac = self.m3(&x.0[..3], &y.0[..3]);
        let bd = self.m3(&x.0[3..], &y.0[3..]);
        let ad = self.m3(&x.0[..3], &y.0[3..]);
        let bc = self.m3(&x.0[3..], &y.0[..3]);
        let mut z = [0; 6];
        for i in 0..3 {
            z[i] = (ac[i] + self.d * bd[i]) % self.p;
            z[i + 3] = (ad[i] + bc[i]) % self.p;
        }
        S(z)
    }
    pub fn pow(self, mut x: S, mut n: u128) -> S {
        let mut y = ONE;
        while n != 0 {
            if n & 1 != 0 {
                y = self.mul(y, x);
            }
            x = self.mul(x, x);
            n >>= 1;
        }
        y
    }
    pub fn inv(self, x: S) -> S {
        assert_ne!(x, Z);
        let y = self.pow(
            x,
            (self.p as u128).pow(if x.0[3..] == [0, 0, 0] { 3 } else { 6 }) - 2,
        );
        assert_eq!(self.mul(x, y), ONE);
        y
    }
    pub fn div(self, x: S, y: S) -> S {
        self.mul(x, self.inv(y))
    }
    pub fn sqrt(self, x: S) -> Option<S> {
        if x.0[3..] == [0, 0, 0] {
            return self.sqrt_degree(x, 3).or_else(|| {
                self.sqrt_degree(self.div(x, self.c(self.d)), 3)
                    .map(|r| self.mul(r, S([0, 0, 0, 1, 0, 0])))
            });
        }
        self.sqrt_degree(x, 6)
    }
    fn sqrt_degree(self, x: S, degree: u32) -> Option<S> {
        if x == Z {
            return Some(Z);
        }
        let q = (self.p as u128).pow(degree);
        if self.pow(x, (q - 1) / 2) != ONE {
            return None;
        }
        let s = (q - 1).trailing_zeros();
        let h = (q - 1) >> s;
        let mut z = Z;
        'outer: for k in 1..self.p {
            for candidate in [S([k, 1, 0, 0, 0, 0]), S([k, 0, 0, 1, 0, 0])] {
                if degree == 3 && candidate.0[3] != 0 {
                    continue;
                }
                if self.pow(candidate, (q - 1) / 2) != ONE {
                    z = candidate;
                    break 'outer;
                }
            }
        }
        assert_ne!(z, Z);
        let mut c = self.pow(z, h);
        let mut y = self.pow(x, (h + 1) / 2);
        let mut t = self.pow(x, h);
        let mut m = s;
        while t != ONE {
            let mut v = t;
            let mut i = 0;
            while v != ONE {
                v = self.mul(v, v);
                i += 1;
                assert!(i < m);
            }
            let b = self.pow(c, 1u128 << (m - i - 1));
            y = self.mul(y, b);
            c = self.mul(b, b);
            t = self.mul(t, c);
            m = i;
        }
        assert_eq!(self.mul(y, y), x);
        Some(y)
    }
    pub fn lift(self, q: Q) -> S {
        S([q.0, 0, 0, q.1, 0, 0])
    }
    pub fn fp(self, s: S) -> Option<u64> {
        if s.0[1..].iter().all(|x| *x == 0) {
            Some(s.0[0])
        } else {
            None
        }
    }
    pub fn pn(self, x: SP) -> SP {
        x.map(|(x, y)| (x, self.neg(y)))
    }
    pub fn on(self, x: SP, a: S, b: S) -> bool {
        x.is_none_or(|(x, y)| {
            self.mul(y, y) == self.add(self.add(self.mul(self.mul(x, x), x), self.mul(a, x)), b)
        })
    }
    pub fn pa(self, x: SP, y: SP, a: S) -> SP {
        let (Some((x1, y1)), Some((x2, y2))) = (x, y) else {
            return x.or(y);
        };
        if x1 == x2 && self.add(y1, y2) == Z {
            return None;
        }
        let m = if x1 == x2 {
            self.div(
                self.add(self.mul(self.c(3), self.mul(x1, x1)), a),
                self.mul(self.c(2), y1),
            )
        } else {
            self.div(self.sub(y2, y1), self.sub(x2, x1))
        };
        let x3 = self.sub(self.sub(self.mul(m, m), x1), x2);
        Some((x3, self.sub(self.mul(m, self.sub(x1, x3)), y1)))
    }
    pub fn pm(self, mut x: SP, mut n: u128, a: S) -> SP {
        let mut z = None;
        while n != 0 {
            if n & 1 != 0 {
                z = self.pa(z, x, a);
            }
            x = self.pa(x, x, a);
            n >>= 1;
        }
        z
    }
    pub fn frob(self, x: SP) -> SP {
        x.map(|(x, y)| (self.pow(x, self.p as u128), self.pow(y, self.p as u128)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quad_roots_and_group_reference() {
        for p in [7, 11, 19, 59] {
            let e = Quad::new(p, 1, 1);
            for a in 0..p {
                for b in 0..p {
                    let x = Q(a, b);
                    assert_eq!(e.sqrt(e.mul(x, x)).map(|r| e.mul(r, r)), Some(e.mul(x, x)));
                    if x != Q(0, 0) {
                        assert_eq!(e.mul(x, e.inv(x)), Q(1, 0));
                    }
                }
            }
            for x in e.rational() {
                assert!(e.on(x));
                assert_eq!(e.pa(x, e.pn(x)), None);
            }
        }
    }
    #[test]
    fn sext_tower_and_torsion() {
        let e = Quad::new(59, 1, 13);
        let f = Sext::new(e);
        for x in [
            S([0, 1, 0, 0, 0, 0]),
            S([1, 2, 3, 4, 5, 6]),
            S([0, 0, 0, 1, 0, 0]),
        ] {
            assert_eq!(f.mul(x, f.inv(x)), ONE);
            assert_eq!(f.pow(x, 59u128.pow(6)), x);
            assert_eq!(f.sqrt(f.mul(x, x)).map(|s| f.mul(s, s)), Some(f.mul(x, x)));
        }
        let t = Some((S([0, 1, 0, 0, 0, 0]), Z));
        assert!(f.on(t, f.c(e.a), f.c(e.b)));
        assert_eq!(f.pa(t, t, f.c(e.a)), None);
    }
}
