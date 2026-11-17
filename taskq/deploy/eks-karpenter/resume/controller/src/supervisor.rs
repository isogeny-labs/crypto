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

#[path = "deadline.rs"]
mod deadline;
#[path = "owner_observation.rs"]
mod owner_observation;

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
            "seal",
            "prune",
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
    /// ecbench, or argv[0] in command mode (both bound by build.ecbench_sha256).
    pub ecbench_sha256: String,
    pub tools: BTreeMap<String, String>,
    /// Command mode `binaries`; omitted for ecbench, preserving manifest bytes.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub payload_binaries: BTreeMap<String, String>,
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
/// Executable the request binds with build.ecbench_sha256.
fn payload_executable(req: &Request) -> &str {
    req.command.as_ref().map_or(ECBENCH, |c| c.argv[0].as_str())
}
fn payload_hashes(req: &Request) -> Result<(String, BTreeMap<String, String>), String> {
    let main = sha_file(Path::new(payload_executable(req)))?.0;
    let mut binaries = BTreeMap::new();
    for path in req.command.iter().flat_map(|c| c.binaries.keys()) {
        binaries.insert(path.clone(), sha_file(Path::new(path))?.0);
    }
    Ok((main, binaries))
}
/// Checked at supervisor admission; command mode checks again immediately
/// before launch and before CRIU restore.
pub(crate) fn verify_payload_identity(req: &Request) -> Result<(), String> {
    let (main, binaries) = payload_hashes(req)?;
    match &req.command {
        None if main != req.build.ecbench_sha256 => {
            Err("ecbench binary hash does not match request".into())
        }
        Some(_) if main != req.build.ecbench_sha256 => {
            Err("command argv[0] hash does not match build.ecbench_sha256".into())
        }
        Some(c) if binaries != c.binaries => {
            Err("command binary hashes do not match request".into())
        }
        _ => Ok(()),
    }
}
fn runtime(req: &Request) -> Result<Runtime, String> {
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
    let (ecbench_sha256, payload_binaries) = payload_hashes(req)?;
    Ok(Runtime {
        kernel: fs::read_to_string("/proc/sys/kernel/osrelease")
            .map_err(error)?
            .trim()
            .into(),
        cpu_identity: identity,
        ecbench_sha256,
        tools,
        payload_binaries,
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
fn continue_tree(s: &Status, cutoff: Option<deadline::Deadline>) -> Result<(), String> {
    let pid = live(s)?;
    for p in tree(pid)?.iter().rev() {
        credentials(*p)?;
        if let Some(cutoff) = cutoff {
            deadline::check(cutoff)?;
        }
        if unsafe { libc::kill(*p, libc::SIGCONT) } != 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
    }
    Ok(())
}
// Root-owned transport outside the payload-writable computation tree.
fn prepare_diagnostic_launch(req: &Request) -> Result<(), String> {
    use std::io::Write;
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let Some(transport) = &req.diagnostic_transport else {
        return Ok(());
    };
    transport.validate(&req.spec)?;
    let parent = fs::symlink_metadata("/work").map_err(error)?;
    if !parent.is_dir()
        || parent.file_type().is_symlink()
        || parent.uid() != 0
        || parent.mode() & 0o022 != 0
    {
        return Err(
            "diagnostic transport parent must be root-owned and not payload-writable".into(),
        );
    }
    let dir = Path::new("/work/diagnostic-input");
    fs::create_dir(dir).map_err(error)?;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700)).map_err(error)?;
    let path = dir.join("computation-launch.json");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o400)
        .open(&path)
        .map_err(error)?;
    file.write_all(transport.computation.raw_json.as_bytes())
        .and_then(|_| file.sync_all())
        .map_err(error)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o444)).map_err(error)?;
    file.sync_all().map_err(error)?;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o555)).map_err(error)?;
    File::open(dir).and_then(|f| f.sync_all()).map_err(error)?;
    File::open("/work")
        .and_then(|f| f.sync_all())
        .map_err(error)
}

