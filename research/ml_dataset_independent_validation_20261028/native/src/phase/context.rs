//! Typed admitted-context adapter, independent of either transport envelope.
use super::{
    binding, budget::Budget, io, schema::*, CheckedPhaseAudit, POLICY_BYTES, POLICY_HASH,
    POLICY_PATH,
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ContextArtifact {
    pub path: String,
    pub role: String,
    pub encoding: String,
    pub sha256: String,
    pub bytes: u64,
}
pub(crate) trait ContextReader {
    /// Reader binds the same consumed raw buffer to this exact descriptor,
    /// reserves raw/scratch in the carried phase ledger before allocating,
    /// and accounts physical reads in the parent operation.
    fn read_context(
        &mut self,
        artifact: &ContextArtifact,
        cap: u64,
        budget: &mut Budget,
    ) -> Result<Vec<u8>>;
}
pub(crate) fn reconstruct_admitted(
    reader: &mut dyn ContextReader,
    members: &BTreeMap<String, ContextArtifact>,
    index: u64,
    budget: &mut Budget,
) -> Result<Binding> {
    budget.charge(131072, "admitted context bookkeeping")?;
    let mut inputs = BTreeMap::new();
    for name in [
        "run.json",
        "plan.json",
        "protocol.json",
        "config.json",
        "attempt.json",
    ] {
        let identity = members
            .get(name)
            .context("partial binding missing authoritative member")?;
        ensure!(
            identity.path == name && identity.encoding == "json",
            "partial authoritative context identity"
        );
        let raw = reader.read_context(identity, crate::common::MAX_JSON, budget)?;
        ensure!(
            raw.len() as u64 == identity.bytes && crate::common::digest(&raw) == identity.sha256,
            "partial authoritative consumed byte identity"
        );
        inputs.insert(name, raw);
    }
    binding::from_bytes(&inputs, index, None, budget)
}
pub(crate) fn audit_admitted(
    reader: &mut dyn ContextReader,
    members: &BTreeMap<String, ContextArtifact>,
    bound: &Binding,
    budget: &mut Budget,
) -> Result<CheckedPhaseAudit> {
    budget.charge(131072, "admitted phase projection bookkeeping")?;
    let policy = members
        .get(POLICY_PATH)
        .context("partial phase policy absent")?;
    ensure!(
        policy.role == "restricted-measurement-policy" && policy.encoding == "json",
        "partial policy descriptor"
    );
    let raw = reader.read_context(policy, 16384, budget)?;
    ensure!(
        raw == POLICY_BYTES
            && crate::common::digest(&raw) == POLICY_HASH
            && policy.sha256 == POLICY_HASH
            && raw.len() as u64 == policy.bytes,
        "partial phase fixed policy bytes"
    );
    let _: FixedPolicy = io::typed(&raw, budget)?;
    let mut identities = Vec::with_capacity(9);
    identities.push(serde_json::to_value(policy)?);
    let mut records = Vec::with_capacity(4);
    for phase in Phase::ALL {
        let s = members.get(&phase.path(true));
        let e = members.get(&phase.path(false));
        ensure!(
            s.is_some() || e.is_none(),
            "partial phase end without start"
        );
        if let Some(a) = s {
            ensure!(
                a.role == "phase-start" && a.encoding == "json",
                "partial phase start descriptor"
            );
            let raw = reader.read_context(a, 65536, budget)?;
            ensure!(
                raw.len() as u64 == a.bytes && crate::common::digest(&raw) == a.sha256,
                "partial phase start consumed identity"
            );
            let start = io::typed::<Start>(&raw, budget)?;
            let start_value = crate::common::read_json_bytes(&raw)?;
            identities.push(serde_json::to_value(a)?);
            let end = if let Some(a) = e {
                ensure!(
                    a.role == "hardware-and-resource-telemetry" && a.encoding == "json",
                    "partial phase end descriptor"
                );
                let raw = reader.read_context(a, 524288, budget)?;
                ensure!(
                    raw.len() as u64 == a.bytes && crate::common::digest(&raw) == a.sha256,
                    "partial phase end consumed identity"
                );
                let metrics = io::typed::<Metrics>(&raw, budget)?;
                let value = crate::common::read_json_bytes(&raw)?;
                identities.push(serde_json::to_value(a)?);
                Some((metrics, value, a.sha256.clone()))
            } else {
                None
            };
            records.push((phase, start, start_value, a.sha256.clone(), end));
        }
    }
    // This is the same pure lifecycle/projection validator as the sealed path.
    Ok(super::validate_context(bound.clone(), records, identities, budget)?.partial())
}
