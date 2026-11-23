//! Bounded diagnostic prefixes, never accepted mathematical output or cost.
//! Producer activation is deliberately disabled until instrument admission.
use super::canonical::{reject_duplicate_keys, sha256_hex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
pub const MAX_FILES: usize = 256;
pub const MAX_BYTES: usize = 65536;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub session: String,
    pub execution: u64,
    pub workload_sha256: String,
    pub binary_sha256: String,
    pub protocol_sha256: String,
    pub seed: u64,
    pub attempt: u64,
    pub source_sha256: String,
    pub method_config_sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub schema: String,
    pub relative_leaf: String,
    pub identity: Identity,
}
impl Context {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "ecbench.diagnostic_context/v1"
            || self.relative_leaf.is_empty()
            || self.relative_leaf.len() > 80
            || !self
                .relative_leaf
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || self.identity.session.is_empty()
            || self.identity.session.len() > 128
        {
            return Err("invalid diagnostic context".into());
        }
        for h in [
            &self.identity.workload_sha256,
            &self.identity.binary_sha256,
            &self.identity.protocol_sha256,
            &self.identity.source_sha256,
            &self.identity.method_config_sha256,
        ] {
            if h.len() != 64 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err("invalid diagnostic digest".into());
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    schema: String,
    identity: Identity,
    sequence: u64,
    diagnostic_only: bool,
    complete: bool,
    accepted_result: Option<u64>,
    candidate_over_rho: Option<f64>,
    boundary: String,
    epoch: String,
    walk: Option<u64>,
    completed_candidates: u64,
    counters: Counters,
    prior_io: IoTotals,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Counters {
    field: BTreeMap<String, u128>,
    arithmetic: BTreeMap<String, u128>,
    symbolic: BTreeMap<String, u128>,
    #[serde(default)]
    phases: BTreeMap<String, PhaseTotals>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct IoTotals {
    attempted: u64,
    committed: u64,
    published: u64,
    published_bytes: u64,
    bytes: u64,
    completed_write_ns: u128,
    failure: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Allocation {
    alloc_calls: u64,
    dealloc_calls: u64,
    realloc_calls: u64,
    failures: u64,
    requested_bytes: u64,
    released_bytes: u64,
    peak_live_requested_bytes: u64,
    peak_growth_over_entry_bytes: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PhaseTotals {
    elapsed_ns: u128,
    field: BTreeMap<String, u128>,
    arithmetic: BTreeMap<String, u128>,
    symbolic: BTreeMap<String, u128>,
    allocation: Option<Allocation>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub sha256: String,
    pub bytes: usize,
    pub sequence: Option<u64>,
    pub classification: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: String,
    pub identity: Identity,
    pub context_sha256: String,
    pub record_sha256: String,
    pub entries: Vec<Entry>,
    pub diagnostic_only: bool,
    pub counter_semantics_verified: bool,
    pub durability_confirmed: Option<bool>,
    pub ingestion_wall_ns: u128,
    pub scope: String,
}
fn regular(path: &Path) -> Result<(), String> {
    let m = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !m.file_type().is_file() {
        return Err("diagnostic entry is not a regular file".into());
    }
    Ok(())
}
fn bounded_read(path: &Path) -> Result<Vec<u8>, String> {
    regular(path)?;
    let mut opts = fs::OpenOptions::new();
    opts.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.custom_flags(libc::O_NOFOLLOW);
    }
    let file = opts.open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("nonregular opened diagnostic".into());
    }
    let mut bytes = Vec::new();
    file.take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_BYTES {
        return Err("diagnostic byte cap".into());
    }
    Ok(bytes)
}
fn validate_counter_keys(c: &Counters) -> Result<(), String> {
    const FIELD: &[&str] = &[
        "m",
        "s",
        "i",
        "field_adds",
        "bit_ops",
        "ec_adds",
        "jac_adds",
        "sqrt_calls",
    ];
    const ARITH: &[&str] = &[
        "base_adds",
        "base_muls",
        "base_inversions",
        "scalar_adds",
        "scalar_muls",
        "scalar_inversions",
    ];
    const SYMBOLIC: &[&str] = &[
        "walk_label_calls",
        "scalar_adds",
        "scalar_muls",
        "scalar_negations",
        "scalar_inversions",
        "modular_reductions",
        "sparse_accumulations",
        "payload_terms",
        "parent_hops",
        "partition_terms",
        "primality_checks",
        "collection_entry_visits",
        "collection_elements_copied",
        "collection_elements_initialized",
        "collection_comparisons",
        "map_calls",
        "map_key_comparisons",
        "map_entry_visits",
        "map_cloned_entries",
    ];
    for (m, keys) in [
        (&c.field, FIELD),
        (&c.arithmetic, ARITH),
        (&c.symbolic, SYMBOLIC),
    ] {
        if m.len() != keys.len() || keys.iter().any(|k| !m.contains_key(*k)) {
            return Err("diagnostic counter schema mismatch".into());
        }
    }
    if c.field
        .values()
        .chain(c.arithmetic.values())
        .any(|v| *v > u64::MAX as u128)
    {
        return Err("counter exceeds source u64 range".into());
    }
    if c.phases.keys().any(|k| {
        !matches!(
            k.as_str(),
            "solver_setup"
                | "target_validation"
                | "walk_generation"
                | "sieve"
                | "candidate_verification"
                | "elimination"
                | "recovery_check"
                | "iterator_bookkeeping"
        )
    }) {
        return Err("unknown phase".into());
    }
    for phase in c.phases.values() {
        validate_counter_keys(&Counters {
            field: phase.field.clone(),
            arithmetic: phase.arithmetic.clone(),
            symbolic: phase.symbolic.clone(),
            phases: BTreeMap::new(),
        })?;
    }
    Ok(())
}
fn map_monotonic(a: &BTreeMap<String, u128>, b: &BTreeMap<String, u128>) -> bool {
    a.iter().all(|(k, v)| b.get(k).is_some_and(|n| n >= v))
}
fn monotonic(a: &Counters, b: &Counters) -> bool {
    map_monotonic(&a.field, &b.field)
        && map_monotonic(&a.arithmetic, &b.arithmetic)
        && map_monotonic(&a.symbolic, &b.symbolic)
        && a.phases.iter().all(|(k, x)| {
            b.phases.get(k).is_some_and(|y| {
                x.elapsed_ns <= y.elapsed_ns
                    && map_monotonic(&x.field, &y.field)
                    && map_monotonic(&x.arithmetic, &y.arithmetic)
                    && map_monotonic(&x.symbolic, &y.symbolic)
                    && match (&x.allocation, &y.allocation) {
                        (None, None) => true,
                        (Some(a), Some(b)) => {
                            a.alloc_calls <= b.alloc_calls
                                && a.dealloc_calls <= b.dealloc_calls
                                && a.realloc_calls <= b.realloc_calls
                                && a.failures <= b.failures
                                && a.requested_bytes <= b.requested_bytes
                                && a.released_bytes <= b.released_bytes
                                && a.peak_live_requested_bytes <= b.peak_live_requested_bytes
                                && a.peak_growth_over_entry_bytes <= b.peak_growth_over_entry_bytes
                        }
                        _ => false,
                    }
            })
        })
}
fn history(previous: Option<&Snapshot>, s: &Snapshot, bytes: u64) -> Result<(), String> {
    let io = &s.prior_io;
    if io.attempted != s.sequence
        || io.committed != s.sequence
        || io.published != s.sequence
        || io.bytes != bytes
        || io.published_bytes != bytes
        || io.failure.is_some()
    {
        return Err("diagnostic IO history mismatch".into());
    }
    let stage = s.boundary.as_str();
    let walked = matches!(
        stage,
        "walk_generation_complete" | "sieve_complete" | "candidate_processing_complete"
    );
    if walked != s.walk.is_some()
        || (stage != "candidate_processing_complete" && s.completed_candidates != 0)
        || (stage == "candidate_processing_complete"
            && !matches!(
                s.completed_candidates,
                1 | 4 | 16 | 64 | 256 | 1024 | 4096 | 16384
            ))
    {
        return Err("diagnostic progress shape mismatch".into());
    }
    let Some(old) = previous else {
        return if stage == "rational_table_complete" && io.completed_write_ns == 0 {
            Ok(())
        } else {
            Err("invalid first diagnostic boundary".into())
        };
    };
    if io.completed_write_ns < old.prior_io.completed_write_ns {
        return Err("diagnostic IO clock regressed".into());
    }
    let old_stage = old.boundary.as_str();
    let next_walk =
        stage == "walk_generation_complete" && s.walk == old.walk.and_then(|w| w.checked_add(1));
    let valid = match old_stage {
        "rational_table_complete" => stage == "cold_setup_complete",
        "cold_setup_complete" => stage == "candidate_setup_complete",
        "candidate_setup_complete" => stage == "walk_generation_complete" && s.walk == Some(0),
        "walk_generation_complete" => {
            (stage == "sieve_complete" && s.walk == old.walk)
                || next_walk
                || (stage == "solver_returning" && old.walk == Some(15))
        }
        "sieve_complete" => {
            (stage == "candidate_processing_complete"
                && s.walk == old.walk
                && s.completed_candidates == 1)
                || next_walk
                || stage == "solver_returning"
        }
        "candidate_processing_complete" => {
            (stage == "candidate_processing_complete"
                && s.walk == old.walk
                && Some(s.completed_candidates)
                    == match old.completed_candidates {
                        1 => Some(4),
                        4 => Some(16),
                        16 => Some(64),
                        64 => Some(256),
                        256 => Some(1024),
                        1024 => Some(4096),
                        4096 => Some(16384),
                        _ => None,
                    })
                || next_walk
                || stage == "solver_returning"
        }
        _ => false,
    };
    if !valid {
        return Err("diagnostic boundary transition mismatch".into());
    }
    Ok(())
}
/// Trusted runner-owned session root; no live child may mutate the leaf during ingestion.
/// Caller seals this manifest separately after record sealing. No math result is imported.
pub fn ingest(root: &Path, context: &Context, sealed_record: &[u8]) -> Result<Manifest, String> {
    let start = std::time::Instant::now();
    context.validate()?;
    let record: Value = serde_json::from_slice(sealed_record).map_err(|e| e.to_string())?;
    reject_duplicate_keys(std::str::from_utf8(sealed_record).map_err(|e| e.to_string())?)?;
    let expected_seed = context.identity.seed.to_string();
    if record["session_id"] != context.identity.session
        || record["seq"].as_u64() != Some(context.identity.execution)
        || record["algorithm_seed"].as_str() != Some(expected_seed.as_str())
        || record["binary_sha256"] != context.identity.binary_sha256
    {
        return Err("diagnostic record association mismatch".into());
    }
    let workload = serde_json::to_vec(&record["workload"]).map_err(|e| e.to_string())?;
    let method = serde_json::to_vec(&record["method"]).map_err(|e| e.to_string())?;
    if sha256_hex(&workload) != context.identity.workload_sha256
        || sha256_hex(&method) != context.identity.method_config_sha256
    {
        return Err("diagnostic workload/method association mismatch".into());
    }
    let leaf = root.join(&context.relative_leaf);
    if !fs::symlink_metadata(&leaf)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_dir()
    {
        return Err("diagnostic leaf is not a directory".into());
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&leaf).map_err(|e| e.to_string())? {
        if paths.len() > MAX_FILES {
            return Err("diagnostic directory entry cap".into());
        }
        paths.push(entry.map_err(|e| e.to_string())?.path());
    }
    paths.sort();
    let mut entries = Vec::new();
    let mut previous: Option<Snapshot> = None;
    let mut expected = 0u64;
    let mut temporary = 0;
    let mut temporary_sequence = None;
    let mut committed_bytes = 0u64;
    for path in paths {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid diagnostic filename")?
            .to_string();
        let suffix = if name.ends_with(".json") {
            ".json"
        } else if name.ends_with(".tmp") {
            ".tmp"
        } else {
            return Err("unexpected diagnostic entry".into());
        };
        let prefix = name.strip_suffix(suffix).unwrap();
        if prefix.len() != 6 || !prefix.bytes().all(|b| b.is_ascii_digit()) {
            return Err("invalid diagnostic filename".into());
        }
        let n: u64 = prefix.parse().map_err(|_| "invalid sequence")?;
        if n >= MAX_FILES as u64 {
            return Err("diagnostic sequence cap".into());
        }
        let bytes = bounded_read(&path)?;
        if suffix == ".tmp" {
            temporary += 1;
            temporary_sequence = Some(n);
            if temporary > 1 {
                return Err("multiple temporary diagnostics".into());
            }
            entries.push(Entry {
                name,
                sha256: sha256_hex(&bytes),
                bytes: bytes.len(),
                sequence: None,
                classification: "incomplete_temporary".into(),
            });
            continue;
        }
        let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
        reject_duplicate_keys(text)?;
        let s: Snapshot = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if s.schema != "pencil-progress/v1"
            || s.identity != context.identity
            || s.sequence != expected
            || n != expected
            || !s.diagnostic_only
            || s.complete
            || s.accepted_result.is_some()
            || s.candidate_over_rho.is_some()
            || s.walk.is_some_and(|w| w >= 16)
        {
            return Err("invalid diagnostic prefix".into());
        }
        let setup = matches!(
            s.boundary.as_str(),
            "rational_table_complete" | "cold_setup_complete"
        );
        let candidate = matches!(
            s.boundary.as_str(),
            "candidate_setup_complete"
                | "walk_generation_complete"
                | "sieve_complete"
                | "candidate_processing_complete"
                | "solver_returning"
        );
        if !(setup && s.epoch == "cold_setup" || candidate && s.epoch == "candidate") {
            return Err("invalid diagnostic boundary/epoch".into());
        }
        validate_counter_keys(&s.counters)?;
        history(previous.as_ref(), &s, committed_bytes)?;
        if let Some(old) = &previous {
            if old.epoch == "candidate" && s.epoch == "cold_setup"
                || old.epoch == s.epoch && !monotonic(&old.counters, &s.counters)
            {
                return Err("diagnostic counter/epoch regression".into());
            }
        }
        expected += 1;
        committed_bytes += bytes.len() as u64;
        entries.push(Entry {
            name,
            sha256: sha256_hex(&bytes),
            bytes: bytes.len(),
            sequence: Some(s.sequence),
            classification: "structurally_valid_prefix".into(),
        });
        previous = Some(s);
    }
    if temporary_sequence.is_some_and(|n| n != expected && Some(n) != expected.checked_sub(1)) {
        return Err("temporary does not belong to final write".into());
    }
    if let Some(n) = temporary_sequence {
        if let Some(committed) = entries.iter().find(|entry| entry.sequence == Some(n)) {
            let tmp = entries
                .iter()
                .find(|entry| entry.classification == "incomplete_temporary")
                .ok_or("missing temporary entry")?;
            if committed.sha256 != tmp.sha256 || committed.bytes != tmp.bytes {
                return Err("published temporary differs from committed bytes".into());
            }
        }
    }
    Ok(Manifest{schema:"ecbench.diagnostic_manifest/v1".into(),identity:context.identity.clone(),context_sha256:sha256_hex(&serde_json::to_vec(context).map_err(|e|e.to_string())?),record_sha256:sha256_hex(sealed_record),entries,diagnostic_only:true,counter_semantics_verified:false,durability_confirmed:None,ingestion_wall_ns:start.elapsed().as_nanos(),scope:"ingestion only; excludes setup, solver, record and manifest publication; no accepted cost".into()})
}
/// Shared by actual runner assembly. A late value is never accepted after a stop.
pub fn accepted_after_stop<T>(value: Option<&T>, timed_out: bool, interrupted: bool) -> Option<&T> {
    value.filter(|_| !timed_out && !interrupted)
}

#[cfg(test)]
mod diagnostic_ingestion_tests {
    use super::*;
    fn fixture(tag: &str) -> (PathBuf, Context, Vec<u8>, Value) {
        let dir =
            std::env::temp_dir().join(format!("ecbench-diagnostic-{}-{tag}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        fs::create_dir(dir.join("leaf")).unwrap();
        let record = serde_json::json!({"session_id":"fixture","seq":0,"algorithm_seed":"7","binary_sha256":"a".repeat(64),"workload":{"x":1},"method":{"x":2}});
        let c = Context {
            schema: "ecbench.diagnostic_context/v1".into(),
            relative_leaf: "leaf".into(),
            identity: Identity {
                session: "fixture".into(),
                execution: 0,
                seed: 7,
                attempt: 0,
                binary_sha256: "a".repeat(64),
                workload_sha256: sha256_hex(&serde_json::to_vec(&record["workload"]).unwrap()),
                method_config_sha256: sha256_hex(&serde_json::to_vec(&record["method"]).unwrap()),
                protocol_sha256: "b".repeat(64),
                source_sha256: "c".repeat(64),
            },
        };
        let names=[("field","m s i field_adds bit_ops ec_adds jac_adds sqrt_calls"),("arithmetic","base_adds base_muls base_inversions scalar_adds scalar_muls scalar_inversions"),("symbolic","walk_label_calls scalar_adds scalar_muls scalar_negations scalar_inversions modular_reductions sparse_accumulations payload_terms parent_hops partition_terms primality_checks collection_entry_visits collection_elements_copied collection_elements_initialized collection_comparisons map_calls map_key_comparisons map_entry_visits map_cloned_entries")];
        let mut counters = serde_json::Map::new();
        for (name, keys) in names {
            counters.insert(
                name.into(),
                serde_json::to_value(
                    keys.split_whitespace()
                        .map(|k| (k, 0u64))
                        .collect::<BTreeMap<_, _>>(),
                )
                .unwrap(),
            );
        }
        let row = serde_json::json!({"schema":"pencil-progress/v1","identity":c.identity,"sequence":0,"diagnostic_only":true,"complete":false,"accepted_result":null,"candidate_over_rho":null,"boundary":"rational_table_complete","epoch":"cold_setup","walk":null,"completed_candidates":0,"counters":counters,"prior_io":{"attempted":0,"committed":0,"published":0,"published_bytes":0,"bytes":0,"completed_write_ns":0,"failure":null}});
        (dir, c, serde_json::to_vec(&record).unwrap(), row)
    }
    fn write(dir: &Path, n: u64, row: &Value) {
        let bytes: u64 = (0..n)
            .map(|i| {
                fs::metadata(dir.join("leaf").join(format!("{i:06}.json")))
                    .unwrap()
                    .len()
            })
            .sum();
        let mut row = row.clone();
        row["prior_io"] = serde_json::json!({"attempted":n,"committed":n,"published":n,"published_bytes":bytes,"bytes":bytes,"completed_write_ns":n,"failure":null});
        fs::write(
            dir.join("leaf").join(format!("{n:06}.json")),
            serde_json::to_vec(&row).unwrap(),
        )
        .unwrap();
    }
    fn candidate_start(d: &Path, row: &mut Value) {
        write(d, 0, row);
        row["sequence"] = 1.into();
        row["boundary"] = "cold_setup_complete".into();
        write(d, 1, row);
        row["sequence"] = 2.into();
        row["epoch"] = "candidate".into();
        row["boundary"] = "candidate_setup_complete".into();
    }
    #[test]
    fn prefix_and_record_binding() {
        let (d, c, r, row) = fixture("bind");
        write(&d, 0, &row);
        let m = ingest(&d, &c, &r).unwrap();
        assert_eq!(m.entries.len(), 1);
        assert_eq!(m.record_sha256, sha256_hex(&r));
        let mut numeric_seed: Value = serde_json::from_slice(&r).unwrap();
        numeric_seed["algorithm_seed"] = c.identity.seed.into();
        assert!(ingest(&d, &c, &serde_json::to_vec(&numeric_seed).unwrap()).is_err());
        assert!(!m.counter_semantics_verified);
        assert_eq!(m.durability_confirmed, None);
        let mut wrong = c.clone();
        wrong.identity.seed = 8;
        assert!(ingest(&d, &wrong, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn invalid_data_never_accepted() {
        let (d, c, r, mut row) = fixture("invalid");
        row["accepted_result"] = 1.into();
        write(&d, 0, &row);
        assert!(ingest(&d, &c, &r).is_err());
        row["accepted_result"] = Value::Null;
        row["epoch"] = "unknown".into();
        write(&d, 0, &row);
        assert!(ingest(&d, &c, &r).is_err());
        fs::write(
            d.join("leaf/000000.json"),
            b"{\"sequence\":0,\"sequence\":0}",
        )
        .unwrap();
        assert!(ingest(&d, &c, &r).is_err());
        fs::write(d.join("leaf/000000.json"), vec![b'x'; MAX_BYTES + 1]).unwrap();
        assert!(ingest(&d, &c, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn epoch_reset_and_regression() {
        let (d, c, r, mut row) = fixture("epoch");
        row["counters"]["field"]["m"] = 9.into();
        candidate_start(&d, &mut row);
        row["counters"]["field"]["m"] = 2.into();
        write(&d, 2, &row);
        assert!(ingest(&d, &c, &r).is_ok());
        row["sequence"] = 3.into();
        row["boundary"] = "walk_generation_complete".into();
        row["walk"] = 0.into();
        row["counters"]["field"]["m"] = 1.into();
        write(&d, 3, &row);
        assert!(ingest(&d, &c, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn incomplete_temporary_is_separate() {
        let (d, c, r, row) = fixture("tmp");
        write(&d, 0, &row);
        fs::write(d.join("leaf/000001.tmp"), b"partial").unwrap();
        let m = ingest(&d, &c, &r).unwrap();
        assert_eq!(m.entries[1].classification, "incomplete_temporary");
        assert!(m.entries[1].sequence.is_none());
        fs::remove_dir_all(d).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn symlink_refused() {
        let (d, c, r, _) = fixture("symlink");
        std::os::unix::fs::symlink("/dev/null", d.join("leaf/000000.json")).unwrap();
        assert!(ingest(&d, &c, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn stop_filter_is_authoritative() {
        let late = serde_json::json!({"recovered":7,"total_gae":1});
        assert!(accepted_after_stop(Some(&late), true, false).is_none());
        assert!(accepted_after_stop(Some(&late), false, true).is_none());
        assert!(accepted_after_stop(Some(&late), false, false).is_some());
    }
    #[test]
    fn actual_phase_shape_and_unknown_counter() {
        let (d, c, r, mut row) = fixture("phase");
        candidate_start(&d, &mut row);
        row["counters"]["phases"] = serde_json::json!({"solver_setup":{"elapsed_ns":17,"field":row["counters"]["field"],"arithmetic":row["counters"]["arithmetic"],"symbolic":row["counters"]["symbolic"],"allocation":null}});
        write(&d, 2, &row);
        assert!(ingest(&d, &c, &r).is_ok());
        row["counters"]["phases"]["solver_setup"]["field"]["unknown"] = 0.into();
        write(&d, 2, &row);
        assert!(ingest(&d, &c, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn impossible_histories_are_rejected() {
        let (d, c, r, mut row) = fixture("history");
        candidate_start(&d, &mut row);
        write(&d, 2, &row);
        row["sequence"] = 3.into();
        row["boundary"] = "walk_generation_complete".into();
        write(&d, 3, &row);
        assert!(ingest(&d, &c, &r).is_err());
        row["walk"] = 0.into();
        write(&d, 3, &row);
        assert!(ingest(&d, &c, &r).is_ok());
        row["sequence"] = 4.into();
        row["boundary"] = "sieve_complete".into();
        write(&d, 4, &row);
        row["sequence"] = 5.into();
        row["boundary"] = "candidate_processing_complete".into();
        row["completed_candidates"] = 2.into();
        write(&d, 5, &row);
        assert!(ingest(&d, &c, &r).is_err());
        row["completed_candidates"] = 1.into();
        write(&d, 5, &row);
        assert!(ingest(&d, &c, &r).is_ok());
        row["sequence"] = 6.into();
        row["completed_candidates"] = 16.into();
        write(&d, 6, &row);
        assert!(ingest(&d, &c, &r).is_err());
        row["boundary"] = "solver_returning".into();
        row["walk"] = Value::Null;
        row["completed_candidates"] = 0.into();
        write(&d, 6, &row);
        assert!(ingest(&d, &c, &r).is_ok());
        row["sequence"] = 7.into();
        write(&d, 7, &row);
        assert!(ingest(&d, &c, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn phase_io_and_temporary_history_rejected() {
        let (d, c, r, mut row) = fixture("phase-history");
        candidate_start(&d, &mut row);
        row["counters"]["phases"] = serde_json::json!({"solver_setup":{"elapsed_ns":17,"field":row["counters"]["field"],"arithmetic":row["counters"]["arithmetic"],"symbolic":row["counters"]["symbolic"],"allocation":null}});
        write(&d, 2, &row);
        row["sequence"] = 3.into();
        row["boundary"] = "walk_generation_complete".into();
        row["walk"] = 0.into();
        row["counters"]["phases"]["solver_setup"]["elapsed_ns"] = 16.into();
        write(&d, 3, &row);
        assert!(ingest(&d, &c, &r).is_err());
        row["counters"]["phases"]["solver_setup"]["elapsed_ns"] = 18.into();
        write(&d, 3, &row);
        assert!(ingest(&d, &c, &r).is_ok());
        let file = d.join("leaf/000003.json");
        let mut changed: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
        changed["prior_io"]["bytes"] = 0.into();
        fs::write(&file, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(ingest(&d, &c, &r).is_err());
        write(&d, 3, &row);
        fs::write(d.join("leaf/000099.tmp"), b"partial").unwrap();
        assert!(ingest(&d, &c, &r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn directory_entry_limit() {
        let (d, c, r, _) = fixture("entry-cap");
        for n in 0..MAX_FILES + 2 {
            fs::write(d.join("leaf").join(format!("{n:06}.json")), b"{}").unwrap();
        }
        assert!(ingest(&d, &c, &r).unwrap_err().contains("entry cap"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn published_temporary_requires_identical_bytes() {
        let (d, c, r, row) = fixture("published-tmp");
        write(&d, 0, &row);
        let src = d.join("leaf/000000.json");
        let tmp = d.join("leaf/000000.tmp");
        fs::write(&tmp, b"different").unwrap();
        assert!(ingest(&d, &c, &r).is_err());
        fs::copy(src, tmp).unwrap();
        assert!(ingest(&d, &c, &r).is_ok());
        fs::remove_dir_all(d).unwrap();
    }
}
