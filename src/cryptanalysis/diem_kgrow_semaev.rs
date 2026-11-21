//! Semaev decomposition oracles for [`super::diem_kgrow`].
//!
//! Reuses existing lab instruments — not a new solver:
//! - `k = 2`: [`line_oracle::semaev_s3`] pairs-and-check on the Diem line
//!   \(x \in \mathbb{F}_p\), with [`LineOracle`] preferred when `p > 16`.
//! - `k = 3`: [`LineS4Oracle`] (symmetrised \(S_4\) + Macaulay) when
//!   `p ≡ 1 (mod 3)` and `p ≥ 13`; otherwise refused.
//!
//! Curves are subfield models \(a,b \in \mathbb{F}_p\). Absolute
//! \(\mathbb{F}_p\) same-target remains gated.

use crate::cryptanalysis::diem_kgrow::{
    k_of_p, log2_extension_ic_catalog, log2_rho_catalog, CellRequest, ChargedCellReport,
};
use crate::cryptanalysis::ext_curve::{ExtCurve, ExtField, ExtPoint, Fp2, Fp3};
use crate::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters,
};
use crate::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::line_oracle::{semaev_s3, LineOracle};
use crate::cryptanalysis::line_s4_oracle::LineS4Oracle;

/// Which decomposition instrument a cell uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OracleKind {
    Brute,
    Semaev,
}

impl Default for OracleKind {
    fn default() -> Self {
        Self::Semaev
    }
}

impl OracleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Brute => "brute",
            Self::Semaev => "semaev-line",
        }
    }
}

fn diem_fp_line_base<F: ExtField>(
    curve: &ExtCurve<F>,
) -> Result<FactorBase<ExtPoint<F::El>>, String>
where
    F::El: Copy,
{
    let f = &curve.f;
    let p = f.p();
    let mut points = Vec::new();
    let mut col_of = Vec::new();
    let mut coef_of = Vec::new();
    let mut columns = 0usize;
    for t in 1..p {
        let x = f.from_fp(t);
        let lifts = curve.lift_x(x);
        if lifts.is_empty() {
            continue;
        }
        let col = columns;
        columns += 1;
        for (j, pt) in lifts.into_iter().enumerate() {
            points.push(pt);
            col_of.push(col);
            coef_of.push(if j == 0 { 1 } else { p.saturating_sub(1).max(1) });
        }
    }
    if points.len() < 2 {
        return Err(format!("Diem line base too small on F_{p}^{}", f.degree()));
    }
    FactorBase::from_column_map(
        format!("Diem x∈F_p line over F_{}^{}", p, f.degree()),
        points,
        col_of,
        coef_of,
        columns,
        |pt| curve.key(pt),
        |pt| curve.key(&curve.neg(*pt)),
        |pt| f.pack(pt.x),
    )
}

fn frobenius_line_base(
    curve: &ExtCurve<Fp3>,
    line: <Fp3 as ExtField>::El,
) -> Result<FactorBase<ExtPoint<<Fp3 as ExtField>::El>>, String> {
    let f = &curve.f;
    let p = f.p();
    let mut points = Vec::new();
    let mut col_of = Vec::new();
    let mut coef_of = Vec::new();
    let mut columns = 0usize;
    for t in 1..p {
        let x = f.scale(line, t);
        let lifts = curve.lift_x(x);
        if lifts.is_empty() {
            continue;
        }
        let col = columns;
        columns += 1;
        for (j, pt) in lifts.into_iter().enumerate() {
            points.push(pt);
            col_of.push(col);
            coef_of.push(if j == 0 { 1 } else { p.saturating_sub(1).max(1) });
        }
    }
    if points.len() < 3 {
        return Err(format!("s·F_p line base too small for S₄ at p={p}"));
    }
    FactorBase::from_column_map(
        format!("Frobenius line s·F_p over F_{p}^3"),
        points,
        col_of,
        coef_of,
        columns,
        |pt| curve.key(pt),
        |pt| curve.key(&curve.neg(*pt)),
        |pt| f.pack(pt.x),
    )
}

