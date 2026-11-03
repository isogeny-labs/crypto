//! **E43 Stage A** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.58,
//! G2.258, G2.261–G2.264, G2.274): FV (`chain-fifo-profile`) and FW
//! (`chain-fifo-tails`) against A counted, F (`chain-fifo`), V
//! (`chain-profile`) and W (`chain-tails`) on fresh calls — E25's ten
//! curves, base and precompute from E25's stream, then the first `--targets`
//! targets of stream `seed ^ 0xE43A_0001` (per-target seeds as E37's Stage
//! A), one thread; per call existing (E25), A counted, F, V, W, FV and FW
//! from the same per-target seed.
//!
//! Identity per call (G2.258): every solver returns the existing solver's
//! solutions, degree, outcome and decomposition set; V and W return A
//! counted's fallback flag, phase counts and pivot digest, FV and FW return
//! F's; FV's stored cells equal V's on calls without fallback; FV's lead
//! scans equal F's minus `Σ_r lead_r`; FW's offset operations equal FV's;
//! FW's cells at build equal W's; FW's counter equals final lengths plus
//! discarded buffers; `tail_FW − tail_W = (U_F − U_A) + (R_FW − R_W)`; FW's
//! lead scans are `≤` FV's.  Cells charged at the band: A, F
//! `rows · cols · (1 + fallback)` (E31; F's selection work is 0); V, W, FV,
//! FW their stored cells, A's convention on a fallback call.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e43_stage_a --targets 1000 --json experiments/50_compose_fw/stage_a.json
//! ```
//! Restart-safe with `--resume`: curves already in the JSON are kept.

use crypto_lib::cryptanalysis::chain_pivot_rules::{ArrivalRule, RuleCheck};
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, solve_weighted_traced, v4_system, Pivot,
    SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::macaulay_profile::{profile_measure, BucketOrder, ProfileCheck};
use crypto_lib::cryptanalysis::macaulay_tails::{tails_measure_ordered, TailsCheck};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use serde_json::Value;

const SALT: u64 = 0xE43A_0001;
const NAMES: [&str; 7] = [
    "existing",
    "chain-counted",
    "chain-fifo",
    "chain-profile",
    "chain-tails",
    "chain-fifo-profile",
    "chain-fifo-tails",
];
const A: usize = 1;
const F: usize = 2;
const V: usize = 3;
const W: usize = 4;
const FV: usize = 5;
const FW: usize = 6;
/// Ratio pairs `(reference, arm)` reported per curve: `S_ref / S_arm`.
const PAIRS: [(usize, usize); 14] = [
    (A, F),
    (A, V),
    (V, W),
    (A, FV),
    (F, FV),
    (V, FV),
    (W, FV),
    (0, FV),
    (A, FW),
    (F, FW),
    (V, FW),
    (W, FW),
    (0, FW),
    (FV, FW),
];

fn pivot(k: usize) -> Option<Pivot> {
    match k {
        A => Some(Pivot::Counted(PivotRule::Lowest, PivotCheck::default())),
        F => Some(Pivot::Rule(ArrivalRule::Fifo, RuleCheck::default())),
        V => Some(Pivot::Profile(ProfileCheck::default())),
        W => Some(Pivot::Tails(TailsCheck::default())),
        FV => Some(Pivot::ProfileOrdered(
            ProfileCheck::default(),
            BucketOrder::Fifo,
        )),
        FW => Some(Pivot::TailsOrdered(
            TailsCheck::default(),
            BucketOrder::Fifo,
        )),
        _ => None,
    }
}

fn phases(s: &QuarticSolve) -> [u64; 9] {
    [
        s.weil_muls,
        s.triangularise_muls,
        s.echelon_muls,
        s.nf_muls,
        s.charpoly_muls,
        s.roots_muls,
        s.eigenvector_muls,
        s.verify_muls,
        s.total_muls,
    ]
}

/// Per call, in stream order.
#[derive(Default, Serialize, Clone)]
struct Arm {
    solver: &'static str,
    call: Vec<u64>,
    cells: Vec<u64>,
    offset_ops: Vec<u64>,
    tail_ops: Vec<u64>,
    lead_scan: Vec<u64>,
    fallback: Vec<bool>,
}

