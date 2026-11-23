//! Read fixed-panel proof objects without rerunning the producer.
//! The independent registry binds mathematical inputs; raw records additionally
//! bind the exact certificate and input-sidecar bytes.
use crate::{independent, kernel::Certificate, source_sha256};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const INPUT_CONTRACT: &str = "fixed_retained_panel_v2";
type Files = BTreeMap<String, Vec<u8>>;

#[derive(Clone, Debug)]
enum ExpectedInput {
    S3 {
        p: u64,
        a: u64,
        b: u64,
        d: usize,
        target_x: u64,
        liftable: bool,
    },
    Synthetic {
        f: Vec<u64>,
        g: Vec<u64>,
        h: Vec<Vec<u64>>,
        xs: Vec<u64>,
        ts: Vec<u64>,
    },
}
impl ExpectedInput {
    fn kind(&self) -> &'static str {
        match self {
            Self::S3 { .. } => "s3",
            Self::Synthetic { .. } => "synthetic",
        }
    }
}

/// This registry is independently specified from the frozen 27-object panel.
/// No producer metadata supplies its kind, curve, target, policy or synthetic
/// polynomial definitions.
fn expected_registry() -> BTreeMap<String, ExpectedInput> {
    let mut expected = BTreeMap::new();
    let families: [(&str, u64, u64, u64, usize, &[u64]); 3] = [
        ("f97", 97, 2, 3, 2, &[0, 1, 3, 30]),
        ("f97-rank-one", 97, 96, 0, 2, &[0, 1, 2]),
        ("prime31", 2147483647, 847514846, 2087444724, 7, &[0, 1, 2]),
    ];
    for (name, p, a, b, d, targets) in families {
        for &target_x in targets {
            for liftable in [false, true] {
                let policy = if liftable { "liftable" } else { "all" };
                expected.insert(
                    format!("{name}-d{d}-x{target_x}-{policy}"),
                    ExpectedInput::S3 {
                        p,
                        a,
                        b,
                        d,
                        target_x,
                        liftable,
                    },
                );
            }
        }
    }
    // Independently expanded (x-1)(x-2)(x-3) and t(t-1)(t-2) over F97.
    let f = vec![91, 11, 91, 1];
    let g = vec![0, 2, 94, 1];
    let inputs = [
        ("unit", vec![vec![1]]),
        ("zero", vec![vec![0]]),
        ("nonunit_lead", vec![vec![96], vec![0, 1]]),
        ("degree_drop", vec![vec![2, 94, 1], vec![0, 96, 1]]),
        ("diagonal", vec![vec![0, 96], vec![1]]),
    ];
    for (name, h) in inputs {
        expected.insert(
            format!("synthetic-{name}"),
            ExpectedInput::Synthetic {
                f: f.clone(),
                g: g.clone(),
                h,
                xs: vec![1, 2, 3],
                ts: vec![0, 1, 2],
            },
        );
    }
    expected.insert(
        "synthetic-empty-F".into(),
        ExpectedInput::Synthetic {
            f: vec![1],
            g: g.clone(),
            h: vec![vec![0]],
            xs: vec![],
            ts: vec![0, 1, 2],
        },
    );
    expected.insert(
        "synthetic-empty-G".into(),
        ExpectedInput::Synthetic {
            f,
            g: vec![1],
            h: vec![vec![0]],
            xs: vec![1, 2, 3],
            ts: vec![],
        },
    );
    expected
}

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

fn load_files(path: &Path) -> Result<Files, String> {
    let mut files = BTreeMap::new();
    for entry in fs::read_dir(path).map_err(|e| format!("read proof directory: {e}"))? {
        let entry = entry.map_err(|e| format!("proof directory entry: {e}"))?;
        let filename = entry
            .file_name()
            .to_str()
            .ok_or("non-UTF8 proof filename")?
            .to_owned();
        if filename.ends_with(".certificate.json") || filename.ends_with(".input.json") {
            if !entry
                .file_type()
                .map_err(|e| format!("proof file type: {e}"))?
                .is_file()
            {
                return Err("certificate and input sidecar must be ordinary files".into());
            }
            let bytes = fs::read(entry.path()).map_err(|e| format!("read {filename}: {e}"))?;
            if files.insert(filename, bytes).is_some() {
                return Err("duplicate proof filename".into());
            }
        }
    }
    Ok(files)
}

