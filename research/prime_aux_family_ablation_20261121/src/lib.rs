//! Matched-|V| auxiliary-curve family ablation for prime-field PKM IC.
//!
//! Families (report shortlist #1/#5/#6/#7):
//! - `j1728` baseline: `w^2 = x^3 - 3x` on `p ≡ 3 (mod 4)`
//! - `j0`: `y^2 = x^3 + b` with smooth `#E` searched on M19
//! - `montgomery`: birational short-Weierstrass of `By^2 = x^3 + A x^2 + x`
//! - `twist`: quadratic twist of the target as aux abscissa source
//!
//! Axes: (A) lift / FB hit rate, (B) solving degree + cols vs j1728, (C) trace-replay prep.

use std::collections::HashMap;
use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use crypto_lib::cryptanalysis::symmetrized_semaev::ParamS5;

#[derive(Clone, Debug, Serialize)]
pub struct AblationRow {
    pub family: String,
    pub p: u64,
    pub aux_a: u64,
    pub aux_b: u64,
    pub aux_order: u64,
    pub aux_smooth_note: String,
    pub depth: u32,
    pub m: u32,
    pub matched_v: usize,
    pub lift_rate: f64,
    pub fb_columns: usize,
    pub solving_degree_max: u32,
    pub max_cols: u64,
    pub scratch_ops_per_call: f64,
    pub trace_ops_per_call: f64,
    pub ops_speedup_x: f64,
    pub wall_speedup_x: f64,
    pub planted_hit_verified: bool,
    pub param_s5_spec_mults: u64,
    pub score_a: f64,
    pub score_b: f64,
    pub score_c: f64,
    pub composite_score: f64,
    pub verdict: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct AblationSummary {
    pub schema: String,
    pub date: String,
    pub standing: String,
    pub protocol: String,
    pub rows: Vec<AblationRow>,
    pub ranking: Vec<String>,
}

fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a as u128 + b as u128;
    (s % p as u128) as u64
}
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

