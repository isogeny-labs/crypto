//! Two-summand decomposition `R = ±P_1 ± P_2` over an `F_2`-subspace
//! factor base, with and without symmetry breaking, on any odd-degree
//! binary curve `y² + xy = x³ + a·x² + b` (ECC2-131 and the calibration
//! degrees).
//!
//! The factor base is `V_l = {x : deg x < l}` and the large-prime domain
//! `V_{l'}` with `l < l'` (Diem / Faugère–Perret–Petit–Renault subspace
//! bases; the low-degree choice is Petit–Quisquater's).  A decomposition
//! is a pair of abscissae `x_1, x_2 ∈ V_{l'}` of subgroup points with
//! `S_3(x_1, x_2, x_R) = 0`, where for these curves
//!
//! ```text
//! S_3(x_1, x_2, x_3) = (x_1x_2 + x_1x_3 + x_2x_3)² + x_1x_2x_3 + b.
//! ```
//!
//! It has 0, 1 or 2 large primes as 2, 1 or 0 of its abscissae lie in
//! `V_l`.
//!
//! # Symmetry breaking
//!
//! `S_3` is symmetric in `x_1, x_2`.  In the elementary symmetric
//! functions `e_1 = x_1 + x_2`, `e_2 = x_1x_2` and with `c = x_R` it reads
//! `(e_2 + c·e_1)² + c·e_2 + b = 0`, i.e. with `e_2 = c·t`
//!
//! ```text
//! t² + t = e_1² + b/c²,
//! ```
//!
//! and `x_1, x_2` are the roots of `X² + e_1X + e_2`, i.e. `X = e_1·s`
//! with `s² + s = e_2/e_1²`.  Because `V_{l'}` is closed under addition,
//! `x_1, x_2 ∈ V_{l'}` forces `e_1 ∈ V_{l'}`, and `x_1 ∈ V_{l'}` then gives
//! `x_2 = x_1 + e_1 ∈ V_{l'}`.  [`SymmetricOracle`] therefore enumerates
//! `e_1 ∈ V_{l'}` (each unordered pair once) instead of `x_1 ∈ V_{l'}`
//! (each unordered pair twice).  This is the `m! = 2` saving, and it is
//! special to subspace bases: an interval `[0, B)` of a prime field is not
//! closed under addition, `e_1` ranges over `[0, 2B)`, and the saving
//! vanishes (see the module report).
//!
//! The half-trace and the trace are `F_2`-linear, so along a Gray-code
//! order of `e_1` both `H(e_1²)` and `Tr(e_1)` change by one precomputed
//! word per step.  The per-step field work is one product for `e_2`, a
//! share of one batched inversion of `e_1`, one squaring and two products
//! for `e_2/e_1²`, and, for each `e_2` with `Tr(e_2/e_1²) = 0`, one
//! half-trace and one product for `x_1`.
//!
//! [`BaselineOracle`] is the unbroken reference: enumerate `x_1 ∈ V_{l'}`,
//! lift to `±P_1`, evaluate `x(R + P_1)` and `x(R − P_1)` with one shared
//! batched inversion ([`SymCurve::plus_minus_keys`]) and test membership.
//! Both report the same decompositions (a test checks it), so their step
//! rates compare directly.

use super::curves::{BinaryW, Pt, SymCurve};
use super::gf2w::{self, W3};

/// One decomposition found by an oracle, with both abscissae in `V_{l'}`
/// and `x_1 ≤ x_2` (as integers) so each unordered pair has one form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Decomp {
    pub x1: W3,
    pub x2: W3,
}

impl Decomp {
    fn new(a: W3, b: W3) -> Self {
        let key = |v: &W3| (v[2], v[1], v[0]);
        if key(&a) <= key(&b) {
            Self { x1: a, x2: b }
        } else {
            Self { x1: b, x2: a }
        }
    }

    /// Large primes against the factor base `V_l`: 0, 1 or 2.
    pub fn large_primes(&self, l: u32) -> u32 {
        !gf2w::degree_below(&self.x1, l) as u32 + !gf2w::degree_below(&self.x2, l) as u32
    }
}

