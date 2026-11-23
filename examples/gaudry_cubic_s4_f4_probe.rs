//! Explicitly separated basis-only and complete dense/sparse extraction controls.
use clap::Parser;
use crypto_lib::cryptanalysis::{f4_fp as f4, gaudry_cubic as cubic};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Parser)]
struct Args {
    #[arg(long, value_delimiter = ',', default_value = "67,127,1039")]
    primes: Vec<u64>,
    #[arg(long, value_delimiter = ',', default_value = "59,61,71")]
    seeds: Vec<u64>,
    #[arg(long, default_value_t = 4)]
    residuals_per_instance: u64,
    #[arg(long, default_value_t = 8)]
    batch_iterations: usize,
    #[arg(long, default_value_t = 5)]
    rounds: u64,
    #[arg(long, default_value_t = 1)]
    warmup_rounds: u64,
    #[arg(long, default_value_t = 24)]
    f4_max_degree: u32,
    #[arg(long, default_value_t = 50)]
    f4_deadline_ms: u64,
    /// Complete native F4 coordinate solves, dense versus checked sparse FGLM.
    #[arg(long)]
    extraction_comparison: bool,
    /// Compare guarded delayed walks; keep scalar or dense as an explicit reference.
    #[arg(long)]
    delayed_comparison: bool,
    #[arg(long)]
    scalar_reference: bool,
    #[arg(long)]
    source_revision: String,
    #[arg(long)]
    out: PathBuf,
}

fn digest(value: &impl Serialize) -> Result<String, serde_json::Error> {
    Ok(blake3::hash(&serde_json::to_vec(value)?)
        .to_hex()
        .to_string())
}

fn write_row(out: &mut impl Write, row: &Value) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer(&mut *out, row)?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}

fn staircase_dimension(basis: &[f4::Poly]) -> Option<usize> {
    let leads: Vec<&[u32]> = basis
        .iter()
        .filter_map(|f| f.first().map(|t| t.0.as_slice()))
        .collect();
    if leads.iter().any(|m| m.iter().all(|&e| e == 0)) {
        return Some(0);
    }
    let mut bounds = [u32::MAX; 3];
    for m in &leads {
        if m.len() != 3 {
            return None;
        }
        for k in 0..3 {
            if m[k] > 0 && (0..3).all(|j| j == k || m[j] == 0) {
                bounds[k] = bounds[k].min(m[k]);
            }
        }
    }
    let volume = bounds
        .iter()
        .try_fold(1u64, |a, &b| a.checked_mul(b as u64))?;
    if volume > 4096 {
        return None;
    }
    let mut dimension = 0;
    for a in 0..bounds[0] {
        for b in 0..bounds[1] {
            for c in 0..bounds[2] {
                let m = [a, b, c];
                if !leads.iter().any(|l| (0..3).all(|k| l[k] <= m[k])) {
                    dimension += 1;
                }
            }
        }
    }
    Some(dimension)
}

fn elementary([a, b, c]: [u64; 3], p: u64) -> [u64; 3] {
    [
        (a + b + c) % p,
        ((a * b) % p + (a * c) % p + (b * c) % p) % p,
        (((a * b) % p) * c) % p,
    ]
}