fn is_prime_u64_det(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

/// Find a random prime-order curve over fixed `p`, optionally fixing `a`.
pub fn find_prime_order_curve_over_fixed_p(
    p: u64,
    a_fixed: Option<u64>,
    seed: u64,
) -> (PrimeCurve, PrimePoint, u64) {
    let mut rng = StdRng::seed_from_u64(seed ^ p);
    let two_sqrt = ((4.0 * (p as f64).sqrt()).ceil() as u64).max(16);
    let m_step = ((two_sqrt as f64).sqrt().ceil() as u64).max(4);
    let lo = (p + 1).saturating_sub(two_sqrt / 2 + 2);
    let hi = p + 1 + two_sqrt / 2 + 2;

    for trial in 0..4096u64 {
        let a = a_fixed.unwrap_or_else(|| rng.gen_range(0..p));
        let b = if trial == 0 {
            1 + (seed % (p - 1))
        } else {
            rng.gen_range(1..p)
        };
        let disc = addm(
            mulm(4, mulm(mulm(a, a, p), a, p), p),
            mulm(27, mulm(b, b, p), p),
            p,
        );
        if disc == 0 {
            continue;
        }
        let curve = PrimeCurve { p, a, b };
        let mut pts = Vec::with_capacity(2);
        let mut x_probe = addm(17, mulm(trial + 1, 131, p), p);
        for _ in 0..128 {
            if let Some(y) = curve.sqrt(curve.rhs(x_probe)) {
                if y != 0 {
                    pts.push(PrimePoint::affine(x_probe, y));
                    if pts.len() == 2 {
                        break;
                    }
                }
            }
            x_probe = addm(x_probe, 1, p);
        }
        if pts.len() < 2 {
            continue;
        }
        let mut ops = GroupOps::default();
        let p0 = pts[0];
        let mut baby: HashMap<(u64, u64), u64> = HashMap::with_capacity(m_step as usize + 2);
        let mut cur = PrimePoint::INFINITY;
        for j in 0..m_step {
            let key = if cur.infinity {
                (u64::MAX, u64::MAX)
            } else {
                (cur.x, cur.y)
            };
            baby.entry(key).or_insert(j);
            cur = curve.add(&mut ops, cur, p0);
        }
        let mut gamma = curve.mul(&mut ops, p0, lo);
        let max_k = (hi - lo) / m_step + 2;
        let mut cand_order: Option<u64> = None;
        for k in 0..=max_k {
            let key = if gamma.infinity {
                (u64::MAX, u64::MAX)
            } else {
                (gamma.x, gamma.y)
            };
            if let Some(&j) = baby.get(&key) {
                if lo + k * m_step >= j {
                    let n_cand = lo + k * m_step - j;
                    if (lo..=hi).contains(&n_cand)
                        && is_prime_u64_det(n_cand)
                        && curve.mul(&mut ops, p0, n_cand).infinity
                        && curve.mul(&mut ops, pts[1], n_cand).infinity
                    {
                        cand_order = Some(n_cand);
                        break;
                    }
                }
            }
            let neg_g = curve.neg(gamma);
            let nkey = if neg_g.infinity {
                (u64::MAX, u64::MAX)
            } else {
                (neg_g.x, neg_g.y)
            };
            if let Some(&j) = baby.get(&nkey) {
                let n_cand = lo + k * m_step + j;
                if (lo..=hi).contains(&n_cand)
                    && is_prime_u64_det(n_cand)
                    && curve.mul(&mut ops, p0, n_cand).infinity
                    && curve.mul(&mut ops, pts[1], n_cand).infinity
                {
                    cand_order = Some(n_cand);
                    break;
                }
            }
            let stride = curve.mul(&mut ops, p0, m_step);
            gamma = curve.add(&mut ops, gamma, stride);
        }
        let Some(r) = cand_order else {
            continue;
        };
        // Generator of prime order.
        for &pt in &pts {
            if !curve.mul(&mut ops, pt, r).infinity {
                continue;
            }
            // Ensure not identity and order exactly r.
            if pt.infinity {
                continue;
            }
            let half = r / 2;
            if half > 1 && curve.mul(&mut ops, pt, half).infinity {
                continue;
            }
            return (curve, pt, r);
        }
    }
    panic!("no prime-order curve found for p={p}");
}

fn velu2_step(pt: PrimePoint, r_k: u64, d_k: u64, cod: &PrimeCurve) -> PrimePoint {
    let p = cod.p;
    if pt.infinity || pt.x == r_k {
        return PrimePoint::INFINITY;
    }
    let diff = subm(pt.x, r_k, p);
    let inv = {
        let mut b = diff;
        let mut e = p - 2;
        let mut acc = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                acc = mulm(acc, b, p);
            }
            b = mulm(b, b, p);
            e >>= 1;
        }
        acc
    };
    let nx = addm(pt.x, mulm(d_k, inv, p), p);
    let inv2 = mulm(inv, inv, p);
    let dinv2 = mulm(d_k, inv2, p);
    let one_minus = subm(1, dinv2, p);
    let ny = mulm(pt.y, one_minus, p);
    PrimePoint::affine(nx, ny)
}

fn build_velu2_tower(
    aux: &PrimeCurve,
    ker_top: PrimePoint,
    depth: u32,
    odd_shift: u32,
) -> (Vec<(u64, u64)>, Vec<PrimeCurve>, PrimePoint) {
    let p = aux.p;
    let mut ops = GroupOps::default();
    let mut k_pt = aux.mul(&mut ops, ker_top, 1u64 << (odd_shift - depth));
    let mut cur_c = aux.clone();
    let mut steps = Vec::with_capacity(depth as usize);
    let mut chain_cods = Vec::with_capacity(depth as usize);
    let ker_base = k_pt;
    for s_idx in 0..depth {
        let t_ker = cur_c.mul(&mut ops, k_pt, 1u64 << (depth - s_idx - 1));
        assert!(
            !t_ker.infinity && t_ker.y == 0,
            "missing 2-torsion at step {s_idx}"
        );
        let tx2 = mulm(t_ker.x, t_ker.x, p);
        let d_k = addm(mulm(3, tx2, p), cur_c.a, p);
        let next_a = subm(cur_c.a, mulm(5, d_k, p), p);
        let next_b = subm(cur_c.b, mulm(7, mulm(t_ker.x, d_k, p), p), p);
        let next_c = PrimeCurve {
            p,
            a: next_a,
            b: next_b,
        };
        steps.push((t_ker.x, d_k));
        k_pt = velu2_step(k_pt, t_ker.x, d_k, &next_c);
        chain_cods.push(next_c.clone());
        cur_c = next_c;
    }
    assert!(k_pt.infinity);
    (steps, chain_cods, ker_base)
}

