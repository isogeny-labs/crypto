//! Separate complete proof replay of actual-root artifacts. No producer arithmetic.
use relation_replay_checker::packed;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
};
type Result<T> = std::result::Result<T, String>;
type Native = Vec<Vec<(Vec<u32>, u64)>>;
type Polynomial = BTreeMap<Vec<usize>, u64>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(path: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 * 1024 * 1024 {
        return Err("file type/size".into());
    }
    let b = fs::read(path).map_err(|e| e.to_string())?;
    if b.len() > 16 * 1024 * 1024 {
        return Err("file grew past cap".into());
    }
    Ok(b)
}
fn value(b: &[u8]) -> Result<Value> {
    serde_json::from_slice(b).map_err(|e| e.to_string())
}
fn require(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn native(b: &[u8], n: usize, p: u64) -> Result<Native> {
    let v: Native = serde_json::from_slice(b).map_err(|e| e.to_string())?;
    require(!v.is_empty() && v.len() <= 256, "native basis shape")?;
    for f in &v {
        require(!f.is_empty() && f[0].1 == 1, "native nonmonic/empty")?;
        let mut seen = BTreeSet::new();
        for (m, c) in f {
            require(
                m.len() == n
                    && m.iter().map(|e| u64::from(*e)).sum::<u64>() <= 64
                    && *c > 0
                    && *c < p
                    && seen.insert(m.clone()),
                "native term shape",
            )?;
        }
        for pair in f.windows(2) {
            require(
                grevlex(&pair[0].0, &pair[1].0) == Ordering::Greater,
                "native order",
            )?;
        }
    }
    Ok(v)
}
fn grevlex(a: &[u32], b: &[u32]) -> Ordering {
    a.iter()
        .map(|x| u64::from(*x))
        .sum::<u64>()
        .cmp(&b.iter().map(|x| u64::from(*x)).sum::<u64>())
        .then_with(|| {
            a.iter()
                .zip(b)
                .rev()
                .find_map(|(x, y)| (x != y).then(|| y.cmp(x)))
                .unwrap_or(Ordering::Equal)
        })
}
fn maps(v: &Native) -> Vec<Polynomial> {
    v.iter()
        .map(|f| {
            f.iter()
                .map(|(m, c)| (m.iter().map(|x| *x as usize).collect(), *c))
                .collect()
        })
        .collect()
}
fn original(b: &[u8], n: usize, p: u64) -> Result<(Value, Vec<Polynomial>)> {
    let v = value(b)?;
    require(
        v["prime"] == p && v["variables"].as_array().is_some_and(|a| a.len() == n),
        "original field/variables",
    )?;
    let fs = v["polynomials"].as_array().ok_or("original polynomials")?;
    require(!fs.is_empty() && fs.len() <= 32, "original count")?;
    let mut out = Vec::new();
    for f in fs {
        let terms = f.as_array().ok_or("original terms")?;
        require(terms.len() <= 32, "original terms cap")?;
        let mut poly = Polynomial::new();
        for t in terms {
            let ex = t["exponents"].as_array().ok_or("original exponents")?;
            require(ex.len() == n, "original exponent shape")?;
            let m = ex
                .iter()
                .map(|x| {
                    x.as_u64()
                        .filter(|x| *x <= 4)
                        .map(|x| x as usize)
                        .ok_or("original exponent")
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;
            require(m.iter().sum::<usize>() <= 4, "original degree")?;
            let c = t["coefficient"]
                .as_i64()
                .ok_or("original coefficient")?
                .rem_euclid(p as i64) as u64;
            let c = (poly.get(&m).copied().unwrap_or(0) + c) % p;
            if c == 0 {
                poly.remove(&m);
            } else {
                poly.insert(m, c);
            }
        }
        out.push(poly);
    }
    Ok((v["variables"].clone(), out))
}
fn load(root: &Path, name: &str, overlay: &BTreeMap<String, Vec<u8>>) -> Result<Vec<u8>> {
    if let Some(b) = overlay.get(name) {
        return Ok(b.clone());
    }
    read(&root.join(name))
}
fn audit(
    root: &Path,
    refs: &Path,
    pins: &Value,
    overlay: &BTreeMap<String, Vec<u8>>,
) -> Result<Value> {
    let matrix = value(&load(root, "matrix.json", overlay)?)?;
    require(
        matrix["schema"] == "relation-root-integration-matrix/v1"
            && matrix["case_count"] == 36
            && matrix["timing_claim"] == false,
        "matrix metadata",
    )?;
    let cases = matrix["cases"].as_array().ok_or("cases")?;
    require(cases.len() == 36, "case count")?;
    let mut seen = BTreeSet::new();
    let mut results = Vec::new();
    for row in cases {
        let p = row["prime"].as_u64().ok_or("prime")?;
        let n = row["n"].as_u64().ok_or("n")? as usize;
        let form = row["form"].as_str().ok_or("form")?;
        let kernel = row["kernel"].as_str().ok_or("kernel")?;
        require(
            [2147483647, 9007199254740881].contains(&p)
                && [3, 4, 6].contains(&n)
                && [
                    "compact-sparse",
                    "compact-dag",
                    "packed-sparse",
                    "packed-dag",
                ]
                .contains(&form)
                && ["frontier103", "products231", "batch487"].contains(&kernel)
                && seen.insert((p, n, form.to_owned(), kernel.to_owned())),
            "case domain/duplicate",
        )?;
        let id = format!("p{p}-n{n}-{form}-{kernel}");
        let file = |name: &str| load(root, &format!("{id}/{name}"), overlay);
        let outcome = value(&file("outcome.json")?)?;
        require(outcome == *row, "matrix/case binding")?;
        require(
            row["seed"] == 7
                && row["graph"] == "LAST-star"
                && row["timing_claim"] == false
                && row.get("observed_solving_degree") == Some(&Value::Null),
            "case declaration",
        )?;
        let input = file("original.json")?;
        let proof = file("proof.json")?;
        let basis_bytes = file("basis.json")?;
        let control_bytes = file("control-basis.json")?;
        for (key, bytes) in [
            ("original_sha256", &input),
            ("proof_sha256", &proof),
            ("basis_sha256", &basis_bytes),
            ("control_basis_sha256", &control_bytes),
        ] {
            require(row[key] == sha(bytes), "artifact hash")?;
        }
        let ref_name = format!("p{p}-n{n}-frontier/original.json");
        let frozen = read(&refs.join(&ref_name))?;
        let suffix = format!("/{ref_name}");
        let pin = pins["files"]
            .as_array()
            .ok_or("pin files")?
            .iter()
            .find(|x| x["path"].as_str().is_some_and(|x| x.ends_with(&suffix)))
            .ok_or("frozen input pin")?;
        require(pin["sha256"] == sha(&frozen), "frozen input digest")?;
        require(
            original(&input, n, p)? == original(&frozen, n, p)?,
            "frozen input differs",
        )?;
        let checked = if form.ends_with("-dag") {
            packed::verify_dag_json(&input, &proof)
        } else {
            packed::verify_json(&input, &proof)
        }?;
        let basis = native(&basis_bytes, n, p)?;
        let expected: Vec<Polynomial> = checked
            .basis
            .iter()
            .map(|f| f.iter().cloned().collect())
            .collect();
        require(
            maps(&basis) == expected,
            "native basis differs from separately checked proof",
        )?;
        let mut sorted = basis.clone();
        sorted.sort();
        let control = native(&control_bytes, n, p)?;
        require(sorted == control, "control basis differs")?;
        require(
            row["sorted_basis_sha256"]
                == sha(&serde_json::to_vec(&sorted).map_err(|e| e.to_string())?),
            "sorted native digest",
        )?;
        require(
            row["replay_stats"]
                == serde_json::to_value(&checked.stats).map_err(|e| e.to_string())?,
            "replay stats differ",
        )?;
        let pairs = basis.len() * (basis.len() - 1) / 2;
        require(
            row["native_checks"]["certification_pairs"] == pairs
                && row["native_checks"]["certification_reductions"]
                    == pairs + expected_input_count(&input)?,
            "native coverage count",
        )?;
        require(
            row["control_basis_matches"] == true && row["final_replay_accepts"] == true,
            "receipt booleans",
        )?;
        require(
            row["mutations_rejected"]
                == json!([
                    "basis_coefficient",
                    "replay_stats",
                    "native_checks",
                    "observed_degree",
                    "original_wire",
                    "proof_wire",
                    "changed_input"
                ]),
            "mutation receipt classes",
        )?;
        results.push(json!({"case":id,"proof_sha256":sha(&proof),"frozen_input_sha256":sha(&frozen),"complete_separate_replay":true,"native_basis_matches":true,"native_runtime_operation_counts_independently_reexecuted":false}));
    }
    Ok(
        json!({"schema":"relation-root-matrix-audit/v1","status":"producer_replay_checks_passed","cases":results,"case_count":36,"signed_execution_verdict":false,"independent_host":false,"timing_claim":false}),
    )
}
fn expected_input_count(input: &[u8]) -> Result<usize> {
    Ok(value(input)?["polynomials"]
        .as_array()
        .ok_or("input count")?
        .len())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    require(
        a.len() == 5,
        "usage: root-matrix-audit RAW REFERENCES PINS REPORT",
    )?;
    let root = Path::new(&a[1]);
    let refs = Path::new(&a[2]);
    let pins = value(&read(Path::new(&a[3]))?)?;
    let mut report = audit(root, refs, &pins, &BTreeMap::new())?;
    // Forge both per-case metadata and matrix metadata, so only semantic
    // replay/stat binding can reject the two sabotage views. Raw files stay intact.
    let matrix = value(&read(&root.join("matrix.json"))?)?;
    let row = &matrix["cases"][0];
    let id = format!(
        "p{}-n{}-{}-{}",
        row["prime"].as_u64().unwrap(),
        row["n"].as_u64().unwrap(),
        row["form"].as_str().unwrap(),
        row["kernel"].as_str().unwrap()
    );
    let mut canaries = Vec::new();
    for kind in ["native_basis", "replay_stats"] {
        let mut forged = row.clone();
        let mut altered = matrix.clone();
        let mut overlay = BTreeMap::new();
        if kind == "native_basis" {
            let mut basis: Native =
                serde_json::from_slice(&read(&root.join(format!("{id}/basis.json")))?)
                    .map_err(|e| e.to_string())?;
            // A nonleading coefficient change preserves shape, monicity and order.
            require(basis[0].len() > 1, "canary requires a tail")?;
            let p = row["prime"].as_u64().unwrap();
            let c = &mut basis[0][1].1;
            *c = if *c == p - 1 { 1 } else { *c + 1 };
            let bytes = serde_json::to_vec(&basis).map_err(|e| e.to_string())?;
            forged["basis_sha256"] = json!(sha(&bytes));
            basis.sort();
            forged["sorted_basis_sha256"] =
                json!(sha(&serde_json::to_vec(&basis).map_err(|e| e.to_string())?));
            overlay.insert(format!("{id}/basis.json"), bytes);
        } else {
            forged["replay_stats"]["cofactor_terms"] =
                json!(row["replay_stats"]["cofactor_terms"].as_u64().unwrap() + 1);
        }
        altered["cases"][0] = forged.clone();
        overlay.insert("matrix.json".into(), serde_json::to_vec(&altered).unwrap());
        overlay.insert(
            format!("{id}/outcome.json"),
            serde_json::to_vec(&forged).unwrap(),
        );
        let error = audit(root, refs, &pins, &overlay)
            .err()
            .ok_or("sabotage view accepted")?;
        require(
            error.contains(if kind == "native_basis" {
                "native basis differs"
            } else {
                "replay stats differ"
            }),
            "canary rejected for unrelated reason",
        )?;
        canaries.push(
            json!({"kind":kind,"hash_consistent_metadata":true,"rejected":true,"reason":error}),
        );
    }
    report["sabotage_controls"] = json!(canaries);
    let bytes = serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[4])
        .map_err(|e| e.to_string())?
        .write_all(&bytes)
        .map_err(|e| e.to_string())?;
    println!("36 complete separate proof replays; native/control bases bound; two semantic sabotage views rejected; unsigned correctness evidence");
    Ok(())
}
