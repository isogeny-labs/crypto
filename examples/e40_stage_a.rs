//! **E40 Stage A** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.50,
//! G2.204, G2.206–G2.209, G2.215): V (`chain-profile`, rows stored from
//! their lead) against A counted on fresh calls — E25's ten curves, base and
//! precompute from E25's stream, then the first `--targets` targets of stream
//! `seed ^ 0xE40A_0001` (per-target seeds as E37's Stage A), one thread;
//! per call existing (E25), A, A counted and V from the same per-target seed.
//!
//! Identity per call (G2.204): every solver returns the existing solver's
//! solutions, degree, outcome and decomposition set; V returns A counted's
//! fallback flag, every phase count and pivot digest; V's stored-cell
//! counter equals the measured lengths of its rows; V's lead-scan reads equal
//! A counted's minus `Σ_r offset_r`.  Cells: A `rows · cols · (1 + fallback)`
//! (E31); V its stored cells, or A's convention on a fallback call.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e40_stage_a --targets 1000 --json experiments/47_macaulay_entries/stage_a.json
//! ```

use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, solve_weighted_traced, v4_system, Pivot,
    SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::macaulay_profile::{profile_measure, ProfileCheck};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

const SALT: u64 = 0xE40A_0001;
const NAMES: [&str; 4] = ["existing", "chain", "chain-counted", "chain-profile"];
const A_COUNTED: usize = 2;
const V: usize = 3;

fn pivot(k: usize) -> Option<Pivot> {
    match k {
        0 => None,
        1 => Some(Pivot::Uncounted { sparsest: false }),
        2 => Some(Pivot::Counted(PivotRule::Lowest, PivotCheck::default())),
        _ => Some(Pivot::Profile(ProfileCheck::default())),
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
    /// Cells charged at the band (A: `rows · cols · (1 + fallback)`; V: stored
    /// cells, A's convention on a fallback call).
    cells: Vec<u64>,
    offset_ops: Vec<u64>,
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
    lead_scan_mean: f64,
    fallbacks: usize,
    /// Mean per-call `S(g) = C + 97 g (cells + offset ops)`, g = 0, 0.1, 1.
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
    /// Calls where V matched A counted on fallback flag, phases and digest.
    v_equals_a_counted: usize,
    /// Calls where V's stored counter equals its measured row lengths.
    stored_counter_ok: usize,
    /// Calls where V's lead scans equal A counted's minus `Σ offsets`.
    lead_scan_relation_ok: usize,
    /// A counted's cells on clean calls are all `176,064`.
    a_clean_cells_all_176064: bool,
    summaries: Vec<Summary>,
    /// `S_A / S_V` of the means at g = 0, 0.1, 1 (A = A counted), and under σ1.
    ratio_a_over_v: [f64; 3],
    ratio_a_over_v_sigma1: [f64; 3],
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
            let (mut v_eq, mut stored_ok, mut lead_ok) = (0usize, 0usize, 0usize);
            let mut a_clean_ok = true;
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
                        let cells = if k == V {
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
                    let (sa, sv) = (&sts[A_COUNTED], &sts[V]);
                    if fells[V] == fells[A_COUNTED]
                        && phases(sa) == phases(sv)
                        && sa.pivot_digest == sv.pivot_digest
                    {
                        v_eq += 1;
                    }
                    if !fells[A_COUNTED] && (sa.rows * sa.cols) as u64 != 176_064 {
                        a_clean_ok = false;
                    }
                    if let Some(m) =
                        profile_measure(&comps, MACAULAY_BOUND, p, ProfileCheck::default())
                    {
                        if m.counter == m.measured {
                            stored_ok += 1;
                        }
                        if sv.lead_scan_reads + m.offsets == sa.lead_scan_reads {
                            lead_ok += 1;
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
                                    + 97.0 * g * (a.cells[i] + a.offset_ops[i] + extra) as f64
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
                        lead_scan_mean: mean_u(a.lead_scan.iter().copied()),
                        fallbacks: a.fallback.iter().filter(|&&f| f).count(),
                        s_mean: [0.0, 0.1, 1.0].map(|g| s(g, false)),
                        s_sigma1_mean: [0.0, 0.1, 1.0].map(|g| s(g, true)),
                    }
                })
                .collect();
            let ratio: [f64; 3] =
                core::array::from_fn(|g| summaries[A_COUNTED].s_mean[g] / summaries[V].s_mean[g]);
            let ratio1: [f64; 3] = core::array::from_fn(|g| {
                summaries[A_COUNTED].s_sigma1_mean[g] / summaries[V].s_sigma1_mean[g]
            });
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree {agree:?}, V = A counted {v_eq}, stored ok {stored_ok}, lead relation {lead_ok}, A clean 176064 {a_clean_ok}"
            );
            for s in &summaries {
                eprintln!(
                    "  {:>14}: C {:>9.0} | cells {:>9.0} (clean {:>9.0} [{}, {}]) | ops {:>7.0} | lead {:>7.0} | fb {:>2} | S {:.4e} {:.4e} {:.4e}",
                    s.solver, s.call_mean, s.cells_mean, s.cells_clean_mean, s.cells_clean_min,
                    s.cells_clean_max, s.offset_ops_mean, s.lead_scan_mean, s.fallbacks,
                    s.s_mean[0], s.s_mean[1], s.s_mean[2]
                );
            }
            eprintln!(
                "  S_A/S_V: {:.6} {:.4} {:.4} | sigma1 {:.4} {:.4} {:.4}",
                ratio[0], ratio[1], ratio[2], ratio1[0], ratio1[1], ratio1[2]
            );
            out.push(Curve {
                p,
                seed,
                stream_salt: SALT,
                calls,
                agree,
                v_equals_a_counted: v_eq,
                stored_counter_ok: stored_ok,
                lead_scan_relation_ok: lead_ok,
                a_clean_cells_all_176064: a_clean_ok,
                summaries,
                ratio_a_over_v: ratio,
                ratio_a_over_v_sigma1: ratio1,
                arms,
            });
            if !json.is_empty() {
                std::fs::write(&json, serde_json::to_string(&out).unwrap()).unwrap();
            }
        }
    }
}
