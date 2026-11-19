//! Publish the accepted pair-table incumbent's historical native source/build
//! without its old point-bearing execution. The archive is data-only.
use super::{ordinary_build, sat_control::native};
use crypto_lib::cryptanalysis::{
    prepared_control_archive::verify_tar_inventory,
    prepared_sat_control::{canonical_sha, sha256},
};
use flate2::read::MultiGzDecoder;
use native::{load, read, require, save};
use serde_json::{json, Map, Value};
use std::{fs, io::Read, path::Path};

const QUESTION: &str = "fresh-paired-n17-incumbent-source-publication-v1";
const STATUS: &str = "PASS_DATA_ONLY_INCUMBENT_TARGET_FREE_SOURCE_PUBLICATION";
const CURVE_ID: &str = "EC1N17Ckb1hbbe2b5b6b1e6";
const SOURCE_SHA: &str = "240b8daa0478aacb1f6fc248de9fd5ed9b0773c59d3ae869ced0bab8be438378";
const WORKER_SHA: &str = "b3d8afacc746a34cf15562452dbec3d75deadbb9faf16051e97ad8fa07cbbd66";
const BUILD_POLICY_SHA: &str = "a91e39eb92505d0aa962f1c99b02c0e27d09a4d6d5061dada252dfdddff8252c";
const DEPENDENCIES_SHA: &str = "b4890137e51d57bd0fb9c5293e774968f8af4f62a67fe3b7e3922246104931ef";
const MATERIAL_FILES: [&str; 5] = [
    "worker",
    "build-record.json",
    "build.log",
    "dependency-manifest.json",
    "source-preparation.json",
];
const SIDECARS: [&str; 4] = [
    "build-record.json",
    "build.log",
    "dependency-manifest.json",
    "source-preparation.json",
];

