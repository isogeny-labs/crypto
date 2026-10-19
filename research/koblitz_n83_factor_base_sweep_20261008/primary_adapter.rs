//! Replay-bound bridge from the stored primary N83 points to the generic IC base.
//!
//! This bridge checks every JSONL point and signed-Frobenius label, then
//! independently rebuilds the orbit maps in the same point order.

use super::{
    compact_cold, curve, frozen_v2_source_commit, general, load_object, parse_point, point_json,
    point_set_hash, v2_design_hash, v2_row_is_declared, words, write_new_json, Result, PREFIX,
    SEEDS, STUDY, V2_SIZES, V2_SUBDIR,
};
use crypto_lib::binary_ecc::curve::point_neg;
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
    chain_s3_max_variables, finite_domain_clause_count, frobenius_orbit_domain_size,
    ChainedS3Encoding,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_explicit_frobenius_point_factor_base, build_union_s4_encoding,
    koblitz_index_calculus_dlp_with_factor_base_and_progress, koblitz_point_count, point_key,
    ChainS3Limits, DecompositionStrategy, FrobeniusFactorBase, KoblitzCurve, KoblitzIcOptions,
    SatDecompositionOptions,
};
use crypto_lib::cryptanalysis::koblitz_relation_solver::WideRankTracker;
use crypto_lib::cryptanalysis::sat::{
    RootPropagationResult, SolveResult, SolverProgress, SolverStats,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

fn cgroup_v2_root_from_membership(membership: &str) -> io::Result<PathBuf> {
    let path = membership
        .lines()
        .find_map(|line| line.strip_prefix("0::"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "cgroup v2 membership missing"))?;
    let relative = path.strip_prefix('/').ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "cgroup v2 path is not absolute")
    })?;
    if Path::new(relative)
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "cgroup v2 path traverses above the mount",
        ));
    }
    Ok(Path::new("/sys/fs/cgroup").join(relative))
}

fn cgroup_v2_file(name: &str) -> io::Result<PathBuf> {
    let membership = fs::read_to_string("/proc/self/cgroup")?;
    Ok(cgroup_v2_root_from_membership(&membership)?.join(name))
}

fn checked_cgroup_memory_limit(memory_mib: u64) -> Result<u64> {
    let bytes = memory_mib
        .checked_mul(1024 * 1024)
        .filter(|&value| value > 0)
        .ok_or("invalid capacity memory limit")?;
    let actual: u64 = fs::read_to_string(cgroup_v2_file("memory.max")?)?
        .trim()
        .parse()
        .map_err(|_| "capacity worker requires a finite cgroup memory.max")?;
    let swap: u64 = fs::read_to_string(cgroup_v2_file("memory.swap.max")?)?
        .trim()
        .parse()
        .map_err(|_| "capacity worker requires a finite cgroup memory.swap.max")?;
    if actual != bytes || swap != 0 {
        return Err("capacity worker requires the declared hard memory cap and zero swap".into());
    }
    Ok(bytes)
}

fn process_usage() -> Value {
    #[cfg(unix)]
    {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } == 0 {
            let usage = unsafe { usage.assume_init() };
            let seconds =
                |value: libc::timeval| value.tv_sec as f64 + value.tv_usec as f64 / 1_000_000.0;
            let multiplier = if cfg!(target_os = "macos") { 1 } else { 1024 };
            return json!({
                "cpu_seconds":seconds(usage.ru_utime)+seconds(usage.ru_stime),
                "peak_rss_bytes":(usage.ru_maxrss.max(0) as u64).saturating_mul(multiplier)
            });
        }
    }
    json!({"cpu_seconds":null,"peak_rss_bytes":null})
}

pub(super) struct PrimaryBase {
    pub(super) curve: KoblitzCurve,
    pub(super) factor_base: FrobeniusFactorBase,
}

fn from_record(row: &Value, bytes: &[u8]) -> Result<PrimaryBase> {
    let mut lines = std::str::from_utf8(bytes)?.lines();
    let header: Value = serde_json::from_str(lines.next().ok_or("missing base header")?)?;
    let columns = row["columns"].as_u64().ok_or("column count")? as usize;
    if columns == 0 || header["orbit_columns"].as_u64() != Some(columns as u64) {
        return Err("primary orbit-column count mismatch".into());
    }
    let pinned = curve(0)?;
    if row["a"].as_u64() != Some(0)
        || header["curve_a"].as_u64() != Some(0)
        || header["schema"] != "n83.factor-base/v1"
        || header["study"] != STUDY
        || header["curve_slug"] != "icv1-f2m83-tm6151469093347-debefd74"
        || header["n"] != 83
        || header["policy"] != row["policy"]
        || header["seed"] != row["seed"]
        || header["closure"] != "signed_frobenius"
        || header["modulus_low_terms"] != json!([0, 1, 2, 45])
        || header["subgroup_order"] != pinned.subgroup_order.to_string()
        || header["cofactor"] != pinned.cofactor.to_string()
        || header["frobenius_lambda"] != pinned.lambda.to_string()
        || header["generator"] != point_json(words(pinned.generator()))
        || header["point_count"].as_u64() != Some((166 * columns) as u64)
        || row["points"].as_u64() != Some((166 * columns) as u64)
    {
        return Err("primary curve or manifest binding mismatch".into());
    }
    let reps = header["representatives"]
        .as_array()
        .ok_or("representatives")?;
    if reps.len() != columns {
        return Err("representative count mismatch".into());
    }

    let mut points = Vec::with_capacity(166 * columns);
    let mut point_words = Vec::with_capacity(166 * columns);
    let mut orbits = Vec::with_capacity(2 * columns);
    let mut orbit_of = Vec::with_capacity(166 * columns);
    let mut signed_orbits = Vec::with_capacity(columns);
    let mut signed_orbit_of = Vec::with_capacity(166 * columns);
    let mut x_values = BTreeMap::new();
    let mut seen = HashSet::with_capacity(166 * columns);

    for (column, value) in reps.iter().enumerate() {
        let rep_word = parse_point(value)?.ok_or("identity representative")?;
        let rep = general(Some(rep_word));
        if !pinned.curve.is_on_curve(&rep)
            || pinned.mul(&rep, &pinned.subgroup_order) != BinaryPoint::Infinity
            || pinned.mul(&rep, &pinned.lambda) != pinned.frobenius(&rep)
        {
            return Err("primary representative subgroup or eigenvalue mismatch".into());
        }
        let mut current = rep.clone();
        let mut coefficient = BigUint::one();
        let mut positive_orbit = Vec::with_capacity(83);
        let mut negative_orbit = Vec::with_capacity(83);
        let mut signed_members = Vec::with_capacity(166);
        for phase in 0..83u32 {
            if let BinaryPoint::Affine { x, .. } = &current {
                x_values.entry(x.to_biguint()).or_insert_with(|| x.clone());
            } else {
                return Err("identity inside primary orbit".into());
            }
            for negative in [false, true] {
                let point = if negative {
                    point_neg(&current)
                } else {
                    current.clone()
                };
                let entry: Value = serde_json::from_str(lines.next().ok_or("missing point row")?)?;
                let expected_coefficient = if negative {
                    (&pinned.subgroup_order - &coefficient).to_string()
                } else {
                    coefficient.to_string()
                };
                let index = points.len();
                if entry["point"] != point_json(words(&point))
                    || entry["column"].as_u64() != Some(column as u64)
                    || entry["phase"].as_u64() != Some(phase as u64)
                    || entry["negative"].as_bool() != Some(negative)
                    || entry["coefficient"] != expected_coefficient
                    || !seen.insert(point_key(&point))
                {
                    return Err("primary point, label or distinctness mismatch".into());
                }
                point_words.push(words(&point));
                points.push(point);
                orbit_of.push((2 * column + usize::from(negative), phase));
                signed_orbit_of.push((column, phase, negative));
                signed_members.push(index);
                if negative {
                    negative_orbit.push(index);
                } else {
                    positive_orbit.push(index);
                }
            }
            current = pinned.frobenius(&current);
            coefficient = (&coefficient * &pinned.lambda) % &pinned.subgroup_order;
        }
        if current != rep {
            return Err("primary orbit did not close".into());
        }
        orbits.push(positive_orbit);
        orbits.push(negative_orbit);
        signed_orbits.push(signed_members);
    }
    if lines.next().is_some()
        || points.len() != 166 * columns
        || x_values.len() != 83 * columns
        || row["point_set_blake3"] != point_set_hash(point_words)?
    {
        return Err("primary point count or set hash mismatch".into());
    }

    let curve = KoblitzCurve {
        a: 0,
        n: 83,
        trace: -1,
        group_order: koblitz_point_count(0, 83),
        subgroup_order: pinned.subgroup_order,
        cofactor: pinned.cofactor,
        lambda: pinned.lambda,
        frobenius_is_endomorphism: true,
        curve: pinned.curve,
        k: 1,
        q: 2,
        a_index: 0,
        b_index: 1,
        subfield_basis: vec![F2mElement::one(83)],
    };
    let factor_base = build_explicit_frobenius_point_factor_base(&curve, points)
        .ok_or("primary point set is not a complete Frobenius base")?;
    if factor_base.subspace != x_values.into_values().collect::<Vec<_>>()
        || factor_base.orbits != orbits
        || factor_base.orbit_of != orbit_of
        || factor_base.signed_orbits != signed_orbits
        || factor_base.signed_orbit_of != signed_orbit_of
    {
        return Err("primary orbit maps differ from the stored labels".into());
    }
    Ok(PrimaryBase { curve, factor_base })
}

fn load_panel_selected(
    root: &Path,
    columns: usize,
    policy: &str,
    seed: u64,
) -> Result<(PrimaryBase, Value, String)> {
    if ![
        "public_x_sequential",
        "public_x_hash",
        "public_x_gray_prefix",
    ]
    .contains(&policy)
        || !SEEDS.contains(&seed)
    {
        return Err("primary base policy or seed outside frozen panel".into());
    }
    let manifest_bytes = fs::read(root.join("manifest.json"))?;
    let manifest_hash = blake3::hash(&manifest_bytes).to_hex().to_string();
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let replay: Value = serde_json::from_slice(&fs::read(root.join("replay.json"))?)?;
    let storage: Value = serde_json::from_slice(&fs::read(root.join("upload-receipt.json"))?)?;
    let v2 = manifest["schema"] == "n83.factor-base-panel/v2-size-frontier";
    let expected_count = if v2 { 1 } else { 54 };
    let storage_prefix = if v2 {
        format!("{PREFIX}/{STUDY}/{V2_SUBDIR}")
    } else {
        PREFIX.to_owned()
    };
    let panel_shape_valid = if v2 {
        manifest["status"] == "completed_factor_base_object"
            && manifest["size_design_blake3"] == v2_design_hash()
            && manifest["expected_base_count"] == 1
            && manifest["completed_base_count"] == 1
    } else {
        manifest["schema"] == "n83.factor-base-panel/v1"
            && manifest["status"] == "completed_factor_base_panel"
            && manifest["expected_base_count"] == 54
            && manifest["completed_base_count"] == 54
    };
    if !panel_shape_valid
        || manifest["study"] != STUDY
        || replay["schema"]
            != if v2 {
                "n83.factor-base-replay/v2-size-frontier"
            } else {
                "n83.factor-base-replay/v1"
            }
        || replay["status"] != "PASS"
        || replay["panel_manifest_blake3"] != manifest_hash
        || storage["schema"]
            != if v2 {
                "n83.factor-base-storage/v2-size-frontier"
            } else {
                "n83.factor-base-storage/v1"
            }
        || storage["status"] != "PASS"
        || storage["manifest_blake3"] != manifest_hash
        || storage["bucket"] != "crypto-autoresearcher"
        || storage["prefix"] != storage_prefix
    {
        return Err("panel or replay receipt is not complete and bound".into());
    }
    let rows = manifest["bases"].as_array().ok_or("panel base rows")?;
    if rows.len() != expected_count || (v2 && !v2_row_is_declared(&rows[0])) {
        return Err("panel base count mismatch".into());
    }
    let mut rows = rows.iter().filter(|row| {
        row["a"].as_u64() == Some(0)
            && row["policy"] == policy
            && row["seed"].as_u64() == Some(seed)
            && row["columns"].as_u64() == Some(columns as u64)
    });
    let row = rows.next().ok_or("requested primary base absent")?;
    if rows.next().is_some() {
        return Err("ambiguous primary base selection".into());
    }
    let expected_uri = if v2 {
        format!(
            "{storage_prefix}/a0/{}",
            row["object"].as_str().ok_or("object path")?
        )
    } else {
        format!(
            "{PREFIX}/{STUDY}/a0/{}",
            row["object"].as_str().ok_or("object path")?
        )
    };
    if row["s3_uri"] != expected_uri {
        return Err("selected base S3 destination mismatch".into());
    }
    let checks = replay["checks"].as_array().ok_or("replay checks")?;
    let matching: Vec<_> = checks
        .iter()
        .filter(|check| check["object"] == row["object"])
        .collect();
    if matching.len() != 1
        || matching[0]["status"] != "PASS"
        || matching[0]["points_checked"].as_u64() != Some(166 * columns as u64)
        || matching[0]["representatives_checked"].as_u64() != Some(columns as u64)
    {
        return Err("selected object lacks a successful replay".into());
    }
    let stored = storage["objects"].as_array().ok_or("storage object rows")?;
    if stored.len() != expected_count
        || stored
            .iter()
            .filter(|item| item["s3_uri"] == row["s3_uri"])
            .count()
            != 1
        || !stored.iter().any(|item| {
            item["s3_uri"] == row["s3_uri"]
                && item["compressed_blake3"] == row["compressed_blake3"]
                && item["downloaded_hash_matches"] == true
        })
    {
        return Err("selected object lacks a matching S3 round-trip receipt".into());
    }
    let base = from_record(row, &load_object(root, row)?)?;
    Ok((base, row.clone(), manifest_hash))
}

fn load_panel(root: &Path, columns: usize) -> Result<(PrimaryBase, Value, String)> {
    load_panel_selected(root, columns, "public_x_hash", SEEDS[0])
}

const D18_OBJECT_BLAKE3: &str = "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682";
const D18_OBJECT_SHA256: &str = "347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7";
const D18_POINT_SET_BLAKE3: &str =
    "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0";
const D18_SOURCE_COMMIT: &str = "b901026b6b05cd1fe9b909620d6746a5a931520c";
const PAIRED_CORPUS_SHA256: &str =
    "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
const PAIRED_VALIDATION_SHA256: &str =
    "1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f";

fn file_sha256(path: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(&fs::read(path)?)))
}

