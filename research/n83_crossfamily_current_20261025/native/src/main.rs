//! Native sealed-input gate for the N83 current-source cross-family panel.
//! The producer and independent replay extend this gate before cloud dispatch.
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
mod aws;
mod bundle;
mod collect;
#[cfg(target_os = "linux")]
mod provider;
#[cfg(target_os = "linux")]
mod replay;
#[cfg(target_os = "linux")]
mod runner;
mod select;
mod sigv4;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const ORDER: u64 = 8_569_786_107_849_059;
const DOMAIN: &str = "n83-crossfamily-current16-20261025-v1";
const CURVE_ID: &str = "EC1N83Ckb1a1haacb69ee9f37";
const SOURCE_SHA256: &str = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8";
const FROZEN_SOURCE: [(&str, &str); 6] = [
    ("examples/koblitz_orbit_dlp_fast.rs", SOURCE_SHA256),
    (
        "Cargo.lock",
        "ac5cd7a3459be776180afe1eb2975c76b1a88f84bd1e99aa20475927ec260a90",
    ),
    (
        "research/n83_indexed5_20261013/primary_target.jsonl",
        "21c8dfc7d488909e5c4a42c839fcc01bdefa9821c11ba001626a1d15697240e8",
    ),
    (
        "research/n83_indexed5_20261013/answer.jsonl",
        "651eb7d0ef08a1655036746311b19b6989e340ca4752b6d675606055e59c12a1",
    ),
    (
        "research/n83_dense_root_20261013/rho_config.json",
        "cf76ebb5372acfc1355b88374f875980be66cdffba1a82cb6e4c2f28bcfa723b",
    ),
    (
        "examples/n83_rho_optimization_eval.rs",
        "fb14072f4fa5d08e18d8b79130b6d58a220adae0f76369cc9be949691c83eaa0",
    ),
];
const FILES: [(&str, &str); 4] = [
    (
        "targets.jsonl",
        "fd56a59079fc6cded14539320e0cc7ab88deda0f76fa4fbaf6b663c0cd7ac1b6",
    ),
    (
        "answers.jsonl",
        "82f2cfdf0cf045f356e88a2c2347525e726f151ba27b0a693cfa08a6fe6312ee",
    ),
    (
        "workloads.jsonl",
        "613cb7b784f49f861021067b26683cbff74bf4402936202997b790812e328fe8",
    ),
    (
        "exclusion_audit.json",
        "dbece6cd0fddadd8de5d7143decfebc7856c3b602b2094430c990bac792ba78a",
    ),
];

#[derive(Clone, Copy)]
struct Arm {
    name: &'static str,
    k: usize,
    layout: &'static str,
    id: &'static str,
    manifest_dir: &'static str,
}
const ARMS: [Arm; 4] = [
    Arm {
        name: "k3300_wide3",
        k: 3300,
        layout: "root11_wide3",
        id: "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0hd2459867be83",
        manifest_dir: "n83_k3300_current_source_prefix_20261025",
    },
    Arm {
        name: "k3450_wide3",
        k: 3450,
        layout: "root11_wide3",
        id: "IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0h622efa367d76",
        manifest_dir: "n83_crossfamily_current_20261025",
    },
    Arm {
        name: "k3500_root4",
        k: 3500,
        layout: "root4_standard",
        id: "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h0de90f01951d",
        manifest_dir: "n83_crossfamily_current_20261025",
    },
    Arm {
        name: "k3596_root4",
        k: 3596,
        layout: "root4_standard",
        id: "IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81",
        manifest_dir: "n83_k3596_current_source_full_20261025",
    },
];

