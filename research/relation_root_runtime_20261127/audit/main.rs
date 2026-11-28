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
        matrix["schema"] == "relation-pairgraph-correctness/v1"
            && matrix["paired_cases"] == 108
            && matrix["constructions"] == 216
            && matrix["mutations_rejected"] == 864
            && matrix["timing_claim"] == false,
        "matrix declaration",
    )?;
    let rows = matrix["cases"].as_array().ok_or("cases")?;
    require(rows.len() == 108, "case count")?;
    let mut seen = BTreeSet::new();
    let mut roster = BTreeSet::from(["matrix.json".to_string()]);
    let mut results = vec![];
    for row in rows {
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
                    "flat-sparse",
                    "flat-dag",
                ]
                .contains(&form)
                && ["frontier103", "products231", "batch487"].contains(&kernel)
                && seen.insert((p, n, form.to_string(), kernel.to_string())),
            "case roster",
        )?;
        require(
            row["seed"] == 7
                && row["graph"] == "LAST-star"
                && row["mutations_rejected"] == 8
                && row["observed_solving_degree"].is_null()
                && row["timing_claim"] == false
                && row["same_original_proof_basis_and_replay_stats"] == true,
            "case declarations",
        )?;
        let id = format!("p{p}-n{n}-{form}-{kernel}");
        let file = |name: &str| load(root, &format!("{id}/{name}"), overlay);
        for name in ["original.json", "proof.json", "basis.json", "outcome.json"] {
            roster.insert(format!("{id}/{name}"));
        }
        require(
            value(&file("outcome.json")?)? == *row,
            "matrix/outcome binding",
        )?;
        let input = file("original.json")?;
        let proof = file("proof.json")?;
        let reference = format!("p{p}-n{n}-frontier/original.json");
        let frozen = read(&refs.join(&reference))?;
        let suffix = format!("/{reference}");
        let pin = pins["files"]
            .as_array()
            .ok_or("pins")?
            .iter()
            .find(|v| v["path"].as_str().is_some_and(|x| x.ends_with(&suffix)))
            .ok_or("frozen reference pin")?;
        require(
            pin["sha256"] == sha(&frozen) && original(&input, n, p)? == original(&frozen, n, p)?,
            "frozen original binding",
        )?;
        let checked = if form.ends_with("-dag") {
            packed::verify_dag_json(&input, &proof)
        } else {
            packed::verify_json(&input, &proof)
        }?;
        let basis = native(&file("basis.json")?, n, p)?;
        let expected: Vec<Polynomial> = checked
            .basis
            .iter()
            .map(|f| f.iter().cloned().collect())
            .collect();
        require(
            maps(&basis) == expected,
            "native basis vs complete separate proof",
        )?;
        let pairs = basis.len() * (basis.len() - 1) / 2;
        let inputs = original(&input, n, p)?.1.len();
        let exhaustive = &row["exhaustive"];
        let graph = &row["pair_graph"];
        let edges = &graph["pair_graph"];
        require(
            exhaustive["certification_pairs"] == pairs
                && exhaustive["certification_reductions"] == pairs + inputs
                && exhaustive.get("pair_graph").is_none(),
            "exhaustive coverage",
        )?;
        let product = edges["product_edges"].as_u64().ok_or("product edges")?;
        let chain = edges["chain_edges"].as_u64().ok_or("chain edges")?;
        let direct = edges["direct_edges"].as_u64().ok_or("direct edges")?;
        require(
            product
                .checked_add(chain)
                .and_then(|v| v.checked_add(direct))
                == Some(pairs as u64)
                && graph["certification_pairs"] == pairs
                && graph["certification_reductions"] == direct + inputs as u64
                && edges["resource_refusals"] == 0,
            "graph coverage",
        )?;
        results.push(json!({"case":id,"proof_sha256":sha(&proof),"input_sha256":sha(&input),"complete_proof_and_basis_replay":true,"native_coverage_metadata_bound":true,"native_operation_counts_independently_reexecuted":false}));
    }
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_str().ok_or("filename")?.to_string();
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        if ty.is_file() {
            actual.insert(name);
        } else if ty.is_dir() {
            for child in fs::read_dir(entry.path()).map_err(|e| e.to_string())? {
                let child = child.map_err(|e| e.to_string())?;
                require(
                    child.file_type().map_err(|e| e.to_string())?.is_file(),
                    "nonregular case input",
                )?;
                actual.insert(format!(
                    "{name}/{}",
                    child.file_name().to_str().ok_or("child filename")?
                ));
            }
        } else {
            return Err("nonregular root input".into());
        }
    }
    require(actual == roster, "exact stored artifact roster")?;
    Ok(
        json!({"schema":"relation-pairgraph-complete-replay/v1","cases":results,"case_count":108,"native_operation_counts_independently_reexecuted":false,"signed_execution_verdict":false,"independent_host":false,"timing_claim":false}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    require(args.len() == 5, "audit ROOT REFERENCES PINS OUTPUT")?;
    let root = Path::new(&args[1]);
    let refs = Path::new(&args[2]);
    let pins = value(&read(Path::new(&args[3]))?)?;
    let mut report = audit(root, refs, &pins, &BTreeMap::new())?;
    let matrix = value(&read(&root.join("matrix.json"))?)?;
    let row = &matrix["cases"][0];
    let id = format!(
        "p{}-n{}-{}-{}",
        row["prime"].as_u64().unwrap(),
        row["n"].as_u64().unwrap(),
        row["form"].as_str().unwrap(),
        row["kernel"].as_str().unwrap()
    );
    let mut rejected = vec![];
    for kind in [
        "native_basis",
        "native_coverage",
        "proof_wire",
        "input_wire",
        "matrix_roster",
    ] {
        let mut overlay = BTreeMap::new();
        match kind {
            "native_basis" => {
                let mut basis: Native =
                    serde_json::from_slice(&read(&root.join(format!("{id}/basis.json")))?)
                        .map_err(|e| e.to_string())?;
                require(basis[0].len() > 1, "basis canary tail")?;
                let p = row["prime"].as_u64().unwrap();
                basis[0][1].1 = if basis[0][1].1 == p - 1 {
                    1
                } else {
                    basis[0][1].1 + 1
                };
                overlay.insert(
                    format!("{id}/basis.json"),
                    serde_json::to_vec(&basis).map_err(|e| e.to_string())?,
                );
            }
            "native_coverage" => {
                let mut altered = matrix.clone();
                altered["cases"][0]["pair_graph"]["certification_pairs"] = json!(0);
                overlay.insert(
                    "matrix.json".into(),
                    serde_json::to_vec(&altered).map_err(|e| e.to_string())?,
                );
                overlay.insert(
                    format!("{id}/outcome.json"),
                    serde_json::to_vec(&altered["cases"][0]).map_err(|e| e.to_string())?,
                );
            }
            "proof_wire" | "input_wire" => {
                let name = if kind == "proof_wire" {
                    "proof.json"
                } else {
                    "original.json"
                };
                let mut bytes = read(&root.join(format!("{id}/{name}")))?;
                bytes[0] = b'!';
                overlay.insert(format!("{id}/{name}"), bytes);
            }
            _ => {
                let mut altered = matrix.clone();
                altered["cases"].as_array_mut().unwrap().pop();
                overlay.insert(
                    "matrix.json".into(),
                    serde_json::to_vec(&altered).map_err(|e| e.to_string())?,
                );
            }
        }
        require(
            audit(root, refs, &pins, &overlay).is_err(),
            "sabotage view accepted",
        )?;
        rejected.push(kind);
    }
    report["canaries_rejected"] = json!(rejected);
    let bytes = serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[4])
        .map_err(|e| e.to_string())?
        .write_all(&bytes)
        .map_err(|e| e.to_string())?;
    println!("108 complete separate proof/basis replays and five refused sabotage views; native operation counters remain unverified execution data");
    Ok(())
}
