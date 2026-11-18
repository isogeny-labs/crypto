//! The weak family of `jv_cover.rs`: `E_α : y^2 = x(x - α)(x - σα)` over
//! `F_{q^3}`, `q = p^2`, `α ∈ F_{q^3} \ F_q`, i.e. `y^2 = x^3 + a2 x^2 + a4 x`
//! with `a2 = -(α + σα)`, `a4 = α σα`. Affine arithmetic matching the
//! library's `EllE` (same formulas, same point encoding).

use crate::field::{is_prime_u64, Tower, E6};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pt {
    pub x: E6,
    pub y: E6,
    pub inf: bool,
}
impl Pt {
    pub const INF: Pt = Pt { x: E6::ZERO, y: E6::ZERO, inf: true };
}

pub struct Curve<'a> {
    pub t: &'a Tower,
    pub alpha: E6,
    pub a2: E6,
    pub a4: E6,
}

impl<'a> Curve<'a> {
    pub fn new(t: &'a Tower, alpha: E6) -> Curve<'a> {
        let sa = t.sigma(&alpha);
        Curve { t, alpha, a2: t.neg(&t.add(&alpha, &sa)), a4: t.mul(&alpha, &sa) }
    }
    pub fn rhs(&self, x: &E6) -> E6 {
        let t = self.t;
        let x2 = t.sq(x);
        t.add(&t.add(&t.mul(&x2, x), &t.mul(&self.a2, &x2)), &t.mul(&self.a4, x))
    }
    pub fn on_curve(&self, p: &Pt) -> bool {
        p.inf || self.t.sq(&p.y) == self.rhs(&p.x)
    }
    pub fn neg(&self, p: &Pt) -> Pt {
        if p.inf { *p } else { Pt { x: p.x, y: self.t.neg(&p.y), inf: false } }
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let t = self.t;
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        let lam = if p.x == q.x {
            if p.y == t.neg(&q.y) || p.y == E6::ZERO {
                return Pt::INF;
            }
            let x2 = t.sq(&p.x);
            let num = t.add(
                &t.add(&t.add(&x2, &t.add(&x2, &x2)), &t.mul(&t.add(&self.a2, &self.a2), &p.x)),
                &self.a4,
            );
            t.mul(&num, &t.inv(&t.add(&p.y, &p.y)))
        } else {
            t.mul(&t.sub(&q.y, &p.y), &t.inv(&t.sub(&q.x, &p.x)))
        };
        let x3 = t.sub(&t.sub(&t.sub(&t.sq(&lam), &self.a2), &p.x), &q.x);
        let y3 = t.sub(&t.mul(&lam, &t.sub(&p.x, &x3)), &p.y);
        Pt { x: x3, y: y3, inf: false }
    }
    pub fn mul(&self, pt: &Pt, mut k: u128) -> Pt {
        let mut acc = Pt::INF;
        let mut base = *pt;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(&acc, &base);
            }
            k >>= 1;
            if k > 0 {
                base = self.add(&base, &base);
            }
        }
        acc
    }
    /// `j = 256 (a2^2 - 3a4)^3 / (a4^2 (a2^2 - 4a4))`; `None` if singular.
    pub fn j_invariant(&self) -> Option<E6> {
        let t = self.t;
        let a22 = t.sq(&self.a2);
        let three = t.from_fp(3);
        let four = t.from_fp(4);
        let num0 = t.sub(&a22, &t.mul(&three, &self.a4));
        let num = t.mul(&t.from_fp(256), &t.mul(&t.sq(&num0), &num0));
        let den = t.mul(&t.sq(&self.a4), &t.sub(&a22, &t.mul(&four, &self.a4)));
        if den == E6::ZERO {
            return None;
        }
        Some(t.mul(&num, &t.inv(&den)))
    }
}

