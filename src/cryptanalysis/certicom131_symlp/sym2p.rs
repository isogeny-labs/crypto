//! Two-summand decomposition `R = ±P_1 ± P_2` over a small-abscissa
//! factor base `{x < B}` on a prime curve `y² = x³ + a·x + b` (ECCp-131
//! and the prime calibration curves), with and without symmetry breaking.
//!
//! With `e_1 = x_1 + x_2`, `e_2 = x_1x_2` and `c = x_R`, Semaev's
//!
//! ```text
//! S_3 = (x_1 − x_2)²c² − 2((x_1 + x_2)(x_1x_2 + a) + 2b)c + (x_1x_2 − a)² − 4b(x_1 + x_2)
//! ```
//!
//! becomes the quadratic in `e_2`
//!
//! ```text
//! e_2² − (4c² + 2e_1c + 2a)·e_2 + (e_1²c² − 2a·e_1c − 4bc + a² − 4b·e_1) = 0,
//! ```
//!
//! and `x_1, x_2 = (e_1 ± √(e_1² − 4e_2))/2`.  [`PrimeSymmetric`] sweeps
//! `e_1` and finds each unordered pair once.  Unlike a subspace of
//! `GF(2^n)`, the interval `[0, B')` is not closed under addition: `e_1`
//! ranges over `[0, 2B')`, twice the abscissae, and each `e_1` costs two
//! square roots against one for a lifted `x_1`.  [`PrimeBaseline`] sweeps
//! `x_1 ∈ [0, B')`, lifts it and evaluates `x(R ± P_1)` with one shared
//! batched inversion.  The two report the same pairs (tested); the report
//! compares their rates.

use num_bigint::BigUint;

use super::curves::{Key, PrimeW, Pt, SymCurve, E3, NO_KEY};
use super::sym2::OracleCounts;
use crate::cryptanalysis::prime_small_x::PrimeField;

/// An unordered pair of abscissae (canonical integer keys, `x1 ≤ x2`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PDecomp {
    pub x1: Key,
    pub x2: Key,
}

/// Integer order on keys (little-endian words).
#[inline(always)]
pub fn key_lt(a: &Key, b: &Key) -> bool {
    (a[2], a[1], a[0]) < (b[2], b[1], b[0])
}

impl PDecomp {
    pub fn new(a: Key, b: Key) -> Self {
        if key_lt(&b, &a) {
            Self { x1: b, x2: a }
        } else {
            Self { x1: a, x2: b }
        }
    }

    /// Large primes against the factor base `{x < bound}`.
    pub fn large_primes(&self, bound: &Key) -> u32 {
        !key_lt(&self.x1, bound) as u32 + !key_lt(&self.x2, bound) as u32
    }
}

pub fn key_of_big(v: &BigUint) -> Key {
    let d = v.to_u64_digits();
    assert!(d.len() <= 3);
    [
        d.first().copied().unwrap_or(0),
        d.get(1).copied().unwrap_or(0),
        d.get(2).copied().unwrap_or(0),
    ]
}

fn big_of_key(k: &Key) -> BigUint {
    BigUint::from_slice(&[
        k[0] as u32,
        (k[0] >> 32) as u32,
        k[1] as u32,
        (k[1] >> 32) as u32,
        k[2] as u32,
        (k[2] >> 32) as u32,
    ])
}

/// Unbroken reference: `x_1 ∈ [start, start + count)`, both abscissae
/// below `bound`.  A pair with both ends in the swept range is reported
/// twice; `emit` sees both.
pub struct PrimeBaseline<'c> {
    c: &'c PrimeW,
    pub bound: Key,
    batch: usize,
}

impl<'c> PrimeBaseline<'c> {
    pub fn new(c: &'c PrimeW, bound: &BigUint) -> Self {
        Self {
            c,
            bound: key_of_big(bound),
            batch: 64,
        }
    }

