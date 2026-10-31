//! Complete-plan aggregation of public synthetic mathematics records. File hashes
//! establish byte integrity, not authorization, authenticity, or a scientific review.
use crate::{analyzer, disjoint, grid, Fraction};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
type WorkloadRounds<'a> = BTreeMap<usize, BTreeMap<(bool, u64), BTreeMap<String, &'a Value>>>;
fn s<'a>(v: &'a Value, p: &str) -> Result<&'a str, String> {
    v.pointer(p)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {p}"))
}
fn n(v: &Value, p: &str) -> Result<u64, String> {
    v.pointer(p)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing unsigned integer {p}"))
}
fn a<'a>(v: &'a Value, p: &str) -> Result<&'a Vec<Value>, String> {
    v.pointer(p)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array {p}"))
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read_bound(v: &Value) -> Result<Vec<u8>, String> {
    let b = fs::read(s(v, "/path")?).map_err(|e| e.to_string())?;
    if sha(&b) != s(v, "/sha256")? {
        return Err("input digest mismatch".into());
    }
    Ok(b)
}
fn f(v: &Value) -> Result<Option<Fraction>, String> {
    if v.is_null() {
        return Ok(None);
    }
    let x = s(v, "/numerator")?
        .parse()
        .map_err(|_| "invalid fraction numerator")?;
    let y = s(v, "/denominator")?
        .parse()
        .map_err(|_| "invalid fraction denominator")?;
    Ok(Some(Fraction::new(x, y).map_err(|e| format!("{e:?}"))?))
}
fn fj(v: Fraction) -> Value {
    json!({"numerator":v.numerator().to_string(),"denominator":v.denominator().to_string()})
}
#[derive(Clone)]
struct CellSum {
    count: u64,
    c: Fraction,
    r: Fraction,
    cc: Fraction,
    rr: Fraction,
}
struct Sums {
    expected: u64,
    cells: Vec<CellSum>,
}
impl Sums {
    fn new(expected: u64) -> Self {
        let z = Fraction::new(0, 1).unwrap();
        Self {
            expected,
            cells: vec![
                CellSum {
                    count: 0,
                    c: z,
                    r: z,
                    cc: z,
                    rr: z
                };
                36
            ],
        }
    }
    fn push(&mut self, rows: &Value) -> Result<(), String> {
        let rows = rows.as_array().ok_or("pair cells missing")?;
        if rows.len() != 36 {
            return Err("pair grid cardinality mismatch".into());
        }
        for ((out, row), cell) in self.cells.iter_mut().zip(rows).zip(grid()) {
            if row["square_twice"] != json!(cell.square_twice)
                || row["inversion"] != json!(cell.inversion)
                || row["native_tenths"] != json!(cell.native_tenths)
            {
                return Err("pair grid identity mismatch".into());
            }
            match (
                f(&row["candidate"])?,
                f(&row["reference"])?,
                f(&row["candidate_including_calibration"])?,
                f(&row["reference_including_calibration"])?,
            ) {
                (Some(c), Some(r), Some(cc), Some(rr)) => {
                    out.count += 1;
                    out.c = out.c.plus(c).map_err(|e| format!("{e:?}"))?;
                    out.r = out.r.plus(r).map_err(|e| format!("{e:?}"))?;
                    out.cc = out.cc.plus(cc).map_err(|e| format!("{e:?}"))?;
                    out.rr = out.rr.plus(rr).map_err(|e| format!("{e:?}"))?;
                }
                (None, None, None, None) => {}
                _ => return Err("partial pair pricing rejected".into()),
            }
        }
        Ok(())
    }
    fn output(&self) -> Result<Value, String> {
        let mut rows = vec![];
        for (sum, cell) in self.cells.iter().zip(grid()) {
            if sum.count > self.expected {
                return Err("aggregate denominator exceeded".into());
            }
            let complete = sum.count == self.expected && self.expected > 0;
            rows.push(json!({"square_twice":cell.square_twice,"inversion":cell.inversion,"native_tenths":cell.native_tenths,"expected_pairs":self.expected,"admitted_pairs":sum.count,"missing_or_ineligible_pairs":self.expected-sum.count,"complete":complete,"candidate_sum":if complete{fj(sum.c)}else{Value::Null},"reference_sum":if complete{fj(sum.r)}else{Value::Null},"algorithm_ratio_of_sums":if complete{fj(sum.c.divided_by(sum.r).map_err(|e|format!("{e:?}"))?)}else{Value::Null},"candidate_sum_including_calibration":if complete{fj(sum.cc)}else{Value::Null},"reference_sum_including_calibration":if complete{fj(sum.rr)}else{Value::Null},"ratio_of_sums_including_calibration":if complete{fj(sum.cc.divided_by(sum.rr).map_err(|e|format!("{e:?}"))?)}else{Value::Null},"partial_admitted_sums_diagnostic_only":{"candidate":fj(sum.c),"reference":fj(sum.r),"candidate_including_calibration":fj(sum.cc),"reference_including_calibration":fj(sum.rr)},"qualified_ratio":null}));
        }
        Ok(Value::Array(rows))
    }
}
fn expected(plan: &Value, e: &Value, entry: &Value, binary: &str) -> Result<Value, String> {
    let w = a(plan, "/workloads")?
        .get(n(e, "/workload")? as usize)
        .ok_or("workload index out of range")?;
    let arm = a(plan, "/arms")?
        .get(n(e, "/arm")? as usize)
        .ok_or("arm index out of range")?;
    Ok(
        json!({"session":entry.get("session_id").ok_or("session identity field missing")?,"binary_sha256":binary,"seq":n(e,"/seq")?,"round":n(e,"/round")?,"warmup":e["warmup"],"arm":s(arm,"/name")?,"role":s(arm,"/role")?,"seed":n(e,"/algorithm_seed")?.to_string(),"workload":w,"method":arm["method"]}),
    )
}
const DEFAULT_PLANS: [(&str, &str); 7] = [
    (
        "cohort-16.json",
        "35e6311a7105a29c2459543ad0b281ed91e43aa63a99f66226f95a1f3c9cc2ba",
    ),
    (
        "cohort-20.json",
        "51a99e83c38b4d5d6b19f82337918b2f64f34b4934a5361298e84fc2f8bd432f",
    ),
    (
        "cohort-24.json",
        "7d34d0ce87c5a167ee1e94fecb08bcac34c9b4339570deb6377effe75efc28c4",
    ),
    (
        "cohort-28.json",
        "87760bc68b7598250c22976f15e4dbb36b5f2d738647de501222de1a8b27b001",
    ),
    (
        "cohort-32.json",
        "a9aebbc3120b61380a6936408635ac0f0e60715b4b4918dc826fbab4b578ae96",
    ),
    (
        "cohort-36.json",
        "6bb162d846bfbfe68b1bf7ba330abbc1e11c403b003215c960e7e71d81be3aa2",
    ),
    (
        "cohort-40.json",
        "8f2ec73a139f84b660573bd67c54ebfc567626ce7068a135dcb52c61891b22d5",
    ),
];
const UPPER_GATE_PLAN_SHA256: &str =
    "481d795e6208cfb3ca648c463de2d7b232adfb7d28bebc341afa4f58e119dfcf";
