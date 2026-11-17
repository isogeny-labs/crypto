//! Frozen known-answer instance generation and matched native solver instrument.
use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_neg, scalar_mul},
    BinaryCurve, BinaryPoint, F2mElement, TraceZeroVariety,
};
use full_trace_zero_backend::cryptanalysis::trace_zero_relations::{
    decompose_full_trace_zero_with_encoding, TraceZeroMembershipEncoding as Encoding,
    TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits,
};
use num_bigint::RandBigInt;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeMap, time::Duration};
use trace_zero_leads::{fixture, window};

pub type WirePoint = Option<[String; 2]>;
pub fn wire(p: &BinaryPoint) -> WirePoint {
    match p {
        BinaryPoint::Infinity => None,
        BinaryPoint::Affine { x, y } => Some([
            x.to_biguint().to_str_radix(16),
            y.to_biguint().to_str_radix(16),
        ]),
    }
}
pub fn unwire(p: &WirePoint, n: u32) -> BinaryPoint {
    match p {
        None => BinaryPoint::Infinity,
        Some([x, y]) => BinaryPoint::Affine {
            x: F2mElement::from_hex(x, n),
            y: F2mElement::from_hex(y, n),
        },
    }
}
fn key(p: &BinaryPoint) -> String {
    serde_json::to_string(&wire(p)).unwrap()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    pub id: String,
    pub n: u32,
    pub dimension: usize,
    pub kind: String,
    pub seed: u64,
    pub base: Vec<WirePoint>,
    pub target: WirePoint,
    pub truth: bool,
    pub truth_witness: Option<Vec<usize>>,
}

pub fn subspace_base(c: &BinaryCurve, d: usize) -> Vec<BinaryPoint> {
    let v = TraceZeroVariety::new(c).unwrap();
    let basis = window(c, d);
    let mut out = Vec::new();
    for word in 1..1usize << d {
        let mut x = F2mElement::zero(c.m);
        for (i, b) in basis.iter().enumerate() {
            if word >> i & 1 != 0 {
                x = x.add(b);
            }
        }
        let inv = x.flt_inverse(&c.irreducible).unwrap();
        let rhs = x.add(&inv.square(&c.irreducible));
        if rhs.trace(&c.irreducible) != 0 {
            continue;
        }
        let y = x.mul(&rhs.half_trace(&c.irreducible).unwrap(), &c.irreducible);
        let p = BinaryPoint::Affine { x, y };
        assert!(c.is_on_curve(&p));
        if v.contains(&p) {
            out.push(p.clone());
            out.push(point_neg(&p));
        }
    }
    out
}

/// Complete generic pair-table reference, never exposed to candidate solvers.
pub fn truth(c: &BinaryCurve, base: &[BinaryPoint], t: &BinaryPoint) -> Option<Vec<usize>> {
    let mut pairs = BTreeMap::new();
    for (i, p) in base.iter().enumerate() {
        for (j, q) in base.iter().enumerate() {
            pairs.entry(key(&point_add(c, p, q))).or_insert((i, j));
        }
    }
    for (k, p) in base.iter().enumerate() {
        if let Some(&(i, j)) = pairs.get(&key(&point_add(c, t, &point_neg(p)))) {
            return Some(vec![i, j, k]);
        }
    }
    None
}

pub fn prepare() -> Vec<Case> {
    let mut cases = Vec::new();
    for n in [7, 9, 83, 131] {
        for d in [4, 5] {
            let c = fixture(n);
            let base = subspace_base(&c, d);
            assert!(
                !base.is_empty(),
                "empty preregistered base at n={n},d={d}; amend before any solver panel"
            );
            let generator = if c.generator == BinaryPoint::Infinity {
                base[0].clone()
            } else {
                c.generator.clone()
            };
            for (i, kind) in ["planted", "cancellation", "uniform-0", "uniform-1"]
                .iter()
                .enumerate()
            {
                let seed = 202611090000 + n as u64 * 100 + d as u64 * 10 + i as u64;
                let mut rng = StdRng::seed_from_u64(seed);
                let target = match *kind {
                    "planted" => (0..3).fold(BinaryPoint::Infinity, |s, _| {
                        point_add(&c, &s, &base[rng.gen_range(0..base.len())])
                    }),
                    "cancellation" => base[rng.gen_range(0..base.len())].clone(),
                    _ => scalar_mul(&c, &generator, &rng.gen_biguint_below(&c.order)),
                };
                let witness = truth(&c, &base, &target);
                if *kind == "planted" || *kind == "cancellation" {
                    assert!(witness.is_some());
                }
                cases.push(Case {
                    id: format!("n{n}-d{d}-{kind}"),
                    n,
                    dimension: d,
                    kind: kind.to_string(),
                    seed,
                    base: base.iter().map(wire).collect(),
                    target: wire(&target),
                    truth: witness.is_some(),
                    truth_witness: witness,
                });
            }
        }
    }
    // A controlled UNSAT case: no sum of three +/-G is zero in the order11 group.
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let g = subspace_base(&c, 4)
        .into_iter()
        .find(|p| v.contains(p))
        .unwrap();
    let base = vec![g.clone(), point_neg(&g)];
    assert!(truth(&c, &base, &BinaryPoint::Infinity).is_none());
    cases.insert(
        0,
        Case {
            id: "negative-control-n5".into(),
            n: 5,
            dimension: 0,
            kind: "negative-control".into(),
            seed: 20261109,
            base: base.iter().map(wire).collect(),
            target: None,
            truth: false,
            truth_witness: None,
        },
    );
    cases
}

