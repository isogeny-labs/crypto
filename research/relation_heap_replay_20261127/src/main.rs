//! Whole-certificate differential replay; structural counts only, no clocks.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{cmp::Ordering, collections::BTreeSet, fs, io::Write, path::Path};
fn read(p: &Path) -> Result<Vec<u8>, String> {
    fs::read(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn load(p: &Path) -> Result<Value, String> {
    serde_json::from_slice(&read(p)?).map_err(|e| e.to_string())
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn cmp(a: &[usize], b: &[usize]) -> Ordering {
    a.iter()
        .sum::<usize>()
        .cmp(&b.iter().sum::<usize>())
        .then_with(|| {
            a.iter()
                .rev()
                .zip(b.iter().rev())
                .find_map(|(a, b)| (a != b).then(|| b.cmp(a)))
                .unwrap_or(Ordering::Equal)
        })
}
fn lead(row: &Value) -> Vec<usize> {
    row["polynomial"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            t["exponents"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .max_by(|a, b| cmp(a, b))
        .unwrap()
}
fn check(mode: usize, dag: bool, input: &[u8], wire: &[u8]) -> Result<Value, String> {
    match mode {
        0 => {
            let r = if dag {
                original::packed::verify_dag_json(input, wire)
            } else {
                original::packed::verify_json(input, wire)
            }?;
            serde_json::to_value(r).map_err(|e| e.to_string())
        }
        1 => {
            let r = if dag {
                candidate::packed::verify_dag_json(input, wire)
            } else {
                candidate::packed::verify_json(input, wire)
            }?;
            serde_json::to_value(r).map_err(|e| e.to_string())
        }
        2 => {
            let r = if dag {
                candidate::packed::verify_dag_json_heap(input, wire)
            } else {
                candidate::packed::verify_json_heap(input, wire)
            }?;
            serde_json::to_value(r).map_err(|e| e.to_string())
        }
        _ => Err("mode".into()),
    }
}
fn native_basis_matches(native: &[u8], checked: &Value) -> Result<bool, String> {
    type Basis = Vec<Vec<(Vec<usize>, u64)>>;
    let mut a: Basis = serde_json::from_slice(native).map_err(|e| e.to_string())?;
    let mut b: Basis =
        serde_json::from_value(checked["basis"].clone()).map_err(|e| e.to_string())?;
    for row in &mut a {
        row.sort()
    }
    for row in &mut b {
        row.sort()
    }
    a.sort();
    b.sort();
    Ok(a == b)
}
fn main_run(study: &Path, out: &Path) -> Result<(), String> {
    let pins = load(&study.join("INPUT_PINS.json"))?;
    let cases = pins["cases"].as_array().ok_or("cases")?;
    let fixed = load(Path::new(
        "research/relation_scan_call_instrument_20261127/SYNTHETIC_INPUT_PINS.json",
    ))?;
    let mut expected = BTreeSet::new();
    for lane in ["original", "pairgraph"] {
        for p in [2147483647u64, 9007199254740881] {
            for n in [3, 4, 6] {
                let forms = if lane == "original" {
                    vec!["flat-sparse", "flat-dag"]
                } else {
                    vec![
                        "compact-sparse",
                        "compact-dag",
                        "packed-sparse",
                        "packed-dag",
                        "flat-sparse",
                        "flat-dag",
                    ]
                };
                for form in forms {
                    for kernel in ["frontier103", "products231", "batch487"] {
                        expected.insert(format!("{lane}/p{p}-n{n}-{form}-{kernel}"));
                    }
                }
            }
        }
    }
    let ids: BTreeSet<_> = cases
        .iter()
        .map(|c| c["id"].as_str().unwrap().to_string())
        .collect();
    if ids != expected || cases.len() != 144 {
        return Err("fixed144 roster".into());
    }
    let mut records = vec![];
    let mut negative_checks = 0;
    let mut product_total = 0u64;
    let mut final_total = 0u64;
    for c in cases {
        let id = c["id"].as_str().ok_or("id")?;
        let dir = study.join("inputs").join(id);
        let input = read(&dir.join("original.json"))?;
        let wire = read(&dir.join("proof.json"))?;
        let basis = read(&dir.join("basis.json"))?;
        for (name, data) in [
            ("original.json", &input),
            ("proof.json", &wire),
            ("basis.json", &basis),
        ] {
            if Some(hash(data).as_str()) != c["files"][name].as_str() {
                return Err("frozen input hash".into());
            }
        }
        let p = c["prime"].as_u64().ok_or("prime")?;
        let n = c["n"].as_u64().ok_or("dimension")? as usize;
        let key = format!("p{p}-n{n}");
        if Some(hash(&input).as_str()) != fixed["input_sha256"][&key].as_str() {
            return Err("original fixed6input pin".into());
        }
        let iv: Value = serde_json::from_slice(&input).map_err(|e| e.to_string())?;
        let wv: Value = serde_json::from_slice(&wire).map_err(|e| e.to_string())?;
        let dag = c["dag"].as_bool().ok_or("dag")?;
        if iv["prime"] != p
            || iv["variables"].as_array().ok_or("variables")?.len() != n
            || wv["version"] != if dag { 2 } else { 1 }
        {
            return Err("input field/dimension/version".into());
        }
        let original = check(0, dag, &input, &wire)?;
        let copied = check(1, dag, &input, &wire)?;
        let heap = check(2, dag, &input, &wire)?;
        let (products, final_entries) = candidate::packed::heap_work();
        if original != copied || original != heap || !native_basis_matches(&basis, &original)? {
            return Err("basis/Stats differential mismatch".into());
        }
        if heap["stats"]["referenced_term_products"] != json!(products) || final_entries > products
        {
            return Err("heap complete-product accounting".into());
        }
        let mut canaries = vec![];
        for kind in [
            "basis_coefficient",
            "cofactor_coefficient",
            "future_reference",
            "missing_pair",
            "nonstandard_pair",
            "duplicate_cofactor",
            "changed_input_constant",
        ] {
            let mut bad = wv.clone();
            let mut bad_input = iv.clone();
            match kind {
                "basis_coefficient" => {
                    let t = &mut bad["basis"][0]["polynomial"][0]["coefficient"];
                    *t = json!(if t.as_u64().unwrap() == 1 { 2 } else { 1 });
                }
                "cofactor_coefficient" => {
                    let t = &mut bad["basis"][0]["proof"][0]["coefficient"];
                    *t = json!(if t.as_u64().unwrap() == 1 { 2 } else { 1 });
                }
                "future_reference" => bad["basis"][0]["proof"][0]["generator"] = json!(1u64 << 30),
                "missing_pair" => {
                    bad["pair_reductions"]
                        .as_array_mut()
                        .unwrap()
                        .pop()
                        .ok_or("empty pairs")?;
                }
                "nonstandard_pair" => {
                    let left = bad["pair_reductions"][0]["left"].as_u64().unwrap() as usize;
                    let right = bad["pair_reductions"][0]["right"].as_u64().unwrap() as usize;
                    let a = lead(&bad["basis"][left]);
                    let b = lead(&bad["basis"][right]);
                    let lcm: Vec<_> = a.iter().zip(&b).map(|(a, b)| (*a).max(*b)).collect();
                    let sa: Vec<_> = lcm.iter().zip(a).map(|(x, y)| x - y).collect();
                    let sb: Vec<_> = lcm.iter().zip(b).map(|(x, y)| x - y).collect();
                    bad["pair_reductions"][0]["product_criterion"] = json!(false);
                    bad["pair_reductions"][0]["proof"] = json!([{"generator":left,"exponents":sa,"coefficient":1},{"generator":right,"exponents":sb,"coefficient":p-1}]);
                }
                "duplicate_cofactor" => {
                    let first = bad["basis"][0]["proof"][0].clone();
                    bad["basis"][0]["proof"].as_array_mut().unwrap().push(first);
                }
                "changed_input_constant" => {
                    let f = bad_input["polynomials"][0]
                        .as_array_mut()
                        .ok_or("original polynomial")?;
                    if let Some(t) = f.iter_mut().find(|t| {
                        t["exponents"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .all(|e| e.as_u64() == Some(0))
                    }) {
                        let c = t["coefficient"].as_i64().unwrap();
                        t["coefficient"] = json!((c + 1).rem_euclid(p as i64));
                    } else {
                        f.push(json!({"exponents":vec![0;n],"coefficient":1}));
                    }
                }
                _ => unreachable!(),
            }
            let bi = serde_json::to_vec(&bad_input).unwrap();
            let bw = serde_json::to_vec(&bad).unwrap();
            let mut errors = vec![];
            for mode in 0..3 {
                errors.push(
                    check(mode, dag, &bi, &bw)
                        .err()
                        .ok_or_else(|| format!("{id}: accepted{kind} mode{mode}"))?,
                );
                negative_checks += 1;
            }
            if errors[0] != errors[1]
                || errors[0] != errors[2]
                || (kind == "nonstandard_pair" && !errors[0].contains("nonstandard"))
            {
                return Err(format!("{id}: diagnostic mismatch{kind}"));
            }
            canaries.push(json!({"kind":kind,"all3_rejected":true,"diagnostic":errors[0]}));
        }
        for (name, data) in [
            ("original.json", &input),
            ("proof.json", &wire),
            ("basis.json", &basis),
        ] {
            if read(&dir.join(name))? != *data {
                return Err("original corpus mutated".into());
            }
        }
        product_total += products as u64;
        final_total += final_entries as u64;
        records.push(json!({"case":id,"basis_and_all_Stats_equal":true,"native_basis_matches":true,"old_tree_updates":products,"heap_pushes":products,"heap_final_map_entries":final_entries,"checked_sha256":hash(&serde_json::to_vec(&original).unwrap()),"canaries":canaries}));
    }
    if negative_checks != 3024 {
        return Err("negative roster count".into());
    }
    let report = json!({"schema":"heap-whole-proof-differential/v1","proofs":144,"positive_replays":432,"negative_mutations":1008,"negative_replays":negative_checks,"records":records,"old_identity_map_updates":product_total,"heap_pushes":product_total,"heap_final_map_entries":final_total,"all_source_data_unchanged":true,"all_basis_Stats_and_native_bases_match":true,"timing_observations":0,"full_call_speedup_established":false,"original_15x_achieved":false,"scope":"isolated checker prototype, guarded-scan pre-main99 proof corpus; no current merged-root/native qualification"});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out)
        .map_err(|e| e.to_string())?;
    f.write_all(&serde_json::to_vec_pretty(&report).unwrap())
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())?;
    println!("144proofs,432complete positive replays,3024exact-diagnostic negative refusals; identity map updates {product_total}, heap final entries {final_total}; no timing or15x claim");
    Ok(())
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let result = if a.len() == 3 {
        main_run(Path::new(&a[1]), Path::new(&a[2]))
    } else {
        Err("usage: probe STUDY OUT".into())
    };
    if let Err(e) = result {
        eprintln!("ERROR: {e}");
        std::process::exit(1)
    }
}
