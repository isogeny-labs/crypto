//! Matched native comparison of protected-target pivot and symbolic-graph IC.
//! Includes the existing same-instance cubic rho reference. Clocks and the
//! incomplete multiplication model require external admission before ranking.
//! Run only through the repository's experiment/checkpoint/isolation contract.
//! Raw output is create-new JSONL; failed solves are retained, never filtered.

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::Instant;

use clap::{Parser, ValueEnum};
use crypto_lib::cryptanalysis::gaudry_cubic::{
    generate_instance3, run_gaudry_with_sampler, run_rho3, GaudryOptions, LargePrimeBackend,
    ResidualSampler3, Solver, SubspaceBase,
};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::json;

#[derive(Parser)]
struct Args {
    #[arg(long, value_delimiter = ',', default_value = "7,13,19,31")]
    primes: Vec<u64>,
    #[arg(long, value_delimiter = ',', default_value = "3,7,11,23")]
    seeds: Vec<u64>,
    #[arg(long, default_value_t = 5)]
    rounds: usize,
    #[arg(long, default_value_t = 1)]
    warmup_rounds: usize,
    #[arg(
        long,
        value_enum,
        value_delimiter = ',',
        default_value = "mitm,groebner,pair,symmetric-pair,symmetric-mitm"
    )]
    oracles: Vec<Oracle>,
    #[arg(long, value_enum, default_value = "fresh")]
    sampler: Sampler,
    #[arg(long, default_value_t = 10_000)]
    max_residuals: u64,
    /// Flush a start event before every arm, outside its measured interval.
    #[arg(long)]
    execution_starts: bool,
    /// Frozen source commit, externally checked against binary/build receipt.
    #[arg(long)]
    source_revision: String,
    #[arg(long)]
    out: std::path::PathBuf,
}

#[derive(Clone, Copy, ValueEnum)]
enum Oracle {
    Mitm,
    Groebner,
    F4,
    F4Krylov,
    F4KrylovDelayed,
    F4TraceDelayed,
    F4TracePairGraph,
    Pair,
    SymmetricPair,
    PreparedPair,
    PseudoPair,
    DelayedPseudoPair,
    SymmetricMitm,
}

#[derive(Clone, Copy, ValueEnum)]
enum Sampler {
    Fresh,
    RAdding,
    RAddingLong,
}

impl Sampler {
    fn mode(self) -> ResidualSampler3 {
        match self {
            Self::Fresh => ResidualSampler3::FreshScalar,
            Self::RAdding => ResidualSampler3::RAdding32,
            Self::RAddingLong => ResidualSampler3::RAddingLong,
        }
    }
}

