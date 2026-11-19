//! Build and publish the strong-rho reference from the already accepted,
//! target-free F5 source snapshot. Replay checks the complete source/build
//! archive as data and never executes an archived binary.
use super::{f5_source_custody, ordinary_build, sat_control::native};
use crypto_lib::cryptanalysis::{
    prepared_control_archive::verify_tar_inventory,
    prepared_sat_control::{canonical_sha, sha256},
};
use flate2::read::MultiGzDecoder;
use native::{load, read, require, save};
use serde_json::{json, Map, Value};
use std::{fs, io::Read, path::Path, process::Command};

const QUESTION: &str = "fresh-paired-n17-rho-source-publication-v1";
const STATUS: &str = "PASS_DATA_ONLY_RHO_TARGET_FREE_SOURCE_PUBLICATION";
const CURVE_ID: &str = "EC1N17Ckb1hbbe2b5b6b1e6";
const F5_SOURCE_SEAL: &str = "bd4c63599b107f97e13d26735bde87296f7800b04ac5df4120c981421df320ee";
const F5_ARCHIVE_SHA: &str = "cdcd3372d4dc627aaadc630fdcf7284610f1810f3ec30ff9ff83bc8f63ff2446";
const SOURCE_COMMIT: &str = "bed4864cca41a214e8907264fb3f1d493036b80f";
const SOURCE_SHA: &str = "a34f2e9e61c78ab5ae19c75a3b0649dac6f7ae9dd7adf1c497c05da4eb5bae9f";
const SOURCE_FILES: usize = 6955;
const BUILD_ARGS: [&str; 5] = ["build", "--locked", "--offline", "--release", "--bin"];

