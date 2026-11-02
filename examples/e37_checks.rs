//! E37 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.44, G2.164–G2.165):
//! negative controls that corrupt state the solve reads, with a read probe
//! as their positive check, on E25's ten curves (32 calls each, a control
//! stream `StdRng(seed ^ 0xE37C_0001)` after E25's base and precompute).
//!
//! Per call and rule (F, S, H, L): the clean run (probe on) must return the
//! uncounted solver A's solutions, degree and decomposition set; F and L
//! charge no selection work; E31's perturbed coefficient must change every
//! non-empty solution set.  Controls, each aimed from the clean run's read
//! log of the same call:
//!
//! - C_SH (S, H): the inverse permutation maps the contents of `r₁` (the
//!   row of the first left-looking normal-form read) and `r₂` (the row of
//!   the last such read whose row and hash differ from `r₁`'s) to each other;
//! - C_F (F): at `c*`, the first contested column in the normal-form read
//!   log, the second arrival is filed by its next nonzero without its update;
//! - C_0 (F, S, H, L, calibration): one nonzero right of the pivot of the
//!   first normal-form-read row with such an entry moved by one.
//!
//! **Positive check:** the control run's read log first differs from the
//! clean run's at an entry of the same phase and column with a different row
//! hash.  **Detected:** solutions, degree, decomposition set or the fallback
//! flag differ from the clean run's.  Exits non-zero only when a clean run
//! differs from the reference (an identity failure); thresholds are judged
//! by `e37_tables`.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e37_checks --json experiments/44_pivot_rules_confirm/checks.json
//! ```

use std::collections::{HashMap, HashSet};

use crypto_lib::cryptanalysis::chain_pivot_rules::{
    probe_begin, probe_end, take_diag, ArrivalRule, ReadProbe, RuleCheck,
};
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use serde_json::json;

const SALT: u64 = 0xE37C_0001;
const CALLS_PER_CURVE: usize = 32;
const CURVES: [(u64, u64); 10] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
];
const RULES: [(ArrivalRule, &str); 4] = [
    (ArrivalRule::Fifo, "F"),
    (ArrivalRule::Static, "S"),
    (ArrivalRule::FifoStatic, "H"),
    (ArrivalRule::Lifo, "L"),
];

/// `(solutions, degree, decomposition set, fell back, probe, state)`.
type Run = (
    Option<Vec<[u64; 4]>>,
    u16,
    Vec<[u64; 4]>,
    bool,
    ReadProbe,
    QuarticSolve,
);

/// One control's tallies.
#[derive(Default, Serialize, Clone)]
struct Tally {
    calls: usize,
    /// Calls where the control could be aimed (a target row or column found).
    aimed: usize,
    /// Calls where the corruption moved different contents (diagnostic).
    effective: usize,
    /// Positive check: first log difference at the same phase and column,
    /// different row hash.
    positive: usize,
    /// First log difference elsewhere (another column or phase, or one log
    /// a prefix of the other): structure changed, no same-column read.
    structural: usize,
    /// Logs identical.
    unread: usize,
    /// Detected (any call), and detected with a positive check.
    detected: usize,
    detected_positive: usize,
    /// Calls with a non-empty clean solution set, and those detected with a
    /// positive check.
    nonempty: usize,
    nonempty_detected_positive: usize,
    /// Detections that came through the fallback flag only.
    via_fallback: usize,
}

fn log_key(p: &ReadProbe) -> Vec<(u8, u32, u64)> {
    p.reads.iter().map(|r| (r.0, r.1, r.3)).collect()
}

