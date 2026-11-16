//! Post-run original-group audit; fresh pair tables shared only inside verifier.
use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryPoint, TraceZeroVariety,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};
use trace_zero_experiments::{unwire, wire, Case, WirePoint};
use trace_zero_leads::fixture;
fn key(p: &BinaryPoint) -> String {
    serde_json::to_string(&wire(p)).unwrap()
}
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    let input: Value = serde_json::from_slice(&fs::read(&a[0]).unwrap()).unwrap();
    let cases: Vec<Case> = serde_json::from_value(input["cases"].clone()).unwrap();
    let mut batches: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, c) in cases.iter().enumerate() {
        batches
            .entry(format!(
                "{}:{}",
                c.n,
                serde_json::to_string(&c.base).unwrap()
            ))
            .or_default()
            .push(i);
    }
    let mut truth = BTreeMap::new();
    let mut bases = Vec::new();
    for ids in batches.values() {
        let first = &cases[ids[0]];
        let c = fixture(first.n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let b: Vec<_> = first.base.iter().map(|p| unwire(p, c.m)).collect();
        assert!(b.iter().all(|p| v.contains(p)));
        let mut pairs = BTreeMap::new();
        for (i, p) in b.iter().enumerate() {
            for (j, q) in b.iter().enumerate().skip(i) {
                pairs.entry(key(&point_add(&c, p, q))).or_insert((i, j));
            }
        }
        for &index in ids {
            let case = &cases[index];
            let t = unwire(&case.target, c.m);
            assert!(v.contains(&t));
            let witness = b.iter().enumerate().find_map(|(k, p)| {
                pairs
                    .get(&key(&point_add(&c, &t, &point_neg(p))))
                    .map(|&(i, j)| vec![i, j, k])
            });
            assert_eq!(witness.is_some(), case.truth, "truth {}", case.id);
            truth.insert(case.id.clone(), witness.is_some());
            if case.kind.starts_with("distinct") {
                assert!(case.truth && t != BinaryPoint::Infinity && !b.contains(&t));
            }
        }
        bases.push(json!({"n":c.m,"dimension":first.dimension,"base_size":b.len(),"cases":ids.len(),"independent_pair_entries":pairs.len()}));
    }
    let mut reports = Vec::new();
    for path in &a[1..] {
        let rows: Vec<Value> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let mut seen = BTreeSet::new();
        let mut replayed = 0;
        let mut groups: BTreeMap<String, Value> = BTreeMap::new();
        let mut repeats: BTreeMap<String, Value> = BTreeMap::new();
        for r in &rows {
            let c = cases.iter().find(|c| r["instance"] == c.id).unwrap();
            let arm = r["arm"].as_str().unwrap();
            let rep = r["replicate"].as_u64().unwrap();
            assert!(rep < 2);
            assert!(seen.insert((c.id.clone(), arm.to_string(), rep)));
            let z = &r["result"];
            assert_eq!(z["valid"], true);
            let outcome = z["outcome"].as_str().unwrap();
            assert_eq!(r["decided"], u64::from(outcome != "Inconclusive"));
            if outcome == "Found" {
                assert!(truth[&c.id]);
                let raw: Vec<WirePoint> = serde_json::from_value(z["points"].clone()).unwrap();
                let curve = fixture(c.n);
                let b: Vec<_> = c.base.iter().map(|p| unwire(p, c.n)).collect();
                let ps: Vec<_> = raw.iter().map(|p| unwire(p, c.n)).collect();
                assert_eq!(ps.len(), 3);
                assert!(ps.iter().all(|p| b.contains(p)));
                assert_eq!(
                    ps.iter()
                        .fold(BinaryPoint::Infinity, |s, p| point_add(&curve, &s, p)),
                    unwire(&c.target, c.n)
                );
                replayed += 1;
            } else if outcome == "Refuted" {
                assert!(!truth[&c.id]);
            } else {
                assert_eq!(outcome, "Inconclusive");
            }
            let mut labels = vec![arm.to_string()];
            if c.id.starts_with("scale-") {
                labels.push(format!("{arm}/new-scale"));
            }
            if c.kind.starts_with("distinct") {
                labels.push(format!("{arm}/distinct"));
                labels.push(format!("{arm}/n{}/d{}/distinct", c.n, c.dimension));
            }
            for label in labels {
                let e = groups
                    .entry(label)
                    .or_insert(json!({"rows":0,"Found":0,"Refuted":0,"Inconclusive":0}));
                e["rows"] = json!(e["rows"].as_u64().unwrap() + 1);
                e[outcome] = json!(e[outcome].as_u64().unwrap() + 1);
            }
            let e = repeats
                .entry(format!("{arm}/rep{rep}"))
                .or_insert(json!({"rows":0,"decided":0}));
            e["rows"] = json!(e["rows"].as_u64().unwrap() + 1);
            e["decided"] =
                json!(e["decided"].as_u64().unwrap() + u64::from(outcome != "Inconclusive"));
        }
        let arms: BTreeSet<_> = rows.iter().map(|r| r["arm"].as_str().unwrap()).collect();
        assert_eq!(rows.len(), cases.len() * arms.len() * 2);
        for c in &cases {
            for arm in &arms {
                for rep in 0..2 {
                    assert!(seen.contains(&(c.id.clone(), arm.to_string(), rep)));
                }
            }
        }
        reports.push(json!({"path":path,"rows":rows.len(),"unique_complete_grid":true,"saved_point_tuples_replayed":replayed,"groups":groups,"repeat_counts":repeats}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Post-run producer audit, not a signed independent review; fresh complete pair table per distinct base, never used by candidate trials","cases_truth_rechecked":truth.len(),"base_profiles":bases,"panels":reports})).unwrap());
}
