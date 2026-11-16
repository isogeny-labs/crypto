//! Independent generic-group replay of a full-D18 four-summand worker result.
//! Usage: n83_d18_native_relation_replay BASE_DIR WORKER_RESULT WORKER_BINARY
//!        FIXTURE_DIR NEW_RECEIPT

use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul};
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
    ChainedS3Encoding, FrobeniusOrbitRootEncoding,
};
use crypto_lib::cryptanalysis::koblitz_fast_arith::FastBinaryCurve128;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::hash::sha256::Sha256;
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const POINTS: usize = 5_418_738;
const COLUMNS: usize = 32_643;
const OBJECT: &str =
    "objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz";
const OBJECT_SHA256: &str = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b";
const PRODUCER_MANIFEST_SHA256: &str =
    "1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce";
const S3_MANIFEST_SHA256: &str = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616";
const S3_OBJECT_KEY: &str = "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz";
const POINT_SET: &str = "31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d";
const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
const CORPUS_BLAKE3: &str = "831eaa7be2387634fd610f6706103b2af9b4e274f0db59923c0280af3cbed74b";
const NATURAL_TARGET: [&str; 2] = ["1834913121178197242856238", "8797964190732846213981377"];
const FROZEN_MANIFEST: &[u8] = include_bytes!("../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/manifest.json");

fn classify_cap_phases(phases: &[Value]) -> Result<&'static str> {
    let names = phases
        .iter()
        .map(|row| row["phase"].as_str())
        .collect::<Vec<_>>();
    match names.as_slice() {
        [Some("started")] => Ok("CAPACITY_CENSORED_BEFORE_MODEL"),
        [Some("started"), Some("import_checked")] => Ok("CAPACITY_CENSORED_DURING_MODEL"),
        [Some("started"), Some("import_checked"), Some("model_constructed")] => {
            Ok("PASS_CENSORED_SEARCH")
        }
        _ => Err("cap phase order or terminal state differs".into()),
    }
}

fn require_terminal_exit(worker: &Path) -> Result<()> {
    let name = worker.to_str().ok_or("worker result path is not UTF-8")?;
    let exit_text = fs::read_to_string(format!("{name}.exit.txt"))?;
    if exit_text.trim() != "0" {
        return Err("terminal worker exit code is not zero".into());
    }
    Ok(())
}

fn audit_cap(path: &Path, cap: &Value, binary_sha: &str, receipt: &Path) -> Result<()> {
    let name = path.to_str().ok_or("cap path is not UTF-8")?;
    let result_name = name
        .strip_suffix(".cap.json")
        .ok_or("cap filename differs")?;
    let exit_text = fs::read_to_string(format!("{result_name}.exit.txt"))?;
    if exit_text.trim() != "124" {
        return Err("worker exit code is not the internal cap code".into());
    }
    let mode = cap["mode"].as_str().ok_or("cap mode absent")?;
    if !["positive", "natural"].contains(&mode)
        || cap["schema"] != "n83.full-d18-m4-cap/v1"
        || cap["status"] != "UNKNOWN_wall_cap"
        || cap["root_encoding"] != "shared_root_bits"
        || cap["root_choice_pinned"] != false
        || cap["choice_pin_policy"] != "none"
        || cap["search_policy"] != "phase_zero_first_ordered_first_pair_xor_cadence32"
        || cap["budget_seconds"] != if mode == "positive" { 300 } else { 2_400 }
        || cap["isolation"] != "L0_shared_host_no_hard_cgroup"
        || cap["elapsed_process_wall_ms"].as_f64().is_none_or(|ms| {
            ms < if mode == "positive" {
                300_000.0
            } else {
                2_400_000.0
            }
        })
        || !cap
            .as_object()
            .is_some_and(|object| object.contains_key("selected_best_total_runtime"))
        || !cap["selected_best_total_runtime"].is_null()
    {
        return Err("internal cap identity or elapsed wall differs".into());
    }
    let phases = fs::read_to_string(format!("{result_name}.phases.jsonl"))?
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let classification = classify_cap_phases(&phases)?;
    let first = &phases[0]["detail"];
    if first["mode"] != mode
        || first["search_policy"] != cap["search_policy"]
        || first["binary_sha256"] != binary_sha
        || first["source_commit"]
            .as_str()
            .is_none_or(|commit| commit.len() != 40)
    {
        return Err("cap phase source or binary identity differs".into());
    }
    let commit = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !commit.status.success()
        || String::from_utf8(commit.stdout)?.trim() != first["source_commit"]
    {
        return Err("cap source commit differs from current checkout".into());
    }
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;
    if !dirty.status.success() || !dirty.stdout.is_empty() {
        return Err("native cap audit requires a clean checkout".into());
    }
    let (verifier_sha, _) = sha256_file(&std::env::current_exe()?)?;
    let record = json!({
        "schema":"n83.d18-native-cap-audit/v1","status":classification,
        "mode":mode,"worker_source_commit":first["source_commit"],
        "worker_binary_sha256":binary_sha,"verifier_binary_sha256":verifier_sha,
        "producer_manifest_sha256":PRODUCER_MANIFEST_SHA256,
        "s3_manifest_sha256":S3_MANIFEST_SHA256,"object_sha256":OBJECT_SHA256,
        "worker_exit_code":124,"worker_status":"UNKNOWN_wall_cap",
        "phase_count":phases.len(),"selected_best_total_runtime":Value::Null
    });
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(receipt)?;
    serde_json::to_writer_pretty(&mut out, &record)?;
    out.write_all(b"\n")?;
    out.sync_all()?;
    Ok(())
}

