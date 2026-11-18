//! Synthetic codec controls only. No solver is invoked and no timer is read.
#[path = "../src/bin/ecbench/endpoint_pairs.rs"]
mod endpoint_pairs;

use crypto_lib::cryptanalysis::ecbench::{canonical, isolation, record, runner, spec};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    dir: PathBuf,
    plan: spec::Plan,
    source: String,
    destination: String,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}
fn write(dir: &std::path::Path, name: &str, value: &impl serde::Serialize) -> String {
    let text = serde_json::to_string_pretty(value).unwrap() + "\n";
    std::fs::write(dir.join(name), &text).unwrap();
    canonical::sha256_hex(text.as_bytes())
}
impl Fixture {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "endpoint-codec-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        let input = json!({"schema":"ecbench.spec/v1", "label":"SYNTHETIC CODEC FIXTURE; NOT A RUN",
            "workloads":{"curves":[
                {"kind":"prime_mont4_explicit","p":"223","a":"74","b":"122","group_order":"205","r":41,"gx":"119","gy":"198"},
                {"kind":"prime_mont4_explicit","p":"223","a":"69","b":"3","group_order":"205","r":41,"gx":"30","gy":"23"}],
                "targets_per_curve":2,"target_seed":20261118,"target_kind":"planted_shared"},
            "arms":[{"name":"rho","role":"reference","method":{"id":"rho.negation","params":{}}},
                {"name":"rho-aa","role":"control","method":{"id":"rho.negation","params":{}}}],
            "measurement":{"rounds":6,"warmup":1,"order":"paired_curves_v1","seed":20261118,"algorithm_seed_round":"measured_index"}});
        write(&dir, "spec.json", &input);
        let plan = spec::plan(spec::Spec::from_json(&input.to_string()).unwrap()).unwrap();
        let doc = runner::PlanDoc {
            schema: "ecbench.plan/v1".into(),
            spec_id: plan.spec_id.clone(),
            arms: plan.arms.clone(),
            workloads: plan.workloads.clone(),
            executions: plan.executions.clone(),
        };
        write(&dir, "plan.json", &doc);
        let source = plan.workloads[0].curve.slug.clone();
        let destination = plan.workloads[2].curve.slug.clone();
        Self {
            dir,
            plan,
            source,
            destination,
        }
    }
    fn join(&self, plan_only: bool) -> Result<Value, String> {
        endpoint_pairs::join(&self.dir, &self.source, &self.destination, "rho", plan_only)
            .map(|v| serde_json::to_value(v).unwrap())
    }
    fn record(&self, seq: usize, status: &str) -> record::Record {
        let e = &self.plan.executions[seq];
        let a = &self.plan.arms[e.arm];
        let w = &self.plan.workloads[e.workload];
        let n = self.plan.executions[..=seq]
            .iter()
            .filter(|x| {
                self.plan.arms[x.arm].method.method_id == a.method.method_id
                    && x.workload == e.workload
            })
            .count();
        record::Record {
            schema: record::RECORD_SCHEMA.into(),
            record_id: String::new(),
            session_id: "SYNTHETIC-CODEC-ONLY".into(),
            run_id: format!("{}{}R{n}", a.method.method_id, w.workload_id),
            seq: e.seq,
            round: e.round,
            warmup: e.warmup,
            arm: a.name.clone(),
            role: serde_json::to_value(a.role)
                .unwrap()
                .as_str()
                .unwrap()
                .into(),
            method: a.method.clone(),
            workload: w.clone(),
            algorithm_seed: e.algorithm_seed.to_string(),
            outcome: record::Outcome {
                status: status.into(),
                recovered: None,
                matches_target: None,
                matches_planted: None,
                error: Some("synthetic codec fixture, no solver invocation".into()),
                exit: None,
                stderr_sha256: None,
            },
            cost: record::Cost {
                unit: record::UNIT.into(),
                total_gae: Some(3934.0137668849675),
                s: None,
                lower_bound: true,
                unpriced: vec!["fixture_only_uncharged".into()],
                deterministic: true,
                nondeterminism: vec![],
            },
            boundaries: record::Boundaries {
                automorphisms_available: 2,
                automorphisms_used: None,
                floor_s: 1.0,
                ratio_to_floor: None,
                floor_rule: None,
            },
            phases: vec![],
            counters: BTreeMap::new(),
            factor_base: None,
            fbm_id: None,
            detail: json!({"scope":"synthetic_codec_only"}),
            online: None,
            solver: None,
            allocator: None,
            field_ops: None,
            time: record::Timing::default(),
            isolation: record::IsolationRecord {
                level: spec::Level::L0,
                blockers: vec!["synthetic codec fixture".into()],
                run_cpu: None,
                reserved: vec![],
                node: None,
                placement: isolation::SelfPlacement::default(),
                foreign_busy_ticks: None,
                steal_ticks: None,
                idle_sibling_busy_ticks: None,
                before: Default::default(),
                after: Default::default(),
            },
            env_class_id: "SYNTHETIC-NO-HOST".into(),
            binary_sha256: Some("a".repeat(64)),
            work: None,
        }
    }
    fn observations(&self, rows: &mut [record::Record]) -> Vec<String> {
        let lines: Vec<_> = rows
            .iter_mut()
            .map(|r| {
                r.record_id.clear();
                r.seal()
            })
            .collect();
        self.raw(&lines, rows);
        lines
    }
    fn raw(&self, lines: &[String], rows: &[record::Record]) {
        let text = if lines.is_empty() {
            String::new()
        } else {
            lines.join("\n") + "\n"
        };
        std::fs::write(self.dir.join("records.jsonl"), &text).unwrap();
        let mut counts = BTreeMap::<String, u64>::new();
        for r in rows {
            *counts.entry(r.outcome.status.clone()).or_default() += 1;
        }
        let host_sha = write(
            &self.dir,
            "host.json",
            &json!({"scope":"synthetic_codec_only"}),
        );
        let session = json!({"schema":"ecbench.session/v1","session_id":"SYNTHETIC-CODEC-ONLY","label":"fixture","spec_id":self.plan.spec_id,"spec_sha256":self.plan.spec_sha256,
            "env_class_id":"SYNTHETIC-NO-HOST","host_sha256":host_sha,"plan_sha256":canonical::sha256_hex(&std::fs::read(self.dir.join("plan.json")).unwrap()),"binary_sha256":"a".repeat(64),
            "git_commit":null,"git_dirty":null,"options":{"cpus":"none","lock_path":"fixture-only","wait_for_lock":false,"allow_busy":false,"thresholds":isolation::Thresholds::default(),"exe":"fixture-no-executable"},
            "cpu_plan":null,"preflight":null,"eviction":null,"child_env_sha256":"b".repeat(64),"child_env":{},"started_unix_ms":0,"finished_unix_ms":0,"status":"interrupted","executions_planned":self.plan.executions.len(),"records_written":rows.len(),"records_sha256":canonical::sha256_hex(text.as_bytes()),"status_counts":counts,"last_seq":rows.iter().map(|r|r.seq).max(),"stop":null});
        write(&self.dir, "session.json", &session);
    }
}

