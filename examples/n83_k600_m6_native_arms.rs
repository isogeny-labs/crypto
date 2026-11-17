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

fn frozen_image_identity(lock: &Value) -> Result<String> {
    let image = lock["image_id"].as_str().ok_or("arm image ID missing")?;
    let build = lock["build_source_commit"]
        .as_str()
        .ok_or("arm build source missing")?;
    if !image.starts_with("sha256:")
        || image.len() != 71
        || !image[7..].bytes().all(|b| b.is_ascii_hexdigit())
        || build.len() != 40
        || !build.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("arm build image identity invalid".into());
    }
    Ok(format!("{image} linux/arm64 {build}"))
}

fn committed_preflight_revision(
    repo: &Path,
    lock_path: &Path,
    head: &str,
    image: &str,
) -> Result<(String, String)> {
    let lock_path = fs::canonicalize(lock_path)?;
    let relative = lock_path
        .strip_prefix(repo)?
        .to_str()
        .ok_or("preflight lock path encoding invalid")?;
    let bytes = fs::read(&lock_path)?;
    let lock: Value = serde_json::from_slice(&bytes)?;
    let source = lock["source_commit"]
        .as_str()
        .ok_or("preflight source commit missing")?;
    if lock["schema"] != "n83.k600-m6-native-preflight-lock/v1"
        || lock["status"] != "PREPARED_ONLY"
        || lock["source_root"] != repo.to_string_lossy().as_ref()
        || lock["image_id"] != image
        || source.len() != 40
        || !source.bytes().all(|b| b.is_ascii_hexdigit())
        || source == head
    {
        return Err("preflight lock identity differs from two-revision contract".into());
    }
    let tracked = Command::new("git")
        .args(["show", &format!("HEAD:{relative}")])
        .current_dir(repo)
        .output()?;
    let ancestor = Command::new("git")
        .args(["merge-base", "--is-ancestor", source, head])
        .current_dir(repo)
        .status()?;
    let parents = Command::new("git")
        .args(["rev-list", "--parents", "-n", "1", head])
        .current_dir(repo)
        .output()?;
    let changed = Command::new("git")
        .args(["diff", "--name-status", "--no-renames", source, head])
        .current_dir(repo)
        .output()?;
    if !tracked.status.success()
        || tracked.stdout != bytes
        || !ancestor.success()
        || !parents.status.success()
        || parents.stdout != format!("{head} {source}\n").as_bytes()
        || !changed.status.success()
        || changed.stdout != format!("A\t{relative}\n").as_bytes()
    {
        return Err("preflight lock must be the only tree change after build source".into());
    }
    let hashes = lock["source_sha256"]
        .as_object()
        .ok_or("preflight source hashes missing")?;
    if hashes.is_empty() {
        return Err("preflight source hashes empty".into());
    }
    for (name, expected) in hashes {
        let path = Path::new(name);
        if path.is_absolute()
            || path
                .components()
                .any(|c| !matches!(c, std::path::Component::Normal(_)))
            || expected.as_str() != Some(sha256_file(&repo.join(path))?.as_str())
        {
            return Err("preflight source hash differs at execution revision".into());
        }
    }
    Ok((source.to_owned(), sha256_file(&lock_path)?))
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
    preflight_lock: &Path,
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
    let (build_source, preflight_lock_sha) =
        committed_preflight_revision(&repo, preflight_lock, &commit, image)?;
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
    let expected = format!("{image} linux/arm64 {build_source}");
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
        "build_source_commit":build_source,"preflight_lock_sha256":preflight_lock_sha,
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

fn bounded_container(
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
    let limits = json!({"max_variables":125_000,"max_domain_clauses":350_000,
        "max_models":1,"conflict_budget":10_000});
    if preflight["schema"] != "n83.k600-m6-native-preflight-lock/v1"
        || preflight["source_commit"] != arm_lock["build_source_commit"]
        || arm_lock["preflight_lock_sha256"] != hashes[0]
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
        || preflight["memory_mib"] != 4096
        || preflight["cpu_limit"] != 1
        || preflight["summands"] != 6
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
        || config["source_commit"] != arm_lock["build_source_commit"]
        || config["study"] != "koblitz_n83_factor_base_sweep_20261008"
        || config["input_kind"] != "artifact_store_v3"
        || config["curve_a"] != 0
        || config["fixture"] != 0
        || config["strategy"] != "orbit-chain-s3"
        || config["max_trials"] != 0
        || config["orbit_columns"] != 600
        || config["summands"] != 6
        || config["policy"] != "public_x_hash"
        || config["seed"] != 2_026_100_801u64
        || config["chain_s3_limits"] != limits
        || config["source_attestation_mode"] != "supervised_snapshot"
        || config["memory_cgroup_limit_bytes"] != 4_294_967_296u64
        || config["memory_cgroup_swap_limit_bytes"] != 0
        || summary["schema"] != "n83.primary-store-chain/v1"
        || summary["status"] != "PREFLIGHT_ONLY"
        || summary["study"] != "koblitz_n83_factor_base_sweep_20261008"
        || summary["input_kind"] != "artifact_store_v3"
        || summary["curve_a"] != 0
        || summary["fixture"] != 0
        || summary["strategy"] != "orbit-chain-s3"
        || summary["orbit_columns"] != 600
        || summary["summands"] != 6
        || summary["policy"] != "public_x_hash"
        || summary["seed"] != 2_026_100_801u64
        || summary["source_commit"] != arm_lock["build_source_commit"]
        || summary["source_attestation_mode"] != "supervised_snapshot"
        || summary["store_name"] != STORE_NAME
        || summary["object"] != STORE_NAME
        || summary["point_set_blake3"] != POINT_SET
        || summary["store_manifest_sha256"] != preflight["base_sha256"]["manifest.json"]
        || summary["chain_s3_limits"] != limits
        || summary["max_trials"] != 0
        || summary["solver_stage_executed"] != false
        || summary["report"]["trials"] != 0
        || summary["report"]["relations"] != 0
        || summary["report"]["sat_calls"] != 0
        || summary["report"]["matrix_rows"] != 0
        || !summary["attempt_records"]
            .as_array()
            .is_some_and(Vec::is_empty)
        || !summary["relation_matrix"]
            .as_array()
            .is_some_and(Vec::is_empty)
        || !summary["verified_log"].is_null()
        || summary["memory_cgroup_limit_bytes"] != 4_294_967_296u64
        || summary["memory_cgroup_swap_limit_bytes"] != 0
        || !summary["total_index_calculus_runtime_ms"].is_null()
        || !summary["selected_best_total_runtime"].is_null()
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

fn audited_constructed_export(
    arm_lock: &Value,
    arm_lock_path: &Path,
    preflight_hashes: &[String],
    mode: &str,
    export_root: &Path,
) -> Result<(PathBuf, Vec<String>)> {
    if !["positive-pinned", "positive-unpinned"].contains(&mode) {
        return Err("constructed control mode required before natural query".into());
    }
    let root = fs::canonicalize(export_root)?;
    let stage_path = root.join("export-stage.json");
    let export_path = root.join(format!("{mode}.json"));
    let xcnf_path = root.join(format!("{mode}.xcnf"));
    let stage: Value = serde_json::from_slice(&fs::read(&stage_path)?)?;
    let export: Value = serde_json::from_slice(&fs::read(&export_path)?)?;
    let hashes = vec![
        sha256_file(&stage_path)?,
        sha256_file(&export_path)?,
        sha256_file(&xcnf_path)?,
    ];
    if stage["schema"] != "n83.k600-m6-native-arm-export/v1"
        || stage["status"] != "PASS_EXPORT_ONLY"
        || stage["mode"] != mode
        || stage["source_commit"] != arm_lock["source_commit"]
        || stage["lock_sha256"] != sha256_file(arm_lock_path)?
        || stage["preflight_outer_sha256"] != preflight_hashes[1]
        || stage["export_json_sha256"] != hashes[1]
        || stage["xcnf_sha256"] != hashes[2]
        || stage["exporter_exit_code"] != 0
        || stage["solver_executed"] != false
        || stage["model_replayed"] != false
        || !stage["total_index_calculus_runtime_ms"].is_null()
        || !stage["selected_best_total_runtime"].is_null()
    {
        return Err("constructed XCNF export stage differs from frozen lock".into());
    }
    validate_export(&export, arm_lock, mode, &xcnf_path)?;
    Ok((xcnf_path, hashes))
}

fn audited_natural_export(
    arm_lock: &Value,
    arm_lock_path: &Path,
    preflight_hashes: &[String],
    unpinned_replay_sha: &str,
    export_root: &Path,
) -> Result<(PathBuf, Vec<String>)> {
    let root = fs::canonicalize(export_root)?;
    if root.join("failure.json").exists() {
        return Err("natural export has a terminal failure receipt".into());
    }
    let stage_path = root.join("export-stage.json");
    let export_path = root.join("natural.json");
    let xcnf_path = root.join("natural.xcnf");
    let stage: Value = serde_json::from_slice(&fs::read(&stage_path)?)?;
    let export: Value = serde_json::from_slice(&fs::read(&export_path)?)?;
    let hashes = vec![
        sha256_file(&stage_path)?,
        sha256_file(&export_path)?,
        sha256_file(&xcnf_path)?,
    ];
    if stage["schema"] != "n83.k600-m6-native-arm-export/v1"
        || stage["status"] != "PASS_EXPORT_ONLY"
        || stage["mode"] != "natural"
        || stage["source_commit"] != arm_lock["source_commit"]
        || stage["lock_sha256"] != sha256_file(arm_lock_path)?
        || stage["preflight_outer_sha256"] != preflight_hashes[1]
        || stage["unpinned_replay_stage_sha256"] != unpinned_replay_sha
        || stage["export_json_sha256"] != hashes[1]
        || stage["xcnf_sha256"] != hashes[2]
        || stage["exporter_exit_code"] != 0
        || stage["solver_executed"] != false
        || stage["model_replayed"] != false
        || !stage["total_index_calculus_runtime_ms"].is_null()
        || !stage["selected_best_total_runtime"].is_null()
    {
        return Err("natural XCNF export stage differs from two replayed controls".into());
    }
    validate_export(&export, arm_lock, "natural", &xcnf_path)?;
    Ok((xcnf_path, hashes))
}

fn docker_solver_args(
    lock: &Value,
    solver: &Path,
    xcnf: &Path,
    name: &str,
    solver_seconds: u64,
) -> Vec<String> {
    let mount = |source: &Path, destination: &str| {
        format!(
            "type=bind,src={},dst={destination},readonly",
            source.display()
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
        mount(solver, "/solver"),
        "--mount".into(),
        mount(xcnf, "/input.xcnf"),
        lock["image_id"].as_str().unwrap_or_default().into(),
        "/solver".into(),
        "--threads".into(),
        "1".into(),
        "--random".into(),
        "20261115".into(),
        "--maxtime".into(),
        solver_seconds.to_string(),
        "--maxconfl".into(),
        "10000".into(),
        "--verb".into(),
        "1".into(),
        "--verbstat".into(),
        "1".into(),
        "--printsol".into(),
        "1".into(),
        "/input.xcnf".into(),
    ]
}

fn export(
    lock_path: &Path,
    preflight_lock_path: &Path,
    preflight_root: &Path,
    mode: &str,
    output: &Path,
    outer_seconds: u64,
    natural_controls: Option<(&Path, &Path, &Path, &Path, &Path, &Path)>,
) -> Result<Value> {
    let stage_start = Instant::now();
    if !matches!(
        (mode, natural_controls.is_some()),
        ("positive-pinned", false) | ("positive-unpinned", false) | ("natural", true)
    ) || !output.is_absolute()
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
    let control_dirs = natural_controls
        .map(|(a, b, c, d, e, f)| -> Result<_> {
            Ok([
                fs::canonicalize(a)?,
                fs::canonicalize(b)?,
                fs::canonicalize(c)?,
                fs::canonicalize(d)?,
                fs::canonicalize(e)?,
                fs::canonicalize(f)?,
            ])
        })
        .transpose()?;
    if parent.starts_with(stage_root)
        || control_dirs
            .as_ref()
            .is_some_and(|dirs| dirs.iter().any(|dir| parent.starts_with(dir)))
    {
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
    let control_hashes = control_dirs
        .as_ref()
        .map(|dirs| {
            audited_unpinned_replay(
                &lock,
                &lock_path,
                &preflight_hashes,
                &dirs[0],
                &dirs[1],
                &dirs[2],
                &dirs[3],
                &dirs[4],
                &dirs[5],
            )
        })
        .transpose()?;
    let image = lock["image_id"].as_str().ok_or("image ID missing")?;
    let inspected = docker_short(&["image", "inspect", "--format", "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}", image])?;
    if inspected != frozen_image_identity(&lock)? {
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
        let (exit, wall_cap, wall_ms) = bounded_container(&mut command, &name, outer_seconds)?;
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
            != frozen_image_identity(&lock)? {
            return Err("frozen image changed after export".into());
        }
        let (after_panel, _) = staged_panel(stage_root, &commit)?;
        if after_panel != panel
            || preflight_gate(&lock, preflight_lock_path, preflight_root)? != preflight_hashes
            || control_dirs
                .as_ref()
                .map(|dirs| {
                    audited_unpinned_replay(
                        &lock,
                        &lock_path,
                        &preflight_hashes,
                        &dirs[0],
                        &dirs[1],
                        &dirs[2],
                        &dirs[3],
                        &dirs[4],
                        &dirs[5],
                    )
                })
                .transpose()?
                != control_hashes
        {
            return Err("panel changed after export".into());
        }
        let result = json!({
            "schema":"n83.k600-m6-native-arm-export/v1", "status":"PASS_EXPORT_ONLY",
            "mode":mode, "source_commit":commit, "lock_sha256":lock_sha,
            "preflight_outer_sha256":preflight_hashes[1],
            "unpinned_replay_stage_sha256":control_hashes.as_ref().map(|hashes|&hashes[3]),
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

fn solve(
    lock_path: &Path,
    preflight_lock_path: &Path,
    preflight_root: &Path,
    mode: &str,
    export_root: &Path,
    output: &Path,
    outer_seconds: u64,
    pinned_controls: Option<(&Path, &Path, &Path)>,
    natural_controls: Option<(&Path, &Path, &Path, &Path, &Path, &Path)>,
) -> Result<Value> {
    let stage_start = Instant::now();
    if !matches!(
        (mode, pinned_controls.is_some(), natural_controls.is_some()),
        ("positive-pinned", false, false)
            | ("positive-unpinned", true, false)
            | ("natural", false, true)
    ) || !output.is_absolute()
        || output.exists()
        || !(6..=3_600).contains(&outer_seconds)
    {
        return Err(
            "frozen mode/control, new absolute output, and 6..3600 outer seconds required".into(),
        );
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    let lock_path = fs::canonicalize(lock_path)?;
    let lock_sha = sha256_file(&lock_path)?;
    let lock: Value = serde_json::from_slice(&fs::read(&lock_path)?)?;
    let commit = clean_source_commit(&repo)?;
    let panel = PathBuf::from(lock["panel_root"].as_str().ok_or("panel root missing")?);
    let stage_root = panel.parent().ok_or("stage root missing")?;
    let export_root = fs::canonicalize(export_root)?;
    let pinned_controls = pinned_controls
        .map(|(export, solver, replay)| -> Result<_> {
            Ok((
                fs::canonicalize(export)?,
                fs::canonicalize(solver)?,
                fs::canonicalize(replay)?,
            ))
        })
        .transpose()?;
    let natural_controls = natural_controls
        .map(|(a, b, c, d, e, f)| -> Result<_> {
            Ok([
                fs::canonicalize(a)?,
                fs::canonicalize(b)?,
                fs::canonicalize(c)?,
                fs::canonicalize(d)?,
                fs::canonicalize(e)?,
                fs::canonicalize(f)?,
            ])
        })
        .transpose()?;
    if parent.starts_with(&repo)
        || parent.starts_with(stage_root)
        || parent.starts_with(&export_root)
        || pinned_controls.as_ref().is_some_and(|(a, b, c)| {
            parent.starts_with(a) || parent.starts_with(b) || parent.starts_with(c)
        })
        || natural_controls
            .as_ref()
            .is_some_and(|dirs| dirs.iter().any(|dir| parent.starts_with(dir)))
    {
        return Err("solver output overlaps frozen source or input".into());
    }
    let (checked_panel, _) = staged_panel(stage_root, &commit)?;
    let solver = fs::canonicalize(
        lock["solver_binary"]
            .as_str()
            .ok_or("solver binary missing")?,
    )?;
    let exporter = fs::canonicalize(
        lock["exporter_replayer_binary"]
            .as_str()
            .ok_or("exporter binary missing")?,
    )?;
    let supervisor = fs::canonicalize(std::env::current_exe()?)?;
    if lock["schema"] != "n83.k600-m6-native-arms-lock/v1"
        || lock["status"] != "PREPARED_ONLY"
        || lock["source_commit"] != commit
        || lock["source_root"] != repo.to_string_lossy().as_ref()
        || checked_panel != panel
        || lock["staging_receipt_sha256"] != sha256_file(&stage_root.join("staging.json"))?
        || lock["store_name"] != STORE_NAME
        || lock["object_sha256"] != OBJECT_SHA
        || lock["point_set_blake3"] != POINT_SET
        || lock["solver_sha256"] != sha256_file(&solver)?
        || lock["exporter_replayer_sha256"] != sha256_file(&exporter)?
        || lock["supervisor_sha256"] != sha256_file(&supervisor)?
        || lock["max_conflicts"] != 10_000
        || lock["max_models"] != 1
        || lock["solver_launch_admitted"] != false
        || !is_linux_arm64_elf(&solver)?
    {
        return Err("K600 arm lock, source, base, or solver changed before launch".into());
    }
    let preflight_hashes = preflight_gate(&lock, preflight_lock_path, preflight_root)?;
    let natural_hashes = natural_controls
        .as_ref()
        .map(|dirs| {
            audited_unpinned_replay(
                &lock,
                &lock_path,
                &preflight_hashes,
                &dirs[0],
                &dirs[1],
                &dirs[2],
                &dirs[3],
                &dirs[4],
                &dirs[5],
            )
        })
        .transpose()?;
    let (xcnf, export_hashes) = if mode == "natural" {
        audited_natural_export(
            &lock,
            &lock_path,
            &preflight_hashes,
            &natural_hashes.as_ref().ok_or("natural control missing")?[3],
            &export_root,
        )?
    } else {
        audited_constructed_export(&lock, &lock_path, &preflight_hashes, mode, &export_root)?
    };
    let pinned_hashes = pinned_controls
        .as_ref()
        .map(|(a, b, c)| audited_pinned_replay(&lock, &lock_path, &preflight_hashes, a, b, c))
        .transpose()?;
    let image = lock["image_id"].as_str().ok_or("image ID missing")?;
    let image_identity = frozen_image_identity(&lock)?;
    let image_format = "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}";
    if docker_short(&["image", "inspect", "--format", image_format, image])? != image_identity {
        return Err("frozen solver image differs before launch".into());
    }
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    let stdout_path = output.join("solver.stdout");
    let stderr_path = output.join("solver.stderr");
    let run = (|| -> Result<Value> {
        let name = format!("n83-k600-m6-solve-{mode}-{}", std::process::id());
        let solver_seconds = outer_seconds - 5;
        let args = docker_solver_args(&lock, &solver, &xcnf, &name, solver_seconds);
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
        let (exit, capped, wall_ms) = bounded_container(&mut command, &name, outer_seconds)?;
        let stdout_bytes = fs::metadata(&stdout_path)?.len();
        let status = if capped {
            "UNKNOWN_solver_wall_cap"
        } else if stdout_bytes > 20_000_000 {
            "PRODUCER_FAILURE_output_cap"
        } else if exit == Some(10) {
            "SAT_PENDING_XCNF_REPLAY"
        } else if exit == Some(20) {
            if mode == "natural" {
                "UNSAT_NATURAL_QUERY"
            } else {
                "UNSAT_CONSTRUCTED_CONTROL"
            }
        } else if exit == Some(0) {
            "UNKNOWN_solver_cap_or_exit0"
        } else if matches!(exit, Some(134 | 137)) {
            "RESOURCE_EXIT"
        } else {
            "PRODUCER_FAILURE_solver_exit"
        };
        if clean_source_commit(&repo)? != commit
            || sha256_file(&lock_path)? != lock_sha
            || sha256_file(&solver)? != lock["solver_sha256"]
            || sha256_file(&exporter)? != lock["exporter_replayer_sha256"]
            || sha256_file(&supervisor)? != lock["supervisor_sha256"]
            || sha256_file(&stage_root.join("staging.json"))? != lock["staging_receipt_sha256"]
            || preflight_gate(&lock, preflight_lock_path, preflight_root)? != preflight_hashes
            || (if mode == "natural" {
                audited_natural_export(
                    &lock,
                    &lock_path,
                    &preflight_hashes,
                    &natural_hashes.as_ref().ok_or("natural control missing")?[3],
                    &export_root,
                )?
            } else {
                audited_constructed_export(
                    &lock,
                    &lock_path,
                    &preflight_hashes,
                    mode,
                    &export_root,
                )?
            })
            .1 != export_hashes
            || pinned_controls
                .as_ref()
                .map(|(a, b, c)| {
                    audited_pinned_replay(&lock, &lock_path, &preflight_hashes, a, b, c)
                })
                .transpose()?
                != pinned_hashes
            || natural_controls
                .as_ref()
                .map(|dirs| {
                    audited_unpinned_replay(
                        &lock,
                        &lock_path,
                        &preflight_hashes,
                        &dirs[0],
                        &dirs[1],
                        &dirs[2],
                        &dirs[3],
                        &dirs[4],
                        &dirs[5],
                    )
                })
                .transpose()?
                != natural_hashes
            || docker_short(&["image", "inspect", "--format", image_format, image])?
                != image_identity
        {
            return Err("source, image, solver, preflight, or XCNF changed after solver".into());
        }
        let (after_panel, _) = staged_panel(stage_root, &commit)?;
        if after_panel != panel {
            return Err("factor-base panel changed after solver".into());
        }
        Ok(json!({
            "schema":"n83.k600-m6-native-solver/v1","status":status,"mode":mode,
            "source_commit":commit,"lock_sha256":lock_sha,"image_id":image,
            "preflight_outer_sha256":preflight_hashes[1],
            "export_stage_sha256":export_hashes[0],"xcnf_sha256":export_hashes[2],
            "pinned_replay_stage_sha256":pinned_hashes.as_ref().map(|hashes|&hashes[2]),
            "unpinned_replay_stage_sha256":natural_hashes.as_ref().map(|hashes|&hashes[3]),
            "solver_binary_sha256":lock["solver_sha256"],"stdout_sha256":sha256_file(&stdout_path)?,
            "stderr_sha256":sha256_file(&stderr_path)?,"exit_code":exit,
            "outer_wall_limit_seconds":outer_seconds,"solver_internal_maxtime_seconds":solver_seconds,
            "outer_wall_ms":wall_ms,"stage_wall_ms":stage_start.elapsed().as_millis(),
            "conflict_budget":10_000,"threads":1,"random_seed":20_261_115u64,
            "model_replayed":false,"total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null,
        }))
    })();
    match run {
        Ok(receipt) => {
            write_new_json(&output.join("solver-receipt.json"), &receipt)?;
            if receipt["status"] == "SAT_PENDING_XCNF_REPLAY" {
                Ok(receipt)
            } else {
                Err(format!("K600 {mode} solver ended with {}", receipt["status"]).into())
            }
        }
        Err(error) => {
            let reason = error.to_string();
            let status = if reason.starts_with("CLEANUP_FAILURE") {
                "CLEANUP_FAILURE"
            } else {
                "PRODUCER_FAILURE"
            };
            let _ = write_new_json(
                &output.join("failure.json"),
                &json!({
                    "schema":"n83.k600-m6-native-solver-failure/v1","status":status,
                    "mode":mode,"reason":reason,"source_commit":commit,
                    "stage_wall_ms":stage_start.elapsed().as_millis(),
                    "selected_best_total_runtime":Value::Null,
                }),
            );
            Err(error)
        }
    }
}

fn audited_constructed_solver(
    lock: &Value,
    lock_path: &Path,
    preflight_hashes: &[String],
    export_hashes: &[String],
    solver_root: &Path,
    mode: &str,
    pinned_replay_sha: Option<&str>,
) -> Result<(PathBuf, PathBuf, Vec<String>)> {
    if !matches!(mode, "positive-pinned" | "positive-unpinned") {
        return Err("constructed solver audit mode required".into());
    }
    let root = fs::canonicalize(solver_root)?;
    let receipt_path = root.join("solver-receipt.json");
    let stdout_path = root.join("solver.stdout");
    let stderr_path = root.join("solver.stderr");
    let receipt: Value = serde_json::from_slice(&fs::read(&receipt_path)?)?;
    let hashes = vec![
        sha256_file(&receipt_path)?,
        sha256_file(&stdout_path)?,
        sha256_file(&stderr_path)?,
    ];
    if receipt["schema"] != "n83.k600-m6-native-solver/v1"
        || receipt["status"] != "SAT_PENDING_XCNF_REPLAY"
        || receipt["mode"] != mode
        || receipt["source_commit"] != lock["source_commit"]
        || receipt["lock_sha256"] != sha256_file(lock_path)?
        || receipt["image_id"] != lock["image_id"]
        || receipt["preflight_outer_sha256"] != preflight_hashes[1]
        || receipt["export_stage_sha256"] != export_hashes[0]
        || receipt["xcnf_sha256"] != export_hashes[2]
        || receipt["solver_binary_sha256"] != lock["solver_sha256"]
        || receipt["stdout_sha256"] != hashes[1]
        || receipt["stderr_sha256"] != hashes[2]
        || receipt["exit_code"] != 10
        || receipt["conflict_budget"] != 10_000
        || receipt["threads"] != 1
        || receipt["random_seed"] != 20_261_115u64
        || receipt["pinned_replay_stage_sha256"]
            != pinned_replay_sha.map_or(Value::Null, |s| json!(s))
        || receipt["model_replayed"] != false
        || !receipt["total_index_calculus_runtime_ms"].is_null()
        || !receipt["selected_best_total_runtime"].is_null()
        || fs::metadata(&stdout_path)?.len() > 20_000_000
    {
        return Err(
            "constructed SAT solver receipt or model output differs from frozen XCNF arm".into(),
        );
    }
    Ok((receipt_path, stdout_path, hashes))
}

fn audited_natural_solver(
    lock: &Value,
    lock_path: &Path,
    preflight_hashes: &[String],
    export_hashes: &[String],
    unpinned_replay_sha: &str,
    solver_root: &Path,
) -> Result<(PathBuf, PathBuf, Vec<String>)> {
    let root = fs::canonicalize(solver_root)?;
    if root.join("failure.json").exists() {
        return Err("natural solver has a terminal failure receipt".into());
    }
    let receipt_path = root.join("solver-receipt.json");
    let stdout_path = root.join("solver.stdout");
    let stderr_path = root.join("solver.stderr");
    let receipt: Value = serde_json::from_slice(&fs::read(&receipt_path)?)?;
    let hashes = vec![
        sha256_file(&receipt_path)?,
        sha256_file(&stdout_path)?,
        sha256_file(&stderr_path)?,
    ];
    if receipt["schema"] != "n83.k600-m6-native-solver/v1"
        || receipt["status"] != "SAT_PENDING_XCNF_REPLAY"
        || receipt["mode"] != "natural"
        || receipt["source_commit"] != lock["source_commit"]
        || receipt["lock_sha256"] != sha256_file(lock_path)?
        || receipt["image_id"] != lock["image_id"]
        || receipt["preflight_outer_sha256"] != preflight_hashes[1]
        || receipt["export_stage_sha256"] != export_hashes[0]
        || receipt["xcnf_sha256"] != export_hashes[2]
        || receipt["unpinned_replay_stage_sha256"] != unpinned_replay_sha
        || !receipt["pinned_replay_stage_sha256"].is_null()
        || receipt["solver_binary_sha256"] != lock["solver_sha256"]
        || receipt["stdout_sha256"] != hashes[1]
        || receipt["stderr_sha256"] != hashes[2]
        || receipt["exit_code"] != 10
        || receipt["conflict_budget"] != 10_000
        || receipt["threads"] != 1
        || receipt["random_seed"] != 20_261_115u64
        || receipt["model_replayed"] != false
        || !receipt["total_index_calculus_runtime_ms"].is_null()
        || !receipt["selected_best_total_runtime"].is_null()
        || fs::metadata(&stdout_path)?.len() > 20_000_000
    {
        return Err(
            "natural SAT solver receipt or model output differs from frozen XCNF arm".into(),
        );
    }
    Ok((receipt_path, stdout_path, hashes))
}

fn docker_replay_args(
    lock: &Value,
    mode: &str,
    repo: &Path,
    git_common: &Path,
    panel: &Path,
    export_root: &Path,
    solver_root: &Path,
    exporter: &Path,
    output: &Path,
    name: &str,
) -> Vec<String> {
    let mount = |source: &Path, destination: &str, readonly: bool| {
        format!(
            "type=bind,src={},dst={destination}{}",
            source.display(),
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
        mount(export_root, "/export", true),
        "--mount".into(),
        mount(solver_root, "/solver-receipt", true),
        "--mount".into(),
        mount(exporter, "/replayer", true),
        "--mount".into(),
        mount(output, "/out", false),
        "--workdir".into(),
        "/src".into(),
        "--env".into(),
        "GIT_OPTIONAL_LOCKS=0".into(),
        lock["image_id"].as_str().unwrap_or_default().into(),
        "/replayer".into(),
        "primary-k600-m6-replay".into(),
        "/base".into(),
        format!("/export/{mode}.json"),
        format!("/export/{mode}.xcnf"),
        "/solver-receipt/solver-receipt.json".into(),
        "/solver-receipt/solver.stdout".into(),
        lock["source_commit"].as_str().unwrap_or_default().into(),
        lock["exporter_replayer_sha256"]
            .as_str()
            .unwrap_or_default()
            .into(),
        "/out/replay.json".into(),
    ]
}

fn replay_constructed(
    lock_path: &Path,
    preflight_lock_path: &Path,
    preflight_root: &Path,
    export_root: &Path,
    solver_root: &Path,
    output: &Path,
    outer_seconds: u64,
    mode: &str,
    pinned_controls: Option<(&Path, &Path, &Path)>,
    natural_controls: Option<(&Path, &Path, &Path, &Path, &Path, &Path)>,
) -> Result<Value> {
    let stage_start = Instant::now();
    if !matches!(
        (mode, pinned_controls.is_some(), natural_controls.is_some()),
        ("positive-pinned", false, false)
            | ("positive-unpinned", true, false)
            | ("natural", false, true)
    ) || !output.is_absolute()
        || output.exists()
        || !(1..=3_600).contains(&outer_seconds)
    {
        return Err("new absolute replay output and 1..3600 outer seconds required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    let lock_path = fs::canonicalize(lock_path)?;
    let lock_sha = sha256_file(&lock_path)?;
    let lock: Value = serde_json::from_slice(&fs::read(&lock_path)?)?;
    let commit = clean_source_commit(&repo)?;
    let panel = PathBuf::from(lock["panel_root"].as_str().ok_or("panel root missing")?);
    let stage_root = panel.parent().ok_or("stage root missing")?;
    let export_root = fs::canonicalize(export_root)?;
    let solver_root = fs::canonicalize(solver_root)?;
    let pinned_controls = pinned_controls
        .map(|(a, b, c)| -> Result<_> {
            Ok((
                fs::canonicalize(a)?,
                fs::canonicalize(b)?,
                fs::canonicalize(c)?,
            ))
        })
        .transpose()?;
    let natural_controls = natural_controls
        .map(|(a, b, c, d, e, f)| -> Result<_> {
            Ok([
                fs::canonicalize(a)?,
                fs::canonicalize(b)?,
                fs::canonicalize(c)?,
                fs::canonicalize(d)?,
                fs::canonicalize(e)?,
                fs::canonicalize(f)?,
            ])
        })
        .transpose()?;
    if parent.starts_with(&repo)
        || parent.starts_with(stage_root)
        || parent.starts_with(&export_root)
        || parent.starts_with(&solver_root)
        || pinned_controls.as_ref().is_some_and(|(a, b, c)| {
            parent.starts_with(a) || parent.starts_with(b) || parent.starts_with(c)
        })
        || natural_controls
            .as_ref()
            .is_some_and(|dirs| dirs.iter().any(|dir| parent.starts_with(dir)))
    {
        return Err("replay output overlaps frozen source or input".into());
    }
    let (checked_panel, _) = staged_panel(stage_root, &commit)?;
    let exporter = fs::canonicalize(
        lock["exporter_replayer_binary"]
            .as_str()
            .ok_or("replayer binary missing")?,
    )?;
    let solver = fs::canonicalize(
        lock["solver_binary"]
            .as_str()
            .ok_or("solver binary missing")?,
    )?;
    let supervisor = fs::canonicalize(std::env::current_exe()?)?;
    let git_common = git_common_dir(&repo)?;
    if lock["schema"] != "n83.k600-m6-native-arms-lock/v1"
        || lock["status"] != "PREPARED_ONLY"
        || lock["source_commit"] != commit
        || lock["source_root"] != repo.to_string_lossy().as_ref()
        || checked_panel != panel
        || lock["staging_receipt_sha256"] != sha256_file(&stage_root.join("staging.json"))?
        || lock["object_sha256"] != OBJECT_SHA
        || lock["point_set_blake3"] != POINT_SET
        || lock["exporter_replayer_sha256"] != sha256_file(&exporter)?
        || lock["solver_sha256"] != sha256_file(&solver)?
        || lock["supervisor_sha256"] != sha256_file(&supervisor)?
        || !is_linux_arm64_elf(&exporter)?
    {
        return Err("K600 replay lock, source, base, or binary changed".into());
    }
    let preflight_hashes = preflight_gate(&lock, preflight_lock_path, preflight_root)?;
    let natural_hashes = natural_controls
        .as_ref()
        .map(|dirs| {
            audited_unpinned_replay(
                &lock,
                &lock_path,
                &preflight_hashes,
                &dirs[0],
                &dirs[1],
                &dirs[2],
                &dirs[3],
                &dirs[4],
                &dirs[5],
            )
        })
        .transpose()?;
    let (_, export_hashes) = if mode == "natural" {
        audited_natural_export(
            &lock,
            &lock_path,
            &preflight_hashes,
            &natural_hashes.as_ref().ok_or("natural control missing")?[3],
            &export_root,
        )?
    } else {
        audited_constructed_export(&lock, &lock_path, &preflight_hashes, mode, &export_root)?
    };
    let pinned_hashes = pinned_controls
        .as_ref()
        .map(|(a, b, c)| audited_pinned_replay(&lock, &lock_path, &preflight_hashes, a, b, c))
        .transpose()?;
    let (_, _, solver_hashes) = if mode == "natural" {
        audited_natural_solver(
            &lock,
            &lock_path,
            &preflight_hashes,
            &export_hashes,
            &natural_hashes.as_ref().ok_or("natural control missing")?[3],
            &solver_root,
        )?
    } else {
        audited_constructed_solver(
            &lock,
            &lock_path,
            &preflight_hashes,
            &export_hashes,
            &solver_root,
            mode,
            pinned_hashes.as_ref().map(|hashes| hashes[2].as_str()),
        )?
    };
    let image = lock["image_id"].as_str().ok_or("image ID missing")?;
    let image_format = "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}";
    let image_identity = frozen_image_identity(&lock)?;
    if docker_short(&["image", "inspect", "--format", image_format, image])? != image_identity {
        return Err("frozen replay image differs before launch".into());
    }
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    let stdout_path = output.join("replay.stdout");
    let stderr_path = output.join("replay.stderr");
    let result_path = output.join("replay.json");
    let run = (|| -> Result<Value> {
        let name = format!("n83-k600-m6-replay-{mode}-{}", std::process::id());
        let args = docker_replay_args(
            &lock,
            mode,
            &repo,
            &git_common,
            &panel,
            &export_root,
            &solver_root,
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
        let (exit, capped, wall_ms) = bounded_container(&mut command, &name, outer_seconds)?;
        if capped {
            return Err("UNKNOWN_replay_wall_cap".into());
        }
        if exit != Some(0) {
            return Err(format!("INVALID_MODEL_OR_REPLAY_FAILURE: exit {exit:?}").into());
        }
        let checked: Value = serde_json::from_slice(&fs::read(&result_path)?)?;
        let exported: Value =
            serde_json::from_slice(&fs::read(export_root.join(format!("{mode}.json")))?)?;
        if checked["schema"] != "n83.k600-m6-model-replay/v2"
            || checked["status"] != "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY"
            || checked["mode"] != mode
            || checked["source_commit"] != commit
            || checked["export_binary_sha256"] != lock["exporter_replayer_sha256"]
            || checked["replay_binary_sha256"] != lock["exporter_replayer_sha256"]
            || checked["xcnf_sha256"] != export_hashes[2]
            || checked["solver_stdout_sha256"] != solver_hashes[1]
            || checked["object_sha256"] != OBJECT_SHA
            || checked["target"] != exported["target"]
            || checked["checked_cnf_clauses"] != exported["shape"]["clauses"]
            || checked["checked_xor_rows"] != exported["shape"]["xor_rows"]
            || checked["chosen_indices"]
                .as_array()
                .is_none_or(|v| v.len() != 6)
            || checked["relation_row_decimal"]
                .as_array()
                .is_none_or(|v| v.len() != 600)
            || checked["subgroup_order"] != "2417851639230796216685689"
            || !checked["selected_best_total_runtime"].is_null()
        {
            return Err(
                "native model replay result differs from original XCNF or group row".into(),
            );
        }
        if clean_source_commit(&repo)? != commit
            || sha256_file(&lock_path)? != lock_sha
            || sha256_file(&exporter)? != lock["exporter_replayer_sha256"]
            || sha256_file(&solver)? != lock["solver_sha256"]
            || sha256_file(&supervisor)? != lock["supervisor_sha256"]
            || sha256_file(&stage_root.join("staging.json"))? != lock["staging_receipt_sha256"]
            || preflight_gate(&lock, preflight_lock_path, preflight_root)? != preflight_hashes
            || (if mode == "natural" {
                audited_natural_export(
                    &lock,
                    &lock_path,
                    &preflight_hashes,
                    &natural_hashes.as_ref().ok_or("natural control missing")?[3],
                    &export_root,
                )?
            } else {
                audited_constructed_export(
                    &lock,
                    &lock_path,
                    &preflight_hashes,
                    mode,
                    &export_root,
                )?
            })
            .1 != export_hashes
            || (if mode == "natural" {
                audited_natural_solver(
                    &lock,
                    &lock_path,
                    &preflight_hashes,
                    &export_hashes,
                    &natural_hashes.as_ref().ok_or("natural control missing")?[3],
                    &solver_root,
                )?
            } else {
                audited_constructed_solver(
                    &lock,
                    &lock_path,
                    &preflight_hashes,
                    &export_hashes,
                    &solver_root,
                    mode,
                    pinned_hashes.as_ref().map(|hashes| hashes[2].as_str()),
                )?
            })
            .2 != solver_hashes
            || pinned_controls
                .as_ref()
                .map(|(a, b, c)| {
                    audited_pinned_replay(&lock, &lock_path, &preflight_hashes, a, b, c)
                })
                .transpose()?
                != pinned_hashes
            || natural_controls
                .as_ref()
                .map(|dirs| {
                    audited_unpinned_replay(
                        &lock,
                        &lock_path,
                        &preflight_hashes,
                        &dirs[0],
                        &dirs[1],
                        &dirs[2],
                        &dirs[3],
                        &dirs[4],
                        &dirs[5],
                    )
                })
                .transpose()?
                != natural_hashes
            || docker_short(&["image", "inspect", "--format", image_format, image])?
                != image_identity
        {
            return Err("source, image, solver, preflight, or XCNF changed after replay".into());
        }
        let (after_panel, _) = staged_panel(stage_root, &commit)?;
        if after_panel != panel {
            return Err("factor-base panel changed after replay".into());
        }
        let receipt = json!({
            "schema":match mode {"positive-pinned"=>"n83.k600-m6-native-pinned-replay-stage/v1",
                "positive-unpinned"=>"n83.k600-m6-native-unpinned-replay-stage/v1",
                _=>"n83.k600-m6-native-natural-replay-stage/v1"},
            "status":match mode {"positive-pinned"=>"PASS_PINNED_MODEL_REPLAY",
                "positive-unpinned"=>"PASS_UNPINNED_MODEL_REPLAY",_=>"PASS_NATURAL_MODEL_REPLAY"},
            "mode":mode,"source_commit":commit,
            "lock_sha256":lock_sha,"preflight_outer_sha256":preflight_hashes[1],
            "export_stage_sha256":export_hashes[0],"solver_receipt_sha256":solver_hashes[0],
            "replay_json_sha256":sha256_file(&result_path)?,"xcnf_sha256":export_hashes[2],
            "replay_binary_sha256":lock["exporter_replayer_sha256"],"outer_wall_ms":wall_ms,
            "pinned_replay_stage_sha256":pinned_hashes.as_ref().map(|hashes|&hashes[2]),
            "unpinned_replay_stage_sha256":natural_hashes.as_ref().map(|hashes|&hashes[3]),
            "stage_wall_ms":stage_start.elapsed().as_millis(),"outer_wall_limit_seconds":outer_seconds,
            "checked_relation_row_decimal":checked["relation_row_decimal"],
            "total_index_calculus_runtime_ms":Value::Null,"selected_best_total_runtime":Value::Null,
        });
        write_new_json(&output.join("replay-stage.json"), &receipt)?;
        Ok(receipt)
    })();
    if let Err(error) = &run {
        let reason = error.to_string();
        let status = if reason.starts_with("CLEANUP_FAILURE") {
            "CLEANUP_FAILURE"
        } else if reason.starts_with("UNKNOWN_replay_wall_cap") {
            "UNKNOWN_replay_wall_cap"
        } else if reason.starts_with("INVALID_MODEL_OR_REPLAY_FAILURE") {
            "INVALID_MODEL_OR_REPLAY_FAILURE"
        } else {
            "PRODUCER_FAILURE"
        };
        let _ = write_new_json(
            &output.join("failure.json"),
            &json!({
                "schema":if mode == "natural" {"n83.k600-m6-native-natural-replay-failure/v1"}
                    else {"n83.k600-m6-native-constructed-replay-failure/v1"},"status":status,
                "mode":mode,"reason":reason,"source_commit":commit,"stage_wall_ms":stage_start.elapsed().as_millis(),
                "selected_best_total_runtime":Value::Null,
            }),
        );
    }
    run
}

fn replay_pinned(
    lock_path: &Path,
    preflight_lock_path: &Path,
    preflight_root: &Path,
    export_root: &Path,
    solver_root: &Path,
    output: &Path,
    outer_seconds: u64,
) -> Result<Value> {
    replay_constructed(
        lock_path,
        preflight_lock_path,
        preflight_root,
        export_root,
        solver_root,
        output,
        outer_seconds,
        "positive-pinned",
        None,
        None,
    )
}

fn audited_pinned_replay(
    lock: &Value,
    lock_path: &Path,
    preflight_hashes: &[String],
    pinned_export_root: &Path,
    pinned_solver_root: &Path,
    pinned_replay_root: &Path,
) -> Result<Vec<String>> {
    let export_root = fs::canonicalize(pinned_export_root)?;
    let solver_root = fs::canonicalize(pinned_solver_root)?;
    let replay_root = fs::canonicalize(pinned_replay_root)?;
    let (_, export_hashes) = audited_constructed_export(
        lock,
        lock_path,
        preflight_hashes,
        "positive-pinned",
        &export_root,
    )?;
    let (_, _, solver_hashes) = audited_constructed_solver(
        lock,
        lock_path,
        preflight_hashes,
        &export_hashes,
        &solver_root,
        "positive-pinned",
        None,
    )?;
    if replay_root.join("failure.json").exists() {
        return Err("pinned replay has a terminal failure receipt".into());
    }
    let stage_path = replay_root.join("replay-stage.json");
    let result_path = replay_root.join("replay.json");
    let stage: Value = serde_json::from_slice(&fs::read(&stage_path)?)?;
    let result: Value = serde_json::from_slice(&fs::read(&result_path)?)?;
    let stage_hash = sha256_file(&stage_path)?;
    let result_hash = sha256_file(&result_path)?;
    if stage["schema"] != "n83.k600-m6-native-pinned-replay-stage/v1"
        || stage["status"] != "PASS_PINNED_MODEL_REPLAY"
        || stage["mode"] != "positive-pinned"
        || !stage["pinned_replay_stage_sha256"].is_null()
        || stage["source_commit"] != lock["source_commit"]
        || stage["lock_sha256"] != sha256_file(lock_path)?
        || stage["preflight_outer_sha256"] != preflight_hashes[1]
        || stage["export_stage_sha256"] != export_hashes[0]
        || stage["solver_receipt_sha256"] != solver_hashes[0]
        || stage["replay_json_sha256"] != result_hash
        || stage["xcnf_sha256"] != export_hashes[2]
        || stage["replay_binary_sha256"] != lock["exporter_replayer_sha256"]
        || stage["checked_relation_row_decimal"] != result["relation_row_decimal"]
        || !stage["total_index_calculus_runtime_ms"].is_null()
        || !stage["selected_best_total_runtime"].is_null()
        || result["schema"] != "n83.k600-m6-model-replay/v2"
        || result["status"] != "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY"
        || result["mode"] != "positive-pinned"
        || result["source_commit"] != lock["source_commit"]
        || result["export_binary_sha256"] != lock["exporter_replayer_sha256"]
        || result["replay_binary_sha256"] != lock["exporter_replayer_sha256"]
        || result["solver_stdout_sha256"] != solver_hashes[1]
        || result["xcnf_sha256"] != export_hashes[2]
        || result["object_sha256"] != OBJECT_SHA
        || result["relation_row_decimal"]
            .as_array()
            .is_none_or(|v| v.len() != 600)
        || result["chosen_indices"]
            .as_array()
            .is_none_or(|v| v.len() != 6)
        || result["subgroup_order"] != "2417851639230796216685689"
        || !result["selected_best_total_runtime"].is_null()
    {
        return Err("pinned replay result does not bind the frozen positive control".into());
    }
    Ok(vec![
        export_hashes[0].clone(),
        solver_hashes[0].clone(),
        stage_hash,
        result_hash,
    ])
}

fn audited_unpinned_replay(
    lock: &Value,
    lock_path: &Path,
    preflight_hashes: &[String],
    pinned_export_root: &Path,
    pinned_solver_root: &Path,
    pinned_replay_root: &Path,
    unpinned_export_root: &Path,
    unpinned_solver_root: &Path,
    unpinned_replay_root: &Path,
) -> Result<Vec<String>> {
    let pinned = audited_pinned_replay(
        lock,
        lock_path,
        preflight_hashes,
        pinned_export_root,
        pinned_solver_root,
        pinned_replay_root,
    )?;
    let export_root = fs::canonicalize(unpinned_export_root)?;
    let solver_root = fs::canonicalize(unpinned_solver_root)?;
    let replay_root = fs::canonicalize(unpinned_replay_root)?;
    let (_, export_hashes) = audited_constructed_export(
        lock,
        lock_path,
        preflight_hashes,
        "positive-unpinned",
        &export_root,
    )?;
    let (_, _, solver_hashes) = audited_constructed_solver(
        lock,
        lock_path,
        preflight_hashes,
        &export_hashes,
        &solver_root,
        "positive-unpinned",
        Some(&pinned[2]),
    )?;
    if replay_root.join("failure.json").exists() {
        return Err("unpinned replay has a terminal failure receipt".into());
    }
    let stage_path = replay_root.join("replay-stage.json");
    let result_path = replay_root.join("replay.json");
    let stage: Value = serde_json::from_slice(&fs::read(&stage_path)?)?;
    let result: Value = serde_json::from_slice(&fs::read(&result_path)?)?;
    let export: Value =
        serde_json::from_slice(&fs::read(export_root.join("positive-unpinned.json"))?)?;
    let pinned_export: Value = serde_json::from_slice(&fs::read(
        Path::new(pinned_export_root).join("positive-pinned.json"),
    )?)?;
    let stage_hash = sha256_file(&stage_path)?;
    let result_hash = sha256_file(&result_path)?;
    if stage["schema"] != "n83.k600-m6-native-unpinned-replay-stage/v1"
        || stage["status"] != "PASS_UNPINNED_MODEL_REPLAY"
        || stage["mode"] != "positive-unpinned"
        || stage["pinned_replay_stage_sha256"] != pinned[2]
        || stage["source_commit"] != lock["source_commit"]
        || stage["lock_sha256"] != sha256_file(lock_path)?
        || stage["preflight_outer_sha256"] != preflight_hashes[1]
        || stage["export_stage_sha256"] != export_hashes[0]
        || stage["solver_receipt_sha256"] != solver_hashes[0]
        || stage["replay_json_sha256"] != result_hash
        || stage["xcnf_sha256"] != export_hashes[2]
        || stage["replay_binary_sha256"] != lock["exporter_replayer_sha256"]
        || stage["checked_relation_row_decimal"] != result["relation_row_decimal"]
        || !stage["total_index_calculus_runtime_ms"].is_null()
        || !stage["selected_best_total_runtime"].is_null()
        || result["schema"] != "n83.k600-m6-model-replay/v2"
        || result["status"] != "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY"
        || result["mode"] != "positive-unpinned"
        || result["source_commit"] != lock["source_commit"]
        || result["export_binary_sha256"] != lock["exporter_replayer_sha256"]
        || result["replay_binary_sha256"] != lock["exporter_replayer_sha256"]
        || result["solver_stdout_sha256"] != solver_hashes[1]
        || result["xcnf_sha256"] != export_hashes[2]
        || result["object_sha256"] != OBJECT_SHA
        || result["target"] != export["target"]
        || result["target"] != pinned_export["target"]
        || export["positive"] != pinned_export["positive"]
        || result["checked_cnf_clauses"] != export["shape"]["clauses"]
        || result["checked_xor_rows"] != export["shape"]["xor_rows"]
        || result["relation_row_decimal"]
            .as_array()
            .is_none_or(|v| v.len() != 600)
        || result["chosen_indices"]
            .as_array()
            .is_none_or(|v| v.len() != 6)
        || result["subgroup_order"] != "2417851639230796216685689"
        || !result["selected_best_total_runtime"].is_null()
    {
        return Err(
            "unpinned replay result or constructed target differs from pinned control".into(),
        );
    }
    Ok(vec![
        pinned[2].clone(),
        export_hashes[0].clone(),
        solver_hashes[0].clone(),
        stage_hash,
        result_hash,
    ])
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
        Some("freeze") if args.len() == 8 => {
            let output = PathBuf::from(&args[7]);
            let result = freeze(
                Path::new(&args[2]), Path::new(&args[3]), Path::new(&args[4]),
                &args[5], Path::new(&args[6]), &output,
            );
            (output, result, false)
        }
        Some("export") if args.len() == 8 => {
            let output = PathBuf::from(&args[6]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[7].parse()?;
                export(Path::new(&args[2]), Path::new(&args[3]), Path::new(&args[4]), &args[5], &output, seconds,None)
            })();
            (output, result, false)
        }
        Some("export-natural") if args.len() == 13 => {
            let output = PathBuf::from(&args[11]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[12].parse()?;
                export(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    "natural",&output,seconds,
                    Some((Path::new(&args[5]),Path::new(&args[6]),Path::new(&args[7]),
                        Path::new(&args[8]),Path::new(&args[9]),Path::new(&args[10]))))
            })();
            (output,result,false)
        }
        Some("solve") if args.len() == 9 => {
            let output = PathBuf::from(&args[7]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[8].parse()?;
                solve(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    &args[5],Path::new(&args[6]),&output,seconds,None,None)
            })();
            (output,result,false)
        }
        Some("solve-unpinned") if args.len() == 11 => {
            let output = PathBuf::from(&args[9]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[10].parse()?;
                solve(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    "positive-unpinned",Path::new(&args[5]),&output,seconds,
                    Some((Path::new(&args[6]),Path::new(&args[7]),Path::new(&args[8]))),None)
            })();
            (output,result,false)
        }
        Some("solve-natural") if args.len() == 14 => {
            let output = PathBuf::from(&args[12]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[13].parse()?;
                solve(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    "natural",Path::new(&args[5]),&output,seconds,None,
                    Some((Path::new(&args[6]),Path::new(&args[7]),Path::new(&args[8]),
                        Path::new(&args[9]),Path::new(&args[10]),Path::new(&args[11]))))
            })();
            (output,result,false)
        }
        Some("replay-pinned") if args.len() == 9 => {
            let output = PathBuf::from(&args[7]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[8].parse()?;
                replay_pinned(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    Path::new(&args[5]),Path::new(&args[6]),&output,seconds)
            })();
            (output,result,false)
        }
        Some("replay-unpinned") if args.len() == 12 => {
            let output = PathBuf::from(&args[10]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[11].parse()?;
                replay_constructed(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    Path::new(&args[5]),Path::new(&args[6]),&output,seconds,"positive-unpinned",
                    Some((Path::new(&args[7]),Path::new(&args[8]),Path::new(&args[9]))),None)
            })();
            (output,result,false)
        }
        Some("replay-natural") if args.len() == 15 => {
            let output = PathBuf::from(&args[13]);
            let result = (|| -> Result<Value> {
                let seconds: u64 = args[14].parse()?;
                replay_constructed(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),
                    Path::new(&args[5]),Path::new(&args[6]),&output,seconds,"natural",None,
                    Some((Path::new(&args[7]),Path::new(&args[8]),Path::new(&args[9]),
                        Path::new(&args[10]),Path::new(&args[11]),Path::new(&args[12]))))
            })();
            (output,result,false)
        }
        _ => return Err("usage: stage FROZEN_METADATA_DIR NEW_OUTPUT_DIR | freeze STAGED_DIR EXPORTER_REPLAYER_BINARY SOLVER_BINARY IMAGE_ID COMMITTED_PREFLIGHT_LOCK NEW_LOCK_JSON | export ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN positive-pinned|positive-unpinned NEW_OUTPUT OUTER_SECONDS | solve ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN positive-pinned EXPORT_DIR NEW_OUTPUT OUTER_SECONDS | replay-pinned ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN EXPORT_DIR SOLVER_DIR NEW_OUTPUT OUTER_SECONDS | solve-unpinned ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN UNPINNED_EXPORT PINNED_EXPORT PINNED_SOLVER PINNED_REPLAY NEW_OUTPUT OUTER_SECONDS | replay-unpinned ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN UNPINNED_EXPORT UNPINNED_SOLVER PINNED_EXPORT PINNED_SOLVER PINNED_REPLAY NEW_OUTPUT OUTER_SECONDS | export-natural ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN PINNED_EXPORT PINNED_SOLVER PINNED_REPLAY UNPINNED_EXPORT UNPINNED_SOLVER UNPINNED_REPLAY NEW_OUTPUT OUTER_SECONDS | solve-natural ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN NATURAL_EXPORT PINNED_EXPORT PINNED_SOLVER PINNED_REPLAY UNPINNED_EXPORT UNPINNED_SOLVER UNPINNED_REPLAY NEW_OUTPUT OUTER_SECONDS | replay-natural ARM_LOCK PREFLIGHT_LOCK PREFLIGHT_RUN NATURAL_EXPORT NATURAL_SOLVER PINNED_EXPORT PINNED_SOLVER PINNED_REPLAY UNPINNED_EXPORT UNPINNED_SOLVER UNPINNED_REPLAY NEW_OUTPUT OUTER_SECONDS".into()),
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
    fn runtime_image_label_uses_build_revision_across_all_arms() {
        let image = format!("sha256:{}", "b".repeat(64));
        let build = "a".repeat(40);
        let evidence = "c".repeat(40);
        let mut lock = json!({"image_id":image,"build_source_commit":build,
            "source_commit":evidence});
        assert_eq!(
            frozen_image_identity(&lock).unwrap(),
            format!("{image} linux/arm64 {build}")
        );
        lock["build_source_commit"] = Value::Null;
        assert!(frozen_image_identity(&lock).is_err());
    }

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
            1,
            None,
        )
        .is_err());
        assert!(!output.exists());
        assert!(export(
            Path::new("/missing-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            "natural",
            &output,
            1,
            Some((
                Path::new("/pinned-export"),
                Path::new("/pinned-solver"),
                Path::new("/pinned-replay"),
                Path::new("/unpinned-export"),
                Path::new("/unpinned-solver"),
                Path::new("/unpinned-replay")
            ))
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
        let limits = json!({"max_variables":125_000,"max_domain_clauses":350_000,
            "max_models":1,"conflict_budget":10_000});
        let preflight = json!({
            "schema":"n83.k600-m6-native-preflight-lock/v1", "source_commit":commit,
            "image_id":image,"object_sha256":OBJECT_SHA,"point_set_blake3":POINT_SET,
            "store_remote":REMOTE,"store_name":STORE_NAME,"max_trials":0,
            "max_variables":125_000,"max_domain_clauses":350_000,"max_models":1,
            "conflict_budget":10_000,"memory_mib":4096,"cpu_limit":1,"summands":6,
            "base_sha256":{"manifest.json":"manifest"},
        });
        let config = json!({
            "schema":"n83.primary-store-chain-config/v1","source_commit":commit,
            "max_trials":0,"orbit_columns":600,"summands":6,
            "policy":"public_x_hash","seed":2_026_100_801u64,
            "study":"koblitz_n83_factor_base_sweep_20261008",
            "input_kind":"artifact_store_v3","curve_a":0,"fixture":0,
            "strategy":"orbit-chain-s3","chain_s3_limits":limits,
            "source_attestation_mode":"supervised_snapshot",
            "memory_cgroup_limit_bytes":4_294_967_296u64,
            "memory_cgroup_swap_limit_bytes":0,
        });
        let summary = json!({
            "schema":"n83.primary-store-chain/v1","status":"PREFLIGHT_ONLY",
            "source_commit":commit,"store_name":STORE_NAME,"point_set_blake3":POINT_SET,
            "max_trials":0,"solver_stage_executed":false,
            "memory_cgroup_peak_bytes":1_048_576u64,
            "study":"koblitz_n83_factor_base_sweep_20261008",
            "input_kind":"artifact_store_v3","curve_a":0,"fixture":0,
            "strategy":"orbit-chain-s3","orbit_columns":600,"summands":6,
            "policy":"public_x_hash","seed":2_026_100_801u64,
            "source_attestation_mode":"supervised_snapshot","object":STORE_NAME,
            "store_manifest_sha256":"manifest","chain_s3_limits":limits,
            "report":{"trials":0,"relations":0,"sat_calls":0,"matrix_rows":0},
            "attempt_records":[],"relation_matrix":[],"verified_log":Value::Null,
            "memory_cgroup_limit_bytes":4_294_967_296u64,
            "memory_cgroup_swap_limit_bytes":0,
            "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null,
        });
        let lock_path = root.join("preflight-lock.json");
        let config_path = worker.join("config.json");
        let summary_path = worker.join("summary.json");
        write_new_json(&lock_path, &preflight).unwrap();
        let arm = json!({"source_commit":"c".repeat(40),"build_source_commit":commit,
            "image_id":image,"preflight_lock_sha256":sha256_file(&lock_path).unwrap()});
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
        let mut wrong_summary = summary;
        wrong_summary["fixture"] = json!(1);
        fs::write(&summary_path, serde_json::to_vec(&wrong_summary).unwrap()).unwrap();
        let mut changed_outer = outer;
        changed_outer["worker_summary_sha256"] = json!(sha256_file(&summary_path).unwrap());
        fs::write(
            run.join("outer.json"),
            serde_json::to_vec(&changed_outer).unwrap(),
        )
        .unwrap();
        assert!(preflight_gate(&arm, &lock_path, &run).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn solver_container_args_pin_xcnf_conflicts_and_isolation() {
        let lock = json!({"image_id":format!("sha256:{}","a".repeat(64))});
        let args = docker_solver_args(
            &lock,
            Path::new("/solver-bin"),
            Path::new("/formula.xcnf"),
            "n83-solver-check",
            95,
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
            ("--threads", "1"),
            ("--random", "20261115"),
            ("--maxtime", "95"),
            ("--maxconfl", "10000"),
            ("--printsol", "1"),
        ] {
            assert!(has(key, value), "missing {key} {value}");
        }
        assert!(args.iter().any(|arg| arg == "--read-only"));
        assert!(has(
            "--mount",
            "type=bind,src=/solver-bin,dst=/solver,readonly"
        ));
        assert!(has(
            "--mount",
            "type=bind,src=/formula.xcnf,dst=/input.xcnf,readonly"
        ));
        assert_eq!(args.last().map(String::as_str), Some("/input.xcnf"));
    }

    #[test]
    fn unpinned_solver_waits_for_pinned_model_replay() {
        let output =
            std::env::temp_dir().join(format!("n83-k600-m6-unpinned-gate-{}", std::process::id()));
        assert!(!output.exists());
        assert!(solve(
            Path::new("/missing-arm-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            "positive-unpinned",
            Path::new("/missing-export"),
            &output,
            10,
            None,
            None,
        )
        .is_err());
        assert!(!output.exists());
        assert!(solve(
            Path::new("/missing-arm-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            "positive-unpinned",
            Path::new("/missing-export"),
            &output,
            10,
            Some((
                Path::new("/missing-pinned-export"),
                Path::new("/missing-pinned-solver"),
                Path::new("/missing-pinned-replay")
            )),
            None,
        )
        .is_err());
        assert!(!output.exists());
    }

    #[test]
    fn natural_solver_requires_both_replayed_controls() {
        let output = std::env::temp_dir().join(format!(
            "n83-k600-m6-natural-solver-gate-{}",
            std::process::id()
        ));
        assert!(!output.exists());
        assert!(solve(
            Path::new("/missing-arm-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            "natural",
            Path::new("/missing-export"),
            &output,
            10,
            None,
            None
        )
        .is_err());
        assert!(!output.exists());
    }

    #[test]
    fn pinned_replay_container_has_frozen_inputs_and_hard_caps() {
        let lock = json!({"image_id":format!("sha256:{}","a".repeat(64)),
            "source_commit":"b".repeat(40),"exporter_replayer_sha256":"c".repeat(64)});
        let args = docker_replay_args(
            &lock,
            "positive-pinned",
            Path::new("/source"),
            Path::new("/git-common"),
            Path::new("/panel"),
            Path::new("/export"),
            Path::new("/solver-receipt"),
            Path::new("/replayer"),
            Path::new("/out"),
            "n83-replay-check",
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
            ("--mount", "type=bind,src=/source,dst=/src,readonly"),
            (
                "--mount",
                "type=bind,src=/git-common,dst=/git-common,readonly",
            ),
            ("--mount", "type=bind,src=/panel,dst=/base,readonly"),
            ("--mount", "type=bind,src=/export,dst=/export,readonly"),
            (
                "--mount",
                "type=bind,src=/solver-receipt,dst=/solver-receipt,readonly",
            ),
            ("--mount", "type=bind,src=/replayer,dst=/replayer,readonly"),
            ("--mount", "type=bind,src=/out,dst=/out"),
        ] {
            assert!(has(key, value), "missing {key} {value}");
        }
        assert!(args.iter().any(|arg| arg == "--read-only"));
        assert!(args.iter().any(|arg| arg == "primary-k600-m6-replay"));
        assert_eq!(args.last().map(String::as_str), Some("/out/replay.json"));
        let unpinned = docker_replay_args(
            &lock,
            "positive-unpinned",
            Path::new("/source"),
            Path::new("/git-common"),
            Path::new("/panel"),
            Path::new("/export"),
            Path::new("/solver-receipt"),
            Path::new("/replayer"),
            Path::new("/out"),
            "n83-replay-unpinned-check",
        );
        assert!(unpinned
            .iter()
            .any(|arg| arg == "/export/positive-unpinned.json"));
        assert!(unpinned
            .iter()
            .any(|arg| arg == "/export/positive-unpinned.xcnf"));
        let natural = docker_replay_args(
            &lock,
            "natural",
            Path::new("/source"),
            Path::new("/git-common"),
            Path::new("/panel"),
            Path::new("/export"),
            Path::new("/solver-receipt"),
            Path::new("/replayer"),
            Path::new("/out"),
            "n83-replay-natural-check",
        );
        assert!(natural.iter().any(|arg| arg == "/export/natural.json"));
        assert!(natural.iter().any(|arg| arg == "/export/natural.xcnf"));
    }

    #[test]
    fn replay_requires_existing_pinned_sat_receipt_before_creating_output() {
        let output =
            std::env::temp_dir().join(format!("n83-k600-m6-replay-gate-{}", std::process::id()));
        assert!(!output.exists());
        assert!(replay_pinned(
            Path::new("/missing-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            Path::new("/missing-export"),
            Path::new("/missing-solver"),
            &output,
            10
        )
        .is_err());
        assert!(!output.exists());
        assert!(replay_constructed(
            Path::new("/missing-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            Path::new("/missing-export"),
            Path::new("/missing-solver"),
            &output,
            10,
            "positive-unpinned",
            None,
            None,
        )
        .is_err());
        assert!(!output.exists());
    }

    #[test]
    fn natural_replay_requires_unpinned_control_and_sat_receipt() {
        let output = std::env::temp_dir().join(format!(
            "n83-k600-m6-natural-replay-gate-{}",
            std::process::id()
        ));
        assert!(!output.exists());
        assert!(replay_constructed(
            Path::new("/missing-lock"),
            Path::new("/missing-preflight-lock"),
            Path::new("/missing-preflight-run"),
            Path::new("/missing-natural-export"),
            Path::new("/missing-natural-solver"),
            &output,
            10,
            "natural",
            None,
            None
        )
        .is_err());
        assert!(!output.exists());
    }

    #[test]
    fn pinned_sat_receipt_rejects_changed_model_stdout() {
        let root =
            std::env::temp_dir().join(format!("n83-k600-m6-sat-audit-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let lock_path = root.join("lock.json");
        fs::write(&lock_path, b"frozen lock").unwrap();
        let stdout_path = root.join("solver.stdout");
        let stderr_path = root.join("solver.stderr");
        fs::write(&stdout_path, b"s SATISFIABLE\nv 1 0\n").unwrap();
        fs::write(&stderr_path, b"").unwrap();
        let preflight = vec!["lock".to_string(), "outer".to_string()];
        let export = vec!["stage".to_string(), "json".to_string(), "xcnf".to_string()];
        let lock = json!({"source_commit":"a".repeat(40),"image_id":"image",
            "solver_sha256":"solver"});
        let mut receipt = json!({
            "schema":"n83.k600-m6-native-solver/v1","status":"SAT_PENDING_XCNF_REPLAY",
            "mode":"positive-pinned","source_commit":lock["source_commit"],
            "lock_sha256":sha256_file(&lock_path).unwrap(),"image_id":"image",
            "preflight_outer_sha256":"outer","export_stage_sha256":"stage",
            "xcnf_sha256":"xcnf","solver_binary_sha256":"solver",
            "stdout_sha256":sha256_file(&stdout_path).unwrap(),
            "stderr_sha256":sha256_file(&stderr_path).unwrap(),
            "exit_code":10,"conflict_budget":10_000,"threads":1,
            "random_seed":20_261_115u64,"model_replayed":false,
            "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null,
        });
        write_new_json(&root.join("solver-receipt.json"), &receipt).unwrap();
        assert!(audited_constructed_solver(
            &lock,
            &lock_path,
            &preflight,
            &export,
            &root,
            "positive-pinned",
            None,
        )
        .is_ok());
        fs::write(&stdout_path, b"s SATISFIABLE\nv -1 0\n").unwrap();
        assert!(audited_constructed_solver(
            &lock,
            &lock_path,
            &preflight,
            &export,
            &root,
            "positive-pinned",
            None,
        )
        .is_err());
        fs::write(&stdout_path, b"s SATISFIABLE\nv 1 0\n").unwrap();
        receipt["mode"] = json!("positive-unpinned");
        receipt["pinned_replay_stage_sha256"] = json!("pinned-stage");
        fs::write(
            root.join("solver-receipt.json"),
            serde_json::to_vec(&receipt).unwrap(),
        )
        .unwrap();
        assert!(audited_constructed_solver(
            &lock,
            &lock_path,
            &preflight,
            &export,
            &root,
            "positive-unpinned",
            Some("pinned-stage")
        )
        .is_ok());
        assert!(audited_constructed_solver(
            &lock,
            &lock_path,
            &preflight,
            &export,
            &root,
            "positive-unpinned",
            Some("different-stage")
        )
        .is_err());
        receipt["mode"] = json!("natural");
        receipt["pinned_replay_stage_sha256"] = Value::Null;
        receipt["unpinned_replay_stage_sha256"] = json!("unpinned-stage");
        fs::write(
            root.join("solver-receipt.json"),
            serde_json::to_vec(&receipt).unwrap(),
        )
        .unwrap();
        assert!(audited_natural_solver(
            &lock,
            &lock_path,
            &preflight,
            &export,
            "unpinned-stage",
            &root
        )
        .is_ok());
        assert!(audited_natural_solver(
            &lock,
            &lock_path,
            &preflight,
            &export,
            "wrong-stage",
            &root
        )
        .is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn committed_preflight_lock_allows_only_one_evidence_commit() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let repo = std::env::temp_dir().join(format!(
            "n83-k600-two-revision-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&repo).unwrap();
        let git = |args: &[&str]| -> String {
            let out = Command::new("git")
                .args(args)
                .current_dir(&repo)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout).unwrap().trim().to_owned()
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "N83 Test"]);
        git(&["config", "user.email", "n83@example.invalid"]);
        fs::write(repo.join("source.txt"), b"frozen source").unwrap();
        git(&["add", "source.txt"]);
        git(&["commit", "-qm", "source"]);
        let source = git(&["rev-parse", "HEAD"]);
        let image = format!("sha256:{}", "b".repeat(64));
        let lock_path = repo.join("preflight-lock.json");
        write_new_json(
            &lock_path,
            &json!({
                "schema":"n83.k600-m6-native-preflight-lock/v1","status":"PREPARED_ONLY",
                "source_root":repo,"source_commit":source,"image_id":image,
                "source_sha256":{"source.txt":sha256_file(&repo.join("source.txt")).unwrap()}
            }),
        )
        .unwrap();
        git(&["add", "preflight-lock.json"]);
        git(&["commit", "-qm", "evidence lock"]);
        let head = git(&["rev-parse", "HEAD"]);
        assert_ne!(source, head);
        assert_eq!(
            committed_preflight_revision(&repo, &lock_path, &head, &image)
                .unwrap()
                .0,
            source
        );
        assert!(committed_preflight_revision(&repo, &lock_path, &head, "sha256:wrong").is_err());
        fs::write(&lock_path, b"substituted lock").unwrap();
        assert!(committed_preflight_revision(&repo, &lock_path, &head, &image).is_err());
        git(&["checkout", "--", "preflight-lock.json"]);
        fs::write(repo.join("source.txt"), b"changed source").unwrap();
        assert!(committed_preflight_revision(&repo, &lock_path, &head, &image).is_err());
        git(&["checkout", "--", "source.txt"]);
        fs::write(repo.join("extra.txt"), b"unrelated commit").unwrap();
        git(&["add", "extra.txt"]);
        git(&["commit", "-qm", "unrelated"]);
        let later = git(&["rev-parse", "HEAD"]);
        assert!(committed_preflight_revision(&repo, &lock_path, &later, &image).is_err());
        git(&["checkout", "-qb", "unrelated", &source]);
        assert!(committed_preflight_revision(&repo, &lock_path, &source, &image).is_err());
        fs::remove_dir_all(repo).unwrap();
    }
}
