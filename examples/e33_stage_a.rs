//! E33 Stage A (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.35,
//! G2.124–G2.127, G2.131, G2.133): solver B's pivot selection counted, and
//! two cheaper selections (B′ incremental, B″ estimate), on E31's Stage A
//! calls exactly (`examples/e31_stage_a.rs`: E25's ten curves, the first
//! `--targets` targets of arm A's stream, every solver from the same
//! per-target RNG seed).
//!
//! Solvers, per call: existing (E25), A, A counted, B, B counted, B′, B″.
//! Checks per call: every solver returns the existing solver's solutions,
//! degree, outcome and decomposition set; A counted reproduces A's phase
//! counts and B counted and B′ B's; B counted and B′ choose B's pivot in
//! every column (equal digests); B′'s maintained counts equal a recount
//! (diagnostic, not charged).
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./target/release/examples/e33_stage_a --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 --json experiments/37_solver_b_pivot_count/stage_a.json
//! ```

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

const NAMES: [&str; 7] = [
    "existing",
    "chain",
    "chain-counted",
    "chain-sparsest",
    "chain-sparsest-counted",
    "chain-sparsest-inc",
    "chain-sparsest-est",
];

fn pivot(k: usize) -> Option<Pivot> {
    let co = |rule: PivotRule| {
        Pivot::Counted(
            rule,
            PivotCheck {
                verify: rule == PivotRule::SparsestIncremental,
                ..Default::default()
            },
        )
    };
    match k {
        0 => None,
        1 => Some(Pivot::Uncounted { sparsest: false }),
        2 => Some(co(PivotRule::Lowest)),
        3 => Some(Pivot::Uncounted { sparsest: true }),
        4 => Some(co(PivotRule::SparsestRecount)),
        5 => Some(co(PivotRule::SparsestIncremental)),
        _ => Some(co(PivotRule::SparsestEstimate)),
    }
}

/// Per call, in stream order (compact: only what the analysis reads).
#[derive(Default, Serialize, Clone)]
struct Arm {
    solver: &'static str,
    /// `C`: Weil restriction + solve, `F_p` multiplications.
    call: Vec<u64>,
    echelon: Vec<u64>,
    /// Macaulay entries (`rows · cols`, doubled on a fallback, as E31).
    entries: Vec<u64>,
    /// Selection work `[initial, comparisons, updates]`.
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
    sel_init_mean: f64,
    sel_compare_mean: f64,
    sel_update_mean: f64,
    sel_mean: f64,
    lead_scan_mean: f64,
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
    /// Calls where solver k agreed with the existing one on solutions,
    /// degree, outcome and decomposition set (index as `solvers`).
    agree: Vec<usize>,
    /// Calls where the phase counts equal the reference (A counted vs A;
    /// B counted, B′ vs B): `[A counted, B counted, B′]`.
    phases_equal: [usize; 3],
    /// Calls where the pivot digest equals B's: `[B counted, B′, B″]`
    /// (B″ may differ: it is reported, not required).
    digest_equal_b: [usize; 3],
    /// B′ count mismatches against a recount, summed.
    inc_count_mismatches: u64,
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

fn mean_u(v: &[u64]) -> f64 {
    v.iter().sum::<u64>() as f64 / v.len().max(1) as f64
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
            let (mut ph, mut dg, mut mism) = ([0usize; 3], [0usize; 3], 0u64);
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
                    for (i, (x, base)) in [(2usize, 1usize), (4, 3), (5, 3)].iter().enumerate() {
                        if phases(&sts[*x]) == phases(&sts[*base]) {
                            ph[i] += 1;
                        }
                    }
                    for (i, x) in [4usize, 5, 6].iter().enumerate() {
                        if sts[*x].pivot_digest == sts[4].pivot_digest
                            && (i > 0 || sts[4].pivot_digest != 0)
                        {
                            dg[i] += 1;
                        }
                    }
                    mism += sts[5].pivot_count_mismatches;
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
                        call_mean: mean_u(&a.call),
                        echelon_mean: mean_u(&a.echelon),
                        entries_mean: mean_u(&a.entries),
                        sel_init_mean: mean_u(&a.sel.iter().map(|s| s[0]).collect::<Vec<_>>()),
                        sel_compare_mean: mean_u(&a.sel.iter().map(|s| s[1]).collect::<Vec<_>>()),
                        sel_update_mean: mean_u(&a.sel.iter().map(|s| s[2]).collect::<Vec<_>>()),
                        sel_mean: mean_u(&sel),
                        lead_scan_mean: mean_u(&a.lead_scan),
                        s_mean,
                        fallbacks: a.fallbacks,
                        solved: a.solved,
                    }
                })
                .collect();
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree {agree:?}, phases equal {ph:?}, digest = B {dg:?}, B′ mismatches {mism}"
            );
            for s in &summaries {
                eprintln!(
                    "  {:>22}: C median {:>9} mean {:>10.0} | ech {:>8.0} | sel init {:>9.0} cmp {:>6.0} upd {:>9.0} = {:>9.0} | lead {:>8.0} | S(0,.1,1) {:.4e} {:.4e} {:.4e} | fb {}",
                    s.solver, s.call_median, s.call_mean, s.echelon_mean, s.sel_init_mean,
                    s.sel_compare_mean, s.sel_update_mean, s.sel_mean, s.lead_scan_mean,
                    s.s_mean[0], s.s_mean[1], s.s_mean[2], s.fallbacks
                );
            }
            out.push(Curve {
                p,
                seed,
                n,
                calls,
                agree,
                phases_equal: ph,
                digest_equal_b: dg,
                inc_count_mismatches: mism,
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
