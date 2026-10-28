//! A derived, explicitly projected training view. Raw artifacts remain canonical.
//!
//! Feature records never copy a whole source JSON object. Labels and provenance
//! live in separate restricted files. A bounded Rust external merge deduplicates
//! feature identities without keeping the corpus or a corpus-sized hash set in
//! memory. Export takes the source lease to obtain a stable local snapshot.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::storage;
use super::types::Artifact;

const CHUNK_BYTES: usize = 16 * 1024 * 1024;
const MAX_RECORD_BYTES: usize = 64 * 1024 * 1024;
const MERGE_FAN_IN: usize = 8;
const SPLIT_DOMAIN: &str = "ml-dataset-whole-field-size-split-v1";

fn text(value: &Value, name: &str) -> Result<String, String> {
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("missing string {name}"))
}

fn integer(value: &Value, name: &str) -> Result<u64, String> {
    value
        .as_u64()
        .ok_or_else(|| format!("missing nonnegative integer {name}"))
}

fn boolean(value: &Value, name: &str) -> Result<bool, String> {
    value
        .as_bool()
        .ok_or_else(|| format!("missing boolean {name}"))
}

fn decimal(value: &Value, name: &str) -> Result<String, String> {
    let s = text(value, name)?;
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("{name} is not a decimal integer string"));
    }
    Ok(num_bigint::BigUint::parse_bytes(s.as_bytes(), 10)
        .ok_or_else(|| format!("invalid integer {name}"))?
        .to_string())
}

fn point(value: &Value) -> Result<Value, String> {
    if value.is_null() {
        return Ok(Value::Null);
    }
    Ok(json!({"x":decimal(&value["x"],"point.x")?,"y":decimal(&value["y"],"point.y")?}))
}

fn points(value: &Value, name: &str) -> Result<Value, String> {
    let rows = value
        .as_array()
        .ok_or_else(|| format!("missing array {name}"))?;
    Ok(Value::Array(
        rows.iter().map(point).collect::<Result<_, _>>()?,
    ))
}

