//! E38 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.46, G2.176–G2.179):
//! the pilot and the main control run.  E37's `e37_checks.rs` with the F
//! controls of §9.46 and their positive checks; the clean runs, E31's
//! perturbed coefficient, the digest check, C_SH and C_0 are E37's.
//!
//! Calls: E25's ten curves, base and precompute from E25's stream, then the
//! first `--calls` targets per curve of the stream `StdRng(seed ^ salt)`
//! (pilot P1 `E38D0001`, fallback pilot P2 `E38D0101`, main `E38C0001`).
//!
//! Controls (each aimed from the clean run's probe of the same call):
//! - C_F (i), judged by `P_ext` (strict, or pivot set or border targets
//!   changed): E37's misfile at `c*` = the first contested column of the
//!   phase-1 read log (fallback `cf`: the lower median of the distinct
//!   contested phase-1 read columns, ascending);
//! - C_F^mid (ii), judged by `P_mid` (strict and pivot set and border targets
//!   unchanged): the last-column entry of `r_{l*}` moved by one right after
//!   its update at `c_m` (fallback `cfmid`: the pivot row at `l*`, before its
//!   normalisation);
//! - C_SH (S, H; strict): E37's, fallback `csh`: `r₂` the second distinct read;
//! - C_0 (F, S, H, L; strict; calibration): E37's.
//!
//! **Detected:** solutions, degree, decomposition set or the fallback flag
//! differ from the clean run's; a call without its registered positive check
//! counts as not detected.  Exits non-zero only on a clean-run identity
//! failure; thresholds are judged by `e38_tables`.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e38_checks --salt E38D0001 --calls 8 --json experiments/45_pivot_rules_confirm2/pilot.json
//! RAYON_NUM_THREADS=1 ./e38_checks --salt E38C0001 --calls 32 [--fallback cf,cfmid,csh] --json .../checks.json
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Check {
    Strict,
    Ext,
    Mid,
}

/// One control's tallies.
#[derive(Default, Serialize, Clone)]
struct Tally {
    check: &'static str,
    calls: usize,
    aimed: usize,
    /// The corruption moved different contents (diagnostic).
    effective: usize,
    /// The registered positive check held.
    positive: usize,
    /// Components: E37's strict check; pivot set changed; border targets
    /// changed; both unchanged (structure intact).
    strict: usize,
    pivots_changed: usize,
    targets_changed: usize,
    structure_intact: usize,
    /// Logs identical.
    unread: usize,
    /// Raw detection (any call) and detection with the registered check.
    detected_raw: usize,
    detected_positive: usize,
    nonempty: usize,
    nonempty_detected_positive: usize,
    via_fallback: usize,
    /// Aimed calls where the registered check failed (call labels).
    failed_aimed: Vec<String>,
}

fn log_key(p: &ReadProbe) -> Vec<(u8, u32, u64)> {
    p.reads.iter().map(|r| (r.0, r.1, r.3)).collect()
}

/// 0 = strict positive (same phase/column, other hash), 1 = other, 2 = equal.
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

