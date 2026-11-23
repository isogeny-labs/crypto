// Private proposed helper, included as a supervisor submodule.
use super::*;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::OpenOptionsExt;

const CAP: usize = 256 * 1024;
#[derive(Clone, Serialize)]
pub(super) struct Clock {
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
pub(super) struct Owner {
    started: Result<Clock, String>,
    exec_observed: Option<Value>,
    published: bool,
    #[cfg(test)]
    test_output: Option<PathBuf>,
}
impl Owner {
    pub(super) fn begin() -> Self {
        Self {
            started: Clock::read(),
            exec_observed: None,
            published: false,
            #[cfg(test)]
            test_output: None,
        }
    }
    pub(super) fn observe_exec(&mut self, req: &Request, s: &Status) {
        if self.exec_observed.is_some() || s.attempt != 0 {
            return;
        }
        let Some(pid) = s.pid else {
            return;
        };
        let result = (|| -> Result<Value, String> {
            let before = starttime(pid)?;
            if Some(&before) != s.starttime.as_ref() {
                return Err("PID identity mismatch".into());
            }
            let (hash, bytes) = sha_file(Path::new(&format!("/proc/{pid}/exe")))?;
            if hash != req.build.ecbench_sha256 || starttime(pid)? != before {
                return Err("exec not confirmed".into());
            }
            let clock = Clock::read()?;
            if !self
                .started
                .as_ref()
                .map_err(Clone::clone)?
                .same_epoch(&clock)
            {
                return Err("clock epoch changed".into());
            }
            Ok(
                json!({"pid":pid,"starttime":before,"binary_sha256":hash,"binary_bytes":bytes,"clock":clock}),
            )
        })();
        if let Ok(observation) = result {
            self.exec_observed = Some(observation);
        }
    }
    pub(super) fn poll(
        &mut self,
        req: &Request,
        s: &Status,
        terminal_reason: Option<&str>,
    ) -> Result<Option<i32>, String> {
        if self.published {
            return Ok(None);
        }
        self.observe_exec(req, s);
        let Some(pid) = s.pid else {
            if terminal_reason.is_some() {
                self.publish(req, s, None, None, "no payload PID", None)?;
            }
            return Ok(None);
        };
        let mut status = 0;
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        let got = loop {
            // SAFETY: wait4 writes both outputs only when returning this PID.
            let got = unsafe { libc::wait4(pid, &mut status, libc::WNOHANG, usage.as_mut_ptr()) };
            if got == -1
                && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
            {
                continue;
            }
            break got;
        };
        if got != pid {
            if terminal_reason.is_some() {
                self.publish(
                    req,
                    s,
                    None,
                    None,
                    "terminal request; successful reap not observed",
                    None,
                )?;
            }
            return Ok(None);
        }
        let clock = Clock::read();
        let usage = unsafe { usage.assume_init() };
        let micros = |t: libc::timeval| -> Option<u64> {
            let secs = u64::try_from(t.tv_sec).ok()?;
            let us = u64::try_from(t.tv_usec).ok()?;
            if us >= 1_000_000 {
                return None;
            }
            secs.checked_mul(1_000_000)?.checked_add(us)
        };
        let cpu = json!({"user_microseconds":micros(usage.ru_utime),"system_microseconds":micros(usage.ru_stime),"scope":"wait4 direct-child OS accounting; not allocated worker CPU"});
        let code = if libc::WIFEXITED(status) {
            libc::WEXITSTATUS(status)
        } else if libc::WIFSIGNALED(status) {
            128 + libc::WTERMSIG(status)
        } else {
            return Err("wait4 returned nonterminal state".into());
        };
        if let Err(error) = self.publish(
            req,
            s,
            Some(code),
            Some(cpu),
            terminal_reason.unwrap_or("observed terminal reap"),
            Some(clock),
        ) {
            eprintln!("owner receipt publication failed after successful reap: {error}");
        }
        Ok(Some(code))
    }
    fn publish(
        &mut self,
        req: &Request,
        s: &Status,
        code: Option<i32>,
        cpu: Option<Value>,
        reason: &str,
        clock: Option<Result<Clock, String>>,
    ) -> Result<(), String> {
        let mut findings = Vec::<String>::new();
        if s.attempt != 0 {
            findings.push("restored attempt clock alignment unsupported".into());
        }
        if self.exec_observed.is_none() {
            findings.push("post-exec runner identity not observed".into());
        }
        if code.is_none() {
            findings.push("terminal reap not observed".into());
        }
        let ended = clock.and_then(|v| match v {
            Ok(v) => Some(v),
            Err(e) => {
                findings.push(e);
                None
            }
        });
        if !matches!((&self.started,&ended),(Ok(a),Some(b)) if a.same_epoch(b)) {
            findings.push("clock epoch not established".into());
        }
        let group_quiet = s.pid.is_some_and(|pid| {
            let rc = unsafe { libc::kill(-pid, 0) };
            rc == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
        });
        if !group_quiet {
            findings.push("payload process group quiescence unobserved".into());
        }
        let binding = if findings.is_empty() {
            match bind_files(
                ended.as_ref().ok_or("end clock missing")?,
                self.started.as_ref().map_err(Clone::clone)?.raw_ns,
            ) {
                Ok(v) => Some(v),
                Err(e) => {
                    findings.push(e);
                    None
                }
            }
        } else {
            None
        };
        let receipt = json!({"schema":"isolab.recovery.owner-exit/v1","request_sha256":req.hash(),"attempt":s.attempt,
            "payload_pid":s.pid,"payload_starttime":s.starttime,"terminal_reason":reason,"exit_code":code,"reap_observed":code.is_some(),
            "postexec_runner_observation":self.exec_observed,"owner_begin_clock":self.started.as_ref().ok(),"reap_observed_clock":ended,
            "cpu":cpu,"session_binding":binding,"findings":findings,"accepted_external_closure":null,
            "total_active_worker_ns":null,"boundary":"observed direct-child reap upper boundary; exact kernel exit instant unknown",
            "journal_durability_boundary":"post-reap evidence sync when session_binding exists; no pre-exit durability or ExternalExit fabricated"});
        let bytes = serde_json::to_vec_pretty(&receipt).map_err(error)?;
        if bytes.len() > CAP {
            return Err("owner receipt cap".into());
        }
        let output_dir = attempt_dir(s.attempt);
        #[cfg(test)]
        let output_dir = self.test_output.clone().unwrap_or(output_dir);
        let path = output_dir.join("owner-exit.json");
        let tmp = path.with_extension("publication-tmp");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&tmp)
            .map_err(error)?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(error)?;
        fs::hard_link(&tmp, &path).map_err(error)?;
        self.published = true;
        fs::remove_file(tmp).map_err(error)?;
        File::open(output_dir)
            .and_then(|v| v.sync_all())
            .map_err(error)
    }
}
// Traverse every fixed component without following symlinks. Cap before parsing.
fn read_session(name: &str, cap: usize) -> Result<(Vec<u8>, File, File), String> {
    read_session_at(Path::new("/"), &["work", "compute", "session"], name, cap)
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
fn bind_files(clock: &Clock, owner_begin_raw_ns: u64) -> Result<Value, String> {
    let (prep, prep_file, prep_dir) = read_session("session-preparation.json", 256 * 1024)?;
    let (journal, journal_file, journal_dir) =
        read_session("resource-journal.json", 8 * 1024 * 1024)?;
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
    let ph = crate::contract::digest(&prep);
    let jh = crate::contract::digest(&journal);
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_metadata_cannot_establish_a_binding() {
        assert!(serde_json::from_slice::<Unique>(br#"{"origin_ns":1,"origin_ns":2}"#).is_err());
        assert!(serde_json::from_slice::<Unique>(br#"{"nested":{"x":1,"x":2}}"#).is_err());
        let Unique(v) = serde_json::from_slice(br#"{"origin_ns":18446744073709551615}"#).unwrap();
        assert_eq!(v["origin_ns"].as_u64(), Some(u64::MAX));
    }
    #[test]
    #[allow(
        clippy::zombie_processes,
        reason = "Owner::poll explicitly reaps this child using wait4; failure cleanup uses waitpid"
    )]
    fn real_fake_exec_reap_and_checkpoint_race_share_owner() {
        use std::os::unix::process::CommandExt;
        for checkpoint_path in [false, true] {
            let root = std::env::temp_dir().join(format!(
                "owner-observation-fixture-{}-{}",
                std::process::id(),
                checkpoint_path
            ));
            fs::create_dir(&root).unwrap();
            let mut req = crate::contract::fixture();
            req.build.ecbench_sha256 = sha_file(Path::new("/bin/sleep")).unwrap().0;
            let mut owner = Owner::begin();
            owner.test_output = Some(root.clone());
            let mut command = Command::new("/bin/sh");
            command.args(["-c", "exec /bin/sleep 0.05"]);
            unsafe {
                command.pre_exec(|| {
                    if libc::setsid() < 0 {
                        Err(std::io::Error::last_os_error())
                    } else {
                        Ok(())
                    }
                });
            }
            let child = command.spawn().unwrap();
            let pid = child.id() as i32;
            let mut state = Status {
                request_sha256: req.hash(),
                attempt: 0,
                phase: "running".into(),
                pid: Some(pid),
                starttime: Some(starttime(pid).unwrap()),
                generation: None,
                exit_code: None,
                error: None,
                updated: 0,
            };
            let mut done = false;
            for _ in 0..500 {
                if checkpoint_path {
                    if super::super::checkpoint_completion(&req, &mut state, pid, &mut owner)
                        .unwrap()
                    {
                        done = true;
                        break;
                    }
                } else if let Some(code) = owner.poll(&req, &state, None).unwrap() {
                    assert_eq!(code, 0);
                    done = true;
                    break;
                }
                thread::sleep(Duration::from_millis(1));
            }
            if !done {
                unsafe {
                    libc::kill(-pid, libc::SIGKILL);
                    libc::waitpid(pid, std::ptr::null_mut(), 0);
                }
            }
            assert!(done, "bounded fake process did not terminate");
            let value: Value = read(&root.join("owner-exit.json")).unwrap();
            assert_eq!(value["reap_observed"], true);
            assert_eq!(value["postexec_runner_observation"]["pid"], pid);
            assert_eq!(
                value["postexec_runner_observation"]["binary_sha256"],
                req.build.ecbench_sha256
            );
            assert!(value["accepted_external_closure"].is_null());
            fs::remove_dir_all(root).unwrap();
        }
    }
    #[test]
    fn explicit_missing_reap_stays_partial_and_does_not_retry_payload() {
        let root =
            std::env::temp_dir().join(format!("owner-no-pid-fixture-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let req = crate::contract::fixture();
        let mut owner = Owner::begin();
        owner.test_output = Some(root.clone());
        let state = Status {
            request_sha256: req.hash(),
            attempt: 0,
            phase: "failed".into(),
            pid: None,
            starttime: None,
            generation: None,
            exit_code: None,
            error: Some("deadline".into()),
            updated: 0,
        };
        assert!(owner
            .poll(&req, &state, Some("deadline"))
            .unwrap()
            .is_none());
        let value: Value = read(&root.join("owner-exit.json")).unwrap();
        assert_eq!(value["reap_observed"], false);
        assert!(value["session_binding"].is_null());
        assert!(value["accepted_external_closure"].is_null());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn session_reader_rejects_symlink_components_leaves_and_oversize() {
        use std::os::unix::fs::symlink;
        let root =
            std::env::temp_dir().join(format!("owner-reader-fixture-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("session")).unwrap();
        fs::write(root.join("session/good"), b"{}").unwrap();
        assert_eq!(
            read_session_at(&root, &["session"], "good", 2).unwrap().0,
            b"{}"
        );
        assert!(read_session_at(&root, &["session"], "good", 1).is_err());
        assert!(read_session_at(&root, &["session"], "missing", 2).is_err());
        symlink("good", root.join("session/link")).unwrap();
        assert!(read_session_at(&root, &["session"], "link", 2).is_err());
        symlink("session", root.join("alias")).unwrap();
        assert!(read_session_at(&root, &["alias"], "good", 2).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    #[allow(
        clippy::zombie_processes,
        reason = "Owner::poll uses wait4 and the missing-reap branch explicitly waitpids the killed child"
    )]
    fn terminal_kill_observation_never_invents_reap_or_closure() {
        use std::os::unix::process::CommandExt;
        let root = std::env::temp_dir().join(format!("owner-kill-fixture-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let req = crate::contract::fixture();
        let mut owner = Owner::begin();
        owner.test_output = Some(root.clone());
        let mut command = Command::new("/bin/sleep");
        command.arg("10");
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() < 0 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(())
                }
            });
        }
        let child = command.spawn().unwrap();
        let pid = child.id() as i32;
        let state = Status {
            request_sha256: req.hash(),
            attempt: 0,
            phase: "running".into(),
            pid: Some(pid),
            starttime: Some(starttime(pid).unwrap()),
            generation: None,
            exit_code: None,
            error: None,
            updated: 0,
        };
        assert_eq!(unsafe { libc::kill(-pid, libc::SIGKILL) }, 0);
        let observation = owner
            .poll(&req, &state, Some("synthetic terminal deadline"))
            .unwrap();
        if observation.is_none() {
            unsafe {
                libc::waitpid(pid, std::ptr::null_mut(), 0);
            }
        }
        let value: Value = read(&root.join("owner-exit.json")).unwrap();
        assert_eq!(value["reap_observed"], observation.is_some());
        assert!(value["accepted_external_closure"].is_null());
        assert!(value["total_active_worker_ns"].is_null());
        if observation.is_none() {
            assert!(value["session_binding"].is_null());
        }
        fs::remove_dir_all(root).unwrap();
    }
}
