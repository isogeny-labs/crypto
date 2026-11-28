//! Cold-process comparison on the restricted weak Fp6 cover family.
//! Public challenge generation is outside each solve; transfer is rebuilt
//! inside each IC worker. The planted answer is never supplied to workers.
use clap::Parser;
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as bitops, JvTally, Phase},
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
    #[arg(long)]
    incidence_shared: bool,
    #[arg(long)]
    mst_graph: bool,
    /// Emit incomplete operation-accounting diagnostics; never a ranked total.
    #[arg(long)]
    bitops: bool,
    /// Price rho's executed native tower; preserve harness prices by default.
    #[arg(long, requires = "bitops")]
    native_rho_prices: bool,
    #[arg(long, default_value_t = 0)]
    worker_timeout_seconds: u64,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    mst: bool,
) -> Result<Value, String> {
    solve_worker_inner(curve, method, seed, keep, mst, false, RhoPricing::default())
}

#[derive(Clone, Copy, Default)]
struct RhoPricing {
    native: bool,
    options: Option<StrongOpts>,
}

/// Shared public-input reference for the combined IC/graph comparison.
/// The supplied JSON is decoded with the same answer-rejecting schema.
#[allow(dead_code)] // Linked by the combined public-input comparison example.
pub(crate) fn solve_configured_native_rho(
    public: &Value,
    seed: u64,
    options: StrongOpts,
) -> Result<Value, String> {
    if !(1..=64).contains(&options.batch)
        || !(1..=4096).contains(&options.r)
        || options.dp_bits > 44
    {
        return Err("invalid bounded rho portfolio options".into());
    }
    let curve: PublicCurve = serde_json::from_value(public.clone()).map_err(|e| e.to_string())?;
    solve_worker_counted_mode(
        &curve,
        "rho",
        seed,
        75,
        false,
        RhoPricing {
            native: true,
            options: Some(options),
        },
    )
}

fn solve_worker_counted(
    curve: &PublicCurve,
    method: &str,
    seed: u64,
    keep: usize,
    mst: bool,
    native_rho_prices: bool,
) -> Result<Value, String> {
    solve_worker_counted_mode(
        curve,
        method,
        seed,
        keep,
        mst,
        RhoPricing {
            native: native_rho_prices,
            options: None,
        },
    )
}

fn solve_worker_counted_mode(
    curve: &PublicCurve,
    method: &str,
    seed: u64,
    keep: usize,
    mst: bool,
    pricing: RhoPricing,
) -> Result<Value, String> {
    // Nested parallel iterators must execute on the tally owner's thread.
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .map_err(|e| e.to_string())?;
    pool.install(|| {
        bitops::claim();
        bitops::set_moduli(curve.p, curve.l);
        bitops::enable(true);
        let _instance = bitops::enter(Phase::Instance);
        solve_worker_inner(curve, method, seed, keep, mst, true, pricing)
    })
}

