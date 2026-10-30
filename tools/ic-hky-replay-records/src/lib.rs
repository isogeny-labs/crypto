//! Native protocol, restore and record handling for the registered HKY replay.
//! Scientific equation/rank/verdict computation stays in the separately frozen auditor.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::fd::AsRawFd,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const STUDY: &str = "research/ic_algebraic_crossover_20261023";
pub const GLOBAL_LOCK: &str = "/tmp/crypto-bench.lock";
pub mod records;

pub fn require(condition: bool, message: impl Into<String>) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into().into())
    }
}

pub fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub fn sha_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub fn sha_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 65_536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hex::encode(hash.finalize()))
}

/// Sorted UTF-8 canonical JSON, matching experiment_protocol.canonical for this schema.
pub fn canonical(value: &Value) -> String {
    match value {
        Value::Array(values) => format!(
            "[{}]",
            values.iter().map(canonical).collect::<Vec<_>>().join(",")
        ),
        Value::Object(values) => {
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            format!(
                "{{{}}}",
                entries
                    .into_iter()
                    .map(|(k, v)| format!("{}:{}", serde_json::to_string(k).unwrap(), canonical(v)))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
        _ => serde_json::to_string(value).unwrap(),
    }
}

pub fn protocol_hash(protocol: &Value) -> Result<String> {
    let mut body = protocol
        .as_object()
        .ok_or("protocol is not an object")?
        .clone();
    body.remove("freeze");
    Ok(sha_bytes(canonical(&Value::Object(body)).as_bytes()))
}

pub fn preserve_scientific_contract(protocol: &Value, original: &Value) -> Result<()> {
    require(
        protocol_hash(original)?
            == "32fbd8c1e4d6b6d1f7dcde2cd06084a28551ad4954922621193846b862277f31",
        "original registered scientific protocol changed",
    )?;
    for index in 0..2 {
        for field in ["id", "role", "instrument"] {
            require(
                protocol["arms"][index][field] == original["arms"][index][field],
                format!("scientific arm contract changed: {index}/{field}"),
            )?;
        }
    }
    for field in [
        "frozen_binary_path",
        "input_sha256",
        "producer_receipt_sha256",
        "protocol_tools_commit",
    ] {
        require(
            protocol["extensions"][field] == original["extensions"][field],
            format!("original scientific binding changed: {field}"),
        )?;
    }
    require(
        protocol["analysis_plan"]["script"] == original["analysis_plan"]["script"],
        "scientific analysis script changed",
    )?;
    require(
        protocol["metrics"] == original["metrics"],
        "scientific metric definitions changed",
    )
}

pub fn now() -> Result<String> {
    let output = Command::new("date").args(["-u", "+%FT%TZ"]).output()?;
    require(output.status.success(), "UTC date command failed")?;
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

pub fn git_head(root: &Path) -> Result<String> {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()?;
    require(
        out.status.success(),
        "could not read repository source commit",
    )?;
    Ok(String::from_utf8(out.stdout)?.trim().to_owned())
}

pub fn relative_path(path: &str) -> Result<&Path> {
    let path = Path::new(path);
    require(
        !path.as_os_str().is_empty()
            && path.components().all(|c| matches!(c, Component::Normal(_))),
        "unsafe relative raw-record path",
    )?;
    require(
        !path.to_string_lossy().contains(['\n', '\r']),
        "newline in raw-record path",
    )?;
    Ok(path)
}

pub fn atomic_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let temporary = path.with_extension(format!("tmp-{}-{nonce}", std::process::id()));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&temporary, path)?;
    if let Some(parent) = path.parent() {
        File::open(parent)?.sync_all()?;
    }
    Ok(())
}

pub fn atomic_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    atomic_bytes(path, &bytes)
}