fn try_full_2power_point(aux: &PrimeCurve, odd_shift: u32) -> Option<PrimePoint> {
    let mut ops = GroupOps::default();
    // Scan a wide abscissa window; for non-cyclic 2-primary parts this may miss.
    for x in 2..(aux.p.min(50_000)) {
        if let Some(y) = aux.sqrt(aux.rhs(x)) {
            if y != 0 {
                let q = PrimePoint::affine(x, y);
                if aux.mul(&mut ops, q, 1u64 << odd_shift).infinity
                    && !aux.mul(&mut ops, q, 1u64 << (odd_shift - 1)).infinity
                {
                    return Some(q);
                }
            }
        }
    }
    None
}

fn find_full_2power_point(aux: &PrimeCurve, odd_shift: u32) -> PrimePoint {
    try_full_2power_point(aux, odd_shift)
        .unwrap_or_else(|| panic!("no point of order 2^{odd_shift} on aux"))
}

fn lift_fb_from_coset(
    target: &PrimeCurve,
    r: u64,
    coset_xs: &[u64],
    label: &str,
) -> (FactorBase<PrimePoint>, f64) {
    let p = target.p;
    let mut fb = FactorBase::empty(label.into());
    let mut lifts = 0usize;
    for &cx in coset_xs {
        if let Some(ty) = target.sqrt(target.rhs(cx)) {
            if ty != 0 {
                lifts += 1;
                let ty = ty.min(p - ty);
                let pt = PrimePoint::affine(cx, ty);
                let qn = target.neg(pt);
                let col = fb.abscissae;
                let idx = fb.points.len();
                fb.points.push(pt);
                fb.col_of.push(col);
                fb.coef_of.push(1);
                fb.points.push(qn);
                fb.col_of.push(col);
                fb.coef_of.push(r - 1);
                fb.neg_index.push(idx + 1);
                fb.neg_index.push(idx);
                fb.x_index.insert(cx, vec![idx, idx + 1]);
                fb.abscissae += 1;
            }
        }
    }
    fb.columns = fb.abscissae;
    for (idx, pt) in fb.points.iter().enumerate() {
        fb.point_index.insert(target.key(pt), idx);
    }
    fb.abscissa_list = fb.x_index.keys().copied().collect();
    fb.abscissa_list.sort_unstable();
    let lift_rate = lifts as f64 / coset_xs.len().max(1) as f64;
    (fb, lift_rate)
}

fn pick_coset_with_lifts(
    aux: &PrimeCurve,
    target: &PrimeCurve,
    r: u64,
    steps: &[(u64, u64)],
    chain_cods: &[PrimeCurve],
    ker_base: PrimePoint,
    depth: u32,
    m: u32,
    label: &str,
) -> (Vec<u64>, u64, FactorBase<PrimePoint>, f64) {
    let mut ops = GroupOps::default();
    for x_try in 5..4000u64 {
        let Some(y_try) = aux.sqrt(aux.rhs(x_try)) else {
            continue;
        };
        if y_try == 0 {
            continue;
        }
        let r0 = PrimePoint::affine(x_try, y_try);
        let mut t_end = r0;
        for (si, &(rk, dk)) in steps.iter().enumerate() {
            t_end = velu2_step(t_end, rk, dk, &chain_cods[si]);
        }
        if t_end.infinity || t_end.y == 0 {
            continue;
        }
        let mut coset_xs = Vec::with_capacity(1usize << depth);
        let mut q = r0;
        for _ in 0..(1usize << depth) {
            coset_xs.push(q.x);
            q = aux.add(&mut ops, q, ker_base);
        }
        let (fb, lift) = lift_fb_from_coset(target, r, &coset_xs, label);
        if fb.columns >= m as usize + 1 {
            return (coset_xs, t_end.x, fb, lift);
        }
    }
    panic!("no coset with enough lifts for {label}");
}

