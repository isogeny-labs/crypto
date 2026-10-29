//! One measured worker process, as the tournament's `execute` runs it:
//! the child is pinned to the declared CPU by pinning the calling thread
//! around the spawn (the child inherits it), started in its own process
//! group without a fork of the evaluator (Rust's `posix_spawn` path; no
//! `pre_exec`), capped with `prlimit` while it is still blocked on stdin,
//! and reaped by a blocking wait.  A watchdog thread started before the
//! timing window kills the whole group at the timeout.

use std::io::Write;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use super::json::{self, J};

/// The native evaluator's spawn, recorded in every process record.
pub const SPAWN: &str = "posix_spawn child (Rust std, own process group) without a fork of the \
evaluator; caps by prlimit before job delivery";

/// The environment a worker sees: the tournament's `child_env`.
pub fn child_env() -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = [
        "PATH",
        "LANG",
        "LC_ALL",
        "TZ",
        "CARGO_HOME",
        "RUSTUP_HOME",
        "HOME",
    ]
    .iter()
    .filter_map(|k| std::env::var(k).ok().map(|v| (k.to_string(), v)))
    .collect();
    for (k, v) in [
        ("RAYON_NUM_THREADS", "1"),
        ("IC_ARTIFACT_CACHE", "off"),
        ("IC_F2_BACKEND", "cpu"),
        ("PYTHONHASHSEED", "0"),
    ] {
        env.retain(|(name, _)| name != k);
        env.push((k.into(), v.into()));
    }
    env
}

pub fn affinity() -> Result<Vec<usize>, String> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        if libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut set) != 0 {
            return Err(format!(
                "sched_getaffinity: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok((0..libc::CPU_SETSIZE as usize)
            .filter(|&c| libc::CPU_ISSET(c, &set))
            .collect())
    }
}

fn set_affinity(cpus: &[usize]) -> Result<(), String> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut set);
        for &c in cpus {
            libc::CPU_SET(c, &mut set);
        }
        if libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &set) != 0 {
            return Err(format!(
                "sched_setaffinity: {}",
                std::io::Error::last_os_error()
            ));
        }
    }
    Ok(())
}

fn prlimit(pid: i32, resource: libc::__rlimit_resource_t, value: u64) -> Result<(), String> {
    let lim = libc::rlimit64 {
        rlim_cur: value,
        rlim_max: value,
    };
    if unsafe { libc::prlimit64(pid, resource, &lim, std::ptr::null_mut()) } != 0 {
        return Err(format!("prlimit: {}", std::io::Error::last_os_error()));
    }
    Ok(())
}

struct Watch {
    pid: Option<i32>,
    fired: bool,
    expired: bool,
    cancelled: bool,
}

fn kill_group(state: &mut Watch) {
    if let Some(pid) = state.pid {
        if unsafe { libc::killpg(pid, libc::SIGKILL) } == 0 {
            state.expired = true;
        }
    }
}