/// Publish a complete immutable JSON file without overwriting an existing path.
pub fn write_once_json(path: &Path, value: &Value) -> Result<()> {
    let parent = path
        .parent()
        .ok_or("immutable JSON has no parent directory")?;
    fs::create_dir_all(parent)?;
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let temporary = path.with_extension(format!("tmp-{}-{nonce}", std::process::id()));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    // Same-directory hard-link creation is atomic and fails if the destination exists.
    fs::hard_link(&temporary, path)?;
    File::open(parent)?.sync_all()?;
    fs::remove_file(temporary)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

pub fn validate_schema(value: &Value, schema: &Path) -> Result<()> {
    let schema = read_json(schema)?;
    let validator = jsonschema::validator_for(&schema)?;
    let errors: Vec<_> = validator
        .iter_errors(value)
        .take(12)
        .map(|e| format!("{}: {e}", e.instance_path))
        .collect();
    require(
        errors.is_empty(),
        format!("JSON schema validation failed: {}", errors.join("; ")),
    )
}

pub struct Lock {
    _file: File,
}
impl Lock {
    pub fn acquire(path: &Path, wait: bool) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;
        let flags = libc::LOCK_EX | if wait { 0 } else { libc::LOCK_NB };
        // SAFETY: the descriptor is owned by file and retained for this guard's lifetime.
        require(
            unsafe { libc::flock(file.as_raw_fd(), flags) } == 0,
            "another native replay or busy job owns the lock",
        )?;
        Ok(Self { _file: file })
    }
}

fn excluded(path: &str) -> bool {
    matches!(
        path,
        "SHA256SUMS" | ".batch.lock" | "checkpoint-transport.txt"
    ) || path.contains(".tmp-")
}

pub fn raw_files(root: &Path) -> Result<Vec<String>> {
    fn visit(root: &Path, dir: &Path, output: &mut Vec<String>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            let meta = fs::symlink_metadata(&path)?;
            require(
                !meta.file_type().is_symlink(),
                "raw-record symlinks are not permitted",
            )?;
            if meta.is_dir() {
                visit(root, &path, output)?;
            } else if meta.is_file() {
                let relative = path
                    .strip_prefix(root)?
                    .to_str()
                    .ok_or("non-UTF8 raw path")?
                    .to_owned();
                relative_path(&relative)?;
                if !excluded(&relative) {
                    output.push(relative);
                }
            }
        }
        Ok(())
    }
    let mut output = Vec::new();
    visit(root, root, &mut output)?;
    output.sort();
    Ok(output)
}

pub fn write_manifest(root: &Path) -> Result<()> {
    let mut text = String::new();
    for relative in raw_files(root)? {
        text.push_str(&format!(
            "{}  {relative}\n",
            sha_file(&root.join(&relative))?
        ));
    }
    require(!text.is_empty(), "refusing an empty replay manifest")?;
    atomic_bytes(&root.join("SHA256SUMS"), text.as_bytes())
}

pub fn verify_manifest(root: &Path) -> Result<()> {
    let mut seen = BTreeSet::new();
    let manifest = fs::read_to_string(root.join("SHA256SUMS"))?;
    require(!manifest.trim().is_empty(), "empty replay manifest")?;
    for line in manifest.lines() {
        let (digest, relative) = line.split_once("  ").ok_or("malformed manifest entry")?;
        require(
            digest.len() == 64 && digest.bytes().all(|c| c.is_ascii_hexdigit()),
            "invalid manifest hash",
        )?;
        let path = root.join(relative_path(relative)?);
        require(seen.insert(relative.to_owned()), "duplicate manifest path")?;
        require(
            fs::symlink_metadata(&path)?.is_file(),
            "manifest file is not a regular file",
        )?;
        require(
            sha_file(&path)? == digest,
            format!("restored hash mismatch: {relative}"),
        )?;
    }
    Ok(())
}

/// Crucially, verification precedes every overwrite, including process identity files.
pub fn restore_records(root: &Path, case_paths: &[PathBuf], pid: u32, pgid: i32) -> Result<()> {
    verify_manifest(root)?;
    for path in case_paths {
        if path.join("started.txt").exists() && !path.join("ended.txt").exists() {
            atomic_json(
                &path.join("status.json"),
                &json!({"exit_code":null,"interrupted_by_restore":true,"scope":"completion absent from restored generation; no retry"}),
            )?;
            atomic_bytes(&path.join("ended.txt"), format!("{}\n", now()?).as_bytes())?;
        }
    }
    atomic_bytes(&root.join("runner.pid"), format!("{pid}\n").as_bytes())?;
    atomic_bytes(&root.join("runner.pgid"), format!("{pgid}\n").as_bytes())?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Case {
    pub id: String,
    pub draw: u64,
    pub guesses: u64,
    pub branch: u64,
    pub ordinal: usize,
}

pub fn all_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for draw in [0, 3, 5, 6] {
        for guesses in 0..=4 {
            for branch in 0..1u64 << guesses {
                cases.push(Case {
                    id: format!("d{draw}-k{guesses}-b{branch}"),
                    draw,
                    guesses,
                    branch,
                    ordinal: cases.len(),
                });
            }
        }
    }
    cases
}