fn launch(req: &Request, s: &mut Status, cutoff: deadline::Deadline) -> Result<(), String> {
    if s.phase != "waiting" || s.attempt != 0 {
        return Err("fresh launch is permitted once on attempt zero".into());
    }
    if Path::new("/work/compute").exists() {
        return Err("computation already exists; never restart from zero".into());
    }
    if req.is_command() {
        verify_payload_identity(req)?;
    }
    fs::create_dir("/work/compute").map_err(error)?;
    fs::create_dir("/work/compute/tmp").map_err(error)?;
    let mut owned: Vec<PathBuf> = vec!["/work/compute".into(), "/work/compute/tmp".into()];
    match &req.command {
        None => {
            save(Path::new("/work/compute/original-spec.json"), &req.spec)?;
            owned.push("/work/compute/original-spec.json".into());
        }
        Some(command) => {
            // Create every outputs component so the UID-1000 payload owns it.
            let mut path = PathBuf::from("/work/compute");
            for part in command.outputs_relative().split('/') {
                path.push(part);
                fs::create_dir(&path).map_err(error)?;
                owned.push(path.clone());
            }
        }
    }
    prepare_diagnostic_launch(req)?;
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
    for p in &owned {
        let p = std::ffi::CString::new(p.as_os_str().as_encoded_bytes()).map_err(error)?;
        if unsafe { libc::chown(p.as_ptr(), 1000, 1000) } != 0 {
            return Err(error(std::io::Error::last_os_error()));
        }
    }
    s.phase = "starting".into();
    save(&status_path(s.attempt), s)?;
    let stdout = File::create("/work/compute/run.stdout").map_err(error)?;
    let stderr = File::create("/work/compute/run.stderr").map_err(error)?;
    let mut cmd = payload_command(req);
    cmd.stdin(Stdio::null()).stdout(stdout).stderr(stderr);
    drop_payload_privileges(&mut cmd);
    if req.is_command() {
        // The progress launcher installs this for ecbench; a command payload
        // gets the same computation-only filter after its privileges drop.
        deny_perf_after_privilege_drop(&mut cmd);
    }
    if req.worker_campaign.is_some() {
        deadline::check(cutoff)?;
    }
    let child = cmd.spawn().map_err(error)?;
    s.pid = Some(child.id() as i32);
    s.starttime = Some(starttime(child.id() as i32)?);
    s.phase = "running".into();
    Ok(())
}
/// The computational payload's exact argv/env/cwd. Stdio and credential
/// changes are applied by `launch`; this function spawns nothing.
fn payload_command(req: &Request) -> Command {
    let mut cmd = match &req.command {
        None => {
            let mut cmd = Command::new(format!("{BIN}/isolab-criu-progress"));
            cmd.args([
                ECBENCH,
                "/work/compute/original-spec.json",
                "/work/compute/session",
            ]);
            cmd
        }
        Some(command) => {
            let mut cmd = Command::new(&command.argv[0]);
            cmd.args(&command.argv[1..]);
            cmd
        }
    };
    cmd.current_dir("/work/compute")
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .env("HOME", "/work/compute")
        .env("TMPDIR", "/work/compute/tmp");
    if let Some(command) = &req.command {
        cmd.env("ISOLAB_OUTPUT_DIR", &command.outputs)
            .env("ISOLAB_CRIU", "1")
            .env("RAYON_NUM_THREADS", command.threads.to_string());
    }
    if let Some(transport) = &req.diagnostic_transport {
        cmd.args([
            "--diagnostic-launch",
            "/work/diagnostic-input/computation-launch.json",
            "--diagnostic-launch-sha256",
            &transport.computation.sha256,
            "--diagnostic-spec-sha256",
            &transport.computation.spec_bytes_sha256,
        ]);
    }
    cmd
}
// Same BPF program as progress/src/main.rs deny_perf(): CRIU 4.2.1 rejects
// perf_event descriptors and credentials() requires seccomp filter mode 2.
const PERF_FILTER: [libc::sock_filter; 7] = [
    libc::sock_filter {
        code: 0x20,
        jt: 0,
        jf: 0,
        k: 4,
    }, // seccomp_data.arch
    libc::sock_filter {
        code: 0x15,
        jt: 1,
        jf: 0,
        k: 0xc000003e,
    },
    libc::sock_filter {
        code: 0x06,
        jt: 0,
        jf: 0,
        k: 0x80000000,
    }, // KILL_PROCESS
    libc::sock_filter {
        code: 0x20,
        jt: 0,
        jf: 0,
        k: 0,
    }, // syscall number
    libc::sock_filter {
        code: 0x15,
        jt: 0,
        jf: 1,
        k: libc::SYS_perf_event_open as u32,
    },
    libc::sock_filter {
        code: 0x06,
        jt: 0,
        jf: 0,
        k: 0x00050000 | libc::EPERM as u32,
    },
    libc::sock_filter {
        code: 0x06,
        jt: 0,
        jf: 0,
        k: 0x7fff0000,
    }, // ALLOW
];
/// Async-signal-safe: no allocation; requires no_new_privs already set.
fn install_perf_filter() -> std::io::Result<()> {
    let mut filter = PERF_FILTER;
    let program = libc::sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };
    // SAFETY: valid BPF program with checked architecture; copied by the kernel.
    if unsafe { libc::prctl(libc::PR_SET_SECCOMP, 2, &program as *const libc::sock_fprog) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}
