//! # A field-agnostic front door for hyperelliptic discrete logarithms.
//!
//! [`crate::cryptanalysis::hyperelliptic_index_calculus`] solves a DLP in
//! a prime-order subgroup of `Jac(C)(F_p)`; the characteristic-two engine
//! [`crate::cryptanalysis::binary_hyperelliptic_index_calculus`] does the
//! same over `F_{2^m}`.  This module adds the two things a *tool* needs on
//! top of those engines:
//!
//! - a [`HecGroup`] abstraction so that order finding, baby-step giant-step
//!   and Pohlig–Hellman are written once and run on both field families;
//! - Pohlig–Hellman over an arbitrary composite subgroup order, using
//!   index calculus only in the prime-order components that are large
//!   enough to need it and baby-step giant-step in the small ones.
//!
//! Nothing here changes the underlying solves.  It composes them, verifies
//! every recovered scalar by scalar multiplication, and reports which
//! method produced each residue.
//!
//! ## Scope
//!
//! - Prime-field curves: odd characteristic, `deg f = 2g+1`, `p` at most
//!   64 bits (the arithmetic substrate's checked range).
//! - Binary-field curves: `deg f = 2g+1`, `deg h <= g`, any `m` the field
//!   arithmetic accepts.
//! - Factoring the subgroup order is budgeted trial division plus Pollard
//!   rho; callers that already know the factorisation can pass it in and
//!   skip the search.
//! - The recovered scalar is always checked with `[k]P = Q` before it is
//!   returned.

use std::collections::HashMap;
use std::hash::Hash;

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::binary_ecc::hyperelliptic::{HyperellipticCurve, MumfordDivisor};
use crate::binary_ecc::poly_f2m::F2mPoly;
use crate::cryptanalysis::binary_hyperelliptic_index_calculus::{
    binary_hec_index_calculus_dlp, BhecIndexCalculusParams,
};
use crate::cryptanalysis::curve_traits::arith::is_prime;
use crate::cryptanalysis::hyperelliptic_index_calculus::{
    hec_index_calculus_dlp, HecIndexCalculusParams, HecIndexCalculusReport, RelationSearch,
    SmoothnessTest,
};
use crate::cryptanalysis::pohlig_hellman::crt_combine;
use crate::prime_hyperelliptic::{HyperellipticCurveP, MumfordDivisorP};

/// Configuration for one solve.
#[derive(Clone, Debug)]
pub struct HecSolveConfig {
    /// Index-calculus factor-base cap.  `0` lets each engine choose (full
    /// base on small fields, Gaudry's reduced base otherwise).
    pub fb_size: usize,
    /// Maximum relation-search trials per prime-order component.
    pub max_trials: usize,
    /// Extra rows collected beyond the number of unknowns.
    pub extra_relations: usize,
    pub seed: u64,
    /// Trial-division bound for factoring the subgroup order.
    pub trial_factor_bound: u64,
    /// Baby-step giant-step memory cap, in table entries.
    pub bsgs_max_steps: usize,
    /// Prime components at or below this bound use BSGS instead of index
    /// calculus.  BSGS cost is `sqrt(q)`; IC has a much larger constant.
    pub small_prime_bound: u64,
    /// Relation-collection threads for the binary-field engine.  The prime
    /// engine's counted walk is serial; values above one are ignored there.
    pub threads: usize,
}

impl Default for HecSolveConfig {
    fn default() -> Self {
        Self {
            fb_size: 0,
            max_trials: 500_000,
            extra_relations: 8,
            seed: 1,
            trial_factor_bound: 1 << 16,
            bsgs_max_steps: 2_000_000,
            small_prime_bound: 1 << 20,
            threads: 1,
        }
    }
}

/// One prime-order solve attempted by the driver.
#[derive(Clone, Debug)]
pub struct HecPrimeCall {
    pub order: BigUint,
    /// `"bsgs"` or `"index-calculus"`.
    pub method: &'static str,
    pub scalar: Option<BigUint>,
    pub bsgs_steps: Option<usize>,
    pub index_calculus: Option<HecIndexCalculusReport>,
}

/// The pieces of a full discrete-logarithm result.
#[derive(Clone, Debug)]
pub struct HecSolveReport {
    pub order: BigUint,
    pub factors: Vec<(BigUint, u32)>,
    pub factorisation_complete: bool,
    pub residues: Vec<(BigUint, BigUint)>,
    pub prime_calls: Vec<HecPrimeCall>,
    pub verified: bool,
    pub notes: Vec<String>,
}

