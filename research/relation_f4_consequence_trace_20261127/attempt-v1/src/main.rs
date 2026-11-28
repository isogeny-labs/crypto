//! Fixed-input degree/matrix diagnostic, deliberately without timing output.
mod check;
use certified_relations::{cheap, Poly as AlgebraPoly, Row, System};
use crypto_lib::cryptanalysis::f4_fp::{
    self, certified, F4Options, F4ResourceLimits, Ordering, Poly,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
    time::{Duration, Instant},
};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn put(path: &Path, b: &[u8]) -> Result<(), String> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(b)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
fn save(path: &Path, v: &Value) -> Result<(), String> {
    put(
        path,
        &serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?,
    )
}
fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}
fn jsonread(path: &Path) -> Result<Value, String> {
    serde_json::from_slice(&read(path)?).map_err(|e| e.to_string())
}
fn native(f: &AlgebraPoly) -> Poly {
    f.iter()
        .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
        .collect()
}
fn pool(f: &[AlgebraPoly], n: usize, p: u64) -> Vec<Row> {
    cheap::consequences(f, n, p)
        .into_iter()
        .flat_map(|c| [c.cubic_difference, c.cubic_syzygy])
        .collect()
}
fn row_wire(r: &Row) -> Value {
    json!({"poly":r.poly.iter().collect::<Vec<_>>(),
        "proof":r.proof.iter().map(|((i,m),c)|json!([i,m,c])).collect::<Vec<_>>()})
}
fn arms(k: usize) -> Vec<Value> {
    let mut out = vec![
        json!({"id":"original-a","aux":[]}),
        json!({"id":"original-aa","aux":[]}),
        json!({"id":"full","aux":(0..k).collect::<Vec<_>>()}),
    ];
    for i in 0..k {
        out.push(json!({"id":format!("single-{i:02}"),"aux":[i]}));
    }
    for i in 0..k {
        out.push(
            json!({"id":format!("omit-{i:02}"),"aux":(0..k).filter(|j|*j!=i).collect::<Vec<_>>()}),
        );
    }
    out
}
fn prepare(pins: &Path, out: &Path) -> Result<(), String> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let pins = jsonread(pins)?;
    let mut cases = vec![];
    let mut calls = 0;
    for p in [2147483647, 9007199254740881] {
        for n in [3, 4, 6] {
            let id = format!("p{p}-n{n}");
            let bytes = read(
                &PathBuf::from(
                    pins["reference_corpus"]
                        .as_str()
                        .ok_or("reference corpus")?,
                )
                .join(format!("{id}-frontier/original.json")),
            )?;
            if Some(hash(&bytes).as_str()) != pins["input_sha256"][&id].as_str() {
                return Err("input hash".into());
            }
            let s: System = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            if s.prime != p || s.variables.len() != n {
                return Err("input field/dimension".into());
            }
            let f = s.decode()?;
            let e = pool(&f, n, p);
            if e.len() != 2 * (n - 1) {
                return Err("declared cubic pool count".into());
            }
            let mut canaries = 0;
            for r in &e {
                if !certified_relations::verify(r, &f, p) {
                    return Err("auxiliary certificate".into());
                }
                let mut bad = r.clone();
                bad.proof.clear();
                if certified_relations::verify(&bad, &f, p) {
                    return Err("cleared proof accepted".into());
                }
                canaries += 1;
                let mut bad = r.clone();
                let m = vec![0; n];
                let v = (bad.poly.get(&m).copied().unwrap_or(0) + 1) % p;
                if v == 0 {
                    bad.poly.remove(&m);
                } else {
                    bad.poly.insert(m, v);
                }
                if certified_relations::verify(&bad, &f, p) {
                    return Err("changed polynomial accepted".into());
                }
                canaries += 1;
            }
            let dir = out.join(&id);
            fs::create_dir(&dir).map_err(|e| e.to_string())?;
            put(&dir.join("original.json"), &bytes)?;
            let aux = json!(e.iter().map(row_wire).collect::<Vec<_>>());
            save(&dir.join("auxiliary-proofs.json"), &aux)?;
            let a = arms(e.len());
            calls += a.len();
            let case = json!({"id":id,"prime":p,"n":n,"original_sha256":hash(&bytes),
            "auxiliary_sha256":hash(&serde_json::to_vec_pretty(&aux).unwrap()),
            "auxiliary_count":e.len(),"certificate_mutations_rejected":canaries,"arms":a});
            save(&dir.join("case.json"), &case)?;
            cases.push(case);
        }
    }
    if calls != 98 {
        return Err("declared call count".into());
    }
    save(
        &out.join("ROSTER.json"),
        &json!({"schema":"fixed-f4-consequence-roster/v1","calls":calls,"cases":cases}),
    )
}
fn run_case(root: &Path, id: &str) -> Result<(), String> {
    let roster = jsonread(&root.join("ROSTER.json"))?;
    let c = roster["cases"]
        .as_array()
        .ok_or("case roster")?
        .iter()
        .find(|c| c["id"] == id)
        .ok_or("undeclared case")?;
    let dir = root.join(id);
    if jsonread(&dir.join("case.json"))? != *c {
        return Err("case roster drift".into());
    }
    let p = c["prime"].as_u64().ok_or("prime")?;
    let n = c["n"].as_u64().ok_or("n")? as usize;
    let bytes = read(&dir.join("original.json"))?;
    if Some(hash(&bytes).as_str()) != c["original_sha256"].as_str() {
        return Err("original drift".into());
    }
    let s: System = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let f = s.decode()?;
    let e = pool(&f, n, p);
    let aux =
        serde_json::to_vec_pretty(&json!(e.iter().map(row_wire).collect::<Vec<_>>())).unwrap();
    if aux != read(&dir.join("auxiliary-proofs.json"))?
        || Some(hash(&aux).as_str()) != c["auxiliary_sha256"].as_str()
        || arms(e.len()) != *c["arms"].as_array().ok_or("arms")?
    {
        return Err("auxiliary or arm drift".into());
    }
    for row in &e {
        if !certified_relations::verify(row, &f, p) {
            return Err("auxiliary reverify".into());
        }
    }
    let original: Vec<Poly> = f.iter().map(native).collect();
    let mut reference = certified::try_packed_replay_basis_with_native_check(
        &original,
        n,
        p,
        certified::ReplayForm::FlatDag,
        certified::ReplayKernels::FrontierPackedBatch,
        certified::NativeCertification::PairGraph,
    )?;
    if !certified::verify_packed_replay_basis(&reference, &original)
        || reference.observed_solving_degree.is_some()
    {
        return Err("reference native public reverify".into());
    }
    let independent = relation_replay_checker::packed::verify_dag_json(
        &reference.original_json,
        &reference.witness_json,
    )?;
    let mut independent_basis: Vec<Poly> = independent
        .basis
        .iter()
        .map(|g| {
            g.iter()
                .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
                .collect()
        })
        .collect();
    let mut exported = reference.basis.clone();
    independent_basis.sort();
    exported.sort();
    if independent_basis != exported {
        return Err("reference independent export".into());
    }
    reference.basis[0][0].1 = (reference.basis[0][0].1 + 1) % p;
    if certified::verify_packed_replay_basis(&reference, &original) {
        return Err("reference mutation accepted".into());
    }
    reference.basis[0][0].1 = (reference.basis[0][0].1 + p - 1) % p;
    let mut bad_proof = reference.witness_json.clone();
    bad_proof[0] = b'!';
    if relation_replay_checker::packed::verify_dag_json(&reference.original_json, &bad_proof)
        .is_ok()
    {
        return Err("wire mutation accepted".into());
    }
    let self_work = check::validate(&reference.basis, &reference.basis, n, p, 5_000_000)?;
    let mut bad = reference.basis.clone();
    bad[0].push((vec![0; n], 1));
    if check::validate(&bad, &reference.basis, n, p, 5_000_000).is_ok()
        || check::validate(&reference.basis, &reference.basis, n, p, 0).is_ok()
    {
        return Err("independent checker mutation/budget accepted".into());
    }
    put(
        &dir.join("reference-original.json"),
        &reference.original_json,
    )?;
    put(&dir.join("reference-proof.json"), &reference.witness_json)?;
    save(
        &dir.join("reference.json"),
        &json!({"basis":reference.basis,"independent_replay":true,
        "public_native_reverify":true,"observed_solving_degree":null,"checker_term_updates":self_work,
        "mutation_canaries_rejected":4,"role":"verification-only setup, not credited as free runtime work"}),
    )?;
    for a in c["arms"].as_array().ok_or("arms")? {
        let arm = a["id"].as_str().ok_or("arm id")?;
        let mut input = original.clone();
        for i in a["aux"].as_array().ok_or("indices")? {
            input.push(native(&e[i.as_u64().ok_or("index")? as usize].poly));
        }
        let mut opts = F4Options::new(Ordering::Grevlex, 8);
        opts.resource_limits = Some(F4ResourceLimits {
            max_pairs: 100000,
            max_symbolic_monomials: 10000,
            max_matrix_entries: 1000000,
        });
        opts.deadline = Some(Instant::now() + Duration::from_secs(10));
        let (tx, rx) = mpsc::channel();
        let r = f4_fp::observe_progress(tx, || f4_fp::f4(&input, n, p, &opts));
        // Intentionally project only algebraic/counter fields. Engine ms and
        // elapsed_ms are neither read nor serialized by this diagnostic.
        let trace: Vec<_> = rx
            .try_iter()
            .map(|s| {
                json!({"event":s.event,"step":s.step,"degree":s.degree,
            "rows":s.rows,"columns":s.columns,"productive_degree_before_step":s.productive_degree,
            "row_reduction_multiplications":s.field_multiplications})
            })
            .collect();
        let uncensored = !r.timed_out
            && r.resource_limit.is_none()
            && r.pairs_above_bound == 0
            && r.staircase_at_stop.is_none()
            && !r.trace_mismatch;
        let (valid, error, work) = if uncensored {
            match check::validate(&r.basis, &reference.basis, n, p, 5_000_000) {
                Ok(w) => (true, None, Some(w)),
                Err(e) => (false, Some(e), None),
            }
        } else {
            (
                false,
                Some("censored: complete-basis validation not admitted".into()),
                None,
            )
        };
        let record = json!({"schema":"fixed-f4-consequence-observation/v1","case":id,"arm":a,
            "all_originals_retained":true,"basis":r.basis,"trace":trace,"inconsistent":r.inconsistent,
            "degree_reached":r.degree_reached,"last_productive_degree_raw":r.solving_degree,
            "highest_productive_degree_raw":r.solving_degree_max,
            "productive_degree_observation":if r.solving_degree_max==0 {None}else{Some(r.solving_degree_max)},
            "intrinsic_regularity":null,"steps":r.steps,"max_rows":r.max_rows,"max_cols":r.max_cols,
            "max_cols_to_solution":r.max_cols_to_solution,"steps_to_solution":r.steps_to_solution,
            "row_reduction_multiplications_only":r.field_ops,"timed_out":r.timed_out,
            "resource_limit":r.resource_limit,"pairs_above_bound":r.pairs_above_bound,
            "staircase_at_stop":r.staircase_at_stop,"trace_mismatch":r.trace_mismatch,
            "uncensored":uncensored,"independent_complete_basis_valid":valid,"validation_error":error,
            "checker_term_updates":work,"timing_claim":false});
        save(&dir.join(format!("{arm}.json")), &record)?;
        eprintln!(
            "{id} {arm}: uncensored={uncensored} validated={valid} productive_degree={}",
            r.solving_degree_max
        );
    }
    Ok(())
}
fn analyze(root: &Path, out: &Path) -> Result<(), String> {
    let roster = jsonread(&root.join("ROSTER.json"))?;
    let mut cases = vec![];
    let mut total = 0;
    for c in roster["cases"].as_array().ok_or("cases")? {
        let id = c["id"].as_str().ok_or("id")?;
        let dir = root.join(id);
        let base = jsonread(&dir.join("original-a.json"))?;
        let mut aa = jsonread(&dir.join("original-aa.json"))?;
        aa["arm"] = base["arm"].clone();
        let aa_equal = aa == base;
        let aa_admissible = aa_equal
            && base["independent_complete_basis_valid"] == true
            && aa["independent_complete_basis_valid"] == true;
        let mut observations = vec![];
        for a in c["arms"].as_array().ok_or("arms")? {
            let v = jsonread(&dir.join(format!("{}.json", a["id"].as_str().ok_or("arm")?)))?;
            if v["arm"] != *a || v["case"] != id || v["timing_claim"] != false {
                return Err("record roster drift".into());
            }
            let comparable = aa_admissible
                && v["independent_complete_basis_valid"] == true
                && base["productive_degree_observation"].is_u64()
                && v["productive_degree_observation"].is_u64();
            let delta = if comparable {
                Some(
                    v["productive_degree_observation"].as_i64().unwrap()
                        - base["productive_degree_observation"].as_i64().unwrap(),
                )
            } else {
                None
            };
            observations.push(json!({"arm":a,"validated":v["independent_complete_basis_valid"],
                "uncensored":v["uncensored"],"resource_limit":v["resource_limit"],"timed_out":v["timed_out"],
                "pairs_above_bound":v["pairs_above_bound"],"productive_degree":v["productive_degree_observation"],
                "degree_difference_vs_original":delta,"max_rows":v["max_rows"],"max_cols":v["max_cols"],
                "steps":v["steps"],"row_reduction_multiplications_only":v["row_reduction_multiplications_only"]}));
            total += 1;
        }
        cases.push(json!({"case":id,"aa_exact_algebraic_records_equal":aa_equal,"aa_admissible":aa_admissible,"observations":observations}));
    }
    if total != 98 {
        return Err("missing observations".into());
    }
    save(
        out,
        &json!({"schema":"fixed-f4-consequence-summary/v1","calls":total,"cases":cases,
        "timing_observations":0,"speedup_established":false,"intrinsic_regularity":null,
        "original_15x_achieved":false,"group_scalar_labels_present":false}),
    )
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let r = match a.get(1).map(String::as_str) {
        Some("prepare") if a.len() == 4 => prepare(Path::new(&a[2]), Path::new(&a[3])),
        Some("case") if a.len() == 4 => run_case(Path::new(&a[2]), &a[3]),
        Some("analyze") if a.len() == 4 => analyze(Path::new(&a[2]), Path::new(&a[3])),
        _ => Err("usage: prepare PINS OUT | case OUT CASE_ID | analyze OUT SUMMARY".into()),
    };
    if let Err(e) = r {
        eprintln!("ERROR: {e}");
        std::process::exit(1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_roster_has_98_calls_and_originals_first() {
        assert_eq!(
            2 * [4, 6, 10].into_iter().map(|k| arms(k).len()).sum::<usize>(),
            98
        );
        for k in [4, 6, 10] {
            let a = arms(k);
            assert_eq!(a[0]["aux"], json!([]));
            assert_eq!(a[1]["aux"], json!([]));
            let sets: std::collections::BTreeSet<_> =
                a[1..].iter().map(|v| v["aux"].to_string()).collect();
            assert_eq!(sets.len(), a.len() - 1);
        }
    }
}