fn desc(bytes: &[u8]) -> Value {
    json!({"bytes":bytes.len(),"sha256":sha256(bytes)})
}
fn outside(path: &Path, roots: &[&Path]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or("rho source output has no parent")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    require(
        roots.iter().all(|root| !parent.starts_with(root)),
        "rho source output must stay outside source, build and publication",
    )
}
fn source_subset(files: &Map<String, Value>) -> Map<String, Value> {
    files
        .iter()
        .filter_map(|(name, value)| {
            name.strip_prefix("immutable/source/")
                .map(|relative| (relative.to_string(), value.clone()))
        })
        .collect()
}
fn checked_source(files: &Map<String, Value>) -> Result<(), String> {
    let source = source_subset(files);
    require(
        source.len() == SOURCE_FILES && canonical_sha(&json!(source))? == SOURCE_SHA,
        "rho source differs from the accepted F5 source manifest",
    )
}
fn parent_source(publication: &Path, source: &Path) -> Result<(), String> {
    let replay = f5_source_custody::verify(publication, F5_SOURCE_SEAL)?;
    require(
        replay["source_commit"] == SOURCE_COMMIT
            && replay["source_manifest_sha256"] == SOURCE_SHA
            && replay["archive_sha256"] == F5_ARCHIVE_SHA,
        "rho parent differs from accepted target-free F5 publication",
    )?;
    let header = load(&publication.join("SOURCE_PUBLICATION.json"))?;
    let source_files = source_subset(
        header["immutable_files"]
            .as_object()
            .ok_or("F5 parent immutable manifest missing")?,
    );
    require(
        source_files.len() == SOURCE_FILES && native::inventory(source)? == json!(source_files),
        "rho source tree differs from replayed F5 archive",
    )
}
fn tool(path: &Path) -> Result<(String, String), String> {
    let canonical = path.canonicalize().map_err(|e| e.to_string())?;
    require(canonical.is_file(), "rho build tool is not a regular file")?;
    Ok((
        canonical
            .to_str()
            .ok_or("rho tool path is not UTF-8")?
            .to_string(),
        sha256(&read(&canonical, 128 * 1024 * 1024)?),
    ))
}
fn version(path: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(path)
        .args(args)
        .env_clear()
        .env("PATH", "/opt/homebrew/bin:/usr/bin:/bin")
        .output()
        .map_err(|e| e.to_string())?;
    require(
        output.status.success() && output.stderr.is_empty() && output.stdout.len() <= 4096,
        "rho build tool version failed",
    )?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
fn build(source: &Path, root: &Path, cargo: &Path, rustc: &Path) -> Result<Value, String> {
    fs::create_dir(root).map_err(|e| e.to_string())?;
    let home = root.join("cargo-home");
    let target = root.join("cargo-target");
    fs::create_dir(&home).map_err(|e| e.to_string())?;
    fs::create_dir(&target).map_err(|e| e.to_string())?;
    let (cargo_path, cargo_sha) = tool(cargo)?;
    let (rustc_path, rustc_sha) = tool(rustc)?;
    let cargo_version = version(&cargo_path, &["--version"])?;
    let rustc_version = version(&rustc_path, &["-vV"])?;
    let before = native::inventory(source)?;
    let output = Command::new(&cargo_path)
        .args(BUILD_ARGS)
        .arg("ecbench")
        .current_dir(source)
        .env_clear()
        .env("PATH", "/opt/homebrew/bin:/usr/bin:/bin")
        .env("CARGO_HOME", &home)
        .env("CARGO_TARGET_DIR", &target)
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTC", &rustc_path)
        .output()
        .map_err(|e| e.to_string())?;
    let log = [output.stdout, output.stderr].concat();
    require(
        log.len() <= 16 * 1024 * 1024,
        "rho build log exceeded bound",
    )?;
    native::create(&root.join("build.log"), &log)?;
    require(
        output.status.success(),
        "rho offline build failed; build.log retained",
    )?;
    require(
        native::inventory(source)? == before,
        "rho source changed during offline build",
    )?;
    let binary = read(&target.join("release/ecbench"), 128 * 1024 * 1024)?;
    require(
        binary.len() > 32
            && binary[..4] == [0xcf, 0xfa, 0xed, 0xfe]
            && binary[4..8] == [0x0c, 0x00, 0x00, 0x01],
        "rho build did not produce a macOS ARM64 Mach-O",
    )?;
    let record = json!({"schema_version":1,"question":"rho-offline-build-v1",
        "source_commit":SOURCE_COMMIT,"source_manifest_sha256":SOURCE_SHA,
        "source_files":SOURCE_FILES,"f5_source_seal":F5_SOURCE_SEAL,
        "f5_archive_sha256":F5_ARCHIVE_SHA,
        "cargo_path":cargo_path,"cargo_sha256":cargo_sha,"cargo_version":cargo_version,
        "rustc_path":rustc_path,"rustc_sha256":rustc_sha,"rustc_version":rustc_version,
        "argv":["build","--locked","--offline","--release","--bin","ecbench"],
        "environment":{"PATH":"/opt/homebrew/bin:/usr/bin:/bin",
            "CARGO_HOME":home,"CARGO_TARGET_DIR":target,
            "CARGO_INCREMENTAL":"0","RUSTC":rustc_path},
        "exit_code":0,"build_log":desc(&log),"ecbench":desc(&binary)});
    native::create(&root.join("ecbench"), &binary)?;
    save(&root.join("build-record.json"), &record)?;
    Ok(record)
}
fn claims(header: &Value) -> Result<(), String> {
    require(
        header["schema_version"] == 1
            && header["question"] == QUESTION
            && header["role"] == "rho"
            && header["curve_id"] == CURVE_ID
            && header["source_commit"] == SOURCE_COMMIT
            && header["source_manifest_sha256"] == SOURCE_SHA
            && header["f5_source_seal"] == F5_SOURCE_SEAL
            && header["f5_archive_sha256"] == F5_ARCHIVE_SHA
            && header["target_config_archived"] == false
            && header["target_card_created"] == false
            && header["target_worker_calls"] == 0
            && header["source_bound_execution_admitted"] == false
            && header["online_speedup"].is_null(),
        "rho source publication overstates target-free custody",
    )
}
pub(super) fn publish(
    source: &Path,
    f5_publication: &Path,
    cargo: &Path,
    rustc: &Path,
    build_root: &Path,
    publication: &Path,
    out: &Path,
) -> Result<String, String> {
    let source = source.canonicalize().map_err(|e| e.to_string())?;
    let parent = f5_publication.canonicalize().map_err(|e| e.to_string())?;
    parent_source(&parent, &source)?;
    require(
        !publication.starts_with(&source)
            && !publication.starts_with(&parent)
            && !build_root.starts_with(publication)
            && !build_root.starts_with(&source),
        "rho publication/build roots overlap frozen source",
    )?;
    fs::create_dir(publication).map_err(|e| e.to_string())?;
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    outside(out, &[&source, &publication, build_root])?;
    let immutable = publication.join("immutable");
    fs::create_dir(&immutable).map_err(|e| e.to_string())?;
    native::copy_tree(&source, &immutable.join("source"))?;
    require(
        native::inventory(&source)? == native::inventory(&immutable.join("source"))?,
        "rho source changed while copied",
    )?;
    let record = build(&immutable.join("source"), build_root, cargo, rustc)?;
    fs::create_dir(immutable.join("bin")).map_err(|e| e.to_string())?;
    fs::create_dir(immutable.join("build")).map_err(|e| e.to_string())?;
    for (name, dest) in [
        ("ecbench", "bin/ecbench"),
        ("build.log", "build/build.log"),
        ("build-record.json", "build/build-record.json"),
    ] {
        native::create(
            &immutable.join(dest),
            &read(&build_root.join(name), 128 * 1024 * 1024)?,
        )?;
    }
    let files = native::inventory(&immutable)?;
    let source_map: Map<String, Value> = files
        .as_object()
        .ok_or("rho inventory missing")?
        .iter()
        .map(|(name, value)| (format!("immutable/{name}"), value.clone()))
        .collect();
    checked_source(&source_map)?;
    ordinary_build::archive(
        &publication,
        &publication.join("capsule.tar.gz"),
        &source_map,
    )?;
    let archive = read(&publication.join("capsule.tar.gz"), 96 * 1024 * 1024)?;
    let header = json!({"schema_version":1,"question":QUESTION,"role":"rho",
        "curve_id":CURVE_ID,"source_commit":SOURCE_COMMIT,
        "source_manifest_sha256":SOURCE_SHA,"f5_source_seal":F5_SOURCE_SEAL,
        "f5_archive_sha256":F5_ARCHIVE_SHA,"build_record":record,
        "immutable_files":source_map,
        "archive":{"path":"capsule.tar.gz","bytes":archive.len(),"sha256":sha256(&archive)},
        "target_config_archived":false,"target_card_created":false,
        "target_worker_calls":0,"source_bound_execution_admitted":false,
        "online_speedup":null});
    claims(&header)?;
    save(&publication.join("SOURCE_PUBLICATION.json"), &header)?;
    let seal = canonical_sha(&header)?;
    save(
        &publication.join("SOURCE_SEAL.json"),
        &json!({"source_registration_sha256":seal}),
    )?;
    replay(&publication, &seal, out)
}
fn verify(publication: &Path, seal: &str) -> Result<Value, String> {
    let header = load(&publication.join("SOURCE_PUBLICATION.json"))?;
    claims(&header)?;
    require(
        canonical_sha(&header)? == seal
            && load(&publication.join("SOURCE_SEAL.json"))?["source_registration_sha256"] == seal,
        "rho external source seal differs",
    )?;
    let files = header["immutable_files"]
        .as_object()
        .ok_or("rho inventory missing")?;
    checked_source(files)?;
    require(
        files.len() == SOURCE_FILES + 3
            && files.contains_key("immutable/bin/ecbench")
            && files.contains_key("immutable/build/build.log")
            && files.contains_key("immutable/build/build-record.json"),
        "rho archive contains extra or missing material",
    )?;
    let record = &header["build_record"];
    let binary = &files["immutable/bin/ecbench"];
    let record_bytes = serde_json::to_vec_pretty(record).map_err(|e| e.to_string())?;
    require(
        record["schema_version"] == 1
            && record["question"] == "rho-offline-build-v1"
            && record["source_commit"] == SOURCE_COMMIT
            && record["source_manifest_sha256"] == SOURCE_SHA
            && record["source_files"] == SOURCE_FILES
            && record["f5_source_seal"] == F5_SOURCE_SEAL
            && record["f5_archive_sha256"] == F5_ARCHIVE_SHA
            && record["argv"]
                == json!([
                    "build",
                    "--locked",
                    "--offline",
                    "--release",
                    "--bin",
                    "ecbench"
                ])
            && record["environment"]["PATH"] == "/opt/homebrew/bin:/usr/bin:/bin"
            && record["environment"]["RUSTC"] == record["rustc_path"]
            && record["environment"]["CARGO_HOME"]
                .as_str()
                .is_some_and(|p| Path::new(p).is_absolute())
            && record["environment"]["CARGO_TARGET_DIR"]
                .as_str()
                .is_some_and(|p| Path::new(p).is_absolute())
            && record["environment"]["CARGO_INCREMENTAL"] == "0"
            && record["exit_code"] == 0
            && record["ecbench"] == *binary
            && record["build_log"] == files["immutable/build/build.log"]
            && files["immutable/build/build-record.json"] == desc(&record_bytes),
        "rho build record differs from source and binary inventory",
    )?;
    let archive = read(&publication.join("capsule.tar.gz"), 96 * 1024 * 1024)?;
    require(
        header["archive"]
            == json!({"path":"capsule.tar.gz",
        "bytes":archive.len(),"sha256":sha256(&archive)}),
        "rho source archive bytes differ",
    )?;
    let mut tar = Vec::new();
    MultiGzDecoder::new(archive.as_slice())
        .take(512 * 1024 * 1024 + 1)
        .read_to_end(&mut tar)
        .map_err(|e| e.to_string())?;
    require(
        tar.len() <= 512 * 1024 * 1024,
        "rho source archive exceeds decompression bound",
    )?;
    let count = verify_tar_inventory(&tar, &json!(files))?;
    Ok(json!({"schema_version":1,"status":STATUS,"role":"rho",
        "curve_id":CURVE_ID,"source_registration_sha256":seal,
        "source_manifest_sha256":SOURCE_SHA,"f5_source_seal":F5_SOURCE_SEAL,
        "ecbench_sha256":binary["sha256"],"archive_sha256":sha256(&archive),
        "archive_regular_files":count,"target_config_archived":false,
        "target_card_created":false,"archived_binaries_executed":0,
        "target_worker_calls":0,"source_bound_execution_admitted":false,
        "online_speedup":null}))
}
pub(super) fn replay(publication: &Path, seal: &str, out: &Path) -> Result<String, String> {
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    outside(out, &[&publication])?;
    let result = verify(&publication, seal)?;
    save(out, &result)?;
    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}
