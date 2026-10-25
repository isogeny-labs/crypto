//! Qualify x-only fixtures on explicit reference/twist models; no speed claim.
use crypto_lib::binary_ecc::{
    curve::{point_add, point_neg, scalar_mul, BinaryCurve, BinaryPoint},
    F2mElement,
};
use crypto_lib::cryptanalysis::{
    ecbench::claim::id_sha256, koblitz_index_calculus::find_irreducible_sparse, semaev_decomp::Gf2,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};

fn element(x: u64, n: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(x), n)
}
fn span(bs: &[u64]) -> Vec<u64> {
    let mut out = vec![0];
    for &b in bs {
        let len = out.len();
        for i in 0..len {
            out.push(out[i] ^ b);
        }
    }
    out
}
#[allow(dead_code)]
#[path = "ic_algebraic_followup/exact.rs"]
mod exact;
fn trace(gf: &Gf2, mut x: u64) -> u64 {
    let mut t = 0;
    for _ in 0..gf.n {
        t ^= x;
        x = gf.sqr(x);
    }
    assert!(t <= 1);
    t
}
// E0/F2 has four points and Frobenius trace -1. The recurrence is exact.
fn order_e0(n: u32) -> u64 {
    assert!((2..=38).contains(&n));
    let (mut u0, mut u1) = (2i128, -1i128);
    for _ in 2..=n {
        (u0, u1) = (u1, -u1 - 2 * u0);
    }
    u64::try_from((1i128 << n) + 1 - u1).unwrap()
}
fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = vec![];
    let mut p = 2;
    while p <= n / p {
        let mut power = 0;
        while n.is_multiple_of(p) {
            n /= p;
            power += 1;
        }
        if power != 0 {
            out.push((p, power));
        }
        p = if p == 2 { 3 } else { p + 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn roots(gf: &Gf2, quad: &exact::Quadratic<'_>, a: u64, x: u64) -> Vec<u64> {
    let x2 = gf.sqr(x);
    let rhs = gf.mul(x2, x) ^ gf.mul(a, x2) ^ 1;
    let ys = quad.roots(1, x, rhs).unwrap();
    assert!(ys.iter().all(|&y| gf.sqr(y) ^ gf.mul(x, y) == rhs));
    assert_eq!(
        !ys.is_empty(),
        x == 0 || trace(gf, x ^ a ^ gf.sqr(gf.inv(x))) == 0
    );
    ys
}
fn point(x: u64, y: u64, n: u32) -> BinaryPoint {
    BinaryPoint::Affine {
        x: element(x, n),
        y: element(y, n),
    }
}
fn encoded(p: &BinaryPoint) -> Value {
    match p {
        BinaryPoint::Infinity => Value::Null,
        BinaryPoint::Affine { x, y } => {
            json!([
                x.to_biguint().to_u64().unwrap(),
                y.to_biguint().to_u64().unwrap()
            ])
        }
    }
}
struct Model {
    curve: BinaryCurve,
    id: String,
    record: Value,
    a: u64,
    order: u64,
    r: u64,
    unique: bool,
}
fn model(gf: &Gf2, quad: &exact::Quadratic<'_>, a: u64, order: u64) -> Model {
    let factors = factor(order);
    assert_eq!(
        factors.iter().map(|&(p, e)| p.pow(e)).product::<u64>(),
        order
    );
    let r = factors
        .iter()
        .map(|&(p, _)| p)
        .filter(|&p| p > 2)
        .max()
        .unwrap();
    assert!(crypto_lib::cryptanalysis::residual_walk::is_prime_u64(r));
    let h = order / r;
    let r_power = factors.iter().find(|&&(p, _)| p == r).unwrap().1;
    let r_free_cofactor = order / r.pow(r_power);
    let mut curve = BinaryCurve {
        m: gf.n,
        irreducible: find_irreducible_sparse(gf.n).unwrap(),
        a: element(a, gf.n),
        b: element(1, gf.n),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(r),
        cofactor: BigUint::from(h),
    };
    for x in 1..(1u64 << gf.n).min(8193) {
        for y in roots(gf, quad, a, x) {
            let p = point(x, y, gf.n);
            assert!(curve.is_on_curve(&p));
            let mut g = scalar_mul(&curve, &p, &BigUint::from(r_free_cofactor));
            if g != BinaryPoint::Infinity {
                for _ in 0..r_power {
                    let next = scalar_mul(&curve, &g, &BigUint::from(r));
                    if next == BinaryPoint::Infinity {
                        break;
                    }
                    g = next;
                }
                assert!(curve.is_on_curve(&g));
                assert_eq!(
                    scalar_mul(&curve, &g, &BigUint::from(r)),
                    BinaryPoint::Infinity
                );
                curve.generator = g;
                break;
            }
        }
        if curve.generator != BinaryPoint::Infinity {
            break;
        }
    }
    assert_ne!(curve.generator, BinaryPoint::Infinity, "generator scan cap");
    let mut record = json!({
        "field":{"p":2,"n":gf.n,"basis":"polynomial","modulus":gf.irr,
            "element_encoding":"unsigned-integer-polynomial-bits"},
        "curve":{"tag":"bin","model":"y^2+x*y=x^3+a2*x^2+a6",
            "coefficients":{"a1":1,"a2":a,"a3":0,"a4":0,"a6":1},
            "order":order,"trace":((1i128<<gf.n)+1-i128::from(order)) as i64,
            "r":r,"cofactor":h,"generator":encoded(&curve.generator),
            "target_group":"prime-order-subgroup-generated-by-G"}
    });
    let digest = id_sha256(&record).unwrap();
    let id = format!("EC1N{}Cbinh{}", gf.n, &digest[..12]);
    record["curve"]["curve_id"] = json!(id);
    record["registry_status"] = json!("unregistered_research_model");
    record["group_order_factorization"] = json!(factors);
    record["generator_method"] = json!("r_free_cofactor_then_r_power_reduction");
    record["trace_a"] = json!(trace(gf, a));
    record["gcd_r_cofactor"] = json!(gcd(r, h));
    record["unique_order_r_subgroup"] = json!(gcd(r, h) == 1);
    Model {
        curve,
        id,
        record,
        a,
        order,
        r,
        unique: gcd(r, h) == 1,
    }
}
fn r_torsion(model: &Model, p: &BinaryPoint) -> bool {
    assert!(model.curve.is_on_curve(p));
    assert_eq!(
        scalar_mul(&model.curve, p, &BigUint::from(model.order)),
        BinaryPoint::Infinity
    );
    scalar_mul(&model.curve, p, &BigUint::from(model.r)) == BinaryPoint::Infinity
}
fn signed_sums(
    curve: &BinaryCurve,
    options: &[Vec<BinaryPoint>],
    targets: &[BinaryPoint],
) -> (u64, u64) {
    let mut sums = vec![BinaryPoint::Infinity];
    for ps in options {
        sums = sums
            .iter()
            .flat_map(|s| ps.iter().map(move |p| point_add(curve, s, p)))
            .collect();
    }
    assert!(sums.iter().all(|p| curve.is_on_curve(p)));
    (
        sums.len() as u64,
        sums.iter().filter(|p| targets.contains(p)).count() as u64,
    )
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "STUDY");
    let study = Path::new(&args[0]);
    let mut files = vec![study.join("inputs/guess-ell8.jsonl")];
    let mut fresh: Vec<_> = std::fs::read_dir(study.join("inputs/fresh"))
        .unwrap()
        .map(Result::unwrap)
        .map(|p| p.path())
        .filter(|p| p.extension().is_some_and(|e| e == "jsonl"))
        .collect();
    fresh.sort();
    assert_eq!(fresh.len(), 36);
    files.extend(fresh);
    let mut models = BTreeMap::new();
    let mut receipts = vec![];
    let mut sources = vec![];
    let mut totals = [0u64; 6];
    for path in files {
        let bytes = std::fs::read(&path).unwrap();
        let relative = path.strip_prefix(study).unwrap().to_str().unwrap();
        sources.push(json!({"path":relative,"sha256":format!("{:x}",Sha256::digest(&bytes))}));
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let n = rows[0]["n"].as_u64().unwrap() as u32;
        let gf = Gf2::new(&find_irreducible_sparse(n).unwrap());
        let quad = exact::Quadratic::new(&gf);
        let twist_a = (0..n)
            .map(|i| 1u64 << i)
            .find(|&a| trace(&gf, a) == 1)
            .unwrap();
        let e0_order = order_e0(n);
        let pair = [
            model(&gf, &quad, 0, e0_order),
            model(&gf, &quad, twist_a, 2 * (1u64 << n) + 2 - e0_order),
        ];
        for model in &pair {
            models.insert(model.id.clone(), model.record.clone());
        }
        for row in rows {
            let original = relative == "inputs/guess-ell8.jsonl";
            if !original {
                assert_eq!(row["field_modulus"], gf.irr);
            } else {
                assert_eq!(gf.irr, (1 << 22) | 3);
            }
            let x = row["x_r"].as_u64().unwrap();
            let sat = row["solutions"].as_u64().unwrap() != 0;
            let bs: Vec<u64> = serde_json::from_value(row["v_basis"].clone()).unwrap();
            let witness = if original && sat {
                Some(exact::witness(&gf, &bs, x, 3).unwrap())
            } else {
                None
            };
            let mut checked_models = vec![];
            let mut lift_count = 0;
            for model in &pair {
                let ys = roots(&gf, &quad, model.a, x);
                let ps: Vec<_> = ys.iter().map(|&y| point(x, y, n)).collect();
                if ps.len() == 2 {
                    assert_eq!(point_neg(&ps[0]), ps[1]);
                }
                lift_count += ys.len();
                let killed = ps.first().map(|p| r_torsion(model, p));
                for p in &ps {
                    assert_eq!(Some(r_torsion(model, p)), killed);
                }
                let membership = if model.unique { killed } else { None };
                if !ys.is_empty() {
                    totals[0] += 1;
                    match membership {
                        Some(true) => {
                            totals[1] += 1;
                            if sat {
                                totals[3] += 1
                            } else {
                                totals[4] += 1
                            }
                        }
                        Some(false) => totals[2] += 1,
                        None => totals[5] += 1,
                    }
                }
                let mut result = json!({"curve_id":model.id,"y_roots":ys,"killed_by_r":killed,"in_generated_subgroup":membership});
                if original {
                    let mut base_lifted = 0;
                    let mut base_killed = 0;
                    for bx in span(&bs) {
                        let bys = roots(&gf, &quad, model.a, bx);
                        if let Some(&by) = bys.first() {
                            base_lifted += 1;
                            base_killed += u64::from(r_torsion(model, &point(bx, by, n)));
                        }
                    }
                    result["base_curve_abscissae_including_zero"] = json!(base_lifted);
                    result["base_r_torsion_abscissae"] = json!(base_killed);
                    if let Some((xs, aux)) = &witness {
                        let options: Vec<Vec<_>> = xs
                            .iter()
                            .map(|&bx| {
                                roots(&gf, &quad, model.a, bx)
                                    .iter()
                                    .map(|&by| point(bx, by, n))
                                    .collect()
                            })
                            .collect();
                        let (tested, accepted) = signed_sums(&model.curve, &options, &ps);
                        let all_killed = options
                            .iter()
                            .all(|ps| !ps.is_empty() && ps.iter().all(|p| r_torsion(model, p)));
                        result["selected_boolean_witness"] = json!({"summand_xs":xs,"aux_xs":aux,"signed_tuples_tested":tested,"signed_curve_relations":accepted,"all_summands_killed_by_r":all_killed,"usable_generated_subgroup_relations":if model.unique && all_killed {Some(accepted)} else {None}});
                    }
                }
                checked_models.push(result);
            }
            assert_eq!(lift_count, 2);
            receipts.push(json!({"input":relative,"draw":row["draw"],"n":n,"x_r":x,"boolean_solutions":row["solutions"],"models":checked_models}));
        }
    }
    assert_eq!(receipts.len(), 1159);
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-fixture-subgroups/v1","status":"all_lifts_trace_orders_and_membership_checks_agree","sources":sources,"curve_models":models,"targets":receipts,"totals":{"lifted_target_models":totals[0],"in_unique_generated_subgroup":totals[1],"outside_unique_generated_subgroup":totals[2],"sat_in_unique_generated_subgroup":totals[3],"unsat_in_unique_generated_subgroup":totals[4],"generated_subgroup_unknown":totals[5]},"scope":"Reference/twist models only; x-only inputs specify neither model nor subgroup. Non-coprime r/cofactor leaves generated-subgroup membership unknown. Selected SAT witnesses are not complete relation enumeration. No query-law, degree-scaling or cost conclusion."})).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extension_order_and_twist_equal_exhaustive_point_enumeration() {
        for n in 3..=7 {
            let gf = Gf2::new(&find_irreducible_sparse(n).unwrap());
            let a1 = (0..n)
                .map(|i| 1 << i)
                .find(|&a| trace(&gf, a) == 1)
                .unwrap();
            for a in [0, a1] {
                let mut points = 1u64;
                for x in 0..1 << n {
                    for y in 0..1 << n {
                        points += u64::from(
                            gf.sqr(y) ^ gf.mul(x, y)
                                == gf.mul(gf.sqr(x), x) ^ gf.mul(a, gf.sqr(x)) ^ 1,
                        );
                    }
                }
                assert_eq!(
                    points,
                    if a == 0 {
                        order_e0(n)
                    } else {
                        2 * (1 << n) + 2 - order_e0(n)
                    }
                );
            }
        }
    }
    #[test]
    fn generator_and_membership_agree_with_enumerating_actual_subgroup() {
        let gf = Gf2::new(&find_irreducible_sparse(7).unwrap());
        let quad = exact::Quadratic::new(&gf);
        let m = model(&gf, &quad, 0, order_e0(7));
        assert!(m.unique);
        let mut actual = vec![];
        let mut p = BinaryPoint::Infinity;
        for _ in 0..m.r {
            actual.push(p.clone());
            p = point_add(&m.curve, &p, &m.curve.generator);
        }
        assert_eq!(p, BinaryPoint::Infinity);
        for x in 0..128 {
            for y in roots(&gf, &quad, 0, x) {
                let p = point(x, y, 7);
                assert_eq!(r_torsion(&m, &p), actual.contains(&p));
            }
        }
    }
    #[test]
    fn repeated_prime_factor_does_not_fabricate_unique_subgroup_membership() {
        let gf = Gf2::new(&find_irreducible_sparse(10).unwrap());
        let quad = exact::Quadratic::new(&gf);
        let m = model(&gf, &quad, 0, order_e0(10));
        assert!(!m.unique);
        assert!(r_torsion(&m, &m.curve.generator));
        assert_eq!(m.r, 11);
    }
}
