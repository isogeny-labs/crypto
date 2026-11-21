//! # Selectable-\(k\) / \(k(p)\) Diem extension-IC harness (lab slice).
//!
//! Extends [`crate::cryptanalysis::diem_descent`] so the extension degree
//! `k` is a **runtime parameter** (or a Diem-style `k(p)` schedule), not
//! hard-wired to 2/3/4 silos.  The first slice is a brute-force
//! `k`-summand decomposition cell on tiny `E(F_{p^k})` with a catalog
//! cost estimate vs matched \(\sqrt{r}\)-scale rho.
//!
//! ## Predicted cost shape (falsifiable)
//!
//! Catalog Gaudry–Diem / Diem stand-in (same family as
//! `jacobian_transfer::cost::log2_extension_ic`):
//!
//! ```text
//!   log2 C(p,k) ≈ (2 - 2/k)·log2(p) + k(k-1) + elliptic_ops(p^k)
//!   log2 ρ      ≈ (1/2)·log2(r) + elliptic_ops   on the same subgroup
//! ```
//!
//! Diem's asymptotic story needs **growing** `k ≈ √(ln p / ln ln p)`.
//! Fixed `k` is polynomial-in-`p` and cannot yield goal \(L_p[\alpha]\)
//! with \(\alpha < 1/2\).  This harness exists so that schedule can be
//! executed and the α claim tested — not to redefine success as
//! “fixed-k works”.
//!
//! ## Same-target absolute \(\mathbb{F}_p\) note
//!
//! Applying this catalog route to an `E(F_p)` subgroup of size
//! `Θ(p)` cannot beat same-target rho under the present formula
//! (relation term ≥ `p` for `k ≥ 2`).  Lab cells here are
//! **extension-field targets**.  Absolute-\(\mathbb{F}_p\) remains a
//! separate construction obligation.
//!
//! ## What this slice is / is not
//!
//! - **Is:** selectable `k`, `k_of_p` schedule helper, brute `k`-decomp
//!   for tiny rungs, JSON-friendly cell report, unit + example smoke.
//! - **Is not:** Gröbner Semaev solver, ecbench method yet, or a proven
//!   \(L_p[\alpha]\) claim.

use crate::cryptanalysis::diem_descent::{
    build_factor_base, find_2_decomposition, ECurveFpk, Fpk, Pt,
};
use num_bigint::BigUint;
use serde::Serialize;

/// Diem-style heuristic \(k(p) ≈ \sqrt{\ln p / \ln\ln p}\), clamped.
pub fn k_of_p(p: u64, k_min: u32, k_max: u32) -> u32 {
    assert!(k_min >= 2 && k_max >= k_min);
    if p < 3 {
        return k_min;
    }
    let ln_p = (p as f64).ln();
    if ln_p <= std::f64::consts::E {
        return k_min;
    }
    let lnln = ln_p.ln();
    if lnln <= 0.0 {
        return k_min;
    }
    let k = (ln_p / lnln).sqrt().round() as u32;
    k.clamp(k_min, k_max)
}

/// Catalog `log2` of Gaudry–Diem stand-in cost (bezout exponent = 1).
pub fn log2_extension_ic_catalog(p_bits: f64, k: u32) -> f64 {
    let k_f = f64::from(k);
    let leading = (2.0 - 2.0 / k_f) * p_bits + k_f * (k_f - 1.0);
    // Elliptic op over F_{p^k}: ~10 Karatsuba field mults (matches cost model spirit).
    let field_bits = p_bits * k_f;
    let karatsuba = field_bits.powf(1.585);
    leading + (10.0 * karatsuba).log2()
}

/// Matched rho `log2` stand-in: `(1/2) log2(r)` plus the same elliptic op.
pub fn log2_rho_catalog(subgroup_bits: f64, field_bits: f64) -> f64 {
    0.5 * subgroup_bits + (10.0 * field_bits.powf(1.585)).log2()
}

/// Search for `c` such that `θ^k - c` is irreducible over `F_p` (binomial).
///
/// For prime `k`, it is enough that `x^k - c` has no root in `F_p`.
pub fn find_binomial_irr(p: u64, k: u32) -> Option<u64> {
    if p < 3 || k < 2 {
        return None;
    }
    for c in 1..p {
        if binomial_has_root(p, k, c) {
            continue;
        }
        // For prime degree, no root ⇒ irreducible.
        if is_prime_u32(k) {
            return Some(c);
        }
        // Even degree / composite: accept no-root as a lab filter only when
        // k is prime; otherwise require a fuller irreducibility test later.
        if k == 4 {
            // Reject if it factors as quadratics: skip for now.
            continue;
        }
    }
    None
}