/// This core is shared by file reading and the in-memory rejection controls.
/// It dispatches from the independent registry, never from a sidecar's kind.
fn check_loaded(files: &Files, raw_bytes: Option<&[u8]>) -> Result<Value, String> {
    let expected = expected_registry();
    let expected_files: BTreeSet<_> = expected
        .keys()
        .flat_map(|id| [format!("{id}.certificate.json"), format!("{id}.input.json")])
        .collect();
    let observed_files: BTreeSet<_> = files.keys().cloned().collect();
    if observed_files != expected_files {
        let missing: Vec<_> = expected_files.difference(&observed_files).collect();
        let unexpected: Vec<_> = observed_files.difference(&expected_files).collect();
        return Err(format!(
            "fixed fixture file set differs: missing {missing:?}; unexpected {unexpected:?}"
        ));
    }
    let mut records = BTreeMap::new();
    let mut raw_count = 0usize;
    let mut raw_digest = None;
    if let Some(bytes) = raw_bytes {
        raw_digest = Some(sha256(bytes));
        let text = std::str::from_utf8(bytes).map_err(|e| format!("raw UTF8: {e}"))?;
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
        if records.keys().cloned().collect::<BTreeSet<_>>()
            != expected.keys().cloned().collect::<BTreeSet<_>>()
        {
            return Err("persisted raw case set differs from the fixed 27-case registry".into());
        }
    }
    let mut checks = Vec::new();
    let mut identities = 0usize;
    let mut grid_cells = 0usize;
    let mut s3_inputs = 0usize;
    for (id, required) in &expected {
        let bytes = &files[&format!("{id}.certificate.json")];
        let input_bytes = &files[&format!("{id}.input.json")];
        if bytes.len() > 65536 {
            return Err(format!(
                "certificate {id} exceeds declared object size bound"
            ));
        }
        let digest = sha256(bytes);
        let input_digest = sha256(input_bytes);
        if raw_bytes.is_some() {
            let row = &records[id];
            if string(row, "sha256")? != digest || integer(row, "bytes")? != bytes.len() as u64 {
                return Err(format!("certificate byte/hash binding fails: {id}"));
            }
            // Check exact sidecar bytes before parsing or semantic dispatch.
            if string(row, "input_sha256")? != input_digest
                || integer(row, "input_bytes")? != input_bytes.len() as u64
            {
                return Err(format!("input sidecar byte/hash binding fails: {id}"));
            }
        }
        let c: Certificate =
            serde_json::from_slice(bytes).map_err(|e| format!("parse certificate {id}: {e}"))?;
        let input: Value =
            serde_json::from_slice(input_bytes).map_err(|e| format!("parse input {id}: {e}"))?;
        if string(&input, "case_id")? != id {
            return Err(format!("input id differs from certificate filename: {id}"));
        }
        if string(&input, "kind")? != required.kind() {
            return Err(format!("fixed input kind mismatch: {id}"));
        }
        let xs = roots(&input, "x_roots")?;
        let ts = roots(&input, "t_roots")?;
        match required {
            ExpectedInput::S3 {
                p,
                a,
                b,
                d,
                target_x,
                liftable,
            } => {
                if c.p != *p
                    || integer(&input, "p")? != *p
                    || integer(&input, "a")? != *a
                    || integer(&input, "b")? != *b
                    || integer(&input, "d")? != *d as u64
                    || integer(&input, "target_x")? != *target_x
                    || input.get("restrict_liftable").and_then(Value::as_bool) != Some(*liftable)
                {
                    return Err(format!("fixed S3 fixture parameters differ: {id}"));
                }
                if integer(&input, "target_y")? > (p - 1) / 2 {
                    return Err(format!(
                        "public ordinate is not the canonical smaller root: {id}"
                    ));
                }
            }
            ExpectedInput::Synthetic {
                f,
                g,
                h,
                xs: expected_xs,
                ts: expected_ts,
            } => {
                if c.p != 97
                    || c.f != *f
                    || c.g != *g
                    || c.h != *h
                    || xs != *expected_xs
                    || ts != *expected_ts
                {
                    return Err(format!("fixed synthetic input differs: {id}"));
                }
            }
        }
        let v = independent::verify(&c).map_err(|e| format!("{id}: {e}"))?;
        let grid = independent::verify_grid(&c, &xs, &ts).map_err(|e| format!("{id}: {e}"))?;
        if let ExpectedInput::S3 {
            p,
            a,
            b,
            d,
            target_x,
            liftable,
        } = required
        {
            let selector = roots(&input, "selector_roots")?;
            independent::verify_s3_input(&c, *a, *b, *d, *target_x, &selector, &xs, &ts, *liftable)
                .map_err(|e| format!("{id}: {e}"))?;
            // Registry-derived coordinates and a separately checked public y.
            let modulus = *p as u128;
            let x = *target_x as u128;
            let y = integer(&input, "target_y")? as u128;
            let rhs = (((x * x) % modulus) * x + *a as u128 * x + *b as u128) % modulus;
            if (y * y) % modulus != rhs {
                return Err(format!("invalid public target ordinate: {id}"));
            }
            s3_inputs += 1;
        }
        if raw_bytes.is_some() {
            let row = &records[id];
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
        checks.push(json!({"id":id,"sha256":digest,"bytes":bytes.len(),
            "input_sha256":input_digest,"input_bytes":input_bytes.len(),"kind":required.kind(),
            "branches":v.branches,"support":v.algebraic_support,
            "identity_checks":v.identity_checks,"grid_cells":grid.cells}));
    }
    Ok(json!({"kind":"persisted_certificate_check","passed":true,
        "input_contract":INPUT_CONTRACT,"input_registry_binding":true,
        "certificates":checks.len(),"s3_inputs":s3_inputs,
        "identity_checks":identities,"grid_cells":grid_cells,
        "producer_solver_called":false,"aggregate_certificate_binding":raw_bytes.is_some(),
        "raw_case_records":if raw_bytes.is_some(){Some(raw_count)}else{None},
        "raw_sha256":raw_digest,"input_sidecar_bytes_previously_hashed_in_raw":raw_bytes.is_some(),
        "cases":checks}))
}

/// Without raw JSONL, the independent registry still binds all mathematical
/// inputs and requires the exact retained panel. With raw JSONL, exact sidecar
/// and certificate byte bindings are additionally mandatory.
pub fn check_directory(path: &Path, raw_jsonl: Option<&Path>) -> Result<Value, String> {
    let files = load_files(path)?;
    let raw_bytes = raw_jsonl
        .map(fs::read)
        .transpose()
        .map_err(|e| format!("read raw records: {e}"))?;
    check_loaded(&files, raw_bytes.as_deref())
}

fn edit_input(files: &mut Files, id: &str, edit: impl FnOnce(&mut Value)) -> Result<(), String> {
    let bytes = files
        .get_mut(&format!("{id}.input.json"))
        .ok_or("control input missing")?;
    let mut value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    edit(&mut value);
    *bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
    Ok(())
}

fn substitute_case(files: &Files, target: &str, source: &str) -> Result<Files, String> {
    let mut changed = files.clone();
    for suffix in ["certificate", "input"] {
        let bytes = files
            .get(&format!("{source}.{suffix}.json"))
            .ok_or("control source missing")?;
        changed.insert(format!("{target}.{suffix}.json"), bytes.clone());
    }
    edit_input(&mut changed, target, |v| v["case_id"] = json!(target))?;
    Ok(changed)
}

fn edit_raw(raw: &[u8], id: &str, mut edit: impl FnMut(&mut Value)) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(raw).map_err(|e| e.to_string())?;
    let mut output = Vec::new();
    let mut changed = 0usize;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut value: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        if value.get("kind").and_then(Value::as_str) == Some("case")
            && value.get("id").and_then(Value::as_str) == Some(id)
        {
            edit(&mut value);
            changed += 1;
        }
        output.extend(serde_json::to_vec(&value).map_err(|e| e.to_string())?);
        output.push(b'\n');
    }
    if changed != 1 {
        return Err("control expected exactly one original raw case".into());
    }
    Ok(output)
}

