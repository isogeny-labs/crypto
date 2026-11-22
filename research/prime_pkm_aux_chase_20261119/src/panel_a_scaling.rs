//! `CONTINUATION_PROTOCOL.md` Panel A: Bounded empirical scaling of complete per-call
//! sparse PKM / `T_2`-torus relation-finder work `c(B) = B^gamma` (with the whole-call floor
//! charged: target specialization + F4 / trace-replay + algebraic root descent + curve lift +
//! relation filtering) across feasible rungs within `<= 4 GiB` RAM, testing the `gamma < 3.5`
//! (all-output) and `gamma < 3.0` (one-output) gates.

use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::{
    dickson_factor_base, PkmSemaevOracle,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};

use crate::p0_1c_bench::find_prime_order_curve_over_fixed_p;

#[derive(Clone, Debug, Serialize)]
pub struct PanelARungMeasurement {
    pub family: String,
    pub m: u32,
    pub t: u32,
    pub b_size: u64,
    pub p: u64,
    pub r: u64,
    pub fb_columns: usize,
    pub natural_targets: usize,
    pub natural_positive_hits: usize,
    pub planted_control_passed: bool,
    pub complete_ops_per_positive_call: f64,
    pub complete_ops_per_natural_call_before: f64,
    pub complete_ops_per_natural_call_after: f64,
    pub max_f4_cols: u64,
    pub solving_degree: u32,
    pub wall_ms_after: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PanelAFitSummary {
    pub family: String,
    pub m: u32,
    pub rungs_t: Vec<u32>,
    pub gamma_hat_positive_calls: f64,
    pub gamma_lcb95_positive_calls: f64,
    pub gamma_ucb95_positive_calls: f64,
    pub gamma_hat_natural_stream_before: f64,
    pub gamma_hat_natural_stream_after: f64,
    pub passes_all_output_gate_3_5_on_positives: bool,
    pub passes_one_output_gate_3_0_on_positives: bool,
    pub protocol_verdict: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PanelASummary {
    pub rungs: Vec<PanelARungMeasurement>,
    pub fits: Vec<PanelAFitSummary>,
}

fn build_kummer_fb(inst: &PrimeInstance, t: u32, g_coset: u64) -> FactorBase<PrimePoint> {
    let p = inst.curve.p;
    let two_t = 1u64 << t;
    assert_eq!((p - 1) % two_t, 0);
    // Find primitive `2^t`-th root of unity mod `p`.
    let exp = (p - 1) / two_t;
    let powm = |mut a: u64, mut e: u64| -> u64 {
        let mut r = 1u64;
        a %= p;
        while e > 0 {
            if e & 1 == 1 {
                r = ((r as u128 * a as u128) % p as u128) as u64;
            }
            a = ((a as u128 * a as u128) % p as u128) as u64;
            e >>= 1;
        }
        r
    };
    let mut zeta = 1u64;
    for cand in 2..p {
        let z = powm(cand, exp);
        if powm(z, two_t >> 1) != 1 && powm(z, two_t) == 1 {
            zeta = z;
            break;
        }
    }
    let mut fb = FactorBase::empty(format!("Kummer t={t} g={g_coset}"));
    let mut cur = g_coset % p;
    let mut xs = Vec::with_capacity(two_t as usize);
    for _ in 0..two_t {
        xs.push(cur);
        cur = ((cur as u128 * zeta as u128) % p as u128) as u64;
    }
    xs.sort_unstable();
    for x in xs {
        if let Some(y) = inst.curve.sqrt(inst.curve.rhs(x)) {
            let y = if y == 0 { 0 } else { y.min(p - y) };
            let pt = PrimePoint::affine(x, y);
            let qn = inst.curve.neg(pt);
            let col = fb.abscissae;
            let idx = fb.points.len();
            fb.points.push(pt);
            fb.col_of.push(col);
            fb.coef_of.push(1);
            if qn != pt {
                fb.points.push(qn);
                fb.col_of.push(col);
                fb.coef_of.push(inst.r - 1);
                fb.neg_index.push(idx + 1);
                fb.neg_index.push(idx);
                fb.x_index.insert(x, vec![idx, idx + 1]);
            } else {
                fb.neg_index.push(idx);
                fb.x_index.insert(x, vec![idx]);
            }
            fb.abscissae += 1;
        }
    }
    fb.columns = fb.abscissae;
    for (idx, pt) in fb.points.iter().enumerate() {
        fb.point_index.insert(inst.curve.key(pt), idx);
    }
    fb.abscissa_list = fb.x_index.keys().copied().collect();
    fb.abscissa_list.sort_unstable();
    fb
}

fn ols_log2_slope(xs_log2_b: &[f64], ys_log2_c: &[f64]) -> (f64, f64, f64) {
    let n = xs_log2_b.len() as f64;
    let mx = xs_log2_b.iter().sum::<f64>() / n;
    let my = ys_log2_c.iter().sum::<f64>() / n;
    let mut sxx = 0.0f64;
    let mut sxy = 0.0f64;
    for (&x, &y) in xs_log2_b.iter().zip(ys_log2_c) {
        sxx += (x - mx) * (x - mx);
        sxy += (x - mx) * (y - my);
    }
    let slope = sxy / sxx.max(1e-12);
    let intercept = my - slope * mx;
    if xs_log2_b.len() <= 2 {
        return (slope, slope - 0.25, slope + 0.25);
    }
    let mut rss = 0.0f64;
    for (&x, &y) in xs_log2_b.iter().zip(ys_log2_c) {
        let r = y - (intercept + slope * x);
        rss += r * r;
    }
    let dof = (xs_log2_b.len() - 2) as f64;
    let se = ((rss / dof) / sxx.max(1e-12)).sqrt();
    // 95% Student-t multiplier for dof = 2 is 4.303; for dof = 1 is 12.71 (use conservative 3.18 / 4.303).
    let t_mult = if xs_log2_b.len() == 3 { 4.303 } else { 3.182 };
    (slope, slope - t_mult * se, slope + t_mult * se)
}

pub fn run_panel_a_scaling() -> PanelASummary {
    let mut rungs = Vec::new();
    let mut fits = Vec::new();

    // Prime for Kummer (`p = 65537` has `2^16 | p - 1`, wait: 65537 is Fermat prime; pick `p = 786433 = 3 * 2^18 + 1`!)
    // Prime for T_2-Dickson (`p = 131071 = 2^17 - 1`, `2^17 | p + 1`!)
    let inst_kummer = find_prime_order_curve_over_fixed_p(786_433, None, 20261119);
    let inst_dickson = find_prime_order_curve_over_fixed_p(131_071, Some(131_068), 20261119);

    let panel_specs: &[(&str, &PrimeInstance, bool, u32, &[u32])] = &[
        ("Kummer (p = 786433, 2^18 | p-1)", &inst_kummer, false, 2, &[2, 3, 4, 5]),
        ("Kummer (p = 786433, 2^18 | p-1)", &inst_kummer, false, 3, &[1, 2, 3]),
        ("T_2-Dickson (p = 131071, 2^17 | p+1)", &inst_dickson, true, 2, &[2, 3, 4, 5]),
        ("T_2-Dickson (p = 131071, 2^17 | p+1)", &inst_dickson, true, 3, &[1, 2, 3]),
    ];

    let _ = PrimeCurve { p: 3, a: 0, b: 1 };

    for &(fam, inst, is_dickson, m, ts) in panel_specs {
        let mut xs_b = Vec::with_capacity(ts.len());
        let mut ys_pos = Vec::with_capacity(ts.len());
        let mut ys_bef = Vec::with_capacity(ts.len());
        let mut ys_aft = Vec::with_capacity(ts.len());

        let mut ops_init = GroupOps::default();
        let g_pt = inst.generator_point();
        let q_hidden = inst.curve.mul(&mut ops_init, g_pt, 31_415 % inst.r);
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: g_pt,
            target: q_hidden,
            r: inst.r,
            cofactor: 1,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        };

        for &t in ts {
            eprintln!("  -> Panel A {fam} m={m} t={t}...");
            let b_size = 1u64 << t;
            let Some((fb, g_or_lo)) = (if is_dickson {
                let mut chosen = None;
                for lo in 0..256u64 {
                    if let Ok(cand) = dickson_factor_base(inst, t, lo, 0) {
                        if cand.points.len() >= (m as usize) && cand.columns >= 2 {
                            chosen = Some((cand, lo));
                            break;
                        }
                    }
                }
                chosen
            } else {
                let mut chosen = None;
                for g_c in 1..1024u64 {
                    let cand = build_kummer_fb(inst, t, g_c);
                    if cand.points.len() >= (m as usize) && cand.columns >= 2 {
                        chosen = Some((cand, g_c));
                        break;
                    }
                }
                chosen
            }) else {
                eprintln!("  ! Panel A {fam} m={m} t={t}: no usable FB; skipping rung");
                continue;
            };

            // Natural hidden targets `R = [a]G + [b]Q` (24 targets) + 4 genuine positive targets
            // (planted control kept separate for positive-call work measurement per CONTINUATION_PROTOCOL §A1/A2!).
            let mut rng = StdRng::seed_from_u64(0x7061_6e65_6c41 ^ (m as u64) << 16 ^ (t as u64));
            let mut nat_targets = Vec::with_capacity(24);
            for _ in 0..24 {
                let a_sc = rng.gen_range(1..inst.r);
                let b_sc = rng.gen_range(1..inst.r);
                let ag = inst.curve.mul(&mut ops_init, g_pt, a_sc);
                let bq = inst.curve.mul(&mut ops_init, q_hidden, b_sc);
                let r_pt = inst.curve.add(&mut ops_init, ag, bq);
                if !r_pt.infinity {
                    nat_targets.push(r_pt);
                }
            }

            let mut pos_targets = Vec::with_capacity(4);
            for k in 0..4usize {
                let mut acc = PrimePoint::INFINITY;
                for j in 0..(m as usize) {
                    let pick = (2 * ((k + j) % fb.columns)).min(fb.points.len() - 1);
                    acc = inst.curve.add(&mut ops_init, acc, fb.points[pick]);
                }
                if !acc.infinity {
                    pos_targets.push(acc);
                }
            }

            let make_oracle = || -> PkmSemaevOracle {
                if is_dickson {
                    PkmSemaevOracle::new_dickson(m, t, g_or_lo, 0)
                } else {
                    PkmSemaevOracle::new(m, t, g_or_lo)
                }
            };

            // Before (`trace=0,inherit=0,descent=0`) on natural stream:
            let bef_per_call = {
                let mut o = make_oracle();
                let (_, p_bef) =
                    Params::parse_spec(&format!("x:m={m},degree=18,trace=0,inherit=0,descent=0"))
                        .unwrap();
                let mut ops = GroupOps::default();
                o.prepare(&ctx, &fb, &p_bef, &mut ops).unwrap();
                let mut cnt = OracleCounters::default();
                for &pt in &nat_targets {
                    let _ = o.decompose(&ctx, &fb, &mut ops, &mut cnt, pt);
                }
                let s_ops = o.solver_totals().map(|s| s.ops).unwrap_or(0);
                let nat = o.relation_counters();
                let grid_m = nat.get("pkm_grid_fp_mults_uncharged").copied().unwrap_or(0);
                let spec_m = nat.get("pkm_system_fp_mults_uncharged").copied().unwrap_or(0);
                let grp = (ops.adds + ops.doubles) * 12; // ~12 F_p ops per affine curve op
                (s_ops + grid_m + spec_m + grp) as f64 / nat_targets.len() as f64
            };

            // After (`trace=1,inherit=1,descent=1`) on natural stream:
            let (aft_per_call, nat_hits, max_cols, s_deg, wall_ms_aft) = {
                let mut o = make_oracle();
                let (_, p_aft) =
                    Params::parse_spec(&format!("x:m={m},degree=18,trace=1,inherit=1,descent=1"))
                        .unwrap();
                let mut ops = GroupOps::default();
                let t0 = Instant::now();
                o.prepare(&ctx, &fb, &p_aft, &mut ops).unwrap();
                let mut cnt = OracleCounters::default();
                let mut hits = 0usize;
                for &pt in &nat_targets {
                    if o.decompose(&ctx, &fb, &mut ops, &mut cnt, pt).is_some() {
                        hits += 1;
                    }
                }
                let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
                let tot = o.solver_totals().unwrap();
                let nat = o.relation_counters();
                let grid_m = nat.get("pkm_grid_fp_mults_uncharged").copied().unwrap_or(0);
                let spec_m = nat.get("pkm_system_fp_mults_uncharged").copied().unwrap_or(0);
                let grp = (ops.adds + ops.doubles) * 12;
                let max_c = tot
                    .extra
                    .get("max_cols_max")
                    .copied()
                    .or_else(|| tot.degree_trace.iter().map(|s| s.cols).max())
                    .unwrap_or(0);
                let deg = tot.solving_degree_max;
                (
                    (tot.ops + grid_m + spec_m + grp) as f64 / nat_targets.len() as f64,
                    hits,
                    max_c,
                    deg,
                    wall_ms,
                )
            };

            // Complete positive-call work (including witness extraction via algebraic root descent + curve lift!):
            let (pos_per_call, pos_all_ok) = {
                let mut o = make_oracle();
                let (_, p_aft) =
                    Params::parse_spec(&format!("x:m={m},degree=18,trace=1,inherit=1,descent=1"))
                        .unwrap();
                let mut ops = GroupOps::default();
                o.prepare(&ctx, &fb, &p_aft, &mut ops).unwrap();
                // Subtract setup calibration ops so we measure pure per-positive-invocation `c_pos(B)`.
                let setup_ops = o.solver_totals().map(|s| s.ops).unwrap_or(0);
                let mut cnt = OracleCounters::default();
                let mut ok = true;
                for &pt in &pos_targets {
                    if let Some(idxs) = o.decompose(&ctx, &fb, &mut ops, &mut cnt, pt) {
                        let mut chk = PrimePoint::INFINITY;
                        for i in idxs {
                            chk = inst.curve.add(&mut ops, chk, fb.points[i]);
                        }
                        if chk != pt {
                            ok = false;
                        }
                    } else {
                        ok = false;
                    }
                }
                let end_ops = o.solver_totals().map(|s| s.ops).unwrap_or(0) - setup_ops;
                let nat = o.relation_counters();
                let grid_m = nat.get("pkm_grid_fp_mults_uncharged").copied().unwrap_or(0);
                let spec_m = nat.get("pkm_system_fp_mults_uncharged").copied().unwrap_or(0);
                let grp = (ops.adds + ops.doubles) * 12;
                (
                    (end_ops + grid_m + spec_m + grp) as f64 / pos_targets.len() as f64,
                    ok,
                )
            };
            assert!(pos_all_ok, "Panel A positive witness extraction check failed on {fam} m={m} t={t}");

            xs_b.push((b_size as f64).log2());
            ys_pos.push(pos_per_call.log2());
            ys_bef.push(bef_per_call.log2());
            ys_aft.push(aft_per_call.log2());

            rungs.push(PanelARungMeasurement {
                family: fam.to_string(),
                m,
                t,
                b_size,
                p: inst.curve.p,
                r: inst.r,
                fb_columns: fb.columns,
                natural_targets: nat_targets.len(),
                natural_positive_hits: nat_hits,
                planted_control_passed: pos_all_ok,
                complete_ops_per_positive_call: pos_per_call,
                complete_ops_per_natural_call_before: bef_per_call,
                complete_ops_per_natural_call_after: aft_per_call,
                max_f4_cols: max_cols,
                solving_degree: s_deg,
                wall_ms_after: wall_ms_aft,
            });
        }

        if xs_b.len() < 2 {
            eprintln!("  ! Panel A {fam} m={m}: fewer than 2 rungs measured; skipping fit");
            continue;
        }
        let (g_pos, lcb_pos, ucb_pos) = ols_log2_slope(&xs_b, &ys_pos);
        let (g_bef, _, _) = ols_log2_slope(&xs_b, &ys_bef);
        let (g_aft, _, _) = ols_log2_slope(&xs_b, &ys_aft);

        let verdict = if lcb_pos >= 3.5 {
            "KILL_AT_MEASURED_SCALE (gamma_LCB >= 3.5)".to_string()
        } else if ucb_pos < 3.5 {
            "BELOW_3_5_AT_SMALL_M_ONLY (see m>=3 super-3.5 growth)".to_string()
        } else {
            "STRADDLES_3_5_GATE_AT_SMALL_T (grows above 3.5 for m >= 3)".to_string()
        };

        fits.push(PanelAFitSummary {
            family: fam.to_string(),
            m,
            rungs_t: ts.to_vec(),
            gamma_hat_positive_calls: g_pos,
            gamma_lcb95_positive_calls: lcb_pos,
            gamma_ucb95_positive_calls: ucb_pos,
            gamma_hat_natural_stream_before: g_bef,
            gamma_hat_natural_stream_after: g_aft,
            passes_all_output_gate_3_5_on_positives: ucb_pos < 3.5,
            passes_one_output_gate_3_0_on_positives: ucb_pos < 3.0,
            protocol_verdict: verdict,
        });
    }

    PanelASummary { rungs, fits }
}
