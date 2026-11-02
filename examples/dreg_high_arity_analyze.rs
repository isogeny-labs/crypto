//! Exact classifier for `research/dreg_high_arity_20261101`.
//!
//! Usage:
//! `dreg_high_arity_analyze DRAW_DIR OUTPUT_ROOT [--require LOGICAL_NAME]`
//!
//! A required logical run exits 0 only for terminal valid evidence, 75 while
//! retryable/missing below the four-attempt cap, 76 for terminal unmeasured
//! evidence (including a valid status-zero row whose degree scan is
//! incomplete), and 65 for malformed, contradictory, or otherwise invalid
//! evidence.  Without
//! `--require`, the same precedence is applied across the complete campaign.
//!
//! This program interprets no clocks. It validates the committed draw prefix,
//! the historical identity replay, six satisfiable controls, and the twelve
//! registered UNSAT scans. Missing, interrupted, capped, or incomplete work is
//! `unmeasured`; a contradictory completed record is `invalid_record`. Only a
//! complete saturated degree-four scan can be `above_four`.

use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_DRAWS: u64 = 4096;

#[derive(Clone, Copy)]
struct Cell {
    m: u64,
    n: u64,
    ell: u64,
}

impl Cell {
    const fn new(m: u64, n: u64, ell: u64) -> Self {
        Self { m, n, ell }
    }

    fn id(self) -> String {
        format!("m{}-n{}-l{}", self.m, self.n, self.ell)
    }

    fn source_cell(self) -> String {
        format!("n{}l{}m{}", self.n, self.ell, self.m)
    }

    fn n_vars(self) -> u64 {
        (self.m - 2) * self.n + self.m * self.ell
    }

    fn draw_file(self, draw_dir: &Path) -> PathBuf {
        draw_dir.join(format!("{}.jsonl", self.id()))
    }
}

const CELLS: [Cell; 3] = [
    Cell::new(5, 11, 2),
    Cell::new(5, 13, 3),
    Cell::new(6, 11, 2),
];

#[derive(Clone)]
struct DrawRecord {
    draw: u64,
    solutions: u64,
    outcome: Value,
}

#[derive(Serialize)]
struct PanelSummary {
    cell: String,
    state: String,
    path: String,
    records: usize,
    unsat_draws: Vec<u64>,
    satisfiable_draws: Vec<u64>,
    execution_ref: Option<String>,
    execution_commit: Option<String>,
    errors: Vec<String>,
}

struct Panel {
    summary: PanelSummary,
    unsat: Vec<DrawRecord>,
    sat: Vec<DrawRecord>,
}

#[derive(Clone, Serialize)]
struct AttemptSummary {
    attempt: String,
    exit_status: Option<i32>,
    termination: Option<String>,
    execution_ref: Option<String>,
    execution_commit: Option<String>,
    checkpoint_restored: Option<bool>,
    checkpoint_generation: Option<u64>,
    state: String,
    reason: String,
    output: String,
    status: String,
    finished: String,
    metadata: String,
}

#[derive(Clone, Serialize)]
struct ProcessRow {
    run_id: String,
    kind: String,
    cell: String,
    draw: Option<u64>,
    classification: String,
    reason: String,
    selected_attempt: Option<String>,
    attempts: Vec<AttemptSummary>,
}

#[derive(Clone, Serialize)]
struct CellSummary {
    cell: String,
    classification: String,
    closure_by_four: usize,
    above_four: usize,
    unmeasured: usize,
    invalid: usize,
    controls_passed: usize,
}

#[derive(Serialize)]
struct PackageSummary {
    package_id: String,
    role: String,
    classification: String,
    reason: String,
    subprocesses: Vec<String>,
}

struct FrozenContext {
    protocol_path: PathBuf,
    binary_manifest_path: PathBuf,
    binary_manifest_sha256: String,
    protocol_sha256: String,
    source_commit: String,
    ladder_sha256: String,
    closure_sha256: String,
    analyzer_sha256: String,
    report_sha256: String,
    container_image_digest: String,
    rustc_version: String,
}

struct AttemptBindings {
    execution_ref: String,
    execution_commit: String,
    checkpoint_restored: bool,
    checkpoint_generation: Option<u64>,
}

#[derive(Serialize)]
struct ProvenanceSummary {
    state: String,
    protocol: Option<String>,
    binary_manifest: Option<String>,
    protocol_sha256: Option<String>,
    source_commit: Option<String>,
    binary_manifest_sha256: Option<String>,
    reason: String,
}

#[derive(Serialize)]
struct ResumeProvenanceSummary {
    state: String,
    records: usize,
    reason: String,
}

fn field_u64(v: &Value, key: &str) -> Option<u64> {
    v.get(key).and_then(Value::as_u64)
}

fn field_bool(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(Value::as_bool)
}

fn field_i64(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn parse_json_lines(path: &Path) -> Result<Vec<Value>, String> {
    require_regular_file(path)?;
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .enumerate()
        .map(|(i, line)| {
            if line.trim().is_empty() {
                return Err(format!(
                    "{}:{}: blank JSONL rows are not permitted",
                    path.display(),
                    i + 1
                ));
            }
            serde_json::from_str(line).map_err(|e| format!("{}:{}: {e}", path.display(), i + 1))
        })
        .collect()
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn require_regular_file(path: &Path) -> Result<fs::Metadata, String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if !metadata.file_type().is_file() {
        return Err(format!(
            "{} is not a regular non-symlink file",
            path.display()
        ));
    }
    Ok(metadata)
}

fn sha256_file(path: &Path) -> Result<String, String> {
    require_regular_file(path)?;
    let mut file = fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    let digest = digest.finalize();
    Ok(format!("{digest:x}"))
}

fn safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|part| matches!(part, std::path::Component::Normal(_)))
}

fn manifest_rows(path: &Path) -> Result<Vec<(String, String)>, String> {
    require_regular_file(path)?;
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            return Err(format!(
                "{}:{}: blank manifest rows are not permitted",
                path.display(),
                index + 1
            ));
        }
        let Some((hash, relative)) = line.split_once("  ") else {
            return Err(format!(
                "{}:{}: expected 'SHA256  relative/path'",
                path.display(),
                index + 1
            ));
        };
        if !is_lower_hex(hash, 64) || !safe_relative_path(relative) {
            return Err(format!(
                "{}:{}: malformed hash or unsafe relative path",
                path.display(),
                index + 1
            ));
        }
        if !seen.insert(relative.to_string()) {
            return Err(format!(
                "{}:{}: duplicate manifest path {relative:?}",
                path.display(),
                index + 1
            ));
        }
        rows.push((hash.to_string(), relative.to_string()));
    }
    Ok(rows)
}

fn validate_manifest_descriptor(
    descriptor: &Value,
    output_root: &Path,
    expected_path: &str,
    replay: bool,
) -> Result<usize, String> {
    let object = descriptor
        .as_object()
        .ok_or_else(|| format!("manifest descriptor {expected_path:?} is not an object"))?;
    let exact_keys: BTreeSet<_> = ["path", "sha256", "file_count"].into_iter().collect();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != exact_keys {
        return Err(format!(
            "manifest descriptor {expected_path:?} has missing or unexpected fields"
        ));
    }
    if descriptor.get("path").and_then(Value::as_str) != Some(expected_path) {
        return Err(format!(
            "manifest descriptor path is not the registered {expected_path:?}"
        ));
    }
    let expected_hash = descriptor
        .get("sha256")
        .and_then(Value::as_str)
        .filter(|hash| is_lower_hex(hash, 64))
        .ok_or_else(|| format!("manifest descriptor {expected_path:?} has a malformed SHA-256"))?;
    let expected_count = descriptor
        .get("file_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("manifest descriptor {expected_path:?} has no file_count"))?;
    let path = output_root.join(expected_path);
    if sha256_file(&path)? != expected_hash {
        return Err(format!(
            "manifest file {expected_path:?} differs from its descriptor SHA-256"
        ));
    }
    let rows = manifest_rows(&path)?;
    if u64::try_from(rows.len()).ok() != Some(expected_count) {
        return Err(format!(
            "manifest file {expected_path:?} has {} rows, expected {expected_count}",
            rows.len()
        ));
    }
    if replay {
        for (expected, relative) in &rows {
            let artifact = output_root.join(relative);
            let got = sha256_file(&artifact)
                .map_err(|e| format!("immutable resumed artifact {relative:?}: {e}"))?;
            if &got != expected {
                return Err(format!(
                    "immutable resumed artifact {relative:?} differs from its import hash"
                ));
            }
        }
    }
    Ok(rows.len())
}

fn validate_resume_imports(output_root: &Path) -> Result<usize, String> {
    let entries = match fs::read_dir(output_root) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(format!("{}: {e}", output_root.display())),
    };
    let mut records = Vec::new();
    let mut observed_artifacts = BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("cannot enumerate resume provenance: {e}"))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("resume-import-") {
            require_regular_file(&entry.path())
                .map_err(|e| format!("resume provenance {name:?} is invalid: {e}"))?;
            observed_artifacts.insert(name.clone());
            if name.ends_with(".json") {
                records.push((name, entry.path()));
            }
        }
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    let mut expected_artifacts = BTreeSet::new();
    for (name, path) in &records {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let record: Value =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let object = record
            .as_object()
            .ok_or_else(|| format!("resume record {name:?} is not one object"))?;
        let exact_keys: BTreeSet<_> = [
            "schema",
            "source",
            "destination",
            "imported_at",
            "copy_manifest",
            "immutable_manifest",
            "anchor_manifest",
        ]
        .into_iter()
        .collect();
        if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != exact_keys {
            return Err(format!(
                "resume record {name:?} has missing or unexpected fields"
            ));
        }
        if record.get("schema").and_then(Value::as_str) != Some("dreg.resume-import/v3")
            || record
                .get("source")
                .and_then(Value::as_str)
                .is_none_or(|value| value.is_empty() || !Path::new(value).is_absolute())
            || record
                .get("imported_at")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
        {
            return Err(format!(
                "resume record {name:?} has invalid identity fields"
            ));
        }
        let destination = record
            .get("destination")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("resume record {name:?} has no destination"))?;
        // Historical import records survive later relocation. Their original
        // destination is provenance, not a claim that it is still the active
        // output root; manifest hashes provide the portable custody binding.
        if !Path::new(destination).is_absolute() {
            return Err(format!(
                "resume record {name:?} has a non-absolute destination"
            ));
        }
        let stem = name
            .strip_suffix(".json")
            .ok_or_else(|| format!("resume record {name:?} has no JSON suffix"))?;
        let stamp = stem
            .strip_prefix("resume-import-")
            .ok_or_else(|| format!("resume record {name:?} has no registered prefix"))?;
        let stamp_bytes = stamp.as_bytes();
        if stamp_bytes.len() < 17
            || !stamp_bytes[..8].iter().all(u8::is_ascii_digit)
            || stamp_bytes[8] != b'T'
            || !stamp_bytes[9..15].iter().all(u8::is_ascii_digit)
            || !stamp_bytes[15..stamp_bytes.len() - 1]
                .iter()
                .all(u8::is_ascii_digit)
            || stamp_bytes[stamp_bytes.len() - 1] != b'Z'
        {
            return Err(format!(
                "resume record {name:?} does not have a canonical UTC stamp"
            ));
        }
        let copy_path = format!("{stem}.copy.sha256");
        let immutable_path = format!("{stem}.immutable.sha256");
        let anchor_path = format!("{stem}.anchor.sha256");
        expected_artifacts.extend([
            name.clone(),
            copy_path.clone(),
            immutable_path.clone(),
            anchor_path.clone(),
        ]);
        validate_manifest_descriptor(
            record
                .get("copy_manifest")
                .ok_or_else(|| format!("resume record {name:?} has no copy manifest"))?,
            output_root,
            &copy_path,
            false,
        )?;
        validate_manifest_descriptor(
            record
                .get("immutable_manifest")
                .ok_or_else(|| format!("resume record {name:?} has no immutable manifest"))?,
            output_root,
            &immutable_path,
            true,
        )?;
        validate_manifest_descriptor(
            record
                .get("anchor_manifest")
                .ok_or_else(|| format!("resume record {name:?} has no anchor manifest"))?,
            output_root,
            &anchor_path,
            true,
        )?;
        let immutable = fs::read(output_root.join(&immutable_path))
            .map_err(|e| format!("resume record {name:?} immutable manifest: {e}"))?;
        let anchor = fs::read(output_root.join(&anchor_path))
            .map_err(|e| format!("resume record {name:?} anchor manifest: {e}"))?;
        if immutable != anchor {
            return Err(format!(
                "resume record {name:?} external anchor differs from its immutable manifest"
            ));
        }
    }
    if observed_artifacts != expected_artifacts {
        let unexpected: Vec<_> = observed_artifacts
            .difference(&expected_artifacts)
            .cloned()
            .collect();
        let missing: Vec<_> = expected_artifacts
            .difference(&observed_artifacts)
            .cloned()
            .collect();
        return Err(format!(
            "resume provenance set has unexpected {unexpected:?} and missing {missing:?} files"
        ));
    }
    Ok(records.len())
}

fn is_lower_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn study_file(draw_dir: &Path, name: &str) -> PathBuf {
    let beside_draws = draw_dir.parent().map(|p| p.join(name)).unwrap_or_default();
    if beside_draws.is_file() {
        return beside_draws;
    }
    PathBuf::from("research/dreg_high_arity_20261101").join(name)
}

fn string_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {pointer}"))
}