fn sha256_file(path: &Path) -> Result<(String, u64)> {
    let mut input = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 1 << 20];
    let mut bytes = 0u64;
    loop {
        let read = input.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
        bytes += read as u64;
    }
    Ok((hex::encode(digest.finalize()), bytes))
}

fn unpack_model(bytes: &[u8], bits: usize) -> Result<Vec<bool>> {
    if bits == 0 || bits > 400_000 || bytes.len() != bits.div_ceil(8) {
        return Err("model certificate bit or byte count differs".into());
    }
    if !bits.is_multiple_of(8) && bytes[bits / 8] >> (bits % 8) != 0 {
        return Err("model certificate has nonzero padding bits".into());
    }
    Ok((0..bits)
        .map(|index| bytes[index / 8] & (1 << (index % 8)) != 0)
        .collect())
}

fn read_model_certificate(worker: &Path, result: &Value, variables: usize) -> Result<Vec<bool>> {
    let certificate = &result["model_certificate"];
    let expected_path = worker.with_extension("model.bin");
    if certificate["schema"] != "n83.full-d18-m4-model-certificate/v1"
        || certificate["encoding"] != "bit i of byte i/8 is variable i+1, low bit first"
        || certificate["bits"] != variables
        || certificate["bytes"] != variables.div_ceil(8)
        || certificate["path"].as_str() != expected_path.to_str()
    {
        return Err("worker model certificate metadata differs".into());
    }
    let bytes = fs::read(&expected_path)?;
    if hex::encode(crypto_lib::hash::sha256::sha256(&bytes)) != certificate["sha256"] {
        return Err("retained worker model SHA-256 differs".into());
    }
    unpack_model(&bytes, variables)
}

fn parse_point(fast: &FastBinaryCurve128, value: &Value) -> Result<BinaryPoint> {
    let pair = value.as_array().ok_or("point is not a coordinate pair")?;
    if pair.len() != 2 {
        return Err("point has the wrong coordinate count".into());
    }
    let x: u128 = pair[0].as_str().ok_or("point x is not decimal")?.parse()?;
    let y: u128 = pair[1].as_str().ok_or("point y is not decimal")?.parse()?;
    if x >= 1u128 << 83 || y >= 1u128 << 83 {
        return Err("point coordinate lies outside GF(2^83)".into());
    }
    Ok(BinaryPoint::Affine {
        x: fast.element(x),
        y: fast.element(y),
    })
}

