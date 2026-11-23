//! Short Weierstrass curves y^2 = x^3 + a x + b over F_{p^n}; group order by BSGS.
use crate::field::*;
use std::collections::HashMap;

pub type Pt = Option<(E, E)>;

#[derive(Clone)]
pub struct Curve {
    pub f: Fq,
    pub a: E,
    pub b: E,
}

impl Curve {
    pub fn rhs(&self, x: &E) -> E {
        let f = &self.f;
        let x2 = f.sq(x);
        let x3 = f.mul(&x2, x);
        f.add(&f.add(&x3, &f.mul(&self.a, x)), &self.b)
    }
    pub fn on(&self, p: &Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.f.sq(y) == self.rhs(x),
        }
    }
    pub fn neg(&self, p: &Pt) -> Pt {
        p.map(|(x, y)| (x, self.f.neg(&y)))
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let f = &self.f;
        match (p, q) {
            (None, _) => *q,
            (_, None) => *p,
            (Some((x1, y1)), Some((x2, y2))) => {
                if x1 == x2 {
                    if f.is_zero(&f.add(y1, y2)) {
                        return None;
                    }
                    return self.dbl(p);
                }
                let l = f.mul(&f.sub(y2, y1), &f.inv(&f.sub(x2, x1)));
                let x3 = f.sub(&f.sub(&f.sq(&l), x1), x2);
                let y3 = f.sub(&f.mul(&l, &f.sub(x1, &x3)), y1);
                Some((x3, y3))
            }
        }
    }
    pub fn sub(&self, p: &Pt, q: &Pt) -> Pt {
        self.add(p, &self.neg(q))
    }
    pub fn dbl(&self, p: &Pt) -> Pt {
        let f = &self.f;
        let (x, y) = (*p)?;
        if f.is_zero(&y) {
            return None;
        }
        let num = f.add(&f.mul(&f.c(3), &f.sq(&x)), &self.a);
        let l = f.mul(&num, &f.inv(&f.add(&y, &y)));
        let x3 = f.sub(&f.sq(&l), &f.add(&x, &x));
        let y3 = f.sub(&f.mul(&l, &f.sub(&x, &x3)), &y);
        Some((x3, y3))
    }
    pub fn mul(&self, k: u128, p: &Pt) -> Pt {
        let mut r: Pt = None;
        let mut b = *p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            b = self.dbl(&b);
            k >>= 1;
        }
        r
    }
    pub fn rand_point(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = self.f.rand(rng);
            if let Some(y) = self.f.sqrt(&self.rhs(&x)) {
                let y = if rng.next() & 1 == 1 { self.f.neg(&y) } else { y };
                return Some((x, y));
            }
        }
    }
    pub fn disc_ok(&self) -> bool {
        let f = &self.f;
        let a3 = f.mul(&f.sq(&self.a), &self.a);
        let d = f.add(&f.mul(&f.c(4), &a3), &f.mul(&f.c(27), &f.sq(&self.b)));
        !f.is_zero(&d)
    }
}

fn key(p: &Pt) -> [u64; 8] {
    match p {
        None => [u64::MAX; 8],
        Some((x, y)) => [x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]],
    }
}

fn isqrt(n: u128) -> u128 {
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn lcm(a: u128, b: u128) -> u128 {
    fn g(a: u128, b: u128) -> u128 {
        if b == 0 { a } else { g(b, a % b) }
    }
    a / g(a, b) * b
}

pub fn point_order(c: &Curve, p: &Pt, multiple: u128) -> u128 {
    let mut o = multiple;
    for (r, _) in factor(multiple) {
        while o % r == 0 && c.mul(o / r, p).is_none() {
            o /= r;
        }
    }
    o
}

/// #E(F_q) via BSGS on random points in the Hasse interval. None if not determined.
pub fn group_order(c: &Curve, rng: &mut Rng) -> Option<u128> {
    let q = c.f.q;
    let s = isqrt(4 * q) + 1; // >= 2 sqrt q
    let lo = q + 1 - s;
    let hi = q + 1 + s;
    let width = hi - lo + 1;
    let m = isqrt(width) + 1;
    let mut l = 1u128;
    for _ in 0..20 {
        let pnt = c.rand_point(rng);
        let mut table: HashMap<[u64; 8], u128> = HashMap::with_capacity(m as usize);
        let mut cur: Pt = None;
        for j in 0..m {
            table.entry(key(&cur)).or_insert(j);
            cur = c.add(&cur, &pnt);
        }
        let step = c.mul(m, &pnt);
        let mut g = c.mul(lo, &pnt);
        let mut found = None;
        for i in 0..=m {
            if let Some(&j) = table.get(&key(&c.neg(&g))) {
                found = Some(lo + i * m + j);
                break;
            }
            g = c.add(&g, &step);
        }
        let n0 = found?;
        let o = point_order(c, &pnt, n0);
        l = lcm(l, o);
        if l > width {
            let n = (hi / l) * l;
            if n >= lo && n - l < lo {
                return Some(n);
            }
            return None;
        }
    }
    None
}

/// Curve with #E = h * ell, ell prime, h <= hmax. Returns (curve, N, h, ell).
pub fn prime_order_curve(f: &Fq, hmax: u128, rng: &mut Rng) -> (Curve, u128, u128, u128) {
    loop {
        let c = Curve { f: f.clone(), a: f.rand(rng), b: f.rand(rng) };
        if !c.disc_ok() {
            continue;
        }
        let Some(n) = group_order(&c, rng) else { continue };
        for h in 1..=hmax {
            if n % h == 0 && is_prime(n / h) {
                return (c, n, h, n / h);
            }
        }
    }
}
