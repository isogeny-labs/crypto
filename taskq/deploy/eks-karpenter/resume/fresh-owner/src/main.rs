use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::os::{
    fd::{AsRawFd, FromRawFd},
    unix::{fs::OpenOptionsExt, process::CommandExt},
};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};
fn error(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read_at(path: &Path, name: &str, cap: usize) -> Result<(Vec<u8>, File, File), String> {
    if !path.is_absolute() {
        return Err("absolute directory required".into());
    }
    let mut parts = Vec::new();
    for c in path.components() {
        match c {
            std::path::Component::RootDir => (),
            std::path::Component::Normal(s) => parts.push(s.to_str().ok_or("non UTF8 path")?),
            _ => return Err("non-normal path".into()),
        }
    }
    read_session_at(Path::new("/"), &parts, name, cap)
}
#[derive(Clone, Serialize, Deserialize)]
struct Clock {
    raw_ns: u64,
    boot_id: String,
    time_namespace: String,
    time_namespace_offsets: String,
}
impl Clock {
    fn read() -> Result<Self, String> {
        let mut t = std::mem::MaybeUninit::<libc::timespec>::uninit();
        // SAFETY: initialized timespec is read only after success.
        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, t.as_mut_ptr()) } != 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
        let t = unsafe { t.assume_init() };
        let sec = u64::try_from(t.tv_sec).map_err(error)?;
        let ns = u64::try_from(t.tv_nsec).map_err(error)?;
        if ns >= 1_000_000_000 {
            return Err("invalid monotonic nanoseconds".into());
        }
        Ok(Self {
            raw_ns: sec
                .checked_mul(1_000_000_000)
                .and_then(|v| v.checked_add(ns))
                .ok_or("clock overflow")?,
            boot_id: fs::read_to_string("/proc/sys/kernel/random/boot_id")
                .map_err(error)?
                .trim()
                .into(),
            time_namespace: fs::read_link("/proc/self/ns/time")
                .map_err(error)?
                .to_string_lossy()
                .into_owned(),
            time_namespace_offsets: fs::read_to_string("/proc/self/timens_offsets")
                .map_err(error)?,
        })
    }
    fn same_epoch(&self, other: &Self) -> bool {
        self.boot_id == other.boot_id
            && self.time_namespace == other.time_namespace
            && self.time_namespace_offsets == other.time_namespace_offsets
            && other.raw_ns >= self.raw_ns
    }
}
fn read_session_at(
    root: &Path,
    parts: &[&str],
    name: &str,
    cap: usize,
) -> Result<(Vec<u8>, File, File), String> {
    let mut dir = File::open(root).map_err(error)?;
    for part in parts {
        let part = std::ffi::CString::new(*part).map_err(error)?;
        let fd = unsafe {
            libc::openat(
                dir.as_raw_fd(),
                part.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
        dir = unsafe { File::from_raw_fd(fd) };
    }
    let name = std::ffi::CString::new(name).map_err(error)?;
    let fd = unsafe {
        libc::openat(
            dir.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK,
        )
    };
    if fd < 0 {
        return Err(error(std::io::Error::last_os_error()));
    }
    let file = unsafe { File::from_raw_fd(fd) };
    if !file.metadata().map_err(error)?.is_file() {
        return Err("session evidence not regular file".into());
    }
    let mut bytes = Vec::new();
    (&file)
        .take(cap as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(error)?;
    if bytes.len() > cap {
        return Err("session evidence byte cap".into());
    }
    Ok((bytes, file, dir))
}
fn bind_files(
    session_path: &Path,
    clock: &Clock,
    owner_begin_raw_ns: u64,
) -> Result<Value, String> {
    let (prep, prep_file, prep_dir) =
        read_at(session_path, "session-preparation.json", 256 * 1024)?;
    let (journal, journal_file, journal_dir) =
        read_at(session_path, "resource-journal.json", 8 * 1024 * 1024)?;
    let Unique(p) = serde_json::from_slice(&prep).map_err(error)?;
    let Unique(j) = serde_json::from_slice(&journal).map_err(error)?;
    let text = |v: &Value, key: &str| {
        v.get(key)
            .and_then(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| format!("missing {key}"))
    };
    if p["schema"] != "ecbench.session_preparation/v1"
        || j["schema"] != "ecbench.resource_publication_journal/v1"
        || p.get("clock_id").is_some()
    {
        return Err("preparation/journal schema mismatch".into());
    }
    let ph = digest(&prep);
    let jh = digest(&journal);
    let runner = text(&p, "runner_id")?;
    let session = text(&p, "session_id")?;
    let cid = format!("{runner}-preparation-sha256-{ph}");
    if text(&j, "runner_id")? != runner
        || text(&j, "session_id")? != session
        || text(&j, "clock_id")? != cid
    {
        return Err("exact preparation clock binding mismatch".into());
    }
    if p["clock_type"] != "linux_CLOCK_MONOTONIC"
        || text(&p, "boot_id")? != clock.boot_id
        || text(&p, "time_namespace")? != clock.time_namespace
        || text(&p, "time_namespace_offsets")? != clock.time_namespace_offsets
    {
        return Err("clock namespace mismatch".into());
    }
    let origin = p["origin_ns"].as_u64().ok_or("origin missing")?;
    if origin < owner_begin_raw_ns {
        return Err("preparation origin precedes owner epoch".into());
    }
    let offset = clock
        .raw_ns
        .checked_sub(origin)
        .ok_or("origin after reap observation")?;
    prep_file.sync_all().map_err(error)?;
    journal_file.sync_all().map_err(error)?;
    prep_dir.sync_all().map_err(error)?;
    journal_dir.sync_all().map_err(error)?;
    let sync = Clock::read()?;
    if !clock.same_epoch(&sync) {
        return Err("clock changed during owner evidence sync".into());
    }
    let enclosing = sync
        .raw_ns
        .checked_sub(origin)
        .ok_or("origin after evidence sync")?;
    Ok(
        json!({"preparation_sha256":ph,"journal_sha256":jh,"session_id":session,"runner_id":runner,"clock_id":cid,
        "reap_observed_offset_ns":offset,"journal_sync_clock":sync,"enclosing_evidence_wall_ns":enclosing,
        "estimand":"run_session_origin_to_owner_post_reap_evidence_sync",
        "scope":"includes post-reap evidence read and sync; not exact runner latency, CPU or worker allocation"}),
    )
}

// Reject duplicate keys recursively before interpreting untrusted metadata.
struct Unique(Value);
impl<'de> serde::Deserialize<'de> for Unique {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Unique;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "JSON without duplicate keys")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut a: A) -> Result<Unique, A::Error> {
                let mut out = Vec::new();
                while let Some(Unique(v)) = a.next_element()? {
                    out.push(v);
                }
                Ok(Unique(Value::Array(out)))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut a: A) -> Result<Unique, A::Error> {
                let mut out = serde_json::Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if out.contains_key(&k) {
                        return Err(serde::de::Error::custom("duplicate JSON key"));
                    }
                    let Unique(v) = a.next_value()?;
                    out.insert(k, v);
                }
                Ok(Unique(Value::Object(out)))
            }
        }
        d.deserialize_any(V)
    }
}
pub fn sha_file(path: &Path) -> Result<(String, u64), String> {
    let mut file = File::open(path).map_err(error)?;
    let mut sha = Sha256::new();
    let mut count = 0;
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf).map_err(error)?;
        if n == 0 {
            break;
        }
        sha.update(&buf[..n]);
        count += n as u64;
    }
    Ok((format!("{:x}", sha.finalize()), count))
}
fn starttime(pid: i32) -> Result<String, String> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).map_err(error)?;
    Ok(s.rsplit_once(") ")
        .ok_or("bad proc stat")?
        .1
        .split_whitespace()
        .nth(19)
        .ok_or("missing proc starttime")?
        .into())
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Config {
    schema: String,
    phase: String,
    request_sha256: String,
    launch_sha256: String,
    launch: PathBuf,
    pod_uid: String,
    job_uid: String,
    image_digest: String,
    binary_sha256: String,
    binary: PathBuf,
    argv: Vec<String>,
    spec: PathBuf,
    spec_sha256: String,
    session: PathBuf,
    receipt: PathBuf,
    stop_utc_seconds: u64,
}
fn hash_shape(v: &str) -> bool {
    v.len() == 64
        && v.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn now() -> Result<u64, String> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(error)?
        .as_secs())
}
fn validate(c: &Config) -> Result<(), String> {
    if c.schema != "isolab.fresh-owner-config/v1" || !["run", "replay"].contains(&c.phase.as_str())
    {
        return Err("schema or phase".into());
    }
    if [
        &c.request_sha256,
        &c.launch_sha256,
        &c.binary_sha256,
        &c.spec_sha256,
        &c.image_digest,
    ]
    .iter()
    .any(|v| !hash_shape(v))
    {
        return Err("digest shape".into());
    }
    if c.pod_uid.is_empty()
        || c.job_uid.is_empty()
        || c.pod_uid.len() > 128
        || c.job_uid.len() > 128
    {
        return Err("pod/job identity".into());
    }
    if [&c.binary, &c.spec, &c.session, &c.receipt, &c.launch]
        .iter()
        .any(|v| !v.is_absolute())
    {
        return Err("absolute paths required".into());
    }
    if c.argv.len() > 64
        || c.argv.iter().any(|v| v.len() > 4096)
        || c.argv.first().map(String::as_str)
            != Some(if c.phase == "run" { "run" } else { "verify" })
    {
        return Err("argv phase/bounds".into());
    }
    let argument = |key: &str| -> Result<&str, String> {
        let positions: Vec<_> = c
            .argv
            .iter()
            .enumerate()
            .filter(|(_, v)| v.as_str() == key)
            .map(|(i, _)| i)
            .collect();
        if positions.len() != 1 {
            return Err(format!("exactly one {key} required"));
        }
        c.argv
            .get(positions[0] + 1)
            .map(String::as_str)
            .ok_or_else(|| format!("missing {key} value"))
    };
    if c.phase == "run" {
        if argument("--spec")? != c.spec.to_str().ok_or("spec UTF8")?
            || argument("--out")? != c.session.to_str().ok_or("session UTF8")?
        {
            return Err("run path binding".into());
        }
    } else if argument("--dir")? != c.session.to_str().ok_or("session UTF8")?
        || !c.argv.iter().any(|v| v == "--replay-all")
    {
        return Err("replay binding".into());
    }
    let (launch, _, _) = read_at(
        c.launch.parent().ok_or("launch parent")?,
        c.launch
            .file_name()
            .and_then(|v| v.to_str())
            .ok_or("launch filename")?,
        65536,
    )?;
    if digest(&launch) != c.launch_sha256 {
        return Err("launch digest mismatch".into());
    }
    if c.phase == "run"
        && argument("--diagnostic-launch")? != c.launch.to_str().ok_or("launch UTF8")?
    {
        return Err("launch argument mismatch".into());
    }
    if sha_file(&c.binary)?.0 != c.binary_sha256 {
        return Err("prelaunch binary digest".into());
    }
    let (b, _, _) = read_at(
        c.spec.parent().ok_or("spec parent")?,
        c.spec
            .file_name()
            .and_then(|v| v.to_str())
            .ok_or("spec name")?,
        1024 * 1024,
    )?;
    if digest(&b) != c.spec_sha256 {
        return Err("spec digest".into());
    }
    if now()? >= c.stop_utc_seconds {
        return Err("deadline exhausted".into());
    }
    Ok(())
}
fn publish(path: &Path, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(value).map_err(error)?;
    if bytes.len() > 256 * 1024 {
        return Err("receipt cap".into());
    }
    // Trusted same-UID instrumentation only: output directories are not a root-owned trust boundary.
    let temp = path.with_extension(format!("owner-tmp-{}", std::process::id()));
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(&temp)
        .map_err(error)?;
    f.write_all(&bytes)
        .and_then(|_| f.sync_all())
        .map_err(error)?;
    fs::hard_link(&temp, path).map_err(error)?;
    fs::remove_file(&temp).map_err(error)?;
    File::open(path.parent().ok_or("receipt parent")?)
        .and_then(|f| f.sync_all())
        .map_err(error)
}
static SIGNAL: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
extern "C" fn interrupted(signal: i32) {
    SIGNAL.store(signal, std::sync::atomic::Ordering::Relaxed);
}
// Capture once before validation; never recapture remaining time after a clock step.
struct StopBound {
    anchor: std::time::Instant,
    stop: std::time::Duration,
    remaining: std::time::Duration,
}
impl StopBound {
    fn remaining(
        stop: std::time::Duration,
        wall: std::time::Duration,
    ) -> Result<std::time::Duration, String> {
        stop.checked_sub(wall)
            .filter(|d| !d.is_zero())
            .ok_or_else(|| "owner stop expired".into())
    }
    fn capture(stop_seconds: u64) -> Result<Self, String> {
        let anchor = std::time::Instant::now();
        let wall = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(error)?;
        let stop = std::time::Duration::from_secs(stop_seconds);
        Ok(Self {
            anchor,
            stop,
            remaining: Self::remaining(stop, wall)?,
        })
    }
    fn reached_at(&self, wall: std::time::Duration, elapsed: std::time::Duration) -> bool {
        wall >= self.stop || elapsed >= self.remaining
    }
    fn reached(&self) -> bool {
        let elapsed = std::time::Instant::now().checked_duration_since(self.anchor);
        let wall = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
        match (wall, elapsed) {
            (Ok(wall), Some(elapsed)) => self.reached_at(wall, elapsed),
            _ => true,
        }
    }
}
#[allow(
    clippy::zombie_processes,
    reason = "direct child is explicitly reaped with wait4; unobserved reap is partial evidence"
)]
fn observe(c: &Config, config_sha: &str) -> Result<i32, String> {
    let stop = StopBound::capture(c.stop_utc_seconds)?;
    validate(c)?;
    let begin = Clock::read()?;
    let mut cmd = Command::new(&c.binary);
    cmd.args(&c.argv);
    unsafe {
        cmd.pre_exec(|| {
            if libc::setsid() < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
    if stop.reached() {
        return Err("owner stop expired before child spawn".into());
    }
    let child = cmd.spawn().map_err(error)?;
    let pid = i32::try_from(child.id()).map_err(error)?;
    let mut findings = Vec::<String>::new();
    let observed=(|| -> Result<Value,String> {
        let start=starttime(pid)?;
        let (sha,bytes)=sha_file(Path::new(&format!("/proc/{pid}/exe")))?;
        if sha!=c.binary_sha256 || starttime(pid)?!=start {return Err("postexec identity unavailable".into());}
        Ok(json!({"pid":pid,"starttime":start,"binary_sha256":sha,"binary_bytes":bytes,"clock":Clock::read()?}))
    })().map_err(|e|findings.push(e)).ok();
    if observed.is_none() && findings.is_empty() {
        findings.push("postexec identity unavailable".into());
    }
    let mut killed = None;
    let mut status = 0;
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    let mut exit = None;
    let mut cpu = None;
    let mut end = None;
    loop {
        let got = unsafe { libc::wait4(pid, &mut status, libc::WNOHANG, usage.as_mut_ptr()) };
        if got == pid {
            let u = unsafe { usage.assume_init() };
            if libc::WIFEXITED(status) {
                exit = Some(libc::WEXITSTATUS(status));
            } else if libc::WIFSIGNALED(status) {
                exit = Some(128 + libc::WTERMSIG(status));
            }
            let micros = |t: libc::timeval| -> Option<u64> {
                let s = u64::try_from(t.tv_sec).ok()?;
                let us = u64::try_from(t.tv_usec).ok()?;
                if us >= 1_000_000 {
                    return None;
                }
                s.checked_mul(1_000_000)?.checked_add(us)
            };
            cpu = Some(
                json!({"user_microseconds":micros(u.ru_utime),"system_microseconds":micros(u.ru_stime),"scope":"wait4 direct child; not allocated worker"}),
            );
            match Clock::read() {
                Ok(v) => end = Some(v),
                Err(e) => findings.push(e),
            }
            break;
        }
        if got < 0 {
            let e = std::io::Error::last_os_error();
            if e.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            findings.push(format!("wait4: {e}"));
            break;
        }
        let signal = SIGNAL.load(std::sync::atomic::Ordering::Relaxed);
        if killed.is_none() && (signal != 0 || stop.reached()) {
            // Deadline is a hard process-group stop; no new grace allowance is minted.
            unsafe {
                libc::kill(-pid, libc::SIGKILL);
            }
            killed = Some(std::time::Instant::now());
            findings.push(format!("hard stop requested; incoming signal {signal}"));
        }
        if killed.is_some_and(|t| t.elapsed() >= std::time::Duration::from_millis(100)) {
            findings.push(
                "reap not observed within bounded cleanup; allocation owner must retain charge"
                    .into(),
            );
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    if exit.is_none() {
        findings.push("terminal reap unobserved".into());
    }
    let quiet = unsafe { libc::kill(-pid, 0) } == -1
        && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH);
    if !quiet {
        findings.push("process group quiescence unobserved".into());
    }
    if !end.as_ref().is_some_and(|v| begin.same_epoch(v)) {
        findings.push("clock epoch unestablished".into());
    }
    let binding = if c.phase == "run" && findings.is_empty() && observed.is_some() {
        match bind_files(&c.session, end.as_ref().ok_or("missing end")?, begin.raw_ns) {
            Ok(v) => Some(v),
            Err(e) => {
                findings.push(e);
                None
            }
        }
    } else {
        None
    };
    let receipt = json!({"schema":"isolab.fresh-owner-observation/v1","config_sha256":config_sha,"binding":c,"payload_pid":pid,"postexec_runner_observation":observed,"owner_begin_clock":begin,"reap_observed_clock":end,"reap_observed":exit.is_some(),"exit_code":exit,"raw_wait_status":exit.map(|_|status),"cpu":cpu,"session_binding":binding,"findings":findings,"total_active_worker_ns":null,"exact_kernel_exit_ns":null,"accepted_external_closure":null,"trust_model":"trusted same-UID instrumentation; requires independent controller custody, not root-private","boundary":"direct-child observed reap; replay has no runner preparation closure","receipt_publication_tail_ns":null});
    if let Err(e) = publish(&c.receipt, &receipt) {
        eprintln!("receipt publication failed: {e}");
    }
    // Never convert a child failure to success; publication status is independently checked by consumer.
    Ok(exit.unwrap_or(125))
}
// Integrity/observation consumer only. No mathematical or allocated-worker acceptance.
fn check_receipt(c: &Config, config_sha: &str, raw: &[u8]) -> Result<Value, String> {
    if raw.len() > 262144 {
        return Err("receipt cap".into());
    }
    let Unique(v) = serde_json::from_slice(raw).map_err(error)?;
    if v["schema"] != "isolab.fresh-owner-observation/v1"
        || v["config_sha256"] != config_sha
        || v["binding"] != serde_json::to_value(c).map_err(error)?
    {
        return Err("receipt identity/config mismatch".into());
    }
    if v["reap_observed"] != true || v["exit_code"] != 0 || v["findings"] != json!([]) {
        return Err("owner observation incomplete or child failed".into());
    }
    if !v["total_active_worker_ns"].is_null()
        || !v["accepted_external_closure"].is_null()
        || !v["exact_kernel_exit_ns"].is_null()
    {
        return Err("unsupported claimed cost or closure".into());
    }
    if v["postexec_runner_observation"]["binary_sha256"] != c.binary_sha256
        || v["postexec_runner_observation"]["pid"] != v["payload_pid"]
    {
        return Err("postexec binding missing".into());
    }
    let start: Clock = serde_json::from_value(v["owner_begin_clock"].clone()).map_err(error)?;
    let end: Clock = serde_json::from_value(v["reap_observed_clock"].clone()).map_err(error)?;
    if !start.same_epoch(&end) {
        return Err("owner clock mismatch".into());
    }
    if c.phase == "run" {
        let b = &v["session_binding"];
        if !b.is_object() || b["estimand"] != "run_session_origin_to_owner_post_reap_evidence_sync"
        {
            return Err("session binding missing".into());
        }
        for (file, key, cap) in [
            ("session-preparation.json", "preparation_sha256", 262144),
            ("resource-journal.json", "journal_sha256", 8 * 1024 * 1024),
        ] {
            let (bytes, _, _) = read_at(&c.session, file, cap)?;
            if b[key] != digest(&bytes) {
                return Err("retained session hash mismatch".into());
            }
        }
    } else if !v["session_binding"].is_null() {
        return Err("replay cannot claim runner closure".into());
    }
    Ok(
        json!({"schema":"isolab.fresh-owner-check/v1","receipt_sha256":digest(raw),"config_sha256":config_sha,"phase":c.phase,"identity_and_observation_checked":true,"accepted_external_closure":null,"total_active_worker_ns":null,"scope":"trusted instrumentation observation only; full native journal and scientific acceptance remain separate"}),
    )
}
fn main() {
    let result = (|| -> Result<i32, String> {
        let args: Vec<_> = std::env::args().collect();
        let check = args.get(1).is_some_and(|v| v == "--check");
        let args = if check { &args[1..] } else { &args[..] };
        if args.len() != 3 || !hash_shape(&args[2]) {
            return Err("usage: isolab-fresh-owner CONFIG EXPECTED_CONFIG_SHA256".into());
        }
        let path = Path::new(&args[1]);
        let (bytes, _, _) = read_at(
            path.parent().ok_or("config parent")?,
            path.file_name()
                .and_then(|v| v.to_str())
                .ok_or("config name")?,
            65536,
        )?;
        if digest(&bytes) != args[2] {
            return Err("config digest mismatch".into());
        }
        let Unique(value) = serde_json::from_slice(&bytes).map_err(error)?;
        let c: Config = serde_json::from_value(value).map_err(error)?;
        if check {
            let (raw, _, _) = read_at(
                c.receipt.parent().ok_or("receipt parent")?,
                c.receipt
                    .file_name()
                    .and_then(|v| v.to_str())
                    .ok_or("receipt name")?,
                262144,
            )?;
            let checked = check_receipt(&c, &args[2], &raw)?;
            println!("{}", serde_json::to_string(&checked).map_err(error)?);
            return Ok(0);
        }
        unsafe {
            libc::signal(
                libc::SIGTERM,
                interrupted as *const () as libc::sighandler_t,
            );
            libc::signal(libc::SIGINT, interrupted as *const () as libc::sighandler_t);
        }
        observe(&c, &args[2])
    })();
    match result {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("fresh owner: {e}");
            std::process::exit(125)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_and_hash_guards() {
        assert!(serde_json::from_slice::<Unique>(br#"{"a":{"b":1,"b":2}}"#).is_err());
        assert!(hash_shape(&"a".repeat(64)));
        assert!(!hash_shape(&"G".repeat(64)));
    }
    #[test]
    fn immutable_publication() {
        let p = std::env::temp_dir().join(format!("fresh-owner-{}", std::process::id()));
        fs::create_dir(&p).unwrap();
        let path = p.join("receipt");
        publish(&path, &json!({"exit":7})).unwrap();
        assert!(publish(&path, &json!({"exit":0})).is_err());
        assert_eq!(
            serde_json::from_slice::<Value>(&fs::read(&path).unwrap()).unwrap()["exit"],
            7
        );
        fs::remove_dir_all(p).unwrap();
    }
    #[test]
    fn native_direct_children_preserve_success_failure_and_replay_scope() {
        let p = std::env::temp_dir().join(format!("fresh-owner-process-{}", std::process::id()));
        fs::create_dir(&p).unwrap();
        let spec = p.join("spec.json");
        fs::write(&spec, b"{}").unwrap();
        for (phase, binary, expected) in [("run", "/bin/false", 1), ("replay", "/bin/true", 0)] {
            let c = Config {
                schema: "isolab.fresh-owner-config/v1".into(),
                phase: phase.into(),
                request_sha256: "a".repeat(64),
                launch_sha256: digest(b"{}"),
                launch: spec.clone(),
                pod_uid: "fixture-pod".into(),
                job_uid: "fixture-job".into(),
                image_digest: "c".repeat(64),
                binary_sha256: sha_file(Path::new(binary)).unwrap().0,
                binary: binary.into(),
                argv: if phase == "run" {
                    vec![
                        "run".into(),
                        "--spec".into(),
                        spec.to_str().unwrap().into(),
                        "--out".into(),
                        p.to_str().unwrap().into(),
                        "--diagnostic-launch".into(),
                        spec.to_str().unwrap().into(),
                    ]
                } else {
                    vec![
                        "verify".into(),
                        "--dir".into(),
                        p.to_str().unwrap().into(),
                        "--replay-all".into(),
                    ]
                },
                spec: spec.clone(),
                spec_sha256: digest(b"{}"),
                session: p.clone(),
                receipt: p.join(format!("{phase}.json")),
                stop_utc_seconds: now().unwrap() + 10,
            };
            assert_eq!(observe(&c, &"d".repeat(64)).unwrap(), expected);
            let v: Value = serde_json::from_slice(&fs::read(&c.receipt).unwrap()).unwrap();
            assert_eq!(v["exit_code"], expected);
            assert_eq!(v["reap_observed"], true);
            assert!(v["total_active_worker_ns"].is_null());
            assert!(v["accepted_external_closure"].is_null());
            if phase == "replay" {
                assert!(v["session_binding"].is_null());
            }
            let mut expired = c;
            expired.stop_utc_seconds = now().unwrap();
            assert!(validate(&expired).unwrap_err().contains("deadline"));
        }
        fs::remove_dir_all(p).unwrap();
    }
    #[test]
    fn bounded_nofollow_reader_rejects_link_and_cap() {
        let p = std::env::temp_dir().join(format!("fresh-owner-read-{}", std::process::id()));
        fs::create_dir(&p).unwrap();
        fs::write(p.join("raw"), b"abc").unwrap();
        std::os::unix::fs::symlink(p.join("raw"), p.join("link")).unwrap();
        assert!(read_at(&p, "link", 100).is_err());
        assert!(read_at(&p, "raw", 2).is_err());
        assert_eq!(read_at(&p, "raw", 3).unwrap().0, b"abc");
        fs::remove_dir_all(p).unwrap();
    }
}

#[cfg(test)]
mod monotonic_stop_tests {
    use super::StopBound;
    use std::time::{Duration, Instant};
    #[test]
    fn immutable_bound_survives_wall_rollback_and_forward_step() {
        let b = StopBound {
            anchor: Instant::now(),
            stop: Duration::from_secs(100),
            remaining: Duration::from_millis(1500),
        };
        assert!(!b.reached_at(Duration::from_secs(98), Duration::from_millis(1499)));
        assert!(b.reached_at(Duration::from_secs(1), Duration::from_millis(1500)));
        assert!(b.reached_at(Duration::from_secs(100), Duration::ZERO));
        assert_eq!(b.remaining, Duration::from_millis(1500));
    }
    #[test]
    fn admission_preserves_fractional_remaining_and_rejects_expiry() {
        assert_eq!(
            StopBound::remaining(Duration::from_secs(100), Duration::from_millis(99875)).unwrap(),
            Duration::from_millis(125)
        );
        assert!(StopBound::remaining(Duration::from_secs(100), Duration::from_secs(100)).is_err());
        assert!(StopBound::remaining(Duration::from_secs(100), Duration::from_secs(101)).is_err());
    }
}