fn validate_checkpoint_restore(meta: &Value) -> Result<(bool, Option<u64>), String> {
    let restore = meta
        .get("checkpoint_restore")
        .and_then(Value::as_object)
        .ok_or_else(|| "metadata checkpoint_restore is not an object".to_string())?;
    let exact_keys: BTreeSet<_> = ["restored", "generation", "manifest_sha256", "manifest_json"]
        .into_iter()
        .collect();
    if restore.keys().map(String::as_str).collect::<BTreeSet<_>>() != exact_keys {
        return Err("metadata checkpoint_restore has missing or unexpected fields".into());
    }
    match restore.get("restored").and_then(Value::as_bool) {
        Some(false)
            if restore.get("generation") == Some(&Value::Null)
                && restore.get("manifest_sha256") == Some(&Value::Null)
                && restore.get("manifest_json") == Some(&Value::Null) =>
        {
            Ok((false, None))
        }
        Some(false) => Err("non-restored metadata carries checkpoint provenance".into()),
        Some(true) => {
            let generation = restore
                .get("generation")
                .and_then(Value::as_u64)
                .filter(|generation| *generation >= 1)
                .ok_or_else(|| "restored checkpoint has no integer generation".to_string())?;
            let expected_hash = restore
                .get("manifest_sha256")
                .and_then(Value::as_str)
                .filter(|hash| is_lower_hex(hash, 64))
                .ok_or_else(|| "restored checkpoint has no valid manifest SHA-256".to_string())?;
            let manifest_text = restore
                .get("manifest_json")
                .and_then(Value::as_str)
                .filter(|text| !text.is_empty())
                .ok_or_else(|| "restored checkpoint has no embedded manifest JSON".to_string())?;
            if sha256_bytes(manifest_text.as_bytes()) != expected_hash {
                return Err("embedded checkpoint manifest differs from its SHA-256".into());
            }
            let manifest: Value = serde_json::from_str(manifest_text)
                .map_err(|e| format!("embedded checkpoint manifest is malformed: {e}"))?;
            let manifest_object = manifest
                .as_object()
                .ok_or_else(|| "embedded checkpoint manifest is not an object".to_string())?;
            let manifest_keys: BTreeSet<_> = [
                "schema",
                "job_id",
                "spec_sha256",
                "generation",
                "committed",
                "files",
            ]
            .into_iter()
            .collect();
            if manifest_object
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
                != manifest_keys
                || manifest.get("schema").and_then(Value::as_str) != Some("spot.checkpoint/v1")
                || manifest.get("job_id").and_then(Value::as_str) != Some("dreg-closure")
                || manifest.get("committed").and_then(Value::as_bool) != Some(true)
                || manifest.get("generation").and_then(Value::as_u64) != Some(generation)
                || manifest
                    .get("spec_sha256")
                    .and_then(Value::as_str)
                    .is_none_or(|hash| !is_lower_hex(hash, 64))
            {
                return Err(
                    "embedded checkpoint manifest is not the committed selected generation".into(),
                );
            }
            let files = manifest
                .get("files")
                .and_then(Value::as_array)
                .filter(|files| !files.is_empty())
                .ok_or_else(|| "embedded checkpoint manifest has no files".to_string())?;
            let mut file_paths = BTreeSet::new();
            for file in files {
                let object = file.as_object().ok_or_else(|| {
                    "embedded checkpoint manifest has a non-object file row".to_string()
                })?;
                let keys: BTreeSet<_> = ["path", "sha256", "bytes"].into_iter().collect();
                let path = file.get("path").and_then(Value::as_str).unwrap_or("");
                let mut path_bytes = path.bytes();
                if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != keys
                    || path_bytes
                        .next()
                        .is_none_or(|byte| !byte.is_ascii_alphanumeric())
                    || !path_bytes.all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-')
                    })
                    || path.contains("..")
                    || file
                        .get("sha256")
                        .and_then(Value::as_str)
                        .is_none_or(|hash| !is_lower_hex(hash, 64))
                    || file.get("bytes").and_then(Value::as_u64).is_none()
                {
                    return Err("embedded checkpoint manifest has an invalid file row".into());
                }
                if !file_paths.insert(path) {
                    return Err("embedded checkpoint manifest repeats a file path".into());
                }
            }
            if !file_paths.contains("catalog.json") {
                return Err(
                    "embedded checkpoint manifest must contain exactly one catalog.json".into(),
                );
            }
            Ok((true, Some(generation)))
        }
        None => Err("metadata checkpoint_restore.restored is not Boolean".into()),
    }
}

impl FrozenContext {
    fn load(draw_dir: &Path) -> Result<Self, String> {
        let protocol_path = fs::canonicalize(study_file(draw_dir, "protocol.json"))
            .map_err(|e| format!("resolve frozen protocol: {e}"))?;
        let binary_manifest_path = fs::canonicalize(study_file(draw_dir, "binary.sha256"))
            .map_err(|e| format!("resolve frozen binary manifest: {e}"))?;
        let protocol_text = fs::read_to_string(&protocol_path)
            .map_err(|e| format!("{}: {e}", protocol_path.display()))?;
        let protocol: Value = serde_json::from_str(&protocol_text)
            .map_err(|e| format!("{}: {e}", protocol_path.display()))?;
        if string_at(&protocol, "/freeze/status")? != "frozen" {
            return Err("protocol freeze status is not frozen".into());
        }
        if protocol
            .pointer("/freeze/filed_before_affected_data")
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err("protocol was not filed before affected data".into());
        }
        let protocol_sha256 = string_at(&protocol, "/freeze/protocol_sha256")?.to_string();
        let source_commit = string_at(&protocol, "/freeze/commit")?.to_string();
        if !is_lower_hex(&protocol_sha256, 64) || !is_lower_hex(&source_commit, 40) {
            return Err("protocol freeze hashes are malformed".into());
        }
        let mut body = protocol.clone();
        body.as_object_mut()
            .ok_or_else(|| "protocol is not an object".to_string())?
            .remove("freeze");
        let canonical = serde_json::to_vec(&body).map_err(|e| e.to_string())?;
        if sha256_bytes(&canonical) != protocol_sha256 {
            return Err("protocol body does not match its frozen SHA-256".into());
        }

        let arms = protocol
            .get("arms")
            .and_then(Value::as_array)
            .ok_or_else(|| "protocol arms are missing".to_string())?;
        if arms.len() != 5 {
            return Err(format!("protocol has {} arms, expected five", arms.len()));
        }
        let first = &arms[0];
        let closure_sha256 = string_at(first, "/instrument/binary_sha256")?.to_string();
        let ladder_sha256 = string_at(first, "/instrument/flags/ladder_binary_sha256")?.to_string();
        let analyzer_sha256 =
            string_at(first, "/instrument/flags/analyzer_binary_sha256")?.to_string();
        let report_sha256 = string_at(first, "/instrument/flags/report_binary_sha256")?.to_string();
        let ladder_source_sha256 =
            string_at(first, "/instrument/flags/ladder_source_sha256")?.to_string();
        let closure_source_sha256 =
            string_at(first, "/instrument/flags/closure_source_sha256")?.to_string();
        let analyzer_source_sha256 =
            string_at(first, "/instrument/flags/analyzer_source_sha256")?.to_string();
        let report_source_sha256 =
            string_at(first, "/instrument/flags/report_source_sha256")?.to_string();
        let container_image_digest =
            string_at(first, "/instrument/flags/container_image_digest")?.to_string();
        let rustc_version = string_at(first, "/instrument/flags/rustc_version")?.to_string();
        for hash in [
            &closure_sha256,
            &ladder_sha256,
            &analyzer_sha256,
            &report_sha256,
            &ladder_source_sha256,
            &closure_source_sha256,
            &analyzer_source_sha256,
            &report_source_sha256,
        ] {
            if !is_lower_hex(hash, 64) {
                return Err("protocol contains a malformed instrument SHA-256".into());
            }
        }
        let expected_env = json!({
            "F4_F2_MAX_ROWS": "50000000",
            "F4_F2_MAX_COLS": "50000000",
            "RAYON_NUM_THREADS": "4",
            "DREG_CKPT_SECONDS": "300"
        });
        for arm in arms {
            if string_at(arm, "/instrument/commit")? != source_commit
                || string_at(arm, "/instrument/binary_sha256")? != closure_sha256
                || string_at(arm, "/instrument/flags/ladder_binary_sha256")? != ladder_sha256
                || string_at(arm, "/instrument/flags/analyzer_binary_sha256")? != analyzer_sha256
                || string_at(arm, "/instrument/flags/report_binary_sha256")? != report_sha256
                || string_at(arm, "/instrument/flags/ladder_source_sha256")? != ladder_source_sha256
                || string_at(arm, "/instrument/flags/closure_source_sha256")?
                    != closure_source_sha256
                || string_at(arm, "/instrument/flags/analyzer_source_sha256")?
                    != analyzer_source_sha256
                || string_at(arm, "/instrument/flags/report_source_sha256")? != report_source_sha256
                || string_at(arm, "/instrument/flags/container_image_digest")?
                    != container_image_digest
                || string_at(arm, "/instrument/flags/rustc_version")? != rustc_version
            {
                return Err("protocol arms disagree on frozen instrument identity".into());
            }
            if arm.pointer("/instrument/env") != Some(&expected_env)
                || arm.pointer("/instrument/threads").and_then(Value::as_u64) != Some(4)
                || string_at(arm, "/instrument/flags/address_space_bytes")? != "32000000000"
            {
                return Err("protocol arm has unexpected environment or limits".into());
            }
            let expected_cols = if arm.get("role").and_then(Value::as_str) == Some("reference") {
                "155000"
            } else {
                "400000"
            };
            if string_at(arm, "/instrument/flags/max_cols")? != expected_cols {
                return Err("protocol arm has an unexpected column cap".into());
            }
        }
        if protocol
            .pointer("/execution/timeout_seconds")
            .and_then(Value::as_u64)
            != Some(86_400)
            || protocol
                .pointer("/execution/checkpoint/grace_seconds")
                .and_then(Value::as_u64)
                != Some(60)
        {
            return Err("protocol execution timeout/grace differs from the frozen limits".into());
        }

        let manifest_text = fs::read_to_string(&binary_manifest_path)
            .map_err(|e| format!("{}: {e}", binary_manifest_path.display()))?;
        let binary_manifest_sha256 = sha256_bytes(manifest_text.as_bytes());
        let mut manifest = BTreeMap::new();
        for (line_no, line) in manifest_text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() != 2 || !is_lower_hex(fields[0], 64) {
                return Err(format!(
                    "{}:{}: malformed binary manifest row",
                    binary_manifest_path.display(),
                    line_no + 1
                ));
            }
            if manifest.insert(fields[1], fields[0]).is_some() {
                return Err(format!("duplicate binary manifest label {}", fields[1]));
            }
        }
        for (label, expected) in [
            ("dreg_ladder", ladder_sha256.as_str()),
            ("dreg_closure", closure_sha256.as_str()),
            ("dreg_high_arity_analyze", analyzer_sha256.as_str()),
            ("dreg_high_arity_report", report_sha256.as_str()),
        ] {
            if manifest.get(label).copied() != Some(expected) {
                return Err(format!("binary manifest mismatch for {label}"));
            }
        }
        let current_exe = env::current_exe().map_err(|e| format!("current executable: {e}"))?;
        if sha256_file(&current_exe)? != analyzer_sha256 {
            return Err("running analyzer binary differs from the frozen manifest".into());
        }
        for (path, expected) in [
            ("examples/dreg_ladder.rs", ladder_source_sha256.as_str()),
            ("examples/dreg_closure.rs", closure_source_sha256.as_str()),
            (
                "examples/dreg_high_arity_analyze.rs",
                analyzer_source_sha256.as_str(),
            ),
            (
                "examples/dreg_high_arity_report.rs",
                report_source_sha256.as_str(),
            ),
        ] {
            if sha256_file(Path::new(path))? != expected {
                return Err(format!("{path} differs from the frozen protocol"));
            }
        }

        Ok(Self {
            protocol_path,
            binary_manifest_path,
            binary_manifest_sha256,
            protocol_sha256,
            source_commit,
            ladder_sha256,
            closure_sha256,
            analyzer_sha256,
            report_sha256,
            container_image_digest,
            rustc_version,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_attempt_meta(
        &self,
        meta_path: &Path,
        logical: &str,
        attempt: &str,
        expected_input: &Path,
        expected_draw: u64,
        expected_mode: &str,
        expected_max_cols: u64,
        _output_root: &Path,
    ) -> Result<AttemptBindings, String> {
        require_regular_file(meta_path)?;
        let text =
            fs::read_to_string(meta_path).map_err(|e| format!("{}: {e}", meta_path.display()))?;
        let meta: Value =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", meta_path.display()))?;
        if !meta.is_object() {
            return Err("attempt metadata is not one JSON object".into());
        }
        let expect_string = |pointer: &str, expected: &str| -> Result<(), String> {
            let got = meta.pointer(pointer).and_then(Value::as_str);
            if got == Some(expected) {
                Ok(())
            } else {
                Err(format!(
                    "metadata {pointer} is {got:?}, expected {expected:?}"
                ))
            }
        };
        expect_string("/schema", "dreg.high-arity-attempt/v1")?;
        expect_string("/logical_name", logical)?;
        expect_string("/attempt", attempt)?;
        expect_string("/protocol_sha256", &self.protocol_sha256)?;
        expect_string("/source_commit", &self.source_commit)?;
        expect_string("/binaries/ladder", &self.ladder_sha256)?;
        expect_string("/binaries/closure", &self.closure_sha256)?;
        expect_string("/binaries/analyzer", &self.analyzer_sha256)?;
        expect_string("/binaries/report", &self.report_sha256)?;
        expect_string(
            "/build/container_image_digest",
            &self.container_image_digest,
        )?;
        expect_string("/build/rustc_version", &self.rustc_version)?;
        expect_string("/mode", expected_mode)?;
        let execution_ref = meta
            .pointer("/execution/ref")
            .and_then(Value::as_str)
            .filter(|reference| reference.starts_with("refs/heads/"))
            .ok_or_else(|| "metadata has no valid pushed execution ref".to_string())?;
        let execution_commit = meta
            .pointer("/execution/commit")
            .and_then(Value::as_str)
            .filter(|commit| is_lower_hex(commit, 40))
            .ok_or_else(|| "metadata has no valid pushed execution commit".to_string())?;
        if meta
            .pointer("/execution")
            .and_then(Value::as_object)
            .is_none_or(|execution| {
                execution
                    .keys()
                    .map(String::as_str)
                    .collect::<BTreeSet<_>>()
                    != ["ref", "commit"].into_iter().collect()
            })
        {
            return Err("metadata has no valid pushed execution identity".into());
        }
        if meta.get("draw").and_then(Value::as_u64) != Some(expected_draw) {
            return Err("metadata draw differs from the logical draw".into());
        }
        if meta
            .get("parent_environment_cleared")
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err("metadata does not attest a cleared parent environment".into());
        }
        let expected_env = json!({
            "PATH": "/usr/bin:/bin",
            "F4_F2_MAX_ROWS": "50000000",
            "F4_F2_MAX_COLS": "50000000",
            "RAYON_NUM_THREADS": "4",
            "DREG_CKPT_SECONDS": "300"
        });
        if meta.get("sanitized_env") != Some(&expected_env) {
            return Err("metadata sanitized environment differs from the frozen allowlist".into());
        }
        let expected_limits = json!({
            "address_space_bytes": 32_000_000_000u64,
            "timeout_seconds": 86_400,
            "term_grace_seconds": 60,
            "max_cols": expected_max_cols,
            "degree_to": 4
        });
        if meta.get("limits") != Some(&expected_limits) {
            return Err("metadata limits differ from the registered limits".into());
        }

        let input_path_text = string_at(&meta, "/input/path")?;
        let input_path = Path::new(input_path_text);
        if !input_path.is_absolute() || input_path.file_name() != expected_input.file_name() {
            return Err("metadata input path differs from the registered input".into());
        }
        let input_sha = sha256_file(expected_input)?;
        expect_string("/input/sha256", &input_sha)?;

        let checkpoint_text = string_at(&meta, "/checkpoint_dir")?.to_string();
        let checkpoint = Path::new(&checkpoint_text);
        if !checkpoint.is_absolute()
            || checkpoint.file_name().and_then(|x| x.to_str()) != Some(logical)
            || checkpoint
                .parent()
                .and_then(Path::file_name)
                .and_then(|x| x.to_str())
                != Some("checkpoints")
        {
            return Err(
                "metadata checkpoint_dir is not an absolute checkpoints/<logical> path".into(),
            );
        }
        let satisfiable = expected_mode == "sat";
        let mut argv = Vec::new();
        if satisfiable {
            argv.push(Value::String("--satisfiable".into()));
        }
        for value in [
            "--to".to_string(),
            "4".to_string(),
            "--max-cols".to_string(),
            expected_max_cols.to_string(),
            "--draw".to_string(),
            expected_draw.to_string(),
            "--checkpoint-dir".to_string(),
            checkpoint_text,
            input_path_text.to_string(),
        ] {
            argv.push(Value::String(value));
        }
        if meta.get("argv").and_then(Value::as_array) != Some(&argv) {
            return Err("metadata argv differs from the registered invocation".into());
        }
        let (checkpoint_restored, checkpoint_generation) = validate_checkpoint_restore(&meta)?;
        Ok(AttemptBindings {
            execution_ref: execution_ref.to_string(),
            execution_commit: execution_commit.to_string(),
            checkpoint_restored,
            checkpoint_generation,
        })
    }
}