#[derive(Serialize)]
struct Summary {
    solver: &'static str,
    call_mean: f64,
    cells_mean: f64,
    cells_clean_mean: f64,
    cells_clean_min: u64,
    cells_clean_max: u64,
    offset_ops_mean: f64,
    tail_ops_mean: f64,
    lead_scan_mean: f64,
    fallbacks: usize,
    /// Mean per-call `S(g) = C + 97 g (cells + offset ops + tail ops)`.
    s_mean: [f64; 3],
    /// σ1: plus the lead-scan reads.
    s_sigma1_mean: [f64; 3],
}

#[derive(Serialize, Default)]
struct Checks {
    /// Calls where solver k returned the existing solver's output.
    agree: Vec<usize>,
    v_equals_a: usize,
    w_equals_a: usize,
    fv_equals_f: usize,
    fw_equals_f: usize,
    fv_stored_equals_v: usize,
    fv_lead_equals_f_minus_offsets: usize,
    fw_offsets_equal_fv: usize,
    fw_built_equals_w: usize,
    fw_counter_ok: usize,
    tail_identity: usize,
    fw_tail_measure_equals_solve: usize,
    fw_lead_le_fv: usize,
    same_fallbacks: usize,
}

#[derive(Serialize)]
struct Curve {
    p: u64,
    seed: u64,
    stream_salt: u64,
    calls: usize,
    checks: Checks,
    /// Re-allocations (sum over calls) of FW and W; the most on one call.
    fw_reallocs: u64,
    w_reallocs: u64,
    fw_reallocs_max: u64,
    /// Row updates (sum over calls) under F's and A's order.
    updates_f: u64,
    updates_a: u64,
    /// FW's and W's cells at build (mean over calls).
    built_mean: f64,
    /// Interactions per call (means over all calls; min, max over calls):
    /// `δ_FV = K_FV − K_V`, `δ_FW = K_FW − K_W`, and the reads term
    /// `Δreads = (off_FV − off_V) − 2 ΔU`.
    delta_fv_mean: f64,
    delta_fv_min: i64,
    delta_fv_max: i64,
    delta_fw_mean: f64,
    delta_fw_min: i64,
    delta_fw_max: i64,
    delta_reads_mean: f64,
    /// F's saving in cells-equivalent at 1 GAE, `(C_A − C_F)/97` (mean).
    break_even: f64,
    summaries: Vec<Summary>,
    /// `S_ref / S_arm` of the means for each of [`PAIRS`] at g = 0, 0.1, 1,
    /// and under σ1.
    ratios: Vec<(String, [f64; 3])>,
    ratios_sigma1: Vec<(String, [f64; 3])>,
    arms: Vec<Arm>,
}

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

fn mean_u(v: impl Iterator<Item = u64>) -> f64 {
    let (s, n) = v.fold((0u64, 0u64), |(s, n), x| (s + x, n + 1));
    s as f64 / n.max(1) as f64
}

