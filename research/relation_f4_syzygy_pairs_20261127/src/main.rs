//! Exploratory fixed-panel pair syzygies, exact space gates and actual F4.
#[path = "../../relation_f4_consequence_trace_20261127/src/check.rs"]
mod check;
use certified_relations::{cheap, degree_space as ds, Poly as Algebra, Row, System};
use crypto_lib::cryptanalysis::f4_fp::{
    self as f4, certified, F4Options, F4ResourceLimits, Ordering, Poly,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::{Read, Write},
    path::Path,
    sync::mpsc,
    time::{Duration, Instant},
};
const PRIOR: &str = "research/relation_f4_consequence_trace_20261127";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn bytes(p: &Path) -> Result<Vec<u8>, String> {
    fs::read(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn load(p: &Path) -> Result<Value, String> {
    serde_json::from_slice(&bytes(p)?).map_err(|e| e.to_string())
}
fn put(p: &Path, b: &[u8]) -> Result<(), String> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .map_err(|e| e.to_string())?;
    f.write_all(b)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
fn save(p: &Path, v: &Value) -> Result<(), String> {
    put(p, &serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?)
}
fn native(f: &Algebra) -> Poly {
    f.iter()
        .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
        .collect()
}
fn pool(f: &[Algebra], n: usize, p: u64) -> Vec<Row> {
    cheap::consequences(f, n, p)
        .into_iter()
        .flat_map(|c| [c.cubic_difference, c.cubic_syzygy])
        .collect()
}
fn wire(e: &[Row]) -> Value {
    json!(e.iter().map(|r|json!({"poly":r.poly.iter().collect::<Vec<_>>(),"proof":r.proof.iter().map(|((i,m),c)|json!([i,m,c])).collect::<Vec<_>>()})).collect::<Vec<_>>())
}
fn arms(k: usize) -> Vec<Value> {
    assert!(k >= 4 && k % 2 == 0);
    let mut out = vec![
        json!({"id":"original-a","aux":[]}),
        json!({"id":"original-aa","aux":[]}),
    ];
    let mut seen = BTreeSet::from([Vec::<usize>::new()]);
    let mut push = |id: String, mut ids: Vec<usize>| {
        ids.sort_unstable();
        ids.dedup();
        if seen.insert(ids.clone()) {
            out.push(json!({"id":id,"aux":ids}));
        }
    };
    push("full".into(), (0..k).collect());
    push("differences".into(), (0..k).step_by(2).collect());
    push("syzygies".into(), (1..k).step_by(2).collect());
    for i in 0..k / 2 {
        for j in i + 1..k / 2 {
            push(format!("pair-{i:02}-{j:02}"), vec![2 * i + 1, 2 * j + 1]);
            let mut with: Vec<_> = (0..k).step_by(2).collect();
            with.extend([2 * i + 1, 2 * j + 1]);
            push(format!("pair-differences-{i:02}-{j:02}"), with);
        }
    }
    out
}
fn supported(p: u64) -> bool {
    p == 2147483647
}
fn prepare(out: &Path) -> Result<(), String> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let pins = load(Path::new(
        "research/relation_scan_call_instrument_20261127/SYNTHETIC_INPUT_PINS.json",
    ))?;
    let mut cases = vec![];
    let mut count = 0;
    for p in [2147483647, 9007199254740881] {
        for n in [3, 4, 6] {
            let id = format!("p{p}-n{n}");
            let parent = Path::new(PRIOR).join("data").join(&id);
            let original = bytes(&parent.join("original.json"))?;
            if Some(hash(&original).as_str()) != pins["input_sha256"][&id].as_str() {
                return Err("fixed input hash".into());
            }
            let s: System = serde_json::from_slice(&original).map_err(|e| e.to_string())?;
            if s.prime != p || s.variables.len() != n {
                return Err("input field/dimension".into());
            }
            let f = s.decode()?;
            let e = pool(&f, n, p);
            if e.len() != 2 * (n - 1) {
                return Err("candidate count".into());
            }
            let mut canaries = 0;
            for row in &e {
                if !certified_relations::verify(row, &f, p) {
                    return Err("auxiliary membership".into());
                }
                let mut bad = row.clone();
                bad.proof.clear();
                if certified_relations::verify(&bad, &f, p) {
                    return Err("cleared proof accepted".into());
                }
                canaries += 1;
                let mut bad = row.clone();
                let m = vec![0; n];
                let c = (bad.poly.get(&m).copied().unwrap_or(0) + 1) % p;
                if c == 0 {
                    bad.poly.remove(&m);
                } else {
                    bad.poly.insert(m, c);
                }
                if certified_relations::verify(&bad, &f, p) {
                    return Err("polynomial mutation accepted".into());
                }
                canaries += 1;
            }
            let dir = out.join(&id);
            fs::create_dir(&dir).map_err(|e| e.to_string())?;
            put(&dir.join("original.json"), &original)?;
            let w = serde_json::to_vec_pretty(&wire(&e)).unwrap();
            put(&dir.join("auxiliary-proofs.json"), &w)?;
            let a = arms(e.len());
            count += a.len();
            let c = json!({"id":id,"prime":p,"n":n,"original_sha256":hash(&original),"auxiliary_sha256":hash(&w),"auxiliary_count":e.len(),"certificate_mutations_rejected":canaries,"f4_supported":supported(p),"arms":a});
            save(&dir.join("case.json"), &c)?;
            cases.push(c);
        }
    }
    if count != 82 {
        return Err("declared arm count".into());
    }
    save(
        &out.join("ROSTER.json"),
        &json!({"schema":"syzygy-pair-roster/v1","arms":count,"cases":cases}),
    )
}
fn store_space(path: &Path, s: &ds::Snapshot) -> Result<(), String> {
    let data = serde_json::to_vec(s).map_err(|e| e.to_string())?;
    if data.len() > 128 * 1024 * 1024 {
        return Err("space plaintext cap".into());
    }
    let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    enc.write_all(&data).map_err(|e| e.to_string())?;
    put(path, &enc.finish().map_err(|e| e.to_string())?)
}
fn read_space(path: &Path) -> Result<ds::Snapshot, String> {
    let data = bytes(path)?;
    let mut reader = flate2::read::GzDecoder::new(data.as_slice()).take(128 * 1024 * 1024 + 1);
    let mut plain = vec![];
    reader.read_to_end(&mut plain).map_err(|e| e.to_string())?;
    if plain.len() > 128 * 1024 * 1024 {
        return Err("space plaintext cap".into());
    }
    serde_json::from_slice(&plain).map_err(|e| e.to_string())
}
fn rank_matches(base: Option<usize>, rank: Option<usize>) -> bool {
    base.is_some() && base == rank
}
fn basis_comparison(a: &[Poly], b: &[Poly]) -> bool {
    let clean = |g: &[Poly]| {
        let mut g = g.to_vec();
        for row in &mut g {
            row.sort()
        }
        g.sort();
        g
    };
    clean(a) == clean(b)
}
fn execute(root: &Path, id: &str) -> Result<(), String> {
    let roster = load(&root.join("ROSTER.json"))?;
    let case = roster["cases"]
        .as_array()
        .ok_or("cases")?
        .iter()
        .find(|c| c["id"] == id)
        .ok_or("undeclared case")?;
    let dir = root.join(id);
    if load(&dir.join("case.json"))? != *case {
        return Err("case roster drift".into());
    }
    let p = case["prime"].as_u64().ok_or("prime")?;
    let n = case["n"].as_u64().ok_or("dimension")? as usize;
    let b = bytes(&dir.join("original.json"))?;
    if Some(hash(&b).as_str()) != case["original_sha256"].as_str() {
        return Err("original drift".into());
    }
    let s: System = serde_json::from_slice(&b).map_err(|e| e.to_string())?;
    let f = s.decode()?;
    let e = pool(&f, n, p);
    let w = serde_json::to_vec_pretty(&wire(&e)).unwrap();
    if w != bytes(&dir.join("auxiliary-proofs.json"))?
        || Some(hash(&w).as_str()) != case["auxiliary_sha256"].as_str()
        || arms(e.len()) != *case["arms"].as_array().ok_or("arms")?
    {
        return Err("auxiliary/arm drift".into());
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
        return Err("reference native checks".into());
    }
    let checked = relation_replay_checker::packed::verify_dag_json(
        &reference.original_json,
        &reference.witness_json,
    )?;
    let rb: Vec<Poly> = checked
        .basis
        .iter()
        .map(|g| {
            g.iter()
                .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
                .collect()
        })
        .collect();
    if !basis_comparison(&reference.basis, &rb) {
        return Err("reference independent export".into());
    }
    let parent = Path::new(PRIOR).join("data").join(id);
    if reference.original_json != bytes(&parent.join("reference-original.json"))?
        || reference.witness_json != bytes(&parent.join("reference-proof.json"))?
    {
        return Err("prior reference differs".into());
    }
    reference.basis[0][0].1 = (reference.basis[0][0].1 + 1) % p;
    if certified::verify_packed_replay_basis(&reference, &original) {
        return Err("mutated reference accepted".into());
    }
    reference.basis[0][0].1 = (reference.basis[0][0].1 + p - 1) % p;
    let mut bad = reference.witness_json.clone();
    bad[0] = b'!';
    if relation_replay_checker::packed::verify_dag_json(&reference.original_json, &bad).is_ok() {
        return Err("bad wire accepted".into());
    }
    check::validate(&rb, &rb, n, p, 5_000_000)?;
    if check::validate(&rb, &rb, n, p, 0).is_ok() {
        return Err("checker zero budget accepted".into());
    }
    let mut bad = rb.clone();
    bad[0].push((vec![0; n], 1));
    if check::validate(&bad, &rb, n, p, 5_000_000).is_ok() {
        return Err("checker bad basis accepted".into());
    }
    put(
        &dir.join("reference-original.json"),
        &reference.original_json,
    )?;
    put(&dir.join("reference-proof.json"), &reference.witness_json)?;
    save(
        &dir.join("reference.json"),
        &json!({"basis":rb,"native_public_reverify":true,"independent_wire_replay":true,"prior_exact_wire_match":true,"canaries_rejected":4,"observed_solving_degree":null}),
    )?;
    let mut baselines = vec![];
    for d in [2, 3, 4] {
        let snapshot = ds::snapshot(&f, n, p, d, &e, ds::Mode::Closed, ds::Limits::default())?;
        let rank = ds::replay(&f, n, p, &e, &snapshot, ds::Limits::default())?.exact_space_rank;
        store_space(&dir.join(format!("baseline-D{d}.json.gz")), &snapshot)?;
        baselines.push(rank);
    }
    for arm in case["arms"].as_array().ok_or("arms")? {
        let name = arm["id"].as_str().ok_or("arm id")?;
        let indices: Vec<_> = arm["aux"]
            .as_array()
            .ok_or("aux")?
            .iter()
            .map(|i| i.as_u64().map(|i| i as usize).ok_or("index"))
            .collect::<Result<_, _>>()?;
        let extra: Vec<_> = indices.iter().map(|i| e[*i].clone()).collect();
        let mut gate = vec![];
        let mut pass = true;
        for d in [3, 2, 4] {
            let k = d - 2;
            let (rank, file) = if indices.len() == e.len() {
                (baselines[k], format!("baseline-D{d}.json.gz"))
            } else {
                let snapshot =
                    ds::snapshot(&f, n, p, d, &extra, ds::Mode::Closed, ds::Limits::default())?;
                let rank = ds::replay(&f, n, p, &extra, &snapshot, ds::Limits::default())?
                    .exact_space_rank;
                let name = format!("{name}-D{d}.json.gz");
                store_space(&dir.join(&name), &snapshot)?;
                (rank, name)
            };
            gate.push(json!({"degree":d,"baseline_rank":baselines[k],"rank":rank,"snapshot":file}));
            if !rank_matches(baselines[k], rank) {
                pass = false;
                break;
            }
        }
        let observation = if supported(p) {
            let mut input = original.clone();
            input.extend(extra.iter().map(|r| native(&r.poly)));
            let mut opts = F4Options::new(Ordering::Grevlex, 8);
            opts.resource_limits = Some(F4ResourceLimits {
                max_pairs: 100000,
                max_symbolic_monomials: 10000,
                max_matrix_entries: 1000000,
            });
            opts.deadline = Some(Instant::now() + Duration::from_secs(10));
            let (tx, rx) = mpsc::channel();
            let r = f4::observe_progress(tx, || f4::f4(&input, n, p, &opts));
            let trace:Vec<_>=rx.try_iter().map(|v|json!({"event":v.event,"step":v.step,"degree":v.degree,"rows":v.rows,"columns":v.columns,"productive_degree_before_step":v.productive_degree,"row_reduction_multiplications":v.field_multiplications})).collect();
            let uncensored = !r.timed_out
                && r.resource_limit.is_none()
                && r.pairs_above_bound == 0
                && r.staircase_at_stop.is_none()
                && !r.trace_mismatch;
            let (valid, error, work) = if uncensored {
                match check::validate(&r.basis, &rb, n, p, 5_000_000) {
                    Ok(w) => (true, None, Some(w)),
                    Err(e) => (false, Some(e), None),
                }
            } else {
                (false, Some("censored".into()), None)
            };
            json!({"basis":r.basis,"trace":trace,"inconsistent":r.inconsistent,"degree_reached":r.degree_reached,"highest_productive_degree_raw":r.solving_degree_max,"last_productive_degree_raw":r.solving_degree,"productive_degree_observation":if r.solving_degree_max==0{None}else{Some(r.solving_degree_max)},"steps":r.steps,"max_rows":r.max_rows,"max_cols":r.max_cols,"steps_to_solution":r.steps_to_solution,"max_cols_to_solution":r.max_cols_to_solution,"row_reduction_multiplications_only":r.field_ops,"timed_out":r.timed_out,"resource_limit":r.resource_limit,"pairs_above_bound":r.pairs_above_bound,"staircase_at_stop":r.staircase_at_stop,"trace_mismatch":r.trace_mismatch,"uncensored":uncensored,"independent_complete_basis_valid":valid,"validation_error":error,"checker_term_updates":work})
        } else {
            Value::Null
        };
        save(
            &dir.join(format!("{name}.json")),
            &json!({"schema":"syzygy-pair-observation/v1","case":id,"arm":arm,"all_originals_retained":true,"polynomial_space_gate_pass":pass,"space_checks":gate,"scalar_gate":"not evaluated: no P/Q labels","f4_supported":supported(p),"unsupported_reason":if supported(p){None}else{Some("existing regular-F4 p<2^32 Barrett assertion; not called")},"observation":observation,"intrinsic_regularity":null,"timing_claim":false}),
        )?;
        eprintln!(
            "{id} {name}: degree={} polynomial_gate={pass}",
            observation["productive_degree_observation"]
        );
    }
    Ok(())
}
fn replay(root: &Path, out: &Path) -> Result<(), String> {
    let roster = load(&root.join("ROSTER.json"))?;
    let mut cases = vec![];
    let mut calls = 0;
    let mut unsupported = 0;
    for case in roster["cases"].as_array().ok_or("cases")? {
        let id = case["id"].as_str().ok_or("id")?;
        let p = case["prime"].as_u64().ok_or("prime")?;
        let n = case["n"].as_u64().ok_or("n")? as usize;
        let dir = root.join(id);
        let original = bytes(&dir.join("original.json"))?;
        if Some(hash(&original).as_str()) != case["original_sha256"].as_str() {
            return Err("replay input drift".into());
        }
        let s: System = serde_json::from_slice(&original).map_err(|e| e.to_string())?;
        let f = s.decode()?;
        let e = pool(&f, n, p);
        let reference_original = bytes(&dir.join("reference-original.json"))?;
        let rs: System = serde_json::from_slice(&reference_original).map_err(|e| e.to_string())?;
        if rs.prime != p || rs.variables.len() != n || rs.decode()? != f {
            return Err("reference original mismatch".into());
        }
        let checked = relation_replay_checker::packed::verify_dag_json(
            &reference_original,
            &bytes(&dir.join("reference-proof.json"))?,
        )?;
        let rb: Vec<Poly> = checked
            .basis
            .iter()
            .map(|g| {
                g.iter()
                    .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
                    .collect()
            })
            .collect();
        check::validate(&rb, &rb, n, p, 5_000_000)?;
        let mut baseline = vec![];
        for d in [2, 3, 4] {
            let s = read_space(&dir.join(format!("baseline-D{d}.json.gz")))?;
            if s.degree != d || s.mode != ds::Mode::Closed {
                return Err("wrong baseline cap/mode".into());
            }
            baseline.push(ds::replay(&f, n, p, &e, &s, ds::Limits::default())?.exact_space_rank);
        }
        let mut obs = vec![];
        for arm in case["arms"].as_array().ok_or("arms")? {
            let name = arm["id"].as_str().ok_or("id")?;
            let v = load(&dir.join(format!("{name}.json")))?;
            if v["arm"] != *arm
                || v["case"] != id
                || v["all_originals_retained"] != true
                || v["timing_claim"] != false
            {
                return Err("observation drift".into());
            }
            let indices: Vec<_> = arm["aux"]
                .as_array()
                .ok_or("aux")?
                .iter()
                .map(|i| i.as_u64().unwrap() as usize)
                .collect();
            let extra: Vec<_> = indices.iter().map(|i| e[*i].clone()).collect();
            let mut pass = true;
            let checks = v["space_checks"].as_array().ok_or("space checks")?;
            for (i, c) in checks.iter().enumerate() {
                if i >= 3 || c["degree"] != [3, 2, 4][i] {
                    return Err("degree check order".into());
                }
                let d = c["degree"].as_u64().unwrap() as usize;
                let snap = read_space(&dir.join(c["snapshot"].as_str().ok_or("snapshot")?))?;
                if snap.degree != d || snap.mode != ds::Mode::Closed {
                    return Err("wrong snapshot cap/mode".into());
                }
                let rank =
                    ds::replay(&f, n, p, &extra, &snap, ds::Limits::default())?.exact_space_rank;
                if c["rank"] != json!(rank) || c["baseline_rank"] != json!(baseline[d - 2]) {
                    return Err("rank drift".into());
                }
                if !rank_matches(baseline[d - 2], rank) {
                    if i + 1 != checks.len() {
                        return Err("continued beyond obstruction".into());
                    }
                    pass = false;
                }
            }
            if checks.is_empty()
                || (pass && checks.len() != 3)
                || v["polynomial_space_gate_pass"] != pass
            {
                return Err("incomplete/unsupported space decision".into());
            }
            let r = &v["observation"];
            if supported(p) {
                calls += 1;
                if r["independent_complete_basis_valid"] == true {
                    let basis: Vec<Poly> =
                        serde_json::from_value(r["basis"].clone()).map_err(|e| e.to_string())?;
                    check::validate(&basis, &rb, n, p, 5_000_000)?;
                } else if r["uncensored"] == true {
                    return Err("uncensored basis validation failed".into());
                }
            } else {
                unsupported += 1;
                if !r.is_null() || v["f4_supported"] != false {
                    return Err("fabricated unsupported observation".into());
                }
            }
            obs.push(json!({"arm":arm,"polynomial_space_gate_pass":pass,"space_checks":v["space_checks"],"f4_supported":supported(p),"productive_degree":r["productive_degree_observation"],"complete_basis_valid":r["independent_complete_basis_valid"],"max_rows":r["max_rows"],"max_cols":r["max_cols"],"steps":r["steps"],"row_reduction_multiplications_only":r["row_reduction_multiplications_only"]}));
        }
        let mut aa = true;
        let mut prior_match = true;
        if supported(p) {
            let a = load(&dir.join("original-a.json"))?;
            let b = load(&dir.join("original-aa.json"))?;
            aa = a["observation"] == b["observation"];
            for name in ["original-a", "original-aa", "full"] {
                let v = load(&dir.join(format!("{name}.json")))?;
                let old = load(
                    &Path::new(PRIOR)
                        .join("data")
                        .join(id)
                        .join(format!("{name}.json")),
                )?;
                for (k, value) in v["observation"].as_object().ok_or("observation")? {
                    if old[k] != *value {
                        prior_match = false;
                    }
                }
            }
            if !aa || !prior_match {
                return Err("fresh A/A or prior original/full comparison mismatch".into());
            }
        }
        cases.push(json!({"case":id,"fresh_aa_exact":if supported(p){Some(aa)}else{None},"prior_original_full_exact":if supported(p){Some(prior_match)}else{None},"baseline_ranks_D2_D3_D4":baseline,"observations":obs}));
    }
    if calls != 41 || unsupported != 41 {
        return Err("arm count".into());
    }
    save(
        out,
        &json!({"schema":"syzygy-pair-summary/v1","declared_arms":82,"actual_f4_calls":calls,"unsupported_f4_not_called":unsupported,"cases":cases,"intrinsic_regularity":null,"group_scalar_labels_present":false,"timing_observations":0,"speedup_established":false,"original_15x_achieved":false}),
    )?;
    println!(
        "41 F4 arms,41 explicit unsupported arms; all space snapshots and completed bases replayed"
    );
    Ok(())
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let r = match a.get(1).map(String::as_str) {
        Some("prepare") if a.len() == 3 => prepare(Path::new(&a[2])),
        Some("case") if a.len() == 4 => execute(Path::new(&a[2]), &a[3]),
        Some("replay") if a.len() == 4 => replay(Path::new(&a[2]), Path::new(&a[3])),
        _ => Err("usage: prepare OUT | case OUT ID | replay OUT SUMMARY".into()),
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
    fn exact_82_arm_roster_unique_except_aa() {
        let mut total = 0;
        for (k, count) in [(4, 5), (6, 11), (10, 25)] {
            let a = arms(k);
            assert_eq!(a.len(), count);
            let sets: BTreeSet<_> = a[1..].iter().map(|a| a["aux"].to_string()).collect();
            assert_eq!(sets.len(), a.len() - 1);
            total += count;
        }
        assert_eq!(total * 2, 82);
    }
    #[test]
    fn absent_or_unequal_space_ranks_never_pass() {
        assert!(rank_matches(Some(3), Some(3)));
        assert!(!rank_matches(None, None));
        assert!(!rank_matches(Some(3), None));
        assert!(!rank_matches(Some(3), Some(2)));
    }
    #[test]
    fn unsupported_fixed_prime_never_enters_regular_f4() {
        assert!(supported(2147483647));
        assert!(!supported(9007199254740881));
        assert!(!supported(101));
    }
    #[test]
    fn equivalent_export_orders_match_and_changed_coefficients_do_not() {
        let a = vec![vec![(vec![1], 1), (vec![0], 3)]];
        let b = vec![vec![(vec![0], 3), (vec![1], 1)]];
        assert!(basis_comparison(&a, &b));
        assert!(!basis_comparison(&a, &[vec![(vec![0], 4), (vec![1], 1)]]));
    }
}
