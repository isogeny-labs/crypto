//! Read-only S3 input staging for the frozen K600 six-summand XCNF arms.
//! This component does not export a formula, run a solver, or replay a model.

use crypto_lib::store::{backend::RemoteSpec, Store};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const REMOTE: &str = "s3://setec-astronomy-artifacts/store";
const STORE_NAME: &str = "factor-bases/icv1-f2m83-tm6151469093347-debefd74/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz";
const OBJECT: &str =
    "objects/b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e.jsonl.gz";
const OBJECT_SHA: &str = "3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf";
const POINT_SET: &str = "ac664f5f0c7122c52a08c0a288b543efb709111a99dc2a5cb2bef025ffffee21";
const META: &[(&str, &str, u64)] = &[
    (
        "manifest.json",
        "5bd862987844f709e03beeab9fd265e3d2024011bcadcb82f5af9c72018df6a6",
        54_400,
    ),
    (
        "replay.json",
        "fa5e545c27dd64c32a47ddae05a2a0b3a031545d700897f3e0e34f59abc0d72e",
        11_475,
    ),
    (
        "upload-receipt.json",
        "0eae40ffa0e938470671cf796633c1e47b61ff676df0807e7a7e8592e56bb828",
        18_526,
    ),
    (
        "probe-corpus.json",
        "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd",
        9_475,
    ),
    (
        "probe-validation.json",
        "1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f",
        6_622,
    ),
];

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buf = [0u8; 65_536];
    loop {
        let count = file.read(&mut buf)?;
        if count == 0 {
            break;
        }
        hash.update(&buf[..count]);
    }
    Ok(hex::encode(hash.finalize()))
}

fn check_file(path: &Path, sha: &str, bytes: u64) -> Result<()> {
    if fs::metadata(path)?.len() != bytes || sha256_file(path)? != sha {
        return Err(format!("frozen input differs: {}", path.display()).into());
    }
    Ok(())
}

fn validate_metadata(source: &Path) -> Result<Value> {
    for (name, sha, bytes) in META {
        check_file(&source.join(name), sha, *bytes)?;
    }
    let manifest: Value = serde_json::from_slice(&fs::read(source.join("manifest.json"))?)?;
    let matches: Vec<_> = manifest["bases"]
        .as_array()
        .ok_or("K600 panel bases missing")?
        .iter()
        .filter(|row| {
            row["a"] == 0
                && row["columns"] == 600
                && row["policy"] == "public_x_hash"
                && row["seed"] == 2_026_100_801u64
        })
        .collect();
    if manifest["schema"] != "n83.factor-base-panel/v1"
        || manifest["study"] != "koblitz_n83_factor_base_sweep_20261008"
        || matches.len() != 1
        || matches[0]["points"] != 99_600
        || matches[0]["object"] != OBJECT
        || matches[0]["point_set_blake3"] != POINT_SET
        || matches[0]["compressed_blake3"]
            != "b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e"
    {
        return Err("K600 legacy manifest selection differs".into());
    }
    Ok(manifest)
}

fn clean_source_commit(repo: &Path) -> Result<String> {
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .output()?;
    if !status.status.success() || !status.stdout.is_empty() {
        return Err("source checkout is not clean".into());
    }
    let commit = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()?;
    if !commit.status.success() {
        return Err("source commit unavailable".into());
    }
    let value = String::from_utf8(commit.stdout)?.trim().to_owned();
    if value.len() != 40 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("source commit invalid".into());
    }
    Ok(value)
}

