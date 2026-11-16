//! Native outer admission and zero-trial runner for the frozen primary N83 K600 m6 cell.
//! `prepare` creates a lock that must be committed before `run` is allowed.
//! This does not run the constructed positive or natural arm.

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

const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const REMOTE: &str = "s3://setec-astronomy-artifacts/store";
const OBJECT_SHA: &str = "3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf";
const SOURCE_MANIFEST_SHA: &str =
    "438cd804ba99160121fecce4e072b1b71ae87ea5be58110bbb7e12752df185ff";
const POINTER_SHA: &str = "432355b8ac52b5b0f019e8bbb4dcec496f41946b1c20a413c38b8b175e243404";
const STORE_NAME: &str = "factor-bases/icv1-f2m83-tm6151469093347-debefd74/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz";
const SOURCE_FILES: &[&str] = &[
    "examples/n83_k600_m6_native_preflight.rs",
    "examples/koblitz_n83_factor_base_export.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
    "src/cryptanalysis/binary_semaev_chain_sat.rs",
    "src/cryptanalysis/sat.rs",
    "src/cryptanalysis/koblitz_index_calculus.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
];
const BASE_FILES: &[&str] = &[
    "manifest.json",
    "replay.json",
    "storage.json",
    "import.json",
];
const FIXTURE_FILES: &[&str] = &["probe-corpus.json", "probe-validation.json"];

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    Ok(hex::encode(hash.finalize()))
}

fn matches_sha256(path: &Path, expected: &str) -> bool {
    sha256_file(path).ok().as_deref() == Some(expected)
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn write_new_json(path: &Path, value: &Value) -> Result<()> {
    let mut file = File::options().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn command_output_bounded(
    program: &str,
    args: &[&str],
    root: &Path,
    timeout: Duration,
) -> Result<String> {
    let mut child = Command::new(program)
        .args(args)
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let deadline = Instant::now() + timeout;
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("{program} {:?} exceeded the command timeout", args).into());
        }
        thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "{program} {:?} failed with {}: {}",
            args,
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn command_output(program: &str, args: &[&str], root: &Path) -> Result<String> {
    command_output_bounded(program, args, root, Duration::from_secs(15))
}

fn clean_commit(repo: &Path) -> Result<String> {
    let status = command_output("git", &["status", "--porcelain"], repo)?;
    if !status.is_empty() {
        return Err("source checkout is not clean".into());
    }
    let commit = command_output("git", &["rev-parse", "HEAD"], repo)?;
    if commit.len() != 40 || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("source commit is invalid".into());
    }
    Ok(commit)
}

fn exact_hashes(root: &Path, names: &[&str]) -> Result<Value> {
    let mut map = serde_json::Map::new();
    for name in names {
        map.insert((*name).to_owned(), json!(sha256_file(&root.join(name))?));
    }
    Ok(Value::Object(map))
}

fn check_hashes(root: &Path, names: &[&str], expected: &Value) -> Result<()> {
    if &exact_hashes(root, names)? != expected {
        return Err("frozen input hash changed".into());
    }
    Ok(())
}

fn is_linux_arm64_elf(path: &Path) -> Result<bool> {
    let mut header = [0u8; 20];
    File::open(path)?.read_exact(&mut header)?;
    Ok(&header[..4] == b"\x7fELF"
        && header[4] == 2 // ELF64
        && header[5] == 1 // little endian
        && u16::from_le_bytes([header[18], header[19]]) == 183) // EM_AARCH64
}

