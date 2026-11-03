//! Freeze 32 public N83 points for the native K1107 versus K2304 bridge.
//! Usage: n83_bridge_fixture points <points.jsonl> <answers.jsonl>
//!        n83_bridge_fixture candidates <output-dir>

use std::collections::{BTreeSet, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};

const DOMAIN: &[u8] = b"n83-native-small-intermediate-bridge32-20261018-v1";
const SUBGROUP_ORDER: u64 = 8_569_786_107_849_059;
const COUNT: u32 = 32;
const CURRENT_SOURCE: &str = "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829";
const PRIOR_SOURCE: &str = "8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12";
const CURRENT_COMMIT: &str = "c54d94c6e6db4978521b50208ce62bed24b580b4";
const PRIOR: &[(usize, &str)] = &[
    (1107, "research/n83_k992_cold_followup_20261017/candidate_IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h82329aa062fa.json"),
    (2304, "research/n83_intermediate_factor_base_holdout_20261017/candidate_IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0hf7c27ecf3b60.json"),
];

fn write_once(path: &Path, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        if fs::read(path)? != content {
            return Err(format!("sealed file differs: {}", path.display()).into());
        }
        return Ok(());
    }
    let mut stream = OpenOptions::new().write(true).create_new(true).open(path)?;
    stream.write_all(content)?;
    stream.sync_all()?;
    Ok(())
}

fn scalar(index: u32) -> (BigUint, String) {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    hasher.update(&index.to_le_bytes());
    let digest = hasher.finalize();
    let scalar = BigUint::from(1u8)
        + BigUint::from_bytes_le(digest.as_bytes()) % BigUint::from(SUBGROUP_ORDER - 1);
    (scalar, digest.to_hex().to_string())
}

fn run(points_path: &str, answers_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let curve = KoblitzCurve::new(1, 83).ok_or("N83 Koblitz construction failed")?;
    if curve.subgroup_order.to_u64() != Some(SUBGROUP_ORDER)
        || curve.curve.irreducible.low_terms != [0, 2, 4, 7]
    {
        return Err("unexpected curve identity".into());
    }
    let mut points = String::new();
    let mut answers = String::new();
    let mut distinct = HashSet::new();
    for index in 0..COUNT {
        let (secret, digest) = scalar(index);
        let target = curve.mul(curve.generator(), &secret);
        let BinaryPoint::Affine { x, y } = target else {
            return Err(format!("point {index} is infinity").into());
        };
        let coordinates = [x.to_biguint().to_string(), y.to_biguint().to_string()];
        if !distinct.insert(coordinates.clone()) {
            return Err(format!("point {index} repeats").into());
        }
        points.push_str(&format!("{}\n", json!(coordinates)));
        answers.push_str(&format!(
            "{}\n",
            json!({
            "fixture_index":index,"scalar":secret.to_string(),
            "point":coordinates,"seed_blake3":digest})
        ));
    }
    write_once(Path::new(points_path), points.as_bytes())?;
    write_once(Path::new(answers_path), answers.as_bytes())?;
    println!(
        "{}",
        json!({"schema":"n83.native-bridge-fixture/v1",
                             "point_count":COUNT,
                             "domain":String::from_utf8_lossy(DOMAIN)})
    );
    Ok(())
}

fn candidate_id(record: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let mut stripped = record.clone();
    stripped
        .as_object_mut()
        .ok_or("candidate is not an object")?
        .remove("candidate_id");
    let digest = crypto_lib::hash::sha256(&serde_json::to_vec(&stripped)?);
    let points = record["factor_base"]["actual_usable_points"]
        .as_u64()
        .ok_or("actual base size absent")?;
    Ok(format!(
        "IC1N83Ckb1a1fb{points}PDP4rootRCguidedLAgaussTDdirectISO0h{}",
        &hex::encode(digest)[..12]
    ))
}