/// The group operations the generic driver needs.
///
/// This is deliberately a small interface: it is the intersection of the
/// prime-field and binary-field Jacobian implementations, not an attempt to
/// hide their arithmetic.
pub trait HecGroup {
    type Elem: Clone + std::fmt::Debug;
    type Key: Hash + Eq + Clone;

    fn identity(&self) -> Self::Elem;
    fn add(&self, a: &Self::Elem, b: &Self::Elem) -> Self::Elem;
    fn neg(&self, a: &Self::Elem) -> Self::Elem;
    fn scalar_mul(&self, a: &Self::Elem, k: &BigUint) -> Self::Elem;
    fn is_identity(&self, a: &Self::Elem) -> bool;
    fn same(&self, a: &Self::Elem, b: &Self::Elem) -> bool;
    fn key(&self, a: &Self::Elem) -> Self::Key;
    fn describe(&self) -> String;
    fn elem_json(&self, a: &Self::Elem) -> serde_json::Value;

    /// Solve `target = k · base` in the prime-order subgroup generated by
    /// `base`.
    fn solve_prime_order(
        &self,
        base: &Self::Elem,
        target: &Self::Elem,
        order: &BigUint,
        cfg: &HecSolveConfig,
    ) -> HecPrimeCall;
}

impl HecGroup for HyperellipticCurveP {
    type Elem = MumfordDivisorP;
    type Key = (Vec<Vec<u32>>, Vec<Vec<u32>>);

    fn identity(&self) -> Self::Elem {
        MumfordDivisorP::identity(self.p.clone())
    }

    fn add(&self, a: &Self::Elem, b: &Self::Elem) -> Self::Elem {
        a.add_fast(b, self)
    }

    fn neg(&self, a: &Self::Elem) -> Self::Elem {
        a.neg_fast(self)
    }

    fn scalar_mul(&self, a: &Self::Elem, k: &BigUint) -> Self::Elem {
        a.scalar_mul_fast(k, self)
    }

    fn is_identity(&self, a: &Self::Elem) -> bool {
        a.is_identity()
    }

    fn same(&self, a: &Self::Elem, b: &Self::Elem) -> bool {
        a == b
    }

    fn key(&self, a: &Self::Elem) -> Self::Key {
        (
            a.u.coeffs.iter().map(|c| c.to_u32_digits()).collect(),
            a.v.coeffs.iter().map(|c| c.to_u32_digits()).collect(),
        )
    }

    fn describe(&self) -> String {
        format!("Jac(y^2=f(x)) over F_p, p={}, genus={}", self.p, self.genus)
    }

    fn elem_json(&self, a: &Self::Elem) -> serde_json::Value {
        serde_json::json!({
            "u": a.u.coeffs.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            "v": a.v.coeffs.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        })
    }

    fn solve_prime_order(
        &self,
        base: &Self::Elem,
        target: &Self::Elem,
        order: &BigUint,
        cfg: &HecSolveConfig,
    ) -> HecPrimeCall {
        let params = HecIndexCalculusParams {
            fb_size: if cfg.fb_size == 0 {
                usize::MAX
            } else {
                cfg.fb_size
            },
            extra_relations: cfg.extra_relations,
            max_trials: cfg.max_trials,
            seed: cfg.seed,
            search: RelationSearch::factor_base_walk(),
            linear_algebra:
                crate::cryptanalysis::hyperelliptic_index_calculus::LinearAlgebra::Sparse,
            smoothness: SmoothnessTest::Gcd,
            large_primes: false,
        };
        let (scalar, report) = hec_index_calculus_dlp(self, base, target, order, &params);
        HecPrimeCall {
            order: order.clone(),
            method: "index-calculus",
            scalar,
            bsgs_steps: None,
            index_calculus: Some(report),
        }
    }
}

impl HecGroup for HyperellipticCurve {
    type Elem = MumfordDivisor;
    type Key = (Vec<Vec<u8>>, Vec<Vec<u8>>);

    fn identity(&self) -> Self::Elem {
        MumfordDivisor::identity(self.m)
    }

    fn add(&self, a: &Self::Elem, b: &Self::Elem) -> Self::Elem {
        a.add_fast(b, self)
    }

    fn neg(&self, a: &Self::Elem) -> Self::Elem {
        a.neg_fast(self)
    }

    fn scalar_mul(&self, a: &Self::Elem, k: &BigUint) -> Self::Elem {
        a.scalar_mul_fast(k, self)
    }

    fn is_identity(&self, a: &Self::Elem) -> bool {
        a.is_identity()
    }

    fn same(&self, a: &Self::Elem, b: &Self::Elem) -> bool {
        a == b
    }

