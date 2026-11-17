//! Frozen native analysis: complete costs, matched comparisons, all outcomes.
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
};
#[allow(dead_code)]
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn put(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?;
    writeln!(f, "{}", serde_json::to_string_pretty(v)?)?;
    Ok(())
}
fn n(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}
fn key(t: &Value, rep: &Value, drop: &[&str], cohort: Option<&Value>) -> String {
    let mut v = t.clone();
    let m = v.as_object_mut().unwrap();
    m.remove("id");
    m.remove("seed");
    for d in drop {
        m.remove(*d);
    }
    serde_json::to_string(&json!([v, rep, cohort])).unwrap()
}
fn successful(r: &Value) -> bool {
    r["complete_cost"] == true && r["scalar_verified"] == true && r["status"] == "recovered"
}
fn phase(r: &Value, s: &str) -> u64 {
    r["cost"]["exclusive_phases"].as_array().map_or(0, |a| {
        a.iter()
            .filter(|x| x["phase"] == s)
            .map(|x| n(&x["total_ir"]))
            .sum()
    })
}
const HASHES: [&str; 5] = [
    "base_sha256",
    "known_rows_sha256",
    "query_targets_sha256",
    "witnesses_sha256",
    "query_rows_sha256",
];
fn equal_problem(a: &Value, b: &Value) -> bool {
    HASHES.iter().all(|h| !a[*h].is_null() && a[*h] == b[*h]) && a["status"] == b["status"]
}
// A complete pre-query attempt can omit query hashes by construction.
// This never makes a missing-hash recovery eligible for a cost ratio.
fn matched_pre_query_failure(a: &Value, b: &Value) -> bool {
    (a["status"] == "underfilled" || a["status"] == "unsupported-alpha")
        && a["status"] == b["status"]
        && a["complete_attempt_cost"] == true
        && b["complete_attempt_cost"] == true
        && ["base_sha256", "known_rows_sha256"]
            .iter()
            .all(|h| !a[*h].is_null() && a[*h] == b[*h])
        && [
            "query_targets_sha256",
            "witnesses_sha256",
            "query_rows_sha256",
        ]
        .iter()
        .all(|h| a[*h].is_null() && b[*h].is_null())
}
fn bounds(v: &[f64]) -> Value {
    if v.is_empty() {
        Value::Null
    } else {
        let mut a = v.to_vec();
        a.sort_by(f64::total_cmp);
        json!({"n":a.len(),"min":a[0],"median":a[a.len()/2],"max":a[a.len()-1],"interval":"observed range only, not confidence interval"})
    }
}
fn range_pairs(a: &[Value], metric: &str) -> Value {
    bounds(
        &a.iter()
            .filter_map(|v| v[metric].as_f64())
            .collect::<Vec<_>>(),
    )
}
fn file(base: &Path, p: &str, kind: &str) -> Result<Value> {
    let q = base.join(p);
    Ok(json!({"path":p,"sha256":sha(&q)?,"bytes":fs::metadata(q)?.len(),"kind":kind}))
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("summary WAVE AUDIT_DIR FRESH_OUTPUT_DIR".into());
    }
    let wave = Path::new(&a[0]);
    let audit = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    if out.exists() {
        return Err("fresh analysis output required".into());
    }
    fs::create_dir_all(out)?;
    let plan = read(wave.join("PLAN.json"))?;
    let receipt = read(wave.join("runs/TRANSPORT.json"))?;
    let proof = read(audit.join("AUDIT.json"))?;
    let admit = read(audit.join("ADMISSION.json"))?;
    if admit["status"] != "native-counters-and-independent-GP-verified"
        || admit["audit_sha256"] != sha(audit.join("AUDIT.json"))?
    {
        return Err("strict independent replay not admitted".into());
    }
    let cells = plan["cells"].as_array().ok_or("cells")?;
    let raw = fs::read_to_string(wave.join("runs/campaign-v1/ROWS.jsonl"))?;
    let rows: Vec<Value> = raw
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    if rows.len() != cells.len() || n(&proof["actual_rows"]) != rows.len() as u64 {
        return Err("incomplete planned ledger".into());
    }
    let mut counts = BTreeMap::<String, u64>::new();
    let mut targets = BTreeSet::new();
    let mut fixtures = BTreeSet::new();
    let mut groups = BTreeMap::<String, Vec<usize>>::new();
    let mut resource = BTreeMap::<String, Vec<f64>>::new();
    let mut references = BTreeMap::<String, (usize, u64)>::new();
    let mut regress = BTreeMap::<String, Vec<usize>>::new();
    let mut aa = BTreeMap::<String, Vec<f64>>::new();
    let mut data = vec![];
    for (i, r) in rows.iter().enumerate() {
        if n(&r["index"]) != i as u64 || r["task_id"] != cells[i]["task"]["id"] {
            return Err("plan/row position mismatch".into());
        }
        let c = &cells[i];
        let t = &c["task"];
        let co = c["cohort"].as_str().unwrap();
        *counts
            .entry(format!("{co}|{}", r["status"].as_str().unwrap()))
            .or_default() += 1;
        fixtures.insert(t["fixture"]["id"].to_string());
        if co != "AA" {
            targets.insert(serde_json::to_string(&json!([
                t["fixture"]["id"],
                t["target"]
            ]))?);
        }
        if let Some(x) = r["elapsed_s_for_resource_sizing_only"].as_f64() {
            resource
                .entry(format!("{co}|{}", t["method"].as_str().unwrap()))
                .or_default()
                .push(x);
        }
        let rkey =
            serde_json::to_string(&json!([t["fixture"]["id"], t["target"], c["replicate"]]))?;
        if co.contains("reference") && successful(r) {
            let ir = n(&r["cost"]["total_ir"]);
            if references.get(&rkey).is_none_or(|x| ir < x.1) {
                references.insert(rkey.clone(), (i, ir));
            }
        }
        if co == "AA" && r["cost"]["total_ir"].is_number() {
            aa.entry(c["aa_group"].as_str().unwrap().into())
                .or_default()
                .push(n(&r["cost"]["total_ir"]) as f64);
        }
        for (drop, tag) in [
            (&["compression"][..], "compression"),
            (&["family"][..], "family"),
            (&["evaluation"][..], "evaluation"),
        ] {
            groups
                .entry(format!(
                    "{tag}|{}",
                    key(t, &c["replicate"], drop, Some(&c["cohort"]))
                ))
                .or_default()
                .push(i);
        }
        if co == "regression-60" {
            regress
                .entry(key(t, &c["replicate"], &["method"], Some(&c["cohort"])))
                .or_default()
                .push(i);
        }
        data.push(json!({"index":i,"task_id":r["task_id"],"cohort":co,"fixture":t["fixture"]["id"],"p":t["fixture"]["p"],"r":t["fixture"]["r"],"target_label":t["target_label"],"replicate":c["replicate"],"method":t["method"],"family":t["family"],"B":t["b"],"shift":t["shift"],"alpha":t["alpha"],"evaluation":t["evaluation"],"compression":t["compression"],"queries":t["queries"],"status":r["status"],"complete_attempt_cost":r["complete_attempt_cost"],"complete_cost":r["complete_cost"],"total_ir":r["cost"]["total_ir"],"setup_ir":r["cost"]["setup_ir"],"online_ir":r["cost"]["online_ir"],"linear_algebra_ir":phase(r,"linear-algebra"),"phases":r["cost"]["exclusive_phases"],"s_ir":if successful(r){json!(n(&r["cost"]["total_ir"])as f64/(n(&t["fixture"]["r"])as f64).sqrt())}else{Value::Null},"reference_key":rkey,"max_sampled_tree_rss_bytes":r["max_sampled_tree_rss_bytes"]}));
    }
    let mut compression = vec![];
    let mut family = vec![];
    let mut evaluation = vec![];
    for (k, ids) in groups {
        if k.starts_with("compression|") {
            let l = ids
                .iter()
                .copied()
                .find(|i| cells[*i]["task"]["compression"] == false);
            let r = ids
                .iter()
                .copied()
                .find(|i| cells[*i]["task"]["compression"] == true);
            if let (Some(l), Some(r)) = (l, r) {
                let eq = equal_problem(&rows[l], &rows[r]);
                if rows[l]["complete_attempt_cost"] == true
                    && rows[r]["complete_attempt_cost"] == true
                    && !rows[l]["base_sha256"].is_null()
                    && !eq
                    && !matched_pre_query_failure(&rows[l], &rows[r])
                {
                    return Err(format!("compression changed equations/outcome at {l}/{r}").into());
                }
                compression.push(json!({"full":l,"compressed":r,"cohort":cells[l]["cohort"],"alpha":cells[l]["task"]["alpha"],"same_problem":eq,"both_recovered":successful(&rows[l])&&successful(&rows[r]),"total_full_over_compressed":if eq&&successful(&rows[l])&&successful(&rows[r]){json!(n(&rows[l]["cost"]["total_ir"])as f64/n(&rows[r]["cost"]["total_ir"])as f64)}else{Value::Null},"matrix_full_over_compressed":if eq&&phase(&rows[r],"linear-algebra")>0{json!(phase(&rows[l],"linear-algebra")as f64/phase(&rows[r],"linear-algebra")as f64)}else{Value::Null}}));
            }
        } else if k.starts_with("family|") {
            let l = ids
                .iter()
                .copied()
                .find(|i| cells[*i]["task"]["family"] == "low-x");
            let r = ids
                .iter()
                .copied()
                .find(|i| cells[*i]["task"]["family"] == "twist-max");
            if let (Some(l), Some(r)) = (l, r) {
                family.push(json!({"ordinary":l,"twist":r,"cohort":cells[l]["cohort"],"alpha":cells[l]["task"]["alpha"],"ordinary_status":rows[l]["status"],"twist_status":rows[r]["status"],"both_recovered":successful(&rows[l])&&successful(&rows[r]),"ordinary_over_twist":if successful(&rows[l])&&successful(&rows[r]){json!(n(&rows[l]["cost"]["total_ir"])as f64/n(&rows[r]["cost"]["total_ir"])as f64)}else{Value::Null}}));
            }
        } else if k.starts_with("evaluation|") {
            let l = ids
                .iter()
                .copied()
                .find(|i| cells[*i]["task"]["evaluation"] == "map");
            let r = ids
                .iter()
                .copied()
                .find(|i| cells[*i]["task"]["evaluation"] == "scalar");
            if let (Some(l), Some(r)) = (l, r) {
                let eq = equal_problem(&rows[l], &rows[r]);
                if !rows[l]["base_sha256"].is_null()
                    && !rows[r]["base_sha256"].is_null()
                    && !eq
                    && !matched_pre_query_failure(&rows[l], &rows[r])
                {
                    return Err("map/scalar problems differ".into());
                }
                evaluation.push(json!({"map":l,"scalar":r,"same_problem":eq,"both_recovered":successful(&rows[l])&&successful(&rows[r]),"map_over_scalar":if eq&&successful(&rows[l])&&successful(&rows[r]){json!(n(&rows[l]["cost"]["total_ir"])as f64/n(&rows[r]["cost"]["total_ir"])as f64)}else{Value::Null}}));
            }
        }
    }
    // Scalar controls use a separate cohort: match across that explicit spelling too.
    let mut maps = BTreeMap::<String, usize>::new();
    for (i, c) in cells.iter().enumerate() {
        if c["cohort"] == "heldout" && c["task"]["alpha"] == "augment" {
            maps.insert(key(&c["task"], &c["replicate"], &["evaluation"], None), i);
        }
    }
    for (i, c) in cells.iter().enumerate() {
        if c["cohort"] == "heldout-scalar-control" {
            if let Some(j) = maps.get(&key(&c["task"], &c["replicate"], &["evaluation"], None)) {
                let eq = equal_problem(&rows[*j], &rows[i]);
                if !rows[*j]["base_sha256"].is_null()
                    && !eq
                    && !matched_pre_query_failure(&rows[*j], &rows[i])
                {
                    return Err("heldout map/scalar problem mismatch".into());
                }
                evaluation.push(json!({"map":j,"scalar":i,"same_problem":eq,"both_recovered":successful(&rows[*j])&&successful(&rows[i]),"map_over_scalar":if eq&&successful(&rows[*j])&&successful(&rows[i]){json!(n(&rows[*j]["cost"]["total_ir"])as f64/n(&rows[i]["cost"]["total_ir"])as f64)}else{Value::Null}}));
            }
        }
    }
    let mut regression = vec![];
    for (ids_key, ids) in regress {
        let x = &rows[ids[0]];
        let equal = ids.iter().all(|i| {
            equal_problem(x, &rows[*i])
                || matched_pre_query_failure(x, &rows[*i])
                || x["status"] == rows[*i]["status"] && x["base_sha256"].is_null()
        });
        if !equal {
            return Err("regression collector disagreement".into());
        }
        regression.push(json!({"key":ids_key,"rows":ids,"identical_problem_and_outcome":equal}));
    }
    let mut best = BTreeMap::<String, (usize, u64)>::new();
    let mut gaps = vec![];
    for (i, d) in data.iter_mut().enumerate() {
        let k = d["reference_key"].as_str().unwrap().to_owned();
        if let Some((j, v)) = references.get(&k) {
            if successful(&rows[i]) {
                let ratio = n(&rows[i]["cost"]["total_ir"]) as f64 / *v as f64;
                d["strongest_completed_reference_row"] = json!(j);
                d["cold_over_strongest_completed_reference"] = json!(ratio);
                if !cells[i]["cohort"].as_str().unwrap().contains("reference")
                    && cells[i]["cohort"] != "AA"
                {
                    gaps.push(ratio);
                    let ir = n(&rows[i]["cost"]["total_ir"]);
                    if best.get(&k).is_none_or(|x| ir < x.1) {
                        best.insert(k, (i, ir));
                    }
                }
            }
        }
    }
    let best:Vec<_>=best.values().map(|(i,_)|json!({"row":i,"cohort":cells[*i]["cohort"],"ratio_to_reference":data[*i]["cold_over_strongest_completed_reference"],"selection":"hindsight diagnostic, not an executable baseline selector"})).collect();
    let aa: Value = aa
        .iter()
        .map(|(k, v)| (k.clone(), bounds(v)))
        .collect::<serde_json::Map<_, _>>()
        .into();
    let resources: Value = resource
        .iter()
        .map(|(k, v)| (k.clone(), bounds(v)))
        .collect::<serde_json::Map<_, _>>()
        .into();
    let summary = json!({"schema":"twist.cold-summary/v1","plan_sha256":sha(wave.join("PLAN.json"))?,"rows_sha256":sha(wave.join("runs/campaign-v1/ROWS.jsonl"))?,"audit_sha256":sha(audit.join("AUDIT.json"))?,"admission_sha256":sha(audit.join("ADMISSION.json"))?,"planned":cells.len(),"attempted":rows.iter().filter(|r|r["status"]!="not-started-global-cap").count(),"independent_public_DLP_targets":targets.len(),"fixture_models":fixtures.len(),"status_by_cohort":counts,"aa_ir":aa,"resource_elapsed_s_sizing_only":resources,"compression_pair_count":compression.len(),"compression_total_ratio":range_pairs(&compression,"total_full_over_compressed"),"compression_matrix_ratio":range_pairs(&compression,"matrix_full_over_compressed"),"family_pair_count":family.len(),"ordinary_over_twist":range_pairs(&family,"ordinary_over_twist"),"map_over_scalar":range_pairs(&evaluation,"map_over_scalar"),"regression_matched_groups":regression.len(),"cold_over_reference":bounds(&gaps),"best_observed_per_target_replicate":best,"compression_pairs":compression,"family_pairs":family,"evaluation_pairs":evaluation,"regression":regression,"rows":data,"unit":"single-event ROI guest instructions Ir","s_ir_definition":"Ir/sqrt(r), no GAE conversion or random-map constant","scope":"Cold software interval with per-target setup; no clock, primitive GAE, fastest-global, asymptotic, novelty or NIST claim"});
    put(out.join("SUMMARY.json"), &summary)?;
    let mut csv = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("ROWS.csv"))?;
    let fields = [
        "index",
        "task_id",
        "cohort",
        "fixture",
        "p",
        "r",
        "target_label",
        "replicate",
        "method",
        "family",
        "B",
        "shift",
        "alpha",
        "evaluation",
        "compression",
        "queries",
        "status",
        "total_ir",
        "setup_ir",
        "online_ir",
        "linear_algebra_ir",
        "s_ir",
        "cold_over_strongest_completed_reference",
    ];
    writeln!(csv, "{}", fields.join(","))?;
    for d in summary["rows"].as_array().unwrap() {
        writeln!(
            csv,
            "{}",
            fields
                .iter()
                .map(|f| match &d[*f] {
                    Value::String(s) => format!("\"{}\"", s.replace('"', "\"\"")),
                    x => x.to_string(),
                })
                .collect::<Vec<_>>()
                .join(",")
        )?;
    }
    let protocol = read(wave.join("protocol.json"))?;
    let mut host = read(wave.join("HOST.json"))?;
    if host["summary"].get("memory_bytes").is_none() {
        host["summary"]["memory_bytes"] = Value::Null;
    }
    let relative_audit = audit.strip_prefix(wave)?.to_str().unwrap();
    let evidence = vec![
        file(wave, "PLAN.json", "other")?,
        file(wave, "runs/campaign-v1/ROWS.jsonl", "raw")?,
        file(wave, &format!("{relative_audit}/AUDIT.json"), "analysis")?,
        file(wave, &format!("{relative_audit}/ADMISSION.json"), "receipt")?,
    ];
    let mut runs = vec![];
    for (i, r) in rows.iter().enumerate() {
        let c = &cells[i];
        let complete = r["complete_attempt_cost"] == true;
        let is_verified = complete;
        let status = if complete {
            "succeeded"
        } else if r["status"] == "timeout" {
            "timeout"
        } else if r["status"] == "not-started-global-cap" {
            "cancelled"
        } else {
            "failed"
        };
        runs.push(json!({"run_id":r["task_id"],"arm":if c["cohort"]=="AA"{"positive"}else{"profiled"},"instance_id":format!("{}-Q{}",c["task"]["fixture"]["id"].as_str().unwrap(),c["task"]["target_label"]),"seed":c["task"]["seed"],"replicate":c["replicate"],"warmup":false,"status":status,"outcome_class":if complete{"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},"measurements":{"counted":if complete{json!({"Ir":r["cost"]["total_ir"]})}else{json!({})},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":r["max_sampled_tree_rss_bytes"],"counters":{"successful_DLP":if successful(r){1}else{0},"complete_attempt":if complete{1}else{0}}},"isolation":{"tier":"L0","contended":null,"blockers":["Counted software ROI; wall clock not admitted."]},"verification":{"status":if is_verified{"verified"}else{"no_claim"},"verifier":"Independent raw Ir parser and strict GP group/map/full-fiber replay; verified completed attempt does not imply successful DLP."},"exclusion":{"excluded":false,"rule":null},"artifacts":evidence,"started_at":null,"finished_at":null,"worker":r["worker_sha256"],"backend_ref":format!("runs/campaign-v1/cell-{i:05}")}));
    }
    let attempted = summary["attempted"].clone();
    let completed = rows
        .iter()
        .filter(|r| r["complete_attempt_cost"] == true)
        .count();
    put(
        out.join("experiment.export.template.json"),
        &json!({"schema":"experiment.export/v1","export_id":format!("{}-all-profiles",protocol["protocol_id"].as_str().unwrap()),"created":receipt["finished"],"protocol":{"path":"protocol.json","sha256":sha(wave.join("protocol.json"))?,"protocol_id":protocol["protocol_id"],"version":protocol["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"twist-cold-20261029","role":"research-and-implementation"},"source":{"repo":"crypto","commit":protocol["arms"][0]["instrument"]["commit"],"dirty":false},"backend":"local","host":{"path":"HOST.json","sha256":sha(wave.join("HOST.json"))?,"class_id":null,"summary":host["summary"]},"arms":protocol["arms"].as_array().unwrap().iter().map(|x|json!({"id":x["id"],"role":x["role"],"binary_sha256":x["instrument"]["binary_sha256"],"commit":x["instrument"]["commit"]})).collect::<Vec<_>>(),"runs":runs,"flow":{"planned":cells.len(),"attempted":attempted,"completed":completed,"not_completed":cells.len()-completed,"interrupted":0,"resumed":0,"excluded":[]},"files":[file(wave,"FROZEN.json","receipt")?,file(wave,"protocol.json","protocol")?],"analysis":{"script":{"path":"../runner/src/bin/twist-cold-summary-v2.rs","sha256":sha("research/twist_routes_1_3_20261021/runner/src/bin/twist-cold-summary-v2.rs")?,"bytes":fs::metadata("research/twist_routes_1_3_20261021/runner/src/bin/twist-cold-summary-v2.rs")?.len(),"kind":"analysis"},"blinded":false,"blinding_key_sha256":null,"outputs":[],"unmasked_at":null},"notes":[{"kind":"observation","text":"All planned outcomes retained. Successful process/replayed fiber can be ambiguous or unsupported: successful_DLP counter distinguishes scalar recovery. Replicates are fresh instrument executions, not independent targets.","by":"native frozen summary"}],"reviews_dir":"reviews"}),
    )?;
    println!(
        "{}",
        json!({"planned":cells.len(),"compression_pairs":summary["compression_pair_count"],"ordinary_over_twist":summary["ordinary_over_twist"],"compression_total_ratio":summary["compression_total_ratio"],"resources":summary["resource_elapsed_s_sizing_only"]})
    );
    Ok(())
}

#[cfg(test)]
mod correction_tests {
    use super::*;
    #[test]
    fn missing_query_hashes_only_match_declared_pre_query_failures() {
        let a = json!({"status":"underfilled","complete_attempt_cost":true,"base_sha256":"base-A","known_rows_sha256":"known-A"});
        assert!(matched_pre_query_failure(&a, &a));
        let mut b = a.clone();
        b["known_rows_sha256"] = json!("known-B");
        assert!(!matched_pre_query_failure(&a, &b));
        let mut c = a.clone();
        c["status"] = json!("recovered");
        assert!(!matched_pre_query_failure(&c, &c));
        assert!(!equal_problem(&c, &c));
        let mut d = a.clone();
        d["query_rows_sha256"] = json!("rows-A");
        assert!(!matched_pre_query_failure(&d, &d));
    }
}
