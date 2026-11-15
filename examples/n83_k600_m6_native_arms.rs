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

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 || args[1] != "stage" {
        return Err("usage: stage FROZEN_METADATA_DIR NEW_OUTPUT_DIR".into());
    }
    let output = PathBuf::from(&args[3]);
    let result = stage(Path::new(&args[2]), &output);
    if let Err(error) = &result {
        if output.is_dir() && !output.join("staging.json").exists() {
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
}
