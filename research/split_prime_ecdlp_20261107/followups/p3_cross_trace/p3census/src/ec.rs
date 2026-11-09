//! Short-Weierstrass arithmetic over F_p, p < 2^32, affine, one inversion per
//! effectual group operation. `ops` counts additions and doublings that were
//! not trivial (adding the identity is free; P + (-P) = O is free).
use crate::util::*;
use std::collections::HashMap;

pub type Pt = Option<(u64, u64)>;

#[derive(Clone, Copy, Debug)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

pub fn inv_p(a: u64, p: u64) -> u64 {
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (p as i64, a as i64);
    while nr != 0 {
        let q = r / nr;
        let x = t - q * nt;
        t = nt;
        nt = x;
        let y = r - q * nr;
        r = nr;
        nr = y;
    }
    debug_assert!(r == 1);
    if t < 0 {
        t += p as i64;
    }
    t as u64
}

impl Curve {
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        ((x * x % p) * x % p + self.a * x % p + self.b) % p
    }
    pub fn on_curve(&self, pt: Pt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => y * y % self.p == self.rhs(x),
        }
    }
    pub fn neg(&self, pt: Pt) -> Pt {
        pt.map(|(x, y)| (x, if y == 0 { 0 } else { self.p - y }))
    }
    pub fn dbl(&self, ops: &mut u64, pt: Pt) -> Pt {
        let (x, y) = pt?;
        if y == 0 {
            return None;
        }
        let p = self.p;
        *ops += 1;
        let lam = ((x * x % p) * 3 % p + self.a) % p * inv_p(2 * y % p, p) % p;
        let x3 = (lam * lam % p + 2 * p - 2 * x) % p;
        let y3 = (lam * ((x + p - x3) % p) % p + p - y) % p;
        Some((x3, y3))
    }
    pub fn add(&self, ops: &mut u64, p1: Pt, p2: Pt) -> Pt {
        let (x1, y1) = match p1 {
            None => return p2,
            Some(v) => v,
        };
        let (x2, y2) = match p2 {
            None => return p1,
            Some(v) => v,
        };
        let p = self.p;
        if x1 == x2 {
            return if y1 == y2 { self.dbl(ops, p1) } else { None };
        }
        *ops += 1;
        let lam = (y2 + p - y1) % p * inv_p((x2 + p - x1) % p, p) % p;
        let x3 = (lam * lam % p + 2 * p - x1 - x2) % p;
        let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
        Some((x3, y3))
    }
    /// Left-to-right double-and-add; every non-trivial operation is counted.
    pub fn mul(&self, ops: &mut u64, k: u64, pt: Pt) -> Pt {
        let mut r: Pt = None;
        if k == 0 {
            return None;
        }
        let top = 63 - k.leading_zeros();
        for i in (0..=top).rev() {
            r = self.dbl(ops, r);
            if (k >> i) & 1 == 1 {
                r = self.add(ops, r, pt);
            }
        }
        r
    }
    pub fn random_point(&self, rng: &mut Rng) -> (u64, u64) {
        loop {
            let x = rng.below(self.p);
            if let Some(y) = sqrt_mod(self.rhs(x), self.p) {
                if y != 0 {
                    return (x, if rng.next() & 1 == 1 { y } else { self.p - y });
                }
            }
        }
    }
    pub fn disc_nonzero(&self) -> bool {
        let p = self.p;
        let a3 = (self.a * self.a % p) * self.a % p;
        (4 * a3 % p + 27 * (self.b * self.b % p) % p) % p != 0
    }
    /// j = 1728 * 4a^3 / (4a^3 + 27 b^2) mod p
    pub fn j_inv(&self) -> u64 {
        let p = self.p;
        let a3 = (self.a * self.a % p) * self.a % p;
        let num = 1728 * (4 * a3 % p) % p;
        let den = (4 * a3 % p + 27 * (self.b * self.b % p) % p) % p;
        num * inv_p(den, p) % p
    }
}

/// All k in [-W, W] with (p+1+k) P = O, by baby-step giant-step.
/// None when P has too small an order for the table (retry with another point).
pub fn hasse_candidates(c: &Curve, pt: (u64, u64)) -> Option<Vec<u64>> {
    let p = c.p;
    let w = (2.0 * (p as f64).sqrt()).floor() as i64 + 2;
    let m = ((2 * w + 1) as f64).sqrt().ceil() as i64;
    let s = 2 * m + 1;
    let mut o = 0u64;
    let mut tab: HashMap<u64, (i64, u64)> = HashMap::with_capacity(m as usize * 2);
    let mut cur: Pt = Some(pt);
    for j in 1..=m {
        let (x, y) = cur?;
        if tab.insert(x, (j, y)).is_some() {
            return None;
        }
        cur = c.add(&mut o, cur, Some(pt));
        if cur.is_none() {
            return None;
        }
    }
    let q0 = c.mul(&mut o, p + 1, Some(pt));
    let gs = c.mul(&mut o, s as u64, Some(pt));
    let i_max = (w + s - 1) / s + 1;
    // R_i = -Q0 - i*S*P, start at i = -i_max : -Q0 + i_max*S*P
    let big = c.mul(&mut o, i_max as u64, gs);
    let mut r = c.add(&mut o, c.neg(q0), big);
    let mut out = vec![];
    for i in -i_max..=i_max {
        let kk: Option<i64> = match r {
            None => Some(i * s),
            Some((x, y)) => match tab.get(&x) {
                Some(&(j, yj)) => {
                    if y == yj {
                        Some(i * s + j)
                    } else if y + yj == p {
                        Some(i * s - j)
                    } else {
                        None
                    }
                }
                None => None,
            },
        };
        if let Some(k) = kk {
            if k.abs() <= w {
                out.push((p as i64 + 1 + k) as u64);
            }
        }
        r = c.add(&mut o, r, c.neg(gs));
    }
    out.sort();
    out.dedup();
    Some(out)
}

#[derive(Debug)]
pub enum Order {
    Prime(u64),
    Composite,
}

/// Exact group order via BSGS on random points. If the candidate list of the
/// first point holds a prime c, then c*P = O with P != O forces ord(P) = c, so
/// c | #E and #E = c (the Hasse interval is shorter than c). If it holds no
/// prime, #E (which is in the list) is composite.
pub fn order_prime_or_composite(c: &Curve, rng: &mut Rng) -> Result<Order, String> {
    for _ in 0..16 {
        let pt = c.random_point(rng);
        if let Some(cands) = hasse_candidates(c, pt) {
            let primes: Vec<u64> = cands.iter().copied().filter(|&x| is_prime(x)).collect();
            return match primes.len() {
                0 => Ok(Order::Composite),
                1 => Ok(Order::Prime(primes[0])),
                _ => Err("two prime candidates (impossible)".into()),
            };
        }
    }
    Err("bsgs failed on 16 points".into())
}
