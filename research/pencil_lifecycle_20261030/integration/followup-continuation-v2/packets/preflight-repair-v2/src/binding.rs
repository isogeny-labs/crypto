//! Structural binding core, not execution or authenticated-custody admission.
//! Caller must use actual owner observations from its separately verified custody path.
use crate::canonical::{reject_duplicate_keys, sha256_hex, short_id};
use serde_json::{json, Value};
use std::collections::BTreeSet;
const RULE:&str="runner_id + '-preparation-sha256-' + lowercase SHA256 of exact session-preparation.json bytes; file deliberately omits clock_id to avoid a cycle";
const BOUNDARY: &str =
    "run_session entry before parsing; excludes CLI/process startup and input acquisition";
pub struct Evidence<'a> {
    pub preparation: &'a [u8],
    pub session: &'a [u8],
    pub host: &'a [u8],
    pub plan: &'a [u8],
    pub launch: &'a [u8],
    /// Derived with the actual Spec::identity_view and canonical::short_id.
    pub spec_id: &'a str,
    pub spec_sha256: &'a str,
    pub owner: &'a OwnerObservation,
}
/// Comparison input from the owner verifier; this struct cannot authenticate its own provenance.
#[derive(Clone)]
pub struct OwnerObservation {
    pub session_sha256: String,
    pub host_sha256: String,
    pub preparation_sha256: String,
    pub binary_sha256: String,
    pub runner_pid: u64,
    pub boot_id: String,
    pub time_namespace: String,
    pub time_namespace_offsets: String,
    pub origin_ns: u64,
}
#[derive(Debug)]
pub struct BoundSession {
    pub session_id: String,
    pub runner_id: String,
    pub clock_id: String,
}
pub fn parse(raw: &[u8], cap: usize) -> Result<Value, String> {
    if raw.len() > cap {
        return Err("input cap".into());
    }
    let s = std::str::from_utf8(raw).map_err(|e| e.to_string())?;
    reject_duplicate_keys(s)?;
    serde_json::from_slice(raw).map_err(|e| e.to_string())
}
fn text<'a>(v: &'a Value, k: &str) -> Result<&'a str, String> {
    v[k].as_str()
        .filter(|x| !x.is_empty())
        .ok_or_else(|| format!("missing {k}"))
}
fn u(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64().ok_or_else(|| format!("missing integer {k}"))
}
fn require(ok: bool, msg: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
pub fn verify(e: Evidence<'_>) -> Result<BoundSession, String> {
    let p = parse(e.preparation, 65536)?;
    let s = parse(e.session, 4 * 1024 * 1024)?;
    let h = parse(e.host, 4 * 1024 * 1024)?;
    let plan = parse(e.plan, 32 * 1024 * 1024)?;
    let l = parse(e.launch, 65536)?;
    let allowed = [
        "schema",
        "clock_type",
        "origin_ns",
        "boot_id",
        "time_namespace",
        "time_namespace_offsets",
        "origin_boundary",
        "observations",
        "preparation_wall_ns",
        "lock_wait_wall_ns",
        "lock_wait_is_subset_of_preparation",
        "interval_kind",
        "active_cpu_ns",
        "active_worker_ns",
        "external_alignment_supported",
        "external_alignment_conditions",
        "restore_alignment_supported",
        "external_exit_observation",
        "final_publication_tail_pending",
        "session_id",
        "runner_id",
        "clock_identity_rule",
        "launch_sha256",
        "spec_sha256",
        "plan_sha256",
        "host_sha256",
    ];
    let obj = p.as_object().ok_or("preparation not object")?;
    require(
        obj.len() == allowed.len() && allowed.iter().all(|k| obj.contains_key(*k)),
        "preparation fields",
    )?;
    require(
        p["schema"] == "ecbench.session_preparation/v1"
            && p["clock_type"] == "linux_CLOCK_MONOTONIC"
            && p["clock_identity_rule"] == RULE
            && p["origin_boundary"] == BOUNDARY,
        "preparation schema/clock rule",
    )?;
    require(
        p["lock_wait_is_subset_of_preparation"] == true
            && p["interval_kind"] == "wall_only"
            && p["external_alignment_supported"] == true
            && p["restore_alignment_supported"] == false
            && p["final_publication_tail_pending"] == true,
        "preparation clock flags",
    )?;
    require(p["external_alignment_conditions"]=="same verified boot/time namespace and raw CLOCK_MONOTONIC; authenticated owner receipt required","alignment condition")?;
    for k in [
        "active_cpu_ns",
        "active_worker_ns",
        "external_exit_observation",
    ] {
        require(p[k].is_null(), "unknown metric must stay null")?;
    }
    let obs = p["observations"].as_object().ok_or("observations")?;
    require(
        obs.len() == 3
            && [
                "lock_wait_started_ns",
                "lock_acquired_ns",
                "execute_entry_ns",
            ]
            .iter()
            .all(|k| obs.contains_key(*k)),
        "observation fields",
    )?;
    let start=u(&p["observations"],"lock_wait_started_ns")?;
    let acquired=u(&p["observations"],"lock_acquired_ns")?;
    let entry=u(&p["observations"],"execute_entry_ns")?;
    require(start<=acquired && acquired<=entry,"preparation observation order")?;
    require(
        p["preparation_wall_ns"] == p["observations"]["execute_entry_ns"],
        "preparation wall",
    )?;
    let wait = match (
        p["observations"]["lock_wait_started_ns"].as_u64(),
        p["observations"]["lock_acquired_ns"].as_u64(),
    ) {
        (Some(a), Some(b)) => json!(b.checked_sub(a).ok_or("lock time reversal")?),
        _ => Value::Null,
    };
    require(p["lock_wait_wall_ns"] == wait, "lock wait wall")?;
    for k in [
        "session_id",
        "runner_id",
        "boot_id",
        "time_namespace",
        "time_namespace_offsets",
    ] {
        text(&p, k)?;
    }
    let ph = sha256_hex(e.plan);
    let hh = sha256_hex(e.host);
    let sh = sha256_hex(e.session);
    let prep = sha256_hex(e.preparation);
    require(
        s["schema"] == "ecbench.session/v1" && plan["schema"] == "ecbench.plan/v1",
        "session/plan schema",
    )?;
    require(
        s["spec_id"] == e.spec_id
            && plan["spec_id"] == e.spec_id
            && s["spec_sha256"] == e.spec_sha256
            && p["spec_sha256"] == e.spec_sha256
            && l["expected_spec_sha256"] == e.spec_sha256,
        "spec lineage",
    )?;
    require(
        s["plan_sha256"] == ph
            && p["plan_sha256"] == ph
            && s["host_sha256"] == hh
            && p["host_sha256"] == hh,
        "plan/host join",
    )?;
    let started = u(&s, "started_unix_ms")?;
    let sid=short_id("ECBS1",&json!({"spec_sha256":e.spec_sha256,"host_sha256":hh,"plan_sha256":ph,"started_unix_ms":started.to_string()}))?.0;
    require(
        s["session_id"] == sid && p["session_id"] == sid,
        "derived ECBS1 identity",
    )?;
    let runner = format!("{sid}-runner-{}", e.owner.runner_pid);
    require(
        e.owner.runner_pid > 0 && p["runner_id"] == runner,
        "owner runner PID",
    )?;
    require(
        s["binary_sha256"] == l["binary_sha256"] && s["binary_sha256"] == e.owner.binary_sha256,
        "binary join",
    )?;
    require(p["launch_sha256"] == sha256_hex(e.launch), "launch hash")?;
    let opts = &s["options"]["diagnostic_launch"];
    require(
        opts["raw_json"].as_str() == std::str::from_utf8(e.launch).ok()
            && opts["raw_sha256"] == sha256_hex(e.launch),
        "session diagnostic launch",
    )?;
    let slots = plan["executions"]
        .as_array()
        .ok_or("plan executions")?
        .len() as u64;
    require(
        u(&s, "executions_planned")? == slots && u(&s, "records_written")? <= slots,
        "session execution counts",
    )?;
    require(
        ["running", "complete", "interrupted"].contains(&text(&s, "status")?),
        "session status",
    )?;
    if s["status"] == "complete" {
        require(
            s["records_written"] == slots && s["finished_unix_ms"].as_u64().is_some(),
            "complete session bounds",
        )?;
    }
    if s["status"] == "running" {
        require(s["finished_unix_ms"].is_null(), "running session finished")?;
    }
    require(h["schema"]=="ecbench.host/v1","host schema")?;
    require(h["build"]["binary_sha256"]==s["binary_sha256"],"host/session binary")?;
    require(text(&h,"env_class_id")?==text(&s,"env_class_id")?,"host/session environment class")?;
    require(
        e.owner.session_sha256 == sh
            && e.owner.host_sha256 == hh
            && e.owner.preparation_sha256 == prep,
        "owner exact bytes",
    )?;
    require(
        p["boot_id"] == e.owner.boot_id
            && p["time_namespace"] == e.owner.time_namespace
            && p["time_namespace_offsets"] == e.owner.time_namespace_offsets
            && u(&p, "origin_ns")? == e.owner.origin_ns,
        "owner clock observations",
    )?;
    Ok(BoundSession {
        session_id: sid,
        runner_id: runner.clone(),
        clock_id: format!("{runner}-preparation-sha256-{prep}"),
    })
}
/// Validate exactly the supplied actual ordered subset; does not require future contexts.
pub fn context_keys(rows: &[(u64, u64, String)], planned: &[u64]) -> Result<(), String> {
    require(!rows.is_empty(), "actual context subset empty")?;
    let mut seen = BTreeSet::new();
    let mut leaves = BTreeSet::new();
    let mut prior = None;
    for (seq, attempt, leaf) in rows {
        let pos = planned
            .iter()
            .position(|v| v == seq)
            .ok_or("context not enrolled")?;
        require(prior.is_none_or(|x| pos > x), "context plan order")?;
        prior = Some(pos);
        require(
            seen.insert(*seq) && leaves.insert(leaf.clone()),
            "duplicate context",
        )?;
        require(
            *leaf == format!("diagnostic-{seq:06}-attempt-{attempt}"),
            "relative leaf mismatch",
        )?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture {
        p: Value,
        s: Value,
        h: Value,
        plan: Value,
        l: Value,
    }
    fn bytes(v: &Value) -> Vec<u8> {
        serde_json::to_vec(v).unwrap()
    }
    fn fixture() -> Fixture {
        let h = json!({"schema":"ecbench.host/v1","fixture":true,"env_class_id":"fixture-class","build":{"binary_sha256":"binary-hash"}});
        let plan = json!({"schema":"ecbench.plan/v1","spec_id":"spec-fixture","executions":[{"seq":0},{"seq":1}]});
        let l = json!({"expected_spec_sha256":"spec-hash","binary_sha256":"binary-hash"});
        let hh = sha256_hex(&bytes(&h));
        let ph = sha256_hex(&bytes(&plan));
        let lb = bytes(&l);
        let lh = sha256_hex(&lb);
        let sid=short_id("ECBS1",&json!({"spec_sha256":"spec-hash","host_sha256":hh,"plan_sha256":ph,"started_unix_ms":"42"})).unwrap().0;
        let p = json!({"schema":"ecbench.session_preparation/v1","clock_type":"linux_CLOCK_MONOTONIC","origin_ns":12,"boot_id":"fixture-boot","time_namespace":"fixture-ns","time_namespace_offsets":"monotonic 0 0\n","origin_boundary":BOUNDARY,"observations":{"lock_wait_started_ns":1,"lock_acquired_ns":2,"execute_entry_ns":3},"preparation_wall_ns":3,"lock_wait_wall_ns":1,"lock_wait_is_subset_of_preparation":true,"interval_kind":"wall_only","active_cpu_ns":null,"active_worker_ns":null,"external_alignment_supported":true,"external_alignment_conditions":"same verified boot/time namespace and raw CLOCK_MONOTONIC; authenticated owner receipt required","restore_alignment_supported":false,"external_exit_observation":null,"final_publication_tail_pending":true,"session_id":sid,"runner_id":format!("{sid}-runner-17"),"clock_identity_rule":RULE,"launch_sha256":lh,"spec_sha256":"spec-hash","plan_sha256":ph,"host_sha256":hh});
        let s = json!({"schema":"ecbench.session/v1","env_class_id":"fixture-class","session_id":sid,"spec_id":"spec-fixture","spec_sha256":"spec-hash","host_sha256":hh,"plan_sha256":ph,"binary_sha256":"binary-hash","started_unix_ms":42,"finished_unix_ms":null,"status":"running","executions_planned":2,"records_written":0,"options":{"diagnostic_launch":{"raw_json":String::from_utf8(lb).unwrap(),"raw_sha256":lh}}});
        Fixture { p, s, h, plan, l }
    }
    impl Fixture {
        fn check(&self) -> Result<BoundSession, String> {
            let (pb, sb, hb, planb, lb) = (
                bytes(&self.p),
                bytes(&self.s),
                bytes(&self.h),
                bytes(&self.plan),
                bytes(&self.l),
            );
            let owner = OwnerObservation {
                session_sha256: sha256_hex(&sb),
                host_sha256: sha256_hex(&hb),
                preparation_sha256: sha256_hex(&pb),
                binary_sha256: "binary-hash".into(),
                runner_pid: 17,
                boot_id: "fixture-boot".into(),
                time_namespace: "fixture-ns".into(),
                time_namespace_offsets: "monotonic 0 0\n".into(),
                origin_ns: 12,
            };
            verify(Evidence {
                preparation: &pb,
                session: &sb,
                host: &hb,
                plan: &planb,
                launch: &lb,
                spec_id: "spec-fixture",
                spec_sha256: "spec-hash",
                owner: &owner,
            })
        }
    }
    #[test]
    fn actual_stage_rejects_all_missing_and_minimal_prep() {
        let f = fixture();
        assert!(f.check().is_ok());
        for v in [
            json!({}),
            json!({"session_id":f.p["session_id"],"runner_id":f.p["runner_id"],"plan_sha256":f.p["plan_sha256"],"launch_sha256":f.p["launch_sha256"]}),
        ] {
            let mut bad = fixture();
            bad.p = v;
            assert!(bad.check().is_err());
        }
    }
    #[test]
    fn preparation_mutations_fail_even_if_owner_byte_pins_updated() {
        for k in [
            "schema",
            "clock_type",
            "origin_boundary",
            "clock_identity_rule",
            "session_id",
            "runner_id",
            "spec_sha256",
            "plan_sha256",
            "host_sha256",
            "launch_sha256",
            "boot_id",
            "time_namespace",
            "time_namespace_offsets",
            "external_alignment_conditions",
        ] {
            let mut f = fixture();
            f.p[k] = json!("changed");
            assert!(f.check().is_err(), "{k}");
        }
        for k in [
            "active_worker_ns",
            "active_cpu_ns",
            "external_exit_observation",
            "origin_ns",
        ] {
            let mut f = fixture();
            f.p[k] = json!(999);
            assert!(f.check().is_err(), "{k}");
        }
        let mut f = fixture();
        f.p["clock_id"] = json!("manufactured");
        assert!(f.check().is_err());
    }
    #[test]
    fn session_host_plan_and_launch_mutations_fail() {
        for k in [
            "schema",
            "session_id",
            "spec_id",
            "spec_sha256",
            "host_sha256",
            "plan_sha256",
            "binary_sha256",
            "status",
        ] {
            let mut f = fixture();
            f.s[k] = json!("changed");
            assert!(f.check().is_err(), "{k}");
        }
        for k in ["started_unix_ms", "executions_planned", "records_written"] {
            let mut f = fixture();
            f.s[k] = json!(999);
            assert!(f.check().is_err(), "{k}");
        }
        let mut f = fixture();
        f.h["changed"] = json!(true);
        assert!(f.check().is_err());
        let mut f = fixture();
        f.l["binary_sha256"] = json!("changed");
        assert!(f.check().is_err());
        let mut f = fixture();
        f.s["options"]["diagnostic_launch"]["raw_json"] = json!("{}");
        assert!(f.check().is_err());
    }
    #[test]
    fn self_consistent_changed_session_still_requires_derived_ecbs1() {
        let mut f = fixture();
        f.s["session_id"] = json!("fake");
        f.p["session_id"] = json!("fake");
        f.p["runner_id"] = json!("fake-runner-17");
        assert!(f.check().is_err());
    }
    #[test]
    fn leaf_order_and_subset_are_strict() {
        let planned = [10, 11, 12];
        assert!(context_keys(&[(11, 2, "diagnostic-000011-attempt-2".into())], &planned).is_ok());
        assert!(context_keys(&[], &planned).is_err());
        for rows in [
            vec![(11, 2, "execution_11".into())],
            vec![(13, 2, "diagnostic-000013-attempt-2".into())],
            vec![
                (11, 2, "diagnostic-000011-attempt-2".into()),
                (11, 2, "diagnostic-000011-attempt-2".into()),
            ],
            vec![
                (12, 2, "diagnostic-000012-attempt-2".into()),
                (11, 2, "diagnostic-000011-attempt-2".into()),
            ],
        ] {
            assert!(context_keys(&rows, &planned).is_err());
        }
    }
    #[test]
    fn duplicate_json_keys_rejected_before_value_parse() {
        assert!(parse(br#"{"schema":1,"schema":2}"#, 65536).is_err());
    }
    #[test]
    fn actual_preparation_requires_three_ordered_numeric_observations() {
        for k in ["lock_wait_started_ns","lock_acquired_ns","execute_entry_ns"] {
            for v in [Value::Null,json!("3")] {
                let mut f=fixture();f.p["observations"][k]=v;assert!(f.check().is_err());
            }
            let mut f=fixture();f.p["observations"].as_object_mut().unwrap().remove(k);assert!(f.check().is_err());
        }
        for (a,b,c) in [(2,1,3),(1,4,3)] {
            let mut f=fixture();f.p["observations"]=json!({"lock_wait_started_ns":a,"lock_acquired_ns":b,"execute_entry_ns":c});
            f.p["preparation_wall_ns"]=json!(c);f.p["lock_wait_wall_ns"]=json!(0);assert!(f.check().is_err());
        }
    }
    #[test]
    fn host_schema_binary_and_environment_mutations_reject() {
        for k in ["schema","env_class_id"] {let mut f=fixture();f.h[k]=json!("changed");assert!(f.check().is_err());}
        let mut f=fixture();f.h["build"]["binary_sha256"]=json!("changed");assert!(f.check().is_err());
        let mut f=fixture();f.s["env_class_id"]=json!("changed");assert!(f.check().is_err());
    }

}
