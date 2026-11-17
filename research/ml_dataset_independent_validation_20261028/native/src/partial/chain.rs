//! Complete backward ancestry and forward admission, before artifact transfer.
use super::{
    budget::*,
    capture::{self, CaptureState},
    io::{self, Root},
    schema::*,
    source::{AdmittedRef, ObjectReader},
};
use anyhow::{ensure, Context, Result};
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;

pub(crate) struct Snapshot {
    pub anchor: Anchor,
    pub receipt: Receipt,
    pub manifest: Manifest,
}
pub(crate) struct Member<'a> {
    pub artifact: &'a Artifact,
    pub object: AdmittedRef<'a>,
}
pub(crate) struct Preflight<'a> {
    pub members: Vec<Member<'a>>,
    pub profile: &'static str,
    pub context_count: usize,
    pub artifact_bytes: u64,
}
pub(crate) fn prefix(anchor: &Anchor, budget: &mut Budget) -> Result<String> {
    budget.charge(1024, "attempt output prefix")?;
    Ok(format!(
        "interrupted-attempts/{}/{}",
        anchor.run_sha256, anchor.attempt_sha256
    ))
}
pub(crate) fn metadata_path(anchor: &Anchor, kind: &str, budget: &mut Budget) -> Result<String> {
    budget.charge(1024, "metadata output path")?;
    Ok(format!(
        "interrupted-attempts/{}/{}/snapshots/{:020}/{kind}.json",
        anchor.run_sha256, anchor.attempt_sha256, anchor.ordinal
    ))
}
pub(crate) fn artifact_path(
    anchor: &Anchor,
    artifact: &Artifact,
    budget: &mut Budget,
) -> Result<String> {
    budget.charge(
        artifact
            .path
            .len()
            .checked_mul(8)
            .and_then(|n| n.checked_add(1024))
            .context("partial artifact path allocation")?,
        "artifact output path",
    )?;
    Ok(format!(
        "interrupted-attempts/{}/{}/artifacts/{}",
        anchor.run_sha256, anchor.attempt_sha256, artifact.path
    ))
}
pub(crate) fn validate_anchor(a: &Anchor, budget: &mut Budget) -> Result<()> {
    ensure!(
        a.schema == ANCHOR_SCHEMA && a.ordinal < CHAIN_MAX,
        "partial anchor schema/ordinal"
    );
    for hash in [&a.binding_sha256, &a.run_sha256, &a.attempt_sha256] {
        ensure!(
            crate::common::valid_hex(hash, 64),
            "partial anchor digest grammar"
        );
    }
    validate_remote(&a.remote)?;
    validate_ref(&a.receipt, METADATA_MAX, true)?;
    validate_ref(&a.catalog, METADATA_MAX, false)?;
    budget.charge(1024, "catalog key grammar")?;
    ensure!(
        a.catalog.key
            == format!(
                "ml-dataset/interrupted-attempts/{}/{}/snapshots/{:020}.json",
                a.run_sha256, a.attempt_sha256, a.ordinal
            ),
        "partial catalog key identity"
    );
    Ok(())
}
fn claims(a: bool, b: bool, c: bool) -> Result<()> {
    ensure!(
        !a && !b && !c,
        "partial prohibited completion/restoration/science claim"
    );
    Ok(())
}
fn read_metadata<T: DeserializeOwned>(
    source: &mut dyn ObjectReader,
    reference: &ObjectRef,
    cas: bool,
    path: &str,
    store: Option<&Root>,
    budget: &mut Budget,
) -> Result<T> {
    let admitted = AdmittedRef::new(reference, METADATA_MAX, cas, budget)?;
    budget.raw(reference.bytes)?;
    let mut input = source.open(&admitted, path, budget)?;
    let raw = io::exact_bytes(&mut input, reference.bytes, &reference.sha256, budget)?;
    if let Some(store) = store {
        budget.reserve_disk(reference.bytes)?;
        store.write_bytes(path, &raw, budget)?;
    }
    io::typed(&raw, budget)
}
pub(crate) fn load(
    source: &mut dyn ObjectReader,
    terminal: &Anchor,
    store: Option<&Root>,
    budget: &mut Budget,
) -> Result<Vec<Snapshot>> {
    budget.charge(4096 + terminal.remote.len() * 8, "terminal anchor copy")?;
    let mut current = terminal.clone();
    let mut chain = Vec::new();
    let mut seen = BTreeSet::new();
    loop {
        validate_anchor(&current, budget)?;
        ensure!(
            current.remote == terminal.remote
                && current.binding_sha256 == terminal.binding_sha256
                && current.run_sha256 == terminal.run_sha256
                && current.attempt_sha256 == terminal.attempt_sha256,
            "partial predecessor anchor identity"
        );
        budget.charge(256, "receipt cycle set")?;
        ensure!(
            seen.insert(current.receipt.sha256.clone()),
            "partial receipt digest cycle"
        );
        let catalog: Catalog = read_metadata(
            source,
            &current.catalog,
            false,
            &metadata_path(&current, "catalog", budget)?,
            store,
            budget,
        )?;
        claims(
            catalog.cell_complete,
            catalog.solver_state_restorable,
            catalog.scientific_result_verified,
        )?;
        ensure!(
            catalog.schema == CATALOG_SCHEMA
                && catalog.remote == current.remote
                && catalog.binding_sha256 == current.binding_sha256
                && catalog.run_sha256 == current.run_sha256
                && catalog.attempt_sha256 == current.attempt_sha256
                && catalog.ordinal == current.ordinal
                && catalog.receipt == current.receipt,
            "partial catalog/anchor disagreement"
        );
        let receipt: Receipt = read_metadata(
            source,
            &current.receipt,
            true,
            &metadata_path(&current, "receipt", budget)?,
            store,
            budget,
        )?;
        claims(
            receipt.cell_complete,
            receipt.solver_state_restorable,
            receipt.scientific_result_verified,
        )?;
        ensure!(
            receipt.schema == RECEIPT_SCHEMA
                && receipt.remote == current.remote
                && receipt.binding_sha256 == current.binding_sha256
                && receipt.ordinal == current.ordinal,
            "partial receipt/anchor disagreement"
        );
        let manifest: Manifest = read_metadata(
            source,
            &receipt.manifest,
            true,
            &metadata_path(&current, "manifest", budget)?,
            store,
            budget,
        )?;
        claims(
            manifest.cell_complete,
            manifest.solver_state_restorable,
            manifest.scientific_result_verified,
        )?;
        validate_binding(&manifest.binding)?;
        ensure!(
            matches!(manifest.schema.as_str(), MANIFEST_V1 | MANIFEST_V2),
            "partial unknown manifest profile"
        );
        ensure!(
            manifest.ordinal == current.ordinal
                && manifest.previous == receipt.previous
                && binding_digest(&manifest.binding, budget)? == current.binding_sha256
                && manifest.binding.run_sha256 == current.run_sha256
                && manifest.binding.attempt_sha256 == current.attempt_sha256,
            "partial manifest/receipt/anchor disagreement"
        );
        let previous = manifest.previous.as_ref();
        ensure!(
            (current.ordinal == 0) == previous.is_none(),
            "partial missing or unexpected predecessor"
        );
        if let Some(previous) = previous {
            ensure!(
                previous.ordinal.checked_add(1) == Some(current.ordinal),
                "partial predecessor ordinal gap"
            );
        }
        ensure!(
            chain.len() < CHAIN_MAX as usize,
            "consumer_resource_limit: partial chain"
        );
        budget.reserve_vec(&mut chain, 1, "retained chain records")?;
        budget.charge(
            4096 + current.remote.len() * 8,
            "next predecessor anchor copy",
        )?;
        let next = previous.cloned();
        chain.push(Snapshot {
            anchor: current,
            receipt,
            manifest,
        });
        if let Some(next) = next {
            current = next;
        } else {
            break;
        }
    }
    ensure!(
        chain.len() as u64 == terminal.ordinal + 1,
        "partial incomplete predecessor chain"
    );
    chain.reverse();
    Ok(chain)
}

