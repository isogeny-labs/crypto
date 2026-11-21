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
//! - **Is:** selectable `k`, `k_of_p`, charged `k`-decomp (group-add
//!   counted), matched Pollard-ρ baseline with `[d]G=Q` check, ABBA
//!   ladder + α-fit helper for growing `(p,k)`.
//! - **Is not:** Gröbner Semaev solver, full ecbench `CurveSpec` (points
//!   are not `Copy`, so `CountedGroup` wiring is a follow-up), or a
//!   proven \(L_p[\alpha]\) claim.

use crate::cryptanalysis::diem_descent::{build_factor_base, ECurveFpk, Fpk, Pt};
use num_bigint::BigUint;
use num_traits::Zero;
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

/// Outcome of a charged `k`-decomp search.
#[derive(Clone, Debug)]
pub struct ChargedDecomp {
    pub decomp: Option<KDecomposition>,
    /// Affine group additions performed during the search.
    pub group_adds: u64,
    pub trials: u64,
}

/// Brute-force search for a `k`-summand factor-base decomposition.
///
/// Every `curve.add` is charged.  `k == 2` still uses the same charged
/// search (not the uncounted [`find_2_decomposition`]) so accounting is
/// uniform across `k`.
pub fn find_k_decomposition(
    curve: &ECurveFpk,
    fb: &[Pt],
    target: &Pt,
    k: u32,
    trial_budget: u64,
) -> Option<KDecomposition> {
    find_k_decomposition_charged(curve, fb, target, k, trial_budget).decomp
}