fn pick_subfield_ab(p: u64, seed: u64) -> (u64, u64) {
    let mut a = 1 + (seed % (p.saturating_sub(1)).max(1));
    let mut b = 1 + ((seed / 3) % (p.saturating_sub(1)).max(1));
    for _ in 0..64 {
        let a3 = a.wrapping_mul(a).wrapping_mul(a) % p;
        let b2 = b.wrapping_mul(b) % p;
        let disc = (4 * a3 + 27 * b2) % p;
        if disc != 0 && a % p != 0 {
            return (a % p, b % p);
        }
        a = (a + 1) % p;
        if a == 0 {
            a = 1;
        }
        b = (b + 2) % p;
        if b == 0 {
            b = 1;
        }
    }
    (1 % p, 1 % p)
}

fn exhaustive_dlog_ext<F: ExtField>(
    curve: &ExtCurve<F>,
    g: ExtPoint<F::El>,
    target: ExtPoint<F::El>,
    order: u64,
    step_cap: u64,
) -> Result<(u64, u64), String>
where
    F::El: Copy,
{
    let mut ops = GroupOps::default();
    let mut acc = curve.identity();
    for d in 0..order {
        if acc == target {
            let mut check_ops = GroupOps::default();
            let check = curve.mul(&mut check_ops, g, d);
            if check != target {
                return Err("semaev rho: [d]G ≠ Q".into());
            }
            return Ok((d, ops.adds + ops.doubles));
        }
        acc = curve.add(&mut ops, acc, g);
        if ops.adds > step_cap {
            return Err("semaev rho exhausted".into());
        }
    }
    Err("semaev rho: target not in <G>".into())
}

/// Approximate \(F_p\) multiplications inside one `semaev_s3` evaluation
/// over an extension (matches the expansion's multiply count order).
const S3_FP_MULS_PROXY: u64 = 24;

/// Pairs-and-Semaev-\(S_3\): for each unordered FB pair, test
/// `S₃(x_i,x_j,x_R)=0` then settle signs by group arithmetic.
fn semaev_s3_pairs_decompose<F: ExtField>(
    curve: &ExtCurve<F>,
    fb: &FactorBase<ExtPoint<F::El>>,
    target: ExtPoint<F::El>,
) -> (Option<Vec<usize>>, u64, u64)
where
    F::El: Copy,
{
    if target.infinity {
        return (None, 0, 0);
    }
    let f = &curve.f;
    let mut fp_muls = 0u64;
    let mut adds = 0u64;
    let n = fb.points.len();
    for i in 0..n {
        for j in i..n {
            let pi = fb.points[i];
            let pj = fb.points[j];
            let s = semaev_s3(f, curve.a, curve.b, pi.x, pj.x, target.x);
            fp_muls += S3_FP_MULS_PROXY;
            if s != f.zero() {
                continue;
            }
            // Sign lift: ε₁Pᵢ + ε₂Pⱼ = R.
            for (si, qi) in [(1i32, pi), (-1, curve.neg(pi))] {
                for (sj, qj) in [(1i32, pj), (-1, curve.neg(pj))] {
                    let mut ops = GroupOps::default();
                    let sum = curve.add(&mut ops, qi, qj);
                    adds += ops.adds + ops.doubles;
                    let _ = (si, sj);
                    if sum == target {
                        return (Some(vec![i, j]), adds, fp_muls);
                    }
                }
            }
        }
    }
    (None, adds, fp_muls)
}