#[test]
fn unexecuted_plan_retains_all_pairs_and_warmups() {
    let f = Fixture::new();
    let v = f.join(true).unwrap();
    assert_eq!(v["pairs"].as_array().unwrap().len(), 14);
    assert_eq!(
        v["pairs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| p["warmup"] == true)
            .count(),
        2
    );
    assert_eq!(v["observations_selected"], 0);
    assert_eq!(v["result_admitted"], false);
    for p in v["pairs"].as_array().unwrap() {
        assert_ne!(p["source"]["workload_id"], p["destination"]["workload_id"]);
        assert_eq!(p["source_status"], "missing");
        assert!(p["source"]["record"].is_null());
    }
    assert!(f.join(false).is_err());
}

#[test]
fn failed_incomplete_observations_are_lossless_and_unadmitted() {
    let f = Fixture::new();
    let mut rows = vec![
        f.record(0, "timeout"),
        f.record(2, "error"),
        f.record(8, "exhausted"),
    ];
    let lines = f.observations(&mut rows);
    let output = endpoint_pairs::join(&f.dir, &f.source, &f.destination, "rho", false).unwrap();
    let serialized = serde_json::to_string(&output).unwrap();
    for line in lines {
        assert!(serialized.contains(&line));
    }
    let v = serde_json::to_value(output).unwrap();
    assert_eq!(v["observations_in_input"], 3);
    assert_eq!(v["observations_selected"], 3);
    assert!(v["cost_ratio"].is_null());
    assert!(v["wall_ratio"].is_null());
    assert!(v["conductor_gap"].is_null());
    assert_eq!(v["result_admitted"], false);
    let statuses: Vec<_> = v["pairs"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|p| {
            [
                p["source_status"].as_str().unwrap(),
                p["destination_status"].as_str().unwrap(),
            ]
        })
        .collect();
    for s in ["timeout", "error", "exhausted", "missing"] {
        assert!(statuses.contains(&s));
    }
    assert!(f.join(true).is_err());
}

#[test]
fn rejects_modified_plans_and_ambiguous_selectors() {
    let f = Fixture::new();
    assert!(endpoint_pairs::join(&f.dir, &f.source, &f.source, "rho", true).is_err());
    assert!(endpoint_pairs::join(&f.dir, &f.source, &f.destination, "unknown", true).is_err());
    assert!(endpoint_pairs::join(&f.dir, &f.source, "unknown", "rho", true).is_err());
    let mut v: Value =
        serde_json::from_slice(&std::fs::read(f.dir.join("plan.json")).unwrap()).unwrap();
    v["executions"][0]["algorithm_seed"] = json!(0);
    write(&f.dir, "plan.json", &v);
    assert!(f.join(true).unwrap_err().contains("plan differs"));
}

#[test]
fn rejects_resealed_identity_tampering_duplicates_and_bad_costs() {
    for field in [
        "seed",
        "session",
        "binary",
        "env",
        "workload",
        "round",
        "warmup",
        "arm",
        "method",
        "run",
        "unit",
        "negative",
        "status",
        "duplicate",
    ] {
        let f = Fixture::new();
        let mut r = f.record(0, "error");
        match field {
            "seed" => r.algorithm_seed = "0".into(),
            "session" => r.session_id = "other".into(),
            "binary" => r.binary_sha256 = Some("c".repeat(64)),
            "env" => r.env_class_id = "other".into(),
            "workload" => r.workload.target_index += 1,
            "round" => r.round += 1,
            "warmup" => r.warmup = !r.warmup,
            "arm" => r.arm = "other".into(),
            "method" => r.method.method_id = "other".into(),
            "run" => r.run_id = "other".into(),
            "unit" => r.cost.unit = "seconds".into(),
            "negative" => r.cost.total_gae = Some(-1.0),
            "status" => r.outcome.status = "unknown".into(),
            _ => {}
        }
        let mut rows = if field == "duplicate" {
            vec![r.clone(), r]
        } else {
            vec![r]
        };
        f.observations(&mut rows);
        assert!(f.join(false).is_err(), "accepted {field}");
    }
}

#[test]
fn rejects_broken_seals_session_digests_and_duplicate_json_keys() {
    let f = Fixture::new();
    let mut rows = vec![f.record(0, "crashed")];
    let mut lines = f.observations(&mut rows);
    lines[0] = lines[0].replace("crashed", "timeout");
    f.raw(&lines, &rows);
    assert!(f.join(false).unwrap_err().contains("seal"));
    f.observations(&mut rows);
    std::fs::write(f.dir.join("host.json"), "{}\n").unwrap();
    assert!(f.join(false).unwrap_err().contains("digest"));
    let f = Fixture::new();
    let text = std::fs::read_to_string(f.dir.join("plan.json")).unwrap();
    std::fs::write(
        f.dir.join("plan.json"),
        text.replacen('{', "{\"schema\":\"duplicate\",", 1),
    )
    .unwrap();
    assert!(f.join(true).is_err());
}

#[test]
fn decimal_lexemes_survive_the_join_without_float_reserialization() {
    let f = Fixture::new();
    let mut rows = vec![f.record(0, "wrong_answer")];
    let mut lines = f.observations(&mut rows);
    let original = serde_json::to_string(&rows[0].cost.total_gae.unwrap()).unwrap();
    let exotic = "3.934013766884967500000e3";
    let blank = lines[0]
        .replace(
            &format!("\"record_id\":\"{}\"", rows[0].record_id),
            "\"record_id\":\"\"",
        )
        .replace(
            &format!("\"total_gae\":{original}"),
            &format!("\"total_gae\":{exotic}"),
        );
    assert!(blank.contains(exotic));
    let id = format!("ECR1h{}", &canonical::sha256_hex(blank.as_bytes())[..12]);
    lines[0] = blank.replace("\"record_id\":\"\"", &format!("\"record_id\":\"{id}\""));
    f.raw(&lines, &rows);
    let joined = endpoint_pairs::join(&f.dir, &f.source, &f.destination, "rho", false).unwrap();
    assert!(serde_json::to_string_pretty(&joined)
        .unwrap()
        .contains(&lines[0]));
}

#[test]
fn rejects_unplanned_records_and_inconsistent_session_counters_or_completion() {
    for field in [
        "unplanned",
        "count",
        "status_counts",
        "last_seq",
        "complete",
        "running",
        "records_hash",
        "spec_hash",
        "plan_hash",
    ] {
        let f = Fixture::new();
        let mut r = f.record(0, "error");
        if field == "unplanned" {
            r.seq = u64::MAX;
        }
        f.observations(&mut [r]);
        let mut s: Value =
            serde_json::from_slice(&std::fs::read(f.dir.join("session.json")).unwrap()).unwrap();
        match field {
            "count" => s["records_written"] = json!(100),
            "status_counts" => s["status_counts"] = json!({"verified":1}),
            "last_seq" => s["last_seq"] = json!(999),
            "complete" => s["status"] = json!("complete"),
            "running" => s["status"] = json!("running"),
            "records_hash" => s["records_sha256"] = json!("0".repeat(64)),
            "spec_hash" => s["spec_sha256"] = json!("0".repeat(64)),
            "plan_hash" => s["plan_sha256"] = json!("0".repeat(64)),
            _ => {}
        }
        write(&f.dir, "session.json", &s);
        assert!(f.join(false).is_err(), "accepted {field}");
    }
}

#[test]
fn complete_synthetic_fixture_retains_verified_and_other_arm_observations() {
    let f = Fixture::new();
    let mut rows: Vec<_> = (0..f.plan.executions.len())
        .map(|seq| f.record(seq, "verified"))
        .collect();
    f.observations(&mut rows);
    let mut s: Value =
        serde_json::from_slice(&std::fs::read(f.dir.join("session.json")).unwrap()).unwrap();
    s["status"] = json!("complete");
    write(&f.dir, "session.json", &s);
    let v = f.join(false).unwrap();
    assert_eq!(v["observations_in_input"], 56);
    assert_eq!(v["observations_selected"], 28);
    assert_eq!(v["result_admitted"], false);
    for p in v["pairs"].as_array().unwrap() {
        for endpoint in ["source", "destination"] {
            assert_eq!(p[endpoint]["record"]["outcome"]["status"], "verified");
            assert_eq!(p[endpoint]["record"]["cost"]["lower_bound"], true);
            assert_eq!(
                p[endpoint]["record"]["cost"]["unpriced"][0],
                "fixture_only_uncharged"
            );
        }
    }
}
