//! Source-bound Linux ARM64 build receipt producer for the frozen N83 K600 m6 cell.
//! This runs only on Linux aarch64 from a clean committed checkout. The outer
//! budget controller's `charge build` wraps this entire invocation.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const STEPS: [&str; 3] = ["rust-binaries", "solver-binary", "runtime-image"];
const OUTPUTS: [&str; 6] = [
    "worker",
    "exporter",
    "solver",
    "arm-supervisor",
    "preflight-supervisor",
    "budget-controller",
];

fn sha256(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut bytes = [0u8; 65_536];
    loop {
        let count = file.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        digest.update(&bytes[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = File::options().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn clean_head(repo: &Path) -> Result<String> {
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .output()?;
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()?;
    if !status.status.success() || !status.stdout.is_empty() || !head.status.success() {
        return Err("source checkout is not clean".into());
    }
    let commit = String::from_utf8(head.stdout)?.trim().to_owned();
    if commit.len() != 40 || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("source commit invalid".into());
    }
    Ok(commit)
}

fn committed_plan(repo: &Path, path: &Path) -> Result<(Value, String)> {
    let path = fs::canonicalize(path)?;
    let relative = path
        .strip_prefix(repo)?
        .to_str()
        .ok_or("plan path encoding invalid")?;
    let bytes = fs::read(&path)?;
    let tracked = Command::new("git")
        .args(["show", &format!("HEAD:{relative}")])
        .current_dir(repo)
        .output()?;
    if !tracked.status.success() || tracked.stdout != bytes {
        return Err("build plan is not committed unchanged at HEAD".into());
    }
    Ok((serde_json::from_slice(&bytes)?, sha256(&path)?))
}

fn linux_arm64_elf(path: &Path) -> Result<bool> {
    let mut header = [0u8; 64];
    File::open(path)?.read_exact(&mut header)?;
    Ok(&header[..4] == b"\x7fELF"
        && header[4] == 2
        && header[5] == 1
        && header[6] == 1
        && matches!(u16::from_le_bytes([header[16], header[17]]), 2 | 3)
        && u16::from_le_bytes([header[18], header[19]]) == 183)
}

fn executable_file(path: &Path) -> Result<bool> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        Ok(fs::metadata(path)?.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        Ok(path.is_file())
    }
}

fn expand(value: &str, output: &Path, commit: &str) -> String {
    value
        .replace("${OUT}", &output.to_string_lossy())
        .replace("${COMMIT}", commit)
}

fn pinned_inputs(spec: &Value) -> Result<Vec<(PathBuf, String)>> {
    let mut inputs = Vec::new();
    for entry in spec["inputs"].as_array().ok_or("build inputs missing")? {
        let path = PathBuf::from(entry["path"].as_str().ok_or("input path missing")?);
        let digest = entry["sha256"].as_str().ok_or("input digest missing")?;
        if !path.is_absolute()
            || digest.len() != 64
            || !digest.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err("build input identity invalid".into());
        }
        let path = fs::canonicalize(path)?;
        if !path.is_file() || sha256(&path)? != digest {
            return Err("build input digest differs".into());
        }
        inputs.push((path, digest.to_owned()));
    }
    Ok(inputs)
}

fn inputs_unchanged(inputs: &[(PathBuf, String)]) -> bool {
    inputs
        .iter()
        .all(|(path, want)| sha256(path).ok().as_deref() == Some(want))
}

fn stop_child_group(child: &mut std::process::Child) {
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    #[cfg(not(unix))]
    {
        let _ = child.kill();
    }
}

fn command_receipt(
    name: &str,
    spec: &Value,
    repo: &Path,
    output: &Path,
    commit: &str,
) -> Result<Value> {
    let program = PathBuf::from(spec["program"].as_str().ok_or("build program missing")?);
    if !program.is_absolute() {
        return Err("build program must be absolute".into());
    }
    let program = fs::canonicalize(program)?;
    let want_sha = spec["program_sha256"]
        .as_str()
        .ok_or("build program digest missing")?;
    if sha256(&program)? != want_sha {
        return Err("build program digest differs".into());
    }
    let inputs = pinned_inputs(spec)?;
    let env = spec["env"].as_object().ok_or("build env missing")?;
    let mut environment = serde_json::Map::new();
    for (key, value) in env {
        if key.is_empty() || key.contains('=') || key.contains('\0') {
            return Err("build env key invalid".into());
        }
        let value = value.as_str().ok_or("build env value is not a string")?;
        environment.insert(key.clone(), json!(expand(value, output, commit)));
    }
    let args: Vec<String> = spec["args"]
        .as_array()
        .ok_or("build args missing")?
        .iter()
        .map(|a| {
            a.as_str()
                .ok_or_else(|| -> Box<dyn Error> { "non-string build arg".into() })
                .map(|s| expand(s, output, commit))
        })
        .collect::<Result<_>>()?;
    let limit = spec["timeout_seconds"]
        .as_u64()
        .filter(|n| (1..=3_600).contains(n))
        .ok_or("build step timeout invalid")?;
    let stdout = output.join(format!("{name}.stdout"));
    let stderr = output.join(format!("{name}.stderr"));
    let started = Instant::now();
    let mut command = Command::new(&program);
    command.args(&args).current_dir(repo).env_clear();
    #[cfg(unix)]
    command.process_group(0);
    for (key, value) in &environment {
        command.env(key, value.as_str().ok_or("build env value invalid")?);
    }
    let mut child = command
        .stdout(Stdio::from(
            File::options().write(true).create_new(true).open(&stdout)?,
        ))
        .stderr(Stdio::from(
            File::options().write(true).create_new(true).open(&stderr)?,
        ))
        .spawn()?;
    let (exit, capped) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status.code(), false),
            Ok(None) if started.elapsed() < Duration::from_secs(limit) => {
                thread::sleep(Duration::from_millis(100))
            }
            Ok(None) => {
                stop_child_group(&mut child);
                child.wait()?;
                break (None, true);
            }
            Err(error) => {
                stop_child_group(&mut child);
                let _ = child.wait();
                return Err(error.into());
            }
        }
    };
    let source_ok = clean_head(repo).ok().as_deref() == Some(commit);
    let program_ok = sha256(&program)? == want_sha;
    let input_ok = inputs_unchanged(&inputs);
    let receipt = json!({"schema":"n83.k600-m6-linux-build-step/v1",
        "name":name,"status":if exit==Some(0)&&!capped&&source_ok&&program_ok&&input_ok {"PASS"} else {"STOPPED"},
        "program":program,"program_sha256":want_sha,"args":args,"timeout_seconds":limit,
        "inputs":inputs.iter().map(|(path,sha)|json!({"path":path,"sha256":sha})).collect::<Vec<_>>(),
        "env":environment,
        "exit_code":exit,"wall_capped":capped,"source_unchanged":source_ok,
        "program_unchanged":program_ok,"inputs_unchanged":input_ok,
        "wall_to_receipt_start_ms":started.elapsed().as_millis(),
        "stdout_sha256":sha256(&stdout)?,"stderr_sha256":sha256(&stderr)?,
        "cleanup_verified":!capped,
    });
    write_new(&output.join(format!("{name}.json")), &receipt)?;
    Ok(receipt)
}