fn load_paired_base(
    arm: &str,
    root: &Path,
    evidence_root: &Path,
) -> Result<(PrimaryBase, Value, String, &'static str, &'static str)> {
    match arm {
        "U" => {
            let (base, row, manifest_hash) =
                load_panel_selected(root, 16_627, "public_x_hash", 2026100801)?;
            if row["object"] != format!("objects/{K16627_OBJECT_BLAKE3}.jsonl.gz")
                || row["point_set_blake3"] != K16627_POINT_SET_BLAKE3
                || file_sha256(&root.join("manifest.json"))?
                    != "0c09f62bde61d3c3e987644508c53ab56fd3f545219013b9bde925ba29969275"
                || file_sha256(&root.join("replay.json"))?
                    != "75b3bc17d8366e258e1b8f3f8de5f42911eb893c3d029a2e40c65b9e59cffe60"
                || file_sha256(&root.join("upload-receipt.json"))?
                    != "70e82dfef4ad735a6974b46a4d6bca726dba4cc68be5f35ca354fab3262b445b"
                || file_sha256(&root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256
                || file_sha256(&root.join(format!("objects/{K16627_OBJECT_BLAKE3}.jsonl.gz")))?
                    != "2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369"
            {
                return Err("uniform paired input differs from frozen panel".into());
            }
            Ok((
                base,
                row,
                manifest_hash,
                K16627_OBJECT_BLAKE3,
                K16627_POINT_SET_BLAKE3,
            ))
        }
        "D" => {
            let (base, row) = load_d18_export(root, &evidence_root.join("export-v1"))?;
            let imported: Value =
                serde_json::from_slice(&fs::read(evidence_root.join("import-v1/import.json"))?)?;
            if imported["schema"] != "n83.primary-d18-factor-base-adapter/v1"
                || imported["status"] != "PASS"
                || imported["object"] != row["object"]
                || imported["point_set_blake3"] != D18_POINT_SET_BLAKE3
                || imported["orbit_columns"] != 16_627
                || imported["points"] != 2_760_082
                || imported["selected_best_total_runtime"] != Value::Null
            {
                return Err("d18 paired import receipt differs".into());
            }
            let manifest_hash = blake3::hash(&fs::read(root.join("manifest.json"))?)
                .to_hex()
                .to_string();
            Ok((
                base,
                row,
                manifest_hash,
                D18_OBJECT_BLAKE3,
                D18_POINT_SET_BLAKE3,
            ))
        }
        _ => Err("paired arm must be U or D".into()),
    }
}

fn load_d18_export(export_root: &Path, evidence_root: &Path) -> Result<(PrimaryBase, Value)> {
    let manifest_bytes = fs::read(export_root.join("manifest.json"))?;
    let replay_bytes = fs::read(export_root.join("replay.json"))?;
    if manifest_bytes != fs::read(evidence_root.join("manifest.json"))?
        || replay_bytes != fs::read(evidence_root.join("replay.json"))?
    {
        return Err("d18 export and committed evidence copies differ".into());
    }
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let replay: Value = serde_json::from_slice(&replay_bytes)?;
    let custody: Value = serde_json::from_slice(&fs::read(evidence_root.join("s3-custody.json"))?)?;
    let object = format!("objects/{D18_OBJECT_BLAKE3}.jsonl.gz");
    let uri = format!("{PREFIX}/{STUDY}/d18-direct-subgroup-v1/a0/{object}");
    if manifest["schema"] != "n83.direct-subgroup-orbit-export/v1"
        || manifest["status"] != "EXPORTED_UNREPLAYED"
        || manifest["study"] != STUDY
        || manifest["curve_id"] != "icv1-f2m83-tm6151469093347-debefd74"
        || manifest["policy"] != "direct_subgroup_x_d18"
        || manifest["source_commit"] != D18_SOURCE_COMMIT
        || manifest["columns"] != 16_627
        || manifest["points"] != 2_760_082
        || manifest["object"] != object
        || manifest["compressed_blake3"] != D18_OBJECT_BLAKE3
        || manifest["compressed_sha256"] != D18_OBJECT_SHA256
        || manifest["point_set_blake3"] != D18_POINT_SET_BLAKE3
        || manifest["bytes"] != 81_844_623u64
        || manifest["selected_best_total_runtime"] != Value::Null
        || replay["schema"] != "n83.direct-subgroup-orbit-replay/v1"
        || replay["status"] != "PASS"
        || replay["object"] != object
        || replay["compressed_blake3"] != D18_OBJECT_BLAKE3
        || replay["compressed_sha256"] != D18_OBJECT_SHA256
        || replay["point_set_blake3"] != D18_POINT_SET_BLAKE3
        || replay["representatives_checked"] != 16_627
        || replay["points_checked"] != 2_760_082
        || replay["selected_best_total_runtime"] != Value::Null
        || custody["schema"] != "n83.d18-s3-custody/v1"
        || custody["status"] != "PASS"
        || custody["s3_uri"] != uri
        || custody["source_commit"] != D18_SOURCE_COMMIT
        || custody["curve_id"] != manifest["curve_id"]
        || custody["manifest_expected"]["blake3"] != D18_OBJECT_BLAKE3
        || custody["manifest_expected"]["sha256"] != D18_OBJECT_SHA256
        || custody["manifest_expected"]["bytes"] != 81_844_623u64
        || custody["local"] != custody["manifest_expected"]
        || custody["fresh_get"] != custody["manifest_expected"]
        || custody["head_content_length"] != 81_844_623u64
        || custody["server_side_encryption"] != "AES256"
    {
        return Err("d18 export, replay or S3 custody binding differs".into());
    }
    let compressed = fs::read(export_root.join(&object))?;
    if compressed.len() != 81_844_623
        || format!("{:x}", Sha256::digest(&compressed)) != D18_OBJECT_SHA256
    {
        return Err("d18 archive length or SHA-256 differs".into());
    }
    let row = json!({
        "a":0, "columns":16_627, "points":2_760_082,
        "policy":"direct_subgroup_x_d18", "seed":0,
        "object":object, "compressed_blake3":D18_OBJECT_BLAKE3,
        "plain_blake3":manifest["plain_blake3"],
        "point_set_blake3":D18_POINT_SET_BLAKE3
    });
    let base = from_record(&row, &load_object(export_root, &row)?)?;
    Ok((base, row))
}

pub(super) fn check_d18_export(export_root: &Path, evidence_root: &Path) -> Result<Value> {
    let started = Instant::now();
    let (base, row) = load_d18_export(export_root, evidence_root)?;
    Ok(json!({
        "schema":"n83.primary-d18-factor-base-adapter/v1", "status":"PASS",
        "curve_id":"icv1-f2m83-tm6151469093347-debefd74",
        "source_commit":D18_SOURCE_COMMIT, "policy":row["policy"],
        "adapter_source_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "object":row["object"], "compressed_blake3":D18_OBJECT_BLAKE3,
        "point_set_blake3":D18_POINT_SET_BLAKE3,
        "orbit_columns":16_627, "points":base.factor_base.points.len(),
        "frobenius_orbits":base.factor_base.orbits.len(),
        "signed_frobenius_orbits":base.factor_base.signed_orbits.len(),
        "import_elapsed_ms_informational_L0":started.elapsed().as_secs_f64()*1000.0,
        "solver_stage_executed":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    }))
}

pub(super) fn check_panel(root: &Path, columns: usize) -> Result<Value> {
    let (base, row, _) = load_panel(root, columns)?;
    Ok(json!({
        "schema": "n83.primary-factor-base-adapter/v1",
        "study": STUDY,
        "status": "PASS",
        "curve_a": 0,
        "subgroup_order": base.curve.subgroup_order.to_string(),
        "object": row["object"],
        "point_set_blake3": row["point_set_blake3"],
        "orbit_columns": columns,
        "points": base.factor_base.points.len(),
        "frobenius_orbits": base.factor_base.orbits.len(),
        "signed_frobenius_orbits": base.factor_base.signed_orbits.len(),
        "solver_stage_executed": false,
        "total_index_calculus_runtime_ms": Value::Null,
        "selected_best_total_runtime": Value::Null
    }))
}

/// Convert a checked summand witness into exact signed-orbit
/// coefficients. The retained primary base consists entirely of order-r
/// points, so replaying the row against its representatives must reproduce
/// the original target without a cofactor projection.
fn checked_primary_relation_row(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    indices: &[usize],
    target: &BinaryPoint,
) -> Result<Vec<BigUint>> {
    let r = &kc.subgroup_order;
    if r <= &BigUint::one() || fb.signed_orbit_of.len() != fb.points.len() {
        return Err("invalid primary relation domain".into());
    }
    let mut row = vec![BigUint::zero(); fb.unknowns()];
    let mut point_sum = BinaryPoint::Infinity;
    for &index in indices {
        let point = fb
            .points
            .get(index)
            .ok_or("relation point index out of range")?;
        let &(column, phase, negative) = fb
            .signed_orbit_of
            .get(index)
            .ok_or("relation point label absent")?;
        let rep_index = *fb
            .signed_orbits
            .get(column)
            .and_then(|orbit| orbit.first())
            .ok_or("relation orbit representative absent")?;
        let representative = fb
            .points
            .get(rep_index)
            .ok_or("relation representative index out of range")?;
        if phase >= kc.n || kc.mul(representative, r) != BinaryPoint::Infinity {
            return Err("relation representative outside the primary subgroup".into());
        }
        let positive = kc.lambda.modpow(&BigUint::from(phase), r);
        let coefficient = if negative && !positive.is_zero() {
            r - &positive
        } else {
            positive
        };
        if kc.mul(representative, &coefficient) != *point {
            return Err("relation point and signed-Frobenius label disagree".into());
        }
        row[column] = (&row[column] + coefficient) % r;
        point_sum = kc.add(&point_sum, point);
    }
    if point_sum != *target {
        return Err("relation point sum differs from target".into());
    }
    let mut row_sum = BinaryPoint::Infinity;
    for (column, coefficient) in row.iter().enumerate() {
        if coefficient.is_zero() {
            continue;
        }
        let rep_index = fb.signed_orbits[column][0];
        row_sum = kc.add(&row_sum, &kc.mul(&fb.points[rep_index], coefficient));
    }
    if row_sum != *target {
        return Err("relation row failed independent group replay".into());
    }
    Ok(row)
}

/// The container sees only a supervisor-made source snapshot, never the
/// checkout's Git metadata or unrelated files. The outer supervisor must
/// independently attest a clean checkout at this commit and retain the
/// snapshot hashes; this worker checks that the three files it executes
/// match its compiled bytes.
fn checked_primary_probe_source(root: &Path) -> Result<()> {
    for (path, compiled) in [
        (
            "examples/koblitz_n83_factor_base_export.rs",
            include_bytes!("../../examples/koblitz_n83_factor_base_export.rs").as_slice(),
        ),
        (
            "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
            include_bytes!("primary_adapter.rs").as_slice(),
        ),
        (
            "research/koblitz_n83_factor_base_sweep_20261008/compact_cold.rs",
            include_bytes!("compact_cold.rs").as_slice(),
        ),
    ] {
        if blake3::hash(&fs::read(root.join(path))?) != blake3::hash(compiled) {
            return Err("compiled S3 probe source differs from frozen source".into());
        }
    }
    Ok(())
}

fn frozen_primary_probe_source() -> Result<(String, &'static str)> {
    if let Some(path) = std::env::var_os("ICV1_FROZEN_SOURCE_DIR") {
        let root = std::path::PathBuf::from(path);
        let attestation: Value = serde_json::from_slice(&fs::read(root.join("attestation.json"))?)?;
        let commit = attestation["source_commit"]
            .as_str()
            .ok_or("source snapshot lacks a commit")?;
        if attestation["schema"] != "n83.primary-probe-source-attestation/v1"
            || attestation["status_clean"] != true
            || commit.len() != 40
            || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("invalid supervised source attestation".into());
        }
        checked_primary_probe_source(&root)?;
        Ok((commit.to_owned(), "supervised_snapshot"))
    } else {
        let commit = frozen_v2_source_commit()?;
        checked_primary_probe_source(Path::new(env!("CARGO_MANIFEST_DIR")))?;
        Ok((commit, "clean_checkout"))
    }
}

/// Pin the retained-base importer and both layers of the chained-S3 model.
/// The outer guard freezes these sources at a clean commit before launching
/// the Linux worker. The worker compares the snapshot to its compiled bytes.
fn frozen_chain_source() -> Result<(String, &'static str)> {
    let files: [(&str, &[u8]); 6] = [
        (
            "examples/koblitz_n83_factor_base_export.rs",
            include_bytes!("../../examples/koblitz_n83_factor_base_export.rs"),
        ),
        (
            "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
            include_bytes!("primary_adapter.rs"),
        ),
        (
            "src/cryptanalysis/binary_semaev_chain_sat.rs",
            include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs"),
        ),
        (
            "src/cryptanalysis/sat.rs",
            include_bytes!("../../src/cryptanalysis/sat.rs"),
        ),
        (
            "src/cryptanalysis/koblitz_index_calculus.rs",
            include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs"),
        ),
        (
            "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
            include_bytes!("size-frontier-v2.json"),
        ),
    ];
    let (root, commit, mode) = if let Some(path) = std::env::var_os("ICV1_FROZEN_SOURCE_DIR") {
        let root = std::path::PathBuf::from(path);
        let attestation: Value = serde_json::from_slice(&fs::read(root.join("attestation.json"))?)?;
        let commit = attestation["source_commit"]
            .as_str()
            .ok_or("chain source snapshot lacks a commit")?;
        if attestation["schema"] != "n83.chain-s3-source-attestation/v1"
            || attestation["status_clean"] != true
            || commit.len() != 40
            || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("invalid chained-S3 source attestation".into());
        }
        (root, commit.to_owned(), "supervised_snapshot")
    } else {
        (
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            frozen_v2_source_commit()?,
            "clean_checkout",
        )
    };
    for (path, compiled) in files {
        if blake3::hash(&fs::read(root.join(path))?) != blake3::hash(compiled) {
            return Err("compiled chained-S3 source differs from frozen source".into());
        }
    }
    Ok((commit, mode))
}