fn run_iso_oracle(
    target: &PrimeCurve,
    g: PrimePoint,
    r: u64,
    fb: &FactorBase<PrimePoint>,
    m: u32,
    depth: u32,
    v_xs: &[u64],
    steps: &[(u64, u64)],
    c_term: u64,
    fast: bool,
) -> (f64, u32, u64, f64, bool) {
    let ctx = InstanceCtx {
        group: target,
        generator: g,
        target: g,
        r,
        cofactor: 1,
        group_order: r,
        name: "ablation-target".into(),
        field_degree: None,
    };
    let mut oracle = PkmSemaevOracle::new_isogeny(m, depth, v_xs.to_vec(), steps.to_vec(), c_term);
    let spec_s = if fast {
        format!("x:m={m},degree=16,trace=1,inherit=1,descent=1")
    } else {
        format!("x:m={m},degree=16,trace=0,inherit=0,descent=0")
    };
    let (_, params) = Params::parse_spec(&spec_s).unwrap();
    let mut ops = GroupOps::default();
    let t0 = Instant::now();
    oracle.prepare(&ctx, fb, &params, &mut ops).unwrap();
    let mut counters = OracleCounters::default();
    let targets: Vec<PrimePoint> = (1..=16u64)
        .map(|k| target.mul(&mut ops, g, 10_000 + k * 7919))
        .collect();
    for &t_pt in &targets {
        let _ = oracle.decompose(&ctx, fb, &mut ops, &mut counters, t_pt);
    }
    let mut planted_sum = PrimePoint::INFINITY;
    for j in 0..(m as usize) {
        planted_sum = target.add(&mut ops, planted_sum, fb.points[2 * j]);
    }
    let hit = oracle.decompose(&ctx, fb, &mut ops, &mut counters, planted_sum);
    let hit_ok = if let Some(idxs) = hit {
        let mut s = PrimePoint::INFINITY;
        for i in idxs {
            s = target.add(&mut ops, s, fb.points[i]);
        }
        s == planted_sum
    } else {
        false
    };
    let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    let tot = oracle.solver_totals().unwrap();
    let max_c = tot
        .extra
        .get("max_cols_max")
        .copied()
        .or_else(|| tot.degree_trace.iter().map(|s| s.cols).max())
        .unwrap_or(0);
    (
        tot.ops as f64 / (targets.len() + 1) as f64,
        tot.solving_degree_max,
        max_c,
        wall_ms,
        hit_ok,
    )
}

fn score_row(
    lift_rate: f64,
    deg: u32,
    max_cols: u64,
    ops_x: f64,
    baseline_deg: u32,
    baseline_cols: u64,
) -> (f64, f64, f64, f64, String) {
    // A: lift rate in [0,1] → 0..3
    let a = (lift_rate * 3.0).clamp(0.0, 3.0);
    // B: lower degree/cols than baseline → higher score
    let deg_ratio = if baseline_deg == 0 {
        1.0
    } else {
        deg as f64 / baseline_deg as f64
    };
    let col_ratio = if baseline_cols == 0 {
        1.0
    } else {
        max_cols as f64 / baseline_cols as f64
    };
    let b =
        (3.0 - 1.5 * (deg_ratio - 1.0).max(0.0) - 1.5 * (col_ratio - 1.0).max(0.0)).clamp(0.0, 3.0);
    // C: trace speedup
    let c = ((ops_x - 1.0) / 2.0).clamp(0.0, 3.0);
    let composite = 2.0 * a + 3.0 * b + c;
    let verdict = if deg_ratio < 0.95 && lift_rate >= 0.05 {
        "CONTINUE_DEGREE".into()
    } else if lift_rate > 0.15 && ops_x >= 1.5 {
        "CONTINUE_PREP_OR_LIFT".into()
    } else if lift_rate < 0.02 {
        "KILL_LIFT".into()
    } else {
        "HOLD_PREP_ONLY".into()
    };
    (a, b, c, composite, verdict)
}

fn run_j1728_family(p: u64, odd_shift: u32, depth: u32, m: u32) -> AblationRow {
    let (target, g, r) = find_prime_order_curve_over_fixed_p(p, Some(p - 3), 20261121);
    let aux = PrimeCurve { p, a: p - 3, b: 0 };
    assert_eq!(aux.point_count(), p + 1, "j1728 #E=p+1");
    let ker_top = find_full_2power_point(&aux, odd_shift);
    let (steps, chain_cods, ker_base) = build_velu2_tower(&aux, ker_top, depth, odd_shift);
    let (v_xs, c_term, fb, lift) = pick_coset_with_lifts(
        &aux,
        &target,
        r,
        &steps,
        &chain_cods,
        ker_base,
        depth,
        m,
        "j1728-aux",
    );
    let (scr_ops, deg, max_c, scr_ms, scr_hit) =
        run_iso_oracle(&target, g, r, &fb, m, depth, &v_xs, &steps, c_term, false);
    let (tr_ops, _, _, tr_ms, tr_hit) =
        run_iso_oracle(&target, g, r, &fb, m, depth, &v_xs, &steps, c_term, true);
    assert!(scr_hit && tr_hit);
    let ops_x = scr_ops / tr_ops.max(1.0);
    let wall_x = scr_ms / tr_ms.max(1e-6);
    let (a, b, c, comp, verdict) = score_row(lift, deg, max_c, ops_x, deg, max_c);
    let param = ParamS5::build(m as usize, target.a, target.b, p).expect("ParamS5");
    let (_, spec_m) = param.specialize(77 % p);
    AblationRow {
        family: "j1728".into(),
        p,
        aux_a: aux.a,
        aux_b: aux.b,
        aux_order: p + 1,
        aux_smooth_note: format!("#E=p+1=2^{odd_shift}"),
        depth,
        m,
        matched_v: v_xs.len(),
        lift_rate: lift,
        fb_columns: fb.columns,
        solving_degree_max: deg,
        max_cols: max_c,
        scratch_ops_per_call: scr_ops,
        trace_ops_per_call: tr_ops,
        ops_speedup_x: ops_x,
        wall_speedup_x: wall_x,
        planted_hit_verified: tr_hit,
        param_s5_spec_mults: spec_m,
        score_a: a,
        score_b: b,
        score_c: c,
        composite_score: comp,
        verdict,
    }
}

