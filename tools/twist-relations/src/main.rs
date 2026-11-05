use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use twist_relations::{benchmark, evidence, exhaustive, valid_prime};

const HELP: &str = "twist-relations --suite smoke|benchmark|exhaustive|all --output-dir DIR\n\
  [--seeds N] [--seed-start N] [--primes P ...] [--exhaustive-primes P ...]\n\
  [--max-attempts-per-solve N] (tightens the 1000*n relation limit)\n\
twist-relations verify FILE.json\n\
Benchmark fields: prime, 7 <= p <= 1000003, p = 3 mod 4.\n\
Exhaustive fields: same conditions with p <= 127.\n\
Use fresh output directories. Counts are diagnostic; no speedup is inferred.";

#[derive(Clone, Debug, Serialize)]
struct Config {
    suite: String,
    seeds: u64,
    seed_start: u64,
    primes: Vec<u64>,
    exhaustive_primes: Vec<u64>,
    output_dir: PathBuf,
    max_attempts_per_solve: Option<u64>,
}
fn parse(args: &[String]) -> Result<Config, String> {
    let (mut suite, mut seeds, mut seed_start, mut primes, mut ex_primes, mut out) =
        ("smoke".to_string(), None, 0, None, None, None);
    let mut i = 0;
    let mut max_attempts_per_solve = None;
    while i < args.len() {
        let flag = &args[i];
        i += 1;
        if i == args.len() {
            return Err(format!("Missing value for {flag}"));
        }
        let value = &args[i];
        match flag.as_str() {
            "--suite" => {
                suite = value.clone();
                i += 1;
            }
            "--output-dir" => {
                out = Some(PathBuf::from(value));
                i += 1;
            }
            "--seeds" => {
                seeds = Some(value.parse::<u64>().map_err(|_| "Invalid seed count")?);
                i += 1;
            }
            "--seed-start" => {
                seed_start = value.parse::<u64>().map_err(|_| "Invalid seed start")?;
                i += 1;
            }
            "--max-attempts-per-solve" => {
                let limit = value.parse::<u64>().map_err(|_| "Invalid attempt limit")?;
                if limit == 0 {
                    return Err("Attempt limit must be positive".into());
                }
                max_attempts_per_solve = Some(limit);
                i += 1;
            }
            "--primes" | "--exhaustive-primes" => {
                let mut values = Vec::new();
                while i < args.len() && !args[i].starts_with("--") {
                    values.push(args[i].parse::<u64>().map_err(|_| "Invalid prime")?);
                    i += 1;
                }
                if values.is_empty() {
                    return Err("Empty prime list".into());
                }
                if flag == "--primes" {
                    primes = Some(values);
                } else {
                    ex_primes = Some(values);
                }
            }
            _ => return Err(format!("Unknown flag {flag}")),
        }
    }
    if !["smoke", "benchmark", "exhaustive", "all"].contains(&suite.as_str()) {
        return Err("Unknown suite".into());
    }
    let smoke = suite == "smoke";
    let config = Config {
        suite,
        seeds: seeds.unwrap_or(if smoke { 2 } else { 30 }),
        seed_start,
        primes: primes.unwrap_or(if smoke {
            vec![43, 67]
        } else {
            vec![1019, 4091, 16319]
        }),
        exhaustive_primes: ex_primes.unwrap_or(if smoke {
            vec![7, 11]
        } else {
            vec![7, 11, 19, 31, 43]
        }),
        output_dir: out.ok_or("--output-dir is required")?,
        max_attempts_per_solve,
    };
    if config.seeds == 0
        || config.seeds > 10000
        || config.seed_start.checked_add(config.seeds).is_none()
    {
        return Err("Seed count must be 1..10000 and the range must not overflow".into());
    }
    for (values, max) in [
        (&config.primes, 1_000_003),
        (&config.exhaustive_primes, 127),
    ] {
        let mut unique = std::collections::HashSet::new();
        if values
            .iter()
            .any(|&p| !valid_prime(p, max) || !unique.insert(p))
        {
            return Err("Invalid or duplicate field prime".into());
        }
    }
    if config.output_dir.exists()
        && (!config.output_dir.is_dir()
            || fs::read_dir(&config.output_dir)
                .map_err(|e| e.to_string())?
                .next()
                .is_some())
    {
        return Err("Output directory must be new or empty".into());
    }
    Ok(config)
}

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    evidence::write_json_atomic(path, value)
}
fn source() -> serde_json::Value {
    let mut files = serde_json::Map::new();
    for (name, text) in [
        ("core.rs", include_str!("core.rs")),
        ("benchmark.rs", include_str!("benchmark.rs")),
        ("evidence.rs", include_str!("evidence.rs")),
        ("exhaustive.rs", include_str!("exhaustive.rs")),
        ("tr04.rs", include_str!("tr04.rs")),
        ("tr04_select.rs", include_str!("tr04_select.rs")),
        ("tr04_relation.rs", include_str!("tr04_relation.rs")),
        ("tr04_verify.rs", include_str!("tr04_verify.rs")),
        ("tr04_protocol.rs", include_str!("tr04_protocol.rs")),
        ("tr04_report.rs", include_str!("tr04_report.rs")),
        ("lib.rs", include_str!("lib.rs")),
        ("main.rs", include_str!("main.rs")),
        ("Cargo.toml", include_str!("../Cargo.toml")),
        ("Cargo.lock", include_str!("../Cargo.lock")),
    ] {
        files.insert(name.into(), json!(sha(text.as_bytes())));
    }
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let binary_hash = env::current_exe()
        .ok()
        .and_then(|p| fs::read(p).ok())
        .map(|b| sha(&b));
    json!({"compiled_source_sha256":files,"binary_sha256":binary_hash,
        "working_tree_revision_at_execution":revision,
        "declared_build_revision":option_env!("TWIST_SOURCE_REVISION"),
        "os":env::consts::OS,"arch":env::consts::ARCH,
        "rng":"splitmix64-rejection-v1; public synthetic fixtures only",
        "historical_python_rng_compatible":false})
}

