//! Fixed complete-addition and raw-label correctness controls, not a solver.
use crate::aggregation::TargetCircuit;
use crate::index::{canonical_point, negate, IndexTable};
use crate::{Circuit, Result, Spec, WitnessInput};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use torus_s4::{Curve, Field, Point};
use torus_s4_tree::{Fp2, TorusParameters};

pub const FIXTURE: &[u8] = include_bytes!("../../../research/curve25519_torus_native_20261115/membership-circuit-v1/native-v1/controls.json");
pub struct Artifacts {
    pub report: Value,
    pub trace: Vec<u8>,
    pub system: Value,
    pub witness: Vec<String>,
}
fn decimal(v: &Value) -> Result<BigUint> {
    v.as_str()
        .ok_or("expected decimal string")?
        .parse()
        .map_err(|_| "bad decimal".into())
}
fn point_json(p: &Point) -> Value {
    match p {
        None => Value::Null,
        Some((u, v)) => json!([u.to_string(), v.to_string()]),
    }
}
fn curve(s: &Spec) -> Result<Curve> {
    Ok(Curve {
        f: Field::new(s.p.clone())?,
        a: s.curve_a.clone().ok_or("missing curve")?,
    })
}
fn total(c: &Curve, points: impl IntoIterator<Item = Point>) -> Result<Point> {
    points.into_iter().try_fold(None, |s, p| c.add(&s, &p))
}
fn actual_inputs(s: &Spec) -> Result<Vec<WitnessInput>> {
    let frozen: Value = serde_json::from_slice(FIXTURE).map_err(|e| e.to_string())?;
    frozen["actual"]["cases"]
        .as_array()
        .ok_or("missing v1 cases")?
        .iter()
        .map(|r| {
            let j = r["j"].as_u64().ok_or("missing j")?;
            Ok(WitnessInput {
                z: Fp2 {
                    a: decimal(&r["z"][0])?,
                    b: decimal(&r["z"][1])?,
                },
                selector: j - 1,
                slack: s.k - j,
                branch: r["branch"].as_u64().ok_or("missing branch")?.into(),
                raw_v: decimal(&r["raw_point"][1])?,
            })
        })
        .collect()
}
fn make_input(s: &Spec, j: u64, t: u64) -> Result<(WitnessInput, Point)> {
    let c = curve(s)?;
    let f = &c.f;
    let z =
        s.h.pow(&(BigUint::from(j) + BigUint::from(s.order_factor) * t), f);
    let half = f.mul(&c.a, &f.inv(&2u32.into())?);
    let center = f.sub(&0u32.into(), &half);
    let delta = f.mul(&half, &f.inv(&z.a)?);
    let mut found = vec![];
    for (branch, u) in [f.add(&center, &delta), f.sub(&center, &delta)]
        .into_iter()
        .enumerate()
    {
        if let Some(v) = f.sqrt(&c.rhs(&u))? {
            found.push((branch, u, v));
        }
    }
    if found.len() != 1 {
        return Err("toy raw branch not unique".into());
    }
    let (branch, u, v) = found.remove(0);
    let point = Some((u, v.clone()));
    Ok((
        WitnessInput {
            z,
            selector: j - 1,
            slack: s.k - j,
            branch: branch.into(),
            raw_v: v,
        },
        point,
    ))
}
fn row_json(row: &[crate::index::IndexedTerm]) -> Value {
    json!(row
        .iter()
        .map(|r| json!({"j":r.j,"t":r.t,"coefficient":r.coefficient,
        "canonical_image":point_json(&r.point)}))
        .collect::<Vec<_>>())
}