/// Charged Semaev cell for \(k = 2\).
pub fn run_semaev_cell_k2(req: &CellRequest) -> Result<ChargedCellReport, String> {
    let p = req.p;
    let f = Fp2::new(p)?;
    let (a0, b0) = pick_subfield_ab(p, req.seed);
    let curve = ExtCurve {
        f,
        a: f.from_fp(a0),
        b: f.from_fp(b0),
    };
    let fb = diem_fp_line_base(&curve)?;
    let g = fb.points[0];
    let order_proxy = p.saturating_pow(2).min(4096);

    let use_line_oracle = p > 16;
    let mut line_oracle = if use_line_oracle {
        let mut o = LineOracle::new(f, f.one());
        let mut ops = GroupOps::default();
        let ctx = InstanceCtx {
            group: &curve,
            generator: g,
            target: g,
            r: order_proxy.max(2),
            cofactor: 1,
            group_order: order_proxy.max(2),
            name: format!("diem-kgrow-semaev-k2-p{p}"),
            field_degree: Some(2),
        };
        o.prepare(&ctx, &fb, &Params::default(), &mut ops)?;
        Some(o)
    } else {
        None
    };

    let mut recovered = 0u32;
    let mut ic_adds = 0u64;
    let mut ic_fp_muls = 0u64;
    let mut ic_trials = 0u64;
    let mut last_target = curve.identity();
    let n = fb.points.len();
    let planted = req.planted_decompositions.max(1);

    for t in 0..planted {
        let i = ((req.seed as usize) + t as usize) % n;
        let j = ((req.seed as usize) + t as usize + 1) % n;
        let mut plant_ops = GroupOps::default();
        let target = curve.add(&mut plant_ops, fb.points[i], fb.points[j]);
        last_target = target;

        let mut hit = None;
        if let Some(ref mut o) = line_oracle {
            let mut cell_ops = GroupOps::default();
            let mut counters = OracleCounters::default();
            let muls_before = o.solver_totals().map(|s| s.ops).unwrap_or(0);
            let ctx_t = InstanceCtx {
                group: &curve,
                generator: g,
                target,
                r: order_proxy.max(2),
                cofactor: 1,
                group_order: order_proxy.max(2),
                name: format!("diem-kgrow-semaev-k2-p{p}"),
                field_degree: Some(2),
            };
            hit = o.decompose(&ctx_t, &fb, &mut cell_ops, &mut counters, target);
            let muls_after = o.solver_totals().map(|s| s.ops).unwrap_or(muls_before);
            ic_fp_muls += muls_after.saturating_sub(muls_before);
            ic_adds += cell_ops.adds + cell_ops.doubles;
        }
        if hit.is_none() {
            let (h, adds, muls) = semaev_s3_pairs_decompose(&curve, &fb, target);
            ic_adds += adds;
            ic_fp_muls += muls;
            hit = h;
        }
        ic_trials += 1;
        if hit.is_some() {
            recovered += 1;
        }
    }

    let step_cap = order_proxy.saturating_mul(4).max(1024);
    let (rho_d, rho_adds, rho_ok) =
        match exhaustive_dlog_ext(&curve, g, last_target, order_proxy, step_cap) {
            Ok((d, adds)) => (Some(d), adds, true),
            Err(_) => (None, order_proxy / 2, false),
        };

    let note = if use_line_oracle {
        "Semaev LineOracle S₃ (fallback pairs-and-S₃) on Diem x∈F_p / F_{p²}. Absolute F_p gate closed."
    } else {
        "Semaev pairs-and-S₃ (line_oracle::semaev_s3) on Diem x∈F_p / F_{p²}. Absolute F_p gate closed."
    };

    Ok(pack_report(
        req,
        2,
        false,
        k_of_p(p, req.k_min, req.k_max),
        0,
        fb.points.len(),
        planted,
        recovered,
        ic_adds,
        ic_fp_muls,
        ic_trials,
        rho_adds,
        rho_d,
        rho_ok,
        note,
    ))
}

