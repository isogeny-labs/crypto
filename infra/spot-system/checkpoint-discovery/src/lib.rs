//! Discovery never runs a workload or treats a filename as proof of recovery.
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Options {
    pub root: PathBuf,
    pub search_roots: Vec<PathBuf>,
    pub proc_root: PathBuf,
    pub job: Option<String>,
    pub max_entries: usize,
    pub max_hash_bytes: u64,
}

impl Options {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            search_roots: vec![],
            proc_root: "/proc".into(),
            job: None,
            max_entries: 100_000,
            max_hash_bytes: 1024 * 1024 * 1024,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    job_id: String,
    spec_sha256: String,
    generation: u64,
    committed: bool,
    files: Vec<CheckpointFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointFile {
    path: PathBuf,
    sha256: String,
    bytes: u64,
}

struct Scan {
    entries_left: usize,
    bytes_left: u64,
    issues: BTreeSet<String>,
    skipped_symlinks: usize,
}

fn safe_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn is_digest(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

// Reject symlinks in every component, not only the final file. O_NONBLOCK
// prevents a substituted FIFO from hanging discovery before fstat rejects it.
fn open_regular(path: &Path) -> Result<File, String> {
    let mut file =
        File::open(if path.is_absolute() { "/" } else { "." }).map_err(|e| e.to_string())?;
    let components: Vec<_> = path
        .components()
        .filter(|c| !matches!(c, Component::RootDir | Component::CurDir))
        .collect();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(name) = component else {
            return Err("parent traversal".into());
        };
        use std::os::unix::ffi::OsStrExt;
        let name = CString::new(name.as_bytes()).map_err(|_| "invalid path")?;
        let flags = libc::O_RDONLY
            | libc::O_NOFOLLOW
            | libc::O_NONBLOCK
            | libc::O_CLOEXEC
            | if index + 1 < components.len() {
                libc::O_DIRECTORY
            } else {
                0
            };
        let fd = unsafe { libc::openat(file.as_raw_fd(), name.as_ptr(), flags) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }
        file = unsafe { File::from_raw_fd(fd) };
    }
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("not a regular file".into());
    }
    Ok(file)
}

fn read_json(path: &Path) -> Result<Value, String> {
    let file = open_regular(path)?;
    let mut bytes = Vec::new();
    file.take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > 1024 * 1024 {
        return Err("JSON exceeds 1 MiB limit".into());
    }
    serde_json::from_slice(&bytes).map_err(|_| "invalid JSON".into())
}

fn manifest_name(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|s| s.to_str()),
        Some("checkpoint-manifest.json")
    )
}

fn candidate_name(path: &Path) -> bool {
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    name.ends_with(".ckpt")
        || name.ends_with(".checkpoint")
        || name == "checkpoint.json"
        || name == "checkpoint-manifest.json"
        || name.ends_with(".ckpt.json")
}

impl Scan {
    fn issue(&mut self, path: &Path, why: &str) {
        self.issues.insert(format!("{}: {why}", path.display()));
    }

