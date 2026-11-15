//! One native process, preserving failures and all isolation observations.
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::{canonical::sha256_hex, host, isolation as iso, measurement as m, signals};

pub struct Config {
    pub argv: Vec<String>,
    pub directory: PathBuf,
    pub output: PathBuf,
    pub library: PathBuf,
    pub public_inputs: Vec<PathBuf>,
    pub cpus: iso::CpuRequest,
    pub timeout: Duration,
    pub diagnostic: bool,
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(bytes)
        .and_then(|()| f.sync_all())
        .map_err(|e| e.to_string())
}

fn hash_fd(file: &File) -> Result<String, String> {
    let mut file = file.try_clone().map_err(|e| e.to_string())?;
    file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    Ok(sha256_hex(&bytes))
}

fn ns(t: libc::timeval) -> Result<u64, String> {
    u64::try_from(t.tv_sec)
        .ok()
        .and_then(|s| s.checked_mul(1_000_000_000))
        .and_then(|s| {
            u64::try_from(t.tv_usec)
                .ok()
                .and_then(|u| u.checked_mul(1000))
                .and_then(|u| s.checked_add(u))
        })
        .ok_or_else(|| "invalid rusage interval".into())
}

fn hash_inputs(paths: &[PathBuf]) -> Result<std::collections::BTreeMap<String, String>, String> {
    let mut hashes = std::collections::BTreeMap::new();
    for path in paths {
        let name = path.to_str().ok_or("input path must be UTF8")?.to_owned();
        if !path.is_absolute() || hashes.contains_key(&name) {
            return Err("absolute, distinct public input paths required".into());
        }
        hashes.insert(
            name,
            hash_fd(&File::open(path).map_err(|e| e.to_string())?)?,
        );
    }
    Ok(hashes)
}

/// A process-resident report must belong to this exact process and fit the
/// parent's enclosing interval. Missing/malformed reports never gain a level.
pub fn parse_child(
    bytes: &[u8],
    pid: u32,
    parent_wall: u64,
) -> Result<crate::ChildObservations, String> {
    if bytes.len() > 131_072 {
        return Err("observation byte cap".into());
    }
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    crate::canonical::reject_duplicate_keys(text)?;
    let child: crate::ChildObservations =
        serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if child.schema != crate::CHILD_SCHEMA || child.pid != pid {
        return Err("observation schema/process mismatch".into());
    }
    if child.algorithm_wall_ns == 0 || child.algorithm_wall_ns > parent_wall {
        return Err("observation interval outside parent cold interval".into());
    }
    Ok(child)
}

/// Collect one process. This does not verify its mathematical output, establish
/// A/A admission or promote partial counters to a complete native-work price.
pub fn run(config: &Config) -> Result<Value, String> {
    if config.argv.is_empty()
        || !Path::new(&config.argv[0]).is_absolute()
        || !config.library.is_absolute()
        || !config.directory.is_absolute()
        || config.timeout.is_zero()
        || config.timeout > Duration::from_secs(21_600)
    {
        return Err("absolute executable/library/workdir and timeout in (0,21600] required".into());
    }
    if !config.diagnostic && matches!(config.cpus, iso::CpuRequest::None) {
        return Err("strict measurements require CPU reservation".into());
    }
    if !config.diagnostic && config.public_inputs.is_empty() {
        return Err("strict measurements require explicit public input bindings".into());
    }
    signals::install();
    let _lock = iso::BenchLock::acquire(iso::DEFAULT_LOCK, false)?;
    std::fs::create_dir(&config.output)
        .map_err(|e| format!("new output directory required: {e}"))?;
    let result = collect(config);
    if let Err(error) = &result {
        write_new(
            &config.output.join("blocked.json"),
            &serde_json::to_vec_pretty(
                &json!({"schema":"cubic.native_command_blocked/v1", "error":error,
                "performance_admitted":false}),
            )
            .map_err(|e| e.to_string())?,
        )?;
    }
    result
}

