use super::{budget::Budget, io, protocol, schema::Binding};
use anyhow::{ensure, Context, Result};
use serde_json::Value;
use std::{collections::BTreeMap, path::Path};

pub(super) fn reconstruct(
    root: &Path,
    members: &BTreeMap<String, Value>,
    index: u64,
    plan_sha: &str,
    budget: &mut Budget,
) -> Result<Binding> {
    let mut inputs = BTreeMap::new();
    for name in [
        "run.json",
        "plan.json",
        "protocol.json",
        "config.json",
        "attempt.json",
    ] {
        let a = members
            .get(name)
            .context("phase binding missing authoritative member")?;
        super::descriptor(a)?;
        let raw = io::bytes(root, a, crate::common::MAX_JSON, budget)?;
        inputs.insert(name, raw);
    }
    from_bytes(&inputs, index, Some(plan_sha), budget)
}

/// Same semantic constructor for sealed and admitted partial context. The
/// raw inputs are supplied by the appropriate hash-bound descriptor reader.
pub(super) fn from_bytes(
    inputs: &BTreeMap<&str, Vec<u8>>,
    index: u64,
    expected_plan_sha: Option<&str>,
    budget: &mut Budget,
) -> Result<Binding> {
    ensure!(
        [
            "run.json",
            "plan.json",
            "protocol.json",
            "config.json",
            "attempt.json"
        ]
        .iter()
        .all(|name| inputs.contains_key(name)),
        "phase missing raw authoritative context"
    );
    let run = io::value(&inputs["run.json"], budget)?;
    let plan = io::value(&inputs["plan.json"], budget)?;
    let config = io::value(&inputs["config.json"], budget)?;
    let attempt = io::value(&inputs["attempt.json"], budget)?;
    // Existing plan validation builds deduplication encodings/sets; canonical
    // cloning and its serialization also coexist with the retained input tree.
    // Reserve their conservative variable scratch explicitly, before either
    // canonical_plan or the repeated validation inside config_at can allocate.
    reserve_plan_scratch(inputs["plan.json"].len(), budget)?;
    let canonical = crate::audit::canonical_plan(&plan)?;
    let computed_plan = crate::common::digest(&serde_json::to_vec(&canonical)?);
    ensure!(
        expected_plan_sha.is_none_or(|plan_sha| computed_plan == plan_sha)
            && run["plan_sha256"] == computed_plan
            && attempt["plan_sha256"] == computed_plan,
        "phase binding canonical plan disagreement"
    );
    ensure!(
        run["schema"] == "crypto.ml-dataset.run/v1"
            && attempt["schema"] == "crypto.ml-dataset.attempt/v1"
            && attempt["cell_index"].as_u64() == Some(index),
        "phase binding run/attempt identity"
    );
    ensure!(
        crate::audit::config_at(&plan, index)? == config,
        "phase binding config/index mismatch"
    );
    let count = run["shard_count"]
        .as_u64()
        .context("phase binding shard count")?;
    let shard = run["shard_index"]
        .as_u64()
        .context("phase binding shard index")?;
    ensure!(
        count > 0 && shard < count && index % count == shard,
        "phase binding shard mismatch"
    );
    let source = run["source_commit"]
        .as_str()
        .context("phase binding source")?;
    let binary = run["binary_sha256"]
        .as_str()
        .context("phase binding binary")?;
    ensure!(
        crate::common::valid_hex(source, 40) && crate::common::valid_hex(binary, 64),
        "phase binding source/binary identity"
    );
    let hash = |name: &str| crate::common::digest(&inputs[name]);
    ensure!(
        plan["protocol_sha256"] == hash("protocol.json"),
        "phase binding protocol file digest"
    );
    protocol::verify(&inputs["protocol.json"], budget)?;
    Ok(Binding {
        schema: "crypto.ml-dataset.attempt-binding/v1".into(),
        run_sha256: hash("run.json"),
        plan_sha256: hash("plan.json"),
        protocol_sha256: hash("protocol.json"),
        config_sha256: hash("config.json"),
        source_commit: source.into(),
        binary_sha256: binary.into(),
        cell_index: index,
        attempt_sha256: hash("attempt.json"),
    })
}

fn reserve_plan_scratch(raw_bytes: usize, budget: &mut Budget) -> Result<()> {
    let amount = raw_bytes
        .checked_add(1)
        .and_then(|n| n.checked_mul(32))
        .context("consumer_resource_limit: canonical plan scratch")?;
    budget.charge(
        amount,
        "canonical plan serialization, copies and revalidation",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plan_scratch_is_additive_and_reserved_before_copying() {
        let mut budget = Budget::new(3200);
        budget.charge(1, "retained prior input").unwrap();
        assert!(reserve_plan_scratch(99, &mut budget)
            .unwrap_err()
            .to_string()
            .starts_with("consumer_resource_limit"));
        assert_eq!(budget.used(), 1);
        assert!(reserve_plan_scratch(usize::MAX, &mut budget).is_err());
    }
}