/// Search `y^2 = x^3 + b` for smooth `#E` (prefer high 2-adic or 3-adic valuation).
fn find_smooth_j0_aux(p: u64, seed: u64) -> (PrimeCurve, u64, String) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut best: Option<(PrimeCurve, u64, u32, String)> = None;
    for _ in 0..64 {
        let b = rng.gen_range(1..p);
        // j=0 needs a=0 and nonsingular ⇒ b≠0; disc = 27 b^2 ≠ 0.
        let aux = PrimeCurve { p, a: 0, b };
        let n = aux.point_count();
        let mut v2 = 0u32;
        let mut t = n;
        while t % 2 == 0 {
            t /= 2;
            v2 += 1;
        }
        let mut v3 = 0u32;
        while t % 3 == 0 {
            t /= 3;
            v3 += 1;
        }
        let smooth_score = v2 + 2 * v3;
        let note = format!("#E={n}, v2={v2}, v3={v3}, cofactor_odd={t}");
        if best
            .as_ref()
            .map(|(_, _, s, _)| smooth_score > *s)
            .unwrap_or(true)
            && (v2 >= 3 || v3 >= 2)
        {
            best = Some((aux, n, smooth_score, note));
        }
    }
    let (aux, n, _, note) = best.expect("no smooth-ish j=0 aux found on M19");
    (aux, n, note)
}

fn run_j0_family(p: u64, depth: u32, m: u32, baseline: &AblationRow) -> AblationRow {
    // Target stays a generic prime-order curve; only the *aux* is j=0.
    let (target, g, r) = find_prime_order_curve_over_fixed_p(p, Some(p - 3), 20261121 + 7);
    let (aux, n, note) = find_smooth_j0_aux(p, 20261121);
    // Prefer 2-power tower when v2(#E) >= depth; else fall back to abscissa FB without isogeny.
    let mut v2 = 0u32;
    let mut t = n;
    while t % 2 == 0 {
        t /= 2;
        v2 += 1;
    }
    if v2 >= depth {
        let ker_top = find_full_2power_point(&aux, v2);
        let (steps, chain_cods, ker_base) = build_velu2_tower(&aux, ker_top, depth, v2);
        let (v_xs, c_term, fb, lift) = pick_coset_with_lifts(
            &aux,
            &target,
            r,
            &steps,
            &chain_cods,
            ker_base,
            depth,
            m,
            "j0-aux",
        );
        let (scr_ops, deg, max_c, scr_ms, scr_hit) =
            run_iso_oracle(&target, g, r, &fb, m, depth, &v_xs, &steps, c_term, false);
        let (tr_ops, _, _, tr_ms, tr_hit) =
            run_iso_oracle(&target, g, r, &fb, m, depth, &v_xs, &steps, c_term, true);
        let ops_x = scr_ops / tr_ops.max(1.0);
        let wall_x = scr_ms / tr_ms.max(1e-6);
        let (a, bsc, c, comp, verdict) = score_row(
            lift,
            deg,
            max_c,
            ops_x,
            baseline.solving_degree_max,
            baseline.max_cols,
        );
        let param = ParamS5::build(m as usize, target.a, target.b, p).expect("ParamS5");
        let (_, spec_m) = param.specialize(77 % p);
        AblationRow {
            family: "j0".into(),
            p,
            aux_a: 0,
            aux_b: aux.b,
            aux_order: n,
            aux_smooth_note: note,
            depth,
            m,
            matched_v: v_xs.len(),
            lift_rate: lift,
            fb_columns: fb.columns,
            solving_degree_max: deg,
            max_cols: max_c,
            scratch_ops_per_call: scr_ops,
            trace_ops_per_call: tr_ops,
            ops_speedup_x: ops_x,
            wall_speedup_x: wall_x,
            planted_hit_verified: scr_hit && tr_hit,
            param_s5_spec_mults: spec_m,
            score_a: a,
            score_b: bsc,
            score_c: c,
            composite_score: comp,
            verdict,
        }
    } else {
        // Abscissa-only lift comparison at matched |V|=2^depth (no isogeny oracle).
        let mut xs = Vec::new();
        for x in 2..p {
            if aux.sqrt(aux.rhs(x)).is_some_and(|y| y != 0) {
                xs.push(x);
                if xs.len() >= (1usize << depth) {
                    break;
                }
            }
        }
        let (fb, lift) = lift_fb_from_coset(&target, r, &xs, "j0-abscissa");
        let (a, bsc, c, comp, verdict) = score_row(
            lift,
            baseline.solving_degree_max,
            baseline.max_cols,
            1.0,
            baseline.solving_degree_max,
            baseline.max_cols,
        );
        let _ = g;
        let param = ParamS5::build(m as usize, 0, aux.b, p).expect("ParamS5");
        let (_, spec_m) = param.specialize(77 % p);
        AblationRow {
            family: "j0-abscissa".into(),
            p,
            aux_a: 0,
            aux_b: aux.b,
            aux_order: n,
            aux_smooth_note: format!("{note}; insufficient v2 for depth={depth} tower"),
            depth,
            m,
            matched_v: xs.len(),
            lift_rate: lift,
            fb_columns: fb.columns,
            solving_degree_max: 0,
            max_cols: 0,
            scratch_ops_per_call: 0.0,
            trace_ops_per_call: 0.0,
            ops_speedup_x: 1.0,
            wall_speedup_x: 1.0,
            planted_hit_verified: false,
            param_s5_spec_mults: spec_m,
            score_a: a,
            score_b: bsc,
            score_c: c,
            composite_score: comp,
            verdict: if lift > baseline.lift_rate * 1.05 {
                "CONTINUE_LIFT".into()
            } else {
                verdict
            },
        }
    }
}