fn action(value: &Value, name: &str) -> Result<Value, String> {
    let rows = value
        .as_array()
        .ok_or_else(|| format!("missing action {name}"))?;
    rows.iter()
        .map(|v| {
            if v.is_null() {
                Ok(Value::Null)
            } else {
                integer(v, name).map(Value::from)
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Value::Array)
}

/// This is an allowlist by structure and scalar type, not a recursive blacklist.
/// Future fields on curves, points, certificates, candidates, or orbit records
/// cannot silently become model features.
fn feature_projection(public: &Value, base: &Value) -> Result<Value, String> {
    if public["schema"] != "ml-ecdlp-instance/v1" || base["schema"] != "ml-ecdlp-factor-base/v1" {
        return Err("unsupported source feature schema".into());
    }
    let source_field = &public["field"];
    let mut field = json!({
        "characteristic":decimal(&source_field["characteristic"],"field.characteristic")?,
        "degree":integer(&source_field["degree"],"field.degree")?,
        "basis":text(&source_field["basis"],"field.basis")?,
    });
    if source_field.get("modulus").is_some() {
        field["modulus"] = json!(decimal(&source_field["modulus"], "field.modulus")?);
    }
    if let Some(terms) = source_field.get("irreducible_low_terms") {
        field["irreducible_low_terms"] = Value::Array(
            terms
                .as_array()
                .ok_or("irreducible low terms must be an array")?
                .iter()
                .map(|v| integer(v, "irreducible term").map(Value::from))
                .collect::<Result<_, _>>()?,
        );
    }
    let c = &public["curve"];
    let curve = json!({
        "model":text(&c["model"],"curve.model")?,
        "a":decimal(&c["a"],"curve.a")?,"b":decimal(&c["b"],"curve.b")?,
        "group_order":decimal(&c["group_order"],"curve.group_order")?,
        "subgroup_order":decimal(&c["subgroup_order"],"curve.subgroup_order")?,
        "cofactor":decimal(&c["cofactor"],"curve.cofactor")?,
    });
    let symmetry = &public["symmetry"];
    let orbits = base["orbits"]
        .as_array()
        .ok_or("missing factor-base orbits")?
        .iter()
        .map(|orbit| {
            Ok(json!({"points":points(&orbit["points"],"orbit.points")?,
            "closed":boolean(&orbit["closed"],"orbit.closed")?,
            "stabilizer_size":integer(&orbit["stabilizer_size"],"orbit.stabilizer_size")?}))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(json!({
        "family":text(&public["family"],"family")?,"n":integer(&public["n"],"n")?,
        "m":integer(&public["m"],"m")?,"twist":boolean(&public["twist"],"twist")?,
        "field":field,"curve":curve,"generator":point(&public["generator"])?,
        "target":point(&public["target"])?,
        "symmetry":{"coefficient_field_degree":integer(&symmetry["coefficient_field_degree"],"coefficient_field_degree")?,
            "frobenius_power":integer(&symmetry["frobenius_power"],"frobenius_power")?,
            "frobenius_nontrivial_on_model":boolean(&symmetry["frobenius_nontrivial_on_model"],"frobenius_nontrivial_on_model")?,
            "negation":boolean(&symmetry["negation"],"negation")?},
        "factor_base":{"rule":text(&base["rule"],"factor_base.rule")?,
            "points":points(&base["points"],"factor_base.points")?,
            "negation_action":action(&base["negation_action"],"negation_action")?,
            "frobenius_action":action(&base["frobenius_action"],"frobenius_action")?,
            "frobenius_power":integer(&base["frobenius_power"],"factor_base.frobenius_power")?,"orbits":orbits},
    }))
}

fn split_for_size(n: u64) -> (String, &'static str) {
    let component = format!("field-size-n{n}");
    let hash = super::digest(format!("{SPLIT_DOMAIN}\n{component}").as_bytes());
    let bucket = u64::from_str_radix(&hash[..16], 16).expect("hex digest") % 100;
    (
        component,
        if bucket < 80 {
            "train"
        } else if bucket < 90 {
            "validation"
        } else {
            "test"
        },
    )
}

#[derive(Eq, PartialEq)]
struct Record {
    key: String,
    json: String,
}

struct CappedWriter {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for CappedWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "export row exceeds {}-byte limit; split the source artifact",
                    self.limit
                ),
            ));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn bounded_json(value: &Value, limit: usize) -> Result<String, String> {
    let mut writer = CappedWriter {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut writer, value).map_err(|e| e.to_string())?;
    String::from_utf8(writer.bytes).map_err(|e| e.to_string())
}

impl Record {
    fn new(key: &str, value: &Value) -> Result<Self, String> {
        crate::store::index::validate_sha256(key)?;
        let json = bounded_json(value, MAX_RECORD_BYTES)?;
        Ok(Self {
            key: key.into(),
            json,
        })
    }
    fn write(&self, writer: &mut impl Write) -> Result<(), String> {
        writeln!(writer, "{}\t{}", self.key, self.json).map_err(|e| e.to_string())
    }
}

struct HeapEntry {
    record: Record,
    run: usize,
}
impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.record.key == other.record.key && self.run == other.run
    }
}
impl Eq for HeapEntry {}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .record
            .key
            .cmp(&self.record.key)
            .then_with(|| other.run.cmp(&self.run))
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn new_file(path: &Path) -> Result<File, String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))
}

