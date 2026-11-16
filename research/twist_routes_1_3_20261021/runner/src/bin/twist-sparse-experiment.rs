//! Prospective rectangular experiment on fresh public targets; native analysis.
#![allow(dead_code)]
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
};
#[path = "../cold_20261029/arithmetic.rs"]
mod arithmetic;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const S: &str = "research/twist_routes_1_3_20261021";
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?;
    writeln!(f, "{}", serde_json::to_string_pretty(v)?)?;
    Ok(())
}
fn digest(x: &str) -> String {
    hex::encode(source_sha256::sha256(x.as_bytes()))
}
fn n(x: &Value) -> u64 {
    x.as_u64().unwrap_or(0)
}
fn bsize(r: u64) -> usize {
    let mut k = 1usize;
    while (k as u64).pow(4) < r {
        k += 1;
    }
    ((2 * k + 7) / 8) * 8
}
fn arms() -> Vec<Value> {
    let mut a = vec![];
    for (family, prefix) in [("low-x", "ordinary"), ("twist-max", "twist")] {
        for (suffix, alpha, comp, sparse, eval) in [
            ("full", "none", false, false, "map"),
            ("dense", "augment", true, false, "map"),
            ("sparse", "augment", true, true, "map"),
            ("map", "augment", false, false, "map"),
            ("scalar", "augment", false, false, "scalar"),
        ] {
            a.push(json!({"id":format!("{prefix}-{suffix}"),"role":if prefix=="ordinary"&&suffix=="full"{"baseline"}else{"candidate"},"method":"group","family":family,"alpha":alpha,"compression":comp,"sparse":sparse,"evaluation":eval}));
        }
    }
    for method in ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"] {
        a.push(json!({"id":method,"role":"reference","method":method,"family":"reference","alpha":"none","compression":false,"sparse":false,"evaluation":"map"}));
    }
    a.push(json!({"id":"ordinary-aa","role":"control_aa","method":"group","family":"low-x","alpha":"none","compression":false,"sparse":false,"evaluation":"map"}));
    a.push(json!({"id":"positive","role":"control_positive","method":"gp-poly","family":"low-x","alpha":"none","compression":false,"sparse":false,"evaluation":"map"}));
    a
}
fn plan(input: &str, old_plan: &str, pins: &str, out: &str) -> Result<()> {
    let data = read(input)?;
    let old = read(old_plan)?;
    let mut seen = BTreeSet::new();
    for c in old["cells"].as_array().ok_or("old cells")? {
        seen.insert(serde_json::to_string(&json!([
            c["task"]["fixture"]["id"],
            c["task"]["target"]
        ]))?);
    }
    let aa = arms();
    let mut cells = vec![];
    let mut workloads = vec![];
    for (fi, fv) in data["heldout"]
        .as_array()
        .ok_or("heldout")?
        .iter()
        .enumerate()
    {
        let f: arithmetic::Fixture = serde_json::from_value(fv.clone())?;
        for label in 0..3 {
            let mut nonce = 0;
            let (mut k, mut q);
            loop {
                let d = source_sha256::sha256(
                    format!("twist-sparse-confirm-20261030-v1|{}|{label}|{nonce}", f.id).as_bytes(),
                );
                k = if label == 0 {
                    2
                } else {
                    u64::from_be_bytes(d[..8].try_into().unwrap()) % (f.r - 3) + 3
                };
                q = f.curve().mul(f.g(), k);
                let key = serde_json::to_string(&json!([f.id, q]))?;
                if label == 0 || !seen.contains(&key) {
                    seen.insert(key);
                    break;
                }
                nonce += 1;
            }
            let wid = format!(
                "{}-{}",
                f.id,
                if label == 0 {
                    "Q2-control".into()
                } else {
                    format!("fresh-{label}")
                }
            );
            let bytes = source_sha256::sha256(format!("sparse-walk-seed|{wid}").as_bytes());
            let seed = u64::from_be_bytes(bytes[..8].try_into().unwrap());
            workloads.push(json!({"id":wid,"fixture":f,"target":q,"expected_scalar":k,"class":if label==0{"Q2-control"}else{"fresh"},"nonce":nonce}));
            for arm in &aa {
                let id = format!("sparse-{}", &digest(&format!("{}|{wid}", arm["id"]))[..24]);
                let isref = arm["role"] == "reference";
                let positive = arm["id"] == "positive";
                let first = positive && label == 0 && fi < 6;
                let b = if isref {
                    0
                } else if positive {
                    8
                } else {
                    bsize(f.r)
                };
                let task = json!({"id":id,"fixture":f,"target":q,"target_label":label,"method":arm["method"],"family":arm["family"],"b":b,"shift":1,"alpha":arm["alpha"],"evaluation":arm["evaluation"],"compression":arm["compression"],"queries":if isref{0}else if positive{16}else{8*b},"seed":seed});
                cells.push(json!({"index":0,"task":task,"replicate":0,"cohort":if first{"AA"}else if positive{"positive"}else if isref{"confirmation-reference"}else if arm["id"]=="ordinary-aa"{"AA-control"}else{"confirmation"},"expected_scalar":k,"aa_group":if first{json!("known-Q2-GP-positive")}else{Value::Null},"timeout_s":180,"arm":arm["id"],"workload_id":wid,"target_class":if label==0{"Q2-control"}else{"fresh"},"sparse_compression":arm["sparse"]}));
            }
        }
    }
    cells.sort_by_key(|c| {
        (
            c["cohort"] != "AA",
            digest(c["task"]["id"].as_str().unwrap()),
        )
    });
    for (i, c) in cells.iter_mut().enumerate() {
        c["index"] = json!(i);
    }
    assert_eq!(workloads.len(), 36);
    assert_eq!(cells.len(), 612);
    write(
        out,
        &json!({"schema":"twist.cold-plan/v1","kind":"sparse-confirmation","source_pins":pins,"source_pins_sha256":sha(pins)?,"fixture_manifest":input,"fixture_manifest_sha256":sha(input)?,"previous_plan":old_plan,"previous_plan_sha256":sha(old_plan)?,"planned_cells":cells.len(),"cells":cells,"workloads":workloads,"arms":aa,"replicates":1,"unit":"single-event user-space guest instructions Ir","cold_target_count":1,"memory_cap_bytes":4294967296u64,"disk_cap_bytes":500000000u64,"global_budget_s":3600,"no_new_target_outcomes_used_to_choose_cases":true,"original_E28_final_labels_opened":false}),
    )
}
fn complete(r: &Value) -> bool {
    r["complete_cost"] == true && r["scalar_verified"] == true && r["status"] == "recovered"
}
const HASHES: [&str; 5] = [
    "base_sha256",
    "known_rows_sha256",
    "query_targets_sha256",
    "witnesses_sha256",
    "query_rows_sha256",
];
fn same(a: &Value, b: &Value) -> bool {
    a["status"] == b["status"] && HASHES.iter().all(|h| !a[*h].is_null() && a[*h] == b[*h])
}
fn prefix(a: &Value, b: &Value) -> bool {
    (a["status"] == "underfilled" || a["status"] == "unsupported-alpha")
        && a["status"] == b["status"]
        && HASHES[..2]
            .iter()
            .all(|h| !a[*h].is_null() && a[*h] == b[*h])
        && HASHES[2..]
            .iter()
            .all(|h| a[*h].is_null() && b[*h].is_null())
}
fn phase(r: &Value, name: &str) -> u64 {
    r["cost"]["exclusive_phases"].as_array().map_or(0, |v| {
        v.iter()
            .filter(|x| x["phase"] == name)
            .map(|x| n(&x["total_ir"]))
            .sum()
    })
}
fn range(v: &[f64]) -> Value {
    if v.is_empty() {
        Value::Null
    } else {
        let mut z = v.to_vec();
        z.sort_by(f64::total_cmp);
        json!({"n":z.len(),"min":z[0],"median":z[z.len()/2],"max":z[z.len()-1],"interval":"observed case range, no confidence inference"})
    }
}
fn file(d: &Path, p: &str, k: &str) -> Result<Value> {
    let x = d.join(p);
    Ok(json!({"path":p,"sha256":sha(&x)?,"bytes":fs::metadata(x)?.len(),"kind":k}))
}
fn summary(wave: &str, audit: &str, out: &str) -> Result<()> {
    let d = Path::new(wave);
    let out = Path::new(out);
    if out.exists() {
        return Err("fresh summary required".into());
    }
    fs::create_dir_all(out)?;
    let p = read(d.join("PLAN.json"))?;
    let protocol = read(d.join("protocol.json"))?;
    let admission = read(Path::new(audit).join("ADMISSION.json"))?;
    if admission["status"] != "native-counters-and-independent-GP-verified" {
        return Err("independent admission required".into());
    }
    let text = fs::read_to_string(d.join("runs/campaign-v1/ROWS.jsonl"))?;
    let rows: Vec<Value> = text
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    let cells = p["cells"].as_array().unwrap();
    if rows.len() != cells.len() {
        return Err("all rows required".into());
    }
    let mut by = BTreeMap::new();
    let mut statuses = BTreeMap::<String, u64>::new();
    let mut plotrows = vec![];
    let mut pairs = vec![];
    let mut exports = vec![];
    for (i, r) in rows.iter().enumerate() {
        let c = &cells[i];
        if n(&r["index"]) != i as u64
            || r["task_id"] != c["task"]["id"]
            || r["arm"] != c["arm"]
            || r["sparse_compression"] != c["sparse_compression"]
        {
            return Err("source/cell/mode identity".into());
        }
        by.insert(
            (
                c["workload_id"].as_str().unwrap().to_string(),
                c["arm"].as_str().unwrap().to_string(),
            ),
            i,
        );
        *statuses
            .entry(format!(
                "{}|{}",
                c["arm"].as_str().unwrap(),
                r["status"].as_str().unwrap()
            ))
            .or_default() += 1;
        if c["arm"] == "positive" && c["target_class"] == "Q2-control" && !complete(r) {
            return Err("positive Q2 detector failed".into());
        }
        plotrows.push(json!({"index":i,"workload":c["workload_id"],"class":c["target_class"],"arm":c["arm"],"fixture":c["task"]["fixture"]["id"],"r":c["task"]["fixture"]["r"],"status":r["status"],"total_ir":r["cost"]["total_ir"],"setup_ir":r["cost"]["setup_ir"],"linear_algebra_ir":phase(r,"linear-algebra"),"complete_DLP":complete(r)}));
        let mut row = json!({"run_id":r["task_id"],"arm":c["arm"],"instance_id":c["workload_id"],"seed":c["task"]["seed"].as_u64().unwrap().to_string(),"replicate":0,"warmup":false,"status":if r["complete_attempt_cost"]==true{"succeeded"}else if r["status"]=="timeout"{"timeout"}else{"failed"},"outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},"measurements":{"counted":{},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":r["max_sampled_tree_rss_bytes"],"counters":{"successful_DLP":if complete(r){1}else{0}}},"isolation":{"tier":"L0","contended":null,"blockers":["Guest instruction diagnostic only, no admitted wall time."]},"verification":{"status":"verified","verifier":"Independent raw trace parser, strict GP group/map/full-fiber/source-scalar replay; unsuccessful DLP statuses preserved."},"exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":r["worker_sha256"],"backend_ref":format!("runs/campaign-v1/cell-{i:05}")});
        if r["complete_attempt_cost"] == true {
            row["measurements"]["counted"]["cold-Ir"] = r["cost"]["total_ir"].clone();
        } else {
            row["verification"]["status"] = json!("no_claim");
            if row["status"] == "timeout" {
                row["outcome_class"] = json!("not_completed");
            }
        }
        exports.push(row);
    }
    let mut comparisons: Vec<(String, String, String)> = vec![
        (
            "progression".into(),
            "ordinary-full".into(),
            "twist-full".into(),
        ),
        ("aa".into(), "ordinary-full".into(), "ordinary-aa".into()),
    ];
    for family in ["ordinary", "twist"] {
        for (id, a, b) in [
            ("sparse-over-dense", "dense", "sparse"),
            ("sparse-over-full", "map", "sparse"),
            ("map-over-scalar", "scalar", "map"),
        ] {
            comparisons.push((
                format!("{family}-{id}"),
                format!("{family}-{a}"),
                format!("{family}-{b}"),
            ));
        }
    }
    for w in p["workloads"].as_array().unwrap() {
        let wid = w["id"].as_str().unwrap();
        for (id, a, b) in &comparisons {
            let ia = by[&(wid.into(), a.clone())];
            let ib = by[&(wid.into(), b.clone())];
            let ra = &rows[ia];
            let rb = &rows[ib];
            if id != "progression" && !(same(ra, rb) || prefix(ra, rb)) {
                return Err(format!("paired problem mismatch {id}:{wid}").into());
            }
            let ok = complete(ra) && complete(rb);
            let den = n(&ra["cost"]["total_ir"]);
            pairs.push(json!({"comparison":id,"workload":wid,"class":w["class"],"a":ia,"b":ib,"a_status":ra["status"],"b_status":rb["status"],"both_recovered":ok,"b_over_a":if ok{json!(n(&rb["cost"]["total_ir"])as f64/den as f64)}else{Value::Null},"matrix_b_over_a":if ok&&phase(ra,"linear-algebra")>0{json!(phase(rb,"linear-algebra")as f64/phase(ra,"linear-algebra")as f64)}else{Value::Null}}));
        }
    }
    let mut ratios = BTreeMap::new();
    for (id, _, _) in &comparisons {
        let eligible: Vec<_> = pairs
            .iter()
            .filter(|x| x["comparison"] == id.as_str() && x["class"] == "fresh")
            .collect();
        ratios.insert(id.clone(),json!({"cold":range(&eligible.iter().filter_map(|x|x["b_over_a"].as_f64()).collect::<Vec<_>>()),"matrix":range(&eligible.iter().filter_map(|x|x["matrix_b_over_a"].as_f64()).collect::<Vec<_>>()),"all_fresh_planned_pairs":eligible.len()}));
    }
    let mut gaps = BTreeMap::<String, Vec<f64>>::new();
    for (i, z) in plotrows.iter_mut().enumerate() {
        let c = &cells[i];
        let wid = c["workload_id"].as_str().unwrap();
        let best = ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"]
            .iter()
            .map(|a| by[&(wid.to_string(), a.to_string())])
            .filter(|j| complete(&rows[*j]))
            .min_by_key(|j| n(&rows[*j]["cost"]["total_ir"]));
        if let Some(j) = best {
            if complete(&rows[i]) {
                let ratio =
                    n(&rows[i]["cost"]["total_ir"]) as f64 / n(&rows[j]["cost"]["total_ir"]) as f64;
                z["strongest_completed_reference_row"] = json!(j);
                z["cold_over_reference"] = json!(ratio);
                if c["target_class"] == "fresh" {
                    gaps.entry(c["arm"].as_str().unwrap().to_string())
                        .or_default()
                        .push(ratio);
                }
            }
        }
    }
    let gaps: BTreeMap<_, _> = gaps.iter().map(|(a, v)| (a, range(v))).collect();
    write(
        out.join("SUMMARY.json"),
        &json!({"schema":"twist.sparse-summary/v1","planned_profiles":cells.len(),"workloads":36,"fresh_public_DLP_targets":24,"Q2_controls":12,"status_by_arm":statuses,"fresh_target_ratios":ratios,"fresh_target_reference_gaps":gaps,"all_pairs":pairs,"rows":plotrows,"plan_sha256":sha(d.join("PLAN.json"))?,"ledger_sha256":sha(d.join("runs/campaign-v1/ROWS.jsonl"))?,"independent_admission_sha256":sha(Path::new(audit).join("ADMISSION.json"))?,"scope":"Finite software Ir; no timing, GAE, novelty, asymptotic or NIST claim."}),
    )?;
    let mut host = read(d.join("HOST.json"))?;
    host["summary"]["memory_bytes"] = host["summary"]["memory_bytes"].clone();
    let transport = read(d.join("runs/TRANSPORT.json"))?;
    let complete = exports
        .iter()
        .filter(|r| r["outcome_class"] == "completed")
        .count();
    let rel = out.strip_prefix(d)?.to_str().unwrap();
    write(
        d.join("experiment.export.json"),
        &json!({"schema":"experiment.export/v1","export_id":"twist-sparse-confirmation-20261030","created":transport["finished"],"protocol":{"path":"protocol.json","sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"twist-sparse-20261030","role":"research-and-implementation"},"source":{"repo":"crypto","commit":protocol["arms"][0]["instrument"]["commit"],"dirty":false},"backend":"local","host":{"path":"HOST.json","sha256":sha(d.join("HOST.json"))?,"class_id":null,"summary":host["summary"]},"arms":protocol["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect::<Vec<_>>(),"runs":exports,"flow":{"planned":cells.len(),"attempted":cells.len(),"completed":complete,"not_completed":cells.len()-complete,"interrupted":0,"resumed":0,"excluded":[]},"files":[file(d,"PLAN.json","other")?,file(d,"FROZEN.json","receipt")?,file(d,"runs/campaign-v1/ROWS.jsonl","raw")?,file(d,"audit-v1/ADMISSION.json","receipt")?],"analysis":{"script":file(d,"../runner/src/bin/twist-sparse-experiment.rs","analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(d,&format!("{rel}/SUMMARY.json"),"analysis")?],"unmasked_at":null},"notes":[{"kind":"observation","by":"native analysis","text":"Exactly36 matched public target workloads across17 arms.24 new target points and12 Q=2G controls on previously used12 models. One fresh process per arm/workload; no population confidence, held-out-model or validated GAE benchmark claim. Independent verifier processes are linked certificates, not extra performance observations. All failed/unsupported/underfilled DLPs retained."}],"reviews_dir":"reviews"}),
    )
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("arms") if a.len() == 2 => write(&a[1], &json!(arms())),
        Some("plan") if a.len() == 5 => plan(&a[1], &a[2], &a[3], &a[4]),
        Some("summary") if a.len() == 4 => summary(&a[1], &a[2], &a[3]),
        _ => Err("plan FIXTURES OLD_PLAN PINS FRESH_PLAN | summary WAVE AUDIT FRESH_OUT".into()),
    }
}