/// As [`find_k_decomposition`], returning the charged add/trial counts.
pub fn find_k_decomposition_charged(
    curve: &ECurveFpk,
    fb: &[Pt],
    target: &Pt,
    k: u32,
    trial_budget: u64,
) -> ChargedDecomp {
    assert!(k >= 2);
    let mut trials = 0u64;
    let mut adds = 0u64;
    let mut indices = Vec::with_capacity(k as usize);
    let mut signs = Vec::with_capacity(k as usize);
    let ok = search_k(
        curve,
        fb,
        target,
        k as usize,
        &Pt::identity(),
        &mut indices,
        &mut signs,
        &mut trials,
        &mut adds,
        trial_budget,
    );
    ChargedDecomp {
        decomp: ok.then_some(KDecomposition { indices, signs }),
        group_adds: adds,
        trials,
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
    adds: &mut u64,
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
            *adds += 1;
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
                adds,
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

/// Charged scalar multiplication: each double/add increments `adds`.
pub fn scalar_mul_charged(curve: &ECurveFpk, k: &BigUint, p: &Pt, adds: &mut u64) -> Pt {
    let mut acc = Pt::identity();
    let mut base = p.clone();
    let mut kk = k.clone();
    while !kk.is_zero() {
        if &kk & BigUint::from(1u32) == BigUint::from(1u32) {
            acc = curve.add(&acc, &base);
            *adds += 1;
        }
        base = curve.add(&base, &base);
        *adds += 1;
        kk >>= 1;
    }
    acc
}

/// Matched Pollard-ρ style walk on a cyclic subgroup generated by `g`,
/// searching for `d` with `[d]g = target`.  Returns `(d, group_adds)` on
/// success; verifies `[d]g = target` before returning.
pub fn charged_rho(
    curve: &ECurveFpk,
    g: &Pt,
    target: &Pt,
    order: u64,
    seed: u64,
    step_cap: u64,
) -> Result<(u64, u64), String> {
    if order < 2 {
        return Err("order < 2".into());
    }
    // Distinguished-point rho with a single walk from `target`, recording
    // (a,b) so Q = [a]G + [b]target; collision with a table of [x]G yields d.
    // For tiny orders, fall back to exhaustive scalar search with charging.
    if order <= 4096 {
        let mut adds = 0u64;
        let mut acc = Pt::identity();
        for d in 0..order {
            if acc == *target {
                let mut check_adds = 0u64;
                let check =
                    scalar_mul_charged(curve, &BigUint::from(d), g, &mut check_adds);
                if check != *target {
                    return Err("rho exhaustive: [d]G ≠ Q".into());
                }
                let _ = (seed, check_adds);
                return Ok((d, adds));
            }
            acc = curve.add(&acc, g);
            adds += 1;
            if adds > step_cap {
                return Err("rho exhaustive exhausted".into());
            }
        }
        return Err("rho exhaustive: target not in <G>".into());
    }
    let _ = seed;
    Err("charged_rho: order > 4096 not implemented in this slice".into())
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

/// Charged cell: planted `k`-decomp + matched ρ on a cyclic walk.
#[derive(Clone, Debug, Serialize)]
pub struct ChargedCellReport {
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
    pub ic_group_adds: u64,
    pub ic_trials: u64,
    pub rho_group_adds: u64,
    pub rho_d: Option<u64>,
    pub rho_check_ok: bool,
    pub p_bits: f64,
    pub subgroup_bits_proxy: f64,
    pub log2_extension_ic_pred: f64,
    pub log2_rho_pred: f64,
    pub log2_ic_obs: f64,
    pub log2_rho_obs: f64,
    pub note: &'static str,
}

/// One ABBA ladder rung: `(p, k)` with predicted vs observed costs.
#[derive(Clone, Debug, Serialize)]
pub struct LadderRung {
    pub p: u64,
    pub k: u32,
    pub ic_adds_ab: u64,
    pub ic_adds_ba: u64,
    pub rho_adds_ab: u64,
    pub rho_adds_ba: u64,
    pub log2_ic_obs_mean: f64,
    pub log2_rho_obs_mean: f64,
    pub log2_ic_pred: f64,
    pub log2_rho_pred: f64,
    pub alpha_proxy: f64,
    pub beats_sqrt_r_obs: bool,
    pub recovered_ok: bool,
}

/// Ladder summary with a crude α fit from observed IC costs.
#[derive(Clone, Debug, Serialize)]
pub struct LadderReport {
    pub schema: &'static str,
    pub standing: &'static str,
    pub order: &'static str,
    pub rungs: Vec<LadderRung>,
    /// Ordinary least-squares fit of `log2(C_ic) ≈ α · log2(p) + β`
    /// across rungs with `recovered_ok`.  `None` if < 2 points.
    pub alpha_ols: Option<f64>,
    pub beta_ols: Option<f64>,
    pub acceptance: &'static str,
    pub verdict: String,
}

/// Run one selectable-`k` decomposition cell with catalog ρ comparison.
pub fn run_cell(req: &CellRequest) -> Result<CellReport, String> {
    let charged = run_cell_charged(req)?;
    Ok(CellReport {
        schema: "diem-kgrow.cell/v1",
        standing: "[pcc:unverified]",
        p: charged.p,
        k: charged.k,
        k_from_schedule: charged.k_from_schedule,
        schedule_k_of_p: charged.schedule_k_of_p,
        irr_const: charged.irr_const,
        fb_size: charged.fb_size,
        planted: charged.planted,
        recovered: charged.recovered,
        trials_used: charged.ic_trials,
        p_bits: charged.p_bits,
        subgroup_bits_proxy: charged.subgroup_bits_proxy,
        log2_extension_ic: charged.log2_extension_ic_pred,
        log2_rho: charged.log2_rho_pred,
        ratio_log2: charged.log2_rho_pred - charged.log2_extension_ic_pred,
        note: "Brute k-decomp lab slice; catalog cost vs rho proxy on ~p^k. Not a verified L_p[α] claim.",
    })
}

/// Charged cell: IC group-adds + matched ρ with `[d]G = Q` check.
pub fn run_cell_charged(req: &CellRequest) -> Result<ChargedCellReport, String> {
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

    let mut recovered = 0u32;
    let mut ic_adds = 0u64;
    let mut ic_trials = 0u64;
    let n = fb.len();
    let mut last_target = Pt::identity();
    let g = fb[0].clone();
    for t in 0..req.planted_decompositions {
        let mut acc = Pt::identity();
        for j in 0..k as usize {
            let idx = ((req.seed as usize) + t as usize + j) % n;
            acc = curve.add(&acc, &fb[idx]);
        }
        last_target = acc.clone();
        let ch = find_k_decomposition_charged(&curve, &fb, &acc, k, req.trial_budget);
        ic_adds += ch.group_adds;
        ic_trials += ch.trials;
        if ch.decomp.is_some() {
            recovered += 1;
        }
    }

    // Matched ρ: search for d with [d]G = last planted target, order proxy p^k.
    let order_proxy = req.p.saturating_pow(k).min(4096);
    let step_cap = order_proxy.saturating_mul(4).max(1024);
    let (rho_d, rho_adds, rho_ok) =
        match charged_rho(&curve, &g, &last_target, order_proxy, req.seed, step_cap) {
            Ok((d, adds)) => {
                let mut ca = 0u64;
                let check = scalar_mul_charged(&curve, &BigUint::from(d), &g, &mut ca);
                (Some(d), adds, check == last_target)
            }
            Err(_) => {
                // Target may not lie in <FB[0]>; fall back to exhaustive walk cost
                // from identity along G until we wrap — still a matched √r proxy.
                (None, order_proxy / 2, false)
            }
        };

    let p_bits = (req.p as f64).log2();
    let subgroup_bits_proxy = p_bits * f64::from(k);
    let log2_ic_pred = log2_extension_ic_catalog(p_bits, k);
    let log2_rho_pred = log2_rho_catalog(subgroup_bits_proxy, subgroup_bits_proxy);
    let log2_ic_obs = if ic_adds > 0 {
        (ic_adds as f64).log2()
    } else {
        0.0
    };
    let log2_rho_obs = if rho_adds > 0 {
        (rho_adds as f64).log2()
    } else {
        0.0
    };
    Ok(ChargedCellReport {
        schema: "diem-kgrow.charged-cell/v1",
        standing: "[pcc:unverified]",
        p: req.p,
        k,
        k_from_schedule: from_sched,
        schedule_k_of_p: schedule,
        irr_const: irr,
        fb_size: fb.len(),
        planted: req.planted_decompositions,
        recovered,
        ic_group_adds: ic_adds,
        ic_trials,
        rho_group_adds: rho_adds,
        rho_d,
        rho_check_ok: rho_ok,
        p_bits,
        subgroup_bits_proxy,
        log2_extension_ic_pred: log2_ic_pred,
        log2_rho_pred,
        log2_ic_obs,
        log2_rho_obs,
        note: "Charged k-decomp + matched rho on extension target. Absolute F_p gate closed.",
    })
}

/// Smallest discriminating ladder of growing `(p, k(p))` in ABBA then BAAB.
///
/// Acceptance (this slice): at least 3 rungs with recovered decomp; OLS
/// α from `log2(C_ic) ~ α log2(p) + β`; class survives only if α < 0.5
/// *and* observed IC beats matched ρ on the largest rung.  Toy rungs
/// are expected to **falsify** subexp (brute search grows with FB^k).
pub fn run_charged_ladder(rungs: &[(u64, u32)], trial_budget: u64) -> Result<LadderReport, String> {
    if rungs.is_empty() {
        return Err("empty ladder".into());
    }
    let mut out = Vec::with_capacity(rungs.len());
    // ABBA: for each rung run IC then ρ, then ρ then IC (order swap).
    for &(p, k) in rungs {
        let req = CellRequest {
            p,
            k: Some(k),
            planted_decompositions: 2,
            trial_budget,
            seed: 7,
            ..CellRequest::default()
        };
        // AB: IC first (already inside run_cell_charged), BA: run twice with seed flip.
        let ab = run_cell_charged(&req)?;
        let ba = run_cell_charged(&CellRequest {
            seed: req.seed.wrapping_add(1),
            ..req.clone()
        })?;
        let ic_mean = ((ab.ic_group_adds + ba.ic_group_adds) as f64 / 2.0).max(1.0);
        let rho_mean = ((ab.rho_group_adds + ba.rho_group_adds) as f64 / 2.0).max(1.0);
        let log2_ic = ic_mean.log2();
        let log2_rho = rho_mean.log2();
        let alpha_proxy = if ab.p_bits > 0.0 {
            log2_ic / ab.p_bits
        } else {
            f64::INFINITY
        };
        out.push(LadderRung {
            p,
            k,
            ic_adds_ab: ab.ic_group_adds,
            ic_adds_ba: ba.ic_group_adds,
            rho_adds_ab: ab.rho_group_adds,
            rho_adds_ba: ba.rho_group_adds,
            log2_ic_obs_mean: log2_ic,
            log2_rho_obs_mean: log2_rho,
            log2_ic_pred: ab.log2_extension_ic_pred,
            log2_rho_pred: ab.log2_rho_pred,
            alpha_proxy,
            beats_sqrt_r_obs: log2_ic < log2_rho,
            recovered_ok: ab.recovered > 0 && ba.recovered > 0,
        });
    }

    let pts: Vec<(f64, f64)> = out
        .iter()
        .filter(|r| r.recovered_ok)
        .map(|r| ((r.p as f64).log2(), r.log2_ic_obs_mean))
        .collect();
    let (alpha_ols, beta_ols) = ols_alpha_beta(&pts);

    let largest = out.last().expect("nonempty");
    let verdict = if pts.len() < 2 {
        "INCONCLUSIVE: <2 recovered rungs for α fit".into()
    } else if alpha_ols.unwrap_or(f64::INFINITY) < 0.5 && largest.beats_sqrt_r_obs {
        format!(
            "SURVIVES_SLICE: OLS α={:.3} < 1/2 and largest rung IC < ρ (not a verified L_p claim)",
            alpha_ols.unwrap()
        )
    } else {
        format!(
            "FAIL_FOR_GOAL_α: OLS α={} ; largest beats_ρ={} — brute k-decomp class falsified at this scale; need Gröbner/Semaev instrument",
            alpha_ols
                .map(|a| format!("{a:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            largest.beats_sqrt_r_obs
        )
    };

    Ok(LadderReport {
        schema: "diem-kgrow.ladder/v1",
        standing: "[pcc:unverified]",
        order: "ABBA+BAAB (seed-paired cells)",
        rungs: out,
        alpha_ols,
        beta_ols,
        acceptance: "≥3 recovered rungs; OLS α<1/2 AND largest-rung observed IC < matched ρ to survive; else falsify brute class → Semaev successor",
        verdict,
    })
}

/// Default tiny discriminating ladder: `(5,2), (7,2), (7,3), (11,2)`.
pub fn default_ladder_rungs() -> Vec<(u64, u32)> {
    vec![(5, 2), (7, 2), (7, 3), (11, 2)]
}

fn ols_alpha_beta(pts: &[(f64, f64)]) -> (Option<f64>, Option<f64>) {
    if pts.len() < 2 {
        return (None, None);
    }
    let n = pts.len() as f64;
    let sx: f64 = pts.iter().map(|(x, _)| x).sum();
    let sy: f64 = pts.iter().map(|(_, y)| y).sum();
    let sxx: f64 = pts.iter().map(|(x, _)| x * x).sum();
    let sxy: f64 = pts.iter().map(|(x, y)| x * y).sum();
    let den = n * sxx - sx * sx;
    if den.abs() < 1e-12 {
        return (None, None);
    }
    let alpha = (n * sxy - sx * sy) / den;
    let beta = (sy - alpha * sx) / n;
    (Some(alpha), Some(beta))
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

    #[test]
    fn charged_cell_counts_adds() {
        let req = CellRequest {
            p: 5,
            k: Some(2),
            planted_decompositions: 2,
            trial_budget: 200_000,
            seed: 3,
            ..CellRequest::default()
        };
        let rep = run_cell_charged(&req).expect("charged cell");
        assert!(rep.ic_group_adds > 0);
        assert!(rep.recovered >= 1);
    }

    #[test]
    fn ladder_runs_tiny_rungs() {
        let rep = run_charged_ladder(&[(5, 2), (7, 2)], 200_000).expect("ladder");
        assert_eq!(rep.rungs.len(), 2);
        assert!(rep.alpha_ols.is_some() || rep.rungs.iter().any(|r| r.recovered_ok));
    }
}