fn candidates(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(out_dir)?;
    for &(k, source) in PRIOR {
        let mut record: Value = serde_json::from_slice(&fs::read(source)?)?;
        if record["candidate_id"] != candidate_id(&record)?
            || record["factor_base"]["actual_usable_points"] != 166 * k
            || record["factor_base"]["effective_columns"] != k
            || record["implementation"]["source_sha256"]["ic"] != PRIOR_SOURCE
        {
            return Err(format!("K{k}: prior manifest differs").into());
        }
        record["implementation"]["source_commit"] = json!(CURRENT_COMMIT);
        record["implementation"]["source_sha256"]["ic"] = json!(CURRENT_SOURCE);
        for (section, key) in [
            ("point_decomposition", "solver_source_sha256"),
            ("relation_collection", "source_digest"),
            ("relation_linear_algebra", "source_digest"),
            ("target_descent", "source_digest"),
        ] {
            if record[section][key] != PRIOR_SOURCE {
                return Err(format!("K{k}: {section} source differs").into());
            }
            record[section][key] = json!(CURRENT_SOURCE);
        }
        for (section, key) in [
            ("point_decomposition", "variant"),
            ("target_descent", "policy"),
        ] {
            let original = record[section][key].as_str().ok_or("variant absent")?;
            record[section][key] = json!(format!("{original}_two_worker_cancel"));
        }
        let flags = &mut record["implementation"]["algorithm_flags"];
        flags["target_threads"] = json!(2);
        flags["target_cancel"] = json!(true);
        flags["indexed5_id30_request"] = json!(false);
        let id = candidate_id(&record)?;
        record["candidate_id"] = json!(id);
        let mut bytes = serde_json::to_vec_pretty(&record)?;
        bytes.push(b'\n');
        write_once(&out_dir.join(format!("candidate_{id}.json")), &bytes)?;
        println!("K{k} {id}");
    }
    Ok(())
}

fn encoded_points(path: &Path) -> Result<BTreeSet<[String; 2]>, Box<dyn std::error::Error>> {
    let mut result = BTreeSet::new();
    for line in fs::read_to_string(path)?.lines() {
        let Ok(values) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        let Some(values) = values.as_array() else {
            continue;
        };
        if values.len() != 2 {
            continue;
        }
        let pair = values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .or_else(|| value.as_u64().map(|n| n.to_string()))
            })
            .collect::<Option<Vec<_>>>();
        if let Some(pair) = pair {
            result.insert([pair[0].clone(), pair[1].clone()]);
        }
    }
    Ok(result)
}

fn audit(
    target: &Path,
    external_finalists: &Path,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = target.canonicalize()?;
    let new = encoded_points(&target)?;
    if new.len() != COUNT as usize {
        return Err("32 distinct fresh points required".into());
    }
    let mut paths = Vec::new();
    for directory in fs::read_dir("research")? {
        let directory = directory?;
        if !directory.file_type()?.is_dir()
            || !directory.file_name().to_string_lossy().starts_with("n83")
        {
            continue;
        }
        for entry in fs::read_dir(directory.path())? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.file_type()?.is_file()
                && name.contains("target")
                && name.ends_with(".jsonl")
                && entry.path().canonicalize()? != target
            {
                paths.push(entry.path());
            }
        }
    }
    paths.sort();
    let mut prior = BTreeSet::new();
    let mut receipts = Vec::new();
    for path in paths {
        let points = encoded_points(&path)?;
        prior.extend(points.iter().cloned());
        receipts.push(
            json!({"path":path,"sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(&path)?)),
                             "point_count":points.len()}),
        );
    }
    let finalist = encoded_points(external_finalists)?;
    let prior_overlap = new.intersection(&prior).count();
    let finalist_overlap = new.intersection(&finalist).count();
    let result = json!({
        "schema":"n83.native-bridge-exclusion/v1",
        "status":if prior_overlap==0 && finalist_overlap==0 {"PASS_ZERO_OVERLAP"} else {"OVERLAP_FOUND"},
        "fresh_targets_sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(&target)?)),
        "fresh_count":new.len(),"prior_unique_count":prior.len(),
        "prior_overlap_count":prior_overlap,"prior_files":receipts,
        "finalists_sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(external_finalists)?)),
        "finalists_count":finalist.len(),"finalists_overlap_count":finalist_overlap,
    });
    let mut bytes = serde_json::to_vec_pretty(&result)?;
    bytes.push(b'\n');
    write_once(output, &bytes)?;
    println!("{result}");
    if prior_overlap != 0 || finalist_overlap != 0 {
        return Err("fresh target overlap".into());
    }
    Ok(())
}