fn parse_args() -> (u64, usize, HashSet<String>, Option<String>) {
    let args: Vec<String> = std::env::args().collect();
    let val = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let salt = u64::from_str_radix(&val("--salt").expect("--salt <hex>"), 16).expect("hex salt");
    let calls = val("--calls").map_or(32, |v| v.parse().expect("--calls"));
    let fb: HashSet<String> = val("--fallback")
        .map(|v| {
            v.split(',')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();
    (salt, calls, fb, val("--json"))
}

fn lower_median<T: Copy>(v: &[T]) -> Option<T> {
    (!v.is_empty()).then(|| v[(v.len() - 1) / 2])
}

fn main() {
    let (salt, per_curve_calls, fallback, json_path) = parse_args();
    for f in &fallback {
        assert!(
            ["cf", "cfmid", "csh"].contains(&f.as_str()),
            "unknown fallback {f}"
        );
    }
    let mut fails: Vec<String> = Vec::new();
    let mut check = |ok: bool, what: String| {
        if !ok {
            eprintln!("FAIL: {what}");
            fails.push(what);
        }
    };
    let mk = |c: Check| Tally {
        check: match c {
            Check::Strict => "strict",
            Check::Ext => "P_ext",
            Check::Mid => "P_mid",
        },
        ..Default::default()
    };
    let mut t_sh = [mk(Check::Strict), mk(Check::Strict)];
    let mut t_f = mk(Check::Ext);
    let mut t_mid = mk(Check::Mid);
    let mut t_0 = vec![mk(Check::Strict); RULES.len()];
    let mut perturb = [0usize; 4];
    let mut digest_differs = [0usize; 4];
    let (mut calls, mut nonempty, mut contested_calls) = (0usize, 0usize, 0usize);
    let mut per_curve = Vec::new();
    let mut aims: Vec<serde_json::Value> = Vec::new();
    for (p, seed) in CURVES {
        let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
        let curve = ki.inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
        let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
        let pre = SymmetrisedV4::precompute(&ki, &curve, &mut rng, 8).expect("precompute");
        let mut rng = StdRng::seed_from_u64(seed ^ salt);
        let sseed = seed ^ salt;
        let n = ki.inst.n;
        let (mut done, mut residuals) = (0usize, 0u64);
        let calls0 = calls;
        while done < per_curve_calls {
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
            let at = format!("p={p} seed={seed} call={calls}");
            let ua = run(&comps, Pivot::Uncounted { sparsest: false });
            let ac = run(
                &comps,
                Pivot::Counted(PivotRule::Lowest, PivotCheck::default()),
            );
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
            let judge =
                |t: &mut Tally, clean: &Run, bad: &Run, aimed: bool, effective: u64, how: Check| {
                    t.calls += 1;
                    t.aimed += usize::from(aimed);
                    t.effective += usize::from(effective > 0);
                    let kind = first_difference(&clean.4, &bad.4);
                    let pc = clean.4.pivots != bad.4.pivots;
                    let tc = clean.4.targets != bad.4.targets;
                    let strict = aimed && kind == 0;
                    t.strict += usize::from(strict);
                    t.pivots_changed += usize::from(aimed && pc);
                    t.targets_changed += usize::from(aimed && tc);
                    t.structure_intact += usize::from(aimed && !pc && !tc);
                    t.unread += usize::from(aimed && kind == 2 && !pc && !tc);
                    let pos = aimed
                        && match how {
                            Check::Strict => strict,
                            Check::Ext => strict || pc || tc,
                            Check::Mid => strict && !pc && !tc,
                        };
                    t.positive += usize::from(pos);
                    if aimed && !pos {
                        t.failed_aimed.push(at.clone());
                    }
                    let det = !same(clean, bad) || clean.3 != bad.3;
                    t.detected_raw += usize::from(det);
                    if det && pos {
                        t.detected_positive += 1;
                        if same(clean, bad) {
                            t.via_fallback += 1;
                        }
                    }
                    let ne = clean.0.as_ref().is_some_and(|s| !s.is_empty());
                    t.nonempty += usize::from(ne);
                    t.nonempty_detected_positive += usize::from(ne && det && pos);
                };
            let ctl = |rule: ArrivalRule, rc: RuleCheck| -> (Run, u64) {
                let _ = take_diag();
                let bad = run(&comps, Pivot::Rule(rule, rc));
                (bad, take_diag().corrupt_changed)
            };
            // C_0 on every rule (E37's).
            for (i, (rule, _)) in RULES.iter().enumerate() {
                let c = &clean[i];
                let target = c.4.reads.iter().find(|r| r.0 == 1 && r.4 > 0).map(|r| r.2);
                let (bad, eff) = ctl(
                    *rule,
                    RuleCheck {
                        bump_row: target,
                        ..Default::default()
                    },
                );
                judge(&mut t_0[i], c, &bad, target.is_some(), eff, Check::Strict);
            }
            // C_SH on S and H (E37's; fallback: the second distinct read).
            for (j, k) in [1usize, 2].into_iter().enumerate() {
                let c = &clean[k];
                let nf: Vec<_> = c.4.reads.iter().filter(|r| r.0 == 1).collect();
                let pair = nf.first().and_then(|r1| {
                    let ok = |r: &&&(u8, u32, u32, u64, u32)| r.2 != r1.2 && r.3 != r1.3;
                    let r2 = if fallback.contains("csh") {
                        nf.iter().find(ok)
                    } else {
                        nf.iter().rev().find(ok)
                    };
                    r2.map(|r2| (r1.2, r2.2))
                });
                let (bad, eff) = ctl(
                    RULES[k].0,
                    RuleCheck {
                        swap_rows: pair,
                        ..Default::default()
                    },
                );
                judge(&mut t_sh[j], c, &bad, pair.is_some(), eff, Check::Strict);
            }
            // C_F (i): E37's misfile, judged by P_ext.
            let c = &clean[0];
            let contested: HashSet<u32> = c.4.contested.iter().copied().collect();
            let cstar = if fallback.contains("cf") {
                let mut cols: Vec<u32> =
                    c.4.reads
                        .iter()
                        .filter(|r| r.0 == 1 && contested.contains(&r.1))
                        .map(|r| r.1)
                        .collect();
                cols.sort_unstable();
                cols.dedup();
                lower_median(&cols)
            } else {
                c.4.reads
                    .iter()
                    .find(|r| r.0 == 1 && contested.contains(&r.1))
                    .map(|r| r.1)
            };
            let (bad, eff) = ctl(
                ArrivalRule::Fifo,
                RuleCheck {
                    misfile_at: cstar,
                    ..Default::default()
                },
            );
            judge(&mut t_f, c, &bad, cstar.is_some(), eff, Check::Ext);
            // C_F^mid (ii): U = uncontested phase-1-read pivot columns.
            let mut u: BTreeMap<u32, u32> = BTreeMap::new();
            for r in
                c.4.reads
                    .iter()
                    .filter(|r| r.0 == 1 && !contested.contains(&r.1))
            {
                u.insert(r.1, r.2);
            }
            let (rc, aim) = if fallback.contains("cfmid") {
                let cols: Vec<u32> = u.keys().copied().collect();
                let l = lower_median(&cols);
                (
                    RuleCheck {
                        bump_pivot: l,
                        ..Default::default()
                    },
                    json!({"control": "cf_mid_b", "l": l, "u": cols.len()}),
                )
            } else {
                let elig: Vec<(u32, u32)> = u
                    .iter()
                    .filter_map(|(&l, &row)| {
                        c.4.updates
                            .iter()
                            .filter(|&&(cc, rr)| rr == row && cc < l)
                            .map(|&(cc, _)| cc)
                            .max()
                            .map(|cm| (cm, row))
                    })
                    .collect();
                let pick = lower_median(&elig);
                let l =
                    pick.and_then(|(_, row)| u.iter().find(|(_, &r)| r == row).map(|(&l, _)| l));
                (
                    RuleCheck {
                        bump_update: pick,
                        ..Default::default()
                    },
                    json!({"control": "cf_mid", "c_m": pick.map(|x| x.0), "l": l, "u": u.len(), "eligible": elig.len()}),
                )
            };
            let aimed = rc.bump_update.is_some() || rc.bump_pivot.is_some();
            let (bad, eff) = ctl(ArrivalRule::Fifo, rc);
            judge(&mut t_mid, c, &bad, aimed, eff, Check::Mid);
            let mut aim = aim;
            aim["call"] = json!(at);
            aim["c_star"] = json!(cstar);
            aims.push(aim);
            calls += 1;
        }
        per_curve.push(
            json!({"p": p, "seed": seed, "calls": calls - calls0, "residuals_drawn": residuals}),
        );
        eprintln!(
            "p={p} seed={seed}: {} calls; positive/detected+ C_F {}/{}, C_F^mid {}/{}, C_SH(S) {}/{}, C_SH(H) {}/{}",
            calls - calls0,
            t_f.positive,
            t_f.detected_positive,
            t_mid.positive,
            t_mid.detected_positive,
            t_sh[0].positive,
            t_sh[0].detected_positive,
            t_sh[1].positive,
            t_sh[1].detected_positive
        );
    }
    check(
        calls == CURVES.len() * per_curve_calls,
        format!("calls {calls}"),
    );
    for (i, (_, name)) in RULES.iter().enumerate() {
        check(
            perturb[i] == nonempty,
            format!(
                "{name}: perturbed coefficient detected {}/{nonempty}",
                perturb[i]
            ),
        );
    }
    let mut fbs: Vec<&String> = fallback.iter().collect();
    fbs.sort();
    let out = json!({
        "experiment": "e38",
        "what": "E38 controls (§9.46, G2.176–G2.179): F controls with P_ext / P_mid, C_SH, C_0; E25's ten curves",
        "salt": format!("{salt:#x}"),
        "calls_per_curve": per_curve_calls,
        "fallbacks_used": fbs,
        "calls": calls,
        "nonempty": nonempty,
        "contested_calls_f": contested_calls,
        "curves": per_curve,
        "perturbed_detected": {"F": perturb[0], "S": perturb[1], "H": perturb[2], "L": perturb[3]},
        "digest_differs_from_a": {"F": digest_differs[0], "S": digest_differs[1], "H": digest_differs[2], "L": digest_differs[3]},
        "c_f": {"F": t_f},
        "c_f_mid": {"F": t_mid},
        "c_sh": {"S": t_sh[0], "H": t_sh[1]},
        "c_0": {"F": t_0[0], "S": t_0[1], "H": t_0[2], "L": t_0[3]},
        "aims": aims,
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