pub fn isqrt128(n: u128) -> u128 {
    let mut r = (n as f64).sqrt() as u128;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

/// The library's Hasse window and baby-step giant-step: the least `m` in
/// `[q6 + 1 - (2⌊√q6⌋ + 2), q6 + 1 + (2⌊√q6⌋ + 2)]` with `[m]P = O`.
pub fn bsgs_order(c: &Curve, pt: &Pt) -> Option<u128> {
    let q6 = c.t.q6();
    let two_sqrt = 2 * isqrt128(q6) + 2;
    let lo = q6 + 1 - two_sqrt;
    let width = 2 * two_sqrt;
    let steps = isqrt128(width) + 1;
    let mut table: HashMap<Pt, u128> = HashMap::with_capacity(steps as usize);
    let mut jp = Pt::INF;
    for j in 0..steps {
        if !jp.inf {
            table.entry(jp).or_insert(j);
        }
        jp = c.add(&jp, pt);
    }
    let giant = c.mul(pt, steps);
    let mut t = c.mul(pt, lo);
    let mut i = 0u128;
    while i * steps <= width + steps {
        let m = if t.inf { Some(lo + i * steps) } else { table.get(&c.neg(&t)).map(|&j| lo + i * steps + j) };
        if m.is_some() {
            return m;
        }
        t = c.add(&t, &giant);
        i += 1;
    }
    None
}

/// `l = #E / 4` when the BSGS order of `pt` is `4l`, `l` a prime below `2^64`,
/// and every point in `checks` is killed by `4l`. Returns `(#E, l)`.
pub fn order_4_prime(c: &Curve, pt: &Pt, checks: &[Pt]) -> Option<(u128, u64)> {
    let m = bsgs_order(c, pt)?;
    if m % 4 != 0 || m / 4 > u64::MAX as u128 || !is_prime_u64((m / 4) as u64) {
        return None;
    }
    if checks.iter().any(|q| !c.mul(q, m).inf) {
        return None;
    }
    Some((m, (m / 4) as u64))
}

/// Smallest `k <= kmax` with `(p^6)^k ≡ 1 (mod l)`; `None` if none.
pub fn embedding_degree(p: u64, l: u64, kmax: u32) -> Option<u32> {
    let l128 = l as u128;
    let base = (p as u128).pow(6) % l128;
    let mut acc = 1u128;
    for k in 1..=kmax {
        acc = acc * base % l128;
        if acc == 1 {
            return Some(k);
        }
    }
    None
}

/// The `Cover::new` admission of `jv_cover.rs` without the transfer: the
/// polynomials `N(x) = Π (x - σ^i α)` and `F(x) = N(x)(x + Tr α) + Σ D_i(...)`
/// must have coefficients in `F_q`. (The transfer to `Jac_H` is checked by the
/// frozen pipeline's own public-input validation, outside this generator.)
pub fn cover_admits(t: &Tower, alpha: &E6) -> bool {
    if alpha.in_fq() {
        return false;
    }
    let sal = t.sigma(alpha);
    let s2al = t.sigma(&sal);
    let lin = |r: &E6| -> Vec<E6> { vec![t.neg(r), E6::ONE] };
    let pmul = |a: &[E6], b: &[E6]| -> Vec<E6> {
        let mut out = vec![E6::ZERO; a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = t.add(&out[i + j], &t.mul(x, y));
            }
        }
        out
    };
    let padd = |a: &[E6], b: &[E6]| -> Vec<E6> {
        let n = a.len().max(b.len());
        (0..n)
            .map(|i| t.add(a.get(i).unwrap_or(&E6::ZERO), b.get(i).unwrap_or(&E6::ZERO)))
            .collect()
    };
    let n6 = pmul(&pmul(&lin(alpha), &lin(&sal)), &lin(&s2al));
    let d0 = t.mul(&t.sub(alpha, &s2al), &t.sub(&sal, &s2al));
    let d1 = t.sigma(&d0);
    let d2 = t.sigma(&d1);
    let trace = t.add(&t.add(alpha, &sal), &s2al);
    let mut fx = pmul(&n6, &[trace, E6::ONE]);
    let terms = [
        (d0, pmul(&lin(alpha), &lin(&sal))),
        (d1, pmul(&lin(&sal), &lin(&s2al))),
        (d2, pmul(&lin(alpha), &lin(&s2al))),
    ];
    for (di, poly) in terms {
        let scaled: Vec<E6> = poly.iter().map(|c| t.mul(c, &di)).collect();
        fx = padd(&fx, &scaled);
    }
    n6.iter().all(|c| c.in_fq()) && fx.iter().all(|c| c.in_fq())
}
