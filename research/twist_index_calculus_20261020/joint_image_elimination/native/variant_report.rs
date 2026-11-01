//! Record/export consistency for all attempted variant constructions.
//! Does not certify the generic S5 ideal, complete roots or an IC price.
use super::*;
fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn seal(p: &Path) -> Value {
    let b = std::fs::read(p).unwrap();
    json!({"sha256":hex::encode(hash::sha256::sha256(&b)),"bytes":b.len()})
}
fn validate(run: &Path, t: &Value) -> Value {
    let id = t["id"].as_str().unwrap();
    let root = run.join("exports").join(id);
    let s = read(&root.join("summary.json"));
    assert_eq!(s["schema"], "joint-image-variant-export/v1");
    assert_eq!(s["status"], "exported-not-solved");
    for k in ["p", "seed", "arm", "query_index"] {
        assert_eq!(s[k], t[k]);
    }
    assert_eq!(s["public_input_sha256"], t["public_sha256"]);
    let public = checked_public(
        Path::new(t["public_path"].as_str().unwrap()),
        t["public_sha256"].as_str().unwrap(),
    );
    assert!(
        t["query_index"].as_u64().unwrap() < public["query_trace"].as_array().unwrap().len() as u64
    );
    let pure = t["arm"] == "l0-peel";
    let encoding = if pure {
        "elementary-four"
    } else {
        "elementary-three-and-source"
    };
    assert_eq!(s["root_encoding"], encoding);
    if pure {
        assert_eq!(s["four_factor_coefficient_independence"], true);
    }
    let mut systems = Vec::new();
    for name in ["system", "geometric"] {
        let jp = root.join(format!("{name}.json"));
        let mp = root.join(format!("{name}.ms"));
        let v = read(&jp);
        assert_eq!(v["schema"], "joint-image-system/v1");
        for k in ["p", "seed", "arm", "query_index"] {
            assert_eq!(v[k], t[k]);
        }
        assert_eq!(v["root_encoding"], encoding);
        let nv = v["variables"].as_u64().unwrap() as usize;
        assert_eq!(
            nv,
            if pure {
                if name == "system" {
                    4
                } else {
                    5
                }
            } else if name == "system" {
                5
            } else {
                6
            }
        );
        let polys: Vec<f4_fp::Poly> = serde_json::from_value(v["equations"].clone()).unwrap();
        let p = t["p"].as_u64().unwrap();
        let stats:Vec<_>=polys.iter().map(|g|{
            let mut seen=BTreeSet::new();
            for(e,c)in g {assert_eq!(e.len(),nv);assert!(*c>0&&*c<p);assert!(seen.insert(e));}
            json!({"terms":g.len(),"degree":g.iter().map(|(e,_)|e.iter().sum::<u32>()).max().unwrap_or(0)})
        }).collect();
        if let Some(recorded) = v.get("stats") {
            assert_eq!(*recorded, json!(stats));
        }
        let equations_sha = hex::encode(hash::sha256::sha256(
            &serde_json::to_vec(&v["equations"]).unwrap(),
        ));
        if let Some(recorded) = v.get("input_equations_sha256") {
            assert_eq!(*recorded, equations_sha);
        }
        assert_eq!(
            std::fs::read_to_string(&mp).unwrap(),
            msolve_input(&polys, p, nv)
        );
        if name == "system" {
            assert_eq!(s["equation_stats"], json!(stats));
        } else {
            assert_eq!(s["geometric_variables"], nv);
            assert_eq!(s["geometric_equations"], polys.len());
        }
        systems.push(json!({"name":name,"variables":nv,"equations":polys.len(),"stats":stats,
            "equations_sha256":equations_sha,"equation_digest_present_in_input":v.get("input_equations_sha256").is_some(),
            "stats_present_in_input":v.get("stats").is_some(),"json":seal(&jp),"msolve":seal(&mp)}));
    }
    json!({"id":id,"p":t["p"],"seed":t["seed"],"arm":t["arm"],"query_index":t["query_index"],
        "fixed_position":t["fixed_position"],"first_positive_control":t["first_positive_control"],
        "summary":seal(&root.join("summary.json")),"relation_stats":s["relation_stats"],"unreduced_relation_stats":s["unreduced_relation_stats"],
        "map_or_symmetry_checks":s["map_or_symmetry_checks"],"systems":systems,"status":"record-consistency-pass"})
}
pub(super) fn construction(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let workload = read(&run.join("workload.json"));
    assert_eq!(workload["schema"], "joint-image-workload/v1");
    let tasks = workload["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 297);
    let mut seen = BTreeSet::new();
    let mut results = Vec::new();
    let mut failed = Vec::new();
    let mut missing = Vec::new();
    let mut invalid = Vec::new();
    let mut executed = 0;
    let mut reused = 0;
    for t in tasks {
        let id = t["id"].as_str().unwrap();
        assert!(id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-'));
        assert!(seen.insert(id));
        let ep = run.join("metadata").join(format!("{id}.exit"));
        if !ep.is_file() {
            missing.push(id);
            continue;
        }
        let exit = std::fs::read_to_string(&ep)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let mode =
            std::fs::read_to_string(run.join("metadata").join(format!("{id}.mode.txt"))).unwrap();
        match mode.trim() {
            "executed-new-export" => executed += 1,
            "reused-existing-complete-export" => {
                assert_eq!(t["p"], 37);
                assert_eq!(t["seed"], 0);
                assert_eq!(t["query_index"], 2);
                reused += 1;
                let original = std::fs::read_to_string(
                    run.join("metadata").join(format!("{id}.reused-from.txt")),
                )
                .unwrap();
                let original = Path::new(original.trim());
                for f in [
                    "summary.json",
                    "system.json",
                    "system.ms",
                    "geometric.json",
                    "geometric.ms",
                ] {
                    assert_eq!(
                        seal(&original.join(f)),
                        seal(&run.join("exports").join(id).join(f))
                    );
                }
            }
            _ => panic!("unknown export mode"),
        }
        if exit != 0 {
            failed.push(json!({"id":id,"exit":exit,"scope":"bounded constructor failure, not zero physical yield"}));
            continue;
        }
        match std::panic::catch_unwind(|| validate(run, t)) {
            Ok(v) => results.push(v),
            Err(_) => invalid.push(id),
        }
    }
    let pass = failed.is_empty()
        && missing.is_empty()
        && invalid.is_empty()
        && results.len() == 297
        && executed == 294
        && reused == 3;
    write(
        out,
        &json!({"schema":"joint-image-variant-construction-report/v1","status":if pass {"pass"}else{"partial"},
        "workload":seal(&run.join("workload.json")),"requested_positions":297,"executed_positions":executed,"reused_positions":reused,
        "successful_records":results,"bounded_failures":failed,"missing":missing,"invalid_records":invalid,
        "scope":"Public-input, workload, arity, coefficient range, equation digest/statistics and exact msolve serialization consistency. Producer map/symmetry check counts are retained, not re-executed here. No independent generic S5-ideal derivation, complete roots, physical-set completeness or price claim."}),
    );
    pass
}