/// Counters every oracle keeps, in units the report prices.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OracleCounts {
    /// Enumerated values (`e_1` or `x_1`).
    pub steps: u64,
    /// Steps that passed the first trace filter.
    pub trace_pass: u64,
    /// Candidate pairs whose abscissae both lie in `V_{l'}` (before the
    /// subgroup check).
    pub in_domain: u64,
    /// Decompositions over the subgroup (after the lift and sign check).
    pub decomps: u64,
}

impl OracleCounts {
    pub fn add(&mut self, o: &OracleCounts) {
        self.steps += o.steps;
        self.trace_pass += o.trace_pass;
        self.in_domain += o.in_domain;
        self.decomps += o.decomps;
    }
}

/// Whether `x_1, x_2` are abscissae of subgroup points with
/// `R = ±P_1 ± P_2`; returns the decomposition if so.
pub fn confirm(c: &BinaryW, r: &Pt<W3>, x1: &W3, x2: &W3) -> Option<Decomp> {
    let p1 = c.lift_x(x1)?;
    for p in [p1, c.neg(&p1)] {
        // R − P_1 should be ±P_2.
        if let Some(d) = c.add(r, &c.neg(&p)) {
            if d.x == *x2 && c.in_subgroup(&d) {
                return Some(Decomp::new(*x1, *x2));
            }
        }
    }
    None
}

/// Symmetry-broken enumeration over `e_1 = x_1 + x_2`.
pub struct SymmetricOracle<'c> {
    c: &'c BinaryW,
    /// Large-prime domain dimension `l'`.
    pub lp_dim: u32,
    /// `H(z^{2j})` and `Tr(z^j)` for the basis `z^j`, `j < l'`.
    ht_sq_basis: Vec<W3>,
    tr_basis: Vec<u32>,
    batch: usize,
}

impl<'c> SymmetricOracle<'c> {
    pub fn new(c: &'c BinaryW, lp_dim: u32) -> Self {
        assert!(c.f.n % 2 == 1, "the half-trace needs odd n");
        assert!(lp_dim < c.f.n, "l' must be below n");
        let f = &c.f;
        let ht_sq_basis = (0..lp_dim)
            .map(|j| f.half_trace(&f.sqr(&f.monomial(j))))
            .collect();
        let tr_basis = (0..lp_dim).map(|j| f.trace(&f.monomial(j))).collect();
        Self {
            c,
            lp_dim,
            ht_sq_basis,
            tr_basis,
            batch: 64,
        }
    }

