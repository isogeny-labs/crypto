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
    frobenius_orbit_domain_size_with_encoding, ChainedS3Encoding, FrobeniusOrbitRootEncoding,
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_explicit_frobenius_point_factor_base, build_union_s4_encoding,
    koblitz_index_calculus_dlp_with_factor_base_and_progress, koblitz_point_count, point_key,
    ChainS3Limits, DecompositionStrategy, FrobeniusFactorBase, KoblitzCurve, KoblitzIcOptions,
    SatDecompositionOptions,
use crypto_lib::cryptanalysis::koblitz_relation_solver::WideRankTracker;
use crypto_lib::cryptanalysis::sat::{
    to_dimacs_xor, RootPropagationResult, SolveResult, SolverProgress, SolverStats,
use crypto_lib::hash::sha256;
use crypto_lib::store::{backend::RemoteSpec, Store};
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
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
        .map_err(|_| "capacity worker requires a finite cgroup memory.swap.max")?;
    if actual != bytes || swap != 0 {
        return Err("capacity worker requires the declared hard memory cap and zero swap".into());
    Ok(bytes)

fn process_usage() -> Value {
    #[cfg(unix)]
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
    json!({"cpu_seconds":null,"peak_rss_bytes":null})

pub(super) struct PrimaryBase {
    pub(super) curve: KoblitzCurve,
    pub(super) factor_base: FrobeniusFactorBase,

fn from_record(row: &Value, bytes: &[u8]) -> Result<PrimaryBase> {
    let mut lines = std::str::from_utf8(bytes)?.lines();
    let header: Value = serde_json::from_str(lines.next().ok_or("missing base header")?)?;
    let columns = row["columns"].as_u64().ok_or("column count")? as usize;
    if columns == 0 || header["orbit_columns"].as_u64() != Some(columns as u64) {
        return Err("primary orbit-column count mismatch".into());
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
        return Err("primary curve or manifest binding mismatch".into());
    let reps = header["representatives"]
        .as_array()
        .ok_or("representatives")?;
    if reps.len() != columns {
        return Err("representative count mismatch".into());

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
                    coefficient.to_string()
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
                    positive_orbit.push(index);
            current = pinned.frobenius(&current);
            coefficient = (&coefficient * &pinned.lambda) % &pinned.subgroup_order;
        if current != rep {
            return Err("primary orbit did not close".into());
        orbits.push(positive_orbit);
        orbits.push(negative_orbit);
        signed_orbits.push(signed_members);
    if lines.next().is_some()
        || points.len() != 166 * columns
        || x_values.len() != 83 * columns
        || row["point_set_blake3"] != point_set_hash(point_words)?
        return Err("primary point count or set hash mismatch".into());

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
        return Err("primary orbit maps differ from the stored labels".into());
    Ok(PrimaryBase { curve, factor_base })

/// Replay a newly constructed object with the generic binary-curve path
/// before it is admitted to the artifact store.
pub(super) fn replay_constructed_primary(row: &Value, bytes: &[u8]) -> Result<(usize, usize)> {
    let base = from_record(row, bytes)?;
    Ok((
        base.factor_base.points.len(),
        base.factor_base.signed_orbits.len(),
    ))

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
        return Err("primary base policy or seed outside frozen panel".into());
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
    let panel_shape_valid = if v2 {
        manifest["status"] == "completed_factor_base_object"
            && manifest["size_design_blake3"] == v2_design_hash()
            && manifest["expected_base_count"] == 1
            && manifest["completed_base_count"] == 1
        manifest["schema"] == "n83.factor-base-panel/v1"
            && manifest["status"] == "completed_factor_base_panel"
            && manifest["expected_base_count"] == 54
            && manifest["completed_base_count"] == 54
    if !panel_shape_valid
        || manifest["study"] != STUDY
        || replay["schema"]
            != if v2 {
                "n83.factor-base-replay/v2-size-frontier"
                "n83.factor-base-replay/v1"
        || replay["status"] != "PASS"
        || replay["panel_manifest_blake3"] != manifest_hash
        || storage["schema"]
                "n83.factor-base-storage/v2-size-frontier"
                "n83.factor-base-storage/v1"
        || storage["status"] != "PASS"
        || storage["manifest_blake3"] != manifest_hash
        || storage["bucket"] != "crypto-autoresearcher"
        || storage["prefix"] != storage_prefix
        return Err("panel or replay receipt is not complete and bound".into());
    let rows = manifest["bases"].as_array().ok_or("panel base rows")?;
    if rows.len() != expected_count || (v2 && !v2_row_is_declared(&rows[0])) {
        return Err("panel base count mismatch".into());
    let mut rows = rows.iter().filter(|row| {
        row["a"].as_u64() == Some(0)
            && row["policy"] == policy
            && row["seed"].as_u64() == Some(seed)
            && row["columns"].as_u64() == Some(columns as u64)
    });
    let row = rows.next().ok_or("requested primary base absent")?;
    if rows.next().is_some() {
        return Err("ambiguous primary base selection".into());
    let expected_uri = if v2 {
        format!(
            "{storage_prefix}/a0/{}",
            row["object"].as_str().ok_or("object path")?
        )
            "{PREFIX}/{STUDY}/a0/{}",
    if row["s3_uri"] != expected_uri {
        return Err("selected base S3 destination mismatch".into());
    let checks = replay["checks"].as_array().ok_or("replay checks")?;
    let matching: Vec<_> = checks
        .iter()
        .filter(|check| check["object"] == row["object"])
        .collect();
    if matching.len() != 1
        || matching[0]["status"] != "PASS"
        || matching[0]["points_checked"].as_u64() != Some(166 * columns as u64)
        || matching[0]["representatives_checked"].as_u64() != Some(columns as u64)
        return Err("selected object lacks a successful replay".into());
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
        return Err("selected object lacks a matching S3 round-trip receipt".into());
    let base = from_record(row, &load_object(root, row)?)?;
    Ok((base, row.clone(), manifest_hash))

fn load_panel(root: &Path, columns: usize) -> Result<(PrimaryBase, Value, String)> {
    load_panel_selected(root, columns, "public_x_hash", SEEDS[0])

/// Successor custody path for a newly published primary base. The historical
/// panel reader above remains bound to its original S3 bucket and receipts.
fn load_store_primary_base(root: &Path, store: &Store) -> Result<(PrimaryBase, Value)> {
    let manifest_sha256 = sha256_hex(&manifest_bytes);
    let storage: Value = serde_json::from_slice(&fs::read(root.join("storage.json"))?)?;
    let row = &manifest["base"];
    let remote = manifest["remote"].as_str().ok_or("store remote missing")?;
    let store_name = row["store_name"].as_str().ok_or("store name missing")?;
    let digest = row["compressed_sha256"]
        .as_str()
        .ok_or("compressed SHA-256 missing")?;
    let bytes = row["bytes"].as_u64().ok_or("compressed size missing")?;
    let plain_bytes = row["plain_bytes"].as_u64().ok_or("plain size missing")?;
    let columns = row["columns"].as_u64().ok_or("orbit columns missing")?;
    let source_commit = manifest["source_commit"]
        .ok_or("source commit missing")?;
    if manifest["schema"] != "n83.factor-base-panel/v3-artifact-store"
        || manifest["status"] != "completed_factor_base_object"
        || manifest["curve_id"] != "icv1-f2m83-tm6151469093347-debefd74"
        || source_commit.len() != 40
        || !source_commit.bytes().all(|b| b.is_ascii_hexdigit())
        || row["a"] != 0
        || row["closure"] != "signed_frobenius"
        || ![
            "public_x_sequential",
            "public_x_hash",
            "public_x_gray_prefix",
        ]
        .any(|policy| row["policy"] == *policy)
        || columns == 0
        || row["points"].as_u64() != columns.checked_mul(166)
        || bytes == 0
        || plain_bytes == 0
        || !store_name.starts_with("factor-bases/icv1-f2m83-tm6151469093347-debefd74/")
        || !store_name.ends_with(".jsonl.gz")
        || store.cfg.remote.as_ref() != Some(&RemoteSpec::parse(remote)?)
        return Err("successor primary base manifest differs from custody contract".into());
    if replay["schema"] != "n83.factor-base-replay/v3-artifact-store"
        || replay["manifest_sha256"] != manifest_sha256
        || replay["store_name"] != store_name
        || replay["point_set_blake3"] != row["point_set_blake3"]
        || replay["points_checked"].as_u64() != row["points"].as_u64()
        || replay["representatives_checked"].as_u64() != Some(columns)
        || storage["schema"] != "n83.factor-base-storage/v3-artifact-store"
        || storage["manifest_sha256"] != manifest_sha256
        || storage["remote"] != remote
        || storage["store_name"] != store_name
        || storage["sha256"] != digest
        || storage["bytes"].as_u64() != Some(bytes)
        || storage["fresh_download_sha256"] != digest
        || storage["fresh_download_bytes"].as_u64() != Some(bytes)
        return Err("successor primary base lacks replay or fresh-download custody".into());
    let (indexed_name, indexed_digest, indexed_bytes) = store.lookup(store_name)?;
    if indexed_name.as_deref() != Some(store_name)
        || indexed_digest != digest
        || indexed_bytes != Some(bytes)
        return Err("artifact-store index differs from successor manifest".into());
    let resolved = store.ensure(store_name)?;
    if resolved.sha256 != digest || resolved.bytes != bytes {
        return Err("resolved artifact differs from successor manifest".into());
    let compressed = fs::read(&resolved.path)?;
    if compressed.len() as u64 != bytes
        || sha256_hex(&compressed) != digest
        || blake3::hash(&compressed).to_hex().as_str()
            != row["compressed_blake3"]
                .as_str()
                .ok_or("compressed BLAKE3 missing")?
        return Err("successor compressed object differs".into());
    let mut plain = Vec::new();
    GzDecoder::new(&compressed[..])
        .take(plain_bytes.checked_add(1).ok_or("plain size overflow")?)
        .read_to_end(&mut plain)?;
    if plain.len() as u64 != plain_bytes
        || blake3::hash(&plain).to_hex().as_str()
            != row["plain_blake3"].as_str().ok_or("plain BLAKE3 missing")?
        return Err("successor decompressed object differs".into());
    let base = from_record(row, &plain)?;
    if row["point_set_blake3"]
        .is_none_or(|value| value.len() != 64)
        || base.factor_base.points.len() as u64 != row["points"].as_u64().unwrap_or(0)
        return Err("successor factor-base point count or digest differs".into());
    Ok((base, row.clone()))

pub(super) fn verify_store_v3(root: &Path, store: &Store) -> Result<Value> {
    let (base, row) = load_store_primary_base(root, store)?;
    let receipt = json!({
        "schema":"n83.factor-base-store-import/v3",
        "status":"PASS","curve_id":base.curve.label(),
        "store_name":row["store_name"],
        "compressed_sha256":row["compressed_sha256"],
        "point_set_blake3":row["point_set_blake3"],
        "points_checked":base.factor_base.points.len(),
        "orbit_columns_checked":base.factor_base.signed_orbits.len(),
        "selected_best_total_runtime":Value::Null,
    Ok(receipt)

pub(super) fn check_store_v3_cli(root: &Path, output: &Path) -> Result<Value> {
    if output.exists() {
        return Err("successor import output already exists".into());
    let store = Store::open_default()?;
    let receipt = verify_store_v3(root, &store)?;
    write_new_json(output, &receipt)?;

const FULL_D18_MANIFEST: &[u8] =
    include_bytes!("verification/d18-full-orbit-20261025/manifest.json");
const FULL_D18_REPLAY: &[u8] = include_bytes!("verification/d18-full-orbit-20261025/replay.json");
const FULL_D18_CUSTODY: &[u8] =
    include_bytes!("verification/d18-full-orbit-20261025/s3-independent-audit.json");
const FULL_D18_OBJECT: &str = "c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f";
const FULL_D18_SHA256: &str = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b";
const FULL_D18_POINT_SET: &str = "31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d";
const FULL_D18_PUBLIC_CORPUS_SHA256: &str =
    "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";

fn sha256_hex(bytes: &[u8]) -> String {
    sha256::sha256(bytes)
        .map(|byte| format!("{byte:02x}"))
        .collect()

fn pack_model_bits(model: &[bool]) -> Vec<u8> {
    let mut packed = vec![0u8; model.len().div_ceil(8)];
    for (index, &value) in model.iter().enumerate() {
        if value {
            packed[index / 8] |= 1 << (index % 8);
    packed

fn full_d18_phase(file: &mut fs::File, started: Instant, phase: &str, detail: Value) -> Result<()> {
    writeln!(
        file,
        "{}",
        json!({
            "schema":"n83.full-d18-m4-phase/v1", "phase":phase,
            "elapsed_process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "detail":detail,
    )?;
    file.sync_all()?;
    Ok(())

fn full_d18_load(root: &Path) -> Result<(PrimaryBase, Value)> {
    if manifest_bytes != FULL_D18_MANIFEST {
        return Err("full D18 producer manifest differs from committed evidence".into());
    let replay: Value = serde_json::from_slice(FULL_D18_REPLAY)?;
    let custody: Value = serde_json::from_slice(FULL_D18_CUSTODY)?;
    let object = format!("objects/{FULL_D18_OBJECT}.jsonl.gz");
    if manifest["schema"] != "n83.direct-subgroup-orbit-export/v1"
        || manifest["policy"] != "direct_subgroup_x_d18_full"
        || manifest["source_commit"] != "5bac5f1d9221b908f743520aa9039dd48d175f0e"
        || manifest["columns"] != 32_643
        || manifest["points"] != 5_418_738
        || manifest["object"] != object
        || manifest["compressed_blake3"] != FULL_D18_OBJECT
        || manifest["compressed_sha256"] != FULL_D18_SHA256
        || manifest["point_set_blake3"] != FULL_D18_POINT_SET
        || manifest["bytes"] != 160_747_529u64
        || manifest["selected_best_total_runtime"] != Value::Null
        || replay["points_checked"] != 5_418_738
        || replay["point_set_blake3"] != FULL_D18_POINT_SET
        || replay["compressed_sha256"] != FULL_D18_SHA256
        || custody["status"] != "PASS_FULL_OBJECT_AND_MANIFEST"
        || custody["source_commit"] != manifest["source_commit"]
        || custody["fresh_get"]["sha256"] != FULL_D18_SHA256
        || custody["fresh_get"]["blake3"] != FULL_D18_OBJECT
        || custody["point_set_blake3"] != FULL_D18_POINT_SET
        return Err("full D18 archive, generic replay or S3 custody binding differs".into());
    let compressed = fs::read(root.join(&object))?;
    let actual_sha = sha256_hex(&compressed);
    if compressed.len() != 160_747_529 || actual_sha != FULL_D18_SHA256 {
        return Err("full D18 archive length or SHA-256 differs".into());
    let row = json!({
        "a":0, "columns":32_643, "points":5_418_738,
        "policy":"direct_subgroup_x_d18_full", "seed":0,
        "object":object, "compressed_blake3":FULL_D18_OBJECT,
        "plain_blake3":manifest["plain_blake3"],
        "point_set_blake3":FULL_D18_POINT_SET,
    let base = from_record(&row, &load_object(root, &row)?)?;
    Ok((base, row))

/// Export the retained K2048 five-summand orbit circuit for an external XOR
/// solver. The constructed control can pin witness x coordinates or leave
/// them free while retaining the same public target and validation witness.
pub(super) fn k2048_m5_xcnf_cli(mode: &str, root: &Path, output: &Path) -> Result<Value> {
    k2048_m5_xcnf_export(
        mode,
        root,
        output,
        if mode == "natural-sorted" { 1 } else { 0 },
    )

/// Export a frozen public corpus fixture with the same retained K2048 base.
/// The constructed fixture-zero control remains the separate legacy command.
pub(super) fn k2048_m5_xcnf_fixture_cli(root: &Path, fixture: u64, output: &Path) -> Result<Value> {
    if fixture >= 32 {
        return Err("K2048 m5 public fixture outside sealed corpus".into());
    k2048_m5_xcnf_export("natural", root, output, fixture)

fn k2048_m5_xcnf_export(mode: &str, root: &Path, output: &Path, fixture: u64) -> Result<Value> {
    const OBJECT: &str =
        "objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz";
    const OBJECT_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
    const MANIFEST_SHA256: &str =
        "68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8";
    const REPLAY_SHA256: &str = "5ad374ef0042b2ebd6562746b7ca67e46c6b0f4d46c4ba25abae83fd2dd411e2";
    const STORAGE_SHA256: &str = "f1139c1dd61beac0e1d9e80a1d02e983f98c7b1abc6887f1816f013d4693bf4c";
    const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
        "positive",
        "positive-unpinned",
        "natural",
        "positive-sorted-pinned",
        "positive-sorted-unpinned",
        "natural-sorted",
    .contains(&mode)
        || output.exists()
        || output.with_extension("xcnf").exists()
        return Err("K2048 m5 mode or fresh output differs from protocol".into());
    for (name, expected) in [
        (OBJECT, OBJECT_SHA256),
        ("manifest.json", MANIFEST_SHA256),
        ("replay.json", REPLAY_SHA256),
        ("upload-receipt.json", STORAGE_SHA256),
        ("probe-corpus.json", CORPUS_SHA256),
    ] {
        if sha256_hex(&fs::read(root.join(name))?) != expected {
            return Err(format!("K2048 m5 input {name} differs from frozen SHA-256").into());
    let (source_commit, source_attestation_mode) = frozen_chain_source()?;
    let binary_sha256 = sha256_hex(&fs::read(std::env::current_exe()?)?);
    let started = Instant::now();
    let (base, row, manifest_blake3) = load_panel_selected(root, 2_048, "public_x_hash", SEEDS[0])?;
    let fb = &base.factor_base;
    if fb.points.len() != 339_968 || fb.signed_orbits.len() != 2_048 {
        return Err("K2048 m5 imported base size differs".into());
    let representatives: Vec<BigUint> = fb
        .signed_orbits
        .map(|orbit| match &fb.points[orbit[0]] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("K2048 representative is infinity".into()),
        .collect::<Result<_>>()?;
    let (public, corpus_blake3) = public_target_fixture(root, &base.curve, fixture)?;
    if fixture == 0
        && point_json(words(&public))
            != json!(["1834913121178197242856238", "8797964190732846213981377"])
        return Err("K2048 m5 public target differs from frozen fixture".into());
    let sorted = matches!(
        "positive-sorted-pinned" | "positive-sorted-unpinned" | "natural-sorted"
    );
    let pinned = matches!(mode, "positive" | "positive-sorted-pinned");
    let constructed = matches!(
        "positive" | "positive-unpinned" | "positive-sorted-pinned" | "positive-sorted-unpinned"
    let mut positive = None;
    let target = if constructed {
        let mut indices: [usize; 5] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
        if indices
            .any(|&index| matches!(fb.points[index], BinaryPoint::Infinity))
            return Err("K2048 constructed representative is identity".into());
        if sorted {
            indices.sort_by_key(|&index| match &fb.points[index] {
                BinaryPoint::Affine { x, .. } => x.to_biguint(),
                BinaryPoint::Infinity => unreachable!("representatives checked nonidentity"),
        let mut partial = Vec::with_capacity(5);
        let mut sum = BinaryPoint::Infinity;
        for &index in &indices {
            sum = base.curve.add(&sum, &fb.points[index]);
            partial.push(sum.clone());
        if partial
            .any(|point| !matches!(point, BinaryPoint::Affine { .. }))
            return Err("K2048 m5 positive has an identity intermediate".into());
        let relation = checked_primary_relation_row(&base.curve, fb, &indices, &sum)?;
        positive = Some((indices, partial, relation));
        sum
        public
    let BinaryPoint::Affine { x: target_x, .. } = &target else {
        return Err("K2048 m5 target is identity".into());
    let model_started = Instant::now();
    let mut enc = ChainedS3Encoding::build(
        5,
        &base.curve.curve.irreducible,
        &base.curve.curve.b,
        Some(target_x),
        0,
        200_000,
    .map_err(|_| "K2048 m5 chained-S3 circuit construction failed")?;
    enc.constrain_frobenius_orbit_summands_with_encoding(
        &representatives,
        FrobeniusOrbitRootEncoding::SharedBits,
        3_000_000,
    .map_err(|_| "K2048 m5 shared-root orbit domain failed")?;
    if sorted {
        enc.order_all_summands(200_000)
            .map_err(|_| "K2048 m5 all-summand ordering failed")?;
    if pinned {
        let (indices, partial, _) = positive.as_ref().ok_or("K2048 positive witness missing")?;
        let points: Vec<&BinaryPoint> = indices
            .map(|&index| &fb.points[index])
            .chain(partial[1..4].iter())
            .chain(std::iter::once(&target))
            .collect();
        if points.len() != 9 {
            return Err("K2048 m5 positive block count differs".into());
        for (block, point) in points.iter().enumerate() {
            let BinaryPoint::Affine { x, .. } = point else {
                return Err("K2048 m5 positive pin is identity".into());
            };
            let x = x.to_biguint();
            for bit in 0..83usize {
                let wire = (block * 83 + bit + 1) as i32;
                if !enc
                    .solver
                    .add_clause(vec![if x.bit(bit as u64) { wire } else { -wire }])
                    return Err("K2048 m5 positive pin contradicted circuit".into());
    let model_ms = model_started.elapsed().as_secs_f64() * 1000.0;
    let shape = json!({
        "variables":enc.solver.n_vars(), "clauses":enc.solver.n_clauses(),
        "xor_rows":enc.solver.n_xors(), "model_ms":model_ms,
        "summands":5,"identity_mask":0,"root_encoding":"shared_root_bits",
        "domain_representatives":representatives.len(),
        "positive_x_pins":if pinned { 9*83 } else { 0 },
        "summand_ordering":if sorted { "all_nondecreasing_x" } else { "none" },
    let export_started = Instant::now();
    let xcnf = to_dimacs_xor(&enc.solver);
    if xcnf.len() > 200_000_000 {
        return Err("K2048 m5 XCNF exceeds frozen output cap".into());
    let xcnf_path = output.with_extension("xcnf");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&xcnf_path)?;
    file.write_all(xcnf.as_bytes())?;
    let xcnf_sha256 = sha256_hex(xcnf.as_bytes());
    let positive_receipt = if let Some((indices, _, row)) = positive {
        let row_bytes =
            serde_json::to_vec(&row.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        Some(json!({
            "indices":indices,
            "relation_row_decimal_blake3":blake3::hash(&row_bytes).to_hex().to_string(),
        }))
        None
    let result = json!({
        "schema":"n83.k2048-m5-xcnf-export/v1",
        "status":"PASS_XCNF_EXPORT_ONLY", "mode":mode,
        "curve_id":base.curve.label(),"curve_a":0,
        "source_commit":source_commit,"source_attestation_mode":source_attestation_mode,
        "binary_sha256":binary_sha256,"object":row["object"],
        "object_sha256":OBJECT_SHA256,"point_set_blake3":row["point_set_blake3"],
        "panel_manifest_blake3":manifest_blake3,"public_corpus_blake3":corpus_blake3,
        "public_fixture":if mode == "natural" || mode == "natural-sorted" { Some(fixture) } else { None },
        "target":point_json(words(&target)),"shape":shape,
        "positive":positive_receipt,
        "xcnf_sha256":xcnf_sha256,"xcnf_bytes":xcnf.len(),
        "xcnf_path":xcnf_path.to_string_lossy(),
        "export_ms":export_started.elapsed().as_secs_f64()*1000.0,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(),"solver_search_executed":false,
        "total_index_calculus_runtime_ms":Value::Null,
    write_new_json(output, &result)?;
    Ok(result)

/// Export the retained full-base K600 six-summand orbit circuit. Constructed
/// controls and the public point use the same archive and identity pattern.
pub(super) fn k600_m6_xcnf_cli(mode: &str, root: &Path, output: &Path) -> Result<Value> {
        "objects/b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e.jsonl.gz";
    const OBJECT_SHA256: &str = "3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf";
        "5bd862987844f709e03beeab9fd265e3d2024011bcadcb82f5af9c72018df6a6";
    const REPLAY_SHA256: &str = "fa5e545c27dd64c32a47ddae05a2a0b3a031545d700897f3e0e34f59abc0d72e";
    const STORAGE_SHA256: &str = "0eae40ffa0e938470671cf796633c1e47b61ff676df0807e7a7e8592e56bb828";
    const VALIDATION_SHA256: &str =
        "1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f";
    if !["positive-pinned", "positive-unpinned", "natural"].contains(&mode)
        return Err("K600 m6 mode or fresh output differs from protocol".into());
        ("probe-validation.json", VALIDATION_SHA256),
            return Err(format!("K600 m6 input {name} differs from frozen SHA-256").into());
    let (base, row, manifest_blake3) = load_panel_selected(root, 600, "public_x_hash", SEEDS[0])?;
    if fb.points.len() != 99_600
        || fb.signed_orbits.len() != 600
        || row["point_set_blake3"]
            != "ac664f5f0c7122c52a08c0a288b543efb709111a99dc2a5cb2bef025ffffee21"
        return Err("K600 m6 imported base identity differs".into());
            BinaryPoint::Infinity => Err("K600 representative is infinity".into()),
    let (public, corpus_blake3) = public_target_fixture(root, &base.curve, 0)?;
    let pinned = mode == "positive-pinned";
    let constructed = mode != "natural";
        let indices: [usize; 6] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
        let mut partial = Vec::with_capacity(6);
            return Err("K600 m6 constructed chain has an identity point".into());
        return Err("K600 m6 target is identity".into());
        6,
        125_000,
    .map_err(|_| "K600 m6 chained-S3 circuit construction failed")?;
    enc.constrain_frobenius_orbit_summands(&representatives, 125_000, 350_000)
        .map_err(|_| "K600 m6 orbit domain failed")?;
        let (indices, partial, _) = positive.as_ref().ok_or("K600 positive witness missing")?;
            .chain(partial[1..5].iter())
        if points.len() != 11 {
            return Err("K600 m6 positive block count differs".into());
                return Err("K600 m6 positive pin is identity".into());
                    return Err("K600 m6 positive pin contradicted circuit".into());
        "summands":6,"identity_mask":0,"root_encoding":"per_representative",
        "positive_x_pins":if pinned { 11*83 } else { 0 },
        "summand_ordering":"none",
        return Err("K600 m6 XCNF exceeds frozen output cap".into());
    let positive_receipt = if let Some((indices, partial, row)) = positive {
            "points":indices.map(|index| planted_point_json(&fb.points[index])),
            "partial_sums":partial.iter().map(planted_point_json).collect::<Vec<_>>(),
        "schema":"n83.k600-m6-xcnf-export/v1",
        "public_fixture":if mode == "natural" { Some(0) } else { None },

/// Parse a complete DIMACS model. Zero is the unused index; 1 and 2 encode
/// true and false so duplicate and missing assignments cannot be hidden.
fn k600_model_values(stdout: &str, variables: usize) -> Result<Vec<u8>> {
    let mut values = vec![0u8; variables.checked_add(1).ok_or("model size overflow")?];
    let mut statuses = 0usize;
    for line in stdout.lines() {
        if line.starts_with("s ") {
            if line != "s SATISFIABLE" {
                return Err("solver status is not SAT".into());
            statuses += 1;
        } else if let Some(rest) = line.strip_prefix("v ") {
            for token in rest.split_whitespace() {
                let literal: i64 = token.parse()?;
                if literal == 0 {
                    continue;
                let index = usize::try_from(literal.unsigned_abs())?;
                if index == 0 || index > variables || values[index] != 0 {
                    return Err("model variable out of range or repeated".into());
                values[index] = if literal > 0 { 1 } else { 2 };
    if statuses != 1 || values[1..].contains(&0) {
        return Err("solver model is absent, partial, or ambiguous".into());
    Ok(values)

/// Check the original XCNF, including parity of signed XOR literals.
fn k600_check_xcnf(formula: &str, values: &[u8], cnf: usize, xor: usize) -> Result<()> {
    let mut lines = formula.lines();
    let header = lines.next().ok_or("empty XCNF")?;
    if header
        != format!(
            "p cnf {} {}",
            values.len() - 1,
            cnf.checked_add(xor).ok_or("row count overflow")?
        return Err("XCNF header differs".into());
    let (mut seen_cnf, mut seen_xor) = (0usize, 0usize);
    for (offset, line) in lines.enumerate() {
        if line.starts_with("c ") {
            continue;
        let (is_xor, body) = if let Some(body) = line.strip_prefix("x ") {
            (true, body)
        } else {
            (false, line)
        };
        let tokens: Vec<_> = body.split_whitespace().collect();
        if tokens.len() < 2 || tokens.last() != Some(&"0") {
            return Err(format!("unterminated XCNF row {}", offset + 2).into());
        let mut parity = false;
        let mut satisfied = false;
        for token in &tokens[..tokens.len() - 1] {
            let literal: i64 = token.parse()?;
            let index = usize::try_from(literal.unsigned_abs())?;
            if literal == 0 || index >= values.len() {
                return Err(format!("invalid XCNF literal at row {}", offset + 2).into());
            let truth = (values[index] == 1) == (literal > 0);
            parity ^= truth;
            satisfied |= truth;
        if is_xor {
            seen_xor += 1;
            if !parity {
                return Err(format!("XOR row {} fails", offset + 2).into());
            seen_cnf += 1;
            if !satisfied {
                return Err(format!("CNF row {} fails", offset + 2).into());
    if (seen_cnf, seen_xor) != (cnf, xor) {
        return Err("XCNF row counts differ".into());

fn k600_model_x(values: &[u8], block: usize) -> BigUint {
    let mut x = BigUint::zero();
    for bit in 0..83usize {
        if values[block * 83 + bit + 1] == 1 {
            x |= BigUint::one() << bit;
    x

fn verify_k600_positive_witness(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    target: &BinaryPoint,
    positive: &Value,
) -> Result<Vec<usize>> {
    let indices = positive["indices"].as_array().ok_or("positive indices")?;
    let points = positive["points"].as_array().ok_or("positive points")?;
    let partial = positive["partial_sums"]
        .ok_or("positive partial sums")?;
    if indices.len() != 6 || points.len() != 6 || partial.len() != 6 {
        return Err("positive witness length differs".into());
    let mut sum = BinaryPoint::Infinity;
    let mut selected = Vec::with_capacity(6);
    for i in 0..6 {
        let index = indices[i].as_u64().ok_or("positive index")? as usize;
        let point = fb
            .points
            .get(index)
            .ok_or("positive index outside archive")?;
        if points[i] != planted_point_json(point) {
            return Err("positive point differs from archive".into());
        sum = kc.add(&sum, point);
        if partial[i] != planted_point_json(&sum) {
            return Err("positive partial sum differs".into());
        selected.push(index);
    if selected.iter().collect::<HashSet<_>>().len() != 6 || sum != *target {
        return Err("positive witness is repeated or misses target".into());
    let row = checked_primary_relation_row(kc, fb, &selected, target)?;
    let row_bytes = serde_json::to_vec(&row.iter().map(ToString::to_string).collect::<Vec<_>>())?;
    if positive["relation_row_decimal_blake3"] != blake3::hash(&row_bytes).to_hex().to_string() {
        return Err("positive relation-row hash differs".into());
    Ok(selected)

/// Independently replay a solver model against the saved formula and the
/// exact retained point base. This command does not perform SAT search.
pub(super) fn k600_m6_replay_cli(
    export_path: &Path,
    xcnf_path: &Path,
    receipt_path: &Path,
    stdout_path: &Path,
    source_commit: &str,
    exporter_binary_sha256: &str,
    output: &Path,
) -> Result<Value> {
    const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f0731247f9ee3759dcbd";
        return Err("K600 replay output already exists".into());
        (
            "manifest.json",
            "5bd862987844f709e03beeab9fd265e3d2024011bcadcb82f5af9c72018df6a6",
        ),
            "replay.json",
            "fa5e545c27dd64c32a47ddae05a2a0b3a031545d700897f3e0e34f59abc0d72e",
            "upload-receipt.json",
            "0eae40ffa0e938470671cf796633c1e47b61ff676df0807e7a7e8592e56bb828",
            "probe-validation.json",
            "1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f",
            return Err(format!("K600 replay input {name} differs from frozen SHA-256").into());
    let export: Value = serde_json::from_slice(&fs::read(export_path)?)?;
    let receipt: Value = serde_json::from_slice(&fs::read(receipt_path)?)?;
    let mode = export["mode"].as_str().ok_or("K600 export mode missing")?;
    if export["schema"] != "n83.k600-m6-xcnf-export/v1"
        || export["status"] != "PASS_XCNF_EXPORT_ONLY"
        || !["positive-pinned", "positive-unpinned", "natural"].contains(&mode)
        || export["curve_id"] != "icv1-f2m83-tm6151469093347-debefd74"
        || export["curve_a"] != 0
        || export["source_commit"] != source_commit
        || export["source_attestation_mode"] != "clean_checkout"
        || export["binary_sha256"] != exporter_binary_sha256
        || export["object"] != OBJECT
        || export["object_sha256"] != OBJECT_SHA256
        || receipt["exit_code"] != 10
        || !["SAT_PENDING_REPLAY", "SAT_PENDING_XCNF_REPLAY"]
            .any(|s| receipt["status"] == *s)
        || receipt["xcnf_sha256"] != export["xcnf_sha256"]
        return Err("K600 export, source, or SAT receipt binding differs".into());
    let shape = &export["shape"];
    if shape["summands"] != 6
        || shape["identity_mask"] != 0
        || shape["root_encoding"] != "per_representative"
        || shape["domain_representatives"] != 600
        || shape["summand_ordering"] != "none"
        || shape["positive_x_pins"] != if mode == "positive-pinned" { 913 } else { 0 }
        return Err("K600 XCNF search policy differs".into());
    let variables = usize::try_from(shape["variables"].as_u64().ok_or("variable count")?)?;
    let clauses = usize::try_from(shape["clauses"].as_u64().ok_or("clause count")?)?;
    let xors = usize::try_from(shape["xor_rows"].as_u64().ok_or("XOR count")?)?;
    if variables < 913 || variables > 125_000 {
        return Err("K600 XCNF variable count exceeds frozen bound".into());
    let xcnf = fs::read(xcnf_path)?;
    if xcnf.len() > 200_000_000
        || export["xcnf_bytes"].as_u64() != Some(xcnf.len() as u64)
        || export["xcnf_sha256"] != sha256_hex(&xcnf)
        return Err("original XCNF bytes differ".into());
    let stdout = fs::read(stdout_path)?;
    if stdout.len() > 20_000_000 || receipt["stdout_sha256"] != sha256_hex(&stdout) {
        return Err("solver output bytes differ".into());
    let values = k600_model_values(std::str::from_utf8(&stdout)?, variables)?;
    k600_check_xcnf(std::str::from_utf8(&xcnf)?, &values, clauses, xors)?;
    if sha256_hex(&fs::read(root.join(OBJECT))?) != OBJECT_SHA256 {
        return Err("retained K600 factor-base bytes differ".into());
        || row["point_set_blake3"] != export["point_set_blake3"]
        || export["panel_manifest_blake3"] != manifest_blake3
        return Err("retained K600 factor-base identity differs".into());
    let (_, corpus_blake3) = public_target_fixture(root, &base.curve, 0)?;
    if export["public_corpus_blake3"] != corpus_blake3 {
        return Err("K600 public corpus binding differs".into());
    let target = general(Some(
        parse_point(&export["target"])?.ok_or("identity target")?,
    ));
    if !base.curve.curve.is_on_curve(&target)
        || base.curve.mul(&target, &base.curve.subgroup_order) != BinaryPoint::Infinity
        return Err("K600 target fails curve or subgroup check".into());
    if mode == "natural" {
        if !export["positive"].is_null()
            || export["public_fixture"] != 0
            || sha256_hex(&fs::read(root.join("probe-corpus.json"))?) != CORPUS_SHA256
            return Err("natural public fixture identity differs".into());
        let (public, _) = public_target_fixture(root, &base.curve, 0)?;
        if target != public {
            return Err("natural target differs from public fixture".into());
        verify_k600_positive_witness(&base.curve, fb, &target, &export["positive"])?;
    let xs: Vec<_> = (0..6).map(|block| k600_model_x(&values, block)).collect();
    let intermediate: Vec<_> = (6..10).map(|block| k600_model_x(&values, block)).collect();
    if k600_model_x(&values, 10)
        != match &target {
            BinaryPoint::Affine { x, .. } => x.to_biguint(),
            BinaryPoint::Infinity => unreachable!(),
        return Err("model target block differs".into());
    let candidates: Vec<Vec<usize>> = xs
        .map(|x| {
            fb.points
                .iter()
                .enumerate()
                .filter_map(|(i, p)| match p {
                    BinaryPoint::Affine { x: px, .. } if px.to_biguint() == *x => Some(i),
                    _ => None,
                })
                .collect()
    if candidates.iter().any(|c| c.is_empty() || c.len() > 2) {
        return Err("model x-coordinate has no bounded archive lift".into());
    let mut chosen = None;
    for mask in 0..64usize {
        let mut indices = Vec::with_capacity(6);
        let mut valid = true;
        for (j, lifts) in candidates.iter().enumerate() {
            let bit = (mask >> j) & 1;
            if bit >= lifts.len() {
                valid = false;
                break;
            let index = lifts[bit];
            indices.push(index);
            if (1..=4).contains(&j) {
                match &sum {
                    BinaryPoint::Affine { x, .. } if x.to_biguint() == intermediate[j - 1] => (),
                    _ => {
                        valid = false;
                        break;
                    }
        if valid && sum == target {
            chosen = Some(indices);
            break;
    let chosen = chosen.ok_or("SAT model has no checked six-point relation lift")?;
    let relation = checked_primary_relation_row(&base.curve, fb, &chosen, &target)?;
    let chosen_points: Vec<_> = chosen
        .map(|&i| point_json(words(&fb.points[i])))
    let columns: Vec<_> = chosen.iter().map(|&i| fb.signed_orbit_of[i].0).collect();
        "schema":"n83.k600-m6-model-replay/v2", "status":"PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY",
        "mode":mode,"source_commit":source_commit,"export_binary_sha256":exporter_binary_sha256,
        "replay_binary_sha256":sha256_hex(&fs::read(std::env::current_exe()?)?),
        "xcnf_sha256":export["xcnf_sha256"],"solver_stdout_sha256":receipt["stdout_sha256"],
        "object_sha256":OBJECT_SHA256,"model_values_sha256":sha256_hex(&values),
        "checked_cnf_clauses":clauses,"checked_xor_rows":xors,"target":export["target"],
        "x_coordinates":xs.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "intermediate_x_coordinates":intermediate.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "chosen_indices":chosen,"chosen_points":chosen_points,"columns":columns,
        "relation_row_decimal":relation.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "subgroup_order":base.curve.subgroup_order.to_string(),

pub(super) fn check_full_d18_export(root: &Path) -> Result<Value> {
    let (base, row) = full_d18_load(root)?;
    Ok(json!({
        "schema":"n83.primary-full-d18-adapter/v1", "status":"PASS",
        "curve_id":"icv1-f2m83-tm6151469093347-debefd74",
        "source_commit":source_commit,
        "source_attestation_mode":source_attestation_mode,
        "binary_sha256":binary_sha256,
        "policy":row["policy"], "object":row["object"],
        "orbit_columns":base.factor_base.signed_orbits.len(),
        "points":base.factor_base.points.len(),
        "frobenius_orbits":base.factor_base.orbits.len(),
        "import_elapsed_ms_informational_L0":started.elapsed().as_secs_f64()*1000.0,
        "solver_stage_executed":false,
    }))

fn full_d18_four_x_lift(
    xs: &[F2mElement],
) -> Result<Option<([usize; 4], Vec<BigUint>)>> {
    if xs.len() != 4 {
        return Err("full D18 SAT model has the wrong summand count".into());
    let mut choices: [Vec<usize>; 4] = std::array::from_fn(|_| Vec::new());
    for (index, point) in fb.points.iter().enumerate() {
        if let BinaryPoint::Affine { x, .. } = point {
            for (slot, wanted) in xs.iter().enumerate() {
                if x == wanted {
                    choices[slot].push(index);
                    if choices[slot].len() > 2 {
                        return Err("full D18 x-coordinate has more than two stored lifts".into());
    if choices.iter().any(Vec::is_empty) {
        return Ok(None);
    for &a in &choices[0] {
        for &b in &choices[1] {
            for &c in &choices[2] {
                for &d in &choices[3] {
                    let indices = [a, b, c, d];
                    let sum = indices.iter().fold(BinaryPoint::Infinity, |acc, &index| {
                        kc.add(&acc, &fb.points[index])
                    });
                    if sum == *target {
                        return Ok(Some((
                            indices,
                            checked_primary_relation_row(kc, fb, &indices, target)?,
                        )));
    Ok(None)

/// Construct the exact first natural-query circuit from the archived full
/// base without starting a SAT search. This L0 capacity diagnostic has no
/// cgroup claim or relation/rank inference.
pub(super) fn full_d18_m4_model_cli(
    fixture_root: &Path,
    full_d18_m4_model_impl(
        fixture_root,
        FrobeniusOrbitRootEncoding::PerRepresentative,

pub(super) fn full_d18_m4_shared_model_cli(

fn full_d18_m4_model_impl(
    encoding: FrobeniusOrbitRootEncoding,
        return Err("full D18 model output already exists".into());
    let import_ms = started.elapsed().as_secs_f64() * 1000.0;
    if sha256_hex(&fs::read(fixture_root.join("probe-corpus.json"))?)
        != FULL_D18_PUBLIC_CORPUS_SHA256
        return Err("full D18 model public corpus differs".into());
    let (target, corpus_hash) = public_target(fixture_root, &base.curve)?;
    if point_json(words(&target))
        != json!(["1834913121178197242856238", "8797964190732846213981377"])
        return Err("full D18 model target differs".into());
        return Err("full D18 model target is identity".into());
    let representatives: Vec<BigUint> = base
        .factor_base
        .map(|orbit| match &base.factor_base.points[orbit[0]] {
            BinaryPoint::Infinity => Err("full D18 model representative is infinity".into()),
    let size = frobenius_orbit_domain_size_with_encoding(83, 4, &representatives, encoding)
        .ok_or("full D18 model domain size overflow")?;
    let required_variables = chain_s3_max_variables(83, 4)
        .ok_or("full D18 model chain size overflow")?
        .checked_add(size.variables)
        .ok_or("full D18 model total size overflow")?;
    let expected_clauses = match encoding {
        FrobeniusOrbitRootEncoding::PerRepresentative => 11_238_656,
        FrobeniusOrbitRootEncoding::SharedBits => 2_621_168,
    if required_variables != 329_726 || size.clauses != expected_clauses {
        return Err("full D18 model preflight differs from frozen protocol".into());
    let encoding_name = match encoding {
        FrobeniusOrbitRootEncoding::PerRepresentative => "per_representative",
        FrobeniusOrbitRootEncoding::SharedBits => "shared_root_bits",
    let preflight = json!({
        "schema":"n83.full-d18-m4-model-preflight/v1", "status":"PASS_INPUTS",
        "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
        "binary_sha256":binary_sha256, "object":row["object"],
        "public_corpus_blake3":corpus_hash, "target":point_json(words(&target)),
        "orbit_columns":representatives.len(),
        "required_max_variables":required_variables,
        "required_domain_clauses":size.clauses,
        "root_encoding":encoding_name,
        "import_ms":import_ms,
        "isolation":"L0_shared_host_no_hard_cgroup",
        "solver_search_executed":false,
    let preflight_path = std::path::PathBuf::from(format!("{}.preflight.json", output.display()));
    write_new_json(&preflight_path, &preflight)?;
    let model_start = Instant::now();
        4,
        400_000,
    .map_err(|_| "full D18 model circuit construction failed")?;
        encoding,
        12_000_000,
    .map_err(|_| "full D18 model domain installation failed")?;
        "schema":"n83.full-d18-m4-model/v1", "status":"PASS_MODEL_CONSTRUCTION_ONLY",
        "preflight":preflight,
        "sat_variables":enc.solver.n_vars(),
        "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(),
        "sat_and_gates":enc.n_and_gates,
        "sat_s3_nodes":enc.n_s3_nodes,
        "model_ms":model_start.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(),

/// One frozen four-summand D18 access gate. The supervisor owns an
/// independent wall cap; this worker requires its hard 32-GiB, zero-swap
/// cgroup before importing or constructing the full factor base.
pub(super) fn full_d18_m4_cli(
    mode: &str,
    full_d18_m4_impl(
        false,
        "none",
        1_200,
        1,

/// Local, positive-control-only SAT diagnostic. This shares the frozen model
/// and verifier, but makes no hard-cgroup or capacity-worker claim.
pub(super) fn full_d18_m4_l0_positive_cli(
        true,
        300,

pub(super) fn full_d18_m4_l0_shared_positive_cli(

pub(super) fn full_d18_m4_l0_shared_root_pinned_cli(
        "selector_and_phase_zero",

pub(super) fn full_d18_m4_l0_shared_phase_pinned_cli(
        "phase_zero",

pub(super) fn full_d18_m4_l0_shared_selector_pinned_cli(
        "selector",

/// Search the unchanged free-choice planted formula with Frobenius phase
/// wires prioritized and initially false. No selector or phase is pinned.
pub(super) fn full_d18_m4_l0_shared_phase_first_cli(

/// Bounded local screening of the frozen public target, using the exact
/// phase-priority policy that passed the free-choice planted control.
pub(super) fn full_d18_m4_l0_shared_phase_first_natural_cli(

/// Longer local screen of the same public target and free-choice model.
/// It stays an L0 shared-host diagnostic, with no hard-memory claim.
pub(super) fn full_d18_m4_l0_shared_phase_first_natural_1200_cli(

/// Export the same full-D18 shared-root formula as CryptoMiniSat XCNF.
/// This source-bound arm performs no native relation search.
pub(super) fn full_d18_m4_l0_xcnf_cli(

/// Export the ordered-first-pair D18 formula used by the native cadence-32
/// control. The cadence affects native search only; no solver runs here.
pub(super) fn full_d18_m4_l0_ordered_xcnf_cli(
        32,

/// Export the same ordered D18 construction for one sealed public fixture.
pub(super) fn full_d18_m4_l0_ordered_xcnf_fixture_cli(
    fixture: u64,
    full_d18_m4_impl_fixture(
        fixture,

/// Export the unchanged constructed full-D18 XCNF and save one native SAT
/// assignment so an independent verifier can test every exported constraint.
pub(super) fn full_d18_m4_l0_xcnf_witness_cli(

/// Source-bound Linux access gate for the free-choice shared-root circuit.
/// The worker requires an exact 8-GiB zero-swap cgroup before importing S3.
pub(super) fn full_d18_m4_linux_shared_phase_first_cli(

/// Exact S3 first-pair symmetry arm. The first two summands share one domain
/// and their S3 node is exchangeable; lex ordering removes one duplicate
/// orientation without pinning an orbit selector or Frobenius phase.
pub(super) fn full_d18_m4_l0_ordered_first_pair_cli(
    let budget = if mode == "natural" { 2_400 } else { 300 };
        budget,

/// Keep the exact first-pair symmetry arm and change only the cadence of
/// full native XOR elimination after search decisions. Every complete
/// assignment still receives the full XOR check before SAT is returned.
pub(super) fn full_d18_m4_l0_ordered_xor_cadence_32_cli(

fn full_d18_m4_impl(
    l0_diagnostic: bool,
    choice_pins: &'static str,
    phase_zero_first: bool,
    budget_seconds: u64,
    export_xcnf: bool,
    native_xcnf_witness: bool,
    ordered_first_pair: bool,
    xor_decision_cadence: usize,
        l0_diagnostic,
        choice_pins,
        phase_zero_first,
        budget_seconds,
        export_xcnf,
        native_xcnf_witness,
        ordered_first_pair,
        xor_decision_cadence,

fn full_d18_m4_impl_fixture(
    public_fixture: u64,
    if public_fixture != 0 && (mode != "natural" || !export_xcnf || !ordered_first_pair) {
        return Err("nonzero public fixture requires ordered natural XCNF export".into());
    if !["positive", "natural"].contains(&mode)
        || ((native_xcnf_witness || (ordered_first_pair && xor_decision_cadence == 32))
            && output.with_extension("model.bin").exists())
        return Err("full D18 m4 mode or fresh output differs from protocol".into());
    if l0_diagnostic && mode != "positive" && !phase_zero_first {
        return Err("L0 natural screening requires the phase-first policy".into());
    if !["none", "phase_zero", "selector", "selector_and_phase_zero"].contains(&choice_pins) {
        return Err("unsupported planted choice-pin policy".into());
    if choice_pins != "none"
        && (!l0_diagnostic || encoding != FrobeniusOrbitRootEncoding::SharedBits)
        return Err("root-choice pins require the L0 shared-root planted control".into());
    if phase_zero_first
        && (encoding != FrobeniusOrbitRootEncoding::SharedBits || choice_pins != "none")
        return Err("phase-first policy requires the free-choice shared-root mode".into());
    if ordered_first_pair
        && (!l0_diagnostic
            || encoding != FrobeniusOrbitRootEncoding::SharedBits
            || !phase_zero_first
            || choice_pins != "none"
            || native_xcnf_witness
            || (export_xcnf && xor_decision_cadence != 32))
        return Err("ordered first pair differs from the frozen symmetry arm".into());
    if xor_decision_cadence != 1 && !(ordered_first_pair && xor_decision_cadence == 32) {
        return Err("XOR cadence differs from the frozen native solver arm".into());
    if (!l0_diagnostic && budget_seconds != 1_200)
        || (l0_diagnostic
            && budget_seconds != 300
            && !(budget_seconds == 1_200
                && mode == "natural"
                && encoding == FrobeniusOrbitRootEncoding::SharedBits
                && phase_zero_first)
            && !(budget_seconds == 2_400 && mode == "natural" && ordered_first_pair))
        return Err("full D18 m4 wall budget differs from a frozen mode".into());
    if export_xcnf
            || budget_seconds != 300
            || output.with_extension("xcnf").exists())
        return Err("full D18 XCNF export differs from the frozen mode".into());
    if native_xcnf_witness && (!export_xcnf || mode != "positive") {
        return Err("full D18 native XCNF witness differs from the frozen mode".into());
    let root_choice_pinned = choice_pins == "selector_and_phase_zero";
    let search_policy = if ordered_first_pair && xor_decision_cadence == 32 {
        "phase_zero_first_ordered_first_pair_xor_cadence32"
    } else if ordered_first_pair {
        "phase_zero_first_ordered_first_pair"
    } else if phase_zero_first {
        "phase_zero_first"
        "default"
    let hard_memory_mib = if phase_zero_first { 8_192 } else { 32_768 };
    let isolation = if l0_diagnostic {
        "L0_shared_host_no_hard_cgroup"
        "hard_8GiB_cgroup_zero_swap"
        "hard_32GiB_cgroup_zero_swap"
    let memory_limit_bytes = if l0_diagnostic {
        Some(checked_cgroup_memory_limit(hard_memory_mib)?)
    let phase_path = std::path::PathBuf::from(format!("{}.phases.jsonl", output.display()));
    let mut phases = OpenOptions::new()
        .open(&phase_path)?;
    full_d18_phase(
        &mut phases,
        started,
        "started",
        json!({"mode":mode, "root_encoding":encoding_name, "root_choice_pinned":root_choice_pinned, "choice_pin_policy":choice_pins, "search_policy":search_policy, "source_commit":source_commit, "binary_sha256":binary_sha256}),
    let cap_path = std::path::PathBuf::from(format!("{}.cap.json", output.display()));
    let cap_mode = mode.to_owned();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(budget_seconds));
        let cap = json!({
            "schema":"n83.full-d18-m4-cap/v1", "status":"UNKNOWN_wall_cap",
            "mode":cap_mode, "root_encoding":encoding_name, "root_choice_pinned":root_choice_pinned, "choice_pin_policy":choice_pins, "search_policy":search_policy, "budget_seconds":budget_seconds,
            "isolation":isolation,
            "selected_best_total_runtime":Value::Null,
        });
        if write_new_json(&cap_path, &cap).is_err() {
            std::process::exit(125);
        std::process::exit(124);
    let import_start = Instant::now();
    let import_ms = import_start.elapsed().as_secs_f64() * 1000.0;
        "import_checked",
        json!({"import_ms":import_ms, "object":row["object"],
            "point_set_blake3":FULL_D18_POINT_SET}),
        return Err("full D18 public corpus differs from frozen fixture".into());
    let (public, corpus_hash) = public_target_fixture(fixture_root, &base.curve, public_fixture)?;
    if public_fixture == 0
        return Err("full D18 public target point differs".into());
            BinaryPoint::Infinity => Err("full D18 representative is infinity".into()),
    let mut planted = None;
    let target = if mode == "positive" {
        let indices: [usize; 4] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
        let mut partial = Vec::with_capacity(4);
            return Err("full D18 positive control has an identity intermediate".into());
        let row = checked_primary_relation_row(&base.curve, fb, &indices, &sum)?;
        let changed = base.curve.add(&sum, base.curve.generator());
        if checked_primary_relation_row(&base.curve, fb, &indices, &changed).is_ok() {
            return Err("full D18 changed-target control passed unexpectedly".into());
        planted = Some((indices, partial, row));
        public.clone()
        return Err("full D18 target is identity".into());
    .map_err(|_| "full D18 m4 circuit construction failed")?;
    .map_err(|_| "full D18 m4 domain installation failed")?;
    if ordered_first_pair {
        enc.order_first_pair(400_000)
            .map_err(|_| "full D18 first-pair ordering failed")?;
    if choice_pins != "none" {
        let (indices, _, _) = planted
            .as_ref()
            .ok_or("root-pinned control has no witness")?;
        let handles = enc
            .orbit_choice_wires()
            .ok_or("root-pinned control lacks choice wires")?
            .to_vec();
        if handles.len() != 4 {
            return Err("root-pinned control choice blocks differ".into());
        let mut sorted_representatives = representatives.clone();
        sorted_representatives.sort_unstable();
        for (block, &index) in indices.iter().enumerate() {
            let BinaryPoint::Affine { x, .. } = &fb.points[index] else {
                return Err("root-pinned control point is infinity".into());
            let column = sorted_representatives
                .binary_search(&x.to_biguint())
                .map_err(|_| "root-pinned control representative missing")?;
            if fb.signed_orbit_of[index].1 != 0 {
                return Err("root-pinned control representative has nonzero phase".into());
            if ["selector", "selector_and_phase_zero"].contains(&choice_pins) {
                    .add_clause(vec![handles[block].selectors[column] as i32])
                    return Err("root-pinned control selector contradicted model".into());
            if ["phase_zero", "selector_and_phase_zero"].contains(&choice_pins) {
                for &wire in &handles[block].phase {
                    if !enc.solver.add_clause(vec![-(wire as i32)]) {
                        return Err("root-pinned control phase contradicted model".into());
    if let Some((indices, partial, _)) = &planted {
        // The documented chained-S3 layout allocates four summand blocks,
        // two intermediate blocks, then the target, each of 83 x bits.
        let points: Vec<_> = indices
            .chain(partial[1..3].iter())
                return Err("full D18 positive pin is identity".into());
                    return Err("full D18 positive pin contradicted the model".into());
    if phase_zero_first {
        let phase_wires: Vec<u32> = enc
            .ok_or("phase-first control lacks choice wires")?
            .flat_map(|handles| handles.phase.iter().copied())
        if phase_wires.len() != 28 {
            return Err("phase-first control phase-wire count differs".into());
        let mut priority: Vec<u32> = (1..=(4 * 83)).collect();
        priority.extend_from_slice(&phase_wires);
        enc.solver.set_branch_priority(&priority);
        enc.solver.set_saved_phase_for_vars(&phase_wires, false);
    enc.solver.set_xor_decision_cadence(xor_decision_cadence);
    let model_ms = model_start.elapsed().as_secs_f64() * 1000.0;
        "root_encoding":encoding_name, "root_choice_pinned":root_choice_pinned, "choice_pin_policy":choice_pins, "search_policy":search_policy,
        "ordered_first_pair":ordered_first_pair,
        "xor_decision_cadence":xor_decision_cadence,
        "import_ms":import_ms, "object":row["object"],
    let shape_path = std::path::PathBuf::from(format!("{}.shape.json", output.display()));
    write_new_json(&shape_path, &shape)?;
    full_d18_phase(&mut phases, started, "model_constructed", shape.clone())?;
    if export_xcnf {
        let export_start = Instant::now();
        let xcnf = to_dimacs_xor(&enc.solver);
        if xcnf.len() > 400_000_000 {
            return Err("full D18 XCNF exceeds the frozen output cap".into());
        let xcnf_path = output.with_extension("xcnf");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&xcnf_path)?;
        file.write_all(xcnf.as_bytes())?;
        file.sync_all()?;
        let xcnf_sha256 = sha256_hex(xcnf.as_bytes());
        let export_ms = export_start.elapsed().as_secs_f64() * 1000.0;
        full_d18_phase(
            &mut phases,
            started,
            "xcnf_exported",
            json!({"xcnf_sha256":xcnf_sha256,"xcnf_bytes":xcnf.len(),"export_ms":export_ms}),
        )?;
        if native_xcnf_witness {
            enc.solver.conflict_budget = 100_000;
            enc.solver.deadline = Some(started + Duration::from_secs(budget_seconds));
            let search_start = Instant::now();
            let answer = enc.solver.solve();
            let search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
            full_d18_phase(
                &mut phases,
                started,
                "solver_returned",
                json!({"answer":format!("{answer:?}"),"search_ms":search_ms,
                    "conflicts":enc.solver.conflicts()}),
            )?;
            if answer != SolveResult::Sat {
                return Err("constructed full D18 XCNF witness did not return SAT".into());
            let model = enc.solver.model();
            if !enc.verify_external_assignment(&model) {
                return Err("native XCNF witness fails original clauses, XOR or field".into());
            let xs = enc
                .decode_summands_assignment(&model)
                .ok_or("native XCNF witness summands missing")?;
            let (indices, relation_row) = full_d18_four_x_lift(&base.curve, fb, &xs, &target)?
                .ok_or("native XCNF witness lacks a checked group lift")?;
            let (planted_indices, _, planted_row) = planted
                .as_ref()
                .ok_or("native XCNF witness lacks constructed relation")?;
            if &indices != planted_indices || &relation_row != planted_row {
                return Err("native XCNF witness differs from constructed relation".into());
            let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
            if rank.insert(relation_row.clone()) != 1 {
                return Err("native XCNF witness row failed rank-one insertion".into());
            let changed = base.curve.add(&target, base.curve.generator());
            if checked_primary_relation_row(&base.curve, fb, &indices, &changed).is_ok() {
                return Err("native XCNF witness accepted altered target".into());
            let packed = pack_model_bits(&model);
            let model_path = output.with_extension("model.bin");
            let mut model_file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&model_path)?;
            model_file.write_all(&packed)?;
            model_file.sync_all()?;
            let model_sha256 = sha256_hex(&packed);
                "native_model_verified",
                json!({"model_sha256":model_sha256,"model_bits":model.len(),
                    "indices":indices,"rank_after_insert":1}),
            let result = json!({
                "schema":"n83.full-d18-m4-l0-xcnf-native-witness/v1",
                "status":"PASS_NATIVE_XCNF_WITNESS", "mode":mode,
                "source_commit":source_commit,
                "source_attestation_mode":source_attestation_mode,
                "binary_sha256":binary_sha256,
                "isolation":isolation, "budget_seconds":budget_seconds,
                "curve_a":0, "summands":4, "identity_mask":0,
                "root_encoding":encoding_name,"choice_pin_policy":choice_pins,
                "search_policy":search_policy,"target":point_json(words(&target)),
                "object":row["object"],"point_set_blake3":FULL_D18_POINT_SET,
                "public_corpus_blake3":corpus_hash,
                "shape":shape,"xcnf_sha256":xcnf_sha256,"xcnf_bytes":xcnf.len(),
                "xcnf_path":xcnf_path.to_string_lossy(),"export_ms":export_ms,
                "model_encoding":"bit i of byte i/8 is variable i+1, low bit first",
                "model_bits":model.len(),"model_bytes":packed.len(),
                "model_sha256":model_sha256,
                "model_path":model_path.to_string_lossy(),
                "solver_answer":"Sat","solver_search_executed":true,
                "solver_conflicts":enc.solver.conflicts(),"search_ms":search_ms,
                "relation_indices":indices,
                "relation_row_decimal_blake3":blake3::hash(&serde_json::to_vec(
                    &relation_row.iter().map(ToString::to_string).collect::<Vec<_>>())?)
                    .to_hex().to_string(),
                "rank_after_insert":1,"altered_target_rejected":true,
                "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
                "process_usage":process_usage(),
                "total_index_calculus_runtime_ms":Value::Null,
                "selected_best_total_runtime":Value::Null,
            write_new_json(output, &result)?;
            return Ok(result);
        let result = json!({
            "schema":"n83.full-d18-m4-l0-xcnf-export/v1",
            "status":"PASS_XCNF_EXPORT_ONLY", "mode":mode,
            "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
            "binary_sha256":binary_sha256, "isolation":isolation,
            "budget_seconds":budget_seconds, "curve_a":0,
            "summands":4, "identity_mask":0,
            "root_encoding":encoding_name, "choice_pin_policy":choice_pins,
            "search_policy":search_policy, "target":point_json(words(&target)),
            "public_fixture":public_fixture,
            "object":row["object"], "point_set_blake3":FULL_D18_POINT_SET,
            "shape":shape, "xcnf_sha256":xcnf_sha256,
            "xcnf_bytes":xcnf.len(), "xcnf_path":xcnf_path.to_string_lossy(),
            "export_ms":export_ms,
            "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "process_usage":process_usage(), "solver_search_executed":false,
            "total_index_calculus_runtime_ms":Value::Null,
        write_new_json(output, &result)?;
        return Ok(result);
    enc.solver.conflict_budget = 100_000;
    enc.solver.deadline = Some(started + Duration::from_secs(budget_seconds));
    let search_start = Instant::now();
    let answer = enc.solver.solve();
    let search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
    let limit_reason = if answer != SolveResult::Unknown {
        "not_applicable"
        let conflict_cap = enc.solver.conflicts() >= enc.solver.conflict_budget;
        let wall_cap = started.elapsed() >= Duration::from_secs(budget_seconds);
        match (conflict_cap, wall_cap) {
            (true, true) => "conflict_and_wall_cap",
            (true, false) => "conflict_cap",
            (false, true) => "wall_cap",
            (false, false) => "unknown_internal_limit",
        "solver_returned",
        json!({"answer":format!("{answer:?}"), "search_ms":search_ms,
            "conflicts":enc.solver.conflicts(),"limit_reason":limit_reason}),
    let mut relation = Value::Null;
    let status = match answer {
        SolveResult::Unknown => "UNKNOWN_search_cap",
        SolveResult::Unsat if mode == "positive" => "INVALID_POSITIVE_CONTROL",
        SolveResult::Unsat => "UNVERIFIED_UNSAT",
        SolveResult::Sat => {
            let xs = enc.decode_summands();
            if !enc.verify_external_assignment(&enc.solver.model()) {
                "INVALID_MODEL"
            } else if let Some((indices, row)) =
                full_d18_four_x_lift(&base.curve, fb, &xs, &target)?
            {
                let mut rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
                let rank_after_insert = rank.insert(row.clone());
                let changed = base.curve.add(&target, base.curve.generator());
                let tamper_rejected =
                    checked_primary_relation_row(&base.curve, fb, &indices, &changed).is_err();
                relation = json!({
                    "indices":indices,
                    "row_decimal_blake3":blake3::hash(&serde_json::to_vec(
                        &row.iter().map(ToString::to_string).collect::<Vec<_>>())?)
                        .to_hex().to_string(),
                    "rank_after_insert":rank_after_insert,
                    "tamper_rejected":tamper_rejected,
                    "matches_planted_row":planted.as_ref().map(|(_, _, expected)| expected == &row),
                });
                if rank_after_insert != 1 || !tamper_rejected {
                    "INVALID_RELATION"
                } else if mode == "positive"
                    && planted
                        .as_ref()
                        .is_some_and(|(_, _, expected)| expected != &row)
                    "INVALID_POSITIVE_CONTROL"
                } else if mode == "positive" {
                    "PASS_CONSTRUCTED_CONTROL"
                    "PENDING_INDEPENDENT_NATURAL_REPLAY"
                "MODEL_GROUP_REJECTED"
    let mut model_certificate = Value::Null;
    if answer == SolveResult::Sat && ordered_first_pair && xor_decision_cadence == 32 {
        let model = enc.solver.model();
        if model.len() != enc.solver.n_vars() as usize {
            return Err("full D18 SAT assignment length differs from formula".into());
        let packed = pack_model_bits(&model);
        let model_path = output.with_extension("model.bin");
        let mut model_file = OpenOptions::new()
            .open(&model_path)?;
        model_file.write_all(&packed)?;
        model_file.sync_all()?;
        model_certificate = json!({
            "schema":"n83.full-d18-m4-model-certificate/v1",
            "encoding":"bit i of byte i/8 is variable i+1, low bit first",
            "bits":model.len(),"bytes":packed.len(),
            "sha256":sha256_hex(&packed),
            "path":model_path.to_string_lossy(),
    let peak_bytes: Option<u64> = if l0_diagnostic {
        Some(
            fs::read_to_string("/sys/fs/cgroup/memory.peak")?
                .trim()
                .parse()?,
    let solver_stats = json!({
        "decisions":enc.solver.stats.decisions,
        "conflicts":enc.solver.stats.conflicts,
        "restarts":enc.solver.stats.restarts,
        "propagations":enc.solver.stats.propagations,
        "xor_passes":enc.solver.stats.xor_passes,
        "xor_propagations":enc.solver.stats.xor_propagations,
        "xor_conflicts":enc.solver.stats.xor_conflicts,
        "xor_repivots":enc.solver.stats.xor_repivots,
        "xor_row_ops":enc.solver.stats.xor_row_ops,
        "xor_row_scans":enc.solver.stats.xor_row_scans,
        "clause_visits":enc.solver.stats.clause_visits,
        "clause_lit_visits":enc.solver.stats.clause_lit_visits,
        "ns_propagate_clauses":enc.solver.stats.ns_propagate_clauses,
        "ns_propagate_xors":enc.solver.stats.ns_propagate_xors,
        "ns_analyze":enc.solver.stats.ns_analyze,
        "schema":if l0_diagnostic {if mode == "positive" {"n83.full-d18-m4-l0-positive/v1"} else {"n83.full-d18-m4-l0-natural/v1"}} else {"n83.full-d18-m4-worker/v1"}, "status":status,
        "mode":mode, "source_commit":source_commit,
        "isolation":isolation,
        "budget_seconds":budget_seconds,
        "phase_file":phase_path.to_string_lossy(),
        "curve_a":0, "summands":4, "identity_mask":0,
        "object":row["object"], "point_set_blake3":FULL_D18_POINT_SET,
        "public_corpus_blake3":corpus_hash,
        "target":point_json(words(&target)),
        "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_peak_bytes":peak_bytes,
        "shape":shape, "search_ms":search_ms,
        "solver_answer":format!("{answer:?}"),
        "solver_limit_reason":limit_reason,
        "solver_conflicts":enc.solver.conflicts(),
        "solver_stats":solver_stats,
        "model_certificate":model_certificate,
        "relation":relation,

pub(super) fn check_panel(root: &Path, columns: usize) -> Result<Value> {
    let (base, row, _) = load_panel(root, columns)?;
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

/// Convert a checked four-summand witness into exact signed-orbit
/// coefficients. The retained primary base consists entirely of order-r
/// points, so replaying the row against its representatives must reproduce
/// the original target without a cofactor projection.
fn checked_primary_relation_row(
    indices: &[usize],
) -> Result<Vec<BigUint>> {
    let r = &kc.subgroup_order;
    if r <= &BigUint::one() || fb.signed_orbit_of.len() != fb.points.len() {
        return Err("invalid primary relation domain".into());
    let mut row = vec![BigUint::zero(); fb.unknowns()];
    let mut point_sum = BinaryPoint::Infinity;
    for &index in indices {
            .ok_or("relation point index out of range")?;
        let &(column, phase, negative) = fb
            .signed_orbit_of
            .ok_or("relation point label absent")?;
        let rep_index = *fb
            .signed_orbits
            .get(column)
            .and_then(|orbit| orbit.first())
            .ok_or("relation orbit representative absent")?;
        let representative = fb
            .get(rep_index)
            .ok_or("relation representative index out of range")?;
        if phase >= kc.n || kc.mul(representative, r) != BinaryPoint::Infinity {
            return Err("relation representative outside the primary subgroup".into());
        let positive = kc.lambda.modpow(&BigUint::from(phase), r);
        let coefficient = if negative && !positive.is_zero() {
            r - &positive
            positive
        if kc.mul(representative, &coefficient) != *point {
            return Err("relation point and signed-Frobenius label disagree".into());
        row[column] = (&row[column] + coefficient) % r;
        point_sum = kc.add(&point_sum, point);
    if point_sum != *target {
        return Err("relation point sum differs from target".into());
    let mut row_sum = BinaryPoint::Infinity;
    for (column, coefficient) in row.iter().enumerate() {
        if coefficient.is_zero() {
        let rep_index = fb.signed_orbits[column][0];
        row_sum = kc.add(&row_sum, &kc.mul(&fb.points[rep_index], coefficient));
    if row_sum != *target {
        return Err("relation row failed independent group replay".into());
    Ok(row)

/// The container sees only a supervisor-made source snapshot, never the
/// checkout's Git metadata or unrelated files. The outer supervisor must
/// independently attest a clean checkout at this commit and retain the
/// snapshot hashes; this worker checks that the three files it executes
/// match its compiled bytes.
fn checked_primary_probe_source(root: &Path) -> Result<()> {
    for (path, compiled) in [
            "examples/koblitz_n83_factor_base_export.rs",
            include_bytes!("../../examples/koblitz_n83_factor_base_export.rs").as_slice(),
            "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
            include_bytes!("primary_adapter.rs").as_slice(),
            "research/koblitz_n83_factor_base_sweep_20261008/compact_cold.rs",
            include_bytes!("compact_cold.rs").as_slice(),
        if blake3::hash(&fs::read(root.join(path))?) != blake3::hash(compiled) {
            return Err("compiled S3 probe source differs from frozen source".into());

pub(super) fn frozen_primary_probe_source() -> Result<(String, &'static str)> {
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
            return Err("invalid supervised source attestation".into());
        checked_primary_probe_source(&root)?;
        Ok((commit.to_owned(), "supervised_snapshot"))
        let commit = frozen_v2_source_commit()?;
        checked_primary_probe_source(Path::new(env!("CARGO_MANIFEST_DIR")))?;
        Ok((commit, "clean_checkout"))

/// Pin the retained-base importer and both layers of the chained-S3 model.
/// The outer guard freezes these sources at a clean commit before launching
/// the Linux worker. The worker compares the snapshot to its compiled bytes.
fn frozen_chain_source() -> Result<(String, &'static str)> {
    let files: [(&str, &[u8]); 6] = [
            include_bytes!("../../examples/koblitz_n83_factor_base_export.rs"),
            include_bytes!("primary_adapter.rs"),
            "src/cryptanalysis/binary_semaev_chain_sat.rs",
            include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs"),
            "src/cryptanalysis/sat.rs",
            include_bytes!("../../src/cryptanalysis/sat.rs"),
            "src/cryptanalysis/koblitz_index_calculus.rs",
            include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs"),
            "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
            include_bytes!("size-frontier-v2.json"),
    ];
    let (root, commit, mode) = if let Some(path) = std::env::var_os("ICV1_FROZEN_SOURCE_DIR") {
            .ok_or("chain source snapshot lacks a commit")?;
        if attestation["schema"] != "n83.chain-s3-source-attestation/v1"
            return Err("invalid chained-S3 source attestation".into());
        (root, commit.to_owned(), "supervised_snapshot")
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            frozen_v2_source_commit()?,
            "clean_checkout",
    for (path, compiled) in files {
            return Err("compiled chained-S3 source differs from frozen source".into());
    Ok((commit, mode))

/// Construct one identity-pattern model from an exact replayed primary base.
/// This is a capacity diagnostic only: no SAT search or relation claim occurs.
/// A hard cgroup memory cap is mandatory here; the caller supplies the wall cap.
#[allow(clippy::too_many_arguments)]
pub(super) fn chain_capacity_cli(
    m: usize,
    identity_mask: u32,
    max_variables: u32,
    max_domain_clauses: usize,
    memory_mib: u64,
    frobenius_orbits: bool,
    order_first_pair: bool,
    if !([64, 256, 600].contains(&columns) || V2_SIZES.contains(&columns))
        || !(4..=6).contains(&m)
        || identity_mask >= (1u32 << (m - 2))
        || max_variables == 0
        || max_domain_clauses == 0
        || (order_first_pair && !frobenius_orbits)
        return Err("chained-S3 capacity parameters outside supported domain".into());
    let memory_limit_bytes = checked_cgroup_memory_limit(memory_mib)?;
    let start = Instant::now();
    let (base, row, manifest_hash) = load_panel_selected(root, columns, policy, seed)?;
    let base_import_ms = start.elapsed().as_secs_f64() * 1000.0;
    let (target, corpus_hash) = public_target(root, &base.curve)?;
    let target_validation_ms = start.elapsed().as_secs_f64() * 1000.0 - base_import_ms;
    let mut codes: Vec<BigUint> = base
        .subspace
        .map(F2mElement::to_biguint)
    codes.sort_unstable();
    codes.dedup();
    if codes.len() != 83 * columns {
        return Err(
            "primary coordinate domain does not contain 83 distinct x values per column".into(),
        );
    let representatives: Vec<BigUint> = if frobenius_orbits {
        base.factor_base
            .map(|orbit| {
                match orbit
                    .first()
                    .and_then(|&index| base.factor_base.points.get(index))
                    Some(BinaryPoint::Affine { x, .. }) => Ok(x.to_biguint()),
                    _ => Err("replayed primary orbit lacks an affine representative".into()),
            })
            .collect::<Result<_>>()?
        Vec::new()
    let orbit_size = if frobenius_orbits {
        if representatives.len() != columns
            || representatives.iter().collect::<HashSet<_>>().len() != columns
            return Err("primary orbit domain lacks distinct representatives".into());
            frobenius_orbit_domain_size(83, m, representatives.len())
                .ok_or("invalid Frobenius-orbit domain")?,
    let required_variables = chain_s3_max_variables(83, m)
        .ok_or("unsupported chained-S3 arity")?
        .checked_add(orbit_size.map_or(0, |size| size.variables))
        .and_then(|value| value.checked_add(if order_first_pair { 82 } else { 0 }))
        .ok_or("chained-S3 variable count overflow")?;
    let required_domain_clauses = if let Some(size) = orbit_size {
        size.clauses
        finite_domain_clause_count(&codes, 83, m).ok_or("invalid chained-S3 domain")?
    let mut enc = None;
    let status = if required_variables > u64::from(max_variables) {
        "UNKNOWN_variable_cap"
    } else if required_domain_clauses > max_domain_clauses {
        "UNKNOWN_domain_clause_cap"
        let target_x = match &target {
            BinaryPoint::Affine { x, .. } => Some(x),
            BinaryPoint::Infinity => None,
        let mut built = ChainedS3Encoding::build(
            m,
            &base.curve.curve.irreducible,
            &base.curve.curve.b,
            target_x,
            identity_mask,
            max_variables,
        .map_err(|_| "chained-S3 model build failed after preflight")?;
        if frobenius_orbits {
            built
                .constrain_frobenius_orbit_summands(
                    &representatives,
                    max_variables,
                    max_domain_clauses,
                )
                .map_err(|_| "chained-S3 orbit-domain install failed after preflight")?;
                .constrain_summands(&codes, max_domain_clauses)
                .map_err(|_| "chained-S3 finite-domain install failed after preflight")?;
        if order_first_pair {
                .order_first_pair(max_variables)
                .map_err(|_| "chained-S3 first-pair order failed after preflight")?;
        enc = Some(built);
        "PASS_model_construction_only"
    let model_construction_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    let cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
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
        "source_exporter_blake3":blake3::hash(include_bytes!("../../examples/koblitz_n83_factor_base_export.rs")).to_hex().to_string(),
        "source_adapter_blake3":blake3::hash(include_bytes!("primary_adapter.rs")).to_hex().to_string(),
        "source_chain_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs")).to_hex().to_string(),
        "source_index_calculus_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string()
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
    receipt
        .as_object_mut()
        .ok_or("invalid chained-S3 receipt")?
        .extend(
            measurements
                .as_object()
                .ok_or("invalid chained-S3 measurements")?
                .clone(),

fn planted_point_json(point: &BinaryPoint) -> Value {
    match point {
        BinaryPoint::Infinity => json!({"infinity": true}),
        BinaryPoint::Affine { x, y } => json!({
            "x_decimal": x.to_biguint().to_string(),
            "y_decimal": y.to_biguint().to_string(),
        }),

/// Find the at-most-two stored point lifts for each x coordinate without
/// allocating a second multi-million-point index. Check the complete group
/// equation before treating an x-coordinate model as a relation.
fn checked_four_x_lift(
        return Err("four-summand model has the wrong x count".into());
                        return Err("stored base has more than two lifts of an x coordinate".into());
                        let row = checked_primary_relation_row(kc, fb, &indices, target)?;
                        return Ok(Some((indices, row)));

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

impl WitnessPhases {
    fn create(
        output: &Path,
        start: Instant,
        source_commit: String,
        schema: &'static str,
    ) -> Result<Self> {
        if output.exists() {
            return Err("complete-witness output already exists".into());
        let mut path = output.as_os_str().to_os_string();
        path.push(".phases.jsonl");
        let path = PathBuf::from(path);
        let file = OpenOptions::new()
            .open(&path)?;
        Ok(Self {
            file,
            path,
            start,
            next_sequence: 0,
            source_commit,
            schema,

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
            "object_blake3":K16627_OBJECT_BLAKE3,
            "point_set_blake3":K16627_POINT_SET_BLAKE3,
            "details":details,
        serde_json::to_writer(&mut self.file, &record)?;
        self.file.write_all(b"\n")?;
        self.file.flush()?;
        self.file.sync_all()?;
        self.next_sequence += 1;
        Ok(())

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

fn write_progress_sample(
    file: &mut File,
    sequence: &mut u64,
    schema: &str,
    progress: &SolverProgress,
    sample_kind: &str,
) -> io::Result<()> {
    let memory_current_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.current")?)?
        .map_err(io::Error::other)?;
    let snapshot = progress.snapshot();
    let record = json!({
        "schema":schema,
        "sequence":*sequence,
        "sample_kind":sample_kind,
        "elapsed_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "memory_current_bytes":memory_current_bytes,
        "object_blake3":K16627_OBJECT_BLAKE3,
        "point_set_blake3":K16627_POINT_SET_BLAKE3,
        "phase":format!("{:?}",snapshot.phase),
        "completed_decisions":snapshot.decisions,
        "completed_conflicts":snapshot.conflicts,
        "completed_restarts":snapshot.restarts,
        "completed_xor_passes":snapshot.xor_passes,
        "completed_xor_nanoseconds":snapshot.xor_nanoseconds,
        "completed_xor_row_scans":snapshot.xor_row_scans,
        "completed_xor_repivots":snapshot.xor_repivots,
        "completed_xor_row_ops":snapshot.xor_row_ops,
    serde_json::to_writer(&mut *file, &record).map_err(io::Error::other)?;
    file.write_all(b"\n")?;
    file.flush()?;
    *sequence += 1;

struct ProgressSampler {
    stop: mpsc::Sender<()>,
    join: thread::JoinHandle<io::Result<()>>,

impl ProgressSampler {
    fn start(
        progress: Arc<SolverProgress>,
        path.push(".progress.jsonl");
            .open(PathBuf::from(path))?;
        let mut sequence = 0;
        write_progress_sample(
            &mut file,
            &mut sequence,
            &source_commit,
            &progress,
            "entered",
        let (stop, receiver) = mpsc::channel();
        let join = thread::spawn(move || loop {
            match receiver.recv_timeout(Duration::from_secs(10)) {
                Err(mpsc::RecvTimeoutError::Timeout) => write_progress_sample(
                    &mut file,
                    &mut sequence,
                    start,
                    schema,
                    &source_commit,
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
                        &progress,
                        "returned",
                    )?;
                    return Ok(());
        Ok(Self { stop, join })

    fn finish(self) -> Result<()> {
        self.stop.send(())?;
        self.join
            .join()
            .map_err(|_| "progress sampler thread panicked")??;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OrbitChoicePins {
    None,
    SelectorAndPhase,
    SelectorOnly,
    PhaseOnly,

impl OrbitChoicePins {
    fn selector(self) -> bool {
        matches!(self, Self::SelectorAndPhase | Self::SelectorOnly)

    fn phase(self) -> bool {
        matches!(self, Self::SelectorAndPhase | Self::PhaseOnly)

    fn schema(self) -> &'static str {
        match self {
            Self::SelectorAndPhase => "n83.k16627-m4-selector-positive/v1",
            Self::SelectorOnly => "n83.k16627-m4-selector-only/v1",
            Self::PhaseOnly => "n83.k16627-m4-phase-only/v1",
            Self::None => "n83.k16627-m4-search-progress/v1",

    fn phase_schema(self) -> &'static str {
            Self::SelectorAndPhase => "n83.k16627-m4-selector-positive-phase/v1",
            Self::SelectorOnly => "n83.k16627-m4-selector-only-phase/v1",
            Self::PhaseOnly => "n83.k16627-m4-phase-only-phase/v1",
            Self::None => "n83.k16627-m4-search-progress-phase/v1",

/// Phase-observable complete x-chain witness on the exact retained primary
/// object. The seven blocks are fixed only after the full orbit domain is
/// installed, and a SAT answer is checked against the original group law.
pub(super) fn chain_witness_cli(root: &Path, memory_mib: u64, output: &Path) -> Result<Value> {
    chain_witness_impl(
        memory_mib,
        OrbitChoicePins::None,

/// The same complete witness, with a separate durable root-propagation gate.
pub(super) fn chain_root_phase_cli(root: &Path, memory_mib: u64, output: &Path) -> Result<Value> {
    chain_witness_impl(root, memory_mib, output, true, false, OrbitChoicePins::None)

/// The exact root-phase workload with separately fsynced search-progress
/// samples while the solver is active.
pub(super) fn chain_search_progress_cli(
    chain_witness_impl(root, memory_mib, output, true, true, OrbitChoicePins::None)

/// The full m=4 orbit model with exact planted representative selectors and
/// phase-zero bits fixed after the seven x blocks. This remains a positive
/// control; the original orbit-domain search is unchanged.
pub(super) fn chain_selector_positive_cli(
        OrbitChoicePins::SelectorAndPhase,

/// The same exact witness with representative selectors fixed but all
/// Frobenius phase wires free.
pub(super) fn chain_selector_only_cli(
        OrbitChoicePins::SelectorOnly,

/// The same exact witness with phase-zero units fixed but representative
/// selector wires free. Run only under the preregistered conditional arm.
pub(super) fn chain_phase_only_cli(root: &Path, memory_mib: u64, output: &Path) -> Result<Value> {
        OrbitChoicePins::PhaseOnly,

fn chain_witness_impl(
    checkpoint_root: bool,
    sample_progress: bool,
    pins: OrbitChoicePins,
    if memory_mib != 4096 {
        return Err("complete-witness control requires the frozen 4096 MiB cap".into());
    let phase_schema = if pins != OrbitChoicePins::None {
        pins.phase_schema()
    } else if sample_progress {
        "n83.k16627-m4-search-progress-phase/v1"
    } else if checkpoint_root {
        "n83.k16627-m4-root-phase/v1"
        "n83.k16627-m4-witness-phase/v1"
    let mut phases = WitnessPhases::create(output, start, source_commit.clone(), phase_schema)?;
    let (base, base_row, manifest_hash) =
        load_panel_selected(root, 16_627, "public_x_hash", 2026100801)?;
    if base_row["object"] != format!("objects/{K16627_OBJECT_BLAKE3}.jsonl.gz")
        || base_row["point_set_blake3"] != K16627_POINT_SET_BLAKE3
        || base.factor_base.signed_orbits.len() != 16_627
        || base.factor_base.points.len() != 2_760_082
        return Err("complete-witness control imported a different retained object".into());
    phases.mark("base_import_done", json!({"base_import_ms":base_import_ms}))?;

    let indices: [usize; 4] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
    let points: [&BinaryPoint; 4] = indices.map(|index| &fb.points[index]);
    let summand_xs: Vec<F2mElement> = points
        .map(|point| match point {
            BinaryPoint::Affine { x, .. } => Ok(x.clone()),
            BinaryPoint::Infinity => Err("planted orbit representative is infinity"),
        .collect::<std::result::Result<_, _>>()?;
    let mut sums = Vec::with_capacity(4);
    for point in points {
        sum = base.curve.add(&sum, point);
        sums.push(sum.clone());
    let target = sum;
    let affine = sums
        .all(|point| matches!(point, BinaryPoint::Affine { .. }));
    let distinct = summand_xs
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
            "affine_partial_sums":affine, "distinct_summand_x":distinct,
        write_new_json(output, &receipt)?;
        return Ok(receipt);
    let preflight_row = checked_primary_relation_row(&base.curve, fb, &indices, &target)?;
    let mut preflight_rank = WideRankTracker::new(&base.curve.subgroup_order, fb.unknowns());
    let preflight_rank_after_insert = preflight_rank.insert(preflight_row.clone());
    let tamper_target = base.curve.add(&target, base.curve.generator());
    let tamper_rejected =
        checked_primary_relation_row(&base.curve, fb, &indices, &tamper_target).is_err();
    if preflight_rank_after_insert != 1 || !tamper_rejected {
        return Err("complete-witness group preflight failed".into());
    let target_x = match &target {
        BinaryPoint::Affine { x, .. } => x.clone(),
        BinaryPoint::Infinity => return Err("complete-witness target is infinity".into()),
    let sum2_x = match &sums[1] {
        BinaryPoint::Infinity => {
            return Err("complete-witness first intermediate is infinity".into())
    let sum3_x = match &sums[2] {
            return Err("complete-witness second intermediate is infinity".into())
    let planted_xs = [
        summand_xs[0].clone(),
        summand_xs[1].clone(),
        summand_xs[2].clone(),
        summand_xs[3].clone(),
        sum2_x,
        sum3_x,
        target_x.clone(),
    phases.mark(
        "group_preflight_done",
            "point_indices":indices,
            "x_decimal":planted_xs.iter().map(|x| x.to_biguint().to_string()).collect::<Vec<_>>(),
            "rank_after_insert":preflight_rank_after_insert,
            "tamper_rejected":tamper_rejected,

            BinaryPoint::Infinity => Err("complete-witness orbit domain has infinity"),
        Some(&target_x),
        250_000,
    .map_err(|_| "complete-witness chained-S3 model build failed")?;
    enc.constrain_frobenius_orbit_summands(&representatives, 250_000, 6_000_000)
        .map_err(|_| "complete-witness orbit-domain install failed")?;
    if checkpoint_root
        && (enc.solver.n_vars() != 201_598
            || enc.solver.n_clauses() != 5_915_238
            || enc.solver.n_xors() != 3_901)
        return Err("root-phase control constructed a different full orbit model".into());
    let model_domain_ms = model_start.elapsed().as_secs_f64() * 1000.0;
        "model_domain_done",
            "model_domain_ms":model_domain_ms,
            "sat_variables":enc.solver.n_vars(),
            "sat_clauses":enc.solver.n_clauses(),
            "sat_xor_rows":enc.solver.n_xors(),
    for (block, x) in planted_xs.iter().enumerate() {
        enc.pin_x_block(block, x)
            .map_err(|_| "complete-witness x-block pin failed")?;
    phases.mark("seven_blocks_pinned", json!({"x_blocks":7}))?;
    let mut orbit_choice_receipt = Value::Null;
    if pins != OrbitChoicePins::None {
        if sorted_representatives
            .windows(2)
            .any(|pair| pair[0] == pair[1])
            return Err("selector control has duplicate representatives".into());
        let wires = enc
            .ok_or("selector control has no installed orbit-choice wires")?;
        if wires.len() != 4 {
            return Err("selector control has the wrong summand count".into());
        let mut choices = Vec::with_capacity(4);
        for (slot, x) in summand_xs.iter().enumerate() {
            let value = x.to_biguint();
            let representative_index = sorted_representatives
                .binary_search(&value)
                .map_err(|_| "selector control planted x is not a representative")?;
            let handle = &wires[slot];
            if handle.selectors.len() != sorted_representatives.len() || handle.phase.len() != 7 {
                return Err("selector control allocated unexpected choice wires".into());
            choices.push((
                slot,
                representative_index,
                handle.selectors[representative_index],
                handle.phase.clone(),
                value,
            ));
        for (_, _, selector, phase, _) in &choices {
            if pins.selector() && !enc.solver.add_clause(vec![*selector as i32]) {
                return Err("orbit-choice selector unit contradicted model".into());
            if pins.phase() {
                for &wire in phase {
                        return Err("orbit-choice phase-zero unit contradicted model".into());
        orbit_choice_receipt = json!(choices
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
            .collect::<Vec<_>>());
        phases.mark("orbit_choices_pinned", orbit_choice_receipt.clone())?;
    enc.solver.deadline = Some(start + Duration::from_secs(1800));
    let root_result = if checkpoint_root {
        phases.mark(
            "root_propagation_entered",
            json!({"stats":root_phase_stats(&enc.solver.stats)}),
        let root_start = Instant::now();
        let result = enc.solver.propagate_root();
            "root_propagation_returned",
            json!({
                "result":format!("{result:?}"),
                "root_propagation_ms":root_start.elapsed().as_secs_f64()*1000.0,
                "stats":root_phase_stats(&enc.solver.stats),
            }),
        Some(result)
    let (answer, solver_ms) = if root_result == Some(RootPropagationResult::Unsat) {
        (SolveResult::Unsat, 0.0)
        phases.mark("sat_entered", json!({"conflict_budget":100_000}))?;
        let sampler = if sample_progress {
            let progress = Arc::new(SolverProgress::default());
            enc.solver.set_progress_observer(Arc::clone(&progress));
            Some(ProgressSampler::start(
                output,
                start,
                if pins != OrbitChoicePins::None {
                    pins.schema()
                    "n83.k16627-m4-search-progress/v1"
                },
                source_commit.clone(),
                progress,
            )?)
            None
        let solver_start = Instant::now();
        let answer = enc.solver.solve();
        if let Some(sampler) = sampler {
            sampler.finish()?;
        (answer, solver_start.elapsed().as_secs_f64() * 1000.0)
    let models = usize::from(answer == SolveResult::Sat);
        "sat_returned",
            "answer":format!("{answer:?}"),
            "solver_ms":solver_ms,
            "conflicts":enc.solver.conflicts(),
            "models":models,

        SolveResult::Unsat => "INVALID_UNSAT_PLANTED",
            if !enc.verify_model() || enc.decode_x_blocks() != planted_xs {
            } else if let Some((lifted_indices, lifted_row)) =
                checked_four_x_lift(&base.curve, fb, &summand_xs, &target)?
                phases.mark(
                    "group_lift_done",
                    json!({
                        "point_indices":lifted_indices,
                        "group_verified":true,
                    }),
                )?;
                let rank_after_insert = rank.insert(lifted_row.clone());
                phases.mark("rank_done", json!({"rank_after_insert":rank_after_insert}))?;
                let matches_preflight_row = lifted_row == preflight_row;
                let dense: Vec<_> = lifted_row.iter().map(ToString::to_string).collect();
                    "point_indices":lifted_indices,
                    "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense)?).to_hex().to_string(),
                    "matches_preflight_row":matches_preflight_row,
                    "group_verified":true,
                if rank_after_insert != 1 || !matches_preflight_row {
                    "INVALID_RELATION_CONTROL"
                    match pins {
                        OrbitChoicePins::SelectorAndPhase => "PASS_SELECTOR_PINNED_WITNESS",
                        OrbitChoicePins::SelectorOnly => "PASS_SELECTOR_ONLY_WITNESS",
                        OrbitChoicePins::PhaseOnly => "PASS_PHASE_ONLY_WITNESS",
                        OrbitChoicePins::None => "PASS_COMPLETE_WITNESS",
                "INVALID_GROUP_LIFT"
    let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        "schema":if pins != OrbitChoicePins::None {pins.schema()}
            else if sample_progress {"n83.k16627-m4-search-progress/v1"}
            else if checkpoint_root {"n83.k16627-m4-root-phase/v1"}
            else {"n83.k16627-m4-witness/v1"},
        "status":status,
        "curve_a":0, "orbit_columns":16_627, "summands":4, "identity_mask":0,
        "policy":"public_x_hash", "seed":2026100801u64,
        "panel_manifest_blake3":manifest_hash,
        "source_index_calculus_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string(),
        "phase_file":phases.path.to_string_lossy(),
        "progress_file":if sample_progress {
            json!(format!("{}.progress.jsonl",output.display()))
        } else {Value::Null},
        "point_indices":indices, "x_decimal":planted_xs.iter().map(|x| x.to_biguint().to_string()).collect::<Vec<_>>(),
        "max_variables":250_000, "max_domain_clauses":6_000_000,
        "conflict_budget":100_000, "max_models":1,
        "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(), "solver_models":models,
        "base_import_ms":base_import_ms, "model_domain_ms":model_domain_ms,
        "solver_ms":solver_ms, "process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "memory_cgroup_swap_limit_bytes":0,
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        receipt["orbit_choice_pins"] = orbit_choice_receipt;
    if checkpoint_root {
        receipt["source_sat_blake3"] = Value::String(
            blake3::hash(include_bytes!("../../src/cryptanalysis/sat.rs"))
                .to_hex()
                .to_string(),
        receipt["root_propagation_result"] = root_result
            .map(|result| Value::String(format!("{result:?}")))
            .unwrap_or(Value::Null);

/// Bound a constructed-target SAT control to the exact replayed K=16,627
/// primary object. The pinned arm fixes four x wires only after the complete
/// orbit-domain model is installed. The unpinned arm uses the same target and
/// model, with no access to those fixed values in its solver clauses.
pub(super) fn chain_planted_cli(
    pinned: bool,
        return Err("planted control requires the frozen 4096 MiB cap".into());
    if base_row["object"]
        != "objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz"
        || base_row["point_set_blake3"]
            != "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"
        return Err("planted control imported a different retained object".into());
    if fb.signed_orbits.len() != 16_627 || fb.points.len() != 2_760_082 {
        return Err("planted control imported the wrong retained base size".into());
    let planted_indices: [usize; 4] = std::array::from_fn(|column| fb.signed_orbits[column][0]);
    let planted_points: [&BinaryPoint; 4] = planted_indices.map(|index| &fb.points[index]);
    let planted_xs: Vec<F2mElement> = planted_points
    let mut partial = BinaryPoint::Infinity;
    for point in planted_points {
        partial = base.curve.add(&partial, point);
        sums.push(partial.clone());
    let target = partial;
    let control_valid = planted_xs
        == 4
        && sums
            .all(|point| matches!(point, BinaryPoint::Affine { .. }));
    let target_json = json!({
        "columns": [0,1,2,3],
        "point_indices": planted_indices,
        "points": planted_points.map(|point| planted_point_json(point)),
        "partial_sums": sums.iter().map(planted_point_json).collect::<Vec<_>>(),
        "target": planted_point_json(&target),
    if !control_valid {
            "schema":"n83.k16627-m4-planted/v1", "status":"INVALID_PLANTED_CONTROL",
            "arm":if pinned {"pinned"} else {"unpinned"},
            "object":base_row["object"], "point_set_blake3":base_row["point_set_blake3"],
            "source_commit":source_commit, "planted":target_json,
            "base_import_ms":base_import_ms,
            "solver_search_executed":false,
    let mut codes: Vec<BigUint> = fb.subspace.iter().map(F2mElement::to_biguint).collect();
    if codes.len() != 83 * 16_627 {
        return Err("planted control coordinate closure mismatch".into());
            BinaryPoint::Infinity => Err("planted orbit domain has infinity"),
        BinaryPoint::Affine { x, .. } => x,
        BinaryPoint::Infinity => return Err("invalid planted target".into()),
    .map_err(|_| "planted chained-S3 model build failed")?;
        .map_err(|_| "planted orbit-domain install failed")?;
        // The chained-S3 builder allocates the first m*n wires for the
        // summand x blocks; the core's own model blocker uses this layout.
        for (slot, x) in planted_xs.iter().enumerate() {
                let wire = (slot * 83 + bit + 1) as i32;
                enc.solver
                    .add_clause(vec![if value.bit(bit as u64) { wire } else { -wire }]);
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
                    invalid_models += 1;
                    break ("INVALID_MODEL", None);
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
                    let nonzero: Vec<_> = row
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
                group_rejected_models += 1;
                if models >= max_models {
                    break ("UNKNOWN_model_cap", None);
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
                        .collect(),
                );
    let solver_ms = solver_start.elapsed().as_secs_f64() * 1000.0;
        "schema":"n83.k16627-m4-planted/v1", "status":status,
        "arm":if pinned {"pinned"} else {"unpinned"},
        "object":base_row["object"], "point_set_blake3":base_row["point_set_blake3"],
        "planted":target_json, "relation":relation,
        "conflict_budget":conflict_budget, "max_models":max_models,
        "invalid_models":invalid_models, "group_rejected_models":group_rejected_models,
        "base_import_ms":base_import_ms, "model_construction_ms":model_construction_ms,

/// Probe the published primary point with a replayed retained base through
/// the point-only wide compact index. A hit is also converted to a full-width
/// relation row and independently replayed. This is one relation diagnostic,
/// not a rank or total-runtime result. A Linux cgroup with a hard memory limit and
/// zero swap is mandatory; the caller must also impose a process-wall cap.
pub(super) fn s3_probe_cli(
    unordered_pairs: bool,
    max_candidate_states: usize,
    if ![64, 256, 600].contains(&columns) {
        return Err("S3 probe supports retained primary K=64/256/600 bases".into());
    let (source_commit, source_attestation_mode) = frozen_primary_probe_source()?;
    let base_import_ms = started.elapsed().as_secs_f64() * 1000.0;
    let target_validation_ms = started.elapsed().as_secs_f64() * 1000.0 - base_import_ms;
    let representatives: Vec<_> = base
        .map(|orbit| base.factor_base.points[orbit[0]].clone())
    let probe_started = Instant::now();
    let probe = compact_cold::probe_checked_four_sum(
        &base.curve,
        &base.factor_base.points,
        &target,
        unordered_pairs,
        max_candidate_states,
    let probe_ms = probe_started.elapsed().as_secs_f64() * 1000.0;
    let row_started = Instant::now();
    let relation_row = if probe["status"] == "HIT" {
        let indices: [usize; 4] = probe["point_indices"]
            .as_array()
            .ok_or("compact hit lacks point indices")?
            .map(|value| {
                value
                    .as_u64()
                    .and_then(|index| usize::try_from(index).ok())
                    .ok_or("compact hit has invalid point index")
            .collect::<std::result::Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "compact hit is not a four-summand witness")?;
        let row = checked_primary_relation_row(&base.curve, &base.factor_base, &indices, &target)?;
        let dense_decimal: Vec<_> = row.iter().map(ToString::to_string).collect();
        let nonzero: Vec<_> = row
            .enumerate()
            .filter(|(_, coefficient)| !coefficient.is_zero())
            .map(|(column, coefficient)| {
                json!({"column":column,"coefficient_decimal":coefficient.to_string()})
            "schema":"n83.primary-wide-relation-row/v1",
            "modulus_decimal":base.curve.subgroup_order.to_string(),
            "orbit_columns":columns,
            "nonzero":nonzero,
            "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense_decimal)?).to_hex().to_string(),
            "group_verified":true
    } else if probe["status"] == "MISS" {
        return Err("compact probe returned an unknown status".into());
    let relation_row_stage_executed = relation_row.is_some();
    let relation_row_ms = row_started.elapsed().as_secs_f64() * 1000.0;
        .map_err(|_| "S3 probe requires a numeric cgroup memory.peak")?;
        "schema":"n83.primary-compact-s3-probe/v1",
        "study":STUDY,
        "status":probe["status"],
        "curve_a":0,
        "fixture":0,
        "orbit_columns":columns,
        "policy":policy,
        "seed":seed,
        "object":row["object"],
        "source_compact_blake3":blake3::hash(include_bytes!("compact_cold.rs")).to_hex().to_string(),
        "max_candidate_states":max_candidate_states,
        "base_import_ms":base_import_ms,
        "target_validation_ms":target_validation_ms,
        "point_probe_ms":probe_ms,
        "relation_row_ms":relation_row_ms,
        "probe":probe,
        "relation_row":relation_row,
        "relation_row_stage_executed":relation_row_stage_executed,
        "rank_stage_executed":false,
        "column_log_verification":false,

/// Construct the exact K=64/256/600 finite-domain factored S4 model and
/// stop before search. The caller must impose and independently verify a
/// hard process-wall limit; this worker refuses an absent cgroup memory cap.
pub(super) fn build_capacity_cli(
        return Err("capacity gate supports only retained primary K=64/256/600 bases".into());
    let (base, row, manifest_hash) = load_panel(root, columns)?;
    let enc = build_union_s4_encoding(
        &base.factor_base,
        SatDecompositionOptions {
            factored_s4: true,
            ..Default::default()
        },
        .map_err(|_| "capacity worker requires a numeric cgroup memory.peak")?;
        "schema":"n83.factored-s4-capacity-worker/v1",
        "status":"PASS_model_construction_only",
        "source_union_builder_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string(),
        "source_factored_encoder_blake3":blake3::hash(include_bytes!("../../src/cryptanalysis/semaev_sat.rs")).to_hex().to_string(),
        "base_points":base.factor_base.points.len(),
        "legal_x_coordinates":base.factor_base.subspace.len(),
        "sat_x_variables":enc.n_x_vars,
        "sat_e_variables":enc.n_e_vars,
        "sat_aux_variables":enc.n_aux_vars,
        "trivially_unsat":enc.trivially_unsat,
        "process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "model_lifting_executed":false,

fn public_target(root: &Path, kc: &KoblitzCurve) -> Result<(BinaryPoint, String)> {
    public_target_fixture(root, kc, 0)

fn public_target_fixture(
) -> Result<(BinaryPoint, String)> {
    let corpus: Value = serde_json::from_slice(&fs::read(root.join("probe-corpus.json"))?)?;
    if corpus["schema"] != "n83.public-probe-corpus/v1" || corpus["study"] != STUDY {
        return Err("public target corpus mismatch".into());
    let targets = corpus["targets"].as_array().ok_or("public targets")?;
    let mut selected = targets
        .filter(|row| row["a"] == 0 && row["fixture"] == fixture);
    let row = selected.next().ok_or("primary public fixture absent")?;
    if selected.next().is_some() {
        return Err("ambiguous primary public fixture".into());
    let encoded = parse_point(&row["point"])?.ok_or("identity public target")?;
    let target = general(Some(encoded));
    if !kc.curve.is_on_curve(&target)
        || kc.mul(&target, &kc.subgroup_order) != BinaryPoint::Infinity
        return Err("primary public target fails group validation".into());
    let corpus_hash = blake3::hash(&serde_json::to_vec(&corpus)?)
        .to_hex()
        .to_string();
    Ok((target, corpus_hash))

fn candidate_matches_validation(
    corpus_hash: &str,
    candidate: &BigUint,
) -> Result<bool> {
    let validation: Value = serde_json::from_slice(&fs::read(root.join("probe-validation.json"))?)?;
    if validation["schema"] != "n83.public-probe-validation/v1"
        || validation["corpus_canonical_json_blake3"] != corpus_hash
        || validation["oracle_receives_known_answer_scalars"] != false
        return Err("public validation sidecar mismatch".into());
    let entries = validation["validation"]
        .ok_or("validation entries")?;
    let mut selected = entries
        .filter(|row| row["a"] == 0 && row["fixture"] == 0);
    let row = selected.next().ok_or("primary validation fixture absent")?;
        return Err("ambiguous primary validation fixture".into());
    let expected: BigUint = row["known_answer_scalar"]
        .ok_or("known-answer encoding")?
    Ok(*candidate == expected)

/// Run one bounded primary workload. The public target is the solver's only
/// target input; its known-answer sidecar is opened only after a candidate log.
pub(super) fn run_cli(
    strategy_name: &str,
    max_trials: usize,
    run_dir: &Path,
    run_with_config(
        columns,
        m,
        strategy_name,
        max_trials,
        run_dir,
        None,

struct ChainRunConfig<'a> {
    policy: &'a str,
    fixture_root: &'a Path,
    limits: ChainS3Limits,
    memory_limit_bytes: u64,
    source_attestation_mode: &'static str,

struct StoreRunConfig<'a> {
    store: &'a Store,

fn seal_store_run_error(run_dir: &Path, result: Result<Value>) -> Result<Value> {
    match result {
        Ok(summary) => Ok(summary),
        Err(error) => {
            if run_dir.is_dir()
                && !run_dir.join("summary.json").exists()
                && !run_dir.join("cap.json").exists()
                let receipt = json!({
                    "schema":"n83.primary-store-command-error/v1",
                    "status":"COMMAND_ERROR",
                    "reason":error.to_string(),
                    "solver_stage_executed":Value::Null,
                    "total_index_calculus_runtime_ms":Value::Null,
                    "selected_best_total_runtime":Value::Null,
                write_new_json(&run_dir.join("failure.json"), &receipt).map_err(|write_error| {
                    format!("{error}; failed to seal store command error: {write_error}")
                })?;
            Err(error)

/// Run a bounded enumerate trial against a verified v3 artifact-store base.
/// Base construction and publication are outside this diagnostic interval.
pub(super) fn run_store_trial_cli(
    if columns == 0
        || !(2..=6).contains(&m)
        || max_trials > 1000
        || budget_seconds == 0
        || budget_seconds > 7200
        || !fixture_root.join("probe-corpus.json").is_file()
        || !fixture_root.join("probe-validation.json").is_file()
        return Err("v3 store trial caps or fixture inputs invalid".into());
    let config = StoreRunConfig {
        store: &store,
        source_commit,
        source_attestation_mode,
    seal_store_run_error(
        run_with_config(
            root,
            columns,
            "enumerate",
            max_trials,
            budget_seconds,
            run_dir,
            None,
            Some(&config),

/// Run a bounded chained-S3 search against a verified v3 artifact-store base.
/// The hard cgroup gate applies even to a zero-query preflight.
pub(super) fn run_store_chain_cli(
        || columns > 32_643
        || limits.max_variables == 0
        || limits.max_variables > i32::MAX as u32
        || limits.max_domain_clauses == 0
        || limits.max_domain_clauses > i32::MAX as usize
        || limits.max_models == 0
        || limits.max_models > 1_000_000
        || limits.conflict_budget == 0
        || limits.conflict_budget > 10_000_000_000
        || !(128..=65_536).contains(&memory_mib)
        return Err("v3 store chained-S3 caps or fixture inputs invalid".into());
    let manifest: Value = serde_json::from_slice(&fs::read(root.join("manifest.json"))?)?;
    let policy = manifest["base"]["policy"]
        .ok_or("v3 store policy missing")?;
    let seed = manifest["base"]["seed"]
        .as_u64()
        .ok_or("v3 store seed missing")?;
    let chain_config = ChainRunConfig {
        policy,
        seed,
        limits,
        memory_limit_bytes,
        source_commit: source_commit.clone(),
    let store_config = StoreRunConfig {
    let strategy = match (frobenius_orbits, order_first_pair) {
        (false, false) => "chain-s3",
        (true, false) => "orbit-chain-s3",
        (true, true) => "ordered-orbit-chain-s3",
        (false, true) => unreachable!("checked above"),
            strategy,
            Some(&chain_config),
            Some(&store_config),

fn supported_chain_wall_budget(
    conflicts: u64,
    seconds: u64,
) -> bool {
    (1..=7200).contains(&seconds)
        || (m == 5
            && frobenius_orbits
            && !order_first_pair
            && matches!(
                (columns, conflicts, seconds),
                (2048, 1_500_000, 8957) | (4096, 1_500_000, 9000)
            ))

/// Execute one replay-bound primary chained-S3 search only inside a verified
/// hard-memory cgroup. The outer supervisor owns the separate wall deadline.
pub(super) fn run_chain_cli(
    karatsuba: bool,
    shallow_karatsuba: bool,
    if ![64, 256, 600].contains(&columns) && !V2_SIZES.contains(&columns)
        .contains(&policy)
        || !supported_chain_wall_budget(
            limits.conflict_budget,
            frobenius_orbits,
            order_first_pair,
        || memory_mib < 128
        || memory_mib > 65536
        || (karatsuba && frobenius_orbits)
        || (karatsuba && shallow_karatsuba)
            "primary chained-S3 search caps or base selection outside supported domain".into(),
    let config = ChainRunConfig {
    let strategy = match (
        frobenius_orbits,
        order_first_pair,
        karatsuba,
        shallow_karatsuba,
    ) {
        (false, false, false, false) => "chain-s3",
        (false, false, true, false) => "chain-s3-karatsuba",
        (false, false, false, true) => "chain-s3-shallow-karatsuba",
        (true, false, false, false) => "orbit-chain-s3",
        (true, false, false, true) => "orbit-chain-s3-shallow-karatsuba",
        (true, true, false, false) => "ordered-orbit-chain-s3",
        (true, true, false, true) => "ordered-orbit-chain-s3-shallow-karatsuba",
        _ => unreachable!("checked above"),
        strategy,
        Some(&config),

fn run_with_config(
    chain: Option<&ChainRunConfig<'_>>,
    store: Option<&StoreRunConfig<'_>>,
    if let (Some(chain_config), Some(store_config)) = (chain, store) {
        if chain_config.source_commit != store_config.source_commit
            || chain_config.fixture_root != store_config.fixture_root
            || chain_config.source_attestation_mode != store_config.source_attestation_mode
            return Err("v3 store chain source or fixture mismatch".into());
    if !(store.is_some()
        || [64, 256, 600].contains(&columns)
        || (chain.is_some() && V2_SIZES.contains(&columns)))
        return Err("unsupported primary K or summand count".into());
    let strategy = match strategy_name {
        "enumerate" => DecompositionStrategy::Enumerate,
        "sat-m3" => {
            return Err(
                "primary SAT S4 model replay and resource-capacity gate is incomplete for F2^83"
                    .into(),
            )
        "chain-s3" if chain.is_some() && (4..=6).contains(&m) => DecompositionStrategy::ChainedS3,
        "chain-s3-karatsuba" if chain.is_some() && (5..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3Karatsuba
        "chain-s3-shallow-karatsuba" if chain.is_some() && (5..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3ShallowKaratsuba
        "orbit-chain-s3" if chain.is_some() && (4..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3FrobeniusOrbit
        "orbit-chain-s3-shallow-karatsuba" if chain.is_some() && (5..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3FrobeniusOrbitShallowKaratsuba
        "ordered-orbit-chain-s3" if chain.is_some() && (4..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3FrobeniusOrbitOrdered
        "ordered-orbit-chain-s3-shallow-karatsuba" if chain.is_some() && (5..=6).contains(&m) => {
            DecompositionStrategy::ChainedS3FrobeniusOrbitOrderedShallowKaratsuba
        _ => return Err("unsupported or unguarded primary strategy".into()),
    if budget_seconds == 0 || budget_seconds > 86_400 {
        return Err("primary wall cap must be 1..86400 seconds".into());
    let trace_chained_s3_phases = match std::env::var("N83_CHAIN_PHASE_TRACE") {
        Ok(value) if value == "1" => true,
        Ok(value) if value == "0" => false,
        Err(std::env::VarError::NotPresent) => false,
        _ => return Err("N83_CHAIN_PHASE_TRACE must be 0 or 1".into()),
    if trace_chained_s3_phases
        && (!matches!(
            DecompositionStrategy::ChainedS3
                | DecompositionStrategy::ChainedS3FrobeniusOrbit
                | DecompositionStrategy::ChainedS3FrobeniusOrbitOrdered
        ) || max_trials != 1)
        return Err("N83 phase trace requires one chained-S3 trial".into());
    fs::create_dir(run_dir)?;
    let mut config_record = json!({
        "schema":"n83.primary-cold-config/v1", "study":STUDY,
        "panel_dir":root.to_string_lossy(),
        "input_kind":if store.is_some() { "artifact_store_v3" } else { "historical_panel_v1v2" },
        "fixture_dir":chain.map(|config| config.fixture_root.to_string_lossy().to_string())
            .or_else(|| store.map(|config| config.fixture_root.to_string_lossy().to_string())),
        "curve_a":0, "fixture":0, "orbit_columns":columns,
        "summands":m, "strategy":strategy_name, "max_trials":max_trials,
        "budget_seconds":budget_seconds, "allow_direct_relation":false,
        "trace_chained_s3_phases":trace_chained_s3_phases,
        "source_commit":chain.map(|config| config.source_commit.as_str())
            .or_else(|| store.map(|config| config.source_commit.as_str())),
        "source_attestation_mode":chain.map(|config| config.source_attestation_mode)
            .or_else(|| store.map(|config| config.source_attestation_mode)),
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
    if store.is_some() {
        config_record["schema"] = json!(if chain.is_some() {
            "n83.primary-store-chain-config/v1"
            "n83.primary-store-trial-config/v1"
        config_record["base_dir"] = json!(root.to_string_lossy().to_string());
        config_record["panel_dir"] = Value::Null;
        config_record["measurement_scope"] = json!(if chain.is_some() {
            "store import and bounded chained-S3 trial; base construction and publication are outside this interval"
            "store import and bounded enumerate trial; base construction and publication are outside this interval"
    write_new_json(&run_dir.join("config.json"), &config_record)?;
    let cap_path = run_dir.join("cap.json");
    let cap_strategy = strategy_name.to_owned();
    let deadline = start + Duration::from_secs(budget_seconds);
        std::thread::sleep(deadline.saturating_duration_since(Instant::now()));
            "schema":"n83.primary-cold-cap/v1", "status":"UNKNOWN_budget",
            "orbit_columns":columns, "summands":m, "strategy":cap_strategy,
            "max_trials":max_trials, "budget_seconds":budget_seconds,
            "elapsed_process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
            "selected_best_total_runtime":Value::Null

    let (base, row, manifest_hash) = if let Some(config) = store {
        let (base, row) = load_store_primary_base(root, config.store)?;
        if row["columns"].as_u64() != Some(columns as u64) {
            return Err("v3 store base column count differs from requested workload".into());
        if let Some(chain_config) = chain {
            if row["policy"] != chain_config.policy || row["seed"] != chain_config.seed {
                return Err("v3 store chain selection differs from verified base".into());
        let manifest_hash = sha256_hex(&fs::read(root.join("manifest.json"))?);
        (base, row, manifest_hash)
    } else if let Some(config) = chain {
        load_panel_selected(root, columns, config.policy, config.seed)?
        load_panel(root, columns)?
    let base_import_ms = import_start.elapsed().as_secs_f64() * 1000.0;
    let target_start = Instant::now();
    let fixture_root = chain
        .map(|config| config.fixture_root)
        .or_else(|| store.map(|config| config.fixture_root))
        .unwrap_or(root);
    let target_validation_ms = target_start.elapsed().as_secs_f64() * 1000.0;

    let mut options = KoblitzIcOptions::default();
    options.m = m;
    options.strategy = strategy;
    options.max_trials = max_trials;
    options.seed = 2026100901;
    options.allow_direct_relation = false;
    if let Some(config) = chain {
        options.chain_s3_limits = config.limits;
    let mut events = OpenOptions::new()
        .open(run_dir.join("events.jsonl"))?;
    let mut event_write_failed = false;
    let report = koblitz_index_calculus_dlp_with_factor_base_and_progress(
        &options,
        &mut |event| {
            let line = json!({
                "elapsed_process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
                "event":format!("{event:?}")
            if writeln!(events, "{line}")
                .and_then(|_| events.flush())
                .is_err()
                event_write_failed = true;
    if event_write_failed {
        return Err("primary progress-event write failed".into());
    let Some(report) = report else {
        let summary = json!({
            "schema":"n83.primary-cold-result/v1", "status":"PRODUCER_FAILURE",
            "reason":"generic solver rejected the primary base or strategy",
            "solver_stage_executed":false, "selected_best_total_runtime":Value::Null
        write_new_json(&run_dir.join("summary.json"), &summary)?;
        return Err("generic primary solver rejected the imported base or strategy".into());
    let verification_start = Instant::now();
    let candidate_valid = match &report.log {
        Some(candidate) => {
            base.curve.mul(base.curve.generator(), candidate) == target
                && candidate_matches_validation(fixture_root, &corpus_hash, candidate)
                    .unwrap_or(false)
        None => false,
    let post_solver_validation_ms = verification_start.elapsed().as_secs_f64() * 1000.0;
    // A source-bound zero-trial store preflight must report its real cgroup peak.
    // The in-process test fixture has no Linux cgroup and remains explicitly
    // marked `test_fixture`, so it cannot be confused with an admitted run.
    let fixture_only = store.is_some()
        && max_trials == 0
        && chain.is_some_and(|config| config.source_attestation_mode == "test_fixture");
    let memory_cgroup_peak_bytes = if chain.is_some() && !fixture_only {
            fs::read_to_string(cgroup_v2_file("memory.peak")?)?
                .parse::<u64>()?,
    let status = if report.inconsistent_relations != 0
        || report.verification_failures != 0
        || report.sat_invalid_models != 0
        || report.direct_relation
        || (report.log.is_some() && !candidate_valid)
        "PRODUCER_FAILURE"
    } else if candidate_valid {
        "PASS_verified_target_only"
    } else if !report.m_cofactor_admissible {
        "INADMISSIBLE_cofactor_class"
    } else if max_trials == 0 {
        "PREFLIGHT_ONLY"
    } else if chain.is_some() && report.sat_unknowns != 0 {
        "UNKNOWN_solver_cap"
        "UNKNOWN_trial_cap"
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
    let attempt_records: Vec<_> = report
        .attempt_records
        .map(|attempt| {
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
    let relation_matrix: Vec<_> = report
        .relation_matrix
        .map(|relation| {
                "a":relation.coef_a.to_string(),
                "b":relation.coef_b.to_string(),
                "summands":relation.summands,
                "summand_negated":relation.summand_negated,
                "row":relation.row.iter().map(ToString::to_string).collect::<Vec<_>>()
    let rank_history: Vec<_> = report
        .rank_history
        .map(|rank| {
                "rows":rank.rows,
                "columns":rank.columns,
                "rank":rank.rank,
                "candidate_produced":rank.candidate_produced,
                "candidate_verified":rank.candidate_verified
    let mut summary = json!({
        "schema":"n83.primary-cold-result/v1", "study":STUDY,
        "status":status, "curve_a":0, "fixture":0,
        "object":row["object"], "point_set_blake3":row["point_set_blake3"],
        "orbit_columns":columns, "summands":m, "strategy":strategy_name,
        "policy":row["policy"], "seed":row["seed"],
        "max_trials":max_trials, "sampler_seed":options.seed,
        "source_chain_blake3":chain.map(|_| blake3::hash(include_bytes!("../../src/cryptanalysis/binary_semaev_chain_sat.rs")).to_hex().to_string()),
        "source_index_calculus_blake3":chain.map(|_| blake3::hash(include_bytes!("../../src/cryptanalysis/koblitz_index_calculus.rs")).to_hex().to_string()),
        "process_usage":if chain.is_some() { process_usage() } else { Value::Null },
        "solver_ms":solver_ms, "post_solver_validation_ms":post_solver_validation_ms,
        "pre_summary_process_wall_ms":start.elapsed().as_secs_f64()*1000.0,
        "solver_stage_executed":if chain.is_some() {report.sat_calls>0} else {report.trials>0},
        "report":report_details,
        "attempt_records":attempt_records,
        "relation_matrix":relation_matrix,
        "rank_history":rank_history,
        "verified_log":if candidate_valid { report.log.as_ref().map(ToString::to_string) } else { None },
        summary["schema"] = json!(if chain.is_some() {
            "n83.primary-store-chain/v1"
            "n83.primary-store-trial/v1"
        summary["input_kind"] = json!("artifact_store_v3");
        summary["object"] = row["store_name"].clone();
        summary["store_name"] = row["store_name"].clone();
        summary["store_manifest_sha256"] = summary["panel_manifest_blake3"].clone();
        summary["panel_manifest_blake3"] = Value::Null;
        summary["measurement_scope"] = json!(if chain.is_some() {
            "bounded store import and chained-S3 trial only; base construction and publication are outside this interval; total cold index-calculus runtime remains unset"
            "bounded store import and enumerate trial only; base construction and publication are outside this interval; total cold index-calculus runtime remains unset"
        summary["input_kind"] = json!("historical_panel_v1v2");
    write_new_json(&run_dir.join("summary.json"), &summary)?;
    if status == "PRODUCER_FAILURE" {
        Err("primary solver result failed verification; inspect summary.json".into())
        Ok(summary)

#[cfg(test)]
mod tests {
    use super::super::construct;
    use super::*;

    #[test]
    fn continuation_wall_admission_preserves_legacy_and_rejects_nearby_requests() {
        for (columns, seconds) in [(2048, 8957), (4096, 9000)] {
            assert!(supported_chain_wall_budget(
                columns, 5, 1_500_000, seconds, true, false
            for (k, m, conflicts, wall, orbits, ordered) in [
                (columns, 5, 1_500_000, seconds - 1, true, false),
                (columns, 5, 1_500_000, seconds + 1, true, false),
                (columns, 5, 1_500_001, seconds, true, false),
                (columns, 6, 1_500_000, seconds, true, false),
                (columns, 5, 1_500_000, seconds, false, false),
                (columns, 5, 1_500_000, seconds, true, true),
                (8192, 5, 1_500_000, seconds, true, false),
            ] {
                assert!(!supported_chain_wall_budget(
                    k, m, conflicts, wall, orbits, ordered
                ));
        for seconds in [1, 6000, 7200] {
                64, 6, 100, seconds, false, false
        assert!(!supported_chain_wall_budget(
            2048, 5, 1_500_000, 0, true, false
            2048, 5, 1_500_000, 9000, true, false
            2048, 5, 1_500_000, 8987, true, false
            4096, 5, 1_500_000, 8957, true, false

    fn actual_chain_entry_passes_continuation_admission_before_cgroup_or_input_refusal() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let tmp =
            std::env::temp_dir().join(format!("n83-wall-admission-{}-{nonce}", std::process::id()));
        let fixtures = tmp.join("fixtures");
        fs::create_dir_all(&fixtures).unwrap();
        fs::write(fixtures.join("probe-corpus.json"), b"{}").unwrap();
        fs::write(fixtures.join("probe-validation.json"), b"{}").unwrap();
            // The deliberately absent panel prevents any SAT execution even
            // if the test happens to run inside a matching hard-memory cgroup.
            let err = run_chain_cli(
                &tmp.join("absent-panel"),
                &fixtures,
                columns,
                "public_x_hash",
                2026100801,
                5,
                1,
                ChainS3Limits {
                    max_variables: 150000,
                    max_domain_clauses: 2000000,
                    max_models: 32,
                    conflict_budget: 1500000,
                seconds,
                8192,
                &tmp.join(format!("out-{columns}")),
                true,
                false,
            .unwrap_err()
            .to_string();
            assert_ne!(
                err,
                "primary chained-S3 search caps or base selection outside supported domain"
            );
        fs::remove_dir_all(tmp).unwrap();
    use crypto_lib::cryptanalysis::koblitz_index_calculus::{
        enumerate_decompose, koblitz_index_calculus_dlp_with_factor_base, KoblitzIcOptions,
    use crypto_lib::cryptanalysis::koblitz_relation_solver::WideRankTracker;
    use crypto_lib::store::{Config as StoreConfig, PushMeta};
    use flate2::{write::GzEncoder, Compression};
    use std::collections::HashMap;
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    fn k600_native_replay_rejects_partial_and_false_xcnf_models() {
        let formula = "p cnf 3 3\n1 -2 0\n3 0\nx -1 2 3 0\n";
        let model = k600_model_values("s SATISFIABLE\nv 1 -2 3 0\n", 3).unwrap();
        assert_eq!(model, [0, 1, 2, 1]);
        k600_check_xcnf(formula, &model, 2, 1).unwrap();
        assert!(k600_model_values("s SATISFIABLE\nv 1 -2 0\n", 3).is_err());
        assert!(k600_model_values("s SATISFIABLE\nv 1 -2 3 3 0\n", 3).is_err());
        assert!(k600_model_values("s UNSATISFIABLE\nv 1 -2 3 0\n", 3).is_err());
        assert!(k600_check_xcnf(formula, &[0, 2, 2, 1], 2, 1).is_err());
        assert!(k600_check_xcnf(formula, &[0, 1, 1, 1], 2, 1).is_err());
        assert!(k600_check_xcnf(formula, &model, 3, 0).is_err());
        assert!(k600_check_xcnf("p cnf 3 3\n1 -2 0\n3 0\nx -1 2 3\n", &model, 2, 1).is_err());

    fn k600_positive_witness_checks_exported_objects_and_relation_row() {
        let (bytes, manifest_row) = construct(
            0,
            2,
            17,
            Instant::now() + Duration::from_secs(60),
        .unwrap();
        let base = from_record(&manifest_row, &bytes).unwrap();
        let kc = &base.curve;
        let fb = &base.factor_base;
        let indices = [0usize, 2, 4, 6, 8, 10];
        let mut partial_sums = Vec::new();
            sum = kc.add(&sum, &fb.points[index]);
            assert!(matches!(sum, BinaryPoint::Affine { .. }));
            partial_sums.push(planted_point_json(&sum));
        let row = checked_primary_relation_row(kc, fb, &indices, &sum).unwrap();
            serde_json::to_vec(&row.iter().map(ToString::to_string).collect::<Vec<_>>()).unwrap();
        let positive = json!({
            "points":indices.iter().map(|&i| planted_point_json(&fb.points[i])).collect::<Vec<_>>(),
            "partial_sums":partial_sums,
        assert_eq!(
            verify_k600_positive_witness(kc, fb, &sum, &positive).unwrap(),
            indices
        let mut bad = positive.clone();
        bad["points"][0] = point_json(words(&fb.points[0]));
        assert!(verify_k600_positive_witness(kc, fb, &sum, &bad).is_err());
        bad["partial_sums"][4]["x_decimal"] = json!("0");
        bad["relation_row_decimal_blake3"] = json!("0");
        let mut bad = positive;
        bad["indices"][5] = json!(0);

    fn successor_store_import_checks_index_bytes_replay_and_custody() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
        let root = std::env::temp_dir().join(format!("n83-store-v3-{nonce}"));
        let bundle = root.join("bundle");
        let remote = root.join("remote");
        fs::create_dir_all(&bundle).unwrap();
        fs::create_dir_all(&remote).unwrap();
        let (plain, mut row) = construct(
            SEEDS[0],
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&plain).unwrap();
        let compressed = encoder.finish().unwrap();
        let object = root.join("base.jsonl.gz");
        fs::write(&object, &compressed).unwrap();
        let cfg = StoreConfig {
            index_path: root.join("index.json"),
            cache_dir: root.join("cache"),
            remote: Some(RemoteSpec::Dir(remote.clone())),
            endpoint: None,
            aws: "aws".into(),
        let mut store = Store::open(cfg).unwrap();
        let name = "factor-bases/icv1-f2m83-tm6151469093347-debefd74/control-k2.jsonl.gz";
        let pushed = store
            .push(
                name,
                &object,
                PushMeta {
                    description: "N83 native store control".into(),
                    provenance: "release test".into(),
                    no_cache: true,
                    ..Default::default()
            .unwrap();
        assert!(pushed.uploaded);
        let pulled = store.ensure(name).unwrap();
        assert!(pulled.fetched);
        assert_eq!(pulled.sha256, pushed.sha256);
        row["closure"] = json!("signed_frobenius");
        row["store_name"] = json!(name);
        row["compressed_sha256"] = json!(pushed.sha256);
        row["compressed_blake3"] = json!(blake3::hash(&compressed).to_hex().to_string());
        row["plain_blake3"] = json!(blake3::hash(&plain).to_hex().to_string());
        row["bytes"] = json!(compressed.len());
        row["plain_bytes"] = json!(plain.len());
        let remote_uri = format!("file://{}", remote.display());
        write_new_json(
            &bundle.join("manifest.json"),
            &json!({
                "schema":"n83.factor-base-panel/v3-artifact-store",
                "status":"completed_factor_base_object","study":STUDY,
                "curve_id":"icv1-f2m83-tm6151469093347-debefd74",
                "source_commit":"a".repeat(40),"remote":remote_uri,"base":row,
        let manifest_sha256 = sha256_hex(&fs::read(bundle.join("manifest.json")).unwrap());
            &bundle.join("replay.json"),
                "schema":"n83.factor-base-replay/v3-artifact-store","status":"PASS",
                "manifest_sha256":manifest_sha256,"store_name":name,
                "point_set_blake3":row["point_set_blake3"],
                "points_checked":332,"representatives_checked":2,
        let storage = json!({
            "schema":"n83.factor-base-storage/v3-artifact-store","status":"PASS",
            "manifest_sha256":manifest_sha256,"remote":remote_uri,
            "store_name":name,"sha256":pushed.sha256,"bytes":compressed.len(),
            "fresh_download_sha256":pushed.sha256,"fresh_download_bytes":compressed.len(),
        write_new_json(&bundle.join("storage.json"), &storage).unwrap();
        let (imported, checked) = load_store_primary_base(&bundle, &store).unwrap();
        assert_eq!(imported.factor_base.points.len(), 332);
        assert_eq!(imported.factor_base.signed_orbits.len(), 2);
        assert_eq!(checked["point_set_blake3"], row["point_set_blake3"]);
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/koblitz_n83_factor_base_sweep_20261008/pilot-01");
        let config = StoreRunConfig {
            store: &store,
            fixture_root: &fixture_root,
            source_commit: "a".repeat(40),
            source_attestation_mode: "test_fixture",
        let trial = run_with_config(
            &bundle,
            7200,
            &root.join("trial"),
        assert_eq!(trial["input_kind"], "artifact_store_v3");
        assert_eq!(trial["schema"], "n83.primary-store-trial/v1");
        assert_eq!(trial["store_name"], name);
        assert_eq!(trial["store_manifest_sha256"], manifest_sha256);
        assert!(trial["panel_manifest_blake3"].is_null());
        assert!(trial["total_index_calculus_runtime_ms"].is_null());
        let one_query = run_with_config(
            1,
            60,
            &root.join("one-query"),
        assert_eq!(one_query["schema"], "n83.primary-store-trial/v1");
        assert_eq!(one_query["store_name"], name);
        assert_eq!(one_query["report"]["trials"], 1);
        assert_eq!(one_query["solver_stage_executed"], true);
        assert!(one_query["total_index_calculus_runtime_ms"].is_null());
        let chain_config = ChainRunConfig {
            policy: "public_x_hash",
            seed: SEEDS[0],
            limits: ChainS3Limits {
                max_variables: 10_000,
                max_domain_clauses: 100_000,
                max_models: 1,
                conflict_budget: 1_000,
            },
            memory_limit_bytes: 256 * 1024 * 1024,
        let chain_preflight = run_with_config(
            5,
            "orbit-chain-s3",
            &root.join("chain-preflight"),
        assert_eq!(chain_preflight["schema"], "n83.primary-store-chain/v1");
        assert_eq!(chain_preflight["store_name"], name);
        assert_eq!(chain_preflight["strategy"], "orbit-chain-s3");
        assert_eq!(chain_preflight["solver_stage_executed"], false);
        assert!(chain_preflight["total_index_calculus_runtime_ms"].is_null());
        let wrong_chain_config = ChainRunConfig {
            policy: "public_x_sequential",
            ..chain_config
        let wrong_chain_dir = root.join("wrong-chain-policy");
        assert!(seal_store_run_error(
            &wrong_chain_dir,
            run_with_config(
                &bundle,
                2,
                "orbit-chain-s3",
                0,
                7200,
                &wrong_chain_dir,
                Some(&wrong_chain_config),
                Some(&config),
            ),
        .unwrap_err()
        .to_string()
        .contains("selection differs from verified base"));
        let rejected: Value =
            serde_json::from_slice(&fs::read(wrong_chain_dir.join("failure.json")).unwrap())
                .unwrap();
        assert_eq!(rejected["schema"], "n83.primary-store-command-error/v1");
        assert_eq!(rejected["status"], "COMMAND_ERROR");
        assert!(rejected["total_index_calculus_runtime_ms"].is_null());
        let wrong_columns_dir = root.join("wrong-columns");
            &wrong_columns_dir,
                3,
                "enumerate",
                &wrong_columns_dir,
                None,
        .is_err());
        assert!(wrong_columns_dir.join("failure.json").is_file());
        let mut bad = storage.clone();
        bad["fresh_download_sha256"] = json!("0");
        fs::write(
            bundle.join("storage.json"),
            serde_json::to_vec(&bad).unwrap(),
        assert!(load_store_primary_base(&bundle, &store).is_err());
            serde_json::to_vec(&storage).unwrap(),
        let mut old: Value =
            serde_json::from_slice(&fs::read(bundle.join("manifest.json")).unwrap()).unwrap();
        old["schema"] = json!("n83.factor-base-panel/v1");
            bundle.join("manifest.json"),
            serde_json::to_vec(&old).unwrap(),
        fs::remove_dir_all(root).unwrap();

    fn packed_model_uses_low_bit_first_and_zero_padding() {
        let model = [true, false, true, false, false, false, false, false, true];
        assert_eq!(pack_model_bits(&model), [0b0000_0101, 0b0000_0001]);
        assert!(pack_model_bits(&[]).is_empty());

    fn cgroup_v2_membership_resolves_container_and_native_scope() {
            cgroup_v2_root_from_membership("0::/\n").unwrap(),
            Path::new("/sys/fs/cgroup")
            cgroup_v2_root_from_membership(
                "8:cpu:/legacy\n0::/user.slice/user-1000.slice/app.slice/run.scope\n"
            .unwrap(),
            Path::new("/sys/fs/cgroup/user.slice/user-1000.slice/app.slice/run.scope")
        assert!(cgroup_v2_root_from_membership("8:cpu:/legacy\n").is_err());
        assert!(cgroup_v2_root_from_membership("0::/../escape\n").is_err());

    fn selector_and_phase_modes_preserve_small_chain_model_and_group_lift() {
        let kc = KoblitzCurve::new(0, 7).unwrap();
        let p = kc.generator().clone();
        let q = kc.mul(&p, &BigUint::from(2u32));
        let three = kc.mul(&p, &BigUint::from(3u32));
        let target = kc.mul(&p, &BigUint::from(5u32));
        let x = |point: &BinaryPoint| match point {
            BinaryPoint::Affine { x, .. } => x.clone(),
            BinaryPoint::Infinity => panic!("small control point is infinity"),
        let (xp, xq) = (x(&p), x(&q));
        assert_ne!(xp, xq);
        let expected = [
            xp.clone(),
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
                2_000,
            enc.constrain_frobenius_orbit_summands(&representatives, 2_000, 10_000)
            for (block, value) in expected.iter().enumerate() {
                enc.pin_x_block(block, value).unwrap();
            let handles = enc.orbit_choice_wires().unwrap().to_vec();
            for (slot, value) in expected[..4].iter().enumerate() {
                let index = representatives.binary_search(&value.to_biguint()).unwrap();
                if mode.selector() {
                    assert!(enc
                        .solver
                        .add_clause(vec![handles[slot].selectors[index] as i32]));
                if mode.phase() {
                    for &wire in &handles[slot].phase {
                        assert!(enc.solver.add_clause(vec![-(wire as i32)]));
            if contradict {
                let first_index = representatives.binary_search(&xp.to_biguint()).unwrap();
                let opposite = if mode.selector() {
                    -(handles[0].selectors[first_index] as i32)
                    handles[0].phase[0] as i32
                enc.solver.add_clause(vec![opposite]);
            let progress = observed.then(|| Arc::new(SolverProgress::default()));
            if let Some(progress) = &progress {
                enc.solver.set_progress_observer(Arc::clone(progress));
                assert_eq!(answer, SolveResult::Unsat);
                return (answer, None);
            assert_eq!(answer, SolveResult::Sat);
            assert!(enc.verify_model());
            assert_eq!(enc.decode_x_blocks(), expected);
            let mut sum = BinaryPoint::Infinity;
            for value in enc.decode_summands() {
                let point = if value == xp {
                    &p
                } else if value == xq {
                    &q
                    panic!("unknown small summand x")
                sum = kc.add(&sum, point);
            assert_eq!(sum, target);
            if let Some(progress) = progress {
                assert_eq!(progress.snapshot().xor_passes, enc.solver.stats.xor_passes);
            (answer, Some(enc.solver.model()))
        for mode in [
            OrbitChoicePins::SelectorAndPhase,
            OrbitChoicePins::SelectorOnly,
            OrbitChoicePins::PhaseOnly,
        ] {
            for contradict in [false, true] {
                assert_eq!(run(mode, false, contradict), run(mode, true, contradict));

    fn supervised_source_snapshot_rejects_a_changed_compiled_file() {
        let snapshot = std::env::temp_dir().join(format!("n83-probe-source-{nonce}"));
        let checkout = Path::new(env!("CARGO_MANIFEST_DIR"));
        for relative in [
            let destination = snapshot.join(relative);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::copy(checkout.join(relative), destination).unwrap();
        assert!(checked_primary_probe_source(&snapshot).is_ok());
            snapshot.join("research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs"),
            b"changed source",
        assert!(checked_primary_probe_source(&snapshot).is_err());
        fs::remove_dir_all(snapshot).unwrap();

    fn small_primary_export_maps_to_generic_orbits_and_zero_trial_preflight() {
        let (bytes, row) = construct(
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
                point,
                &base.factor_base.points[base.factor_base.signed_orbits[column][index % 166]]
        let indices = [0, 2, 165, 168];
        let target = indices.iter().fold(BinaryPoint::Infinity, |sum, &index| {
            kc.add(&sum, &fb.points[index])
        let relation = checked_primary_relation_row(kc, fb, &indices, &target).unwrap();
        let xs: Vec<_> = indices
            .map(|&index| match &fb.points[index] {
                BinaryPoint::Affine { x, .. } => x.clone(),
                BinaryPoint::Infinity => panic!("primary point is infinity"),
        let lifted = full_d18_four_x_lift(kc, fb, &xs, &target)
            .expect("four stored x coordinates must lift to the original group target");
        assert_eq!(lifted.1, relation);
        let (lifted_indices, lifted_row) = checked_four_x_lift(kc, fb, &xs, &target)
            .expect("constructed four-point target has a stored group lift");
        assert_eq!(lifted_row, relation);
            lifted_indices
                .fold(BinaryPoint::Infinity, |sum, &index| {
                    kc.add(&sum, &fb.points[index])
                }),
            target
        let lambda_82 = kc.lambda.modpow(&BigUint::from(82u32), &kc.subgroup_order);
            relation[0],
            (BigUint::one() + &kc.lambda + &kc.subgroup_order - lambda_82) % &kc.subgroup_order
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
            let row = checked_primary_relation_row(kc, fb, &indices, &target).unwrap();
            assert_eq!(rank.insert(row.clone()), column + 1);
            assert_eq!(rank.insert(row), column + 1);
        let mut opts = KoblitzIcOptions::default();
        opts.strategy = DecompositionStrategy::Enumerate;
        opts.max_trials = 0;
        opts.allow_direct_relation = false;
        // The default pair table packs keys into u64 and cannot represent
        // this degree-83 base. The preflight itself makes no query.
        let report = koblitz_index_calculus_dlp_with_factor_base(
            &base.curve,
            base.curve.generator(),
            &base.factor_base,
            &opts,
        assert_eq!(report.factor_base_size, 332);
        assert_eq!(report.orbit_count, 2);
        assert_eq!(report.trials, 0);
        assert!(report.log.is_none());

        let mut entries: Vec<Value> = std::str::from_utf8(&bytes)
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
        let mut wrong_header = entries.clone();
        wrong_header[0]["seed"] = json!(18);
        let bad_header = wrong_header
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(from_record(&row, bad_header.as_bytes()).is_err());
        entries[1]["coefficient"] = json!("0");
        let bad = entries
        assert!(from_record(&row, bad.as_bytes()).is_err());

    fn wide_enumerator_matches_generic_witness_order_on_primary_curve() {
        let index: HashMap<_, _> = fb
            .map(|(i, p)| (point_key(p), i))
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
            if m == 1 {
                let i = *index.get(&point_key(target))?;
                return (i >= start).then(|| vec![i]);
            for i in start..fb.points.len() {
                let rest = kc.add(target, &point_neg(&fb.points[i]));
                if let Some(mut tail) = reference(kc, fb, index, &rest, m - 1, i) {
                    let mut result = vec![i];
                    result.append(&mut tail);
                    return Some(result);
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
        for (target, m) in cases {
                enumerate_decompose(kc, fb, &index, &target, m),
                reference(kc, fb, &index, &target, m, 0),
                "wide witness at m={m}"

    fn public_target_and_post_solver_validation_bind_the_same_fixture() {
        let suffix = SystemTime::now()
        let root = std::env::temp_dir().join(format!(
            "n83-primary-fixture-{}-{suffix}",
            std::process::id()
        fs::create_dir(&root).unwrap();
        let target = base.curve.generator().clone();
        let second = base.curve.mul(&target, &BigUint::from(2u8));
        let corpus = json!({
            "schema":"n83.public-probe-corpus/v1", "study":STUDY,
            "targets":[
                {"a":0,"fixture":0,"point":point_json(words(&target))},
                {"a":0,"fixture":1,"point":point_json(words(&second))}
            ]
        write_new_json(&root.join("probe-corpus.json"), &corpus).unwrap();
        let (selected, hash) = public_target(&root, &base.curve).unwrap();
        assert_eq!(selected, target);
        let (selected_second, second_hash) = public_target_fixture(&root, &base.curve, 1).unwrap();
        assert_eq!(selected_second, second);
        assert_eq!(second_hash, hash);
        assert!(public_target_fixture(&root, &base.curve, 2).is_err());
        let validation = json!({
            "schema":"n83.public-probe-validation/v1",
            "corpus_canonical_json_blake3":hash,
            "oracle_receives_known_answer_scalars":false,
            "validation":[{"a":0,"fixture":0,"known_answer_scalar":"1"}]
        write_new_json(&root.join("probe-validation.json"), &validation).unwrap();
        assert!(candidate_matches_validation(&root, &hash, &BigUint::one()).unwrap());
        assert!(!candidate_matches_validation(&root, &hash, &BigUint::from(2u8)).unwrap());
        let duplicate = json!({
                {"a":0,"fixture":0,"point":point_json(words(&target))}
            root.join("probe-corpus.json"),
            serde_json::to_vec(&duplicate).unwrap(),
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
                64,
                SEEDS[0],
                    max_variables: 0,
                    max_domain_clauses: 1,
                    max_models: 1,
                    conflict_budget: 1,
                256,
                &invalid_caps,
                orbit_domain,
            .unwrap_err();
            assert!(error.to_string().contains("outside supported domain"));
            assert!(!invalid_caps.exists());

    fn v2_primary_selection_requires_replay_and_s3_receipts_before_object_import() {
        let root = std::env::temp_dir().join(format!("n83-v2-import-gate-{suffix}"));
        let digest = "a".repeat(64);
        let object = format!("objects/{digest}.jsonl.gz");
        let uri = format!(
            "s3://crypto-autoresearcher/factor-bases/icv1/etc/{STUDY}/v2-size-frontier/a0/{object}"
        let manifest = json!({
            "schema":"n83.factor-base-panel/v2-size-frontier", "study":STUDY,
            "status":"completed_factor_base_object", "expected_base_count":1,
            "completed_base_count":1, "size_design_blake3":v2_design_hash(),
            "bases":[{"a":0,"policy":"public_x_hash","seed":SEEDS[0],
                "columns":1182,"object":object,"s3_uri":uri,
                "compressed_blake3":digest,"plain_blake3":"b".repeat(64)}],
        write_new_json(&root.join("manifest.json"), &manifest).unwrap();
        let manifest_hash = blake3::hash(&fs::read(root.join("manifest.json")).unwrap())
            .to_hex()
            &root.join("replay.json"),
                "schema":"n83.factor-base-replay/v2-size-frontier", "status":"PASS",
                "panel_manifest_blake3":manifest_hash,
                "checks":[{"object":object,"points_checked":166*1182,
                    "representatives_checked":1182,"status":"PASS"}],
            "schema":"n83.factor-base-storage/v2-size-frontier", "status":"PASS",
            "bucket":"crypto-autoresearcher",
            "prefix":format!("{PREFIX}/{STUDY}/{V2_SUBDIR}"),
            "manifest_blake3":manifest_hash,
            "objects":[{"s3_uri":uri,"compressed_blake3":digest,
                "downloaded_hash_matches":true}],
        write_new_json(&root.join("upload-receipt.json"), &storage).unwrap();
        let error = load_panel_selected(&root, 1182, "public_x_hash", SEEDS[0])
            .err()
        assert!(error.to_string().contains("No such file"), "{error}");
        let mut bad = storage;
        bad["objects"][0]["downloaded_hash_matches"] = json!(false);
            root.join("upload-receipt.json"),
        assert!(error.to_string().contains("S3 round-trip"), "{error}");






















































































































