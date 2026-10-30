//! Bounded native timing wrapper for a single local dataset command.
//! Build: rustc --edition=2021 -O runner.rs -o runtime-preflight-runner
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn sha256(path: &Path) -> io::Result<String> {
    let output = Command::new("sha256sum").arg("--").arg(path).output()?;
    if !output.status.success() {
        return Err(io::Error::other("sha256sum failed"));
    }
    let text = String::from_utf8(output.stdout).map_err(io::Error::other)?;
    let hash = text.split_whitespace().next().unwrap_or("");
    if hash.len() != 64 || !hash.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(io::Error::other("invalid sha256sum output"));
    }
    Ok(hash.to_owned())
}

fn unix_ns() -> io::Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_nanos())
}

fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 8 || args[6] != "--" {
        return Err("usage: runner BINARY SOURCE_COMMIT ARM TIMEOUT_SECONDS OUTPUT -- generate|worker [dataset arguments...]".into());
    }
    let binary = PathBuf::from(&args[1]);
    if !binary.is_absolute() || !binary.is_file() {
        return Err("binary must be an absolute regular file".into());
    }
    let source = &args[2];
    if source.len() != 40 || !source.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("source commit must be 40 hexadecimal characters".into());
    }
    let arm = &args[3];
    if arm.is_empty()
        || arm.len() > 80
        || !arm
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c))
    {
        return Err("arm must contain only ASCII letters, digits, dash or underscore".into());
    }
    let seconds: u64 = args[4].parse()?;
    if !(1..=3600).contains(&seconds) {
        return Err("timeout must be 1..3600 seconds".into());
    }
    let out = PathBuf::from(&args[5]);
    if !out.is_absolute() || out.exists() {
        return Err("output must be a new absolute directory".into());
    }
    let kind = &args[7];
    if kind != "generate" && kind != "worker" {
        return Err("only dataset generate and dataset worker are allowed".into());
    }
    if args[8..].iter().any(|a| {
        a == "--publish"
            || a == "--remote"
            || a.starts_with("--remote=")
            || a.starts_with("--publish=")
            || a.starts_with("--budget-")
    }) {
        return Err("remote, publication and cloud budget options are forbidden".into());
    }
    let option_value = |flag: &str| -> Result<String, Box<dyn std::error::Error>> {
        let values: Vec<_> = args[8..]
            .iter()
            .enumerate()
            .filter_map(|(i, a)| {
                if a == flag {
                    args.get(i + 9).cloned()
                } else {
                    a.strip_prefix(&format!("{flag}=")).map(str::to_owned)
                }
            })
            .collect();
        if values.len() != 1 {
            return Err(format!("exactly one {flag} is required").into());
        }
        Ok(values[0].clone())
    };
    if PathBuf::from(option_value("--out")?) != out.join("attempt") {
        return Err("dataset --out must be OUTPUT/attempt".into());
    }
    if kind == "generate" && option_value("--max-cases")? != "1" {
        return Err("generate preflight requires --max-cases 1".into());
    }
    let expected_input = if kind == "generate" {
        ["--plan", "--protocol"].as_slice()
    } else {
        ["--config"].as_slice()
    };
    let mut inputs = Vec::new();
    for flag in expected_input {
        let path = PathBuf::from(option_value(flag)?);
        if !path.is_absolute() || !path.is_file() {
            return Err(format!("{flag} must name an absolute regular file").into());
        }
        inputs.push(((*flag).to_owned(), path.clone(), sha256(&path)?));
    }
    fs::create_dir(&out)?;
    if kind == "worker" {
        fs::create_dir(out.join("attempt"))?;
    }
    let pre = sha256(&binary)?;
    let stdout_path = out.join("stdout.log");
    let stderr_path = out.join("stderr.log");
    let stdout = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stdout_path)?;
    let stderr = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stderr_path)?;
    let mut command = Command::new("timeout");
    command
        .arg("--signal=TERM")
        .arg("--kill-after=5s")
        .arg(format!("{seconds}s"))
        .arg(&binary)
        .arg("dataset")
        .args(&args[7..])
        .current_dir(&out)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("RAYON_NUM_THREADS", "1")
        .env("TMPDIR", &out)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout.try_clone()?))
        .stderr(Stdio::from(stderr.try_clone()?));
    let started_unix_ns = unix_ns()?;
    let start = Instant::now();
    let result = command.status();
    let elapsed = start.elapsed().as_nanos();
    let finished_unix_ns = unix_ns()?;
    stdout.sync_all()?;
    stderr.sync_all()?;
    let post = sha256(&binary)?;
    let mut input_json = Vec::new();
    let mut inputs_stable = true;
    for (flag, path, before) in &inputs {
        let after = sha256(path)?;
        inputs_stable &= *before == after;
        input_json.push(format!(
            "{{\"flag\":{},\"path\":{},\"sha256_before\":{},\"sha256_after\":{}}}",
            quote(flag),
            quote(&path.to_string_lossy()),
            quote(before),
            quote(&after)
        ));
    }
    let stdout_hash = sha256(&stdout_path)?;
    let stderr_hash = sha256(&stderr_path)?;
    let (exit, launch_error) = match result {
        Ok(status) => (status.code().unwrap_or(-1), String::new()),
        Err(error) => (-1, error.to_string()),
    };
    let mut argv = vec![binary.to_string_lossy().into_owned(), "dataset".into()];
    argv.extend(args[7..].iter().cloned());
    let argv_json = argv.iter().map(|x| quote(x)).collect::<Vec<_>>().join(",");
    let binary_stable = pre == post;
    let success = exit == 0 && binary_stable && inputs_stable;
    let receipt = format!(
        "{{\n  \"schema\": \"ml-dataset.native-runtime-preflight/v1\",\n  \"classification\": \"SOFTWARE\",\n  \"source_commit\": {},\n  \"source_commit_is_supplied_not_git_verified\": true,\n  \"arm\": {},\n  \"command_kind\": {},\n  \"argv\": [{}],\n  \"input_files\": [{}],\n  \"inputs_stable\": {},\n  \"binary_sha256_before\": {},\n  \"binary_sha256_after\": {},\n  \"binary_stable\": {},\n  \"started_unix_nanoseconds\": \"{}\",\n  \"finished_unix_nanoseconds\": \"{}\",\n  \"observed_wall_nanoseconds\": \"{}\",\n  \"timing_scope\": \"timeout_process_spawn_through_wait_excludes_receipt_writes\",\n  \"measurement_source\": \"retained_command_receipts\",\n  \"observed_commands\": 1,\n  \"clock_valid_for_comparison\": false,\n  \"timeout_seconds\": {},\n  \"exit_code\": {},\n  \"timed_out\": {},\n  \"termination_ambiguous\": {},\n  \"launch_error\": {},\n  \"stdout_sha256\": {},\n  \"stderr_sha256\": {},\n  \"stdout_bytes\": {},\n  \"stderr_bytes\": {},\n  \"command_completed\": {},\n  \"verified_cloud_dataset_cells\": 0\n}}\n",
        quote(source), quote(arm), quote(kind), argv_json, input_json.join(","), inputs_stable,
        quote(&pre), quote(&post), binary_stable,
        started_unix_ns, finished_unix_ns, elapsed, seconds, exit, exit == 124, exit == 137,
        quote(&launch_error), quote(&stdout_hash), quote(&stderr_hash),
        fs::metadata(&stdout_path)?.len(), fs::metadata(&stderr_path)?.len(), success,
    );
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("receipt.json"))?;
    file.write_all(receipt.as_bytes())?;
    file.sync_all()?;
    println!("{}", out.join("receipt.json").display());
    Ok(success)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("runtime preflight: {error}");
            ExitCode::from(2)
        }
    }
}