impl Oracle {
    fn solver(self) -> Solver {
        match self {
            Self::Mitm => Solver::MeetInTheMiddle,
            Self::Groebner => Solver::Groebner,
            Self::F4 => Solver::F4Groebner,
            Self::F4Krylov => Solver::F4Krylov,
            Self::F4KrylovDelayed => Solver::F4KrylovDelayed,
            Self::F4TraceDelayed => Solver::F4TraceDelayed,
            Self::F4TracePairGraph => Solver::F4TracePairGraph,
            Self::Pair => Solver::PairOnly,
            Self::SymmetricPair => Solver::SymmetricPairOnly,
            Self::PreparedPair => Solver::PreparedSymmetricPairOnly,
            Self::PseudoPair => Solver::PseudoRemainderPairOnly,
            Self::DelayedPseudoPair => Solver::DelayedPseudoRemainderPairOnly,
            Self::SymmetricMitm => Solver::SymmetricMeetInTheMiddle,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let sampler = args.sampler.mode();
    if args.rounds < 5
        || args.warmup_rounds == 0
        || args.primes.is_empty()
        || args.seeds.is_empty()
        || args.oracles.is_empty()
        || args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err(
            "need nonempty inputs, a warmup, at least five rounds, and a full source SHA".into(),
        );
    }
    let total_rounds = args
        .warmup_rounds
        .checked_add(args.rounds)
        .ok_or("round count overflow")?;
    let mut output = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args.out)?,
    );
    let mut failures = 0usize;
    for &p in &args.primes {
        if p < 7 || p % 3 != 1 || !is_prime_u64(p) {
            return Err(format!(
                "unsupported cubic fixture p={p}; require prime p >= 7, p=1 mod 3"
            )
            .into());
        }
        for &seed in &args.seeds {
            let inst = generate_instance3(p, seed);
            for &oracle in &args.oracles {
                let solver = oracle.solver();
                for iteration in 0..total_rounds {
                    let (phase, round) = if iteration < args.warmup_rounds {
                        ("warmup", iteration)
                    } else {
                        ("measurement", iteration - args.warmup_rounds)
                    };
                    // A/A control and candidate are interleaved; every arm pays
                    // fresh base setup and starts its oracle from the same seed.
                    let mut arms = ["pivot_a", "graph", "pivot_b", "rho_a", "rho_b"];
                    if round % 2 == 1 {
                        arms.reverse();
                    }
                    let mut reference = None;
                    let mut rho_reference = None;
                    let mut diagnostics = Vec::new();
                    for arm in arms {
                        if args.execution_starts {
                            serde_json::to_writer(
                                &mut output,
                                &json!({
                                    "schema":"gaudry.cubic.graph-comparison/v2","record_type":"execution_start",
                                    "source_revision":args.source_revision,"sampler":sampler,"phase":phase,
                                    "p":p,"seed":seed,"round":round,"arm":arm,"oracle_block":solver,
                                    "field":inst.curve.field,"a":inst.curve.a,"b":inst.curve.b,
                                    "generator":inst.curve.g,"target":inst.q,"subgroup_order":inst.curve.n,
                                    "known_answer_scalar":inst.d
                                }),
                            )?;
                            writeln!(output)?;
                            output.flush()?;
                        }
                        inst.curve.reset();
                        let start = Instant::now();
                        if arm.starts_with("rho_") {
                            let report = run_rho3(&inst, seed);
                            let before_replay = inst.curve.ops();
                            let replay = report
                                .recovered_scalar
                                .map(|d| inst.curve.mul(&inst.curve.g, d) == inst.q);
                            let replay_adds = inst.curve.ops() - before_replay;
                            let elapsed = start.elapsed().as_nanos();
                            let signature =
                                (report.steps, report.group_ops, report.recovered_scalar);
                            let aa_equal =
                                rho_reference.as_ref().is_none_or(|old| old == &signature);
                            rho_reference.get_or_insert(signature);
                            let valid =
                                replay == Some(true) && report.correct == Some(true) && aa_equal;
                            if !valid {
                                failures += 1;
                            }
                            let cost = report.group_ops + replay_adds;
                            diagnostics.push((arm, cost as f64, elapsed, valid));
                            let record = json!({
                                "schema":"gaudry.cubic.graph-comparison/v2", "record_type":"execution",
                                "source_revision":args.source_revision, "phase":phase,"sampler":sampler,
                                "p":p, "seed":seed, "round":round, "arm":arm, "oracle_block":solver,
                                "field":inst.curve.field,"a":inst.curve.a,"b":inst.curve.b,
                                "generator":inst.curve.g,"target":inst.q,"subgroup_order":inst.curve.n,
                                "known_answer_scalar":inst.d, "report":report,
                                "group_replay":replay,"group_replay_adds":replay_adds,"aa_equal":aa_equal,
                                "legacy_model_cold_gae":cost,"unadmitted_wall_ns":elapsed,
                                "reference_status":"existing_cubic_radding_reference_not_fastest_external",
                                "clock_status":"requires_external_L2_and_AA_admission"
                            });
                            serde_json::to_writer(&mut output, &record)?;
                            writeln!(output)?;
                            output.flush()?;
                            continue;
                        }
                        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(seed));
                        let base_muls = inst.curve.field.muls();
                        let base_adds = inst.curve.ops();
                        let small = (base.len() as f64).powf(2.0 / 3.0).ceil() as usize;
                        let opts = GaudryOptions {
                            max_residuals: args.max_residuals,
                            solver,
                            small_base: small.max(1),
                            max_large_primes: 2,
                            ..GaudryOptions::default()
                        };
                        let backend = if arm == "graph" {
                            LargePrimeBackend::SymbolicGraph
                        } else {
                            LargePrimeBackend::Pivot
                        };
                        let report =
                            run_gaudry_with_sampler(&inst, &base, seed, &opts, backend, sampler);
                        let replay_before = inst.curve.ops();
                        let replay = report
                            .recovered_scalar
                            .map(|d| inst.curve.mul(&inst.curve.g, d) == inst.q);
                        let replay_adds = inst.curve.ops() - replay_before;
                        let elapsed = start.elapsed().as_nanos();
                        let signature = (
                            report.residuals,
                            report.relations_verified,
                            report.oracle_fp_muls,
                            report.group_ops,
                            report.lp_histogram,
                            report.recovered_scalar,
                            report.lp_input_digest.clone(),
                        );
                        let prefix_equal = reference.as_ref().is_none_or(|old| old == &signature);
                        reference.get_or_insert(signature);
                        let valid =
                            replay == Some(true) && report.correct == Some(true) && prefix_equal;
                        if !valid {
                            failures += 1;
                        }
                        let setup_gae =
                            base_adds as f64 + base_muls as f64 / report.fp_muls_per_add;
                        let cost = setup_gae + report.total_ops + replay_adds as f64;
                        diagnostics.push((arm, cost, elapsed, valid));
                        let record = json!({
                            "schema":"gaudry.cubic.graph-comparison/v2","record_type":"execution",
                            "source_revision":args.source_revision,"sampler":sampler,
                            "phase":phase,"p":p,"seed":seed,"round":round,"arm":arm,"oracle_block":solver,"options":opts,
                            "field":inst.curve.field,"a":inst.curve.a,"b":inst.curve.b,
                            "generator":inst.curve.g,"target":inst.q,"subgroup_order":inst.curve.n,
                            "known_answer_scalar":inst.d,
                            "base_setup_fp_muls":base_muls,"base_setup_adds":base_adds,
                            "group_replay_adds":replay_adds,
                            "legacy_model_cold_gae":cost,
                            "prefix_equal":prefix_equal,"group_replay":replay,
                            "report":report,
                            "unadmitted_wall_ns":elapsed,
                            "cost_status":"legacy_mul_equivalent_diagnostic_not_full_native_budget",
                            "unpriced_scope":["dense_and_pivot_scalar_adds_and_inversions",
                                "oracle_scalar_inversions",
                                "hash_and_collection_primitives_outside_graph_meter",
                                "base_lift_integer_and_rng_work","calibration_work"],
                            "hash_work_note":"lp_input_hash_bytes records fingerprint input size; native hash instructions remain unpriced",
                            "clock_status":"requires_external_L2_and_AA_admission"
                        });
                        serde_json::to_writer(&mut output, &record)?;
                        writeln!(output)?;
                        output.flush()?;
                    }
                    // A failed arm invalidates the complete block. Raw rows
                    // remain flushed even when a later arm fails or is capped.
                    let block_valid = diagnostics.iter().all(|row| row.3);
                    let rho = diagnostics.iter().find(|row| row.0 == "rho_a").unwrap();
                    let ratios: Vec<_> = diagnostics
                        .iter()
                        .filter(|row| !row.0.starts_with("rho_"))
                        .map(|row| {
                            json!({
                                "arm":row.0,
                                "legacy_cost_over_rho":row.1 / rho.1,
                                "unadmitted_wall_over_rho":row.2 as f64 / rho.2.max(1) as f64
                            })
                        })
                        .collect();
                    serde_json::to_writer(
                        &mut output,
                        &json!({
                            "schema":"gaudry.cubic.graph-comparison/v2","record_type":"block_diagnostic",
                        "source_revision":args.source_revision,"phase":phase,"sampler":sampler,
                            "p":p,"seed":seed,"round":round,"oracle_block":solver,
                            "block_valid":block_valid,"ratios":ratios,
                            "parity_status":"unadmitted_diagnostic_only_incomplete_pricing_and_reference_scope"
                        }),
                    )?;
                    writeln!(output)?;
                    output.flush()?;
                }
            }
        }
    }
    if failures != 0 {
        return Err(format!("{failures} failed or unmatched rows retained in output").into());
    }
    Ok(())
}
