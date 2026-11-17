//! Decomposition oracles at `m >= 3` on Koblitz `K_0`, one target set,
//! five engines: pair-table root finding (the production default),
//! exhaustive enumeration (the reference verdict), Groebner with the
//! default engine `InheritedF4 { max_degree: 3 }`, Groebner with
//! `MatrixF4 { max_degree: 3 }`, and Crossbred (production parameters
//! and a tuned `(D, k)`).
//!
//! ```bash
//! cargo run --release --example crossbred_m3_bench -- 7 9 15      # n list, m = 3
//! cargo run --release --example crossbred_m3_bench -- 9:3 --targets=24 --roots
//! ```
//!
//! Two levels:
//!
//! * **oracle** — the decomposition functions exactly as the pipeline
//!   calls them (`groebner_decompose`, `crossbred_decompose`,
//!   `PairSumTable::decompose`, `enumerate_decompose`): verdict
//!   (decomposed / none / unknown), every returned witness re-added in the
//!   group, cost per target;
//! * **roots** (`--roots`) — the full Boolean root set of the same system
//!   from InheritedF4, MatrixF4 and tuned Crossbred, compared as sets.
//!
//! Units: native word ops (F4: the process-wide `f4_word_ops_total`
//! delta, Crossbred: extraction + transform + filter + solve-row word
//! ops), reported as bit ops with one 64-bit word op = 64 bit ops; the
//! auxiliary `bitop_tally` word ops (packing, hashing, harvest) beside
//! it.  Wall time is indicative only.  Pair-table cost is reported as
//! wall time and the `|F|` probe bound of the `m = 3` scan.

