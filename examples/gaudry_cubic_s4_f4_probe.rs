//! Basis-phase feasibility only. F4 does not yet recover decompositions here.
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
                let qualified = cubic::solve_s4_subspace_with(
                    &inst,
                    &pre,
                    &point.x,
                    &mut StdRng::seed_from_u64(rng_seed),
                    &mut cubic::SolveStats::default(),
                    cubic::SolveMode::default(),
                );
                let qualifier_ns = qualifier_started.elapsed().as_nanos();
                write_row(
                    &mut out,
                    &json!({"schema":"gaudry.cubic.s4-component/v1","record_type":"fixture",
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
                        if round % 2 == 1 {
                            arms.reverse();
                        }
                        for arm in arms {
                            let mut valid = qualified.is_some();
                            let mut signatures = Vec::new();
                            let mut work = 0u64;
                            let elapsed;
                            if arm.starts_with("macaulay") {
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
                                &json!({"schema":"gaudry.cubic.s4-component/v1","record_type":"execution",
                                "source_revision":args.source_revision,"p":p,"seed":seed,"ordinal":ordinal,
                                "phase":phase,"round":round,"arm":arm,"input_digest":input_digest,
                                "batch_iterations":args.batch_iterations,"phase_wall_ns":elapsed,"valid":valid,
                                "intra_batch_consistent":consistent,"signature":signatures.first(),"counted_field_work":work,
                                "f4_max_degree":args.f4_max_degree,"f4_deadline_ms":args.f4_deadline_ms,
                                "output_kind":if arm.starts_with("f4") {"basis_only_no_decomposition_recovery"} else {"complete_S4_coordinate_solve"}}),
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
    #[test]
    fn quotient_bounds_and_reference_root_checks_are_exact() {
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