fn deny_perf_after_privilege_drop(cmd: &mut Command) {
    // SAFETY: registered after drop_payload_privileges, so NNP is set first.
    unsafe {
        cmd.pre_exec(install_perf_filter);
    }
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
fn checkpoint(
    req: &Request,
    s: &mut Status,
    n: u64,
    owner: &mut owner_observation::Owner,
    cutoff: deadline::Deadline,
) -> Result<(), String> {
    // Completion can race the controller's last status read. Preserve the
    // successful computation instead of treating this as a failed checkpoint.
    if s.phase == "completed"
        && (s.exit_code == Some(0) || (req.is_command() && s.exit_code.is_some()))
    {
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
    s.phase = "checkpointing".into();
    s.generation = Some(n);
    save(&status_path(s.attempt), s)?;
    // ecbench reaps short-lived workers. CRIU can catch an already exited
    // child, whose zombie state is quiescent but cannot satisfy our strict
    // stopped/credential restore contract. Never commit such a capture.
    // Let this SAME computation reap it, then capture again. No measurement
    // epoch, deadline, or logical checkpoint identity is retried/reset.
    let mut captured = None;
    for capture in 1..=8 {
        if deadline::check(cutoff).is_err() {
            return Err("effective computation cutoff expired during checkpoint".into());
        }
        if checkpoint_completion(req, s, pid, owner)? {
            return Ok(());
        }
        let candidate = dir.join(format!("capture-{capture}"));
        fs::create_dir(&candidate).map_err(error)?;
        let mut cmd = criu();
        cmd.args(["dump", "--tree", &pid.to_string()]);
        common_criu(&mut cmd, &candidate);
        cmd.args(["--log-file", "dump.log"]);
        let result = if req.worker_campaign.is_some() {
            deadline::output(
                &mut cmd,
                cutoff,
                &candidate.join("dump.stdout"),
                &candidate.join("dump.stderr"),
                false,
            )?
        } else {
            cmd.output().map_err(error)?
        };
        let receipt = json!({"exit_code":result.status.code(),"stderr":String::from_utf8_lossy(&result.stderr)});
        save(&candidate.join("dump-result.json"), &receipt)?;
        if !result.status.success() {
            return Err("CRIU dump failed; retained log, no generation committed".into());
        }
        let pids = tree(pid)?;
        let mut has_zombie = false;
        for p in &pids {
            let st = payload_status(*p)?;
            fs::write(candidate.join(format!("process-{p}-status.txt")), &st).map_err(error)?;
            credentials(*p)?;
            has_zombie |= !stopped_or_zombie(&st)?;
            // Every thread must be quiescent, not merely its process leader.
            for task in fs::read_dir(format!("/proc/{p}/task")).map_err(error)? {
                let path = task.map_err(error)?.path().join("status");
                has_zombie |= !stopped_or_zombie(&fs::read_to_string(path).map_err(error)?)?;
            }
        }
        if !has_zombie {
            fs::rename(&candidate, &dump).map_err(error)?;
            save(&dir.join("dump-result.json"), &receipt)?;
            captured = Some(pids);
            break;
        }
        save(
            &candidate.join("not-committed.json"),
            &json!({"reason":"captured zombie; resume same computation to reap before recapturing","capture":capture}),
        )?;
        if !stopped_or_zombie(&payload_status(pid)?)? {
            if !checkpoint_completion(req, s, pid, owner)? {
                return Err("captured zombie root was not waitable".into());
            }
            return Ok(());
        }
        if capture == 8 {
            return Err(
                "checkpoint exhausted eight captures containing zombies; no generation committed"
                    .into(),
            );
        }
        continue_tree(s, req.worker_campaign.as_ref().map(|_| cutoff))?;
        thread::sleep(Duration::from_millis(10));
    }
    let pids = captured.ok_or("checkpoint has no admitted stopped capture")?;
    sync_work()?;
    let mut entries = BTreeMap::new();
    files(Path::new("/work/compute"), &mut entries)?;
    files(&dump, &mut entries)?;
    let manifest = Manifest {
        schema: "isolab.criu.generation/v1".into(),
        request_sha256: req.hash(),
        generation: n,
        runtime: runtime(req)?,
        root_pid: pid,
        pids,
        files: entries,
    };
    save(&dir.join("manifest.json"), &manifest)?;
    sync_work()?;
    s.phase = "stopped".into();
    Ok(())
}
fn checkpoint_completion(
    req: &Request,
    s: &mut Status,
    pid: i32,
    owner: &mut owner_observation::Owner,
) -> Result<bool, String> {
    if s.pid != Some(pid) {
        return Err("checkpoint owner PID mismatch".into());
    }
    match owner.poll(req, s, None)? {
        None => Ok(false),
        Some(code) if code == 0 || req.is_command() => {
            s.exit_code = Some(code);
            s.phase = "completed".into();
            Ok(true)
        }
        Some(_) => Err("computation failed during checkpoint".into()),
    }
}
// Only a stopped task is eligible for a committed generation. Zombies are
// distinguished solely to request an uncommitted recapture; all other states
// remain errors. In particular a ptrace stop is not our SIGSTOP contract.
fn stopped_or_zombie(s: &str) -> Result<bool, String> {
    match s
        .lines()
        .find_map(|line| line.strip_prefix("State:"))
        .map(str::trim)
    {
        Some("T (stopped)") => Ok(true),
        Some("Z (zombie)") => Ok(false),
        state => Err(format!("checkpoint task is not stopped: {state:?}")),
    }
}
fn restore(
    req: &Request,
    s: &mut Status,
    n: u64,
    expected: Option<&str>,
    cutoff: deadline::Deadline,
) -> Result<(), String> {
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
        || m.runtime != runtime(req)?
        || m.runtime.ecbench_sha256 != req.build.ecbench_sha256
        || req
            .command
            .as_ref()
            .is_some_and(|c| m.runtime.payload_binaries != c.binaries)
    {
        return Err("checkpoint request/runtime identity mismatch".into());
    }
    if req.is_command() {
        verify_payload_identity(req)?;
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
    let result = if req.worker_campaign.is_some() {
        deadline::output(
            &mut cmd,
            cutoff,
            &attempt.join("restore.stdout"),
            &attempt.join("restore.stderr"),
            false,
        )?
    } else {
        cmd.output().map_err(error)?
    };
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
fn verify(s: &mut Status, cutoff: Option<deadline::Deadline>) -> Result<(), String> {
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
    let result = if let Some(cutoff) = cutoff {
        deadline::output(
            &mut command,
            cutoff,
            &dir.join("audit-command.stdout"),
            &dir.join("audit-command.stderr"),
            true,
        )?
    } else {
        command.output().map_err(error)?
    };
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
/// Supervisor phase after the payload root is reaped. ecbench requires exit 0
/// for native replay; command mode records ANY exit as completed (never
/// retried), unless the supervisor already failed and killed the payload.
fn completion_phase(req: &Request, current: &str, code: i32) -> &'static str {
    match &req.command {
        None if code == 0 => "completed",
        None => "failed",
        Some(_) if current == "failed" => "failed",
        Some(_) => "completed",
    }
}

pub const MAX_SEALED_FILES: usize = 1024;
/// S3 single PUT bound; larger outputs stay on the retained PVC, listed as skipped.
pub const MAX_SEALED_FILE_BYTES: u64 = 5 * 1024 * 1024 * 1024;
const MAX_SEALED_PATH_BYTES: usize = 900;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SealedFile {
    /// Relative to /work/compute, e.g. `out/result.json` or `run.stdout`.
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkippedEntry {
    pub path: String,
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandSeal {
    pub schema: String,
    pub request_sha256: String,
    pub attempt: u32,
    pub exit_code: i32,
    pub outputs: String,
    pub files: Vec<SealedFile>,
    pub skipped: Vec<SkippedEntry>,
    pub sha256sums_sha256: String,
}
/// Name accepted for upload: UTF-8 relative path, normalized, no control
/// characters that would break SHA256SUMS lines or S3 keys.
pub(crate) fn sealable_path(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= MAX_SEALED_PATH_BYTES
        && !path.starts_with('/')
        && !path.bytes().any(|c| c < 0x20 || c == 0x7f || c == b'\\')
        && path
            .split('/')
            .all(|c| !c.is_empty() && c != "." && c != "..")
}
pub(crate) fn sha256sums(files: &[SealedFile]) -> Vec<u8> {
    files
        .iter()
        .map(|f| format!("{}  {}\n", f.sha256, f.path))
        .collect::<String>()
        .into_bytes()
}
fn seal_walk(
    root: &Path,
    rel: &str,
    files: &mut Vec<SealedFile>,
    skipped: &mut Vec<SkippedEntry>,
) -> Result<(), String> {
    let full = root.join(rel);
    let meta = match fs::symlink_metadata(&full) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            skipped.push(SkippedEntry {
                path: rel.into(),
                reason: "absent".into(),
            });
            return Ok(());
        }
        Err(e) => return Err(error(e)),
    };
    let kind = meta.file_type();
    if kind.is_dir() {
        let mut names = Vec::new();
        for entry in fs::read_dir(&full).map_err(error)? {
            names.push(entry.map_err(error)?.file_name());
        }
        names.sort();
        for name in names {
            match name.to_str() {
                Some(name) => seal_walk(root, &format!("{rel}/{name}"), files, skipped)?,
                None => skipped.push(SkippedEntry {
                    path: format!("{rel}/{}", name.to_string_lossy()),
                    reason: "non-UTF-8 name".into(),
                }),
            }
        }
    } else if !kind.is_file() {
        skipped.push(SkippedEntry {
            path: rel.into(),
            reason: "not a regular file or directory".into(),
        });
    } else if !sealable_path(rel) {
        skipped.push(SkippedEntry {
            path: rel.into(),
            reason: "unsupported path name".into(),
        });
    } else if meta.len() > MAX_SEALED_FILE_BYTES {
        skipped.push(SkippedEntry {
            path: rel.into(),
            reason: "exceeds 5 GiB single-object bound".into(),
        });
    } else {
        if files.len() >= MAX_SEALED_FILES {
            return Err("command outputs exceed 1024 files; retained on the PVC only".into());
        }
        let (sha256, bytes) = sha_file(&full)?;
        files.push(SealedFile {
            path: rel.into(),
            sha256,
            bytes,
        });
    }
    Ok(())
}
/// Hash the outputs tree (no symlink is followed) plus run.stdout/run.stderr.
pub(crate) fn seal_tree(
    root: &Path,
    outputs: &str,
) -> Result<(Vec<SealedFile>, Vec<SkippedEntry>), String> {
    let (mut files, mut skipped) = (Vec::new(), Vec::new());
    seal_walk(root, outputs, &mut files, &mut skipped)?;
    for log in ["run.stdout", "run.stderr"] {
        seal_walk(root, log, &mut files, &mut skipped)?;
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok((files, skipped))
}
fn seal_dir(attempt: u32) -> PathBuf {
    attempt_dir(attempt).join("command")
}
/// Command mode replacement for native replay: freeze and hash the outputs.
fn seal(req: &Request, s: &mut Status) -> Result<(), String> {
    let command = req.command.as_ref().ok_or("seal is command mode only")?;
    if s.phase == "sealed" {
        return Ok(());
    }
    let exit_code = s.exit_code.ok_or("command exit code missing")?;
    if s.phase != "completed" {
        return Err("seal requires a completed command".into());
    }
    // The root was reaped; no remaining member of its process group may write
    // after hashing. (Same group kill the supervisor uses on failure.)
    if let Some(pid) = s.pid {
        unsafe { libc::kill(-pid, libc::SIGKILL) };
    }
    let (files, skipped) = seal_tree(Path::new("/work/compute"), command.outputs_relative())?;
    let sums = sha256sums(&files);
    let dir = seal_dir(s.attempt);
    fs::create_dir_all(&dir).map_err(error)?;
    atomic(&dir.join("SHA256SUMS"), &sums)?;
    save(
        &dir.join("seal.json"),
        &CommandSeal {
            schema: "isolab.recovery.command-seal/v1".into(),
            request_sha256: req.hash(),
            attempt: s.attempt,
            exit_code,
            outputs: command.outputs.clone(),
            files,
            skipped,
            sha256sums_sha256: crate::contract::digest(&sums),
        },
    )?;
    sync_work()?;
    s.phase = "sealed".into();
    Ok(())
}
/// Generations to delete from the live PVC once `newest` is committed: every
/// `gK` with K < newest-1. Refused when `newest` is absent or any newer
/// (uncommitted) generation directory exists.
fn prune_plan(existing: &[u64], newest: u64) -> Result<Vec<u64>, String> {
    if newest == 0 || !existing.contains(&newest) {
        return Err("newest committed generation is absent on the PVC".into());
    }
    if existing.iter().any(|k| *k > newest) {
        return Err("a newer generation exists and is not committed".into());
    }
    let mut prune: Vec<u64> = existing
        .iter()
        .copied()
        .filter(|k| *k < newest - 1)
        .collect();
    prune.sort_unstable();
    prune.dedup();
    Ok(prune)
}
fn tree_bytes(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return 0;
    };
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| tree_bytes(&e.path()))
                .sum()
        })
        .unwrap_or(0)
}
/// Delete PVC-local generation directories older than the two newest committed
/// ones. Never touches S3 or EBS. Every outcome, including a refusal, is a
/// receipt; housekeeping never fails the computation.
fn prune_generations(dir: &Path, newest: u64, expected_manifest: Option<&str>) -> Value {
    let mut existing = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name();
            let number = name
                .to_str()
                .and_then(|n| n.strip_prefix('g'))
                .filter(|d| !d.is_empty() && d.bytes().all(|c| c.is_ascii_digit()))
                .and_then(|d| d.parse::<u64>().ok());
            let is_dir = fs::symlink_metadata(entry.path()).is_ok_and(|m| m.is_dir());
            if let (Some(number), true) = (number, is_dir) {
                existing.push(number);
            }
        }
    }
    existing.sort_unstable();
    let manifest = sha_file(&dir.join(format!("g{newest}/manifest.json"))).map(|v| v.0);
    let plan = if expected_manifest.is_none() || manifest.as_deref().ok() != expected_manifest {
        Err("newest generation manifest is not the committed manifest".to_string())
    } else {
        prune_plan(&existing, newest)
    };
    let (mut pruned, mut errors) = (Vec::new(), Vec::new());
    if let Ok(plan) = &plan {
        for k in plan {
            let path = dir.join(format!("g{k}"));
            let bytes = tree_bytes(&path);
            match fs::remove_dir_all(&path) {
                Ok(()) => pruned.push(json!({"generation":k,"bytes":bytes})),
                Err(e) => errors.push(json!({"generation":k,"error":error(e)})),
            }
        }
    }
    json!({"schema":"isolab.recovery.generation-prune/v1","newest_committed":newest,
        "existing_before":existing,"kept":existing.iter().filter(|k| **k + 1 >= newest).collect::<Vec<_>>(),
        "pruned":pruned,"errors":errors,"refused":plan.err(),
        "scope":"PVC-local /work/generations only; S3 archives and EBS snapshots untouched"})
}
fn prune(
    req: &Request,
    s: &Status,
    newest: u64,
    expected_manifest: Option<&str>,
) -> Result<(), String> {
    if !req.is_command() {
        return Err("generation pruning is command mode only".into());
    }
    let mut receipt = prune_generations(
        &Path::new(ROOT).join("generations"),
        newest,
        expected_manifest,
    );
    receipt["request_sha256"] = json!(req.hash());
    receipt["attempt"] = json!(s.attempt);
    receipt["at"] = json!(now());
    save(
        &attempt_dir(s.attempt).join(format!("prune-g{newest}.json")),
        &receipt,
    )
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
// Explicit transport from the controller-owned Pod manifest. The digest binds
// exact bytes; it is not independent attestation of the campaign CAS.
fn computation_cutoff(
    req: &Request,
    attempt: u32,
    raw: Option<&str>,
    expected: Option<&str>,
) -> Result<u64, String> {
    let Some(binding) = &req.worker_campaign else {
        return if raw.is_none() && expected.is_none() {
            Ok(req.deadline_unix_seconds)
        } else {
            Err("unexpected reservation for legacy computation".into())
        };
    };
    let raw = raw.ok_or("campaign computation reservation missing")?;
    let expected = expected.ok_or("campaign reservation digest missing")?;
    if raw.len() > 4096
        || !crate::contract::hex(expected, 64)
        || crate::contract::digest(raw.as_bytes()) != expected
    {
        return Err("campaign computation reservation bytes/digest mismatch".into());
    }
    let r: crate::campaign::Reservation = serde_json::from_str(raw).map_err(error)?;
    // Typed deny-unknown parsing also rejects duplicate reservation fields.
    binding.validate(
        &crate::contract::digest(&serde_json::to_vec(&req.spec).map_err(error)?),
        req.deadline_unix_seconds,
    )?;
    crate::campaign::Ledger {
        schema: "isolab.worker-admission-ledger/v1".into(),
        policy: binding.policy.clone(),
        reservations: vec![r.clone()],
    }
    .validate()?;
    if r.request_sha256 != req.hash()
        || r.attempt != attempt
        || r.epoch != crate::campaign::Epoch::Computation
        || r.ceiling_seconds != binding.computation_ceiling_seconds
        || r.shutdown_reserve_seconds != binding.shutdown_reserve_seconds
        || r.deadline_unix_seconds > req.deadline_unix_seconds
    {
        return Err("campaign computation reservation identity/envelope mismatch".into());
    }
    Ok(r.deadline_unix_seconds.min(req.deadline_unix_seconds))
}
fn cutoff_from_environment(req: &Request, attempt: u32) -> Result<u64, String> {
    let read = |name: &str| -> Result<Option<String>, String> {
        match std::env::var(name) {
            Ok(v) => Ok(Some(v)),
            Err(std::env::VarError::NotPresent) => Ok(None),
            Err(e) => Err(error(e)),
        }
    };
    let raw = read("ISOLAB_COMPUTATION_RESERVATION")?;
    let expected = read("ISOLAB_COMPUTATION_RESERVATION_SHA256")?;
    let cutoff = computation_cutoff(req, attempt, raw.as_deref(), expected.as_deref())?;
    if let Some(raw) = raw {
        let reservation: crate::campaign::Reservation =
            serde_json::from_str(&raw).map_err(error)?;
        reservation.remaining_payload_seconds(deadline::now()?)?;
    }
    Ok(cutoff)
}

pub fn supervise(req: Request, attempt: u32) -> Result<(), String> {
    let utc_cutoff = cutoff_from_environment(&req, attempt)?;
    let cutoff = if req.worker_campaign.is_some() {
        deadline::Deadline::capture(utc_cutoff)?
    } else {
        deadline::Deadline::utc_only(utc_cutoff)
    };
    if req.worker_campaign.is_some() && deadline::check(cutoff).is_err() {
        return Err("effective computation cutoff expired before supervisor admission".into());
    }
    if unsafe { libc::getpid() } != 1 || unsafe { libc::geteuid() } != 0 {
        return Err("trusted supervisor must be root PID 1 in its own Pod namespace".into());
    }
    if req.worker_campaign.is_some() {
        let namespace = std::env::var("ISOLAB_COMPUTATION_PRIVATE_PID_NAMESPACE").map_err(error)?;
        deadline::install_namespace_backstop(cutoff, &namespace)?;
    }
    // Full runtime identity first (tools, kernel, CPU), as before; then the
    // request-bound payload hashes (ecbench, or argv[0] plus binaries).
    runtime(&req)?;
    verify_payload_identity(&req)?;
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
    let mut owner = owner_observation::Owner::begin();
    loop {
        if deadline::check(cutoff).is_err() {
            if let Some(pid) = s.pid {
                unsafe { libc::kill(-pid, libc::SIGKILL) };
            }
            if let Err(error) = owner.poll(&req, &s, Some("effective computation cutoff expired")) {
                eprintln!("owner receipt: {error}");
            }
            s.phase = "failed".into();
            s.error = Some("effective computation cutoff expired".into());
            save(&status_path(attempt), &s)?;
            return Err("effective computation cutoff expired".into());
        }
        // The progress launcher execs ecbench in the same PID. Owner observation
        // distinguishes confirmed post-exec identity from startup failure.
        match owner.poll(&req, &s, None) {
            Ok(Some(code)) => {
                s.exit_code = Some(code);
                s.phase = completion_phase(&req, &s.phase, code).into();
            }
            Ok(None) => {}
            Err(error) => eprintln!("owner observation: {error}"),
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
            let result = if deadline::check(cutoff).is_err() {
                Err("effective computation cutoff expired before operation".into())
            } else if op.request_sha256 != req.hash() || op.attempt != attempt {
                Err("operation request mismatch".into())
            } else {
                match op.action.as_str() {
                    "start" => launch(&req, &mut s, cutoff),
                    "checkpoint" => checkpoint(&req, &mut s, op.generation, &mut owner, cutoff),
                    "continue" => {
                        if s.phase == "stopped" && s.generation == Some(op.generation) {
                            continue_tree(&s, req.worker_campaign.as_ref().map(|_| cutoff))
                                .map(|()| s.phase = "running".into())
                        } else {
                            Err("continue does not match stopped generation".into())
                        }
                    }
                    "restore" => restore(
                        &req,
                        &mut s,
                        op.generation,
                        op.manifest_sha256.as_deref(),
                        cutoff,
                    ),
                    "verify" if req.is_command() => {
                        Err("command mode has no native replay; seal its outputs".into())
                    }
                    "verify" => verify(&mut s, req.worker_campaign.as_ref().map(|_| cutoff)),
                    "seal" => seal(&req, &mut s),
                    "prune" => prune(&req, &s, op.generation, op.manifest_sha256.as_deref()),
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
        checkpoint(
            &req,
            &mut s,
            1,
            &mut owner_observation::Owner::begin(),
            deadline::Deadline::utc_only(req.deadline_unix_seconds),
        )
        .unwrap();
        assert_eq!(s.phase, "completed");
        assert!(s.generation.is_none());
    }
    #[test]
    fn running_and_traced_tasks_cannot_be_recapture_candidates() {
        assert_eq!(stopped_or_zombie("State:\tT (stopped)\n"), Ok(true));
        assert_eq!(stopped_or_zombie("State:\tZ (zombie)\n"), Ok(false));
        for state in [
            "R (running)",
            "S (sleeping)",
            "D (disk sleep)",
            "t (tracing stop)",
            "",
        ] {
            assert!(stopped_or_zombie(&format!("State:\t{state}\n")).is_err());
        }
    }
    #[test]
    fn real_exited_child_requires_recapture_without_reaping_it() {
        // Exercise the actual /proc state from the live qualification failure.
        // Child executes only async-signal-safe _exit after a test-runner fork.
        let child = unsafe { libc::fork() };
        assert!(child >= 0);
        if child == 0 {
            unsafe { libc::_exit(17) }
        }
        let mut info = unsafe { std::mem::zeroed::<libc::siginfo_t>() };
        let waited = unsafe {
            libc::waitid(
                libc::P_PID,
                child as u32,
                &mut info,
                libc::WEXITED | libc::WNOWAIT,
            )
        };
        let observed = payload_status(child).and_then(|s| stopped_or_zombie(&s));
        let mut status = 0;
        let reaped = unsafe { libc::waitpid(child, &mut status, 0) };
        assert_eq!(waited, 0);
        assert_eq!(observed, Ok(false));
        assert_eq!(reaped, child);
        assert_eq!(libc::WEXITSTATUS(status), 17);
    }
}

#[cfg(test)]
mod cutoff_transport_tests {
    use super::*;
    #[test]
    fn legacy_cutoff_is_unchanged_and_ambient_transport_rejected() {
        let req = crate::contract::fixture();
        assert!(req.worker_campaign.is_none());
        assert_eq!(
            computation_cutoff(&req, 0, None, None).unwrap(),
            req.deadline_unix_seconds
        );
        assert!(computation_cutoff(&req, 0, Some("{}"), None).is_err());
        assert!(computation_cutoff(&req, 0, None, Some(&"a".repeat(64))).is_err());
    }
}

#[cfg(test)]
mod absolute_binding_tests {
    use super::*;
    #[test]
    fn delayed_schedule_does_not_rebase_absolute_reservation() {
        use crate::campaign::{Binding, Epoch, Policy, Reservation};
        let mut req = crate::contract::fixture();
        req.deadline_unix_seconds = 1793396000;
        let spec = crate::contract::digest(&serde_json::to_vec(&req.spec).unwrap());
        let mut route: Vec<_> = (0..7).map(|n| format!("{n:064x}")).collect();
        route.push(spec);
        let policy = Policy {
            schema: "isolab.worker-admission-policy/v1".into(),
            authorization_sha256:
                "201df07743540ac247e2ae0ec79c82c7b284aa16dd07d468924c87cf2a4b7a2b".into(),
            cumulative_ceiling_seconds: 25200,
            absolute_deadline_unix_seconds: 1793398120,
            latest_scientific_stop_unix_seconds: 1793396320,
            allocation_unit: "conservative_worker_allocation_seconds".into(),
            route_spec_sha256: route,
            prior_charge_seconds: 0,
            reconciliation_receipt_sha256: "a".repeat(64),
            owner_enforcement_review_sha256: "b".repeat(64),
        };
        req.worker_campaign = Some(Binding {
            policy: policy.clone(),
            ledger_uid: "fixture-ledger".into(),
            computation_ceiling_seconds: 1000,
            fresh_ceiling_seconds: 1000,
            shutdown_reserve_seconds: 60,
        });
        let r = Reservation {
            request_sha256: req.hash(),
            attempt: 2,
            epoch: Epoch::Computation,
            policy_sha256: policy.hash().unwrap(),
            admitted_unix_seconds: 1793390000,
            deadline_unix_seconds: 1793390940,
            ceiling_seconds: 1000,
            shutdown_reserve_seconds: 60,
        };
        let raw = serde_json::to_string(&r).unwrap();
        let sha = crate::contract::digest(raw.as_bytes());
        let cutoff = computation_cutoff(&req, 2, Some(&raw), Some(&sha)).unwrap();
        assert_eq!(cutoff, 1793390940);
        assert_eq!(r.remaining_payload_seconds(1793390900).unwrap(), 40);
        assert!(r.remaining_payload_seconds(1793390940).is_err());
        assert!(r.remaining_payload_seconds(1793389999).is_err());
        assert!(computation_cutoff(&req, 3, Some(&raw), Some(&sha)).is_err());
        assert!(computation_cutoff(&req, 2, Some(&raw), Some(&"0".repeat(64))).is_err());
        assert!(computation_cutoff(&req, 2, None, None).is_err());
        let mut fresh = r;
        fresh.epoch = Epoch::Fresh;
        let raw = serde_json::to_string(&fresh).unwrap();
        assert!(computation_cutoff(
            &req,
            2,
            Some(&raw),
            Some(&crate::contract::digest(raw.as_bytes()))
        )
        .is_err());
    }
}

#[cfg(test)]
mod command_mode_tests {
    use super::*;
    use std::ffi::OsStr;
    fn args(cmd: &Command) -> Vec<&OsStr> {
        cmd.get_args().collect()
    }
    fn envs(cmd: &Command) -> BTreeMap<String, String> {
        cmd.get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.unwrap().to_string_lossy().into_owned(),
                )
            })
            .collect()
    }
    fn base_env() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("PATH".into(), "/usr/local/bin:/usr/bin:/bin".into()),
            ("HOME".into(), "/work/compute".into()),
            ("TMPDIR".into(), "/work/compute/tmp".into()),
        ])
    }
    #[test]
    fn ecbench_launch_argv_and_environment_are_unchanged() {
        let req = crate::contract::fixture();
        let cmd = payload_command(&req);
        assert_eq!(
            cmd.get_program(),
            OsStr::new("/opt/isolab/recovery/isolab-criu-progress")
        );
        assert_eq!(
            args(&cmd),
            [
                "/usr/local/bin/ecbench",
                "/work/compute/original-spec.json",
                "/work/compute/session"
            ]
        );
        assert_eq!(cmd.get_current_dir(), Some(Path::new("/work/compute")));
        assert_eq!(envs(&cmd), base_env());
        let mut diagnostic = req.clone();
        diagnostic.diagnostic_transport = Some(crate::contract::DiagnosticTransport {
            schema: "isolab.diagnostic-transport/v1".into(),
            computation: crate::contract::DiagnosticLaunchBytes {
                raw_json: "{}".into(),
                sha256: "1".repeat(64),
                spec_bytes_sha256: "2".repeat(64),
            },
            fresh: crate::contract::DiagnosticLaunchBytes {
                raw_json: "{}".into(),
                sha256: "3".repeat(64),
                spec_bytes_sha256: "4".repeat(64),
            },
        });
        let cmd = payload_command(&diagnostic);
        let (one, two) = ("1".repeat(64), "2".repeat(64));
        assert_eq!(
            args(&cmd)[3..],
            [
                "--diagnostic-launch",
                "/work/diagnostic-input/computation-launch.json",
                "--diagnostic-launch-sha256",
                one.as_str(),
                "--diagnostic-spec-sha256",
                two.as_str()
            ]
        );
    }
    #[test]
    fn command_launch_execs_argv_with_bound_environment() {
        let req = crate::contract::command_fixture();
        let cmd = payload_command(&req);
        assert_eq!(
            cmd.get_program(),
            OsStr::new("/opt/study/launch/job-prime-tower-criu.sh")
        );
        assert_eq!(args(&cmd), ["main"]);
        assert_eq!(cmd.get_current_dir(), Some(Path::new("/work/compute")));
        let mut expected = base_env();
        expected.insert("ISOLAB_OUTPUT_DIR".into(), "/work/compute/out".into());
        expected.insert("ISOLAB_CRIU".into(), "1".into());
        expected.insert("RAYON_NUM_THREADS".into(), "2".into());
        assert_eq!(envs(&cmd), expected);
        let mut single = req.clone();
        let c = single.command.as_mut().unwrap();
        c.argv.truncate(1);
        c.threads = 1;
        let cmd = payload_command(&single);
        assert!(args(&cmd).is_empty());
        assert_eq!(envs(&cmd)["RAYON_NUM_THREADS"], "1");
    }
    #[test]
    fn command_completion_records_any_exit_but_ecbench_requires_zero() {
        let ecbench = crate::contract::fixture();
        let command = crate::contract::command_fixture();
        assert_eq!(completion_phase(&ecbench, "running", 0), "completed");
        assert_eq!(completion_phase(&ecbench, "running", 3), "failed");
        assert_eq!(completion_phase(&command, "running", 0), "completed");
        assert_eq!(completion_phase(&command, "running", 3), "completed");
        assert_eq!(completion_phase(&command, "running", 137), "completed");
        // A supervisor-killed payload stays failed; its exit is not a result.
        assert_eq!(completion_phase(&command, "failed", 137), "failed");
        let mut s = Status {
            request_sha256: command.hash(),
            attempt: 0,
            phase: "completed".into(),
            pid: None,
            starttime: None,
            generation: None,
            exit_code: Some(3),
            error: None,
            updated: 0,
        };
        // A late checkpoint request does not turn a nonzero command exit into failure.
        checkpoint(
            &command,
            &mut s,
            1,
            &mut owner_observation::Owner::begin(),
            deadline::Deadline::utc_only(command.deadline_unix_seconds),
        )
        .unwrap();
        assert_eq!(s.phase, "completed");
        assert!(s.generation.is_none());
        let op = Operation {
            request_sha256: command.hash(),
            attempt: 0,
            action: "seal".into(),
            generation: 0,
            manifest_sha256: None,
        };
        assert_eq!(op.id().unwrap(), format!("seal-{:020}", 0));
        // Seal is refused outside command mode and before completion.
        assert!(seal(&ecbench, &mut s.clone()).is_err());
        s.phase = "running".into();
        assert!(seal(&command, &mut s).is_err());
    }
    #[test]
    fn argv0_and_listed_binaries_are_hash_verified() {
        let sh = sha_file(Path::new("/bin/sh")).unwrap().0;
        let sleep = sha_file(Path::new("/bin/sleep")).unwrap().0;
        let mut req = crate::contract::command_fixture();
        req.build.ecbench_sha256 = sh.clone();
        let c = req.command.as_mut().unwrap();
        c.argv[0] = "/bin/sh".into();
        c.binaries = BTreeMap::from([("/bin/sleep".into(), sleep.clone())]);
        req.validate().unwrap();
        verify_payload_identity(&req).unwrap();
        let (main, binaries) = payload_hashes(&req).unwrap();
        assert_eq!(main, sh);
        assert_eq!(binaries["/bin/sleep"], sleep);
        let mut wrong = req.clone();
        wrong.build.ecbench_sha256 = "0".repeat(64);
        assert!(verify_payload_identity(&wrong)
            .unwrap_err()
            .contains("argv[0]"));
        let mut wrong = req.clone();
        wrong
            .command
            .as_mut()
            .unwrap()
            .binaries
            .insert("/bin/sleep".into(), "0".repeat(64));
        assert!(verify_payload_identity(&wrong).is_err());
        let mut missing = req.clone();
        missing
            .command
            .as_mut()
            .unwrap()
            .binaries
            .insert("/opt/absent/binary".into(), "0".repeat(64));
        assert!(verify_payload_identity(&missing).is_err());
        // The ecbench path still hashes the fixed ecbench location only.
        let mut ecbench = crate::contract::fixture();
        ecbench.build.ecbench_sha256 = sh;
        assert_eq!(payload_executable(&ecbench), ECBENCH);
        assert!(payload_hashes(&ecbench).map_or(true, |(_, b)| b.is_empty()));
    }
    #[test]
    fn manifest_runtime_bytes_are_unchanged_without_command_binaries() {
        let runtime = Runtime {
            kernel: "k".into(),
            cpu_identity: vec!["vendor_id: x".into()],
            ecbench_sha256: "a".repeat(64),
            tools: BTreeMap::from([("criu".into(), "b".repeat(64))]),
            payload_binaries: BTreeMap::new(),
        };
        let legacy = format!(
            r#"{{"kernel":"k","cpu_identity":["vendor_id: x"],"ecbench_sha256":"{}","tools":{{"criu":"{}"}}}}"#,
            "a".repeat(64),
            "b".repeat(64)
        );
        assert_eq!(serde_json::to_string(&runtime).unwrap(), legacy);
        assert_eq!(serde_json::from_str::<Runtime>(&legacy).unwrap(), runtime);
    }
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn command_perf_filter_denies_perf_and_reports_seccomp_filter_mode() {
        let pid = unsafe { libc::fork() };
        assert!(pid >= 0);
        if pid == 0 {
            let nnp = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } == 0;
            if !nnp || install_perf_filter().is_err() {
                unsafe { libc::_exit(2) }
            }
            let rc = unsafe {
                libc::syscall(
                    libc::SYS_perf_event_open,
                    std::ptr::null::<u8>(),
                    0,
                    -1,
                    -1,
                    0,
                )
            };
            let denied =
                rc == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM);
            let mode = unsafe { libc::prctl(libc::PR_GET_SECCOMP, 0, 0, 0, 0) };
            unsafe { libc::_exit(if denied && mode == 2 { 0 } else { 3 }) }
        }
        let mut status = 0;
        assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
        assert_eq!(status, 0);
    }
    #[test]
    fn seal_hashes_outputs_and_logs_without_following_links() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("command-seal-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("out/nested")).unwrap();
        fs::write(root.join("out/b.txt"), b"bee").unwrap();
        fs::write(root.join("out/nested/a.json"), b"{}").unwrap();
        fs::write(root.join("out/bad\nname"), b"x").unwrap();
        fs::write(root.join("secret"), b"not an output").unwrap();
        symlink("../secret", root.join("out/link")).unwrap();
        fs::write(root.join("run.stdout"), b"stdout").unwrap();
        fs::write(root.join("unlisted"), b"z").unwrap();
        let (files, skipped) = seal_tree(&root, "out").unwrap();
        let paths: Vec<_> = files.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, ["out/b.txt", "out/nested/a.json", "run.stdout"]);
        assert_eq!(files[0].sha256, crate::contract::digest(b"bee"));
        assert_eq!(files[0].bytes, 3);
        let reasons: BTreeMap<_, _> = skipped
            .iter()
            .map(|s| (s.path.as_str(), s.reason.as_str()))
            .collect();
        assert_eq!(reasons["out/link"], "not a regular file or directory");
        assert_eq!(reasons["out/bad\nname"], "unsupported path name");
        assert_eq!(reasons["run.stderr"], "absent");
        let sums = String::from_utf8(sha256sums(&files)).unwrap();
        assert_eq!(sums.lines().count(), 3);
        assert!(sums.starts_with(&format!("{}  out/b.txt\n", crate::contract::digest(b"bee"))));
        // An absent outputs directory is recorded, not an error.
        let (files, skipped) = seal_tree(&root, "missing").unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(skipped[0].path, "missing");
        for bad in ["", "/abs", "a/../b", "a//b", "./a", "a\\b", "tab\tname"] {
            assert!(!sealable_path(bad), "{bad:?}");
        }
        fs::remove_dir_all(&root).unwrap();
    }
    #[test]
    fn prune_plan_keeps_exactly_the_latest_two_committed() {
        assert_eq!(prune_plan(&[1], 1), Ok(vec![]));
        assert_eq!(prune_plan(&[1, 2], 2), Ok(vec![]));
        assert_eq!(prune_plan(&[1, 2, 3], 3), Ok(vec![1]));
        assert_eq!(prune_plan(&[1, 2, 3, 4, 5], 5), Ok(vec![1, 2, 3]));
        // A clone restored from an older snapshot keeps its own older directory.
        assert_eq!(prune_plan(&[3, 6], 6), Ok(vec![3]));
        assert!(prune_plan(&[1, 2, 3], 2).is_err(), "uncommitted g3 exists");
        assert!(prune_plan(&[1, 2], 3).is_err(), "newest absent");
        assert!(prune_plan(&[], 0).is_err());
    }
    #[test]
    fn prune_generations_deletes_only_old_local_directories() {
        use std::os::unix::fs::symlink;
        let dir = std::env::temp_dir().join(format!("prune-gen-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let other = std::env::temp_dir().join(format!("prune-target-{}", std::process::id()));
        let _ = fs::remove_dir_all(&other);
        fs::create_dir_all(other.join("keep")).unwrap();
        for k in 1..=5 {
            fs::create_dir_all(dir.join(format!("g{k}/dump"))).unwrap();
            fs::write(dir.join(format!("g{k}/dump/pages.img")), vec![0u8; 10 * k]).unwrap();
            fs::write(dir.join(format!("g{k}/manifest.json")), format!("m{k}")).unwrap();
        }
        fs::write(dir.join("notes"), b"x").unwrap();
        symlink(&other, dir.join("g0")).unwrap();
        let m5 = crate::contract::digest(b"m5");
        // Wrong or missing manifest: nothing is deleted.
        for expected in [None, Some("0".repeat(64))] {
            let r = prune_generations(&dir, 5, expected.as_deref());
            assert!(r["refused"].is_string());
            assert_eq!(r["pruned"], json!([]));
            assert!(dir.join("g1").exists());
        }
        // Uncommitted newer generation present: nothing is deleted.
        fs::create_dir(dir.join("g6")).unwrap();
        let r = prune_generations(&dir, 5, Some(&m5));
        assert!(r["refused"].as_str().unwrap().contains("not committed"));
        assert!(dir.join("g1").exists());
        fs::remove_dir(dir.join("g6")).unwrap();
        let r = prune_generations(&dir, 5, Some(&m5));
        assert!(r["refused"].is_null());
        assert_eq!(r["kept"], json!([4, 5]));
        assert_eq!(
            r["pruned"],
            json!([{"generation":1,"bytes":12},{"generation":2,"bytes":22},{"generation":3,"bytes":32}])
        );
        for k in 1..=3 {
            assert!(!dir.join(format!("g{k}")).exists());
        }
        for k in 4..=5 {
            assert!(dir.join(format!("g{k}/dump/pages.img")).exists());
        }
        assert!(dir.join("notes").exists());
        assert!(
            other.join("keep").exists(),
            "symlinked g0 is never followed"
        );
        // Idempotent repeat.
        let again = prune_generations(&dir, 5, Some(&m5));
        assert_eq!(again["pruned"], json!([]));
        fs::remove_dir_all(&dir).unwrap();
        fs::remove_dir_all(&other).unwrap();
    }
    #[test]
    fn prune_is_never_executed_for_ecbench_requests() {
        let req = crate::contract::fixture();
        let s = Status {
            request_sha256: req.hash(),
            attempt: 0,
            phase: "running".into(),
            pid: None,
            starttime: None,
            generation: Some(5),
            exit_code: None,
            error: None,
            updated: 0,
        };
        assert!(prune(&req, &s, 5, Some(&"d".repeat(64))).is_err());
        let op = Operation {
            request_sha256: req.hash(),
            attempt: 0,
            action: "prune".into(),
            generation: 5,
            manifest_sha256: None,
        };
        assert_eq!(op.id().unwrap(), format!("prune-{:020}", 5));
    }
}
