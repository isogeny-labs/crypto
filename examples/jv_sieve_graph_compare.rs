//! Cold-process comparison on the restricted weak Fp6 cover family.
//! Public challenge generation is outside each solve; transfer is rebuilt
//! inside each IC worker. The planted answer is never supplied to workers.
use clap::Parser;
use crypto_lib::cryptanalysis::{
    jv_cover::{self as cover, EllE, Fq3, PtE6, E2, E6},
    jv_rho_strong::{self as rho, CurveInstance, StrongOpts},
    jv_sieve::{self as sieve, SieveGraphOptions},
    residual_walk::is_prime_u64,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

#[derive(Parser)]
struct Args {
    #[arg(long, value_delimiter = ',', default_value = "127")]
    primes: Vec<u64>,
    #[arg(long, value_delimiter = ',', default_value = "239")]
    seeds: Vec<u64>,
    #[arg(long, default_value_t = 1)]
    warmup_rounds: usize,
    #[arg(long, default_value_t = 5)]
    rounds: usize,
    #[arg(long, default_value_t = 75)]
    retained_percent: usize,
    #[arg(long, default_value = "")]
    source_revision: String,
    #[arg(long)]
    out: Option<PathBuf>,
    #[arg(long, hide = true)]
    worker: Option<String>,
    #[arg(long, hide = true)]
    fixture: Option<PathBuf>,
    #[arg(long, hide = true, default_value_t = 239)]
    walk_seed: u64,
    #[arg(long)]
    capacity_report: bool,
    #[arg(long)]
    validation_only: bool,
    #[arg(long, default_value_t = 0)]
    worker_timeout_seconds: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct PublicPoint {
    x: [[u64; 2]; 3],
    y: [[u64; 2]; 3],
    inf: bool,
}
impl PublicPoint {
    fn from_native(p: PtE6) -> Self {
        Self {
            x: p.x.0.map(|e| e.0),
            y: p.y.0.map(|e| e.0),
            inf: p.inf,
        }
    }
    fn native(&self) -> PtE6 {
        PtE6 {
            x: E6(self.x.map(E2)),
            y: E6(self.y.map(E2)),
            inf: self.inf,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicCurve {
    p: u64,
    l: u64,
    alpha: [[u64; 2]; 3],
    g: PublicPoint,
    q: PublicPoint,
}

fn rho_options(l: u64) -> StrongOpts {
    StrongOpts {
        negation: true,
        batch: 16,
        dp_bits: 16.min((l.ilog2() / 2).saturating_sub(4)),
        r: 256,
    }
}

fn without_times(v: &mut Value) {
    if let Some(o) = v.as_object_mut() {
        o.remove("wall_ms");
        o.remove("filter_call_wall_ns");
        for v in o.values_mut() {
            without_times(v);
        }
    } else if let Some(a) = v.as_array_mut() {
        for v in a {
            without_times(v);
        }
    }
}

fn solve_worker(
    curve: &PublicCurve,
    method: &str,
    seed: u64,
    keep: usize,
) -> Result<Value, String> {
    if !is_prime_u64(curve.p)
        || !is_prime_u64(curve.l)
        || !(5..=1823).contains(&curve.p)
        || !(3..1 << 63).contains(&curve.l)
        || !(1..=100).contains(&keep)
        || !["ic", "graph", "rho"].contains(&method)
        || curve.g.inf
        || curve.q.inf
        || curve
            .alpha
            .iter()
            .flatten()
            .chain(curve.g.x.iter().flatten())
            .chain(curve.g.y.iter().flatten())
            .chain(curve.q.x.iter().flatten())
            .chain(curve.q.y.iter().flatten())
            .any(|&v| v >= curve.p)
    {
        return Err("invalid public curve/worker configuration".into());
    }
    let started = Instant::now();
    let f = Fq3::new(curve.p);
    let alpha = E6(curve.alpha.map(E2));
    let ec = EllE::new(&f, &alpha);
    let (g, q) = (curve.g.native(), curve.q.native());
    if !ec.mul(&g, curve.l).inf {
        return Err("public generator has wrong order".into());
    }
    let (scalar, work, native_verified, capped) = if method == "rho" {
        let instance = CurveInstance {
            p: curve.p,
            l: curve.l,
            d: 0,
            a2: ec.a2,
            a4: ec.a4,
            g,
            q,
        };
        let (counted, scalar) =
            rho::strong_rho_counted_with_scalar(&instance, seed, 0, rho_options(curve.l));
        let verified = counted.verified;
        let capped = counted.run.capped;
        (
            scalar,
            serde_json::to_value(counted).map_err(|e| e.to_string())?,
            verified,
            capped,
        )
    } else {
        // Zero is an answer placeholder, not the challenge's actual logarithm.
        let spec = cover::spec_from_curve(curve.p, alpha, curve.l, g, q, 0)
            .ok_or("public transfer/subgroup validation failed")?;
        let options = SieveGraphOptions {
            retained_percent: keep,
            ..Default::default()
        };
        let (report, scalar) = sieve::run_cover_sieve_dlp_on_with_filter(
            &spec,
            seed,
            0,
            1.0,
            Some(64),
            1.25,
            None,
            false,
            (method == "graph").then_some(&options),
        );
        let verified = report.solved;
        let capped = report.exhausted;
        (
            scalar,
            serde_json::to_value(report).map_err(|e| e.to_string())?,
            verified,
            capped,
        )
    };
    let verified =
        native_verified && scalar.is_some_and(|d| d > 0 && d < curve.l && ec.mul(&g, d) == q);
    let elapsed = started.elapsed().as_nanos();
    let mut semantic_work = work.clone();
    without_times(&mut semantic_work);
    Ok(
        json!({"method":method,"solver_answer_placeholder":0,"recovered_scalar":scalar,
        "verified":verified,"capped":capped,"inner_wall_ns":elapsed,"work":work,
        "signature":{"scalar":scalar,"verified":verified,"capped":capped,"work":semantic_work}}),
    )
}

fn write_row(out: &mut BufWriter<File>, row: &Value) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer(&mut *out, row)?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if let Some(method) = &args.worker {
        let curve = serde_json::from_reader(File::open(
            args.fixture.as_ref().ok_or("worker fixture required")?,
        )?)?;
        let result = solve_worker(&curve, method, args.walk_seed, args.retained_percent)?;
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    if args.capacity_report {
        let rows: Vec<_> = args
            .primes
            .iter()
            .map(|&p| {
                let capacity = sieve::relations_estimate(p, 9);
                json!({"p":p,"factor_base_upper_bound":p,"m9_model_relations":capacity,
                "qualifies_model_guard":capacity>=1.25*p as f64,
                "scope":"existing heuristic capacity model; no runtime or guaranteed-yield claim"})
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&rows)?);
        return Ok(());
    }
    if (!args.validation_only && (args.rounds < 5 || args.warmup_rounds == 0))
        || (args.validation_only
            && (args.rounds != 1 || args.warmup_rounds != 0 || args.worker_timeout_seconds != 900))
        || args.primes.is_empty()
        || args.seeds.is_empty()
        || args
            .primes
            .iter()
            .any(|&p| !(5..=1823).contains(&p) || !is_prime_u64(p))
        || !(1..=100).contains(&args.retained_percent)
        || args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
        || (args.validation_only
            && args
                .primes
                .iter()
                .any(|&p| sieve::relations_estimate(p, 9) < 1.25 * p as f64))
    {
        return Err("invalid frozen cover comparison parameters".into());
    }
    let path = args.out.as_ref().ok_or("output required")?;
    let dir = path.parent().ok_or("output parent required")?;
    let mut out = BufWriter::new(OpenOptions::new().write(true).create_new(true).open(path)?);
    let mut failures = 0;
    for &p in &args.primes {
        for &seed in &args.seeds {
            let prepared = Instant::now();
            let spec = cover::generate_spec(p, seed);
            let curve = PublicCurve {
                p,
                l: spec.l,
                alpha: spec.alpha.0.map(|e| e.0),
                g: PublicPoint::from_native(spec.g),
                q: PublicPoint::from_native(spec.q),
            };
            let bytes = serde_json::to_vec(&serde_json::to_value(&curve)?)?;
            let digest = blake3::hash(&bytes).to_hex().to_string();
            let fixture_path = dir.join(format!("public-{p}-{seed}.json"));
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&fixture_path)?
                .write_all(&bytes)?;
            let prepare_ns = prepared.elapsed().as_nanos();
            let schema = if args.validation_only {
                "cover.sieve.graph-validation/v1"
            } else {
                "cover.sieve.graph-process/v1"
            };
            let base = json!({"schema":schema,"p":p,"seed":seed,
            "source_revision":args.source_revision,"family":curve,"input_digest":digest,
            "retained_percent":args.retained_percent,"rho_options":rho_options(spec.l),
            "validation_only":args.validation_only,"worker_timeout_seconds":args.worker_timeout_seconds,
            "capacity_model_m9":sieve::relations_estimate(p,9),
            "sieve_recipe":if args.validation_only {"incidence_shared"}else{"historical_default"}});
            let mut fixture = base.clone();
            fixture["record_type"] = json!("fixture");
            fixture["expected_scalar"] = json!(spec.d);
            fixture["challenge_preparation_ns"] = json!(prepare_ns);
            write_row(&mut out, &fixture)?;
            let phases = if args.validation_only {
                vec![("validation", 1)]
            } else {
                vec![("warmup", args.warmup_rounds), ("measurement", args.rounds)]
            };
            for (phase, rounds) in phases {
                for round in 0..rounds {
                    let mut arms = if args.validation_only {
                        vec!["ic_a", "graph_a"]
                    } else {
                        vec!["ic_a", "graph_a", "rho_a", "ic_b", "graph_b", "rho_b"]
                    };
                    if round % 2 == 1 {
                        arms.reverse();
                    }
                    for arm in arms {
                        let method = arm.split('_').next().unwrap();
                        let mut row = base.clone();
                        row["phase"] = json!(phase);
                        row["round"] = json!(round);
                        row["arm"] = json!(arm);
                        row["record_type"] = json!("execution_start");
                        let progress_path = dir.join(format!(
                            "progress-p{p}-s{seed}-{phase}-{round}-{arm}.stderr"
                        ));
                        row["progress_path"] = json!(progress_path);
                        write_row(&mut out, &row)?;
                        let started = Instant::now();
                        let executable = std::env::current_exe()?;
                        let mut command = if args.worker_timeout_seconds > 0 {
                            let mut c = Command::new("timeout");
                            c.args([
                                "--signal=TERM",
                                "--kill-after=5s",
                                &format!("{}s", args.worker_timeout_seconds),
                            ])
                            .arg(&executable);
                            c
                        } else {
                            Command::new(&executable)
                        };
                        command
                            .args(["--worker", method, "--fixture"])
                            .arg(&fixture_path)
                            .args([
                                "--walk-seed",
                                &seed.to_string(),
                                "--retained-percent",
                                &args.retained_percent.to_string(),
                            ]);
                        for (key, _) in std::env::vars_os()
                            .filter(|(key, _)| key.to_string_lossy().starts_with("JV_"))
                        {
                            command.env_remove(key);
                        }
                        command
                            .env("RAYON_NUM_THREADS", "1")
                            .env("OMP_NUM_THREADS", "1");
                        let progress = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&progress_path)?;
                        command.stderr(Stdio::from(progress));
                        if args.validation_only {
                            command
                                .env("JV_SIEVE_PROGRESS", "1")
                                .env("JV_ADDS_INC", "1")
                                .env("JV_ADDS_SHARE", "1");
                        }
                        let result = command.output()?;
                        let elapsed = started.elapsed().as_nanos();
                        let worker: Value =
                            serde_json::from_slice(&result.stdout).unwrap_or(Value::Null);
                        let valid = result.status.success()
                            && worker["verified"] == true
                            && worker["capped"] == false
                            && worker["recovered_scalar"] == spec.d
                            && worker["solver_answer_placeholder"] == 0;
                        failures += usize::from(!valid);
                        row["record_type"] = json!("execution");
                        row["cold_process_wall_ns"] = json!(elapsed);
                        row["child_exit"] = json!(result.status.code());
                        row["valid"] = json!(valid);
                        row["worker"] = worker;
                        row["stderr"] = json!(std::fs::read_to_string(&progress_path)?);
                        write_row(&mut out, &row)?;
                    }
                }
            }
        }
    }
    if failures > 0 {
        return Err(format!("{failures} failed cold processes retained").into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_fixture_has_no_answer_and_rho_returns_its_actual_scalar() {
        let spec = cover::generate_spec(7, 3);
        let curve = PublicCurve {
            p: 7,
            l: spec.l,
            alpha: spec.alpha.0.map(|e| e.0),
            g: PublicPoint::from_native(spec.g),
            q: PublicPoint::from_native(spec.q),
        };
        let serialized = serde_json::to_value(&curve).unwrap();
        assert!(serialized.get("d").is_none() && serialized.get("expected_scalar").is_none());
        let got = solve_worker(&curve, "rho", 3, 75).unwrap();
        assert_eq!(got["recovered_scalar"], spec.d);
        assert_eq!(got["verified"], true);
        assert_eq!(got["work"]["run"]["correct"], false); // no planted answer supplied
    }
}
