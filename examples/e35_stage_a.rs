//! E35 Stage 0 and Stage A (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.40,
//! G2.144–G2.149): solver B's pivot schedule learned per curve and replayed
//! (`chain-sparsest-trace`, T), on E31/E33's Stage A calls exactly (E25's
//! ten curves, the first `--targets` targets of arm A's stream, every
//! solver from the same per-target RNG seed, one thread).
//!
//! Per curve: the trace is learned as the pipeline learns it
//! (`chain_trace::learn_gls`: three targets of the learning stream, B′'s
//! elimination, per-column mode), so the calls below are out of sample.
//!
//! Stage 0, per call (a diagnostic, nothing charged): B's pivot sequence
//! (B′'s elimination, `chain_trace::learn_one`; its digest is checked
//! against B counted's) against the curve's trace — shape, whole sequence,
//! per-column agreement over the call's contested columns — the number of
//! distinct sequences per curve, and every curve's sequences against every
//! other curve's trace.
//!
//! Stage A, per call: existing (E25), A counted, B counted, B″ and T;
//! outputs, phase counts, selection work, entries, lead scans; for T the
//! contested columns, misses, shape fallbacks and whether its pivots are B's.
//!
//! ```text
//! RAYON_NUM_THREADS=1 e35_stage_a --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 --json experiments/39_chain_trace/stage_a.json
//! ```

use std::collections::HashMap;

use crypto_lib::cryptanalysis::chain_trace::{
    last_replay, learn_gls, learn_one, reset_last_replay, PivotTrace, TraceLearning,
};
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, solve_weighted_traced, v4_system, Pivot,
    SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

const NAMES: [&str; 5] = [
    "existing",
    "chain-counted",
    "chain-sparsest-counted",
    "chain-sparsest-est",
    "chain-sparsest-trace",
];

fn pivot(k: usize, trace: &'static PivotTrace) -> Option<Pivot> {
    let co = |rule| Pivot::Counted(rule, PivotCheck::default());
    match k {
        0 => None,
        1 => Some(co(PivotRule::Lowest)),
        2 => Some(co(PivotRule::SparsestRecount)),
        3 => Some(co(PivotRule::SparsestEstimate)),
        _ => Some(Pivot::Trace(trace, false)),
    }
}

/// Per call, in stream order.
#[derive(Default, Serialize, Clone)]
struct Arm {
    solver: &'static str,
    /// `C`: Weil restriction + solve, `F_p` multiplications.
    call: Vec<u64>,
    echelon: Vec<u64>,
    /// Macaulay entries (`rows · cols`, doubled on a fallback, as E31).
    entries: Vec<u64>,
    /// Selection work `[initial/lookups, comparisons/tests, updates]`.
    sel: Vec<[u64; 3]>,
    lead_scan: Vec<u64>,
    fallbacks: u64,
    solved: u64,
}

#[derive(Serialize)]
struct Summary {
    solver: &'static str,
    call_median: u64,
    call_mean: f64,
    echelon_mean: f64,
    entries_mean: f64,
    sel_mean: [f64; 3],
    sel_total_mean: f64,
    lead_scan_mean: f64,
    /// Mean per-call `S(g) = C + 97 g (entries + selection)`, g = 0, 0.1, 1
    /// (T's learning not included; see `s_mean_learning`).
    s_mean: [f64; 3],
    /// The same with T's learning amortised over this curve's calls.
    s_mean_learning: [f64; 3],
    /// The same with lead-scan reads priced too (G2.153's sensitivity).
    s_mean_lead: [f64; 3],
    fallbacks: u64,
    solved: u64,
}

#[derive(Serialize, Default)]
struct Stage0 {
    calls: usize,
    /// Calls whose `rows × cols` equals the trace's.
    shape_equal: usize,
    /// Calls whose whole pivot-row sequence equals the trace's.
    sequence_equal: usize,
    /// Distinct pivot-row sequences among the calls, and the share of the
    /// most frequent one.
    distinct_sequences: usize,
    modal_sequence_calls: usize,
    modal_equals_trace: bool,
    /// Contested columns (B's) over all calls, and those where B's pivot
    /// row equals the trace's; and where row and position both equal.
    contested_columns: u64,
    contested_row_agree: u64,
    contested_choice_agree: u64,
    /// Columns with a pivot in either, and agreement over all of them.
    columns: u64,
    columns_row_agree: u64,
    /// Calls where B′'s digest (the instrument) equals B counted's.
    instrument_digest_equal_b: usize,
    /// Learned sequences equal to the trace (of 3).
    learning_sequences_equal_to_mode: u64,
}