#[derive(Serialize, Deserialize)]
struct BenchmarkFile {
    schema: String,
    records: Vec<benchmark::Record>,
}

fn execute(config: Config) -> Result<(), String> {
    fs::create_dir_all(&config.output_dir).map_err(|e| e.to_string())?;
    let out = &config.output_dir;
    let mut manifest = json!({"schema":"twist-relations/manifest/v1","status":"running",
        "started_unix_utc":timestamp(),"configuration":config,"source":source(),"artifacts":[]});
    write_json(&out.join("manifest.json"), &manifest)?;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<(), String> {
            if ["smoke", "benchmark", "all"].contains(&config.suite.as_str()) {
                let mut evidence = evidence::Writer::new(out)?;
                let records = benchmark::run_observed(
                    &config.primes,
                    config.seeds,
                    config.seed_start,
                    benchmark::Options {
                        max_attempts: config.max_attempts_per_solve,
                    },
                    |event| evidence.record(event),
                )
                .map_err(|failure| {
                    let detail =
                        serde_json::to_string(&failure).unwrap_or_else(|_| failure.error.clone());
                    if let Err(error) = write_json(&out.join("benchmark-failure.json"), &failure) {
                        return format!("{detail}; cannot publish failure file: {error}");
                    }
                    detail
                })?;
                let mut report = String::from("# Native toy-curve diagnostics\n\nThese are instrumented diagnostics, not an admitted performance comparison.\nNo matched rho, A/A, L2 host admission, or external-tool comparison is supplied.\n\n| p | n | seed | method | total ns | group-add calls | character tests | table entries |\n|---:|---:|---:|:---|---:|---:|---:|---:|\n");
                for r in &records {
                    report.push_str(&format!(
                        "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                        r.parameters.p,
                        r.group_order,
                        r.seed,
                        r.method,
                        r.total_ns,
                        r.counts.group_add_calls,
                        r.counts.character_tests,
                        r.table_entries
                    ));
                }
                report.push_str("\nPublic curve discovery and input-target generation are excluded equally. All method-specific setup, joint base-log recovery, and verification are charged. In-solve evidence publication is included in total_ns and separated as evidence_io_ns; final-record and aggregate exports are outside that interval. See JSON for exact points, recovered scalars, and phase counts. Timings are native; historical Python timings are not comparable.\n");
                write_json(
                    &out.join("benchmark.json"),
                    &BenchmarkFile {
                        schema: "twist-relations/benchmark/v1".into(),
                        records,
                    },
                )?;
                fs::write(out.join("BENCHMARK.md"), report).map_err(|e| e.to_string())?;
            }
            if ["smoke", "exhaustive", "all"].contains(&config.suite.as_str()) {
                let audit = exhaustive::run(&config.exhaustive_primes);
                let bases: u64 = audit.curves.iter().map(|c| c.factor_bases).sum();
                let triples: u64 = audit.curves.iter().map(|c| c.f3_triples).sum();
                let mixed: u64 = audit.curves.iter().map(|c| c.f3_mixed_zero_count).sum();
                fs::write(out.join("EXHAUSTIVE.md"),format!("# Exact finite-scope audit\n\nFields: {:?}. Every nonsingular coefficient model with prime group order at least seven was inspected.\n\nModels: {}. Three-x bases: {bases}. Finite x triples: {triples}. Mixed-character S3 zeros: {mixed}.\n\nAll S3 zero sets were independently cross-checked against signed curve/twist additions. Coefficient models can be isomorphic. Oracle log labels are used only for exhaustive analysis, never by the benchmark. This finite audit makes no general speedup or impossibility claim. Full histograms and witness bases are in exhaustive.json.\n",audit.primes,audit.curves.len())).map_err(|e| e.to_string())?;
                write_json(&out.join("exhaustive.json"), &audit)?;
            }
            Ok(())
        },
    ));
    let result = match result {
        Ok(result) => result,
        Err(_) => Err("Algebraic verification or declared search budget failed; see stderr".into()),
    };
    let mut artifacts = Vec::new();
    for name in [
        "benchmark.json",
        "BENCHMARK.md",
        "exhaustive.json",
        "EXHAUSTIVE.md",
        "benchmark-cases.jsonl",
        "active-case.json",
        "benchmark-failure.json",
    ] {
        if let Ok(bytes) = fs::read(out.join(name)) {
            artifacts.push(json!({"path":name,"sha256":sha(&bytes)}));
        }
    }
    if let Ok(entries) = fs::read_dir(out.join("cases")) {
        let mut paths: Vec<_> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "json"))
            .collect();
        paths.sort();
        for path in paths {
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            artifacts.push(json!({"path":format!("cases/{}",path.file_name().unwrap().to_string_lossy()),"sha256":sha(&bytes)}));
        }
    }
    manifest["artifacts"] = json!(artifacts);
    manifest["finished_unix_utc"] = json!(timestamp());
    manifest["status"] = json!(if result.is_ok() { "passed" } else { "failed" });
    if let Err(error) = &result {
        manifest["error"] = json!(error);
    }
    write_json(&out.join("manifest.json"), &manifest)?;
    result
}