fn validate_draw_outcome(v: &Value, solutions: u64) -> Result<(), String> {
    let outcome = v
        .get("outcome")
        .and_then(Value::as_object)
        .ok_or_else(|| "outcome is not an object".to_string())?;
    match outcome.get("kind").and_then(Value::as_str) {
        Some("satisfiable") if solutions > 0 && outcome.len() == 1 => Ok(()),
        Some("satisfiable") => {
            Err("satisfiable outcome must have a positive solution count and no extra fields".into())
        }
        Some("at_least")
            if solutions == 0
                && outcome.get("degree").and_then(Value::as_u64) == Some(4)
                && outcome.len() == 2 =>
        {
            Ok(())
        }
        Some("resolved")
            if solutions == 0
                && outcome
                    .get("degree")
                    .and_then(Value::as_u64)
                    .is_some_and(|d| (1..=3).contains(&d))
                && outcome.get("refuted").and_then(Value::as_bool).is_some()
                && outcome.len() == 3 =>
        {
            Ok(())
        }
        Some("caps_hit")
            if solutions == 0
                && outcome.len() == 2
                && match outcome.get("built") {
                    Some(Value::Null) => true,
                    Some(built) => built
                        .as_u64()
                        .is_some_and(|degree| (1..=2).contains(&degree)),
                    None => false,
                } =>
        {
            Ok(())
        }
        Some(kind) => Err(format!(
            "outcome kind {kind:?} is inconsistent with the exact solution count or registered degree-3 draw scan"
        )),
        None => Err("outcome.kind is missing".into()),
    }
}

fn validate_generation_sidecar(
    cell: Cell,
    draw_path: &Path,
    record_count: usize,
    frozen: &Result<FrozenContext, String>,
) -> Result<(String, String), String> {
    let context = frozen
        .as_ref()
        .map_err(|e| format!("frozen provenance is unavailable: {e}"))?;
    let sidecar_path = draw_path.with_extension("generation.json");
    let log_path = draw_path.with_extension("log");
    require_regular_file(&sidecar_path)?;
    require_regular_file(&log_path)?;
    let text = fs::read_to_string(&sidecar_path)
        .map_err(|e| format!("{}: {e}", sidecar_path.display()))?;
    let sidecar: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", sidecar_path.display()))?;
    if !sidecar.is_object() {
        return Err("generation sidecar is not one object".into());
    }
    let cell_id = cell.id();
    for (pointer, expected) in [
        ("/schema", "dreg.high-arity-generation/v1"),
        ("/cell", cell_id.as_str()),
        ("/protocol_sha256", context.protocol_sha256.as_str()),
        ("/source_commit", context.source_commit.as_str()),
        ("/binary_sha256", context.ladder_sha256.as_str()),
        (
            "/build/container_image_digest",
            context.container_image_digest.as_str(),
        ),
        ("/build/rustc_version", context.rustc_version.as_str()),
    ] {
        if sidecar.pointer(pointer).and_then(Value::as_str) != Some(expected) {
            return Err(format!(
                "generation sidecar {pointer} differs from frozen provenance"
            ));
        }
    }
    let execution_ref = sidecar
        .pointer("/execution/ref")
        .and_then(Value::as_str)
        .filter(|reference| reference.starts_with("refs/heads/"))
        .ok_or_else(|| "generation sidecar has no valid pushed execution ref".to_string())?;
    let execution_commit = sidecar
        .pointer("/execution/commit")
        .and_then(Value::as_str)
        .filter(|commit| is_lower_hex(commit, 40))
        .ok_or_else(|| "generation sidecar has no valid pushed execution commit".to_string())?;
    if sidecar
        .pointer("/execution")
        .and_then(Value::as_object)
        .is_none_or(|execution| {
            execution
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
                != ["ref", "commit"].into_iter().collect()
        })
    {
        return Err("generation sidecar has malformed pushed execution identity".into());
    }
    let expected_env = json!({
        "PATH": "/usr/bin:/bin",
        "F4_F2_MAX_ROWS": "50000000",
        "F4_F2_MAX_COLS": "50000000",
        "RAYON_NUM_THREADS": "4",
        "DREG_CKPT_SECONDS": "300"
    });
    if sidecar
        .get("parent_environment_cleared")
        .and_then(Value::as_bool)
        != Some(true)
        || sidecar.get("sanitized_env") != Some(&expected_env)
    {
        return Err("generation sidecar does not bind the frozen environment".into());
    }

    let draw_name = draw_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "draw path has no UTF-8 basename".to_string())?;
    let log_name = log_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "generator log has no UTF-8 basename".to_string())?;
    let draw_sha = sha256_file(draw_path)?;
    let log_sha = sha256_file(&log_path)?;
    let draw_bytes = fs::metadata(draw_path)
        .map_err(|e| format!("{}: {e}", draw_path.display()))?
        .len();
    let log_bytes = fs::metadata(&log_path)
        .map_err(|e| format!("{}: {e}", log_path.display()))?
        .len();
    let record_count = u64::try_from(record_count)
        .map_err(|_| "draw record count does not fit in u64".to_string())?;
    for (pointer, expected) in [
        ("/draw_jsonl/path", draw_name),
        ("/draw_jsonl/sha256", draw_sha.as_str()),
        ("/generator_log/path", log_name),
        ("/generator_log/sha256", log_sha.as_str()),
    ] {
        if sidecar.pointer(pointer).and_then(Value::as_str) != Some(expected) {
            return Err(format!(
                "generation sidecar {pointer} differs from the committed file"
            ));
        }
    }
    for (pointer, expected) in [
        ("/draw_jsonl/bytes", draw_bytes),
        ("/draw_jsonl/records", record_count),
        ("/generator_log/bytes", log_bytes),
    ] {
        if sidecar.pointer(pointer).and_then(Value::as_u64) != Some(expected) {
            return Err(format!(
                "generation sidecar {pointer} differs from the committed file"
            ));
        }
    }

    let audit = sidecar
        .get("replay_audit")
        .and_then(Value::as_object)
        .ok_or_else(|| "generation sidecar has no replay_audit object".to_string())?;
    let audit_keys: BTreeSet<_> = [
        "result",
        "expect_m",
        "expect_n",
        "expect_lambda",
        "expect_seed",
        "expect_count",
        "expect_d_max",
        "expect_ffd_max",
        "ignored_value_fields",
    ]
    .into_iter()
    .collect();
    if audit.keys().map(String::as_str).collect::<BTreeSet<_>>() != audit_keys
        || sidecar
            .pointer("/replay_audit/result")
            .and_then(Value::as_str)
            != Some("passed")
        || sidecar
            .pointer("/replay_audit/expect_m")
            .and_then(Value::as_u64)
            != Some(cell.m)
        || sidecar
            .pointer("/replay_audit/expect_n")
            .and_then(Value::as_u64)
            != Some(cell.n)
        || sidecar
            .pointer("/replay_audit/expect_lambda")
            .and_then(Value::as_u64)
            != Some(cell.ell)
        || sidecar
            .pointer("/replay_audit/expect_seed")
            .and_then(Value::as_u64)
            != Some(20_261_101)
        || sidecar
            .pointer("/replay_audit/expect_count")
            .and_then(Value::as_u64)
            != Some(record_count)
        || sidecar
            .pointer("/replay_audit/expect_d_max")
            .and_then(Value::as_u64)
            != Some(3)
        || sidecar
            .pointer("/replay_audit/expect_ffd_max")
            .and_then(Value::as_u64)
            != Some(3)
        || sidecar.pointer("/replay_audit/ignored_value_fields") != Some(&json!(["secs"]))
    {
        return Err("generation replay-audit receipt differs from the registered audit".into());
    }

    let argv = sidecar
        .get("audit_argv")
        .and_then(Value::as_array)
        .ok_or_else(|| "generation sidecar has no audit_argv array".to_string())?;
    let staging = argv
        .get(1)
        .and_then(Value::as_str)
        .filter(|path| Path::new(path).is_absolute())
        .ok_or_else(|| "audit_argv has no absolute staging input".to_string())?;
    let stage_marker = format!(".m{}-n{}-l{}.jsonl.attempt.", cell.m, cell.n, cell.ell);
    if !staging.contains(&stage_marker) {
        return Err("audit_argv does not identify the registered staging draw".into());
    }
    let expected_argv = [
        "--audit-jsonl".to_string(),
        staging.to_string(),
        "--expect-m".to_string(),
        cell.m.to_string(),
        "--expect-n".to_string(),
        cell.n.to_string(),
        "--expect-lambda".to_string(),
        cell.ell.to_string(),
        "--expect-seed".to_string(),
        "20261101".to_string(),
        "--expect-count".to_string(),
        record_count.to_string(),
        "--expect-d-max".to_string(),
        "3".to_string(),
        "--expect-ffd-max".to_string(),
        "3".to_string(),
    ];
    if argv
        != &expected_argv
            .into_iter()
            .map(Value::String)
            .collect::<Vec<_>>()
    {
        return Err("generation audit_argv differs from the registered replay".into());
    }
    let expected_generator_argv = [
        "--cells".to_string(),
        format!("{}:{}:3", cell.n, cell.ell),
        "--m".to_string(),
        cell.m.to_string(),
        "--unsat".to_string(),
        "4".to_string(),
        "--min-sat".to_string(),
        "2".to_string(),
        "--ffd-max".to_string(),
        "3".to_string(),
        "--seed".to_string(),
        "20261101".to_string(),
        "--max-draws".to_string(),
        "4096".to_string(),
    ];
    if sidecar.get("argv").and_then(Value::as_array)
        != Some(
            &expected_generator_argv
                .into_iter()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    {
        return Err("generation argv differs from the registered generator".into());
    }
    Ok((execution_ref.to_string(), execution_commit.to_string()))
}