    fn key(&self, a: &Self::Elem) -> Self::Key {
        (
            a.u.coeffs
                .iter()
                .map(|c| c.to_biguint().to_bytes_be())
                .collect(),
            a.v.coeffs
                .iter()
                .map(|c| c.to_biguint().to_bytes_be())
                .collect(),
        )
    }

    fn describe(&self) -> String {
        format!(
            "Jac(y^2+h(x)y=f(x)) over F_2^{}, genus={}",
            self.m, self.genus
        )
    }

    fn elem_json(&self, a: &Self::Elem) -> serde_json::Value {
        let hex = |p: &F2mPoly| -> Vec<String> {
            p.coeffs
                .iter()
                .map(|c| format!("{:x}", c.to_biguint()))
                .collect()
        };
        serde_json::json!({"u": hex(&a.u), "v": hex(&a.v)})
    }

    fn solve_prime_order(
        &self,
        base: &Self::Elem,
        target: &Self::Elem,
        order: &BigUint,
        cfg: &HecSolveConfig,
    ) -> HecPrimeCall {
        let params = BhecIndexCalculusParams {
            fb_size: cfg.fb_size,
            extra_relations: cfg.extra_relations,
            max_trials: cfg.max_trials,
            seed: cfg.seed,
            threads: cfg.threads,
            ..Default::default()
        };
        let (scalar, report) = binary_hec_index_calculus_dlp(self, base, target, order, &params);
        HecPrimeCall {
            order: order.clone(),
            method: "index-calculus",
            scalar,
            bsgs_steps: None,
            index_calculus: Some(report),
        }
    }
}

/// Baby-step giant-step for `target = k · base` in a group of known order.
///
/// Returns `(k, table_entries)`; `None` when the table would exceed
/// [`HecSolveConfig::bsgs_max_steps`] or the answer fails verification.
pub fn bsgs<G: HecGroup>(
    g: &G,
    base: &G::Elem,
    target: &G::Elem,
    order: &BigUint,
    max_steps: usize,
) -> Option<(BigUint, usize)> {
    if order.is_zero() {
        return None;
    }
    if order.is_one() {
        return g.is_identity(target).then(|| (BigUint::zero(), 0));
    }
    let steps = order.sqrt() + BigUint::one();
    let steps_u = match steps.to_u64_digits().as_slice() {
        [v] => *v as usize,
        _ => return None,
    };
    if steps_u > max_steps {
        return None;
    }

    let mut table: HashMap<G::Key, usize> = HashMap::with_capacity(steps_u);
    let mut acc = g.identity();
    for j in 0..steps_u {
        table.entry(g.key(&acc)).or_insert(j);
        acc = g.add(&acc, base);
    }
    let stride = g.scalar_mul(base, &BigUint::from(steps_u));
    let neg_stride = g.neg(&stride);
    let mut cur = target.clone();
    for i in 0..=steps_u {
        if let Some(&j) = table.get(&g.key(&cur)) {
            let k = BigUint::from(i as u64) * BigUint::from(steps_u) + BigUint::from(j);
            if &k < order && g.same(&g.scalar_mul(base, &k), target) {
                return Some((k, steps_u));
            }
        }
        cur = g.add(&cur, &neg_stride);
    }
    None
}