pub struct Context {
    pub root: PathBuf,
    pub study: PathBuf,
    pub protocol_path: PathBuf,
    pub protocol: Value,
    pub run: PathBuf,
    pub cases: Vec<Case>,
}

impl Context {
    pub fn open(root: &Path, protocol_path: &Path, frozen: bool) -> Result<Self> {
        let root = root.canonicalize()?;
        let study = root.join(STUDY);
        let protocol_path = protocol_path.canonicalize()?;
        require(
            protocol_path.starts_with(&study),
            "continuation protocol lies outside the study",
        )?;
        let protocol = read_json(&protocol_path)?;
        let native = &protocol["extensions"]["native_resume"];
        let schema = native["protocol_schema"]["path"]
            .as_str()
            .ok_or("missing native protocol schema path")?;
        require(
            sha_file(Path::new(schema))?
                == native["protocol_schema"]["sha256"]
                    .as_str()
                    .ok_or("missing protocol schema hash")?,
            "protocol schema changed",
        )?;
        validate_schema(&protocol, Path::new(schema))?;
        if frozen {
            require(
                protocol["freeze"]["status"] == "frozen"
                    && protocol["freeze"]["filed_before_affected_data"] == true,
                "prospective native continuation is not frozen",
            )?;
            require(
                protocol["freeze"]["protocol_sha256"] == protocol_hash(&protocol)?,
                "frozen protocol digest changed",
            )?;
        }
        let run = study.join(relative_path(
            native["run_directory"]
                .as_str()
                .ok_or("missing native run directory")?,
        )?);
        let instance_list = native["instances"]
            .as_array()
            .ok_or("missing native instance register")?;
        let ids: BTreeSet<_> = instance_list
            .iter()
            .map(|v| v.as_str().map(str::to_owned).ok_or("instance has no id"))
            .collect::<std::result::Result<_, _>>()?;
        let cases: Vec<_> = all_cases()
            .into_iter()
            .filter(|c| ids.contains(&c.id))
            .collect();
        require(
            ids.len() == 122
                && cases.len() == 122
                && !ids.contains("d0-k0-b0")
                && !ids.contains("d0-k1-b0"),
            "continuation must retain exactly the 122 not-yet-run tuples",
        )?;
        require(
            instance_list.len() == cases.len()
                && cases
                    .iter()
                    .zip(instance_list)
                    .all(|(case, id)| id == &case.id),
            "native instance order differs from the original tuple order",
        )?;
        require(
            protocol["sample_size"]["instances"] == 122
                && protocol["sample_size"]["replicates"] == 1
                && protocol["sample_size"]["warmups"] == 0,
            "native sample grid changed",
        )?;
        require(
            protocol["randomization"]["seed_derivation"]["method"] == "explicit_list"
                && protocol["randomization"]["seed_derivation"]["seeds"] == json!([0]),
            "native metadata sentinel seed changed",
        )?;
        require(
            protocol["arms"].as_array().map(Vec::len) == Some(2)
                && protocol["arms"][0]["id"] == "hky"
                && protocol["arms"][1]["id"] == "hky-aa",
            "native A/A arms changed",
        )?;
        require(
            protocol["arms"][0]["instrument"] == protocol["arms"][1]["instrument"],
            "A/A instruments differ",
        )?;
        require(
            native["expected_new_processes"] == 244,
            "native process register changed",
        )?;
        require(
            native["caps"]
                == json!({"address_space_bytes":8589934592u64,"per_child_seconds":900,"outer_seconds":250000,"threads":1,"hky_chunk":1024}),
            "native execution caps changed",
        )?;
        Ok(Self {
            root,
            study,
            protocol_path,
            protocol,
            run,
            cases,
        })
    }

    pub fn native(&self) -> &Value {
        &self.protocol["extensions"]["native_resume"]
    }