    /// Sweep `e_1 = base ⊕ gray(i)` for `i ∈ [0, 2^k)`, where `base ∈
    /// V_{l'}` has its low `k` bits clear (`k ≤ 63`, `k ≤ l'`).  Calls
    /// `emit` for every confirmed decomposition.  Every unordered pair with
    /// `x_1 + x_2` in the swept coset is found exactly once.
    pub fn sweep(
        &self,
        r: &Pt<W3>,
        base: W3,
        k: u32,
        mut emit: impl FnMut(Decomp),
    ) -> OracleCounts {
        let c = self.c;
        let f = &c.f;
        assert!(k <= 63 && k <= self.lp_dim);
        assert!(base[0] & ((1u64 << k) - 1) == 0 || k == 0);
        assert!(gf2w::degree_below(&base, self.lp_dim));
        let mut counts = OracleCounts::default();
        let xr = r.x;
        if gf2w::is_zero(&xr) {
            return counts;
        }
        // β = b/c², with Tr(β) and H(β).
        let inv_c = f.inv(&xr);
        let beta = f.mul(&c.b, &f.sqr(&inv_c));
        let tr_beta = f.trace(&beta);
        let ht_beta = f.half_trace(&beta);
        // State at i = 0: e_1 = base.
        let mut e1 = base;
        let mut h = f.half_trace(&f.sqr(&base));
        let mut tr = f.trace(&base);

        let mut e1s: Vec<W3> = Vec::with_capacity(self.batch);
        let mut e2s: Vec<W3> = Vec::with_capacity(self.batch);
        let mut invs: Vec<W3> = Vec::with_capacity(self.batch);
        let mut scratch: Vec<W3> = Vec::with_capacity(self.batch);

        let mut flush = |e1s: &mut Vec<W3>,
                         e2s: &mut Vec<W3>,
                         invs: &mut Vec<W3>,
                         counts: &mut OracleCounts,
                         emit: &mut dyn FnMut(Decomp)| {
            invs.clear();
            invs.extend_from_slice(e1s);
            f.batch_inv(invs, &mut scratch);
            for ((e1, e2a), ie1) in e1s.iter().zip(e2s.iter()).zip(invs.iter()) {
                // e_2/e_1² for e_2 = c·t and e_2 + c.
                let ie2 = f.sqr(ie1);
                let ua = f.mul(e2a, &ie2);
                let w = f.mul(&xr, &ie2);
                for u in [ua, gf2w::xor(ua, w)] {
                    if f.trace(&u) != 0 {
                        continue;
                    }
                    let s = f.half_trace(&u);
                    let x1 = f.mul(e1, &s);
                    if !gf2w::degree_below(&x1, self.lp_dim) {
                        continue;
                    }
                    counts.in_domain += 1;
                    let x2 = gf2w::xor(x1, *e1);
                    if let Some(d) = confirm(c, r, &x1, &x2) {
                        counts.decomps += 1;
                        emit(d);
                    }
                }
            }
            e1s.clear();
            e2s.clear();
        };

        let n_steps: u64 = 1u64 << k;
        for i in 0..n_steps {
            if i > 0 {
                // gray(i) = gray(i−1) ⊕ 2^j with j = trailing zeros of i.
                let j = i.trailing_zeros() as usize;
                e1[0] ^= 1u64 << j;
                h = gf2w::xor(h, self.ht_sq_basis[j]);
                tr ^= self.tr_basis[j];
            }
            counts.steps += 1;
            // Tr(e_1² + β) = Tr(e_1) + Tr(β) must vanish; e_1 = 0 is the
            // doubling case x_1 = x_2, left out.
            if tr != tr_beta || gf2w::is_zero(&e1) {
                continue;
            }
            counts.trace_pass += 1;
            // t = H(e_1²) + H(β) solves t² + t = e_1² + β; e_2 = c·t.
            let t = gf2w::xor(h, ht_beta);
            e1s.push(e1);
            e2s.push(f.mul(&xr, &t));
            if e1s.len() == self.batch {
                flush(&mut e1s, &mut e2s, &mut invs, &mut counts, &mut emit);
            }
        }
        if !e1s.is_empty() {
            flush(&mut e1s, &mut e2s, &mut invs, &mut counts, &mut emit);
        }
        counts
    }
}

/// Unbroken reference: enumerate `x_1 ∈ V_{l'}`, lift, and test
/// `x(R ± P_1) ∈ V_{l'}` with one shared batched inversion per pair.
pub struct BaselineOracle<'c> {
    c: &'c BinaryW,
    pub lp_dim: u32,
    batch: usize,
}

impl<'c> BaselineOracle<'c> {
    pub fn new(c: &'c BinaryW, lp_dim: u32) -> Self {
        assert!(lp_dim < c.f.n);
        Self {
            c,
            lp_dim,
            batch: 64,
        }
    }