fn vanishes(poly: &f4::Poly, point: [u64; 3], p: u64) -> bool {
    let mut sum = 0;
    for (monomial, coefficient) in poly {
        let mut term = *coefficient;
        for k in 0..3 {
            for _ in 0..monomial[k] {
                term = ((term as u128 * point[k] as u128) % p as u128) as u64;
            }
        }
        sum = (sum + term) % p;
    }
    sum == 0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if (args.delayed_comparison && !args.extraction_comparison)
        || (args.scalar_reference && !args.delayed_comparison)
    {
        return Err("delayed/scalar controls require complete extraction mode".into());
    }
    if args.extraction_comparison && (args.f4_max_degree != 24 || args.f4_deadline_ms != 0) {
        return Err("complete extraction uses the native degree24/no-deadline solver".into());
    }
    let schema = if args.delayed_comparison {
        "gaudry.cubic.s4-delayed/v1"
    } else if args.extraction_comparison {
        "gaudry.cubic.s4-extraction/v1"
    } else {
        "gaudry.cubic.s4-component/v1"
    };
    if args.source_revision.len() != 40
        || args.rounds < 5
        || args.warmup_rounds == 0
        || args.batch_iterations == 0
        || args.residuals_per_instance == 0
        || args.primes.is_empty()
        || args.seeds.is_empty()
    {
        return Err("nonempty frozen inputs, SHA and retained warmup/five rounds required".into());
    }
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args.out)?,
    );
    let mut failures = 0u64;
    for &p in &args.primes {
        for &seed in &args.seeds {
            let inst = cubic::generate_instance3(p, seed);
            inst.curve.reset();
            let before = Instant::now();
            let pre = cubic::SymmetrisedS4::precompute(&inst.curve);
            let setup_ns = before.elapsed().as_nanos();
            let setup_fp_muls = inst.curve.field.muls();
            let mut input_rng = StdRng::seed_from_u64(seed ^ 0x5334_4634);
            for ordinal in 0..args.residuals_per_instance {
                let (a, b, point) = loop {
                    let a = input_rng.gen_range(0..inst.curve.n);
                    let b = input_rng.gen_range(1..inst.curve.n);
                    let point = inst.curve.add(
                        &inst.curve.mul(&inst.curve.g, a),
                        &inst.curve.mul(&inst.q, b),
                    );
                    if !point.inf {
                        break (a, b, point);
                    }
                };
                let input = || {
                    pre.weil_restrict(&inst.curve.field, &point.x)
                        .map(|component| {
                            let terms: Vec<_> = component
                                .into_iter()
                                .map(|(e, c)| (e.map(u32::from).to_vec(), c))
                                .collect();
                            f4::normalise(&terms, p, f4::Ordering::Grevlex)
                        })
                };
                let components = input();
                let input_digest = digest(&components)?;
                let rng_seed = seed ^ ordinal.wrapping_mul(0x9e37_79b9) ^ 0x534f_4c56;
                let qualifier_started = Instant::now();
                let qualified = if args.extraction_comparison {
                    cubic::solve_s4_f4(
                        &inst,
                        &pre,
                        &point.x,
                        &mut StdRng::seed_from_u64(rng_seed),
                        &mut cubic::SolveStats::default(),
                        &mut cubic::S4F4Stats::default(),
                    )
                } else {
                    cubic::solve_s4_subspace_with(
                        &inst,
                        &pre,
                        &point.x,
                        &mut StdRng::seed_from_u64(rng_seed),
                        &mut cubic::SolveStats::default(),
                        cubic::SolveMode::default(),
                    )
                };
                let qualifier_ns = qualifier_started.elapsed().as_nanos();
                write_row(
                    &mut out,
                    &json!({"schema":schema,"record_type":"fixture",
                    "source_revision":args.source_revision,"p":p,"seed":seed,"ordinal":ordinal,
                    "field":inst.curve.field,"a":inst.curve.a,"b":inst.curve.b,"generator":inst.curve.g,
                    "target":inst.q,"subgroup_order":inst.curve.n,"known_answer_scalar":inst.d,
                    "residual":point,"residual_coefficients":[a,b],"input_digest":input_digest,"components":components,
                    "symbolic_setup_ns":setup_ns,"symbolic_setup_fp_muls":setup_fp_muls,
                    "qualifier_ns":qualifier_ns,"qualified_triples":qualified,
                    "boundary":"engine phase only; shared symbolic setup/input generation/qualification excluded and reported"}),
                )?;
                for (phase, count) in [("warmup", args.warmup_rounds), ("measurement", args.rounds)]
                {
                    for round in 0..count {
                        let mut arms = ["macaulay_a", "f4_a", "macaulay_b", "f4_b"];
                        if args.extraction_comparison {
                            arms = ["dense_a", "krylov_a", "dense_b", "krylov_b"];
                        }
                        if args.delayed_comparison {
                            arms = if args.scalar_reference {
                                ["scalar_a", "delayed_a", "scalar_b", "delayed_b"]
                            } else {
                                ["dense_a", "delayed_a", "dense_b", "delayed_b"]
                            };
                        }
                        if round % 2 == 1 {
                            arms.reverse();
                        }
                        for arm in arms {
                            if args.extraction_comparison {
                                write_row(
                                    &mut out,
                                    &json!({"schema":schema,"record_type":"execution_start",
                                "source_revision":args.source_revision,"p":p,"seed":seed,"ordinal":ordinal,
                                "phase":phase,"round":round,"arm":arm}),
                                )?;
                            }
                            let mut valid = qualified.is_some();
                            let mut signatures = Vec::new();
                            let mut work = 0u64;
                            let elapsed;
                            if args.extraction_comparison {
                                let started = Instant::now();
                                let mut results = Vec::with_capacity(args.batch_iterations);
                                for _ in 0..args.batch_iterations {
                                    let mut stats = cubic::SolveStats::default();
                                    let mut native = cubic::S4F4Stats::default();
                                    let solver = if arm.starts_with("delayed") {
                                        cubic::solve_s4_f4_krylov_delayed
                                    } else if arm.starts_with("krylov") || arm.starts_with("scalar")
                                    {
                                        cubic::solve_s4_f4_krylov
                                    } else {
                                        cubic::solve_s4_f4
                                    };
                                    let triples = solver(
                                        &inst,
                                        &pre,
                                        &point.x,
                                        &mut StdRng::seed_from_u64(rng_seed),
                                        &mut stats,
                                        &mut native,
                                    );
                                    results.push((triples, stats, native));
                                }
                                elapsed = started.elapsed().as_nanos();
                                for (triples, stats, native) in results {
                                    valid &= triples == qualified && triples.is_some();
                                    work += stats.fp_muls;
                                    signatures.push(json!([
                                        triples,
                                        stats.fp_muls,
                                        stats.quotient_dim_total,
                                        native
                                    ]));
                                }
                            } else if arm.starts_with("macaulay") {
                                let started = Instant::now();
                                let mut results = Vec::with_capacity(args.batch_iterations);
                                for _ in 0..args.batch_iterations {
                                    let mut stats = cubic::SolveStats::default();
                                    let triples = cubic::solve_s4_subspace_with(
                                        &inst,
                                        &pre,
                                        &point.x,
                                        &mut StdRng::seed_from_u64(rng_seed),
                                        &mut stats,
                                        cubic::SolveMode::default(),
                                    );
                                    results.push((triples, stats));
                                }
                                elapsed = started.elapsed().as_nanos();
                                for (triples, stats) in results {
                                    valid &= triples == qualified && triples.is_some();
                                    work += stats.fp_muls;
                                    signatures.push(json!([
                                        triples,
                                        stats.fp_muls,
                                        stats.quotient_dim_total
                                    ]));
                                }
                            } else {
                                let started = Instant::now();
                                let mut results = Vec::with_capacity(args.batch_iterations);
                                for _ in 0..args.batch_iterations {
                                    let mut opts = f4::F4Options::new(
                                        f4::Ordering::Grevlex,
                                        args.f4_max_degree,
                                    );
                                    opts.deadline = Some(
                                        Instant::now() + Duration::from_millis(args.f4_deadline_ms),
                                    );
                                    results.push(f4::f4(&input(), 3, p, &opts));
                                }
                                elapsed = started.elapsed().as_nanos();
                                for report in results {
                                    let dimension = staircase_dimension(&report.basis);
                                    valid &= !report.timed_out
                                        && report.pairs_above_bound == 0
                                        && report.staircase_at_stop.is_none()
                                        && dimension.is_some()
                                        && qualified.as_ref().is_some_and(|triples| {
                                            triples.iter().all(|&t| {
                                                report
                                                    .basis
                                                    .iter()
                                                    .all(|f| vanishes(f, elementary(t, p), p))
                                            })
                                        });
                                    work += report.field_ops;
                                    signatures.push(json!([
                                        digest(&report.basis)?,
                                        report.field_ops,
                                        dimension,
                                        report.timed_out,
                                        report.pairs_above_bound,
                                        report.degree_reached,
                                        report.max_rows,
                                        report.max_cols
                                    ]));
                                }
                            }
                            let consistent = signatures.windows(2).all(|w| w[0] == w[1]);
                            valid &= consistent;
                            if !valid {
                                failures += 1;
                            }
                            write_row(
                                &mut out,
                                &json!({"schema":schema,"record_type":"execution",
                                "source_revision":args.source_revision,"p":p,"seed":seed,"ordinal":ordinal,
                                "phase":phase,"round":round,"arm":arm,"input_digest":input_digest,
                                "batch_iterations":args.batch_iterations,"phase_wall_ns":elapsed,"valid":valid,
                                "intra_batch_consistent":consistent,"signature":signatures.first(),"counted_field_work":work,
                                "f4_max_degree":args.f4_max_degree,"f4_deadline_ms":args.f4_deadline_ms,
                                "output_kind":if !args.extraction_comparison && arm.starts_with("f4") {"basis_only_no_decomposition_recovery"} else {"complete_S4_coordinate_solve"}}),
                            )?;
                        }
                    }
                }
            }
        }
    }
    if failures > 0 {
        Err(format!("{failures} invalid/capped batches retained").into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // F4's default per-report operation differences read a process-wide
    // counter. Keep solves from other tests outside these exact comparisons.
    static F4_COUNTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn sparse_coordinate_extraction_matches_dense_on_all_tiny_targets_and_holdouts() {
        let _counts = F4_COUNTS.lock().unwrap();
        for (p, seed, scalars) in [
            (7, 3, vec![]),
            (127, 83, vec![1, 13, 29]),
            (1039, 89, vec![1, 13, 29]),
        ] {
            let inst = cubic::generate_instance3(p, seed);
            let pre = cubic::SymmetrisedS4::precompute(&inst.curve);
            let scalars = if p == 7 {
                (1..inst.curve.n).collect()
            } else {
                scalars
            };
            for d in scalars.into_iter().filter(|&d| d < inst.curve.n) {
                let r = inst.curve.mul(&inst.curve.g, d);
                let mut work = cubic::S4F4Stats::default();
                let mut scalar_stats = cubic::SolveStats::default();
                let dense = cubic::solve_s4_f4(
                    &inst,
                    &pre,
                    &r.x,
                    &mut StdRng::seed_from_u64(91),
                    &mut cubic::SolveStats::default(),
                    &mut cubic::S4F4Stats::default(),
                );
                let fast = cubic::solve_s4_f4_krylov(
                    &inst,
                    &pre,
                    &r.x,
                    &mut StdRng::seed_from_u64(91),
                    &mut scalar_stats,
                    &mut work,
                );
                // Both public coordinate APIs document None as unsupported,
                // which the full collector routes through its charged oracle.
                assert_eq!(dense, fast, "p={p} d={d}");
                let mut delayed_stats = cubic::SolveStats::default();
                let delayed = cubic::solve_s4_f4_krylov_delayed(
                    &inst,
                    &pre,
                    &r.x,
                    &mut StdRng::seed_from_u64(91),
                    &mut delayed_stats,
                    &mut cubic::S4F4Stats::default(),
                );
                assert_eq!(fast, delayed, "delayed p={p} d={d}");
                assert_eq!(
                    scalar_stats.fp_muls, delayed_stats.fp_muls,
                    "semantic work p={p} d={d}"
                );
                assert!(work.krylov.completed + work.krylov.dense_fallbacks <= 1);
                if fast.is_some() && work.quotient_dimension_total > 0 {
                    assert_eq!(work.krylov.completed + work.krylov.dense_fallbacks, 1);
                }
            }
        }
    }

    #[test]
    fn observed_unsupported_coordinate_case_keeps_the_exact_group_fallback() {
        let _counts = F4_COUNTS.lock().unwrap();
        let inst = cubic::generate_instance3(7, 3);
        let pre = cubic::SymmetrisedS4::precompute(&inst.curve);
        let base = cubic::SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        let c = &inst.curve;
        let r = c.mul(&c.g, 171);
        let mut native = cubic::S4F4Stats::default();
        assert!(cubic::solve_s4_f4_krylov(
            &inst,
            &pre,
            &r.x,
            &mut StdRng::seed_from_u64(91),
            &mut cubic::SolveStats::default(),
            &mut native
        )
        .is_none());
        assert_eq!(native.unsupported_quotients, 1);
        let mut stats = cubic::SolveStats::default();
        let got = cubic::subspace_triple_oracle_groebner(
            &inst,
            &base,
            &pre,
            &r,
            &mut StdRng::seed_from_u64(91),
            &mut stats,
        );
        let mut expected = Vec::new();
        for i in 0..base.len() {
            for j in i..base.len() {
                for k in j..base.len() {
                    for si in [1, -1] {
                        for sj in [1, -1] {
                            for sk in [1, -1] {
                                let sum = c.add(
                                    &c.add(
                                        &c.mul_signed(&base.points[i], si),
                                        &c.mul_signed(&base.points[j], sj),
                                    ),
                                    &c.mul_signed(&base.points[k], sk),
                                );
                                if sum == r {
                                    let mut row = std::collections::BTreeMap::<usize, i64>::new();
                                    for (index, sign) in [(i, si), (j, sj), (k, sk)] {
                                        *row.entry(index).or_default() += sign;
                                    }
                                    expected.push(
                                        row.into_iter()
                                            .filter(|(_, s)| *s != 0)
                                            .collect::<Vec<_>>(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        expected.sort();
        expected.dedup();
        assert_eq!(got, expected);
        assert!(stats.fp_muls > 0);
    }

    #[test]
    fn sparse_and_dense_graph_pipelines_recover_the_same_relation_prefix() {
        let _counts = F4_COUNTS.lock().unwrap();
        let inst = cubic::generate_instance3(19, 11);
        let base = cubic::SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(11));
        let opts = cubic::GaudryOptions {
            solver: cubic::Solver::F4Krylov,
            small_base: 2,
            max_large_primes: 2,
            max_residuals: 10000,
            ..cubic::GaudryOptions::default()
        };
        let fast = cubic::run_gaudry_with_sampler(
            &inst,
            &base,
            11,
            &opts,
            cubic::LargePrimeBackend::SymbolicGraph,
            cubic::ResidualSampler3::RAddingLong,
        );
        let pivot = cubic::run_gaudry_with_sampler(
            &inst,
            &base,
            11,
            &opts,
            cubic::LargePrimeBackend::Pivot,
            cubic::ResidualSampler3::RAddingLong,
        );
        let dense = cubic::run_gaudry_with_sampler(
            &inst,
            &base,
            11,
            &cubic::GaudryOptions {
                solver: cubic::Solver::F4Groebner,
                ..opts
            },
            cubic::LargePrimeBackend::SymbolicGraph,
            cubic::ResidualSampler3::RAddingLong,
        );
        let delayed = cubic::run_gaudry_with_sampler(
            &inst,
            &base,
            11,
            &cubic::GaudryOptions {
                solver: cubic::Solver::F4KrylovDelayed,
                ..opts
            },
            cubic::LargePrimeBackend::SymbolicGraph,
            cubic::ResidualSampler3::RAddingLong,
        );
        for r in [&fast, &pivot, &dense, &delayed] {
            assert_eq!(r.correct, Some(true));
        }
        assert_eq!(fast.lp_input_digest, pivot.lp_input_digest);
        assert_eq!(fast.lp_input_digest, dense.lp_input_digest);
        assert_eq!(fast.residuals, dense.residuals);
        assert_eq!(fast.lp_input_digest, delayed.lp_input_digest);
        assert_eq!(fast.oracle_fp_muls, delayed.oracle_fp_muls);
    }

    #[test]
    fn quotient_bounds_and_reference_root_checks_are_exact() {
        let _counts = F4_COUNTS.lock().unwrap();
        let p = 7;
        let basis = vec![
            vec![(vec![2, 0, 0], 1)],
            vec![(vec![0, 2, 0], 1)],
            vec![(vec![0, 0, 3], 1)],
        ];
        assert_eq!(staircase_dimension(&basis), Some(12));
        assert_eq!(staircase_dimension(&basis[..2]), None);
        assert_eq!(staircase_dimension(&[vec![(vec![0, 0, 0], 1)]]), Some(0));
        assert_eq!(elementary([1, 2, 3], p), [6, 4, 6]);
        assert!(vanishes(
            &vec![(vec![1, 0, 0], 1), (vec![0, 0, 0], 1)],
            [6, 0, 0],
            p
        ));
        assert!(!vanishes(
            &vec![(vec![1, 0, 0], 1), (vec![0, 0, 0], 1)],
            [5, 0, 0],
            p
        ));
    }

    #[test]
    fn native_basis_preserves_actual_s4_reference_solutions() {
        let _counts = F4_COUNTS.lock().unwrap();
        let inst = cubic::generate_instance3(7, 3);
        let pre = cubic::SymmetrisedS4::precompute(&inst.curve);
        let qualified = cubic::solve_s4_subspace_with(
            &inst,
            &pre,
            &inst.curve.g.x,
            &mut StdRng::seed_from_u64(91),
            &mut cubic::SolveStats::default(),
            cubic::SolveMode::default(),
        )
        .unwrap();
        let input = pre
            .weil_restrict(&inst.curve.field, &inst.curve.g.x)
            .map(|component| {
                let terms: Vec<_> = component
                    .into_iter()
                    .map(|(e, c)| (e.map(u32::from).to_vec(), c))
                    .collect();
                f4::normalise(&terms, 7, f4::Ordering::Grevlex)
            });
        let report = f4::f4(&input, 3, 7, &f4::F4Options::new(f4::Ordering::Grevlex, 24));
        assert!(!report.timed_out);
        assert_eq!(report.pairs_above_bound, 0);
        assert!(staircase_dimension(&report.basis).is_some());
        for triple in qualified {
            assert!(report
                .basis
                .iter()
                .all(|f| vanishes(f, elementary(triple, 7), 7)));
        }
    }
}
