//! Trusted PID 1 for one computational attempt. It has no Kubernetes/AWS
//! credentials. CRIU operations are serialized and their receipts live on EBS.
use crate::{contract::Request, io::*};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    os::unix::{fs::PermissionsExt, process::CommandExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

const ROOT: &str = "/work";
const BIN: &str = "/opt/isolab/recovery";
const ECBENCH: &str = "/usr/local/bin/ecbench";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub request_sha256: String,
    pub attempt: u32,
    pub phase: String,
    pub pid: Option<i32>,
    pub starttime: Option<String>,
    pub generation: Option<u64>,
    pub exit_code: Option<i32>,
    pub error: Option<String>,
    pub updated: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub request_sha256: String,
    pub attempt: u32,
    pub action: String,
    pub generation: u64,
    pub manifest_sha256: Option<String>,
}
impl Operation {
    pub fn id(&self) -> Result<String, String> {
        if ![
            "start",
            "checkpoint",
            "continue",
            "restore",
            "verify",
            "stop",
        ]
        .contains(&self.action.as_str())
        {
            return Err("unknown supervisor action".into());
        }
        Ok(format!("{}-{:020}", self.action, self.generation))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Runtime {
    pub kernel: String,
    pub cpu_identity: Vec<String>,
    pub ecbench_sha256: String,
    pub tools: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: String,
    pub request_sha256: String,
    pub generation: u64,
    pub runtime: Runtime,
    pub root_pid: i32,
    pub pids: Vec<i32>,
    pub files: BTreeMap<String, (String, u64)>,
}
fn attempt_dir(attempt: u32) -> PathBuf {
    Path::new(ROOT).join(format!("control/attempt-{attempt}"))
}
fn status_path(attempt: u32) -> PathBuf {
    attempt_dir(attempt).join("status.json")
}
fn payload_status(pid: i32) -> Result<String, String> {
    fs::read_to_string(format!("/proc/{pid}/status")).map_err(error)
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
fn credentials(pid: i32) -> Result<(), String> {
    let s = payload_status(pid)?;
    credential_text(&s)?;
    for task in fs::read_dir(format!("/proc/{pid}/task")).map_err(error)? {
        let path = task.map_err(error)?.path().join("status");
        credential_text(&fs::read_to_string(path).map_err(error)?)?;
    }
    Ok(())
}
fn credential_text(s: &str) -> Result<(), String> {
    for (key, value) in [
        ("Uid:", "1000\t1000\t1000\t1000"),
        ("Gid:", "1000\t1000\t1000\t1000"),
        ("Groups:", ""),
        ("CapInh:", "0000000000000000"),
        ("CapPrm:", "0000000000000000"),
        ("CapEff:", "0000000000000000"),
        ("CapBnd:", "0000000000000000"),
        ("CapAmb:", "0000000000000000"),
        ("NoNewPrivs:", "1"),
        ("Seccomp:", "2"),
    ] {
        if s.lines().find_map(|l| l.strip_prefix(key)).map(str::trim) != Some(value) {
            return Err(format!("payload credential mismatch: {key}"));
        }
    }
    Ok(())
}
fn tree(pid: i32) -> Result<Vec<i32>, String> {
    let mut out = vec![pid];
    let mut index = 0;
    while index < out.len() {
        let p = out[index];
        for task in fs::read_dir(format!("/proc/{p}/task")).map_err(error)? {
            let children =
                fs::read_to_string(task.map_err(error)?.path().join("children")).map_err(error)?;
            for c in children.split_whitespace() {
                let c: i32 = c.parse().map_err(error)?;
                if !out.contains(&c) {
                    out.push(c);
                }
            }
        }
        index += 1;
    }
    Ok(out)
}
fn runtime() -> Result<Runtime, String> {
    let cpu = fs::read_to_string("/proc/cpuinfo").map_err(error)?;
    let first = cpu.split("\n\n").next().unwrap_or("");
    let identity = first
        .lines()
        .filter(|s| {
            ["vendor_id", "cpu family", "model\t", "stepping", "flags"]
                .iter()
                .any(|k| s.starts_with(k))
        })
        .map(str::to_owned)
        .collect();
    let mut tools = BTreeMap::new();
    for name in ["criu", "isolab-criu-progress", "isolab-eks-recovery"] {
        tools.insert(name.into(), sha_file(&Path::new(BIN).join(name))?.0);
    }
    Ok(Runtime {
        kernel: fs::read_to_string("/proc/sys/kernel/osrelease")
            .map_err(error)?
            .trim()
            .into(),
        cpu_identity: identity,
        ecbench_sha256: sha_file(Path::new(ECBENCH))?.0,
        tools,
    })
}
fn criu() -> Command {
    let mut c = Command::new(format!("{BIN}/criu"));
    c.env("LD_LIBRARY_PATH", format!("{BIN}/lib"));
    c
}
fn common_criu(c: &mut Command, dir: &Path) {
    c.arg("--images-dir").arg(dir).args([
        "--shell-job",
        "--manage-cgroups=ignore",
        "--cpu-cap=all",
        "--file-locks",
        "--leave-stopped",
        "-v4",
    ]);
}
fn files(path: &Path, out: &mut BTreeMap<String, (String, u64)>) -> Result<(), String> {
    for e in fs::read_dir(path).map_err(error)? {
        let e = e.map_err(error)?;
        let t = e.file_type().map_err(error)?;
        let p = e.path();
        if t.is_dir() {
            files(&p, out)?;
        } else if t.is_file() {
            let key = p
                .strip_prefix(ROOT)
                .map_err(error)?
                .to_str()
                .ok_or("nonUTF8 checkpoint path")?
                .to_string();
            out.insert(key, sha_file(&p)?);
        } else {
            return Err(format!("unsupported mutable file type: {}", p.display()));
        }
    }
    Ok(())
}
fn sync_work() -> Result<(), String> {
    let f = File::open(ROOT).map_err(error)?;
    use std::os::fd::AsRawFd;
    if unsafe { libc::syncfs(f.as_raw_fd()) } != 0 {
        return Err(error(std::io::Error::last_os_error()));
    }
    Ok(())
}
fn generation_dir(n: u64) -> PathBuf {
    Path::new(ROOT).join(format!("generations/g{n}"))
}
fn live(s: &Status) -> Result<i32, String> {
    let pid = s.pid.ok_or("payload PID missing")?;
    if Some(starttime(pid)?) != s.starttime {
        return Err("payload PID reused".into());
    }
    credentials(pid)?;
    Ok(pid)
}
fn continue_tree(s: &Status) -> Result<(), String> {
    let pid = live(s)?;
    for p in tree(pid)?.iter().rev() {
        credentials(*p)?;
        if unsafe { libc::kill(*p, libc::SIGCONT) } != 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
    }
    Ok(())
}
fn launch(req: &Request, s: &mut Status) -> Result<(), String> {
    if s.phase != "waiting" || s.attempt != 0 {
        return Err("fresh launch is permitted once on attempt zero".into());
    }
    if Path::new("/work/compute").exists() {
        return Err("computation already exists; never restart from zero".into());
    }
    fs::create_dir("/work/compute").map_err(error)?;
    fs::create_dir("/work/compute/tmp").map_err(error)?;
    save(Path::new("/work/compute/original-spec.json"), &req.spec)?;
    // Keep payload PIDs away from CRIU and kubectl exec's short-lived helpers on a
    // fresh restore namespace. No restore is allowed if any saved PID is occupied.
    for _ in 0..512 {
        let pid = unsafe { libc::fork() };
        if pid < 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
        if pid == 0 {
            unsafe { libc::_exit(0) }
        }
        unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
    }
    for p in [
        "/work/compute",
        "/work/compute/tmp",
        "/work/compute/original-spec.json",
    ] {
        let p = std::ffi::CString::new(p).unwrap();
        if unsafe { libc::chown(p.as_ptr(), 1000, 1000) } != 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
    }
    s.phase = "starting".into();
    save(&status_path(s.attempt), s)?;
    let stdout = File::create("/work/compute/run.stdout").map_err(error)?;
    let stderr = File::create("/work/compute/run.stderr").map_err(error)?;
    let mut cmd = Command::new(format!("{BIN}/isolab-criu-progress"));
    cmd.args([
        ECBENCH,
        "/work/compute/original-spec.json",
        "/work/compute/session",
    ])
    .current_dir("/work/compute")
    .env_clear()
    .env("PATH", "/usr/local/bin:/usr/bin:/bin")
    .env("HOME", "/work/compute")
    .env("TMPDIR", "/work/compute/tmp")
    .stdin(Stdio::null())
    .stdout(stdout)
    .stderr(stderr);
    drop_payload_privileges(&mut cmd);
    let child = cmd.spawn().map_err(error)?;
    s.pid = Some(child.id() as i32);
    s.starttime = Some(starttime(child.id() as i32)?);
    s.phase = "running".into();
    Ok(())
}
fn drop_payload_privileges(cmd: &mut Command) {
    // SAFETY: pre_exec only invokes async-signal-safe credential/session syscalls.
    unsafe {
        cmd.pre_exec(|| {
            for cap in 0..=40 {
                if libc::prctl(libc::PR_CAPBSET_DROP, cap, 0, 0, 0) != 0
                    && std::io::Error::last_os_error().raw_os_error() != Some(libc::EINVAL)
                {
                    return Err(std::io::Error::last_os_error());
                }
            }
            if libc::setgroups(0, std::ptr::null()) != 0
                || libc::setgid(1000) != 0
                || libc::setuid(1000) != 0
                || libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0
                || libc::setsid() < 0
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}
fn checkpoint(req: &Request, s: &mut Status, n: u64) -> Result<(), String> {
    // Completion can race the controller's last status read. Preserve the
    // successful computation instead of treating this as a failed checkpoint.
    if s.phase == "completed" && s.exit_code == Some(0) {
        return Ok(());
    }
    if s.phase != "running" {
        return Err("checkpoint requires running computation".into());
    }
    let pid = live(s)?;
    let dir = generation_dir(n);
    fs::create_dir_all(&dir).map_err(error)?;
    fs::set_permissions(
        Path::new(ROOT).join("generations"),
        fs::Permissions::from_mode(0o700),
    )
    .map_err(error)?;
    let dump = dir.join("dump");
    fs::create_dir(&dump).map_err(error)?;
    s.phase = "checkpointing".into();
    s.generation = Some(n);
    save(&status_path(s.attempt), s)?;
    let mut cmd = criu();
    cmd.args(["dump", "--tree", &pid.to_string()]);
    common_criu(&mut cmd, &dump);
    cmd.args(["--log-file", "dump.log"]);
    let result = cmd.output().map_err(error)?;
    save(
        &dir.join("dump-result.json"),
        &json!({"exit_code":result.status.code(),"stderr":String::from_utf8_lossy(&result.stderr)}),
    )?;
    if !result.status.success() {
        return Err("CRIU dump failed; retained log, no generation committed".into());
    }
    let pids = tree(pid)?;
    for p in &pids {
        credentials(*p)?;
        let st = payload_status(*p)?;
        if !st
            .lines()
            .any(|l| l.starts_with("State:") && l.contains("T (stopped)"))
        {
            return Err("dump did not leave complete tree stopped".into());
        }
        fs::write(dir.join(format!("process-{p}-status.txt")), st).map_err(error)?;
    }
    sync_work()?;
    let mut entries = BTreeMap::new();
    files(Path::new("/work/compute"), &mut entries)?;
    files(&dump, &mut entries)?;
    let manifest = Manifest {
        schema: "isolab.criu.generation/v1".into(),
        request_sha256: req.hash(),
        generation: n,
        runtime: runtime()?,
        root_pid: pid,
        pids,
        files: entries,
    };
    save(&dir.join("manifest.json"), &manifest)?;
    sync_work()?;
    s.phase = "stopped".into();
    Ok(())
}
fn restore(req: &Request, s: &mut Status, n: u64, expected: Option<&str>) -> Result<(), String> {
    if s.phase != "waiting" || s.attempt == 0 {
        return Err("restore requires a fresh replacement attempt".into());
    }
    let dir = generation_dir(n);
    if expected != Some(sha_file(&dir.join("manifest.json"))?.0.as_str()) {
        return Err("snapshot manifest differs from committed generation".into());
    }
    let m: Manifest = read(&dir.join("manifest.json"))?;
    if m.schema != "isolab.criu.generation/v1"
        || m.request_sha256 != req.hash()
        || m.generation != n
        || m.runtime != runtime()?
        || m.runtime.ecbench_sha256 != req.build.ecbench_sha256
    {
        return Err("checkpoint request/runtime identity mismatch".into());
    }
    for (file, expected) in &m.files {
        let p = Path::new(file);
        if p.is_absolute()
            || p.components()
                .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            return Err("unsafe checkpoint file path".into());
        }
        let full = Path::new(ROOT).join(p);
        if fs::symlink_metadata(&full)
            .map_err(error)?
            .file_type()
            .is_symlink()
            || sha_file(&full)? != *expected
        {
            return Err(format!("checkpoint file hash mismatch: {file}"));
        }
    }
    for pid in &m.pids {
        if *pid <= 1 || Path::new(&format!("/proc/{pid}")).exists() {
            return Err(format!(
                "saved PID {pid} is already occupied; restore refused"
            ));
        }
    }
    s.phase = "restoring".into();
    s.generation = Some(n);
    save(&status_path(s.attempt), s)?;
    let attempt = attempt_dir(s.attempt);
    let mut cmd = criu();
    cmd.arg("restore");
    common_criu(&mut cmd, &dir.join("dump"));
    cmd.args(["--restore-detached", "--pidfile"])
        .arg(attempt.join("restored.pid"))
        .arg("--log-file")
        .arg(attempt.join("restore.log"));
    // CRIU 4.2.1 inherits its caller's supplementary groups when restoring empty
    // groups. Clear them BEFORE restore, then validate every stopped process.
    unsafe {
        cmd.pre_exec(|| {
            if libc::setgroups(0, std::ptr::null()) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let result = cmd.output().map_err(error)?;
    save(
        &attempt.join("restore-result.json"),
        &json!({"exit_code":result.status.code(),"stderr":String::from_utf8_lossy(&result.stderr)}),
    )?;
    if !result.status.success() {
        return Err("CRIU restore failed; no fresh command fallback".into());
    }
    for pid in &m.pids {
        credentials(*pid)?;
        let st = payload_status(*pid)?;
        fs::write(attempt.join(format!("restored-{pid}-status.txt")), &st).map_err(error)?;
        if !st
            .lines()
            .any(|l| l.starts_with("State:") && l.contains("T (stopped)"))
        {
            return Err("restored process was not stopped for credential admission".into());
        }
    }
    s.pid = Some(m.root_pid);
    s.starttime = Some(starttime(m.root_pid)?);
    s.phase = "stopped".into();
    Ok(())
}
fn verify(s: &mut Status) -> Result<(), String> {
    if s.phase != "completed" || s.exit_code != Some(0) {
        return Err("successful computation required for native replay".into());
    }
    let dir = attempt_dir(s.attempt);
    let mut command = Command::new(ECBENCH);
    command
        .args([
            "verify",
            "--dir",
            "/work/compute/session",
            "--replay-all",
            "--exit-code",
            "--out",
            "/work/compute/audit.json",
        ])
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .env("HOME", "/work/compute")
        .env("TMPDIR", "/work/compute/tmp")
        .current_dir("/work/compute")
        .stdin(Stdio::null());
    // Replay executes the tested binary too; it must not inherit the trusted
    // supervisor's root identity or capabilities.
    drop_payload_privileges(&mut command);
    let result = command.output().map_err(error)?;
    fs::write(dir.join("audit.stdout"), &result.stdout).map_err(error)?;
    fs::write(dir.join("audit.stderr"), &result.stderr).map_err(error)?;
    if !result.status.success() {
        return Err("native computation audit failed".into());
    }
    let v: Value = serde_json::from_slice(&fs::read("/work/compute/audit.json").map_err(error)?)
        .map_err(error)?;
    if v["ok"] != true
        || v["records_bound"] != true
        || v["regraded"] != true
        || v["records"].as_u64().unwrap_or(0) == 0
        || v["verified_records"] != v["records"]
        || !v["replays"]
            .as_array()
            .is_some_and(|r| !r.is_empty() && r.iter().all(|x| x["reproduced"] == true))
    {
        return Err("native computation replay not admitted".into());
    }
    save(&dir.join("audit.json"), &v)?;
    s.phase = "verified".into();
    Ok(())
}
pub fn rpc(attempt: u32, bytes: &[u8]) -> Result<(), String> {
    let op: Operation = serde_json::from_slice(bytes).map_err(error)?;
    if op.attempt != attempt {
        return Err("operation attempt mismatch".into());
    }
    let id = op.id()?;
    let path = attempt_dir(attempt).join(format!("requests/{id}.json"));
    if path.exists() {
        let old: Operation = read(&path)?;
        if serde_json::to_vec(&old).map_err(error)? != serde_json::to_vec(&op).map_err(error)? {
            return Err("operation identity already used".into());
        }
        return Ok(());
    }
    save(&path, &op)
}
pub fn status(attempt: u32) -> Result<Status, String> {
    read(&status_path(attempt))
}
pub fn supervise(req: Request, attempt: u32) -> Result<(), String> {
    if unsafe { libc::getpid() } != 1 || unsafe { libc::geteuid() } != 0 {
        return Err("trusted supervisor must be root PID 1 in its own Pod namespace".into());
    }
    if runtime()?.ecbench_sha256 != req.build.ecbench_sha256 {
        return Err("ecbench binary hash does not match request".into());
    }
    let dir = attempt_dir(attempt);
    if dir.exists() {
        return Err("attempt already exists: supervisor may not restart an attempt".into());
    }
    fs::create_dir_all(dir.join("requests")).map_err(error)?;
    fs::create_dir(dir.join("done")).map_err(error)?;
    fs::set_permissions(
        Path::new(ROOT).join("control"),
        fs::Permissions::from_mode(0o700),
    )
    .map_err(error)?;
    let mut s = Status {
        request_sha256: req.hash(),
        attempt,
        phase: "waiting".into(),
        pid: None,
        starttime: None,
        generation: None,
        exit_code: None,
        error: None,
        updated: now(),
    };
    save(&status_path(attempt), &s)?;
    loop {
        if now() >= req.deadline_unix_seconds {
            if let Some(pid) = s.pid {
                unsafe { libc::kill(-pid, libc::SIGKILL) };
            }
            s.phase = "failed".into();
            s.error = Some("original UTC deadline expired".into());
            save(&status_path(attempt), &s)?;
            return Err("original UTC deadline expired".into());
        }
        // Reap the native root (including roots reparented to this PID 1 by CRIU).
        if let Some(pid) = s.pid {
            let mut status = 0;
            let got = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
            if got == pid {
                let code = if libc::WIFEXITED(status) {
                    libc::WEXITSTATUS(status)
                } else {
                    128 + libc::WTERMSIG(status)
                };
                s.exit_code = Some(code);
                s.phase = if code == 0 { "completed" } else { "failed" }.into();
            }
        }
        let mut operations: Vec<_> = fs::read_dir(dir.join("requests"))
            .map_err(error)?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        operations.sort();
        for path in operations {
            let done = dir
                .join("done")
                .join(path.file_name().ok_or("operation path")?);
            if done.exists() {
                continue;
            }
            let op: Operation = read(&path)?;
            let result = if op.request_sha256 != req.hash() || op.attempt != attempt {
                Err("operation request mismatch".into())
            } else {
                match op.action.as_str() {
                    "start" => launch(&req, &mut s),
                    "checkpoint" => checkpoint(&req, &mut s, op.generation),
                    "continue" => {
                        if s.phase == "stopped" && s.generation == Some(op.generation) {
                            continue_tree(&s).map(|()| s.phase = "running".into())
                        } else {
                            Err("continue does not match stopped generation".into())
                        }
                    }
                    "restore" => {
                        restore(&req, &mut s, op.generation, op.manifest_sha256.as_deref())
                    }
                    "verify" => verify(&mut s),
                    "stop" => {
                        if let Some(pid) = s.pid {
                            unsafe { libc::kill(-pid, libc::SIGKILL) };
                        }
                        s.phase = "failed".into();
                        Err("controller stopped computation".into())
                    }
                    _ => Err("unsupported action".into()),
                }
            };
            if let Err(e) = &result {
                s.error = Some(e.clone());
                s.phase = "failed".into();
                if let Some(pid) = s.pid {
                    unsafe { libc::kill(-pid, libc::SIGKILL) };
                }
            }
            save(
                &done,
                &json!({"operation":op,"ok":result.is_ok(),"error":result.err(),"at":now()}),
            )?;
        }
        s.updated = now();
        save(&status_path(attempt), &s)?;
        thread::sleep(Duration::from_millis(250));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const GOOD:&str="Uid:\t1000\t1000\t1000\t1000\nGid:\t1000\t1000\t1000\t1000\nGroups:\t\nCapInh:\t0000000000000000\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000000\nCapBnd:\t0000000000000000\nCapAmb:\t0000000000000000\nNoNewPrivs:\t1\nSeccomp:\t2\n";
    #[test]
    fn restored_group_zero_is_rejected_before_continue() {
        assert!(credential_text(GOOD).is_ok());
        assert!(credential_text(&GOOD.replace("Groups:\t\n", "Groups:\t0\n")).is_err());
    }
    #[test]
    fn changed_ids_capabilities_and_missing_nnp_are_rejected() {
        for (old, new) in [
            ("1000", "0"),
            ("CapEff:\t0000000000000000", "CapEff:\t0000000000000001"),
            ("NoNewPrivs:\t1", "NoNewPrivs:\t0"),
            ("Seccomp:\t2", "Seccomp:\t0"),
        ] {
            assert!(credential_text(&GOOD.replace(old, new)).is_err());
        }
    }
    #[test]
    fn completed_computation_wins_race_with_checkpoint_request() {
        let req = crate::contract::fixture();
        let mut s = Status {
            request_sha256: req.hash(),
            attempt: 0,
            phase: "completed".into(),
            pid: None,
            starttime: None,
            generation: None,
            exit_code: Some(0),
            error: None,
            updated: 0,
        };
        checkpoint(&req, &mut s, 1).unwrap();
        assert_eq!(s.phase, "completed");
        assert!(s.generation.is_none());
    }
}