fn draw_panel(cell: Cell, draw_dir: &Path, frozen: &Result<FrozenContext, String>) -> Panel {
    let path = cell.draw_file(draw_dir);
    let mut errors = Vec::new();
    let values = match parse_json_lines(&path) {
        Ok(v) => v,
        Err(e) => {
            let state = if path.is_file() {
                "invalid"
            } else {
                "unmeasured"
            };
            return Panel {
                summary: PanelSummary {
                    cell: cell.id(),
                    state: state.into(),
                    path: path.display().to_string(),
                    records: 0,
                    unsat_draws: Vec::new(),
                    satisfiable_draws: Vec::new(),
                    execution_ref: None,
                    execution_commit: None,
                    errors: vec![e],
                },
                unsat: Vec::new(),
                sat: Vec::new(),
            };
        }
    };

    let mut records = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous = None;
    let mut first_terminal = None;
    let mut unsat_seen = 0usize;
    let mut sat_seen = 0usize;

    let (execution_ref, execution_commit) =
        match validate_generation_sidecar(cell, &path, values.len(), frozen) {
            Ok((reference, commit)) => (Some(reference), Some(commit)),
            Err(error) => {
                errors.push(format!("generation provenance: {error}"));
                (None, None)
            }
        };

    for (i, v) in values.iter().enumerate() {
        let Some(draw) = field_u64(v, "draw") else {
            errors.push(format!("record {i} has no integer draw"));
            continue;
        };
        if draw >= MAX_DRAWS {
            errors.push(format!("draw {draw} is outside 0..{}", MAX_DRAWS - 1));
        }
        if !seen.insert(draw) {
            errors.push(format!("duplicate draw {draw}"));
        }
        if previous.map_or(draw != 0, |p| draw != p + 1) {
            errors.push(format!("draw {draw} does not continue the exact prefix"));
        }
        previous = Some(draw);

        for (key, got, want) in [
            ("m", field_u64(v, "m"), cell.m),
            ("n", field_u64(v, "n"), cell.n),
            ("ell", field_u64(v, "ell"), cell.ell),
            ("n_vars", field_u64(v, "n_vars"), cell.n_vars()),
        ] {
            if got != Some(want) {
                errors.push(format!("draw {draw}: {key} is {got:?}, expected {want}"));
            }
        }
        if v.get("cell").and_then(Value::as_str) != Some(cell.source_cell().as_str()) {
            errors.push(format!("draw {draw}: wrong cell label"));
        }
        if field_u64(v, "d_max") != Some(3) {
            errors.push(format!("draw {draw}: d_max is not the registered value 3"));
        }
        if field_u64(v, "n_eqs") != Some((cell.m - 1) * cell.n) {
            errors.push(format!("draw {draw}: wrong equation count"));
        }
        if field_i64(v, "surplus") != Some(cell.n as i64 - (cell.m * cell.ell) as i64) {
            errors.push(format!("draw {draw}: wrong surplus"));
        }
        let field_bound = 1u64 << cell.n;
        match v.get("v_basis").and_then(Value::as_array) {
            Some(basis)
                if basis.len() == cell.ell as usize
                    && basis
                        .iter()
                        .all(|x| x.as_u64().is_some_and(|x| x < field_bound)) => {}
            _ => errors.push(format!("draw {draw}: malformed subspace basis")),
        }
        if field_u64(v, "x_r").is_none_or(|x| x >= field_bound) {
            errors.push(format!("draw {draw}: malformed target abscissa"));
        }

        let Some(solutions) = field_u64(v, "solutions") else {
            errors.push(format!("draw {draw}: missing exact solution count"));
            continue;
        };
        if let Err(e) = validate_draw_outcome(v, solutions) {
            errors.push(format!("draw {draw}: {e}"));
        }
        if v.get("secs")
            .and_then(Value::as_f64)
            .is_none_or(|seconds| !seconds.is_finite() || seconds < 0.0)
        {
            errors.push(format!(
                "draw {draw}: diagnostic seconds is missing, negative, or non-finite"
            ));
        }
        match (solutions, v.get("ffd")) {
            (_, Some(Value::Null)) => {}
            (0, Some(ffd)) if ffd.as_u64().is_some_and(|d| (1..=3).contains(&d)) => {}
            _ => errors.push(format!("draw {draw}: malformed first-fall degree")),
        }
        if solutions == 0 {
            unsat_seen += 1;
        } else {
            sat_seen += 1;
        }
        if first_terminal.is_none() && unsat_seen >= 4 && sat_seen >= 2 {
            first_terminal = Some(i);
        }
        records.push(DrawRecord {
            draw,
            solutions,
            outcome: v.get("outcome").cloned().unwrap_or(Value::Null),
        });
    }

    if first_terminal != values.len().checked_sub(1) {
        errors.push(match first_terminal {
            Some(i) => format!("stream continued past the first quota-complete record {i}"),
            None => "stream ended before four UNSAT and two SAT records".into(),
        });
    }

    let unsat: Vec<_> = records
        .iter()
        .filter(|r| r.solutions == 0)
        .take(4)
        .cloned()
        .collect();
    let sat: Vec<_> = records
        .iter()
        .filter(|r| r.solutions > 0)
        .take(2)
        .cloned()
        .collect();
    let state = if errors.is_empty() && unsat.len() == 4 && sat.len() == 2 {
        "ready"
    } else if errors.iter().any(|e| {
        !e.contains("ended before") && !e.contains("No such file") && !e.contains("not found")
    }) {
        "invalid"
    } else {
        "unmeasured"
    };
    Panel {
        summary: PanelSummary {
            cell: cell.id(),
            state: state.into(),
            path: path.display().to_string(),
            records: values.len(),
            unsat_draws: unsat.iter().map(|r| r.draw).collect(),
            satisfiable_draws: sat.iter().map(|r| r.draw).collect(),
            execution_ref,
            execution_commit,
            errors,
        },
        unsat,
        sat,
    }
}

enum CompletedRecord {
    Value(Value, String, Vec<AttemptSummary>),
    Unmeasured(String, Vec<AttemptSummary>),
    Invalid(String, Vec<AttemptSummary>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttemptTermination {
    Success,
    Resource(&'static str),
    DeterministicFailure,
}

impl AttemptTermination {
    fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Resource(kind) => kind,
            Self::DeterministicFailure => "nonzero_exit",
        }
    }
}

fn allocator_failure(stderr_path: &Path) -> Result<bool, String> {
    require_regular_file(stderr_path)?;
    let stderr =
        fs::read_to_string(stderr_path).map_err(|e| format!("{}: {e}", stderr_path.display()))?;
    Ok(stderr.lines().any(|line| {
        let line = line.trim();
        line.strip_prefix("memory allocation of ")
            .and_then(|rest| rest.strip_suffix(" bytes failed"))
            .is_some_and(|bytes| !bytes.is_empty() && bytes.bytes().all(|b| b.is_ascii_digit()))
    }))
}

fn expected_termination(rc: i32, stderr_path: &Path) -> Result<AttemptTermination, String> {
    match rc {
        0 => Ok(AttemptTermination::Success),
        124 => Ok(AttemptTermination::Resource("timeout")),
        125 => Ok(AttemptTermination::Resource("checkpoint_invalid")),
        137 => Ok(AttemptTermination::Resource("signal_kill")),
        143 => Ok(AttemptTermination::Resource("signal_term")),
        134 if allocator_failure(stderr_path)? => Ok(AttemptTermination::Resource("address_space")),
        _ => Ok(AttemptTermination::DeterministicFailure),
    }
}

fn validate_finished(
    path: &Path,
    rc: i32,
    stderr_path: &Path,
) -> Result<AttemptTermination, String> {
    require_regular_file(path)?;
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let object = value
        .as_object()
        .ok_or_else(|| "finished receipt is not one JSON object".to_string())?;
    let exact_keys: BTreeSet<_> = ["schema", "finished_at", "exit_code", "termination"]
        .into_iter()
        .collect();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != exact_keys {
        return Err("finished receipt has missing or unexpected fields".into());
    }
    if value.get("schema").and_then(Value::as_str) != Some("dreg.high-arity-finished/v1") {
        return Err("finished receipt has the wrong schema".into());
    }
    if value
        .get("finished_at")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        return Err("finished receipt has no nonempty finished_at string".into());
    }
    if value.get("exit_code").and_then(Value::as_i64) != Some(i64::from(rc)) {
        return Err("finished receipt exit_code disagrees with status".into());
    }
    let expected = expected_termination(rc, stderr_path)?;
    if value.get("termination").and_then(Value::as_str) != Some(expected.label()) {
        return Err(format!(
            "finished receipt termination disagrees with exit status {rc}"
        ));
    }
    Ok(expected)
}

fn validate_unpaired_finished(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let rc = value
        .get("exit_code")
        .and_then(Value::as_i64)
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| "finished receipt has no i32 exit_code".to_string())?;
    let stderr = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("stderr.log");
    let output = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("out.jsonl");
    require_regular_file(&output)?;
    require_regular_file(&stderr)?;
    validate_finished(path, rc, &stderr).map(|_| ())
}

fn is_registered_atomic_temp(name: &str) -> bool {
    [
        ".allocation.tmp.",
        ".meta.tmp.",
        ".status.tmp.",
        ".finished.tmp.",
        ".analyzer-status.tmp.",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix) && name.len() > prefix.len())
}

/// Validate the shallow append-only layout of one attempt and report whether
/// anything other than metadata or an interrupted atomic temp has been
/// installed.  Temp-only allocations are retryable; arbitrary files,
/// directories, symlinks, and special nodes are durable custody violations.
#[derive(Debug)]
struct AttemptLayout {
    allocated: bool,
    process_evidence: bool,
}

fn inspect_attempt_layout(dir: &Path) -> Result<AttemptLayout, String> {
    let allowed: BTreeSet<_> = [
        "allocated.json",
        "meta.json",
        "out.jsonl",
        "stderr.log",
        "finished.json",
        "status",
        "analysis.json",
        "analyzer.stderr",
        "analyzer.status",
    ]
    .into_iter()
    .collect();
    let mut allocated = false;
    let mut process_evidence = false;
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("cannot enumerate {}: {e}", dir.display()))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| format!("{} has a non-UTF-8 entry", dir.display()))?;
        let file_type = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", entry.path().display()))?;
        if !file_type.is_file() {
            return Err(format!(
                "attempt contains a non-regular or symlink entry {}",
                entry.path().display()
            ));
        }
        if is_registered_atomic_temp(&name) {
            continue;
        }
        if !allowed.contains(name.as_str()) {
            return Err(format!("attempt contains unexpected file {name:?}"));
        }
        if name == "allocated.json" {
            allocated = true;
        } else if name != "meta.json" {
            process_evidence = true;
        }
    }
    Ok(AttemptLayout {
        allocated,
        process_evidence,
    })
}