use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::bitop_tally;
use crypto_lib::cryptanalysis::crossbred::{
    extract_crossbred, extract_crossbred_counted, solve_crossbred, CrossbredParams, SearchOptions,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, f4_word_ops_total, solve_boolean_system, FieldStructure,
    SolveOptions, SolverEngine,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_frobenius_factor_base, crossbred_decompose, enumerate_decompose, groebner_decompose,
    KoblitzCurve, PairSumTable,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::masks_low64;
use num_bigint::BigUint;
use std::time::{Duration, Instant};

const BITS_PER_WORD_OP: u64 = 64;

#[derive(Default, Clone)]
struct Arm {
    name: &'static str,
    decomposed: u32,
    none: u32,
    unknown: u32,
    disagree: u32,
    bad_witness: u32,
    native_word_ops: u64,
    aux_word_ops: u64,
    wall: Duration,
}

impl Arm {
    fn new(name: &'static str) -> Self {
        Arm {
            name,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Verdict {
    Yes,
    No,
    Unknown,
}

fn record(
    arm: &mut Arm,
    kc: &KoblitzCurve,
    points: &[BinaryPoint],
    target: &BinaryPoint,
    idxs: &Option<Vec<usize>>,
    exhausted: bool,
    reference: Option<bool>,
) -> Verdict {
    let v = match (idxs, exhausted) {
        (Some(_), _) => Verdict::Yes,
        (None, true) => Verdict::Unknown,
        (None, false) => Verdict::No,
    };
    match v {
        Verdict::Yes => arm.decomposed += 1,
        Verdict::No => arm.none += 1,
        Verdict::Unknown => arm.unknown += 1,
    }
    if let Some(ix) = idxs {
        let sum = ix
            .iter()
            .fold(BinaryPoint::Infinity, |s, &i| kc.add(&s, &points[i]));
        if sum != *target {
            arm.bad_witness += 1;
        }
    }
    if let Some(r) = reference {
        let ok = match v {
            Verdict::Yes => r,
            Verdict::No => !r,
            Verdict::Unknown => true, // counted separately
        };
        if !ok {
            arm.disagree += 1;
        }
    }
    v
}

fn system_degree(eqs: &[crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly]) -> u32 {
    eqs.iter()
        .flat_map(|p| p.terms.iter())
        .map(|t| t.mask.count_ones())
        .max()
        .unwrap_or(2)
        .max(2)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut targets_per_cell = 12u32;
    let mut roots_level = false;
    let mut node_budget = 4096usize;
    let mut skip: Vec<String> = Vec::new();
    let mut cases: Vec<(u32, usize)> = Vec::new();
    for a in &args {
        if let Some(v) = a.strip_prefix("--targets=") {
            targets_per_cell = v.parse().expect("targets");
        } else if a == "--roots" {
            roots_level = true;
        } else if let Some(v) = a.strip_prefix("--budget=") {
            node_budget = v.parse().expect("budget");
        } else if let Some(v) = a.strip_prefix("--skip=") {
            skip.extend(v.split(',').map(str::to_owned));
        } else if let Some((n, m)) = a.split_once(':') {
            cases.push((n.parse().expect("n"), m.parse().expect("m")));
        } else {
            cases.push((a.parse().expect("n"), 3));
        }
    }
    if cases.is_empty() {
        cases = vec![(7, 3), (9, 3), (15, 3)];
    }
    let on = |name: &str| !skip.iter().any(|s| s == name);

    for &(n, m) in &cases {
        let Some(kc) = KoblitzCurve::new(0, n) else {
            println!("n={n}: no K_0 curve");
            continue;
        };
        let Some(fb) = build_frobenius_factor_base(&kc, 0) else {
            println!("n={n}: no factor base");
            continue;
        };
        let st = FieldStructure::new(kc.n, &kc.curve.irreducible);
        let index_of = fb.index_map();
        let g = kc.generator().clone();
        let t_build = Instant::now();
        let table = PairSumTable::build(&kc, &fb);
        let table_build = t_build.elapsed();

        // Probe system for the shape and the tuned (D, k).
        let probe = kc.mul(&g, &BigUint::from(7u32));
        let BinaryPoint::Affine { x: x_probe, .. } = &probe else {
            continue;
        };
        let Some(probe_sys) =
            build_decomposition_system(&fb.subspace_basis, x_probe, &kc.curve.b, m, &st)
        else {
            println!("n={n} m={m}: no decomposition system");
            continue;
        };
        let v = probe_sys.n_vars;
        let deg = system_degree(&probe_sys.equations);
        let budget_bits = SearchOptions::default().max_enumerated_bits as usize;
        let prod_params = CrossbredParams {
            macaulay_degree: deg,
            enumerated: (v / 2).clamp(1, v.saturating_sub(1).min(budget_bits)),
            target_degree: 1,
            ..CrossbredParams::default()
        };
        let mut tuned: Option<CrossbredParams> = None;
        'pick: for d in deg..=(deg + 1) {
            for k in 2..v.min(budget_bits + 1) {
                let params = CrossbredParams {
                    macaulay_degree: d,
                    enumerated: k,
                    target_degree: 1,
                    max_rows: 20_000,
                };
                if let Some(xb) = extract_crossbred(&probe_sys.equations, v, &params) {
                    if xb.stats.kernel_dim >= v - k {
                        tuned = Some(params);
                        break 'pick;
                    }
                }
            }
        }

        println!();
        println!(
            "=== K_0 / F_2^{n}, m = {m}: |F| = {}, ell = {}, v = {v}, eqs = {}, deg = {deg}, \
             pair table build {:.1} ms ===",
            fb.points.len(),
            fb.ell,
            probe_sys.equations.len(),
            table_build.as_secs_f64() * 1e3
        );
        println!(
            "crossbred production params: D={} k={} max_rows={}; tuned: {}",
            prod_params.macaulay_degree,
            prod_params.enumerated,
            prod_params.max_rows,
            tuned
                .map(|p| format!("D={} k={} max_rows={}", p.macaulay_degree, p.enumerated, p.max_rows))
                .unwrap_or_else(|| "none found".into())
        );

        let mut arms = vec![
            Arm::new("pair_table"),
            Arm::new("enumerate"),
            Arm::new("groebner_inherited_f4_3"),
            Arm::new("groebner_matrix_f4_3"),
            Arm::new("crossbred_prod"),
            Arm::new("crossbred_tuned"),
        ];
        // roots-level arms: inherited, matrix, crossbred tuned
        let mut roots_arms = vec![
            Arm::new("roots_inherited_f4_3"),
            Arm::new("roots_matrix_f4_3"),
            Arm::new("roots_crossbred_tuned"),
        ];
        let mut root_sets_equal = 0u32;
        let mut root_sets_differ = 0u32;
        let mut root_counts: Vec<usize> = Vec::new();
        let mut fingerprint: u64 = 0xcbf29ce484222325;
        let mut fp = |x: u64| {
            fingerprint ^= x;
            fingerprint = fingerprint.wrapping_mul(0x100000001b3);
        };
        let mut done_targets = 0u32;

        for t in 1..=targets_per_cell {
            let target = kc.mul(&g, &BigUint::from(t * 7 + 1));
            let BinaryPoint::Affine { x: x_r, .. } = &target else {
                continue;
            };
            done_targets += 1;

            // Reference: exhaustive enumeration.
            let reference = if on("enumerate") {
                let t0 = Instant::now();
                let e = enumerate_decompose(&kc, &fb, &index_of, &target, m);
                arms[1].wall += t0.elapsed();
                record(&mut arms[1], &kc, &fb.points, &target, &e, false, None);
                Some(e.is_some())
            } else {
                None
            };
            fp(reference.map_or(2, u64::from));

            if let (true, Some(table)) = (on("pair"), table.as_ref()) {
                let t0 = Instant::now();
                let r = table.decompose(&kc, &fb, &target, m);
                arms[0].wall += t0.elapsed();
                let vd = record(&mut arms[0], &kc, &fb.points, &target, &r, false, reference);
                fp(vd as u64);
            }

            for (ai, engine, key) in [
                (2usize, SolverEngine::InheritedF4 { max_degree: 3 }, "inh"),
                (3, SolverEngine::MatrixF4 { max_degree: 3 }, "mf4"),
            ] {
                if !on(key) {
                    continue;
                }
                let w0 = f4_word_ops_total();
                let b0 = bitop_tally::snapshot();
                let t0 = Instant::now();
                let (r, stats) = groebner_decompose(
                    &kc, &fb, &index_of, &st, &target, m, engine, node_budget,
                );
                arms[ai].wall += t0.elapsed();
                arms[ai].native_word_ops += f4_word_ops_total() - w0;
                arms[ai].aux_word_ops += bitop_tally::snapshot().since(b0).word_ops;
                let vd = record(
                    &mut arms[ai],
                    &kc,
                    &fb.points,
                    &target,
                    &r,
                    stats.exhausted,
                    reference,
                );
                fp(vd as u64);
            }

            let sys = build_decomposition_system(&fb.subspace_basis, x_r, &kc.curve.b, m, &st)
                .expect("system");
            for (ai, params, key) in [(4usize, Some(prod_params), "xbp"), (5, tuned, "xbt")] {
                let Some(params) = params else { continue };
                if !on(key) {
                    continue;
                }
                let b0 = bitop_tally::snapshot();
                let t0 = Instant::now();
                let pass = if ai == 4 { None } else { Some(params) };
                let (r, stats) =
                    crossbred_decompose(&kc, &fb, &index_of, &st, &target, m, pass);
                arms[ai].wall += t0.elapsed();
                arms[ai].aux_word_ops += bitop_tally::snapshot().since(b0).word_ops;
                // Extraction word ops are not returned by crossbred_decompose;
                // re-extract with the same parameters (deterministic) to count them.
                let mut ext_ops = 0u64;
                let ext = extract_crossbred_counted(&sys.equations, sys.n_vars, &params, &mut ext_ops);
                let ext_native = ext.as_ref().map_or(ext_ops, |x| x.stats.word_ops);
                arms[ai].native_word_ops += ext_native
                    + stats.transform_word_ops
                    + stats.filter_word_ops
                    + stats.solve_row_ops;
                let vd = record(
                    &mut arms[ai],
                    &kc,
                    &fb.points,
                    &target,
                    &r,
                    stats.exhausted,
                    reference,
                );
                fp(vd as u64);
            }

            if roots_level {
                let mut sets: Vec<Option<Vec<u64>>> = Vec::new();
                for (ri, engine) in [
                    (0usize, SolverEngine::InheritedF4 { max_degree: 3 }),
                    (1, SolverEngine::MatrixF4 { max_degree: 3 }),
                ] {
                    if (ri == 1 && !on("mf4")) || (ri == 0 && !on("inh")) {
                        sets.push(None);
                        continue;
                    }
                    let w0 = f4_word_ops_total();
                    let b0 = bitop_tally::snapshot();
                    let t0 = Instant::now();
                    let (roots, stats) = solve_boolean_system(
                        &sys.equations,
                        sys.n_vars,
                        &SolveOptions {
                            engine,
                            max_solutions: usize::MAX,
                            node_budget: 1 << 22,
                            ..Default::default()
                        },
                    );
                    roots_arms[ri].wall += t0.elapsed();
                    roots_arms[ri].native_word_ops += f4_word_ops_total() - w0;
                    roots_arms[ri].aux_word_ops += bitop_tally::snapshot().since(b0).word_ops;
                    if stats.exhausted {
                        roots_arms[ri].unknown += 1;
                        sets.push(None);
                    } else {
                        let mut r = masks_low64(&roots);
                        r.sort_unstable();
                        sets.push(Some(r));
                    }
                }
                if let Some(params) = tuned.filter(|_| on("xbt")) {
                    let b0 = bitop_tally::snapshot();
                    let t0 = Instant::now();
                    let xb = extract_crossbred(&sys.equations, sys.n_vars, &params);
                    let (mut got, ok, ops) = match &xb {
                        Some(xb) => {
                            let (got, s) = solve_crossbred(&sys.equations, xb, &SearchOptions::default());
                            let ops = xb.stats.word_ops
                                + s.transform_word_ops
                                + s.filter_word_ops
                                + s.solve_row_ops;
                            (got, !s.exhausted, ops)
                        }
                        None => (Vec::new(), false, 0),
                    };
                    roots_arms[2].wall += t0.elapsed();
                    roots_arms[2].native_word_ops += ops;
                    roots_arms[2].aux_word_ops += bitop_tally::snapshot().since(b0).word_ops;
                    if ok {
                        got.sort_unstable();
                        sets.push(Some(got));
                    } else {
                        roots_arms[2].unknown += 1;
                        sets.push(None);
                    }
                } else {
                    sets.push(None);
                }
                let present: Vec<&Vec<u64>> = sets.iter().flatten().collect();
                if let Some(first) = present.first() {
                    root_counts.push(first.len());
                    for r in first.iter() {
                        fp(*r);
                    }
                    if present.iter().all(|s| s == first) {
                        root_sets_equal += 1;
                    } else {
                        root_sets_differ += 1;
                    }
                }
            }
        }

        let per = |x: u64| x / u64::from(done_targets.max(1));
        println!();
        println!(
            "| arm | targets | yes | no | unknown | disagree vs enumerate | bad witness | native bit ops/target | aux bit ops/target | wall ms/target |"
        );
        println!("|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
        let print_arm = |a: &Arm| {
            let ran = a.decomposed + a.none + a.unknown;
            if ran == 0 && a.wall.is_zero() {
                return;
            }
            println!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.3} |",
                a.name,
                done_targets,
                a.decomposed,
                a.none,
                a.unknown,
                a.disagree,
                a.bad_witness,
                per(a.native_word_ops * BITS_PER_WORD_OP),
                per(a.aux_word_ops * BITS_PER_WORD_OP),
                a.wall.as_secs_f64() * 1e3 / f64::from(done_targets.max(1)),
            );
        };
        for a in &arms {
            print_arm(a);
        }
        if roots_level {
            for a in &roots_arms {
                print_arm(a);
            }
            println!(
                "root sets: equal on {root_sets_equal} targets, differ on {root_sets_differ}; \
                 root counts {root_counts:?}"
            );
        }
        println!("pair-table m=3 probe bound: |F| = {} per target", fb.points.len());
        println!("verdict fingerprint (FNV-1a over reference, verdicts, roots): {fingerprint:016x}");
        for a in arms.iter().chain(roots_arms.iter()) {
            let ran = a.decomposed + a.none + a.unknown;
            if ran == 0 && a.wall.is_zero() {
                continue;
            }
            println!(
                "JSON {{\"n\":{n},\"m\":{m},\"v\":{v},\"fb\":{},\"arm\":\"{}\",\"targets\":{done_targets},\
                 \"yes\":{},\"no\":{},\"unknown\":{},\"disagree\":{},\"bad_witness\":{},\
                 \"native_word_ops\":{},\"aux_word_ops\":{},\"wall_s\":{:.6}}}",
                fb.points.len(),
                a.name,
                a.decomposed,
                a.none,
                a.unknown,
                a.disagree,
                a.bad_witness,
                a.native_word_ops,
                a.aux_word_ops,
                a.wall.as_secs_f64()
            );
        }
    }
}