fn exact_plan_inventory(entries: &[Value], expected: &[(&str, &str)]) -> bool {
    entries.len() == expected.len()
        && expected.iter().all(|(name, digest)| {
            entries
                .iter()
                .any(|entry| entry["name"] == *name && entry["sha256"] == *digest)
        })
}
/// Reads the actual prepared-plan validation schema and ecbench plans. This does
/// not freeze a protocol or authenticate a caller-provided digest.
pub fn aggregate(manifest: &Value) -> Result<Value, String> {
    if !manifest["analysis_scope"].is_null() {
        return Err("full1260 route does not accept a gate scope".into());
    }
    if !exact_plan_inventory(a(manifest, "/plans")?, &DEFAULT_PLANS) {
        return Err("exact original seven-plan membership required".into());
    }
    aggregate_scope(manifest, 1260, None, None)
}
/// Exact prepared preflight membership, not a caller-selected denominator.
pub fn p739_gate60_scope() -> Value {
    json!({"kind":"p739-gate60","expected_workloads":60,"prepared_plan_sha256":"bc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776"})
}
pub fn aggregate_p739_gate60(manifest: &Value) -> Result<Value, String> {
    if manifest["analysis_scope"] != p739_gate60_scope() {
        return Err("exact p739 gate60 scope required".into());
    }
    let entries = a(manifest, "/plans")?;
    if entries.len() != 1
        || entries[0]["name"] != "gate-p739.json"
        || entries[0]["sha256"] != p739_gate60_scope()["prepared_plan_sha256"]
    {
        return Err("exact prepared p739 gate60 plan required".into());
    }
    // read_bound checks actual bytes, so editing or omitting a target is refused.
    read_bound(&entries[0])?;
    aggregate_scope(manifest, 60, None, None)
}
pub fn p186379_target0_scope() -> Value {
    json!({"kind":"gate-p186379-target0/v1","expected_workloads":1,"expected_executions":18,"expected_measured_pairs":5,"prepared_plan_sha256":UPPER_GATE_PLAN_SHA256,"charged_separately_from_full_cohort":true})
}
pub fn aggregate_p186379_target0(manifest: &Value) -> Result<Value, String> {
    if manifest["analysis_scope"] != p186379_target0_scope() {
        return Err("exact p186379 target0 gate scope required".into());
    }
    let entries = a(manifest, "/plans")?;
    if entries.len() != 1
        || entries[0]["name"] != "gate-p186379-target0.json"
        || entries[0]["sha256"] != UPPER_GATE_PLAN_SHA256
    {
        return Err("exact prepared p186379 target0 plan required".into());
    }
    read_bound(&entries[0])?;
    aggregate_scope(manifest, 1, None, None)
}
pub fn aggregate_disjoint_p739(manifest: &Value) -> Result<Value, String> {
    let validation = disjoint::validate_disjoint_manifest(manifest)?;
    let mut output = aggregate_scope(
        manifest,
        1260,
        Some(&validation["normalized_prepared_plan_validation"]),
        Some(&validation["lineage"]),
    )?;
    output["verified_disjoint_enrollment"] = validation;
    Ok(output)
}
/// Shared exporter API: recompute enrollment and return only the validated
/// per-plan lineage. The full validation remains attached to aggregate output.
pub fn validate_disjoint_manifest(manifest: &Value) -> Result<Value, String> {
    Ok(disjoint::validate_disjoint_manifest(manifest)?["lineage"].clone())
}
/// Shared exporter API: normalize the preparation receipt without weakening
/// native preparation's distinct receipt schema.
pub fn normalized_disjoint_receipt(manifest: &Value) -> Result<Value, String> {
    Ok(
        disjoint::validate_disjoint_manifest(manifest)?["normalized_prepared_plan_validation"]
            .clone(),
    )
}
fn aggregate_scope(
    manifest: &Value,
    target_count: usize,
    prepared_override: Option<&Value>,
    lineage: Option<&Value>,
) -> Result<Value, String> {
    if s(manifest, "/schema")? != "pencil-price-cohort-input/v1" {
        return Err("unsupported cohort input schema".into());
    }
    let separate_resources: Value = if manifest["separate_resources"].is_null() {
        Value::Null
    } else {
        serde_json::from_slice(&read_bound(&manifest["separate_resources"])?)
            .map_err(|e| e.to_string())?
    };
    let receipt: Value = if let Some(receipt) = prepared_override {
        receipt.clone()
    } else {
        let receipt_bytes = read_bound(&manifest["prepared_plan_validation"])?;
        serde_json::from_slice(&receipt_bytes).map_err(|e| e.to_string())?
    };
    if s(&receipt, "/schema")? != "pencil-prepared-plan-validation/v1"
        || receipt["passed"] != json!(true)
    {
        return Err("not a successful prepared-plan validation receipt".into());
    }
    let declared = receipt["files"]
        .as_object()
        .ok_or("receipt file map missing")?;
    let entries = a(manifest, "/plans")?;
    if entries.len() != declared.len() {
        return Err("plan set differs from receipt".into());
    }
    let binary = s(manifest, "/binary_sha256")?;
    let protocol = s(manifest, "/protocol_sha256")?;
    let lineage_plans: BTreeMap<&str, &Value> = match lineage {
        Some(value) => a(value, "/plans")?
            .iter()
            .map(|v| Ok((s(v, "/name")?, v)))
            .collect::<Result<_, String>>()?,
        None => BTreeMap::new(),
    };
    let labels = ["pencil", "rho", "rho_AA", "rho"];
    let mut seen_plans = BTreeSet::new();
    let mut seen_workloads = BTreeSet::new();
    let mut seen_sessions = BTreeSet::new();
    let mut pairs = vec![];
    let mut controls = vec![];
    let mut missing = vec![];
    let mut plan_observations = vec![];
    let mut total_executions = 0u64;
    let mut total_measured = 0u64;
    let mut observed_executions = 0u64;
    let mut observed_warmups = 0u64;
    let mut outcomes: BTreeMap<String, u64> = BTreeMap::new();
    let mut all = Sums::new((target_count * 5) as u64);
    let mut streams: Vec<_> = (0..5).map(|_| Sums::new(target_count as u64)).collect();
    for entry in entries {
        let name = s(entry, "/name")?;
        if !seen_plans.insert(name)
            || declared.get(name).and_then(Value::as_str) != Some(s(entry, "/sha256")?)
        {
            return Err("plan filename/digest differs from receipt or duplicated".into());
        }
        let session = match entry.get("session_id") {
            Some(Value::String(value)) if !value.is_empty() => Some(value.as_str()),
            Some(Value::Null) => None,
            _ => return Err("plan session identity must be nonempty string or null".into()),
        };
        if let Some(session) = session {
            if !seen_sessions.insert(session) {
                return Err("session reused across plans".into());
            }
        } else if !entry["records"].is_null() {
            return Err("records require a session identity".into());
        }
        let plan_bytes = read_bound(entry)?;
        let plan_text = String::from_utf8(plan_bytes).map_err(|e| e.to_string())?;
        let plan: Value = serde_json::from_str(&plan_text).map_err(|e| e.to_string())?;
        let records_text = if entry["records"].is_null() {
            String::new()
        } else {
            String::from_utf8(read_bound(&entry["records"])?).map_err(|e| e.to_string())?
        };
        let diagnostics =
            pencil_paired_export::export_with_plan(&records_text, labels, &plan_text)?;
        let mut records = BTreeMap::new();
        for line in records_text.lines().filter(|x| !x.trim().is_empty()) {
            let record: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            if Some(s(&record, "/session_id")?) != session
                || s(&record, "/binary_sha256")? != binary
            {
                return Err("record session/binary differs from manifest".into());
            }
            let seq = n(&record, "/seq")?;
            if records.insert(seq, record).is_some() {
                return Err("duplicate record seq".into());
            }
        }
        observed_executions += records.len() as u64;
        let arms = a(&plan, "/arms")?;
        if arms.len() != 3
            || arms[0]["name"] != json!("pencil")
            || arms[1]["name"] != json!("rho")
            || arms[2]["name"] != json!("rho_AA")
            || arms[0]["role"] != json!("candidate")
            || arms[1]["role"] != json!("reference")
            || arms[2]["role"] != json!("control")
            || arms[1]["method"] != arms[2]["method"]
            || arms[0]["method"]["id"] != json!("ic.three_leads_genus2_pencil_symbolic_v2")
            || arms[1]["method"]["id"] != json!("rho.three_leads_negation_v1")
        {
            return Err("three-arm identity differs from registered design".into());
        }
        let executions = a(&plan, "/executions")?;
        total_executions += executions.len() as u64;
        let mut by_workload: WorkloadRounds<'_> = BTreeMap::new();
        for e in executions {
            let wi = n(e, "/workload")? as usize;
            let ai = n(e, "/arm")? as usize;
            let warmup = e["warmup"].as_bool().ok_or("plan warmup missing")?;
            if !warmup {
                total_measured += 1;
            }
            let arm = s(arms.get(ai).ok_or("arm index out of range")?, "/name")?;
            let group = by_workload
                .entry(wi)
                .or_default()
                .entry((warmup, n(e, "/round")?))
                .or_default();
            if group.insert(arm.into(), e).is_some() {
                return Err("duplicate planned arm slot".into());
            }
            let seq = n(e, "/seq")?;
            if let Some(record) = records.get(&seq) {
                if warmup {
                    observed_warmups += 1;
                }
                let status = record
                    .pointer("/outcome/status")
                    .and_then(Value::as_str)
                    .unwrap_or("missing_status");
                *outcomes.entry(format!("{arm}:{status}")).or_default() += 1;
            } else {
                missing.push(json!({"plan":name,"session_id":entry["session_id"],"seq":seq,"expected":expected(&plan,e,entry,binary)?}));
            }
        }
        for (wi, w) in a(&plan, "/workloads")?.iter().enumerate() {
            let wid = s(w, "/workload_id")?;
            if !seen_workloads.insert(wid.to_string()) {
                return Err("workload repeated across cohort plans".into());
            }
            let groups = by_workload.get(&wi).ok_or("workload has no executions")?;
            let measured: Vec<_> = groups.iter().filter(|((warmup, _), _)| !*warmup).collect();
            let warmups: Vec<_> = groups.iter().filter(|((warmup, _), _)| *warmup).collect();
            if measured.len() != 5 || warmups.len() != 1 || groups.values().any(|g| g.len() != 3) {
                return Err(
                    "workload lacks exactly five measured and one warmup three-arm rounds".into(),
                );
            }
            for (stream, ((_, round), group)) in measured.into_iter().enumerate() {
                let ce = group["pencil"];
                let re = group["rho"];
                let cseq = n(ce, "/seq")?;
                let rseq = n(re, "/seq")?;
                let (plan_protocol, coverage_review) =
                    if let Some(lineage) = lineage_plans.get(name) {
                        (
                            s(lineage, "/data_protocol_sha256")?,
                            &lineage["coverage_review"],
                        )
                    } else {
                        (protocol, &manifest["coverage_review"])
                    };
                let bundle = json!({"schema":"pencil-price-pair-input/v1","p":n(w,"/curve_spec/p")?,"r":n(w,"/curve_spec/r")?,"protocol_sha256":plan_protocol,"coverage_review":coverage_review,"candidate":{"expected":expected(&plan,ce,entry,binary)?,"record":records.get(&cseq)},"reference":{"expected":expected(&plan,re,entry,binary)?,"record":records.get(&rseq)}});
                let result = analyzer::analyze(&bundle)?;
                all.push(&result["cells"])?;
                streams[stream].push(&result["cells"])?;
                pairs.push(json!({"plan":name,"session_id":entry["session_id"],"workload_id":wid,"stream_index":stream,"round":round,"candidate_seq":cseq,"reference_seq":rseq,"candidate_record_id":result["candidate_record_id"],"reference_record_id":result["reference_record_id"],"coverage_blockers":result["coverage_blockers"],"cells":result["cells"]}));
            }
        }
        controls.extend(
            a(&diagnostics, "/target_summaries")?
                .iter()
                .map(|x| json!({"plan":name,"target":x})),
        );
        plan_observations.push(json!({"name":name,"plan_binding":diagnostics["plan_binding"],"record_artifact":entry["records"],"session_id":entry["session_id"],"execution_history":entry.get("execution_history").unwrap_or(&Value::Null),"source_validation_binary_sha256":receipt["binary_sha256"]}));
    }
    if seen_workloads.len() != target_count
        || total_executions != (target_count * 18) as u64
        || total_measured != (target_count * 15) as u64
        || n(&receipt, "/targets")? != target_count as u64
        || n(&receipt, "/planned_executions")? != total_executions
        || n(&receipt, "/measured_executions")? != total_measured
        || n(&receipt, "/warmup_executions")? != total_executions - total_measured
    {
        return Err("cohort dimensions differ from original design or receipt".into());
    }
    let mut stream_rows = vec![];
    for (i, total) in streams.iter().enumerate() {
        stream_rows.push(json!({"stream_index":i,"cells":total.output()?}));
    }
    let cold_ok = controls
        .iter()
        .all(|x| x["target"]["cold_AA"]["calibration_passed"] == json!(true));
    let online_ok = controls
        .iter()
        .all(|x| x["target"]["online_AA"]["calibration_passed"] == json!(true));
    Ok(
        json!({"schema":"pencil-price-cohort-output/v1","aggregate_source_sha256":sha(include_bytes!("aggregate.rs")),"paired_export_source_sha256":sha(include_bytes!("../../paired_export/src/lib.rs")),"paired_export_plan_source_sha256":sha(include_bytes!("../../paired_export/src/plan.rs")),"mapping":analyzer::mapping_contract(),"input_manifest":manifest,"input_manifest_integrity_sha256":sha(&serde_json::to_vec(manifest).map_err(|e|e.to_string())?),"integrity":"prepared-plan receipt, plans, records and optional resource-file digests checked; exact record-to-plan identities checked","authenticity":"not_established_by_this_tool; prepared-plan validation and provided digests do not freeze or authorize experiments","qualified_ratios":null,"separate_resource_observations":separate_resources,"expected_workloads":target_count,"expected_measured_pairs":target_count*5,"expected_executions":total_executions,"observed_executions":observed_executions,"expected_warmup_executions":total_executions-total_measured,"observed_warmup_executions":observed_warmups,"all_expected_records_present":missing.is_empty(),"missing_executions":missing,"outcome_counts":outcomes,"aggregate_cells":all.output()?,"streams":stream_rows,"pair_rows":pairs,"AA_rule":"per target all five measured rho_AA/rho pairs must individually pass inclusive[0.90,1.10], independently for cold and online; no aggregate rescue","all_target_cold_AA_passed":cold_ok,"all_target_online_AA_passed":online_ok,"target_controls":controls,"plan_observations":plan_observations,"resource_policy":"Raw records remain bound by file digest. Warmups, audits, reporting, retries and infrastructure are separate observations; no overlapping clocks or invented GAE conversion."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rows(c: Fraction, r: Fraction) -> Value {
        json!(grid().into_iter().map(|cell|json!({"square_twice":cell.square_twice,"inversion":cell.inversion,"native_tenths":cell.native_tenths,"candidate":fj(c),"reference":fj(r),"candidate_including_calibration":fj(c),"reference_including_calibration":fj(r)})).collect::<Vec<_>>())
    }
    #[test]
    fn ratios_are_sums_of_prices_not_mean_ratios() {
        let mut sums = Sums::new(2);
        sums.push(&rows(
            Fraction::new(1, 2).unwrap(),
            Fraction::new(1, 4).unwrap(),
        ))
        .unwrap();
        sums.push(&rows(
            Fraction::new(3, 4).unwrap(),
            Fraction::new(1, 2).unwrap(),
        ))
        .unwrap();
        for row in sums.output().unwrap().as_array().unwrap() {
            assert_eq!(
                row["algorithm_ratio_of_sums"],
                fj(Fraction::new(5, 3).unwrap())
            );
            assert_ne!(
                row["algorithm_ratio_of_sums"],
                fj(Fraction::new(7, 4).unwrap())
            );
        }
    }
    #[test]
    fn one_missing_pair_nulls_full_denominator() {
        let mut sums = Sums::new(2);
        sums.push(&rows(
            Fraction::new(2, 1).unwrap(),
            Fraction::new(1, 1).unwrap(),
        ))
        .unwrap();
        for row in sums.output().unwrap().as_array().unwrap() {
            assert!(row["algorithm_ratio_of_sums"].is_null());
            assert_eq!(row["admitted_pairs"], 1);
            assert_eq!(row["missing_or_ineligible_pairs"], 1);
        }
    }
    fn fixture() -> (std::path::PathBuf, Value) {
        let dir = std::env::temp_dir().join(format!(
            "pencil-aggregate-software-fixture-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        let arms = json!([{"name":"pencil","role":"candidate","method":{"id":"ic.three_leads_genus2_pencil_symbolic_v2","params":{}}},{"name":"rho","role":"reference","method":{"id":"rho.three_leads_negation_v1","params":{}}},{"name":"rho_AA","role":"control","method":{"id":"rho.three_leads_negation_v1","params":{}}}]);
        let executions:Vec<_>=(0..6).flat_map(|round|(0..3).map(move|arm|json!({"seq":round*3+arm,"round":round,"warmup":round==0,"arm":arm,"workload":0,"algorithm_seed":round+10}))).collect();
        let plan = json!({"schema":"ecbench.plan/v1","spec_id":"synthetic-software-fixture-not-frozen","arms":arms,"workloads":[{"workload_id":"W-test-only","curve_spec":{"p":739,"r":34147}}],"executions":executions});
        let path = dir.join("plan.json");
        let bytes = serde_json::to_vec(&plan).unwrap();
        fs::write(&path, &bytes).unwrap();
        let hash = sha(&bytes);
        let receipt = json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":1,"planned_executions":18,"measured_executions":15,"warmup_executions":3,"solver_executions_performed":0,"binary_sha256":"b".repeat(64),"files":{"plan.json":hash}});
        let rb = serde_json::to_vec(&receipt).unwrap();
        let rp = dir.join("receipt.json");
        fs::write(&rp, &rb).unwrap();
        let manifest = json!({"schema":"pencil-price-cohort-input/v1","prepared_plan_validation":{"path":rp,"sha256":sha(&rb)},"binary_sha256":"b".repeat(64),"protocol_sha256":"a".repeat(64),"coverage_review":null,"plans":[{"name":"plan.json","path":path,"sha256":hash,"session_id":"synthetic-software-session","records":null}]});
        (dir, manifest)
    }
    #[test]
    fn exact_p739_gate_preserves_all_sixty_targets_and_rejects_edits() {
        let (dir, mut manifest) = fixture();
        let bytes = include_bytes!("../tests/fixtures/gate-p739.json");
        let path = dir.join("gate-p739.json");
        fs::write(&path, bytes).unwrap();
        assert_eq!(sha(bytes), p739_gate60_scope()["prepared_plan_sha256"]);
        let receipt = json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":60,"planned_executions":1080,"measured_executions":900,"warmup_executions":180,"solver_executions_performed":0,"binary_sha256":"b".repeat(64),"files":{"gate-p739.json":sha(bytes)}});
        let rb = serde_json::to_vec(&receipt).unwrap();
        let rp = dir.join("gate-receipt.json");
        fs::write(&rp, &rb).unwrap();
        manifest["prepared_plan_validation"] = json!({"path":rp,"sha256":sha(&rb)});
        manifest["plans"] = json!([{"name":"gate-p739.json","path":path,"sha256":sha(bytes),"session_id":"software-gate-not-executed","records":null}]);
        manifest["analysis_scope"] = p739_gate60_scope();
        let out = aggregate_p739_gate60(&manifest).unwrap();
        assert_eq!(out["expected_workloads"], 60);
        assert_eq!(out["expected_measured_pairs"], 300);
        assert_eq!(out["expected_executions"], 1080);
        assert_eq!(out["missing_executions"].as_array().unwrap().len(), 1080);
        assert_eq!(out["observed_executions"], 0);
        assert!(out["aggregate_cells"][0]["algorithm_ratio_of_sums"].is_null());
        assert!(aggregate(&manifest).is_err());
        let mut wrong = manifest.clone();
        wrong["analysis_scope"]["expected_workloads"] = json!(59);
        assert!(aggregate_p739_gate60(&wrong).is_err());
        let mut plan: Value = serde_json::from_slice(bytes).unwrap();
        plan["workloads"].as_array_mut().unwrap().pop();
        let edited = serde_json::to_vec(&plan).unwrap();
        fs::write(&path, &edited).unwrap();
        assert!(aggregate_p739_gate60(&manifest).is_err());
        manifest["plans"][0]["sha256"] = json!(sha(&edited));
        assert!(aggregate_p739_gate60(&manifest).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn absent_whole_target_stays_in_plan_denominator() {
        let (dir, manifest) = fixture();
        let out = aggregate_scope(&manifest, 1, None, None).unwrap();
        assert_eq!(out["expected_measured_pairs"], 5);
        assert_eq!(out["expected_executions"], 18);
        assert_eq!(out["missing_executions"].as_array().unwrap().len(), 18);
        assert_eq!(out["pair_rows"].as_array().unwrap().len(), 5);
        assert_eq!(out["all_target_cold_AA_passed"], false);
        assert_eq!(out["all_target_online_AA_passed"], false);
        assert!(out["aggregate_cells"]
            .as_array()
            .unwrap()
            .iter()
            .all(|x| x["algorithm_ratio_of_sums"].is_null()));
        assert!(aggregate(&manifest)
            .unwrap_err()
            .contains("exact original seven-plan membership"));
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn exact_upper_gate_is_separate_and_membership_pinned() {
        let (dir, mut manifest) = fixture();
        let bytes = include_bytes!("../tests/fixtures/gate-p186379-target0.json");
        let path = dir.join("gate-p186379-target0.json");
        fs::write(&path, bytes).unwrap();
        assert_eq!(sha(bytes), UPPER_GATE_PLAN_SHA256);
        let receipt = json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":1,"planned_executions":18,"measured_executions":15,"warmup_executions":3,"solver_executions_performed":0,"binary_sha256":"b".repeat(64),"files":{"gate-p186379-target0.json":sha(bytes)}});
        let rb = serde_json::to_vec(&receipt).unwrap();
        let rp = dir.join("upper-receipt.json");
        fs::write(&rp, &rb).unwrap();
        manifest["prepared_plan_validation"] = json!({"path":rp,"sha256":sha(&rb)});
        manifest["plans"] = json!([{"name":"gate-p186379-target0.json","path":path,"sha256":sha(bytes),"session_id":"software-upper-not-executed","records":null}]);
        manifest["analysis_scope"] = p186379_target0_scope();
        let out = aggregate_p186379_target0(&manifest).unwrap();
        assert_eq!(out["expected_workloads"], 1);
        assert_eq!(out["expected_measured_pairs"], 5);
        assert_eq!(out["expected_executions"], 18);
        assert_eq!(out["observed_executions"], 0);
        assert_eq!(out["missing_executions"].as_array().unwrap().len(), 18);
        assert!(aggregate(&manifest).is_err());
        assert!(aggregate_p739_gate60(&manifest).is_err());
        let mut wrong = manifest.clone();
        wrong["analysis_scope"]["charged_separately_from_full_cohort"] = json!(false);
        assert!(aggregate_p186379_target0(&wrong).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn changed_plan_bytes_fail_even_with_same_cardinality() {
        let (dir, manifest) = fixture();
        let path = s(&manifest["plans"][0], "/path").unwrap();
        let mut plan: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        plan["workloads"][0]["workload_id"] = json!("different-target");
        fs::write(path, serde_json::to_vec(&plan).unwrap()).unwrap();
        assert!(aggregate_scope(&manifest, 1, None, None)
            .unwrap_err()
            .contains("digest mismatch"));
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn duplicate_plan_cannot_replace_missing_plan() {
        let (dir, mut manifest) = fixture();
        let p = manifest["plans"][0].clone();
        manifest["plans"].as_array_mut().unwrap().push(p);
        assert!(aggregate_scope(&manifest, 1, None, None)
            .unwrap_err()
            .contains("plan set differs"));
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn malformed_grid_and_overflow_reject() {
        let mut sums = Sums::new(2);
        let mut row = rows(Fraction::new(1, 1).unwrap(), Fraction::new(1, 1).unwrap());
        row[0]["inversion"] = json!(999);
        assert!(sums.push(&row).is_err());
        let mut sums = Sums::new(2);
        sums.push(&rows(
            Fraction::new(u128::MAX, 1).unwrap(),
            Fraction::new(1, 1).unwrap(),
        ))
        .unwrap();
        assert!(sums
            .push(&rows(
                Fraction::new(1, 1).unwrap(),
                Fraction::new(1, 1).unwrap()
            ))
            .is_err());
    }
    fn records_for_fixture(manifest: &Value) -> Vec<Value> {
        let plan: Value =
            serde_json::from_slice(&fs::read(s(&manifest["plans"][0], "/path").unwrap()).unwrap())
                .unwrap();
        plan["executions"].as_array().unwrap().iter().map(|e|{let arm=&plan["arms"][e["arm"].as_u64().unwrap()as usize];json!({"schema":"ecbench.record/v1","record_id":format!("synthetic-{}",e["seq"]),"session_id":"synthetic-software-session","binary_sha256":"b".repeat(64),"seq":e["seq"],"round":e["round"],"warmup":e["warmup"],"arm":arm["name"],"role":arm["role"],"method":arm["method"],"workload":plan["workloads"][0],"algorithm_seed":e["algorithm_seed"].as_u64().unwrap().to_string(),"outcome":{"status":"verified","matches_target":true,"matches_planted":true,"error":null},"time":{"method_call_wall_ns":100,"online_clock_diagnostics":{"error":null}},"isolation":{"level":"L2"},"online":{"wall_ns":100,"phases_ns":{"rho_solve":100}},"detail":{}})}).collect()
    }
    fn attach_records(dir: &std::path::Path, manifest: &mut Value, records: &[Value]) {
        let text = records
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        let path = dir.join("records.jsonl");
        fs::write(&path, text.as_bytes()).unwrap();
        manifest["plans"][0]["records"] = json!({"path":path,"sha256":sha(text.as_bytes())});
    }
    #[test]
    fn individual_aa_failure_cannot_hide_in_aggregate_average() {
        let (dir, mut manifest) = fixture();
        let mut records = records_for_fixture(&manifest);
        attach_records(&dir, &mut manifest, &records);
        let pass = aggregate_scope(&manifest, 1, None, None).unwrap();
        assert_eq!(pass["all_target_cold_AA_passed"], true);
        assert_eq!(pass["all_target_online_AA_passed"], true);
        assert_eq!(pass["observed_warmup_executions"], 3);
        let one = records
            .iter_mut()
            .find(|r| r["arm"] == "rho_AA" && r["round"] == 1)
            .unwrap();
        one["time"]["method_call_wall_ns"] = json!(120);
        attach_records(&dir, &mut manifest, &records);
        let fail = aggregate_scope(&manifest, 1, None, None).unwrap();
        assert_eq!(fail["all_target_cold_AA_passed"], false);
        assert_eq!(fail["all_target_online_AA_passed"], true); // mean cold ratio1.04 does not rescue failing1.20.
        assert_eq!(fail["expected_measured_pairs"], 5);
        assert_eq!(fail["all_expected_records_present"], true);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn duplicate_and_wrong_identity_records_are_rejected() {
        let (dir, mut manifest) = fixture();
        let mut records = records_for_fixture(&manifest);
        records.push(records[0].clone());
        attach_records(&dir, &mut manifest, &records);
        assert!(aggregate_scope(&manifest, 1, None, None).is_err());
        records.pop();
        records[0]["binary_sha256"] = json!("c".repeat(64));
        attach_records(&dir, &mut manifest, &records);
        assert!(aggregate_scope(&manifest, 1, None, None)
            .unwrap_err()
            .contains("session/binary"));
        fs::remove_dir_all(dir).unwrap();
    }
}
