//! E36 Stage A (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.41,
//! G2.154–G2.161): pivot rules whose selection work fits the §5a band, on
//! E31/E33's Stage A calls exactly (E25's ten curves, the first `--targets`
//! targets of arm A's stream, every solver from the same per-target seed).
//!
//! Solvers, per call: existing (E25), A, A counted, B″ (E33), A's rule
//! through E36's echelon (positive control), F, S, H, K, L (negative
//! control).  Per call: `C`, echelon multiplications, entries, selection
//! work by part, lead-scan reads, pivot digest and the uncharged σ2/σ3
//! tallies.  Checks: every solver returns the existing solver's solutions,
//! degree, outcome and decomposition set; the positive control reproduces
//! A counted's phase counts and digest.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e36_stage_a --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 --json experiments/40_pivot_rules/stage_a.json
//! ```

use crypto_lib::cryptanalysis::chain_pivot_rules::{take_diag, ArrivalRule, RuleCheck};
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

const NAMES: [&str; 10] = [
    "existing",
    "chain",
    "chain-counted",
    "chain-sparsest-est",
    "chain-lowest-e36",
    "chain-fifo",
    "chain-static",
    "chain-fifo-static",
    "chain-est-k2",
    "chain-lifo",
];
const A_COUNTED: usize = 2;
const LOWEST: usize = 4;

fn pivot(k: usize) -> Option<Pivot> {
    let co = |rule| Pivot::Counted(rule, PivotCheck::default());
    let ru = |r| Pivot::Rule(r, RuleCheck::default());
    match k {
        0 => None,
        1 => Some(Pivot::Uncounted { sparsest: false }),
        2 => Some(co(PivotRule::Lowest)),
        3 => Some(co(PivotRule::SparsestEstimate)),
        4 => Some(ru(ArrivalRule::Lowest)),
        5 => Some(ru(ArrivalRule::Fifo)),
        6 => Some(ru(ArrivalRule::Static)),
        7 => Some(ru(ArrivalRule::FifoStatic)),
        8 => Some(ru(ArrivalRule::EstimateK2)),
        _ => Some(ru(ArrivalRule::Lifo)),
    }
}

/// Per call, in stream order.
#[derive(Default, Serialize, Clone)]
struct Arm {
    solver: &'static str,
    /// `C`: Weil restriction + solve, `F_p` multiplications.
    call: Vec<u64>,
    echelon: Vec<u64>,
    /// Macaulay entries (`rows · cols`, doubled on a fallback, as E31/E33).
    entries: Vec<u64>,
    /// Selection work `[initial, comparisons, updates]`.
    sel: Vec<[u64; 3]>,
    lead_scan: Vec<u64>,
    digest: Vec<u64>,
    /// Uncharged tallies `[contested, ordered candidates, row updates]`
    /// (E36 rules only; zero for the others).
    diag: Vec<[u64; 3]>,
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
    sel_init_mean: f64,
    sel_compare_mean: f64,
    sel_update_mean: f64,
    sel_mean: f64,
    lead_scan_mean: f64,
    contested_mean: f64,
    ordered_candidates_mean: f64,
    row_updates_mean: f64,
    /// Mean per-call `S(g) = C + 97 g (entries + selection)` at g = 0, 0.1, 1.
    s_mean: [f64; 3],
    fallbacks: u64,
    solved: u64,
}