fn expect_rejection(
    name: &str,
    files: &Files,
    raw: Option<&[u8]>,
    outcomes: &mut Vec<Value>,
) -> Result<(), String> {
    match check_loaded(files, raw) {
        Ok(_) => Err(format!(
            "negative control unexpectedly accepted: {name}, raw={}",
            raw.is_some()
        )),
        Err(reason) => {
            outcomes.push(
                json!({"name":name,"raw_binding":raw.is_some(),"rejected":true,"reason":reason}),
            );
            Ok(())
        }
    }
}

fn expect_acceptance(
    name: &str,
    files: &Files,
    raw: Option<&[u8]>,
    outcomes: &mut Vec<Value>,
) -> Result<(), String> {
    let result = check_loaded(files, raw).map_err(|e| format!("positive control {name}: {e}"))?;
    outcomes.push(
        json!({"name":name,"raw_binding":raw.is_some(),"passed":true,
        "certificates":result["certificates"],"s3_inputs":result["s3_inputs"]}),
    );
    Ok(())
}

/// Frozen correction controls: 17 semantic/artifact mutations in two modes,
/// five raw-only rejection checks, and three positive checks. Mutations apply
/// only to in-memory copies. Original proof files and raw bytes are read-only.
pub fn negative_controls(path: &Path, raw_jsonl: Option<&Path>) -> Result<Value, String> {
    let raw_path = raw_jsonl.ok_or("the complete fixed control panel requires RAW_JSONL")?;
    let files = load_files(path)?;
    let raw = fs::read(raw_path).map_err(|e| format!("read raw controls: {e}"))?;
    let mut positive = Vec::new();
    let mut negative = Vec::new();
    expect_acceptance("original_panel_with_raw", &files, Some(&raw), &mut positive)?;
    expect_acceptance("original_panel_without_raw", &files, None, &mut positive)?;
    let id = "f97-d2-x0-all";
    let mut mutations = Vec::new();
    for (name, field, value) in [
        ("s3_kind_downgrade", "kind", json!("synthetic")),
        ("s3_field_change", "p", json!(101)),
        ("s3_a_change", "a", json!(3)),
        ("s3_b_change", "b", json!(4)),
        ("s3_degree_change", "d", json!(3)),
        ("s3_target_x_change", "target_x", json!(1)),
        ("s3_lift_policy_change", "restrict_liftable", json!(true)),
        // At x=0 on y^2=x^3+2x+3 over F97, both 10 and 87 square to 3.
        // The retained fixture requires the smaller ordinate, 10.
        ("s3_target_ordinate_sign_flip", "target_y", json!(87)),
    ] {
        let mut changed = files.clone();
        edit_input(&mut changed, id, |v| v[field] = value)?;
        mutations.push((name, changed));
    }
    let mut changed = files.clone();
    edit_input(&mut changed, "synthetic-zero", |v| v["kind"] = json!("s3"))?;
    mutations.push(("synthetic_kind_upgrade", changed));
    for (name, source) in [
        ("synthetic_F_change", "synthetic-empty-F"),
        ("synthetic_G_change", "synthetic-empty-G"),
        ("synthetic_H_change", "synthetic-unit"),
    ] {
        mutations.push((name, substitute_case(&files, "synthetic-zero", source)?));
    }
    let unknown = "unregistered-case";
    let mut changed = substitute_case(&files, unknown, id)?;
    changed.remove(&format!("{id}.certificate.json"));
    changed.remove(&format!("{id}.input.json"));
    mutations.push(("unknown_case", changed));
    let mut changed = files.clone();
    changed.remove("synthetic-unit.certificate.json");
    changed.remove("synthetic-unit.input.json");
    mutations.push(("missing_case", changed));
    mutations.push((
        "extra_case",
        substitute_case(&files, unknown, "synthetic-unit")?,
    ));
    let mut changed = files.clone();
    changed.remove(&format!("{id}.input.json"));
    mutations.push(("missing_sidecar", changed));
    let mut changed = files.clone();
    edit_input(&mut changed, id, |v| v["case_id"] = json!("f97-d2-x1-all"))?;
    mutations.push(("wrong_sidecar_case_id", changed));
    if mutations.len() != 17 {
        return Err("incorrect frozen semantic-control count".into());
    }
    for (name, changed) in &mutations {
        expect_rejection(name, changed, Some(&raw), &mut negative)?;
        expect_rejection(name, changed, None, &mut negative)?;
    }
    let mut whitespace = files.clone();
    whitespace
        .get_mut(&format!("{id}.input.json"))
        .ok_or("control sidecar missing")?
        .push(b'\n');
    expect_acceptance(
        "sidecar_whitespace_without_raw",
        &whitespace,
        None,
        &mut positive,
    )?;
    expect_rejection(
        "sidecar_whitespace_with_raw",
        &whitespace,
        Some(&raw),
        &mut negative,
    )?;
    let changed = edit_raw(&raw, id, |v| v["input_sha256"] = json!("0".repeat(64)))?;
    expect_rejection(
        "raw_input_sha256_change",
        &files,
        Some(&changed),
        &mut negative,
    )?;
    let changed = edit_raw(&raw, id, |v| {
        let bytes = v["input_bytes"]
            .as_u64()
            .expect("baseline already checked input_bytes");
        v["input_bytes"] = json!(bytes + 1);
    })?;
    expect_rejection(
        "raw_input_bytes_change",
        &files,
        Some(&changed),
        &mut negative,
    )?;
    let changed = edit_raw(&raw, id, |v| {
        v.as_object_mut()
            .expect("case row is an object")
            .remove("input_sha256");
    })?;
    expect_rejection(
        "raw_input_sha256_missing",
        &files,
        Some(&changed),
        &mut negative,
    )?;
    let changed = edit_raw(&raw, id, |v| {
        v.as_object_mut()
            .expect("case row is an object")
            .remove("input_bytes");
    })?;
    expect_rejection(
        "raw_input_bytes_missing",
        &files,
        Some(&changed),
        &mut negative,
    )?;
    if negative.len() != 39 || positive.len() != 3 {
        return Err("incorrect complete frozen control counts".into());
    }
    Ok(json!({"kind":"fixed_panel_checker_controls","passed":true,
        "input_contract":INPUT_CONTRACT,"rejected":negative.len(),"positive_checks":positive.len(),
        "producer_solver_called":false,"proof_artifacts_modified":false,
        "raw_sha256":sha256(&raw),"positive_controls":positive,"negative_controls":negative}))
}
