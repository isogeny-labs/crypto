//! Native, source-bound external-XOR process gate for the retained N83 K2048 m5 formula.
//! Usage: n83_k2048_m5_native_gate positive|natural LOCK_JSON NEW_RUN_DIR

use crypto_lib::hash::sha256::Sha256;
use serde_json::{json, Value};
use std::error::Error;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn file_sha(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut sha = Sha256::new();
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha.update(&buffer[..count]);
    }
    Ok(hex::encode(sha.finalize()))
}

fn unix_ms() -> Result<u128> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

fn load_1m() -> Result<f64> {
    let mut values = [0.0f64; 3];
    if unsafe { libc::getloadavg(values.as_mut_ptr(), 3) } != 3 {
        return Err("getloadavg failed".into());
    }
    Ok(values[0])
}

fn swapouts() -> Result<u64> {
    let output = Command::new("/usr/bin/vm_stat").output()?;
    if !output.status.success() {
        return Err("vm_stat failed".into());
    }
    let text = String::from_utf8(output.stdout)?;
    let value = text
        .split("Swapouts:")
        .nth(1)
        .ok_or("vm_stat has no swapout count")?
        .split_whitespace()
        .next()
        .ok_or("vm_stat swapout value absent")?
        .trim_end_matches('.')
        .parse()?;
    Ok(value)
}

fn free_bytes(path: &Path) -> Result<u64> {
    let name = CString::new(path.as_os_str().as_bytes())?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(name.as_ptr(), &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok((stat.f_bavail as u64).saturating_mul(stat.f_frsize as u64))
}

fn get_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("lock {key} absent").into())
}

fn get_u64(value: &Value, key: &str) -> Result<u64> {
    value[key]
        .as_u64()
        .ok_or_else(|| format!("lock {key} absent").into())
}

