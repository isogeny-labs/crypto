use crate::{
    array, audit, canonical, hash, integer, protocol_hash, require, text, validate_schema,
    EvidenceRoot, Result,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const MAX_ROWS: usize = 16_384;
fn status(s: &str) -> Result<&'static str> {
    match s {
        "verified" => Ok("succeeded"),
        "exhausted" | "wrong_answer" | "error" | "crashed" => Ok("failed"),
        "timeout" => Ok("timeout"),
        _ => Err(format!("unsupported record outcome {s}")),
    }
}
fn role(s: &str) -> Result<&'static str> {
    match s {
        "reference" => Ok("reference"),
        "baseline" => Ok("baseline"),
        "candidate" => Ok("candidate"),
        "control" | "control_aa" => Ok("control_aa"),
        "control_positive" => Ok("control_positive"),
        "control_negative" => Ok("control_negative"),
        _ => Err("unsupported arm role".into()),
    }
}
fn counter(n: &Value) -> Result<Value> {
    let token = if let Some(s) = n.as_str() {
        s.to_owned()
    } else {
        n.to_string()
    };
    require(
        !token.is_empty() && token.bytes().all(|b| b.is_ascii_digit()),
        "counter must be an exact nonnegative integer",
    )?;
    let token = token.trim_start_matches('0');
    let token = if token.is_empty() { "0" } else { token };
    Ok(match token.parse::<u64>() {
        Ok(n) if n <= 9_007_199_254_740_991 => json!(n),
        _ => json!(token),
    })
}
fn seconds(v: &Value) -> Result<Value> {
    if v.is_null() {
        return Ok(Value::Null);
    }
    Ok(json!(integer(v)? as f64 / 1e9))
}

pub fn flow(rows: &[Value]) -> Value {
    let mut excluded: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for r in rows {
        if r["exclusion"]["excluded"] == true {
            excluded
                .entry(
                    r["exclusion"]["rule"]
                        .as_str()
                        .unwrap_or("unspecified")
                        .into(),
                )
                .or_default()
                .push(r["run_id"].clone());
        }
    }
    json!({"planned":rows.len(),"attempted":rows.len(),"completed":rows.iter().filter(|r|r["outcome_class"]=="completed").count(),"not_completed":rows.iter().filter(|r|r["outcome_class"]=="not_completed").count(),"interrupted":rows.iter().filter(|r|r["status"]=="interrupted").count(),"resumed":rows.iter().filter(|r|integer(&r["resumed"]).is_ok_and(|v|v>0)).count(),"excluded":excluded.into_iter().map(|(rule,run_ids)|json!({"rule":rule,"run_ids":run_ids})).collect::<Vec<_>>()})
}