/// Charged Semaev cell for \(k = 3\) via [`LineS4Oracle`].
pub fn run_semaev_cell_k3(req: &CellRequest) -> Result<ChargedCellReport, String> {
    let p = req.p;
    if p < 13 || p % 3 != 1 {
        return Err(format!(
            "LineS4Oracle needs p≥13 and p≡1 (mod 3); got p={p}"
        ));
    }
    let f = Fp3::new(p)?;
    let (a0, b0) = pick_subfield_ab(p, req.seed);
    let curve = ExtCurve {
        f,
        a: f.from_fp(a0),
        b: f.from_fp(b0),
    };
    let line = f.s();
    let mut oracle = LineS4Oracle::new(f, line, req.seed);
    let fb = frobenius_line_base(&curve, line)?;
    let g = fb.points[0];
    let order_proxy = p.saturating_pow(3).min(4096);
    let mut ops = GroupOps::default();
    let ctx_prep = InstanceCtx {
        group: &curve,
        generator: g,
        target: g,
        r: order_proxy.max(2),
        cofactor: 1,
        group_order: order_proxy.max(2),
        name: format!("diem-kgrow-semaev-k3-p{p}"),
        field_degree: Some(3),
    };
    oracle.prepare(&ctx_prep, &fb, &Params::default(), &mut ops)?;

    let mut recovered = 0u32;
    let mut ic_adds = 0u64;
    let mut ic_fp_muls = 0u64;
    let mut ic_trials = 0u64;
    let mut last_target = curve.identity();
    let n = fb.points.len();
    let planted = req.planted_decompositions.max(1);

    for t in 0..planted {
        let i0 = ((req.seed as usize) + t as usize) % n;
        let i1 = ((req.seed as usize) + t as usize + 1) % n;
        let i2 = ((req.seed as usize) + t as usize + 2) % n;
        let mut plant_ops = GroupOps::default();
        let s01 = curve.add(&mut plant_ops, fb.points[i0], fb.points[i1]);
        let target = curve.add(&mut plant_ops, s01, fb.points[i2]);
        last_target = target;
        let mut cell_ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let ctx_t = InstanceCtx {
            group: &curve,
            generator: g,
            target,
            r: order_proxy.max(2),
            cofactor: 1,
            group_order: order_proxy.max(2),
            name: format!("diem-kgrow-semaev-k3-p{p}"),
            field_degree: Some(3),
        };
        let muls_before = oracle.stats.fp_muls;
        let hit = oracle.decompose(&ctx_t, &fb, &mut cell_ops, &mut counters, target);
        ic_fp_muls += oracle.stats.fp_muls.saturating_sub(muls_before);
        ic_adds += cell_ops.adds + cell_ops.doubles;
        ic_trials += 1;
        if hit.is_some() {
            recovered += 1;
        }
    }

    let step_cap = order_proxy.saturating_mul(4).max(1024);
    let (rho_d, rho_adds, rho_ok) =
        match exhaustive_dlog_ext(&curve, g, last_target, order_proxy, step_cap) {
            Ok((d, adds)) => (Some(d), adds, true),
            Err(_) => (None, order_proxy / 2, false),
        };

    Ok(pack_report(
        req,
        3,
        false,
        k_of_p(p, req.k_min, req.k_max),
        0,
        fb.points.len(),
        planted,
        recovered,
        ic_adds,
        ic_fp_muls,
        ic_trials,
        rho_adds,
        rho_d,
        rho_ok,
        "Semaev LineS4Oracle (S₄+Macaulay) on s·F_p over F_{p³}. Absolute F_p gate closed.",
    ))
}

fn pack_report(
    req: &CellRequest,
    k: u32,
    from_sched: bool,
    schedule: u32,
    irr: u64,
    fb_size: usize,
    planted: u32,
    recovered: u32,
    ic_adds: u64,
    ic_fp_muls: u64,
    ic_trials: u64,
    rho_adds: u64,
    rho_d: Option<u64>,
    rho_ok: bool,
    note: &'static str,
) -> ChargedCellReport {
    let p_bits = (req.p as f64).log2();
    let subgroup_bits_proxy = p_bits * f64::from(k);
    let log2_ic_pred = log2_extension_ic_catalog(p_bits, k);
    let log2_rho_pred = log2_rho_catalog(subgroup_bits_proxy, subgroup_bits_proxy);
    let ic_total = ic_adds.saturating_add(ic_fp_muls).max(1);
    let log2_ic_obs = (ic_total as f64).log2();
    let log2_rho_obs = if rho_adds > 0 {
        (rho_adds as f64).log2()
    } else {
        0.0
    };
    ChargedCellReport {
        schema: "diem-kgrow.charged-cell/v1",
        standing: "[pcc:unverified]",
        p: req.p,
        k,
        k_from_schedule: from_sched,
        schedule_k_of_p: schedule,
        irr_const: irr,
        fb_size,
        planted,
        recovered,
        ic_group_adds: ic_total,
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
        note,
    }
}

pub fn run_semaev_cell(req: &CellRequest) -> Result<ChargedCellReport, String> {
    let schedule = k_of_p(req.p, req.k_min, req.k_max);
    let (k, from_sched) = match req.k {
        Some(k) => (k, false),
        None => (schedule, true),
    };
    let mut rep = match k {
        2 => run_semaev_cell_k2(req)?,
        3 => run_semaev_cell_k3(req)?,
        _ => {
            return Err(format!(
                "semaev-line oracle supports k=2 (S₃) and k=3 (LineS4Oracle) only; got k={k}"
            ))
        }
    };
    rep.k_from_schedule = from_sched;
    rep.schedule_k_of_p = schedule;
    Ok(rep)
}