fn check_relation(
    kc: &KoblitzCurve,
    target: &BinaryPoint,
    entries: &[(usize, BinaryPoint, BinaryPoint, BigUint)],
    expected_digest: &str,
) -> Result<()> {
    if entries.len() != 4 {
        return Err("relation does not have four summands".into());
    }
    let mut row = vec![BigUint::from(0u8); COLUMNS];
    let mut point_sum = BinaryPoint::Infinity;
    let mut rep_sum = BinaryPoint::Infinity;
    for (column, point, representative, coefficient) in entries {
        if *column >= COLUMNS || coefficient >= &kc.subgroup_order {
            return Err("relation column or coefficient out of range".into());
        }
        if scalar_mul(&kc.curve, representative, coefficient) != *point {
            return Err("representative label does not reconstruct point".into());
        }
        row[*column] = (&row[*column] + coefficient) % &kc.subgroup_order;
        point_sum = point_add(&kc.curve, &point_sum, point);
    }
    if point_sum != *target
        || row
            .iter()
            .all(|coefficient| coefficient == &BigUint::from(0u8))
    {
        return Err("relation point sum or rank-one condition failed".into());
    }
    let mut representatives = BTreeMap::<usize, &BinaryPoint>::new();
    for (column, _, representative, _) in entries {
        representatives.insert(*column, representative);
    }
    for (column, representative) in representatives {
        rep_sum = point_add(
            &kc.curve,
            &rep_sum,
            &scalar_mul(&kc.curve, representative, &row[column]),
        );
    }
    if rep_sum != *target {
        return Err("coefficient row does not replay the original target".into());
    }
    let decimal = row.iter().map(ToString::to_string).collect::<Vec<_>>();
    let digest = blake3::hash(&serde_json::to_vec(&decimal)?)
        .to_hex()
        .to_string();
    if digest != expected_digest {
        return Err("coefficient-row digest differs".into());
    }
    let changed = point_add(&kc.curve, target, kc.generator());
    if changed == point_sum || changed == rep_sum {
        return Err("changed-target control was accepted".into());
    }
    Ok(())
}

fn check_assignment(
    worker: &Path,
    result: &Value,
    kc: &KoblitzCurve,
    fast: &FastBinaryCurve128,
    target: &BinaryPoint,
    representatives: &[Value],
    entries: &[(usize, BinaryPoint, BinaryPoint, BigUint)],
) -> Result<()> {
    let variables = result["shape"]["variables"]
        .as_u64()
        .ok_or("worker formula variable count absent")? as usize;
    let model = read_model_certificate(worker, result, variables)?;
    let BinaryPoint::Affine { x: target_x, .. } = target else {
        return Err("assignment target is infinity".into());
    };
    let representative_x = representatives
        .iter()
        .map(|value| match parse_point(fast, value)? {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("assignment representative is infinity".into()),
        })
        .collect::<Result<Vec<_>>>()?;
    let mut enc = ChainedS3Encoding::build(
        4,
        &kc.curve.irreducible,
        &kc.curve.b,
        Some(target_x),
        0,
        400_000,
    )
    .map_err(|_| "assignment circuit reconstruction failed")?;
    enc.constrain_frobenius_orbit_summands_with_encoding(
        &representative_x,
        FrobeniusOrbitRootEncoding::SharedBits,
        400_000,
        12_000_000,
    )
    .map_err(|_| "assignment orbit-domain reconstruction failed")?;
    enc.order_first_pair(400_000)
        .map_err(|_| "assignment first-pair ordering failed")?;
    if result["mode"] == "positive" {
        let mut sum = BinaryPoint::Infinity;
        let mut partial = Vec::with_capacity(4);
        for (_, point, _, _) in entries {
            sum = point_add(&kc.curve, &sum, point);
            partial.push(sum.clone());
        }
        let points = entries
            .iter()
            .map(|(_, point, _, _)| point)
            .chain(partial[1..3].iter())
            .chain(std::iter::once(target));
        for (block, point) in points.enumerate() {
            let BinaryPoint::Affine { x, .. } = point else {
                return Err("assignment positive pin is infinity".into());
            };
            let x = x.to_biguint();
            for bit in 0..83usize {
                let wire = (block * 83 + bit + 1) as i32;
                if !enc
                    .solver
                    .add_clause(vec![if x.bit(bit as u64) { wire } else { -wire }])
                {
                    return Err("assignment positive pin contradicts reconstructed formula".into());
                }
            }
        }
    }
    if enc.solver.n_vars() as usize != variables
        || result["shape"]["clauses"] != enc.solver.n_clauses()
        || result["shape"]["xor_rows"] != enc.solver.n_xors()
    {
        return Err("reconstructed assignment formula shape differs".into());
    }
    if !enc.verify_external_assignment(&model) {
        return Err("retained assignment fails original CNF, XOR or field checks".into());
    }
    let xs = enc
        .decode_summands_assignment(&model)
        .ok_or("retained assignment summands cannot be decoded")?;
    if xs.len() != entries.len()
        || xs.iter().zip(entries).any(|(x, (_, point, _, _))| {
            !matches!(point, BinaryPoint::Affine { x: expected, .. } if x == expected)
        })
    {
        return Err("retained assignment summands differ from checked group row".into());
    }
    Ok(())
}

