// Native, bounded CLI software fixture. Never a research-corpus producer.
use clap::Parser;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const STRATEGY: &str = "prime-affine-s3-chain-f4-visitor/v1";
const INPUT_SCHEMA: &str = "ml-dataset.prime-f4-chain-input/v1";
const COMMAND_SECONDS: u64 = 45;
const TOTAL_SECONDS: u64 = 240;
const RUNNER_SOURCE: &str = include_str!("run.rs");
static CANCELLED: AtomicBool = AtomicBool::new(false);

#[derive(Parser)]
struct Args {
    #[arg(long)]
    binary: PathBuf,
    #[arg(long)]
    binary_sha256: String,
    #[arg(long)]
    source_commit: String,
    #[arg(long)]
    fixture_commit: String,
    #[arg(long)]
    fixture: PathBuf,
    #[arg(long)]
    scratch: PathBuf,
}

fn require(ok: bool, message: &str) -> Result<()> {
    if !ok {
        return Err(message.into());
    }
    Ok(())
}

fn hash(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_reader(File::open(path)?)?)
}
fn now() -> Result<String> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string())
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

// Explicit sorting also works if a future dependency enables preserve_order.
fn canonical(value: &Value) -> Result<String> {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            let parts = keys
                .into_iter()
                .map(|k| {
                    Ok(format!(
                        "{}:{}",
                        serde_json::to_string(k)?,
                        canonical(&map[k])?
                    ))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(format!("{{{}}}", parts.join(",")))
        }
        Value::Array(xs) => Ok(format!(
            "[{}]",
            xs.iter()
                .map(canonical)
                .collect::<Result<Vec<_>>>()?
                .join(",")
        )),
        Value::Number(n) => {
            let text = n.to_string();
            require(
                !text.contains(['.', 'e', 'E']),
                "fixture protocol must use integer JSON numbers",
            )?;
            Ok(text)
        }
        _ => Ok(serde_json::to_string(value)?),
    }
}

fn checked_file(root: &Path, name: &str) -> Result<PathBuf> {
    let relative = Path::new(name);
    require(!name.is_empty(), "empty artifact path")?;
    let mut path = root.to_path_buf();
    for part in relative.components() {
        let Component::Normal(part) = part else {
            return Err("unsafe artifact path".into());
        };
        path.push(part);
        require(
            !fs::symlink_metadata(&path)?.file_type().is_symlink(),
            "symlink in retained artifacts",
        )?;
    }
    require(
        fs::symlink_metadata(&path)?.is_file(),
        "artifact is not an ordinary file",
    )?;
    Ok(path)
}

fn validate_fixture(directory: &Path) -> Result<()> {
    let manifest = read_json(&checked_file(directory, "fixture-manifest.json")?)?;
    require(
        manifest["schema"] == "ml-dataset.cli-correctness-fixture/v2",
        "unknown native fixture schema",
    )?;
    let files = manifest["files"]
        .as_object()
        .ok_or("missing fixture files")?;
    require(
        files.len() == 3
            && ["README.md", "protocol.json", "run.rs"]
                .iter()
                .all(|s| files.contains_key(*s)),
        "unexpected fixture file set",
    )?;
    for (name, record) in files {
        let path = checked_file(directory, name)?;
        require(
            record["sha256"] == hash(&path)? && record["bytes"] == fs::metadata(&path)?.len(),
            "fixture digest or size differs",
        )?;
    }
    let mut protocol = read_json(&directory.join("protocol.json"))?;
    require(
        protocol["schema"] == "experiment.protocol/v1",
        "unknown protocol schema",
    )?;
    let freeze = protocol
        .as_object_mut()
        .ok_or("protocol is not an object")?
        .remove("freeze")
        .ok_or("missing freeze")?;
    require(
        freeze["status"] == "frozen" && freeze["filed_before_affected_data"] == true,
        "protocol not frozen before execution",
    )?;
    let body_hash = format!("{:x}", Sha256::digest(canonical(&protocol)?.as_bytes()));
    require(
        freeze["protocol_sha256"] == body_hash,
        "protocol body differs from freeze digest",
    )?;
    require(
        protocol["analysis_plan"]["script"] == "run.rs"
            && protocol["analysis_plan"]["script_sha256"] == hash(&directory.join("run.rs"))?,
        "protocol does not bind native runner",
    )?;
    require(
        hash(&directory.join("run.rs"))?
            == format!("{:x}", Sha256::digest(RUNNER_SOURCE.as_bytes())),
        "fixture source differs from the compiled native runner",
    )
}