    pub fn guard(&self) -> Result<()> {
        let p = &self.protocol;
        require(
            protocol_hash(&read_json(&self.protocol_path)?)? == protocol_hash(p)?,
            "protocol body changed after it was loaded",
        )?;
        require(
            protocol_hash(p)? == p["freeze"]["protocol_sha256"],
            "protocol changed",
        )?;
        let extensions = &p["extensions"];
        require(
            sha_file(Path::new(
                extensions["frozen_binary_path"]
                    .as_str()
                    .ok_or("no scientific binary")?,
            ))? == p["arms"][0]["instrument"]["binary_sha256"],
            "scientific binary changed",
        )?;
        require(
            sha_file(&self.study.join("inputs/guess-ell8.jsonl"))? == extensions["input_sha256"],
            "scientific input changed",
        )?;
        require(
            sha_file(
                &self
                    .study
                    .join("pipeline-receipts/guess-completion-20261025-01/receipt.json"),
            )? == extensions["producer_receipt_sha256"],
            "producer receipt changed",
        )?;
        require(
            sha_file(&self.root.join("examples/ic_guess_completed_audit.rs"))?
                == p["analysis_plan"]["script"]["sha256"],
            "scientific analysis source changed",
        )?;
        require(
            git_head(
                &self
                    .root
                    .parent()
                    .ok_or("no workspace parent")?
                    .join("crypto-autoresearcher"),
            )? == extensions["protocol_tools_commit"],
            "canonical protocol source revision changed",
        )?;
        let native = self.native();
        require(
            sha_file(&std::env::current_exe()?)? == native["binary_sha256"],
            "native controller executable changed",
        )?;
        for (relative, expected) in native["source_sha256"]
            .as_object()
            .ok_or("missing native source pins")?
        {
            require(
                sha_file(&self.root.join(relative_path(relative)?))?
                    == expected.as_str().ok_or("invalid native source hash")?,
                format!("native orchestration source changed: {relative}"),
            )?;
        }
        let original = read_json(&self.study.join("protocol-hky-restart-20261026.json"))?;
        preserve_scientific_contract(p, &original)?;
        require(
            protocol_hash(&original)? == native["original_protocol_sha256"],
            "original protocol changed",
        )?;
        require(
            sha_file(
                &self.study.join(relative_path(
                    native["legacy_archive"]["path"]
                        .as_str()
                        .ok_or("missing legacy archive path")?,
                )?),
            )? == native["legacy_archive"]["sha256"],
            "legacy raw archive changed",
        )?;
        let schema = native["export_schema"]["path"]
            .as_str()
            .ok_or("missing export schema")?;
        require(
            sha_file(Path::new(schema))? == native["export_schema"]["sha256"],
            "export schema changed",
        )?;
        Ok(())
    }

    pub fn case_paths(&self) -> Vec<PathBuf> {
        self.cases
            .iter()
            .flat_map(|c| ["hky", "hky-aa"].map(|a| self.run.join(a).join(&c.id)))
            .collect()
    }