fn binomial_has_root(p: u64, k: u32, c: u64) -> bool {
    for x in 0..p {
        if mod_pow(x, u64::from(k), p) == c % p {
            return true;
        }
    }
    false
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut acc = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = (acc as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    acc
}

fn is_prime_u32(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3u32;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

/// A successful `k`-summand decomposition `R = Σ ε_i FB[i_j]`.
#[derive(Clone, Debug, Serialize)]
pub struct KDecomposition {
    pub indices: Vec<usize>,
    pub signs: Vec<i32>,
}

/// Brute-force search for a `k`-summand factor-base decomposition.
///
/// `k == 2` delegates to [`find_2_decomposition`].  Larger `k` uses a
/// depth-`k` signed search capped by `trial_budget` additions.
pub fn find_k_decomposition(
    curve: &ECurveFpk,
    fb: &[Pt],
    target: &Pt,
    k: u32,
    trial_budget: u64,
) -> Option<KDecomposition> {
    assert!(k >= 2);
    if k == 2 {
        return find_2_decomposition(curve, fb, target).map(|d| KDecomposition {
            indices: vec![d.indices.0, d.indices.1],
            signs: vec![d.signs.0, d.signs.1],
        });
    }
    let mut trials = 0u64;
    let mut indices = Vec::with_capacity(k as usize);
    let mut signs = Vec::with_capacity(k as usize);
    if search_k(
        curve,
        fb,
        target,
        k as usize,
        &Pt::identity(),
        &mut indices,
        &mut signs,
        &mut trials,
        trial_budget,
    ) {
        Some(KDecomposition { indices, signs })
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
fn search_k(
    curve: &ECurveFpk,
    fb: &[Pt],
    target: &Pt,
    remaining: usize,
    acc: &Pt,
    indices: &mut Vec<usize>,
    signs: &mut Vec<i32>,
    trials: &mut u64,
    budget: u64,
) -> bool {
    if *trials >= budget {
        return false;
    }
    if remaining == 0 {
        *trials += 1;
        return acc == target;
    }
    for (i, p) in fb.iter().enumerate() {
        for s in [1i32, -1] {
            if *trials >= budget {
                return false;
            }
            let ps = if s == -1 { p.neg() } else { p.clone() };
            let next = curve.add(acc, &ps);
            *trials += 1;
            indices.push(i);
            signs.push(s);
            if search_k(
                curve,
                fb,
                target,
                remaining - 1,
                &next,
                indices,
                signs,
                trials,
                budget,
            ) {
                return true;
            }
            indices.pop();
            signs.pop();
        }
    }
    false
}

/// One lab cell: selectable `(p,k)` or schedule-derived `k`.
#[derive(Clone, Debug, Serialize)]
pub struct CellRequest {
    pub p: u64,
    /// If `None`, use [`k_of_p`] with `(k_min,k_max)`.
    pub k: Option<u32>,
    pub k_min: u32,
    pub k_max: u32,
    pub trial_budget: u64,
    pub planted_decompositions: u32,
    pub seed: u64,
}

impl Default for CellRequest {
    fn default() -> Self {
        Self {
            p: 7,
            k: Some(3),
            k_min: 2,
            k_max: 6,
            trial_budget: 2_000_000,
            planted_decompositions: 8,
            seed: 1,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CellReport {
    pub schema: &'static str,
    pub standing: &'static str,
    pub p: u64,
    pub k: u32,
    pub k_from_schedule: bool,
    pub schedule_k_of_p: u32,
    pub irr_const: u64,
    pub fb_size: usize,
    pub planted: u32,
    pub recovered: u32,
    pub trials_used: u64,
    pub p_bits: f64,
    pub subgroup_bits_proxy: f64,
    pub log2_extension_ic: f64,
    pub log2_rho: f64,
    pub ratio_log2: f64,
    pub note: &'static str,
}

/// Build a tiny nonsingular short-Weierstrass model over `F_{p^k}`.
pub fn toy_curve(p: u64, k: u32) -> Result<(ECurveFpk, u64), String> {
    let irr = find_binomial_irr(p, k)
        .ok_or_else(|| format!("no binomial irreducible θ^{k} - c over F_{p}"))?;
    // Prefer a,b in F_p for a simple model; scan a few pairs.
    for a0 in 0..p.min(16) {
        for b0 in 1..p.min(16) {
            let a = Fpk::from_base(a0, p, k);
            let b = Fpk::from_base(b0, p, k);
            let curve = ECurveFpk::new(a, b, p, k, irr);
            // Disc -16(4a³+27b²) ≠ 0 over F_p is enough for this model.
            let aa = (a0 as i128 * a0 as i128 % p as i128) as u64 * a0 % p;
            let bb = b0 * b0 % p;
            let disc = (4 * aa as u128 + 27 * bb as u128) % p as u128;
            if disc == 0 {
                continue;
            }
            let fb = build_factor_base(&curve);
            if fb.len() >= k as usize {
                return Ok((curve, irr));
            }
        }
    }
    Err(format!("no toy curve with |FB| ≥ {k} over F_{p}^{k}"))
}

/// Run one selectable-`k` decomposition cell with catalog ρ comparison.
pub fn run_cell(req: &CellRequest) -> Result<CellReport, String> {
    let schedule = k_of_p(req.p, req.k_min, req.k_max);
    let (k, from_sched) = match req.k {
        Some(k) => (k, false),
        None => (schedule, true),
    };
    if k < 2 {
        return Err("k must be ≥ 2".into());
    }
    let (curve, irr) = toy_curve(req.p, k)?;
    let fb = build_factor_base(&curve);
    if fb.len() < k as usize {
        return Err(format!("factor base too small: {} < {k}", fb.len()));
    }

    // Plant R = Σ_{j=0..k-1} FB[(seed+j) % |FB|] and recover.
    let mut recovered = 0u32;
    let mut trials_used = 0u64;
    let n = fb.len();
    for t in 0..req.planted_decompositions {
        let mut acc = Pt::identity();
        for j in 0..k as usize {
            let idx = ((req.seed as usize) + t as usize + j) % n;
            acc = curve.add(&acc, &fb[idx]);
        }
        let before = trials_used;
        match find_k_decomposition(&curve, &fb, &acc, k, req.trial_budget) {
            Some(_) => {
                recovered += 1;
                trials_used = before + 1; // success path; budget accounting coarse
            }
            None => {
                trials_used = before + req.trial_budget.min(req.trial_budget);
            }
        }
    }

    let p_bits = (req.p as f64).log2();
    // Proxy: order of E(F_{p^k}) is about p^k; use that for rho scale on the full group.
    let subgroup_bits_proxy = p_bits * f64::from(k);
    let log2_ic = log2_extension_ic_catalog(p_bits, k);
    let log2_rho = log2_rho_catalog(subgroup_bits_proxy, subgroup_bits_proxy);
    Ok(CellReport {
        schema: "diem-kgrow.cell/v1",
        standing: "[pcc:unverified]",
        p: req.p,
        k,
        k_from_schedule: from_sched,
        schedule_k_of_p: schedule,
        irr_const: irr,
        fb_size: fb.len(),
        planted: req.planted_decompositions,
        recovered,
        trials_used,
        p_bits,
        subgroup_bits_proxy,
        log2_extension_ic: log2_ic,
        log2_rho,
        ratio_log2: log2_rho - log2_ic,
        note: "Brute k-decomp lab slice; catalog cost vs rho proxy on ~p^k. Not a verified L_p[α] claim.",
    })
}

/// Scalar multiply helper exposed for smoke drivers.
pub fn scalar_mul(curve: &ECurveFpk, k: &BigUint, p: &Pt) -> Pt {
    curve.scalar_mul(k, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k_of_p_clamps_and_grows() {
        assert_eq!(k_of_p(5, 2, 6), 2);
        let k_big = k_of_p(1_000_000_007, 2, 16);
        assert!((2..=16).contains(&k_big));
    }

    #[test]
    fn selectable_k2_recovers_planted() {
        let req = CellRequest {
            p: 5,
            k: Some(2),
            planted_decompositions: 4,
            trial_budget: 100_000,
            seed: 3,
            ..CellRequest::default()
        };
        let rep = run_cell(&req).expect("k=2 cell");
        assert_eq!(rep.k, 2);
        assert!(rep.recovered >= 1, "expected ≥1 recovered decomp, got {rep:?}");
    }

    #[test]
    fn selectable_k3_recovers_planted() {
        let req = CellRequest {
            p: 7,
            k: Some(3),
            planted_decompositions: 4,
            trial_budget: 2_000_000,
            seed: 1,
            ..CellRequest::default()
        };
        let rep = run_cell(&req).expect("k=3 cell");
        assert_eq!(rep.k, 3);
        assert!(
            rep.recovered >= 1,
            "expected ≥1 recovered k=3 decomp, got {rep:?}"
        );
    }

    #[test]
    fn schedule_mode_picks_k() {
        let req = CellRequest {
            p: 7,
            k: None,
            k_min: 2,
            k_max: 3,
            planted_decompositions: 2,
            trial_budget: 2_000_000,
            seed: 2,
            ..CellRequest::default()
        };
        let rep = run_cell(&req).expect("schedule cell");
        assert!(rep.k_from_schedule);
        assert!((2..=3).contains(&rep.k));
    }
}