/// `execute(command, job, directory, timeout, memory, cpu)`: the process
/// record the tournament keeps, with `stdout.json` and `stderr.txt` in
/// `directory`.
pub fn execute(
    command: &[String],
    job: &J,
    directory: &Path,
    timeout: f64,
    memory: u64,
    cpu: usize,
) -> Result<J, String> {
    std::fs::create_dir_all(directory).map_err(|e| format!("{}: {e}", directory.display()))?;
    let payload = json::dumps_line(job, true);
    let stdout = std::fs::File::create(directory.join("stdout.json")).map_err(|e| e.to_string())?;
    let stderr = std::fs::File::create(directory.join("stderr.txt")).map_err(|e| e.to_string())?;
    let shared = Arc::new((
        Mutex::new(Watch {
            pid: None,
            fired: false,
            expired: false,
            cancelled: false,
        }),
        Condvar::new(),
    ));
    let watcher = {
        let shared = Arc::clone(&shared);
        std::thread::spawn(move || {
            let (lock, cv) = &*shared;
            let deadline = Instant::now() + Duration::from_secs_f64(timeout);
            let mut state = lock.lock().expect("watch lock");
            loop {
                if state.cancelled {
                    return;
                }
                let now = Instant::now();
                if now >= deadline {
                    break;
                }
                state = cv
                    .wait_timeout(state, deadline - now)
                    .expect("watch lock")
                    .0;
            }
            state.fired = true;
            kill_group(&mut state);
        })
    };
    let inherited = affinity()?;
    set_affinity(&[cpu])?;
    let start = Instant::now();
    let spawned = Command::new(&command[0])
        .args(&command[1..])
        .env_clear()
        .envs(child_env())
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(stderr)
        .process_group(0)
        .spawn();
    set_affinity(&inherited)?;
    let finish = |shared: &Arc<(Mutex<Watch>, Condvar)>| {
        let (lock, cv) = &**shared;
        lock.lock().expect("watch lock").cancelled = true;
        cv.notify_all();
    };
    let mut child = match spawned {
        Ok(c) => c,
        Err(e) => {
            finish(&shared);
            let _ = watcher.join();
            return Err(format!("{}: {e}", command[0]));
        }
    };
    let pid = child.id() as i32;
    {
        let (lock, _) = &*shared;
        let mut state = lock.lock().expect("watch lock");
        state.pid = Some(pid);
        if state.fired {
            kill_group(&mut state);
        }
    }
    let caps = prlimit(pid, libc::RLIMIT_AS, memory).and(prlimit(pid, libc::RLIMIT_CORE, 0));
    if let Some(mut stdin) = child.stdin.take() {
        // A worker that exits before reading its job closes the pipe.
        let _ = stdin.write_all(payload.as_bytes());
    }
    let status = child.wait();
    let wall = start.elapsed().as_secs_f64();
    finish(&shared);
    let _ = watcher.join();
    caps?;
    let status = status.map_err(|e| format!("wait: {e}"))?;
    let expired = shared.0.lock().expect("watch lock").expired;
    let code = status
        .code()
        .unwrap_or_else(|| -status.signal().unwrap_or(0));
    Ok(J::Obj(vec![
        ("exit_code".into(), J::Int(code as i128)),
        ("process_wall_seconds".into(), J::Float(wall)),
        (
            "process_status".into(),
            J::Str(if expired { "TIMEOUT" } else { "EXITED" }.into()),
        ),
        (
            "command".into(),
            J::Arr(command.iter().map(|s| J::Str(s.clone())).collect()),
        ),
        ("memory_cap_bytes".into(), J::Int(memory as i128)),
        ("cpu".into(), J::Int(cpu as i128)),
        ("spawn".into(), J::Str(SPAWN.into())),
    ]))
}

/// A worker run for fixture generation: `subprocess.run([binary],
/// input=json.dumps(job), capture_output=True, env=child_env(),
/// timeout=…, check=True)`; the parsed stdout.
pub fn run_capture(binary: &Path, job: &J, timeout: f64) -> Result<J, String> {
    let mut child = Command::new(binary)
        .env_clear()
        .envs(child_env())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", binary.display()))?;
    let payload = json::dumps_line(job, false);
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(payload.as_bytes());
    }
    let pid = child.id() as i32;
    let done = Arc::new((Mutex::new(false), Condvar::new()));
    let killer = {
        let done = Arc::clone(&done);
        std::thread::spawn(move || {
            let (lock, cv) = &*done;
            let deadline = Instant::now() + Duration::from_secs_f64(timeout);
            let mut finished = lock.lock().expect("lock");
            while !*finished {
                let now = Instant::now();
                if now >= deadline {
                    unsafe { libc::kill(pid, libc::SIGKILL) };
                    return true;
                }
                finished = cv.wait_timeout(finished, deadline - now).expect("lock").0;
            }
            false
        })
    };
    let output = child.wait_with_output();
    {
        let (lock, cv) = &*done;
        *lock.lock().expect("lock") = true;
        cv.notify_all();
    }
    let timed_out = killer.join().unwrap_or(false);
    let output = output.map_err(|e| e.to_string())?;
    if timed_out {
        return Err(format!(
            "TimeoutExpired: {} after {timeout} s",
            binary.display()
        ));
    }
    if !output.status.success() {
        return Err(format!(
            "CalledProcessError: {} returned {:?}",
            binary.display(),
            output.status.code()
        ));
    }
    json::parse(&String::from_utf8_lossy(&output.stdout))
}
