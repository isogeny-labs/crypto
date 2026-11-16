//! Deterministic finite branch audit. This is not a universal proof.
#![recursion_limit = "256"]
#[path = "projective_law_gate.rs"]
mod law;

use five_leads_study::three_leads::{Divisor, Jacobian};
use serde_json::json;
use std::collections::BTreeMap;

fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn points(j: &Jacobian) -> Vec<Divisor> {
    let mut out = Vec::new();
    for x in 0..j.f.p.min(1000) {
        let fx = j
            .polynomial
            .iter()
            .rev()
            .fold(0, |a, &c| j.f.add(j.f.mul(a, x), c));
        if let Some(y) = j.f.sqrt(fx) {
            out.push(j.point(x, y));
            if y != 0 {
                out.push(j.point(x, j.f.neg(y)));
            }
        }
        if out.len() >= 128 {
            break;
        }
    }
    assert!(out.len() >= 32);
    out
}

fn add_branch(map: &mut BTreeMap<String, u64>, branch: &str) {
    *map.entry(branch.to_owned()).or_default() += 1;
}

fn main() {
    let models = std::fs::read_to_string(
        "research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl",
    )
    .expect("42 frozen model receipts");
    let mut primes = std::collections::BTreeSet::new();
    for line in models.lines() {
        let row: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["formula_model_y2_monic_quintic_no_x4"], true);
        assert_eq!(row["sampled_transfer_divisors_valid"], 100);
        assert_eq!(row["sampled_group_law_pairs_valid"], 100);
        let p = row["p"].as_u64().unwrap();
        let normalized: Vec<u64> =
            serde_json::from_value(row["normalized_polynomial"].clone()).expect("model polynomial");
        assert_eq!(law::normal_curve(p).polynomial, normalized);
        assert!(primes.insert(p));
    }
    assert_eq!(primes.len(), 42);
    for p in primes {
        let j = law::normal_curve(p);
        let pts = points(&j);
        let seed = j.add(pts[0], pts[2]);
        assert_eq!(seed.degree, 2);
        let mut states = Vec::with_capacity(2048);
        let mut state = Divisor::O;
        for _ in 0..2048 {
            state = j.add(state, seed);
            if state.degree == 2 {
                states.push(state);
            }
        }
        assert!(states.len() >= 1024);
        let mut rng = p ^ 0x7289_6543_abcd_ef17;
        let n22 = if p == 739 { 200_000 } else { 20_000 };
        let n12 = if p == 739 { 40_000 } else { 4_000 };
        let mut branches22 = BTreeMap::new();
        let mut branches12 = BTreeMap::new();
        let mut failures = 0u64;
        let mut first_failure = None;
        for i in 0..n22 {
            let a = states[(next(&mut rng) as usize) % states.len()];
            let b = states[(next(&mut rng) as usize) % states.len()];
            let (za, wa, zb, wb) = if i & 1 == 0 {
                (1, 1, 1, 1)
            } else {
                (2, 3, 5, 7)
            };
            let aw = law::weight_degree_two(a, za, wa, &j.f);
            let bw = law::weight_degree_two(b, zb, wb, &j.f);
            let (actual, branch) = law::add22_weighted(aw, bw, &j.polynomial, &j.f);
            add_branch(&mut branches22, branch);
            let expected = j.add(a, b);
            let good = actual
                .map(|v| law::to_affine(v, &j.f) == expected)
                .unwrap_or(false);
            if !good {
                failures += 1;
                first_failure.get_or_insert_with(|| {
                    json!({"kind":"2+2","p":p,"a":a,"b":b,"weights":[za,wa,zb,wb],"branch":branch,
                        "expected":expected,"got":actual.map(|v| law::to_affine(v,&j.f))})
                });
            }
        }
        for i in 0..n12 {
            let a = pts[(next(&mut rng) as usize) % pts.len()];
            let b = states[(next(&mut rng) as usize) % states.len()];
            let (za, wa, zb, wb) = if i & 1 == 0 {
                (1, 1, 1, 1)
            } else {
                (2, 3, 5, 7)
            };
            let point = law::D1 {
                x: j.f.mul(a.u[0], j.f.neg(j.f.sq(za))),
                y: j.f.mul(a.v[0], j.f.mul(j.f.pow(za, 5), wa)),
                z: za,
                w: wa,
            };
            let bw = law::weight_degree_two(b, zb, wb, &j.f);
            let (actual, branch) = law::add12_weighted_complete(point, bw, &j.polynomial, &j.f);
            add_branch(&mut branches12, branch);
            let expected = j.add(a, b);
            let good = actual
                .map(|v| law::to_affine(v, &j.f) == expected)
                .unwrap_or(false);
            if !good {
                failures += 1;
                first_failure.get_or_insert_with(|| {
                    json!({"kind":"1+2","p":p,"a":a,"b":b,"weights":[za,wa,zb,wb],"branch":branch,
                        "expected":expected,"got":actual.map(|v| law::to_affine(v,&j.f))})
                });
            }
        }
        println!(
            "{}",
            json!({"schema":"projective-proof-audit/v1","p":p,"seed":format!("{:016x}",p^0x7289_6543_abcd_ef17),
                "state_pool_len":states.len(),"sampled_2plus2":n22,"sampled_1plus2":n12,
                "projective_scales":[[1,1,1,1],[2,3,5,7]],"branches_2plus2":branches22,
                "branches_1plus2":branches12,"failures":failures,"first_failure":first_failure})
        );
        assert_eq!(failures, 0, "weighted arithmetic mismatch at p={p}");
    }
}