/// 0 = positive (same phase/column, other hash), 1 = structural, 2 = equal.
fn first_difference(clean: &ReadProbe, bad: &ReadProbe) -> u8 {
    let (a, b) = (log_key(clean), log_key(bad));
    match a.iter().zip(&b).position(|(x, y)| x != y) {
        Some(k) => {
            if (a[k].0, a[k].1) == (b[k].0, b[k].1) {
                0
            } else {
                1
            }
        }
        None if a.len() != b.len() => 1,
        None => 2,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_path = args
        .iter()
        .position(|a| a == "--json")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let mut fails: Vec<String> = Vec::new();
    let mut check = |ok: bool, what: String| {
        if !ok {
            eprintln!("FAIL: {what}");
            fails.push(what);
        }
    };
    let mut t_sh = [Tally::default(), Tally::default()]; // S, H
    let mut t_f = Tally::default();
    let mut t_0 = vec![Tally::default(); RULES.len()];
    let mut perturb = [0usize; 4];
    let mut digest_differs = [0usize; 4];
    let (mut calls, mut nonempty, mut contested_calls) = (0usize, 0usize, 0usize);
    let mut per_curve = Vec::new();
    for (p, seed) in CURVES {
        let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
        let curve = ki.inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
        let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
        let pre = SymmetrisedV4::precompute(&ki, &curve, &mut rng, 8).expect("precompute");
        let mut rng = StdRng::seed_from_u64(seed ^ SALT);
        let sseed = seed ^ SALT;
        let n = ki.inst.n;
        let (mut done, mut residuals) = (0usize, 0u64);
        let calls0 = calls;
        while done < CALLS_PER_CURVE {
            let (a, b) = (rng.gen_range(0..n), rng.gen_range(1..n));
            let tseed = mix64(sseed ^ residuals.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            residuals += 1;
            let r = curve.add(&curve.mul(&ki.inst.g, a), &curve.mul(&ki.inst.q, b));
            let Some(z_r) = ki.z_of_x(f, &r.x) else {
                continue;
            };
            done += 1;
            let mut fp = 0u64;
            let comps = v4_system(&pre, f, &z_r, &mut fp);
            // E31's control: one coefficient of the first equation moved by one.
            let mut bent: Vec<HashMap<[u8; 4], u64>> = comps.clone();
            if let Some(eq) = bent.first_mut() {
                let mut keys: Vec<[u8; 4]> = eq.keys().copied().collect();
                keys.sort_unstable();
                if let Some(k) = keys.first() {
                    let v = eq[k];
                    eq.insert(*k, (v + 1) % p);
                }
            }
            let run = |cs: &[HashMap<[u8; 4], u64>], pivot: Pivot| -> Run {
                let mut tr = StdRng::seed_from_u64(tseed);
                let mut st = QuarticSolve::default();
                probe_begin();
                let (s, d, info) =
                    solve_weighted_chain_with(cs, MACAULAY_BOUND, p, &mut tr, &mut st, pivot);
                let pb = probe_end();
                let lb = lift_back_v4(&ki, &curve, s.as_deref().unwrap_or(&[]), &r, &mut tr);
                (s, d, lb.decompositions, info.fell_back, pb, st)
            };
            let same = |x: &Run, y: &Run| (&x.0, x.1, &x.2) == (&y.0, y.1, &y.2);
            let at = format!("p = {p} seed {seed} call {calls}");
            let ua = run(&comps, Pivot::Uncounted { sparsest: false });
            let ac = run(&comps, Pivot::Counted(PivotRule::Lowest, PivotCheck::default()));
            let has_sols = ua.0.as_ref().is_some_and(|s| !s.is_empty());
            nonempty += usize::from(has_sols);
            let mut clean: Vec<Run> = Vec::new();
            for (i, (rule, name)) in RULES.iter().enumerate() {
                let x = run(&comps, Pivot::Rule(*rule, RuleCheck::default()));
                check(same(&x, &ua), format!("{at}: {name} output"));
                if matches!(rule, ArrivalRule::Fifo | ArrivalRule::Lifo) {
                    check(
                        x.5.pivot_init_ops + x.5.pivot_compare_ops + x.5.pivot_update_ops == 0,
                        format!("{at}: {name} charged selection work"),
                    );
                }
                if i == 0 && !x.4.contested.is_empty() {
                    contested_calls += 1;
                }
                if x.5.pivot_digest != ac.5.pivot_digest {
                    digest_differs[i] += 1;
                }
                if has_sols {
                    let y = run(&bent, Pivot::Rule(*rule, RuleCheck::default()));
                    if y.0 != x.0 {
                        perturb[i] += 1;
                    }
                }
                clean.push(x);
            }
            // Judge one control run against the clean run of its rule.
            let judge = |t: &mut Tally, clean: &Run, bad: &Run, aimed: bool, effective: u64| {
                t.calls += 1;
                t.aimed += usize::from(aimed);
                t.effective += usize::from(effective > 0);
                let kind = if aimed { first_difference(&clean.4, &bad.4) } else { 2 };
                match kind {
                    0 => t.positive += 1,
                    1 => t.structural += 1,
                    _ => t.unread += 1,
                }
                let det = !same(clean, bad) || clean.3 != bad.3;
                t.detected += usize::from(det);
                if det && kind == 0 {
                    t.detected_positive += 1;
                    if same(clean, bad) {
                        t.via_fallback += 1;
                    }
                }
                let ne = clean.0.as_ref().is_some_and(|s| !s.is_empty());
                t.nonempty += usize::from(ne);
                t.nonempty_detected_positive += usize::from(ne && det && kind == 0);
            };
            // C_0 on every rule.
            for (i, (rule, _)) in RULES.iter().enumerate() {
                let c = &clean[i];
                let target = c.4.reads.iter().find(|r| r.0 == 1 && r.4 > 0).map(|r| r.2);
                let _ = take_diag();
                let bad = run(
                    &comps,
                    Pivot::Rule(
                        *rule,
                        RuleCheck {
                            bump_row: target,
                            ..Default::default()
                        },
                    ),
                );
                let eff = take_diag().corrupt_changed;
                judge(&mut t_0[i], c, &bad, target.is_some(), eff);
            }
            // C_SH on S and H.
            for (j, k) in [1usize, 2].into_iter().enumerate() {
                let c = &clean[k];
                let nf: Vec<_> = c.4.reads.iter().filter(|r| r.0 == 1).collect();
                let pair = nf.first().and_then(|r1| {
                    nf.iter()
                        .rev()
                        .find(|r| r.2 != r1.2 && r.3 != r1.3)
                        .map(|r2| (r1.2, r2.2))
                });
                let _ = take_diag();
                let bad = run(
                    &comps,
                    Pivot::Rule(
                        RULES[k].0,
                        RuleCheck {
                            swap_rows: pair,
                            ..Default::default()
                        },
                    ),
                );
                let eff = take_diag().corrupt_changed;
                judge(&mut t_sh[j], c, &bad, pair.is_some(), eff);
            }
            // C_F on F.
            {
                let c = &clean[0];
                let contested: HashSet<u32> = c.4.contested.iter().copied().collect();
                let cstar = c
                    .4
                    .reads
                    .iter()
                    .find(|r| r.0 == 1 && contested.contains(&r.1))
                    .map(|r| r.1);
                let _ = take_diag();
                let bad = run(
                    &comps,
                    Pivot::Rule(
                        ArrivalRule::Fifo,
                        RuleCheck {
                            misfile_at: cstar,
                            ..Default::default()
                        },
                    ),
                );
                let eff = take_diag().corrupt_changed;
                judge(&mut t_f, c, &bad, cstar.is_some(), eff);
            }
            calls += 1;
        }
        per_curve.push(json!({"p": p, "seed": seed, "calls": calls - calls0, "residuals_drawn": residuals}));
        eprintln!(
            "p={p} seed={seed}: {} calls; so far positive/detected C_SH(S) {}/{}, C_SH(H) {}/{}, C_F {}/{}",
            calls - calls0,
            t_sh[0].positive,
            t_sh[0].detected_positive,
            t_sh[1].positive,
            t_sh[1].detected_positive,
            t_f.positive,
            t_f.detected_positive
        );
    }
    check(calls == CURVES.len() * CALLS_PER_CURVE, format!("calls {calls}"));
    for (i, (_, name)) in RULES.iter().enumerate() {
        check(
            perturb[i] == nonempty,
            format!("{name}: perturbed coefficient detected {}/{nonempty}", perturb[i]),
        );
    }
    let out = json!({
        "experiment": "e37",
        "what": "E37 controls (§9.44, G2.165): read-probe positive checks and detection, 32 calls on each of E25's ten curves, control stream salt 0xE37C0001",
        "calls": calls,
        "nonempty": nonempty,
        "contested_calls_f": contested_calls,
        "curves": per_curve,
        "perturbed_detected": {"F": perturb[0], "S": perturb[1], "H": perturb[2], "L": perturb[3]},
        "digest_differs_from_a": {"F": digest_differs[0], "S": digest_differs[1], "H": digest_differs[2], "L": digest_differs[3]},
        "c_sh": {"S": t_sh[0], "H": t_sh[1]},
        "c_f": {"F": t_f},
        "c_0": {"F": t_0[0], "S": t_0[1], "H": t_0[2], "L": t_0[3]},
        "identity_failures": fails.len(),
    });
    let s = serde_json::to_string_pretty(&out).unwrap();
    println!("{s}");
    if let Some(path) = json_path {
        std::fs::write(path, &s).unwrap();
    }
    if !fails.is_empty() {
        std::process::exit(1);
    }
}