pub(crate) fn context_role(path: &str) -> Option<&'static str> {
    match path {
        "run.json" => Some("restricted-run-binding"),
        "plan.json" => Some("restricted-frozen-plan"),
        "protocol.json" => Some("restricted-frozen-protocol"),
        "config.json" => Some("restricted-provenance"),
        "attempt.json" => Some("attempt-provenance"),
        "public.json" => Some("public-instance"),
        "witness.json" => Some("restricted-known-answer"),
        "factor-base.json" => Some("factor-base"),
        "host.json" => Some("host-measurement-context"),
        "worker-process.json" => Some("process-provenance"),
        "measurement-policy.json" => Some("restricted-measurement-policy"),
        "telemetry/curve-construction-start.json"
        | "telemetry/known-answer-replay-start.json"
        | "telemetry/factor-base-replay-start.json"
        | "telemetry/solver-start.json" => Some("phase-start"),
        "telemetry/curve-construction.json"
        | "telemetry/known-answer-replay.json"
        | "telemetry/factor-base-replay.json"
        | "telemetry/solver.json" => Some("hardware-and-resource-telemetry"),
        _ => None,
    }
}
pub(crate) fn coverage(value: &Coverage, capture: Option<&CaptureState>) -> Result<()> {
    ensure!(
        value.artifact_only
            && !value.in_flight_solver_memory_captured
            && !value.uncommitted_event_tail_captured
            && !value.unemitted_backend_internals_captured
            && !value.solver_resume_supported,
        "partial prohibited coverage claim"
    );
    ensure!(
        value.capture_present == capture.is_some()
            && value.committed_events == capture.map_or(0, |s| s.cursor.event_count)
            && value.captured_event_end == capture.map_or(0, |s| s.cursor.byte_end)
            && value.stop == capture.and_then(|s| s.stop)
            && value.capture_stopped == capture.is_some_and(|s| s.stop.is_some()),
        "partial coverage disagreement"
    );
    Ok(())
}
fn initial_context(m: &Manifest) -> Result<()> {
    for (name, expected) in [
        ("run.json", &m.binding.run_sha256),
        ("plan.json", &m.binding.plan_sha256),
        ("protocol.json", &m.binding.protocol_sha256),
        ("config.json", &m.binding.config_sha256),
        ("attempt.json", &m.binding.attempt_sha256),
    ] {
        let a = m
            .context
            .iter()
            .find(|a| a.path.strip_prefix("context/") == Some(name))
            .context("partial initial authoritative context missing")?;
        ensure!(
            &a.sha256 == expected,
            "partial initial raw context digest disagreement"
        );
    }
    if m.schema == MANIFEST_V2 {
        ensure!(
            m.context
                .iter()
                .any(|a| a.path == "context/measurement-policy.json"),
            "partial initial checked policy missing"
        );
    }
    Ok(())
}
pub(crate) fn preflight<'a>(chain: &'a [Snapshot], budget: &mut Budget) -> Result<Preflight<'a>> {
    let first = chain.first().context("partial empty lineage")?;
    let checked = first.manifest.schema == MANIFEST_V2;
    let mut paths = BTreeSet::<&str>::new();
    let mut context = BTreeSet::<&str>::new();
    let mut context_bytes = 0u64;
    let mut previous_observation: Option<&Observation> = None;
    let mut capture = None;
    let mut members = Vec::new();
    let mut artifact_bytes = 0u64;
    for (ordinal, snapshot) in chain.iter().enumerate() {
        let m = &snapshot.manifest;
        ensure!(
            m.ordinal == ordinal as u64
                && m.schema == first.manifest.schema
                && m.binding == first.manifest.binding,
            "partial profile or binding switch"
        );
        validate_observation(&m.observation)?;
        if let Some(old) = previous_observation {
            ensure!(
                m.observation.observed_unix_seconds >= old.observed_unix_seconds
                    && (old.status == Status::Running || m.observation.status != Status::Running),
                "partial observation regression"
            );
        }
        previous_observation = Some(&m.observation);
        let delta_artifacts = m.delta.as_ref().map_or(&[][..], |d| d.artifacts.as_slice());
        let count = m
            .context
            .len()
            .checked_add(delta_artifacts.len())
            .context("partial snapshot artifact overflow")?;
        ensure!(
            count <= SNAPSHOT_ARTIFACTS && m.objects.len() == count,
            "consumer_resource_limit: snapshot artifacts/membership"
        );
        ensure!(
            snapshot.receipt.artifact_count == count as u64,
            "partial receipt artifact count"
        );
        let mut bytes = 0u64;
        for (artifact, object) in m.context.iter().chain(delta_artifacts).zip(&m.objects) {
            validate_artifact(artifact)?;
            ensure!(
                &object.artifact == artifact
                    && object.object.sha256 == artifact.sha256
                    && object.object.bytes == artifact.bytes,
                "partial exact ordered object membership"
            );
            budget.charge(256, "global path and prefix inventory")?;
            ensure!(
                paths.insert(&artifact.path),
                "partial duplicate or redefined artifact path"
            );
            let mut ancestor = artifact.path.as_str();
            while let Some((parent, _)) = ancestor.rsplit_once('/') {
                ensure!(
                    !paths.contains(parent),
                    "partial file/directory prefix conflict"
                );
                ancestor = parent;
            }
            // Existing descendants may precede their conflicting parent.
            if let Some(after) = paths.range(artifact.path.as_str()..).nth(1) {
                ensure!(
                    !(after.starts_with(artifact.path.as_str())
                        && after.as_bytes().get(artifact.path.len()) == Some(&b'/')),
                    "partial file/directory prefix conflict"
                );
            }
            let admitted = AdmittedRef::new(&object.object, ARTIFACT_MAX, true, budget)?;
            budget.reserve_vec(&mut members, 1, "admitted artifact references")?;
            members.push(Member {
                artifact,
                object: admitted,
            });
            bytes = bytes
                .checked_add(artifact.bytes)
                .context("partial artifact byte sum overflow")?;
        }
        ensure!(
            snapshot.receipt.artifact_bytes == bytes,
            "partial receipt artifact bytes"
        );
        artifact_bytes = artifact_bytes
            .checked_add(bytes)
            .context("partial total artifact bytes")?;
        for a in &m.context {
            let path = a
                .path
                .strip_prefix("context/")
                .context("partial context namespace")?;
            let role = context_role(path).context("partial context path allowlist")?;
            ensure!(a.encoding == "json", "partial context encoding");
            if checked {
                ensure!(a.role == role, "partial context role disagreement");
                let cap = if path == "measurement-policy.json" {
                    16384
                } else if a.role == "phase-start" {
                    65536
                } else if a.role == "hardware-and-resource-telemetry" {
                    524288
                } else {
                    ARTIFACT_MAX
                };
                ensure!(
                    a.bytes <= cap,
                    "consumer_resource_limit: checked context file"
                );
            } else {
                ensure!(
                    path != "measurement-policy.json" && !path.starts_with("telemetry/"),
                    "partial legacy policy/phase smuggling"
                );
            }
            budget.charge(256, "context inventory set")?;
            ensure!(context.insert(path), "partial repeated context identity");
            context_bytes = context_bytes
                .checked_add(a.bytes)
                .context("partial context bytes overflow")?;
            ensure!(
                context.len() <= if checked { 19 } else { 10 },
                "consumer_resource_limit: context identities"
            );
            ensure!(
                !checked || context_bytes <= CONTEXT_BYTES_MAX,
                "consumer_resource_limit: checked context bytes"
            );
        }
        if ordinal == 0 {
            initial_context(m)?;
        }
        if let Some(delta) = &m.delta {
            capture = Some(capture::admit_delta(
                delta,
                capture.as_ref(),
                &m.binding,
                budget,
            )?);
        }
        coverage(&m.coverage, capture.as_ref())?;
        if checked {
            let mut permits = true;
            for (start, end) in [
                (
                    "telemetry/curve-construction-start.json",
                    "telemetry/curve-construction.json",
                ),
                (
                    "telemetry/known-answer-replay-start.json",
                    "telemetry/known-answer-replay.json",
                ),
                (
                    "telemetry/factor-base-replay-start.json",
                    "telemetry/factor-base-replay.json",
                ),
                ("telemetry/solver-start.json", "telemetry/solver.json"),
            ] {
                let s = context.contains(start);
                let e = context.contains(end);
                ensure!(
                    (!e || s) && (!s || permits),
                    "partial structural phase prefix gap"
                );
                permits = s && e;
            }
        }
    }
    Ok(Preflight {
        members,
        profile: if checked { MANIFEST_V2 } else { MANIFEST_V1 },
        context_count: context.len(),
        artifact_bytes,
    })
}