fn seal(directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let targets = directory.join("targets.jsonl");
    let answers = directory.join("answers.jsonl");
    let workloads = directory.join("workloads.json");
    let exclusion = directory.join("exclusion.json");
    let exclusion_data: Value = serde_json::from_slice(&fs::read(&exclusion)?)?;
    if exclusion_data["status"] != "PASS_ZERO_OVERLAP"
        || exclusion_data["fresh_count"] != COUNT
        || exclusion_data["fresh_targets_sha256"]
            != hex::encode(crypto_lib::hash::sha256(&fs::read(&targets)?))
    {
        return Err("fresh-point exclusion audit differs".into());
    }
    let mut candidates = Vec::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("candidate_IC1N83") && name.ends_with(".json"))
        {
            let manifest: Value = serde_json::from_slice(&fs::read(&path)?)?;
            if manifest["candidate_id"] != candidate_id(&manifest)?
                || manifest["implementation"]["source_sha256"]["ic"] != CURRENT_SOURCE
            {
                return Err("candidate identity or source differs".into());
            }
            candidates.push((manifest, path));
        }
    }
    if candidates.len() != 2 {
        return Err("exactly two candidate manifests required".into());
    }
    let mut by_k = serde_json::Map::new();
    let mut candidate_hashes = serde_json::Map::new();
    for (manifest, path) in candidates {
        let k = manifest["factor_base"]["effective_columns"]
            .as_u64()
            .ok_or("candidate K absent")?;
        let key = format!("candidate_k{k}");
        if k != 1107 && k != 2304 || by_k.contains_key(&key) {
            return Err("candidate K differs".into());
        }
        by_k.insert(key, manifest["candidate_id"].clone());
        candidate_hashes.insert(
            path.file_name().unwrap().to_string_lossy().into_owned(),
            json!(hex::encode(crypto_lib::hash::sha256(&fs::read(&path)?))),
        );
    }
    let source = Path::new("examples/koblitz_orbit_dlp_fast.rs");
    if hex::encode(crypto_lib::hash::sha256(&fs::read(source)?)) != CURRENT_SOURCE {
        return Err("executed IC source differs".into());
    }
    let frozen = [
        "examples/n83_bridge_fixture.rs",
        "examples/n83_bridge_verify.rs",
        "examples/n83_bridge_run.rs",
        "examples/n83_bridge_analyze.rs",
        "examples/n83_bridge_select.rs",
        "examples/n83_bridge_archive.rs",
        "examples/n83_rho_optimization_eval.rs",
        "Cargo.lock",
        ".github/workflows/n83-small-intermediate-bridge.yml",
        "research/n83_small_intermediate_bridge_20261018/PROTOCOL.md",
    ];
    let mut source_hashes = serde_json::Map::new();
    for name in frozen {
        source_hashes.insert(
            name.into(),
            json!(hex::encode(crypto_lib::hash::sha256(&fs::read(name)?))),
        );
    }
    let mut record = json!({
        "schema":"n83.native-bridge-protocol/v1",
        "status":"SEALED_UNMEASURED",
        "source_sha256":CURRENT_SOURCE,
        "targets_sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(&targets)?)),
        "answers_sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(&answers)?)),
        "workloads_sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(&workloads)?)),
        "exclusion_sha256":hex::encode(crypto_lib::hash::sha256(&fs::read(&exclusion)?)),
        "calibration_target_sha256":hex::encode(crypto_lib::hash::sha256(
            &fs::read("research/n83_cold_factor_base_20261016/targets.jsonl")?)),
        "rho_config_sha256":hex::encode(crypto_lib::hash::sha256(
            &fs::read("research/n83_dense_root_20261013/rho_config.json")?)),
        "source_hashes":source_hashes,"candidate_manifest_sha256":candidate_hashes,
        "point_count":COUNT,"host_strata":4,"points_per_host":8,
        "calibration_repetitions_per_side":3,"calibration_max_relative_spread":"0.10",
        "ic_cap_seconds":900,"rho_cap_seconds":120,"producer_cap_seconds":16200,
        "rss_cap_bytes":61u64*1024*1024*1024/4,
    });
    let object = record.as_object_mut().unwrap();
    object.extend(by_k);
    let mut bytes = serde_json::to_vec_pretty(&record)?;
    bytes.push(b'\n');
    write_once(&directory.join("protocol.json"), &bytes)?;
    println!(
        "{}",
        json!({"status":"SEALED","protocol_sha256":hex::encode(
        crypto_lib::hash::sha256(&bytes))})
    );
    Ok(())
}