    /// Sweep `x_1 = base ⊕ i` for `i ∈ [0, 2^k)` (same contract as
    /// [`SymmetricOracle::sweep`]).  A pair with both abscissae in the swept
    /// block is reported twice (once from each end); `emit` sees both and a
    /// caller deduplicates.
    pub fn sweep(
        &self,
        r: &Pt<W3>,
        base: W3,
        k: u32,
        mut emit: impl FnMut(Decomp),
    ) -> OracleCounts {
        let c = self.c;
        assert!(k <= 63 && k <= self.lp_dim);
        assert!(gf2w::degree_below(&base, self.lp_dim));
        let mut counts = OracleCounts::default();
        let mut pts: Vec<Pt<W3>> = Vec::with_capacity(self.batch);
        let mut keys = Vec::with_capacity(self.batch);
        let mut flush = |pts: &mut Vec<Pt<W3>>, counts: &mut OracleCounts| {
            c.plus_minus_keys(r, pts, &mut keys);
            for (p, pm) in pts.iter().zip(keys.iter()) {
                for x2 in pm {
                    if !gf2w::degree_below(x2, self.lp_dim) {
                        continue;
                    }
                    counts.in_domain += 1;
                    // R ± P_1 = ±P_2 with P_1, R in the subgroup puts P_2 there.
                    counts.decomps += 1;
                    emit(Decomp::new(p.x, *x2));
                }
            }
            pts.clear();
        };
        for i in 0..(1u64 << k) {
            counts.steps += 1;
            let mut x1 = base;
            x1[0] ^= i;
            // Lift: needs Tr(x + a + b/x²) = 0 and the subgroup trace.
            let Some(p) = c.lift_x(&x1) else { continue };
            counts.trace_pass += 1;
            pts.push(p);
            if pts.len() == self.batch {
                flush(&mut pts, &mut counts);
            }
        }
        if !pts.is_empty() {
            flush(&mut pts, &mut counts);
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::BTreeSet;

    /// On a small domain both oracles find the same unordered pairs, and
    /// the symmetric one finds each once.
    fn agree(c: &BinaryW, lp_dim: u32, seed: u64) -> usize {
        let mut rng = StdRng::seed_from_u64(seed);
        let sym = SymmetricOracle::new(c, lp_dim);
        let base = BaselineOracle::new(c, lp_dim);
        let mut total = 0;
        for _ in 0..6 {
            let r = c.random_point(&mut rng);
            let mut a = Vec::new();
            sym.sweep(&r, gf2w::ZERO, lp_dim, |d| a.push(d));
            let set_a: BTreeSet<Decomp> = a.iter().copied().collect();
            assert_eq!(set_a.len(), a.len(), "symmetric oracle repeated a pair");
            let mut b = BTreeSet::new();
            base.sweep(&r, gf2w::ZERO, lp_dim, |d| {
                b.insert(d);
            });
            // The baseline also reports doublings x_1 = x_2 (R = 2P_1),
            // which the symmetric sweep leaves out (e_1 = 0).
            b.retain(|d| d.x1 != d.x2);
            assert_eq!(set_a, b, "oracles disagree");
            total += a.len();
        }
        total
    }

    #[test]
    fn oracles_agree_on_calibration_curves() {
        let mut rng = StdRng::seed_from_u64(11);
        let mut found = 0;
        for n in [17u32, 19, 23] {
            let c = BinaryW::random(n, &mut rng);
            found += agree(&c, n / 2 + 2, rng.gen());
        }
        assert!(found > 0, "no decompositions at all: the test is vacuous");
    }

    /// A pair planted on ECC2-131 itself is found by the symmetric oracle
    /// when the sweep covers its `e_1`.
    #[test]
    fn planted_pair_found_on_ecc2_131() {
        let (c, _, _) = BinaryW::ecc2_131();
        let mut rng = StdRng::seed_from_u64(12);
        let lp_dim = 40;
        let k = 12;
        for _ in 0..4 {
            // Two subgroup points with abscissae in V_{l'}.
            let lift_low = |rng: &mut StdRng| loop {
                let x = [rng.gen::<u64>() & ((1u64 << 40) - 1), 0, 0];
                if let Some(p) = c.lift_x(&x) {
                    return p;
                }
            };
            let p1 = lift_low(&mut rng);
            let p2 = lift_low(&mut rng);
            let r = c.add(&p1, &p2).unwrap();
            let e1 = gf2w::xor(p1.x, p2.x);
            let base = [e1[0] & !((1u64 << k) - 1), 0, 0];
            let mut got = Vec::new();
            SymmetricOracle::new(&c, lp_dim).sweep(&r, base, k, |d| got.push(d));
            assert!(
                got.contains(&Decomp::new(p1.x, p2.x)),
                "planted pair missed"
            );
        }
    }
}
