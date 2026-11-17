//! Staged retained-plan validation. Neither stage admits scientific execution.
use super::diagnostic_binding_core::{self as core, OwnerObservation};
use super::diagnostic_inventory::{self as inventory, Input};
use super::{
    canonical::{sha256_hex, short_id},
    diagnostic_launch::{ChildEnvelope, DiagnosticLaunch},
    diagnostic_resources::{Identity, SessionResourceJournal},
    host::HostCapsule,
    runner::{prepare_lifecycle_child, resource_journal_capacity, PlanDoc, Session},
    spec::Spec,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Report {
    pub stage: &'static str,
    pub software_bindings_complete: bool,
    pub scientific_execution_admitted: bool,
    pub cohort_workloads: usize,
    pub cohort_slots: usize,
    pub separate_gate_slots: usize,
    pub route: Option<String>,
    pub session_id: Option<String>,
    pub clock_id: Option<String>,
    pub actual_context_hashes: Vec<String>,
    pub journal_upper_bound_bytes: Option<usize>,
    pub external_admission_obligations: Vec<&'static str>,
}
fn report() -> Report {
    Report {
        stage: "static_inventory_only",
        software_bindings_complete: false,
        scientific_execution_admitted: false,
        cohort_workloads: 1260,
        cohort_slots: 22680,
        separate_gate_slots: 18,
        route: None,
        session_id: None,
        clock_id: None,
        actual_context_hashes: vec![],
        journal_upper_bound_bytes: None,
        external_admission_obligations: vec![
            "authenticated_owner_custody",
            "budget_and_deadline_enforcement",
            "prospective_protocol_review_and_freeze",
            "image_qualification_and_publication",
            "timing_qualification",
            "prior_p739_reuse_applicability",
        ],
    }
}
/// Success means inventory checked, explicitly not complete actual bindings.
pub fn check_inventory(
    inputs: &[Input<'_>],
    artifacts: &[(&str, &[u8])],
) -> Result<Report, String> {
    inventory::check(inputs, artifacts)?;
    Ok(report())
}
pub struct ActualSession<'a> {
    pub route: &'a str,
    pub plan: &'a [u8],
    pub spec: &'a [u8],
    pub host: &'a [u8],
    pub session: &'a [u8],
    pub preparation: &'a [u8],
    pub launch: &'a [u8],
    pub actual_contexts: &'a [Vec<u8>],
    pub owner_observation: &'a OwnerObservation,
}
/// Required non-optional input. Verify this actual prepared session and supplied ordered
/// context subset; future contexts/sessions do not need to exist. This is not a launcher.
pub fn check_actual_session(
    inputs: &[Input<'_>],
    artifacts: &[(&str, &[u8])],
    a: ActualSession<'_>,
) -> Result<Report, String> {
    inventory::check(inputs, artifacts)?;
    inventory::verify_route(a.route, a.plan, a.spec)?;
    let launch = DiagnosticLaunch::from_bytes(a.launch)?;
    let doc = launch.document()?;
    let plan: PlanDoc = serde_json::from_value(core::parse(a.plan, 32 * 1024 * 1024)?)
        .map_err(|e| e.to_string())?;
    let session: Session = serde_json::from_value(core::parse(a.session, 4 * 1024 * 1024)?)
        .map_err(|e| e.to_string())?;
    let host: HostCapsule =
        serde_json::from_value(core::parse(a.host, 4 * 1024 * 1024)?).map_err(|e| e.to_string())?;
    core::parse(a.spec, 32 * 1024 * 1024)?; // reject duplicate keys before typed parsing
    let spec = Spec::from_json(std::str::from_utf8(a.spec).map_err(|e| e.to_string())?)?;
    let (spec_id, spec_sha) = short_id("ECS1", &spec.identity_view())?;
    if host.schema != "ecbench.host/v1"
        || host.build.binary_sha256 != session.binary_sha256
        || host.env_class_id != session.env_class_id
        || super::host::recompute_class(&host)? != host.env_class_id
    {
        return Err("host schema/build/environment mismatch".into());
    }
    launch.validate_spec(&spec, &spec_sha, host.build.binary_sha256.as_deref())?;

    let bound = core::verify(core::Evidence {
        preparation: a.preparation,
        session: a.session,
        host: a.host,
        plan: a.plan,
        launch: a.launch,
        spec_id: &spec_id,
        spec_sha256: &spec_sha,
        owner: a.owner_observation,
    })?;
    // Validate every configured arm even when this observed subset contains only one arm.
    if plan.arms.len() != doc.arms.len() {
        return Err("launch arm count".into());
    }
    for (arm, b) in plan.arms.iter().zip(&doc.arms) {
        if arm.name != b.name
            || arm.role != b.role
            || arm.method.id != b.method
            || super::diagnostic_launch::resolved_method_binding(&arm.method)?
                != b.method_config_sha256
        {
            return Err("launch arm/config binding".into());
        }
    }
    let mut keys = vec![];
    let mut hashes = vec![];
    for raw in a.actual_contexts {
        let c: ChildEnvelope =
            serde_json::from_value(core::parse(raw, 65536)?).map_err(|e| e.to_string())?;
        c.validate(&launch)?;
        let seq = c.prefix.identity.execution;
        let e = plan
            .executions
            .iter()
            .find(|e| e.seq == seq)
            .ok_or("context not in plan")?;
        let arm = plan.arms.get(e.arm).ok_or("arm index")?;
        let w = plan.workloads.get(e.workload).ok_or("workload index")?;
        let expected = prepare_lifecycle_child(
            &launch,
            &bound.session_id,
            e,
            arm,
            w,
            std::path::Path::new("unused-metadata-only"),
        )?;
        if c != expected.envelope {
            return Err("context differs from actual producer metadata".into());
        }
        keys.push((
            seq,
            c.prefix.identity.attempt,
            c.prefix.relative_leaf.clone(),
        ));
        hashes.push(sha256_hex(raw));
    }
    core::context_keys(
        &keys,
        &plan.executions.iter().map(|e| e.seq).collect::<Vec<_>>(),
    )?;
    let journal = SessionResourceJournal::begin(
        bound.session_id.clone(),
        bound.runner_id.clone(),
        bound.clock_id.clone(),
        0,
    )?;
    let id = Identity {
        session_id: bound.session_id.clone(),
        runner_id: bound.runner_id,
        clock_id: bound.clock_id.clone(),
        scientific_run_id: doc.scientific_run_id,
        attempt: doc.attempt,
        seq: 0,
        protocol_sha256: doc.protocol_sha256,
        binary_sha256: doc.binary_sha256,
        source_manifest_sha256: doc.source_receipt.source_manifest_sha256,
    };
    let mut out = report();
    out.stage = "actual_session_subset_checked";
    out.software_bindings_complete = true;
    out.route = Some(a.route.into());
    out.session_id = Some(bound.session_id);
    out.clock_id = Some(bound.clock_id);
    out.actual_context_hashes = hashes;
    out.journal_upper_bound_bytes = Some(resource_journal_capacity(
        &journal,
        id,
        plan.executions.len(),
    )?);
    Ok(out)
}