#[derive(Serialize)]
struct Curve {
    p: u64,
    seed: u64,
    n: u64,
    calls: usize,
    /// Calls where solver k agreed with the existing one on solutions,
    /// degree, outcome and decomposition set (index as `solvers`).
    agree: Vec<usize>,
    /// Calls where T's pivot digest equals B counted's.
    trace_digest_equal_b: usize,
    /// T's diagnostics, summed over calls.
    trace_contested: u64,
    trace_misses: u64,
    trace_misses_shifted: u64,
    trace_shape_fallbacks: u64,
    /// Calls on which T missed at least once.
    trace_calls_with_miss: usize,
    learning: TraceLearning,
    stage0: Stage0,
    solvers: Vec<&'static str>,
    summaries: Vec<Summary>,
    /// Per call: T's misses (for the figures).
    trace_misses_per_call: Vec<u64>,
    arms: Vec<Arm>,
}

#[derive(Serialize)]
struct CrossCurve {
    /// `[from curve][trace of curve]`: fraction of `from`'s calls whose
    /// whole sequence equals that trace, and per-column row agreement over
    /// `from`'s contested columns (shape mismatch counts as disagreement).
    curves: Vec<(u64, u64)>,
    sequence_equal: Vec<Vec<f64>>,
    contested_agree: Vec<Vec<f64>>,
}

#[derive(Serialize)]
struct Out {
    curves: Vec<Curve>,
    cross: Option<CrossCurve>,
}

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

fn list(args: &[String], k: &str, d: &str) -> Vec<u64> {
    let s: String = arg(args, k, d.to_string());
    s.split(',').map(|x| x.trim().parse().unwrap()).collect()
}

fn mean_u(v: &[u64]) -> f64 {
    v.iter().sum::<u64>() as f64 / v.len().max(1) as f64
}

