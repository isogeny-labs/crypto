//! Lossless schema export and descriptive stratification of already-audited data.
//! Does not rerun, select, reprice or alter primary measurements.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
static EXPORT_BASE: OnceLock<PathBuf> = OnceLock::new();
fn reference(p: &Path) -> Result<PathBuf> {
    let absolute = fs::canonicalize(p)?;
    let base = EXPORT_BASE.get().ok_or("export base not initialized")?;
    let a = absolute.components().collect::<Vec<_>>();
    let b = base.components().collect::<Vec<_>>();
    let common = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let mut relative = PathBuf::new();
    for _ in common..b.len() {
        relative.push("..");
    }
    for part in &a[common..] {
        relative.push(part.as_os_str());
    }
    Ok(relative)
}
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(v)? + "\n")?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn file(p: impl AsRef<Path>, kind: &str) -> Result<Value> {
    let p = p.as_ref();
    Ok(json!({"path":reference(p)?,"sha256":sha(p)?,"bytes":fs::metadata(p)?.len(),"kind":kind}))
}
fn date(t: u64) -> String {
    let z = (t / 86400) as i64 + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    let s = t % 86400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        s / 3600,
        s / 60 % 60,
        s % 60
    )
}
fn base(protocol: &Value, path: &Path, frozen: &Value, host: &Value, id: &str) -> Value {
    json!({"schema":"experiment.export/v1","export_id":id,"created":String::from_utf8(Command::new("date").args(["-u","+%Y-%m-%dT%H:%M:%SZ"]).output().unwrap().stdout).unwrap().trim(),
        "protocol":{"path":reference(path).unwrap(),"sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},
        "producer":{"runtime":"codex","model":"GPT-6","session":"twist-progression-20261027","role":"research-and-implementation"},
        "source":{"repo":"crypto","commit":frozen["source_revision"],"dirty":false},"backend":"local","host":host,
        "arms":protocol["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect::<Vec<_>>(),
        "runs":[],"flow":{},"files":[],"analysis":{"script":null,"blinded":false,"blinding_key_sha256":null,"outputs":[],"unmasked_at":null},
        "notes":[{"kind":"observation","text":"Exact pinned scientific source; the worktree also contains subsequent metadata and this exporter. No native wall-time performance is admitted. Independent reviewer/host replay is absent.","by":"native exporter"}],"reviews_dir":"reviews"})
}
fn phase_exports(v: &Path, out: &Path, frozen: &Value, host: &Value) -> Result<()> {
    let archive = vec![
        file(v.join("runs.tar.xz"), "raw")?,
        file(v.join("RAW_MEMBERS.json"), "receipt")?,
    ];
    let audit_path = v.join("geometry-audit-v1/AUDIT.json");
    let audit = read(&audit_path)?;
    if audit["status"] != "independent-constructions-and-all-finite-query-counts-verified" {
        return Err("exact geometry audit missing".into());
    }
    let p = v.join("geometry.protocol.json");
    let protocol = read(&p)?;
    let mut geometry = base(
        &protocol,
        &p,
        frozen,
        host,
        "twist-progression-geometry-20261027-v1",
    );
    // One producer process emits 240 constructions and all 7,680 query records.
    // The independently executed audit verifies its output, not 240 host runs.
    let mut row = json!({"run_id":"geometry-census-000","arm":"producer","instance_id":"all-240-bases-7680-queries","seed":202610260001u64,
        "replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,
        "checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},
        "measurements":{"counted":{},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,
            "counters":{"construction_records":audit["construction_records"],"query_records":audit["query_records"],"filled_bases":audit["filled_bases"]}},
        "isolation":{"tier":"L0","contended":null,"blockers":["Exact finite census; no performance claim."]},
        "verification":{"status":"verified","verifier":"independent square table, Euclidean affine group arithmetic and complete signed-tuple census"},
        "exclusion":{"excluded":false,"rule":null},"artifacts":archive,"started_at":null,"finished_at":null,
        "worker":frozen["binaries"]["worker"]["sha256"],"backend_ref":"runs/geometry-v1"});
    for (key, name) in [
        ("started_at", "started.txt"),
        ("finished_at", "finished.txt"),
    ] {
        row[key] =
            json!(fs::read_to_string(v.join("runs/geometry-transport-v1").join(name))?.trim());
    }
    geometry["runs"] = json!([row]);
    geometry["flow"] = json!({"planned":1,"attempted":1,"completed":1,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]});
    geometry["files"] = json!([file(&p, "protocol")?, file(&audit_path, "analysis")?]);
    geometry["analysis"]["outputs"] = json!([file(&audit_path, "analysis")?]);
    let analysis_source =
        Path::new("research/twist_routes_1_3_20261021/runner/src/bin/twist-progression.rs");
    geometry["analysis"]["script"] = file(analysis_source, "analysis")?;
    write(out.join("geometry.export.json"), &geometry)?;

    let p = v.join("preflight.protocol.json");
    let protocol = read(&p)?;
    let mut preflight = base(
        &protocol,
        &p,
        frozen,
        host,
        "twist-progression-preflight-20261027-v1",
    );
    let raw = read(v.join("runs/preflight-v1/RECORDS.json"))?;
    let rows = raw.as_array().ok_or("preflight rows absent")?;
    if rows.len() != 60 {
        return Err("all 60 preflight attempts required".into());
    }
    let mut runs = vec![];
    for r in rows {
        let ok = r["status"] == "complete-awaiting-independent-audit";
        runs.push(json!({"run_id":format!("preflight-{:03}",r["index"].as_u64().unwrap()),"arm":r["arm"],"instance_id":r["workload"],"seed":0,"replicate":0,"warmup":false,
            "status":if ok{"succeeded"}else if r["status"].as_str().unwrap_or("").contains("budget"){"timeout"}else{"failed"},
            "outcome_class":if !ok && r["status"].as_str().unwrap_or("").contains("budget"){"not_completed"}else{"completed"},"attempts":1,"resumed":0,
            "checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},
            "measurements":{"counted":{},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,
                "counters":{"operational_elapsed_seconds_not_performance":r["operational_elapsed_s_not_admitted_time"]}},
            "isolation":{"tier":"L0","contended":null,"blockers":["Operational sizing only; not admitted performance or pooled with main data."]},
            "verification":{"status":"not_run","verifier":null},
            "exclusion":{"excluded":true,"rule":"operational-preflight-not-main-performance"},"artifacts":archive,
            "started_at":r["started_epoch_s"].as_u64().map(date),"finished_at":r["finished_epoch_s"].as_u64().map(date),
            "worker":frozen["binaries"]["worker"]["sha256"],"backend_ref":r["directory"]}));
    }
    let ids = runs.iter().map(|r| r["run_id"].clone()).collect::<Vec<_>>();
    preflight["runs"] = json!(runs);
    let finished = runs
        .iter()
        .filter(|r| r["outcome_class"] == "completed")
        .count();
    preflight["flow"] = json!({"planned":60,"attempted":60,"completed":finished,"not_completed":60-finished,"interrupted":0,"resumed":0,
        "excluded":[{"rule":"operational-preflight-not-main-performance","run_ids":ids}]});
    preflight["files"] = json!([
        file(&p, "protocol")?,
        file(v.join("PREFLIGHT_RUNTIME.json"), "analysis")?
    ]);
    preflight["analysis"]["outputs"] = json!([file(v.join("PREFLIGHT_RUNTIME.json"), "analysis")?]);
    preflight["analysis"]["script"] = file(analysis_source, "analysis")?;
    write(out.join("preflight.export.json"), &preflight)
}
fn run(v: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh export directory required".into());
    }
    fs::create_dir_all(out)?;
    EXPORT_BASE
        .set(fs::canonicalize(out)?)
        .map_err(|_| "export base already initialized")?;
    let frozen = read(v.join("FROZEN.json"))?;
    let hp = v.join("HOST.json");
    let host_data = read(&hp)?;
    let host = json!({"path":reference(&hp)?,"sha256":sha(&hp)?,"class_id":null,"summary":host_data["summary"]});
    phase_exports(v, out, &frozen, &host)?;
    let admission = read(v.join("cost-audit-v1/ADMISSION.json"))?;
    if admission["status"] != "independent-finite-census-raw-Ir-and-PARI-replay-verified" {
        return Err("cost admission absent".into());
    }
    let plan = read(v.join("PLAN.json"))?;
    let raw = read(v.join("runs/cost-v1/RECORDS.json"))?;
    let checks = read(v.join("cost-audit-v1/COST_CHECKS.json"))?;
    if plan["cells"].as_array().map(Vec::len) != Some(880)
        || raw.as_array().map(Vec::len) != Some(880)
        || checks.as_array().map(Vec::len) != Some(880)
    {
        return Err("expected all 880 planned, recorded and audited cells".into());
    }
    // Raw records travel as one deterministic archive. backend_ref identifies
    // the member directory; RAW_MEMBERS records every member's hash and size.
    let archive_refs = vec![
        file(v.join("runs.tar.xz"), "raw")?,
        file(v.join("RAW_MEMBERS.json"), "receipt")?,
    ];
    let ppath = v.join("cost.protocol.json");
    let protocol = read(&ppath)?;
    let mut export = base(
        &protocol,
        &ppath,
        &frozen,
        &host,
        "twist-progression-cost-20261027-v1",
    );
    let mut runs = vec![];
    let mut excluded = BTreeMap::<String, Vec<String>>::new();
    let mut strata = BTreeMap::<(String, String, u64), (usize, usize)>::new();
    let mut attempted = 0usize;
    let mut completed = 0usize;
    let mut process_completed = 0usize;
    for ((cell, row), check) in plan["cells"]
        .as_array()
        .unwrap()
        .iter()
        .zip(raw.as_array().unwrap())
        .zip(checks.as_array().unwrap())
    {
        let index = cell["index"].as_u64().unwrap();
        let id = format!("progression-cost-{index:03}");
        let ok = check["accepted"] == true;
        let reason = check["reason"].as_str().unwrap_or("not-completed");
        let started = row["status"] != "not-started-global-cap";
        attempted += usize::from(started);
        completed += usize::from(ok);
        let group = strata
            .entry((
                cell["family"].as_str().unwrap().into(),
                cell["arm"].as_str().unwrap().into(),
                cell["arity"].as_u64().unwrap(),
            ))
            .or_default();
        group.0 += 1;
        group.1 += usize::from(ok);
        if !ok {
            excluded.entry(reason.into()).or_default().push(id.clone());
        }
        let dir = v.join("runs/cost-v1").join(format!("cell-{index:03}"));
        let mut counters = serde_json::Map::new();
        let worker = read(dir.join("worker.json")).ok();
        if let Some(w) = worker {
            for key in ["exceptional_candidates_enumerated", "invalid_signed_lifts"] {
                if w[key].is_number() {
                    counters.insert(key.into(), w[key].clone());
                }
            }
            if let Some(bs) = w["branches"].as_array() {
                for (i, b) in bs.iter().enumerate() {
                    for key in [
                        "solver_field_multiplications_row_reduction",
                        "f4_runs",
                        "max_rows",
                        "max_cols",
                        "candidates_rejected",
                    ] {
                        if b[key].is_number() {
                            counters.insert(format!("branch-{i}.{key}"), b[key].clone());
                        }
                    }
                }
            }
        }
        let mut counted = serde_json::Map::new();
        if row["Ir"].is_number() {
            counted.insert("Ir".into(), row["Ir"].clone());
        }
        let timeout = reason.contains("budget");
        process_completed += usize::from(!timeout);
        runs.push(json!({"run_id":id,"arm":cell["arm"],"instance_id":cell["workload"],"seed":0,"replicate":0,"warmup":false,
            "status":if ok{"succeeded"}else if timeout{"timeout"}else{"failed"},"outcome_class":if timeout{"not_completed"}else{"completed"},"attempts":1,"resumed":0,
            "checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},
            "measurements":{"counted":counted,"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":counters},
            "isolation":{"tier":"L0","contended":null,"blockers":["No native-time isolation admission; only simulated guest instructions are compared."]},
            "verification":{"status":if ok{"verified"}else{"not_run"},"verifier":if ok{Some("independent finite census + frozen raw-Ir audit + PARI replay")}else{None}},
            "exclusion":{"excluded":!ok,"rule":if ok{None}else{Some(reason)}},"artifacts":archive_refs,
            "started_at":row["started_epoch_s"].as_u64().map(date),"finished_at":row["finished_epoch_s"].as_u64().map(date),"worker":frozen["binaries"]["worker"]["sha256"],"backend_ref":row["directory"]}));
    }
    let unmask_path = v.join("blind-v1/UNMASKING.json");
    let unmask = read(&unmask_path)?;
    let mask_receipt = read(v.join("blind-v1/RECEIPT.json"))?;
    export["runs"] = json!(runs);
    export["flow"] = json!({"planned":880,"attempted":attempted,"completed":process_completed,"not_completed":880-process_completed,"interrupted":0,"resumed":0,
        "excluded":excluded.iter().map(|(rule,run_ids)|json!({"rule":rule,"run_ids":run_ids})).collect::<Vec<_>>()});
    let audit_source =
        PathBuf::from("research/twist_routes_1_3_20261021/runner/src/bin/twist-progression.rs");
    export["analysis"] = json!({"script":file(audit_source,"analysis")?,"blinded":true,"blinding_key_sha256":mask_receipt["key_sha256"],"outputs":[file(v.join("cost-audit-v1/AUDIT.json"),"analysis")?,file(v.join("blind-v1/MASKED_ANALYSIS.json"),"analysis")?,file(&unmask_path,"receipt")?],"unmasked_at":unmask["unmasked_epoch_s"].as_u64().map(date)});
    export["files"] = json!([
        file(v.join("FROZEN.json"), "receipt")?,
        file(v.join("cost-audit-v1/ADMISSION.json"), "receipt")?,
        file(v.join("pari-replay-v2/RECEIPT.json"), "receipt")?,
        file(&ppath, "protocol")?,
        file(&hp, "host")?
    ]);
    write(out.join("cost.export.json"), &export)?;
    write(
        out.join("STRATIFIED.json"),
        &json!({"schema":"twist.progression-descriptive-strata/v1","flow":export["flow"],"accepted_scientific_outputs":completed,
        "by_family_arm_arity":strata.into_iter().map(|((family,arm,arity),(requested,accepted))|json!({"family":family,"arm":arm,"arity":arity,"requested":requested,"accepted":accepted})).collect::<Vec<_>>(),
        "failure_reasons":excluded.iter().map(|(reason,ids)|json!({"reason":reason,"cells":ids.len()})).collect::<Vec<_>>(),
        "source_records_sha256":sha(v.join("runs/cost-v1/RECORDS.json"))?,"primary_measurements_modified":false}),
    )
}
fn main() -> Result<()> {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if a.len() != 2 {
        return Err("usage: twist-progression-export V4 FRESH-OUT".into());
    }
    run(Path::new(&a[0]), Path::new(&a[1]))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dates_are_metadata_not_duration_claims() {
        assert_eq!(date(0), "1970-01-01T00:00:00Z");
        assert_eq!(date(86401), "1970-01-02T00:00:01Z");
    }
}