/// Montgomery `B y^2 = x^3 + A x^2 + x` → short Weierstrass via standard map.
fn montgomery_to_weierstrass(p: u64, a_mont: u64, b_mont: u64) -> PrimeCurve {
    // (x,y) ↦ (u,v) = (x/B + A/(3B), y/B) gives v^2 = u^3 + a u + b with
    // a = (3 - A^2)/(3 B^2), b = (2 A^3 - 9 A)/(27 B^3).
    let inv_b = {
        let mut b = b_mont;
        let mut e = p - 2;
        let mut acc = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                acc = mulm(acc, b, p);
            }
            b = mulm(b, b, p);
            e >>= 1;
        }
        acc
    };
    let inv_b2 = mulm(inv_b, inv_b, p);
    let inv_b3 = mulm(inv_b2, inv_b, p);
    let inv3 = {
        let mut b = 3u64 % p;
        let mut e = p - 2;
        let mut acc = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                acc = mulm(acc, b, p);
            }
            b = mulm(b, b, p);
            e >>= 1;
        }
        acc
    };
    let a2 = mulm(a_mont, a_mont, p);
    let a3 = mulm(a2, a_mont, p);
    let a_w = mulm(subm(3, a2, p), mulm(inv3, inv_b2, p), p);
    let b_w = mulm(
        subm(mulm(2, a3, p), mulm(9, a_mont, p), p),
        mulm(mulm(inv3, inv3, p), mulm(inv3, inv_b3, p), p),
        p,
    );
    PrimeCurve { p, a: a_w, b: b_w }
}

