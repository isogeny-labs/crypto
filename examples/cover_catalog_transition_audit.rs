//! Audit exact membership changes before refreshing the catalog assessment pins.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, error::Error, fs, io::Write, process::Command};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const OLD: &str = "ff48e2800f62b45c746b3f391540c07b16af4e75";
const NEW: &str = "8e4c99759fd8d9594ed207d793b07b59ebaf19f7";
const BEFORE_LEDGER_REFRESH: &str = "05bb1e403a13ca609a2a09f0635dfbe46772e27f";
fn require(ok: bool, msg: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn blob(rev: &str, path: &str) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .args(["show", &format!("{rev}:{path}")])
        .output()?;
    require(out.status.success(), "cannot read pinned Git blob")?;
    Ok(out.stdout)
}
fn models(v: &Value) -> Result<BTreeMap<String, Value>> {
    let mut result = BTreeMap::new();
    for row in v["curves"].as_array().ok_or("missing curves")? {
        let model = row["model_json"].as_str().ok_or("missing model")?;
        require(
            result.insert(sha(model.as_bytes()), row.clone()).is_none(),
            "duplicate model",
        )?;
    }
    Ok(result)
}
fn representations(v: &BTreeMap<String, Value>) -> Result<BTreeMap<String, Value>> {
    let mut result = BTreeMap::new();
    for (model, row) in v {
        for r in row["representations"]
            .as_array()
            .ok_or("missing representations")?
        {
            let key = r["ec1"].as_str().ok_or("missing EC1")?;
            let identity = json!({"model_sha256":model,"ec1":key,"curve_uid":r["curve_uid"],
                "field_sha256":r["field_sha256"],"field":r["field"],"curve":r["curve"]});
            require(
                result.insert(key.into(), identity).is_none(),
                "duplicate EC1",
            )?;
        }
    }
    Ok(result)
}
fn added(before: &BTreeMap<String, Value>, after: &BTreeMap<String, Value>) -> Vec<Value> {
    after
        .iter()
        .filter(|(key, _)| !before.contains_key(*key))
        .map(|(key, row)| json!({"key":key,"record":row}))
        .collect()
}
fn retained(before: &BTreeMap<String, Value>, after: &BTreeMap<String, Value>) -> Result<()> {
    for (key, row) in before {
        require(
            after.get(key) == Some(row),
            &format!("removed/changed identity {key}"),
        )?;
    }
    Ok(())
}
fn audit() -> Result<Value> {
    let path = "docs/curves/registry.json";
    let old_bytes = blob(OLD, path)?;
    let new_bytes = blob(NEW, path)?;
    require(
        fs::read(path)? == new_bytes,
        "current registry differs from pinned catalog",
    )?;
    let before = models(&serde_json::from_slice(&old_bytes)?)?;
    let after = models(&serde_json::from_slice(&new_bytes)?)?;
    require(
        before.len() == 2100 && after.len() == 2572,
        "catalog population mismatch",
    )?;
    for (key, row) in &before {
        let Some(current) = after.get(key) else {
            continue;
        };
        for field in ["model_json", "slug", "icv1"] {
            require(
                row[field] == current[field],
                "retained model identity changed",
            )?;
        }
    }
    let before_reps = representations(&before)?;
    let after_reps = representations(&after)?;
    require(
        before_reps.len() == 664 && after_reps.len() == 977,
        "EC1 population mismatch",
    )?;
    let common_reps: BTreeMap<_, _> = before_reps
        .iter()
        .filter(|(k, _)| after_reps.contains_key(*k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    retained(&common_reps, &after_reps)?;
    let mut blobs = Vec::new();
    for path in [
        "docs/curves/registry.json",
        "docs/curves/covers.json",
        "docs/curves/cover-links.yaml",
    ] {
        let old = blob(OLD, path)?;
        let new = blob(NEW, path)?;
        require(
            fs::read(path)? == new,
            "current catalog file differs from pinned commit",
        )?;
        blobs.push(json!({"path":path,"old_sha256":sha(&old),"new_sha256":sha(&new)}));
    }
    let ledger_path = "docs/curves/cover-transfer-assessments.json";
    let old_ledger: Value = serde_json::from_slice(&blob(BEFORE_LEDGER_REFRESH, ledger_path)?)?;
    let ledger: Value = serde_json::from_slice(&fs::read(ledger_path)?)?;
    let mut changes = Vec::new();
    let mut digests = BTreeMap::new();
    for section in [
        "claim_contract",
        "result_templates",
        "profiles",
        "controls",
        "open_obligations",
        "global_cost_accounting",
    ] {
        digests.insert(
            section,
            sha(serde_json::to_string(&ledger[section])?.as_bytes()),
        );
        if old_ledger[section] != ledger[section] {
            changes.push(
                json!({"section":section,"before":old_ledger[section],"after":ledger[section]}),
            );
        }
    }
    for section in ["claim_contract", "profiles", "global_cost_accounting"] {
        require(
            old_ledger[section] == ledger[section],
            "claim, profile or cost contract changed",
        )?;
    }
    let old_templates = old_ledger["result_templates"]
        .as_object()
        .ok_or("missing old templates")?;
    let templates = ledger["result_templates"]
        .as_object()
        .ok_or("missing templates")?;
    require(
        old_templates.len() == templates.len(),
        "template keys changed",
    )?;
    for (key, old) in old_templates {
        let new = templates.get(key).ok_or("template removed")?;
        require(old["status"] == new["status"], "template status changed")?;
    }
    Ok(
        json!({"schema":"cover-catalog-transition/v1","status":"PASS_PINNED_TRANSITION_INVENTORY",
        "old_commit":OLD,"new_commit":NEW,"catalog_blobs":blobs,
        "old_models":before.len(),"new_models":after.len(),"retained_models":before.keys().filter(|k|after.contains_key(*k)).count(),
        "old_ec1":before_reps.len(),"new_ec1":after_reps.len(),"retained_ec1":common_reps.len(),
        "new_ec1_bearing_models":after.values().filter(|r|!r["representations"].as_array().unwrap().is_empty()).count(),
        "new_multiple_ec1_models":after.values().filter(|r|r["representations"].as_array().unwrap().len()>1).map(|r|json!({"slug":r["slug"],"representations":r["representations"].as_array().unwrap().len()})).collect::<Vec<_>>(),
        "added_models":added(&before,&after),"added_ec1":added(&before_reps,&after_reps),
        "removed_models":added(&after,&before),"removed_ec1":added(&after_reps,&before_reps),
        "semantic_digests":digests,"semantic_changes_for_review":changes,
        "scope":"Exact catalog transition and unchanged template statuses. Arithmetic, all-element transfer and performance are separate gates."}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    require(
        args.len() == 2,
        "usage: cover_catalog_transition_audit NEW_OUTPUT",
    )?;
    let result = audit()?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[1])?;
    serde_json::to_writer_pretty(&mut file, &result)?;
    file.write_all(b"\n")?;
    println!("PASS pinned inventory: {} retained models, {} removed models, {} added models; {} retained EC1 identities, {} removed, {} added", result["retained_models"],result["removed_models"].as_array().unwrap().len(),result["added_models"].as_array().unwrap().len(),result["retained_ec1"],result["removed_ec1"].as_array().unwrap().len(),result["added_ec1"].as_array().unwrap().len());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_changed_or_removed_retained_identity() {
        let before = BTreeMap::from([("EC1".into(), json!({"field":"f2m83","generator":[1,2]}))]);
        let mut after = before.clone();
        assert!(retained(&before, &after).is_ok());
        after.get_mut("EC1").unwrap()["generator"] = json!([1, 3]);
        assert!(retained(&before, &after).is_err());
        after.clear();
        assert!(retained(&before, &after).is_err());
    }
    #[test]
    fn rejects_duplicate_models_even_with_different_labels() {
        assert!(models(
            &json!({"curves":[{"model_json":"{}","slug":"a"},{"model_json":"{}","slug":"b"}]})
        )
        .is_err());
    }
}