const G: [f64; 3] = [0.0, 0.1, 1.0];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sizes = list(&args, "--sizes", "101,149,197,269,397");
    let seeds = list(&args, "--seeds", "1,2");
    let targets: usize = arg(&args, "--targets", 1000);
    let threads: u64 = arg(&args, "--threads", 4);
    let json: String = arg(&args, "--json", String::new());
    let mut out = Out {
        curves: Vec::new(),
        cross: None,
    };
    // Stage 0 cross-curve material: each curve's trace and B's per-call
    // sequences with their contested flags.
    type Seq = (usize, usize, Vec<u32>, Vec<bool>);
    let mut traces: Vec<&'static PivotTrace> = Vec::new();
    let mut all_seqs: Vec<Vec<Seq>> = Vec::new();
    for &p in &sizes {
        for &seed in &seeds {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let (trace, learning) = learn_gls(&ki, &pre, seed).expect("learning");
            let trace: &'static PivotTrace = Box::leak(Box::new(trace));
            let trace_rows = trace.rows_only();
            let n = inst.n;
            let mut arms: Vec<Arm> = NAMES
                .iter()
                .map(|s| Arm {
                    solver: s,
                    ..Default::default()
                })
                .collect();
            let mut agree = vec![0usize; NAMES.len()];
            let mut s0 = Stage0 {
                learning_sequences_equal_to_mode: learning.sequences_equal_to_mode,
                ..Default::default()
            };
            let mut seq_count: HashMap<Vec<u32>, usize> = HashMap::new();
            let mut seqs: Vec<Seq> = Vec::new();
            let (mut t_dg, mut t_cont, mut t_miss, mut t_shift, mut t_shape, mut t_cwm) =
                (0usize, 0u64, 0u64, 0u64, 0u64, 0usize);
            let mut miss_per_call = Vec::new();
            let mut calls = 0usize;
            let mut residuals = 0u64;
            'outer: loop {
                let batch: Vec<(u64, u64, u64)> = (0..threads * 2)
                    .map(|k| {
                        (
                            rng.gen_range(0..n),
                            rng.gen_range(1..n),
                            mix64(seed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
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
                    let mut sts: Vec<QuarticSolve> = Vec::new();
                    let mut tw = None;
                    for k in 0..NAMES.len() {
                        let mut trng = StdRng::seed_from_u64(tseed);
                        let mut st = QuarticSolve {
                            weil_muls: weil,
                            ..Default::default()
                        };
                        reset_last_replay();
                        let (sols, deg, fell) = match pivot(k, trace) {
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
                        if k == 4 {
                            tw = Some(last_replay());
                        }
                        let lb =
                            lift_back_v4(&ki, &c, sols.as_deref().unwrap_or(&[]), &r, &mut trng);
                        let arm = &mut arms[k];
                        arm.call.push(st.total_muls);
                        arm.echelon.push(st.echelon_muls);
                        arm.entries
                            .push((st.rows * st.cols) as u64 * (1 + u64::from(fell)));
                        arm.sel.push([
                            st.pivot_init_ops,
                            st.pivot_compare_ops,
                            st.pivot_update_ops,
                        ]);
                        arm.lead_scan.push(st.lead_scan_reads);
                        arm.fallbacks += u64::from(fell);
                        arm.solved += u64::from(sols.is_some());
                        results.push((sols, deg, st.outcome.clone(), lb.decompositions));
                        sts.push(st);
                    }
                    for k in 0..NAMES.len() {
                        if results[k] == results[0] {
                            agree[k] += 1;
                        }
                    }
                    if sts[4].pivot_digest == sts[2].pivot_digest && sts[2].pivot_digest != 0 {
                        t_dg += 1;
                    }
                    let tw = tw.unwrap_or_default();
                    t_cont += tw.contested;
                    t_miss += tw.misses;
                    t_shift += tw.misses_shifted;
                    t_shape += u64::from(tw.shape_fallback);
                    t_cwm += usize::from(tw.misses > 0);
                    miss_per_call.push(tw.misses);
                    // Stage 0: B's sequence on this call (B′'s elimination).
                    if let Ok(l) = learn_one(&comps, p) {
                        s0.calls += 1;
                        if l.st.pivot_digest == sts[2].pivot_digest {
                            s0.instrument_digest_equal_b += 1;
                        }
                        let rows_seq: Vec<u32> = l.choice.iter().map(|c| c.0).collect();
                        if l.rows == trace.rows && l.cols == trace.cols {
                            s0.shape_equal += 1;
                            if rows_seq == trace_rows {
                                s0.sequence_equal += 1;
                            }
                            for col in 0..l.cols {
                                let (bc, tc) = (l.choice[col], trace.choice[col]);
                                if bc.0 != u32::MAX || tc.0 != u32::MAX {
                                    s0.columns += 1;
                                    s0.columns_row_agree += u64::from(bc.0 == tc.0);
                                }
                                if l.contested[col] {
                                    s0.contested_columns += 1;
                                    s0.contested_row_agree += u64::from(bc.0 == tc.0);
                                    s0.contested_choice_agree += u64::from(bc == tc);
                                }
                            }
                        } else {
                            s0.contested_columns +=
                                l.contested.iter().filter(|&&x| x).count() as u64;
                            s0.columns += rows_seq.iter().filter(|&&x| x != u32::MAX).count() as u64;
                        }
                        *seq_count.entry(rows_seq.clone()).or_insert(0) += 1;
                        seqs.push((l.rows, l.cols, rows_seq, l.contested));
                    }
                }
            }
            s0.distinct_sequences = seq_count.len();
            if let Some((modal, &k)) = seq_count.iter().max_by_key(|(s, &k)| (k, s.len())) {
                s0.modal_sequence_calls = k;
                s0.modal_equals_trace = *modal == trace_rows;
            }
            let ncalls = calls as f64;
            let summaries: Vec<Summary> = arms
                .iter()
                .enumerate()
                .map(|(k, a)| {
                    let sel: Vec<u64> = a.sel.iter().map(|s| s[0] + s[1] + s[2]).collect();
                    let s_of = |g: f64, extra: &dyn Fn(usize) -> f64| {
                        (0..a.call.len())
                            .map(|i| {
                                a.call[i] as f64
                                    + 97.0 * g * (a.entries[i] + sel[i]) as f64
                                    + extra(i) * g
                            })
                            .sum::<f64>()
                            / a.call.len() as f64
                    };
                    let s_mean = G.map(|g| s_of(g, &|_| 0.0));
                    let s_mean_learning = G.map(|g| {
                        let base = s_of(g, &|_| 0.0);
                        if k == 4 {
                            base + (learning.muls as f64
                                + 97.0 * g * (learning.entries + learning.selection_ops) as f64)
                                / ncalls
                        } else {
                            base
                        }
                    });
                    let s_mean_lead = G.map(|g| {
                        let ls = &a.lead_scan;
                        let base = s_of(g, &|i| 97.0 * ls[i] as f64);
                        if k == 4 {
                            base + (learning.muls as f64
                                + 97.0
                                    * g
                                    * (learning.entries
                                        + learning.selection_ops
                                        + learning.lead_scan_reads)
                                        as f64)
                                / ncalls
                        } else {
                            base
                        }
                    });
                    let mut sorted = a.call.clone();
                    sorted.sort_unstable();
                    Summary {
                        solver: a.solver,
                        call_median: sorted[sorted.len() / 2],
                        call_mean: mean_u(&a.call),
                        echelon_mean: mean_u(&a.echelon),
                        entries_mean: mean_u(&a.entries),
                        sel_mean: [0, 1, 2]
                            .map(|j| mean_u(&a.sel.iter().map(|s| s[j]).collect::<Vec<_>>())),
                        sel_total_mean: mean_u(&sel),
                        lead_scan_mean: mean_u(&a.lead_scan),
                        s_mean,
                        s_mean_learning,
                        s_mean_lead,
                        fallbacks: a.fallbacks,
                        solved: a.solved,
                    }
                })
                .collect();
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree {agree:?}; T: digest=B {t_dg}, contested {t_cont}, misses {t_miss} (shifted {t_shift}), calls with a miss {t_cwm}, shape fallbacks {t_shape}"
            );
            eprintln!(
                "  stage0: shape {}/{}, sequence = trace {}, distinct {}, modal {} (= trace {}), contested agree {}/{} rows ({} choices), all columns {}/{}, instrument = B {}, learned = mode {}/3",
                s0.shape_equal, s0.calls, s0.sequence_equal, s0.distinct_sequences,
                s0.modal_sequence_calls, s0.modal_equals_trace, s0.contested_row_agree,
                s0.contested_columns, s0.contested_choice_agree, s0.columns_row_agree, s0.columns,
                s0.instrument_digest_equal_b, s0.learning_sequences_equal_to_mode
            );
            eprintln!(
                "  learning: {} muls, {} entries, selection {:?} = {} (mode {}), drawn {}",
                learning.muls,
                learning.entries,
                learning.selection,
                learning.selection_ops,
                learning.mode_ops,
                learning.drawn
            );
            for s in &summaries {
                eprintln!(
                    "  {:>22}: C median {:>9} mean {:>10.0} | ech {:>8.0} | sel {:>9.0} | S(0,.1,1) {:.5e} {:.5e} {:.5e} | +learn {:.5e} {:.5e} {:.5e} | fb {}",
                    s.solver, s.call_median, s.call_mean, s.echelon_mean, s.sel_total_mean,
                    s.s_mean[0], s.s_mean[1], s.s_mean[2], s.s_mean_learning[0],
                    s.s_mean_learning[1], s.s_mean_learning[2], s.fallbacks
                );
            }
            traces.push(trace);
            all_seqs.push(seqs);
            out.curves.push(Curve {
                p,
                seed,
                n,
                calls,
                agree,
                trace_digest_equal_b: t_dg,
                trace_contested: t_cont,
                trace_misses: t_miss,
                trace_misses_shifted: t_shift,
                trace_shape_fallbacks: t_shape,
                trace_calls_with_miss: t_cwm,
                learning,
                stage0: s0,
                solvers: NAMES.to_vec(),
                summaries,
                trace_misses_per_call: miss_per_call,
                arms,
            });
            if !json.is_empty() {
                std::fs::write(&json, serde_json::to_string(&out).unwrap()).unwrap();
            }
        }
    }
    // Stage 0 across curves.
    let m = traces.len();
    let mut seq_eq = vec![vec![0.0; m]; m];
    let mut cont = vec![vec![0.0; m]; m];
    for (i, seqs) in all_seqs.iter().enumerate() {
        for (j, t) in traces.iter().enumerate() {
            let tr = t.rows_only();
            let (mut eq, mut agree, mut tot) = (0usize, 0u64, 0u64);
            for (rows, cols, s, con) in seqs {
                let shape = *rows == t.rows && *cols == t.cols;
                eq += usize::from(shape && *s == tr);
                for c in 0..*cols {
                    if con[c] {
                        tot += 1;
                        agree += u64::from(shape && s[c] == tr[c]);
                    }
                }
            }
            seq_eq[i][j] = eq as f64 / seqs.len().max(1) as f64;
            cont[i][j] = agree as f64 / tot.max(1) as f64;
        }
    }
    out.cross = Some(CrossCurve {
        curves: out.curves.iter().map(|c| (c.p, c.seed)).collect(),
        sequence_equal: seq_eq,
        contested_agree: cont,
    });
    if let Some(x) = &out.cross {
        eprintln!("cross-curve contested agreement (row: calls of curve i, column: trace of curve j):");
        for (i, r) in x.contested_agree.iter().enumerate() {
            eprintln!(
                "  {:?}: {}",
                x.curves[i],
                r.iter().map(|v| format!("{v:.3}")).collect::<Vec<_>>().join(" ")
            );
        }
    }
    if !json.is_empty() {
        std::fs::write(&json, serde_json::to_string(&out).unwrap()).unwrap();
    }
}
