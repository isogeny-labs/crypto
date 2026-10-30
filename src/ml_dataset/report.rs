//! Immutable partial coverage and measurement reports. This is deliberately not
//! experiment.export/v1, a training feature export, or a fresh solver/replay run.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use super::storage;
use super::types::{Artifact, CaseConfig};

const INPUT_LIMIT: u64 = 32 * 1024 * 1024;
const ROW_LIMIT: usize = 32 * 1024 * 1024;
const CHUNK_BYTES: usize = 8 * 1024 * 1024;
const FAN_IN: usize = 8;
const MAX_DEPTH: usize = 32;

fn limited_file(root: &Path, name: &str) -> Result<PathBuf, String> {
    let path = storage::contained_path(root, name, false)?;
    let metadata = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > INPUT_LIMIT {
        return Err(format!(
            "report JSON must be an ordinary file of at most {INPUT_LIMIT} bytes: {}",
            path.display()
        ));
    }
    Ok(path)
}

fn read_json<T: DeserializeOwned>(root: &Path, name: &str) -> Result<T, String> {
    let path = limited_file(root, name)?;
    serde_json::from_reader(BufReader::new(
        File::open(path)
            .map_err(|e| e.to_string())?
            .take(INPUT_LIMIT + 1),
    ))
    .map_err(|e| format!("report input {name}: {e}"))
}

struct CappedBytes(Vec<u8>);
impl Write for CappedBytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > ROW_LIMIT.saturating_sub(self.0.len()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "report row byte limit exceeded",
            ));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn fresh(path: &Path) -> Result<File, String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path).map_err(|e| e.to_string())
}

#[derive(Eq, PartialEq)]
struct Row {
    key: String,
    bytes: Vec<u8>,
}

fn next(reader: &mut BufReader<File>) -> Result<Option<Row>, String> {
    let mut bytes = Vec::new();
    loop {
        let available = reader.fill_buf().map_err(|e| e.to_string())?;
        if available.is_empty() {
            if bytes.is_empty() {
                return Ok(None);
            }
            return Err("unterminated report sort row".into());
        }
        let newline = available.iter().position(|b| *b == b'\n');
        let count = newline.map_or(available.len(), |n| n + 1);
        if count > (ROW_LIMIT + 66).saturating_sub(bytes.len()) {
            return Err("report sort row exceeds byte limit".into());
        }
        bytes.extend_from_slice(&available[..count]);
        reader.consume(count);
        if newline.is_some() {
            break;
        }
    }
    if bytes.len() < 66 || bytes[64] != b'\t' {
        return Err("invalid report sort key".into());
    }
    let key = std::str::from_utf8(&bytes[..64])
        .map_err(|e| e.to_string())?
        .to_owned();
    crate::store::index::validate_sha256(&key)?;
    bytes.pop();
    Ok(Some(Row {
        key,
        bytes: bytes.split_off(65),
    }))
}