fn hash(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn jsonl(path: &Path) -> Result<Vec<Value>> {
    let bytes = fs::read(path)?;
    if !bytes.ends_with(b"\n") {
        return Err(format!("{} missing terminal newline", path.display()).into());
    }
    bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Ok(serde_json::from_slice(line)?))
        .collect()
}
fn scalar_from_seed(index: usize) -> (String, u64) {
    let mut bytes = DOMAIN.as_bytes().to_vec();
    bytes.extend_from_slice(&(index as u32).to_le_bytes());
    let digest = Sha256::digest(&bytes);
    let scalar = 1
        + (BigUint::from_bytes_le(&digest) % BigUint::from(ORDER - 1))
            .to_u64()
            .unwrap();
    (hex::encode(digest), scalar)
}
fn point(value: &Value) -> Result<BinaryPoint> {
    let pair = value.as_array().ok_or("point is not an array")?;
    if pair.len() != 2 {
        return Err("point has wrong arity".into());
    }
    let mut coords = Vec::new();
    for item in pair {
        let word = BigUint::parse_bytes(
            item.as_str().ok_or("coordinate is not text")?.as_bytes(),
            10,
        )
        .ok_or("invalid decimal coordinate")?;
        if word >= (BigUint::from(1u8) << 83) {
            return Err("noncanonical coordinate".into());
        }
        coords.push(word);
    }
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_biguint(&coords[0], 83),
        y: F2mElement::from_biguint(&coords[1], 83),
    })
}
fn canonical(value: &Value) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(value)?)
}
fn source(root: &Path) -> Result<()> {
    for (name, expected) in FROZEN_SOURCE {
        if hash(&fs::read(root.join(name))?) != expected {
            return Err(format!("frozen source/input hash differs: {name}").into());
        }
    }
    Ok(())
}
fn fixture(root: &Path) -> Result<Value> {
    let here = root.join("research/n83_crossfamily_current_20261025");
    let manifest = read(&here.join("fixture_manifest.json"))?;
    if manifest["schema"] != "n83.crossfamily-current16-fixture/v1"
        || manifest["curve_id"] != CURVE_ID
        || manifest["domain"] != DOMAIN
        || manifest["point_count"] != 16
        || manifest["subgroup_order"] != ORDER
        || manifest["status"] != "SEALED_UNMEASURED"
        || hash(&fs::read(here.join("generate_fixture.py"))?) != manifest["generator_sha256"]
    {
        return Err("sealed fixture manifest differs".into());
    }
    for (name, expected) in FILES {
        let digest = hash(&fs::read(here.join(name))?);
        if digest != expected || manifest["artifacts_sha256"][name] != expected {
            return Err(format!("sealed input differs: {name}").into());
        }
    }
    let audit = read(&here.join("exclusion_audit.json"))?;
    if audit["status"] != "PASS_ZERO_OVERLAP"
        || audit["overlap_count"] != 0
        || audit["duplicate_count"] != 0
    {
        return Err("exclusion audit differs".into());
    }
    let targets = jsonl(&here.join("targets.jsonl"))?;
    let answers = jsonl(&here.join("answers.jsonl"))?;
    let workloads = jsonl(&here.join("workloads.jsonl"))?;
    if targets.len() != 16
        || answers.len() != 16
        || workloads.len() != 16
        || manifest["workload_ids"].as_array().map(Vec::len) != Some(16)
    {
        return Err("fixture cardinality differs".into());
    }
    let curve = KoblitzCurve::new(1, 83).ok_or("N83 curve unavailable")?;
    if curve.subgroup_order.to_u64() != Some(ORDER)
        || curve.curve.irreducible.low_terms != [0, 2, 4, 7]
    {
        return Err("curve identity differs".into());
    }
    let mut unique = BTreeSet::new();
    for index in 0..16 {
        let target = point(&targets[index])?;
        let (seed, scalar) = scalar_from_seed(index);
        let expected_scalar = scalar.to_string();
        let line = format!(
            "{}\n",
            canonical(&targets[index]).and_then(|b| Ok(String::from_utf8(b)?))?
        );
        let mut workload = workloads[index].clone();
        let workload_id = workload
            .as_object_mut()
            .ok_or("workload is not an object")?
            .remove("workload_id")
            .ok_or("workload ID absent")?;
        let expected_id = &hash(&canonical(&workload)?)[..12];
        if answers[index]["fixture_index"] != index
            || answers[index]["seed_sha256"] != seed
            || answers[index]["scalar"].as_str() != Some(expected_scalar.as_str())
            || workloads[index]["point_index"] != index
            || workloads[index]["curve_id"] != CURVE_ID
            || workloads[index]["kind"] != "primary_single_target"
            || workloads[index]["target_count"] != 1
            || workloads[index]["target_point"] != targets[index]
            || workloads[index]["target_domain"] != DOMAIN
            || workloads[index]["target_generation_seed_sha256"] != seed
            || workloads[index]["target_stream_sha256"] != hash(line.as_bytes())
            || workload_id != expected_id
            || manifest["workload_ids"][index] != workload_id
            || curve.mul(curve.generator(), &BigUint::from(scalar)) != target
            || curve.mul(&target, &curve.subgroup_order) != BinaryPoint::Infinity
            || !unique.insert(canonical(&targets[index])?)
        {
            return Err(format!("fixture point {index} differs").into());
        }
    }
    Ok(
        json!({"schema":"n83.crossfamily-current-native-fixture/v1","status":"PASS_SEALED_FIXTURE","curve_id":CURVE_ID,"subgroup_order":ORDER,"target_count":16,"targets_sha256":FILES[0].1,"answers_sha256":FILES[1].1}),
    )
}
fn arms(root: &Path) -> Result<Value> {
    let mut hashes = serde_json::Map::new();
    for arm in ARMS {
        let path = root
            .join("research")
            .join(arm.manifest_dir)
            .join(format!("candidate_{}.json", arm.id));
        let bytes = fs::read(&path)?;
        let record: Value = serde_json::from_slice(&bytes)?;
        let mut stripped = record.clone();
        stripped
            .as_object_mut()
            .ok_or("candidate manifest is not an object")?
            .remove("candidate_id");
        let identity = hash(&canonical(&stripped)?);
        if record["candidate_id"] != arm.id
            || !arm.id.ends_with(&format!("h{}", &identity[..12]))
            || record["factor_base"]["actual_usable_points"] != 166 * arm.k
            || record["factor_base"]["effective_columns"] != arm.k
            || record["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256
            || (arm.layout == "root4_standard")
                != (record["point_decomposition"]["root_slot"]["bytes"] == 4)
        {
            return Err(format!("candidate manifest differs: {}", arm.name).into());
        }
        hashes.insert(arm.id.into(), Value::String(hash(&bytes)));
    }
    Ok(
        json!({"schema":"n83.crossfamily-current-choice/v1","source_sha256":SOURCE_SHA256,"ordered_candidate_ids":ARMS.iter().map(|arm| arm.id).collect::<Vec<_>>(),"candidate_manifest_sha256":hashes}),
    )
}
fn schedule(stratum: usize) -> Result<Value> {
    if stratum > 1 {
        return Err("stratum must be 0 or 1".into());
    }
    let mut events = Vec::with_capacity(256);
    for local in 0..8 {
        let index = stratum * 8 + local;
        for offset in 0..4 {
            let arm_index = (index + offset) % 4;
            let arm = ARMS[arm_index];
            for repetition in 0..3 {
                events.push(json!({"point_index":index,"candidate":arm.name,"name":format!("cal_before_{repetition}_{}",arm.name),"kind":"calibration"}));
            }
            let pair = if (index + arm_index).is_multiple_of(2) {
                ["ic", "rho"]
            } else {
                ["rho", "ic"]
            };
            for kind in pair {
                events.push(json!({"point_index":index,"candidate":arm.name,"name":if kind == "ic" {arm.name.into()} else {format!("rho_{}",arm.name)},"kind":kind}));
            }
            for repetition in 0..3 {
                events.push(json!({"point_index":index,"candidate":arm.name,"name":format!("cal_after_{repetition}_{}",arm.name),"kind":"calibration"}));
            }
        }
    }
    Ok(Value::Array(events))
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() == 5 && args[1] == "aws-plan" {
        let record = aws::plan(
            &PathBuf::from(&args[2]),
            &PathBuf::from(&args[3]),
            &PathBuf::from(&args[4]),
        )?;
        println!(
            "{}",
            json!({"status":record["status"],"source_head_sha":record["source_head_sha"],
            "projected_at_bids_usd":record["projected_at_bids_usd"]})
        );
        return Ok(());
    }
    if args.len() == 6 && args[1] == "aws-allocate" {
        println!(
            "{}",
            aws::allocate(
                &PathBuf::from(&args[2]),
                &PathBuf::from(&args[3]),
                &PathBuf::from(&args[4]),
                &PathBuf::from(&args[5])
            )?
        );
        return Ok(());
    }
    if args.len() == 4 && args[1] == "aws-collect" {
        println!(
            "{}",
            collect::run(&PathBuf::from(&args[2]), &PathBuf::from(&args[3]))?
        );
        return Ok(());
    }
    if args.len() == 4 && args[1] == "aws-guard" {
        println!(
            "{}",
            aws::guard(&PathBuf::from(&args[2]), &PathBuf::from(&args[3]))?
        );
        return Ok(());
    }
    if args.len() == 5 && args[1] == "select" {
        println!(
            "{}",
            select::run(
                &PathBuf::from(&args[2]),
                &PathBuf::from(&args[3]),
                &PathBuf::from(&args[4])
            )?
        );
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    if args.len() == 4 && args[1] == "bind-instance" {
        println!(
            "{}",
            provider::bind_allocation(&PathBuf::from(&args[2]), &PathBuf::from(&args[3]))?
        );
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    if args.len() == 2 && args[1] == "cpu-list" {
        let available = runner::affinity()?;
        if available.len() < 4 {
            return Err("four available CPUs required".into());
        }
        println!(
            "{}",
            available[..4]
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
        return Ok(());
    }
    if args.len() == 4 && args[1] == "bundle-build" {
        let root = PathBuf::from(&args[2]);
        source(&root)?;
        fixture(&root)?;
        arms(&root)?;
        println!("{}", bundle::build(&root, &PathBuf::from(&args[3]))?);
        return Ok(());
    }
    if (args.len() == 3 || args.len() == 4) && args[1] == "bundle-verify" {
        let root = (args.len() == 4).then(|| PathBuf::from(&args[3]));
        println!(
            "{}",
            bundle::verify(&PathBuf::from(&args[2]), root.as_deref())?
        );
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    if args.len() == 9 && args[1] == "run" {
        let root = PathBuf::from(&args[2]);
        let stratum: usize = args[3].to_string_lossy().parse()?;
        source(&root)?;
        fixture(&root)?;
        let choice = arms(&root)?;
        let events = schedule(stratum)?;
        let bound = provider::verify(
            &root,
            stratum,
            &PathBuf::from(&args[6]),
            &PathBuf::from(&args[7]),
        )?;
        return runner::run(
            runner::RunInputs {
                root: &root,
                stratum,
                ic: &PathBuf::from(&args[4]),
                rho: &PathBuf::from(&args[5]),
                output: &PathBuf::from(&args[8]),
            },
            bound,
            choice,
            events,
        );
    }
    #[cfg(target_os = "linux")]
    if (args.len() == 5 || args.len() == 6) && args[1] == "replay" {
        let root = PathBuf::from(&args[2]);
        let stratum: usize = args[3].to_string_lossy().parse()?;
        source(&root)?;
        fixture(&root)?;
        arms(&root)?;
        let analysis_output = if args.len() == 6 {
            PathBuf::from(&args[5])
        } else {
            PathBuf::from(&args[4]).join("analysis.json")
        };
        let analysis = replay::run(
            &root,
            &PathBuf::from(&args[4]),
            stratum,
            &schedule(stratum)?,
            &analysis_output,
        )?;
        println!("{}", json!({"status":analysis["status"],"stratum":stratum}));
        if analysis["status"] != "STRATUM_ADMITTED" {
            return Err("stratum replay did not pass Tier 1 gate".into());
        }
        return Ok(());
    }
    if args.len() != 4 || (args[1] != "preflight" && args[1] != "fixture-only") {
        return Err(
            "usage: n83-crossfamily-current-native (preflight|fixture-only) ROOT STRATUM".into(),
        );
    }
    let root = PathBuf::from(&args[2]);
    let stratum: usize = args[3].to_string_lossy().parse()?;
    if args[1] == "preflight" {
        source(&root)?;
    }
    let inputs = fixture(&root)?;
    let choice = arms(&root)?;
    let events = schedule(stratum)?;
    println!(
        "{}",
        json!({"inputs":inputs,"choice":choice,"events":events})
    );
    Ok(())
}
