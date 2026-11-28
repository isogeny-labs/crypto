//! Finite cofactor-projected interpretation of the original seven x-only fixtures.
use crypto_lib::binary_ecc::{
    curve::{point_add, scalar_mul},
    BinaryCurve, BinaryPoint, F2mElement,
};
use crypto_lib::cryptanalysis::{
    ecbench::claim::id_sha256,
    koblitz_index_calculus::{find_irreducible_sparse, points_with_x},
    residual_walk::is_prime_u64,
    semaev_decomp::Gf2,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};
#[allow(dead_code)]
#[path = "ic_algebraic_followup/exact.rs"]
mod exact;
fn element(x: u64, n: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(x), n)
}
fn span(bs: &[u64]) -> Vec<u64> {
    let mut v = vec![0];
    for &b in bs {
        let len = v.len();
        for i in 0..len {
            v.push(v[i] ^ b);
        }
    }
    v.sort_unstable();
    v
}
fn key(p: &BinaryPoint) -> Option<(u64, u64)> {
    match p {
        BinaryPoint::Infinity => None,
        BinaryPoint::Affine { x, y } => Some((x.raw_bits()[0], y.raw_bits()[0])),
    }
}
fn encode(p: &BinaryPoint) -> Value {
    key(p).map_or(Value::Null, |p| json!([p.0, p.1]))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "STUDY");
    let study = Path::new(&args[0]);
    let source = std::fs::read(study.join("inputs/guess-ell8.jsonl")).unwrap();
    let input_sha = hex::encode(Sha256::digest(&source));
    assert_eq!(
        input_sha,
        "95ee147e1221ce4ef6c98bdd0b808398a01362343ffaa66debc217d6eeff5a9b"
    );
    let inputs: Vec<Value> = std::str::from_utf8(&source)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(inputs.len(), 7);
    let models =
        read(&study.join("pipeline-receipts/fixture-subgroups-enumerated-20261025/receipt.json"));
    let identities = read(&study.join("curve-identity-adapter.v1.json"));
    let n = 22;
    let irr = find_irreducible_sparse(n).unwrap();
    let gf = Gf2::new(&irr);
    let mut rows = vec![];
    for model in models["curve_models"]
        .as_object()
        .unwrap()
        .values()
        .filter(|m| m["field"]["n"] == n)
    {
        assert_eq!(model["field"]["modulus"], gf.irr);
        let c = &model["curve"];
        let r = c["r"].as_u64().unwrap();
        let h = c["cofactor"].as_u64().unwrap();
        assert_eq!(c["order"], r.checked_mul(h).unwrap());
        assert!(is_prime_u64(r));
        let coords = c["generator"].as_array().unwrap();
        let g = BinaryPoint::Affine {
            x: element(coords[0].as_u64().unwrap(), n),
            y: element(coords[1].as_u64().unwrap(), n),
        };
        let curve = BinaryCurve {
            m: n,
            irreducible: irr.clone(),
            a: element(c["coefficients"]["a2"].as_u64().unwrap(), n),
            b: element(1, n),
            generator: g.clone(),
            order: BigUint::from(r),
            cofactor: BigUint::from(h),
        };
        assert!(curve.is_on_curve(&g));
        assert_eq!(
            scalar_mul(&curve, &g, &BigUint::from(r)),
            BinaryPoint::Infinity
        );
        let identity = identities["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|i| i["producer_label"] == c["curve_id"])
            .unwrap();
        for input in &inputs {
            let bs: Vec<_> = input["v_basis"]
                .as_array()
                .unwrap()
                .iter()
                .map(|b| b.as_u64().unwrap())
                .collect();
            let mut inventory = BTreeSet::new();
            let (mut full, mut strict, mut projected_nonzero, mut zeros) = (0u64, 0u64, 0u64, 0u64);
            for x in span(&bs) {
                for p in points_with_x(&curve, &element(x, n)) {
                    assert!(curve.is_on_curve(&p));
                    full += 1;
                    if scalar_mul(&curve, &p, &BigUint::from(r)) == BinaryPoint::Infinity {
                        strict += 1;
                    }
                    let hp = scalar_mul(&curve, &p, &BigUint::from(h));
                    assert_eq!(
                        scalar_mul(&curve, &hp, &BigUint::from(r)),
                        BinaryPoint::Infinity
                    );
                    if let Some(k) = key(&hp) {
                        projected_nonzero += 1;
                        inventory.insert(k);
                    } else {
                        zeros += 1;
                    }
                }
            }
            let xr = input["x_r"].as_u64().unwrap();
            let mut target_points = points_with_x(&curve, &element(xr, n));
            target_points.sort_by_key(key);
            let mut row = json!({"draw":input["draw"],"producer_model_label":c["curve_id"],"curve_uid":identity["canonical_identity"]["curve_uid"],
                "a":c["coefficients"]["a2"],"r":r,"h":h,"full_curve_base_points":full,"strict_prime_subgroup_base_points":strict,
                "projected_nonzero_with_multiplicity":projected_nonzero,"projected_distinct_nonzero_points":inventory.len(),"projected_zero_points":zeros,
                "projected_inventory_sha256":id_sha256(&json!(inventory)).unwrap(),"boolean_solutions":input["solutions"],
                "target_lift":!target_points.is_empty(),"target":null,"projected_target":null,"projected_target_identity":null,
                "projected_target_direct_base_hit":null,"selected_curve_witness_relations":0,"selected_projected_witness_relations":0,
                "selected_all_summands_in_prime_subgroup_relations":0,"complete_query_cost":null});
            if let Some(target) = target_points.first() {
                let ht = scalar_mul(&curve, target, &BigUint::from(h));
                assert_eq!(
                    scalar_mul(&curve, &ht, &BigUint::from(r)),
                    BinaryPoint::Infinity
                );
                row["target"] = encode(target);
                row["projected_target"] = encode(&ht);
                row["projected_target_identity"] = json!(ht == BinaryPoint::Infinity);
                row["projected_target_direct_base_hit"] =
                    json!(key(&ht).is_some_and(|k| inventory.contains(&k)));
                if input["solutions"].as_u64().unwrap() > 0 {
                    let (xs, _) = exact::witness(&gf, &bs, xr, 3).unwrap();
                    let choices: Vec<_> = xs
                        .iter()
                        .map(|&x| points_with_x(&curve, &element(x, n)))
                        .collect();
                    let (mut valid, mut projected, mut all_strict) = (0u64, 0u64, 0u64);
                    for a in &choices[0] {
                        for b in &choices[1] {
                            for d in &choices[2] {
                                let sum = point_add(&curve, &point_add(&curve, a, b), d);
                                if &sum != target {
                                    continue;
                                }
                                valid += 1;
                                let ha = scalar_mul(&curve, a, &BigUint::from(h));
                                let hb = scalar_mul(&curve, b, &BigUint::from(h));
                                let hd = scalar_mul(&curve, d, &BigUint::from(h));
                                assert_eq!(
                                    point_add(&curve, &point_add(&curve, &ha, &hb), &hd),
                                    ht
                                );
                                projected += 1;
                                if [a, b, d].iter().all(|p| {
                                    scalar_mul(&curve, p, &BigUint::from(r))
                                        == BinaryPoint::Infinity
                                }) {
                                    all_strict += 1;
                                }
                            }
                        }
                    }
                    row["selected_witness_x"] = json!(xs);
                    row["selected_curve_witness_relations"] = json!(valid);
                    row["selected_projected_witness_relations"] = json!(projected);
                    row["selected_all_summands_in_prime_subgroup_relations"] = json!(all_strict);
                }
            }
            rows.push(row);
        }
    }
    assert_eq!(rows.len(), 14);
    let total = |field: &str| rows.iter().map(|r| r[field].as_u64().unwrap()).sum::<u64>();
    println!(
        "{}",
        json!({"schema":"ic-cofactor-projection-audit/v1","input_sha256":input_sha,"rows":rows,
        "selected_curve_witness_relations":total("selected_curve_witness_relations"),"selected_projected_witness_relations":total("selected_projected_witness_relations"),
        "selected_all_summands_in_prime_subgroup_relations":total("selected_all_summands_in_prime_subgroup_relations"),
        "status":"all_finite_projected_membership_and_relation_checks_agree",
        "scope":"Original seven fixtures and two explicit models; one canonical target sign and one selected Boolean witness per SAT draw. Projected relations do not supply factor-base logs, a complete query law or full IC/DLP cost."})
    );
}