fn validate_bundle(
    manifest: &Value,
    replay: &Value,
    storage: &Value,
    import: &Value,
    manifest_sha256: &str,
) -> Result<()> {
    let base = &manifest["base"];
    if manifest["schema"] != "n83.factor-base-panel/v3-artifact-store"
        || manifest["status"] != "completed_factor_base_object"
        || manifest["curve_id"] != CURVE
        || manifest["remote"] != REMOTE
        || manifest["custody_mode"] != "read_only_adoption"
        || manifest["source_manifest_sha256"] != SOURCE_MANIFEST_SHA
        || manifest["artifact_pointer_sha256"] != POINTER_SHA
        || base["a"] != 0
        || base["columns"] != 600
        || base["points"] != 99_600
        || base["policy"] != "public_x_hash"
        || base["seed"] != 2_026_100_801u64
        || base["closure"] != "signed_frobenius"
        || base["store_name"] != STORE_NAME
        || base["compressed_sha256"] != OBJECT_SHA
        || base["bytes"] != 3_025_798u64
        || replay["schema"] != "n83.factor-base-replay/v3-artifact-store"
        || replay["status"] != "PASS"
        || replay["manifest_sha256"] != manifest_sha256
        || replay["point_set_blake3"] != base["point_set_blake3"]
        || replay["points_checked"] != 99_600
        || replay["representatives_checked"] != 600
        || storage["schema"] != "n83.factor-base-storage/v3-artifact-store"
        || storage["status"] != "PASS"
        || storage["manifest_sha256"] != manifest_sha256
        || storage["custody_mode"] != "read_only_adoption"
        || storage["remote"] != REMOTE
        || storage["sha256"] != OBJECT_SHA
        || storage["bytes"] != 3_025_798u64
        || storage["fresh_download_sha256"] != OBJECT_SHA
        || storage["fresh_download_bytes"] != 3_025_798u64
        || storage["uploaded"] != false
        || import["schema"] != "n83.factor-base-store-import/v3"
        || import["status"] != "PASS"
        || import["points_checked"] != 99_600
        || import["orbit_columns_checked"] != 600
        || import["store_name"] != STORE_NAME
        || import["point_set_blake3"] != base["point_set_blake3"]
        || !import["selected_best_total_runtime"].is_null()
    {
        return Err("K600 S3 adoption bundle differs from frozen cell".into());
    }
    Ok(())
}