fn k_of(a: &Arm, i: usize) -> i64 {
    (a.cells[i] + a.offset_ops[i] + a.tail_ops[i]) as i64
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let targets: usize = arg(&args, "--targets", 1000);
    let threads: u64 = arg(&args, "--threads", 4);
    let json: String = arg(&args, "--json", String::new());
    let resume = args.iter().any(|a| a == "--resume");
    // Restart safety: keep the curves already written (each curve is
    // written whole, atomically, after it completes).
    let mut out: Vec<Value> = if resume {
        std::fs::read_to_string(&json)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    for p in [101u64, 149, 197, 269, 397] {
        for seed in [1u64, 2] {
            if out.iter().any(|c| c["p"] == p && c["seed"] == seed) {
                eprintln!("p={p} seed={seed}: kept from {json}");
                continue;
            }
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let mut rng = StdRng::seed_from_u64(seed ^ SALT);
            let sseed = seed ^ SALT;
            let n = inst.n;
            let mut arms: Vec<Arm> = NAMES
                .iter()
                .map(|s| Arm {
                    solver: s,
                    ..Default::default()
                })
                .collect();
            let mut ck = Checks {
                agree: vec![0; NAMES.len()],
                ..Default::default()
            };
            let (mut fw_r, mut w_r, mut fw_rmax, mut upd_f, mut upd_a) =
                (0u64, 0u64, 0u64, 0u64, 0u64);
            let mut built_sum = 0f64;
            let mut dreads = 0f64;
            let mut calls = 0usize;
            let mut residuals = 0u64;
            'outer: loop {
                let batch: Vec<(u64, u64, u64)> = (0..threads * 2)
                    .map(|k| {
                        (
                            rng.gen_range(0..n),
                            rng.gen_range(1..n),
                            mix64(sseed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
                        )
                    })
                    .collect();
                for (a, b, tseed) in batch {
                    if calls >= targets {
                        break 'outer;
                    }
                    residuals += 1;
                    let c = inst.curve();
                    let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                    let c = inst.curve();
                    let before = c.f.muls();
                    let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                        continue;
                    };
                    let mut fp = 0u64;
                    let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                    let weil = c.f.muls() - before + fp;
                    calls += 1;
                    let mut results = Vec::new();
                    let mut sts = Vec::new();
                    let mut fells = Vec::new();
                    for k in 0..NAMES.len() {
                        let mut trng = StdRng::seed_from_u64(tseed);
                        let mut st = QuarticSolve {
                            weil_muls: weil,
                            ..Default::default()
                        };
                        let (sols, deg, fell) = match pivot(k) {
                            None => {
                                let (s, d) = solve_weighted_traced(
                                    &comps,
                                    MACAULAY_BOUND,
                                    p,
                                    &mut trng,
                                    &mut st,
                                    true,
                                    false,
                                    None,
                                    None,
                                );
                                (s, d, false)
                            }
                            Some(pv) => {
                                let (s, d, info) = solve_weighted_chain_with(
                                    &comps,
                                    MACAULAY_BOUND,
                                    p,
                                    &mut trng,
                                    &mut st,
                                    pv,
                                );
                                (s, d, info.fell_back)
                            }
                        };
                        let lb =
                            lift_back_v4(&ki, &c, sols.as_deref().unwrap_or(&[]), &r, &mut trng);
                        let dense = (st.rows * st.cols) as u64;
                        let cells = if k >= V {
                            if fell {
                                2 * dense
                            } else {
                                st.macaulay_stored_entries
                            }
                        } else {
                            dense * (1 + u64::from(fell))
                        };
                        let arm = &mut arms[k];
                        arm.call.push(st.total_muls);
                        arm.cells.push(cells);
                        arm.offset_ops.push(st.profile_offset_ops);
                        arm.tail_ops.push(st.tail_extent_ops);
                        arm.lead_scan.push(st.lead_scan_reads);
                        arm.fallback.push(fell);
                        results.push((sols, deg, st.outcome.clone(), lb.decompositions));
                        sts.push(st);
                        fells.push(fell);
                    }
                    for k in 0..NAMES.len() {
                        ck.agree[k] += usize::from(results[k] == results[0]);
                    }
                    let same = |x: usize, refk: usize| {
                        fells[x] == fells[refk]
                            && phases(&sts[x]) == phases(&sts[refk])
                            && sts[x].pivot_digest == sts[refk].pivot_digest
                    };
                    ck.v_equals_a += usize::from(same(V, A));
                    ck.w_equals_a += usize::from(same(W, A));
                    ck.fv_equals_f += usize::from(same(FV, F));
                    ck.fw_equals_f += usize::from(same(FW, F));
                    ck.same_fallbacks += usize::from(fells.iter().skip(1).all(|&x| x == fells[A]));
                    if fells[V] || fells[FV] {
                        ck.fv_stored_equals_v += 1;
                    } else {
                        ck.fv_stored_equals_v += usize::from(
                            sts[FV].macaulay_stored_entries == sts[V].macaulay_stored_entries,
                        );
                    }
                    let offs = profile_measure(&comps, MACAULAY_BOUND, p, ProfileCheck::default())
                        .map_or(0, |m| m.offsets);
                    ck.fv_lead_equals_f_minus_offsets +=
                        usize::from(sts[FV].lead_scan_reads + offs == sts[F].lead_scan_reads);
                    ck.fw_offsets_equal_fv +=
                        usize::from(sts[FW].profile_offset_ops == sts[FV].profile_offset_ops);
                    ck.fw_lead_le_fv +=
                        usize::from(sts[FW].lead_scan_reads <= sts[FV].lead_scan_reads);
                    let mf = tails_measure_ordered(
                        &comps,
                        MACAULAY_BOUND,
                        p,
                        TailsCheck::default(),
                        BucketOrder::Fifo,
                    );
                    let ma = tails_measure_ordered(
                        &comps,
                        MACAULAY_BOUND,
                        p,
                        TailsCheck::default(),
                        BucketOrder::Lowest,
                    );
                    if let (Some(mf), Some(ma)) = (mf, ma) {
                        let (f, a_) = (mf.measure, ma.measure);
                        ck.fw_built_equals_w += usize::from(mf.built_cells == ma.built_cells);
                        ck.fw_counter_ok += usize::from(f.counter == f.final_lengths + f.discarded);
                        ck.tail_identity += usize::from(
                            f.tail_ops as i64 - a_.tail_ops as i64
                                == (mf.updates as i64 - ma.updates as i64)
                                    + (f.reallocs as i64 - a_.reallocs as i64),
                        );
                        ck.fw_tail_measure_equals_solve += usize::from(
                            f.tail_ops == sts[FW].tail_extent_ops
                                && a_.tail_ops == sts[W].tail_extent_ops,
                        );
                        fw_r += f.reallocs;
                        w_r += a_.reallocs;
                        fw_rmax = fw_rmax.max(f.reallocs);
                        upd_f += mf.updates;
                        upd_a += ma.updates;
                        built_sum += mf.built_cells as f64;
                        dreads += (sts[FV].profile_offset_ops as f64
                            - sts[V].profile_offset_ops as f64)
                            - 2.0 * (mf.updates as f64 - ma.updates as f64);
                    }
                }
            }
            let summaries: Vec<Summary> = arms
                .iter()
                .map(|a| {
                    let clean: Vec<u64> = a
                        .cells
                        .iter()
                        .zip(&a.fallback)
                        .filter(|(_, &f)| !f)
                        .map(|(&c, _)| c)
                        .collect();
                    let s = |g: f64, sigma1: bool| -> f64 {
                        (0..a.call.len())
                            .map(|i| {
                                let extra = if sigma1 { a.lead_scan[i] } else { 0 };
                                a.call[i] as f64
                                    + 97.0
                                        * g
                                        * (a.cells[i] + a.offset_ops[i] + a.tail_ops[i] + extra)
                                            as f64
                            })
                            .sum::<f64>()
                            / a.call.len() as f64
                    };
                    Summary {
                        solver: a.solver,
                        call_mean: mean_u(a.call.iter().copied()),
                        cells_mean: mean_u(a.cells.iter().copied()),
                        cells_clean_mean: mean_u(clean.iter().copied()),
                        cells_clean_min: clean.iter().copied().min().unwrap_or(0),
                        cells_clean_max: clean.iter().copied().max().unwrap_or(0),
                        offset_ops_mean: mean_u(a.offset_ops.iter().copied()),
                        tail_ops_mean: mean_u(a.tail_ops.iter().copied()),
                        lead_scan_mean: mean_u(a.lead_scan.iter().copied()),
                        fallbacks: a.fallback.iter().filter(|&&f| f).count(),
                        s_mean: [0.0, 0.1, 1.0].map(|g| s(g, false)),
                        s_sigma1_mean: [0.0, 0.1, 1.0].map(|g| s(g, true)),
                    }
                })
                .collect();
            let rt = |x: usize, y: usize, sigma1: bool| -> [f64; 3] {
                core::array::from_fn(|g| {
                    if sigma1 {
                        summaries[x].s_sigma1_mean[g] / summaries[y].s_sigma1_mean[g]
                    } else {
                        summaries[x].s_mean[g] / summaries[y].s_mean[g]
                    }
                })
            };
            let label = |x: usize, y: usize| format!("{}/{}", NAMES[x], NAMES[y]);
            let ratios: Vec<(String, [f64; 3])> = PAIRS
                .iter()
                .map(|&(x, y)| (label(x, y), rt(x, y, false)))
                .collect();
            let ratios_sigma1: Vec<(String, [f64; 3])> = PAIRS
                .iter()
                .map(|&(x, y)| (label(x, y), rt(x, y, true)))
                .collect();
            let dfv: Vec<i64> = (0..calls)
                .map(|i| k_of(&arms[FV], i) - k_of(&arms[V], i))
                .collect();
            let dfw: Vec<i64> = (0..calls)
                .map(|i| k_of(&arms[FW], i) - k_of(&arms[W], i))
                .collect();
            let mean_i = |v: &[i64]| v.iter().sum::<i64>() as f64 / v.len().max(1) as f64;
            let cn = calls.max(1) as f64;
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree {:?}, V=A {} W=A {} FV=F {} FW=F {} | FV stored=V {} lead {} | FW ops=FV {} built=W {} counter {} tail-id {} lead<=FV {} | fb same {} | reallocs FW {} W {} (max {fw_rmax}) | upd F {} A {}",
                ck.agree, ck.v_equals_a, ck.w_equals_a, ck.fv_equals_f, ck.fw_equals_f,
                ck.fv_stored_equals_v, ck.fv_lead_equals_f_minus_offsets, ck.fw_offsets_equal_fv,
                ck.fw_built_equals_w, ck.fw_counter_ok, ck.tail_identity, ck.fw_lead_le_fv,
                ck.same_fallbacks, fw_r, w_r, upd_f, upd_a
            );
            for s in &summaries {
                eprintln!(
                    "  {:>18}: C {:>9.0} | cells {:>9.0} (clean {:>9.0} [{}, {}]) | ops {:>7.0} | tail {:>6.0} | lead {:>7.0} | fb {:>2} | S {:.4e} {:.4e} {:.4e}",
                    s.solver, s.call_mean, s.cells_mean, s.cells_clean_mean, s.cells_clean_min,
                    s.cells_clean_max, s.offset_ops_mean, s.tail_ops_mean, s.lead_scan_mean,
                    s.fallbacks, s.s_mean[0], s.s_mean[1], s.s_mean[2]
                );
            }
            for (l, r) in &ratios {
                eprintln!("  {l:>40}: {:.6} {:.4} {:.4}", r[0], r[1], r[2]);
            }
            let curve = Curve {
                p,
                seed,
                stream_salt: SALT,
                calls,
                checks: ck,
                fw_reallocs: fw_r,
                w_reallocs: w_r,
                fw_reallocs_max: fw_rmax,
                updates_f: upd_f,
                updates_a: upd_a,
                built_mean: built_sum / cn,
                delta_fv_mean: mean_i(&dfv),
                delta_fv_min: dfv.iter().copied().min().unwrap_or(0),
                delta_fv_max: dfv.iter().copied().max().unwrap_or(0),
                delta_fw_mean: mean_i(&dfw),
                delta_fw_min: dfw.iter().copied().min().unwrap_or(0),
                delta_fw_max: dfw.iter().copied().max().unwrap_or(0),
                delta_reads_mean: dreads / cn,
                break_even: (summaries[A].call_mean - summaries[F].call_mean) / 97.0,
                summaries,
                ratios,
                ratios_sigma1,
                arms,
            };
            out.push(serde_json::to_value(&curve).unwrap());
            if !json.is_empty() {
                let tmp = format!("{json}.part");
                std::fs::write(&tmp, serde_json::to_string(&out).unwrap()).unwrap();
                std::fs::rename(&tmp, &json).unwrap();
            }
        }
    }
    // Curves in the registered order whatever the resume order.
    out.sort_by_key(|c| (c["p"].as_u64(), c["seed"].as_u64()));
    if !json.is_empty() {
        let tmp = format!("{json}.part");
        std::fs::write(&tmp, serde_json::to_string(&out).unwrap()).unwrap();
        std::fs::rename(&tmp, &json).unwrap();
    }
}