fn write_new_json(path: &Path, value: &Value) -> Result<()> {
    let mut file = File::options().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn stage(source: &Path, output: &Path) -> Result<Value> {
    if !output.is_absolute() || output.exists() {
        return Err("new absolute output directory required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let source = fs::canonicalize(source)?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    if parent.starts_with(&repo) || parent.starts_with(&source) {
        return Err("output must be outside source checkout and panel".into());
    }
    let source_commit = clean_source_commit(&repo)?;
    validate_metadata(&source)?;
    let mut cfg = crypto_lib::store::Config::discover()?;
    if cfg.remote.as_ref() != Some(&RemoteSpec::parse(REMOTE)?) {
        return Err("astronomy store remote differs".into());
    }
    let index_path = cfg.index_path.clone();
    let index_sha = sha256_file(&index_path)?;
    let indexed = Store::open(cfg.clone())?;
    let (name, sha, bytes) = indexed.lookup(STORE_NAME)?;
    if name.as_deref() != Some(STORE_NAME) || sha != OBJECT_SHA || bytes != Some(3_025_798) {
        return Err("K600 store index differs".into());
    }
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    let panel = output.join("panel");
    fs::create_dir_all(panel.join("objects"))?;
    for (name, sha, bytes) in META {
        fs::copy(source.join(name), panel.join(name))?;
        check_file(&panel.join(name), sha, *bytes)?;
    }
    cfg.cache_dir = output.join("fresh-cache");
    let fetched = Store::open(cfg)?.ensure(STORE_NAME)?;
    if !fetched.fetched || fetched.sha256 != OBJECT_SHA || fetched.bytes != 3_025_798 {
        return Err("fresh K600 S3 GET differs".into());
    }
    let object = panel.join(OBJECT);
    fs::copy(&fetched.path, &object)?;
    check_file(&object, OBJECT_SHA, 3_025_798)?;
    validate_metadata(&source)?;
    validate_metadata(&panel)?;
    if sha256_file(&index_path)? != index_sha || clean_source_commit(&repo)? != source_commit {
        return Err("source or store index changed during staging".into());
    }
    let receipt = json!({
        "schema":"n83.k600-m6-native-arm-staging/v1", "status":"PASS_READ_ONLY_STAGE",
        "source_commit":source_commit, "source_metadata_root":source,
        "panel_root":panel, "remote":REMOTE, "store_name":STORE_NAME,
        "store_index_sha256":index_sha, "object_sha256":OBJECT_SHA,
        "object_bytes":3_025_798, "point_set_blake3":POINT_SET,
        "fresh_s3_get":true, "uploaded":false, "solver_executed":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    write_new_json(&output.join("staging.json"), &receipt)?;
    Ok(receipt)
}

fn staged_panel(stage_root: &Path, commit: &str) -> Result<(PathBuf, Value)> {
    let stage_root = fs::canonicalize(stage_root)?;
    let receipt_path = stage_root.join("staging.json");
    let receipt: Value = serde_json::from_slice(&fs::read(&receipt_path)?)?;
    let panel = stage_root.join("panel");
    let source = fs::canonicalize(
        receipt["source_metadata_root"]
            .as_str()
            .ok_or("staging metadata root missing")?,
    )?;
    let expected_source = fs::canonicalize(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/koblitz_n83_factor_base_sweep_20261008/pilot-01"),
    )?;
    if receipt["schema"] != "n83.k600-m6-native-arm-staging/v1"
        || receipt["status"] != "PASS_READ_ONLY_STAGE"
        || receipt["source_commit"] != commit
        || receipt["panel_root"] != panel.to_string_lossy().as_ref()
        || receipt["remote"] != REMOTE
        || receipt["store_name"] != STORE_NAME
        || receipt["object_sha256"] != OBJECT_SHA
        || receipt["object_bytes"] != 3_025_798
        || receipt["point_set_blake3"] != POINT_SET
        || receipt["fresh_s3_get"] != true
        || receipt["uploaded"] != false
        || receipt["solver_executed"] != false
        || receipt["total_index_calculus_runtime_ms"] != Value::Null
        || receipt["selected_best_total_runtime"] != Value::Null
        || source != expected_source
    {
        return Err("staging receipt differs from frozen read-only input".into());
    }
    validate_metadata(&source)?;
    validate_metadata(&panel)?;
    for (name, sha, bytes) in META {
        check_file(&panel.join(name), sha, *bytes)?;
    }
    check_file(&panel.join(OBJECT), OBJECT_SHA, 3_025_798)?;
    let cfg = crypto_lib::store::Config::discover()?;
    if cfg.remote.as_ref() != Some(&RemoteSpec::parse(REMOTE)?)
        || sha256_file(&cfg.index_path)?
            != receipt["store_index_sha256"]
                .as_str()
                .ok_or("staging index digest missing")?
    {
        return Err("store remote or index changed after staging".into());
    }
    Ok((panel, receipt))
}

fn freeze(
    stage_root: &Path,
    exporter: &Path,
    solver: &Path,
    image: &str,
    output: &Path,
) -> Result<Value> {
    if !output.is_absolute() || output.exists() {
        return Err("new absolute lock path required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("lock parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("lock must be outside source checkout".into());
    }
    let commit = clean_source_commit(&repo)?;
    let (panel, receipt) = staged_panel(stage_root, &commit)?;
    let exporter = fs::canonicalize(exporter)?;
    let solver = fs::canonicalize(solver)?;
    for binary in [&exporter, &solver] {
        if !binary.is_file() || fs::metadata(binary)?.len() == 0 {
            return Err(format!("binary missing or empty: {}", binary.display()).into());
        }
    }
    if !image.starts_with("sha256:")
        || image.len() != 71
        || !image[7..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("full Docker image ID required".into());
    }
    let inspected = Command::new("docker")
        .args(["image", "inspect", "--format", "{{.Id}}", image])
        .output()?;
    if !inspected.status.success() || String::from_utf8(inspected.stdout)?.trim() != image {
        return Err("Docker image ID does not resolve exactly".into());
    }
    let exporter_sha = sha256_file(&exporter)?;
    let solver_sha = sha256_file(&solver)?;
    let supervisor = fs::canonicalize(std::env::current_exe()?)?;
    let supervisor_sha = sha256_file(&supervisor)?;
    let receipt_sha = sha256_file(&fs::canonicalize(stage_root)?.join("staging.json"))?;
    if clean_source_commit(&repo)? != commit {
        return Err("source changed while freezing arms".into());
    }
    let (panel_after, _) = staged_panel(stage_root, &commit)?;
    if panel_after != panel
        || sha256_file(&fs::canonicalize(stage_root)?.join("staging.json"))? != receipt_sha
        || sha256_file(&exporter)? != exporter_sha
        || sha256_file(&solver)? != solver_sha
        || sha256_file(&supervisor)? != supervisor_sha
    {
        return Err("input or binary changed while freezing arms".into());
    }
    let lock = json!({
        "schema":"n83.k600-m6-native-arms-lock/v1", "status":"PREPARED_ONLY",
        "source_commit":commit, "source_root":repo, "staging_receipt_sha256":receipt_sha,
        "panel_root":panel, "store_index_sha256":receipt["store_index_sha256"],
        "remote":REMOTE, "store_name":STORE_NAME, "object_sha256":OBJECT_SHA,
        "object_bytes":3_025_798, "point_set_blake3":POINT_SET,
        "exporter_replayer_binary":exporter, "exporter_replayer_sha256":exporter_sha,
        "solver_binary":solver, "solver_sha256":solver_sha, "image_id":image,
        "supervisor_binary":supervisor, "supervisor_sha256":supervisor_sha,
        "max_variables":125_000, "max_domain_clauses":350_000,
        "max_conflicts":10_000, "max_models":1,
        "memory_limit_bytes":4_294_967_296u64, "swap_limit_bytes":0,
        "cpus":1, "pids_limit":64, "network":"none",
        "positive_executed":false, "natural_executed":false,
        "worker_launch_admitted":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    write_new_json(output, &lock)?;
    Ok(lock)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (output, result, stage_created_output) = match args.get(1).map(String::as_str) {
        Some("stage") if args.len() == 4 => {
            let output = PathBuf::from(&args[3]);
            let stage_created_output = !output.exists();
            let result = stage(Path::new(&args[2]), &output);
            (output, result, stage_created_output)
        }
        Some("freeze") if args.len() == 7 => {
            let output = PathBuf::from(&args[6]);
            let result = freeze(
                Path::new(&args[2]), Path::new(&args[3]), Path::new(&args[4]),
                &args[5], &output,
            );
            (output, result, false)
        }
        _ => return Err("usage: stage FROZEN_METADATA_DIR NEW_OUTPUT_DIR | freeze STAGED_DIR EXPORTER_REPLAYER_BINARY SOLVER_BINARY IMAGE_ID NEW_LOCK_JSON".into()),
    };
    if let Err(error) = &result {
        if stage_created_output && output.is_dir() && !output.join("staging.json").exists() {
            let _ = write_new_json(
                &output.join("failure.json"),
                &json!({
                    "schema":"n83.k600-m6-native-arm-staging-failure/v1",
                    "status":"PRODUCER_FAILURE", "reason":error.to_string(),
                    "selected_best_total_runtime":Value::Null,
                }),
            );
        }
    }
    println!("{}", result?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_k600_metadata_matches_frozen_arm_input() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/koblitz_n83_factor_base_sweep_20261008/pilot-01");
        validate_metadata(&source).unwrap();
    }

    #[test]
    fn changed_metadata_byte_is_rejected() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/koblitz_n83_factor_base_sweep_20261008/pilot-01");
        let original = fs::read(source.join("manifest.json")).unwrap();
        let tmp =
            std::env::temp_dir().join(format!("n83-k600-m6-arm-metadata-{}", std::process::id()));
        fs::create_dir(&tmp).unwrap();
        for (name, _, _) in META {
            fs::copy(source.join(name), tmp.join(name)).unwrap();
        }
        let mut changed = original;
        changed[0] ^= 1;
        fs::write(tmp.join("manifest.json"), changed).unwrap();
        assert!(validate_metadata(&tmp).is_err());
        fs::remove_dir_all(tmp).unwrap();
    }

    #[test]
    fn staged_receipt_cannot_hide_a_changed_object() {
        let source = fs::canonicalize(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("research/koblitz_n83_factor_base_sweep_20261008/pilot-01"),
        )
        .unwrap();
        let root =
            std::env::temp_dir().join(format!("n83-k600-m6-arm-object-{}", std::process::id()));
        let panel = root.join("panel");
        fs::create_dir_all(panel.join("objects")).unwrap();
        for (name, _, _) in META {
            fs::copy(source.join(name), panel.join(name)).unwrap();
        }
        fs::write(panel.join(OBJECT), b"changed").unwrap();
        let commit = "a".repeat(40);
        let receipt = json!({
            "schema":"n83.k600-m6-native-arm-staging/v1", "status":"PASS_READ_ONLY_STAGE",
            "source_commit":commit, "source_metadata_root":source, "panel_root":panel,
            "remote":REMOTE, "store_name":STORE_NAME, "store_index_sha256":"unused",
            "object_sha256":OBJECT_SHA, "object_bytes":3_025_798,
            "point_set_blake3":POINT_SET, "fresh_s3_get":true, "uploaded":false,
            "solver_executed":false, "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null,
        });
        write_new_json(&root.join("staging.json"), &receipt).unwrap();
        let error = staged_panel(&root, &commit).unwrap_err().to_string();
        assert!(error.contains("frozen input differs"), "{error}");
        fs::remove_dir_all(root).unwrap();
    }
}
