//! Cold certified F4 reuse across distinct residuals. Coordinate phase only;
//! dense qualification and supplied fixtures are reported separately.
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::Instant;

use clap::{Parser, ValueEnum};
use crypto_lib::cryptanalysis::gaudry_cubic as cubic;
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};

#[derive(Clone, Copy, ValueEnum)]
enum Certifier {
    Exhaustive,
    PairGraph,
}
impl Certifier {
    fn name(self) -> &'static str {
        match self {
            Self::Exhaustive => "exhaustive",
            Self::PairGraph => "pair-graph",
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, value_delimiter = ',', default_value = "67,127,1039")]
    primes: Vec<u64>,
    #[arg(long, value_delimiter = ',', default_value = "173,179,181")]
    seeds: Vec<u64>,
    #[arg(long, default_value_t = 1)]
    warmup_rounds: usize,
    #[arg(long, default_value_t = 5)]
    rounds: usize,
    #[arg(long, default_value_t = 32)]
    residuals_per_instance: usize,
    #[arg(long, default_value_t = 8)]
    batch_iterations: usize,
    #[arg(long, default_value_t = 24)]
    f4_max_degree: u32,
    #[arg(long, default_value_t = 0)]
    f4_deadline_ms: u64,
    #[arg(long, value_enum, default_value = "exhaustive")]
    trace_certifier: Certifier,
    #[arg(long)]
    source_revision: String,
    #[arg(long)]
    out: std::path::PathBuf,
}

type Outputs = Vec<Option<Vec<[u64; 3]>>>;

fn canonical(mut answer: Option<Vec<[u64; 3]>>) -> Option<Vec<[u64; 3]>> {
    if let Some(roots) = &mut answer {
        for triple in roots.iter_mut() {
            triple.sort_unstable();
        }
        roots.sort_unstable();
        roots.dedup();
    }
    answer
}

fn write_row(out: &mut BufWriter<std::fs::File>, row: &Value) -> std::io::Result<()> {
    serde_json::to_writer(&mut *out, row)?;
    writeln!(out)?;
    out.flush()
}

