//! Candidate I2 source. It consumes already validated/pinned retained files;
//! it does not launch a payload, authenticate a controller, or accept a row.
use super::canonical::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path},
};

pub const SCHEMA: &str = "ecbench.pencil-attempt-join/v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilePin {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

impl FilePin {
    pub fn checked(path: String, bytes: &[u8]) -> Result<Self, String> {
        if path.is_empty() || path.len() > 4096 {
            return Err("attempt pin path bound".into());
        }
        Ok(Self {
            path,
            sha256: sha256_hex(bytes),
            bytes: bytes.len().try_into().map_err(|_| "attempt pin byte overflow")?,
        })
    }

    /// Bounded streaming pin for relation files that can be much larger than
    /// the native JSON metadata files. Symlinks and non-regular inputs fail.
    pub fn checked_file(path: &Path, max_bytes: u64) -> Result<Self, String> {
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let mut file = options.open(path).map_err(|error| error.to_string())?;
        let metadata = file.metadata().map_err(|error| error.to_string())?;
        if !metadata.is_file() || metadata.len() > max_bytes {
            return Err("attempt pin regular-file/byte bound".into());
        }
        let mut hash = Sha256::new();
        let mut total = 0u64;
        let mut buffer = [0u8; 1024 * 1024];
        loop {
            let n = file.read(&mut buffer).map_err(|error| error.to_string())?;
            if n == 0 { break; }
            total = total.checked_add(n as u64).ok_or("attempt pin byte overflow")?;
            if total > max_bytes { return Err("attempt pin byte bound".into()); }
            hash.update(&buffer[..n]);
        }
        if total != metadata.len() {
            return Err("attempt pin file changed while hashing".into());
        }
        Ok(Self {
            path: path.to_str().ok_or("non-UTF8 attempt pin path")?.to_owned(),
            sha256: format!("{:x}", hash.finalize()),
            bytes: total,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArmKind { Pencil, Rho, RhoAa }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalStatus {
    Returned, SolverError, TraceError, Timeout, Signal, SpawnError, ControllerError,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptJoin {
    pub schema: String,
    pub session_id: String,
    pub execution: u64,
    pub attempt: u64,
    pub arm: ArmKind,
    pub measured: bool,
    pub pair_id: Option<String>,
    pub plan_sha256: String,
    pub workload_sha256: String,
    pub method_config_sha256: String,
    pub algorithm_seed: u64,
    pub source_manifest_sha256: String,
    pub binary_sha256: String,
    pub protocol_sha256: String,
    pub context: FilePin,
    pub ordinary_record: Option<FilePin>,
    pub progress_files: Vec<FilePin>,
    pub progress_totals: Option<FilePin>,
    pub relation_stream: Option<FilePin>,
    pub relation_prefix: Option<FilePin>,
    pub stream_error: Option<String>,
    pub resource_prefix: Option<FilePin>,
    pub resource_tail: Option<FilePin>,
    pub process: FilePin,
    pub controller: FilePin,
    pub cost_record: FilePin,
    pub terminal_status: TerminalStatus,
    pub eligible_for_pair: bool,
    pub ineligibility_reasons: Vec<String>,
    /// Exact parsed raw ledger. No missing value is converted to zero.
    pub raw_costs: Value,
    pub owner_authentication_verified: bool,
}

fn is_hex(s: &str, n: usize) -> bool {
    s.len() == n && s.bytes().all(|b| b.is_ascii_hexdigit())
}

impl AttemptJoin {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != SCHEMA || self.session_id.is_empty() {
            return Err("attempt identity".into());
        }
        for digest in [
            &self.workload_sha256, &self.method_config_sha256,
            &self.source_manifest_sha256, &self.binary_sha256, &self.protocol_sha256,
            &self.plan_sha256,
        ] {
            if !is_hex(digest, 64) { return Err("attempt digest".into()); }
        }
        if self.owner_authentication_verified {
            return Err("attempt join cannot authenticate its controller input".into());
        }
        if self.measured != self.pair_id.is_some()
            || self.pair_id.as_ref().is_some_and(|p| p.is_empty() || p.len() > 256)
        {
            return Err("attempt pair identity".into());
        }
        let complete = self.relation_stream.is_some();
        let partial = self.relation_prefix.is_some();
        match self.arm {
            ArmKind::Pencil if complete && partial => {
                return Err("pencil cannot carry complete and partial streams".into())
            }
            ArmKind::Pencil if !complete && !partial
                && !matches!(self.terminal_status,
                    TerminalStatus::SolverError
                    | TerminalStatus::SpawnError
                    | TerminalStatus::ControllerError) =>
            {
                return Err("pencil missing stream without pre-stream failure".into())
            }
            ArmKind::Rho | ArmKind::RhoAa if complete || partial || self.stream_error.is_some() => {
                return Err("rho arm carries pencil stream".into())
            }
            _ => {}
        }
        if complete && (self.stream_error.is_some() || self.terminal_status == TerminalStatus::TraceError) {
            return Err("complete stream contradicts trace failure".into());
        }
        if partial && self.stream_error.is_none()
            && !matches!(self.terminal_status, TerminalStatus::Timeout | TerminalStatus::Signal)
        {
            return Err("partial stream lacks typed interruption/failure".into());
        }
        if self.eligible_for_pair {
            if !self.ineligibility_reasons.is_empty()
                || self.ordinary_record.is_none()
                || self.resource_prefix.is_none()
                || self.resource_tail.is_none()
                || self.terminal_status != TerminalStatus::Returned
            {
                return Err("eligible attempt has incomplete evidence".into());
            }
            if self.arm == ArmKind::Pencil && !complete {
                return Err("eligible pencil attempt lacks terminal stream".into());
            }
        } else if self.ineligibility_reasons.is_empty() {
            return Err("ineligible attempt lacks reason".into());
        }
        let mut paths = BTreeSet::new();
        let pins = std::iter::once(&self.context)
            .chain(self.ordinary_record.iter())
            .chain(self.progress_files.iter())
            .chain(self.progress_totals.iter())
            .chain(self.relation_stream.iter())
            .chain(self.relation_prefix.iter())
            .chain(self.resource_prefix.iter())
            .chain(self.resource_tail.iter())
            .chain(std::iter::once(&self.process))
            .chain(std::iter::once(&self.controller))
            .chain(std::iter::once(&self.cost_record));
        for pin in pins {
            if !is_hex(&pin.sha256, 64) || !paths.insert(&pin.path) {
                return Err("attempt file pin".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PrimaryKey { session_id: String, execution: u64 }

/// Validates identity uniqueness and enrollment cardinality. It intentionally
/// does not compute scientific ratios; the existing full-cohort aggregator does.
pub fn validate_full_cohort(
    rows: &[AttemptJoin],
    expected_plan_hashes: &BTreeSet<String>,
) -> Result<(), String> {
    if expected_plan_hashes.len() != 7
        || expected_plan_hashes.iter().any(|h| !is_hex(h, 64))
    {
        return Err("expected seven pinned plans".into());
    }
    let mut primary = BTreeMap::new();
    let mut all = BTreeSet::new();
    let mut plans_seen = BTreeSet::new();
    let mut measured_pairs = BTreeMap::<String, BTreeMap<ArmKind, u64>>::new();
    let mut measured = 0usize;
    let mut primary_by_arm = BTreeMap::<ArmKind, usize>::new();
    let mut measured_by_arm = BTreeMap::<ArmKind, usize>::new();
    let mut sessions_by_plan = BTreeMap::<String, BTreeSet<String>>::new();
    let mut primary_by_plan = BTreeMap::<String, usize>::new();
    let mut measured_by_plan = BTreeMap::<String, usize>::new();
    for row in rows {
        row.validate()?;
        if !expected_plan_hashes.contains(&row.plan_sha256) {
            return Err("unbound cohort plan".into());
        }
        if !all.insert((row.session_id.clone(), row.execution, row.attempt)) {
            return Err("duplicate attempt identity".into());
        }
        if row.attempt == 0 {
            plans_seen.insert(row.plan_sha256.clone());
            sessions_by_plan
                .entry(row.plan_sha256.clone())
                .or_default()
                .insert(row.session_id.clone());
            *primary_by_plan.entry(row.plan_sha256.clone()).or_default() += 1;
            if row.measured {
                *measured_by_plan.entry(row.plan_sha256.clone()).or_default() += 1;
            }
            let key = PrimaryKey { session_id: row.session_id.clone(), execution: row.execution };
            if primary.insert(key, row.arm).is_some() { return Err("duplicate primary".into()); }
            measured += usize::from(row.measured);
            *primary_by_arm.entry(row.arm).or_default() += 1;
            if row.measured { *measured_by_arm.entry(row.arm).or_default() += 1; }
            if let Some(pair_id) = &row.pair_id {
                if measured_pairs.entry(pair_id.clone()).or_default()
                    .insert(row.arm, row.algorithm_seed).is_some()
                {
                    return Err("duplicate measured pair arm".into());
                }
            }
        }
    }
    if primary.len() != 22_680 || measured != 18_900 {
        return Err("full cohort cardinality".into());
    }
    if plans_seen != *expected_plan_hashes || measured_pairs.len() != 6_300 {
        return Err("full cohort plan or pair cardinality".into());
    }
    for plan in expected_plan_hashes {
        if sessions_by_plan.get(plan).map(BTreeSet::len) != Some(1)
            || primary_by_plan.get(plan) != Some(&3_240)
            || measured_by_plan.get(plan) != Some(&2_700)
        {
            return Err("per-plan session or execution cardinality".into());
        }
    }
    for arms in measured_pairs.values() {
        if arms.len() != 3
            || ![ArmKind::Pencil, ArmKind::Rho, ArmKind::RhoAa]
                .into_iter().all(|arm| arms.contains_key(&arm))
            || arms.values().copied().collect::<BTreeSet<_>>().len() != 1
        {
            return Err("measured pair arm or seed mismatch".into());
        }
    }
    for arm in [ArmKind::Pencil, ArmKind::Rho, ArmKind::RhoAa] {
        if primary_by_arm.get(&arm) != Some(&7_560)
            || measured_by_arm.get(&arm) != Some(&6_300)
        {
            return Err("full cohort arm cardinality".into());
        }
    }
    Ok(())
}

/// Postrun export only. Caller supplies independently checked plan hashes and
/// attempt FilePins; this function neither launches work nor authenticates them.
pub fn export_full_cohort_new(
    rows: &[AttemptJoin],
    expected_plan_hashes: &BTreeSet<String>,
    path: &Path,
) -> Result<FilePin, String> {
    validate_full_cohort(rows, expected_plan_hashes)?;
    if !path.is_absolute() || path.parent().is_none() {
        return Err("attempt export requires absolute file path".into());
    }
    if path
        .components()
        .any(|component| !matches!(component, Component::RootDir | Component::Normal(_)))
    {
        return Err("attempt export path traversal".into());
    }
    let parent = path.parent().expect("checked parent");
    if fs::canonicalize(parent).map_err(|error| error.to_string())? != parent {
        return Err("attempt export parent must be canonical".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::symlink_metadata(parent).map_err(|error| error.to_string())?;
        if !metadata.is_dir()
            || metadata.file_type().is_symlink()
            // SAFETY: geteuid has no arguments or memory preconditions.
            || metadata.uid() != unsafe { libc::geteuid() }
            || metadata.mode() & 0o077 != 0
        {
            return Err("attempt export parent must be owner-only and owned by caller".into());
        }
    }
    let bytes = serde_json::to_vec(&serde_json::json!({
        "schema": "ecbench.pencil-attempt-cohort/v1",
        "primary_execution_slots": 22_680,
        "measured_execution_slots": 18_900,
        "measured_triplets": 6_300,
        "expected_plan_hashes": expected_plan_hashes,
        "attempts": rows,
        "owner_authentication_verified": false
    }))
    .map_err(|error| error.to_string())?;
    if bytes.len() > 256 * 1024 * 1024 {
        return Err("attempt export byte bound".into());
    }
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o400).custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options.open(path).map_err(|error| error.to_string())?;
    file.write_all(&bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| error.to_string())?;
    FilePin::checked(
        path.to_str().ok_or("non-UTF8 attempt export path")?.to_owned(),
        &bytes,
    )
}