fn validate_allocation(path: &Path, logical: &str, attempt: &str) -> Result<(), String> {
    require_regular_file(path)?;
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let object = value
        .as_object()
        .ok_or_else(|| "attempt allocation marker is not one object".to_string())?;
    let exact_keys: BTreeSet<_> = ["schema", "logical_name", "attempt", "allocated_at"]
        .into_iter()
        .collect();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != exact_keys
        || value.get("schema").and_then(Value::as_str) != Some("dreg.high-arity-allocation/v1")
        || value.get("logical_name").and_then(Value::as_str) != Some(logical)
        || value.get("attempt").and_then(Value::as_str) != Some(attempt)
        || value
            .get("allocated_at")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
    {
        return Err("attempt allocation marker is malformed or mismatched".into());
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn completed_record(
    output_root: &Path,
    logical: &str,
    frozen: &Result<FrozenContext, String>,
    expected_input: &Path,
    expected_draw: u64,
    expected_mode: &str,
    expected_max_cols: u64,
) -> CompletedRecord {
    let attempts_dir = output_root.join("runs").join(logical).join("attempts");
    let entries = match fs::read_dir(&attempts_dir) {
        Ok(entries) => entries,
        Err(e) => {
            return CompletedRecord::Unmeasured(
                format!("{}: {e}", attempts_dir.display()),
                Vec::new(),
            );
        }
    };
    let mut dirs = Vec::new();
    let mut layout_errors = Vec::new();
    for entry in entries {
        match entry {
            Ok(e) => match e.file_type() {
                Ok(file_type) if file_type.is_dir() => dirs.push(e),
                Ok(_) => layout_errors.push(format!(
                    "unexpected non-directory or symlink entry {}",
                    e.path().display()
                )),
                Err(error) => layout_errors.push(format!(
                    "cannot inspect attempt entry {}: {error}",
                    e.path().display()
                )),
            },
            Err(e) => {
                return CompletedRecord::Invalid(
                    format!("cannot enumerate attempt: {e}"),
                    Vec::new(),
                );
            }
        }
    }
    dirs.sort_by_key(|e| e.file_name());
    let directory_count = dirs.len();

    let mut summaries = Vec::new();
    let mut successes = Vec::new();
    let mut terminal_zero_attempt: Option<String> = None;
    let mut hard_invalid = layout_errors;
    if dirs.len() > 4 {
        hard_invalid.push(format!(
            "{} attempts exist, exceeding the registered maximum four",
            dirs.len()
        ));
    }
    for (position, entry) in dirs.into_iter().enumerate() {
        let attempt = entry.file_name().to_string_lossy().to_string();
        let dir = entry.path();
        let out = dir.join("out.jsonl");
        let status = dir.join("status");
        let finished = dir.join("finished.json");
        let metadata = dir.join("meta.json");
        let allocation = dir.join("allocated.json");
        let layout = match inspect_attempt_layout(&dir) {
            Ok(value) => value,
            Err(error) => {
                hard_invalid.push(format!("attempt {attempt}: {error}"));
                AttemptLayout {
                    allocated: allocation.exists(),
                    process_evidence: true,
                }
            }
        };
        // Atomic writers can leave same-directory dot-temp files when a host
        // disappears.  Such a temp-only allocation has made no scientific
        // claim and remains retryable; a malformed installed meta.json does
        // make a durable, invalid claim.
        let expected_attempt = format!("{:04}", position + 1);
        let name_valid = attempt.len() == 4
            && attempt.bytes().all(|b| b.is_ascii_digit())
            && attempt == expected_attempt;
        if !name_valid {
            hard_invalid.push(format!(
                "attempt directory {attempt:?} is not the contiguous registered name {expected_attempt}"
            ));
        }
        if let Some(previous) = &terminal_zero_attempt {
            hard_invalid.push(format!(
                "attempt {attempt} follows terminal status-zero attempt {previous}"
            ));
        }
        if !layout.allocated {
            if metadata.exists() || layout.process_evidence {
                hard_invalid.push(format!(
                    "attempt {attempt}: process evidence exists without an atomic allocation marker"
                ));
            }
            if position + 1 != directory_count {
                hard_invalid.push(format!(
                    "attempt {attempt}: an unallocated setup directory precedes another attempt"
                ));
            }
            // A final empty/temp-only directory can be reused by the runner
            // and has not consumed one of the four registered attempts.
            continue;
        }
        if let Err(error) = validate_allocation(&allocation, logical, &attempt) {
            hard_invalid.push(format!("attempt {attempt}: {error}"));
        }
        let setup_only = !metadata.exists() && !layout.process_evidence;
        if setup_only {
            if position + 1 != directory_count {
                hard_invalid.push(format!(
                    "attempt {attempt}: a marker-only setup directory precedes another attempt"
                ));
            }
            // Allocation identity is durable, but no closure process was
            // launched. The runner reuses this final directory, so it does
            // not consume one of the four infrastructure attempts.
            continue;
        }
        let meta_validation = Some(match frozen {
            Ok(context) => context.validate_attempt_meta(
                &metadata,
                logical,
                &attempt,
                expected_input,
                expected_draw,
                expected_mode,
                expected_max_cols,
                output_root,
            ),
            Err(e) => Err(format!("frozen provenance is unavailable: {e}")),
        });
        let meta_error = meta_validation
            .as_ref()
            .and_then(|result| result.as_ref().err())
            .cloned();
        if let Some(e) = &meta_error {
            hard_invalid.push(format!("attempt {attempt}: {e}"));
        }
        let status_text = fs::read_to_string(&status);
        let rc = status_text
            .as_ref()
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok());
        let mut termination = None;
        let (mut state, mut reason) = match (&status_text, rc) {
            (Err(e), _) if finished.exists() => match validate_unpaired_finished(&finished) {
                Ok(()) => (
                    "incomplete",
                    format!(
                        "valid finished receipt is not yet committed by its atomic status: {e}"
                    ),
                ),
                Err(receipt_error) => {
                    let why = format!(
                        "invalid unpaired finished receipt ({receipt_error}); status is missing: {e}"
                    );
                    hard_invalid.push(format!("attempt {attempt}: {why}"));
                    ("invalid", why)
                }
            },
            (Err(e), _) => ("not_completed", format!("missing status: {e}")),
            (Ok(_), None) => {
                hard_invalid.push(format!("attempt {attempt}: malformed status"));
                ("invalid", "malformed status".into())
            }
            (Ok(_), Some(rc)) if !finished.exists() => {
                let why = format!(
                    "installed process exit status {rc} is missing its prior finished receipt"
                );
                hard_invalid.push(format!("attempt {attempt}: {why}"));
                ("invalid", why)
            }
            (Ok(_), Some(rc)) if require_regular_file(&out).is_err() => {
                let why = format!(
                    "installed completion receipt/status {rc} lacks regular retained stdout"
                );
                hard_invalid.push(format!("attempt {attempt}: {why}"));
                ("invalid", why)
            }
            (Ok(_), Some(rc)) if require_regular_file(&dir.join("stderr.log")).is_err() => {
                let why = format!(
                    "installed completion receipt/status {rc} lacks regular retained stderr"
                );
                hard_invalid.push(format!("attempt {attempt}: {why}"));
                ("invalid", why)
            }
            (Ok(_), Some(rc)) => match validate_finished(&finished, rc, &dir.join("stderr.log")) {
                Err(e) => {
                    let why = format!("invalid finished receipt: {e}");
                    hard_invalid.push(format!("attempt {attempt}: {why}"));
                    ("invalid", why)
                }
                Ok(kind @ AttemptTermination::Resource(_)) => {
                    termination = Some(kind.label().to_string());
                    (
                        "resource_interrupted",
                        format!("explicit {} termination at exit status {rc}", kind.label()),
                    )
                }
                Ok(kind @ AttemptTermination::DeterministicFailure) => {
                    termination = Some(kind.label().to_string());
                    let why =
                        format!("unrecognized deterministic process failure at exit status {rc}");
                    hard_invalid.push(format!("attempt {attempt}: {why}"));
                    ("invalid", why)
                }
                Ok(kind @ AttemptTermination::Success) => {
                    termination = Some(kind.label().to_string());
                    match parse_json_lines(&out) {
                        Ok(mut rows) if rows.len() == 1 => {
                            successes.push((attempt.clone(), rows.remove(0)));
                            if terminal_zero_attempt.is_none() {
                                terminal_zero_attempt = Some(attempt.clone());
                            }
                            (
                                "candidate_complete",
                                "status zero, valid receipt, and exactly one JSON row".into(),
                            )
                        }
                        Ok(rows) => {
                            let why = format!(
                                "status zero but output has {} JSON rows, expected one",
                                rows.len()
                            );
                            hard_invalid.push(format!("attempt {attempt}: {why}"));
                            ("invalid", why)
                        }
                        Err(e) => {
                            let why = format!("status zero with missing or invalid output: {e}");
                            hard_invalid.push(format!("attempt {attempt}: {why}"));
                            ("invalid", why)
                        }
                    }
                }
            },
        };
        if let Some(e) = meta_error {
            state = "invalid";
            reason = format!("invalid metadata: {e}; {reason}");
        }
        summaries.push(AttemptSummary {
            attempt,
            exit_status: rc,
            termination,
            execution_ref: meta_validation
                .as_ref()
                .and_then(|result| result.as_ref().ok())
                .map(|binding| binding.execution_ref.clone()),
            execution_commit: meta_validation
                .as_ref()
                .and_then(|result| result.as_ref().ok())
                .map(|binding| binding.execution_commit.clone()),
            checkpoint_restored: meta_validation
                .as_ref()
                .and_then(|result| result.as_ref().ok())
                .map(|binding| binding.checkpoint_restored),
            checkpoint_generation: meta_validation
                .as_ref()
                .and_then(|result| result.as_ref().ok())
                .and_then(|binding| binding.checkpoint_generation),
            state: state.into(),
            reason,
            output: out.display().to_string(),
            status: status.display().to_string(),
            finished: finished.display().to_string(),
            metadata: metadata.display().to_string(),
        });
    }
    if !hard_invalid.is_empty() {
        return CompletedRecord::Invalid(hard_invalid.join("; "), summaries);
    }
    match successes.len() {
        0 => CompletedRecord::Unmeasured("no complete status-zero attempt".into(), summaries),
        1 => {
            let (attempt, value) = successes.remove(0);
            CompletedRecord::Value(value, attempt, summaries)
        }
        n => CompletedRecord::Invalid(
            format!("{n} successful attempts exist; expected exactly one"),
            summaries,
        ),
    }
}

fn collect_attempt_evidence(
    root: &Path,
    directory: &Path,
    rows: &mut Vec<(String, PathBuf)>,
) -> Result<(), String> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(format!("{}: {e}", directory.display())),
    };
    for entry in entries {
        let entry = entry.map_err(|e| format!("cannot enumerate attempt evidence: {e}"))?;
        let file_type = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", entry.path().display()))?;
        if file_type.is_symlink() {
            return Err(format!(
                "attempt evidence contains symlink {}",
                entry.path().display()
            ));
        }
        if file_type.is_dir() {
            collect_attempt_evidence(root, &entry.path(), rows)?;
        } else if file_type.is_file() {
            let base = entry.file_name();
            let base = base
                .to_str()
                .ok_or_else(|| "attempt evidence has a non-UTF-8 filename".to_string())?;
            if base.starts_with('.') && base.contains(".tmp.") {
                continue;
            }
            let relative = entry
                .path()
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_str()
                .ok_or_else(|| "attempt evidence has a non-UTF-8 path".to_string())?
                .to_string();
            if relative.contains(['\n', '\t', '\\'])
                || relative.starts_with(' ')
                || relative.ends_with(' ')
            {
                return Err(format!("attempt evidence has unsafe path {relative:?}"));
            }
            rows.push((relative, entry.path()));
        } else {
            return Err(format!(
                "attempt evidence contains special file {}",
                entry.path().display()
            ));
        }
    }
    Ok(())
}

fn attempt_evidence_identity(run_dir: &Path) -> Result<(String, usize), String> {
    let mut rows = Vec::new();
    collect_attempt_evidence(run_dir, &run_dir.join("attempts"), &mut rows)?;
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    let mut manifest = Vec::new();
    for (relative, path) in &rows {
        let hash = sha256_file(path)?;
        manifest.extend_from_slice(format!("{hash}  {relative}\n").as_bytes());
    }
    Ok((sha256_bytes(&manifest), rows.len()))
}

fn canonical_terminal_row(row: &ProcessRow) -> Result<String, String> {
    let mut value = serde_json::to_value(row).map_err(|e| e.to_string())?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| "serialized process row is not an object".to_string())?;
    object.remove("reason");
    if let Some(attempts) = object.get_mut("attempts").and_then(Value::as_array_mut) {
        for attempt in attempts {
            let attempt = attempt
                .as_object_mut()
                .ok_or_else(|| "serialized attempt summary is not an object".to_string())?;
            for key in ["reason", "output", "status", "finished", "metadata"] {
                attempt.remove(key);
            }
        }
    }
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

fn validate_final_pointer(
    output_root: &Path,
    row: &ProcessRow,
    frozen: &Result<FrozenContext, String>,
) -> Result<(), String> {
    let run_dir = output_root.join("runs").join(&row.run_id);
    let path = run_dir.join("final.json");
    if !path.exists() {
        return Ok(());
    }
    require_regular_file(&path)?;
    let context = frozen
        .as_ref()
        .map_err(|e| format!("frozen provenance is unavailable: {e}"))?;
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let final_row: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let object = final_row
        .as_object()
        .ok_or_else(|| "terminal pointer is not one object".to_string())?;
    let exact_keys: BTreeSet<_> = [
        "schema",
        "logical_name",
        "classification",
        "selected_attempt",
        "terminal_exit_code",
        "row_sha256",
        "protocol_sha256",
        "analyzer_sha256",
        "evidence_sha256",
        "evidence_files",
        "created_at",
    ]
    .into_iter()
    .collect();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != exact_keys {
        return Err("terminal pointer has missing or unexpected fields".into());
    }
    let exit_code = row_exit_code(row);
    if !matches!(exit_code, 0 | 65 | 76) {
        return Err("terminal pointer exists for a retryable row".into());
    }
    let canonical = canonical_terminal_row(row)?;
    let row_sha = sha256_bytes(canonical.as_bytes());
    let (evidence_sha, evidence_files) = attempt_evidence_identity(&run_dir)?;
    if final_row.get("schema").and_then(Value::as_str) != Some("dreg.high-arity-final/v1")
        || final_row.get("logical_name").and_then(Value::as_str) != Some(row.run_id.as_str())
        || final_row.get("classification").and_then(Value::as_str)
            != Some(row.classification.as_str())
        || final_row.get("selected_attempt") != Some(&json!(row.selected_attempt))
        || final_row.get("terminal_exit_code").and_then(Value::as_i64) != Some(i64::from(exit_code))
        || final_row.get("row_sha256").and_then(Value::as_str) != Some(row_sha.as_str())
        || final_row.get("protocol_sha256").and_then(Value::as_str)
            != Some(context.protocol_sha256.as_str())
        || final_row.get("analyzer_sha256").and_then(Value::as_str)
            != Some(context.analyzer_sha256.as_str())
        || final_row.get("evidence_sha256").and_then(Value::as_str) != Some(evidence_sha.as_str())
        || final_row.get("evidence_files").and_then(Value::as_u64)
            != u64::try_from(evidence_files).ok()
        || final_row
            .get("created_at")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
    {
        return Err("terminal pointer disagrees with its row or retained evidence".into());
    }
    Ok(())
}

fn common_errors(v: &Value, cell: Cell, draw: &DrawRecord, expected_input: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    if !v.is_object() {
        return vec!["record is not a JSON object".into()];
    }
    for (key, got, want) in [
        ("m", field_u64(v, "m"), cell.m),
        ("n", field_u64(v, "n"), cell.n),
        ("ell", field_u64(v, "ell"), cell.ell),
        ("n_vars", field_u64(v, "n_vars"), cell.n_vars()),
        ("draw", field_u64(v, "draw"), draw.draw),
    ] {
        if got != Some(want) {
            errors.push(format!("{key} is {got:?}, expected {want}"));
        }
    }
    if v.get("cell").and_then(Value::as_str) != Some(cell.source_cell().as_str()) {
        errors.push("cell label differs from the committed draw".into());
    }
    if v.get("committed_outcome") != Some(&draw.outcome) {
        errors.push("committed_outcome differs from the draw file".into());
    }
    match v.get("source").and_then(Value::as_str) {
        Some(source) => {
            let source = Path::new(source);
            if !source.is_absolute() || source.file_name() != expected_input.file_name() {
                errors.push("source does not identify the committed draw input".into());
            }
        }
        None => errors.push("source is missing or is not a string".into()),
    }
    if v.get("checkpoint_segments")
        .is_some_and(|x| x.as_u64().is_none())
    {
        errors.push("checkpoint_segments is not an integer".into());
    }
    if v.get("engine").is_some() {
        errors.push("sparse/alternate engine output is not registered".into());
    }
    errors
}

fn scan_rows(v: &Value) -> Result<Vec<&Value>, String> {
    let scan = v
        .get("scan")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing scan array".to_string())?;
    for (expected, step) in (3u64..).zip(scan.iter()) {
        if expected > 4 {
            return Err("scan extends past the registered degree four".into());
        }
        if field_u64(step, "degree") != Some(expected) {
            return Err(format!(
                "scan degree is {:?}, expected {expected}",
                field_u64(step, "degree")
            ));
        }
        for key in ["xl_refuted", "closure_refuted", "saturated"] {
            if field_bool(step, key).is_none() {
                return Err(format!("degree {expected} has no Boolean {key}"));
            }
        }
        for key in [
            "xl_rank",
            "closure_rank",
            "columns",
            "closure_products",
            "closure_word_ops",
        ] {
            if field_u64(step, key).is_none() {
                return Err(format!("degree {expected} has no integer {key}"));
            }
        }
        if step.get("secs").and_then(Value::as_f64).is_none() {
            return Err(format!(
                "degree {expected} has no numeric diagnostic seconds field"
            ));
        }
    }
    let implied_measured_to = scan.last().map_or(2, |s| field_u64(s, "degree").unwrap());
    if field_u64(v, "measured_to") != Some(implied_measured_to) {
        return Err(format!(
            "measured_to is {:?}, but the scan ends at {implied_measured_to}",
            field_u64(v, "measured_to")
        ));
    }
    Ok(scan.iter().collect())
}

