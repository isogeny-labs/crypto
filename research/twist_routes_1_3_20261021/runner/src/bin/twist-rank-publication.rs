//! Versioned native aggregation/export for the exact rank-repair panel.
#![allow(dead_code)]
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const S: &str = "research/twist_routes_1_3_20261021";
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    if p.as_ref().exists() {
        return Err("fresh publication output required".into());
    }
    fs::write(p, serde_json::to_string_pretty(v)? + "\n")?;
    Ok(())
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn file(p: &Path, base: &Path, kind: &str) -> Result<Value> {
    let a = fs::canonicalize(p)?;
    let b = fs::canonicalize(base)?;
    let aa: Vec<_> = a.components().collect();
    let bb: Vec<_> = b.components().collect();
    let n = aa.iter().zip(&bb).take_while(|(a, b)| a == b).count();
    let mut rel = PathBuf::new();
    for _ in n..bb.len() {
        rel.push("..");
    }
    for c in &aa[n..] {
        rel.push(c.as_os_str());
    }
    Ok(json!({"path":rel,"sha256":sha(p)?,"bytes":fs::metadata(p)?.len(),"kind":kind}))
}
fn aggregate() -> Result<()> {
    let v = Path::new(S).join("execution-v14");
    let a = read(v.join("ADMISSION.json"))?;
    assert_eq!(
        a["status"],
        "native-and-independent-PARI-rank-panel-verified"
    );
    let mut effects = vec![];
    let mut groups: BTreeMap<String, Value> = BTreeMap::new();
    for cfg in a["configurations"].as_array().unwrap() {
        let id = cfg["id"].as_str().unwrap();
        if cfg["status"] != "filled" {
            continue;
        }
        for mode in [2, 3, 23] {
            let cell = |on: bool| {
                a["cells"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|c| {
                        c["configuration"] == id && c["mode"] == mode && c["alpha_links"] == on
                    })
                    .unwrap()
            };
            let off = cell(false);
            let on = cell(true);
            let was = off["target_identifiable"].as_bool().unwrap();
            let now = on["target_identifiable"].as_bool().unwrap();
            assert!(
                !was || now,
                "valid added relations cannot lose identifiability"
            );
            let row = json!({"configuration":id,"kind":cfg["kind"],"transform":cfg["transform"],"B":cfg["requested_B"],"shift":cfg["shift"],"mode":mode,"off_identifiable":was,"on_identifiable":now,"newly_identifiable":!was&&now,"off_rank":off["total_rank"],"on_rank":on["total_rank"],"off_known_rank":off["known_rank"],"on_known_rank":on["known_rank"],"distinct_query_equations":off["distinct_query_equations"],"raw_witness_rows":off["raw_witness_rows"]});
            let key = format!(
                "{}:{}",
                cfg["kind"].as_str().unwrap(),
                cfg["transform"].as_str().unwrap()
            );
            let g=groups.entry(key).or_insert_with(||json!({"kind":cfg["kind"],"transform":cfg["transform"],"analyzed_pairs":0,"off_identifiable":0,"on_identifiable":0,"newly_identifiable":0}));
            for (field, increment) in [
                ("analyzed_pairs", 1),
                ("off_identifiable", usize::from(was)),
                ("on_identifiable", usize::from(now)),
                ("newly_identifiable", usize::from(!was && now)),
            ] {
                g[field] = json!(g[field].as_u64().unwrap() + increment as u64);
            }
            effects.push(row);
        }
    }
    write(
        v.join("SUMMARY.json"),
        &json!({"schema":"twist.rank-repair-summary/v1","status":a["status"],"summary":a["summary"],"configurations":a["configurations"],"cells":a["cells"],"link_effects":effects,"descriptive_groups":groups.values().collect::<Vec<_>>(),"performance_claim":false,"novelty_claim":false,"grouping_is_exact_named_fixture_only":true}),
    )
}
fn export() -> Result<()> {
    let v = Path::new(S).join("execution-v14");
    let p = read(v.join("protocol.json"))?;
    let f = read(v.join("FROZEN.json"))?;
    let a = read(v.join("ADMISSION.json"))?;
    let h = read(v.join("HOST.json"))?;
    let native = read(v.join("runs/producer-transport.json"))?;
    let pari = read(v.join("runs/pari-v1/RECEIPT.json"))?;
    assert_eq!(native["exit_code"], 0);
    assert_eq!(pari["exit_code"], 0);
    assert_eq!(native["binary_sha256"], f["binary"]["sha256"]);
    let mut e = read(Path::new(S).join("execution-v13/experiment.export.json"))?;
    e["export_id"] = json!("twist-rank-repair-20261028-v1");
    e["created"] = native["finished"].clone();
    e["protocol"] = json!({"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]});
    e["source"]["commit"] = f["source_revision"].clone();
    e["producer"]["session"] = json!("twist-rank-repair-20261028");
    e["host"] = json!({"path":"HOST.json","sha256":sha(v.join("HOST.json"))?,"class_id":null,"summary":h["summary"]});
    e["arms"]=json!(p["arms"].as_array().unwrap().iter().map(|r|json!({"id":r["id"],"role":r["role"],"binary_sha256":r["instrument"]["binary_sha256"],"commit":r["instrument"]["commit"]})).collect::<Vec<_>>());
    let artifacts = vec![
        file(&v.join("runs.tar.xz"), &v, "raw")?,
        file(&v.join("RAW_MEMBERS.json"), &v, "receipt")?,
        file(&v.join("ADMISSION.json"), &v, "receipt")?,
        file(&v.join("SUMMARY.json"), &v, "analysis")?,
        file(&v.join("runs/pari-v1/RECEIPT.json"), &v, "receipt")?,
    ];
    let mut rows = e["runs"].as_array().unwrap().clone();
    assert_eq!(rows.len(), 2);
    for (i, r) in rows.iter_mut().enumerate() {
        r["run_id"] = json!(format!(
            "rank-repair-{}-000",
            if i == 0 { "native" } else { "pari" }
        ));
        r["instance_id"] = json!("p827-fixed-42-base-panel");
        r["worker"] = p["arms"][i]["instrument"]["binary_sha256"].clone();
        r["status"] = json!("succeeded");
        r["verification"] = json!({"status":"verified","verifier":"Native complete witness/fiber census and independent PARI base/map/query/row/rank/kernel/recovery replay"});
        r["artifacts"] = json!(artifacts);
        r["measurements"]["counted"] =
            json!({"target_identifiable_systems":a["summary"]["target_identifiable_systems"]});
        r["measurements"]["counters"] = a["summary"].clone();
        r["backend_ref"] = json!(if i == 0 {
            "runs/rank-v1"
        } else {
            "runs/pari-v1"
        });
        r["started_at"] = if i == 0 {
            native["started"].clone()
        } else {
            Value::Null
        };
        r["finished_at"] = if i == 0 {
            native["finished"].clone()
        } else {
            Value::Null
        };
    }
    e["runs"] = json!(rows);
    e["analysis"] = json!({"script":file(&Path::new(S).join("runner/src/rank_repair_20261028.rs"),&v,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("SUMMARY.json"),&v,"analysis")?],"unmasked_at":null});
    e["files"] = json!([
        file(&v.join("FROZEN.json"), &v, "receipt")?,
        file(&v.join("protocol.json"), &v, "protocol")?
    ]);
    e["notes"] = json!([{"kind":"observation","text":"42 fixed target-blind base configurations; all underfills/excluded matrices and zero queries retained. Complete exact witness and affine-nullspace checks; known alpha equations given to all ordinary/twist arms. No factor-base log oracle, timed/cost/generalization/novelty claim. Development controls are separate, not pooled scientific rows.","by":"native frozen publication tool"}]);
    write(v.join("experiment.export.json"), &e)
}
fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("aggregate") => aggregate(),
        Some("export") => export(),
        _ => Err("aggregate|export".into()),
    }
}
