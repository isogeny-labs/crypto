//! Read-only S3 input staging for the frozen K600 six-summand XCNF arms.
//! This component does not export a formula, run a solver, or replay a model.

use crypto_lib::store::{backend::RemoteSpec, Store};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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

fn is_linux_arm64_elf(path: &Path) -> Result<bool> {
    let mut header = [0u8; 20];
    File::open(path)?.read_exact(&mut header)?;
    Ok(&header[..4] == b"\x7fELF"
        && header[4] == 2
        && header[5] == 1
        && u16::from_le_bytes([header[18], header[19]]) == 183)
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

fn git_common_dir(repo: &Path) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(repo)
        .output()?;
    if !output.status.success() {
        return Err("Git common directory unavailable".into());
    }
    Ok(fs::canonicalize(String::from_utf8(output.stdout)?.trim())?)
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
    if parent.starts_with(fs::canonicalize(stage_root)?) {
        return Err("lock must be outside staged input".into());
    }
    let exporter = fs::canonicalize(exporter)?;
    let solver = fs::canonicalize(solver)?;
    for binary in [&exporter, &solver] {
        if !binary.is_file() || !is_linux_arm64_elf(binary)? {
            return Err(format!("Linux arm64 ELF binary required: {}", binary.display()).into());
        }
    }
    if !image.starts_with("sha256:")
        || image.len() != 71
        || !image[7..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("full Docker image ID required".into());
    }
    let expected = format!("{image} linux/arm64 {commit}");
    let inspected = docker_short(&["image", "inspect", "--format", "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}", image])?;
    if inspected != expected {
        return Err("Docker image ID, Linux arm64 architecture, or source revision differs".into());
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
        "solver_launch_admitted":false,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    write_new_json(output, &lock)?;
    Ok(lock)
}

fn docker_short(args: &[&str]) -> Result<String> {
    let mut child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let start = Instant::now();
    while child.try_wait()?.is_none() {
        if start.elapsed() >= Duration::from_secs(15) {
            child.kill()?;
            child.wait()?;
            return Err("Docker control command exceeded 15 seconds".into());
        }
        thread::sleep(Duration::from_millis(20));
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "Docker control command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn docker_export_args(
    lock: &Value,
    mode: &str,
    repo: &Path,
    git_common: &Path,
    panel: &Path,
    exporter: &Path,
    output: &Path,
    name: &str,
) -> Vec<String> {
    let mount = |src: &Path, dst: &str, readonly: bool| {
        format!(
            "type=bind,src={},dst={dst}{}",
            src.display(),
            if readonly { ",readonly" } else { "" }
        )
    };
    vec![
        "run".into(),
        "--rm".into(),
        "--pull".into(),
        "never".into(),
        "--name".into(),
        name.into(),
        "--network".into(),
        "none".into(),
        "--memory".into(),
        "4096m".into(),
        "--memory-swap".into(),
        "4096m".into(),
        "--cpus".into(),
        "1".into(),
        "--cpuset-cpus".into(),
        "0".into(),
        "--pids-limit".into(),
        "64".into(),
        "--cap-drop".into(),
        "ALL".into(),
        "--security-opt".into(),
        "no-new-privileges".into(),
        "--read-only".into(),
        "--tmpfs".into(),
        "/tmp:rw,noexec,nosuid,size=64m".into(),
        "--user".into(),
        format!("{}:{}", unsafe { libc::geteuid() }, unsafe {
            libc::getegid()
        }),
        "--mount".into(),
        mount(repo, "/src", true),
        "--mount".into(),
        mount(git_common, &git_common.to_string_lossy(), true),
        "--mount".into(),
        mount(panel, "/base", true),
        "--mount".into(),
        mount(exporter, "/exporter", true),
        "--mount".into(),
        mount(output, "/out", false),
        "--workdir".into(),
        "/src".into(),
        "--env".into(),
        "GIT_OPTIONAL_LOCKS=0".into(),
        lock["image_id"].as_str().unwrap_or_default().into(),
        "/exporter".into(),
        "primary-k600-m6-xcnf".into(),
        mode.into(),
        "/base".into(),
        format!("/out/{mode}.json"),
    ]
}

fn bounded_export(
    command: &mut Command,
    name: &str,
    outer_seconds: u64,
) -> Result<(Option<i32>, bool, u128)> {
    let start = Instant::now();
    let mut child = command.spawn()?;
    let deadline = Duration::from_secs(outer_seconds);
    let (exit, capped) = loop {
        if let Some(status) = child.try_wait()? {
            break (status.code(), false);
        }
        if start.elapsed() >= deadline {
            let killed = docker_short(&["kill", name]);
            let _ = child.kill();
            child.wait()?;
            if killed.is_err() {
                return Err(
                    "CLEANUP_FAILURE: Docker container kill failed after export wall cap".into(),
                );
            }
            break (None, true);
        }
        thread::sleep(Duration::from_millis(100));
    };
    let remaining = docker_short(&["ps", "-aq", "--filter", &format!("name=^{name}$")])
        .map_err(|error| format!("CLEANUP_FAILURE: {error}"))?;
    if !remaining.is_empty() {
        return Err("CLEANUP_FAILURE: Docker export container remains".into());
    }
    Ok((exit, capped, start.elapsed().as_millis()))
}

fn validate_export(receipt: &Value, lock: &Value, mode: &str, xcnf_path: &Path) -> Result<()> {
    let shape = &receipt["shape"];
    let bytes = fs::metadata(xcnf_path)?.len();
    if receipt["schema"] != "n83.k600-m6-xcnf-export/v1"
        || receipt["status"] != "PASS_XCNF_EXPORT_ONLY"
        || receipt["mode"] != mode
        || receipt["curve_id"] != "icv1-f2m83-tm6151469093347-debefd74"
        || receipt["curve_a"] != 0
        || receipt["source_commit"] != lock["source_commit"]
        || receipt["source_attestation_mode"] != "clean_checkout"
        || receipt["binary_sha256"] != lock["exporter_replayer_sha256"]
        || receipt["object"] != OBJECT
        || receipt["object_sha256"] != OBJECT_SHA
        || receipt["point_set_blake3"] != POINT_SET
        || receipt["solver_search_executed"] != false
        || receipt["total_index_calculus_runtime_ms"] != Value::Null
        || receipt["selected_best_total_runtime"] != Value::Null
        || receipt["xcnf_path"] != format!("/out/{mode}.xcnf")
        || receipt["xcnf_bytes"] != bytes
        || bytes > 200_000_000
        || receipt["xcnf_sha256"] != sha256_file(xcnf_path)?
        || shape["variables"]
            .as_u64()
            .filter(|n| *n >= 913 && *n <= 125_000)
            .is_none()
        || shape["clauses"].as_u64().filter(|n| *n > 0).is_none()
        || shape["xor_rows"].as_u64().filter(|n| *n > 0).is_none()
        || shape["summands"] != 6
        || shape["identity_mask"] != 0
        || shape["root_encoding"] != "per_representative"
        || shape["domain_representatives"] != 600
        || shape["positive_x_pins"] != if mode == "positive-pinned" { 913 } else { 0 }
        || shape["summand_ordering"] != "none"
        || (mode == "natural" && (receipt["public_fixture"] != 0 || !receipt["positive"].is_null()))
        || (mode != "natural" && receipt["positive"].is_null())
    {
        return Err(
            "K600 XCNF export differs from frozen mode, source, base, shape, or bytes".into(),
        );
    }
    Ok(())
}

fn preflight_gate(
    arm_lock: &Value,
    preflight_lock_path: &Path,
    preflight_root: &Path,
) -> Result<Vec<String>> {
    let lock_path = fs::canonicalize(preflight_lock_path)?;
    let root = fs::canonicalize(preflight_root)?;
    let outer_path = root.join("outer.json");
    let config_path = root.join("worker-output/run/config.json");
    let summary_path = root.join("worker-output/run/summary.json");
    let preflight: Value = serde_json::from_slice(&fs::read(&lock_path)?)?;
    let outer: Value = serde_json::from_slice(&fs::read(&outer_path)?)?;
    let config: Value = serde_json::from_slice(&fs::read(&config_path)?)?;
    let summary: Value = serde_json::from_slice(&fs::read(&summary_path)?)?;
    let hashes = vec![
        sha256_file(&lock_path)?,
        sha256_file(&outer_path)?,
        sha256_file(&config_path)?,
        sha256_file(&summary_path)?,
    ];
    if preflight["schema"] != "n83.k600-m6-native-preflight-lock/v1"
        || preflight["source_commit"] != arm_lock["source_commit"]
        || preflight["image_id"] != arm_lock["image_id"]
        || preflight["object_sha256"] != OBJECT_SHA
        || preflight["point_set_blake3"] != POINT_SET
        || preflight["store_remote"] != REMOTE
        || preflight["store_name"] != STORE_NAME
        || preflight["max_trials"] != 0
        || preflight["max_variables"] != 125_000
        || preflight["max_domain_clauses"] != 350_000
        || preflight["max_models"] != 1
        || preflight["conflict_budget"] != 10_000
        || outer["schema"] != "n83.k600-m6-native-outer/v1"
        || outer["status"] != "PREFLIGHT_ONLY"
        || outer["worker_exit_code"] != 0
        || outer["timed_out"] != false
        || !outer["failure_reasons"]
            .as_array()
            .is_some_and(Vec::is_empty)
        || !outer["worker_cap_sha256"].is_null()
        || outer["lock_sha256"] != hashes[0]
        || outer["worker_config_sha256"] != hashes[2]
        || outer["worker_summary_sha256"] != hashes[3]
        || config["schema"] != "n83.primary-store-chain-config/v1"
        || config["source_commit"] != arm_lock["source_commit"]
        || config["max_trials"] != 0
        || config["orbit_columns"] != 600
        || config["summands"] != 6
        || config["policy"] != "public_x_hash"
        || config["seed"] != 2_026_100_801u64
        || summary["schema"] != "n83.primary-store-chain/v1"
        || summary["status"] != "PREFLIGHT_ONLY"
        || summary["source_commit"] != arm_lock["source_commit"]
        || summary["store_name"] != STORE_NAME
        || summary["point_set_blake3"] != POINT_SET
        || summary["max_trials"] != 0
        || summary["solver_stage_executed"] != false
        || summary["memory_cgroup_peak_bytes"]
            .as_u64()
            .filter(|n| *n > 0 && *n <= 4_294_967_296)
            .is_none()
    {
        return Err(
            "zero-trial K600 preflight is missing, failed, or differs from arm lock".into(),
        );
    }
    Ok(hashes)
}

fn export(
    lock_path: &Path,
    preflight_lock_path: &Path,
    preflight_root: &Path,
    mode: &str,
    output: &Path,
    outer_seconds: u64,
) -> Result<Value> {
    let stage_start = Instant::now();
    if !["positive-pinned", "positive-unpinned"].contains(&mode)
        || !output.is_absolute()
        || output.exists()
        || outer_seconds == 0
        || outer_seconds > 3_600
    {
        return Err("frozen K600 mode and new absolute output directory required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("export output must be outside source checkout".into());
    }
    let lock_path = fs::canonicalize(lock_path)?;
    let lock_sha = sha256_file(&lock_path)?;
    let lock: Value = serde_json::from_slice(&fs::read(&lock_path)?)?;
    let commit = clean_source_commit(&repo)?;
    let panel = PathBuf::from(lock["panel_root"].as_str().ok_or("panel root missing")?);
    let stage_root = panel.parent().ok_or("stage root missing")?;
    if parent.starts_with(stage_root) {
        return Err("export output must be outside staged input".into());
    }
    let (checked_panel, _) = staged_panel(stage_root, &commit)?;
    let exporter = fs::canonicalize(
        lock["exporter_replayer_binary"]
            .as_str()
            .ok_or("exporter binary missing")?,
    )?;
    let supervisor = fs::canonicalize(std::env::current_exe()?)?;
    let git_common = git_common_dir(&repo)?;
    if lock["schema"] != "n83.k600-m6-native-arms-lock/v1"
        || lock["status"] != "PREPARED_ONLY"
        || lock["source_commit"] != commit
        || lock["source_root"] != repo.to_string_lossy().as_ref()
        || checked_panel != panel
        || lock["staging_receipt_sha256"] != sha256_file(&stage_root.join("staging.json"))?
        || lock["store_name"] != STORE_NAME
        || lock["object_sha256"] != OBJECT_SHA
        || lock["point_set_blake3"] != POINT_SET
        || lock["exporter_replayer_sha256"] != sha256_file(&exporter)?
        || lock["supervisor_sha256"] != sha256_file(&supervisor)?
        || lock["max_variables"] != 125_000
        || lock["max_domain_clauses"] != 350_000
        || lock["max_conflicts"] != 10_000
        || lock["max_models"] != 1
        || lock["positive_executed"] != false
        || lock["natural_executed"] != false
        || lock["solver_launch_admitted"] != false
        || !is_linux_arm64_elf(&exporter)?
    {
        return Err("K600 arm lock or source changed before export".into());
    }
    let preflight_hashes = preflight_gate(&lock, preflight_lock_path, preflight_root)?;
    let image = lock["image_id"].as_str().ok_or("image ID missing")?;
    let inspected = docker_short(&["image", "inspect", "--format", "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}", image])?;
    if inspected != format!("{image} linux/arm64 {commit}") {
        return Err("frozen Linux arm64 source image changed before export".into());
    }
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    let json_path = output.join(format!("{mode}.json"));
    let xcnf_path = json_path.with_extension("xcnf");
    let stdout_path = output.join("export.stdout");
    let stderr_path = output.join("export.stderr");
    let run = (|| -> Result<Value> {
        let name = format!("n83-k600-m6-{mode}-{}", std::process::id());
        let args = docker_export_args(
            &lock,
            mode,
            &repo,
            &git_common,
            &panel,
            &exporter,
            &output,
            &name,
        );
        let mut command = Command::new("docker");
        command
            .args(&args)
            .stdout(Stdio::from(
                File::options()
                    .write(true)
                    .create_new(true)
                    .open(&stdout_path)?,
            ))
            .stderr(Stdio::from(
                File::options()
                    .write(true)
                    .create_new(true)
                    .open(&stderr_path)?,
            ));
        let (exit, wall_cap, wall_ms) = bounded_export(&mut command, &name, outer_seconds)?;
        if wall_cap {
            return Err(format!("UNKNOWN_export_wall_cap: {outer_seconds} seconds").into());
        }
        if exit != Some(0) {
            return Err(format!(
                "{}: exporter exit status {exit:?}",
                if exit == Some(137) {
                    "RESOURCE_EXIT_137"
                } else {
                    "PRODUCER_FAILURE"
                }
            )
            .into());
        }
        let receipt: Value = serde_json::from_slice(&fs::read(&json_path)?)?;
        validate_export(&receipt, &lock, mode, &xcnf_path)?;
        if clean_source_commit(&repo)? != commit
            || sha256_file(&lock_path)? != lock_sha
            || sha256_file(&exporter)? != lock["exporter_replayer_sha256"]
            || sha256_file(&supervisor)? != lock["supervisor_sha256"]
            || sha256_file(&stage_root.join("staging.json"))? != lock["staging_receipt_sha256"]
        {
            return Err("source, executable, or staged input changed after export".into());
        }
        if docker_short(&["image", "inspect", "--format", "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}", image])?
            != format!("{image} linux/arm64 {commit}") {
            return Err("frozen image changed after export".into());
        }
        let (after_panel, _) = staged_panel(stage_root, &commit)?;
        if after_panel != panel
            || preflight_gate(&lock, preflight_lock_path, preflight_root)? != preflight_hashes
        {
            return Err("panel changed after export".into());
        }
        let result = json!({
            "schema":"n83.k600-m6-native-arm-export/v1", "status":"PASS_EXPORT_ONLY",
            "mode":mode, "source_commit":commit, "lock_sha256":lock_sha,
            "preflight_outer_sha256":preflight_hashes[1],
            "export_json_sha256":sha256_file(&json_path)?, "xcnf_sha256":sha256_file(&xcnf_path)?,
            "exporter_exit_code":0, "outer_wall_ms":wall_ms,
            "outer_wall_limit_seconds":outer_seconds,
            "stage_wall_ms":stage_start.elapsed().as_millis(),
            "solver_executed":false, "model_replayed":false,
            "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null,
        });
        write_new_json(&output.join("export-stage.json"), &result)?;
        Ok(result)
    })();
    if let Err(error) = &run {
        let reason = error.to_string();
        let status = if reason.starts_with("UNKNOWN_export_wall_cap") {
            "UNKNOWN_export_wall_cap"
        } else if reason.starts_with("RESOURCE_EXIT_137") {
            "RESOURCE_EXIT_137"
        } else if reason.starts_with("CLEANUP_FAILURE") {
            "CLEANUP_FAILURE"
        } else {
            "PRODUCER_FAILURE"
        };
        let _ = write_new_json(
            &output.join("failure.json"),
            &json!({
                "schema":"n83.k600-m6-native-arm-export-failure/v1", "status":status,
                "mode":mode, "reason":reason, "source_commit":commit,
                "outer_wall_limit_seconds":outer_seconds,
                "stage_wall_ms":stage_start.elapsed().as_millis(),
                "selected_best_total_runtime":Value::Null,
            }),
        );
    }
    run
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
        Some("export") if args.len() == 8 => {
            let output = PathBuf::from(&args[6]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[7].parse()?;
                export(Path::new(&args[2]), Path::new(&args[3]), Path::new(&args[4]), &args[5], &output, seconds)
            })();
            (output, result, false)
        }
        _ => return Err("usage: stage FROZEN_METADATA_DIR NEW_OUTPUT_DIR | freeze STAGED_DIR EXPORTER_REPLAYER_BINARY SOLVER_BINARY IMAGE_ID NEW_LOCK_JSON | export ARM_LOCK_JSON PREFLIGHT_LOCK_JSON PREFLIGHT_RUN_DIR positive-pinned|positive-unpinned NEW_OUTPUT_DIR OUTER_SECONDS (natural requires completed positive control)".into()),
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

    #[test]
    fn export_container_args_preserve_hard_resource_and_read_only_input_gates() {
        let lock = json!({"image_id":format!("sha256:{}", "a".repeat(64))});
        let args = docker_export_args(
            &lock,
            "positive-unpinned",
            Path::new("/source"),
            Path::new("/git-common"),
            Path::new("/panel"),
            Path::new("/exporter"),
            Path::new("/output"),
            "n83-export-check",
        );
        let has = |key: &str, value: &str| {
            args.windows(2)
                .any(|pair| pair[0] == key && pair[1] == value)
        };
        for (key, value) in [
            ("--network", "none"),
            ("--memory", "4096m"),
            ("--memory-swap", "4096m"),
            ("--cpus", "1"),
            ("--cpuset-cpus", "0"),
            ("--pids-limit", "64"),
            ("--cap-drop", "ALL"),
            ("--security-opt", "no-new-privileges"),
        ] {
            assert!(has(key, value), "missing {key} {value}");
        }
        assert!(args.iter().any(|arg| arg == "--read-only"));
        for mount in [
            "type=bind,src=/source,dst=/src,readonly",
            "type=bind,src=/git-common,dst=/git-common,readonly",
            "type=bind,src=/panel,dst=/base,readonly",
            "type=bind,src=/exporter,dst=/exporter,readonly",
            "type=bind,src=/output,dst=/out",
        ] {
            assert!(has("--mount", mount), "missing {mount}");
        }
    }

    #[test]
    fn natural_export_is_gated_before_a_constructed_solver_control() {
        let output =
            std::env::temp_dir().join(format!("n83-k600-m6-natural-gate-{}", std::process::id()));
        assert!(!output.exists());
        assert!(export(
            Path::new("/missing-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            "natural",
            &output,
            1
        )
        .is_err());
        assert!(!output.exists());
    }

    #[test]
    fn preflight_gate_binds_worker_receipts_before_export() {
        let root =
            std::env::temp_dir().join(format!("n83-k600-m6-preflight-gate-{}", std::process::id()));
        let run = root.join("run");
        let worker = run.join("worker-output/run");
        fs::create_dir_all(&worker).unwrap();
        let commit = "a".repeat(40);
        let image = format!("sha256:{}", "b".repeat(64));
        let arm = json!({"source_commit":commit,"image_id":image});
        let preflight = json!({
            "schema":"n83.k600-m6-native-preflight-lock/v1", "source_commit":commit,
            "image_id":image,"object_sha256":OBJECT_SHA,"point_set_blake3":POINT_SET,
            "store_remote":REMOTE,"store_name":STORE_NAME,"max_trials":0,
            "max_variables":125_000,"max_domain_clauses":350_000,"max_models":1,
            "conflict_budget":10_000,
        });
        let config = json!({
            "schema":"n83.primary-store-chain-config/v1","source_commit":commit,
            "max_trials":0,"orbit_columns":600,"summands":6,
            "policy":"public_x_hash","seed":2_026_100_801u64,
        });
        let summary = json!({
            "schema":"n83.primary-store-chain/v1","status":"PREFLIGHT_ONLY",
            "source_commit":commit,"store_name":STORE_NAME,"point_set_blake3":POINT_SET,
            "max_trials":0,"solver_stage_executed":false,
            "memory_cgroup_peak_bytes":1_048_576u64,
        });
        let lock_path = root.join("preflight-lock.json");
        let config_path = worker.join("config.json");
        let summary_path = worker.join("summary.json");
        write_new_json(&lock_path, &preflight).unwrap();
        write_new_json(&config_path, &config).unwrap();
        write_new_json(&summary_path, &summary).unwrap();
        let outer = json!({
            "schema":"n83.k600-m6-native-outer/v1","status":"PREFLIGHT_ONLY",
            "worker_exit_code":0,"timed_out":false,"failure_reasons":[],
            "worker_cap_sha256":Value::Null,"lock_sha256":sha256_file(&lock_path).unwrap(),
            "worker_config_sha256":sha256_file(&config_path).unwrap(),
            "worker_summary_sha256":sha256_file(&summary_path).unwrap(),
        });
        write_new_json(&run.join("outer.json"), &outer).unwrap();
        preflight_gate(&arm, &lock_path, &run).unwrap();
        fs::write(&summary_path, b"changed").unwrap();
        assert!(preflight_gate(&arm, &lock_path, &run).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
