//! Read and verify persisted proof objects without rerunning the producer.
use crate::{independent, kernel::Certificate, source_sha256};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

fn integer(v: &Value, name: &str) -> Result<u64, String> {
    v.get(name)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing integer {name}"))
}
fn roots(v: &Value, name: &str) -> Result<Vec<u64>, String> {
    let values = v
        .get(name)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing roots {name}"))?;
    values
        .iter()
        .map(|z| z.as_u64().ok_or_else(|| format!("invalid root in {name}")))
        .collect()
}
fn string<'a>(v: &'a Value, name: &str) -> Result<&'a str, String> {
    v.get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {name}"))
}
fn sha256(bytes: &[u8]) -> String {
    source_sha256::sha256(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// The optional raw JSONL binds proof bytes to every declared persisted case.
/// Without it, the objects are mathematically checked but are not asserted to
/// be the objects named by a producer record. Sidecars are input-checked;
/// the producer raw record does not separately hash their bytes.
pub fn check_directory(path: &Path, raw_jsonl: Option<&Path>) -> Result<Value, String> {
    let mut records = BTreeMap::new();
    let mut raw_count = 0usize;
    let mut raw_digest = None;
    if let Some(raw) = raw_jsonl {
        let bytes = fs::read(raw).map_err(|e| format!("read raw records: {e}"))?;
        raw_digest = Some(sha256(&bytes));
        let text = std::str::from_utf8(&bytes).map_err(|e| format!("raw UTF8: {e}"))?;
        let mut ids = BTreeSet::new();
        for (line, row) in text.lines().enumerate() {
            if row.trim().is_empty() {
                continue;
            }
            let value: Value =
                serde_json::from_str(row).map_err(|e| format!("raw line {}: {e}", line + 1))?;
            if value.get("kind").and_then(Value::as_str) != Some("case") {
                continue;
            }
            let id = string(&value, "id")?.to_owned();
            if !ids.insert(id.clone()) {
                return Err(format!("duplicate raw case: {id}"));
            }
            raw_count += 1;
            match value.get("persist").and_then(Value::as_bool) {
                Some(true) => {
                    records.insert(id, value);
                }
                Some(false) => {}
                None => return Err("raw case lacks explicit persist field".into()),
            }
        }
    }
    let mut paths = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| format!("read proof directory: {e}"))? {
        let entry = entry.map_err(|e| format!("proof directory entry: {e}"))?;
        let filename = entry
            .file_name()
            .to_str()
            .ok_or("non-UTF8 proof filename")?
            .to_owned();
        if filename.ends_with(".certificate.json") {
            if !entry
                .file_type()
                .map_err(|e| format!("proof file type: {e}"))?
                .is_file()
            {
                return Err("certificate must be an ordinary file".into());
            }
            paths.push((filename, entry.path()));
        }
    }
    paths.sort_by(|a, b| a.0.cmp(&b.0));
    if paths.is_empty() {
        return Err("no persisted certificates".into());
    }
    let mut checked_ids = BTreeSet::new();
    let mut checks = Vec::new();
    let mut identities = 0usize;
    let mut grid_cells = 0usize;
    let mut s3_inputs = 0usize;
    for (filename, file) in paths {
        let id = filename.strip_suffix(".certificate.json").unwrap();
        let bytes = fs::read(&file).map_err(|e| format!("read certificate {id}: {e}"))?;
        if bytes.len() > 65536 {
            return Err(format!(
                "certificate {id} exceeds declared object size bound"
            ));
        }
        let c: Certificate =
            serde_json::from_slice(&bytes).map_err(|e| format!("parse certificate {id}: {e}"))?;
        let digest = sha256(&bytes);
        let input_bytes = fs::read(path.join(format!("{id}.input.json")))
            .map_err(|e| format!("read input sidecar {id}: {e}"))?;
        let input: Value =
            serde_json::from_slice(&input_bytes).map_err(|e| format!("parse input {id}: {e}"))?;
        if string(&input, "case_id")? != id {
            return Err(format!("input id differs from certificate filename: {id}"));
        }
        let xs = roots(&input, "x_roots")?;
        let ts = roots(&input, "t_roots")?;
        let v = independent::verify(&c).map_err(|e| format!("{id}: {e}"))?;
        let grid = independent::verify_grid(&c, &xs, &ts).map_err(|e| format!("{id}: {e}"))?;
        let input_kind = string(&input, "kind")?;
        match input_kind {
            "s3" => {
                if integer(&input, "p")? != c.p {
                    return Err(format!("sidecar field differs: {id}"));
                }
                let a = integer(&input, "a")?;
                let b = integer(&input, "b")?;
                let d = usize::try_from(integer(&input, "d")?).map_err(|_| "degree overflow")?;
                let x = integer(&input, "target_x")?;
                let y = integer(&input, "target_y")?;
                let restrict = input
                    .get("restrict_liftable")
                    .and_then(Value::as_bool)
                    .ok_or("sidecar lacks liftability policy")?;
                let selector = roots(&input, "selector_roots")?;
                independent::verify_s3_input(&c, a, b, d, x, &selector, &xs, &ts, restrict)
                    .map_err(|e| format!("{id}: {e}"))?;
                // Additional independently computed point check on the
                // supplied public ordinate; this does not establish order.
                let p = c.p as u128;
                let rhs =
                    (((x as u128 * x as u128) % p) * x as u128 + a as u128 * x as u128 + b as u128)
                        % p;
                if y >= c.p || (y as u128 * y as u128) % p != rhs {
                    return Err(format!("invalid public target ordinate: {id}"));
                }
                s3_inputs += 1;
            }
            "synthetic" => {}
            _ => return Err(format!("unknown input kind: {input_kind}")),
        }
        if raw_jsonl.is_some() {
            let row = records
                .get(id)
                .ok_or_else(|| format!("certificate absent from persisted raw cases: {id}"))?;
            if string(row, "sha256")? != digest || integer(row, "bytes")? != bytes.len() as u64 {
                return Err(format!("certificate byte/hash binding fails: {id}"));
            }
            if integer(row, "B")? != (c.f.len() - 1) as u64
                || integer(row, "e")? != (c.g.len() - 1) as u64
                || integer(row, "blocks")? != c.leaves.len() as u64
                || integer(row, "support")? != c.support_count as u64
                || row.get("empty").and_then(Value::as_bool) != Some(c.verdict_empty)
            {
                return Err(format!("raw summary differs from certificate: {id}"));
            }
            let counts = row
                .get("checks")
                .and_then(Value::as_array)
                .ok_or("raw checks missing")?;
            if counts.len() != 3
                || counts[0].as_u64() != Some(v.identity_checks as u64)
                || counts[1].as_u64() != Some(grid.cells as u64)
            {
                return Err(format!("raw identity/grid check count differs: {id}"));
            }
        }
        identities += v.identity_checks;
        grid_cells += grid.cells;
        checked_ids.insert(id.to_owned());
        checks.push(json!({"id":id,"sha256":digest,"bytes":bytes.len(),
            "input_sha256":sha256(&input_bytes),"kind":input_kind,
            "branches":v.branches,"support":v.algebraic_support,
            "identity_checks":v.identity_checks,"grid_cells":grid.cells}));
    }
    if raw_jsonl.is_some() && records.keys().cloned().collect::<BTreeSet<_>>() != checked_ids {
        return Err("persisted raw case set differs from certificate directory".into());
    }
    Ok(json!({"kind":"persisted_certificate_check","passed":true,
        "certificates":checks.len(),"s3_inputs":s3_inputs,
        "identity_checks":identities,"grid_cells":grid_cells,
        "producer_solver_called":false,"aggregate_certificate_binding":raw_jsonl.is_some(),
        "raw_case_records":if raw_jsonl.is_some(){Some(raw_count)}else{None},
        "raw_sha256":raw_digest,"input_sidecar_bytes_previously_hashed_in_raw":false,
        "cases":checks}))
}