fn run_montgomery_family(p: u64, depth: u32, m: u32, baseline: &AblationRow) -> AblationRow {
    let (target, _g, r) = find_prime_order_curve_over_fixed_p(p, Some(p - 3), 20261121 + 11);
    // Curve25519-like A=486662 scaled into F_p; B=1.
    let a_mont = 486662u64 % p;
    let b_mont = 1u64;
    let aux = montgomery_to_weierstrass(p, a_mont, b_mont);
    let n = aux.point_count();
    let mut v2 = 0u32;
    let mut t = n;
    while t % 2 == 0 {
        t /= 2;
        v2 += 1;
    }
    let note = format!(
        "Mont(A={a_mont},B=1)→Weierstrass a={},b={}; #E={n}, v2={v2}",
        aux.a, aux.b
    );
    if v2 >= depth {
        if let Some(ker_top) = try_full_2power_point(&aux, v2) {
            let (steps, chain_cods, ker_base) = build_velu2_tower(&aux, ker_top, depth, v2);
            let (v_xs, c_term, fb, lift) = pick_coset_with_lifts(
                &aux,
                &target,
                r,
                &steps,
                &chain_cods,
                ker_base,
                depth,
                m,
                "mont-aux",
            );
            let (scr_ops, deg, max_c, scr_ms, scr_hit) =
                run_iso_oracle(&target, _g, r, &fb, m, depth, &v_xs, &steps, c_term, false);
            let (tr_ops, _, _, tr_ms, tr_hit) =
                run_iso_oracle(&target, _g, r, &fb, m, depth, &v_xs, &steps, c_term, true);
            let ops_x = scr_ops / tr_ops.max(1.0);
            let wall_x = scr_ms / tr_ms.max(1e-6);
            let (a, bsc, c, comp, verdict) = score_row(
                lift,
                deg,
                max_c,
                ops_x,
                baseline.solving_degree_max,
                baseline.max_cols,
            );
            let param = ParamS5::build(m as usize, target.a, target.b, p).expect("ParamS5");
            let (_, spec_m) = param.specialize(77 % p);
            return AblationRow {
                family: "montgomery".into(),
                p,
                aux_a: aux.a,
                aux_b: aux.b,
                aux_order: n,
                aux_smooth_note: note,
                depth,
                m,
                matched_v: v_xs.len(),
                lift_rate: lift,
                fb_columns: fb.columns,
                solving_degree_max: deg,
                max_cols: max_c,
                scratch_ops_per_call: scr_ops,
                trace_ops_per_call: tr_ops,
                ops_speedup_x: ops_x,
                wall_speedup_x: wall_x,
                planted_hit_verified: scr_hit && tr_hit,
                param_s5_spec_mults: spec_m,
                score_a: a,
                score_b: bsc,
                score_c: c,
                composite_score: comp,
                verdict,
            };
        }
    }
    {
        let mut xs = Vec::new();
        for x in 2..p {
            if aux.sqrt(aux.rhs(x)).is_some_and(|y| y != 0) {
                xs.push(x);
                if xs.len() >= (1usize << depth) {
                    break;
                }
            }
        }
        let (fb, lift) = lift_fb_from_coset(&target, r, &xs, "mont-abscissa");
        let (a, bsc, c, comp, verdict) = score_row(
            lift,
            baseline.solving_degree_max,
            baseline.max_cols,
            1.0,
            baseline.solving_degree_max,
            baseline.max_cols,
        );
        AblationRow {
            family: "montgomery-abscissa".into(),
            p,
            aux_a: aux.a,
            aux_b: aux.b,
            aux_order: n,
            aux_smooth_note: format!("{note}; insufficient v2 for tower"),
            depth,
            m,
            matched_v: xs.len(),
            lift_rate: lift,
            fb_columns: fb.columns,
            solving_degree_max: 0,
            max_cols: 0,
            scratch_ops_per_call: 0.0,
            trace_ops_per_call: 0.0,
            ops_speedup_x: 1.0,
            wall_speedup_x: 1.0,
            planted_hit_verified: false,
            param_s5_spec_mults: 0,
            score_a: a,
            score_b: bsc,
            score_c: c,
            composite_score: comp,
            verdict: if lift > baseline.lift_rate * 1.05 {
                "CONTINUE_LIFT".into()
            } else {
                verdict
            },
        }
    }
}