fn bounded_line(reader: &mut impl BufRead, limit: usize) -> Result<Vec<u8>, String> {
    let mut line = Vec::new();
    loop {
        let available = reader.fill_buf().map_err(|e| e.to_string())?;
        if available.is_empty() {
            return Ok(line);
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let take = newline.map_or(available.len(), |position| position + 1);
        if take > limit.saturating_sub(line.len()) {
            return Err("external-sort record exceeds bounded line limit".into());
        }
        line.extend_from_slice(&available[..take]);
        reader.consume(take);
        if newline.is_some() {
            return Ok(line);
        }
    }
}

fn next_record(reader: &mut BufReader<File>) -> Result<Option<Record>, String> {
    // 64 hex key bytes, tab and newline are the only non-JSON bytes. Reject
    // before extending the buffer, including a malformed unterminated row.
    let line = bounded_line(reader, MAX_RECORD_BYTES + 66)?;
    if line.is_empty() {
        return Ok(None);
    }
    if !line.ends_with(b"\n") {
        return Err("invalid external-sort record".into());
    }
    let mut line = String::from_utf8(line).map_err(|e| e.to_string())?;
    line.pop();
    let (key, json) = line
        .split_once('\t')
        .ok_or("external-sort record has no key")?;
    crate::store::index::validate_sha256(key)?;
    Ok(Some(Record {
        key: key.into(),
        json: json.into(),
    }))
}

/// Merge at most eight runs, retaining at most one row per run. In the final
/// pass, equal IDs must have identical payloads or export fails closed.
fn merge(runs: &[PathBuf], destination: &Path, final_jsonl: bool) -> Result<u64, String> {
    if runs.len() > MERGE_FAN_IN {
        return Err("merge fan-in limit exceeded".into());
    }
    let mut readers = runs
        .iter()
        .map(|path| {
            File::open(path)
                .map(BufReader::new)
                .map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut heap = BinaryHeap::new();
    for (run, reader) in readers.iter_mut().enumerate() {
        if let Some(record) = next_record(reader)? {
            heap.push(HeapEntry { record, run });
        }
    }
    let mut writer = BufWriter::new(new_file(destination)?);
    let mut previous: Option<Record> = None;
    let mut count = 0u64;
    while let Some(entry) = heap.pop() {
        if final_jsonl {
            let duplicate = previous
                .as_ref()
                .is_some_and(|old| old.key == entry.record.key);
            if duplicate {
                if previous
                    .as_ref()
                    .is_some_and(|old| old.json != entry.record.json)
                {
                    return Err(format!(
                        "export identity collision for {}",
                        entry.record.key
                    ));
                }
            } else {
                writeln!(writer, "{}", entry.record.json).map_err(|e| e.to_string())?;
                count += 1;
            }
            previous = Some(entry.record);
        } else {
            entry.record.write(&mut writer)?;
            count += 1;
        }
        if let Some(record) = next_record(&mut readers[entry.run])? {
            heap.push(HeapEntry {
                record,
                run: entry.run,
            });
        }
    }
    writer.flush().map_err(|e| e.to_string())?;
    writer.get_ref().sync_all().map_err(|e| e.to_string())?;
    Ok(count)
}

struct Sorter {
    directory: PathBuf,
    prefix: &'static str,
    pending: Vec<Record>,
    pending_bytes: usize,
    runs: Vec<PathBuf>,
    sequence: usize,
    chunk_bytes: usize,
}
impl Sorter {
    fn new(directory: &Path, prefix: &'static str) -> Self {
        Self {
            directory: directory.into(),
            prefix,
            pending: Vec::new(),
            pending_bytes: 0,
            runs: Vec::new(),
            sequence: 0,
            chunk_bytes: CHUNK_BYTES,
        }
    }
    fn path(&mut self) -> PathBuf {
        let path = self
            .directory
            .join(format!("{}-{:012}.run", self.prefix, self.sequence));
        self.sequence += 1;
        path
    }
    fn push(&mut self, key: &str, value: &Value) -> Result<(), String> {
        let record = Record::new(key, value)?;
        self.pending_bytes += record.key.len() + record.json.len() + 2;
        self.pending.push(record);
        if self.pending_bytes >= self.chunk_bytes {
            self.flush()?;
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), String> {
        if self.pending.is_empty() {
            return Ok(());
        }
        self.pending
            .sort_by(|a, b| a.key.cmp(&b.key).then_with(|| a.json.cmp(&b.json)));
        let path = self.path();
        let mut writer = BufWriter::new(new_file(&path)?);
        for record in &self.pending {
            record.write(&mut writer)?;
        }
        writer.flush().map_err(|e| e.to_string())?;
        writer.get_ref().sync_all().map_err(|e| e.to_string())?;
        self.runs.push(path);
        self.pending.clear();
        self.pending_bytes = 0;
        Ok(())
    }
    fn finish(mut self, destination: &Path) -> Result<u64, String> {
        self.flush()?;
        while self.runs.len() > MERGE_FAN_IN {
            let old = std::mem::take(&mut self.runs);
            for group in old.chunks(MERGE_FAN_IN) {
                let next = self.path();
                merge(group, &next, false)?;
                self.runs.push(next);
                for path in group {
                    fs::remove_file(path).map_err(|e| e.to_string())?;
                }
            }
        }
        merge(&self.runs, destination, true)
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn install(out: &Path, staged: &Path, relative: &str, role: &str) -> Result<Artifact, String> {
    let (sha256, bytes) = crate::store::hash_file(staged)?;
    let artifact = Artifact {
        role: role.into(),
        path: relative.into(),
        sha256,
        bytes,
        encoding: "jsonl".into(),
    };
    let destination = out.join(relative);
    match fs::hard_link(staged, &destination) {
        Ok(()) => super::sync_dir(destination.parent().expect("export file parent"))?,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            storage::verify_manifest(out, std::slice::from_ref(&artifact))?
        }
        Err(e) => return Err(format!("install {}: {e}", destination.display())),
    }
    Ok(artifact)
}

/// Export local canonical records into a projection safe from known-answer and
/// solver-outcome leakage. This does not download remote sources or train a model.
pub fn export(root: &Path, out: &Path) -> Result<Value, String> {
    let root = super::checked_directory(root, false)?;
    let _source_lease = super::Lease::acquire(&root)?;
    let (plan, binding, plan_sha) = super::bound_run(&root)?;
    let out = super::checked_directory(out, true)?;
    if root.starts_with(&out) || out.starts_with(root.join("cells")) {
        return Err("export destination overlaps source cell storage".into());
    }
    let _out_lease = super::Lease::acquire(&out)?;
    let restricted = super::checked_directory(&out.join("restricted"), true)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&restricted, fs::Permissions::from_mode(0o700))
            .map_err(|e| e.to_string())?;
    }
    let scratch = Scratch(out.join(format!(
        ".export-staging-{}-{}",
        std::process::id(),
        super::unix_nanos()?
    )));
    fs::create_dir(&scratch.0).map_err(|e| e.to_string())?;
    let mut features = Sorter::new(&scratch.0, "feature");
    let mut targets = Sorter::new(&scratch.0, "target");
    let mut index = Sorter::new(&scratch.0, "index");
    let mut feature_occurrences = 0u64;
    let mut sealed = 0u64;
    let mut unsealed = 0u64;
    let mut unavailable = 0u64;
    let mut split_counts = std::collections::BTreeMap::<String, u64>::new();
    let shard_count = integer(&binding["shard_count"], "shard_count")?;
    let shard_index = integer(&binding["shard_index"], "shard_index")?;
    if shard_count == 0 || shard_index >= shard_count {
        return Err("invalid bound source shard".into());
    }
    let source_run_sha = crate::store::hash_file(&root.join("run.json"))?.0;
    let cells = root.join("cells");
    if cells.exists() {
        let cells = super::checked_directory(&cells, false)?;
        for entry in fs::read_dir(cells).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                return Err("unexpected source cell entry".into());
            }
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| "non-UTF8 source cell")?;
            let cell_index: u64 = name.parse().map_err(|_| "nonnumeric source cell")?;
            if name != format!("{cell_index:012}") || cell_index % shard_count != shard_index {
                return Err("source cell directory does not belong to the bound shard".into());
            }
            let expected = plan.cell(cell_index)?;
            let dir = entry.path();
            let relative = format!("cells/{name}");
            if !dir.join("manifest.json").exists() {
                unsealed += 1;
                let id =
                    super::digest(format!("{source_run_sha}\n{relative}\nunsealed").as_bytes());
                index.push(&id,&json!({"schema":"ml-dataset.export-index/v1","record_id":id,"cell":relative,
                    "status":"unsealed_attempt_excluded","feature_id":null,"source_run_sha256":source_run_sha}))?;
                continue;
            }
            let manifest = super::verify_cell(&dir, true)?;
            if manifest.cell_index != cell_index
                || manifest.plan_sha256 != plan_sha
                || manifest.config != expected
                || super::read_json::<Value>(&dir.join("run.json"))? != binding
            {
                return Err("source cell is inconsistent with export run binding".into());
            }
            let manifest_sha = crate::store::hash_file(&dir.join("manifest.json"))?.0;
            let target_id =
                super::digest(format!("{source_run_sha}\n{manifest_sha}\n{cell_index}").as_bytes());
            let has = |path: &str| manifest.artifacts.iter().any(|a| a.path == path);
            let mut feature_id = None;
            let mut component = None;
            let mut split = None;
            if has("public.json") && has("factor-base.json") && has("witness.json") {
                let public: Value = super::read_json(&dir.join("public.json"))?;
                let base: Value = super::read_json(&dir.join("factor-base.json"))?;
                let inputs = feature_projection(&public, &base)?;
                let id = super::digest(bounded_json(&inputs, MAX_RECORD_BYTES)?.as_bytes());
                let (group, partition) = split_for_size(integer(&inputs["n"], "n")?);
                let row = json!({"schema":"ml-dataset.features/v1","feature_id":id,"split_component":group,"split":partition,"inputs":inputs});
                features.push(&id, &row)?;
                feature_occurrences += 1;
                *split_counts.entry(partition.into()).or_default() += 1;
                feature_id = Some(id);
                component = Some(group);
                split = Some(partition);
            } else {
                unavailable += 1;
            }
            let witness_ref = manifest.artifacts.iter().find(|a| a.path == "witness.json");
            let known_scalar = if let Some(artifact) = witness_ref {
                let witness: Value = super::read_json(&dir.join(&artifact.path))?;
                if feature_id.is_some() {
                    Some(decimal(&witness["scalar"], "known answer scalar")?)
                } else {
                    None
                }
            } else {
                None
            };
            let certificate_refs: Vec<_> = manifest
                .artifacts
                .iter()
                .filter(|a| {
                    [
                        "independently-verified-pdp",
                        "restricted-independent-scalar-label",
                    ]
                    .contains(&a.role.as_str())
                })
                .cloned()
                .collect();
            let mut independent_scalar = None;
            for certificate in &certificate_refs {
                if certificate.role == "restricted-independent-scalar-label" {
                    let label: Value = super::read_json(&dir.join(&certificate.path))?;
                    independent_scalar = Some(decimal(&label["scalar"], "independent scalar")?);
                }
            }
            targets.push(&target_id,&json!({"schema":"ml-dataset.restricted-targets/v1","target_id":target_id,
                "feature_id":feature_id,"split_component":component,"split":split,"solver":manifest.config.solver,
                "labels":manifest.outcome,"known_answer_scalar":known_scalar,"independently_recovered_scalar":independent_scalar,
                "witness_artifact":witness_ref,"solution_certificates":certificate_refs,
                "feature_status":if feature_id.is_some(){"available"}else{"missing_verified_public_inputs"}}))?;
            index.push(&target_id,&json!({"schema":"ml-dataset.export-index/v1","record_id":target_id,"target_id":target_id,
                "feature_id":feature_id,"cell":relative,"source_manifest_sha256":manifest_sha,
                "source_run_sha256":source_run_sha,"source_artifacts":manifest.artifacts,"status":"verified_sealed_cell"}))?;
            sealed += 1;
        }
    }
    let features_staged = scratch.0.join("features.jsonl");
    let targets_staged = scratch.0.join("targets.jsonl");
    let index_staged = scratch.0.join("index.jsonl");
    let unique_features = features.finish(&features_staged)?;
    let target_rows = targets.finish(&targets_staged)?;
    let index_rows = index.finish(&index_staged)?;
    if target_rows != sealed || index_rows != sealed + unsealed {
        return Err("export lost or duplicated source identities".into());
    }
    let artifacts = vec![
        install(
            &out,
            &features_staged,
            "features.jsonl",
            "allowlisted-model-inputs",
        )?,
        install(
            &out,
            &targets_staged,
            "restricted/targets.jsonl",
            "restricted-supervised-labels",
        )?,
        install(
            &out,
            &index_staged,
            "restricted/index.jsonl",
            "restricted-source-provenance",
        )?,
    ];
    let manifest = json!({"schema":"ml-dataset.training-export/v1","source_run_sha256":source_run_sha,"plan_sha256":plan_sha,
        "source_commit":binding["source_commit"],"binary_sha256":binding["binary_sha256"],"artifacts":artifacts,
        "sealed_source_cells":sealed,"unsealed_attempts_excluded":unsealed,"cells_without_verified_features":unavailable,
        "feature_occurrences":feature_occurrences,"unique_feature_rows":unique_features,"restricted_target_rows":target_rows,
        "feature_identity":"SHA256 of canonical strictly projected public inputs and factor base; solver and source provenance excluded",
        "split_policy":{"domain":SPLIT_DOMAIN,"component":"field-size-nN","assignment":"first 64 hash bits modulo 100: train 0..79, validation 80..89, test 90..99",
            "occurrences_by_partition":split_counts,"all_same_size_families_twists_generators_keys_stay_together":true,
            "cross_size_algebraic_links_merged":false,"row_random_split_permitted":false},
        "feature_policy":{"version":"strict-structural-allowlist-v1","only_features_jsonl_is_model_input":true,
            "excluded":["private witness","sampling seeds","curve/generator/key indices","configuration","order certificates","construction candidates","construction trace","solver outcomes","solver event streams","labels","source paths"]},
        "sort":{"implementation":"bounded Rust external merge","chunk_bytes":CHUNK_BYTES,"merge_fan_in":MERGE_FAN_IN,"max_row_bytes":MAX_RECORD_BYTES,
            "source_cell_values_materialized":true,"total_process_rss_bound_claimed":false},
        "source_artifacts_modified":false,"remote_download_performed":false});
    let manifest_artifact =
        storage::write_json(&out, "manifest.json", "training-export-manifest", &manifest)?;
    storage::verify_manifest(&out, &artifacts)?;
    Ok(json!({"manifest":manifest_artifact,"export":manifest}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{curves, CellManifest, DatasetPlan};

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "crypto-export-test-{}-{}",
                std::process::id(),
                crate::ml_dataset::unix_nanos().unwrap()
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn source(&self) -> PathBuf {
            let source = self.0.join("source");
            fs::create_dir(&source).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1","purpose":"native export correctness fixture"});
            let freeze_hash = crate::ml_dataset::digest(
                &crate::ml_dataset::protocol_canonical_bytes(&protocol).unwrap(),
            );
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":freeze_hash,
                "filed_before_affected_data":true,"frozen_at":"2026-10-28T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let plan = DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![11],
                arities: vec![2],
                families: vec!["prime".into()],
                solvers: vec!["enumerate".into(), "bsgs".into()],
                curve_seeds: 8,
                generators: 2,
                keys: 2,
                seed: 42,
                factor_base_size: 8,
                degree_bound: 4,
                work_limit: 1000,
                twists: true,
                protocol_sha256: Some(crate::ml_dataset::digest(&protocol)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let plan_sha = crate::ml_dataset::digest(&serde_json::to_vec(&plan).unwrap());
            let binding = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":plan_sha,
                "source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64),"shard_index":0,"shard_count":1});
            storage::write_json(&source, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&source, "run.json", "run", &binding).unwrap();
            storage::write_bytes(&source, "protocol.json", "protocol", "json", &protocol).unwrap();
            let case = curves::generate(&plan.cell(124).unwrap()).unwrap();
            for cell_index in [124, 125] {
                let dir = source.join("cells").join(format!("{cell_index:012}"));
                fs::create_dir_all(&dir).unwrap();
                let config = plan.cell(cell_index).unwrap();
                let mut artifacts = vec![
                    storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
                    storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
                    storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol)
                        .unwrap(),
                    storage::write_json(&dir, "config.json", "restricted-config", &config).unwrap(),
                    storage::write_json(
                        &dir,
                        "public.json",
                        "public-instance",
                        &case.public_instance,
                    )
                    .unwrap(),
                    storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                        .unwrap(),
                    storage::write_json(
                        &dir,
                        "witness.json",
                        "restricted-known-answer",
                        &case.private_witness,
                    )
                    .unwrap(),
                ];
                for path in ["attempt.json", "worker-process.json"] {
                    artifacts
                        .push(storage::write_json(&dir, path, "provenance", &json!({})).unwrap());
                }
                for path in ["stdout.log", "stderr.log"] {
                    artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
                }
                storage::write_json(&dir,"manifest.json","manifest",&CellManifest {schema:"crypto.ml-dataset.cell/v1".into(),
                    cell_index,plan_sha256:plan_sha.clone(),config,
                    outcome:json!({"status":"not_attempted","known_answer_verified":true,"factor_base_verified":true,
                        "solver_completed":false,"scalar_recovered_without_witness":false,"scalar_replay_verified":false}),artifacts}).unwrap();
            }
            source
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn projection_ignores_unknown_secret_and_future_state_at_every_nested_level() {
        let marker = "FORBIDDEN_FUTURE_OR_PRIVATE_VALUE";
        let public = json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":11,"m":2,"twist":false,
            "field":{"characteristic":"2017","modulus":"2017","degree":1,"basis":"canonical-residue","secret":marker},
            "curve":{"model":"short-weierstrass","a":"1","b":"1","group_order":"2011","subgroup_order":"2011","cofactor":"1",
                "order_certificate":{"private":marker},"rejected_curve_samples":marker},
            "generator":{"x":"1","y":"2","scalar":marker},"target":{"x":"3","y":"4","key_index":marker},
            "symmetry":{"coefficient_field_degree":1,"frobenius_power":1,"frobenius_nontrivial_on_model":false,"negation":true,"future":marker},
            "seed":marker,"private_witness":marker,"known_answer_verified":marker,"construction_trace":marker,"sampling":marker});
        let base = json!({"schema":"ml-ecdlp-factor-base/v1","rule":"small-x-cofactor-projection",
            "points":[{"x":"1","y":"2","scalar":marker}],"negation_action":[null],"frobenius_action":[0],"frobenius_power":1,
            "orbits":[{"points":[{"x":"1","y":"2","seed":marker}],"closed":true,"stabilizer_size":1,"future":marker}],
            "construction_candidates":[{"private":marker}],"solver_outcome":marker,"generator_index":marker});
        let projected = feature_projection(&public, &base).unwrap();
        let encoded = serde_json::to_string(&projected).unwrap();
        assert!(!encoded.contains(marker));
        assert!(!encoded.contains("scalar"));
        assert!(!encoded.contains("seed"));
        assert!(!encoded.contains("construction"));
        assert_eq!(projected["target"], json!({"x":"3","y":"4"}));
        let mut poisoned = public;
        poisoned["field"]["basis"] = json!({"private":marker});
        assert!(feature_projection(&poisoned, &base).is_err());
    }

    #[test]
    fn malformed_scratch_lines_and_serialization_stop_at_their_limits() {
        let mut reader = BufReader::with_capacity(16, io::Cursor::new(vec![b'x'; 4096]));
        let error = bounded_line(&mut reader, 32).unwrap_err();
        assert!(error.contains("bounded line limit"));
        // At most one fixed reader buffer can be prefetched beyond the cap;
        // the rest of the malformed, unterminated row is never read.
        assert!(reader.get_ref().position() <= 48);

        let mut reader = BufReader::with_capacity(3, io::Cursor::new(b"abc\nnext\n"));
        assert_eq!(bounded_line(&mut reader, 4).unwrap(), b"abc\n");
        assert_eq!(bounded_line(&mut reader, 5).unwrap(), b"next\n");
        assert!(bounded_line(&mut reader, 5).unwrap().is_empty());

        let value = json!({"escaped":"\n\n\n\n\n\n\n\n"});
        assert!(bounded_json(&value, 16)
            .unwrap_err()
            .contains("exceeds 16-byte limit"));
        let expected = serde_json::to_string(&value).unwrap();
        assert_eq!(bounded_json(&value, expected.len()).unwrap(), expected);
        let mut writer = CappedWriter {
            bytes: Vec::new(),
            limit: 8,
        };
        writer.write_all(b"1234").unwrap();
        assert!(writer.write_all(b"12345").is_err());
        assert_eq!(writer.bytes, b"1234");
    }

    #[test]
    fn external_merge_is_deterministic_deduplicated_and_checks_collisions() {
        let f = Fixture::new();
        let mut sorter = Sorter::new(&f.0, "rows");
        sorter.chunk_bytes = 80;
        for i in (0..40).rev() {
            let key = crate::ml_dataset::digest(format!("row-{i}").as_bytes());
            for _ in 0..2 {
                sorter
                    .push(&key, &json!({"feature_id":key,"value":i}))
                    .unwrap();
            }
        }
        let output = f.0.join("rows.jsonl");
        assert_eq!(sorter.finish(&output).unwrap(), 40);
        let rows: Vec<Value> = BufReader::new(File::open(output).unwrap())
            .lines()
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
            .collect();
        assert_eq!(rows.len(), 40);
        assert!(rows
            .windows(2)
            .all(|pair| pair[0]["feature_id"].as_str() < pair[1]["feature_id"].as_str()));
        let key = "a".repeat(64);
        let mut collision = Sorter::new(&f.0, "collision");
        collision.push(&key, &json!({"value":1})).unwrap();
        collision.push(&key, &json!({"value":2})).unwrap();
        assert!(collision
            .finish(&f.0.join("collision.jsonl"))
            .unwrap_err()
            .contains("identity collision"));
    }

    #[test]
    fn source_export_separates_labels_deduplicates_backends_and_retries_immutably() {
        let f = Fixture::new();
        let source = f.source();
        let out = f.0.join("training");
        let first = export(&source, &out).unwrap();
        assert_eq!(first["export"]["sealed_source_cells"], 2);
        assert_eq!(first["export"]["feature_occurrences"], 2);
        assert_eq!(first["export"]["unique_feature_rows"], 1);
        assert_eq!(first["export"]["restricted_target_rows"], 2);
        let features = fs::read_to_string(out.join("features.jsonl")).unwrap();
        for forbidden in [
            "private_witness",
            "scalar",
            "key_index",
            "generator_index",
            "curve_seed",
            "source_commit",
            "solver_completed",
        ] {
            assert!(!features.contains(forbidden), "{forbidden}");
        }
        let targets = fs::read_to_string(out.join("restricted/targets.jsonl")).unwrap();
        assert!(targets.contains("known_answer_scalar"));
        let second = export(&source, &out).unwrap();
        assert_eq!(first["manifest"], second["manifest"]);
        fs::write(source.join("cells/000000000124/witness.json"), b"corrupt").unwrap();
        assert!(export(&source, &f.0.join("bad-export")).is_err());
        assert!(!f.0.join("bad-export/manifest.json").exists());
    }

    #[test]
    fn splits_group_all_instances_of_the_same_size() {
        let first = split_for_size(23);
        assert_eq!(first, split_for_size(23));
        assert_eq!(first.0, "field-size-n23");
        assert_ne!(first.0, split_for_size(24).0);
    }
}