fn docker_inspect(image_ref: &str, commit: &str) -> Result<String> {
    let mut child = Command::new("docker").args(["image","inspect",image_ref,"--format",
        "{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels \"org.opencontainers.image.revision\"}}"])
        .stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let started = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if started.elapsed() >= Duration::from_secs(15) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("Docker inspect timed out".into());
        }
        thread::sleep(Duration::from_millis(50));
    }
    let result = child.wait_with_output()?;
    if !result.status.success() {
        return Err("Docker image inspect failed".into());
    }
    let value = String::from_utf8(result.stdout)?.trim().to_owned();
    let mut words = value.split_whitespace();
    let id = words.next().ok_or("image ID missing")?;
    if words.next() != Some("linux/arm64")
        || words.next() != Some(commit)
        || words.next().is_some()
        || !id.starts_with("sha256:")
        || id.len() != 71
        || !id[7..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("Docker image architecture, ID, or source label differs".into());
    }
    Ok(id.to_owned())
}

fn docker_image_absent(image_ref: &str) -> Result<bool> {
    let mut child = Command::new("docker")
        .args(["image", "inspect", image_ref, "--format", "{{.Id}}"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let started = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if started.elapsed() >= Duration::from_secs(15) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("Docker image absence check timed out".into());
        }
        thread::sleep(Duration::from_millis(50));
    }
    let result = child.wait_with_output()?;
    if result.status.success() {
        return Ok(false);
    }
    let stderr = String::from_utf8_lossy(&result.stderr);
    if stderr.contains("No such image") || stderr.contains("No such object") {
        return Ok(true);
    }
    Err("Docker image absence could not be established".into())
}

