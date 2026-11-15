//! Native packaging of existing observations. This module never executes experiments.
mod disjoint;
mod erratum;
mod output_contract;
mod protocol_lint;
#[cfg(test)]
#[path = "../tests/unit_contracts/mod.rs"]
mod unit_contracts;
use serde_json::{json, Value as V};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, String>;
pub const PROTOCOL_SCHEMA: &str = include_str!("../schemas/experiment-protocol.v1.json");
pub const EXPORT_SCHEMA: &str = include_str!("../schemas/experiment-export.v1.json");
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn digest(p: &Path) -> Result<String> {
    Ok(hash(&fs::read(p).map_err(|e| e.to_string())?))
}
pub fn load(p: &Path) -> Result<V> {
    serde_json::from_slice(&fs::read(p).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn s(v: &V) -> Result<&str> {
    v.as_str().ok_or_else(|| "required string absent".into())
}
fn arr(v: &V) -> Result<&Vec<V>> {
    v.as_array().ok_or_else(|| "required array absent".into())
}
fn n(v: &V) -> Result<u64> {
    v.as_u64()
        .ok_or_else(|| "required nonnegative integer absent or out of range".into())
}
fn check(ok: bool, msg: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
pub fn bound(v: &V) -> Result<PathBuf> {
    let p = fs::canonicalize(s(&v["path"])?).map_err(|e| e.to_string())?;
    check(digest(&p)? == s(&v["sha256"])?, "hash mismatch")?;
    Ok(p)
}
/// Sorted-key compact JSON, matching the native aggregate manifest convention.
pub fn canonical(v: &V) -> Result<Vec<u8>> {
    serde_json::to_vec(v).map_err(|e| e.to_string())
}
pub fn protocol_hash(v: &V) -> Result<String> {
    let mut p = v.clone();
    p.as_object_mut().ok_or("protocol object")?.remove("freeze");
    Ok(hash(&canonical(&p)?))
}
pub fn validate_schema(v: &V, protocol: bool) -> Result<()> {
    let schema: V = serde_json::from_str(if protocol {
        PROTOCOL_SCHEMA
    } else {
        EXPORT_SCHEMA
    })
    .map_err(|e| e.to_string())?;
    let validator = jsonschema::validator_for(&schema).map_err(|e| e.to_string())?;
    let errs: Vec<_> = validator.iter_errors(v).map(|e| e.to_string()).collect();
    check(errs.is_empty(), &errs.join("; "))
}
/// Native adapter identity is its source and schemas, ordered and byte-bound.
pub fn adapter_hash() -> String {
    hash(
        concat!(
            include_str!("lib.rs"),
            include_str!("main.rs"),
            include_str!("protocol_lint.rs"),
            include_str!("output_contract.rs"),
            include_str!("erratum.rs"),
            include_str!("disjoint.rs"),
            include_str!("../Cargo.lock"),
            include_str!("../Cargo.toml"),
            include_str!("../schemas/experiment-protocol.v1.json"),
            include_str!("../schemas/experiment-export.v1.json")
        )
        .as_bytes(),
    )
}
pub fn prepare_reviewed_provenance_erratum(
    parent: &Path,
    source_manifest: &Path,
    binary_manifest: &Path,
    receipt: &Path,
) -> Result<V> {
    erratum::prepare(parent, source_manifest, binary_manifest, receipt)
}
fn source_pins(p: &V, native: &V) -> Result<()> {
    for (field, file) in [
        ("aggregate_source_sha256", "native_price/src/aggregate.rs"),
        ("paired_export_source_sha256", "paired_export/src/lib.rs"),
        (
            "paired_export_plan_source_sha256",
            "paired_export/src/plan.rs",
        ),
    ] {
        check(
            native[field].is_string()
                && native[field] == p["extensions"]["analysis_support_sha256"][file],
            "native analysis source identity mismatch",
        )?;
    }
    check(
        native["mapping"]["mapping_sha256"].is_string()
            && native["mapping"]["mapping_sha256"]
                == p["extensions"]["finite_cost_model"]["mapping_sha256"],
        "native mapping identity mismatch",
    )
}
pub fn online_reasons(r: &V) -> Vec<String> {
    let mut reasons = vec![];
    let wanted: BTreeSet<_> = [
        "target_query",
        "target_PDP",
        "target_relation_check",
        "target_descent",
        "target_recovery_check",
    ]
    .into_iter()
    .collect();
    let phases = r["online"]["phases_ns"].as_object();
    if phases.map(|p| p.keys().map(String::as_str).collect::<BTreeSet<_>>()) != Some(wanted) {
        reasons.push("phase_names_not_exact_five".into());
    }
    let sum = phases.and_then(|p| p.values().try_fold(0u64, |a, v| a.checked_add(v.as_u64()?)));
    if n(&r["online"]["wall_ns"]).is_err()
        || phases.is_none_or(|p| p.values().any(|v| n(v).is_err()))
    {
        reasons.push("invalid_clock_integer".into());
    } else if sum != r["online"]["wall_ns"].as_u64() {
        reasons.push("invalid_phase_sum".into());
    }
    if r["time"]["online_clock_diagnostics"].get("error") != Some(&V::Null) {
        reasons.push("online_diagnostics_absent_or_error".into());
    }
    reasons
}
/// Validate a draft without freezing it; frozen documents additionally require admission checks.
pub fn validate_protocol(p: &V, base: Option<&Path>) -> Result<()> {
    validate_schema(p, true)?;
    protocol_lint::lint(p)?;
    if let Some(base) = base {
        let script = &p["analysis_plan"]["script"];
        if let (Some(path), Some(expected)) = (script["path"].as_str(), script["sha256"].as_str()) {
            let path = base.join(path);
            if path.is_file() {
                check(
                    digest(&path)? == expected,
                    "protocol analysis script hash mismatch",
                )?;
            }
        }
    }
    if p["freeze"]["status"] == "frozen" {
        protocol_admission(p)?;
    } else if disjoint::selected(p) {
        disjoint::admit_protocol(p)?;
    } else {
        for amendment in arr(&p["amendments"])? {
            check(
                amendment["filed_before_affected_data"] == true,
                "amendment after data",
            )?;
        }
    }
    Ok(())
}
fn basic_protocol_admission(p: &V, allow_post_data: bool) -> Result<()> {
    validate_schema(p, true)?;
    protocol_lint::lint(p)?;
    check(
        p["freeze"]["status"] == "frozen",
        "requires valid frozen protocol",
    )?;
    check(
        p["freeze"]["frozen_at"]
            .as_str()
            .is_some_and(|v| !v.is_empty()),
        "frozen protocol timestamp absent",
    )?;
    check(
        s(&p["freeze"]["protocol_sha256"])? == protocol_hash(p)?,
        "frozen protocol digest mismatch",
    )?;
    check(
        (p["analysis_plan"]["frozen_before_data"] == true || disjoint::selected(p))
            && !p["analysis_plan"]["script"]["sha256"].is_null(),
        "analysis not frozen before data",
    )?;
    check(
        !p["sample_size"]["preflight_runtime"]["estimated_total_seconds"].is_null()
            || protocol_lint::unknown_runtime_gate(p)?,
        "frozen protocol preflight absent",
    )?;
    let mut ids = BTreeSet::new();
    for a in arr(&p["arms"])? {
        check(ids.insert(s(&a["id"])?), "duplicate arm")?;
    }
    for am in arr(&p["amendments"])? {
        check(
            allow_post_data || am["filed_before_affected_data"] == true,
            "amendment after data",
        )?;
    }
    Ok(())
}
fn protocol_admission(p: &V) -> Result<Option<erratum::Admitted>> {
    if disjoint::selected(p) {
        basic_protocol_admission(p, true)?;
        disjoint::admit_protocol(p)?;
        return Ok(None);
    }
    basic_protocol_admission(p, true)?;
    let admitted = erratum::admit(p)?;
    if admitted.is_none() {
        basic_protocol_admission(p, false)?;
    }
    Ok(admitted)
}
fn attach(files: &mut BTreeMap<PathBuf, V>, p: &Path, kind: &str) -> Result<V> {
    let p = fs::canonicalize(p).map_err(|e| e.to_string())?;
    let f = json!({"path":p,"sha256":digest(&p)?,"bytes":fs::metadata(&p).map_err(|e|e.to_string())?.len(),"kind":kind});
    files.insert(p, f.clone());
    Ok(f)
}
fn attach_bound(files: &mut BTreeMap<PathBuf, V>, b: &V, kind: &str) -> Result<PathBuf> {
    let p = bound(b)?;
    attach(files, &p, kind)?;
    Ok(p)
}
fn seconds(v: &V) -> Result<V> {
    if v.is_null() {
        return Ok(V::Null);
    }
    let ns = n(v)?;
    let text = format!("{}.{:09}", ns / 1_000_000_000, ns % 1_000_000_000);
    serde_json::from_str(&text).map_err(|e| e.to_string())
}
fn exact_counter(v: &V) -> V {
    if unsigned_integer(v) && v.as_u64().is_none_or(|n| n > 9_007_199_254_740_991) {
        V::String(v.to_string())
    } else {
        v.clone()
    }
}
pub fn flow(runs: &[V]) -> V {
    json!({"planned":runs.len(),"attempted":runs.len(),"completed":runs.iter().filter(|r|r["outcome_class"]=="completed").count(),"not_completed":runs.iter().filter(|r|r["outcome_class"]=="not_completed").count(),"interrupted":runs.iter().filter(|r|r["status"]=="interrupted").count(),"resumed":runs.iter().filter(|r|r["resumed"].as_u64().unwrap_or(0)>0).count(),"excluded":[]})
}
pub fn check_files(e: &V) -> Result<()> {
    for f in arr(&e["files"])? {
        let p = bound(f).map_err(|e| format!("file changed: {e}"))?;
        check(
            fs::metadata(p).map_err(|e| e.to_string())?.len() == n(&f["bytes"])?,
            "file size changed",
        )?;
    }
    Ok(())
}
pub fn package(req: &V) -> Result<(V, Vec<String>)> {
    package_with(req, replay_analysis)
}
fn replay_analysis(native: &V, protocol: &V) -> Result<()> {
    macro_rules! linked_source {
        ($path:literal) => {
            check(
                protocol["extensions"]["analysis_support_sha256"][$path]
                    == hash(include_bytes!(concat!("../../", $path))),
                concat!("linked analyzer source differs from protocol: ", $path),
            )?;
        };
    }
    linked_source!("native_price/Cargo.toml");
    linked_source!("native_price/Cargo.lock");
    linked_source!("native_price/src/lib.rs");
    linked_source!("native_price/src/analyzer.rs");
    linked_source!("native_price/src/aggregate.rs");
    linked_source!("native_price/src/importer.rs");
    linked_source!("native_price/src/bin/analyze_pair.rs");
    linked_source!("native_price/src/bin/analyze_cohort.rs");
    linked_source!("paired_export/Cargo.toml");
    linked_source!("paired_export/Cargo.lock");
    linked_source!("paired_export/src/lib.rs");
    linked_source!("paired_export/src/plan.rs");
    linked_source!("paired_export/src/main.rs");
    if disjoint::selected(protocol) {
        linked_source!("native_price/src/disjoint.rs");
    }
    let manifest = &native["input_manifest"];
    let gate_scope = pencil_accounting_price_draft::aggregate::p739_gate60_scope();
    let rebuilt = match protocol["sample_size"]["instances"].as_u64() {
        Some(1)
            if protocol["freeze"]["status"] == "frozen"
                && protocol["extensions"]["analysis_scope"]
                    == pencil_accounting_price_draft::aggregate::p186379_target0_scope()
                && manifest["analysis_scope"] == protocol["extensions"]["analysis_scope"] =>
        {
            pencil_accounting_price_draft::aggregate::aggregate_p186379_target0(manifest)?
        }
        Some(1260)
            if disjoint::selected(protocol)
                && protocol["freeze"]["status"] == "frozen"
                && manifest["analysis_scope"] == protocol["extensions"]["analysis_scope"] =>
        {
            pencil_accounting_price_draft::aggregate::aggregate_disjoint_p739(manifest)?
        }
        Some(60)
            if protocol["freeze"]["status"] == "frozen"
                && protocol["extensions"]["analysis_scope"] == gate_scope
                && manifest["analysis_scope"] == gate_scope =>
        {
            pencil_accounting_price_draft::aggregate::aggregate_p739_gate60(manifest)?
        }
        Some(1260)
            if protocol["freeze"]["status"] == "frozen"
                && protocol["extensions"]["analysis_scope"].is_null()
                && manifest["analysis_scope"].is_null() =>
        {
            pencil_accounting_price_draft::aggregate::aggregate(manifest)?
        }
        _ => {
            return Err(
                "protocol and manifest must bind full1260 or exact frozen p739 gate60".into(),
            )
        }
    };
    check(
        rebuilt == *native,
        "native analysis differs from deterministic source-linked replay",
    )
}
fn package_with(req: &V, replay: fn(&V, &V) -> Result<()>) -> Result<(V, Vec<String>)> {
    // Core performs schema, semantic, attachment and exact analysis replay checks.
    // The independent check-export entry point additionally reconstructs an export.
    package_core(req, replay)
}
fn package_core(req: &V, replay: fn(&V, &V) -> Result<()>) -> Result<(V, Vec<String>)> {
    check(
        req["schema"] == "pencil-canonical-package-input/v1",
        "unsupported packaging request",
    )?;
    let helper = bound(&req["canonical_tool"])?;
    let pp = bound(&req["protocol"])?;
    let p = load(&pp)?;
    let provenance_erratum = protocol_admission(&p)?;
    let pin = &p["extensions"]["canonical_packaging"];
    let adapter_pin = provenance_erratum
        .as_ref()
        .map(|_| &p["extensions"]["analysis_provenance_erratum"]["adapter_sha256"])
        .unwrap_or(&pin["adapter_sha256"]);
    check(
        *adapter_pin == adapter_hash()
            && pin["canonical_tool_sha256"] == req["canonical_tool"]["sha256"],
        "native adapter/helper not pinned by protocol",
    )?;
    let np = bound(&req["native_analysis"])?;
    let native = load(&np)?;
    check(
        native["schema"] == "pencil-price-cohort-output/v1",
        "unsupported native output",
    )?;
    let m = &native["input_manifest"];
    let data_protocol_sha256 = provenance_erratum
        .as_ref()
        .map(|a| a.parent_protocol_sha256.as_str())
        .unwrap_or(s(&p["freeze"]["protocol_sha256"])?);
    check(
        native["input_manifest_integrity_sha256"] == hash(&canonical(m)?),
        "native manifest integrity mismatch",
    )?;
    source_pins(&p, &native)?;
    check(
        m["schema"] == "pencil-price-cohort-input/v1"
            && m["protocol_sha256"] == data_protocol_sha256,
        "native protocol identity mismatch",
    )?;
    let binary = &m["binary_sha256"];
    for a in arr(&p["arms"])? {
        check(
            &a["instrument"]["binary_sha256"] == binary,
            "instrument binary identity mismatch",
        )?;
    }
    let script = bound(&req["analysis_script"])?;
    check(
        digest(&script)? == s(&p["analysis_plan"]["script"]["sha256"])?,
        "analysis script identity mismatch",
    )?;
    let mut files = BTreeMap::new();
    let request_path = attach_bound(&mut files, &req["_request_binding"], "receipt")?;
    let mut request_body = req.clone();
    request_body
        .as_object_mut()
        .ok_or("request object absent")?
        .remove("_request_binding");
    check(
        load(&request_path)? == request_body,
        "packaging request sidecar differs",
    )?;
    attach(&mut files, &pp, "protocol")?;
    attach(&mut files, &helper, "analysis")?;
    for (name, bytes) in [
        ("src/lib.rs", include_bytes!("lib.rs").as_slice()),
        ("src/main.rs", include_bytes!("main.rs").as_slice()),
        (
            "src/output_contract.rs",
            include_bytes!("output_contract.rs").as_slice(),
        ),
        ("src/erratum.rs", include_bytes!("erratum.rs").as_slice()),
        ("src/disjoint.rs", include_bytes!("disjoint.rs").as_slice()),
        (
            "src/protocol_lint.rs",
            include_bytes!("protocol_lint.rs").as_slice(),
        ),
        ("Cargo.toml", include_bytes!("../Cargo.toml").as_slice()),
        ("Cargo.lock", include_bytes!("../Cargo.lock").as_slice()),
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(name);
        check(
            digest(&path)? == hash(bytes),
            "retained adapter source differs from compiled source",
        )?;
        attach(&mut files, &path, "analysis")?;
    }
    output_contract::provenance(req, &p, &mut files)?;
    if let Some(a) = &provenance_erratum {
        for (path, kind) in &a.attachments {
            attach(&mut files, path, kind)?;
        }
    }
    let schema_dir = helper
        .parent()
        .and_then(Path::parent)
        .ok_or("helper parent absent")?
        .join("schemas");
    for (name, expected) in [
        ("experiment-protocol.v1.json", PROTOCOL_SCHEMA),
        ("experiment-export.v1.json", EXPORT_SCHEMA),
    ] {
        let path = schema_dir.join(name);
        check(
            digest(&path)? == hash(expected.as_bytes()),
            "canonical schema differs from native embedded schema",
        )?;
        attach(&mut files, &path, "other")?;
    }
    for b in req["additional_sidecars"].as_array().into_iter().flatten() {
        attach_bound(&mut files, b, "raw")?;
    }
    let script_entry = attach(&mut files, &script, "analysis")?;
    let native_entry = attach(&mut files, &np, "summary")?;
    let mut receipt = load(&attach_bound(
        &mut files,
        &m["prepared_plan_validation"],
        "summary",
    )?)?;
    let enrollment = if disjoint::selected(&p) {
        let context = disjoint::load_context(&p, m, &mut files)?;
        receipt = pencil_accounting_price_draft::aggregate::normalized_disjoint_receipt(m)?;
        Some(context)
    } else {
        None
    };
    check(
        receipt["schema"] == "pencil-prepared-plan-validation/v1" && receipt["passed"] == true,
        "invalid plan validation receipt",
    )?;
    if let Some(review) = m.get("coverage_review").filter(|r| !r.is_null()) {
        for b in arr(&review["source_files"])? {
            attach_bound(&mut files, b, "other")?;
        }
        if !review["independent_report"].is_null() {
            attach_bound(&mut files, &review["independent_report"], "report")?;
        }
    }
    let resources = m.get("separate_resources").filter(|v| !v.is_null());
    if let Some(b) = resources {
        let rp = attach_bound(&mut files, b, "raw")?;
        check(
            load(&rp)? == native["separate_resource_observations"],
            "resource sidecar differs from native observation",
        )?;
    }
    let history = if let Some(context) = &enrollment {
        check(
            req["execution_history"].is_null(),
            "disjoint history must be per plan",
        )?;
        context.history(&mut files, binary)?
    } else {
        let hp = attach_bound(&mut files, &req["execution_history"], "receipt")?;
        let history = load(&hp)?;
        check(
            history["schema"] == "pencil-execution-history/v1"
                && history["protocol_sha256"] == data_protocol_sha256
                && &history["binary_sha256"] == binary,
            "execution history identity mismatch",
        )?;
        history
    };
    let mut histories = BTreeMap::new();
    for ev in arr(&history["records"])? {
        let key = s(&ev["record_id"])?;
        check(
            histories.insert(key.to_owned(), ev).is_none(),
            "duplicate execution history",
        )?;
        check(
            n(&ev["attempts"]).is_ok_and(|v| v >= 1) && n(&ev["resumed"]).is_ok(),
            "explicit known attempts/resumed counts required",
        )?;
        let evidence = arr(&ev["evidence"])?;
        check(
            !evidence.is_empty(),
            "execution history requires bound evidence",
        )?;
        for b in evidence {
            attach_bound(&mut files, b, "receipt")?;
        }
        let history_protocol = match &enrollment {
            Some(context) => context.record_protocol(&ev["session_id"])?,
            None => &p,
        };
        output_contract::history(ev, history_protocol)?;
        if n(&ev["resumed"])? > 0 {
            check(
                ev["checkpoint"].is_object()
                    && ev["checkpoint"]["policy"].is_string()
                    && ev["checkpoint"]["policy"] != "none"
                    && n(&ev["checkpoint"]["restored_from_generation"]).is_ok(),
                "restored history requires checkpoint generation",
            )?;
        }
    }
    let (mut sessions, mut workloads, mut records_seen, mut plans_seen) = (
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
    );
    let mut expected = 0u64;
    let mut all_slots = vec![];
    let (mut runs, mut missing, mut diagnostics) = (vec![], vec![], vec![]);
    let roles: BTreeSet<_> = arr(&p["arms"])?
        .iter()
        .map(|a| s(&a["id"]))
        .collect::<Result<_>>()?;
    for entry in arr(&m["plans"])? {
        let name = s(&entry["name"])?;
        check(
            plans_seen.insert(name.to_owned()) && receipt["files"][name] == entry["sha256"],
            "plan set/digest differs from receipt",
        )?;
        let plan = load(&attach_bound(&mut files, entry, "protocol")?)?;
        let plan_protocol = match &enrollment {
            Some(context) => context.protocols.get(name).ok_or("lineage plan absent")?,
            None => &p,
        };
        output_contract::plan(&plan, plan_protocol)?;
        check(plan["schema"] == "ecbench.plan/v1", "invalid plan schema")?;
        let sid = &entry["session_id"];
        if enrollment.is_some() && sid.is_null() {
            check(entry["records"].is_null(), "null session with records")?;
        } else {
            check(sessions.insert(s(sid)?.to_owned()), "duplicate session")?;
        }
        for w in arr(&plan["workloads"])? {
            check(
                workloads.insert(s(&w["workload_id"])?.to_owned()),
                "duplicate workload across plans",
            )?;
        }
        let mut records = BTreeMap::new();
        if !entry["records"].is_null() {
            let raw = attach_bound(&mut files, &entry["records"], "raw")?;
            for line in fs::read_to_string(raw)
                .map_err(|e| e.to_string())?
                .lines()
                .filter(|l| !l.trim().is_empty())
            {
                let r: V = serde_json::from_str(line).map_err(|e| e.to_string())?;
                let seq = n(&r["seq"])?;
                check(
                    !records.contains_key(&seq)
                        && records_seen.insert(s(&r["record_id"])?.to_owned()),
                    "duplicate record",
                )?;
                check(
                    r["schema"] == "ecbench.record/v1"
                        && r["session_id"] == *sid
                        && &r["binary_sha256"] == binary,
                    "record identity mismatch",
                )?;
                records.insert(seq, r);
            }
        }
        let mut seqs = BTreeSet::new();
        for ex in arr(&plan["executions"])? {
            expected = expected.checked_add(1).ok_or("execution overflow")?;
            let seq = n(&ex["seq"])?;
            check(seqs.insert(seq), "duplicate planned sequence")?;
            let arm = arr(&plan["arms"])?
                .get(n(&ex["arm"])? as usize)
                .ok_or("invalid arm index")?;
            let w = arr(&plan["workloads"])?
                .get(n(&ex["workload"])? as usize)
                .ok_or("invalid workload index")?;
            let seed = ex["algorithm_seed"]
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| ex["algorithm_seed"].to_string());
            all_slots.push(json!({"plan":name,"session_id":sid,"seq":seq,"arm":arm["name"],"workload_id":w["workload_id"],"round":ex["round"],"warmup":ex["warmup"],"algorithm_seed":seed}));
            let Some(r) = records.remove(&seq) else {
                let mut missing_slot = json!({"session_id":sid,"seq":seq,"arm":arm["name"],"workload_id":w["workload_id"],"round":ex["round"],"warmup":ex["warmup"],"algorithm_seed":seed});
                if enrollment.is_some() {
                    missing_slot["plan"] = json!(name);
                }
                missing.push(missing_slot);
                continue;
            };
            let rseed = r["algorithm_seed"]
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| r["algorithm_seed"].to_string());
            check(
                r["arm"] == arm["name"]
                    && roles.contains(s(&r["arm"])?)
                    && r["workload"] == *w
                    && r["method"] == arm["method"]
                    && r["round"] == ex["round"]
                    && r["warmup"] == ex["warmup"]
                    && rseed == seed,
                "record differs from exact plan identity",
            )?;
            let ev = histories
                .remove(s(&r["record_id"])?)
                .ok_or("missing or mismatched explicit execution history")?;
            check(
                ev["session_id"] == *sid && ev["seq"] == ex["seq"],
                "missing or mismatched explicit execution history",
            )?;
            let outcome = &r["outcome"];
            let verified = outcome["status"] == "verified"
                && outcome["matches_target"] == true
                && outcome["matches_planted"] == true
                && outcome.get("error") == Some(&V::Null);
            let status = match outcome["status"].as_str() {
                Some("verified") if verified => "succeeded",
                Some("timeout") => "timeout",
                Some("interrupted") => "interrupted",
                Some("cancelled") => "cancelled",
                Some("infra_error") => "infra_error",
                _ => "failed",
            };
            let t = &r["time"];
            let mut counters = serde_json::Map::new();
            for (k, v) in t.as_object().into_iter().flatten() {
                if unsigned_integer(v) {
                    counters.insert(format!("time.{k}"), exact_counter(v));
                }
            }
            if n(&r["online"]["wall_ns"]).is_ok() {
                counters.insert(
                    "online.wall_ns".into(),
                    exact_counter(&r["online"]["wall_ns"]),
                );
            }
            for (k, v) in r["online"]["phases_ns"].as_object().into_iter().flatten() {
                if unsigned_integer(v) {
                    counters.insert(format!("online.phases_ns.{k}"), exact_counter(v));
                }
            }
            for (k, v) in r["counters"]
                .as_object()
                .ok_or("native counters object required")?
            {
                check(
                    unsigned_integer(v) || v.as_str().is_some_and(output_contract::decimal),
                    "native counter must be a nonnegative integer",
                )?;
                counters.insert(format!("native.{k}"), exact_counter(v));
            }
            let rss = if t["max_rss_kib"].is_null() {
                V::Null
            } else {
                json!(n(&t["max_rss_kib"])?
                    .checked_mul(1024)
                    .ok_or("RSS overflow")?)
            };
            diagnostics.push(json!({"record_id":r["record_id"],"online_structure_reasons":online_reasons(&r),"qualification":"not_established_by_packaging","raw_outcome":outcome,"attempt_and_restore_history":"explicit_hash_bound_history_not_independently_reviewed"}));
            runs.push(json!({"run_id":r["record_id"],"arm":r["arm"],"instance_id":w["workload_id"],"seed":rseed,"replicate":ex["round"],"warmup":ex["warmup"],"status":status,"outcome_class":if ["succeeded","failed"].contains(&status){"completed"}else{"not_completed"},"attempts":ev["attempts"],"resumed":ev["resumed"],"checkpoint":ev["checkpoint"],"measurements":{"counted":{},"wall_seconds":seconds(&t["method_call_wall_ns"])?,"wall_valid":false,"cpu_user_seconds":seconds(&t["user_ns"])?,"cpu_sys_seconds":seconds(&t["sys_ns"])?,"max_rss_bytes":rss,"counters":counters},"isolation":{"tier":r["isolation"]["level"],"contended":null,"blockers":["independent clock qualification not established by packaging"]},"verification":{"status":if verified{"verified"}else{"no_claim"},"verifier":"raw ecbench outcome; independent review not inferred"},"exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":null,"backend_ref":r["record_id"]}));
        }
        check(records.is_empty(), "record not in plan")?;
    }
    check(
        histories.is_empty(),
        "execution history references absent records",
    )?;
    let receipt_names: BTreeSet<_> = receipt["files"]
        .as_object()
        .ok_or("receipt files")?
        .keys()
        .cloned()
        .collect();
    check(plans_seen == receipt_names, "incomplete plan set")?;
    check(
        expected == n(&native["expected_executions"])?
            && runs.len() as u64 == n(&native["observed_executions"])?
            && missing.len() == arr(&native["missing_executions"])?.len(),
        "native flow differs from actual records/plans",
    )?;
    output_contract::analysis(&native, &p, &all_slots, &runs, &diagnostics, &receipt)?;
    replay(&native, &p)?;
    let commit = &p["arms"][0]["instrument"]["commit"];
    let arms: Vec<_> = arr(&p["arms"])?
        .iter()
        .map(|a| json!({"id":a["id"],"role":a["role"],"binary_sha256":binary,"commit":commit}))
        .collect();
    let post_data = provenance_erratum.as_ref().map(|a| json!({"applied":true,"filed_before_affected_data":false,"amendment_id":a.amendment_id,"parent_protocol_sha256":a.parent_protocol_sha256,"data_protocol_sha256":a.parent_protocol_sha256,"metadata_protocol_sha256":p["freeze"]["protocol_sha256"],"changed_fields":erratum::FIELDS}));
    let mut e = json!({"schema":"experiment.export/v1","export_id":req["export_id"],"created":chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs,true),"protocol":{"path":pp,"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"native-rust","model":null,"session":null,"role":"executor"},"source":{"repo":p["repository"],"commit":commit,"dirty":null},"backend":"ecbench","host":{"path":null,"sha256":null,"class_id":null,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":null}},"arms":arms,"flow":flow(&runs),"runs":runs,"files":files.into_values().collect::<Vec<_>>(),"analysis":{"script":script_entry,"blinded":false,"blinding_key_sha256":null,"outputs":[native_entry],"unmasked_at":null},"notes":[],"reviews_dir":"reviews","backend_record":{"request_binding":req["_request_binding"],"schema":"pencil-canonical-packaging/v1","qualified":false,"expected_execution_slots":expected,"missing_slots":missing,"diagnostics":diagnostics,"resource_observations_present":resources.is_some(),"flow_scope":"Observed records with explicit known attempt/restore counts; missing slots are not fabricated attempts; separate infrastructure attempts remain in resource sidecars.","native_analysis_sha256":digest(&np)?,"post_data_provenance_erratum":post_data}});
    if let Some(context) = &enrollment {
        e["backend_record"]["disjoint_protocol_lineage"] = context.lineage.clone();
    }
    validate_schema(&e, false)?;
    check_files(&e)?;
    let findings = lint_rows(&e, &p)?;
    Ok((e, findings))
}
pub fn lint_rows(e: &V, p: &V) -> Result<Vec<String>> {
    let mut findings = vec![];
    let runs = arr(&e["runs"])?;
    let mut ids = BTreeSet::new();
    let mut cells = BTreeSet::new();
    for r in runs {
        check(ids.insert(s(&r["run_id"])?), "duplicate run id")?;
        check(
            cells.insert((
                s(&r["arm"])?,
                s(&r["instance_id"])?,
                n(&r["replicate"])?,
                r["warmup"].as_bool().ok_or("warmup boolean")?,
            )),
            "duplicate execution cell",
        )?;
    }
    let measured: Vec<_> = runs.iter().filter(|r| r["warmup"] == false).collect();
    let instances = n(&p["sample_size"]["instances"])?;
    let reps = n(&p["sample_size"]["replicates"])?;
    let expected = (arr(&p["arms"])?.len() as u64)
        .checked_mul(instances)
        .and_then(|v| v.checked_mul(reps))
        .ok_or("sample count overflow")?;
    let warmups = n(&p["sample_size"]["warmups"])?;
    let observed_warmups = runs.iter().filter(|r| r["warmup"] == true).count() as u64;
    let expected_warmups = (arr(&p["arms"])?.len() as u64)
        .checked_mul(instances)
        .and_then(|n| n.checked_mul(warmups))
        .ok_or("warmup count overflow")?;
    if observed_warmups != expected_warmups {
        findings.push(format!(
            "export has {observed_warmups} warmup rows; protocol plans {expected_warmups}"
        ));
    }
    if measured.len() as u64 != expected {
        findings.push(format!(
            "export has {} measured rows; protocol plans {expected}",
            measured.len()
        ));
    }
    for arm in arr(&p["arms"])? {
        let rows: Vec<_> = measured.iter().filter(|r| r["arm"] == arm["id"]).collect();
        let ws: BTreeSet<_> = rows.iter().map(|r| r["instance_id"].as_str()).collect();
        if ws.len() as u64 != instances {
            findings.push(format!("arm {} instance shortfall", arm["id"]));
        }
        for w in ws {
            if rows
                .iter()
                .filter(|r| r["instance_id"].as_str() == w)
                .count() as u64
                != reps
            {
                findings.push(format!("arm {} replicate shortfall", arm["id"]));
            }
            if runs
                .iter()
                .filter(|r| {
                    r["arm"] == arm["id"] && r["instance_id"].as_str() == w && r["warmup"] == true
                })
                .count() as u64
                != warmups
            {
                findings.push(format!("arm {} workload warmup shortfall", arm["id"]));
            }
        }
    }
    Ok(findings)
}
pub fn write_exclusive(path: &Path, v: &V) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(&bytes)
        .and_then(|_| f.write_all(b"\n"))
        .map_err(|e| e.to_string())
}
/// Validate canonical export semantics and all attached files in native Rust.
pub fn check_export(e: &V, p: &V) -> Result<Vec<String>> {
    check_export_with(e, p, replay_analysis)
}
fn check_export_with(e: &V, p: &V, replay: fn(&V, &V) -> Result<()>) -> Result<Vec<String>> {
    validate_schema(e, false)?;
    protocol_admission(p)?;
    check(
        e["protocol"]["sha256"] == p["freeze"]["protocol_sha256"]
            && e["protocol"]["protocol_id"] == p["protocol_id"]
            && e["protocol"]["version"] == p["version"],
        "export protocol binding mismatch",
    )?;
    let mut roles = BTreeMap::new();
    for a in arr(&e["arms"])? {
        check(
            roles.insert(s(&a["id"])?, a).is_none(),
            "duplicate exported arm",
        )?;
        let pa = arr(&p["arms"])?
            .iter()
            .find(|pa| pa["id"] == a["id"])
            .ok_or("unknown exported arm")?;
        check(a["role"] == pa["role"], "export arm role mismatch")?;
        for key in ["binary_sha256", "commit"] {
            if !pa["instrument"][key].is_null() {
                check(
                    a[key] == pa["instrument"][key],
                    "export instrument mismatch",
                )?;
            }
        }
    }
    let runs = arr(&e["runs"])?;
    check(e["flow"] == flow(runs), "flow differs from observed rows")?;
    for r in runs {
        check(roles.contains_key(s(&r["arm"])?), "unknown run arm")?;
        let completed = matches!(r["status"].as_str(), Some("succeeded" | "failed"));
        check(
            r["outcome_class"]
                == if completed {
                    "completed"
                } else {
                    "not_completed"
                },
            "outcome class mismatch",
        )?;
        check(
            r["measurements"]["wall_valid"] == false,
            "packaging never qualifies wall clocks",
        )?;
        check(
            r["exclusion"]["excluded"] == false && r["verification"]["status"] != "refuted",
            "unsupported excluded/refuted row; retain as raw observation",
        )?;
        if n(&r["resumed"])? > 0 {
            check(
                r["checkpoint"].is_object()
                    && r["checkpoint"]["policy"] != "none"
                    && n(&r["checkpoint"]["restored_from_generation"]).is_ok(),
                "restored history requires checkpoint generation",
            )?;
        }
        for a in arr(&r["artifacts"])? {
            bound(a)?;
        }
    }
    check(
        e["analysis"]["script"]["sha256"] == p["analysis_plan"]["script"]["sha256"],
        "analysis script differs from protocol",
    )?;
    check(
        e["analysis"]["blinded"] == false,
        "unsupported blinded export",
    )?;
    for a in arr(&e["analysis"]["outputs"])? {
        bound(a)?;
    }
    bound(&e["analysis"]["script"])?;
    if e["host"]["path"].is_string() && e["host"]["sha256"].is_string() {
        bound(&e["host"])?;
    }
    check_files(e)?;
    let request_binding = &e["backend_record"]["request_binding"];
    let request_path = bound(request_binding)?;
    check(
        arr(&e["files"])?.iter().any(|f| {
            f["path"] == request_binding["path"] && f["sha256"] == request_binding["sha256"]
        }),
        "request sidecar not attached",
    )?;
    let mut request = load(&request_path)?;
    request["_request_binding"] = request_binding.clone();
    let (mut rebuilt, _) = package_core(&request, replay)?;
    rebuilt["created"] = e["created"].clone();
    check(
        rebuilt == *e,
        "canonical export differs from bound source reconstruction",
    )?;
    let mut findings = lint_rows(e, p)?;
    let mut sets = vec![];
    for a in arr(&p["arms"])? {
        if !runs.iter().any(|r| r["arm"] == a["id"]) {
            findings.push(format!("protocol arm {} has no run", a["id"]));
        }
        sets.push(
            runs.iter()
                .filter(|r| r["arm"] == a["id"] && r["warmup"] == false)
                .map(|r| r["instance_id"].as_str())
                .collect::<BTreeSet<_>>(),
        );
    }
    if sets.windows(2).any(|w| w[0] != w[1]) {
        findings.push("measured instance identities differ across arms".into());
    }
    Ok(findings)
}
fn unsigned_integer(v: &V) -> bool {
    v.is_number() && v.to_string().bytes().all(|b| b.is_ascii_digit())
}