    fn walk(&mut self, path: &Path, depth: usize, files: &mut Vec<PathBuf>, checkpoint: bool) {
        if self.entries_left == 0 || depth > 24 {
            self.issue(path, "discovery entry/depth limit reached");
            return;
        }
        self.entries_left -= 1;
        let meta = match fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => {
                self.issue(path, &e.to_string());
                return;
            }
        };
        if meta.file_type().is_symlink() {
            self.skipped_symlinks += 1;
            if checkpoint {
                self.issue(path, "symlink inside checkpoint tree");
            }
            return;
        }
        if meta.is_file() {
            files.push(path.to_path_buf());
            return;
        }
        if !meta.is_dir() {
            self.issue(path, "special file excluded");
            return;
        }
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name();
                            let name = name.to_string_lossy();
                            if !checkpoint
                                && (name.starts_with('.')
                                    || matches!(
                                        name.as_ref(),
                                        "target"
                                            | "node_modules"
                                            | "nodes"
                                            | "ssh-host-keys"
                                            | "__pycache__"
                                    ))
                            {
                                continue;
                            }
                            if self.entries_left == 0 {
                                self.issue(path, "discovery entry limit reached");
                                break;
                            }
                            self.walk(&entry.path(), depth + 1, files, checkpoint);
                        }
                        Err(e) => self.issue(path, &e.to_string()),
                    }
                }
            }
            Err(e) => self.issue(path, &e.to_string()),
        }
    }

    fn verify(&mut self, path: &Path, job_id: &str, fingerprint: &str) -> Result<u64, String> {
        let manifest: Manifest = serde_json::from_value(read_json(path)?)
            .map_err(|_| "invalid checkpoint manifest schema".to_string())?;
        if manifest.schema != "spot.checkpoint/v1" || !manifest.committed {
            return Err("not a committed spot.checkpoint/v1 generation".into());
        }
        if manifest.job_id != job_id
            || manifest.spec_sha256 != fingerprint
            || !is_digest(fingerprint)
        {
            return Err("job/spec identity mismatch".into());
        }
        if manifest.files.is_empty() {
            return Err("empty file inventory".into());
        }
        let mut seen = BTreeSet::new();
        for item in manifest.files {
            if item.path.as_os_str().is_empty()
                || item
                    .path
                    .components()
                    .any(|c| !matches!(c, Component::Normal(_)))
                || !seen.insert(item.path.clone())
                || !is_digest(&item.sha256)
                || manifest_name(&item.path)
            {
                return Err("unsafe, duplicate or invalid file entry".into());
            }
            let mut file = open_regular(&path.parent().unwrap().join(&item.path))?;
            let before = file.metadata().map_err(|e| e.to_string())?;
            if before.len() != item.bytes {
                return Err("file length mismatch".into());
            }
            if item.bytes > self.bytes_left {
                self.issue(path, "hash byte limit reached");
                return Err("hash byte limit reached".into());
            }
            self.bytes_left -= item.bytes;
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 64 * 1024];
            let mut count = 0;
            loop {
                let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
                if n == 0 {
                    break;
                }
                count += n as u64;
                if count > item.bytes {
                    return Err("checkpoint changed during discovery".into());
                }
                hasher.update(&buffer[..n]);
            }
            let after = file.metadata().map_err(|e| e.to_string())?;
            if count != item.bytes
                || before.mtime() != after.mtime()
                || before.mtime_nsec() != after.mtime_nsec()
                || before.ctime() != after.ctime()
                || before.ctime_nsec() != after.ctime_nsec()
            {
                return Err("checkpoint changed during discovery".into());
            }
            if format!("{:x}", hasher.finalize()) != item.sha256 {
                return Err("SHA-256 mismatch".into());
            }
        }
        Ok(manifest.generation)
    }

    fn job(&mut self, root: &Path, job: &Value) -> Value {
        let id = job["id"].as_str().unwrap_or("");
        if !safe_id(id) {
            return json!({"id": id, "status": "invalid", "reason": "unsafe job id"});
        }
        let argv_valid = job["argv"]
            .as_array()
            .is_some_and(|a| !a.is_empty() && a.iter().all(Value::is_string));
        let cwd_valid = job["cwd"].as_str().is_some_and(|p| {
            root.join(p)
                .canonicalize()
                .is_ok_and(|p| p.starts_with(root) && p.is_dir())
        });
        if !argv_valid
            || !cwd_valid
            || !matches!(
                job["checkpoint_contract"].as_str(),
                Some("atomic-files-and-sigterm-v1" | "manifest-sha256-v1")
            )
        {
            return json!({"id": id, "status": "invalid", "reason": "invalid job or checkpoint contract"});
        }
        let state = root.join("state").join(id);
        let checkpoint = state.join("checkpoint");
        let mut files = vec![];
        if checkpoint.symlink_metadata().is_ok() {
            self.walk(&checkpoint, 0, &mut files, true);
        }
        let mut result = json!({"id": id, "status": "fresh", "selected": null,
            "generations": [], "checkpoint_files": files.len()});
        let spec = read_json(&state.join("spec.json"));
        let fingerprint = match spec {
            Ok(ref value)
                if value["spec"] == *job && value["sha256"].as_str().is_some_and(is_digest) =>
            {
                value["sha256"].as_str().unwrap().to_owned()
            }
            _ if !state.exists() => return result,
            _ => {
                result["status"] = json!("invalid");
                result["reason"] = json!("missing or mismatched spec.json");
                return result;
            }
        };
        if state.join("complete.json").exists() {
            match read_json(&state.join("complete.json")) {
                Ok(v) if v["sha256"] == fingerprint => {
                    result["status"] = json!("complete");
                    return result;
                }
                _ => {
                    result["status"] = json!("invalid");
                    result["reason"] = json!("invalid completion marker");
                    return result;
                }
            }
        }
        let mut generations = vec![];
        let mut valid = vec![];
        for path in files.iter().filter(|p| manifest_name(p)) {
            match self.verify(path, id, &fingerprint) {
                Ok(generation) => {
                    valid.push((generation, path.clone()));
                    generations
                        .push(json!({"path": path, "generation": generation, "valid": true}));
                }
                Err(reason) => {
                    generations.push(json!({"path": path, "valid": false, "reason": reason}))
                }
            }
        }
        valid.sort_by(|a, b| b.cmp(a));
        generations.sort_by_key(|v| v["path"].as_str().unwrap_or("").to_owned());
        result["generations"] = json!(generations);
        if valid.len() > 1 && valid[0].0 == valid[1].0 {
            result["status"] = json!("invalid");
            result["reason"] = json!("ambiguous duplicate generation");
        } else if let Some((generation, path)) = valid.first() {
            result["status"] = json!("validated");
            result["selected"] = json!({"manifest": path, "generation": generation,
                "modified_unix": fs::metadata(path).ok().map(|m| m.mtime())});
        } else if !files.is_empty() || state.join("attempt.json").symlink_metadata().is_ok() {
            result["status"] = json!(if generations.is_empty() {
                "unverified"
            } else {
                "invalid"
            });
            result["reason"] =
                json!("previous state/attempt has no integrity-validated committed generation; restart from zero prohibited");
        }
        result
    }
}