fn entries(directory: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(directory)?
        .map(|x| x.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.sort();
    Ok(paths)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    require(
        fs::symlink_metadata(source)?.is_dir(),
        "source is not an ordinary directory",
    )?;
    fs::create_dir(destination)?;
    for path in entries(source)? {
        let kind = fs::symlink_metadata(&path)?.file_type();
        let target = destination.join(path.file_name().ok_or("missing filename")?);
        if kind.is_dir() {
            copy_tree(&path, &target)?;
        } else {
            require(kind.is_file(), "cannot copy symlink or special file")?;
            let mut out = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(target)?;
            std::io::copy(&mut File::open(path)?, &mut out)?;
            out.sync_all()?;
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
extern "C" fn cancelled(_: libc::c_int) {
    CANCELLED.store(true, Ordering::Relaxed);
}

#[cfg(target_os = "linux")]
fn kill_group(pid: u32, signal: i32) {
    // The process is a direct child created below with its own process group.
    unsafe {
        libc::kill(-(pid as i32), signal);
    }
}

#[cfg(target_os = "linux")]
struct ChildGroup {
    child: std::process::Child,
    reaped: bool,
}

#[cfg(target_os = "linux")]
impl Drop for ChildGroup {
    fn drop(&mut self) {
        if !self.reaped {
            kill_group(self.child.id(), libc::SIGKILL);
            let _ = self.child.wait();
        }
    }
}

#[cfg(target_os = "linux")]
fn supervised(mut command: Command, seconds: Duration) -> Result<(i32, bool, bool)> {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
    let mut group = ChildGroup {
        child: command.spawn()?,
        reaped: false,
    };
    let start = Instant::now();
    loop {
        if let Some(status) = group.child.try_wait()? {
            group.reaped = true;
            return Ok((status.code().unwrap_or(-1), false, false));
        }
        let interrupted = CANCELLED.load(Ordering::Relaxed);
        if interrupted || start.elapsed() >= seconds {
            kill_group(group.child.id(), libc::SIGKILL);
            let status = group.child.wait()?;
            group.reaped = true;
            return Ok((status.code().unwrap_or(-1), !interrupted, interrupted));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

struct Runner<'a> {
    args: &'a Args,
    root: PathBuf,
    deadline: Instant,
    commands: Vec<Value>,
    cells: Vec<Value>,
    tamper: Value,
}

impl Runner<'_> {
    #[cfg(target_os = "linux")]
    fn invoke(&mut self, label: &str, arguments: Vec<String>, success: bool) -> Result<PathBuf> {
        require(
            hash(&self.args.binary)? == self.args.binary_sha256,
            "binary changed before command",
        )?;
        require(!CANCELLED.load(Ordering::Relaxed), "fixture cancelled")?;
        let timeout = self
            .deadline
            .checked_duration_since(Instant::now())
            .ok_or("overall fixture deadline exhausted")?
            .min(Duration::from_secs(COMMAND_SECONDS));
        require(
            !arguments
                .iter()
                .any(|x| x == "--publish" || x == "--remote" || x.starts_with("--budget-")),
            "network/publication option forbidden",
        )?;
        let prefix = self.root.join("commands").join(label);
        let mut argv = vec![path_text(&self.args.binary)?, "dataset".into()];
        argv.extend(arguments.clone());
        let mut record = json!({"label":label,"argv":argv,"started_unix_seconds":now()?,"timeout_seconds":timeout.as_secs_f64(),"clock_valid_for_comparison":false});
        write_new(&prefix.with_extension("started.json"), &record)?;
        let stdout = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(prefix.with_extension("stdout"))?;
        let stderr = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(prefix.with_extension("stderr"))?;
        let mut command = Command::new(&self.args.binary);
        command
            .arg("dataset")
            .args(arguments)
            .current_dir(&self.root)
            .env_clear();
        for key in ["PATH", "HOME", "USER", "LOGNAME"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        command
            .env("RAYON_NUM_THREADS", "1")
            .env("LANG", "C.UTF-8")
            .env("LC_ALL", "C.UTF-8")
            .env("TMPDIR", self.root.join("tmp"));
        command
            .stdin(Stdio::null())
            .stdout(stdout.try_clone()?)
            .stderr(stderr.try_clone()?);
        let result = supervised(command, timeout);
        stdout.sync_all()?;
        stderr.sync_all()?;
        record["finished_unix_seconds"] = json!(now()?);
        record["binary_sha256_after"] = json!(hash(&self.args.binary)?);
        for stream in ["stdout", "stderr"] {
            let path = prefix.with_extension(stream);
            record[stream] = json!({"path":path.strip_prefix(&self.root)?,"sha256":hash(&path)?,"bytes":fs::metadata(path)?.len()});
        }
        let mut passed = false;
        match result {
            Ok((code, timeout, interrupted)) => {
                record["exit_status"] = json!(code);
                record["timed_out"] = json!(timeout);
                record["cancelled"] = json!(interrupted);
                passed = !timeout && !interrupted && (code == 0) == success;
            }
            Err(error) => {
                record["spawn_or_wait_error"] = json!(error.to_string());
            }
        }
        write_new(&prefix.with_extension("receipt.json"), &record)?;
        self.commands.push(record);
        require(
            hash(&self.args.binary)? == self.args.binary_sha256,
            "binary changed during command",
        )?;
        require(
            passed,
            &format!("unexpected command exit, timeout or cancellation: {label}"),
        )?;
        Ok(prefix)
    }

    #[cfg(target_os = "linux")]
    fn json_command(&mut self, label: &str, arguments: Vec<String>) -> Result<Value> {
        let prefix = self.invoke(label, arguments, true)?;
        read_json(&prefix.with_extension("stdout"))
    }

    fn status(value: &Value, replay: bool) -> Result<()> {
        let expected = json!({"planned_cells":64,"assigned_shard_cells":1,"completed_shard_receipts":1,"artifact_hashes_verified":true,"known_answer_replay_performed":replay,"shard_attempts_complete":true,"campaign_attempts_complete":false,"full_scope_complete":false,"published_completion_receipts":0,"published_cells":0,"discoverable_catalog_cells":0,"independently_solved_ecdlp_cells":0,"incomplete_attempts":[]});
        bindings(value, &expected)
    }

    #[cfg(target_os = "linux")]
    fn execute(&mut self) -> Result<()> {
        let protocol = self.root.join("fixture/protocol.json");
        let protocol_sha = hash(&protocol)?;
        let plan = self.root.join("plan.json");
        let planned = self.json_command(
            "01-plan",
            strings(&[
                "plan",
                "--out",
                &path_text(&plan)?,
                "--n",
                "7",
                "--m",
                "3",
                "--families",
                "prime",
                "--solvers",
                "f4",
                "--curve-seeds",
                "8",
                "--generators",
                "2",
                "--keys",
                "2",
                "--seed",
                "42",
                "--factor-base-size",
                "2",
                "--degree-bound",
                "4",
                "--work-limit",
                "512",
                "--protocol-sha256",
                &protocol_sha,
            ]),
        )?;
        require(
            planned["planned_cells"] == 64
                && planned["generated_cells"] == 0
                && planned["sha256"] == hash(&plan)?,
            "plan counts or digest differ",
        )?;
        let preflight = self.json_command(
            "02-preflight",
            strings(&[
                "preflight",
                "--plan",
                &path_text(&plan)?,
                "--protocol",
                &path_text(&protocol)?,
            ]),
        )?;
        bindings(
            &preflight,
            &json!({"source_commit":self.args.source_commit,"source_dirty":false,"binary_sha256":self.args.binary_sha256,"plan_sha256":planned["plan_sha256"],"protocol_file_sha256":protocol_sha,"lab_receipt_uri":null,"generated_cells":0,"cloud_permissions_checked":false}),
        )?;
        for (ordinal, index) in [62u64, 63].into_iter().enumerate() {
            let out = self.root.join(format!("shard-{index}"));
            self.invoke(
                &format!("{:02}-generate-{index}", 3 + ordinal * 3),
                strings(&[
                    "generate",
                    "--plan",
                    &path_text(&plan)?,
                    "--protocol",
                    &path_text(&protocol)?,
                    "--out",
                    &path_text(&out)?,
                    "--max-cases",
                    "1",
                    "--shard-index",
                    &index.to_string(),
                    "--shard-count",
                    "64",
                    "--seconds-per-case",
                    "20",
                    "--memory-mib",
                    "512",
                    "--trace-mib",
                    "16",
                ]),
                true,
            )?;
            let run = read_json(&out.join("run.json"))?;
            bindings(
                &run,
                &json!({"source_commit":self.args.source_commit,"binary_sha256":self.args.binary_sha256,"plan_sha256":planned["plan_sha256"],"shard_index":index,"shard_count":64,"seconds_per_case":20,"memory_mib":512,"trace_mib":16}),
            )?;
            let cell = out.join(format!("cells/{index:012}"));
            bindings(
                &read_json(&cell.join("config.json"))?,
                &json!({"family":"prime","n":7,"m":3,"solver":"f4","seed":42,"curve_seed":7,"generator_index":1,"key_index":1,"twist":ordinal==1}),
            )?;
            let manifest = read_json(&cell.join("manifest.json"))?;
            let chain = read_json(&cell.join("prime-f4-input.json"))?;
            require(
                chain["schema"] == INPUT_SCHEMA
                    && chain["strategy"] == STRATEGY
                    && chain["m"] == 3
                    && chain["n_vars"] == 4
                    && chain["domains"][3] == Value::Null,
                "missing or invalid exact chain encoding",
            )?;
            require(
                manifest["outcome"]["strategy"] == STRATEGY
                    && manifest["outcome"]["exhaustive_no_decomposition"] == false,
                "invalid strategy or PDP impossibility label",
            )?;
            let verified = self.json_command(
                &format!("{:02}-verify-{index}", 4 + ordinal * 3),
                strings(&["verify", "--out", &path_text(&out)?]),
            )?;
            Self::status(&verified, true)?;
            let status = self.json_command(
                &format!("{:02}-status-{index}", 5 + ordinal * 3),
                strings(&["status", "--out", &path_text(&out)?]),
            )?;
            Self::status(&status, false)?;
            self.cells.push(json!({"cell_index":index,"twist":ordinal==1,"manifest_sha256":hash(&cell.join("manifest.json"))?,"outcome":manifest["outcome"],"included_in_research_corpus":false}));
        }
        let damaged = self.root.join("tampered-copy");
        copy_tree(&self.root.join("shard-62"), &damaged)?;
        let cell = damaged.join("cells/000000000062");
        let input = cell.join("prime-f4-input.json");
        let original_hash = hash(&input)?;
        let mut chain = read_json(&input)?;
        require(
            chain["domains"][0]
                .as_array()
                .is_some_and(|a| !a.is_empty())
                && chain["domains"][3] == Value::Null,
            "invalid tamper precondition",
        )?;
        chain["domains"][3] = chain["domains"][0].clone();
        let coefficient: u64 = chain["polynomials"][0][0]["coefficient"]
            .as_str()
            .ok_or("missing coefficient")?
            .parse()?;
        let modulus: u64 = chain["modulus"]
            .as_str()
            .ok_or("missing modulus")?
            .parse()?;
        chain["polynomials"][0][0]["coefficient"] =
            json!(((coefficient + 1) % modulus).to_string());
        fs::write(&input, serde_json::to_vec_pretty(&chain)?)?;
        let manifest_path = cell.join("manifest.json");
        let mut manifest = read_json(&manifest_path)?;
        let mut matched = 0;
        for artifact in manifest["artifacts"]
            .as_array_mut()
            .ok_or("missing artifacts")?
        {
            let name = artifact["path"]
                .as_str()
                .ok_or("missing artifact path")?
                .to_owned();
            let path = checked_file(&cell, &name)?;
            artifact["sha256"] = json!(hash(&path)?);
            artifact["bytes"] = json!(fs::metadata(path)?.len());
            matched += usize::from(name == "prime-f4-input.json");
        }
        require(matched == 1, "input must have exactly one manifest entry")?;
        fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
        let status = self.json_command(
            "09-tampered-status",
            strings(&["status", "--out", &path_text(&damaged)?]),
        )?;
        Self::status(&status, false)?;
        let rejected = self.invoke(
            "10-tampered-verify",
            strings(&["verify", "--out", &path_text(&damaged)?]),
            false,
        )?;
        require(
            fs::read_to_string(rejected.with_extension("stderr"))?
                .contains("prime F4 chain input differs at"),
            "rejection did not come from semantic reconstruction",
        )?;
        self.tamper = json!({"source_shard":62,"artifact":"prime-f4-input.json","original_sha256":original_hash,"tampered_sha256":hash(&input)?,"manifest_sha256":hash(&manifest_path)?,"changes":["auxiliary domain restricted to leaf abscissae","one S3 coefficient changed"],"hash_only_status_passed":true,"semantic_verify_rejected":true,"included_in_research_corpus":false});
        Ok(())
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|x| (*x).to_owned()).collect()
}
fn path_text(path: &Path) -> Result<String> {
    Ok(path.to_str().ok_or("path is not UTF-8")?.to_owned())
}
fn bindings(actual: &Value, expected: &Value) -> Result<()> {
    for (key, value) in expected.as_object().ok_or("invalid expected bindings")? {
        require(
            actual.get(key) == Some(value),
            &format!("binding differs: {key}"),
        )?;
    }
    Ok(())
}

fn inventory(root: &Path, directory: &Path, rows: &mut Vec<Value>) -> Result<()> {
    for path in entries(directory)? {
        let kind = fs::symlink_metadata(&path)?.file_type();
        if kind.is_dir() {
            inventory(root, &path, rows)?;
        } else {
            require(kind.is_file(), "special file in output inventory")?;
            rows.push(json!({"path":path.strip_prefix(root)?,"sha256":hash(&path)?,"bytes":fs::metadata(path)?.len()}));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn run(mut args: Args) -> Result<bool> {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    for (value, width) in [
        (&args.binary_sha256, 64),
        (&args.source_commit, 40),
        (&args.fixture_commit, 40),
    ] {
        require(
            value.len() == width
                && value
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "expected exact lowercase hexadecimal identity",
        )?;
    }
    args.binary = args.binary.canonicalize()?;
    args.fixture = args.fixture.canonicalize()?;
    require(
        hash(&args.binary)? == args.binary_sha256,
        "binary does not match expected digest",
    )?;
    validate_fixture(&args.fixture)?;
    let parent = args
        .scratch
        .parent()
        .ok_or("scratch parent required")?
        .canonicalize()?;
    let root = parent.join(args.scratch.file_name().ok_or("scratch name required")?);
    fs::DirBuilder::new().mode(0o700).create(&root)?;
    fs::create_dir(root.join("fixture"))?;
    for name in [
        "README.md",
        "protocol.json",
        "run.rs",
        "fixture-manifest.json",
    ] {
        let source = checked_file(&args.fixture, name)?;
        let mut target = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join("fixture").join(name))?;
        std::io::copy(&mut File::open(source)?, &mut target)?;
        target.sync_all()?;
    }
    validate_fixture(&root.join("fixture"))?;
    for name in [
        "README.md",
        "protocol.json",
        "run.rs",
        "fixture-manifest.json",
    ] {
        fs::set_permissions(
            root.join("fixture").join(name),
            fs::Permissions::from_mode(0o444),
        )?;
    }
    fs::create_dir(root.join("commands"))?;
    fs::create_dir(root.join("tmp"))?;
    let inputs = json!({"schema":"ml-dataset.cli-correctness-inputs/v2","classification":"software_correctness_fixture","runner_language":"Rust","source_commit":args.source_commit,"binary_path":args.binary,"binary_sha256":args.binary_sha256,"fixture_commit":args.fixture_commit,"fixture_commit_is_supplied_not_git_verified":true,"fixture_manifest_sha256":hash(&root.join("fixture/fixture-manifest.json"))?,"protocol_file_sha256":hash(&root.join("fixture/protocol.json"))?,"started_unix_seconds":now()?,"limits":{"seconds_per_case":20,"memory_mib":512,"trace_mib":16},"overall_deadline_seconds":TOTAL_SECONDS,"per_command_timeout_seconds":COMMAND_SECONDS,"corpus_rows_added":0,"publication_requested":false,"cloud_permissions_checked":false});
    write_new(&root.join("execution-inputs.json"), &inputs)?;
    // Handlers only set an atomic flag: cancellation cannot unwind past spawn.
    unsafe {
        libc::signal(libc::SIGINT, cancelled as *const () as libc::sighandler_t);
        libc::signal(libc::SIGTERM, cancelled as *const () as libc::sighandler_t);
    }
    let mut runner = Runner {
        args: &args,
        root: root.clone(),
        deadline: Instant::now() + Duration::from_secs(TOTAL_SECONDS),
        commands: vec![],
        cells: vec![],
        tamper: Value::Null,
    };
    let mut failure = runner.execute().err().map(|e| e.to_string());
    let final_hash = hash(&args.binary)?;
    if final_hash != args.binary_sha256 && failure.is_none() {
        failure = Some("binary changed before final receipt".into());
    }
    let mut rows = vec![];
    inventory(&root, &root, &mut rows)?;
    let mut receipt = inputs;
    let map = receipt.as_object_mut().ok_or("invalid inputs")?;
    map.extend(json!({"schema":"ml-dataset.cli-correctness-receipt/v2","finished_unix_seconds":now()?,"verdict":if failure.is_none(){"passed_software_checks"}else{"failed_software_checks"},"failure":failure,"commands":runner.commands,"verified_local_cells":runner.cells,"semantic_tamper":runner.tamper,"binary_sha256_after":final_hash,"inventory":rows,"clock_valid_for_comparison":false,"verified_cloud_dataset_cells":0,"startup_gate_exercised":false,"runtime_source_must_match_preflight":true}).as_object().ok_or("invalid receipt")?.clone());
    write_new(&root.join("receipt.json"), &receipt)?;
    println!(
        "{}",
        json!({"receipt":root.join("receipt.json"),"passed":failure.is_none(),"failure":failure})
    );
    Ok(failure.is_none())
}

fn main() {
    #[cfg(target_os = "linux")]
    match run(Args::parse()) {
        Ok(true) => (),
        Ok(false) => std::process::exit(1),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!("fixture requires the native Linux supervisor contract");
        std::process::exit(1);
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU64;
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "native-cli-fixture-control-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn watchdog_reaps_a_child_that_ignores_termination() {
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "trap '' TERM; exec sleep 30"]);
        let start = Instant::now();
        let (code, timed_out, cancelled) = supervised(command, Duration::from_millis(50)).unwrap();
        assert_ne!(code, 0);
        assert!(timed_out);
        assert!(!cancelled);
        assert!(start.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn copied_artifacts_cannot_escape_through_a_symlink() {
        use std::os::unix::fs::symlink;
        let scratch = Scratch::new();
        let source = scratch.0.join("source");
        fs::create_dir(&source).unwrap();
        symlink("/etc/passwd", source.join("escape")).unwrap();
        assert!(checked_file(&source, "escape").is_err());
        assert!(checked_file(&source, "../escape").is_err());
        assert!(copy_tree(&source, &scratch.0.join("copy")).is_err());
        assert!(!scratch.0.join("copy/escape").exists());
    }

    #[test]
    fn an_executing_fixture_refuses_source_changed_after_freeze() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ml_dataset_20261028/startup-integration-cli-fixture-native");
        let scratch = Scratch::new();
        let copy = scratch.0.join("fixture");
        copy_tree(&source, &copy).unwrap();
        validate_fixture(&copy).unwrap();
        fs::write(copy.join("run.rs"), "// changed after freeze\n").unwrap();
        assert!(validate_fixture(&copy).is_err());
    }

    #[test]
    fn outputs_cannot_overwrite_an_existing_attempt() {
        let scratch = Scratch::new();
        let path = scratch.0.join("receipt.json");
        write_new(&path, &json!({"original":true})).unwrap();
        assert!(write_new(&path, &json!({"original":false})).is_err());
        assert_eq!(read_json(&path).unwrap(), json!({"original":true}));
    }
}