fn solve_worker_inner(
    curve: &PublicCurve,
    method: &str,
    seed: u64,
    keep: usize,
    mst: bool,
    accounting: bool,
    rho_pricing: RhoPricing,
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
    // Public-domain validation above is excluded. Everything from the public
    // curve to the recovered scalar and its replay below is cold attack work.
    let before = bitops::snapshot();
    let uniform_before = accounting.then(bitops::uniform_snapshot);
    let other_before = bitops::snapshot_other();
    let _attack = bitops::enter(Phase::Model);
    let mut native_calibration = None;
    let (scalar, work, native_verified, capped, rho_work) = if method == "rho" {
        let instance = CurveInstance {
            p: curve.p,
            l: curve.l,
            d: 0,
            a2: ec.a2,
            a4: ec.a4,
            g,
            q,
        };
        let (counted, scalar) = rho::strong_rho_counted_with_scalar(
            &instance,
            seed,
            0,
            rho_pricing.options.unwrap_or_else(|| rho_options(curve.l)),
        );
        let verified = counted.verified;
        let capped = counted.run.capped;
        let tally = if accounting {
            let _harness = bitops::enter(Phase::Harness);
            let prices = if rho_pricing.native {
                rho::measure_native_fq3_prims(curve.p, curve.l)
            } else {
                rho::measure_fq3_prims(curve.p, curve.l)
            };
            if rho_pricing.native {
                native_calibration = Some(
                    json!({"schema":"rho.native-tower-primitives/v1","mul":bitops::prims_json(&prices.mul),"square":bitops::prims_json(&prices.sq),"inverse":bitops::prims_json(&prices.inv),"complete":false,"legacy_tuned_credits_included":false}),
                );
            }
            Some(rho::rho_tally(&counted, &prices))
        } else {
            None
        };
        (
            scalar,
            serde_json::to_value(counted).map_err(|e| e.to_string())?,
            verified,
            capped,
            tally,
        )
    } else {
        // Zero is an answer placeholder, not the challenge's actual logarithm.
        let spec = cover::spec_from_curve(curve.p, alpha, curve.l, g, q, 0)
            .ok_or("public transfer/subgroup validation failed")?;
        let options = SieveGraphOptions {
            retained_percent: keep,
            ..Default::default()
        };
        let (report, scalar) = if mst {
            sieve::run_cover_sieve_dlp_on_with_mst_filter(
                &spec,
                seed,
                0,
                1.0,
                Some(64),
                1.25,
                None,
                false,
                (method == "graph").then_some(&options),
            )
        } else {
            sieve::run_cover_sieve_dlp_on_with_filter(
                &spec,
                seed,
                0,
                1.0,
                Some(64),
                1.25,
                None,
                false,
                (method == "graph").then_some(&options),
            )
        };
        let verified = report.solved;
        let capped = report.exhausted;
        (
            scalar,
            serde_json::to_value(report).map_err(|e| e.to_string())?,
            verified,
            capped,
            None,
        )
    };
    let replay_before = bitops::snapshot();
    let verified = {
        let _verification = bitops::enter(Phase::Verification);
        native_verified && scalar.is_some_and(|d| d > 0 && d < curve.l && ec.mul(&g, d) == q)
    };
    let replay = bitops::snapshot().since(&replay_before);
    let elapsed = started.elapsed().as_nanos();
    let mut semantic_work = work.clone();
    without_times(&mut semantic_work);
    let mut result = json!({"method":method,"solver_answer_placeholder":0,"recovered_scalar":scalar,
        "graph_strategy":if mst {"mst"}else{"pivot"},
        "verified":verified,"capped":capped,"inner_wall_ns":elapsed,"work":work,
        "signature":{"scalar":scalar,"verified":verified,"capped":capped,"work":semantic_work}});
    if accounting {
        let (tally, phases, pricing) = if let Some(mut t) = rho_work {
            // Native rho calls some hooks internally. Its explicit tally is
            // the sole source for that run; only the outer replay is added.
            t.phase_mut(Phase::RhoVerification).add(&replay.total());
            (
                t,
                &Phase::RHO[..],
                if rho_pricing.native {
                    "executed native Barrett tower prime-field primitives"
                } else {
                    "existing generic Fq3 harness primitive prices"
                },
            )
        } else {
            (
                bitops::snapshot().since(&before),
                &Phase::IC[..],
                "executed modular hooks and observed native events",
            )
        };
        let mut native = accounting_json(&tally, phases, curve, method, pricing);
        let uniforms = bitops::uniform_snapshot()
            .since(&uniform_before.expect("counting snapshot"))
            .expect("monotone sampling counters");
        let sampling_phases:Vec<_>=phases.iter().map(|&phase|json!({"phase":phase.name(),"events":bitops::uniform_json(&uniforms.phases[phase as usize]).expect("sampling counters fit")})).collect();
        native["uniform_sampling"] = json!({"schema":"cover.uniform-sampling-diagnostic/v1","complete":false,
            "scope":"selected Fq-coordinate, Wiedemann-vector and initial cover-label samplers; generator and other paths remain incomplete",
            "phases":sampling_phases,"total":bitops::uniform_json(&uniforms.sum(phases).expect("sampling sum fits")).expect("sampling counters fit")});
        if let Some(calibration) = native_calibration {
            native["native_tower_calibration"] = calibration;
            native["counted_items"] = json!(["counted:native_rho_tower_prime_field_arithmetic"]);
            native["known_uncounted"]
                .as_array_mut()
                .expect("gap list")
                .retain(|v| v["name"] != "native_extension_formula_vs_generic_harness_pricing");
        }
        let other_after = bitops::snapshot_other();
        result["work_native"] = native;
        result["work_native"]["other_modulus_ops"] = json!(Phase::IC
            .iter()
            .map(|&p| other_after[p as usize] - other_before[p as usize])
            .sum::<u64>());
        result["signature"]["work_native"] = result["work_native"].clone();
    }
    Ok(result)
}