fn verify(path: &Path) -> Result<(), String> {
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    match value["schema"].as_str() {
        Some("twist-relations/benchmark/v1") => {
            let file: BenchmarkFile = serde_json::from_value(value).map_err(|e| e.to_string())?;
            if file.records.is_empty() {
                return Err("Empty benchmark artifact".into());
            }
            benchmark::verify(&file.records)
        }
        Some("twist-relations/exhaustive/v1") => {
            let audit: exhaustive::Audit =
                serde_json::from_value(value).map_err(|e| e.to_string())?;
            if audit.primes.is_empty() || audit.primes.iter().any(|&p| !valid_prime(p, 127)) {
                return Err("Invalid audit scope".into());
            }
            if exhaustive::run(&audit.primes) != audit {
                return Err("Exhaustive replay mismatch".into());
            }
            Ok(())
        }
        _ => Err("Unsupported artifact schema".into()),
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{HELP}");
        return;
    }
    let result = if args.first().is_some_and(|s| s == "verify") {
        if args.len() != 2 {
            Err("verify takes exactly one artifact path".into())
        } else {
            verify(Path::new(&args[1]))
        }
    } else {
        parse(&args).and_then(execute)
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_scopes_and_seed_ranges() {
        for flags in [
            vec!["--primes", "15"],
            vec!["--primes", "13"],
            vec!["--seeds", "0"],
            vec!["--exhaustive-primes", "7", "7"],
            vec!["--suite", "unknown"],
            vec!["--exhaustive-primes", "131"],
        ] {
            let mut args = vec![
                "--output-dir".to_string(),
                "nonexistent-twist-test-output".into(),
            ];
            args.extend(flags.into_iter().map(String::from));
            assert!(parse(&args).is_err());
        }
    }
    #[test]
    fn records_seed_range_without_reading_process_environment() {
        let args = [
            "--output-dir",
            "nonexistent-twist-test-output",
            "--seeds",
            "3",
            "--seed-start",
            "42",
        ]
        .map(String::from);
        let config = parse(&args).unwrap();
        assert_eq!((config.seed_start, config.seeds), (42, 3));
    }
}