struct HeapRow {
    row: Row,
    reader: usize,
}
impl PartialEq for HeapRow {
    fn eq(&self, other: &Self) -> bool {
        self.row.key == other.row.key && self.reader == other.reader
    }
}
impl Eq for HeapRow {}
impl Ord for HeapRow {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .row
            .key
            .cmp(&self.row.key)
            .then_with(|| other.reader.cmp(&self.reader))
    }
}
impl PartialOrd for HeapRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn merge(paths: &[PathBuf], out: &Path, final_pass: bool) -> Result<u64, String> {
    if paths.len() > FAN_IN {
        return Err("report merge fan-in exceeded".into());
    }
    let mut readers = paths
        .iter()
        .map(|path| {
            File::open(path)
                .map(BufReader::new)
                .map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut heap = BinaryHeap::new();
    for (reader, input) in readers.iter_mut().enumerate() {
        if let Some(row) = next(input)? {
            heap.push(HeapRow { row, reader });
        }
    }
    let mut writer = BufWriter::new(fresh(out)?);
    let mut previous: Option<Row> = None;
    let mut written = 0;
    while let Some(item) = heap.pop() {
        let duplicate = previous.as_ref().is_some_and(|r| r.key == item.row.key);
        if duplicate && previous.as_ref().is_some_and(|r| r.bytes != item.row.bytes) {
            return Err("conflicting report row identity".into());
        }
        if !duplicate {
            if !final_pass {
                write!(writer, "{}\t", item.row.key).map_err(|e| e.to_string())?;
            }
            writer
                .write_all(&item.row.bytes)
                .and_then(|_| writer.write_all(b"\n"))
                .map_err(|e| e.to_string())?;
            written += 1;
        }
        previous = Some(item.row);
        if let Some(row) = next(&mut readers[item.reader])? {
            heap.push(HeapRow {
                row,
                reader: item.reader,
            });
        }
    }
    writer.flush().map_err(|e| e.to_string())?;
    writer.get_ref().sync_all().map_err(|e| e.to_string())?;
    Ok(written)
}

/// Rows are bounded individually; only one chunk and a fixed merge fan-in are
/// resident. Run-path metadata is logarithmic through eager level compaction.
struct Rows {
    dir: PathBuf,
    prefix: &'static str,
    sequence: u64,
    pending: Vec<Row>,
    bytes: usize,
    levels: Vec<Vec<PathBuf>>,
}
impl Rows {
    fn new(dir: &Path, prefix: &'static str) -> Self {
        Self {
            dir: dir.into(),
            prefix,
            sequence: 0,
            pending: vec![],
            bytes: 0,
            levels: vec![],
        }
    }
    fn path(&mut self) -> PathBuf {
        let path = self
            .dir
            .join(format!("{}-{:016}.run", self.prefix, self.sequence));
        self.sequence += 1;
        path
    }
    fn push(&mut self, identity: &str, value: &Value) -> Result<(), String> {
        let mut bytes = CappedBytes(Vec::new());
        serde_json::to_writer(&mut bytes, value).map_err(|e| e.to_string())?;
        self.bytes += bytes.0.len() + 66;
        self.pending.push(Row {
            key: super::digest(identity.as_bytes()),
            bytes: bytes.0,
        });
        if self.bytes >= CHUNK_BYTES {
            self.flush()?;
        }
        Ok(())
    }
    fn add_run(&mut self, mut path: PathBuf) -> Result<(), String> {
        let mut level = 0;
        loop {
            if self.levels.len() <= level {
                self.levels.push(vec![]);
            }
            self.levels[level].push(path);
            if self.levels[level].len() < FAN_IN {
                return Ok(());
            }
            let inputs = std::mem::take(&mut self.levels[level]);
            path = self.path();
            merge(&inputs, &path, false)?;
            for old in inputs {
                fs::remove_file(old).map_err(|e| e.to_string())?;
            }
            level += 1;
        }
    }
    fn flush(&mut self) -> Result<(), String> {
        if self.pending.is_empty() {
            return Ok(());
        }
        self.pending
            .sort_by(|a, b| a.key.cmp(&b.key).then_with(|| a.bytes.cmp(&b.bytes)));
        let path = self.path();
        let mut writer = BufWriter::new(fresh(&path)?);
        for row in &self.pending {
            write!(writer, "{}\t", row.key).map_err(|e| e.to_string())?;
            writer
                .write_all(&row.bytes)
                .and_then(|_| writer.write_all(b"\n"))
                .map_err(|e| e.to_string())?;
        }
        writer.flush().map_err(|e| e.to_string())?;
        writer.get_ref().sync_all().map_err(|e| e.to_string())?;
        self.pending.clear();
        self.bytes = 0;
        self.add_run(path)
    }
    fn finish(mut self, path: &Path) -> Result<u64, String> {
        self.flush()?;
        let mut runs: Vec<_> = std::mem::take(&mut self.levels)
            .into_iter()
            .flatten()
            .collect();
        while runs.len() > FAN_IN {
            let mut outputs = vec![];
            for group in runs.chunks(FAN_IN) {
                let output = self.path();
                merge(group, &output, false)?;
                for input in group {
                    fs::remove_file(input).map_err(|e| e.to_string())?;
                }
                outputs.push(output);
            }
            runs = outputs;
        }
        merge(&runs, path, true)
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn file_ref(root: &Path, relative: &str, role: &str, encoding: &str) -> Result<Artifact, String> {
    let path = storage::contained_path(root, relative, false)?;
    let (sha256, bytes) = crate::store::hash_file(&path)?;
    Ok(Artifact {
        role: role.into(),
        path: relative.into(),
        sha256,
        bytes,
        encoding: encoding.into(),
    })
}

fn axes(config: &CaseConfig) -> Value {
    let mut axes = json!({"family":config.family,"n":config.n,"twist":config.twist,"m":config.m,"solver":config.solver});
    if let Some(policy) = &config.factor_base_policy {
        axes["factor_base_policy"] = json!(policy);
        axes["factor_base_policy_sha256"] = json!(super::digest(
            &serde_json::to_vec(policy).expect("typed recipe serialization")
        ));
    }
    axes
}

fn increment(map: &mut BTreeMap<String, u64>, key: &str) -> Result<(), String> {
    if key.len() > 256 || (!map.contains_key(key) && map.len() >= 4096) {
        return Err("report aggregate category limit exceeded".into());
    }
    *map.entry(key.into()).or_default() += 1;
    Ok(())
}

#[derive(Default, Serialize)]
struct Counts {
    active_sealed_cells: u64,
    active_incomplete_cells: u64,
    retained_interrupted_attempts: u64,
    recorded_known_answer_verified_cells: u64,
    recorded_pdp_witness_verified_cells: u64,
    recorded_independent_scalar_recovery_cells: u64,
    recorded_solver_completed_cells: u64,
    by_family: BTreeMap<String, u64>,
    by_n: BTreeMap<String, u64>,
    by_twist: BTreeMap<String, u64>,
    by_m: BTreeMap<String, u64>,
    by_solver: BTreeMap<String, u64>,
    by_status: BTreeMap<String, u64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    by_factor_base_policy_recipe: BTreeMap<String, u64>,
}
impl Counts {
    fn active(&mut self, config: &CaseConfig, status: &str) -> Result<(), String> {
        increment(&mut self.by_family, &config.family)?;
        increment(&mut self.by_n, &config.n.to_string())?;
        increment(&mut self.by_twist, &config.twist.to_string())?;
        increment(&mut self.by_m, &config.m.to_string())?;
        increment(&mut self.by_solver, &config.solver)?;
        if let Some(policy) = &config.factor_base_policy {
            let hash = super::digest(&serde_json::to_vec(policy).map_err(|e| e.to_string())?);
            increment(
                &mut self.by_factor_base_policy_recipe,
                &format!("{}:{hash}", policy.name()),
            )?;
        }
        increment(&mut self.by_status, status)
    }
}

struct Streams {
    coverage: Rows,
    measurements: Rows,
    artifacts: Rows,
    observed: Rows,
}

fn inventory(
    streams: &mut Streams,
    attempt: &str,
    cell_index: u64,
    artifact: &Artifact,
    sealed: bool,
) -> Result<(), String> {
    streams.artifacts.push(
        &format!("{attempt}/{}", artifact.path),
        &json!({
        "schema":"crypto.ml-dataset.coverage-artifact/v1","attempt":attempt,"cell_index":cell_index,
        "manifest_bound":sealed,"artifact":artifact}),
    )
}

fn measurement(
    streams: &mut Streams,
    dir: &Path,
    attempt: &str,
    index: u64,
    artifact: &Artifact,
    sealed: bool,
) -> Result<(), String> {
    if !artifact.path.starts_with("telemetry/") || !artifact.path.ends_with(".json") {
        return Ok(());
    }
    let raw: Value = read_json(dir, &artifact.path)?;
    if matches!(
        raw["schema"].as_str(),
        Some("crypto.ml-dataset.phase-start/v2" | "crypto.ml-dataset.phase-metrics/v2")
    ) {
        let (kind, raw) = super::telemetry::checked::restricted_record(dir, &artifact.path)?;
        return streams.measurements.push(&format!("{attempt}/{}",artifact.path),&json!({
            "schema":"crypto.ml-dataset.coverage-measurement/v2","attempt":attempt,"cell_index":index,
            "manifest_bound":sealed,"kind":kind,"source_artifact":artifact,"raw":raw,
            "training_cost_eligible":false,"whole_worker_retired_instructions":null,
            "whole_worker_coverage_verified":false,"independently_verified":false,"consumer_v2_admitted":false,
            "platform_admitted":false,"clock_valid_for_comparison":false,"interpreted_as_whole_solve_instructions":false}));
    }
    if ![
        "crypto.ml-dataset.phase-start/v1",
        "crypto.ml-dataset.phase-metrics/v1",
    ]
    .contains(&raw["schema"].as_str().unwrap_or(""))
    {
        return Err("unknown report telemetry schema".into());
    }
    streams.measurements.push(&format!("{attempt}/{}", artifact.path), &json!({
        "schema":"crypto.ml-dataset.coverage-measurement/v1","attempt":attempt,"cell_index":index,
        "manifest_bound":sealed,"kind":"native_phase_record","source_artifact":artifact,
        "raw":raw,"clock_valid_for_comparison":false,"interpreted_as_whole_solve_instructions":false}))
}

fn walk_partial(
    streams: &mut Streams,
    dir: &Path,
    current: &Path,
    attempt: &str,
    index: u64,
    depth: usize,
    role: &str,
) -> Result<(u64, u64), String> {
    if depth > MAX_DEPTH {
        return Err("partial attempt directory depth limit exceeded".into());
    }
    let mut count = 0u64;
    let mut bytes = 0u64;
    for entry in fs::read_dir(current).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err("symlink in incomplete attempt".into());
        }
        if kind.is_dir() {
            let nested =
                walk_partial(streams, dir, &entry.path(), attempt, index, depth + 1, role)?;
            count += nested.0;
            bytes = bytes
                .checked_add(nested.1)
                .ok_or("artifact byte count overflow")?;
        } else if kind.is_file() {
            let path = entry.path();
            let relative = path
                .strip_prefix(dir)
                .map_err(|e| e.to_string())?
                .to_str()
                .ok_or("non-UTF8 attempt artifact")?;
            let artifact = file_ref(dir, relative, role, "native")?;
            inventory(streams, attempt, index, &artifact, false)?;
            // Incomplete telemetry JSON can be a truncated final write. Keep
            // its bytes and diagnostic rather than manufacture a measurement.
            if let Err(error) = measurement(streams, dir, attempt, index, &artifact, false) {
                streams.measurements.push(&format!("{attempt}/{relative}"), &json!({
                    "schema":"crypto.ml-dataset.coverage-measurement/v1","attempt":attempt,"cell_index":index,
                    "manifest_bound":false,"kind":"unreadable_partial_telemetry","source_artifact":artifact,"error":error,
                    "raw":null,"clock_valid_for_comparison":false}))?;
            }
            count += 1;
            bytes = bytes
                .checked_add(artifact.bytes)
                .ok_or("artifact byte count overflow")?;
        } else {
            return Err("nonordinary artifact in incomplete attempt".into());
        }
    }
    Ok((count, bytes))
}

fn index_name(path: &Path) -> Result<u64, String> {
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("non-UTF8 cell name")?;
    let index: u64 = name.parse().map_err(|_| "nonnumeric cell directory")?;
    if name != format!("{index:012}") {
        return Err("noncanonical cell directory".into());
    }
    Ok(index)
}

fn visit(
    streams: &mut Streams,
    counts: &mut Counts,
    root: &Path,
    dir: &Path,
    index: u64,
    retained: bool,
    plan: &super::DatasetPlan,
    binding: &Value,
    plan_sha: &str,
) -> Result<(), String> {
    let dir = super::checked_directory(dir, false)?;
    let attempt = dir
        .strip_prefix(root)
        .map_err(|e| e.to_string())?
        .to_str()
        .ok_or("non-UTF8 attempt path")?;
    let expected = plan.cell(index)?;
    let shard_count = binding["shard_count"]
        .as_u64()
        .ok_or("missing shard count")?;
    let shard_index = binding["shard_index"]
        .as_u64()
        .ok_or("missing shard index")?;
    if index % shard_count != shard_index {
        return Err("observed cell outside bound shard".into());
    }
    streams
        .observed
        .push(&index.to_string(), &json!({"cell_index":index}))?;
    let manifest_path = dir.join("manifest.json");
    if fs::symlink_metadata(&manifest_path).is_ok() {
        if retained {
            return Err(
                "retained interrupted attempt unexpectedly contains a completion manifest".into(),
            );
        }
        for path in [
            "manifest.json",
            "plan.json",
            "protocol.json",
            "run.json",
            "config.json",
        ] {
            limited_file(&dir, path)?;
        }
        let manifest = super::verify_cell(&dir, false)?;
        if manifest.cell_index != index
            || manifest.plan_sha256 != plan_sha
            || manifest.config != expected
            || read_json::<Value>(&dir, "run.json")? != *binding
        {
            return Err("report cell differs from bound run/plan".into());
        }
        let status = manifest.outcome["status"]
            .as_str()
            .ok_or("cell outcome has no status")?;
        counts.active_sealed_cells += 1;
        counts.active(&manifest.config, status)?;
        counts.recorded_known_answer_verified_cells +=
            u64::from(manifest.outcome["known_answer_verified"] == true);
        counts.recorded_pdp_witness_verified_cells +=
            u64::from(manifest.outcome["pdp_witness_verified"] == true);
        counts.recorded_independent_scalar_recovery_cells += u64::from(
            manifest.outcome["scalar_recovered_without_witness"] == true
                && manifest.outcome["scalar_replay_verified"] == true,
        );
        counts.recorded_solver_completed_cells +=
            u64::from(manifest.outcome["solver_completed"] == true);
        let manifest_ref = file_ref(&dir, "manifest.json", "cell-manifest", "json")?;
        let mut bytes = 0u64;
        for artifact in &manifest.artifacts {
            bytes = bytes
                .checked_add(artifact.bytes)
                .ok_or("artifact byte count overflow")?;
            inventory(streams, attempt, index, artifact, true)?;
            measurement(streams, &dir, attempt, index, artifact, true)?;
        }
        inventory(streams, attempt, index, &manifest_ref, true)?;
        let publication = dir.join("publication");
        let publication_inventory = if fs::symlink_metadata(&publication).is_ok() {
            let publication = super::checked_directory(&publication, false)?;
            walk_partial(
                streams,
                &dir,
                &publication,
                attempt,
                index,
                0,
                "local-publication-metadata",
            )?
        } else {
            (0, 0)
        };
        streams.measurements.push(&format!("{attempt}/native-outcome-counters"),&json!({
            "schema":"crypto.ml-dataset.coverage-measurement/v1","attempt":attempt,"cell_index":index,
            "manifest_bound":true,"kind":"native_solver_counters","source_artifact":manifest_ref,
            "counters":manifest.outcome.get("counters"),"counter_scope":manifest.outcome.get("counter_scope"),
            "supervisor":manifest.outcome.get("supervisor"),"clock_valid_for_comparison":false,
            "interpreted_as_retired_instructions":false}))?;
        if let (Some(_), Some(artifact)) = (
            manifest.config.factor_base_policy.as_ref(),
            manifest
                .artifacts
                .iter()
                .find(|a| a.path == "factor-base.json"),
        ) {
            let base: Value = read_json(&dir, "factor-base.json")?;
            streams.measurements.push(&format!("{attempt}/factor-base-construction"), &json!({
                "schema":"crypto.ml-dataset.coverage-measurement/v1","attempt":attempt,"cell_index":index,
                "manifest_bound":true,"kind":"factor_base_construction","source_artifact":artifact,
                "policy":manifest.config.factor_base_policy,"status":base["construction_status"],
                "coordinate_domain_complete":base["coordinate_domain_complete"],
                "actual_points":base["points"].as_array().map(Vec::len),"counters":base["construction_counters"],
                "clock_valid_for_comparison":false,"interpreted_as_retired_instructions":false}))?;
        }
        streams.coverage.push(attempt,&json!({"schema":"crypto.ml-dataset.coverage-cell/v1","cell_index":index,
            "attempt":attempt,"attempt_kind":"active_sealed","status":status,"actual_axes":axes(&manifest.config),
            "manifest":manifest_ref,"member_artifacts":manifest.artifacts.len(),"member_artifact_bytes":bytes,
            "local_publication_metadata_files":publication_inventory.0,"local_publication_metadata_bytes":publication_inventory.1,
            "recorded_labels":{"known_answer_verified":manifest.outcome.get("known_answer_verified"),
                "factor_base_verified":manifest.outcome.get("factor_base_verified"),"pdp_witness_verified":manifest.outcome.get("pdp_witness_verified"),
                "scalar_recovered_without_witness":manifest.outcome.get("scalar_recovered_without_witness"),
                "scalar_replay_verified":manifest.outcome.get("scalar_replay_verified"),"solver_completed":manifest.outcome.get("solver_completed")},
            "artifact_hashes_verified":true,"certificate_replay_performed":false,
            "exact_first_fall_degree":null,"exact_last_fall_degree":null}))?;
    } else {
        let actual = if fs::symlink_metadata(dir.join("config.json")).is_ok() {
            let config: CaseConfig = read_json(&dir, "config.json")?;
            if config != expected {
                return Err("incomplete attempt config differs from plan".into());
            }
            Some(config)
        } else {
            None
        };
        for path in ["plan.json", "run.json", "protocol.json"] {
            if fs::symlink_metadata(dir.join(path)).is_ok() {
                let reference = file_ref(root, path, "root-provenance", "json")?;
                storage::verify_manifest(&dir, std::slice::from_ref(&reference))?;
            }
        }
        let (count, bytes) = walk_partial(
            streams,
            &dir,
            &dir,
            attempt,
            index,
            0,
            "unsealed-attempt-artifact",
        )?;
        if retained {
            counts.retained_interrupted_attempts += 1;
        } else {
            counts.active_incomplete_cells += 1;
            if let Some(config) = &actual {
                counts.active(config, "incomplete_attempt")?;
            } else {
                increment(&mut counts.by_status, "incomplete_attempt")?;
            }
        }
        streams.coverage.push(attempt,&json!({"schema":"crypto.ml-dataset.coverage-cell/v1","cell_index":index,
            "attempt":attempt,"attempt_kind":if retained {"retained_interrupted"} else {"active_incomplete"},
            "status":"incomplete_attempt","actual_axes":actual.as_ref().map(axes),"planned_axes":axes(&expected),
            "observed_artifacts":count,"observed_artifact_bytes":bytes,"manifest":null,"recorded_labels":null,
            "artifact_hashes_verified":false,"observed_bytes_hashed":true,"certificate_replay_performed":false,
            "exact_first_fall_degree":null,"exact_last_fall_degree":null}))?;
    }
    Ok(())
}

fn install(root: &Path, source: &Path, name: &str) -> Result<Artifact, String> {
    let (sha256, bytes) = crate::store::hash_file(source)?;
    let artifact = Artifact {
        path: name.into(),
        role: "restricted-coverage-report".into(),
        encoding: "jsonl".into(),
        sha256,
        bytes,
    };
    let destination = storage::contained_path(root, name, true)?;
    match fs::hard_link(source, &destination) {
        Ok(()) => storage::sync_directory(destination.parent().ok_or("report has no parent")?)?,
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            storage::verify_manifest(root, std::slice::from_ref(&artifact))?
        }
        Err(e) => return Err(e.to_string()),
    }
    Ok(artifact)
}

fn output_directory(root: &Path, requested: &Path) -> Result<PathBuf, String> {
    let absolute = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(requested)
    };
    if absolute
        .components()
        .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err("parent traversal in report destination".into());
    }
    let mut ancestor = absolute.as_path();
    loop {
        match fs::symlink_metadata(ancestor) {
            Ok(_) => break,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                ancestor = ancestor
                    .parent()
                    .ok_or("report destination has no existing ancestor")?
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    let parent = super::checked_directory(ancestor, false)?;
    let candidate = parent.join(absolute.strip_prefix(ancestor).map_err(|e| e.to_string())?);
    if root.starts_with(&candidate) || candidate.starts_with(root) {
        return Err("coverage report must be outside its source run tree".into());
    }
    super::checked_directory(&candidate, true)
}

/// Hash and describe observed artifacts without scientific computation. A new
/// snapshot requires a new destination; byte-identical retries are idempotent.
pub fn report_run(root: &Path, destination: &Path) -> Result<Value, String> {
    let root = super::checked_directory(root, false)?;
    let _source_lease = super::Lease::acquire(&root)?;
    super::assert_no_workers(&root)?;
    for path in ["plan.json", "protocol.json", "run.json"] {
        limited_file(&root, path)?;
    }
    let (plan, binding, plan_sha) = super::bound_run(&root)?;
    let shard_count = binding["shard_count"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("invalid shard count")?;
    let shard_index = binding["shard_index"]
        .as_u64()
        .filter(|n| *n < shard_count)
        .ok_or("invalid shard index")?;
    let destination = output_directory(&root, destination)?;
    let _out_lease = super::Lease::acquire(&destination)?;
    let scratch = Scratch(destination.join(format!(
        ".coverage-staging-{}-{}",
        std::process::id(),
        super::unix_nanos()?
    )));
    fs::create_dir(&scratch.0).map_err(|e| e.to_string())?;
    let mut streams = Streams {
        coverage: Rows::new(&scratch.0, "coverage"),
        measurements: Rows::new(&scratch.0, "measurements"),
        artifacts: Rows::new(&scratch.0, "artifacts"),
        observed: Rows::new(&scratch.0, "observed"),
    };
    let mut counts = Counts::default();
    let cells = root.join("cells");
    if fs::symlink_metadata(&cells).is_ok() {
        let cells = super::checked_directory(&cells, false)?;
        for entry in fs::read_dir(cells).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                return Err("nonordinary cell directory".into());
            }
            visit(
                &mut streams,
                &mut counts,
                &root,
                &entry.path(),
                index_name(&entry.path())?,
                false,
                &plan,
                &binding,
                &plan_sha,
            )?;
        }
    }
    let retained = root.join("interrupted-attempts");
    if fs::symlink_metadata(&retained).is_ok() {
        let retained = super::checked_directory(&retained, false)?;
        for entry in fs::read_dir(retained).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let index = index_name(&entry.path())?;
            let attempts = super::checked_directory(&entry.path(), false)?;
            for attempt in fs::read_dir(attempts).map_err(|e| e.to_string())? {
                let attempt = attempt.map_err(|e| e.to_string())?;
                if !attempt.file_type().map_err(|e| e.to_string())?.is_dir() {
                    return Err("nonordinary retained attempt".into());
                }
                visit(
                    &mut streams,
                    &mut counts,
                    &root,
                    &attempt.path(),
                    index,
                    true,
                    &plan,
                    &binding,
                    &plan_sha,
                )?;
            }
        }
    }
    let mut outputs = Vec::new();
    let mut row_counts = BTreeMap::new();
    for (name, rows) in [
        ("coverage.jsonl", streams.coverage),
        ("measurements.jsonl", streams.measurements),
        ("artifacts.jsonl", streams.artifacts),
        ("observed-cells.jsonl", streams.observed),
    ] {
        let staged = scratch.0.join(name);
        row_counts.insert(name, rows.finish(&staged)?);
        outputs.push(install(&destination, &staged, name)?);
    }
    let total = plan.cell_count()?;
    let assigned = if shard_index >= total {
        0
    } else {
        (total - 1 - shard_index) / shard_count + 1
    };
    let observed = row_counts["observed-cells.jsonl"];
    let unseen = assigned
        .checked_sub(observed)
        .ok_or("more observed cells than bound shard")?;
    let protocol: Value = read_json(&root, "protocol.json")?;
    let source_files = ["plan.json", "run.json", "protocol.json"]
        .iter()
        .map(|path| file_ref(&root, path, "source-provenance", "json"))
        .collect::<Result<Vec<_>, _>>()?;
    let summary = json!({"schema":"crypto.ml-dataset.coverage/v1","source_commit":binding["source_commit"],
        "binary_sha256":binding["binary_sha256"],"source_run_sha256":source_files[1].sha256,
        "plan_sha256":plan_sha,"protocol_file_sha256":plan.protocol_sha256,
        "protocol_body_sha256":protocol["freeze"]["protocol_sha256"],"source_files":source_files,
        "shard_index":shard_index,"shard_count":shard_count,"planned_method_cells":total,"assigned_method_cells":assigned,
        "observed_distinct_cell_identities":observed,"assigned_cells_without_observed_local_attempt":unseen,
        "counts":counts,"row_counts":row_counts,"artifacts":outputs,
        "partial":counts.active_sealed_cells<total,"shard_partial":counts.active_sealed_cells<assigned,
        "shard_attempts_complete":counts.active_sealed_cells==assigned,
        "campaign_attempts_complete":counts.active_sealed_cells==total,
        "coverage_scope":"one bound local run/shard; includes retained interrupted attempts; unattempted slots produce no rows",
        "axis_aggregate_scope":"active attempts with an actual saved config; retained attempts are counted separately; missing configs do not imply observed axes",
        "labels":"recorded producer labels with artifact/provenance hashes and required certificate presence checked",
        "certificate_replay_performed":false,"solver_rerun_performed":false,"remote_rechecked":false,
        "experiment_export_schema":false,"complete_protocol_export_validation_performed":false,
        "exact_first_fall_degree":null,"exact_last_fall_degree":null,"whole_solve_instruction_count":null,
        "clock_valid_for_comparison":false,"report_is_training_features":false,
        "bounds":{"input_json_bytes":INPUT_LIMIT,"serialized_row_bytes":ROW_LIMIT,"sort_chunk_bytes":CHUNK_BYTES,"merge_fan_in":FAN_IN,
            "source_cell_values_materialized":true,"total_process_rss_bound_claimed":false}});
    let artifact = storage::write_json(
        &destination,
        "summary.json",
        "coverage-report-summary",
        &summary,
    )?;
    storage::verify_manifest(&destination, &outputs)?;
    Ok(json!({"summary_artifact":artifact,"summary":summary}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::types::FactorBasePolicy;
    use crate::ml_dataset::{curves, solvers, CellManifest, DatasetPlan};

    struct Fixture {
        root: PathBuf,
        source: PathBuf,
        plan: DatasetPlan,
        binding: Value,
        protocol: Vec<u8>,
    }
    impl Fixture {
        fn new() -> Self {
            Self::with_policies(Vec::new())
        }
        fn with_policies(policies: Vec<FactorBasePolicy>) -> Self {
            let root = std::env::temp_dir().join(format!(
                "crypto-coverage-test-{}-{}",
                std::process::id(),
                super::super::unix_nanos().unwrap()
            ));
            let source = root.join("source");
            fs::create_dir_all(&source).unwrap();
            let mut protocol =
                json!({"schema":"experiment.protocol/v1","purpose":"coverage unit fixture"});
            let body_hash =
                super::super::digest(&super::super::protocol_canonical_bytes(&protocol).unwrap());
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":body_hash,"filed_before_affected_data":true,"frozen_at":"2026-10-28T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let plan = DatasetPlan {
                schema: if policies.is_empty() {
                    "crypto.ml-dataset.plan/v1"
                } else {
                    "crypto.ml-dataset.plan/v2"
                }
                .into(),
                field_sizes: vec![7],
                arities: vec![2],
                families: vec!["koblitz".into()],
                solvers: vec!["bsgs".into(), "enumerate".into()],
                curve_seeds: 1,
                generators: 1,
                keys: 2,
                seed: 19,
                factor_base_size: 4,
                degree_bound: 3,
                work_limit: 10000,
                twists: true,
                protocol_sha256: Some(super::super::digest(&protocol)),
                split_rule: "curve-twist-component-sha256-v1".into(),
                factor_base_policies: policies,
            };
            let binding = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":super::super::digest(&serde_json::to_vec(&plan).unwrap()),
                "source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64),"shard_index":0,"shard_count":1,
                "seconds_per_case":10,"memory_mib":256,"trace_mib":64});
            storage::write_json(&source, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&source, "run.json", "run", &binding).unwrap();
            storage::write_bytes(&source, "protocol.json", "protocol", "json", &protocol).unwrap();
            Self {
                root,
                source,
                plan,
                binding,
                protocol,
            }
        }
        fn checked_cell(&self, index: u64) -> PathBuf {
            let dir = self.source.join("cells").join(format!("{index:012}"));
            fs::create_dir_all(&dir).unwrap();
            storage::write_json(&dir, "plan.json", "restricted-frozen-plan", &self.plan).unwrap();
            storage::write_json(&dir, "run.json", "restricted-run-binding", &self.binding).unwrap();
            storage::write_bytes(
                &dir,
                "protocol.json",
                "restricted-frozen-protocol",
                "json",
                &self.protocol,
            )
            .unwrap();
            storage::write_json(
                &dir,
                "config.json",
                "restricted-provenance",
                &self.plan.cell(index).unwrap(),
            )
            .unwrap();
            storage::write_json(
                &dir,
                "attempt.json",
                "attempt-provenance",
                &json!({
                    "schema":"crypto.ml-dataset.attempt/v1", "cell_index":index,
                    "plan_sha256":self.binding["plan_sha256"], "started_unix_nanos":"1"
                }),
            )
            .unwrap();
            for path in ["stdout.log", "stderr.log"] {
                storage::write_bytes(&dir, path, "process-log", "utf8", b"").unwrap();
            }
            super::super::telemetry::checked::install_policy(&dir).unwrap();
            let binding = super::super::capture::AttemptBinding::from_cell_files(&dir).unwrap();
            storage::write_json(
                &dir,
                "capture-binding.json",
                "partial-capture-binding",
                &binding,
            )
            .unwrap();
            storage::write_json(
                &dir,
                "capture-limits.json",
                "partial-capture-limits",
                &super::super::capture::capture_limits(64).unwrap(),
            )
            .unwrap();
            dir
        }
        fn seal_checked(&self, dir: &Path, index: u64, outcome: Value) -> CellManifest {
            storage::write_json(
                dir,
                "worker-process.json",
                "process-provenance",
                &json!({"software_fixture":true}),
            )
            .unwrap();
            let manifest = CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: self.binding["plan_sha256"].as_str().unwrap().into(),
                config: self.plan.cell(index).unwrap(),
                outcome,
                artifacts: super::super::collect_cell_artifacts(dir).unwrap(),
            };
            storage::write_json(dir, "manifest.json", "cell-manifest", &manifest).unwrap();
            manifest
        }
        fn sealed(&self, index: u64) {
            let dir = self.source.join("cells").join(format!("{index:012}"));
            fs::create_dir_all(&dir).unwrap();
            let config = self.plan.cell(index).unwrap();
            let case = curves::generate(&config).unwrap();
            let result = solvers::solve(&config, &case).unwrap();
            assert_eq!(result.outcome["scalar_recovered_without_witness"], true);
            let mut artifacts = vec![
                storage::write_json(&dir, "plan.json", "plan", &self.plan).unwrap(),
                storage::write_json(&dir, "run.json", "run", &self.binding).unwrap(),
                storage::write_bytes(&dir, "protocol.json", "protocol", "json", &self.protocol)
                    .unwrap(),
                storage::write_json(&dir, "config.json", "restricted-config", &config).unwrap(),
                storage::write_json(
                    &dir,
                    "public.json",
                    "public-instance",
                    &case.public_instance,
                )
                .unwrap(),
                storage::write_json(
                    &dir,
                    "witness.json",
                    "restricted-known-answer",
                    &case.private_witness,
                )
                .unwrap(),
                storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                    .unwrap(),
            ];
            for path in ["attempt.json", "worker-process.json"] {
                artifacts.push(
                    storage::write_json(&dir, path, "provenance", &json!({"unit_test":true}))
                        .unwrap(),
                );
            }
            for path in ["stdout.log", "stderr.log"] {
                artifacts
                    .push(storage::write_bytes(&dir, path, "log", "utf8", b"fixture").unwrap());
            }
            let metric = json!({"schema":"crypto.ml-dataset.phase-metrics/v1","phase":"solver","instructions_status":"measured_multiplexed_estimate",
                "retired_instructions_raw":77,"retired_instructions_exact_in_scope":null,
                "hardware":{"instructions":77,"instructions_time_enabled_ns":100,"instructions_time_running_ns":50,"instructions_scaled":154,"multiplexed":true,"inherit":false},
                "counter_scope":"fixture calling thread user space","clock_valid_for_comparison":false});
            artifacts.push(
                storage::write_json(
                    &dir,
                    "telemetry/solver.json",
                    "hardware-and-resource-telemetry",
                    &metric,
                )
                .unwrap(),
            );
            for artifact in result.artifacts {
                artifacts.push(
                    storage::write_bytes(
                        &dir,
                        &artifact.path,
                        &artifact.role,
                        &artifact.encoding,
                        &artifact.bytes,
                    )
                    .unwrap(),
                );
            }
            let mut outcome = result.outcome;
            outcome["known_answer_verified"] = json!(true);
            outcome["factor_base_verified"] = json!(true);
            let manifest = CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: self.binding["plan_sha256"].as_str().unwrap().into(),
                config,
                outcome,
                artifacts,
            };
            storage::write_json(&dir, "manifest.json", "cell-manifest", &manifest).unwrap();
        }
        // Test-only opt-in export of the already verified sealed fixture. This
        // never changes acquisition, solver inputs, runtime dispatch or claims.
        fn retain_checked(&self, name: &str) {
            let Some(root) = std::env::var_os("CRYPTO_ML_CHECKED_FIXTURE_DIR") else {
                return;
            };
            let root = PathBuf::from(root);
            assert!(root.is_absolute());
            let destination = root.join(name);
            fs::create_dir(&destination).unwrap();
            let cell = "cells/000000000000";
            let manifest: CellManifest =
                super::super::read_json(&self.source.join(cell).join("manifest.json")).unwrap();
            let mut paths = std::collections::BTreeSet::from([
                "run.json".to_owned(),
                "plan.json".to_owned(),
                "protocol.json".to_owned(),
                format!("{cell}/manifest.json"),
            ]);
            paths.extend(
                manifest
                    .artifacts
                    .iter()
                    .map(|a| format!("{cell}/{}", a.path)),
            );
            assert!(paths.len() <= 256);
            let mut total = 0u64;
            for relative in paths {
                let source = storage::contained_path(&self.source, &relative, false).unwrap();
                let metadata = fs::symlink_metadata(&source).unwrap();
                assert!(metadata.is_file());
                total = total.checked_add(metadata.len()).unwrap();
                assert!(total <= 32 * 1024 * 1024);
                let bytes = fs::read(&source).unwrap();
                assert_eq!(bytes.len() as u64, metadata.len());
                storage::write_bytes(
                    &destination,
                    &relative,
                    "retained-test-fixture",
                    "native",
                    &bytes,
                )
                .unwrap();
            }
        }
        fn partial(&self, index: u64, retained: bool) {
            let dir = if retained {
                self.source
                    .join("interrupted-attempts")
                    .join(format!("{index:012}"))
                    .join("saved-attempt")
            } else {
                self.source.join("cells").join(format!("{index:012}"))
            };
            fs::create_dir_all(&dir).unwrap();
            storage::write_json(
                &dir,
                "config.json",
                "config",
                &self.plan.cell(index).unwrap(),
            )
            .unwrap();
            storage::write_bytes(&dir, "telemetry/solver.json", "partial", "json", b"{").unwrap();
            storage::write_bytes(
                &dir,
                "partial.bin",
                "partial",
                "binary",
                b"saved intermediate",
            )
            .unwrap();
        }
    }
    #[test]
    fn checked_actual_worker_seal_verify_and_report_use_mock_counters() {
        use super::super::telemetry::checked;
        let f = Fixture::new();
        let dir = f.checked_cell(0);
        let (result, opens) = checked::tests::with_mock_counters(|| {
            super::super::worker(&f.plan.cell(0).unwrap(), &dir)
        });
        result.unwrap();
        assert!(
            dir.join("live/committed.json").is_file(),
            "actual worker must use bound capture"
        );
        assert_eq!(
            opens, 4,
            "all four real worker phases must use the mock factory"
        );
        let expected_roles = checked::validate_directory(&dir).unwrap();
        assert_eq!(expected_roles.len(), 9);
        let registry: Vec<Artifact> =
            super::super::read_json(&dir.join("worker-artifacts.json")).unwrap();
        // Check the worker's own output before supervisor recovery could fill
        // omitted entries and hide a regression at the registry boundary.
        for (path, role) in expected_roles {
            let registered: Vec<_> = registry.iter().filter(|a| a.path == path).collect();
            assert_eq!(registered.len(), 1, "missing or duplicate {path}");
            let artifact = registered[0];
            let bytes = fs::read(dir.join(&path)).unwrap();
            assert_eq!(artifact.role, role);
            assert_eq!(artifact.encoding, "json");
            assert_eq!(artifact.sha256, super::super::digest(&bytes));
            assert_eq!(artifact.bytes, bytes.len() as u64);
        }
        assert_eq!(
            registry
                .iter()
                .filter(|a| checked::role(&a.path).is_some())
                .count(),
            9
        );
        storage::verify_manifest(&dir, &registry).unwrap();
        let outcome: Value = super::super::read_json(&dir.join("result.json")).unwrap();
        assert_eq!(outcome["known_answer_verified"], true);
        assert_eq!(outcome["factor_base_verified"], true);
        f.seal_checked(&dir, 0, outcome);
        super::super::verify_cell(&dir, true).unwrap();
        let output = f.root.join("checked-report");
        report_run(&f.source, &output).unwrap();
        let rows = jsonl(&output.join("measurements.jsonl"));
        let phases: Vec<_> = rows
            .iter()
            .filter(|row| row["schema"] == "crypto.ml-dataset.coverage-measurement/v2")
            .collect();
        assert_eq!(phases.len(), 8);
        for row in phases {
            assert_eq!(row["manifest_bound"], true);
            assert_eq!(row["kind"], "native_phase_record");
            for flag in [
                "training_cost_eligible",
                "whole_worker_coverage_verified",
                "independently_verified",
                "consumer_v2_admitted",
                "platform_admitted",
                "clock_valid_for_comparison",
                "interpreted_as_whole_solve_instructions",
            ] {
                assert_eq!(row[flag], false, "{flag}");
            }
            assert_eq!(row["whole_worker_retired_instructions"], Value::Null);
            let artifact = &row["source_artifact"];
            let bytes = fs::read(dir.join(artifact["path"].as_str().unwrap())).unwrap();
            assert_eq!(artifact["sha256"], super::super::digest(&bytes));
            assert_eq!(artifact["bytes"], bytes.len());
        }
        // Restricted phase/report fields must not affect the public projection.
        let checked_export = f.root.join("checked-export");
        super::super::export::export(&f.source, &checked_export).unwrap();
        let public_features = fs::read(checked_export.join("features.jsonl")).unwrap();
        f.retain_checked("sealed-worker");
        let mut manifest: CellManifest =
            super::super::read_json(&dir.join("manifest.json")).unwrap();
        manifest.artifacts.retain(|a| {
            a.path != checked::POLICY_PATH
                && !a.path.starts_with("telemetry/")
                && a.path != "capture-binding.json"
                && a.path != "capture-limits.json"
        });
        fs::remove_file(dir.join("capture-binding.json")).unwrap();
        fs::remove_file(dir.join("capture-limits.json")).unwrap();
        fs::remove_file(dir.join(checked::POLICY_PATH)).unwrap();
        fs::remove_dir_all(dir.join("telemetry")).unwrap();
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_vec_pretty(&manifest).unwrap(),
        )
        .unwrap();
        let legacy_export = f.root.join("without-diagnostics-export");
        super::super::export::export(&f.source, &legacy_export).unwrap();
        assert_eq!(
            fs::read(legacy_export.join("features.jsonl")).unwrap(),
            public_features
        );
    }

    #[test]
    fn checked_registryless_start_only_survives_seal_verify_and_report() {
        use super::super::telemetry::checked::{self, types::Phase};
        let f = Fixture::new();
        let dir = f.checked_cell(0);
        let binding = super::super::capture::AttemptBinding::from_cell_files(&dir).unwrap();
        checked::tests::write_phase(&dir, &binding, Phase::CurveConstruction, true);
        assert!(!dir.join("worker-artifacts.json").exists());
        let manifest = f.seal_checked(&dir, 0, json!({"status":"timeout", "known_answer_verified":false,"factor_base_verified":false,"solver_completed":false}));
        let start = manifest
            .artifacts
            .iter()
            .find(|a| a.path == Phase::CurveConstruction.path(true))
            .unwrap();
        assert_eq!(start.role, "phase-start");
        assert_eq!(start.encoding, "json");
        super::super::verify_cell(&dir, false).unwrap();
        let output = f.root.join("start-report");
        report_run(&f.source, &output).unwrap();
        let rows = jsonl(&output.join("measurements.jsonl"));
        let start = rows
            .iter()
            .find(|row| row["schema"] == "crypto.ml-dataset.coverage-measurement/v2")
            .unwrap();
        assert_eq!(start["kind"], "interrupted_or_unfinished");
        assert_eq!(start["raw"]["schema"], "crypto.ml-dataset.phase-start/v2");
        assert_eq!(start["training_cost_eligible"], false);
        assert_eq!(start["whole_worker_retired_instructions"], Value::Null);
        f.retain_checked("sealed-registryless-prefix");
    }

    #[test]
    fn checked_corrupt_partial_is_unpromoted_and_sealed_corruption_is_rejected() {
        use super::super::telemetry::checked::{self, types::Phase};
        let f = Fixture::new();
        let dir = f.checked_cell(0);
        let binding = super::super::capture::AttemptBinding::from_cell_files(&dir).unwrap();
        checked::tests::write_phase(&dir, &binding, Phase::CurveConstruction, true);
        let path = Phase::CurveConstruction.path(true);
        let mut start: Value = super::super::read_json(&dir.join(&path)).unwrap();
        start["binding"]["config_sha256"] = json!("0".repeat(64));
        fs::write(dir.join(&path), serde_json::to_vec(&start).unwrap()).unwrap();
        let artifacts = super::super::collect_cell_artifacts(&dir).unwrap();
        assert_eq!(
            artifacts.iter().find(|a| a.path == path).unwrap().role,
            "unregistered-partial-artifact"
        );
        let output = f.root.join("corrupt-partial-report");
        report_run(&f.source, &output).unwrap();
        let rows = jsonl(&output.join("measurements.jsonl"));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["kind"], "unreadable_partial_telemetry");
        assert_eq!(rows[0]["raw"], Value::Null);
        f.seal_checked(
            &dir,
            0,
            json!({"status":"timeout", "known_answer_verified":false,"factor_base_verified":false}),
        );
        assert!(super::super::verify_cell(&dir, false).is_err());
        assert!(report_run(&f.source, &f.root.join("corrupt-sealed-report")).is_err());
    }

    #[test]
    fn checked_actual_worker_rejects_binding_and_policy_before_counter_open() {
        use super::super::telemetry::checked;
        for corrupt_policy in [false, true] {
            let f = Fixture::new();
            let dir = f.checked_cell(0);
            let mut config = f.plan.cell(0).unwrap();
            if corrupt_policy {
                fs::write(dir.join(checked::POLICY_PATH), b"{}\n").unwrap();
            } else {
                config.work_limit += 1;
            }
            let (result, opens) =
                checked::tests::with_mock_counters(|| super::super::worker(&config, &dir));
            assert!(result.is_err());
            assert_eq!(opens, 0);
            assert!(!dir.join("telemetry").exists());
            assert!(!dir.join("worker-artifacts.json").exists());
        }
    }
    #[test]
    fn checked_capture_context_mutations_fail_sealed_verification() {
        use super::super::telemetry::checked::{self, types::Phase};
        for mutation in [
            "binding",
            "limits",
            "unknown_limits",
            "omitted",
            "relocated",
            "wrong_role",
            "wrong_encoding",
            "policy_removed",
        ] {
            let f = Fixture::new();
            let dir = f.checked_cell(0);
            let binding = super::super::capture::AttemptBinding::from_cell_files(&dir).unwrap();
            checked::tests::write_phase(&dir, &binding, Phase::CurveConstruction, true);
            let mut manifest = f.seal_checked(
                &dir,
                0,
                json!({"status":"timeout","known_answer_verified":false,
                    "factor_base_verified":false,"solver_completed":false}),
            );
            super::super::verify_cell(&dir, false).unwrap();
            match mutation {
                "binding" | "limits" | "unknown_limits" => {
                    let path = if mutation == "binding" {
                        "capture-binding.json"
                    } else {
                        "capture-limits.json"
                    };
                    let mut value: Value = super::super::read_json(&dir.join(path)).unwrap();
                    match mutation {
                        "binding" => value["attempt_sha256"] = json!("0".repeat(64)),
                        "limits" => {
                            // Still structurally valid: only the frozen derivation differs.
                            value["max_payload_count"] = json!(4095);
                            let limits: super::super::capture::CaptureLimits =
                                serde_json::from_value(value.clone()).unwrap();
                            limits.validate().unwrap();
                        }
                        _ => value["unregistered_limit"] = json!(1),
                    }
                    let bytes = serde_json::to_vec(&value).unwrap();
                    fs::write(dir.join(path), &bytes).unwrap();
                    let artifact = manifest
                        .artifacts
                        .iter_mut()
                        .find(|a| a.path == path)
                        .unwrap();
                    artifact.sha256 = super::super::digest(&bytes);
                    artifact.bytes = bytes.len() as u64;
                }
                "omitted" => manifest
                    .artifacts
                    .retain(|a| a.path != "capture-binding.json"),
                "relocated" => {
                    fs::rename(
                        dir.join("capture-binding.json"),
                        dir.join("renamed-binding.json"),
                    )
                    .unwrap();
                    manifest
                        .artifacts
                        .iter_mut()
                        .find(|a| a.path == "capture-binding.json")
                        .unwrap()
                        .path = "renamed-binding.json".into();
                }
                "wrong_role" => {
                    manifest
                        .artifacts
                        .iter_mut()
                        .find(|a| a.path == "capture-binding.json")
                        .unwrap()
                        .role = "unregistered-partial-artifact".into();
                }
                "wrong_encoding" => {
                    manifest
                        .artifacts
                        .iter_mut()
                        .find(|a| a.path == "capture-limits.json")
                        .unwrap()
                        .encoding = "native".into();
                }
                "policy_removed" => {
                    // Remove all phase diagnostics too: no surviving phase schema
                    // may hide a missing capture-to-policy admission check.
                    fs::remove_file(dir.join(checked::POLICY_PATH)).unwrap();
                    fs::remove_dir_all(dir.join("telemetry")).unwrap();
                    manifest.artifacts.retain(|a| {
                        a.path != checked::POLICY_PATH && !a.path.starts_with("telemetry/")
                    });
                }
                _ => unreachable!(),
            }
            // Every mutation remains valid under plain artifact hash checking.
            storage::verify_manifest(&dir, &manifest.artifacts).unwrap();
            fs::write(
                dir.join("manifest.json"),
                serde_json::to_vec_pretty(&manifest).unwrap(),
            )
            .unwrap();
            assert!(
                super::super::verify_cell(&dir, false).is_err(),
                "{mutation}"
            );
        }
    }

    #[test]
    fn checked_capture_context_rejected_before_counter_open() {
        use super::super::telemetry::checked;
        for mutation in [
            "binding",
            "limits",
            "unknown_limits",
            "missing_limits",
            "missing_policy",
        ] {
            let f = Fixture::new();
            let dir = f.checked_cell(0);
            match mutation {
                "binding" => {
                    let mut value: Value =
                        super::super::read_json(&dir.join("capture-binding.json")).unwrap();
                    value["source_commit"] = json!("c".repeat(40));
                    fs::write(
                        dir.join("capture-binding.json"),
                        serde_json::to_vec(&value).unwrap(),
                    )
                    .unwrap();
                }
                "limits" | "unknown_limits" => {
                    let mut value: Value =
                        super::super::read_json(&dir.join("capture-limits.json")).unwrap();
                    if mutation == "limits" {
                        value["max_payload_count"] = json!(4095);
                        let limits: super::super::capture::CaptureLimits =
                            serde_json::from_value(value.clone()).unwrap();
                        limits.validate().unwrap();
                    } else {
                        value["unregistered_limit"] = json!(1);
                    }
                    fs::write(
                        dir.join("capture-limits.json"),
                        serde_json::to_vec(&value).unwrap(),
                    )
                    .unwrap();
                }
                "missing_limits" => fs::remove_file(dir.join("capture-limits.json")).unwrap(),
                "missing_policy" => fs::remove_file(dir.join(checked::POLICY_PATH)).unwrap(),
                _ => unreachable!(),
            }
            let (result, opens) = checked::tests::with_mock_counters(|| {
                super::super::worker(&f.plan.cell(0).unwrap(), &dir)
            });
            assert!(result.is_err(), "{mutation}");
            assert_eq!(opens, 0, "{mutation}");
            assert!(!dir.join("telemetry").exists(), "{mutation}");
            assert!(!dir.join("worker-artifacts.json").exists(), "{mutation}");
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn jsonl(path: &Path) -> Vec<Value> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
            .collect()
    }

    #[test]
    fn partial_report_preserves_claims_pmu_and_attempts_without_fabricating_rows() {
        let f = Fixture::new();
        f.sealed(0);
        f.partial(1, false);
        f.partial(2, true);
        let out = f.root.join("report");
        let report = report_run(&f.source, &out).unwrap();
        let summary = &report["summary"];
        assert_eq!(summary["planned_method_cells"], 8);
        assert_eq!(summary["observed_distinct_cell_identities"], 3);
        assert_eq!(summary["assigned_cells_without_observed_local_attempt"], 5);
        assert_eq!(summary["counts"]["active_sealed_cells"], 1);
        assert_eq!(summary["counts"]["active_incomplete_cells"], 1);
        assert_eq!(summary["counts"]["retained_interrupted_attempts"], 1);
        assert_eq!(summary["counts"]["recorded_known_answer_verified_cells"], 1);
        assert_eq!(
            summary["counts"]["recorded_independent_scalar_recovery_cells"],
            1
        );
        assert_eq!(summary["counts"]["recorded_pdp_witness_verified_cells"], 0);
        assert_eq!(summary["partial"], true);
        assert_eq!(summary["certificate_replay_performed"], false);
        assert!(summary["counts"]
            .get("by_factor_base_policy_recipe")
            .is_none());
        assert_eq!(jsonl(&out.join("coverage.jsonl")).len(), 3);
        let rows = jsonl(&out.join("measurements.jsonl"));
        let pmu = rows
            .iter()
            .find(|row| row["kind"] == "native_phase_record")
            .unwrap();
        assert_eq!(pmu["raw"]["hardware"]["instructions_time_enabled_ns"], 100);
        assert_eq!(pmu["raw"]["hardware"]["instructions_time_running_ns"], 50);
        assert_eq!(
            pmu["raw"]["retired_instructions_exact_in_scope"],
            Value::Null
        );
        assert_eq!(
            rows.iter()
                .filter(|row| row["kind"] == "unreadable_partial_telemetry")
                .count(),
            2
        );
        assert_eq!(report_run(&f.source, &out).unwrap(), report);
        let inventory = jsonl(&out.join("artifacts.jsonl"));
        assert!(inventory
            .iter()
            .any(|row| row["artifact"]["path"] == "partial.bin" && row["manifest_bound"] == false));
    }

    #[test]
    fn policy_recipe_coverage_separates_same_kind_parameters_on_both_twists() {
        let policies = vec![
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 32,
            },
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 64,
            },
        ];
        let f = Fixture::with_policies(policies.clone());
        f.plan.validate().unwrap();
        // Solver varies fastest, then policy, then twist. Keep one BSGS cell
        // for each recipe/twist, with the same curve, generator and target key.
        for index in [0, 2, 4, 6] {
            assert_eq!(f.plan.cell(index).unwrap().solver, "bsgs");
            f.sealed(index);
        }
        let out = f.root.join("policy-report");
        let report = report_run(&f.source, &out).unwrap();
        let summary = &report["summary"];
        assert_eq!(summary["planned_method_cells"], 16);
        assert_eq!(summary["observed_distinct_cell_identities"], 4);
        assert_eq!(summary["assigned_cells_without_observed_local_attempt"], 12);
        assert_eq!(summary["counts"]["active_sealed_cells"], 4);
        assert_eq!(summary["counts"]["by_twist"]["false"], 2);
        assert_eq!(summary["counts"]["by_twist"]["true"], 2);
        let counts = summary["counts"]["by_factor_base_policy_recipe"]
            .as_object()
            .unwrap();
        assert_eq!(counts.len(), 2);
        for policy in &policies {
            let hash = super::super::digest(&serde_json::to_vec(policy).unwrap());
            assert_eq!(counts[&format!("{}:{hash}", policy.name())], 2);
        }

        let coverage = jsonl(&out.join("coverage.jsonl"));
        assert_eq!(coverage.len(), 4);
        for row in &coverage {
            let config = f.plan.cell(row["cell_index"].as_u64().unwrap()).unwrap();
            let policy = config.factor_base_policy.as_ref().unwrap();
            assert_eq!(row["actual_axes"]["factor_base_policy"], json!(policy));
            assert_eq!(
                row["actual_axes"]["factor_base_policy_sha256"],
                super::super::digest(&serde_json::to_vec(policy).unwrap())
            );
            assert_eq!(row["actual_axes"]["twist"], config.twist);
            assert_eq!(row["certificate_replay_performed"], false);
        }
        let measurements = jsonl(&out.join("measurements.jsonl"));
        let construction: Vec<_> = measurements
            .iter()
            .filter(|row| row["kind"] == "factor_base_construction")
            .collect();
        assert_eq!(construction.len(), 4);
        for row in construction {
            let index = row["cell_index"].as_u64().unwrap();
            let config = f.plan.cell(index).unwrap();
            let dir = f.source.join("cells").join(format!("{index:012}"));
            let base: Value = read_json(&dir, "factor-base.json").unwrap();
            assert_eq!(row["policy"], json!(config.factor_base_policy.unwrap()));
            assert_eq!(row["status"], base["construction_status"]);
            assert_eq!(row["counters"], base["construction_counters"]);
            assert_eq!(
                row["coordinate_domain_complete"],
                base["coordinate_domain_complete"]
            );
            assert_eq!(
                row["actual_points"],
                base["points"].as_array().unwrap().len()
            );
            assert_eq!(row["manifest_bound"], true);
            assert_eq!(row["interpreted_as_retired_instructions"], false);
        }
        assert_eq!(report_run(&f.source, &out).unwrap(), report);
    }

    #[test]
    fn tampering_and_snapshot_changes_never_replace_report_outputs() {
        let f = Fixture::new();
        f.sealed(0);
        let out = f.root.join("report");
        report_run(&f.source, &out).unwrap();
        let original = fs::read(out.join("coverage.jsonl")).unwrap();
        fs::write(f.source.join("cells/000000000000/stdout.log"), b"changed").unwrap();
        assert!(report_run(&f.source, &f.root.join("tampered-report"))
            .unwrap_err()
            .contains("artifact mismatch"));
        fs::write(f.source.join("cells/000000000000/stdout.log"), b"fixture").unwrap();
        f.partial(1, false);
        assert!(report_run(&f.source, &out).is_err());
        assert_eq!(fs::read(out.join("coverage.jsonl")).unwrap(), original);
    }

    #[test]
    fn empty_source_is_partial_and_oversized_json_is_rejected_before_parsing() {
        let f = Fixture::new();
        let out = f.root.join("empty-report");
        let report = report_run(&f.source, &out).unwrap();
        assert_eq!(report["summary"]["observed_distinct_cell_identities"], 0);
        assert_eq!(
            report["summary"]["assigned_cells_without_observed_local_attempt"],
            8
        );
        assert!(fs::read(out.join("coverage.jsonl")).unwrap().is_empty());
        let huge = fresh(&f.root.join("huge.json")).unwrap();
        huge.set_len(INPUT_LIMIT + 1).unwrap();
        assert!(read_json::<Value>(&f.root, "huge.json")
            .unwrap_err()
            .contains("at most"));
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_and_overlapping_destinations_fail_without_creating_source_files() {
        let f = Fixture::new();
        f.sealed(0);
        let nested = f.source.join("cells/new-report");
        assert!(report_run(&f.source, &nested)
            .unwrap_err()
            .contains("outside"));
        assert!(!nested.exists());
        let alias = f.root.join("alias");
        std::os::unix::fs::symlink(&f.source, &alias).unwrap();
        assert!(report_run(&alias, &f.root.join("report")).is_err());
        let out = f.root.join("report");
        fs::create_dir(&out).unwrap();
        std::os::unix::fs::symlink(f.source.join("run.json"), out.join("coverage.jsonl")).unwrap();
        assert!(report_run(&f.source, &out).is_err());
    }

    #[test]
    fn external_rows_compact_multiple_levels_deduplicate_and_reject_conflicts() {
        let f = Fixture::new();
        let mut rows = Rows::new(&f.root, "sort");
        for index in (0..100).rev() {
            rows.push(&index.to_string(), &json!({"index":index}))
                .unwrap();
            rows.flush().unwrap();
            rows.push(&index.to_string(), &json!({"index":index}))
                .unwrap();
            rows.flush().unwrap();
        }
        let path = f.root.join("sorted.jsonl");
        assert_eq!(rows.finish(&path).unwrap(), 100);
        assert_eq!(jsonl(&path).len(), 100);
        let mut rows = Rows::new(&f.root, "conflict");
        rows.push("same", &json!(1)).unwrap();
        rows.push("same", &json!(2)).unwrap();
        assert!(rows
            .finish(&f.root.join("conflict.jsonl"))
            .unwrap_err()
            .contains("conflicting"));
    }
}
