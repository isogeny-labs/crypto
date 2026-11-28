//! Producer readback of the frozen native JSONL; not an independent verifier.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs::OpenOptions, io::Write};
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [input, output] = args.as_slice() else {
        return Err("usage: RAW_JSONL NEW_SUMMARY_JSON".into());
    };
    let raw = std::fs::read(input).map_err(|e| e.to_string())?;
    let mut records = 0;
    let mut failures = vec![];
    let mut degrees = vec![];
    let mut full_witnesses = 0;
    let mut supported = 0;
    let mut identity_targets = 0;
    let mut refusals = BTreeMap::<String, u64>::new();
    let mut domain = Value::Null;
    let mut mutations = Value::Null;
    let mut actual = vec![];
    let mut complete = false;
    for line in raw.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
        records += 1;
        let v: Value = serde_json::from_slice(line).map_err(|e| e.to_string())?;
        let name = v["case"].as_str().ok_or("missing case")?;
        if v["passed"] == false {
            failures.push(v.clone());
        }
        if name.starts_with("formal-degree-k") {
            degrees.push(json!({"case":name,"k":v["value"]["k"],"E_degree":v["value"]["E_degree"],"X5_T_degree":v["value"]["X5_T_degree"],"remainder_sha256":v["value"]["remainder_sha256"]}));
        }
        if name.starts_with("toy-signed-support-") {
            supported += 1;
            if v["value"]["R"].is_null() {
                identity_targets += 1;
            }
            if v["value"]["original"]["original_replay_checked"] != true
                || v["value"]["new_predicate"]["accept"] != true
            {
                return Err("supported row lacks original replay".into());
            }
        }
        if name == "toy-signed-support" {
            *refusals
                .entry(
                    v["refusal"]
                        .as_str()
                        .ok_or("unlabelled refusal")?
                        .to_owned(),
                )
                .or_default() += 1;
        }
        if name == "toy-domain-summary" {
            domain = v.clone();
        }
        if name == "coefficient-pencil-101" {
            let arms = v["value"]["arms"].as_array().ok_or("missing pencil arms")?;
            let accepted = arms.iter().filter(|a| a["new"]["accept"] == true).count();
            let charts = arms
                .iter()
                .filter(|a| a.get("chart_refusal").is_some())
                .count();
            mutations = json!({"declared":101,"observed":arms.len(),"accepted_with_original_replay":accepted,"chart_refusals":charts,"predicate_rejections":arms.len()-accepted-charts});
        }
        let original = if name.starts_with("original-actual-") {
            &v["value"]["result"]["original"]
        } else {
            &v["value"]["original"]
        };
        if let Some(w) = original.get("original_witness") {
            let bytes = serde_json::to_vec(w).map_err(|e| e.to_string())?;
            if original["original_witness_sha256"] != format!("{:x}", Sha256::digest(&bytes))
                || original["original_witness_canonical_json_bytes"] != bytes.len()
            {
                return Err("retained full witness digest mismatch".into());
            }
            full_witnesses += 1;
        }
        if name.starts_with("original-actual-") {
            actual.push(json!({"case":name,"passed":v["passed"],"row":original["row"],"original_witness_sha256":original["original_witness_sha256"],"new_predicate":v["value"]["result"]["new_predicate"]}));
        }
        if name == "completion" {
            complete = v["passed"] == true;
        }
    }
    if !complete
        || !failures.is_empty()
        || supported != 1770
        || domain["observed"] != 1792
        || degrees.len() != 8
        || actual.len() != 2
        || full_witnesses != 4
    {
        return Err("unexpected completed fixed-domain readback; inspect raw evidence".into());
    }
    let summary = json!({"schema":"section-coeff.producer-readback/v1","standing":"producer unsigned/unverified; not independent recompute or signed validation","raw_bytes":raw.len(),"raw_sha256":format!("{:x}",Sha256::digest(&raw)),"records":records,"completion":complete,"failed_records":failures,"toy_domain":domain,"supported_original_replays":supported,"identity_R_supported":identity_targets,"chart_refusals":refusals,"formal_degrees":degrees,"coefficient_pencil":mutations,"retained_full_witnesses_checked":full_witnesses,"actual_controls":actual});
    let mut out = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)
        .map_err(|e| e.to_string())?;
    out.write_all(
        serde_json::to_string_pretty(&summary)
            .map_err(|e| e.to_string())?
            .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
    out.write_all(b"\n").map_err(|e| e.to_string())?;
    println!(
        "{}",
        json!({"records":records,"supported":supported,"identity_R":identity_targets,"full_witnesses":full_witnesses,"complete":complete})
    );
    Ok(())
}
