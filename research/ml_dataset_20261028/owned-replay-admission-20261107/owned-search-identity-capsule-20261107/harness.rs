#![allow(dead_code,unused_imports)]
mod ml_dataset {
use sha2::{Digest,Sha256};
fn digest(b:&[u8])->String{hex::encode(Sha256::digest(b))}
pub mod capture {
pub fn validate_sha256(s: &str) -> Result<(), String> {
    if s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        Ok(())
    } else {
        Err(format!("{s:?} is not a lowercase hex SHA-256"))
    }
}
const BINDING_SCHEMA:&str="crypto.ml-dataset.attempt-binding/v1";
#[derive(Clone,Debug,serde::Serialize,serde::Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct AttemptBinding {
    pub schema: String,
    pub run_sha256: String,
    pub plan_sha256: String,
    pub protocol_sha256: String,
    pub config_sha256: String,
    pub source_commit: String,
    pub binary_sha256: String,
    pub cell_index: u64,
    pub attempt_sha256: String,
}
impl AttemptBinding{pub fn validate(&self) -> Result<(), String> {
        if self.schema != BINDING_SCHEMA {
            return Err("unsupported capture binding schema".into());
        }
        for hash in [
            &self.run_sha256,
            &self.plan_sha256,
            &self.protocol_sha256,
            &self.config_sha256,
            &self.binary_sha256,
            &self.attempt_sha256,
        ] {
            validate_sha256(hash)?;
        }
        if self.source_commit.len() != 40
            || !self
                .source_commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("capture requires a full lowercase source commit".into());
        }
        Ok(())
    }
pub fn digest(&self) -> Result<String, String> {
        self.validate()?;
        Ok(super::digest(
            &serde_json::to_vec(self).map_err(|e| e.to_string())?,
        ))
    }}
}
pub mod execution {
use super::capture::AttemptBinding;use serde_json::Value;
const SCHEMA:&str="crypto.ml-dataset.execution-context/v1";
#[derive(Clone,Debug,serde::Serialize,serde::Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct LaunchContext {
    pub schema: String,
    pub binding: AttemptBinding,
    pub boot_id: String,
    pub start_monotonic_ns: u64,
    pub deadline_monotonic_ns: u64,
    pub seconds_per_case: u64,
    pub memory_bytes: u64,
    pub trace_bytes: u64,
}
fn positive(run: &Value, key: &str) -> Result<u64, String> {
    run[key]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or_else(|| format!("missing/zero original {key}"))
}
fn bytes(run: &Value, key: &str) -> Result<u64, String> {
    positive(run, key)?
        .checked_mul(1024 * 1024)
        .ok_or_else(|| format!("original {key} overflow"))
}
impl LaunchContext{pub fn validate(&self, run: &Value, binding: &AttemptBinding) -> Result<(), String> {
        let interval = self
            .deadline_monotonic_ns
            .checked_sub(self.start_monotonic_ns)
            .ok_or("reversed worker deadline")?;
        let duration = self
            .seconds_per_case
            .checked_mul(1_000_000_000)
            .ok_or("worker duration overflow")?;
        if self.schema != SCHEMA
            || &self.binding != binding
            || self.boot_id.len() != 36
            || !self.boot_id.bytes().enumerate().all(|(i, b)| {
                if [8, 13, 18, 23].contains(&i) {
                    b == b'-'
                } else {
                    b.is_ascii_hexdigit()
                }
            })
            || self.seconds_per_case != positive(run, "seconds_per_case")?
            || self.memory_bytes != bytes(run, "memory_mib")?
            || self.trace_bytes != bytes(run, "trace_mib")?
            || interval != duration
        {
            return Err("execution context differs from original run/attempt".into());
        }
        Ok(())
    }}
}
pub mod solvers {
use std::{path::Path,time::Instant};use serde_json::Value;use super::capture::AttemptBinding;
type CaseConfig=Value; // unused token metadata only; no CaseConfig validation claim
pub(super) struct VerifiedWorkerOriginal<'a> {
    config: &'a CaseConfig,
    public: &'a Value,
    factor_base: &'a Value,
    root: &'a Path,
    binding: Option<&'a AttemptBinding>,
    public_sha256: [u8; 32],
    factor_base_sha256: [u8; 32],
    public_bytes: u64,
    factor_base_bytes: u64,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) enum Stop {
    InvalidLimits,
    Overflow,
    Deadline,
    Phase,
    Nonmonotone,
    Work,
    Bytes,
    Trace,
    Additions,
    Observer,
    NativeFailure,
    Allocation,
    Serialization,
    Events,
}
trait DecodeBudget {
    fn reserve_checking(&mut self, work: u64, bytes: u64, now: Instant) -> Result<(), Stop>;
    fn check(&mut self, now: Instant) -> Result<(), Stop>;
    fn fail<T>(&mut self, stop: Stop) -> Result<T, Stop>;
}
mod central{use super::*;
#[path="/home/ec2-user/ml-dataset-resume-20261029/owned-search-identity-capsule-20261107/owned_search_identity_capsule.rs"] mod capsule;
include!("/home/ec2-user/ml-dataset-resume-20261029/owned-search-identity-capsule-20261107/tests.rs");
}
}
}