fn desc(bytes: &[u8]) -> Value {
    json!({"bytes":bytes.len(),"sha256":sha256(bytes)})
}
fn outside(root: &Path, publication: &Path, out: &Path) -> Result<(), String> {
    let parent = out
        .parent()
        .ok_or("incumbent source output lacks parent")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    require(
        !parent.starts_with(root) && !parent.starts_with(publication),
        "incumbent source output must stay outside material and publication",
    )
}
fn pins(
    record: &Value,
    prep: &Value,
    deps: &Value,
    source: &Map<String, Value>,
    worker_sha: &str,
    log_sha: &str,
    record_sha: &str,
) -> Result<Value, String> {
    let files = record["source_manifest"]
        .as_object()
        .ok_or("incumbent source manifest missing")?;
    require(
        files.len() == 454
            && canonical_sha(&json!(files))? == SOURCE_SHA
            && record["accepted_source_manifest_sha256"] == SOURCE_SHA
            && record["worker_sha256"] == WORKER_SHA
            && worker_sha == WORKER_SHA
            && record["build_policy_sha256"] == BUILD_POLICY_SHA
            && canonical_sha(&record["build_policy"])? == BUILD_POLICY_SHA
            && record["dependency_manifest_sha256"] == DEPENDENCIES_SHA
            && canonical_sha(deps)? == DEPENDENCIES_SHA
            && record["prepared_source_receipt"] == *prep
            && record["build_policy"]["prepared_source_receipt_sha256"] == canonical_sha(prep)?
            && prep["source_manifest_sha256"] == SOURCE_SHA
            && prep["reference"] == "pairinv"
            && record["build_policy"]["target"] == "aarch64-apple-darwin"
            && record["build_policy"]["flags"]["release"] == true
            && record["build_policy"]["flags"]["incremental"] == false
            && record["build_log_sha256"] == log_sha,
        "incumbent historical source/build pins differ",
    )?;
    require(
        source.len() == files.len()
            && files.iter().all(|(name, hash)| {
                source
                    .get(name)
                    .is_some_and(|entry| entry["sha256"] == *hash)
            }),
        "incumbent material differs from qualified 454-file source",
    )?;
    Ok(
        json!({"source_manifest_sha256":SOURCE_SHA,"worker_sha256":WORKER_SHA,
        "build_policy_sha256":BUILD_POLICY_SHA,"dependency_manifest_sha256":DEPENDENCIES_SHA,
        "prepared_source_receipt_sha256":canonical_sha(prep)?,
        "build_record_sha256":record_sha,"source_files":files.len()}),
    )
}
fn material(root: &Path) -> Result<Value, String> {
    let mut entries = fs::read_dir(root)
        .map_err(|e| e.to_string())?
        .map(|entry| {
            entry
                .map_err(|e| e.to_string())?
                .file_name()
                .into_string()
                .map_err(|_| "incumbent material name is not UTF-8".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    let mut expected: Vec<&str> = MATERIAL_FILES.into_iter().chain(["source"]).collect();
    expected.sort();
    require(
        entries == expected,
        "incumbent source material has extra or missing files",
    )?;
    let record = load(&root.join("build-record.json"))?;
    let prep = load(&root.join("source-preparation.json"))?;
    let deps = load(&root.join("dependency-manifest.json"))?;
    let source = native::inventory(&root.join("source"))?;
    let actual = source
        .as_object()
        .ok_or("incumbent source inventory missing")?;
    let worker = read(&root.join("worker"), 128 * 1024 * 1024)?;
    // A previous local build of the accepted source, not the archived Linux
    // executable. Never execute it while publishing or replaying source.
    require(
        worker.len() > 32
            && worker[..4] == [0xcf, 0xfa, 0xed, 0xfe]
            && worker[4..8] == [0x0c, 0x00, 0x00, 0x01],
        "incumbent worker is not the retained macOS ARM64 Mach-O",
    )?;
    pins(
        &record,
        &prep,
        &deps,
        actual,
        &sha256(&worker),
        &sha256(&read(&root.join("build.log"), 16 * 1024 * 1024)?),
        &sha256(&read(&root.join("build-record.json"), 16 * 1024 * 1024)?),
    )
}
fn claims(header: &Value) -> Result<(), String> {
    require(
        header["schema_version"] == 1
            && header["question"] == QUESTION
            && header["role"] == "incumbent"
            && header["curve_id"] == CURVE_ID
            && header["target_config_archived"] == false
            && header["target_card_created"] == false
            && header["target_worker_calls"] == 0
            && header["source_bound_execution_admitted"] == false
            && header["online_speedup"].is_null(),
        "incumbent source publication overstates target-free custody",
    )
}
fn verify(publication: &Path, seal: &str) -> Result<Value, String> {
    let header = load(&publication.join("SOURCE_PUBLICATION.json"))?;
    claims(&header)?;
    require(
        canonical_sha(&header)? == seal
            && load(&publication.join("SOURCE_SEAL.json"))?["source_registration_sha256"] == seal,
        "incumbent external source seal differs",
    )?;
    let files = header["immutable_files"]
        .as_object()
        .ok_or("incumbent archive inventory missing")?;
    let source: Map<String, Value> = files
        .iter()
        .filter_map(|(name, value)| {
            name.strip_prefix("source/")
                .map(|relative| (relative.to_string(), value.clone()))
        })
        .collect();
    require(
        files.len() == source.len() + MATERIAL_FILES.len()
            && MATERIAL_FILES.iter().all(|name| files.contains_key(*name))
            && files["worker"]["sha256"] == WORKER_SHA,
        "incumbent archive contains extra or missing material",
    )?;
    for name in SIDECARS {
        require(
            files[name] == desc(&read(&publication.join(name), 16 * 1024 * 1024)?),
            "incumbent source sidecar differs from archived inventory",
        )?;
    }
    let record = load(&publication.join("build-record.json"))?;
    let prep = load(&publication.join("source-preparation.json"))?;
    let deps = load(&publication.join("dependency-manifest.json"))?;
    let checked = pins(
        &record,
        &prep,
        &deps,
        &source,
        WORKER_SHA,
        &sha256(&read(&publication.join("build.log"), 16 * 1024 * 1024)?),
        &sha256(&read(
            &publication.join("build-record.json"),
            16 * 1024 * 1024,
        )?),
    )?;
    require(
        header["material"] == checked,
        "incumbent source publication inventory differs",
    )?;
    let archive = read(&publication.join("capsule.tar.gz"), 96 * 1024 * 1024)?;
    require(
        header["archive"] == desc(&archive),
        "incumbent source archive bytes differ",
    )?;
    let mut tar = Vec::new();
    MultiGzDecoder::new(archive.as_slice())
        .take(512 * 1024 * 1024 + 1)
        .read_to_end(&mut tar)
        .map_err(|e| e.to_string())?;
    require(
        tar.len() <= 512 * 1024 * 1024,
        "incumbent source archive exceeds decompression bound",
    )?;
    let archive_files: Map<String, Value> = files
        .iter()
        .map(|(name, value)| (format!("immutable/{name}"), value.clone()))
        .collect();
    let count = verify_tar_inventory(&tar, &json!(archive_files))?;
    Ok(
        json!({"schema_version":1,"status":STATUS,"role":"incumbent",
        "curve_id":CURVE_ID,"source_registration_sha256":seal,
        "source_manifest_sha256":SOURCE_SHA,"worker_sha256":WORKER_SHA,
        "archive_sha256":sha256(&archive),"archive_regular_files":count,
        "target_config_archived":false,"target_card_created":false,
        "archived_binaries_executed":0,"target_worker_calls":0,
        "source_bound_execution_admitted":false,"online_speedup":null}),
    )
}
pub(super) fn publish(root: &Path, publication: &Path, out: &Path) -> Result<String, String> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let material_receipt = material(&root)?;
    let parent = publication
        .parent()
        .ok_or("incumbent publication lacks parent")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    require(
        !parent.starts_with(&root),
        "incumbent source publication must stay outside material",
    )?;
    fs::create_dir(publication).map_err(|e| e.to_string())?;
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    outside(&root, &publication, out)?;
    let immutable = publication.join("immutable");
    fs::create_dir(&immutable).map_err(|e| e.to_string())?;
    native::copy_tree(&root.join("source"), &immutable.join("source"))?;
    for name in MATERIAL_FILES {
        native::create(
            &immutable.join(name),
            &read(&root.join(name), 128 * 1024 * 1024)?,
        )?;
    }
    let inventory = native::inventory(&immutable)?;
    require(
        material(&immutable)? == material_receipt,
        "incumbent source changed while copied",
    )?;
    for name in SIDECARS {
        native::create(
            &publication.join(name),
            &read(&root.join(name), 16 * 1024 * 1024)?,
        )?;
    }
    let files: Map<String, Value> = inventory
        .as_object()
        .ok_or("incumbent immutable inventory missing")?
        .iter()
        .map(|(name, value)| (format!("immutable/{name}"), value.clone()))
        .collect();
    ordinary_build::archive(&publication, &publication.join("capsule.tar.gz"), &files)?;
    let archive = read(&publication.join("capsule.tar.gz"), 96 * 1024 * 1024)?;
    let header = json!({"schema_version":1,"question":QUESTION,"role":"incumbent",
        "curve_id":CURVE_ID,"source_material_root":root,
        "material":material_receipt,"immutable_files":inventory,
        "archive":desc(&archive),"target_config_archived":false,
        "target_card_created":false,"target_worker_calls":0,
        "source_bound_execution_admitted":false,"online_speedup":null});
    claims(&header)?;
    save(&publication.join("SOURCE_PUBLICATION.json"), &header)?;
    let seal = canonical_sha(&header)?;
    save(
        &publication.join("SOURCE_SEAL.json"),
        &json!({"source_registration_sha256":seal}),
    )?;
    replay(&publication, &seal, out)
}
pub(super) fn replay(publication: &Path, seal: &str, out: &Path) -> Result<String, String> {
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    outside(&publication, &publication, out)?;
    let result = verify(&publication, seal)?;
    save(out, &result)?;
    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}
