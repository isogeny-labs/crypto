//! Matched native comparison of protected-target pivot and symbolic-graph IC.
//! This is an IC component/pipeline diagnostic, not an ecbench rho comparison.
//! Run only through the repository's experiment/checkpoint/isolation contract.
//! Raw output is create-new JSONL; failed solves are retained, never filtered.

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::Instant;

use clap::Parser;
use crypto_lib::cryptanalysis::gaudry_cubic::{
    generate_instance3, run_gaudry_with_eliminator, GaudryOptions, LargePrimeBackend, Solver,
    SubspaceBase,
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
    #[arg(long, default_value_t = 10_000)]
    max_residuals: u64,
    /// Frozen source commit, externally checked against binary/build receipt.
    #[arg(long)]
    source_revision: String,
    #[arg(long)]
    out: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.rounds < 5
        || args.primes.is_empty()
        || args.seeds.is_empty()
        || args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("need nonempty inputs, at least five rounds, and a full source SHA".into());
    }
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
            for solver in [Solver::MeetInTheMiddle, Solver::Groebner, Solver::PairOnly] {
                for round in 0..args.rounds {
                    // A/A control and candidate are interleaved; every arm pays
                    // fresh base setup and starts its oracle from the same seed.
                    let mut arms = ["pivot_a", "graph", "pivot_b"];
                    if round % 2 == 1 {
                        arms.reverse();
                    }
                    let mut reference = None;
                    for arm in arms {
                        inst.curve.reset();
                        let start = Instant::now();
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
                        let report = run_gaudry_with_eliminator(&inst, &base, seed, &opts, backend);
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
                        if replay != Some(true) || report.correct != Some(true) || !prefix_equal {
                            failures += 1;
                        }
                        let setup_gae =
                            base_adds as f64 + base_muls as f64 / report.fp_muls_per_add;
                        let record = json!({
                            "schema":"gaudry.cubic.graph-comparison/v1",
                            "source_revision":args.source_revision,
                            "p":p,"seed":seed,"round":round,"arm":arm,"options":opts,
                            "field":inst.curve.field,"a":inst.curve.a,"b":inst.curve.b,
                            "generator":inst.curve.g,"target":inst.q,"subgroup_order":inst.curve.n,
                            "known_answer_scalar":inst.d,
                            "base_setup_fp_muls":base_muls,"base_setup_adds":base_adds,
                            "group_replay_adds":replay_adds,
                            "legacy_model_cold_gae":setup_gae+report.total_ops+replay_adds as f64,
                            "prefix_equal":prefix_equal,"group_replay":replay,
                            "report":report,
                            "unadmitted_wall_ns":elapsed,
                            "cost_status":"legacy_mul_equivalent_diagnostic_not_full_native_budget",
                            "unpriced_scope":["dense_and_pivot_scalar_adds_and_inversions",
                                "hash_and_collection_primitives_outside_graph_meter",
                                "base_lift_integer_and_rng_work","calibration_work"],
                            "hash_work_note":"lp_input_hash_bytes records fingerprint input size; native hash instructions remain unpriced",
                            "clock_status":"requires_external_L2_and_AA_admission"
                        });
                        serde_json::to_writer(&mut output, &record)?;
                        writeln!(output)?;
                        output.flush()?;
                    }
                }
            }
        }
    }
    if failures != 0 {
        return Err(format!("{failures} failed or unmatched rows retained in output").into());
    }
    Ok(())
}