pub const ARMS: [&str; 4] = ["explicit", "certified", "partial-trace", "certified-aa"];
pub fn execute(case: &Case, arm: &str) -> Value {
    let c = fixture(case.n);
    let v = TraceZeroVariety::new(&c).unwrap();
    let base: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
    let target = unwire(&case.target, c.m);
    assert!(v.contains(&target) && base.iter().all(|p| v.contains(p)));
    let encoding = match arm {
        "explicit" => Encoding::ExplicitPoints,
        "certified" | "certified-aa" => Encoding::CertifiedDomain,
        "partial-trace" => Encoding::CertifiedDomainWithPartialTrace,
        _ => panic!("unknown arm"),
    };
    let report = decompose_full_trace_zero_with_encoding(
        &v,
        &base,
        &target,
        3,
        TraceZeroRelationLimits {
            max_variables: 500_000,
            max_domain_clauses: 1_000_000,
            max_polynomial_terms: 2_000_000,
            max_models: 1024,
            max_patterns: 2,
            conflict_budget: 2000,
            wall_budget: Some(Duration::from_millis(750)),
        },
        encoding,
    );
    let replay = if let Some(path) = &report.indices {
        path.len() == 3
            && path.iter().all(|&i| i < base.len())
            && path
                .iter()
                .fold(BinaryPoint::Infinity, |s, &i| point_add(&c, &s, &base[i]))
                == target
    } else {
        false
    };
    let valid = match report.disposition {
        Disposition::Found => case.truth && replay,
        Disposition::Refuted => !case.truth,
        Disposition::Inconclusive => true,
    };
    json!({"status":if valid{"ok"}else{"invalid"},"outcome":format!("{:?}",report.disposition),
        "truth":case.truth,"replay":replay,"valid":valid,"conflicts":report.conflicts,"models":report.models,
        "patterns_attempted":report.patterns_attempted,"patterns_refuted":report.patterns_refuted,
        "peak_variables":report.peak_variables,"membership_equations":report.full_membership_equations,
        "partial_trace_equations":report.partial_trace_equations,"reason":report.reason})
}

pub fn summarize(rows: &[Value]) -> Value {
    let mut arms = BTreeMap::new();
    for arm in ARMS {
        let r: Vec<_> = rows.iter().filter(|r| r["arm"] == arm).collect();
        let count = |outcome: &str| {
            r.iter()
                .filter(|r| r["result"]["outcome"] == outcome)
                .count()
        };
        arms.insert(arm,json!({"rows":r.len(),"found":count("Found"),"refuted":count("Refuted"),"inconclusive":count("Inconclusive"),
            "invalid":r.iter().filter(|r|r["result"]["valid"]!=true).count(),
            "conflicts_total":r.iter().map(|r|r["result"]["conflicts"].as_u64().unwrap_or(0)).sum::<u64>()}));
    }
    let mut aa_mismatches = Vec::new();
    for a in rows.iter().filter(|r| r["arm"] == "certified") {
        if let Some(b) = rows.iter().find(|r| {
            r["arm"] == "certified-aa"
                && r["instance"] == a["instance"]
                && r["replicate"] == a["replicate"]
        }) {
            if a["result"]["outcome"] != b["result"]["outcome"]
                || a["result"]["conflicts"] != b["result"]["conflicts"]
            {
                aa_mismatches.push(json!({"instance":a["instance"],"replicate":a["replicate"],"a":a["result"],"b":b["result"]}));
            }
        }
    }
    let mut strata: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for row in rows {
        strata
            .entry(format!(
                "{}/n{}/{}",
                row["arm"].as_str().unwrap(),
                row["n"],
                row["kind"].as_str().unwrap()
            ))
            .or_default()
            .push(row);
    }
    let strata:BTreeMap<_,_>=strata.into_iter().map(|(key,r)|{
        let count=|outcome:&str|r.iter().filter(|r|r["result"]["outcome"]==outcome).count();
        (key,json!({"rows":r.len(),"found":count("Found"),"refuted":count("Refuted"),"inconclusive":count("Inconclusive")}))
    }).collect();
    json!({"schema":"trace-zero-panel-summary/v1","scope":"descriptive fixed workload; no population, speed or scaling conclusion",
        "arms":arms,"strata":strata,"aa_mismatches":aa_mismatches,"rows":rows.len()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    #[test]
    fn partial_trace_preserves_small_complete_group_relations() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let base = subspace_base(&c, 4);
        let g = base[0].clone();
        for k in 0..11u32 {
            let t = scalar_mul(&c, &g, &BigUint::from(k));
            let expected = truth(&c, &base, &t).is_some();
            let r = decompose_full_trace_zero_with_encoding(
                &v,
                &base,
                &t,
                3,
                TraceZeroRelationLimits::default(),
                Encoding::CertifiedDomainWithPartialTrace,
            );
            assert_ne!(r.disposition, Disposition::Inconclusive);
            assert_eq!(r.disposition == Disposition::Found, expected);
        }
    }
    #[test]
    fn wire_round_trip_and_small_subspace_lifts_are_exact() {
        for n in [5, 7, 9] {
            let c = fixture(n);
            let v = TraceZeroVariety::new(&c).unwrap();
            for p in subspace_base(&c, 3) {
                assert!(v.contains(&p));
                assert_eq!(unwire(&wire(&p), n), p);
            }
        }
    }
}
