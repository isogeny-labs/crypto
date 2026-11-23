use aps_partition_bridge::{
    quartic_solver::{self, Route},
    sampler,
    solver::CALL_BUDGET_MS,
};
use crypto_lib::cryptanalysis::aps_single_relation::LabeledBase;
use serde::Serialize;
use std::{fs::OpenOptions, io::Write};

#[derive(Serialize)]
struct Sample {
    seed: u64,
    labeled_base: Option<LabeledBase>,
    target: Option<[u64; 2]>,
    error: Option<String>,
    cold: Option<Route>,
    shared: Option<Route>,
    route_errors: Vec<String>,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    source_revision: String,
    source_manifest_sha256: String,
    performance_trial: bool,
    cost_complete: bool,
    execution_attested: bool,
    target_count: usize,
    encoding: &'static str,
    per_cell_budget_ms: u64,
    all_fixed_cases_complete_and_equal: bool,
    samples: Vec<Sample>,
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
        return Err("usage: quartic structural --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_JSON".into());
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[7])
        .map_err(|e| e.to_string())?;
    let mut correct = true;
    let mut samples = Vec::new();
    for seed in sampler::SEEDS {
        let mut sample = Sample {
            seed,
            labeled_base: None,
            target: None,
            error: None,
            cold: None,
            shared: None,
            route_errors: Vec::new(),
        };
        match sampler::even_fixture(seed) {
            Err(error) => {
                correct = false;
                sample.error = Some(error);
            }
            Ok(f) => {
                sample.labeled_base = Some(f.base.clone());
                let (x, y) = f.curve.canonical(f.q);
                sample.target = Some([x, y]);
                for shared in [false, true] {
                    match quartic_solver::run(&f, seed, shared) {
                        Ok(route) => {
                            correct &= route.complete_and_equal;
                            if shared {
                                sample.shared = Some(route);
                            } else {
                                sample.cold = Some(route);
                            }
                        }
                        Err(error) => {
                            correct = false;
                            sample
                                .route_errors
                                .push(format!("shared={shared}: {error}"));
                        }
                    }
                }
            }
        }
        samples.push(sample);
    }
    let report = Report {
        schema: "aps.quartic-label-cells/v1",
        source_revision: args[3].clone(),
        source_manifest_sha256: args[5].clone(),
        performance_trial: false,
        cost_complete: false,
        execution_attested: false,
        target_count: 1,
        encoding: "expanded4",
        per_cell_budget_ms: CALL_BUDGET_MS,
        all_fixed_cases_complete_and_equal: correct,
        samples,
    };
    serde_json::to_writer(&mut file, &report).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    Ok(correct)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(2),
        Err(error) => {
            eprintln!("quartic: {error}");
            std::process::exit(1);
        }
    }
}
