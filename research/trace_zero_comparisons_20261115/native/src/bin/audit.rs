//! Post-run integrity audit and explicitly exploratory static-domain profiles.
use full_trace_zero_backend::binary_ecc::{curve::point_add, BinaryPoint, TraceZeroVariety};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};
use trace_zero_experiments::{truth, unwire, Case};
use trace_zero_leads::fixture;
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    let input: Value = serde_json::from_slice(&fs::read(&a[0]).unwrap()).unwrap();
    let cases: Vec<Case> = serde_json::from_value(input["cases"].clone()).unwrap();
    let mut known = BTreeMap::new();
    let mut profiles = Vec::new();
    for case in &cases {
        let c = fixture(case.n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let b: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
        let t = unwire(&case.target, c.m);
        let actual = truth(&c, &b, &t).is_some();
        assert_eq!(actual, case.truth);
        known.insert(case.id.clone(), actual);
        if case.kind == "planted" && case.n >= 83 {
            for arm in ["direct", "tau-image", "tau-preimage"] {
                let ps: Vec<_> = b
                    .iter()
                    .map(|p| match arm {
                        "tau-image" => v.cofactor_image(p).unwrap(),
                        "tau-preimage" => v.inverse_cofactor_image(p).unwrap(),
                        _ => p.clone(),
                    })
                    .collect();
                let xs: BTreeSet<_> = ps
                    .iter()
                    .map(|p| match p {
                        BinaryPoint::Affine { x, .. } => x.to_biguint(),
                        _ => panic!(),
                    })
                    .collect();
                let mut all = xs.first().unwrap().clone();
                let mut any = BigUint::default();
                for x in &xs {
                    all &= x;
                    any |= x;
                }
                let changing = all ^ any;
                let fixed = (0..c.m).filter(|i| !changing.bit(*i as u64)).count();
                profiles.push(json!({"n":c.m,"dimension":case.dimension,"presentation":arm,"base_size":b.len(),"distinct_x":xs.len(),"common_fixed_x_bits":fixed,"scope":"Post-run exploratory static profile, not preregistered primary comparison or causal proof"}));
            }
        }
    }
    let mut audits = Vec::new();
    for path in &a[1..] {
        let rows: Vec<Value> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let mut repeat: BTreeMap<String, Value> = BTreeMap::new();
        let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
        let mut replayed = 0;
        for r in &rows {
            let case = cases.iter().find(|c| r["instance"] == c.id).unwrap();
            let result = &r["result"];
            assert_eq!(result["valid"], true);
            if result["outcome"] == "Found" {
                assert!(known[&case.id]);
                if let Some(ps) = result.get("points").filter(|p| p.is_array()) {
                    let wires: Vec<trace_zero_experiments::WirePoint> =
                        serde_json::from_value(ps.clone()).unwrap();
                    let c = fixture(case.n);
                    let base: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
                    let points: Vec<_> = wires.iter().map(|p| unwire(p, c.m)).collect();
                    assert_eq!(points.len(), 3);
                    assert!(points.iter().all(|p| base.contains(p)));
                    assert_eq!(
                        points
                            .iter()
                            .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p)),
                        unwire(&case.target, c.m)
                    );
                    replayed += 1;
                } else {
                    assert_eq!(result["replay"], true);
                }
            } else if result["outcome"] == "Refuted" {
                assert!(!known[&case.id]);
            }
            let k = format!("{}/rep{}", r["arm"].as_str().unwrap(), r["replicate"]);
            let e = repeat.entry(k).or_insert(json!({"rows":0,"decided":0}));
            e["rows"] = json!(e["rows"].as_u64().unwrap() + 1);
            e["decided"] = json!(
                e["decided"].as_u64().unwrap() + u64::from(result["outcome"] != "Inconclusive")
            );
            if let Some(reason) = result["reason"].as_str() {
                *reasons
                    .entry(format!("{}/{}", r["arm"].as_str().unwrap(), reason))
                    .or_default() += 1;
            }
        }
        audits.push(json!({"path":path,"rows":rows.len(),"truth_rechecked":true,"saved_original_point_tuples_replayed":replayed,"repeat_counts":repeat,"reasons":reasons}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Post-run integrity audit using native original group, not an independent signed review or new primary decision rule","cases_truth_rechecked":known.len(),"panels":audits,"exploratory_profiles":profiles})).unwrap());
}