pub fn run() -> Result<Artifacts> {
    let old = TorusParameters::actual_four()?;
    let actual = Spec {
        p: old.f.p.clone(),
        h: old.h.clone(),
        order_factor: 60824497,
        q: 213156431,
        k: 20633,
        curve_a: Some(486662u32.into()),
    };
    let ac = curve(&actual)?;
    let member = Circuit::compile(&actual)?;
    let inputs = actual_inputs(&actual)?;
    if inputs.len() != 18 {
        return Err("v1 fixture count changed".into());
    }
    let mut points = vec![];
    for input in &inputs {
        let w = member.witness(input)?;
        member.system.check(&w)?;
        points.push(member.image(&w)?);
    }
    let table = IndexTable::new(&actual)?;
    let mut index_cases = vec![];
    for (i, input) in inputs.iter().enumerate() {
        let w = member.witness(input)?;
        let term = table.decode(&actual, &member, &w)?;
        let expected_t = [0, 1, actual.q - 1][(i / 2) % 3];
        if term.j != input.selector + 1 || term.t != expected_t {
            return Err("actual raw-label recovery mismatch".into());
        }
        index_cases
            .push(json!({"fixture_index":i,"j":term.j,"t":term.t,"coefficient":term.coefficient}));
    }
    let mut cases = vec![];
    for (name, ids, distinct) in [
        ("inverse pair", vec![0, 1], false),
        ("doubling", vec![0, 0], false),
        ("identity intermediate", vec![0, 1, 2], false),
        ("four signed pairs", vec![0, 1, 2, 3, 4, 5, 6, 7], false),
        ("six distinct raw parameters", vec![0, 2, 4, 6, 8, 10], true),
    ] {
        let selected: Vec<_> = ids.iter().map(|i| inputs[*i].clone()).collect();
        let target = total(&ac, ids.iter().map(|i| points[*i].clone()))?;
        let circuit = TargetCircuit::compile(&actual, ids.len(), target.clone(), distinct)?;
        let w = circuit.witness(&selected)?;
        circuit.system.check(&w)?;
        let row = circuit.decode_row(&w, &table)?;
        if (name == "inverse pair" || name == "four signed pairs") && !row.is_empty() {
            return Err("signed raw-index cancellation failed".into());
        }
        cases.push(
            json!({"name":name,"fixture_indices":ids,"target":point_json(&target),
            "stats":circuit.stats(),"row":row_json(&row)}),
        );
    }
    let six_ids = [0, 2, 4, 6, 8, 10];
    let six_inputs: Vec<_> = six_ids.iter().map(|i| inputs[*i].clone()).collect();
    let target = total(&ac, six_ids.iter().map(|i| points[*i].clone()))?;
    let six = TargetCircuit::compile(&actual, 6, target.clone(), true)?;
    let witness = six.witness(&six_inputs)?;
    six.system.check(&witness)?;
    let mut mutation_rejections = 0;
    for i in 0..witness.len() {
        let mut changed = witness.clone();
        changed[i] = ac.f.add(&changed[i], &BigUint::one());
        if six.system.check(&changed).is_ok() {
            return Err(format!("aggregate mutation {i} accepted"));
        }
        mutation_rejections += 1;
    }
    let wrong_target = ac.add(&target, &points[0])?;
    let wrong = TargetCircuit::compile(&actual, 6, wrong_target, true)?;
    if wrong.system.check(&witness).is_ok() {
        return Err("wrong public target accepted".into());
    }
    let duplicate = TargetCircuit::compile(&actual, 2, None, true)?;
    if duplicate
        .witness(&[inputs[0].clone(), inputs[1].clone()])
        .is_ok()
    {
        return Err("distinct raw-index guard accepted repeated parameter".into());
    }
    if six.witness(&six_inputs[..5]).is_ok()
        || six.system.check(&witness[..witness.len() - 1]).is_ok()
    {
        return Err("wrong aggregate witness shape accepted".into());
    }
    let mut cap = actual.clone();
    cap.q = (1u64 << 40) + 1;
    if IndexTable::new(&cap).is_ok() {
        return Err("index resource cap ignored".into());
    }
    if TargetCircuit::compile(&actual, 1, None, true).is_ok() {
        return Err("arity cap ignored".into());
    }

    let toy = Spec {
        p: 101u32.into(),
        h: Fp2 {
            a: 17u32.into(),
            b: 12u32.into(),
        },
        order_factor: 17,
        q: 3,
        k: 8,
        curve_a: Some(95u32.into()),
    };
    let tc = curve(&toy)?;
    let f = &tc.f;
    if toy.h.norm(f) != BigUint::one()
        || toy.h.pow(&51u32.into(), f) != Fp2::one()
        || toy.h.pow(&3u32.into(), f) == Fp2::one()
        || toy.h.pow(&17u32.into(), f) == Fp2::one()
    {
        return Err("fixed toy order51 check failed".into());
    }
    let mut image_group = BTreeSet::new();
    let mut point_count = 1;
    image_group.insert(None);
    for u in 0u32..101 {
        for v in 0u32..101 {
            let p = Some((u.into(), v.into()));
            if tc.contains(&p) {
                point_count += 1;
                image_group.insert(tc.times(&p, 8)?);
            }
        }
    }
    if point_count != 104 || image_group.len() != 13 {
        return Err("fixed toy curve/image order prediction failed".into());
    }
    let toy_member = Circuit::compile(&toy)?;
    let toy_table = IndexTable::new(&toy)?;
    let mut retained = vec![];
    let mut exclusions = vec![];
    let mut toy_labels = vec![];
    for j in 1..=toy.k {
        for t in 0..toy.q {
            let (input, raw) = make_input(&toy, j, t)?;
            let image = tc.times(&raw, 8)?;
            if image.is_none() {
                exclusions.push(json!({"j":j,"t":t,"reason":"identity cofactor image"}));
                if toy_member.witness(&input).is_ok() {
                    return Err("identity leaf guard did not reject".into());
                }
                if canonical_point(&toy, j, t)?.is_some() {
                    return Err("identity raw column replay mismatch".into());
                }
                continue;
            }
            for sign in [0, 1] {
                let mut wi = input.clone();
                let pi = if sign == 0 {
                    image.clone()
                } else {
                    negate(&image, f)
                };
                if sign == 1 {
                    wi.raw_v = f.sub(&BigUint::zero(), &wi.raw_v);
                }
                let w = toy_member.witness(&wi)?;
                toy_member.system.check(&w)?;
                if toy_member.image(&w)? != pi {
                    return Err("toy leaf/cofactor mismatch".into());
                }
                let term = toy_table.decode(&toy, &toy_member, &w)?;
                if term.j != j || term.t != t {
                    return Err("toy index mismatch".into());
                }
                toy_labels.push(json!({"j":j,"t":t,"sign":sign,"image":point_json(&pi),
                "z":[wi.z.a.to_string(),wi.z.b.to_string()],"branch":wi.branch.to_string(),
                "raw_v":wi.raw_v.to_string(),"canonical_coefficient":term.coefficient}));
                retained.push((wi, pi));
            }
        }
    }
    let targets: Vec<_> = image_group.into_iter().collect();
    let circuits: Vec<_> = targets
        .iter()
        .map(|p| TargetCircuit::compile(&toy, 2, p.clone(), true))
        .collect::<Result<_>>()?;
    let mut trace = vec![];
    let mut pairs = 0;
    let mut equal_images = 0;
    let mut inverse_images = 0;
    let mut accepted = 0;
    let mut identity_sums = 0;
    for (left, (li, lp)) in retained.iter().enumerate() {
        for (right, (ri, rp)) in retained.iter().enumerate() {
            if li.z == ri.z {
                continue;
            }
            let expected = tc.add(lp, rp)?;
            let w = circuits[0].witness(&[li.clone(), ri.clone()])?;
            pairs += 1;
            if lp == rp {
                equal_images += 1;
            }
            if lp == &negate(rp, f) {
                inverse_images += 1;
            }
            if expected.is_none() {
                identity_sums += 1;
            }
            for (target, circuit) in targets.iter().zip(&circuits) {
                let accept = circuit.system.check(&w).is_ok();
                if accept != (target == &expected) {
                    return Err(format!("toy target mismatch pair{left},{right}"));
                }
                trace.push(u8::from(accept));
                if trace.len() > 30000 {
                    return Err("fixed control budget exceeded".into());
                }
                if accept {
                    accepted += 1;
                    circuit.decode_row(&w, &toy_table)?;
                }
            }
        }
    }
    if pairs == 0 || equal_images == 0 || inverse_images == 0 || accepted != pairs {
        return Err("fixed toy controls did not exercise required alias/addition cases".into());
    }
    Ok(Artifacts {
        report: json!({"schema":"torus-target-circuit.controls/v2",
            "scope":"fixed correctness only; no root-discovery solver, sampling, benchmark or exponent result",
            "actual":{"index":table.stats(),"index_cases":index_cases,"cases":cases,
                "six_member_stats":six.stats(),"target":point_json(&target)},
            "single_variable_mutations_rejected":mutation_rejections,
            "toy":{"p":101,"A":95,"C":2,"d":99,"h":[17,12],"order":51,"a":17,"q":3,"k":8,
                "curve_points":point_count,"image_group_points":targets.iter().map(point_json).collect::<Vec<_>>(),
                "retained_signed_leaves":toy_labels,"identity_raw_columns_excluded":exclusions,
                "ordered_distinct_raw_pairs":pairs,"equal_image_pairs":equal_images,
                "inverse_image_pairs":inverse_images,"identity_sums":identity_sums,
                "pair_target_checks":trace.len(),"accepted":accepted,"index":toy_table.stats(),
                "trace_layout":"retained_signed_leaves order; outer left, inner right; omit equal z; inner sorted image_group_points including null; 0/1 acceptance"},
            "rejections":["wrong target","repeated raw parameter","wrong member count","short assignment",
                "baby-table cap","arity cap","every single variable increment"],
            "compiler_formally_verified":false,"root_discovery_implemented":false,"pcc_standing":"unverified"}),
        trace,
        system: six.system.canonical_json(),
        witness: witness.iter().map(ToString::to_string).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_public_target_controls_and_raw_index_replay() {
        let a = run().unwrap();
        assert_eq!(a.report["actual"]["index"]["baby_entries"], 14600);
        assert_eq!(a.report["actual"]["six_member_stats"]["max_degree"], 2);
        assert_eq!(a.report["toy"]["curve_points"], 104);
        assert!(a.report["toy"]["pair_target_checks"].as_u64().unwrap() <= 30000);
    }
}