/// Solve a DLP of arbitrary composite order by Pohlig–Hellman.
///
/// `factors` may be supplied by the caller; when it is `None` the order is
/// factored with a budgeted trial division plus Pollard rho.
pub fn pohlig_hellman<G: HecGroup>(
    g: &G,
    base: &G::Elem,
    target: &G::Elem,
    order: &BigUint,
    factors: Option<Vec<(BigUint, u32)>>,
    cfg: &HecSolveConfig,
) -> (Option<BigUint>, HecSolveReport) {
    let mut report = HecSolveReport {
        order: order.clone(),
        factors: Vec::new(),
        factorisation_complete: false,
        residues: Vec::new(),
        prime_calls: Vec::new(),
        verified: false,
        notes: Vec::new(),
    };
    if order.is_zero() {
        report.notes.push("order is zero".into());
        return (None, report);
    }
    if order.is_one() {
        let ok = g.is_identity(target);
        report.verified = ok;
        return (ok.then(BigUint::zero), report);
    }

    let (factors, complete) = match factors {
        Some(f) => (normalise_factors(f), true),
        None => factor_order(order, cfg.trial_factor_bound, cfg.seed),
    };
    report.factors = factors.clone();
    report.factorisation_complete = complete;
    if !complete {
        report
            .notes
            .push("order factorisation is incomplete; pass a known factorisation".into());
        return (None, report);
    }
    let product = factors
        .iter()
        .map(|(q, e)| q.pow(*e))
        .fold(BigUint::one(), |acc, v| acc * v);
    if &product != order {
        report
            .notes
            .push("the supplied factors do not multiply to the stated order".into());
        return (None, report);
    }

    let mut residues = Vec::new();
    let mut all_ok = true;
    for (q, e) in &factors {
        let qe = q.pow(*e);
        if (order % &qe) != BigUint::zero() {
            report
                .notes
                .push(format!("factor {q}^{e} does not divide the stated order"));
            all_ok = false;
            break;
        }
        let cofactor = order / &qe;
        let base_i = g.scalar_mul(base, &cofactor);
        let target_i = g.scalar_mul(target, &cofactor);
        if g.is_identity(&base_i) {
            if g.is_identity(&target_i) {
                residues.push((qe, BigUint::zero()));
            } else {
                report
                    .notes
                    .push(format!("base has no component of order {q}^{e}"));
                all_ok = false;
                break;
            }
            continue;
        }

        let d = if *e == 1 {
            let call = if fits_u64(q).is_some_and(|v| v <= cfg.small_prime_bound) {
                bsgs_call(g, &base_i, &target_i, q, cfg)
            } else {
                g.solve_prime_order(&base_i, &target_i, q, cfg)
            };
            let Some(k) = call.scalar.clone() else {
                report.prime_calls.push(call);
                all_ok = false;
                break;
            };
            report.prime_calls.push(call);
            k
        } else {
            // Digit lifting: solve modulo q, then lift one base-q digit at
            // a time.  Every inner solve is in the order-q subgroup.
            let base_q = g.scalar_mul(&base_i, &q.pow(*e - 1));
            let mut d = BigUint::zero();
            let mut qpow = BigUint::one();
            let mut ok = true;
            for j in 0..*e {
                let d_base = g.scalar_mul(&base_i, &d);
                let diff = g.add(&target_i, &g.neg(&d_base));
                let lifted = g.scalar_mul(&diff, &q.pow(*e - 1 - j));
                let call = if fits_u64(q).is_some_and(|v| v <= cfg.small_prime_bound) {
                    bsgs_call(g, &base_q, &lifted, q, cfg)
                } else {
                    g.solve_prime_order(&base_q, &lifted, q, cfg)
                };
                let Some(c) = call.scalar.clone() else {
                    report.prime_calls.push(call);
                    ok = false;
                    break;
                };
                report.prime_calls.push(call);
                d += &c * &qpow;
                qpow *= q;
                let _ = j;
            }
            if !ok {
                all_ok = false;
                break;
            }
            d % &qe
        };
        residues.push((qe, d));
    }

    report.residues = residues.clone();
    if !all_ok {
        return (None, report);
    }
    let Some(k) = crt_combine(&residues) else {
        report.notes.push("CRT reconstruction failed".into());
        return (None, report);
    };
    if &k < order && g.same(&g.scalar_mul(base, &k), target) {
        report.verified = true;
        (Some(k), report)
    } else {
        report
            .notes
            .push("recovered scalar failed [k]P = Q verification".into());
        (None, report)
    }
}

fn bsgs_call<G: HecGroup>(
    g: &G,
    base: &G::Elem,
    target: &G::Elem,
    order: &BigUint,
    cfg: &HecSolveConfig,
) -> HecPrimeCall {
    let (scalar, steps) = match bsgs(g, base, target, order, cfg.bsgs_max_steps) {
        Some((k, steps)) => (Some(k), Some(steps)),
        None => (None, None),
    };
    HecPrimeCall {
        order: order.clone(),
        method: "bsgs",
        scalar,
        bsgs_steps: steps,
        index_calculus: None,
    }
}

/// Factor an order by trial division and Pollard rho.
///
/// Returns `(factors, complete)`.  A false `complete` means the remaining
/// cofactor could not be split within the budget; it is still returned as
/// a pseudo-factor so the caller can report it.
pub fn factor_order(n: &BigUint, trial_bound: u64, seed: u64) -> (Vec<(BigUint, u32)>, bool) {
    let mut out: Vec<(BigUint, u32)> = Vec::new();
    let mut rest = n.clone();
    let bound = BigUint::from(trial_bound.max(2));
    let mut d = BigUint::from(2u32);
    while d <= bound && &d * &d <= rest {
        let mut e = 0u32;
        while (&rest % &d).is_zero() {
            rest /= &d;
            e += 1;
        }
        if e > 0 {
            out.push((d.clone(), e));
        }
        d += 1u32;
    }
    let mut complete = true;
    if rest > BigUint::one() {
        let mut rng = StdRng::seed_from_u64(seed ^ 0xf00d_1e5b_a11e_900d);
        let mut stack = vec![rest];
        let mut guard = 0usize;
        while let Some(value) = stack.pop() {
            guard += 1;
            if guard > 4096 {
                complete = false;
                out.push((value, 1));
                continue;
            }
            if value.is_one() {
                continue;
            }
            if is_prime(&value) {
                out.push((value, 1));
                continue;
            }
            match pollard_rho(&value, &mut rng, 24) {
                Some(factor) => {
                    stack.push(factor.clone());
                    stack.push(value / factor);
                }
                None => {
                    complete = false;
                    out.push((value, 1));
                }
            }
        }
    }
    combine_factors(&mut out);
    (out, complete)
}

