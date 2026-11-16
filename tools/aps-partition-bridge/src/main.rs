use aps_partition_bridge::{
    sampler,
    solver::{self, Mode, ModeReport},
    trace_control, valuation,
};
use crypto_lib::cryptanalysis::aps_single_relation::LabeledBase;
use serde::Serialize;
use std::{fs::OpenOptions, io::Write};

#[derive(Serialize)]
struct Route {
    mode: Mode,
    report: Option<ModeReport>,
    error: Option<String>,
}

#[derive(Serialize)]
struct Sample {
    seed: u64,
    sampler_error: Option<String>,
    labeled_base: Option<LabeledBase>,
    target: Option<[u64; 2]>,
    routes: Vec<Route>,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    source_revision: String,
    source_manifest_sha256: String,
    performance_trial: bool,
    execution_attested: bool,
    cost_complete: bool,
    whole_ecbench_registry_tested: bool,
    poisoned_pool_fallback_tested: bool,
    per_cell_budget_ms: u64,
    fixed_family_size: usize,
    samples: Vec<Sample>,
    coefficient_controls: Option<trace_control::ControlReport>,
    coefficient_control_error: Option<String>,
    valuation_profiles: Vec<valuation::ProfileReport>,
    valuation_profile_errors: Vec<(u64, String)>,
    all_fixed_cases_complete_and_equal: bool,
}

fn hex(s: &str, n: usize) -> bool {
    s.len() == n
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn run() -> Result<bool, String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 8
        || args[1] != "structural"
        || args[2] != "--source-revision"
        || args[4] != "--source-manifest-sha256"
        || args[6] != "--out"
        || !hex(&args[3], 40)
        || !hex(&args[5], 64)
    {
        return Err("usage: aps-partition-bridge structural --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_JSON".into());
    }
    // Reserve the output before any solver call; refuse replacement immediately.
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[7])
        .map_err(|e| e.to_string())?;
    let mut samples = Vec::new();
    let mut correct = true;
    for seed in sampler::SEEDS {
        let sample = match sampler::fixture(seed) {
            Err(error) => {
                correct = false;
                Sample {
                    seed,
                    sampler_error: Some(error),
                    labeled_base: None,
                    target: None,
                    routes: Vec::new(),
                }
            }
            Ok(f) => {
                let mut routes = Vec::new();
                for mode in [Mode::Cold, Mode::Shared, Mode::SignatureBuckets] {
                    let (report, error) = match solver::run(&f, seed, mode) {
                        Ok(report) => {
                            correct &= report.complete_and_equal;
                            (Some(report), None)
                        }
                        Err(error) => {
                            correct = false;
                            (None, Some(error))
                        }
                    };
                    routes.push(Route {
                        mode,
                        report,
                        error,
                    });
                }
                let (x, y) = f.curve.canonical(f.q);
                Sample {
                    seed,
                    sampler_error: None,
                    labeled_base: Some(f.base),
                    target: Some([x, y]),
                    routes,
                }
            }
        };
        samples.push(sample);
    }
    let (control, error) = match trace_control::run() {
        Ok(report) => {
            correct &= report.complete_and_equal;
            (Some(report), None)
        }
        Err(error) => {
            correct = false;
            (None, Some(error))
        }
    };
    let mut profiles = Vec::new();
    let mut profile_errors = Vec::new();
    for seed in sampler::SEEDS {
        match valuation::run(seed) {
            Ok(report) => {
                correct &= report.all_safe_sign_checks_hold;
                profiles.push(report);
            }
            Err(error) => {
                correct = false;
                profile_errors.push((seed, error));
            }
        }
    }
    let report = Report {
        schema: "aps.partition-production-bridge/v1",
        source_revision: args[3].clone(),
        source_manifest_sha256: args[5].clone(),
        performance_trial: false,
        execution_attested: false,
        cost_complete: false,
        whole_ecbench_registry_tested: false,
        poisoned_pool_fallback_tested: false,
        per_cell_budget_ms: solver::CALL_BUDGET_MS,
        fixed_family_size: sampler::FAMILY_SIZE,
        samples,
        coefficient_controls: control,
        coefficient_control_error: error,
        valuation_profiles: profiles,
        valuation_profile_errors: profile_errors,
        all_fixed_cases_complete_and_equal: correct,
    };
    serde_json::to_writer(&mut file, &report).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    println!("production-linked structural report written; no speed qualification");
    Ok(correct)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(2),
        Err(error) => {
            eprintln!("aps-partition-bridge: {error}");
            std::process::exit(1);
        }
    }
}