fn process_inventory(options: &Options, scan: &mut Scan, roots: &[PathBuf]) -> Vec<Value> {
    let mut results = vec![];
    let entries = match fs::read_dir(&options.proc_root) {
        Ok(e) => e,
        Err(e) => {
            scan.issue(&options.proc_root, &e.to_string());
            return results;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                scan.issue(&options.proc_root, "unreadable process entry");
                continue;
            }
        };
        let pid: u32 = match entry.file_name().to_string_lossy().parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        if pid == std::process::id() {
            continue;
        }
        let path = entry.path();
        let meta = match fs::metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        // Never inspect another user's cmdline or environment. In particular,
        // /proc/PID/environ is never read, copied or included in the report.
        if meta.uid() != unsafe { libc::geteuid() } {
            continue;
        }
        let exe = match fs::read_link(path.join("exe")) {
            Ok(p) => p,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                scan.issue(&path, &e.to_string());
                continue;
            }
        };
        let cwd = match fs::read_link(path.join("cwd")) {
            Ok(p) => p,
            Err(_) => {
                scan.issue(&path, "unreadable cwd");
                continue;
            }
        };
        let output = fs::read_link(path.join("fd/1"))
            .ok()
            .filter(|p| roots.iter().any(|r| p.starts_with(r)));
        // Interpreted experiments use a system Python/shell executable. Their
        // workspace cwd plus a workspace stdout file still exposes the run
        // without inspecting the script, environment or output contents.
        if !roots.iter().any(|r| exe.starts_with(r))
            && !(roots.iter().any(|r| cwd.starts_with(r)) && output.is_some())
        {
            continue;
        }
        let command = match fs::read(path.join("cmdline")) {
            Ok(b) => b,
            Err(_) => {
                scan.issue(&path, "unreadable cmdline");
                continue;
            }
        };
        let args: Vec<_> = command
            .split(|b| *b == 0)
            .filter(|a| !a.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect();
        let mut hints = BTreeSet::new();
        for pair in args.windows(2) {
            if matches!(
                pair[0].as_str(),
                "--checkpoint" | "--checkpoint-dir" | "--resume-from"
            ) {
                let candidate = cwd.join(&pair[1]);
                if roots.iter().any(|r| candidate.starts_with(r)) {
                    hints.insert(candidate);
                }
            }
            if exe.file_name().is_some_and(|n| n == "joux_vitse_edwards") && pair[0] == "--json" {
                hints.insert(cwd.join(format!("{}.ckpt", pair[1])));
            }
            if exe.file_name().is_some_and(|n| n == "rr_degree_ladder") && pair[0] == "--out" {
                hints.insert(cwd.join(&pair[1]));
            }
        }
        let capability = if exe
            .file_name()
            .is_some_and(|n| n == "factor_base_shape_ladder")
        {
            "no-checkpoint-interface"
        } else if !hints.is_empty() {
            "native-resume-candidate-unverified"
        } else {
            "unknown"
        };
        // Deliberately omit command text and file contents; paths are inventory,
        // not a replay command. No automatically inferred command is executed.
        results.push(json!({"pid": pid, "executable": exe, "cwd": cwd, "stdout_path": output,
            "capability": capability, "checkpoint_hints": hints, "command_sha256": hex_digest(&command)}));
    }
    results.sort_by_key(|v| v["pid"].as_u64().unwrap_or(0));
    results
}