/// Construct one identity-pattern model from an exact replayed primary base.
/// This is a capacity diagnostic only: no SAT search or relation claim occurs.
/// A hard cgroup memory cap is mandatory here; the caller supplies the wall cap.
#[allow(clippy::too_many_arguments)]
pub(super) fn chain_capacity_cli(
    root: &Path,
    columns: usize,
    policy: &str,
    seed: u64,
    m: usize,
    identity_mask: u32,
    max_variables: u32,
    max_domain_clauses: usize,
    memory_mib: u64,
    output: &Path,
    frobenius_orbits: bool,
    order_first_pair: bool,
) -> Result<Value> {
    if !([64, 256, 600].contains(&columns) || V2_SIZES.contains(&columns))
        || !(4..=6).contains(&m)
        || identity_mask >= (1u32 << (m - 2))
        || max_variables == 0
        || max_domain_clauses == 0
        || (order_first_pair && !frobenius_orbits)
    {
        return Err("chained-S3 capacity parameters outside supported domain".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let start = Instant::now();
    let (base, row, manifest_hash) = load_panel_selected(root, columns, policy, seed)?;
    let base_import_ms = start.elapsed().as_secs_f64() * 1000.0;
    let (target, corpus_hash) = public_target(root, &base.curve)?;
    let target_validation_ms = start.elapsed().as_secs_f64() * 1000.0 - base_import_ms;
    let mut codes: Vec<BigUint> = base
        .factor_base
        .subspace
        .iter()
        .map(F2mElement::to_biguint)
        .collect();
    codes.sort_unstable();
    codes.dedup();
    if codes.len() != 83 * columns {
        return Err(
            "primary coordinate domain does not contain 83 distinct x values per column".into(),
        );
    }
    let representatives: Vec<BigUint> = if frobenius_orbits {
        base.factor_base
            .signed_orbits
            .iter()
            .map(|orbit| {
                match orbit
                    .first()
                    .and_then(|&index| base.factor_base.points.get(index))
                {
                    Some(BinaryPoint::Affine { x, .. }) => Ok(x.to_biguint()),
                    _ => Err("replayed primary orbit lacks an affine representative".into()),
                }
            })
            .collect::<Result<_>>()?
    } else {
        Vec::new()
    };
    let orbit_size = if frobenius_orbits {
        if representatives.len() != columns
            || representatives.iter().collect::<HashSet<_>>().len() != columns
        {
            return Err("primary orbit domain lacks distinct representatives".into());
        }
        Some(
            frobenius_orbit_domain_size(83, m, representatives.len())
                .ok_or("invalid Frobenius-orbit domain")?,
        )
    } else {
        None
    };
    let required_variables = chain_s3_max_variables(83, m)
        .ok_or("unsupported chained-S3 arity")?
        .checked_add(orbit_size.map_or(0, |size| size.variables))
        .and_then(|value| value.checked_add(if order_first_pair { 82 } else { 0 }))
        .ok_or("chained-S3 variable count overflow")?;
    let required_domain_clauses = if let Some(size) = orbit_size {
        size.clauses
    } else {
        finite_domain_clause_count(&codes, 83, m).ok_or("invalid chained-S3 domain")?
    };
    let model_start = Instant::now();
    let mut enc = None;
    let status = if required_variables > u64::from(max_variables) {
        "UNKNOWN_variable_cap"
    } else if required_domain_clauses > max_domain_clauses {
        "UNKNOWN_domain_clause_cap"
    } else {
        let target_x = match &target {
            BinaryPoint::Affine { x, .. } => Some(x),
            BinaryPoint::Infinity => None,
        };
        let mut built = ChainedS3Encoding::build(
            m,
            &base.curve.curve.irreducible,
            &base.curve.curve.b,
            target_x,
            identity_mask,
            max_variables,
        )
        .map_err(|_| "chained-S3 model build failed after preflight")?;
        if frobenius_orbits {
            built
                .constrain_frobenius_orbit_summands(
                    &representatives,
                    max_variables,
                    max_domain_clauses,
                )
                .map_err(|_| "chained-S3 orbit-domain install failed after preflight")?;
        } else {
            built
                .constrain_summands(&codes, max_domain_clauses)
                .map_err(|_| "chained-S3 finite-domain install failed after preflight")?;
        }
        if order_first_pair {
            built
                .order_first_pair(max_variables)
                .map_err(|_| "chained-S3 first-pair order failed after preflight")?;
        }
        enc = Some(built);
        "PASS_model_construction_only"
    };
    let model_construction_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    let cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()?;
    let mut receipt = json!({
        "schema":"n83.chain-s3-capacity-worker/v1", "study":STUDY, "status":status,
        "curve_a":0, "fixture":0, "orbit_columns":columns, "policy":policy, "seed":seed,
        "summands":m, "identity_mask":identity_mask, "object":row["object"],
        "domain_encoding":if order_first_pair { "frobenius_orbit_phase_ordered_first_pair" }
            else if frobenius_orbits { "frobenius_orbit_phase" } else { "coordinate_trie" },
        "domain_representatives":if frobenius_orbits { Some(representatives.len()) } else { None },
        "point_set_blake3":row["point_set_blake3"], "panel_manifest_blake3":manifest_hash,
        "public_corpus_canonical_json_blake3":corpus_hash,
        "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
        "source_exporter_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_chain_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs")).to_hex().to_string(),
        "source_index_calculus_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string()
    });
    let measurements = json!({
        "memory_cgroup_limit_bytes":memory_limit_bytes, "memory_cgroup_swap_limit_bytes":0,
        "memory_cgroup_peak_bytes":cgroup_peak_bytes,
        "max_variables":max_variables, "max_domain_clauses":max_domain_clauses,
        "required_max_variables":required_variables, "required_domain_clauses":required_domain_clauses,
        "legal_x_coordinates":codes.len(),
        "sat_variables":enc.as_ref().map(|e| e.solver.n_vars()),
        "sat_clauses":enc.as_ref().map(|e| e.solver.n_clauses()),
        "sat_xor_rows":enc.as_ref().map(|e| e.solver.n_xors()),
        "sat_and_gates":enc.as_ref().map(|e| e.n_and_gates),
        "sat_s3_nodes":enc.as_ref().map(|e| e.n_s3_nodes),
        "base_import_ms":base_import_ms, "target_validation_ms":target_validation_ms,
        "model_construction_ms":model_construction_ms,
        "process_wall_ms":start.elapsed().as_secs_f64()*1000.0, "process_usage":process_usage(),
        "solver_search_executed":false, "relation_stage_executed":false,
        "rank_stage_executed":false, "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    receipt
        .as_object_mut()
        .ok_or("invalid chained-S3 receipt")?
        .extend(
            measurements
                .as_object()
                .ok_or("invalid chained-S3 measurements")?
                .clone(),
        );
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

fn planted_point_json(point: &BinaryPoint) -> Value {
    match point {
        BinaryPoint::Infinity => json!({"infinity": true}),
        BinaryPoint::Affine { x, y } => json!({
            "x_decimal": x.to_biguint().to_string(),
            "y_decimal": y.to_biguint().to_string(),
        }),
    }
}

/// Find the at-most-two stored point lifts for each x coordinate without
/// allocating a second multi-million-point index. Check the complete group
/// equation before treating an x-coordinate model as a relation.
fn checked_four_x_lift(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    xs: &[F2mElement],
    target: &BinaryPoint,
) -> Result<Option<([usize; 4], Vec<BigUint>)>> {
    if xs.len() != 4 {
        return Err("four-summand model has the wrong x count".into());
    }
    let mut choices: [Vec<usize>; 4] = std::array::from_fn(|_| Vec::new());
    for (index, point) in fb.points.iter().enumerate() {
        if let BinaryPoint::Affine { x, .. } = point {
            for (slot, wanted) in xs.iter().enumerate() {
                if x == wanted {
                    choices[slot].push(index);
                    if choices[slot].len() > 2 {
                        return Err("stored base has more than two lifts of an x coordinate".into());
                    }
                }
            }
        }
    }
    if choices.iter().any(Vec::is_empty) {
        return Ok(None);
    }
    for &a in &choices[0] {
        for &b in &choices[1] {
            for &c in &choices[2] {
                for &d in &choices[3] {
                    let indices = [a, b, c, d];
                    let sum = indices.iter().fold(BinaryPoint::Infinity, |acc, &index| {
                        kc.add(&acc, &fb.points[index])
                    });
                    if sum == *target {
                        let row = checked_primary_relation_row(kc, fb, &indices, target)?;
                        return Ok(Some((indices, row)));
                    }
                }
            }
        }
    }
    Ok(None)
}

/// Five-point counterpart of the checked four-point lift. It accepts
/// only stored x coordinates and independently checks the full group sum.
fn checked_five_x_lift(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    xs: &[F2mElement],
    target: &BinaryPoint,
) -> Result<Option<([usize; 5], Vec<BigUint>)>> {
    if xs.len() != 5 {
        return Err("five-summand model has the wrong x count".into());
    }
    let mut choices: [Vec<usize>; 5] = std::array::from_fn(|_| Vec::new());
    for (index, point) in fb.points.iter().enumerate() {
        if let BinaryPoint::Affine { x, .. } = point {
            for (slot, wanted) in xs.iter().enumerate() {
                if x == wanted {
                    choices[slot].push(index);
                    if choices[slot].len() > 2 {
                        return Err("stored base has more than two lifts of an x coordinate".into());
                    }
                }
            }
        }
    }
    if choices.iter().any(Vec::is_empty) {
        return Ok(None);
    }
    for &a in &choices[0] {
        for &b in &choices[1] {
            for &c in &choices[2] {
                for &d in &choices[3] {
                    for &e in &choices[4] {
                        let indices = [a, b, c, d, e];
                        let sum = indices.iter().fold(BinaryPoint::Infinity, |acc, &index| {
                            kc.add(&acc, &fb.points[index])
                        });
                        if sum == *target {
                            let row = checked_primary_relation_row(kc, fb, &indices, target)?;
                            return Ok(Some((indices, row)));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

const K16627_OBJECT_BLAKE3: &str =
    "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d";
const K16627_POINT_SET_BLAKE3: &str =
    "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0";

struct WitnessPhases {
    file: File,
    path: PathBuf,
    start: Instant,
    next_sequence: u32,
    source_commit: String,
    schema: &'static str,
    object_blake3: &'static str,
    point_set_blake3: &'static str,
}

impl WitnessPhases {
    fn create(
        output: &Path,
        start: Instant,
        source_commit: String,
        schema: &'static str,
        object_blake3: &'static str,
        point_set_blake3: &'static str,
    ) -> Result<Self> {
        if output.exists() {
            return Err("complete-witness output already exists".into());
        }
        let mut path = output.as_os_str().to_os_string();
        path.push(".phases.jsonl");
        let path = PathBuf::from(path);
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        Ok(Self {
            file,
            path,
            start,
            next_sequence: 0,
            source_commit,
            schema,
            object_blake3,
            point_set_blake3,
        })
    }

    fn mark(&mut self, phase: &str, details: Value) -> Result<()> {
        let memory_current_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.current")?)?
            .trim()
            .parse()?;
        let record = json!({
            "schema":self.schema,
            "sequence":self.next_sequence,
            "phase":phase,
            "elapsed_wall_ms":self.start.elapsed().as_secs_f64()*1000.0,
            "memory_current_bytes":memory_current_bytes,
            "source_commit":self.source_commit,
            "object_blake3":self.object_blake3,
            "point_set_blake3":self.point_set_blake3,
            "details":details,
        });
        serde_json::to_writer(&mut self.file, &record)?;
        self.file.write_all(b"\n")?;
        self.file.flush()?;
        self.file.sync_all()?;
        self.next_sequence += 1;
        Ok(())
    }
}

fn root_phase_stats(stats: &SolverStats) -> Value {
    json!({
        "decisions":stats.decisions,
        "conflicts":stats.conflicts,
        "propagations":stats.propagations,
        "clause_visits":stats.clause_visits,
        "xor_passes":stats.xor_passes,
        "xor_repivots":stats.xor_repivots,
        "xor_row_ops":stats.xor_row_ops,
        "xor_row_scans":stats.xor_row_scans,
        "ns_propagate_clauses":stats.ns_propagate_clauses,
        "ns_propagate_xors":stats.ns_propagate_xors,
    })
}

fn write_progress_sample(
    file: &mut File,
    sequence: &mut u64,
    start: Instant,
    schema: &str,
    source_commit: &str,
    object_blake3: &str,
    point_set_blake3: &str,
    progress: &SolverProgress,
    sample_kind: &str,
) -> io::Result<()> {
    let memory_current_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.current")?)?
        .trim()
        .parse()
        .map_err(io::Error::other)?;
    let snapshot = progress.snapshot();
    let record = json!({
        "schema":schema,
        "sequence":*sequence,
        "sample_kind":sample_kind,
        "elapsed_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "memory_current_bytes":memory_current_bytes,
        "source_commit":source_commit,
        "object_blake3":object_blake3,
        "point_set_blake3":point_set_blake3,
        "phase":format!("{:?}",snapshot.phase),
        "completed_decisions":snapshot.decisions,
        "completed_conflicts":snapshot.conflicts,
        "completed_restarts":snapshot.restarts,
        "completed_xor_passes":snapshot.xor_passes,
        "completed_xor_nanoseconds":snapshot.xor_nanoseconds,
        "completed_xor_row_scans":snapshot.xor_row_scans,
        "completed_xor_repivots":snapshot.xor_repivots,
        "completed_xor_row_ops":snapshot.xor_row_ops,
    });
    serde_json::to_writer(&mut *file, &record).map_err(io::Error::other)?;
    file.write_all(b"\n")?;
    file.flush()?;
    file.sync_all()?;
    *sequence += 1;
    Ok(())
}

struct ProgressSampler {
    stop: mpsc::Sender<()>,
    join: thread::JoinHandle<io::Result<()>>,
}

impl ProgressSampler {
    fn start(
        output: &Path,
        start: Instant,
        schema: &'static str,
        source_commit: String,
        object_blake3: &'static str,
        point_set_blake3: &'static str,
        progress: Arc<SolverProgress>,
    ) -> Result<Self> {
        let mut path = output.as_os_str().to_os_string();
        path.push(".progress.jsonl");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(PathBuf::from(path))?;
        let mut sequence = 0;
        write_progress_sample(
            &mut file,
            &mut sequence,
            start,
            schema,
            &source_commit,
            object_blake3,
            point_set_blake3,
            &progress,
            "entered",
        )?;
        let (stop, receiver) = mpsc::channel();
        let join = thread::spawn(move || loop {
            match receiver.recv_timeout(Duration::from_secs(10)) {
                Err(mpsc::RecvTimeoutError::Timeout) => write_progress_sample(
                    &mut file,
                    &mut sequence,
                    start,
                    schema,
                    &source_commit,
                    object_blake3,
                    point_set_blake3,
                    &progress,
                    "periodic",
                )?,
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                    write_progress_sample(
                        &mut file,
                        &mut sequence,
                        start,
                        schema,
                        &source_commit,
                        object_blake3,
                        point_set_blake3,
                        &progress,
                        "returned",
                    )?;
                    return Ok(());
                }
            }
        });
        Ok(Self { stop, join })
    }

    fn finish(self) -> Result<()> {
        self.stop.send(())?;
        self.join
            .join()
            .map_err(|_| "progress sampler thread panicked")??;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OrbitChoicePins {
    None,
    SelectorAndPhase,
    SelectorOnly,
    PhaseOnly,
}

impl OrbitChoicePins {
    fn selector(self) -> bool {
        matches!(self, Self::SelectorAndPhase | Self::SelectorOnly)
    }

    fn phase(self) -> bool {
        matches!(self, Self::SelectorAndPhase | Self::PhaseOnly)
    }

    fn schema(self) -> &'static str {
        match self {
            Self::SelectorAndPhase => "n83.k16627-m4-selector-positive/v1",
            Self::SelectorOnly => "n83.k16627-m4-selector-only/v1",
            Self::PhaseOnly => "n83.k16627-m4-phase-only/v1",
            Self::None => "n83.k16627-m4-search-progress/v1",
        }
    }

    fn phase_schema(self) -> &'static str {
        match self {
            Self::SelectorAndPhase => "n83.k16627-m4-selector-positive-phase/v1",
            Self::SelectorOnly => "n83.k16627-m4-selector-only-phase/v1",
            Self::PhaseOnly => "n83.k16627-m4-phase-only-phase/v1",
            Self::None => "n83.k16627-m4-search-progress-phase/v1",
        }
    }
}

/// Phase-observable complete x-chain witness on the exact retained primary
/// object. The seven blocks are fixed only after the full orbit domain is
/// installed, and a SAT answer is checked against the original group law.
pub(super) fn chain_witness_cli(root: &Path, memory_mib: u64, output: &Path) -> Result<Value> {
    chain_witness_impl(
        root,
        memory_mib,
        output,
        false,
        false,
        OrbitChoicePins::None,
    )
}

/// The same complete witness, with a separate durable root-propagation gate.
pub(super) fn chain_root_phase_cli(root: &Path, memory_mib: u64, output: &Path) -> Result<Value> {
    chain_witness_impl(root, memory_mib, output, true, false, OrbitChoicePins::None)
}

/// The exact root-phase workload with separately fsynced search-progress
/// samples while the solver is active.
pub(super) fn chain_search_progress_cli(
    root: &Path,
    memory_mib: u64,
    output: &Path,
) -> Result<Value> {
    chain_witness_impl(root, memory_mib, output, true, true, OrbitChoicePins::None)
}

/// The full m=4 orbit model with exact planted representative selectors and
/// phase-zero bits fixed after the seven x blocks. This remains a positive
/// control; the original orbit-domain search is unchanged.
pub(super) fn chain_selector_positive_cli(
    root: &Path,
    memory_mib: u64,
    output: &Path,
) -> Result<Value> {
    chain_witness_impl(
        root,
        memory_mib,
        output,
        true,
        true,
        OrbitChoicePins::SelectorAndPhase,
    )
}

/// The same exact witness with representative selectors fixed but all
/// Frobenius phase wires free.
pub(super) fn chain_selector_only_cli(
    root: &Path,
    memory_mib: u64,
    output: &Path,
) -> Result<Value> {
    chain_witness_impl(
        root,
        memory_mib,
        output,
        true,
        true,
        OrbitChoicePins::SelectorOnly,
    )
}

/// The same exact witness with phase-zero units fixed but representative
/// selector wires free. Run only under the preregistered conditional arm.
pub(super) fn chain_phase_only_cli(root: &Path, memory_mib: u64, output: &Path) -> Result<Value> {
    chain_witness_impl(
        root,
        memory_mib,
        output,
        true,
        true,
        OrbitChoicePins::PhaseOnly,
    )
}

fn chain_witness_impl(
    root: &Path,
    memory_mib: u64,
    output: &Path,
    checkpoint_root: bool,
    sample_progress: bool,
    pins: OrbitChoicePins,
) -> Result<Value> {
    if memory_mib != 4096 {
        return Err("complete-witness control requires the frozen 4096 MiB cap".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let start = Instant::now();
    let phase_schema = if pins != OrbitChoicePins::None {
        pins.phase_schema()
    } else if sample_progress {
        "n83.k16627-m4-search-progress-phase/v1"
    } else if checkpoint_root {
        "n83.k16627-m4-root-phase/v1"
    } else {
        "n83.k16627-m4-witness-phase/v1"
    };
    let mut phases = WitnessPhases::create(
        output,
        start,
        source_commit.clone(),
        phase_schema,
        K16627_OBJECT_BLAKE3,
        K16627_POINT_SET_BLAKE3,
    )?;
    let (base, base_row, manifest_hash) =
        load_panel_selected(root, 16_627, "public_x_hash", 2026100801)?;
    if base_row["object"] != format!("objects/{K16627_OBJECT_BLAKE3}.jsonl.gz")
        || base_row["point_set_blake3"] != K16627_POINT_SET_BLAKE3
        || base.factor_base.signed_orbits.len() != 16_627
        || base.factor_base.points.len() != 2_760_082
    {
        return Err("complete-witness control imported a different retained object".into());
    }
    let base_import_ms = start.elapsed().as_secs_f64() * 1000.0;
    phases.mark("base_import_done", json!({"base_import_ms":base_import_ms}))?;

    let fb = &base.factor_base;
    let indices: [usize; 4] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
    let points: [&BinaryPoint; 4] = indices.map(|index| &fb.points[index]);
    let summand_xs: Vec<F2mElement> = points
        .iter()
        .map(|point| match point {
            BinaryPoint::Affine { x, .. } => Ok(x.clone()),
            BinaryPoint::Infinity => Err("planted orbit representative is infinity"),
        })
        .collect::<std::result::Result<_, _>>()?;
    let mut sum = BinaryPoint::Infinity;
    let mut sums = Vec::with_capacity(4);
    for point in points {
        sum = base.curve.add(&sum, point);
        sums.push(sum.clone());
    }
    let target = sum;
    let affine = sums
        .iter()
        .all(|point| matches!(point, BinaryPoint::Affine { .. }));
    let distinct = summand_xs
        .iter()
        .map(F2mElement::to_biguint)
        .collect::<HashSet<_>>()
        .len()
        == 4;
    if !affine || !distinct {
        let receipt = json!({
            "schema":if pins != OrbitChoicePins::None {pins.schema()}
                else if sample_progress {"n83.k16627-m4-search-progress/v1"}
                else if checkpoint_root {"n83.k16627-m4-root-phase/v1"}
                else {"n83.k16627-m4-witness/v1"},
            "status":"INVALID_PLANTED_CONTROL",
            "source_commit":source_commit, "object_blake3":K16627_OBJECT_BLAKE3,
            "point_set_blake3":K16627_POINT_SET_BLAKE3,
            "affine_partial_sums":affine, "distinct_summand_x":distinct,
            "selected_best_total_runtime":Value::Null,
        });
        write_new_json(output, &receipt)?;
        return Ok(receipt);
    }
    let preflight_row = checked_primary_relation_row(&base.curve, fb, &indices, &target)?;
    let mut preflight_rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
    let preflight_rank_after_insert = preflight_rank.insert(preflight_row.clone());
    let tamper_target = base.curve.add(&target, base.curve.generator());
    let tamper_rejected =
        checked_primary_relation_row(&base.curve, fb, &indices, &tamper_target).is_err();
    if preflight_rank_after_insert != 1 || !tamper_rejected {
        return Err("complete-witness group preflight failed".into());
    }
    let target_x = match &target {
        BinaryPoint::Affine { x, .. } => x.clone(),
        BinaryPoint::Infinity => return Err("complete-witness target is infinity".into()),
    };
    let sum2_x = match &sums[1] {
        BinaryPoint::Affine { x, .. } => x.clone(),
        BinaryPoint::Infinity => {
            return Err("complete-witness first intermediate is infinity".into())
        }
    };
    let sum3_x = match &sums[2] {
        BinaryPoint::Affine { x, .. } => x.clone(),
        BinaryPoint::Infinity => {
            return Err("complete-witness second intermediate is infinity".into())
        }
    };
    let planted_xs = [
        summand_xs[0].clone(),
        summand_xs[1].clone(),
        summand_xs[2].clone(),
        summand_xs[3].clone(),
        sum2_x,
        sum3_x,
        target_x.clone(),
    ];
    phases.mark(
        "group_preflight_done",
        json!({
            "point_indices":indices,
            "x_decimal":planted_xs.iter().map(|x| x.to_biguint().to_string()).collect::<Vec<_>>(),
            "rank_after_insert":preflight_rank_after_insert,
            "tamper_rejected":tamper_rejected,
        }),
    )?;

    let representatives: Vec<BigUint> = fb
        .signed_orbits
        .iter()
        .map(|orbit| match &fb.points[orbit[0]] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("complete-witness orbit domain has infinity"),
        })
        .collect::<std::result::Result<_, _>>()?;
    let model_start = Instant::now();
    let mut enc = ChainedS3Encoding::build(
        4,
        &base.curve.curve.irreducible,
        &base.curve.curve.b,
        Some(&target_x),
        0,
        250_000,
    )
    .map_err(|_| "complete-witness chained-S3 model build failed")?;
    enc.constrain_frobenius_orbit_summands(&representatives, 250_000, 6_000_000)
        .map_err(|_| "complete-witness orbit-domain install failed")?;
    if checkpoint_root
        && (enc.solver.n_vars() != 201_598
            || enc.solver.n_clauses() != 5_915_238
            || enc.solver.n_xors() != 3_901)
    {
        return Err("root-phase control constructed a different full orbit model".into());
    }
    let model_domain_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "model_domain_done",
        json!({
            "model_domain_ms":model_domain_ms,
            "sat_variables":enc.solver.n_vars(),
            "sat_clauses":enc.solver.n_clauses(),
            "sat_xor_rows":enc.solver.n_xors(),
        }),
    )?;
    for (block, x) in planted_xs.iter().enumerate() {
        enc.pin_x_block(block, x)
            .map_err(|_| "complete-witness x-block pin failed")?;
    }
    phases.mark("seven_blocks_pinned", json!({"x_blocks":7}))?;
    let mut orbit_choice_receipt = Value::Null;
    if pins != OrbitChoicePins::None {
        let mut sorted_representatives = representatives.clone();
        sorted_representatives.sort_unstable();
        if sorted_representatives
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err("selector control has duplicate representatives".into());
        }
        let wires = enc
            .orbit_choice_wires()
            .ok_or("selector control has no installed orbit-choice wires")?;
        if wires.len() != 4 {
            return Err("selector control has the wrong summand count".into());
        }
        let mut choices = Vec::with_capacity(4);
        for (slot, x) in summand_xs.iter().enumerate() {
            let value = x.to_biguint();
            let representative_index = sorted_representatives
                .binary_search(&value)
                .map_err(|_| "selector control planted x is not a representative")?;
            let handle = &wires[slot];
            if handle.selectors.len() != sorted_representatives.len() || handle.phase.len() != 7 {
                return Err("selector control allocated unexpected choice wires".into());
            }
            choices.push((
                slot,
                representative_index,
                handle.selectors[representative_index],
                handle.phase.clone(),
                value,
            ));
        }
        for (_, _, selector, phase, _) in &choices {
            if pins.selector() && !enc.solver.add_clause(vec![*selector as i32]) {
                return Err("orbit-choice selector unit contradicted model".into());
            }
            if pins.phase() {
                for &wire in phase {
                    if !enc.solver.add_clause(vec![-(wire as i32)]) {
                        return Err("orbit-choice phase-zero unit contradicted model".into());
                    }
                }
            }
        }
        orbit_choice_receipt = json!(choices
            .iter()
            .map(|(slot, index, selector, phase, value)| {
                json!({
                    "slot":slot,
                    "sorted_representative_index":index,
                    "selector_wire":selector,
                    "phase_wires":phase,
                    "phase":0,
                    "selector_pinned":pins.selector(),
                    "phase_pinned":pins.phase(),
                    "representative_x_decimal":value.to_string(),
                })
            })
            .collect::<Vec<_>>());
        phases.mark("orbit_choices_pinned", orbit_choice_receipt.clone())?;
    }
    enc.solver.conflict_budget = 100_000;
    enc.solver.deadline = Some(start + Duration::from_secs(1800));
    let root_result = if checkpoint_root {
        phases.mark(
            "root_propagation_entered",
            json!({"stats":root_phase_stats(&enc.solver.stats)}),
        )?;
        let root_start = Instant::now();
        let result = enc.solver.propagate_root();
        phases.mark(
            "root_propagation_returned",
            json!({
                "result":format!("{result:?}"),
                "root_propagation_ms":root_start.elapsed().as_secs_f64()*1000.0,
                "stats":root_phase_stats(&enc.solver.stats),
            }),
        )?;
        Some(result)
    } else {
        None
    };
    let (answer, solver_ms) = if root_result == Some(RootPropagationResult::Unsat) {
        (SolveResult::Unsat, 0.0)
    } else {
        phases.mark("sat_entered", json!({"conflict_budget":100_000}))?;
        let sampler = if sample_progress {
            let progress = Arc::new(SolverProgress::default());
            enc.solver.set_progress_observer(Arc::clone(&progress));
            Some(ProgressSampler::start(
                output,
                start,
                if pins != OrbitChoicePins::None {
                    pins.schema()
                } else {
                    "n83.k16627-m4-search-progress/v1"
                },
                source_commit.clone(),
                K16627_OBJECT_BLAKE3,
                K16627_POINT_SET_BLAKE3,
                progress,
            )?)
        } else {
            None
        };
        let solver_start = Instant::now();
        let answer = enc.solver.solve();
        if let Some(sampler) = sampler {
            sampler.finish()?;
        }
        (answer, solver_start.elapsed().as_secs_f64() * 1000.0)
    };
    let models = usize::from(answer == SolveResult::Sat);
    phases.mark(
        "sat_returned",
        json!({
            "answer":format!("{answer:?}"),
            "solver_ms":solver_ms,
            "conflicts":enc.solver.conflicts(),
            "models":models,
        }),
    )?;

    let mut relation = Value::Null;
    let status = match answer {
        SolveResult::Unknown => "UNKNOWN_search_cap",
        SolveResult::Unsat => "INVALID_UNSAT_PLANTED",
        SolveResult::Sat => {
            if !enc.verify_model() || enc.decode_x_blocks() != planted_xs {
                "INVALID_MODEL"
            } else if let Some((lifted_indices, lifted_row)) =
                checked_four_x_lift(&base.curve, fb, &summand_xs, &target)?
            {
                phases.mark(
                    "group_lift_done",
                    json!({
                        "point_indices":lifted_indices,
                        "group_verified":true,
                    }),
                )?;
                let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
                let rank_after_insert = rank.insert(lifted_row.clone());
                phases.mark("rank_done", json!({"rank_after_insert":rank_after_insert}))?;
                let matches_preflight_row = lifted_row == preflight_row;
                let dense: Vec<_> = lifted_row.iter().map(ToString::to_string).collect();
                relation = json!({
                    "point_indices":lifted_indices,
                    "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense)?).to_hex().to_string(),
                    "matches_preflight_row":matches_preflight_row,
                    "rank_after_insert":rank_after_insert,
                    "tamper_rejected":tamper_rejected,
                    "group_verified":true,
                });
                if rank_after_insert != 1 || !matches_preflight_row {
                    "INVALID_RELATION_CONTROL"
                } else {
                    match pins {
                        OrbitChoicePins::SelectorAndPhase => "PASS_SELECTOR_PINNED_WITNESS",
                        OrbitChoicePins::SelectorOnly => "PASS_SELECTOR_ONLY_WITNESS",
                        OrbitChoicePins::PhaseOnly => "PASS_PHASE_ONLY_WITNESS",
                        OrbitChoicePins::None => "PASS_COMPLETE_WITNESS",
                    }
                }
            } else {
                "INVALID_GROUP_LIFT"
            }
        }
    };
    let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()?;
    let mut receipt = json!({
        "schema":if pins != OrbitChoicePins::None {pins.schema()}
            else if sample_progress {"n83.k16627-m4-search-progress/v1"}
            else if checkpoint_root {"n83.k16627-m4-root-phase/v1"}
            else {"n83.k16627-m4-witness/v1"},
        "status":status,
        "curve_a":0, "orbit_columns":16_627, "summands":4, "identity_mask":0,
        "policy":"public_x_hash", "seed":2026100801u64,
        "object_blake3":K16627_OBJECT_BLAKE3,
        "point_set_blake3":K16627_POINT_SET_BLAKE3,
        "panel_manifest_blake3":manifest_hash,
        "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
        "source_exporter_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_chain_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs")).to_hex().to_string(),
        "source_index_calculus_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string(),
        "phase_file":phases.path.to_string_lossy(),
        "progress_file":if sample_progress {
            json!(format!("{}.progress.jsonl",output.display()))
        } else {Value::Null},
        "point_indices":indices, "x_decimal":planted_xs.iter().map(|x| x.to_biguint().to_string()).collect::<Vec<_>>(),
        "relation":relation,
        "max_variables":250_000, "max_domain_clauses":6_000_000,
        "conflict_budget":100_000, "max_models":1,
        "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(), "solver_models":models,
        "solver_conflicts":enc.solver.conflicts(),
        "base_import_ms":base_import_ms, "model_domain_ms":model_domain_ms,
        "solver_ms":solver_ms, "process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_swap_limit_bytes":0,
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        "process_usage":process_usage(),
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    if pins != OrbitChoicePins::None {
        receipt["orbit_choice_pins"] = orbit_choice_receipt;
    }
    if checkpoint_root {
        receipt["source_sat_blake3"] = Value::String(
            blake3::hash(include_bytes!("../../src/cryptanalysis/sat.rs"))
                .to_hex()
                .to_string(),
        );
        receipt["root_propagation_result"] = root_result
            .map(|result| Value::String(format!("{result:?}")))
            .unwrap_or(Value::Null);
    }
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

/// Bound a constructed-target SAT control to the exact replayed K=16,627
/// primary object. The pinned arm fixes four x wires only after the complete
/// orbit-domain model is installed. The unpinned arm uses the same target and
/// model, with no access to those fixed values in its solver clauses.
pub(super) fn chain_planted_cli(
    root: &Path,
    pinned: bool,
    memory_mib: u64,
    output: &Path,
) -> Result<Value> {
    if memory_mib != 4096 {
        return Err("planted control requires the frozen 4096 MiB cap".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let start = Instant::now();
    let (base, base_row, manifest_hash) =
        load_panel_selected(root, 16_627, "public_x_hash", 2026100801)?;
    let base_import_ms = start.elapsed().as_secs_f64() * 1000.0;
    if base_row["object"]
        != "objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz"
        || base_row["point_set_blake3"]
            != "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"
    {
        return Err("planted control imported a different retained object".into());
    }
    let fb = &base.factor_base;
    if fb.signed_orbits.len() != 16_627 || fb.points.len() != 2_760_082 {
        return Err("planted control imported the wrong retained base size".into());
    }
    let planted_indices: [usize; 4] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
    let planted_points: [&BinaryPoint; 4] = planted_indices.map(|index| &fb.points[index]);
    let planted_xs: Vec<F2mElement> = planted_points
        .iter()
        .map(|point| match point {
            BinaryPoint::Affine { x, .. } => Ok(x.clone()),
            BinaryPoint::Infinity => Err("planted orbit representative is infinity"),
        })
        .collect::<std::result::Result<_, _>>()?;
    let mut partial = BinaryPoint::Infinity;
    let mut sums = Vec::with_capacity(4);
    for point in planted_points {
        partial = base.curve.add(&partial, point);
        sums.push(partial.clone());
    }
    let target = partial;
    let control_valid = planted_xs
        .iter()
        .map(F2mElement::to_biguint)
        .collect::<HashSet<_>>()
        .len()
        == 4
        && sums
            .iter()
            .all(|point| matches!(point, BinaryPoint::Affine { .. }));
    let target_json = json!({
        "columns": [0,1,2,3],
        "point_indices": planted_indices,
        "points": planted_points.map(|point| planted_point_json(point)),
        "partial_sums": sums.iter().map(planted_point_json).collect::<Vec<_>>(),
        "target": planted_point_json(&target),
    });
    if !control_valid {
        let receipt = json!({
            "schema":"n83.k16627-m4-planted/v1", "status":"INVALID_PLANTED_CONTROL",
            "arm":if pinned {"pinned"} else {"unpinned"},
            "object":base_row["object"], "point_set_blake3":base_row["point_set_blake3"],
            "source_commit":source_commit, "planted":target_json,
            "base_import_ms":base_import_ms,
            "solver_search_executed":false,
            "selected_best_total_runtime":Value::Null,
        });
        write_new_json(output, &receipt)?;
        return Ok(receipt);
    }
    let mut codes: Vec<BigUint> = fb.subspace.iter().map(F2mElement::to_biguint).collect();
    codes.sort_unstable();
    codes.dedup();
    if codes.len() != 83 * 16_627 {
        return Err("planted control coordinate closure mismatch".into());
    }
    let representatives: Vec<BigUint> = fb
        .signed_orbits
        .iter()
        .map(|orbit| match &fb.points[orbit[0]] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("planted orbit domain has infinity"),
        })
        .collect::<std::result::Result<_, _>>()?;
    let target_x = match &target {
        BinaryPoint::Affine { x, .. } => x,
        BinaryPoint::Infinity => return Err("invalid planted target".into()),
    };
    let model_start = Instant::now();
    let mut enc = ChainedS3Encoding::build(
        4,
        &base.curve.curve.irreducible,
        &base.curve.curve.b,
        Some(target_x),
        0,
        250_000,
    )
    .map_err(|_| "planted chained-S3 model build failed")?;
    enc.constrain_frobenius_orbit_summands(&representatives, 250_000, 6_000_000)
        .map_err(|_| "planted orbit-domain install failed")?;
    if pinned {
        // The chained-S3 builder allocates the first m*n wires for the
        // summand x blocks; the core's own model blocker uses this layout.
        for (slot, x) in planted_xs.iter().enumerate() {
            let value = x.to_biguint();
            for bit in 0..83usize {
                let wire = (slot * 83 + bit + 1) as i32;
                enc.solver
                    .add_clause(vec![if value.bit(bit as u64) { wire } else { -wire }]);
            }
        }
    }
    let model_construction_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    let conflict_budget = if pinned { 100_000 } else { 5_000 };
    let max_models = if pinned { 1 } else { 8 };
    enc.solver.conflict_budget = conflict_budget;
    enc.solver.deadline = Some(start + Duration::from_secs(if pinned { 1800 } else { 900 }));
    let solver_start = Instant::now();
    let mut models = 0usize;
    let mut invalid_models = 0usize;
    let mut group_rejected_models = 0usize;
    let (status, relation) = loop {
        match enc.solver.solve() {
            SolveResult::Unknown => break ("UNKNOWN_search_cap", None),
            // The frozen four points are a witness for this exact target
            // and all-affine identity pattern in either arm.
            SolveResult::Unsat => break ("INVALID_UNSAT_PLANTED", None),
            SolveResult::Sat => {
                models += 1;
                let xs = enc.decode_summands();
                if !enc.verify_model()
                    || xs.len() != 4
                    || xs
                        .iter()
                        .any(|x| codes.binary_search(&x.to_biguint()).is_err())
                    || (pinned && xs != planted_xs)
                {
                    invalid_models += 1;
                    break ("INVALID_MODEL", None);
                }
                if let Some((indices, row)) = checked_four_x_lift(&base.curve, fb, &xs, &target)? {
                    let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
                    let rank_after_insert = rank.insert(row.clone());
                    let tamper_rejected = !pinned
                        || checked_primary_relation_row(
                            &base.curve,
                            fb,
                            &indices,
                            &base.curve.add(&target, base.curve.generator()),
                        )
                        .is_err();
                    if rank_after_insert != 1 || !tamper_rejected {
                        break ("INVALID_RELATION_CONTROL", None);
                    }
                    let nonzero: Vec<_> = row
                        .iter()
                        .enumerate()
                        .filter(|(_, coefficient)| !coefficient.is_zero())
                        .map(|(column, coefficient)| {
                            json!({
                                "column":column, "coefficient_decimal":coefficient.to_string(),
                            })
                        })
                        .collect();
                    let dense: Vec<_> = row.iter().map(ToString::to_string).collect();
                    break (
                        if pinned {
                            "PASS_PLANTED_RELATION"
                        } else {
                            "HIT_UNPINNED_PLANTED_TARGET"
                        },
                        Some(json!({
                            "point_indices":indices, "nonzero":nonzero,
                            "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense)?).to_hex().to_string(),
                            "rank_after_insert":rank_after_insert,
                            "tamper_rejected":if pinned { Some(tamper_rejected) } else { None },
                            "group_verified":true,
                        })),
                    );
                }
                group_rejected_models += 1;
                if models >= max_models {
                    break ("UNKNOWN_model_cap", None);
                }
                let model = enc.solver.model();
                enc.solver.reset_search();
                enc.solver.add_clause(
                    (0..4 * 83usize)
                        .map(|i| {
                            let wire = (i + 1) as i32;
                            if model[i] {
                                -wire
                            } else {
                                wire
                            }
                        })
                        .collect(),
                );
            }
        }
    };
    let solver_ms = solver_start.elapsed().as_secs_f64() * 1000.0;
    let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()?;
    let receipt = json!({
        "schema":"n83.k16627-m4-planted/v1", "status":status,
        "arm":if pinned {"pinned"} else {"unpinned"},
        "curve_a":0, "orbit_columns":16_627, "summands":4, "identity_mask":0,
        "policy":"public_x_hash", "seed":2026100801u64,
        "object":base_row["object"], "point_set_blake3":base_row["point_set_blake3"],
        "panel_manifest_blake3":manifest_hash,
        "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
        "source_exporter_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_chain_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs")).to_hex().to_string(),
        "source_index_calculus_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string(),
        "planted":target_json, "relation":relation,
        "max_variables":250_000, "max_domain_clauses":6_000_000,
        "conflict_budget":conflict_budget, "max_models":max_models,
        "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(), "solver_models":models,
        "invalid_models":invalid_models, "group_rejected_models":group_rejected_models,
        "solver_conflicts":enc.solver.conflicts(),
        "base_import_ms":base_import_ms, "model_construction_ms":model_construction_ms,
        "solver_ms":solver_ms, "process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_swap_limit_bytes":0,
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        "process_usage":process_usage(),
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

/// Probe the published primary point with a replayed retained base through
/// the point-only wide compact index. A hit is also converted to a full-width
/// relation row and independently replayed. This is one relation diagnostic,
/// not a rank or total-runtime result. A Linux cgroup with a hard memory limit and
/// zero swap is mandatory; the caller must also impose a process-wall cap.
pub(super) fn s3_probe_cli(
    root: &Path,
    columns: usize,
    policy: &str,
    seed: u64,
    unordered_pairs: bool,
    max_candidate_states: usize,
    memory_mib: u64,
    output: &Path,
) -> Result<Value> {
    if ![64, 256, 600].contains(&columns) {
        return Err("S3 probe supports retained primary K=64/256/600 bases".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let (source_commit, source_attestation_mode) = frozen_primary_probe_source()?;
    let started = Instant::now();
    let (base, row, manifest_hash) = load_panel_selected(root, columns, policy, seed)?;
    let base_import_ms = started.elapsed().as_secs_f64() * 1000.0;
    let (target, corpus_hash) = public_target(root, &base.curve)?;
    let target_validation_ms = started.elapsed().as_secs_f64() * 1000.0 - base_import_ms;
    let representatives: Vec<_> = base
        .factor_base
        .signed_orbits
        .iter()
        .map(|orbit| base.factor_base.points[orbit[0]].clone())
        .collect();
    let probe_started = Instant::now();
    let probe = compact_cold::probe_checked_four_sum(
        &base.curve,
        &base.factor_base.points,
        &representatives,
        &target,
        unordered_pairs,
        max_candidate_states,
    )?;
    let probe_ms = probe_started.elapsed().as_secs_f64() * 1000.0;
    let row_started = Instant::now();
    let relation_row = if probe["status"] == "HIT" {
        let indices: [usize; 4] = probe["point_indices"]
            .as_array()
            .ok_or("compact hit lacks point indices")?
            .iter()
            .map(|value| {
                value
                    .as_u64()
                    .and_then(|index| usize::try_from(index).ok())
                    .ok_or("compact hit has invalid point index")
            })
            .collect::<std::result::Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "compact hit is not a four-summand witness")?;
        let row = checked_primary_relation_row(&base.curve, &base.factor_base, &indices, &target)?;
        let dense_decimal: Vec<_> = row.iter().map(ToString::to_string).collect();
        let nonzero: Vec<_> = row
            .iter()
            .enumerate()
            .filter(|(_, coefficient)| !coefficient.is_zero())
            .map(|(column, coefficient)| {
                json!({"column":column,"coefficient_decimal":coefficient.to_string()})
            })
            .collect();
        Some(json!({
            "schema":"n83.primary-wide-relation-row/v1",
            "modulus_decimal":base.curve.subgroup_order.to_string(),
            "orbit_columns":columns,
            "nonzero":nonzero,
            "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense_decimal)?).to_hex().to_string(),
            "group_verified":true
        }))
    } else if probe["status"] == "MISS" {
        None
    } else {
        return Err("compact probe returned an unknown status".into());
    };
    let relation_row_stage_executed = relation_row.is_some();
    let relation_row_ms = row_started.elapsed().as_secs_f64() * 1000.0;
    let cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()
        .map_err(|_| "S3 probe requires a numeric cgroup memory.peak")?;
    let receipt = json!({
        "schema":"n83.primary-compact-s3-probe/v1",
        "study":STUDY,
        "status":probe["status"],
        "curve_a":0,
        "fixture":0,
        "orbit_columns":columns,
        "policy":policy,
        "seed":seed,
        "object":row["object"],
        "point_set_blake3":row["point_set_blake3"],
        "panel_manifest_blake3":manifest_hash,
        "public_corpus_canonical_json_blake3":corpus_hash,
        "source_commit":source_commit,
        "source_attestation_mode":source_attestation_mode,
        "source_exporter_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_compact_blake3":blake3::hash(include_bytes!("compact_cold.rs")).to_hex().to_string(),
        "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_swap_limit_bytes":0,
        "memory_cgroup_peak_bytes":cgroup_peak_bytes,
        "max_candidate_states":max_candidate_states,
        "base_import_ms":base_import_ms,
        "target_validation_ms":target_validation_ms,
        "point_probe_ms":probe_ms,
        "relation_row_ms":relation_row_ms,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(),
        "probe":probe,
        "relation_row":relation_row,
        "relation_row_stage_executed":relation_row_stage_executed,
        "rank_stage_executed":false,
        "column_log_verification":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

/// Construct the exact K=64/256/600 finite-domain factored S4 model and
/// stop before search. The caller must impose and independently verify a
/// hard process-wall limit; this worker refuses an absent cgroup memory cap.
pub(super) fn build_capacity_cli(
    root: &Path,
    columns: usize,
    memory_mib: u64,
    output: &Path,
) -> Result<Value> {
    if ![64, 256, 600].contains(&columns) {
        return Err("capacity gate supports only retained primary K=64/256/600 bases".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let start = Instant::now();
    let (base, row, manifest_hash) = load_panel(root, columns)?;
    let base_import_ms = start.elapsed().as_secs_f64() * 1000.0;
    let (target, corpus_hash) = public_target(root, &base.curve)?;
    let target_validation_ms = start.elapsed().as_secs_f64() * 1000.0 - base_import_ms;
    let model_start = Instant::now();
    let enc = build_union_s4_encoding(
        &base.curve,
        &base.factor_base,
        &target,
        SatDecompositionOptions {
            factored_s4: true,
            ..Default::default()
        },
    )?;
    let model_construction_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    let cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()
        .map_err(|_| "capacity worker requires a numeric cgroup memory.peak")?;
    let receipt = json!({
        "schema":"n83.factored-s4-capacity-worker/v1",
        "study":STUDY,
        "status":"PASS_model_construction_only",
        "curve_a":0,
        "fixture":0,
        "orbit_columns":columns,
        "object":row["object"],
        "point_set_blake3":row["point_set_blake3"],
        "panel_manifest_blake3":manifest_hash,
        "public_corpus_canonical_json_blake3":corpus_hash,
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_union_builder_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string(),
        "source_factored_encoder_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/semaev_sat.rs")).to_hex().to_string(),
        "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_swap_limit_bytes":0,
        "memory_cgroup_peak_bytes":cgroup_peak_bytes,
        "base_points":base.factor_base.points.len(),
        "legal_x_coordinates":base.factor_base.subspace.len(),
        "sat_variables":enc.solver.n_vars(),
        "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(),
        "sat_x_variables":enc.n_x_vars,
        "sat_e_variables":enc.n_e_vars,
        "sat_aux_variables":enc.n_aux_vars,
        "trivially_unsat":enc.trivially_unsat,
        "base_import_ms":base_import_ms,
        "target_validation_ms":target_validation_ms,
        "model_construction_ms":model_construction_ms,
        "process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(),
        "solver_search_executed":false,
        "model_lifting_executed":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

fn public_target(root: &Path, kc: &KoblitzCurve) -> Result<(BinaryPoint, String)> {
    public_target_fixture(root, kc, 0)
}

fn public_target_fixture(
    root: &Path,
    kc: &KoblitzCurve,
    fixture_index: u64,
) -> Result<(BinaryPoint, String)> {
    if fixture_index > 63 {
        return Err("public fixture index outside frozen corpus".into());
    }
    let corpus: Value = serde_json::from_slice(&fs::read(root.join("probe-corpus.json"))?)?;
    if corpus["schema"] != "n83.public-probe-corpus/v1" || corpus["study"] != STUDY {
        return Err("public target corpus mismatch".into());
    }
    let targets = corpus["targets"].as_array().ok_or("public targets")?;
    let mut selected = targets
        .iter()
        .filter(|row| row["a"] == 0 && row["fixture"].as_u64() == Some(fixture_index));
    let row = selected.next().ok_or("primary public fixture absent")?;
    if selected.next().is_some() {
        return Err("ambiguous primary public fixture".into());
    }
    let encoded = parse_point(&row["point"])?.ok_or("identity public target")?;
    let target = general(Some(encoded));
    if !kc.curve.is_on_curve(&target)
        || kc.mul(&target, &kc.subgroup_order) != BinaryPoint::Infinity
    {
        return Err("primary public target fails group validation".into());
    }
    let corpus_hash = blake3::hash(&serde_json::to_vec(&corpus)?)
        .to_hex()
        .to_string();
    Ok((target, corpus_hash))
}

fn candidate_matches_validation(
    root: &Path,
    corpus_hash: &str,
    candidate: &BigUint,
) -> Result<bool> {
    let validation: Value = serde_json::from_slice(&fs::read(root.join("probe-validation.json"))?)?;
    if validation["schema"] != "n83.public-probe-validation/v1"
        || validation["corpus_canonical_json_blake3"] != corpus_hash
        || validation["oracle_receives_known_answer_scalars"] != false
    {
        return Err("public validation sidecar mismatch".into());
    }
    let entries = validation["validation"]
        .as_array()
        .ok_or("validation entries")?;
    let mut selected = entries
        .iter()
        .filter(|row| row["a"] == 0 && row["fixture"] == 0);
    let row = selected.next().ok_or("primary validation fixture absent")?;
    if selected.next().is_some() {
        return Err("ambiguous primary validation fixture".into());
    }
    let expected: BigUint = row["known_answer_scalar"]
        .as_str()
        .ok_or("known-answer encoding")?
        .parse()?;
    Ok(*candidate == expected)
}

/// Run one bounded primary workload. The public target is the solver's only
/// target input; its known-answer sidecar is opened only after a candidate log.
#[allow(clippy::too_many_arguments)]
pub(super) fn run_cli(
    root: &Path,
    columns: usize,
    m: usize,
    strategy_name: &str,
    max_trials: usize,
    budget_seconds: u64,
    run_dir: &Path,
) -> Result<Value> {
    run_with_config(
        root,
        columns,
        m,
        strategy_name,
        max_trials,
        budget_seconds,
        run_dir,
        None,
    )
}

struct ChainRunConfig<'a> {
    policy: &'a str,
    seed: u64,
    fixture_root: &'a Path,
    limits: ChainS3Limits,
    memory_limit_bytes: u64,
    source_commit: String,
    source_attestation_mode: &'static str,
}

/// Execute one replay-bound primary chained-S3 search only inside a verified
/// hard-memory cgroup. The outer supervisor owns the separate wall deadline.
#[allow(clippy::too_many_arguments)]
pub(super) fn run_chain_cli(
    root: &Path,
    fixture_root: &Path,
    columns: usize,
    policy: &str,
    seed: u64,
    m: usize,
    max_trials: usize,
    limits: ChainS3Limits,
    budget_seconds: u64,
    memory_mib: u64,
    run_dir: &Path,
    frobenius_orbits: bool,
    order_first_pair: bool,
) -> Result<Value> {
    if ![64, 256, 600].contains(&columns) && !V2_SIZES.contains(&columns)
        || !fixture_root.join("probe-corpus.json").is_file()
        || !fixture_root.join("probe-validation.json").is_file()
        || ![
            "public_x_sequential",
            "public_x_hash",
            "public_x_gray_prefix",
        ]
        .contains(&policy)
        || !SEEDS.contains(&seed)
        || !(4..=6).contains(&m)
        || max_trials > 1000
        || limits.max_variables == 0
        || limits.max_variables > i32::MAX as u32
        || limits.max_domain_clauses == 0
        || limits.max_domain_clauses > i32::MAX as usize
        || limits.max_models == 0
        || limits.max_models > 1_000_000
        || limits.conflict_budget == 0
        || limits.conflict_budget > 10_000_000_000
        || budget_seconds == 0
        || budget_seconds > 7200
        || memory_mib < 128
        || memory_mib > 65536
        || (order_first_pair && !frobenius_orbits)
    {
        return Err(
            "primary chained-S3 search caps or base selection outside supported domain".into(),
        );
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let config = ChainRunConfig {
        policy,
        seed,
        fixture_root,
        limits,
        memory_limit_bytes,
        source_commit,
        source_attestation_mode,
    };
    let strategy = match (frobenius_orbits, order_first_pair) {
        (false, false) => "chain-s3",
        (true, false) => "orbit-chain-s3",
        (true, true) => "ordered-orbit-chain-s3",
        (false, true) => unreachable!("checked above"),
    };
    run_with_config(
        root,
        columns,
        m,
        strategy,
        max_trials,
        budget_seconds,
        run_dir,
        Some(&config),
    )
}

/// Same native m=4 model and verifier for the two frozen K=16,627 point sets.
/// The supervisor supplies the independent 1,800-second wall cap and the
/// read-only, one-CPU, 4-GiB, zero-swap container envelope.
pub(super) fn paired_orbit_cli(
    mode: &str,
    arm: &str,
    root: &Path,
    evidence_root: &Path,
    fixture_root: &Path,
    output: &Path,
) -> Result<Value> {
    if !["preflight", "positive", "natural"].contains(&mode) || !["U", "D"].contains(&arm) {
        return Err("paired mode or arm outside frozen protocol".into());
    }
    if output.exists() {
        return Err("paired output already exists".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(4096)?;
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let binary_sha256 = file_sha256(&std::env::current_exe()?)?;
    let started = Instant::now();
    let cap_path = PathBuf::from(format!("{}.cap.json", output.display()));
    let cap_arm = arm.to_owned();
    let cap_mode = mode.to_owned();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(1790));
        let cap = json!({
            "schema":"n83.paired-orbit-cap/v1", "status":"UNKNOWN_budget",
            "arm":cap_arm, "mode":cap_mode,
            "budget_seconds":1790,
            "elapsed_process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "selected_best_total_runtime":Value::Null
        });
        if write_new_json(&cap_path, &cap).is_err() {
            std::process::exit(125);
        }
        std::process::exit(124);
    });
    let import_start = Instant::now();
    let (base, row, manifest_hash, object_blake3, point_set_blake3) =
        load_paired_base(arm, root, evidence_root)?;
    let import_ms = import_start.elapsed().as_secs_f64() * 1000.0;
    if file_sha256(&fixture_root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256 {
        return Err("paired public corpus differs".into());
    }
    let target_start = Instant::now();
    let (public, corpus_hash) = public_target(fixture_root, &base.curve)?;
    let target_validation_ms = target_start.elapsed().as_secs_f64() * 1000.0;
    if planted_point_json(&public)
        != json!({
            "x_decimal":"1834913121178197242856238",
            "y_decimal":"8797964190732846213981377"
        })
    {
        return Err("paired public fixture point differs".into());
    }
    let fb = &base.factor_base;
    if fb.points.len() != 2_760_082 || fb.signed_orbits.len() != 16_627 {
        return Err("paired factor-base dimensions differ".into());
    }
    if mode == "preflight" {
        let receipt = json!({
            "schema":"n83.paired-orbit-worker/v1", "status":"PASS_PREFLIGHT",
            "arm":arm, "mode":mode, "source_commit":source_commit,
            "source_attestation_mode":source_attestation_mode,
            "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
            "source_example_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
            "binary_sha256":binary_sha256, "object_blake3":object_blake3,
            "point_set_blake3":point_set_blake3, "manifest_blake3":manifest_hash,
            "policy":row["policy"], "seed":row["seed"],
            "public_corpus_blake3":corpus_hash, "public_point":planted_point_json(&public),
            "columns":fb.signed_orbits.len(), "points":fb.points.len(),
            "memory_cgroup_limit_bytes":memory_limit_bytes,
            "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
            "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "solver_stage_executed":false,
            "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null
        });
        write_new_json(output, &receipt)?;
        return Ok(receipt);
    }

    let prepare_start = Instant::now();
    let representatives: Vec<BigUint> = fb
        .signed_orbits
        .iter()
        .map(|orbit| match &fb.points[orbit[0]] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("paired orbit representative is infinity"),
        })
        .collect::<std::result::Result<_, _>>()?;
    let mut positive_indices = None;
    let mut positive_xs = None;
    let mut positive_row = None;
    let target = if mode == "positive" {
        let indices: [usize; 4] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
        let mut sum = BinaryPoint::Infinity;
        let mut partial = Vec::with_capacity(4);
        let mut summand_xs = Vec::with_capacity(4);
        for &index in &indices {
            let BinaryPoint::Affine { x, .. } = &fb.points[index] else {
                return Err("positive representative is infinity".into());
            };
            summand_xs.push(x.clone());
            sum = base.curve.add(&sum, &fb.points[index]);
            partial.push(sum.clone());
        }
        if partial
            .iter()
            .any(|point| !matches!(point, BinaryPoint::Affine { .. }))
            || summand_xs
                .iter()
                .map(F2mElement::to_biguint)
                .collect::<HashSet<_>>()
                .len()
                != 4
        {
            return Err("paired positive control is not affine and distinct".into());
        }
        let row = checked_primary_relation_row(&base.curve, fb, &indices, &sum)?;
        let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
        let tamper = base.curve.add(&sum, base.curve.generator());
        if rank.insert(row.clone()) != 1
            || checked_primary_relation_row(&base.curve, fb, &indices, &tamper).is_ok()
        {
            return Err("paired positive group/rank/tamper preflight failed".into());
        }
        positive_indices = Some(indices);
        positive_xs = Some((summand_xs, partial));
        positive_row = Some(row);
        sum
    } else {
        public.clone()
    };
    let BinaryPoint::Affine { x: target_x, .. } = &target else {
        return Err("paired target is infinity".into());
    };
    let target_hash = blake3::hash(&serde_json::to_vec(&planted_point_json(&target))?)
        .to_hex()
        .to_string();
    let preparation_ms = prepare_start.elapsed().as_secs_f64() * 1000.0;
    let mut phases = WitnessPhases::create(
        output,
        started,
        source_commit.clone(),
        "n83.paired-orbit-phase/v1",
        object_blake3,
        point_set_blake3,
    )?;
    phases.mark(
        "inputs_checked",
        json!({
            "arm":arm, "mode":mode, "binary_sha256":binary_sha256,
            "target_blake3":target_hash, "public_corpus_blake3":corpus_hash,
            "manifest_blake3":manifest_hash, "base_import_ms":import_ms,
            "target_validation_ms":target_validation_ms,
            "domain_preparation_ms":preparation_ms,
        }),
    )?;
    let model_start = Instant::now();
    let mut enc = ChainedS3Encoding::build(
        4,
        &base.curve.curve.irreducible,
        &base.curve.curve.b,
        Some(target_x),
        0,
        300_000,
    )
    .map_err(|_| "paired chained-S3 model build failed")?;
    enc.constrain_frobenius_orbit_summands(&representatives, 300_000, 8_000_000)
        .map_err(|_| "paired orbit-domain install failed")?;
    let model_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "model_domain_done",
        json!({
            "model_ms":model_ms, "variables":enc.solver.n_vars(),
            "clauses":enc.solver.n_clauses(), "xor_rows":enc.solver.n_xors(),
        }),
    )?;
    let pin_start = Instant::now();
    if let Some((summand_xs, partial)) = &positive_xs {
        let mut pinned = summand_xs.clone();
        for point in &partial[1..3] {
            let BinaryPoint::Affine { x, .. } = point else {
                return Err("positive partial sum is infinity".into());
            };
            pinned.push(x.clone());
        }
        // Four summands, x(S2), x(S3), x(target): seven named blocks.
        let pinned = [
            pinned[0].clone(),
            pinned[1].clone(),
            pinned[2].clone(),
            pinned[3].clone(),
            pinned[4].clone(),
            pinned[5].clone(),
            target_x.clone(),
        ];
        for (block, x) in pinned.iter().enumerate() {
            enc.pin_x_block(block, x)
                .map_err(|_| "positive x-block pin failed")?;
        }
        let mut sorted = representatives.clone();
        sorted.sort_unstable();
        let handles = enc
            .orbit_choice_wires()
            .ok_or("positive selector wires absent")?;
        if handles.len() != 4 || sorted.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err("positive orbit selectors differ".into());
        }
        let mut choices = Vec::with_capacity(4);
        for (slot, x) in summand_xs.iter().enumerate() {
            let position = sorted
                .binary_search(&x.to_biguint())
                .map_err(|_| "positive representative missing")?;
            let handle = &handles[slot];
            if handle.selectors.len() != sorted.len() || handle.phase.len() != 7 {
                return Err("positive selector wires differ".into());
            }
            choices.push((handle.selectors[position], handle.phase.clone()));
        }
        for (selector, phase) in choices {
            if !enc.solver.add_clause(vec![selector as i32]) {
                return Err("positive selector pin failed".into());
            }
            for wire in phase {
                if !enc.solver.add_clause(vec![-(wire as i32)]) {
                    return Err("positive phase-zero pin failed".into());
                }
            }
        }
        phases.mark(
            "positive_choices_pinned",
            json!({"selectors":4,"phases":28}),
        )?;
    }
    let positive_pin_ms = if mode == "positive" {
        pin_start.elapsed().as_secs_f64() * 1000.0
    } else {
        0.0
    };
    enc.solver.conflict_budget = 100_000;
    enc.solver.deadline = Some(started + Duration::from_secs(1790));
    phases.mark(
        "root_propagation_entered",
        json!({"stats":root_phase_stats(&enc.solver.stats)}),
    )?;
    let root_start = Instant::now();
    let root_result = enc.solver.propagate_root();
    let root_ms = root_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "root_propagation_returned",
        json!({
            "result":format!("{root_result:?}"), "root_ms":root_ms,
            "stats":root_phase_stats(&enc.solver.stats),
        }),
    )?;
    let search_start = Instant::now();
    let answer = if root_result == RootPropagationResult::Unsat {
        SolveResult::Unsat
    } else {
        phases.mark(
            "sat_entered",
            json!({"conflict_budget":100_000,"max_models":1}),
        )?;
        let progress = Arc::new(SolverProgress::default());
        enc.solver.set_progress_observer(Arc::clone(&progress));
        let sampler = ProgressSampler::start(
            output,
            started,
            "n83.paired-orbit-progress/v1",
            source_commit.clone(),
            object_blake3,
            point_set_blake3,
            progress,
        )?;
        let result = enc.solver.solve();
        sampler.finish()?;
        result
    };
    let search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "sat_returned",
        json!({
            "answer":format!("{answer:?}"), "search_ms":search_ms,
            "conflicts":enc.solver.conflicts(),
        }),
    )?;
    let mut relation = Value::Null;
    let mut relation_check_ms = 0.0;
    let mut rank_ms = 0.0;
    let status = match answer {
        SolveResult::Unknown => "UNKNOWN_search_cap",
        SolveResult::Unsat if mode == "positive" => "INVALID_POSITIVE_CONTROL",
        SolveResult::Unsat => "UNVERIFIED_MODEL_UNSAT",
        SolveResult::Sat => {
            let checked = Instant::now();
            let xs = enc.decode_x_blocks();
            let result = if !enc.verify_model() || xs.len() != 7 || xs[6] != *target_x {
                "INVALID_MODEL"
            } else if let Some((indices, row)) =
                checked_four_x_lift(&base.curve, fb, &xs[..4], &target)?
            {
                let rank_start = Instant::now();
                let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
                let rank_after_insert = rank.insert(row.clone());
                rank_ms = rank_start.elapsed().as_secs_f64() * 1000.0;
                let changed_target = base.curve.add(&target, base.curve.generator());
                let tamper_rejected =
                    checked_primary_relation_row(&base.curve, fb, &indices, &changed_target)
                        .is_err();
                let dense: Vec<_> = row.iter().map(ToString::to_string).collect();
                relation = json!({
                    "point_indices":indices,
                    "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense)?).to_hex().to_string(),
                    "rank_after_insert":rank_after_insert,
                    "group_verified":true, "tamper_rejected":tamper_rejected,
                    "matches_constructed_row":positive_row.as_ref().map(|prior| prior == &row),
                    "matches_constructed_indices":positive_indices.map(|prior| prior == indices),
                });
                phases.mark("relation_checked", relation.clone())?;
                if rank_after_insert != 1 || !tamper_rejected {
                    "INVALID_RELATION"
                } else if mode == "positive" && positive_row.as_ref() != Some(&row) {
                    "INVALID_POSITIVE_CONTROL"
                } else if mode == "positive" {
                    "PASS_CONSTRUCTED_CONTROL"
                } else {
                    "PASS_NATURAL_RELATION_PENDING_REPLAY"
                }
            } else {
                "INVALID_GROUP_LIFT"
            };
            relation_check_ms = checked.elapsed().as_secs_f64() * 1000.0 - rank_ms;
            result
        }
    };
    let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()?;
    let receipt = json!({
        "schema":"n83.paired-orbit-worker/v1", "status":status,
        "arm":arm, "mode":mode, "source_commit":source_commit,
        "source_attestation_mode":source_attestation_mode,
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_example_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "binary_sha256":binary_sha256, "object_blake3":object_blake3,
        "point_set_blake3":point_set_blake3, "manifest_blake3":manifest_hash,
        "policy":row["policy"], "seed":row["seed"],
        "public_corpus_blake3":corpus_hash, "target_blake3":target_hash,
        "target":planted_point_json(&target),
        "columns":fb.signed_orbits.len(), "points":fb.points.len(),
        "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
        "domain_preparation_ms":preparation_ms, "model_domain_ms":model_ms,
        "positive_pin_ms":positive_pin_ms, "root_propagation_ms":root_ms,
        "search_ms":search_ms, "relation_check_ms":relation_check_ms,
        "rank_ms":rank_ms, "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(), "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(), "relation":relation,
        "candidate_id":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

/// New, opt-in five-summand gate. The m=4 worker and its v1 receipt stay intact.
pub(super) fn paired_orbit_m5_cli(
    mode: &str,
    arm: &str,
    fixture_index: u64,
    root: &Path,
    evidence_root: &Path,
    fixture_root: &Path,
    output: &Path,
) -> Result<Value> {
    if !["preflight", "positive", "natural"].contains(&mode)
        || !["U", "D"].contains(&arm)
        || fixture_index > 1
    {
        return Err("paired mode or arm outside frozen protocol".into());
    }
    if output.exists() {
        return Err("paired output already exists".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(4096)?;
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let binary_sha256 = file_sha256(&std::env::current_exe()?)?;
    let started = Instant::now();
    let cap_path = PathBuf::from(format!("{}.cap.json", output.display()));
    let cap_arm = arm.to_owned();
    let cap_mode = mode.to_owned();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(600));
        let memory_cgroup_peak_bytes = match cgroup_v2_file("memory.peak")
            .ok()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|value| value.trim().parse::<u64>().ok())
        {
            Some(value) => value,
            None => std::process::exit(125),
        };
        let cap = json!({
            "schema":"n83.paired-orbit-m5-cap/v1", "status":"UNKNOWN_budget",
            "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
            "process_usage":process_usage(),
            "arm":cap_arm, "mode":cap_mode,
            "summands":5, "fixture_index":fixture_index,
            "budget_seconds":600,
            "elapsed_process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "candidate_id":Value::Null,
            "selected_best_total_runtime":Value::Null
        });
        if write_new_json(&cap_path, &cap).is_err() {
            std::process::exit(125);
        }
        std::process::exit(124);
    });
    let import_start = Instant::now();
    let (base, row, manifest_hash, object_blake3, point_set_blake3) =
        load_paired_base(arm, root, evidence_root)?;
    let import_ms = import_start.elapsed().as_secs_f64() * 1000.0;
    if file_sha256(&fixture_root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256 {
        return Err("paired public corpus differs".into());
    }
    let target_start = Instant::now();
    let (public, corpus_hash) = public_target_fixture(fixture_root, &base.curve, fixture_index)?;
    let target_validation_ms = target_start.elapsed().as_secs_f64() * 1000.0;
    let expected_public = match fixture_index {
        0 => json!({
            "x_decimal":"1834913121178197242856238",
            "y_decimal":"8797964190732846213981377"
        }),
        1 => json!({
            "x_decimal":"5809312440517013698248638",
            "y_decimal":"3568442427236746854022671"
        }),
        _ => return Err("m5 public fixture outside frozen target pair".into()),
    };
    if planted_point_json(&public) != expected_public {
        return Err("paired public fixture point differs".into());
    }
    let fb = &base.factor_base;
    if fb.points.len() != 2_760_082 || fb.signed_orbits.len() != 16_627 {
        return Err("paired factor-base dimensions differ".into());
    }
    let prepare_start = Instant::now();
    let representatives: Vec<BigUint> = fb
        .signed_orbits
        .iter()
        .map(|orbit| match &fb.points[orbit[0]] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("paired orbit representative is infinity"),
        })
        .collect::<std::result::Result<_, _>>()?;
    let mut positive_indices = None;
    let mut positive_xs = None;
    let mut positive_row = None;
    let target = if mode == "positive" {
        let indices: [usize; 5] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
        let mut sum = BinaryPoint::Infinity;
        let mut partial = Vec::with_capacity(5);
        let mut summand_xs = Vec::with_capacity(5);
        for &index in &indices {
            let BinaryPoint::Affine { x, .. } = &fb.points[index] else {
                return Err("positive representative is infinity".into());
            };
            summand_xs.push(x.clone());
            sum = base.curve.add(&sum, &fb.points[index]);
            partial.push(sum.clone());
        }
        if partial
            .iter()
            .any(|point| !matches!(point, BinaryPoint::Affine { .. }))
            || summand_xs
                .iter()
                .map(F2mElement::to_biguint)
                .collect::<HashSet<_>>()
                .len()
                != 5
        {
            return Err("paired positive control is not affine and distinct".into());
        }
        let row = checked_primary_relation_row(&base.curve, fb, &indices, &sum)?;
        let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
        let tamper = base.curve.add(&sum, base.curve.generator());
        if rank.insert(row.clone()) != 1
            || checked_primary_relation_row(&base.curve, fb, &indices, &tamper).is_ok()
        {
            return Err("paired positive group/rank/tamper preflight failed".into());
        }
        positive_indices = Some(indices);
        positive_xs = Some((summand_xs, partial));
        positive_row = Some(row);
        sum
    } else {
        public.clone()
    };
    let BinaryPoint::Affine { x: target_x, .. } = &target else {
        return Err("paired target is infinity".into());
    };
    let target_hash = blake3::hash(&serde_json::to_vec(&planted_point_json(&target))?)
        .to_hex()
        .to_string();
    let preparation_ms = prepare_start.elapsed().as_secs_f64() * 1000.0;
    let mut phases = WitnessPhases::create(
        output,
        started,
        source_commit.clone(),
        "n83.paired-orbit-m5-phase/v1",
        object_blake3,
        point_set_blake3,
    )?;
    phases.mark(
        "inputs_checked",
        json!({
            "arm":arm, "mode":mode, "summands":5,
            "fixture_index":fixture_index, "binary_sha256":binary_sha256,
            "target_blake3":target_hash, "public_corpus_blake3":corpus_hash,
            "manifest_blake3":manifest_hash, "base_import_ms":import_ms,
            "target_validation_ms":target_validation_ms,
            "domain_preparation_ms":preparation_ms,
        }),
    )?;
    let model_start = Instant::now();
    let mut enc = ChainedS3Encoding::build(
        5,
        &base.curve.curve.irreducible,
        &base.curve.curve.b,
        Some(target_x),
        0,
        300_000,
    )
    .map_err(|_| "paired chained-S3 model build failed")?;
    enc.constrain_frobenius_orbit_summands(&representatives, 300_000, 8_000_000)
        .map_err(|_| "paired orbit-domain install failed")?;
    let model_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "model_domain_done",
        json!({
            "model_ms":model_ms, "variables":enc.solver.n_vars(),
            "clauses":enc.solver.n_clauses(), "xor_rows":enc.solver.n_xors(),
        }),
    )?;
    if mode == "preflight" {
        let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
            .trim()
            .parse()?;
        let receipt = json!({
            "schema":"n83.paired-orbit-m5-worker/v1", "status":"PASS_PREFLIGHT",
            "arm":arm, "mode":mode, "summands":5, "fixture_index":fixture_index,
            "source_commit":source_commit,
            "source_attestation_mode":source_attestation_mode,
            "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
            "source_example_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
            "binary_sha256":binary_sha256, "object_blake3":object_blake3,
            "point_set_blake3":point_set_blake3, "manifest_blake3":manifest_hash,
            "policy":row["policy"], "seed":row["seed"],
            "public_corpus_blake3":corpus_hash, "public_point":planted_point_json(&public),
            "target_blake3":target_hash,
            "columns":fb.signed_orbits.len(), "points":fb.points.len(),
            "memory_cgroup_limit_bytes":memory_limit_bytes,
            "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
            "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
            "domain_preparation_ms":preparation_ms, "model_domain_ms":model_ms,
            "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
            "sat_xor_rows":enc.solver.n_xors(),
            "process_usage":process_usage(),
            "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "solver_stage_executed":false,
            "candidate_id":Value::Null,
            "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null
        });
        write_new_json(output, &receipt)?;
        return Ok(receipt);
    }
    let pin_start = Instant::now();
    if let Some((summand_xs, partial)) = &positive_xs {
        let mut pinned = summand_xs.clone();
        for point in &partial[1..4] {
            let BinaryPoint::Affine { x, .. } = point else {
                return Err("positive partial sum is infinity".into());
            };
            pinned.push(x.clone());
        }
        // Five summands, x(S2), x(S3), x(S4), x(target): nine named blocks.
        let pinned = [
            pinned[0].clone(),
            pinned[1].clone(),
            pinned[2].clone(),
            pinned[3].clone(),
            pinned[4].clone(),
            pinned[5].clone(),
            pinned[6].clone(),
            pinned[7].clone(),
            target_x.clone(),
        ];
        for (block, x) in pinned.iter().enumerate() {
            enc.pin_x_block(block, x)
                .map_err(|_| "positive x-block pin failed")?;
        }
        let mut sorted = representatives.clone();
        sorted.sort_unstable();
        let handles = enc
            .orbit_choice_wires()
            .ok_or("positive selector wires absent")?;
        if handles.len() != 5 || sorted.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err("positive orbit selectors differ".into());
        }
        let mut choices = Vec::with_capacity(5);
        for (slot, x) in summand_xs.iter().enumerate() {
            let position = sorted
                .binary_search(&x.to_biguint())
                .map_err(|_| "positive representative missing")?;
            let handle = &handles[slot];
            if handle.selectors.len() != sorted.len() || handle.phase.len() != 7 {
                return Err("positive selector wires differ".into());
            }
            choices.push((handle.selectors[position], handle.phase.clone()));
        }
        for (selector, phase) in choices {
            if !enc.solver.add_clause(vec![selector as i32]) {
                return Err("positive selector pin failed".into());
            }
            for wire in phase {
                if !enc.solver.add_clause(vec![-(wire as i32)]) {
                    return Err("positive phase-zero pin failed".into());
                }
            }
        }
        phases.mark(
            "positive_choices_pinned",
            json!({"selectors":5,"phases":35}),
        )?;
    }
    let positive_pin_ms = if mode == "positive" {
        pin_start.elapsed().as_secs_f64() * 1000.0
    } else {
        0.0
    };
    enc.solver.conflict_budget = 100_000;
    enc.solver.deadline = Some(started + Duration::from_secs(600));
    phases.mark(
        "root_propagation_entered",
        json!({"stats":root_phase_stats(&enc.solver.stats)}),
    )?;
    let root_start = Instant::now();
    let root_result = enc.solver.propagate_root();
    let root_ms = root_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "root_propagation_returned",
        json!({
            "result":format!("{root_result:?}"), "root_ms":root_ms,
            "stats":root_phase_stats(&enc.solver.stats),
        }),
    )?;
    let search_start = Instant::now();
    let answer = if root_result == RootPropagationResult::Unsat {
        SolveResult::Unsat
    } else {
        phases.mark(
            "sat_entered",
            json!({"conflict_budget":100_000,"max_models":1}),
        )?;
        let progress = Arc::new(SolverProgress::default());
        enc.solver.set_progress_observer(Arc::clone(&progress));
        let sampler = ProgressSampler::start(
            output,
            started,
            "n83.paired-orbit-m5-progress/v1",
            source_commit.clone(),
            object_blake3,
            point_set_blake3,
            progress,
        )?;
        let result = enc.solver.solve();
        sampler.finish()?;
        result
    };
    let search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "sat_returned",
        json!({
            "answer":format!("{answer:?}"), "search_ms":search_ms,
            "conflicts":enc.solver.conflicts(),
        }),
    )?;
    let mut relation = Value::Null;
    let mut relation_check_ms = 0.0;
    let mut rank_ms = 0.0;
    let status = match answer {
        SolveResult::Unknown => "UNKNOWN_search_cap",
        SolveResult::Unsat if mode == "positive" => "INVALID_POSITIVE_CONTROL",
        SolveResult::Unsat => "UNVERIFIED_MODEL_UNSAT",
        SolveResult::Sat => {
            let checked = Instant::now();
            let xs = enc.decode_x_blocks();
            let result = if !enc.verify_model() || xs.len() != 9 || xs[8] != *target_x {
                "INVALID_MODEL"
            } else if let Some((indices, row)) =
                checked_five_x_lift(&base.curve, fb, &xs[..5], &target)?
            {
                let rank_start = Instant::now();
                let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
                let rank_after_insert = rank.insert(row.clone());
                rank_ms = rank_start.elapsed().as_secs_f64() * 1000.0;
                let changed_target = base.curve.add(&target, base.curve.generator());
                let tamper_rejected =
                    checked_primary_relation_row(&base.curve, fb, &indices, &changed_target)
                        .is_err();
                let dense: Vec<_> = row.iter().map(ToString::to_string).collect();
                relation = json!({
                    "point_indices":indices,
                    "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense)?).to_hex().to_string(),
                    "rank_after_insert":rank_after_insert,
                    "group_verified":true, "tamper_rejected":tamper_rejected,
                    "matches_constructed_row":positive_row.as_ref().map(|prior| prior == &row),
                    "matches_constructed_indices":positive_indices.map(|prior| prior == indices),
                });
                phases.mark("relation_checked", relation.clone())?;
                if rank_after_insert != 1 || !tamper_rejected {
                    "INVALID_RELATION"
                } else if mode == "positive" && positive_row.as_ref() != Some(&row) {
                    "INVALID_POSITIVE_CONTROL"
                } else if mode == "positive" {
                    "PASS_CONSTRUCTED_CONTROL"
                } else {
                    "PASS_NATURAL_RELATION_PENDING_REPLAY"
                }
            } else {
                "INVALID_GROUP_LIFT"
            };
            relation_check_ms = checked.elapsed().as_secs_f64() * 1000.0 - rank_ms;
            result
        }
    };
    let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()?;
    let receipt = json!({
        "schema":"n83.paired-orbit-m5-worker/v1", "status":status,
        "arm":arm, "mode":mode, "summands":5, "fixture_index":fixture_index,
        "source_commit":source_commit,
        "source_attestation_mode":source_attestation_mode,
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_example_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "binary_sha256":binary_sha256, "object_blake3":object_blake3,
        "point_set_blake3":point_set_blake3, "manifest_blake3":manifest_hash,
        "policy":row["policy"], "seed":row["seed"],
        "public_corpus_blake3":corpus_hash, "target_blake3":target_hash,
        "target":planted_point_json(&target),
        "columns":fb.signed_orbits.len(), "points":fb.points.len(),
        "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
        "domain_preparation_ms":preparation_ms, "model_domain_ms":model_ms,
        "positive_pin_ms":positive_pin_ms, "root_propagation_ms":root_ms,
        "search_ms":search_ms, "relation_check_ms":relation_check_ms,
        "rank_ms":rank_ms, "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(), "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(), "relation":relation,
        "candidate_id":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

/// A separate process rereads the exact base and worker result before a
/// returned row can count as a natural relation or positive control.
pub(super) fn paired_relation_replay_cli(
    arm: &str,
    root: &Path,
    evidence_root: &Path,
    fixture_root: &Path,
    worker_path: &Path,
    output: &Path,
) -> Result<Value> {
    if output.exists() {
        return Err("paired replay output already exists".into());
    }
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let started = Instant::now();
    let (base, row, manifest_hash, object_blake3, point_set_blake3) =
        load_paired_base(arm, root, evidence_root)?;
    let worker: Value = serde_json::from_slice(&fs::read(worker_path)?)?;
    let mode = worker["mode"].as_str().ok_or("paired worker mode absent")?;
    if !["positive", "natural"].contains(&mode)
        || worker["schema"] != "n83.paired-orbit-worker/v1"
        || worker["arm"] != arm
        || worker["status"]
            != if mode == "positive" {
                "PASS_CONSTRUCTED_CONTROL"
            } else {
                "PASS_NATURAL_RELATION_PENDING_REPLAY"
            }
        || worker["object_blake3"] != object_blake3
        || worker["point_set_blake3"] != point_set_blake3
        || worker["manifest_blake3"] != manifest_hash
        || worker["policy"] != row["policy"]
        || worker["seed"] != row["seed"]
        || worker["source_commit"]
            .as_str()
            .is_none_or(|commit| commit.len() != 40)
        || worker["source_adapter_blake3"]
            != blake3::hash(include_bytes!("primary_adapter.rs"))
                .to_hex()
                .to_string()
        || worker["source_example_blake3"]
            != blake3::hash(include_bytes!(
                "../../examples/koblitz_n83_factor_base_export.rs"
            ))
            .to_hex()
            .to_string()
        || worker["selected_best_total_runtime"] != Value::Null
    {
        return Err("paired worker identity or success receipt differs".into());
    }
    let (public, corpus_hash) = public_target(fixture_root, &base.curve)?;
    if file_sha256(&fixture_root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256
        || worker["public_corpus_blake3"] != corpus_hash
    {
        return Err("paired replay public corpus differs".into());
    }
    let fb = &base.factor_base;
    let target = if mode == "positive" {
        (0..4).fold(BinaryPoint::Infinity, |sum, column| {
            base.curve
                .add(&sum, &fb.points[fb.signed_orbits[column][0]])
        })
    } else {
        public
    };
    let target_hash = blake3::hash(&serde_json::to_vec(&planted_point_json(&target))?)
        .to_hex()
        .to_string();
    if worker["target"] != planted_point_json(&target) || worker["target_blake3"] != target_hash {
        return Err("paired replay target differs".into());
    }
    let stored = worker["relation"]["point_indices"]
        .as_array()
        .ok_or("paired point indices absent")?;
    if stored.len() != 4 {
        return Err("paired point count differs".into());
    }
    let mut indices = [0usize; 4];
    for (slot, value) in stored.iter().enumerate() {
        indices[slot] = usize::try_from(value.as_u64().ok_or("paired point index absent")?)?;
        if indices[slot] >= fb.points.len() {
            return Err("paired point index outside base".into());
        }
    }
    let checked = Instant::now();
    let dense = checked_primary_relation_row(&base.curve, fb, &indices, &target)?;
    let dense_strings: Vec<_> = dense.iter().map(ToString::to_string).collect();
    let dense_hash = blake3::hash(&serde_json::to_vec(&dense_strings)?)
        .to_hex()
        .to_string();
    let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
    let rank_after_insert = rank.insert(dense.clone());
    let mut changed_indices = indices;
    changed_indices[0] ^= 1;
    let changed_point_rejected =
        checked_primary_relation_row(&base.curve, fb, &changed_indices, &target).is_err();
    let mut changed_row = dense;
    changed_row[0] = (&changed_row[0] + BigUint::one()) % &base.curve.subgroup_order;
    let changed_strings: Vec<_> = changed_row.iter().map(ToString::to_string).collect();
    let changed_row_rejected = blake3::hash(&serde_json::to_vec(&changed_strings)?)
        .to_hex()
        .as_str()
        != worker["relation"]["dense_decimal_blake3"]
            .as_str()
            .ok_or("paired dense row digest absent")?;
    if worker["relation"]["dense_decimal_blake3"] != dense_hash
        || worker["relation"]["rank_after_insert"] != 1
        || rank_after_insert != 1
        || !changed_point_rejected
        || !changed_row_rejected
    {
        return Err("paired relation, rank or tamper replay failed".into());
    }
    let receipt = json!({
        "schema":"n83.paired-orbit-replay/v1", "status":"PASS",
        "arm":arm, "mode":mode, "replay_source_commit":source_commit,
        "worker_source_commit":worker["source_commit"],
        "source_attestation_mode":source_attestation_mode,
        "worker_sha256":file_sha256(worker_path)?,
        "object_blake3":object_blake3, "point_set_blake3":point_set_blake3,
        "target_blake3":target_hash, "point_indices":indices,
        "dense_decimal_blake3":dense_hash, "rank_after_insert":rank_after_insert,
        "changed_point_rejected":changed_point_rejected,
        "changed_row_rejected":changed_row_rejected,
        "relation_replay_ms":checked.elapsed().as_secs_f64()*1000.0,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "candidate_id":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

/// Separate-process replay of an m=5 paired worker relation and rank row.
pub(super) fn paired_relation_m5_replay_cli(
    arm: &str,
    root: &Path,
    evidence_root: &Path,
    fixture_root: &Path,
    worker_path: &Path,
    output: &Path,
) -> Result<Value> {
    if output.exists() {
        return Err("paired replay output already exists".into());
    }
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let started = Instant::now();
    let (base, row, manifest_hash, object_blake3, point_set_blake3) =
        load_paired_base(arm, root, evidence_root)?;
    let worker: Value = serde_json::from_slice(&fs::read(worker_path)?)?;
    let mode = worker["mode"].as_str().ok_or("paired worker mode absent")?;
    let fixture_index = worker["fixture_index"]
        .as_u64()
        .ok_or("m5 worker fixture index absent")?;
    if !["positive", "natural"].contains(&mode)
        || worker["schema"] != "n83.paired-orbit-m5-worker/v1"
        || worker["summands"] != 5
        || fixture_index > 1
        || worker["arm"] != arm
        || worker["status"]
            != if mode == "positive" {
                "PASS_CONSTRUCTED_CONTROL"
            } else {
                "PASS_NATURAL_RELATION_PENDING_REPLAY"
            }
        || worker["object_blake3"] != object_blake3
        || worker["point_set_blake3"] != point_set_blake3
        || worker["manifest_blake3"] != manifest_hash
        || worker["policy"] != row["policy"]
        || worker["seed"] != row["seed"]
        || worker["source_commit"]
            .as_str()
            .is_none_or(|commit| commit.len() != 40)
        || worker["source_adapter_blake3"]
            != blake3::hash(include_bytes!("primary_adapter.rs"))
                .to_hex()
                .to_string()
        || worker["source_example_blake3"]
            != blake3::hash(include_bytes!(
                "../../examples/koblitz_n83_factor_base_export.rs"
            ))
            .to_hex()
            .to_string()
        || worker["selected_best_total_runtime"] != Value::Null
    {
        return Err("paired worker identity or success receipt differs".into());
    }
    let (public, corpus_hash) = public_target_fixture(fixture_root, &base.curve, fixture_index)?;
    if file_sha256(&fixture_root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256
        || worker["public_corpus_blake3"] != corpus_hash
    {
        return Err("paired replay public corpus differs".into());
    }
    let fb = &base.factor_base;
    let target = if mode == "positive" {
        (0..5).fold(BinaryPoint::Infinity, |sum, column| {
            base.curve
                .add(&sum, &fb.points[fb.signed_orbits[column][0]])
        })
    } else {
        public
    };
    let target_hash = blake3::hash(&serde_json::to_vec(&planted_point_json(&target))?)
        .to_hex()
        .to_string();
    if worker["target"] != planted_point_json(&target) || worker["target_blake3"] != target_hash {
        return Err("paired replay target differs".into());
    }
    let stored = worker["relation"]["point_indices"]
        .as_array()
        .ok_or("paired point indices absent")?;
    if stored.len() != 5 {
        return Err("paired point count differs".into());
    }
    let mut indices = [0usize; 5];
    for (slot, value) in stored.iter().enumerate() {
        indices[slot] = usize::try_from(value.as_u64().ok_or("paired point index absent")?)?;
        if indices[slot] >= fb.points.len() {
            return Err("paired point index outside base".into());
        }
    }
    let checked = Instant::now();
    let dense = checked_primary_relation_row(&base.curve, fb, &indices, &target)?;
    let dense_strings: Vec<_> = dense.iter().map(ToString::to_string).collect();
    let dense_hash = blake3::hash(&serde_json::to_vec(&dense_strings)?)
        .to_hex()
        .to_string();
    let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
    let rank_after_insert = rank.insert(dense.clone());
    let mut changed_indices = indices;
    changed_indices[0] ^= 1;
    let changed_point_rejected =
        checked_primary_relation_row(&base.curve, fb, &changed_indices, &target).is_err();
    let mut changed_row = dense;
    changed_row[0] = (&changed_row[0] + BigUint::one()) % &base.curve.subgroup_order;
    let changed_strings: Vec<_> = changed_row.iter().map(ToString::to_string).collect();
    let changed_row_rejected = blake3::hash(&serde_json::to_vec(&changed_strings)?)
        .to_hex()
        .as_str()
        != worker["relation"]["dense_decimal_blake3"]
            .as_str()
            .ok_or("paired dense row digest absent")?;
    if worker["relation"]["dense_decimal_blake3"] != dense_hash
        || worker["relation"]["rank_after_insert"] != 1
        || rank_after_insert != 1
        || !changed_point_rejected
        || !changed_row_rejected
    {
        return Err("paired relation, rank or tamper replay failed".into());
    }
    let receipt = json!({
        "schema":"n83.paired-orbit-m5-replay/v1", "status":"PASS",
        "arm":arm, "mode":mode, "summands":5, "fixture_index":fixture_index,
        "replay_source_commit":source_commit,
        "worker_source_commit":worker["source_commit"],
        "source_attestation_mode":source_attestation_mode,
        "worker_sha256":file_sha256(worker_path)?,
        "object_blake3":object_blake3, "point_set_blake3":point_set_blake3,
        "target_blake3":target_hash, "point_indices":indices,
        "dense_decimal_blake3":dense_hash, "rank_after_insert":rank_after_insert,
        "changed_point_rejected":changed_point_rejected,
        "changed_row_rejected":changed_row_rejected,
        "relation_replay_ms":checked.elapsed().as_secs_f64()*1000.0,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "candidate_id":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

#[allow(clippy::too_many_arguments)]
fn run_with_config(
    root: &Path,
    columns: usize,
    m: usize,
    strategy_name: &str,
    max_trials: usize,
    budget_seconds: u64,
    run_dir: &Path,
    chain: Option<&ChainRunConfig<'_>>,
) -> Result<Value> {
    if ![64, 256, 600].contains(&columns) && !(chain.is_some() && V2_SIZES.contains(&columns))
        || !(2..=6).contains(&m)
    {
        return Err("unsupported primary K or summand count".into());
    }
    let strategy = match strategy_name {
        "enumerate" => DecompositionStrategy::Enumerate,
        "sat-m3" => {
            return Err(
                "primary SAT S4 model replay and resource-capacity gate is incomplete for F2^83"
                    .into(),
            )
        }
        "chain-s3" if chain.is_some() && (4..=6).contains(&m) => DecompositionStrategy::ChainedS3,
        "orbit-chain-s3" if chain.is_some() && (4..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3FrobeniusOrbit
        }
        "ordered-orbit-chain-s3" if chain.is_some() && (4..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3FrobeniusOrbitOrdered
        }
        _ => return Err("unsupported or unguarded primary strategy".into()),
    };
    if budget_seconds == 0 || budget_seconds > 86_400 {
        return Err("primary wall cap must be 1..86400 seconds".into());
    }
    let start = Instant::now();
    fs::create_dir(run_dir)?;
    write_new_json(
        &run_dir.join("config.json"),
        &json!({
            "schema":"n83.primary-cold-config/v1", "study":STUDY,
            "panel_dir":root.to_string_lossy(),
            "fixture_dir":chain.map(|config| config.fixture_root.to_string_lossy().to_string()),
            "curve_a":0, "fixture":0, "orbit_columns":columns,
            "summands":m, "strategy":strategy_name, "max_trials":max_trials,
            "budget_seconds":budget_seconds, "allow_direct_relation":false,
            "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
            "source_commit":chain.map(|config| config.source_commit.as_str()),
            "source_attestation_mode":chain.map(|config| config.source_attestation_mode),
            "policy":chain.map(|config| config.policy),
            "seed":chain.map(|config| config.seed),
            "chain_s3_limits":chain.map(|config| json!({
                "max_variables":config.limits.max_variables,
                "max_domain_clauses":config.limits.max_domain_clauses,
                "max_models":config.limits.max_models,
                "conflict_budget":config.limits.conflict_budget,
            })),
            "memory_cgroup_limit_bytes":chain.map(|config| config.memory_limit_bytes),
            "memory_cgroup_swap_limit_bytes":chain.map(|_| 0),
        }),
    )?;
    let cap_path = run_dir.join("cap.json");
    let cap_strategy = strategy_name.to_owned();
    let deadline = start + Duration::from_secs(budget_seconds);
    std::thread::spawn(move || {
        std::thread::sleep(deadline.saturating_duration_since(Instant::now()));
        let cap = json!({
            "schema":"n83.primary-cold-cap/v1", "status":"UNKNOWN_budget",
            "orbit_columns":columns, "summands":m, "strategy":cap_strategy,
            "max_trials":max_trials, "budget_seconds":budget_seconds,
            "elapsed_process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
            "selected_best_total_runtime":Value::Null
        });
        if write_new_json(&cap_path, &cap).is_err() {
            std::process::exit(125);
        }
        std::process::exit(124);
    });

    let import_start = Instant::now();
    let (base, row, manifest_hash) = if let Some(config) = chain {
        load_panel_selected(root, columns, config.policy, config.seed)?
    } else {
        load_panel(root, columns)?
    };
    let base_import_ms = import_start.elapsed().as_secs_f64() * 1000.0;
    let target_start = Instant::now();
    let fixture_root = chain.map_or(root, |config| config.fixture_root);
    let (target, corpus_hash) = public_target(fixture_root, &base.curve)?;
    let target_validation_ms = target_start.elapsed().as_secs_f64() * 1000.0;

    let mut options = KoblitzIcOptions::default();
    options.m = m;
    options.strategy = strategy;
    options.max_trials = max_trials;
    options.seed = 2026100901;
    options.allow_direct_relation = false;
    if let Some(config) = chain {
        options.chain_s3_limits = config.limits;
    }
    let mut events = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(run_dir.join("events.jsonl"))?;
    let solver_start = Instant::now();
    let mut event_write_failed = false;
    let report = koblitz_index_calculus_dlp_with_factor_base_and_progress(
        &base.curve,
        &target,
        &base.factor_base,
        &options,
        &mut |event| {
            let line = json!({
                "elapsed_process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
                "event":format!("{event:?}")
            });
            if writeln!(events, "{line}")
                .and_then(|_| events.flush())
                .is_err()
            {
                event_write_failed = true;
            }
        },
    );
    let solver_ms = solver_start.elapsed().as_secs_f64() * 1000.0;
    if event_write_failed {
        return Err("primary progress-event write failed".into());
    }
    let Some(report) = report else {
        let summary = json!({
            "schema":"n83.primary-cold-result/v1", "status":"PRODUCER_FAILURE",
            "reason":"generic solver rejected the primary base or strategy",
            "solver_stage_executed":false, "selected_best_total_runtime":Value::Null
        });
        write_new_json(&run_dir.join("summary.json"), &summary)?;
        return Err("generic primary solver rejected the imported base or strategy".into());
    };
    let verification_start = Instant::now();
    let candidate_valid = match &report.log {
        Some(candidate) => {
            base.curve.mul(base.curve.generator(), candidate) == target
                && candidate_matches_validation(fixture_root, &corpus_hash, candidate)
                    .unwrap_or(false)
        }
        None => false,
    };
    let post_solver_validation_ms = verification_start.elapsed().as_secs_f64() * 1000.0;
    let memory_cgroup_peak_bytes = if chain.is_some() {
        Some(
            fs::read_to_string(cgroup_v2_file("memory.peak")?)?
                .trim()
                .parse::<u64>()?,
        )
    } else {
        None
    };
    let status = if report.inconsistent_relations != 0
        || report.verification_failures != 0
        || report.sat_invalid_models != 0
        || report.direct_relation
        || (report.log.is_some() && !candidate_valid)
    {
        "PRODUCER_FAILURE"
    } else if candidate_valid {
        "PASS_verified_target_only"
    } else if !report.m_cofactor_admissible {
        "INADMISSIBLE_cofactor_class"
    } else if max_trials == 0 {
        "PREFLIGHT_ONLY"
    } else if chain.is_some() && report.sat_unknowns != 0 {
        "UNKNOWN_solver_cap"
    } else {
        "UNKNOWN_trial_cap"
    };
    let report_details = json!({
        "factor_base_size":report.factor_base_size,
        "orbit_count":report.orbit_count,
        "trials":report.trials, "relations":report.relations,
        "independent_relations":report.independent_relations,
        "dependent_relations":report.dependent_relations,
        "inconsistent_relations":report.inconsistent_relations,
        "verification_failures":report.verification_failures,
        "sat_unknowns":report.sat_unknowns,
        "sat_invalid_models":report.sat_invalid_models,
        "sat_group_rejected_models":report.sat_group_rejected_models,
        "sat_calls":report.sat_calls,
        "sat_refutations":report.sat_refutations,
        "sat_models":report.sat_models,
        "sat_conflicts":report.sat_conflicts,
        "linear_solve_attempts":report.linear_solve_attempts,
        "matrix_rows":report.matrix_rows,
        "matrix_columns":report.matrix_columns,
        "terminal_matrix_rank":report.terminal_matrix_rank,
        "rank_checks":report.rank_checks,
        "relation_collection_ns":report.relation_collection_ns.to_string(),
        "linear_algebra_ns":report.linear_algebra_ns.to_string(),
        "m_cofactor_admissible":report.m_cofactor_admissible,
        "direct_relations_skipped":report.direct_relations_skipped
    });
    let attempt_records: Vec<_> = report
        .attempt_records
        .iter()
        .map(|attempt| {
            json!({
                "trial":attempt.trial,
                "a":attempt.coefficient_a.to_string(),
                "b":attempt.coefficient_b.to_string(),
                "target":point_json(words(&attempt.target)),
                "disposition":format!("{:?}",attempt.disposition),
                "decomposition_indices":attempt.decomposition_indices,
                "solver_calls":attempt.solver_calls,
                "models":attempt.models,
                "conflicts":attempt.conflicts,
                "implied_rows":attempt.implied_rows
            })
        })
        .collect();
    let relation_matrix: Vec<_> = report
        .relation_matrix
        .iter()
        .map(|relation| {
            json!({
                "a":relation.coef_a.to_string(),
                "b":relation.coef_b.to_string(),
                "summands":relation.summands,
                "summand_negated":relation.summand_negated,
                "row":relation.row.iter().map(ToString::to_string).collect::<Vec<_>>()
            })
        })
        .collect();
    let rank_history: Vec<_> = report
        .rank_history
        .iter()
        .map(|rank| {
            json!({
                "rows":rank.rows,
                "columns":rank.columns,
                "rank":rank.rank,
                "candidate_produced":rank.candidate_produced,
                "candidate_verified":rank.candidate_verified
            })
        })
        .collect();
    let summary = json!({
        "schema":"n83.primary-cold-result/v1", "study":STUDY,
        "status":status, "curve_a":0, "fixture":0,
        "object":row["object"], "point_set_blake3":row["point_set_blake3"],
        "panel_manifest_blake3":manifest_hash,
        "public_corpus_canonical_json_blake3":corpus_hash,
        "orbit_columns":columns, "summands":m, "strategy":strategy_name,
        "policy":row["policy"], "seed":row["seed"],
        "max_trials":max_trials, "sampler_seed":options.seed,
        "budget_seconds":budget_seconds,
        "fixture_dir":chain.map(|config| config.fixture_root.to_string_lossy().to_string()),
        "source_commit":chain.map(|config| config.source_commit.as_str()),
        "source_attestation_mode":chain.map(|config| config.source_attestation_mode),
        "source_chain_blake3":chain.map(|_| blake3::hash(include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs")).to_hex().to_string()),
        "source_index_calculus_blake3":chain.map(|_| blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string()),
        "chain_s3_limits":chain.map(|config| json!({
            "max_variables":config.limits.max_variables,
            "max_domain_clauses":config.limits.max_domain_clauses,
            "max_models":config.limits.max_models,
            "conflict_budget":config.limits.conflict_budget,
        })),
        "memory_cgroup_limit_bytes":chain.map(|config| config.memory_limit_bytes),
        "memory_cgroup_swap_limit_bytes":chain.map(|_| 0),
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        "process_usage":if chain.is_some() { process_usage() } else { Value::Null },
        "base_import_ms":base_import_ms, "target_validation_ms":target_validation_ms,
        "solver_ms":solver_ms, "post_solver_validation_ms":post_solver_validation_ms,
        "pre_summary_process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "solver_stage_executed":if chain.is_some() {report.sat_calls>0} else {report.trials>0},
        "report":report_details,
        "attempt_records":attempt_records,
        "relation_matrix":relation_matrix,
        "rank_history":rank_history,
        "verified_log":if candidate_valid { report.log.as_ref().map(ToString::to_string) } else { None },
        "column_log_verification":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(&run_dir.join("summary.json"), &summary)?;
    if status == "PRODUCER_FAILURE" {
        Err("primary solver result failed verification; inspect summary.json".into())
    } else {
        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use super::super::construct;
    use super::*;
    use crypto_lib::cryptanalysis::koblitz_index_calculus::{
        enumerate_decompose, koblitz_index_calculus_dlp_with_factor_base, KoblitzIcOptions,
    };
    use crypto_lib::cryptanalysis::koblitz_relation_solver::WideRankTracker;
    use std::collections::HashMap;
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    #[test]
    fn cgroup_v2_membership_resolves_container_and_native_scope() {
        assert_eq!(
            cgroup_v2_root_from_membership("0::/\n").unwrap(),
            Path::new("/sys/fs/cgroup")
        );
        assert_eq!(
            cgroup_v2_root_from_membership(
                "8:cpu:/legacy\n0::/user.slice/user-1000.slice/app.slice/run.scope\n"
            )
            .unwrap(),
            Path::new("/sys/fs/cgroup/user.slice/user-1000.slice/app.slice/run.scope")
        );
        assert!(cgroup_v2_root_from_membership("8:cpu:/legacy\n").is_err());
        assert!(cgroup_v2_root_from_membership("0::/../escape\n").is_err());
    }

    #[test]
    fn selector_and_phase_modes_preserve_small_chain_model_and_group_lift() {
        let kc = KoblitzCurve::new(0, 7).unwrap();
        let p = kc.generator().clone();
        let q = kc.mul(&p, &BigUint::from(2u32));
        let three = kc.mul(&p, &BigUint::from(3u32));
        let target = kc.mul(&p, &BigUint::from(5u32));
        let x = |point: &BinaryPoint| match point {
            BinaryPoint::Affine { x, .. } => x.clone(),
            BinaryPoint::Infinity => panic!("small control point is infinity"),
        };
        let (xp, xq) = (x(&p), x(&q));
        assert_ne!(xp, xq);
        let expected = [
            xp.clone(),
            xp.clone(),
            xp.clone(),
            xq.clone(),
            xq.clone(),
            x(&three),
            x(&target),
        ];
        // P+P=2P, then +P=3P; +2P=5P.
        let mut representatives = vec![xp.to_biguint(), xq.to_biguint()];
        representatives.sort_unstable();

        let run = |mode: OrbitChoicePins, observed: bool, contradict: bool| {
            let mut enc = ChainedS3Encoding::build(
                4,
                &kc.curve.irreducible,
                &kc.curve.b,
                Some(&expected[6]),
                0,
                2_000,
            )
            .unwrap();
            enc.constrain_frobenius_orbit_summands(&representatives, 2_000, 10_000)
                .unwrap();
            for (block, value) in expected.iter().enumerate() {
                enc.pin_x_block(block, value).unwrap();
            }
            let handles = enc.orbit_choice_wires().unwrap().to_vec();
            for (slot, value) in expected[..4].iter().enumerate() {
                let index = representatives.binary_search(&value.to_biguint()).unwrap();
                if mode.selector() {
                    assert!(enc
                        .solver
                        .add_clause(vec![handles[slot].selectors[index] as i32]));
                }
                if mode.phase() {
                    for &wire in &handles[slot].phase {
                        assert!(enc.solver.add_clause(vec![-(wire as i32)]));
                    }
                }
            }
            if contradict {
                let first_index = representatives.binary_search(&xp.to_biguint()).unwrap();
                let opposite = if mode.selector() {
                    -(handles[0].selectors[first_index] as i32)
                } else {
                    handles[0].phase[0] as i32
                };
                enc.solver.add_clause(vec![opposite]);
            }
            let progress = observed.then(|| Arc::new(SolverProgress::default()));
            if let Some(progress) = &progress {
                enc.solver.set_progress_observer(Arc::clone(progress));
            }
            let answer = enc.solver.solve();
            if contradict {
                assert_eq!(answer, SolveResult::Unsat);
                return (answer, None);
            }
            assert_eq!(answer, SolveResult::Sat);
            assert!(enc.verify_model());
            assert_eq!(enc.decode_x_blocks(), expected);
            let mut sum = BinaryPoint::Infinity;
            for value in enc.decode_summands() {
                let point = if value == xp {
                    &p
                } else if value == xq {
                    &q
                } else {
                    panic!("unknown small summand x")
                };
                sum = kc.add(&sum, point);
            }
            assert_eq!(sum, target);
            if let Some(progress) = progress {
                assert_eq!(progress.snapshot().xor_passes, enc.solver.stats.xor_passes);
            }
            (answer, Some(enc.solver.model()))
        };
        for mode in [
            OrbitChoicePins::SelectorAndPhase,
            OrbitChoicePins::SelectorOnly,
            OrbitChoicePins::PhaseOnly,
        ] {
            for contradict in [false, true] {
                assert_eq!(run(mode, false, contradict), run(mode, true, contradict));
            }
        }
    }

    #[test]
    fn five_summand_pinned_chain_model_checks_the_group_sum() {
        let kc = KoblitzCurve::new(0, 7).unwrap();
        let p = kc.generator().clone();
        let q = kc.mul(&p, &BigUint::from(2u8));
        let points = [p.clone(), p.clone(), p.clone(), p.clone(), q.clone()];
        let mut partial = Vec::new();
        let mut sum = BinaryPoint::Infinity;
        for point in &points {
            sum = kc.add(&sum, point);
            partial.push(sum.clone());
        }
        let x = |point: &BinaryPoint| match point {
            BinaryPoint::Affine { x, .. } => x.clone(),
            BinaryPoint::Infinity => panic!("five-point control partial sum is infinity"),
        };
        let mut representatives = vec![x(&p).to_biguint(), x(&q).to_biguint()];
        representatives.sort_unstable();
        assert_ne!(representatives[0], representatives[1]);
        let target_x = x(&sum);
        let mut enc = ChainedS3Encoding::build(
            5,
            &kc.curve.irreducible,
            &kc.curve.b,
            Some(&target_x),
            0,
            2_000,
        )
        .unwrap();
        enc.constrain_frobenius_orbit_summands(&representatives, 2_000, 10_000)
            .unwrap();
        let mut blocks: Vec<_> = points.iter().map(&x).collect();
        blocks.extend(partial[1..4].iter().map(&x));
        blocks.push(target_x);
        assert_eq!(blocks.len(), 9);
        for (index, value) in blocks.iter().enumerate() {
            enc.pin_x_block(index, value).unwrap();
        }
        let handles = enc.orbit_choice_wires().unwrap().to_vec();
        assert_eq!(handles.len(), 5);
        for (slot, point) in points.iter().enumerate() {
            let position = representatives
                .binary_search(&x(point).to_biguint())
                .unwrap();
            assert!(enc
                .solver
                .add_clause(vec![handles[slot].selectors[position] as i32]));
            for &wire in &handles[slot].phase {
                assert!(enc.solver.add_clause(vec![-(wire as i32)]));
            }
        }
        assert_eq!(enc.solver.solve(), SolveResult::Sat);
        assert!(enc.verify_model());
        assert_eq!(enc.decode_x_blocks(), blocks);
        assert_eq!(partial[4], sum);
    }

    #[test]
    fn supervised_source_snapshot_rejects_a_changed_compiled_file() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let snapshot = std::env::temp_dir().join(format!("n83-probe-source-{nonce}"));
        let checkout = Path::new(env!("CARGO_MANIFEST_DIR"));
        for relative in [
            "examples/koblitz_n83_factor_base_export.rs",
            "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
            "research/koblitz_n83_factor_base_sweep_20261008/compact_cold.rs",
        ] {
            let destination = snapshot.join(relative);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::copy(checkout.join(relative), destination).unwrap();
        }
        assert!(checked_primary_probe_source(&snapshot).is_ok());
        fs::write(
            snapshot.join("research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs"),
            b"changed source",
        )
        .unwrap();
        assert!(checked_primary_probe_source(&snapshot).is_err());
        fs::remove_dir_all(snapshot).unwrap();
    }

    #[test]
    fn small_primary_export_maps_to_generic_orbits_and_zero_trial_preflight() {
        let (bytes, row) = construct(
            0,
            "public_x_hash",
            2,
            17,
            Instant::now() + Duration::from_secs(60),
        )
        .unwrap();
        let base = from_record(&row, &bytes).unwrap();
        assert_eq!(base.factor_base.points.len(), 332);
        assert_eq!(base.factor_base.orbits.len(), 4);
        assert_eq!(base.factor_base.signed_orbits.len(), 2);
        assert_eq!(base.factor_base.subspace.len(), 166);
        for (index, point) in base.factor_base.points.iter().enumerate() {
            let (column, phase, negative) = base.factor_base.signed_orbit_of[index];
            assert_eq!(column, index / 166);
            assert_eq!(phase as usize, (index % 166) / 2);
            assert_eq!(negative, index % 2 == 1);
            assert_eq!(
                base.factor_base.orbit_of[index].0,
                2 * column + usize::from(negative)
            );
            assert_eq!(
                point,
                &base.factor_base.points[base.factor_base.signed_orbits[column][index % 166]]
            );
        }
        let kc = &base.curve;
        let fb = &base.factor_base;
        let indices = [0, 2, 165, 168];
        let target = indices.iter().fold(BinaryPoint::Infinity, |sum, &index| {
            kc.add(&sum, &fb.points[index])
        });
        let relation = checked_primary_relation_row(kc, fb, &indices, &target).unwrap();
        let xs: Vec<_> = indices
            .iter()
            .map(|&index| match &fb.points[index] {
                BinaryPoint::Affine { x, .. } => x.clone(),
                BinaryPoint::Infinity => panic!("primary point is infinity"),
            })
            .collect();
        let (lifted_indices, lifted_row) = checked_four_x_lift(kc, fb, &xs, &target)
            .unwrap()
            .expect("constructed four-point target has a stored group lift");
        assert_eq!(lifted_row, relation);
        assert_eq!(
            lifted_indices
                .iter()
                .fold(BinaryPoint::Infinity, |sum, &index| {
                    kc.add(&sum, &fb.points[index])
                }),
            target
        );
        let indices5 = [0, 2, 165, 168, 170];
        let target5 = indices5.iter().fold(BinaryPoint::Infinity, |sum, &index| {
            kc.add(&sum, &fb.points[index])
        });
        let row5 = checked_primary_relation_row(kc, fb, &indices5, &target5).unwrap();
        let xs5: Vec<_> = indices5
            .iter()
            .map(|&index| match &fb.points[index] {
                BinaryPoint::Affine { x, .. } => x.clone(),
                BinaryPoint::Infinity => panic!("five-point control index is infinity"),
            })
            .collect();
        let (lifted5, replayed5) = checked_five_x_lift(kc, fb, &xs5, &target5)
            .unwrap()
            .expect("constructed five-point target has a stored group lift");
        assert_eq!(replayed5, row5);
        assert_eq!(
            lifted5.iter().fold(BinaryPoint::Infinity, |sum, &index| {
                kc.add(&sum, &fb.points[index])
            }),
            target5
        );
        assert!(
            checked_primary_relation_row(kc, fb, &indices5, &kc.add(&target5, kc.generator()))
                .is_err()
        );
        assert!(checked_five_x_lift(kc, fb, &xs5[..4], &target5).is_err());
        let lambda_82 = kc.lambda.modpow(&BigUint::from(82u32), &kc.subgroup_order);
        assert_eq!(
            relation[0],
            (BigUint::one() + &kc.lambda + &kc.subgroup_order - lambda_82) % &kc.subgroup_order
        );
        assert_eq!(relation[1], kc.lambda);
        assert!(relation.iter().any(|coefficient| coefficient.bits() > 64));
        assert!(checked_primary_relation_row(kc, fb, &[usize::MAX, 2, 165, 168], &target).is_err());
        assert!(checked_primary_relation_row(kc, fb, &indices, &point_neg(&target)).is_err());
        let mut bad_labels = fb.clone();
        bad_labels.signed_orbit_of[2] = (1, 1, false);
        assert!(checked_primary_relation_row(kc, &bad_labels, &indices, &target).is_err());
        let mut rank = WideRankTracker::new(&kc.subgroup_order, fb.unknowns());
        for (column, indices) in [(0, [0; 4]), (1, [166; 4])] {
            let target = indices.iter().fold(BinaryPoint::Infinity, |sum, &index| {
                kc.add(&sum, &fb.points[index])
            });
            let row = checked_primary_relation_row(kc, fb, &indices, &target).unwrap();
            assert_eq!(rank.insert(row.clone()), column + 1);
            assert_eq!(rank.insert(row), column + 1);
        }
        let mut opts = KoblitzIcOptions::default();
        opts.max_trials = 0;
        opts.allow_direct_relation = false;
        // The default pair table packs keys into u64 and cannot represent
        // this degree-83 base. The preflight itself makes no query.
        opts.strategy = DecompositionStrategy::Enumerate;
        let report = koblitz_index_calculus_dlp_with_factor_base(
            &base.curve,
            base.curve.generator(),
            &base.factor_base,
            &opts,
        )
        .unwrap();
        assert_eq!(report.factor_base_size, 332);
        assert_eq!(report.orbit_count, 2);
        assert_eq!(report.trials, 0);
        assert!(report.log.is_none());

        let mut entries: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let mut wrong_header = entries.clone();
        wrong_header[0]["seed"] = json!(18);
        let bad_header = wrong_header
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(from_record(&row, bad_header.as_bytes()).is_err());
        entries[1]["coefficient"] = json!("0");
        let bad = entries
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(from_record(&row, bad.as_bytes()).is_err());
    }

    #[test]
    fn wide_enumerator_matches_generic_witness_order_on_primary_curve() {
        let (bytes, row) = construct(
            0,
            "public_x_hash",
            2,
            17,
            Instant::now() + Duration::from_secs(60),
        )
        .unwrap();
        let base = from_record(&row, &bytes).unwrap();
        let kc = &base.curve;
        let fb = &base.factor_base;
        let index: HashMap<_, _> = fb
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| (point_key(p), i))
            .collect();
        fn reference(
            kc: &KoblitzCurve,
            fb: &FrobeniusFactorBase,
            index: &HashMap<(BigUint, BigUint), usize>,
            target: &BinaryPoint,
            m: usize,
            start: usize,
        ) -> Option<Vec<usize>> {
            if m == 0 {
                return (target == &BinaryPoint::Infinity).then(Vec::new);
            }
            if m == 1 {
                let i = *index.get(&point_key(target))?;
                return (i >= start).then(|| vec![i]);
            }
            for i in start..fb.points.len() {
                let rest = kc.add(target, &point_neg(&fb.points[i]));
                if let Some(mut tail) = reference(kc, fb, index, &rest, m - 1, i) {
                    let mut result = vec![i];
                    result.append(&mut tail);
                    return Some(result);
                }
            }
            None
        }
        let p0 = &fb.points[0];
        let p2 = &fb.points[2];
        let two = kc.add(p0, p2);
        let three = kc.add(&kc.add(p0, p0), p2);
        let cases = [
            (BinaryPoint::Infinity, 0),
            (p0.clone(), 1),
            (BinaryPoint::Infinity, 2),
            (two, 2),
            (three, 3),
        ];
        for (target, m) in cases {
            assert_eq!(
                enumerate_decompose(kc, fb, &index, &target, m),
                reference(kc, fb, &index, &target, m, 0),
                "wide witness at m={m}"
            );
        }
    }

    #[test]
    fn public_target_and_post_solver_validation_bind_the_same_fixture() {
        let (bytes, row) = construct(
            0,
            "public_x_hash",
            2,
            17,
            Instant::now() + Duration::from_secs(60),
        )
        .unwrap();
        let base = from_record(&row, &bytes).unwrap();
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "n83-primary-fixture-{}-{suffix}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let target = base.curve.generator().clone();
        let alternate = base.curve.mul(&target, &BigUint::from(2u8));
        let corpus = json!({
            "schema":"n83.public-probe-corpus/v1", "study":STUDY,
            "targets":[
                {"a":0,"fixture":0,"point":point_json(words(&target))},
                {"a":0,"fixture":1,"point":point_json(words(&alternate))}
            ]
        });
        write_new_json(&root.join("probe-corpus.json"), &corpus).unwrap();
        let (selected, hash) = public_target(&root, &base.curve).unwrap();
        assert_eq!(selected, target);
        let (selected_one, hash_one) = public_target_fixture(&root, &base.curve, 1).unwrap();
        assert_eq!(selected_one, alternate);
        assert_eq!(hash_one, hash);
        assert!(public_target_fixture(&root, &base.curve, 64).is_err());
        let validation = json!({
            "schema":"n83.public-probe-validation/v1",
            "corpus_canonical_json_blake3":hash,
            "oracle_receives_known_answer_scalars":false,
            "validation":[{"a":0,"fixture":0,"known_answer_scalar":"1"}]
        });
        write_new_json(&root.join("probe-validation.json"), &validation).unwrap();
        assert!(candidate_matches_validation(&root, &hash, &BigUint::one()).unwrap());
        assert!(!candidate_matches_validation(&root, &hash, &BigUint::from(2u8)).unwrap());
        let duplicate = json!({
            "schema":"n83.public-probe-corpus/v1", "study":STUDY,
            "targets":[
                {"a":0,"fixture":0,"point":point_json(words(&target))},
                {"a":0,"fixture":0,"point":point_json(words(&target))}
            ]
        });
        fs::write(
            root.join("probe-corpus.json"),
            serde_json::to_vec(&duplicate).unwrap(),
        )
        .unwrap();
        assert!(public_target(&root, &base.curve).is_err());
        let rejected_run = root.join("rejected-sat-m3");
        let error = run_cli(&root, 64, 3, "sat-m3", 1, 1, &rejected_run).unwrap_err();
        assert!(error.to_string().contains("gate is incomplete for F2^83"));
        assert!(!rejected_run.exists());
        let unguarded_chain = root.join("unguarded-chain");
        assert!(run_cli(&root, 64, 5, "chain-s3", 1, 1, &unguarded_chain).is_err());
        assert!(!unguarded_chain.exists());
        for orbit_domain in [false, true] {
            let invalid_caps = root.join(format!("invalid-chain-caps-{orbit_domain}"));
            let error = run_chain_cli(
                &root,
                &root,
                64,
                "public_x_hash",
                SEEDS[0],
                5,
                1,
                ChainS3Limits {
                    max_variables: 0,
                    max_domain_clauses: 1,
                    max_models: 1,
                    conflict_budget: 1,
                },
                1,
                256,
                &invalid_caps,
                orbit_domain,
                false,
            )
            .unwrap_err();
            assert!(error.to_string().contains("outside supported domain"));
            assert!(!invalid_caps.exists());
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn v2_primary_selection_requires_replay_and_s3_receipts_before_object_import() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("n83-v2-import-gate-{suffix}"));
        fs::create_dir(&root).unwrap();
        let digest = "a".repeat(64);
        let object = format!("objects/{digest}.jsonl.gz");
        let uri = format!(
            "s3://crypto-autoresearcher/factor-bases/icv1/etc/{STUDY}/v2-size-frontier/a0/{object}"
        );
        let manifest = json!({
            "schema":"n83.factor-base-panel/v2-size-frontier", "study":STUDY,
            "status":"completed_factor_base_object", "expected_base_count":1,
            "completed_base_count":1, "size_design_blake3":v2_design_hash(),
            "bases":[{"a":0,"policy":"public_x_hash","seed":SEEDS[0],
                "columns":1182,"object":object,"s3_uri":uri,
                "compressed_blake3":digest,"plain_blake3":"b".repeat(64)}],
        });
        write_new_json(&root.join("manifest.json"), &manifest).unwrap();
        let manifest_hash = blake3::hash(&fs::read(root.join("manifest.json")).unwrap())
            .to_hex()
            .to_string();
        write_new_json(
            &root.join("replay.json"),
            &json!({
                "schema":"n83.factor-base-replay/v2-size-frontier", "status":"PASS",
                "panel_manifest_blake3":manifest_hash,
                "checks":[{"object":object,"points_checked":166*1182,
                    "representatives_checked":1182,"status":"PASS"}],
            }),
        )
        .unwrap();
        let storage = json!({
            "schema":"n83.factor-base-storage/v2-size-frontier", "status":"PASS",
            "bucket":"crypto-autoresearcher",
            "prefix":format!("{PREFIX}/{STUDY}/{V2_SUBDIR}"),
            "manifest_blake3":manifest_hash,
            "objects":[{"s3_uri":uri,"compressed_blake3":digest,
                "downloaded_hash_matches":true}],
        });
        write_new_json(&root.join("upload-receipt.json"), &storage).unwrap();
        let error = load_panel_selected(&root, 1182, "public_x_hash", SEEDS[0])
            .err()
            .unwrap();
        assert!(error.to_string().contains("No such file"), "{error}");
        let mut bad = storage;
        bad["objects"][0]["downloaded_hash_matches"] = json!(false);
        fs::write(
            root.join("upload-receipt.json"),
            serde_json::to_vec(&bad).unwrap(),
        )
        .unwrap();
        let error = load_panel_selected(&root, 1182, "public_x_hash", SEEDS[0])
            .err()
            .unwrap();
        assert!(error.to_string().contains("S3 round-trip"), "{error}");
        fs::remove_dir_all(root).unwrap();
    }
}