fn root_rng(seed: u64, scalar: u64) -> StdRng {
    StdRng::seed_from_u64(seed ^ scalar.wrapping_mul(0xA57F_C419_D830_196B))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.primes.is_empty()
        || args.seeds.is_empty()
        || args.rounds < 5
        || args.warmup_rounds == 0
        || !(1..=64).contains(&args.batch_iterations)
        || args.residuals_per_instance == 0
        || args.residuals_per_instance > 512
        || !args
            .residuals_per_instance
            .is_multiple_of(args.batch_iterations)
        || args.f4_max_degree != 24
        || args.f4_deadline_ms != 0
        || args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
        || args
            .primes
            .iter()
            .any(|&p| p % 3 != 1 || p >= 1 << 20 || !is_prime_u64(p))
    {
        return Err("invalid frozen trace-batch parameters".into());
    }
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args.out)?,
    );
    let mut failures = 0usize;
    for &p in &args.primes {
        for &seed in &args.seeds {
            let prepare_started = Instant::now();
            let inst = cubic::generate_instance3(p, seed);
            if 2 * args.residuals_per_instance as u64 >= inst.curve.n {
                return Err("supplied group too small for distinct residual abscissae".into());
            }
            let pre = cubic::SymmetrisedS4::precompute(&inst.curve);
            let family = json!([
                inst.curve.field,
                inst.curve.a,
                inst.curve.b,
                inst.curve.g,
                inst.q,
                inst.curve.n
            ]);
            let prepare_ns = prepare_started.elapsed().as_nanos();
            for ordinal in 0..args.residuals_per_instance / args.batch_iterations {
                let fixture_started = Instant::now();
                let scalars: Vec<u64> = (0..args.batch_iterations)
                    .map(|i| (ordinal * args.batch_iterations + i + 1) as u64)
                    .collect();
                let xs: Vec<_> = scalars
                    .iter()
                    .map(|&d| inst.curve.mul(&inst.curve.g, d).x)
                    .collect();
                let unique: std::collections::HashSet<_> = xs.iter().copied().collect();
                if unique.len() != xs.len() {
                    return Err("residual abscissae are not distinct".into());
                }
                let input_digest =
                    blake3::hash(&serde_json::to_vec(&json!([family, scalars, xs]))?)
                        .to_hex()
                        .to_string();
                let fixture_ns = fixture_started.elapsed().as_nanos();
                let qualify_started = Instant::now();
                let mut qualifier_stats = cubic::SolveStats::default();
                let mut qualifier_work = cubic::S4F4Stats::default();
                let expected: Outputs = xs
                    .iter()
                    .zip(&scalars)
                    .map(|(x, &d)| {
                        canonical(cubic::solve_s4_f4(
                            &inst,
                            &pre,
                            x,
                            &mut root_rng(seed, d),
                            &mut qualifier_stats,
                            &mut qualifier_work,
                        ))
                    })
                    .collect();
                let qualifier_ns = qualify_started.elapsed().as_nanos();
                let fixture = json!({"schema":"gaudry.cubic.trace-batch/v1",
                    "source_revision":args.source_revision,"p":p,"seed":seed,"ordinal":ordinal,
                    "family":family,"input_digest":input_digest,"scalars":scalars,"residual_xs":xs,
                    "batch_iterations":args.batch_iterations,"trace_certifier":args.trace_certifier.name()});
                let mut row = fixture.clone();
                row["record_type"] = json!("fixture");
                row["shared_prepare_ns"] = json!(prepare_ns);
                row["fixture_ns"] = json!(fixture_ns);
                row["qualifier_ns"] = json!(qualifier_ns);
                row["qualifier_outputs"] = json!(expected);
                row["qualifier_fp_muls"] = json!(qualifier_stats.fp_muls);
                row["qualifier_complete"] = json!(expected.iter().all(Option::is_some));
                write_row(&mut out, &row)?;
                for (phase, rounds) in
                    [("warmup", args.warmup_rounds), ("measurement", args.rounds)]
                {
                    for round in 0..rounds {
                        let mut arms = ["fresh_a", "trace_a", "fresh_b", "trace_b"];
                        if round % 2 == 1 {
                            arms.reverse();
                        }
                        for arm in arms {
                            let mut row = fixture.clone();
                            row["phase"] = json!(phase);
                            row["round"] = json!(round);
                            row["arm"] = json!(arm);
                            row["record_type"] = json!("execution_start");
                            write_row(&mut out, &row)?;
                            let started = Instant::now();
                            let mut state =
                                arm.starts_with("trace_")
                                    .then(|| match args.trace_certifier {
                                        Certifier::Exhaustive => cubic::S4TraceState::new(&inst),
                                        Certifier::PairGraph => {
                                            cubic::S4TraceState::new_pair_graph(&inst)
                                        }
                                    });
                            let mut stats = cubic::SolveStats::default();
                            let mut work = cubic::S4F4Stats::default();
                            let mut outputs: Outputs = Vec::with_capacity(xs.len());
                            for (x, &d) in xs.iter().zip(&scalars) {
                                let mut rng = root_rng(seed, d);
                                let answer = if let Some(state) = &mut state {
                                    cubic::solve_s4_f4_trace_delayed(
                                        &inst, &pre, x, &mut rng, &mut stats, &mut work, state,
                                    )
                                } else {
                                    cubic::solve_s4_f4_krylov_delayed(
                                        &inst, &pre, x, &mut rng, &mut stats, &mut work,
                                    )
                                };
                                outputs.push(canonical(answer));
                            }
                            let trace_stats = state.as_ref().map(cubic::S4TraceState::stats);
                            drop(state);
                            let elapsed = started.elapsed().as_nanos();
                            let valid = expected.iter().all(Option::is_some) && outputs == expected;
                            failures += usize::from(!valid);
                            row["record_type"] = json!("execution");
                            row["phase_wall_ns"] = json!(elapsed);
                            row["valid"] = json!(valid);
                            row["signature"] = json!({"outputs":outputs,"fp_muls":stats.fp_muls,
                                "quotient_dimension":work.quotient_dimension_total,"trace":trace_stats});
                            row["work"] = serde_json::to_value(work)?;
                            row["output_kind"] =
                                json!("complete_coordinate_batch_distinct_residuals");
                            write_row(&mut out, &row)?;
                        }
                    }
                }
            }
        }
    }
    if failures > 0 {
        Err(format!("{failures} invalid coordinate batches retained").into())
    } else {
        Ok(())
    }
}