fn prepare(args: &[String]) -> Result<()> {
    if args.len() != 8 {
        return Err(
            "prepare REPO BASE_BUNDLE FIXTURES WORKER_ELF IMAGE_ID WORKER_SECONDS NEW_LOCK_JSON"
                .into(),
        );
    }
    let repo = fs::canonicalize(&args[1])?;
    let base = fs::canonicalize(&args[2])?;
    let fixtures = fs::canonicalize(&args[3])?;
    let binary = fs::canonicalize(&args[4])?;
    let image = &args[5];
    let worker_seconds: u64 = args[6].parse()?;
    let lock_path = Path::new(&args[7]);
    if worker_seconds == 0
        || worker_seconds > 300
        || lock_path.exists()
        || !lock_path.is_absolute()
        || !image.starts_with("sha256:")
        || image.len() != 71
        || !image[7..].bytes().all(|b| b.is_ascii_hexdigit())
        || !is_linux_arm64_elf(&binary)?
    {
        return Err("K600 preflight binary, image, cap or lock path invalid".into());
    }
    let commit = clean_commit(&repo)?;
    let lock_parent = fs::canonicalize(lock_path.parent().ok_or("lock parent missing")?)?;
    if !lock_parent.starts_with(&repo) {
        return Err("execution lock must be written in the clean source checkout".into());
    }
    for path in SOURCE_FILES {
        if !repo.join(path).is_file() {
            return Err(format!("missing source {path}").into());
        }
    }
    let inspected = command_output(
        "docker",
        &["image", "inspect", image, "--format", "{{.Id}}"],
        &repo,
    )?;
    if inspected != *image {
        return Err("Docker image ID differs".into());
    }
    let manifest = read_json(&base.join("manifest.json"))?;
    validate_bundle(
        &manifest,
        &read_json(&base.join("replay.json"))?,
        &read_json(&base.join("storage.json"))?,
        &read_json(&base.join("import.json"))?,
        &sha256_file(&base.join("manifest.json"))?,
    )?;
    let config = crypto_lib::store::Config::discover()?;
    if config.remote.as_ref() != Some(&RemoteSpec::parse(REMOTE)?) {
        return Err("store remote differs".into());
    }
    let store = Store::open(config.clone())?;
    let (indexed, digest, bytes) = store.lookup(STORE_NAME)?;
    if indexed.as_deref() != Some(STORE_NAME) || digest != OBJECT_SHA || bytes != Some(3_025_798) {
        return Err("store index differs from K600 object".into());
    }
    let lock = json!({
        "schema":"n83.k600-m6-native-preflight-lock/v1", "status":"PREPARED_ONLY",
        "source_commit":commit, "source_root":repo, "source_sha256":exact_hashes(&repo,SOURCE_FILES)?,
        "base_dir":base, "base_sha256":exact_hashes(&base,BASE_FILES)?,
        "point_set_blake3":manifest["base"]["point_set_blake3"],
        "fixture_dir":fixtures, "fixture_sha256":exact_hashes(&fixtures,FIXTURE_FILES)?,
        "store_index":config.index_path, "store_index_sha256":sha256_file(&config.index_path)?,
        "store_remote":REMOTE, "store_name":STORE_NAME, "object_sha256":OBJECT_SHA,
        "worker_elf":binary, "worker_sha256":sha256_file(&binary)?,
        "supervisor_sha256":sha256_file(&std::env::current_exe()?)?,
        "image_id":image, "worker_seconds":worker_seconds,
        "memory_mib":4096, "cpu_limit":1, "max_trials":0, "summands":6,
        "max_variables":125000, "max_domain_clauses":350000,
        "max_models":1, "conflict_budget":10000,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(lock_path, &lock)
}

fn docker_args(lock: &Value, out: &Path, source: &Path) -> Vec<String> {
    let mount = |src: &str, dst: &str, readonly: bool| {
        format!(
            "type=bind,src={src},dst={dst}{}",
            if readonly { ",readonly" } else { "" }
        )
    };
    let source_text = source.to_string_lossy().to_string();
    let out_text = out.to_string_lossy().to_string();
    vec![
        "run".into(),
        "--rm".into(),
        "--pull".into(),
        "never".into(),
        "--name".into(),
        "n83-k600-m6-native-preflight".into(),
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
        mount(lock["base_dir"].as_str().unwrap(), "/base", true),
        "--mount".into(),
        mount(lock["fixture_dir"].as_str().unwrap(), "/fixtures", true),
        "--mount".into(),
        mount(lock["worker_elf"].as_str().unwrap(), "/worker", true),
        "--mount".into(),
        mount(&source_text, "/source", true),
        "--mount".into(),
        mount(&format!("{out_text}/store-index.json"), "/index.json", true),
        "--mount".into(),
        mount(&format!("{out_text}/fresh-cache"), "/cache", true),
        "--mount".into(),
        mount(&format!("{out_text}/worker-output"), "/out", false),
        "--env".into(),
        "ICV1_FROZEN_SOURCE_DIR=/source".into(),
        "--env".into(),
        "CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store".into(),
        "--env".into(),
        "CRYPTO_STORE_INDEX=/index.json".into(),
        "--env".into(),
        "CRYPTO_STORE_CACHE=/cache".into(),
        lock["image_id"].as_str().unwrap().into(),
        "/worker".into(),
        "primary-store-v3-orbit-chain-cold".into(),
        "/base".into(),
        "/fixtures".into(),
        "600".into(),
        "6".into(),
        "0".into(),
        "125000".into(),
        "350000".into(),
        "1".into(),
        "10000".into(),
        lock["worker_seconds"].as_u64().unwrap().to_string(),
        "4096".into(),
        "/out/run".into(),
    ]
}

fn run(args: &[String]) -> Result<Value> {
    if args.len() != 4 {
        return Err("run COMMITTED_LOCK_JSON NEW_OUTPUT_DIR OUTER_SECONDS".into());
    }
    let lock_path = fs::canonicalize(&args[1])?;
    let lock = read_json(&lock_path)?;
    let out = PathBuf::from(&args[2]);
    let outer_seconds: u64 = args[3].parse()?;
    if lock["schema"] != "n83.k600-m6-native-preflight-lock/v1"
        || lock["status"] != "PREPARED_ONLY"
        || out.exists()
        || !out.is_absolute()
        || outer_seconds == 0
        || outer_seconds > 300
        || outer_seconds
            <= lock["worker_seconds"]
                .as_u64()
                .ok_or("worker cap missing")?
    {
        return Err("K600 preflight lock or outer cap invalid".into());
    }
    let repo = fs::canonicalize(lock["source_root"].as_str().ok_or("source root missing")?)?;
    let base = fs::canonicalize(lock["base_dir"].as_str().ok_or("base directory missing")?)?;
    let fixtures = fs::canonicalize(
        lock["fixture_dir"]
            .as_str()
            .ok_or("fixture directory missing")?,
    )?;
    let binary = fs::canonicalize(lock["worker_elf"].as_str().ok_or("worker path missing")?)?;
    let index = fs::canonicalize(lock["store_index"].as_str().ok_or("index path missing")?)?;
    let out_parent = fs::canonicalize(out.parent().ok_or("output parent missing")?)?;
    if out_parent.starts_with(&repo)
        || out_parent.starts_with(&base)
        || out_parent.starts_with(&fixtures)
    {
        return Err("preflight output must be outside source and frozen inputs".into());
    }
    let commit = clean_commit(&repo)?;
    if !command_output(
        "git",
        &[
            "merge-base",
            "--is-ancestor",
            lock["source_commit"]
                .as_str()
                .ok_or("source commit missing")?,
            &commit,
        ],
        &repo,
    )?
    .is_empty()
    {
        return Err("source commit ancestry check returned output".into());
    }
    let relative = lock_path
        .strip_prefix(&repo)?
        .to_str()
        .ok_or("lock path encoding invalid")?;
    let tracked = Command::new("git")
        .args(["show", &format!("HEAD:{relative}")])
        .current_dir(&repo)
        .output()?;
    if !tracked.status.success() || tracked.stdout != fs::read(&lock_path)? {
        return Err("execution lock is not committed at HEAD".into());
    }
    check_hashes(&repo, SOURCE_FILES, &lock["source_sha256"])?;
    check_hashes(&base, BASE_FILES, &lock["base_sha256"])?;
    check_hashes(&fixtures, FIXTURE_FILES, &lock["fixture_sha256"])?;
    if !is_linux_arm64_elf(&binary)?
        || sha256_file(&std::env::current_exe()?)? != lock["supervisor_sha256"]
        || sha256_file(&binary)? != lock["worker_sha256"]
        || sha256_file(&index)? != lock["store_index_sha256"]
    {
        return Err("binary or index changed".into());
    }
    if command_output(
        "docker",
        &[
            "image",
            "inspect",
            lock["image_id"].as_str().ok_or("image ID missing")?,
            "--format",
            "{{.Id}}",
        ],
        &repo,
    )? != lock["image_id"]
    {
        return Err("image changed".into());
    }
    fs::create_dir(&out)?;
    let out = fs::canonicalize(&out)?;
    let started = Instant::now();
    let source = out.join("source");
    for name in SOURCE_FILES {
        let dest = source.join(name);
        fs::create_dir_all(dest.parent().ok_or("source path has no parent")?)?;
        fs::copy(repo.join(name), dest)?;
    }
    write_new_json(
        &source.join("attestation.json"),
        &json!({"schema":"n83.chain-s3-source-attestation/v1","source_commit":lock["source_commit"],"status_clean":true}),
    )?;
    let attestation_sha256 = sha256_file(&source.join("attestation.json"))?;
    fs::copy(&index, out.join("store-index.json"))?;
    let mut cfg = crypto_lib::store::Config::discover()?;
    if cfg.remote.as_ref() != Some(&RemoteSpec::parse(REMOTE)?) {
        return Err("S3 remote changed".into());
    }
    cfg.index_path = out.join("store-index.json");
    cfg.cache_dir = out.join("fresh-cache");
    let store = Store::open(cfg)?;
    let fetched = store.ensure(STORE_NAME)?;
    if !fetched.fetched || fetched.sha256 != OBJECT_SHA || fetched.bytes != 3_025_798 {
        return Err("fresh S3 object fetch differs".into());
    }
    fs::create_dir(out.join("worker-output"))?;
    let args = docker_args(&lock, &out, &source);
    let occupied = command_output(
        "docker",
        &[
            "ps",
            "-aq",
            "--filter",
            "name=^n83-k600-m6-native-preflight$",
        ],
        &repo,
    )?;
    if !occupied.is_empty() {
        return Err("frozen preflight container name is already in use".into());
    }
    write_new_json(
        &out.join("launch.json"),
        &json!({"schema":"n83.k600-m6-native-launch/v1","status":"STARTED","docker_argv":args,"lock_sha256":sha256_file(&lock_path)?,"fresh_object_sha256":fetched.sha256,"fresh_object_bytes":fetched.bytes,"selected_best_total_runtime":Value::Null}),
    )?;
    let stdout = File::options()
        .create_new(true)
        .write(true)
        .open(out.join("stdout.log"))?;
    let stderr = File::options()
        .create_new(true)
        .write(true)
        .open(out.join("stderr.log"))?;
    let mut child = Command::new("docker")
        .args(&args)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()?;
    let deadline = Instant::now() + Duration::from_secs(outer_seconds);
    let mut kill_error = None;
    let (exit, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (status.code(), false);
        }
        if Instant::now() >= deadline {
            if let Err(error) =
                command_output("docker", &["kill", "n83-k600-m6-native-preflight"], &repo)
            {
                kill_error = Some(error.to_string());
            }
            let _ = child.kill();
            break (child.wait()?.code(), true);
        }
        thread::sleep(Duration::from_millis(100));
    };
    let remaining = command_output(
        "docker",
        &[
            "ps",
            "-aq",
            "--filter",
            "name=^n83-k600-m6-native-preflight$",
        ],
        &repo,
    )
    .ok();
    let mut failure = Vec::new();
    if kill_error.is_some() {
        failure.push("container kill failed");
    }
    if remaining.as_deref().is_none_or(|value| !value.is_empty()) {
        failure.push("container cleanup unverified");
    }
    if check_hashes(&repo, SOURCE_FILES, &lock["source_sha256"]).is_err()
        || check_hashes(&source, SOURCE_FILES, &lock["source_sha256"]).is_err()
        || !matches_sha256(&source.join("attestation.json"), &attestation_sha256)
        || check_hashes(&base, BASE_FILES, &lock["base_sha256"]).is_err()
        || check_hashes(&fixtures, FIXTURE_FILES, &lock["fixture_sha256"]).is_err()
        || !matches_sha256(&binary, lock["worker_sha256"].as_str().unwrap_or_default())
        || std::env::current_exe().ok().is_none_or(|path| {
            !matches_sha256(
                &path,
                lock["supervisor_sha256"].as_str().unwrap_or_default(),
            )
        })
        || !matches_sha256(
            &index,
            lock["store_index_sha256"].as_str().unwrap_or_default(),
        )
        || !matches_sha256(
            &out.join("store-index.json"),
            lock["store_index_sha256"].as_str().unwrap_or_default(),
        )
        || !matches_sha256(&fetched.path, OBJECT_SHA)
        || command_output(
            "docker",
            &[
                "image",
                "inspect",
                lock["image_id"].as_str().unwrap(),
                "--format",
                "{{.Id}}",
            ],
            &repo,
        )
        .ok()
        .as_deref()
            != lock["image_id"].as_str()
    {
        failure.push("frozen source/input/binary changed");
    }
    let summary_path = out.join("worker-output/run/summary.json");
    let config_path = out.join("worker-output/run/config.json");
    let worker_cap_path = out.join("worker-output/run/cap.json");
    let worker_capped = worker_cap_path.exists();
    let summary = read_json(&summary_path).ok();
    let config = read_json(&config_path).ok();
    if timed_out {
        failure.push("outer wall cap");
    }
    if worker_capped {
        failure.push("worker cap receipt present");
    }
    if exit != Some(0) {
        failure.push("worker exit was not zero");
    }
    if !valid_preflight(summary.as_ref(), config.as_ref(), &lock) {
        failure.push("worker preflight receipt invalid");
    }
    let status = if failure.iter().any(|reason| {
        *reason == "container cleanup unverified" || *reason == "container kill failed"
    }) {
        "PRODUCER_FAILURE_container_cleanup"
    } else if failure.contains(&"frozen source/input/binary changed") {
        "PRODUCER_FAILURE_frozen_input_changed"
    } else if timed_out {
        "UNKNOWN_wall_cap"
    } else if worker_capped
        && read_json(&worker_cap_path).ok().is_some_and(|cap| {
            cap["schema"] == "n83.primary-cold-cap/v1"
                && cap["status"] == "UNKNOWN_budget"
                && cap["orbit_columns"] == 600
                && cap["summands"] == 6
                && cap["strategy"] == "orbit-chain-s3"
                && cap["max_trials"] == 0
                && cap["budget_seconds"] == lock["worker_seconds"]
        })
    {
        "UNKNOWN_worker_cap"
    } else if matches!(exit, Some(134 | 137)) {
        "UNKNOWN_resource_or_worker_exit"
    } else if exit != Some(0) {
        "PRODUCER_FAILURE_worker_exit"
    } else if !failure.is_empty() {
        "PRODUCER_FAILURE_worker_receipt"
    } else {
        "PREFLIGHT_ONLY"
    };
    let outer = json!({"schema":"n83.k600-m6-native-outer/v1","status":status,
        "failure_reasons":failure,"worker_exit_code":exit,"timed_out":timed_out,
        "kill_error":kill_error,
        "lock_sha256":sha256_file(&lock_path)?,"stdout_sha256":sha256_file(&out.join("stdout.log"))?,
        "stderr_sha256":sha256_file(&out.join("stderr.log"))?,
        "worker_config_sha256":if config.is_some(){Some(sha256_file(&config_path)?)}else{None},
        "worker_summary_sha256":if summary.is_some(){Some(sha256_file(&summary_path)?)}else{None},
        "worker_cap_sha256":if worker_capped{Some(sha256_file(&worker_cap_path)?)}else{None},
        "elapsed_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "selected_best_total_runtime":Value::Null});
    write_new_json(&out.join("outer.json"), &outer)?;
    Ok(outer)
}

fn valid_preflight(summary: Option<&Value>, config: Option<&Value>, lock: &Value) -> bool {
    let (Some(summary), Some(config)) = (summary, config) else {
        return false;
    };
    config["schema"] == "n83.primary-store-chain-config/v1"
        && config["study"] == "koblitz_n83_factor_base_sweep_20261008"
        && config["input_kind"] == "artifact_store_v3"
        && config["curve_a"] == 0
        && config["fixture"] == 0
        && config["base_dir"] == "/base"
        && config["fixture_dir"] == "/fixtures"
        && config["orbit_columns"] == 600
        && config["summands"] == 6
        && config["strategy"] == "orbit-chain-s3"
        && config["max_trials"] == 0
        && config["budget_seconds"] == lock["worker_seconds"]
        && config["allow_direct_relation"] == false
        && config["chain_s3_limits"]
            == json!({"max_variables":125000,"max_domain_clauses":350000,"max_models":1,"conflict_budget":10000})
        && config["policy"] == "public_x_hash"
        && config["seed"] == 2_026_100_801u64
        && config["source_commit"] == lock["source_commit"]
        && config["source_attestation_mode"] == "supervised_snapshot"
        && config["memory_cgroup_limit_bytes"] == 4_294_967_296u64
        && config["memory_cgroup_swap_limit_bytes"] == 0
        && summary["schema"] == "n83.primary-store-chain/v1"
        && summary["status"] == "PREFLIGHT_ONLY"
        && summary["study"] == "koblitz_n83_factor_base_sweep_20261008"
        && summary["input_kind"] == "artifact_store_v3"
        && summary["curve_a"] == 0
        && summary["fixture"] == 0
        && summary["strategy"] == "orbit-chain-s3"
        && summary["orbit_columns"] == 600
        && summary["summands"] == 6
        && summary["max_trials"] == 0
        && summary["budget_seconds"] == lock["worker_seconds"]
        && summary["policy"] == "public_x_hash"
        && summary["seed"] == 2_026_100_801u64
        && summary["fixture_dir"] == "/fixtures"
        && summary["point_set_blake3"] == lock["point_set_blake3"]
        && summary["store_manifest_sha256"] == lock["base_sha256"]["manifest.json"]
        && summary["chain_s3_limits"] == config["chain_s3_limits"]
        && summary["solver_stage_executed"] == false
        && summary["report"]["trials"] == 0
        && summary["report"]["relations"] == 0
        && summary["report"]["sat_calls"] == 0
        && summary["report"]["matrix_rows"] == 0
        && summary["attempt_records"]
            .as_array()
            .is_some_and(Vec::is_empty)
        && summary["relation_matrix"]
            .as_array()
            .is_some_and(Vec::is_empty)
        && summary["verified_log"].is_null()
        && summary["store_name"] == STORE_NAME
        && summary["source_commit"] == lock["source_commit"]
        && summary["source_attestation_mode"] == "supervised_snapshot"
        && summary["object"] == STORE_NAME
        && summary["memory_cgroup_limit_bytes"] == 4_294_967_296u64
        && summary["memory_cgroup_swap_limit_bytes"] == 0
        && summary["memory_cgroup_peak_bytes"]
            .as_u64()
            .is_some_and(|peak| peak > 0 && peak <= 4_294_967_296)
        && summary["total_index_calculus_runtime_ms"].is_null()
        && summary["selected_best_total_runtime"].is_null()
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("prepare")=>prepare(&args[1..]),
        Some("run")=>{
            let result=run(&args[1..]);
            if let Err(error)=&result {
                if let Some(out)=args.get(3).map(Path::new) {
                    if out.is_dir() && !out.join("outer.json").exists() {
                        let _=write_new_json(&out.join("failure.json"),&json!({
                            "schema":"n83.k600-m6-native-failure/v1","status":"PRODUCER_FAILURE",
                            "error":error.to_string(),"selected_best_total_runtime":Value::Null
                        }));
                    }
                }
            }
            let result=result?;
            println!("{result}");
            if result["status"]=="PREFLIGHT_ONLY"{Ok(())}else{Err("native K600 preflight failed; inspect outer.json".into())}
        },
        _=>Err("usage: prepare REPO BASE_BUNDLE FIXTURES WORKER_ELF IMAGE_ID WORKER_SECONDS NEW_LOCK_JSON | run COMMITTED_LOCK_JSON NEW_OUTPUT_DIR OUTER_SECONDS".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn committed_lock_rejects_changed_source_before_launch() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("n83-k600-lock-{}-{nonce}", std::process::id()));
        let repo = root.join("repo");
        let base = root.join("base");
        let fixtures = root.join("fixtures");
        let binary = root.join("worker");
        let index = root.join("index.json");
        let out = root.join("out");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(&base).unwrap();
        fs::create_dir_all(&fixtures).unwrap();
        fs::write(&binary, b"worker").unwrap();
        fs::write(&index, b"{}").unwrap();
        command_output("git", &["init", "-q"], &repo).unwrap();
        command_output("git", &["config", "user.name", "N83 Test"], &repo).unwrap();
        command_output(
            "git",
            &["config", "user.email", "n83-test@example.invalid"],
            &repo,
        )
        .unwrap();
        for name in SOURCE_FILES {
            let path = repo.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, name.as_bytes()).unwrap();
        }
        command_output("git", &["add", "."], &repo).unwrap();
        command_output("git", &["commit", "-qm", "source"], &repo).unwrap();
        let source_commit = command_output("git", &["rev-parse", "HEAD"], &repo).unwrap();
        let lock_path = repo.join("lock.json");
        write_new_json(
            &lock_path,
            &json!({
                "schema":"n83.k600-m6-native-preflight-lock/v1",
                "status":"PREPARED_ONLY", "source_commit":source_commit,
                "source_root":repo, "base_dir":base, "fixture_dir":fixtures,
                "worker_elf":binary, "store_index":index, "worker_seconds":20,
                "source_sha256":exact_hashes(&repo,SOURCE_FILES).unwrap(),
            }),
        )
        .unwrap();
        command_output("git", &["add", "lock.json"], &repo).unwrap();
        command_output("git", &["commit", "-qm", "lock"], &repo).unwrap();
        fs::write(repo.join(SOURCE_FILES[0]), b"changed and committed").unwrap();
        command_output("git", &["add", "."], &repo).unwrap();
        command_output("git", &["commit", "-qm", "source changed"], &repo).unwrap();
        let args = vec![
            "run".to_owned(),
            lock_path.to_string_lossy().into_owned(),
            out.to_string_lossy().into_owned(),
            "21".to_owned(),
        ];
        let error = run(&args).unwrap_err().to_string();
        assert!(error.contains("frozen input hash changed"), "{error}");
        assert!(!out.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bounded_command_terminates_a_hung_child() {
        assert_eq!(
            command_output_bounded(
                "/bin/echo",
                &["ready"],
                Path::new("/"),
                Duration::from_secs(1),
            )
            .unwrap(),
            "ready"
        );
        let start = Instant::now();
        let result = command_output_bounded(
            "/bin/sleep",
            &["2"],
            Path::new("/"),
            Duration::from_millis(40),
        );
        assert!(result.is_err());
        assert!(start.elapsed() < Duration::from_secs(1));
    }
    #[test]
    fn preflight_receipt_requires_peak_and_zero_solver_work() {
        let limits = json!({"max_variables":125000,"max_domain_clauses":350000,"max_models":1,"conflict_budget":10000});
        let lock = json!({"worker_seconds":20,"source_commit":"a".repeat(40),"point_set_blake3":"point-set","base_sha256":{"manifest.json":"manifest"}});
        let config = json!({"schema":"n83.primary-store-chain-config/v1","study":"koblitz_n83_factor_base_sweep_20261008","input_kind":"artifact_store_v3","curve_a":0,"fixture":0,"base_dir":"/base","fixture_dir":"/fixtures","orbit_columns":600,"summands":6,"strategy":"orbit-chain-s3","max_trials":0,"budget_seconds":20,"allow_direct_relation":false,"chain_s3_limits":limits,"policy":"public_x_hash","seed":2026100801u64,"source_commit":"a".repeat(40),"source_attestation_mode":"supervised_snapshot","memory_cgroup_limit_bytes":4294967296u64,"memory_cgroup_swap_limit_bytes":0});
        let mut summary = json!({"schema":"n83.primary-store-chain/v1","status":"PREFLIGHT_ONLY","study":"koblitz_n83_factor_base_sweep_20261008","input_kind":"artifact_store_v3","curve_a":0,"fixture":0,"strategy":"orbit-chain-s3","orbit_columns":600,"summands":6,"max_trials":0,"budget_seconds":20,"policy":"public_x_hash","seed":2026100801u64,"fixture_dir":"/fixtures","point_set_blake3":"point-set","store_manifest_sha256":"manifest","chain_s3_limits":limits,"solver_stage_executed":false,"report":{"trials":0,"relations":0,"sat_calls":0,"matrix_rows":0},"attempt_records":[],"relation_matrix":[],"verified_log":null,"store_name":STORE_NAME,"object":STORE_NAME,"source_commit":"a".repeat(40),"source_attestation_mode":"supervised_snapshot","memory_cgroup_limit_bytes":4294967296u64,"memory_cgroup_swap_limit_bytes":0,"memory_cgroup_peak_bytes":1024,"total_index_calculus_runtime_ms":null,"selected_best_total_runtime":null});
        assert!(valid_preflight(Some(&summary), Some(&config), &lock));
        summary["memory_cgroup_peak_bytes"] = Value::Null;
        assert!(!valid_preflight(Some(&summary), Some(&config), &lock));
        summary["memory_cgroup_peak_bytes"] = json!(1024);
        summary["report"]["sat_calls"] = json!(1);
        assert!(!valid_preflight(Some(&summary), Some(&config), &lock));
    }
    #[test]
    fn committed_k600_s3_adoption_bundle_matches_preflight_contract() {
        let root=Path::new(env!("CARGO_MANIFEST_DIR")).join("research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/attempt-1");
        let manifest = read_json(&root.join("manifest.json")).unwrap();
        let replay = read_json(&root.join("replay.json")).unwrap();
        let storage = read_json(&root.join("storage.json")).unwrap();
        let import = read_json(&root.join("import.json")).unwrap();
        let digest = sha256_file(&root.join("manifest.json")).unwrap();
        validate_bundle(&manifest, &replay, &storage, &import, &digest).unwrap();
    }
    #[test]
    fn bundle_rejects_changed_identity_and_missing_receipts() {
        let mut m = json!({"schema":"n83.factor-base-panel/v3-artifact-store","status":"completed_factor_base_object","curve_id":CURVE,"remote":REMOTE,"custody_mode":"read_only_adoption","source_manifest_sha256":SOURCE_MANIFEST_SHA,"artifact_pointer_sha256":POINTER_SHA,"base":{"a":0,"columns":600,"points":99600,"policy":"public_x_hash","seed":2026100801u64,"closure":"signed_frobenius","store_name":STORE_NAME,"compressed_sha256":OBJECT_SHA,"bytes":3025798u64,"point_set_blake3":"test-point-set"}});
        let r = json!({"schema":"n83.factor-base-replay/v3-artifact-store","status":"PASS","manifest_sha256":"test-manifest","point_set_blake3":"test-point-set","points_checked":99600,"representatives_checked":600});
        let s = json!({"schema":"n83.factor-base-storage/v3-artifact-store","status":"PASS","manifest_sha256":"test-manifest","custody_mode":"read_only_adoption","remote":REMOTE,"sha256":OBJECT_SHA,"bytes":3025798,"fresh_download_sha256":OBJECT_SHA,"fresh_download_bytes":3025798,"uploaded":false});
        let i = json!({"schema":"n83.factor-base-store-import/v3","status":"PASS","point_set_blake3":"test-point-set","points_checked":99600,"orbit_columns_checked":600,"store_name":STORE_NAME,"selected_best_total_runtime":null});
        assert!(validate_bundle(&m, &r, &s, &i, "test-manifest").is_ok());
        m["base"]["columns"] = json!(599);
        assert!(validate_bundle(&m, &r, &s, &i, "test-manifest").is_err());
        m["base"]["columns"] = json!(600);
        m["remote"] = json!("file:///tmp");
        assert!(validate_bundle(&m, &r, &s, &i, "test-manifest").is_err());
    }
    #[test]
    fn docker_contract_has_hard_isolation_and_zero_trials() {
        let lock = json!({"base_dir":"/base","fixture_dir":"/fixtures","worker_elf":"/worker","image_id":format!("sha256:{}","a".repeat(64)),"worker_seconds":20});
        let args = docker_args(&lock, Path::new("/out"), Path::new("/source"));
        for expected in [
            "none",
            "4096m",
            "1",
            "64",
            "ALL",
            "no-new-privileges",
            "--read-only",
            "--cpuset-cpus",
            "primary-store-v3-orbit-chain-cold",
            "125000",
            "350000",
            "10000",
        ] {
            assert!(args.iter().any(|arg| arg == expected), "missing {expected}");
        }
        assert!(args.windows(3).any(|window| window == ["600", "6", "0"]));
        assert!(args
            .iter()
            .any(|arg| arg == "type=bind,src=/out/worker-output,dst=/out"));
        assert!(!args.iter().any(|arg| arg == "type=bind,src=/out,dst=/out"));
    }
}