    pub fn sweep(
        &self,
        r: &Pt<E3>,
        start: &BigUint,
        count: u64,
        mut emit: impl FnMut(PDecomp),
    ) -> OracleCounts {
        let c = self.c;
        let mut counts = OracleCounts::default();
        let mut pts: Vec<Pt<E3>> = Vec::with_capacity(self.batch);
        let mut keys = Vec::with_capacity(self.batch);
        let mut flush = |pts: &mut Vec<Pt<E3>>, counts: &mut OracleCounts| {
            c.plus_minus_keys(r, pts, &mut keys);
            for (p, pm) in pts.iter().zip(keys.iter()) {
                for k2 in pm {
                    if *k2 == NO_KEY || !key_lt(k2, &self.bound) {
                        continue;
                    }
                    counts.in_domain += 1;
                    counts.decomps += 1;
                    emit(PDecomp::new(c.key(&p.x), *k2));
                }
            }
            pts.clear();
        };
        let mut x = c.embed(start);
        let one = c.c.field.one();
        let mut f = c.c.field.clone();
        for _ in 0..count {
            counts.steps += 1;
            if let Some(p) = c.lift_x(&x) {
                counts.trace_pass += 1;
                pts.push(p);
                if pts.len() == self.batch {
                    flush(&mut pts, &mut counts);
                }
            }
            x = f.add(x, one);
        }
        if !pts.is_empty() {
            flush(&mut pts, &mut counts);
        }
        counts
    }
}

/// Symmetry-broken sweep over `e_1 = x_1 + x_2 ∈ [start, start + count)`.
pub struct PrimeSymmetric<'c> {
    c: &'c PrimeW,
    pub bound: Key,
}

impl<'c> PrimeSymmetric<'c> {
    pub fn new(c: &'c PrimeW, bound: &BigUint) -> Self {
        Self {
            c,
            bound: key_of_big(bound),
        }
    }

    pub fn sweep(
        &self,
        r: &Pt<E3>,
        start: &BigUint,
        count: u64,
        mut emit: impl FnMut(PDecomp),
    ) -> OracleCounts {
        let c = self.c;
        let cv = &c.c;
        let mut f = cv.field.clone();
        let mut counts = OracleCounts::default();
        let (a, b, xr) = (cv.a, cv.b, r.x);
        let two = f.embed_u64(2);
        let four = f.embed_u64(4);
        let inv2 = f.inv(two).expect("odd p");
        // Per-target constants.
        let c2 = f.sqr(xr);
        let four_c2 = f.mul(four, c2);
        let two_a = f.add(a, a);
        let a2 = f.sqr(a);
        let four_b = f.mul(four, b);
        let four_bc = f.mul(four_b, xr);
        let two_c = f.add(xr, xr);
        let two_ac = f.mul(two_a, xr);
        let mut e1 = c.embed(start);
        let one = f.one();
        for _ in 0..count {
            counts.steps += 1;
            // −B_q = 4c² + 2e_1c + 2a;  C_q = e_1²c² − 2a·e_1c − 4bc + a² − 4b·e_1.
            let e1c = f.mul(e1, two_c);
            let t = f.add(four_c2, e1c);
            let mb = f.add(t, two_a);
            let e1sq = f.sqr(e1);
            let e1sq_c2 = f.mul(e1sq, c2);
            let t1 = f.mul(two_ac, e1);
            let t2 = f.mul(four_b, e1);
            let mut cq = f.sub(e1sq_c2, t1);
            cq = f.sub(cq, four_bc);
            cq = f.add(cq, a2);
            cq = f.sub(cq, t2);
            // disc = B_q² − 4C_q.
            let mb2 = f.sqr(mb);
            let fc = f.mul(four, cq);
            let disc = f.sub(mb2, fc);
            if let Some(s) = cv.sqrt(&mut f, disc) {
                counts.trace_pass += 1;
                let ms = f.neg(s);
                let roots = if f.is_zero(s) { vec![s] } else { vec![s, ms] };
                for sv in roots {
                    let num = f.add(mb, sv);
                    let e2 = f.mul(num, inv2);
                    // x_1, x_2 = (e_1 ± √(e_1² − 4e_2))/2.
                    let fe2 = f.mul(four, e2);
                    let d2 = f.sub(e1sq, fe2);
                    if f.is_zero(d2) {
                        continue; // x_1 = x_2
                    }
                    let Some(u) = cv.sqrt(&mut f, d2) else {
                        continue;
                    };
                    let p1 = f.add(e1, u);
                    let x1 = f.mul(p1, inv2);
                    let p2 = f.sub(e1, u);
                    let x2 = f.mul(p2, inv2);
                    let (k1, k2) = (c.key(&x1), c.key(&x2));
                    if !key_lt(&k1, &self.bound) || !key_lt(&k2, &self.bound) {
                        continue;
                    }
                    counts.in_domain += 1;
                    if let Some(d) = confirm(c, r, &x1, &x2) {
                        counts.decomps += 1;
                        emit(d);
                    }
                }
            }
            e1 = f.add(e1, one);
        }
        counts
    }
}