fn run(plan_path: &Path, output: &Path) -> Result<Value> {
    if std::env::consts::OS != "linux" || std::env::consts::ARCH != "aarch64" {
        return Err("build producer requires a Linux ARM64 host".into());
    }
    if !output.is_absolute() || output.exists() {
        return Err("new absolute output directory required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("build output must be outside source".into());
    }
    let commit = clean_head(&repo)?;
    let (plan, plan_sha) = committed_plan(&repo, plan_path)?;
    if plan["schema"] != "n83.k600-m6-linux-build-plan/v1"
        || plan["status"] != "FROZEN_BUILD_PLAN"
        || plan["target"] != "linux/arm64"
    {
        return Err("build plan identity differs".into());
    }
    let steps = plan["steps"].as_array().ok_or("build plan steps missing")?;
    if steps.len() != STEPS.len() {
        return Err("build step count differs".into());
    }
    let image_ref = expand(
        plan["image_ref"].as_str().ok_or("image ref missing")?,
        output,
        &commit,
    );
    if image_ref.is_empty() || !docker_image_absent(&image_ref)? {
        return Err("new image reference required".into());
    }
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    let started = Instant::now();
    let result: Result<Value> = (|| {
        let mut receipts = Vec::new();
        for (name, spec) in STEPS.into_iter().zip(steps) {
            if spec["name"] != name {
                return Err("build step order differs".into());
            }
            let receipt = command_receipt(name, spec, &repo, &output, &commit)?;
            let pass = receipt["status"] == "PASS";
            receipts.push(receipt);
            if !pass {
                return Err(format!("build step {name} stopped").into());
            }
        }
        let mut hashes = serde_json::Map::new();
        for name in OUTPUTS {
            let path = fs::canonicalize(output.join(name))?;
            if !path.starts_with(&output)
                || !path.is_file()
                || !executable_file(&path)?
                || !linux_arm64_elf(&path)?
            {
                return Err(format!("{name} is not an output-local Linux ARM64 ELF").into());
            }
            hashes.insert(name.to_owned(), json!(sha256(&path)?));
        }
        let image_id = docker_inspect(&image_ref, &commit)?;
        if clean_head(&repo)? != commit || sha256(&fs::canonicalize(plan_path)?)? != plan_sha {
            return Err("source or build plan changed".into());
        }
        let receipt = json!({"schema":"n83.k600-m6-linux-build/v1",
            "status":"PASS_SOURCE_MATCHED_BUILD","source_commit":commit,
            "plan_sha256":plan_sha,"image_ref":image_ref,"image_id":image_id,
            "worker_sha256":hashes["worker"],
            "exporter_replayer_sha256":hashes["exporter"],
            "solver_sha256":hashes["solver"],
            "supervisor_sha256":hashes["arm-supervisor"],
            "preflight_supervisor_sha256":hashes["preflight-supervisor"],
            "budget_controller_sha256":hashes["budget-controller"],
            "steps":receipts,"wall_to_receipt_start_ms":started.elapsed().as_millis(),
            "total_index_calculus_runtime_ms":Value::Null,
        });
        write_new(&output.join("build.json"), &receipt)?;
        Ok(receipt)
    })();
    if let Err(error) = &result {
        let _ = write_new(
            &output.join("failure.json"),
            &json!({
                "schema":"n83.k600-m6-linux-build-failure/v1","status":"PRODUCER_FAILURE",
                "source_commit":commit,"plan_sha256":plan_sha,"reason":error.to_string(),
                "wall_to_failure_receipt_start_ms":started.elapsed().as_millis(),
            }),
        );
    }
    result
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 || args[1] != "run" {
        return Err("usage: run COMMITTED_BUILD_PLAN_JSON NEW_OUTPUT_DIR".into());
    }
    println!("{}", run(Path::new(&args[2]), Path::new(&args[3]))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elf_architecture_and_plan_expansion_fail_closed() {
        let path = std::env::temp_dir().join(format!("n83-k600-elf-{}", std::process::id()));
        let mut bytes = [0u8; 64];
        bytes[..4].copy_from_slice(b"\x7fELF");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 1;
        bytes[16..18].copy_from_slice(&2u16.to_le_bytes());
        bytes[18..20].copy_from_slice(&183u16.to_le_bytes());
        fs::write(&path, bytes).unwrap();
        assert!(linux_arm64_elf(&path).unwrap());
        fs::write(&path, &bytes[..20]).unwrap();
        assert!(linux_arm64_elf(&path).is_err());
        bytes[18..20].copy_from_slice(&62u16.to_le_bytes());
        fs::write(&path, bytes).unwrap();
        assert!(!linux_arm64_elf(&path).unwrap());
        fs::remove_file(path).unwrap();
        assert_eq!(
            expand("${OUT}/${COMMIT}", Path::new("/out"), "abc"),
            "/out/abc"
        );
    }

    #[test]
    fn pinned_external_input_change_is_rejected() {
        let path = std::env::temp_dir().join(format!("n83-k600-input-{}", std::process::id()));
        fs::write(&path, b"frozen").unwrap();
        let digest = sha256(&path).unwrap();
        let spec = json!({"inputs":[{"path":path,"sha256":digest}]});
        let inputs = pinned_inputs(&spec).unwrap();
        assert!(inputs_unchanged(&inputs));
        fs::write(&path, b"altered").unwrap();
        assert!(!inputs_unchanged(&inputs));
        assert!(pinned_inputs(&spec).is_err());
        fs::remove_file(path).unwrap();
    }
}