fn combine_factors(factors: &mut Vec<(BigUint, u32)>) {
    factors.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out: Vec<(BigUint, u32)> = Vec::with_capacity(factors.len());
    for (q, e) in factors.drain(..) {
        if let Some(last) = out.last_mut() {
            if last.0 == q {
                last.1 += e;
                continue;
            }
        }
        out.push((q, e));
    }
    *factors = out;
}

fn normalise_factors(mut factors: Vec<(BigUint, u32)>) -> Vec<(BigUint, u32)> {
    combine_factors(&mut factors);
    factors
}

fn pollard_rho(n: &BigUint, rng: &mut StdRng, attempts: usize) -> Option<BigUint> {
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    if (n % &two).is_zero() {
        return Some(two);
    }
    if n <= &one {
        return None;
    }
    for _ in 0..attempts {
        let c = rand_below(rng, n) % (n - &one) + &one;
        let mut x = rand_below(rng, n);
        if x.is_zero() {
            x = two.clone();
        }
        let mut y = x.clone();
        for _ in 0..1_000_000usize {
            x = (&x * &x + &c) % n;
            y = (&y * &y + &c) % n;
            y = (&y * &y + &c) % n;
            let diff = if x >= y { &x - &y } else { &y - &x };
            let d = diff.gcd(n);
            if d > one && &d < n {
                return Some(d);
            }
            if d == *n {
                break;
            }
        }
    }
    None
}

fn fits_u64(n: &BigUint) -> Option<u64> {
    match n.to_u64_digits().as_slice() {
        [] => Some(0),
        [v] => Some(*v),
        _ => None,
    }
}

fn rand_below(rng: &mut StdRng, n: &BigUint) -> BigUint {
    if n.is_zero() {
        return BigUint::zero();
    }
    let bytes = (n.bits() as usize).div_ceil(8) + 8;
    let mut buf = vec![0u8; bytes];
    rng.fill_bytes(&mut buf);
    BigUint::from_bytes_be(&buf) % n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prime_hyperelliptic::FpPoly;

    fn prime_toy_curve(p: u64) -> HyperellipticCurveP {
        let p = BigUint::from(p);
        let f = FpPoly::from_coeffs(
            vec![
                BigUint::one(),
                BigUint::one(),
                BigUint::from(2u32),
                BigUint::from(3u32),
                BigUint::zero(),
                BigUint::one(),
            ],
            p.clone(),
        );
        HyperellipticCurveP::new(p, f, 2)
    }

    #[test]
    fn crt_and_factor_order_round_trip() {
        let n = BigUint::from(2u32 * 3 * 5 * 7 * 11 * 13);
        let (factors, complete) = factor_order(&n, 100, 1);
        assert!(complete);
        assert_eq!(
            factors,
            vec![
                (BigUint::from(2u32), 1),
                (BigUint::from(3u32), 1),
                (BigUint::from(5u32), 1),
                (BigUint::from(7u32), 1),
                (BigUint::from(11u32), 1),
                (BigUint::from(13u32), 1),
            ]
        );
    }

    #[test]
    fn pohlig_hellman_recovers_a_prime_curve_scalar() {
        use crate::cryptanalysis::hyperelliptic_index_calculus::{
            build_factor_base, divisor_order_bsgs,
        };
        let curve = prime_toy_curve(41);
        let fb = build_factor_base(&curve, 8);
        let base = fb.entries[0].divisor.clone();
        let order = divisor_order_bsgs(&curve, &base, 10_000).expect("order");
        let k = BigUint::from(7u32) % &order;
        let target = base.scalar_mul_fast(&k, &curve);
        let cfg = HecSolveConfig {
            bsgs_max_steps: 10_000,
            small_prime_bound: 1 << 20,
            ..Default::default()
        };
        let (answer, report) = pohlig_hellman(&curve, &base, &target, &order, None, &cfg);
        assert_eq!(answer, Some(k), "report: {report:?}");
        assert!(report.verified);
    }
}