fn accounting_json(
    tally: &JvTally,
    phases: &[Phase],
    curve: &PublicCurve,
    method: &str,
    pricing: &str,
) -> Value {
    let widths = bitops::widths(curve.p, curve.l);
    let phase_rows: Vec<_> = phases
        .iter()
        .map(|&phase| {
            let t = tally.phase(phase);
            json!({"phase":phase.name(), "primitives":bitops::prims_json(t),
        "diagnostic_priced_bitops":t.bitops(&widths).expect("known primitives").to_string()})
        })
        .collect();
    let total = tally.sum(phases);
    let gaps = if method == "rho" {
        vec![
            "native_extension_formula_vs_generic_harness_pricing",
            "tuned_reference_bitop_pricing",
            "rho_setup_and_native_bookkeeping_audit",
            "rng_internal_work",
        ]
    } else {
        vec![
            "graph_maps_sets_normalization_scheduling_and_replay_bookkeeping",
            "cover_scalar_rng_and_native_bookkeeping_audit",
            "rng_internal_work",
        ]
    };
    json!({"schema":"cover.graph.accounting-diagnostic/v1", "unit":"bitop/v1",
        "counted_items":if method=="rho" {Vec::<&str>::new()} else {vec!["counted:modular_la_native_scans", "counted:cover_residual_scalar_additions", "counted:selected_modular_uniform_draws"]},
        "complete":false, "total_bitops":Value::Null, "comparison_eligible":false,
        "pricing":pricing, "phases":phase_rows, "observed_primitives":bitops::prims_json(&total),
        "diagnostic_priced_bitops":total.bitops(&widths).expect("known primitives").to_string(),
        "widths":{"field_bits":widths.element_bits(),"r_bits":widths.r_bits},
        "known_uncounted":gaps.into_iter().map(|name| json!({"name":name,"count":Value::Null,"status":"coverage_or_emission_pending"})).collect::<Vec<_>>(),
        "foreign_hook_calls":bitops::foreign(),
        "operand_rechecks":"included where inseparable from existing hooks",
        "excluded":"public domain validation; instance construction; control, allocation and memory under bitop/v1"})
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
        let result = if args.bitops {
            solve_worker_counted(
                &curve,
                method,
                args.walk_seed,
                args.retained_percent,
                args.mst_graph,
                args.native_rho_prices,
            )
        } else {
            solve_worker(
                &curve,
                method,
                args.walk_seed,
                args.retained_percent,
                args.mst_graph,
            )
        }?;
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
        || ((args.validation_only || args.incidence_shared)
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
            "graph_strategy":if args.mst_graph {"mst"}else{"pivot"},
            "sieve_recipe":if args.validation_only || args.incidence_shared {"incidence_shared"}else{"historical_default"}});
            let mut base = base;
            if args.bitops {
                base["operation_accounting"] = json!("incomplete_bitop_diagnostic");
            }
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
                                "--foreground",
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
                        if args.mst_graph {
                            command.arg("--mst-graph");
                        }
                        if args.bitops {
                            command.arg("--bitops");
                        }
                        if args.native_rho_prices {
                            command.arg("--native-rho-prices");
                        }
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
                        if args.validation_only || args.incidence_shared {
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
        // This example is also included by the combined worker's test crate.
        // Fixture construction on parallel test threads must own its hooks.
        bitops::claim();
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
        let mut answer_field = serialized.clone();
        answer_field["q"]["expected_scalar"] = json!(spec.d);
        assert!(serde_json::from_value::<PublicCurve>(answer_field).is_err());
        let got = solve_worker(&curve, "rho", 3, 75, false).unwrap();
        assert_eq!(got["recovered_scalar"], spec.d);
        assert_eq!(got["verified"], true);
        assert_eq!(got["work"]["run"]["correct"], false); // no planted answer supplied
        assert!(got.get("work_native").is_none());
        let diagnostic = solve_worker_counted(&curve, "rho", 3, 75, false, false).unwrap();
        assert_eq!(diagnostic["recovered_scalar"], spec.d);
        assert_eq!(diagnostic["verified"], true);
        let accounting = &diagnostic["work_native"];
        assert_eq!(accounting["complete"], false);
        assert_eq!(accounting["comparison_eligible"], false);
        assert!(accounting["total_bitops"].is_null());
        assert_eq!(accounting["foreign_hook_calls"], 0);
        assert_eq!(accounting["other_modulus_ops"], 0);
        assert!(
            accounting["diagnostic_priced_bitops"]
                .as_str()
                .unwrap()
                .parse::<u128>()
                .unwrap()
                > 0
        );
        assert_eq!(diagnostic["signature"]["work_native"], *accounting);
        let native = solve_worker_counted(&curve, "rho", 3, 75, false, true).unwrap();
        assert_eq!(native["recovered_scalar"], spec.d);
        assert_eq!(native["verified"], true);
        assert_eq!(native["signature"]["work"], diagnostic["signature"]["work"]);
        assert_eq!(
            native["work_native"]["native_tower_calibration"]["inverse"]["field_mul"],
            80
        );
        assert_eq!(
            native["work_native"]["native_tower_calibration"]["inverse"]["field_inv"],
            1
        );
        assert_eq!(native["work_native"]["complete"], false);
        assert_eq!(
            native["work_native"]["counted_items"],
            json!(["counted:native_rho_tower_prime_field_arithmetic"])
        );
        assert!(native["work_native"]["known_uncounted"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v["name"] != "native_extension_formula_vs_generic_harness_pricing"));
    }
}
