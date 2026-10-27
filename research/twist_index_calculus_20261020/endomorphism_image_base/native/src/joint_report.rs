//! Independent accounting of the completed construction suite, without solving.
use super::*;

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn digest(path: &Path) -> Value {
    let b = std::fs::read(path).unwrap();
    json!({"sha256":hex::encode(hash::sha256::sha256(&b)),"bytes":b.len()})
}
fn zero_exit(run: &Path, name: &str) {
    assert_eq!(
        std::fs::read_to_string(run.join("metadata").join(format!("{name}.exit")))
            .unwrap()
            .trim(),
        "0",
        "nonzero or missing execution: {name}"
    );
}
fn task(run: &Path, t: &Value) -> Value {
    let id = t["id"].as_str().unwrap();
    assert!(id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-'));
    zero_exit(run, id);
    let directory = run.join("exports").join(id);
    let s = read(&directory.join("summary.json"));
    assert_eq!(s["schema"], "joint-image-export/v1");
    assert_eq!(s["status"], "exported-not-solved");
    for k in ["p", "seed", "arm", "query_index", "public_input_sha256"] {
        let tk = if k == "public_input_sha256" {
            "public_sha256"
        } else {
            k
        };
        assert_eq!(s[k], t[tk], "workload mismatch: {id} {k}");
    }
    let public = checked_public(
        Path::new(t["public_path"].as_str().unwrap()),
        t["public_sha256"].as_str().unwrap(),
    );
    assert!(
        t["query_index"].as_u64().unwrap() < public["query_trace"].as_array().unwrap().len() as u64
    );
    assert_eq!(
        s["determinant_evaluation_checks"],
        if s["residual"]["inf"] == true { 0 } else { 16 }
    );
    assert_eq!(
        s["map_coordinate_checks"].as_u64().unwrap(),
        s["source_abscissae"].as_array().unwrap().len() as u64
    );
    let pure = t["arm"] == "l0-peel";
    let systems = s["systems"].as_array().unwrap();
    assert_eq!(systems.len(), if pure { 3 } else { 2 });
    let expected: BTreeSet<_> = if pure {
        [
            "geometric",
            "finite-domain",
            "geometric-cubic-only-diagnostic",
        ]
        .into_iter()
        .collect()
    } else {
        ["geometric", "finite-domain"].into_iter().collect()
    };
    assert_eq!(
        systems
            .iter()
            .map(|v| v["encoding"].as_str().unwrap())
            .collect::<BTreeSet<_>>(),
        expected
    );
    let mut checked = Vec::new();
    for entry in systems {
        let encoding = entry["encoding"].as_str().unwrap();
        let jp = directory.join(format!("{encoding}.json"));
        let mp = directory.join(format!("{encoding}.ms"));
        let v = read(&jp);
        assert_eq!(v["schema"], "joint-image-system/v1");
        for k in ["p", "seed", "arm", "query_index"] {
            assert_eq!(v[k], t[k]);
        }
        let nv = v["variables"].as_u64().unwrap() as usize;
        assert_eq!(nv, if encoding == "finite-domain" { 4 } else { 5 });
        let polys: Vec<f4_fp::Poly> = serde_json::from_value(v["equations"].clone()).unwrap();
        let p = t["p"].as_u64().unwrap();
        let stats: Vec<_> = polys.iter().map(|poly| {
            let mut seen = BTreeSet::new();
            for (e,c) in poly {
                assert_eq!(e.len(), nv);
                assert!(*c > 0 && *c < p);
                assert!(seen.insert(e));
            }
            json!({"terms":poly.len(),"degree":poly.iter().map(|(e,_)| e.iter().sum::<u32>()).max().unwrap_or(0)})
        }).collect();
        assert_eq!(v["stats"], json!(stats));
        assert_eq!(entry["stats"], v["stats"]);
        assert_eq!(entry["equations"], polys.len());
        let equation_bytes = serde_json::to_vec(&v["equations"]).unwrap();
        assert_eq!(
            v["input_equations_sha256"],
            hex::encode(hash::sha256::sha256(&equation_bytes))
        );
        assert_eq!(
            std::fs::read_to_string(&mp).unwrap(),
            msolve_input(&polys, p, nv)
        );
        checked.push(json!({"encoding":encoding,"json":digest(&jp),"msolve":digest(&mp),"variables":nv,"equations":polys.len(),"stats":stats}));
    }
    json!({"id":id,"p":t["p"],"seed":t["seed"],"arm":t["arm"],"query_index":t["query_index"],"fixed_position":t["fixed_position"],"first_positive_control":t["first_positive_control"],"historical_s5_regression":t["historical_s5_regression"],"summary":digest(&directory.join("summary.json")),"generic_resultant":s["generic_resultant"],"composed_resultant":s["composed_resultant"],"systems":checked,"status":"pass"})
}

pub(super) fn construction(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    assert!(run.join("complete.txt").is_file());
    for phase in ["prepare", "audit", "workload"] {
        zero_exit(run, phase);
    }
    let w = read(&run.join("workload.json"));
    let source_commit = std::fs::read_to_string(run.join("metadata/source-commit.txt")).unwrap();
    assert_eq!(
        source_commit.trim(),
        "054b22b41d8e8c0d4296c9119b06e88df5d95fab"
    );
    let workload_seal = std::fs::read_to_string(run.join("metadata/workload.sha256")).unwrap();
    assert_eq!(
        digest(&run.join("workload.json"))["sha256"],
        workload_seal.split_whitespace().next().unwrap()
    );
    assert_eq!(w["schema"], "joint-image-workload/v1");
    let tasks = w["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 297, "frozen algebra-v1 workload count changed");
    let mut ids = BTreeSet::new();
    let mut case_arms = BTreeSet::new();
    let mut results = Vec::new();
    let mut failures = Vec::new();
    for t in tasks {
        assert!(ids.insert(t["id"].as_str().unwrap()));
        case_arms.insert((
            t["p"].as_u64().unwrap(),
            t["seed"].as_u64().unwrap(),
            t["arm"].as_str().unwrap(),
        ));
        match std::panic::catch_unwind(|| task(run, t)) {
            Ok(r) => results.push(r),
            Err(e) => {
                let error = e
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_else(|| "invalid construction artifact".into());
                failures.push(json!({"id":t["id"],"error":error}));
            }
        }
    }
    assert_eq!(case_arms.len(), 63);
    let audit = read(&run.join("audit/summary.json"));
    assert_eq!(audit["status"], "pass");
    assert_eq!(audit["cases"].as_array().unwrap().len(), 63);
    assert!(audit["cases"]
        .as_array()
        .unwrap()
        .iter()
        .all(|v| v["status"] == "pass"));
    let mut arms = Vec::new();
    for arm in JOINT_ARMS {
        let selected: Vec<_> = results.iter().filter(|v| v["arm"] == arm).collect();
        let range = |key: &str| {
            let vals: Vec<_> = selected
                .iter()
                .map(|v| v["composed_resultant"][key].as_u64().unwrap())
                .collect();
            json!({"min":vals.iter().min(),"max":vals.iter().max()})
        };
        let u: Vec<_> = selected
            .iter()
            .map(|v| {
                v["composed_resultant"]["variable_degrees"][3]
                    .as_u64()
                    .unwrap()
            })
            .collect();
        arms.push(json!({"arm":arm,"tasks":selected.len(),"fixed_position_tasks":selected.iter().filter(|v|v["fixed_position"]==true).count(),"system_files":selected.iter().map(|v|v["systems"].as_array().unwrap().len()).sum::<usize>(),"extension_terms":range("extension_terms"),"total_degree":range("total_degree"),"source_degree":{"min":u.iter().min(),"max":u.iter().max()}}));
    }
    let pass = failures.is_empty();
    let report = json!({"schema":"joint-image-construction-validation/v1","status":if pass{"pass"}else{"partial"},"run":run,"workload":digest(&run.join("workload.json")),"source_commit":std::fs::read_to_string(run.join("metadata/source-commit.txt")).unwrap().trim(),"tasks_requested":tasks.len(),"tasks_passed":results.len(),"failures":failures,"case_arm_sets":case_arms.len(),"system_files":results.iter().map(|v|v["systems"].as_array().unwrap().len()).sum::<usize>(),"arms":arms,"incidence":{"signed_relations":audit["signed_relations"],"presentations_checked":audit["presentations_checked"],"summary":digest(&run.join("audit/summary.json"))},"tasks":results,"scope":"Construction, encoding consistency and earlier numerical incidence only. No S5 root completeness, solver or end-to-end speed claim."});
    write(out, &report);
    println!(
        "{}",
        json!({"status":report["status"],"tasks":report["tasks_passed"],"system_files":report["system_files"],"arms":report["arms"]})
    );
    pass
}