#[derive(Serialize)]
struct Curve {
    p: u64,
    seed: u64,
    n: u64,
    calls: usize,
    /// Calls where solver k agreed with the existing one (index as `solvers`).
    agree: Vec<usize>,
    /// Calls where the positive control's phase counts and digest equal A
    /// counted's.
    lowest_equals_a: usize,
    /// Calls where solver k's digest differs from A counted's, and calls with
    /// a contested column (from the positive control's tally).
    digest_differs_from_a: Vec<usize>,
    calls_contested: usize,
    solvers: Vec<&'static str>,
    summaries: Vec<Summary>,
    arms: Vec<Arm>,
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

fn mean_u(v: impl Iterator<Item = u64>) -> f64 {
    let (s, n) = v.fold((0u64, 0u64), |(s, n), x| (s + x, n + 1));
    s as f64 / n.max(1) as f64
}

fn phases(s: &QuarticSolve) -> [u64; 10] {
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
        (s.rows * s.cols) as u64,
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sizes = list(&args, "--sizes", "101,149,197,269,397");
    let seeds = list(&args, "--seeds", "1,2");
    let targets: usize = arg(&args, "--targets", 1000);
    let threads: u64 = arg(&args, "--threads", 4);
    let json: String = arg(&args, "--json", String::new());
    let mut out = Vec::new();
    for &p in &sizes {
        for &seed in &seeds {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let n = inst.n;
            let mut arms: Vec<Arm> = NAMES
                .iter()
                .map(|s| Arm {
                    solver: s,
                    ..Default::default()
                })
                .collect();
            let mut agree = vec![0usize; NAMES.len()];
            let mut differs = vec![0usize; NAMES.len()];
            let (mut low_eq, mut contested_calls) = (0usize, 0usize);
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
                    let mut sts = Vec::new();
                    for k in 0..NAMES.len() {
                        let mut trng = StdRng::seed_from_u64(tseed);
                        let mut st = QuarticSolve {
                            weil_muls: weil,
                            ..Default::default()
                        };
                        let _ = take_diag();
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
                        let dg = take_diag();
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
                        arm.digest.push(st.pivot_digest);
                        arm.diag
                            .push([dg.contested, dg.ordered_candidates, dg.row_updates]);
                        arm.fallbacks += u64::from(fell);
                        arm.solved += u64::from(sols.is_some());
                        results.push((sols, deg, st.outcome.clone(), lb.decompositions));
                        sts.push(st);
                    }
                    for k in 0..NAMES.len() {
                        if results[k] == results[0] {
                            agree[k] += 1;
                        }
                        if sts[k].pivot_digest != sts[A_COUNTED].pivot_digest {
                            differs[k] += 1;
                        }
                    }
                    if phases(&sts[LOWEST]) == phases(&sts[A_COUNTED])
                        && sts[LOWEST].pivot_digest == sts[A_COUNTED].pivot_digest
                        && sts[LOWEST].lead_scan_reads == sts[A_COUNTED].lead_scan_reads
                    {
                        low_eq += 1;
                    }
                    if arms[LOWEST].diag.last().is_some_and(|d| d[0] > 0) {
                        contested_calls += 1;
                    }
                }
            }
            let summaries: Vec<Summary> = arms
                .iter()
                .map(|a| {
                    let sel: Vec<u64> = a.sel.iter().map(|s| s[0] + s[1] + s[2]).collect();
                    let s_mean = [0.0, 0.1, 1.0].map(|g| {
                        a.call
                            .iter()
                            .zip(&a.entries)
                            .zip(&sel)
                            .map(|((&c, &e), &s)| c as f64 + 97.0 * g * (e + s) as f64)
                            .sum::<f64>()
                            / a.call.len() as f64
                    });
                    let mut sorted = a.call.clone();
                    sorted.sort_unstable();
                    Summary {
                        solver: a.solver,
                        call_median: sorted[sorted.len() / 2],
                        call_mean: mean_u(a.call.iter().copied()),
                        echelon_mean: mean_u(a.echelon.iter().copied()),
                        entries_mean: mean_u(a.entries.iter().copied()),
                        sel_init_mean: mean_u(a.sel.iter().map(|s| s[0])),
                        sel_compare_mean: mean_u(a.sel.iter().map(|s| s[1])),
                        sel_update_mean: mean_u(a.sel.iter().map(|s| s[2])),
                        sel_mean: mean_u(sel.iter().copied()),
                        lead_scan_mean: mean_u(a.lead_scan.iter().copied()),
                        contested_mean: mean_u(a.diag.iter().map(|d| d[0])),
                        ordered_candidates_mean: mean_u(a.diag.iter().map(|d| d[1])),
                        row_updates_mean: mean_u(a.diag.iter().map(|d| d[2])),
                        s_mean,
                        fallbacks: a.fallbacks,
                        solved: a.solved,
                    }
                })
                .collect();
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree {agree:?}, lowest = A {low_eq}, digest ≠ A {differs:?} (contested calls {contested_calls})"
            );
            for s in &summaries {
                eprintln!(
                    "  {:>18}: C median {:>8} mean {:>9.0} | ech {:>7.0} | sel {:>5.0} {:>5.0} {:>6.0} = {:>6.0} | lead {:>6.0} | cont {:>4.0} ord {:>5.0} upd {:>5.0} | S {:.4e} {:.4e} {:.4e} | fb {}",
                    s.solver, s.call_median, s.call_mean, s.echelon_mean, s.sel_init_mean,
                    s.sel_compare_mean, s.sel_update_mean, s.sel_mean, s.lead_scan_mean,
                    s.contested_mean, s.ordered_candidates_mean, s.row_updates_mean,
                    s.s_mean[0], s.s_mean[1], s.s_mean[2], s.fallbacks
                );
            }
            out.push(Curve {
                p,
                seed,
                n,
                calls,
                agree,
                lowest_equals_a: low_eq,
                digest_differs_from_a: differs,
                calls_contested: contested_calls,
                solvers: NAMES.to_vec(),
                summaries,
                arms,
            });
            if !json.is_empty() {
                std::fs::write(&json, serde_json::to_string(&out).unwrap()).unwrap();
            }
        }
    }
}
