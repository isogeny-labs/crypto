//! Bounded, paired N83 K2048 chained-S3 model diagnostic.
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use crypto_lib::cryptanalysis::sat::{RootPropagationResult, SolveResult};
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
const U_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const D_SHA256: &str = "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7";

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn decimal(value: &str) -> Result<BigUint> {
    Ok(BigUint::parse_bytes(value.as_bytes(), 10).ok_or("invalid decimal field element")?)
}

fn cgroup_value(name: &str) -> Result<u64> {
    let membership = fs::read_to_string("/proc/self/cgroup")?;
    let rel = membership
        .lines()
        .find_map(|line| line.strip_prefix("0::"))
        .ok_or("cgroup v2 membership missing")?
        .strip_prefix('/')
        .ok_or("cgroup path is not absolute")?;
    if rel.split('/').any(|part| part == "..") {
        return Err("cgroup parent traversal".into());
    }
    let path = Path::new("/sys/fs/cgroup").join(rel).join(name);
    Ok(fs::read_to_string(path)?.trim().parse()?)
}

fn checked_header(arm: &str, compressed: &[u8]) -> Result<(Vec<BigUint>, Value)> {
    let expected = match arm {
        "U" => U_SHA256,
        "D" => D_SHA256,
        _ => return Err("arm must be U or D".into()),
    };
    if sha256(compressed) != expected {
        return Err("compressed factor-base digest differs".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(compressed));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    let policy = if arm == "U" {
        "public_x_hash"
    } else {
        "direct_subgroup_x_d18_prefix2048"
    };
    if header["curve_slug"] != CURVE
        || header["policy"] != policy
        || header["closure"] != "signed_frobenius"
        || header["orbit_columns"] != 2048
        || header["point_count"] != 339968
        || (arm == "U" && header["seed"] != 2026100801)
    {
        return Err("factor-base header differs from protocol".into());
    }
    let rows = header["representatives"]
        .as_array()
        .ok_or("representatives absent")?;
    if rows.len() != 2048 {
        return Err("representative count differs".into());
    }
    let mut representatives = Vec::with_capacity(rows.len());
    let mut seen = HashSet::with_capacity(rows.len());
    for row in rows {
        let pair = row.as_array().ok_or("representative is not a point")?;
        if pair.len() != 2 {
            return Err("representative point length differs".into());
        }
        let x = decimal(pair[0].as_str().ok_or("representative x is not decimal")?)?;
        if !seen.insert(x.clone()) {
            return Err("duplicate representative x".into());
        }
        representatives.push(x);
    }
    Ok((representatives, header))
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 7 {
        return Err("usage: n83_k2048_m5_model ARM OBJECT CORPUS FIXTURE MODE OUTPUT".into());
    }
    let arm = args[1].as_str();
    let fixture: usize = args[4].parse()?;
    let mode = args[5].as_str();
    if fixture > 1 || !["preflight", "search"].contains(&mode) {
        return Err("fixture or mode outside protocol".into());
    }
    let output = Path::new(&args[6]);
    if output.exists() {
        return Err("output already exists".into());
    }
    if cgroup_value("memory.max")? != 4 * 1024 * 1024 * 1024
        || cgroup_value("memory.swap.max")? != 0
    {
        return Err("cgroup memory/swap limit differs from protocol".into());
    }
    let started = Instant::now();
    let object = fs::read(&args[2])?;
    let (representatives, header) = checked_header(arm, &object)?;
    let corpus_bytes = fs::read(&args[3])?;
    if sha256(&corpus_bytes) != CORPUS_SHA256 {
        return Err("public corpus digest differs".into());
    }
    let corpus: Value = serde_json::from_slice(&corpus_bytes)?;
    let target = corpus["targets"]
        .as_array()
        .ok_or("public targets absent")?
        .iter()
        .find(|row| row["a"] == 0 && row["fixture"] == fixture)
        .ok_or("target fixture absent")?;
    let xy = target["point"].as_array().ok_or("target point absent")?;
    let target_x = decimal(xy[0].as_str().ok_or("target x absent")?)?;
    let target_y = decimal(xy[1].as_str().ok_or("target y absent")?)?;
    let expected = if fixture == 0 {
        ("1834913121178197242856238", "8797964190732846213981377")
    } else {
        ("5809312440517013698248638", "3568442427236746854022671")
    };
    if target_x != decimal(expected.0)? || target_y != decimal(expected.1)? {
        return Err("public point differs from protocol".into());
    }
    let input_ms = started.elapsed().as_secs_f64() * 1000.0;
    let target_x_element = F2mElement::from_hex(&format!("{target_x:x}"), 83);
    let polynomial = IrreduciblePoly {
        degree: 83,
        low_terms: vec![0, 1, 2, 45],
    };
    let model_start = Instant::now();
    let mut encoding = ChainedS3Encoding::build(
        5,
        &polynomial,
        &F2mElement::one(83),
        Some(&target_x_element),
        0,
        300_000,
    )
    .map_err(|_| "chained-S3 model build failed")?;
    encoding
        .constrain_frobenius_orbit_summands(&representatives, 300_000, 8_000_000)
        .map_err(|_| "orbit-domain install failed")?;
    let model_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    let mut root_ms = 0.0;
    let mut search_ms = 0.0;
    let mut root_status = "SKIPPED".to_owned();
    let mut solve_status = "SKIPPED".to_owned();
    let mut model_verified: Option<bool> = None;
    if mode == "search" {
        encoding.solver.conflict_budget = 5_000;
        encoding.solver.deadline = Some(started + Duration::from_secs(240));
        let root_start = Instant::now();
        let root = encoding.solver.propagate_root();
        root_ms = root_start.elapsed().as_secs_f64() * 1000.0;
        root_status = format!("{root:?}");
        let search_start = Instant::now();
        let answer = if root == RootPropagationResult::Unsat {
            SolveResult::Unsat
        } else {
            encoding.solver.solve()
        };
        search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
        solve_status = format!("{answer:?}");
        if answer == SolveResult::Sat {
            let xs = encoding.decode_x_blocks();
            model_verified =
                Some(encoding.verify_model() && xs.len() == 9 && xs[8] == target_x_element);
        }
    }
    let receipt = json!({
        "schema":"n83.k2048-m5-model/v1", "arm":arm, "mode":mode,
        "fixture_index":fixture, "curve_id":"EC1N83Ckb0h252dacc102a4",
        "target_point":[expected.0,expected.1],
        "object_sha256":sha256(&object), "corpus_sha256":CORPUS_SHA256,
        "policy":header["policy"], "columns":2048, "points":339968,
        "representative_x_count":representatives.len(),
        "source_blake3":blake3::hash(include_bytes!("n83_k2048_m5_model.rs")).to_hex().to_string(),
        "binary_sha256":sha256(&fs::read(std::env::current_exe()?)?),
        "input_ms":input_ms,"model_ms":model_ms,"root_ms":root_ms,"search_ms":search_ms,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "memory_cgroup_peak_bytes":cgroup_value("memory.peak")?,
        "sat_variables":encoding.solver.n_vars(),
        "sat_clauses":encoding.solver.n_clauses(),
        "sat_xor_rows":encoding.solver.n_xors(),
        "conflicts":encoding.solver.conflicts(),
        "root_status":root_status,"solve_status":solve_status,
        "model_verified":model_verified,
        "group_relation_verified":false,"rank":Value::Null,
        "candidate_id":Value::Null,"online_ms":Value::Null
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &receipt)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(&receipt)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("n83 K2048 m5 model: {error}");
        std::process::exit(1);
    }
}