/// `x_1, x_2` are abscissae of curve points with `R = ±P_1 ± P_2`.
pub fn confirm(c: &PrimeW, r: &Pt<E3>, x1: &E3, x2: &E3) -> Option<PDecomp> {
    let p1 = c.lift_x(x1)?;
    for p in [p1, c.neg(&p1)] {
        if let Some(d) = c.add(r, &c.neg(&p)) {
            if d.x == *x2 {
                return Some(PDecomp::new(c.key(x1), c.key(x2)));
            }
        }
    }
    None
}

/// `BigUint` of a key (for reports).
pub fn key_value(k: &Key) -> BigUint {
    big_of_key(k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use std::collections::BTreeSet;

    #[test]
    fn prime_oracles_agree_on_calibration_curves() {
        let mut rng = StdRng::seed_from_u64(31);
        let mut found = 0;
        for bits in [18u32, 20] {
            let c = PrimeW::random(bits, &mut rng);
            let bound = BigUint::from(1u64 << (bits / 2 + 2));
            let base = PrimeBaseline::new(&c, &bound);
            let sym = PrimeSymmetric::new(&c, &bound);
            for _ in 0..6 {
                let r = c.random_point(&mut rng);
                let mut a = Vec::new();
                sym.sweep(&r, &BigUint::from(0u8), 2 << (bits / 2 + 2), |d| a.push(d));
                let set_a: BTreeSet<PDecomp> = a.iter().copied().collect();
                assert_eq!(set_a.len(), a.len(), "symmetric sweep repeated a pair");
                let mut b = BTreeSet::new();
                base.sweep(&r, &BigUint::from(0u8), 1 << (bits / 2 + 2), |d| {
                    b.insert(d);
                });
                b.retain(|d| d.x1 != d.x2);
                assert_eq!(set_a, b, "prime oracles disagree");
                found += a.len();
            }
        }
        assert!(found > 0, "vacuous: no decompositions");
    }

    #[test]
    fn planted_pair_found_on_eccp_131() {
        let (c, _, _) = PrimeW::eccp_131();
        let mut rng = StdRng::seed_from_u64(32);
        let bound = BigUint::from(1u64 << 40);
        let low = |rng: &mut StdRng| loop {
            use rand::Rng;
            let x = BigUint::from(rng.gen::<u64>() & ((1u64 << 40) - 1));
            if let Some(p) = c.lift_x(&c.embed(&x)) {
                return p;
            }
        };
        for _ in 0..3 {
            let p1 = low(&mut rng);
            let p2 = low(&mut rng);
            let r = c.add(&p1, &p2).unwrap();
            let e1 = key_value(&c.key(&p1.x)) + key_value(&c.key(&p2.x));
            let start = &e1 - BigUint::from(5u8);
            let mut got = Vec::new();
            PrimeSymmetric::new(&c, &bound).sweep(&r, &start, 11, |d| got.push(d));
            assert!(
                got.contains(&PDecomp::new(c.key(&p1.x), c.key(&p2.x))),
                "planted pair missed"
            );
        }
    }
}