    pub fn checkpoint(&self) -> Result<()> {
        write_manifest(&self.run)?;
        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.run.join("checkpoint-transport.txt"))?;
        let status = Command::new("bash")
            .args(["scripts/checkpoint.sh", "gate"])
            .current_dir(&self.root)
            .env(
                "CHECKPOINT_TOKEN_FILE",
                "/workspace/.ic-private-checkpoint/upload-token",
            )
            .stdin(Stdio::null())
            .stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log))
            .status()?;
        require(
            status.success(),
            "official S3 checkpoint gate failed; no further case launched",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "ic-native-restore-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&p).unwrap();
        p
    }
    #[test]
    fn old_frozen_protocol_digest_matches_native_canonicalization() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../research/ic_algebraic_crossover_20261023/protocol-hky-restart-20261026.json",
        );
        assert_eq!(
            protocol_hash(&read_json(&path).unwrap()).unwrap(),
            "32fbd8c1e4d6b6d1f7dcde2cd06084a28551ad4954922621193846b862277f31"
        );
    }

    #[test]
    fn native_schema_validator_accepts_the_canonical_protocol_and_compiles_both_schemas() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        for name in ["experiment-protocol.v1.json", "experiment-export.v1.json"] {
            let schema =
                read_json(&workspace.join("crypto-autoresearcher/schemas").join(name)).unwrap();
            jsonschema::validator_for(&schema).unwrap();
        }
        let original = read_json(&Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../research/ic_algebraic_crossover_20261023/protocol-hky-restart-20261026.json",
        ))
        .unwrap();
        validate_schema(
            &original,
            &workspace.join("crypto-autoresearcher/schemas/experiment-protocol.v1.json"),
        )
        .unwrap();
    }

    #[test]
    fn prospective_subset_cannot_change_the_original_solver_inputs_or_metrics() {
        let original = read_json(&Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../research/ic_algebraic_crossover_20261023/protocol-hky-restart-20261026.json",
        ))
        .unwrap();
        let mut continuation = original.clone();
        continuation["sample_size"]["instances"] = json!(122);
        preserve_scientific_contract(&continuation, &original).unwrap();
        continuation["extensions"]["input_sha256"] = json!("0".repeat(64));
        assert!(preserve_scientific_contract(&continuation, &original).is_err());
        let mut continuation = original.clone();
        continuation["arms"][0]["instrument"]["binary_sha256"] = json!("0".repeat(64));
        assert!(preserve_scientific_contract(&continuation, &original).is_err());
        let mut continuation = original.clone();
        continuation["metrics"]["primary"][0]["unit"] = json!("invented-price");
        assert!(preserve_scientific_contract(&continuation, &original).is_err());
    }
    #[test]
    fn restore_verifies_old_process_metadata_before_overwriting_it() {
        let root = fixture();
        fs::write(root.join("runner.pid"), "17\n").unwrap();
        fs::write(root.join("runner.pgid"), "19\n").unwrap();
        let case = root.join("hky/d0-k2-b0");
        fs::create_dir_all(&case).unwrap();
        fs::write(case.join("started.txt"), "old-generation\n").unwrap();
        write_manifest(&root).unwrap();
        restore_records(&root, std::slice::from_ref(&case), 23, 29).unwrap();
        assert_eq!(fs::read_to_string(root.join("runner.pid")).unwrap(), "23\n");
        assert_eq!(
            fs::read_to_string(root.join("runner.pgid")).unwrap(),
            "29\n"
        );
        assert_eq!(
            read_json(&case.join("status.json")).unwrap()["interrupted_by_restore"],
            true
        );
        assert!(case.join("ended.txt").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn tampered_restore_does_not_write_any_new_process_or_completion_metadata() {
        let root = fixture();
        fs::write(root.join("runner.pid"), "17\n").unwrap();
        let case = root.join("hky/d0-k2-b0");
        fs::create_dir_all(&case).unwrap();
        fs::write(case.join("started.txt"), "old\n").unwrap();
        write_manifest(&root).unwrap();
        fs::write(root.join("runner.pid"), "corrupt\n").unwrap();
        assert!(restore_records(&root, std::slice::from_ref(&case), 23, 29).is_err());
        assert_eq!(
            fs::read_to_string(root.join("runner.pid")).unwrap(),
            "corrupt\n"
        );
        assert!(!root.join("runner.pgid").exists());
        assert!(!case.join("ended.txt").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn manifest_rejects_parent_paths_and_duplicate_entries() {
        let root = fixture();
        fs::write(root.join("a"), "public-fixture").unwrap();
        let digest = sha_file(&root.join("a")).unwrap();
        fs::write(root.join("SHA256SUMS"), format!("{digest}  ../a\n")).unwrap();
        assert!(verify_manifest(&root).is_err());
        fs::write(
            root.join("SHA256SUMS"),
            format!("{digest}  a\n{digest}  a\n"),
        )
        .unwrap();
        assert!(verify_manifest(&root).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn fixed_register_has_124_tuples_and_two_balanced_legacy_exclusions() {
        let all = all_cases();
        assert_eq!(all.len(), 124);
        assert_eq!(all[0].id, "d0-k0-b0");
        assert_eq!(all[1].id, "d0-k1-b0");
        assert_eq!(all[2].id, "d0-k1-b1");
        assert_eq!(all[2].ordinal % 2, 0);
        assert_eq!(all.iter().filter(|c| c.ordinal >= 2).count() * 2, 244);
    }
}