fn run_twist_family(p: u64, depth: u32, m: u32, baseline: &AblationRow) -> AblationRow {
    let (target, _g, r) = find_prime_order_curve_over_fixed_p(p, Some(p - 3), 20261121 + 13);
    // Quadratic twist: y^2 = x^3 + a d^2 x + b d^3 for nonsquare d.
    let mut d = 2u64;
    while target.legendre(d) != -1 {
        d += 1;
    }
    let aux = PrimeCurve {
        p,
        a: mulm(target.a, mulm(d, d, p), p),
        b: mulm(target.b, mulm(d, mulm(d, d, p), p), p),
    };
    let n = aux.point_count();
    // Matched |V|: sample 2^depth aux abscissae that are affine on aux; measure lift to target.
    let mut xs = Vec::new();
    for x in 2..p {
        if aux.sqrt(aux.rhs(x)).is_some_and(|y| y != 0) {
            xs.push(x);
            if xs.len() >= (1usize << depth) {
                break;
            }
        }
    }
    let (fb, lift) = lift_fb_from_coset(&target, r, &xs, "twist-abscissa");
    let (a, bsc, c, comp, verdict) = score_row(
        lift,
        baseline.solving_degree_max,
        baseline.max_cols,
        1.0,
        baseline.solving_degree_max,
        baseline.max_cols,
    );
    let note = format!(
        "quadratic twist d={d}; #E_twist={n}; matched |V|={}",
        xs.len()
    );
    AblationRow {
        family: "twist".into(),
        p,
        aux_a: aux.a,
        aux_b: aux.b,
        aux_order: n,
        aux_smooth_note: note,
        depth,
        m,
        matched_v: xs.len(),
        lift_rate: lift,
        fb_columns: fb.columns,
        solving_degree_max: 0,
        max_cols: 0,
        scratch_ops_per_call: 0.0,
        trace_ops_per_call: 0.0,
        ops_speedup_x: 1.0,
        wall_speedup_x: 1.0,
        planted_hit_verified: false,
        param_s5_spec_mults: 0,
        score_a: a,
        score_b: bsc,
        score_c: c,
        composite_score: if lift > baseline.lift_rate * 1.05 {
            comp + 0.5
        } else {
            comp
        },
        verdict: if lift > baseline.lift_rate * 1.05 {
            "CONTINUE_LIFT".into()
        } else if lift < baseline.lift_rate * 0.95 {
            "KILL_LIFT_VS_J1728".into()
        } else {
            verdict
        },
    }
}

pub fn run_ablation() -> AblationSummary {
    let p = (1u64 << 19) - 1; // M19: point_count feasible; p≡3 mod 4 and p≡1 mod 3
    let odd_shift = 19u32;
    let depth = 3u32;
    let m = 2u32;
    eprintln!("=== Prime aux family ablation on M19 (p={p}), depth={depth}, m={m} ===");
    eprintln!("  -> j1728 baseline...");
    let j1728 = run_j1728_family(p, odd_shift, depth, m);
    eprintln!(
        "     lift={:.3} deg={} cols={} ops_x={:.2} verdict={}",
        j1728.lift_rate,
        j1728.solving_degree_max,
        j1728.max_cols,
        j1728.ops_speedup_x,
        j1728.verdict
    );
    eprintln!("  -> j0...");
    let j0 = run_j0_family(p, depth, m, &j1728);
    eprintln!(
        "     lift={:.3} deg={} cols={} note={} verdict={}",
        j0.lift_rate, j0.solving_degree_max, j0.max_cols, j0.aux_smooth_note, j0.verdict
    );
    eprintln!("  -> montgomery...");
    let mont = run_montgomery_family(p, depth, m, &j1728);
    eprintln!(
        "     lift={:.3} deg={} cols={} verdict={}",
        mont.lift_rate, mont.solving_degree_max, mont.max_cols, mont.verdict
    );
    eprintln!("  -> twist...");
    let twist = run_twist_family(p, depth, m, &j1728);
    eprintln!(
        "     lift={:.3} vs j1728={:.3} verdict={}",
        twist.lift_rate, j1728.lift_rate, twist.verdict
    );

    let mut rows = vec![j1728, j0, mont, twist];
    rows.sort_by(|a, b| {
        b.composite_score
            .partial_cmp(&a.composite_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let ranking = rows
        .iter()
        .map(|r| format!("{}:{:.2}:{}", r.family, r.composite_score, r.verdict))
        .collect();
    AblationSummary {
        schema: "prime.aux-family-ablation/v1".into(),
        date: "2026-11-21".into(),
        standing: "[pcc:unverified]".into(),
        protocol: "Matched |V|=2^depth on M19; axes A=lift, B=degree/cols vs j1728, C=trace ops; score=2A+3B+C".into(),
        rows,
        ranking,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j1728_point_count_on_m19() {
        let p = (1u64 << 19) - 1;
        let aux = PrimeCurve { p, a: p - 3, b: 0 };
        assert_eq!(aux.point_count(), p + 1);
    }

    #[test]
    fn montgomery_map_nonsingular_on_m19() {
        let p = (1u64 << 19) - 1;
        let w = montgomery_to_weierstrass(p, 486662 % p, 1);
        let disc = addm(
            mulm(4, mulm(mulm(w.a, w.a, p), w.a, p), p),
            mulm(27, mulm(w.b, w.b, p), p),
            p,
        );
        assert_ne!(disc, 0);
    }
}