/// Canonical schemas define flow.planned as the number of represented rows.
/// The full frozen plan population and every missing identity are retained
/// separately. Missing slots never become fabricated attempts or exclusions.
pub fn build(root: &EvidenceRoot, request_path: &str) -> Result<Value> {
    let req = root.json(request_path)?;
    require(
        req["schema"] == "aps.export-request/v1",
        "unsupported APS export request",
    )?;
    let protocol_path = text(&req["protocol"])?;
    let plan_path = text(&req["plan"])?;
    let records_path = text(&req["records"])?;
    let history_path = text(&req["history"])?;
    let protocol = root.json(protocol_path)?;
    let plan = root.json(plan_path)?;
    let history = root.json(history_path)?;
    validate_schema(&protocol, true)?;
    require(
        protocol["freeze"]["status"] == "frozen"
            && protocol["freeze"]["filed_before_affected_data"] == true,
        "export requires a protocol filed before affected data",
    )?;
    require(
        protocol["analysis_plan"]["blinded"] == false,
        "blinded packaging is unsupported",
    )?;
    require(
        protocol["freeze"]["protocol_sha256"] == protocol_hash(&protocol)?,
        "frozen protocol digest mismatch",
    )?;
    require(
        plan["schema"] == "ecbench.plan/v1",
        "unsupported ecbench plan",
    )?;
    require(
        history["schema"] == "aps.execution-history/v1",
        "explicit execution history is required",
    )?;
    let plan_pin = root.pin(plan_path, "raw")?;
    require(
        protocol["extensions"]["aps_export"]["plan_sha256"] == plan_pin["sha256"],
        "plan is not pinned by the frozen protocol",
    )?;
    require(
        req["source"]["commit"] == protocol["freeze"]["commit"],
        "source differs from frozen protocol",
    )?;
    require(
        req["source"]["repo"] == protocol["repository"],
        "repository differs from protocol",
    )?;
    let pa = array(&plan["arms"])?;
    let workloads = array(&plan["workloads"])?;
    let executions = array(&plan["executions"])?;
    require(
        !executions.is_empty() && executions.len() <= MAX_ROWS,
        "unsupported execution population",
    )?;
    let mut arms = Vec::new();
    let mut arm_names = BTreeSet::new();
    for a in pa {
        let name = text(&a["name"])?;
        require(arm_names.insert(name), "duplicate planned arm")?;
        let p = array(&protocol["arms"])?
            .iter()
            .find(|p| p["id"] == name)
            .ok_or("planned arm missing from protocol")?;
        require(
            if a["role"] == "control" {
                matches!(
                    p["role"].as_str(),
                    Some("control_aa" | "control_positive" | "control_negative")
                )
            } else {
                p["role"] == role(text(&a["role"])?)?
            },
            "protocol/plan role mismatch",
        )?;
        arms.push(json!({"id":name,"role":p["role"],"binary_sha256":p["instrument"]["binary_sha256"],"commit":p["instrument"]["commit"]}));
    }
    require(
        arms.len() == array(&protocol["arms"])?.len(),
        "protocol arms missing from plan",
    )?;
    let mut workload_ids = BTreeSet::new();
    for w in workloads {
        require(
            workload_ids.insert(text(&w["workload_id"])?),
            "duplicate planned workload identity",
        )?;
    }
    let mut slots = BTreeMap::new();
    let mut cells = BTreeSet::new();
    for ex in executions {
        let seq = integer(&ex["seq"])?;
        require(
            slots.insert(seq, ex).is_none(),
            "duplicate planned sequence",
        )?;
        require(
            integer(&ex["arm"])? < pa.len() as u64
                && integer(&ex["workload"])? < workloads.len() as u64,
            "planned index outside population",
        )?;
        require(ex["warmup"].is_boolean(), "missing planned warmup status")?;
        require(
            cells.insert((
                integer(&ex["arm"])?,
                integer(&ex["workload"])?,
                integer(&ex["round"])?,
                ex["warmup"].as_bool().unwrap(),
            )),
            "duplicate planned execution cell",
        )?;
        integer(&ex["algorithm_seed"])?;
    }
    let mut events = BTreeMap::new();
    for ev in array(&history["events"])? {
        let seq = integer(&ev["seq"])?;
        require(
            slots.contains_key(&seq) && events.insert(seq, ev).is_none(),
            "unknown or duplicate history sequence",
        )?;
        require(
            integer(&ev["attempts"])? >= 1,
            "observed execution has no attempt",
        )?;
        let resumed = integer(&ev["resumed"])?;
        if resumed > 0 {
            require(
                matches!(
                    ev["checkpoint"]["policy"].as_str(),
                    Some("application" | "criu")
                ) && !ev["checkpoint"]["restored_from_generation"].is_null(),
                "restore lacks explicit checkpoint generation",
            )?;
        } else {
            require(
                ev["checkpoint"]["restored_from_generation"].is_null(),
                "checkpoint restore is present with zero resumes",
            )?;
        }
    }
    let bytes = root.bytes(records_path)?;
    let raw = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
    let mut records = BTreeMap::new();
    let mut record_ids = BTreeSet::new();
    for line in raw.lines() {
        require(!line.is_empty(), "blank record line")?;
        let r = audit::record_line(line)?;
        let seq = integer(&r["seq"])?;
        require(
            slots.contains_key(&seq) && events.contains_key(&seq),
            "record lacks a planned slot or explicit history",
        )?;
        require(
            record_ids.insert(text(&r["record_id"])?.to_owned())
                && records.insert(seq, r).is_none(),
            "duplicate record identity/sequence",
        )?;
        require(records.len() <= MAX_ROWS, "record population exceeds bound")?;
    }
    let records_pin = root.pin(records_path, "raw")?;
    let mut rows = Vec::new();
    let mut audits = Vec::new();
    let mut missing = Vec::new();
    for (&seq, ex) in &slots {
        let a = &pa[integer(&ex["arm"])? as usize];
        let w = &workloads[integer(&ex["workload"])? as usize];
        let Some(ev) = events.get(&seq) else {
            missing.push(json!({"seq":seq,"arm":a["name"],"instance_id":w["workload_id"],"replicate":ex["round"],"warmup":ex["warmup"]}));
            continue;
        };
        let mut counters = serde_json::Map::new();
        let mut wall = Value::Null;
        let mut user = Value::Null;
        let mut sys = Value::Null;
        let mut rss = Value::Null;
        let mut tier = Value::Null;
        let mut verification = "no_claim";
        let mut verifier = Value::Null;
        let mut backend = Value::Null;
        let (run_id, outcome) = if let Some(r) = records.get(&seq) {
            require(
                r["arm"] == a["name"]
                    && r["role"] == a["role"]
                    && r["method"] == a["method"]
                    && r["workload"] == *w
                    && integer(&r["algorithm_seed"])? == integer(&ex["algorithm_seed"])?
                    && r["round"] == ex["round"]
                    && r["warmup"] == ex["warmup"],
                "record differs from its frozen planned slot",
            )?;
            let instrument = arms.iter().find(|arm| arm["id"] == a["name"]).unwrap();
            if !instrument["binary_sha256"].is_null() {
                require(
                    r["binary_sha256"] == instrument["binary_sha256"],
                    "record binary differs from frozen arm",
                )?;
            }
            let outcome = status(text(&r["outcome"]["status"])?)?;
            if !ev["status"].is_null() {
                require(ev["status"] == outcome, "history/record status mismatch")?;
            }
            if r["method"]["id"] == "ic.aps_v1" {
                if r["detail"]["schema"] == "ecbench.aps/v1" {
                    let metric = audit::metrics(r)?;
                    let math = audit::mathematical(r, audit::MAX_ORACLE_CANDIDATES)?;
                    if outcome == "succeeded" {
                        require(
                            integer(&math["in_budget_witnesses_checked"])? > 0
                                && r["detail"]["outcome"] == "verified",
                            "successful APS record lacks a checked in-budget witness",
                        )?;
                        verification = "verified";
                        verifier=json!("aps-export ordinary-residue witness check; not signed PCC or execution attestation");
                    }
                    audits.push(json!({"seq":seq,"metrics":metric,"mathematical":math}));
                } else {
                    require(
                        outcome != "succeeded",
                        "successful APS record has no attempt transcript",
                    )?;
                    audits.push(
                        json!({"seq":seq,"pre_transcript_failure":true,"execution_attested":false}),
                    );
                }
            }
            let t = &r["time"];
            wall = seconds(&t["method_call_wall_ns"])?;
            user = seconds(&t["user_ns"])?;
            sys = seconds(&t["sys_ns"])?;
            rss = if t["max_rss_kib"].is_null() {
                Value::Null
            } else {
                json!(integer(&t["max_rss_kib"])?
                    .checked_mul(1024)
                    .ok_or("RSS conversion overflow")?)
            };
            tier = r["isolation"]["level"].clone();
            backend = r["record_id"].clone();
            for (k, v) in r["counters"].as_object().ok_or("missing native counters")? {
                counters.insert(k.clone(), counter(v)?);
            }
            for phase in array(&r["phases"])? {
                for (k, v) in phase["native"]
                    .as_object()
                    .ok_or("missing phase native counters")?
                {
                    let name = format!("phase.{}.{}", text(&phase["name"])?, k);
                    require(
                        counters.insert(name, counter(v)?).is_none(),
                        "counter identity collision",
                    )?;
                }
            }
            (text(&r["record_id"])?.to_owned(), outcome.to_owned())
        } else {
            let s = text(&ev["status"])?;
            require(
                matches!(
                    s,
                    "failed" | "timeout" | "cancelled" | "infra_error" | "interrupted"
                ),
                "recordless execution cannot claim success",
            )?;
            (
                format!("{}-seq-{seq}", text(&req["export_id"])?),
                s.to_owned(),
            )
        };
        rows.push(json!({"run_id":run_id,"arm":a["name"],"instance_id":w["workload_id"],"seed":ex["algorithm_seed"],"replicate":ex["round"],"warmup":ex["warmup"],"status":outcome,"outcome_class":if matches!(outcome.as_str(),"succeeded"|"failed"){"completed"}else{"not_completed"},"attempts":ev["attempts"],"resumed":ev["resumed"],"checkpoint":ev["checkpoint"],"measurements":{"counted":{},"wall_seconds":wall,"wall_valid":false,"cpu_user_seconds":user,"cpu_sys_seconds":sys,"max_rss_bytes":rss,"counters":counters},"isolation":{"tier":tier,"contended":null,"blockers":["packaging does not independently qualify wall clocks"]},"verification":{"status":verification,"verifier":verifier},"exclusion":{"excluded":false,"rule":null},"artifacts":if records.contains_key(&seq){vec![records_pin.clone()]}else{vec![]},"started_at":ev["started_at"],"finished_at":ev["finished_at"],"worker":null,"backend_ref":backend}));
    }
    let mut files = Vec::new();
    for (name, kind) in [
        (request_path, "receipt"),
        (protocol_path, "protocol"),
        (plan_path, "raw"),
        (records_path, "raw"),
        (history_path, "receipt"),
    ] {
        files.push(root.pin(name, kind)?);
    }
    let script = &protocol["analysis_plan"]["script"];
    let script_pin = if script["path"].is_string() {
        let pin = root.pin(text(&script["path"])?, "analysis")?;
        require(
            pin["sha256"] == script["sha256"],
            "analysis script changed after freeze",
        )?;
        files.push(pin.clone());
        pin
    } else {
        Value::Null
    };
    let export = json!({"schema":"experiment.export/v1","export_id":req["export_id"],"created":req["created"],"protocol":{"path":protocol_path,"sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},"producer":req["producer"],"source":req["source"],"backend":"ecbench","host":{"path":null,"sha256":null,"class_id":null,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":null}},"arms":arms,"flow":flow(&rows),"runs":rows,"files":files,"analysis":{"script":script_pin,"blinded":false,"blinding_key_sha256":null,"outputs":[],"unmasked_at":null},"notes":[],"reviews_dir":"reviews","backend_record":{"schema":"aps.export-binding/v1","request_path":request_path,"plan_sha256":plan_pin["sha256"],"planned_execution_slots":executions.len(),"missing_execution_slots":missing,"audits":audits,"wall_qualified":false,"native_work_accounting_complete":false,"costs_attested":false,"execution_attested":false,"protocol_design_validated":false,"flow_scope":"Canonical flow counts represented rows; the full frozen population and every unobserved slot are retained separately. No attempt is fabricated."}});
    validate_schema(&export, false)?;
    Ok(export)
}

pub fn check(root: &EvidenceRoot, e: &Value) -> Result<()> {
    validate_schema(e, false)?;
    let request = text(&e["backend_record"]["request_path"])?;
    for pin in array(&e["files"])? {
        let actual = root.pin(text(&pin["path"])?, text(&pin["kind"])?)?;
        require(actual == *pin, "attached evidence digest/size mismatch")?;
    }
    let rebuilt = build(root, request)?;
    require(
        canonical(&rebuilt)? == canonical(e)?,
        "export differs from independent source reconstruction",
    )?;
    require(
        array(&e["backend_record"]["missing_execution_slots"])?.is_empty(),
        "export retains unobserved planned slots; complete-population audit is pending",
    )
}

pub fn digest(v: &Value) -> Result<String> {
    Ok(hash(&canonical(v)?))
}