fn validate_scan_degrees(v: &Value, scan: &[&Value]) -> Result<(), String> {
    let first_xl = scan
        .iter()
        .find(|s| field_bool(s, "xl_refuted") == Some(true))
        .and_then(|s| field_u64(s, "degree"));
    let first_closure = scan
        .iter()
        .find(|s| field_bool(s, "closure_refuted") == Some(true))
        .and_then(|s| field_u64(s, "degree"));
    let json_degree = |key: &str| match v.get(key) {
        Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .map(Some)
            .ok_or_else(|| format!("{key} is neither null nor an integer")),
        None => Err(format!("missing {key}")),
    };
    if json_degree("xl_degree")? != first_xl {
        return Err("xl_degree disagrees with the scan".into());
    }
    if json_degree("closure_degree")? != first_closure {
        return Err("closure_degree disagrees with the scan".into());
    }
    if scan.iter().any(|s| {
        field_bool(s, "xl_refuted") == Some(true) && field_bool(s, "closure_refuted") != Some(true)
    }) {
        return Err("an XL refutation is absent from the closure row".into());
    }
    Ok(())
}

fn process_row(
    run_id: String,
    kind: &str,
    cell: Cell,
    draw: Option<&DrawRecord>,
    classification: &str,
    reason: String,
    selected_attempt: Option<String>,
    attempts: Vec<AttemptSummary>,
) -> ProcessRow {
    ProcessRow {
        run_id,
        kind: kind.into(),
        cell: cell.id(),
        draw: draw.map(|d| d.draw),
        classification: classification.into(),
        reason,
        selected_attempt,
        attempts,
    }
}

fn validate_checkpoint_segment_binding(
    output: &Value,
    selected_attempt: &str,
    attempts: &[AttemptSummary],
) -> Result<(), String> {
    let mut selected = attempts
        .iter()
        .filter(|attempt| attempt.attempt == selected_attempt);
    let attempt = selected
        .next()
        .ok_or_else(|| "selected successful attempt is absent from attempt history".to_string())?;
    if selected.next().is_some() {
        return Err("selected successful attempt is duplicated in attempt history".into());
    }
    let restored = attempt.checkpoint_restored.ok_or_else(|| {
        "selected successful attempt lacks validated checkpoint-restore metadata".to_string()
    })?;
    let segments = output
        .get("checkpoint_segments")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            "successful checkpointed output has no integer checkpoint_segments".to_string()
        })?;
    match (restored, segments) {
        (false, 1) | (true, 2..) => Ok(()),
        (false, _) => Err(format!(
            "metadata records a fresh checkpoint store but output reports {segments} segments"
        )),
        (true, _) => Err(format!(
            "metadata records a restored checkpoint but output reports only {segments} segment(s)"
        )),
    }
}

fn classify_unsat(
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
    draw_dir: &Path,
    cell: Cell,
    slot: usize,
    draw: Option<&DrawRecord>,
) -> ProcessRow {
    let run_id = draw.map_or_else(
        || format!("unsat-{}-slot{slot}", cell.id()),
        |d| format!("unsat-{}-d{}", cell.id(), d.draw),
    );
    let Some(draw) = draw else {
        return process_row(
            run_id,
            "unsat",
            cell,
            None,
            "unmeasured",
            "draw panel did not supply this registered slot".into(),
            None,
            Vec::new(),
        );
    };
    match completed_record(
        output_root,
        &run_id,
        frozen,
        &cell.draw_file(draw_dir),
        draw.draw,
        "unsat",
        400_000,
    ) {
        CompletedRecord::Unmeasured(reason, attempts) => process_row(
            run_id,
            "unsat",
            cell,
            Some(draw),
            "unmeasured",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Invalid(reason, attempts) => process_row(
            run_id,
            "unsat",
            cell,
            Some(draw),
            "invalid_record",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Value(v, attempt, attempts) => {
            let input = cell.draw_file(draw_dir);
            let mut errors = common_errors(&v, cell, draw, &input);
            if let Err(error) = validate_checkpoint_segment_binding(&v, &attempt, &attempts) {
                errors.push(error);
            }
            if field_u64(&v, "max_cols") != Some(400_000) {
                errors.push("max_cols is not the registered value 400000".into());
            }
            let specialised = match v.get("fixed") {
                None => false,
                Some(Value::Array(fixed)) => !fixed.is_empty(),
                Some(_) => true,
            };
            if specialised {
                errors.push("UNSAT replay carries nonempty or malformed specialisation".into());
            }
            if v.get("solutions").is_some() || v.get("refuted_satisfiable").is_some() {
                errors.push("UNSAT record carries satisfiable-control fields".into());
            }
            let scan = match scan_rows(&v) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(e);
                    Vec::new()
                }
            };
            if let Err(e) = validate_scan_degrees(&v, &scan) {
                errors.push(e);
            }
            let measured_to = field_u64(&v, "measured_to");
            let closure = v.get("closure_degree");
            let (class, reason) = if !errors.is_empty() {
                ("invalid_record", errors.join("; "))
            } else if let Some(d) = closure.and_then(Value::as_u64) {
                let matching = scan.last().is_some_and(|s| {
                    field_u64(s, "degree") == Some(d)
                        && field_bool(s, "closure_refuted") == Some(true)
                });
                let complete = scan
                    .iter()
                    .all(|s| field_bool(s, "saturated") == Some(true));
                if (3..=4).contains(&d)
                    && matching
                    && measured_to == Some(d)
                    && scan.len() == (d - 2) as usize
                    && complete
                {
                    (
                        "closure_by_four",
                        format!("complete closure refutation at degree {d}"),
                    )
                } else {
                    (
                        "invalid_record",
                        format!("closure_degree={d} is inconsistent with scan/measured_to"),
                    )
                }
            } else if closure == Some(&Value::Null) && measured_to == Some(4) {
                let d4 = scan.iter().find(|s| field_u64(s, "degree") == Some(4));
                let any_refuted = scan
                    .iter()
                    .any(|s| field_bool(s, "closure_refuted") == Some(true));
                if scan.len() == 2
                    && scan
                        .iter()
                        .all(|s| field_bool(s, "saturated") == Some(true))
                    && d4.and_then(|s| field_bool(s, "saturated")) == Some(true)
                    && !any_refuted
                {
                    (
                        "above_four",
                        "complete saturated degree-four scan did not refute".into(),
                    )
                } else {
                    (
                        "unmeasured",
                        "degree-four row is present but incomplete or unsaturated".into(),
                    )
                }
            } else if measured_to.unwrap_or(0) < 4 {
                (
                    "unmeasured",
                    format!(
                        "scan completed only through degree {}",
                        measured_to.unwrap_or(0)
                    ),
                )
            } else {
                (
                    "invalid_record",
                    "missing or out-of-range closure classification".into(),
                )
            };
            let selected = (class != "invalid_record").then_some(attempt);
            process_row(
                run_id,
                "unsat",
                cell,
                Some(draw),
                class,
                reason,
                selected,
                attempts,
            )
        }
    }
}

fn classify_control(
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
    draw_dir: &Path,
    cell: Cell,
    slot: usize,
    draw: Option<&DrawRecord>,
) -> ProcessRow {
    let run_id = draw.map_or_else(
        || format!("control-{}-slot{slot}", cell.id()),
        |d| format!("control-{}-d{}", cell.id(), d.draw),
    );
    let Some(draw) = draw else {
        return process_row(
            run_id,
            "satisfiable_control",
            cell,
            None,
            "unmeasured",
            "draw panel did not supply this registered control".into(),
            None,
            Vec::new(),
        );
    };
    match completed_record(
        output_root,
        &run_id,
        frozen,
        &cell.draw_file(draw_dir),
        draw.draw,
        "sat",
        400_000,
    ) {
        CompletedRecord::Unmeasured(reason, attempts) => process_row(
            run_id,
            "satisfiable_control",
            cell,
            Some(draw),
            "unmeasured",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Invalid(reason, attempts) => process_row(
            run_id,
            "satisfiable_control",
            cell,
            Some(draw),
            "invalid_record",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Value(v, attempt, attempts) => {
            let input = cell.draw_file(draw_dir);
            let mut errors = common_errors(&v, cell, draw, &input);
            if let Err(error) = validate_checkpoint_segment_binding(&v, &attempt, &attempts) {
                errors.push(error);
            }
            if field_u64(&v, "max_cols") != Some(400_000) {
                errors.push("max_cols is not the registered value 400000".into());
            }
            for key in ["fixed", "xl_degree", "closure_degree"] {
                if v.get(key).is_some() {
                    errors.push(format!("control unexpectedly carries {key}"));
                }
            }
            let scan = match scan_rows(&v) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(e);
                    Vec::new()
                }
            };
            if field_u64(&v, "solutions") != Some(draw.solutions) || draw.solutions == 0 {
                errors.push("positive solution count differs from committed draw".into());
            }
            let refuting_scan = scan.iter().any(|s| {
                field_bool(s, "xl_refuted") == Some(true)
                    || field_bool(s, "closure_refuted") == Some(true)
            });
            if refuting_scan || field_bool(&v, "refuted_satisfiable") == Some(true) {
                errors.push("satisfiable control was refuted".into());
            } else if field_bool(&v, "refuted_satisfiable") != Some(false) {
                errors.push("refuted_satisfiable is missing or malformed".into());
            }
            let complete = field_u64(&v, "measured_to") == Some(4)
                && scan.len() == 2
                && scan
                    .iter()
                    .all(|s| field_bool(s, "saturated") == Some(true));
            let (class, reason) = if !errors.is_empty() {
                ("invalid_record", errors.join("; "))
            } else if complete {
                (
                    "passed",
                    "complete saturated degrees three and four did not refute".into(),
                )
            } else {
                (
                    "unmeasured",
                    "control scan did not completely saturate degrees three and four".into(),
                )
            };
            let selected = (class != "invalid_record").then_some(attempt);
            process_row(
                run_id,
                "satisfiable_control",
                cell,
                Some(draw),
                class,
                reason,
                selected,
                attempts,
            )
        }
    }
}

fn normalize_identity(mut v: Value) -> Value {
    if let Some(map) = v.as_object_mut() {
        map.remove("source");
        map.remove("checkpoint_segments");
        if let Some(scan) = map.get_mut("scan").and_then(Value::as_array_mut) {
            for step in scan {
                if let Some(step) = step.as_object_mut() {
                    step.remove("secs");
                }
            }
        }
    }
    v
}

fn classify_identity(
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
    expected_input: &Path,
    expected_path: &Path,
) -> ProcessRow {
    let run_id = "identity-m4-n12-l5-d24".to_string();
    let cell = Cell::new(4, 12, 5);
    match completed_record(
        output_root,
        &run_id,
        frozen,
        expected_input,
        24,
        "unsat",
        155_000,
    ) {
        CompletedRecord::Unmeasured(reason, attempts) => process_row(
            run_id,
            "identity",
            cell,
            None,
            "unmeasured",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Invalid(reason, attempts) => process_row(
            run_id,
            "identity",
            cell,
            None,
            "invalid_record",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Value(got, attempt, attempts) => {
            let expected = parse_json_lines(expected_path);
            let checkpoint_binding = validate_checkpoint_segment_binding(&got, &attempt, &attempts);
            let (class, reason) = match (checkpoint_binding, expected) {
                (Err(error), _) => ("invalid_record", error),
                (Ok(()), Ok(v))
                    if v.len() == 1
                        && normalize_identity(v[0].clone()) == normalize_identity(got) =>
                {
                    ("passed", "all historical algebraic fields match".into())
                }
                (Ok(()), Ok(v)) if v.len() != 1 => (
                    "invalid_record",
                    format!(
                        "historical identity file has {} rows, expected one",
                        v.len()
                    ),
                ),
                (Ok(()), Ok(_)) => (
                    "invalid_instrument",
                    "historical algebraic identity replay differs".into(),
                ),
                (Ok(()), Err(e)) if expected_path.is_file() => (
                    "invalid_record",
                    format!("historical identity row is malformed: {e}"),
                ),
                (Ok(()), Err(e)) => (
                    "unmeasured",
                    format!("cannot read historical identity: {e}"),
                ),
            };
            let selected = (class == "passed").then_some(attempt);
            process_row(
                run_id, "identity", cell, None, class, reason, selected, attempts,
            )
        }
    }
}

fn apply_panel_gate(row: &mut ProcessRow, panel: &PanelSummary) {
    if panel.state == "ready" {
        return;
    }
    row.classification = if panel.state == "invalid" {
        "invalid_record"
    } else {
        "unmeasured"
    }
    .into();
    let detail = if panel.errors.is_empty() {
        format!("draw panel is {}", panel.state)
    } else {
        format!("draw panel is {}: {}", panel.state, panel.errors.join("; "))
    };
    row.reason = detail;
    row.selected_attempt = None;
}

fn apply_resume_gate(row: &mut ProcessRow, error: &str) {
    row.classification = "invalid_record".into();
    row.reason = format!("resume-import custody is invalid: {error}");
    row.selected_attempt = None;
}

fn apply_final_pointer_gate(
    row: &mut ProcessRow,
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
) {
    if let Err(error) = validate_final_pointer(output_root, row, frozen) {
        row.classification = "invalid_record".into();
        row.reason = format!("terminal evidence pointer is invalid: {error}");
        row.selected_attempt = None;
    }
}

fn cell_classification(
    cell: Cell,
    panel_state: &str,
    identity: &ProcessRow,
    controls: &[ProcessRow],
    unsat: &[ProcessRow],
) -> CellSummary {
    let count = |name: &str| unsat.iter().filter(|r| r.classification == name).count();
    let closure = count("closure_by_four");
    let above = count("above_four");
    let unmeasured = count("unmeasured");
    let invalid = count("invalid_record");
    let controls_passed = controls
        .iter()
        .filter(|r| r.classification == "passed")
        .count();
    let controls_invalid = controls
        .iter()
        .any(|r| r.classification == "invalid_record" || r.classification == "invalid_instrument");
    let controls_unmeasured = controls.iter().any(|r| r.classification == "unmeasured");
    let classification = if panel_state == "invalid"
        || identity.classification == "invalid_record"
        || identity.classification == "invalid_instrument"
        || controls_invalid
        || invalid > 0
    {
        "invalid_instrument"
    } else if panel_state != "ready"
        || identity.classification != "passed"
        || controls_unmeasured
        || unsat.len() != 4
        || controls.len() != 6
        || controls_passed != 6
    {
        "unmeasured"
    } else if closure + above == 4 && closure > 0 && above > 0 {
        "draw_dependent"
    } else if above > 0 {
        "not_universally_four_at_cell"
    } else if unmeasured > 0 {
        "unmeasured"
    } else if closure == 4 {
        "bounded_four_at_cell"
    } else {
        "invalid_instrument"
    };
    CellSummary {
        cell: cell.id(),
        classification: classification.into(),
        closure_by_four: closure,
        above_four: above,
        unmeasured,
        invalid,
        controls_passed,
    }
}

fn historical_expected(draw_dir: &Path) -> PathBuf {
    let repo_relative =
        PathBuf::from("research/dreg_closure_driver_20261016/runs/fresh-m4-12-5.d24.jsonl");
    if repo_relative.is_file() {
        return repo_relative;
    }
    draw_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("research"))
        .join("dreg_closure_driver_20261016/runs/fresh-m4-12-5.d24.jsonl")
}

fn historical_input(expected_path: &Path) -> PathBuf {
    expected_path
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("research/dreg_closure_driver_20261016"))
        .join("draws/m4-12-5.jsonl")
}