fn replay(base: &Path, worker: &Path, binary: &Path, fixture: &Path, receipt: &Path) -> Result<()> {
    if receipt.exists() {
        return Err("receipt path already exists".into());
    }
    let manifest_bytes = fs::read(base.join("manifest.json"))?;
    if manifest_bytes != FROZEN_MANIFEST
        || hex::encode(crypto_lib::hash::sha256::sha256(&manifest_bytes))
            != PRODUCER_MANIFEST_SHA256
    {
        return Err("manifest differs from frozen D18 generation".into());
    }
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    if manifest["curve_id"] != CURVE
        || manifest["object"] != OBJECT
        || manifest["point_set_blake3"] != POINT_SET
        || manifest["columns"] != COLUMNS
        || manifest["points"] != POINTS
    {
        return Err("manifest identity differs".into());
    }
    let s3_manifest_bytes = fs::read(base.join("s3-manifest.json"))?;
    if hex::encode(crypto_lib::hash::sha256::sha256(&s3_manifest_bytes)) != S3_MANIFEST_SHA256 {
        return Err("S3 custody manifest differs from frozen generation".into());
    }
    let s3_manifest: Value = serde_json::from_slice(&s3_manifest_bytes)?;
    if s3_manifest["schema"] != "n83.d18-full-s3-manifest/v1"
        || s3_manifest["curve_id"] != CURVE
        || s3_manifest["producer_manifest_sha256"] != PRODUCER_MANIFEST_SHA256
        || s3_manifest["object_key"] != S3_OBJECT_KEY
        || s3_manifest["compressed_sha256"] != OBJECT_SHA256
        || s3_manifest["fresh_get_sha256"] != OBJECT_SHA256
        || s3_manifest["compressed_bytes"] != 160_747_529u64
        || s3_manifest["points"] != POINTS
        || s3_manifest["columns"] != COLUMNS
        || s3_manifest["policy"] != "direct_subgroup_x_d18_full"
        || s3_manifest["status"] != "PASS_FULL_REPLAY_AND_FRESH_S3_CUSTODY"
        || !s3_manifest["selected_best_total_runtime"].is_null()
    {
        return Err("S3 custody manifest identity differs".into());
    }
    let (object_sha, object_bytes) = sha256_file(&base.join(OBJECT))?;
    if object_sha != OBJECT_SHA256 || object_bytes != 160_747_529 {
        return Err("compressed D18 object differs".into());
    }
    let (binary_sha, _) = sha256_file(binary)?;
    let (corpus_sha, _) = sha256_file(&fixture.join("probe-corpus.json"))?;
    if corpus_sha != CORPUS_SHA256 {
        return Err("public fixture differs".into());
    }
    let result: Value = serde_json::from_slice(&fs::read(worker)?)?;
    if result["status"] == "UNKNOWN_wall_cap" {
        return audit_cap(worker, &result, &binary_sha, receipt);
    }
    require_terminal_exit(worker)?;
    let mode = result["mode"].as_str().ok_or("worker mode absent")?;
    if !["positive", "natural"].contains(&mode) {
        return Err("worker mode differs".into());
    }
    let expected_schema = if mode == "positive" {
        "n83.full-d18-m4-l0-positive/v1"
    } else {
        "n83.full-d18-m4-l0-natural/v1"
    };
    let expected_status = if mode == "positive" {
        "PASS_CONSTRUCTED_CONTROL"
    } else {
        "PENDING_INDEPENDENT_NATURAL_REPLAY"
    };
    if !result.as_object().is_some_and(|object| {
        object.contains_key("selected_best_total_runtime")
            && object.contains_key("total_index_calculus_runtime_ms")
    }) {
        return Err("worker total-cost fields absent".into());
    }
    if result["schema"] != expected_schema
        || result["status"] != expected_status
        || result["binary_sha256"] != binary_sha
        || result["object"] != OBJECT
        || result["point_set_blake3"] != POINT_SET
        || result["public_corpus_blake3"] != CORPUS_BLAKE3
        || result["search_policy"] != "phase_zero_first_ordered_first_pair_xor_cadence32"
        || result["root_encoding"] != "shared_root_bits"
        || result["root_choice_pinned"] != false
        || result["ordered_first_pair"] != true
        || result["xor_decision_cadence"] != 32
        || result["choice_pin_policy"] != "none"
        || result["solver_answer"] != "Sat"
        || result["solver_conflicts"]
            .as_u64()
            .is_none_or(|n| n > 100_000)
        || result["curve_a"] != 0
        || result["summands"] != 4
        || result["identity_mask"] != 0
        || result["source_attestation_mode"] != "clean_checkout"
        || result["isolation"] != "L0_shared_host_no_hard_cgroup"
        || result["budget_seconds"] != if mode == "positive" { 300 } else { 2_400 }
        || !result["selected_best_total_runtime"].is_null()
        || !result["total_index_calculus_runtime_ms"].is_null()
    {
        return Err("worker identity, policy or terminal status differs".into());
    }
    let shape = &result["shape"];
    if shape["variables"] != 329_808
        || shape["clauses"]
            != if mode == "positive" {
                2_807_999
            } else {
                2_807_501
            }
        || shape["xor_rows"] != 3_901
        || shape["search_policy"] != "phase_zero_first_ordered_first_pair_xor_cadence32"
        || shape["object"] != OBJECT
        || shape["point_set_blake3"] != POINT_SET
        || shape["root_choice_pinned"] != false
    {
        return Err("full D18 formula shape differs".into());
    }
    let commit = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !commit.status.success()
        || String::from_utf8(commit.stdout)?.trim() != result["source_commit"]
    {
        return Err("worker source commit differs from current clean checkout".into());
    }
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;
    if !dirty.status.success() || !dirty.stdout.is_empty() {
        return Err("native replay requires a clean checkout".into());
    }
    let phase_path = Path::new(
        result["phase_file"]
            .as_str()
            .ok_or("worker phase path absent")?,
    );
    let phases = fs::read_to_string(phase_path)?;
    let phases = phases
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if phases.len() != 4
        || phases
            .iter()
            .map(|row| row["phase"].as_str())
            .collect::<Vec<_>>()
            != [
                Some("started"),
                Some("import_checked"),
                Some("model_constructed"),
                Some("solver_returned"),
            ]
        || phases[0]["detail"]["source_commit"] != result["source_commit"]
        || phases[0]["detail"]["binary_sha256"] != binary_sha
        || phases[0]["detail"]["search_policy"] != result["search_policy"]
        || phases[0]["detail"]["mode"] != mode
        || phases[2]["detail"] != result["shape"]
        || phases[3]["detail"]["answer"] != "Sat"
        || phases[3]["detail"]["conflicts"] != result["solver_conflicts"]
    {
        return Err("worker phase stream differs from terminal result".into());
    }
    let kc = KoblitzCurve::known_n83_k0().ok_or("primary curve unavailable")?;
    let fast =
        FastBinaryCurve128::new(&kc.curve.irreducible, 0).ok_or("packed field unavailable")?;
    let target = parse_point(&fast, &result["target"])?;
    if !kc.curve.is_on_curve(&target)
        || scalar_mul(&kc.curve, &target, &kc.subgroup_order) != BinaryPoint::Infinity
    {
        return Err("worker target not on the primary subgroup".into());
    }
    if mode == "natural" && result["target"] != json!(NATURAL_TARGET) {
        return Err("natural target differs from frozen public query".into());
    }
    let relation = &result["relation"];
    let indices = relation["indices"]
        .as_array()
        .ok_or("relation indices absent")?;
    if indices.len() != 4
        || relation["rank_after_insert"] != 1
        || relation["tamper_rejected"] != true
        || (mode == "positive" && relation["matches_planted_row"] != true)
    {
        return Err("relation rank or worker controls differ".into());
    }
    let selected = indices
        .iter()
        .map(|value| {
            value
                .as_u64()
                .ok_or("relation index not integral")
                .map(|n| n as usize)
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if selected.iter().any(|&index| index >= POINTS)
        || (mode == "positive" && selected != [0, 166, 332, 498])
    {
        return Err("relation indices differ from frozen domain".into());
    }
    let needed = selected.iter().copied().collect::<BTreeSet<_>>();
    let mut reader = BufReader::new(GzDecoder::new(File::open(base.join(OBJECT))?));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    if header["curve_slug"] != CURVE
        || header["orbit_columns"] != COLUMNS
        || header["point_count"] != POINTS
        || header["subgroup_order"] != kc.subgroup_order.to_string()
    {
        return Err("compressed object header differs".into());
    }
    let reps = header["representatives"]
        .as_array()
        .ok_or("representatives absent")?;
    if reps.len() != COLUMNS {
        return Err("representative count differs".into());
    }
    let mut rows = BTreeMap::new();
    for index in 0..=(*needed.iter().max().ok_or("no selected indices")?) {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err("selected point row absent".into());
        }
        if needed.contains(&index) {
            rows.insert(index, serde_json::from_str::<Value>(&line)?);
        }
    }
    let mut entries = Vec::with_capacity(4);
    for index in &selected {
        let row = rows.get(index).ok_or("selected point row missing")?;
        let column = index / 166;
        let phase = (index % 166) / 2;
        let negative = index % 2 == 1;
        if row["column"] != column || row["phase"] != phase || row["negative"] != negative {
            return Err("selected point index and orbit label disagree".into());
        }
        let representative = parse_point(&fast, &reps[column])?;
        if !kc.curve.is_on_curve(&representative)
            || scalar_mul(&kc.curve, &representative, &kc.subgroup_order) != BinaryPoint::Infinity
            || scalar_mul(&kc.curve, &representative, &kc.lambda) != kc.frobenius(&representative)
        {
            return Err("representative subgroup or Frobenius action failed".into());
        }
        let mut point = representative.clone();
        for _ in 0..phase {
            point = kc.frobenius(&point);
        }
        if negative {
            point = point_neg(&point);
        }
        if parse_point(&fast, &row["point"])? != point {
            return Err("selected point differs from representative action".into());
        }
        let mut coefficient = kc.lambda.modpow(&BigUint::from(phase), &kc.subgroup_order);
        if negative {
            coefficient = (&kc.subgroup_order - coefficient) % &kc.subgroup_order;
        }
        if row["coefficient"] != coefficient.to_string() {
            return Err("selected coefficient differs from Frobenius label".into());
        }
        entries.push((column, point, representative, coefficient));
    }
    check_relation(
        &kc,
        &target,
        &entries,
        relation["row_decimal_blake3"]
            .as_str()
            .ok_or("row digest absent")?,
    )?;
    check_assignment(worker, &result, &kc, &fast, &target, reps, &entries)?;
    let (verifier_sha, _) = sha256_file(&std::env::current_exe()?)?;
    let record = json!({
        "schema":"n83.d18-native-relation-replay/v1",
        "status":"PASS_NATIVE_GROUP_ROW_AND_ASSIGNMENT_REPLAY",
        "mode":mode,"worker_source_commit":result["source_commit"],
        "worker_binary_sha256":binary_sha,"verifier_binary_sha256":verifier_sha,
        "producer_manifest_sha256":PRODUCER_MANIFEST_SHA256,
        "s3_manifest_sha256":S3_MANIFEST_SHA256,"object_sha256":OBJECT_SHA256,
        "point_set_blake3":POINT_SET,"relation_indices":selected,
        "row_decimal_blake3":relation["row_decimal_blake3"],
        "model_certificate_sha256":result["model_certificate"]["sha256"],
        "original_group_sum_checked":true,"signed_orbit_row_checked":true,
        "changed_target_rejected":true,"rank_after_insert":1,
        "original_cnf_xor_checked_by_worker":true,
        "independent_assignment_replay":true,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(receipt)?;
    serde_json::to_writer_pretty(&mut out, &record)?;
    out.write_all(b"\n")?;
    out.sync_all()?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 6 {
        return Err("usage: n83_d18_native_relation_replay BASE_DIR WORKER_RESULT WORKER_BINARY FIXTURE_DIR NEW_RECEIPT".into());
    }
    replay(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
        Path::new(&args[4]),
        Path::new(&args[5]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_replay_requires_zero_worker_exit() {
        let directory = std::env::temp_dir().join(format!(
            "n83-d18-terminal-exit-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let result = directory.join("result.json");
        let exit = directory.join("result.json.exit.txt");
        assert!(require_terminal_exit(&result).is_err());
        fs::write(&exit, "1\n").unwrap();
        assert!(require_terminal_exit(&result).is_err());
        fs::write(&exit, "124\n").unwrap();
        assert!(require_terminal_exit(&result).is_err());
        fs::write(&exit, "0\n").unwrap();
        require_terminal_exit(&result).unwrap();
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn retained_model_certificate_rejects_tampering_and_padding() {
        let directory = std::env::temp_dir().join(format!(
            "n83-d18-model-certificate-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let worker = directory.join("result.json");
        let model_path = worker.with_extension("model.bin");
        let bytes = [0b0000_0101u8, 0b0000_0001u8];
        fs::write(&model_path, bytes).unwrap();
        let certificate = json!({"model_certificate":{
            "schema":"n83.full-d18-m4-model-certificate/v1",
            "encoding":"bit i of byte i/8 is variable i+1, low bit first",
            "bits":9,"bytes":2,
            "sha256":hex::encode(crypto_lib::hash::sha256::sha256(&bytes)),
            "path":model_path.to_string_lossy(),
        }});
        assert_eq!(
            read_model_certificate(&worker, &certificate, 9).unwrap(),
            vec![true, false, true, false, false, false, false, false, true]
        );
        let mut wrong_path = certificate.clone();
        wrong_path["model_certificate"]["path"] = json!("elsewhere.bin");
        assert!(read_model_certificate(&worker, &wrong_path, 9).is_err());
        fs::write(&model_path, [0b0000_0100u8, 0b0000_0001u8]).unwrap();
        assert!(read_model_certificate(&worker, &certificate, 9).is_err());
        fs::write(&model_path, [0b0000_0101u8, 0b0000_0011u8]).unwrap();
        assert!(unpack_model(&fs::read(&model_path).unwrap(), 9).is_err());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn producer_manifest_is_distinct_from_s3_custody_wrapper() {
        let producer_sha = hex::encode(crypto_lib::hash::sha256::sha256(FROZEN_MANIFEST));
        assert_eq!(producer_sha, PRODUCER_MANIFEST_SHA256);
        assert_ne!(producer_sha, S3_MANIFEST_SHA256);
    }

    #[test]
    fn generic_group_replay_rejects_changed_target_and_row_digest() {
        let kc = KoblitzCurve::known_n83_k0().unwrap();
        let generator = kc.generator().clone();
        let points = [1u8, 2, 3, 4].map(|n| scalar_mul(&kc.curve, &generator, &BigUint::from(n)));
        let target = points.iter().fold(BinaryPoint::Infinity, |sum, p| {
            point_add(&kc.curve, &sum, p)
        });
        let entries = points
            .iter()
            .enumerate()
            .map(|(column, point)| (column, point.clone(), point.clone(), BigUint::from(1u8)))
            .collect::<Vec<_>>();
        let mut row = vec!["0".to_owned(); COLUMNS];
        for value in row.iter_mut().take(4) {
            *value = "1".to_owned();
        }
        let digest = blake3::hash(&serde_json::to_vec(&row).unwrap())
            .to_hex()
            .to_string();
        check_relation(&kc, &target, &entries, &digest).unwrap();
        assert!(check_relation(
            &kc,
            &point_add(&kc.curve, &target, kc.generator()),
            &entries,
            &digest
        )
        .is_err());
        assert!(check_relation(&kc, &target, &entries, &"0".repeat(64)).is_err());
    }

    #[test]
    fn cap_classification_requires_an_ordered_nonterminal_prefix() {
        let started = json!({"phase":"started"});
        let imported = json!({"phase":"import_checked"});
        let modeled = json!({"phase":"model_constructed"});
        let returned = json!({"phase":"solver_returned"});
        assert_eq!(
            classify_cap_phases(&[started.clone()]).unwrap(),
            "CAPACITY_CENSORED_BEFORE_MODEL"
        );
        assert_eq!(
            classify_cap_phases(&[started.clone(), imported.clone()]).unwrap(),
            "CAPACITY_CENSORED_DURING_MODEL"
        );
        assert_eq!(
            classify_cap_phases(&[started.clone(), imported.clone(), modeled]).unwrap(),
            "PASS_CENSORED_SEARCH"
        );
        assert!(classify_cap_phases(&[started.clone(), returned]).is_err());
        assert!(classify_cap_phases(&[imported, started]).is_err());
    }
}
