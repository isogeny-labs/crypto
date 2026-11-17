//! Exhaustive support diagnosis of the frozen n=7 smoke; no timing claims.
use crypto_lib::cryptanalysis::ic_boundary::{
    binary_subspace_factor_base, koblitz_instance, BinaryGroup, CountedGroup, GroupOps,
};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::koblitz_strong_rho::{RawPoint, StrongRho, WalkState};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[allow(dead_code)]
#[path = "../src/bin/ecbench/structured_s4_math.rs"]
mod math;

fn point(p: FastPoint) -> serde_json::Value {
    json!({"x":p.x,"y":p.y,"infinity":p.infinity})
}
// Literal private dp_hash from koblitz_strong_rho.rs; its source hash is
// recorded below. Enumerate the entire finite subgroup, not random walks.
fn dp_hash(x: u64, y: u64) -> u64 {
    let mut v = x.rotate_left(7) ^ y ^ 0x2545_f491_4f6c_dd1d;
    v ^= v >> 33;
    v = v.wrapping_mul(0xff51_afd7_ed55_8ccd);
    v ^= v >> 33;
    v = v.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    v ^ (v >> 33)
}
fn main() {
    let inst = koblitz_instance(0, 7).unwrap();
    assert_eq!((inst.group_order, inst.r, inst.cofactor), (116, 29, 4));
    let group = BinaryGroup(&inst.fast);
    let mut ops = GroupOps::default();
    let bs = math::make_basis(&inst.gf, 2, "geometric", 17);
    let fb = binary_subspace_factor_base(&inst, &bs);
    let base: Vec<_> = fb
        .points
        .iter()
        .map(|&p| {
            let rp = group.mul(&mut ops, p, inst.r);
            json!({"point":point(p),"r_multiple":point(rp)})
        })
        .collect();
    let mut reachable = BTreeSet::new();
    let mut triples = 0u64;
    let mut subgroup_triples = 0u64;
    for &p in &fb.points {
        for &q in &fb.points {
            for &s in &fb.points {
                triples += 1;
                let pq = group.add(&mut ops, p, q);
                let sum = group.add(&mut ops, pq, s);
                let rp = group.mul(&mut ops, sum, inst.r);
                if group.is_identity(&rp) {
                    subgroup_triples += 1;
                    reachable.insert((sum.infinity, sum.x, sum.y));
                }
            }
        }
    }
    let rho = StrongRho::new(inst.koblitz.as_ref().unwrap());
    let mut canonical = BTreeSet::new();
    for k in 0..inst.r {
        let p = group.mul(&mut ops, inst.generator, k);
        let state = rho.canonicalize(WalkState {
            point: RawPoint::from_binary(&inst.fast.lower(p)),
            a: k,
            b: 0,
        });
        let (inf, x, y) = match state.point {
            RawPoint::Infinity => (true, 0, 0),
            RawPoint::Affine { x, y } => (false, x, y),
        };
        canonical.insert((inf, x, y));
    }
    let canonical:Vec<_> = canonical.into_iter().map(|(inf,x,y)| {
        let hash = dp_hash(x,y);
        json!({"infinity":inf,"x":x,"y":y,"dp_hash":format!("{hash:016x}"),"distinguished_dp4":hash&15==0})
    }).collect();
    let eligible = canonical
        .iter()
        .filter(|p| p["distinguished_dp4"] == true)
        .count();
    let hash = |path| format!("{:x}", Sha256::digest(std::fs::read(path).unwrap()));
    println!("{}",serde_json::to_string_pretty(&json!({
        "schema":"ic-structured-small-support/v1","status":"enumerated",
        "spec":"specs/structured-pipeline-n7.json","n":7,"a":0,"r":inst.r,"group_order":inst.group_order,"cofactor":inst.cofactor,
        "basis":bs,"family":"geometric","seed":17,"dimension":2,"factor_base_points":base,
        "ordered_triples_including_repetitions":triples,"triples_in_prime_subgroup":subgroup_triples,
        "distinct_prime_subgroup_sums":reachable.len(),"reachable_prime_subgroup_points":reachable,
        "subgroup_elements_enumerated":inst.r,"canonical_points":canonical,"eligible_dp4_canonical_points":eligible,
        "strong_rho_source_sha256":hash("src/cryptanalysis/koblitz_strong_rho.rs"),
        "base_source_sha256":hash("src/cryptanalysis/ic_boundary.rs"),
        "scope":"Exact finite support diagnosis of frozen n=7 inputs; no cost, tuning or larger-field claim."
    })).unwrap());
}