fn collect(config: &Config) -> Result<Value, String> {
    let capsule = host::capture()?;
    write_new(
        &config.output.join("host.json"),
        &serde_json::to_vec_pretty(&capsule).map_err(|e| e.to_string())?,
    )?;
    write_new(
        &config.output.join("request.json"),
        &serde_json::to_vec_pretty(&json!({
            "argv":config.argv,"directory":config.directory,"library":config.library,
            "cpus":config.cpus,"timeout_ns":config.timeout.as_nanos().to_string(),
        "diagnostic":config.diagnostic,"public_inputs":config.public_inputs
        }))
        .map_err(|e| e.to_string())?,
    )?;
    let allowed = iso::allowed_cpus();
    let plan = iso::plan_cpus(&config.cpus, &capsule.stable.topology, allowed.as_deref())?;
    let thresholds = iso::Thresholds::default();
    let mut runner_on_run_cpu = false;
    let mut eviction = None;
    let mut preflight = None;
    if let Some(p) = &plan {
        if iso::leave_reserved(&p.reserved)
            .or_else(|_| iso::leave_reserved(&[p.run_cpu]))
            .is_err()
        {
            runner_on_run_cpu = true;
        }
        eviction = Some(iso::evict(&p.reserved));
        let pre = iso::preflight(p, &thresholds);
        write_new(
            &config.output.join("preflight.json"),
            &serde_json::to_vec_pretty(&pre).map_err(|e| e.to_string())?,
        )?;
        if !pre.quiet && !config.diagnostic {
            return Err(format!("preflight refused: {:?}", pre.reasons));
        }
        preflight = Some(pre);
    }
    if signals::interrupted().is_some() {
        return Err("interrupted before spawn".into());
    }
    let executable = File::open(&config.argv[0]).map_err(|e| e.to_string())?;
    let library = File::open(&config.library).map_err(|e| e.to_string())?;
    let exe_hash = hash_fd(&executable)?;
    let lib_hash = hash_fd(&library)?;
    let inputs_before = hash_inputs(&config.public_inputs)?;
    let observation_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(config.output.join("child-observations.json"))
        .map_err(|e| e.to_string())?;
    let out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config.output.join("stdout.txt"))
        .map_err(|e| e.to_string())?;
    let err = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config.output.join("stderr.txt"))
        .map_err(|e| e.to_string())?;
    let mut command = Command::new(format!("/proc/self/fd/{}", executable.as_raw_fd()));
    command
        .args(&config.argv[1..])
        .current_dir(&config.directory)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    command.env_clear();
    // Only declared runtime lookup paths and fixed thread counts are forwarded.
    // Credentials and the caller's remaining environment are never propagated.
    let mut environment = std::collections::BTreeMap::new();
    for name in ["PATH", "LD_LIBRARY_PATH", "SINGULAR_BIN"] {
        if let Ok(value) = std::env::var(name) {
            environment.insert(name.to_owned(), value);
        }
    }
    for name in [
        "RAYON_NUM_THREADS",
        "OMP_NUM_THREADS",
        "OPENBLAS_NUM_THREADS",
        "MKL_NUM_THREADS",
    ] {
        environment.insert(name.into(), "1".into());
    }
    environment.insert("LC_ALL".into(), "C".into());
    command
        .envs(&environment)
        .env(
            "CUBIC_MEASURE_LIBRARY",
            format!("/proc/self/fd/{}", library.as_raw_fd()),
        )
        .env("CUBIC_MEASURE_FD", observation_file.as_raw_fd().to_string());
    let parent = std::process::id();
    let fds = [
        executable.as_raw_fd(),
        library.as_raw_fd(),
        observation_file.as_raw_fd(),
    ];
    let cpu = plan.as_ref().map(|p| (p.run_cpu, p.node));
    unsafe {
        command.pre_exec(move || {
            signals::prepare_child(parent)?;
            if libc::setpgid(0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            for fd in fds {
                let flags = libc::fcntl(fd, libc::F_GETFD);
                if flags < 0 || libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            if let Some((cpu, node)) = cpu {
                iso::pin_in_child(cpu, node)?;
            }
            Ok(())
        });
    }
    let observed = plan.as_ref().map(|p| p.observed_cpus()).unwrap_or_default();
    let reserved = plan
        .as_ref()
        .map(|p| p.reserved.clone())
        .unwrap_or_default();
    let before = iso::conditions_scanned(&observed, &reserved);
    let begin = Instant::now();
    let child = command.spawn().map_err(|e| format!("spawn: {e}"))?;
    let pid = child.id();
    signals::set_child(pid as i32);
    let state = Arc::new((Mutex::new((false, false)), Condvar::new()));
    let watched = state.clone();
    let timeout = config.timeout;
    let watchdog = std::thread::spawn(move || {
        let (lock, cv) = &*watched;
        let pair = lock.lock().unwrap_or_else(|e| e.into_inner());
        // Scheduling the watchdog must not extend the original deadline.
        let remaining = timeout.saturating_sub(begin.elapsed());
        let (mut pair, wait) = cv
            .wait_timeout_while(pair, remaining, |p| !p.0)
            .unwrap_or_else(|e| e.into_inner());
        if wait.timed_out() && !pair.0 {
            pair.1 = true;
            unsafe {
                libc::kill(-(pid as i32), libc::SIGKILL);
            }
        }
    });
    let mut status = 0;
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    let waited = loop {
        let value = unsafe { libc::wait4(pid as i32, &mut status, 0, &mut usage) };
        if value < 0 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
            continue;
        }
        break value;
    };
    let process_wall_ns =
        u64::try_from(begin.elapsed().as_nanos()).map_err(|_| "process interval overflow")?;
    let timed_out = {
        let (lock, cv) = &*state;
        let mut pair = lock.lock().unwrap_or_else(|e| e.into_inner());
        pair.0 = true;
        cv.notify_all();
        pair.1 || Duration::from_nanos(process_wall_ns) > config.timeout
    };
    let _ = watchdog.join();
    signals::clear_child();
    // A successful leader cannot leave a background computation uncharged.
    let descendants_left = unsafe { libc::kill(-(pid as i32), libc::SIGKILL) } == 0;
    drop(child);
    let after = iso::conditions_scanned(&observed, &reserved);
    if waited != pid as i32 {
        return Err(format!("wait4: {}", std::io::Error::last_os_error()));
    }
    let bytes =
        std::fs::read(config.output.join("child-observations.json")).map_err(|e| e.to_string())?;
    let parsed = parse_child(&bytes, pid, process_wall_ns);
    let observation_error = parsed.as_ref().err().cloned();
    let observations = parsed.ok();
    let timing = m::Timing {
        process_wall_ns,
        user_ns: ns(usage.ru_utime)?,
        sys_ns: ns(usage.ru_stime)?,
        max_rss_kib: u64::try_from(usage.ru_maxrss).map_err(|_| "negative RSS")?,
        minor_faults: usage.ru_minflt as u64,
        major_faults: usage.ru_majflt as u64,
        voluntary_switches: usage.ru_nvcsw as u64,
        involuntary_switches: usage.ru_nivcsw as u64,
        solve_wall_ns: observations.as_ref().map(|o| o.algorithm_wall_ns),
        schedstat_solve: observations.as_ref().and_then(|o| o.schedstat),
        solve: observations.as_ref().map(|o| o.solve.clone()),
        ..Default::default()
    };
    let placement = observations
        .as_ref()
        .map(|o| o.placement.clone())
        .unwrap_or_default();
    let grade = m::grade(&m::GradeInput {
        plan: plan.as_ref(),
        preflight: preflight.as_ref(),
        eviction: eviction.as_ref().map(|e| &e.summary),
        runner_on_run_cpu,
        capsule: &capsule,
        thresholds: &thresholds,
        placement: &placement,
        before: &before,
        after: &after,
        timing: &timing,
        hz: iso::ticks_per_second() as f64,
    });
    let binary_changed = hash_fd(&executable)? != exe_hash || hash_fd(&library)? != lib_hash;
    let inputs_after = hash_inputs(&config.public_inputs)?;
    let inputs_changed = inputs_before != inputs_after;
    let exit_code = libc::WIFEXITED(status).then(|| libc::WEXITSTATUS(status));
    let clean = exit_code == Some(0)
        && !timed_out
        && signals::interrupted().is_none()
        && !descendants_left
        && !binary_changed
        && !inputs_changed
        && observation_error.is_none();
    let record = json!({
        "schema":"cubic.native_command/v1", "argv":config.argv, "working_directory":config.directory,
        "environment":environment, "host":capsule, "executable_sha256":exe_hash, "library_sha256":lib_hash,
        "pid":pid, "cpu_plan":plan, "thresholds":thresholds, "preflight":preflight,
        "inputs_before":inputs_before,"inputs_after":inputs_after,
        "eviction":eviction.as_ref().map(|e| &e.summary), "runner_on_run_cpu":runner_on_run_cpu,
        "before":before, "after":after, "timing":timing, "child":observations,
        "isolation":{"grading_version":m::GRADING_VERSION,"level":grade.level,"blockers":grade.blockers},
        "outcome":{"completed":clean,"exit_code":exit_code,"signal":libc::WIFSIGNALED(status).then(||libc::WTERMSIG(status)),
            "timed_out":timed_out,"interrupted":signals::interrupted(),"descendants_left":descendants_left,
            "binary_changed":binary_changed,"inputs_changed":inputs_changed,"observation_error":observation_error},
        "isolation_wall_eligible":clean && !config.diagnostic && grade.level >= iso::Level::L2,
        "performance_admitted":false, "mathematical_verification":null,
        "complete_native_gae":null, "counter_scope":"child PMU lifecycle; not a complete cold instruction or GAE price",
        "files":{"stdout_sha256":sha256_hex(&std::fs::read(config.output.join("stdout.txt")).map_err(|e|e.to_string())?),
            "stderr_sha256":sha256_hex(&std::fs::read(config.output.join("stderr.txt")).map_err(|e|e.to_string())?),
            "observations_sha256":sha256_hex(&bytes)}
    });
    write_new(
        &config.output.join("record.json"),
        &serde_json::to_vec_pretty(&record).map_err(|e| e.to_string())?,
    )?;
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_wrong_process_and_duplicate_observations_are_rejected() {
        assert!(parse_child(b"", 1, 10).is_err());
        assert!(parse_child(br#"{"pid":1,"pid":2}"#, 1, 10).is_err());
        assert!(parse_child(br#"{"schema":"invented"}"#, 1, 10).is_err());
    }
}