fn row_exit_code(row: &ProcessRow) -> i32 {
    match row.classification.as_str() {
        "passed" | "closure_by_four" | "above_four" => 0,
        "unmeasured" => {
            let has_status_zero = row.attempts.iter().any(|a| a.exit_status == Some(0));
            if has_status_zero || row.attempts.len() >= 4 {
                76
            } else {
                75
            }
        }
        _ => 65,
    }
}

fn main() {
    let args: Vec<_> = env::args_os().collect();
    let required = match args.len() {
        3 => None,
        5 if args[3].to_str() == Some("--require") => Some(args[4].to_string_lossy().to_string()),
        _ => {
            eprintln!(
                "usage: {} DRAW_DIR OUTPUT_ROOT [--require LOGICAL_NAME]",
                args[0].to_string_lossy()
            );
            std::process::exit(2);
        }
    };
    if required.as_deref().is_some_and(str::is_empty) {
        eprintln!("--require needs a nonempty logical name");
        std::process::exit(2);
    }
    let draw_dir = PathBuf::from(&args[1]);
    let output_root = PathBuf::from(&args[2]);
    let expected = historical_expected(&draw_dir);
    let expected_input = historical_input(&expected);
    let frozen = FrozenContext::load(&draw_dir);
    let resume_validation = validate_resume_imports(&output_root);
    let resume_provenance = match &resume_validation {
        Ok(records) => ResumeProvenanceSummary {
            state: "ready".into(),
            records: *records,
            reason: if *records == 0 {
                "no resume import was recorded".into()
            } else {
                "all resume descriptors, manifests, and immutable artifacts validate".into()
            },
        },
        Err(reason) => ResumeProvenanceSummary {
            state: "invalid".into(),
            records: 0,
            reason: reason.clone(),
        },
    };
    let provenance = match &frozen {
        Ok(context) => ProvenanceSummary {
            state: "ready".into(),
            protocol: Some(context.protocol_path.display().to_string()),
            binary_manifest: Some(context.binary_manifest_path.display().to_string()),
            protocol_sha256: Some(context.protocol_sha256.clone()),
            source_commit: Some(context.source_commit.clone()),
            binary_manifest_sha256: Some(context.binary_manifest_sha256.clone()),
            reason: "frozen protocol and binary manifest agree".into(),
        },
        Err(reason) => ProvenanceSummary {
            state: "unavailable".into(),
            protocol: None,
            binary_manifest: None,
            protocol_sha256: None,
            source_commit: None,
            binary_manifest_sha256: None,
            reason: reason.clone(),
        },
    };

    let panels: Vec<_> = CELLS
        .iter()
        .map(|&c| draw_panel(c, &draw_dir, &frozen))
        .collect();
    let mut identity = classify_identity(&output_root, &frozen, &expected_input, &expected);
    apply_final_pointer_gate(&mut identity, &output_root, &frozen);
    if let Err(error) = &resume_validation {
        apply_resume_gate(&mut identity, error);
    }
    let mut control_sets = Vec::new();
    let mut unsat_sets = Vec::new();
    for (&cell, panel) in CELLS.iter().zip(&panels) {
        let mut controls: Vec<_> = (0..2)
            .map(|slot| {
                classify_control(
                    &output_root,
                    &frozen,
                    &draw_dir,
                    cell,
                    slot,
                    panel.sat.get(slot),
                )
            })
            .collect();
        let mut unsat: Vec<_> = (0..4)
            .map(|slot| {
                classify_unsat(
                    &output_root,
                    &frozen,
                    &draw_dir,
                    cell,
                    slot,
                    panel.unsat.get(slot),
                )
            })
            .collect();
        for row in controls.iter_mut().chain(unsat.iter_mut()) {
            apply_panel_gate(row, &panel.summary);
            apply_final_pointer_gate(row, &output_root, &frozen);
            if let Err(error) = &resume_validation {
                apply_resume_gate(row, error);
            }
        }
        control_sets.push(controls);
        unsat_sets.push(unsat);
    }
    let all_controls: Vec<_> = control_sets.iter().flatten().cloned().collect();
    let cells: Vec<_> = CELLS
        .iter()
        .zip(&panels)
        .zip(&unsat_sets)
        .map(|((&cell, panel), unsat)| {
            cell_classification(cell, &panel.summary.state, &identity, &all_controls, unsat)
        })
        .collect();

    let mut rows = vec![identity.clone()];
    for (controls, unsat) in control_sets.iter().zip(&unsat_sets) {
        rows.extend(controls.iter().cloned());
        rows.extend(unsat.iter().cloned());
    }

    assert_eq!(
        rows.len(),
        19,
        "the protocol has exactly nineteen subprocess rows"
    );
    let controls_invalid = all_controls.iter().any(|r| row_exit_code(r) == 65);
    let controls_complete =
        all_controls.len() == 6 && all_controls.iter().all(|r| r.classification == "passed");
    let sat_package_class = if controls_invalid {
        "invalid"
    } else if controls_complete {
        "passed"
    } else {
        "unmeasured"
    };
    let sat_package_reason = match sat_package_class {
        "passed" => "all six complete saturated controls remained unrefuted".to_string(),
        "invalid" => "at least one satisfiable control is invalid or refuted".to_string(),
        _ => "at least one satisfiable control is missing or incomplete".to_string(),
    };
    let mut packages = vec![
        PackageSummary {
            package_id: "identity-replay".into(),
            role: "reference".into(),
            classification: identity.classification.clone(),
            reason: identity.reason.clone(),
            subprocesses: vec![identity.run_id.clone()],
        },
        PackageSummary {
            package_id: "satisfiable-controls".into(),
            role: "control_positive".into(),
            classification: sat_package_class.into(),
            reason: sat_package_reason,
            subprocesses: all_controls.iter().map(|r| r.run_id.clone()).collect(),
        },
    ];
    for (cell, unsat) in cells.iter().zip(&unsat_sets) {
        packages.push(PackageSummary {
            package_id: cell.cell.clone(),
            role: "candidate".into(),
            classification: cell.classification.clone(),
            reason: format!(
                "{} closure-by-four, {} above-four, {} unmeasured, {} invalid; {}/6 shared controls passed",
                cell.closure_by_four,
                cell.above_four,
                cell.unmeasured,
                cell.invalid,
                cell.controls_passed
            ),
            subprocesses: unsat.iter().map(|r| r.run_id.clone()).collect(),
        });
    }
    assert_eq!(
        packages.len(),
        5,
        "the protocol has five composite packages"
    );

    let global = if cells
        .iter()
        .any(|c| c.classification == "invalid_instrument")
        || identity.classification == "invalid_record"
        || identity.classification == "invalid_instrument"
        || sat_package_class == "invalid"
    {
        "invalid_instrument"
    } else if identity.classification != "passed" || sat_package_class != "passed" {
        "unmeasured"
    } else if cells.iter().any(|c| {
        c.classification == "not_universally_four_at_cell" || c.classification == "draw_dependent"
    }) {
        "not_universally_four_in_registered_panel"
    } else if cells.iter().any(|c| c.classification == "unmeasured") {
        "unmeasured"
    } else if cells
        .iter()
        .all(|c| c.classification == "bounded_four_at_cell")
    {
        "bounded_four_across_registered_cells"
    } else {
        "invalid_instrument"
    };
    let summary = json!({
        "schema": "dreg_high_arity.summary/v1",
        "draw_dir": draw_dir,
        "output_root": output_root,
        "historical_identity": expected,
        "frozen_provenance": provenance,
        "resume_provenance": resume_provenance,
        "global_classification": global,
        "draw_panels": panels.iter().map(|p| &p.summary).collect::<Vec<_>>(),
        "rows": &rows,
        "cells": &cells,
        "packages": &packages,
        "timing_interpreted": false
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&summary).expect("summary serializes")
    );

    let exit_code = if let Some(logical) = required {
        match rows.iter().find(|row| row.run_id == logical) {
            Some(row) => row_exit_code(row),
            None => {
                eprintln!("unknown or unavailable registered logical name: {logical}");
                65
            }
        }
    } else {
        let codes: Vec<_> = rows.iter().map(row_exit_code).collect();
        if codes.contains(&65) {
            65
        } else if codes.contains(&75) {
            75
        } else if codes.contains(&76) {
            76
        } else {
            0
        }
    };
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_summary_serializes_explicit_frozen_bindings_or_nulls() {
        let ready = serde_json::to_value(ProvenanceSummary {
            state: "ready".into(),
            protocol: Some("/study/protocol.json".into()),
            binary_manifest: Some("/study/binary.sha256".into()),
            protocol_sha256: Some("a".repeat(64)),
            source_commit: Some("b".repeat(40)),
            binary_manifest_sha256: Some("c".repeat(64)),
            reason: "verified".into(),
        })
        .unwrap();
        assert_eq!(ready["protocol_sha256"], "a".repeat(64));
        assert_eq!(ready["source_commit"], "b".repeat(40));
        assert_eq!(ready["binary_manifest_sha256"], "c".repeat(64));

        let unavailable = serde_json::to_value(ProvenanceSummary {
            state: "unavailable".into(),
            protocol: None,
            binary_manifest: None,
            protocol_sha256: None,
            source_commit: None,
            binary_manifest_sha256: None,
            reason: "missing".into(),
        })
        .unwrap();
        for key in [
            "protocol",
            "binary_manifest",
            "protocol_sha256",
            "source_commit",
            "binary_manifest_sha256",
        ] {
            assert!(unavailable[key].is_null(), "{key} must serialize as null");
        }
    }

    #[test]
    fn identity_normalization_removes_only_non_algebraic_fields() {
        let a = json!({"source":"a", "checkpoint_segments":1, "scan":[{"degree":4,"secs":1.0,"rank":9}]});
        let b = json!({"source":"b", "checkpoint_segments":7, "scan":[{"degree":4,"secs":99.0,"rank":9}]});
        assert_eq!(normalize_identity(a), normalize_identity(b));
    }

    fn scan_step(degree: u64, saturated: bool, xl: bool, closure: bool) -> Value {
        json!({
            "degree": degree,
            "xl_refuted": xl,
            "closure_refuted": closure,
            "saturated": saturated,
            "xl_rank": 1,
            "closure_rank": 2,
            "columns": 3,
            "closure_products": 4,
            "closure_word_ops": 5,
            "secs": 0.25
        })
    }

    #[test]
    fn scan_degree_validation_is_contiguous() {
        let valid = json!({
            "measured_to": 4,
            "scan": [scan_step(3, true, false, false), scan_step(4, true, false, false)]
        });
        assert!(scan_rows(&valid).is_ok());
        let skipped = json!({"measured_to":4,"scan":[scan_step(4, true, false, false)]});
        assert!(scan_rows(&skipped).is_err());
        let inconsistent = json!({"measured_to":4,"scan":[scan_step(3, true, false, false)]});
        assert!(scan_rows(&inconsistent).is_err());
    }

    #[test]
    fn draw_outcomes_are_restricted_to_the_registered_generator() {
        assert!(validate_draw_outcome(&json!({"outcome":{"kind":"satisfiable"}}), 1).is_ok());
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"at_least","degree":4}}), 0).is_ok()
        );
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"caps_hit","built":2}}), 0).is_ok()
        );
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"caps_hit","built":4}}), 0).is_err()
        );
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"caps_hit","built":0}}), 0).is_err()
        );
        assert!(validate_draw_outcome(&json!({"outcome":{"kind":"mystery"}}), 0).is_err());
        assert!(validate_draw_outcome(&json!({"outcome":{"kind":"satisfiable"}}), 0).is_err());
    }

    fn test_dir(label: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock is after epoch")
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "dreg-high-arity-analyze-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create test directory");
        path
    }

    #[test]
    fn jsonl_parser_rejects_blank_physical_rows() {
        let dir = test_dir("blank-jsonl");
        let path = dir.join("rows.jsonl");
        fs::write(&path, "{\"ok\":true}\n   \n").expect("write fixture");
        assert!(parse_json_lines(&path)
            .expect_err("blank row must fail")
            .contains("blank JSONL rows"));
        fs::remove_dir_all(dir).expect("remove test directory");
    }

    #[test]
    fn only_registered_atomic_temps_are_retryable_attempt_layout() {
        let dir = test_dir("attempt-layout");
        fs::write(dir.join(".meta.tmp.ABC123"), "partial").unwrap();
        let layout = inspect_attempt_layout(&dir).unwrap();
        assert!(!layout.allocated && !layout.process_evidence);
        fs::write(dir.join("unregistered.txt"), "evidence").unwrap();
        assert!(inspect_attempt_layout(&dir)
            .expect_err("unexpected evidence must invalidate")
            .contains("unexpected file"));
        fs::remove_dir_all(dir).expect("remove test directory");
    }

    #[test]
    fn allocation_marker_is_exact_and_counts_without_process_output() {
        let dir = test_dir("allocation");
        let marker = dir.join("allocated.json");
        fs::write(
            &marker,
            serde_json::to_vec(&json!({
                "schema":"dreg.high-arity-allocation/v1",
                "logical_name":"example",
                "attempt":"0001",
                "allocated_at":"2026-11-01T00:00:00Z"
            }))
            .unwrap(),
        )
        .unwrap();
        validate_allocation(&marker, "example", "0001").unwrap();
        let layout = inspect_attempt_layout(&dir).unwrap();
        assert!(layout.allocated && !layout.process_evidence);
        fs::remove_dir_all(dir).expect("remove test directory");
    }

    #[test]
    fn marker_only_allocation_does_not_consume_retry_budget() {
        let root = test_dir("allocated-budget");
        let logical = "example";
        let attempts = root.join("runs").join(logical).join("attempts");
        fs::create_dir_all(&attempts).unwrap();
        let name = "0001";
        let dir = attempts.join(name);
        fs::create_dir(&dir).unwrap();
        fs::write(
            dir.join("allocated.json"),
            serde_json::to_vec(&json!({
                "schema":"dreg.high-arity-allocation/v1",
                "logical_name":logical,
                "attempt":name,
                "allocated_at":"2026-11-01T00:00:00Z"
            }))
            .unwrap(),
        )
        .unwrap();
        let frozen: Result<FrozenContext, String> = Err("not needed for setup-only".into());
        let record = completed_record(
            &root,
            logical,
            &frozen,
            Path::new("unused.jsonl"),
            0,
            "unsat",
            400_000,
        );
        let summaries = match record {
            CompletedRecord::Unmeasured(_, attempts) => attempts,
            _ => panic!("marker-only attempts must remain unmeasured"),
        };
        assert!(summaries.is_empty());
        let row = process_row(
            logical.into(),
            "test",
            Cell::new(5, 11, 2),
            None,
            "unmeasured",
            "test".into(),
            None,
            summaries,
        );
        assert_eq!(row_exit_code(&row), 75);
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[cfg(unix)]
    #[test]
    fn evidence_readers_reject_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = test_dir("symlink");
        let target = dir.join("target.jsonl");
        let link = dir.join("link.jsonl");
        fs::write(&target, "{\"ok\":true}\n").unwrap();
        symlink(&target, &link).unwrap();
        assert!(parse_json_lines(&link)
            .expect_err("symlinked evidence must fail")
            .contains("regular non-symlink"));
        fs::remove_dir_all(dir).expect("remove test directory");
    }

    #[test]
    fn finished_receipt_distinguishes_resource_and_deterministic_failures() {
        let dir = test_dir("finished");
        let path = dir.join("finished.json");
        let stderr = dir.join("stderr.log");
        fs::write(&stderr, "").unwrap();
        fs::write(
            &path,
            serde_json::to_vec(&json!({
                "schema":"dreg.high-arity-finished/v1",
                "finished_at":"2026-11-01T00:00:00Z",
                "exit_code":124,
                "termination":"timeout"
            }))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            validate_finished(&path, 124, &stderr).unwrap(),
            AttemptTermination::Resource("timeout")
        );
        fs::write(
            &path,
            serde_json::to_vec(&json!({
                "schema":"dreg.high-arity-finished/v1",
                "finished_at":"2026-11-01T00:00:00Z",
                "exit_code":125,
                "termination":"checkpoint_invalid"
            }))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            validate_finished(&path, 125, &stderr).unwrap(),
            AttemptTermination::Resource("checkpoint_invalid")
        );
        fs::write(
            &path,
            serde_json::to_vec(&json!({
                "schema":"dreg.high-arity-finished/v1",
                "finished_at":"2026-11-01T00:00:00Z",
                "exit_code":101,
                "termination":"nonzero_exit"
            }))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            validate_finished(&path, 101, &stderr).unwrap(),
            AttemptTermination::DeterministicFailure
        );

        fs::write(
            &path,
            serde_json::to_vec(&json!({
                "schema":"dreg.high-arity-finished/v1",
                "finished_at":"2026-11-01T00:00:00Z",
                "exit_code":134,
                "termination":"address_space"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(&stderr, "memory allocation of 4096 bytes failed\n").unwrap();
        assert_eq!(
            validate_finished(&path, 134, &stderr).unwrap(),
            AttemptTermination::Resource("address_space")
        );
        fs::write(&stderr, "thread panicked with SIGABRT\n").unwrap();
        assert!(validate_finished(&path, 134, &stderr).is_err());
        fs::remove_dir_all(dir).expect("remove test directory");
    }

    #[test]
    fn checkpoint_restore_binds_embedded_committed_manifest() {
        let absent = json!({
            "checkpoint_restore":{
                "restored":false,
                "generation":null,
                "manifest_sha256":null,
                "manifest_json":null
            }
        });
        assert_eq!(validate_checkpoint_restore(&absent).unwrap(), (false, None));

        let manifest = concat!(
            "{\"schema\":\"spot.checkpoint/v1\",\"job_id\":\"dreg-closure\",",
            "\"spec_sha256\":\"0000000000000000000000000000000000000000000000000000000000000000\",",
            "\"generation\":3,\"committed\":true,\"files\":[{",
            "\"path\":\"catalog.json\",\"sha256\":\"1111111111111111111111111111111111111111111111111111111111111111\",",
            "\"bytes\":1}]}"
        );
        let restored = json!({
            "checkpoint_restore":{
                "restored":true,
                "generation":3,
                "manifest_sha256":sha256_bytes(manifest.as_bytes()),
                "manifest_json":manifest
            }
        });
        assert_eq!(
            validate_checkpoint_restore(&restored).unwrap(),
            (true, Some(3))
        );
        let mut changed = restored.clone();
        changed["checkpoint_restore"]["generation"] = json!(4);
        assert!(validate_checkpoint_restore(&changed).is_err());

        let wrap_manifest = |manifest: &Value| {
            let manifest = serde_json::to_string(manifest).unwrap();
            json!({
                "checkpoint_restore":{
                    "restored":true,
                    "generation":3,
                    "manifest_sha256":sha256_bytes(manifest.as_bytes()),
                    "manifest_json":manifest
                }
            })
        };
        let manifest_value: Value = serde_json::from_str(manifest).unwrap();

        let mut missing_catalog = manifest_value.clone();
        missing_catalog["files"][0]["path"] = json!("state.json");
        assert!(validate_checkpoint_restore(&wrap_manifest(&missing_catalog)).is_err());

        let mut duplicate_catalog = manifest_value.clone();
        let duplicate_entry = duplicate_catalog["files"][0].clone();
        duplicate_catalog["files"]
            .as_array_mut()
            .unwrap()
            .push(duplicate_entry);
        assert!(validate_checkpoint_restore(&wrap_manifest(&duplicate_catalog)).is_err());

        let mut leading_punctuation = manifest_value;
        leading_punctuation["files"][0]["path"] = json!(".catalog.json");
        assert!(validate_checkpoint_restore(&wrap_manifest(&leading_punctuation)).is_err());
    }

    fn checkpoint_attempt(restored: Option<bool>) -> AttemptSummary {
        AttemptSummary {
            attempt: "0001".into(),
            exit_status: Some(0),
            termination: Some("success".into()),
            execution_ref: Some("refs/heads/test".into()),
            execution_commit: Some("0".repeat(40)),
            checkpoint_restored: restored,
            checkpoint_generation: restored.and_then(|value| value.then_some(3)),
            state: "candidate_complete".into(),
            reason: "test".into(),
            output: "out.jsonl".into(),
            status: "status".into(),
            finished: "finished.json".into(),
            metadata: "meta.json".into(),
        }
    }

    #[test]
    fn successful_output_segments_bind_to_checkpoint_restore_metadata() {
        let fresh = vec![checkpoint_attempt(Some(false))];
        validate_checkpoint_segment_binding(&json!({"checkpoint_segments":1}), "0001", &fresh)
            .unwrap();
        assert!(validate_checkpoint_segment_binding(
            &json!({"checkpoint_segments":2}),
            "0001",
            &fresh,
        )
        .is_err());

        let restored = vec![checkpoint_attempt(Some(true))];
        validate_checkpoint_segment_binding(&json!({"checkpoint_segments":2}), "0001", &restored)
            .unwrap();
        validate_checkpoint_segment_binding(&json!({"checkpoint_segments":9}), "0001", &restored)
            .unwrap();
        assert!(validate_checkpoint_segment_binding(
            &json!({"checkpoint_segments":1}),
            "0001",
            &restored,
        )
        .is_err());
    }

    #[test]
    fn successful_output_requires_segment_and_validated_attempt_bindings() {
        let fresh = vec![checkpoint_attempt(Some(false))];
        assert!(validate_checkpoint_segment_binding(&json!({}), "0001", &fresh).is_err());
        assert!(validate_checkpoint_segment_binding(
            &json!({"checkpoint_segments":"1"}),
            "0001",
            &fresh,
        )
        .is_err());
        assert!(validate_checkpoint_segment_binding(
            &json!({"checkpoint_segments":1}),
            "0002",
            &fresh,
        )
        .is_err());
        assert!(validate_checkpoint_segment_binding(
            &json!({"checkpoint_segments":1}),
            "0001",
            &[checkpoint_attempt(None)],
        )
        .is_err());
    }

    fn test_row(classification: &str) -> ProcessRow {
        ProcessRow {
            run_id: classification.into(),
            kind: "test".into(),
            cell: "m5-n11-l2".into(),
            draw: Some(0),
            classification: classification.into(),
            reason: "test".into(),
            selected_attempt: None,
            attempts: Vec::new(),
        }
    }

    #[test]
    fn completed_above_four_is_not_hidden_by_unmeasured_draws() {
        let identity = test_row("passed");
        let controls = (0..6).map(|_| test_row("passed")).collect::<Vec<_>>();
        let unsat = vec![
            test_row("above_four"),
            test_row("unmeasured"),
            test_row("unmeasured"),
            test_row("unmeasured"),
        ];
        let cell = cell_classification(Cell::new(5, 11, 2), "ready", &identity, &controls, &unsat);
        assert_eq!(cell.classification, "not_universally_four_at_cell");
    }

    #[test]
    fn resource_unmeasured_controls_preserve_but_condition_above_four_evidence() {
        let identity = test_row("passed");
        let mut controls = (0..6).map(|_| test_row("passed")).collect::<Vec<_>>();
        controls[2] = test_row("unmeasured");
        let unsat = vec![
            test_row("above_four"),
            test_row("unmeasured"),
            test_row("unmeasured"),
            test_row("unmeasured"),
        ];
        let cell = cell_classification(Cell::new(5, 11, 2), "ready", &identity, &controls, &unsat);
        assert_eq!(cell.classification, "unmeasured");
        assert_eq!(cell.above_four, 1);
    }

    #[test]
    fn deterministic_failure_invalidates_same_cell_above_four() {
        let identity = test_row("passed");
        let controls = (0..6).map(|_| test_row("passed")).collect::<Vec<_>>();
        let unsat = vec![
            test_row("above_four"),
            test_row("invalid_record"),
            test_row("unmeasured"),
            test_row("unmeasured"),
        ];
        let cell = cell_classification(Cell::new(5, 11, 2), "ready", &identity, &controls, &unsat);
        assert_eq!(cell.classification, "invalid_instrument");
        assert_eq!(cell.above_four, 1);
    }

    #[test]
    fn resume_import_replays_immutable_manifest() {
        let dir = test_dir("resume");
        let artifact_relative = "runs/example/attempts/0001/status";
        let artifact = dir.join(artifact_relative);
        fs::create_dir_all(artifact.parent().unwrap()).unwrap();
        fs::write(&artifact, "124\n").unwrap();
        let artifact_hash = sha256_file(&artifact).unwrap();
        let stem = "resume-import-20261101T0000000Z";
        let copy_path = format!("{stem}.copy.sha256");
        let immutable_path = format!("{stem}.immutable.sha256");
        let anchor_path = format!("{stem}.anchor.sha256");
        fs::write(dir.join(&copy_path), "").unwrap();
        fs::write(
            dir.join(&immutable_path),
            format!("{artifact_hash}  {artifact_relative}\n"),
        )
        .unwrap();
        fs::copy(dir.join(&immutable_path), dir.join(&anchor_path)).unwrap();
        let record = json!({
            "schema":"dreg.resume-import/v3",
            "source":"/durable/source",
            "destination":fs::canonicalize(&dir).unwrap(),
            "imported_at":"2026-11-01T00:00:00Z",
            "copy_manifest":{
                "path":copy_path,
                "sha256":sha256_file(&dir.join(format!("{stem}.copy.sha256"))).unwrap(),
                "file_count":0
            },
            "immutable_manifest":{
                "path":immutable_path,
                "sha256":sha256_file(&dir.join(format!("{stem}.immutable.sha256"))).unwrap(),
                "file_count":1
            },
            "anchor_manifest":{
                "path":anchor_path,
                "sha256":sha256_file(&dir.join(format!("{stem}.anchor.sha256"))).unwrap(),
                "file_count":1
            }
        });
        fs::write(
            dir.join(format!("{stem}.json")),
            serde_json::to_vec(&record).unwrap(),
        )
        .unwrap();
        assert_eq!(validate_resume_imports(&dir).unwrap(), 1);
        fs::write(&artifact, "0\n").unwrap();
        assert!(validate_resume_imports(&dir).is_err());
        fs::remove_dir_all(dir).expect("remove test directory");
    }

    #[test]
    fn cell_formula_matches_registered_sizes() {
        assert_eq!(Cell::new(5, 11, 2).n_vars(), 43);
        assert_eq!(Cell::new(5, 13, 3).n_vars(), 54);
        assert_eq!(Cell::new(6, 11, 2).n_vars(), 56);
    }
}