fn workloads(directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let targets = fs::read_to_string(directory.join("targets.jsonl"))?;
    let answers = fs::read_to_string(directory.join("answers.jsonl"))?;
    let targets: Vec<&str> = targets.lines().collect();
    let answers: Vec<&str> = answers.lines().collect();
    if targets.len() != COUNT as usize || answers.len() != COUNT as usize {
        return Err("32 target workloads required".into());
    }
    let mut rows = Vec::new();
    for index in 0..COUNT as usize {
        let target: Value = serde_json::from_str(targets[index])?;
        let answer: Value = serde_json::from_str(answers[index])?;
        if answer["fixture_index"] != index || answer["point"] != target {
            return Err(format!("target workload {index} differs").into());
        }
        let record = json!({
            "curve_id":"EC1N83Ckb1a1haacb69ee9f37",
            "public_target_q":target,
            "target_count":1,
            "input_law":"BLAKE3_domain_index_scalar_fixture",
            "fixture_domain":String::from_utf8_lossy(DOMAIN),
            "fixture_index":index,
            "seed_blake3":answer["seed_blake3"],
            "online_cache_state":"prepared_target_independent",
            "cold_cache_state":"empty_fresh_process",
        });
        let digest = crypto_lib::hash::sha256(&serde_json::to_vec(&record)?);
        rows.push(json!({"workload_id":hex::encode(digest)[..12].to_owned(),
                         "canonical_record":record}));
    }
    let mut bytes = serde_json::to_vec_pretty(&json!({
        "schema":"n83.native-bridge-workloads/v1","rows":rows
    }))?;
    bytes.push(b'\n');
    write_once(&directory.join("workloads.json"), &bytes)?;
    println!("{}", json!({"status":"SEALED_WORKLOADS","count":COUNT}));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, mode, points, answers] if mode == "points" => run(points, answers),
        [_, mode, out] if mode == "candidates" => candidates(Path::new(out)),
        [_, mode, target, finalists, output] if mode == "audit" =>
            audit(Path::new(target), Path::new(finalists), Path::new(output)),
        [_, mode, directory] if mode == "seal" => seal(Path::new(directory)),
        [_, mode, directory] if mode == "workloads" => workloads(Path::new(directory)),
        _ => Err("usage: n83_bridge_fixture points <points.jsonl> <answers.jsonl> | candidates <output-dir>".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seed_scalars_are_in_the_subgroup_range() {
        for index in 0..COUNT {
            let (value, _) = scalar(index);
            assert!(value >= BigUint::from(1u8));
            assert!(value < BigUint::from(SUBGROUP_ORDER));
        }
    }
}
