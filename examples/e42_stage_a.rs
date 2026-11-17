//! **E42 Stage A** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.54,
//! G2.230, G2.232–G2.235, G2.241–G2.243): W (`chain-tails`, rows reserved to
//! the block after their build end) against V (`chain-profile`) and A
//! counted on fresh calls — E25's ten curves, base and precompute from E25's
//! stream, then the first `--targets` targets of stream `seed ^ 0xE42A_0001`
//! (per-target seeds as E37's Stage A), one thread; per call existing (E25),
//! A counted, V and W from the same per-target seed.
//!
//! Identity per call (G2.230): every solver returns the existing solver's
//! solutions, degree, outcome and decomposition set; W returns A counted's
//! fallback flag, every phase count and pivot digest; W's offset operations
//! equal V's; W's stored-cell counter equals its final row lengths plus the
//! cells of the buffers its re-allocations discarded; W's lead-scan reads
//! are `≤` V's.  Cells: A `rows · cols · (1 + fallback)` (E31); V and W their
//! stored cells, or A's convention on a fallback call.  Stage 0's replay
//! (`stage0_tails`, nothing charged) on the same system gives the fill
//! extent and the dense-update cells (G2.242, G2.243).
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e42_stage_a --targets 1000 --json experiments/49_macaulay_tails/stage_a.json
//! ```

use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, solve_weighted_traced, v4_system, Pivot,
    SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::macaulay_profile::ProfileCheck;
use crypto_lib::cryptanalysis::macaulay_tails::{stage0_tails, tails_measure, TailsCheck};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

const SALT: u64 = 0xE42A_0001;
const NAMES: [&str; 4] = ["existing", "chain-counted", "chain-profile", "chain-tails"];
const A_COUNTED: usize = 1;
const V: usize = 2;
const W: usize = 3;