pub fn discover(options: &Options) -> Result<Value, String> {
    let root = options.root.canonicalize().map_err(|e| e.to_string())?;
    let mut scan = Scan {
        entries_left: options.max_entries,
        bytes_left: options.max_hash_bytes,
        issues: BTreeSet::new(),
        skipped_symlinks: 0,
    };
    let mut jobs = vec![];
    match read_json(&root.join("jobs.json")) {
        Ok(Value::Array(entries)) => {
            let mut ids = BTreeSet::new();
            for job in entries {
                let id = job["id"].as_str().unwrap_or("");
                if options.job.as_ref().is_some_and(|wanted| wanted != id) {
                    continue;
                }
                if !ids.insert(id.to_owned()) {
                    scan.issue(&root.join("jobs.json"), "duplicate job id");
                }
                jobs.push(scan.job(&root, &job));
            }
        }
        _ => scan.issue(
            &root.join("jobs.json"),
            "missing, unreadable or invalid jobs manifest",
        ),
    }
    if options.job.is_some() && jobs.len() != 1 {
        scan.issue(&root.join("jobs.json"), "requested job not uniquely found");
    }
    let mut roots = BTreeSet::from([root.clone()]);
    let mut artifacts = vec![];
    let mut processes = vec![];
    if options.job.is_none() {
        for path in &options.search_roots {
            match path.canonicalize() {
                Ok(p) => {
                    roots.insert(p);
                }
                Err(e) => scan.issue(path, &e.to_string()),
            }
        }
        processes = process_inventory(
            options,
            &mut scan,
            &roots.iter().cloned().collect::<Vec<_>>(),
        );
        let mut priorities = vec![root.clone()];
        let mut scheduled = BTreeSet::from([root.clone()]);
        for process in &processes {
            if let Some(cwd) = process["cwd"].as_str() {
                if let Ok(path) = Path::new(cwd).canonicalize() {
                    if roots.iter().any(|r| path.starts_with(r)) && scheduled.insert(path.clone()) {
                        priorities.push(path);
                    }
                }
            }
        }
        for path in roots {
            if scheduled.insert(path.clone()) {
                priorities.push(path);
            }
        }
        let mut files = vec![];
        for path in &priorities {
            scan.walk(path, 0, &mut files, false);
        }
        let mut seen = BTreeSet::new();
        for path in files.into_iter().filter(|p| candidate_name(p)) {
            if !seen.insert(path.clone()) {
                continue;
            }
            artifacts.push(json!({"path": path, "status": "candidate-only",
                "under_configured_root": path.starts_with(&root), "bytes": fs::metadata(&path).ok().map(|m| m.len())}));
        }
        artifacts.sort_by_key(|v| v["path"].as_str().unwrap_or("").to_owned());
    }
    Ok(
        json!({"schema": "spot.checkpoint-discovery/v1", "root": root,
        "observed_unix": SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        "coverage_complete": scan.issues.is_empty(), "issues": scan.issues,
        "skipped_symlinks": scan.skipped_symlinks, "jobs": jobs, "processes": processes, "artifacts": artifacts}),
    )
}

pub fn admission_allowed(report: &Value) -> bool {
    report["coverage_complete"] == true
        && report["jobs"].as_array().is_some_and(|jobs| {
            jobs.len() == 1
                && matches!(
                    jobs[0]["status"].as_str(),
                    Some("validated" | "fresh" | "complete")
                )
        })
}

pub fn write_report(path: &Path, report: &Value) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let temp = parent.join(format!(
        ".checkpoint-discovery-{}-{}.tmp",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temp)
            .map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(&mut file, report).map_err(|e| e.to_string())?;
        file.write_all(b"\n").map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        fs::rename(&temp, path).map_err(|e| e.to_string())?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    })();
    if result.is_err() {
        let _ = fs::remove_file(temp);
    }
    result
}
