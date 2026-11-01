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
    let (env_class_id, env_class_sha256) = super::host::recompute_class_identity(&host)?;
    if host.schema != "ecbench.host/v1"
        || host.build.binary_sha256 != session.binary_sha256
        || host.env_class_id != session.env_class_id
        || env_class_id != host.env_class_id
        || env_class_sha256 != host.env_class_sha256
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

#[cfg(test)]
mod typed_wrapper_tests {
    // Synthetic metadata tests of the actual wrapper; no production owner/custody claim.
    use super::super::{diagnostic_launch, host, isolation, runner};
    use super::*;
    use serde_json::{json, Value};
    fn bytes(v: &impl serde::Serialize) -> Vec<u8> {
        serde_json::to_vec(v).unwrap()
    }
    struct Data {
        routes: Vec<(String, Vec<u8>, Vec<u8>)>,
        artifacts: Vec<(String, Vec<u8>)>,
    }
    impl Data {
        fn load() -> Self {
            let v:Value=serde_json::from_slice(&std::fs::read(std::env::var("PENCIL_RETAINED_INVENTORY").expect("set PENCIL_RETAINED_INVENTORY to the explicitly hydrated retained inventory manifest")).unwrap()).unwrap();
            Self {
                routes: v["routes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|r| {
                        (
                            r["name"].as_str().unwrap().into(),
                            std::fs::read(r["path"].as_str().unwrap()).unwrap(),
                            std::fs::read(r["spec_path"].as_str().unwrap()).unwrap(),
                        )
                    })
                    .collect(),
                artifacts: v["artifacts"]
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            std::fs::read(v["path"].as_str().unwrap()).unwrap(),
                        )
                    })
                    .collect(),
            }
        }
        fn inputs(&self) -> Vec<Input<'_>> {
            self.routes
                .iter()
                .map(|(r, p, s)| Input {
                    route: r,
                    plan: p,
                    spec: s,
                })
                .collect()
        }
        fn artifacts(&self) -> Vec<(&str, &[u8])> {
            self.artifacts
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_slice()))
                .collect()
        }
    }
    fn host_fixture() -> HostCapsule {
        let mut h:HostCapsule=serde_json::from_value(json!({"schema":"ecbench.host/v1","env_class_id":"ECBENV3hfixture","env_class_sha256":"pending-derivation","stable":{"os":"linux","arch":"aarch64","features":[],"logical_cpus":1,"numa_nodes":1,"topology":{"cpus":[],"nodes":[]},"perf_levels":[],"cpu_settings":{},"virtualization":{}},"build":{"binary_sha256":"d".repeat(64),"debug_assertions":true,"target_os":"linux","target_arch":"aarch64","build_env":{}},"captured_unix_ms":42})).unwrap();
        (h.env_class_id, h.env_class_sha256) = host::recompute_class_identity(&h).unwrap();
        h
    }
    // Mutation occurs before ALL derived byte hashes/identities/contexts are built.
    fn check(d: &Data, i: usize, mutation: &str) -> Result<Report, String> {
        let (route, planb, specb) = &d.routes[i];
        let plan: PlanDoc = serde_json::from_slice(planb).unwrap();
        let spec = Spec::from_json(std::str::from_utf8(specb).unwrap()).unwrap();
        let (spec_id, spec_sha) = short_id("ECS1", &spec.identity_view()).unwrap();
        let mut h = host_fixture();
        let mut session_binary = Some("d".repeat(64));
        match mutation {
            "host_schema" => h.schema = "invalid".into(),
            "host_class_full_digest" => {
                let replacement = if h.env_class_sha256.ends_with('0') {
                    "1"
                } else {
                    "0"
                };
                h.env_class_sha256.replace_range(63..64, replacement);
            }
            "host_binary" => h.build.binary_sha256 = Some("a".repeat(64)),
            "session_binary" => session_binary = Some("a".repeat(64)),
            "class_recompute" => h.stable.logical_cpus = 2,
            "launch_binary" => {
                h.build.binary_sha256 = Some("a".repeat(64));
                session_binary = Some("a".repeat(64));
            }
            _ => (),
        }
        let hb = bytes(&h);
        let hh = sha256_hex(&hb);
        let ph = sha256_hex(planb);
        let sid=short_id("ECBS1",&json!({"spec_sha256":spec_sha,"host_sha256":hh,"plan_sha256":ph,"started_unix_ms":"42"})).unwrap().0;
        let profile = json!({"version":diagnostic_launch::PROFILE_VERSION,"sha256":"a".repeat(64)});
        let arms:Vec<_>=plan.arms.iter().map(|a|json!({"name":a.name,"role":a.role,"method":a.method.id,"method_config_sha256":diagnostic_launch::resolved_method_binding(&a.method).unwrap(),"profile":profile})).collect();
        let mut lv = json!({"schema":diagnostic_launch::LAUNCH_SCHEMA,"expected_spec_sha256":spec_sha,"binary_sha256":"d".repeat(64),"protocol_sha256":"e".repeat(64),"source_receipt":{"scope":"external_manifest_binding_only","receipt_sha256":"f".repeat(64),"source_manifest_sha256":"1".repeat(64)},"scientific_run_id":"synthetic-preflight-unit-test","attempt":0,"profile":profile,"arms":arms});
        if mutation == "spec_binding" {
            lv["expected_spec_sha256"] = json!("b".repeat(64));
        }
        if mutation == "arm_order" {
            lv["arms"].as_array_mut().unwrap().swap(0, 1);
        }
        let lb = bytes(&lv);
        let launch = DiagnosticLaunch::from_bytes(&lb).unwrap();
        let options = runner::RunOptions {
            diagnostic_launch: Some(launch.clone()),
            cpus: isolation::CpuRequest::None,
            lock_path: "synthetic-never-opened".into(),
            wait_for_lock: false,
            allow_busy: false,
            thresholds: isolation::Thresholds::default(),
            exe: "synthetic-never-executed".into(),
        };
        let env = if mutation == "session_environment" {
            "changed".to_string()
        } else {
            h.env_class_id.clone()
        };
        let s:Session=serde_json::from_value(json!({"schema":"ecbench.session/v1","session_id":sid,"label":"synthetic fixture","spec_id":spec_id,"spec_sha256":spec_sha,"env_class_id":env,"host_sha256":hh,"plan_sha256":ph,"binary_sha256":session_binary,"options":options,"child_env_sha256":"2".repeat(64),"child_env":{},"started_unix_ms":42,"status":"running","executions_planned":plan.executions.len(),"records_written":0,"status_counts":{}})).unwrap();
        let sb = bytes(&s);
        let prep = json!({"schema":"ecbench.session_preparation/v1","clock_type":"linux_CLOCK_MONOTONIC","origin_ns":12,"boot_id":"fixture-boot","time_namespace":"fixture-ns","time_namespace_offsets":"monotonic 0 0\n","origin_boundary":"run_session entry before parsing; excludes CLI/process startup and input acquisition","observations":{"lock_wait_started_ns":1,"lock_acquired_ns":2,"execute_entry_ns":3},"preparation_wall_ns":3,"lock_wait_wall_ns":1,"lock_wait_is_subset_of_preparation":true,"interval_kind":"wall_only","active_cpu_ns":null,"active_worker_ns":null,"external_alignment_supported":true,"external_alignment_conditions":"same verified boot/time namespace and raw CLOCK_MONOTONIC; authenticated owner receipt required","restore_alignment_supported":false,"external_exit_observation":null,"final_publication_tail_pending":true,"session_id":sid,"runner_id":format!("{sid}-runner-17"),"clock_identity_rule":"runner_id + '-preparation-sha256-' + lowercase SHA256 of exact session-preparation.json bytes; file deliberately omits clock_id to avoid a cycle","launch_sha256":sha256_hex(&lb),"spec_sha256":spec_sha,"plan_sha256":ph,"host_sha256":hh});
        let pb = bytes(&prep);
        let synthetic_owner = OwnerObservation {
            session_sha256: sha256_hex(&sb),
            host_sha256: hh,
            preparation_sha256: sha256_hex(&pb),
            binary_sha256: session_binary.unwrap(),
            runner_pid: 17,
            boot_id: "fixture-boot".into(),
            time_namespace: "fixture-ns".into(),
            time_namespace_offsets: "monotonic 0 0\n".into(),
            origin_ns: 12,
        };
        let e = &plan.executions[0];
        let contexts = vec![bytes(
            &prepare_lifecycle_child(
                &launch,
                &sid,
                e,
                &plan.arms[e.arm],
                &plan.workloads[e.workload],
                std::path::Path::new("synthetic-never-written"),
            )
            .unwrap()
            .envelope,
        )];
        check_actual_session(
            &d.inputs(),
            &d.artifacts(),
            ActualSession {
                route,
                plan: planb,
                spec: specb,
                host: &hb,
                session: &sb,
                preparation: &pb,
                launch: &lb,
                actual_contexts: &contexts,
                owner_observation: &synthetic_owner,
            },
        )
    }
    #[test]
    #[ignore = "requires explicitly hydrated retained inventory"]
    fn typed_retained_nine_positive() {
        let d = Data::load();
        assert_eq!(d.routes.len(), 9);
        let static_r = check_inventory(&d.inputs(), &d.artifacts()).unwrap();
        assert!(!static_r.software_bindings_complete);
        assert!(!static_r.scientific_execution_admitted);
        for i in 0..9 {
            let r = check(&d, i, "").unwrap_or_else(|e| panic!("{}: {e}", d.routes[i].0));
            assert!(r.software_bindings_complete);
            assert!(!r.scientific_execution_admitted);
            assert_eq!(r.actual_context_hashes.len(), 1);
            assert!(r.journal_upper_bound_bytes.unwrap() > 0);
        }
    }
    #[test]
    #[ignore = "requires explicitly hydrated retained inventory"]
    fn typed_rebound_host_joins() {
        let d = Data::load();
        for m in [
            "host_schema",
            "host_class_full_digest",
            "host_binary",
            "session_binary",
            "class_recompute",
            "session_environment",
        ] {
            assert_eq!(
                check(&d, 8, m).unwrap_err(),
                "host schema/build/environment mismatch",
                "mutation={m}"
            );
        }
    }
    #[test]
    #[ignore = "requires explicitly hydrated retained inventory"]
    fn typed_rebound_launch_spec_policy() {
        let d = Data::load();
        for m in ["launch_binary", "spec_binding"] {
            assert_eq!(
                check(&d, 8, m).unwrap_err(),
                "diagnostic launch spec/binary binding mismatch or missing binary hash",
                "mutation={m}"
            );
        }
        assert_eq!(
            check(&d, 8, "arm_order").unwrap_err(),
            "diagnostic launch differs from exact ordered spec arms"
        );
    }
}