fn checked_lock(mode: &str, lock: &Value, out: &Path) -> Result<(PathBuf, PathBuf, PathBuf, Vec<String>)> {
    if lock["schema"] != "n83.k2048-m5-native-gate-lock/v1"
        || lock["mode"] != mode
        || !["positive", "natural"].contains(&mode)
        || out.exists()
    {
        return Err("native gate mode, lock or output path differs".into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_commit = get_str(lock, "source_commit")?;
    let source_type = Command::new("git")
        .current_dir(root)
        .args(["cat-file", "-t", source_commit])
        .output()?;
    if !source_type.status.success() || source_type.stdout != b"commit\n" {
        return Err("native gate source commit unavailable".into());
    }
    let clean = Command::new("git")
        .current_dir(root)
        .args(["status", "--porcelain", "--untracked-files=normal"])
        .output()?;
    if !clean.status.success() || !clean.stdout.is_empty() {
        return Err("native gate requires a clean worktree".into());
    }
    let source = root.join("examples/n83_k2048_m5_native_gate.rs");
    if file_sha(&source)? != get_str(lock, "runner_source_sha256")?
        || file_sha(&std::env::current_exe()?)? != get_str(lock, "runner_binary_sha256")?
    {
        return Err("native gate runner source or binary differs".into());
    }
    let solver = PathBuf::from(get_str(lock, "solver")?);
    if file_sha(&solver)? != get_str(lock, "solver_sha256")? {
        return Err("native gate solver binary differs".into());
    }
    let formula = PathBuf::from(get_str(lock, "formula")?);
    if file_sha(&formula)? != get_str(lock, "formula_sha256")?
        || fs::metadata(&formula)?.len() != get_u64(lock, "formula_bytes")?
    {
        return Err("native gate formula differs".into());
    }
    let header = fs::read_to_string(&formula)?
        .lines()
        .next()
        .ok_or("native gate formula header absent")?
        .to_owned();
    if header != get_str(lock, "formula_header")? {
        return Err("native gate formula header differs".into());
    }
    let export = PathBuf::from(get_str(lock, "export_receipt")?);
    if file_sha(&export)? != get_str(lock, "export_receipt_sha256")? {
        return Err("native gate export receipt differs".into());
    }
    let export_record: Value = serde_json::from_slice(&fs::read(&export)?)?;
    if export_record["status"] != "PASS_XCNF_EXPORT_ONLY"
        || export_record["mode"] != mode
        || export_record["xcnf_sha256"] != lock["formula_sha256"]
    {
        return Err("native gate export identity differs".into());
    }
    if mode == "natural" {
        let replay = Path::new(get_str(lock, "positive_native_replay")?);
        if file_sha(replay)? != get_str(lock, "positive_native_replay_sha256")? {
            return Err("native positive replay hash differs".into());
        }
        let record: Value = serde_json::from_slice(&fs::read(replay)?)?;
        if record["status"] != "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY" {
            return Err("native positive replay gate differs".into());
        }
    }
    let flags = lock["solver_flags"]
        .as_array()
        .ok_or("native solver flags absent")?
        .iter()
        .map(|v| v.as_str().map(str::to_owned).ok_or("native solver flag is not text".into()))
        .collect::<Result<Vec<String>>>()?;
    if flags != [
        "--threads", "1", "--random", "20261114", "--maxtime", "600",
        "--maxconfl", "1000000", "--verb", "1", "--verbstat", "1",
        "--printsol", "1", "--maxnummatrices", "32", "--autodisablegauss",
        "0", "--gaussusefulcutoff", "0", "--branchstr", "vsids",
    ] {
        return Err("native solver flags differ from frozen arm".into());
    }
    Ok((solver, formula, export, flags))
}

fn timeval_seconds(t: libc::timeval) -> f64 {
    t.tv_sec as f64 + t.tv_usec as f64 / 1_000_000.0
}

fn run(mode: &str, lock_path: &Path, out: &Path) -> Result<()> {
    let lock: Value = serde_json::from_slice(&fs::read(lock_path)?)?;
    let (solver, formula, export, flags) = checked_lock(mode, &lock, out)?;
    if free_bytes(out.parent().ok_or("native output has no parent")?)? < 2 * 1024 * 1024 * 1024 {
        return Err("native gate scratch below 2 GiB".into());
    }
    let mut samples = Vec::new();
    for index in 0..3 {
        samples.push(json!({"unix_ms":unix_ms()?,"load_1m":load_1m()?,"swapouts":swapouts()?}));
        if index != 2 {
            thread::sleep(Duration::from_secs(15));
        }
    }
    fs::create_dir(out)?;
    let mut host = File::create_new(out.join("host-preflight.json"))?;
    serde_json::to_writer_pretty(&mut host, &samples)?;
    host.write_all(b"\n")?;
    host.sync_all()?;
    let swap = samples[0]["swapouts"].as_u64().ok_or("native preflight swapout missing")?;
    if samples.iter().any(|sample| sample["load_1m"].as_f64().is_none_or(|load| load > 24.0)
        || sample["swapouts"].as_u64() != Some(swap))
        || load_1m()? > 24.0
        || swapouts()? != swap
    {
        return Err("native host load or swapout gate failed; preflight retained".into());
    }
    let mut command = vec![solver.to_string_lossy().to_string()];
    command.extend(flags);
    command.push(formula.to_string_lossy().to_string());
    let mut cmd_file = File::create_new(out.join("command.json"))?;
    serde_json::to_writer_pretty(&mut cmd_file, &command)?;
    cmd_file.write_all(b"\n")?;
    cmd_file.sync_all()?;
    let stdout_path = out.join("stdout.txt");
    let stderr_path = out.join("stderr.txt");
    let mut child_command = Command::new(&solver);
    child_command
        .args(&command[1..])
        .stdout(Stdio::from(File::create_new(&stdout_path)?))
        .stderr(Stdio::from(File::create_new(&stderr_path)?));
    unsafe {
        child_command.pre_exec(|| {
            if libc::setsid() < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
    let started_unix_ms = unix_ms()?;
    let began = Instant::now();
    let child = child_command.spawn()?;
    let pid = child.id() as libc::pid_t;
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    let (code, timed_out) = loop {
        let mut raw = 0;
        let waited = unsafe { libc::wait4(pid, &mut raw, libc::WNOHANG, &mut usage) };
        if waited == pid {
            let code = if libc::WIFEXITED(raw) {
                libc::WEXITSTATUS(raw)
            } else if libc::WIFSIGNALED(raw) {
                -libc::WTERMSIG(raw)
            } else {
                -255
            };
            break (code, false);
        }
        if waited < 0 {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            return Err(error.into());
        }
        if began.elapsed() >= Duration::from_secs(660) {
            unsafe { libc::kill(-pid, libc::SIGTERM) };
            thread::sleep(Duration::from_secs(5));
            let mut raw = 0;
            let mut waited = unsafe { libc::wait4(pid, &mut raw, libc::WNOHANG, &mut usage) };
            if waited == 0 {
                unsafe { libc::kill(-pid, libc::SIGKILL) };
                waited = unsafe { libc::wait4(pid, &mut raw, 0, &mut usage) };
            }
            if waited != pid {
                return Err(std::io::Error::last_os_error().into());
            }
            let code = if libc::WIFEXITED(raw) { libc::WEXITSTATUS(raw) } else { -libc::WTERMSIG(raw) };
            break (code, true);
        }
        thread::sleep(Duration::from_millis(100));
    };
    let transcript = fs::read_to_string(&stdout_path)?;
    let statuses = transcript.lines().filter(|line| line.starts_with("s ")).collect::<Vec<_>>();
    let state = match (timed_out, code, statuses.as_slice()) {
        (true, _, _) => "WATCHDOG",
        (false, 10, ["s SATISFIABLE"]) => "SAT_PENDING_REPLAY",
        (false, 15, ["s INDETERMINATE"]) => "INDETERMINATE",
        (false, 20, ["s UNSATISFIABLE"]) => "UNSAT_UNVERIFIED",
        _ => "PRODUCER_FAILURE",
    };
    let conflicts = transcript.lines().filter_map(|line| line.strip_prefix("c conflicts                :"))
        .map(str::trim).collect::<Vec<_>>();
    let record = json!({
        "schema":"n83.k2048-m5-native-solver-run/v1", "mode":mode,"status":state,
        "lock_sha256":file_sha(lock_path)?,"source_commit":lock["source_commit"],
        "runner_binary_sha256":lock["runner_binary_sha256"],"solver_sha256":lock["solver_sha256"],
        "formula_sha256":lock["formula_sha256"],"export_receipt_sha256":file_sha(&export)?,
        "command":command,"started_unix_ms":started_unix_ms,"ended_unix_ms":unix_ms()?,
        "exit_code":code,"timed_out":timed_out,"wall_seconds":began.elapsed().as_secs_f64(),
        "cpu_user_seconds":timeval_seconds(usage.ru_utime),
        "cpu_system_seconds":timeval_seconds(usage.ru_stime),
        "wait4_peak_rss_bytes":if cfg!(target_os="macos") {usage.ru_maxrss as u64} else {(usage.ru_maxrss as u64)*1024},
        "terminal_conflicts":if conflicts.len()==1 {conflicts[0].parse::<u64>().ok()} else {None},
        "host_preflight":samples,"swapouts_after":swapouts()?,
        "stdout_sha256":file_sha(&stdout_path)?,"stderr_sha256":file_sha(&stderr_path)?,
        "selected_best_total_runtime":Value::Null,
    });
    let mut receipt = File::create_new(out.join("receipt.json"))?;
    serde_json::to_writer_pretty(&mut receipt, &record)?;
    receipt.write_all(b"\n")?;
    receipt.sync_all()?;
    println!("{}", json!({"status":state,"exit_code":code,"wall_seconds":record["wall_seconds"],"terminal_conflicts":record["terminal_conflicts"]}));
    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = if args.len() == 4 {
        run(&args[1], Path::new(&args[2]), Path::new(&args[3]))
    } else {
        Err("usage: n83_k2048_m5_native_gate positive|natural LOCK_JSON NEW_RUN_DIR".into())
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