fn pivot(k: usize) -> Option<Pivot> {
    match k {
        0 => None,
        1 => Some(Pivot::Counted(PivotRule::Lowest, PivotCheck::default())),
        2 => Some(Pivot::Profile(ProfileCheck::default())),
        _ => Some(Pivot::Tails(TailsCheck::default())),
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
    /// Cells charged at the band (A: `rows · cols · (1 + fallback)`; V, W:
    /// stored cells, A's convention on a fallback call).
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
    /// Mean over calls that did not fall back.
    cells_clean_mean: f64,
    cells_clean_min: u64,
    cells_clean_max: u64,
    offset_ops_mean: f64,
    tail_ops_mean: f64,
    lead_scan_mean: f64,
    fallbacks: usize,
    /// Mean per-call `S(g) = C + 97 g (cells + offset ops + tail ops)`, g = 0,
    /// 0.1, 1.
    s_mean: [f64; 3],
    /// σ1: plus the lead-scan reads.
    s_sigma1_mean: [f64; 3],
}

#[derive(Serialize)]
struct Curve {
    p: u64,
    seed: u64,
    stream_salt: u64,
    calls: usize,
    /// Calls where solver k returned the existing solver's output.
    agree: Vec<usize>,
    /// Calls where W (resp. V) matched A counted on fallback flag, phases
    /// and digest.
    w_equals_a_counted: usize,
    v_equals_a_counted: usize,
    /// Calls where W's offset operations equal V's.
    w_offsets_equal_v: usize,
    /// Calls where W's counter equals final lengths + discarded buffers.
    stored_counter_ok: usize,
    /// Calls where W's lead scans are `≤` V's.
    lead_scan_le_v: usize,
    /// W's re-allocations (sum over calls) and the most on one call.
    w_reallocs: u64,
    w_reallocs_max: u64,
    /// Means over clean calls of Stage 0's replay: fill extent `G`, W's
    /// stored cells minus `G` (G2.242); dense-update cells V / `G` (G2.243).
    fill_extent_clean_mean: f64,
    w_slack_clean_mean: f64,
    axpy_cells_v_mean: f64,
    axpy_cells_g_mean: f64,
    summaries: Vec<Summary>,
    /// `S_V / S_W`, `S_A / S_W`, `S_A / S_V` of the means at g = 0, 0.1, 1
    /// (A = A counted), and `S_V / S_W` under σ1.
    ratio_v_over_w: [f64; 3],
    ratio_a_over_w: [f64; 3],
    ratio_a_over_v: [f64; 3],
    ratio_v_over_w_sigma1: [f64; 3],
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let targets: usize = arg(&args, "--targets", 1000);
    let threads: u64 = arg(&args, "--threads", 4);
    let json: String = arg(&args, "--json", String::new());
    let mut out = Vec::new();
    for p in [101u64, 149, 197, 269, 397] {
        for seed in [1u64, 2] {
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
            let mut agree = vec![0usize; NAMES.len()];
            let (mut w_eq, mut v_eq, mut off_eq, mut stored_ok, mut lead_ok) =
                (0usize, 0usize, 0usize, 0usize, 0usize);
            let (mut reallocs, mut reallocs_max) = (0u64, 0u64);
            let (mut fill_sum, mut slack_sum, mut clean_n) = (0f64, 0f64, 0usize);
            let (mut axv, mut axg, mut s0n) = (0f64, 0f64, 0usize);
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
                        let cells = if k == V || k == W {
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
                        if results[k] == results[0] {
                            agree[k] += 1;
                        }
                    }
                    let (sa, sv, sw) = (&sts[A_COUNTED], &sts[V], &sts[W]);
                    let same = |x: &QuarticSolve, fx: bool| {
                        fx == fells[A_COUNTED]
                            && phases(sa) == phases(x)
                            && sa.pivot_digest == x.pivot_digest
                    };
                    w_eq += usize::from(same(sw, fells[W]));
                    v_eq += usize::from(same(sv, fells[V]));
                    off_eq += usize::from(sw.profile_offset_ops == sv.profile_offset_ops);
                    lead_ok += usize::from(sw.lead_scan_reads <= sv.lead_scan_reads);
                    if let Some(m) = tails_measure(&comps, MACAULAY_BOUND, p, TailsCheck::default())
                    {
                        if m.counter == m.final_lengths + m.discarded {
                            stored_ok += 1;
                        }
                        reallocs += m.reallocs;
                        reallocs_max = reallocs_max.max(m.reallocs);
                    }
                    if let Some(z) = stage0_tails(&comps, MACAULAY_BOUND, p) {
                        axv += z.dense_update_cells_v as f64;
                        axg += z.dense_update_cells_g as f64;
                        s0n += 1;
                        if !fells[W] {
                            fill_sum += z.final_extent_cells as f64;
                            slack_sum +=
                                sw.macaulay_stored_entries as f64 - z.final_extent_cells as f64;
                            clean_n += 1;
                        }
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
            let (vw, aw, av, vw1) = (
                rt(V, W, false),
                rt(A_COUNTED, W, false),
                rt(A_COUNTED, V, false),
                rt(V, W, true),
            );
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree {agree:?}, W = A counted {w_eq}, V = A counted {v_eq}, W offsets = V {off_eq}, counter ok {stored_ok}, lead W<=V {lead_ok}, reallocs {reallocs} (max {reallocs_max})"
            );
            for s in &summaries {
                eprintln!(
                    "  {:>14}: C {:>9.0} | cells {:>9.0} (clean {:>9.0} [{}, {}]) | ops {:>7.0} | tail {:>6.0} | lead {:>7.0} | fb {:>2} | S {:.4e} {:.4e} {:.4e}",
                    s.solver, s.call_mean, s.cells_mean, s.cells_clean_mean, s.cells_clean_min,
                    s.cells_clean_max, s.offset_ops_mean, s.tail_ops_mean, s.lead_scan_mean,
                    s.fallbacks, s.s_mean[0], s.s_mean[1], s.s_mean[2]
                );
            }
            let fill = fill_sum / clean_n.max(1) as f64;
            let slack = slack_sum / clean_n.max(1) as f64;
            eprintln!(
                "  S_V/S_W: {:.6} {:.4} {:.4} | S_A/S_W {:.6} {:.4} {:.4} | sigma1 V/W {:.4} {:.4} | fill {:.0} slack {:.0} | axpy V {:.0} G {:.0}",
                vw[0], vw[1], vw[2], aw[0], aw[1], aw[2], vw1[1], vw1[2], fill, slack,
                axv / s0n.max(1) as f64, axg / s0n.max(1) as f64
            );
            out.push(Curve {
                p,
                seed,
                stream_salt: SALT,
                calls,
                agree,
                w_equals_a_counted: w_eq,
                v_equals_a_counted: v_eq,
                w_offsets_equal_v: off_eq,
                stored_counter_ok: stored_ok,
                lead_scan_le_v: lead_ok,
                w_reallocs: reallocs,
                w_reallocs_max: reallocs_max,
                fill_extent_clean_mean: fill,
                w_slack_clean_mean: slack,
                axpy_cells_v_mean: axv / s0n.max(1) as f64,
                axpy_cells_g_mean: axg / s0n.max(1) as f64,
                summaries,
                ratio_v_over_w: vw,
                ratio_a_over_w: aw,
                ratio_a_over_v: av,
                ratio_v_over_w_sigma1: vw1,
                arms,
            });
            if !json.is_empty() {
                std::fs::write(&json, serde_json::to_string(&out).unwrap()).unwrap();
            }
        }
    }
}
