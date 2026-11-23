//! Public fixed correctness controls; no solver, search, sampling or timing.
use crate::{Circuit, Result, Spec, WitnessInput};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use torus_s4::{Curve, Field};
use torus_s4_tree::{Fp2, TorusParameters};

pub struct Artifacts {
    pub report: Value,
    pub toy_trace: Vec<u8>,
    pub membership_system: Value,
    pub point_system: Value,
    pub first_witness: Vec<String>,
}
fn input(z: Fp2, selector: u64, slack: u64) -> WitnessInput {
    WitnessInput {
        z,
        selector,
        slack,
        branch: 0u32.into(),
        raw_v: BigUint::zero(),
    }
}
fn key(z: &Fp2) -> (BigUint, BigUint) {
    (z.a.clone(), z.b.clone())
}
fn raw_point(z: &Fp2, curve: &Curve) -> Result<(u64, BigUint, BigUint)> {
    let f = &curve.f;
    let half_a = f.mul(&curve.a, &f.inv(&2u32.into())?);
    let center = f.sub(&0u32.into(), &half_a);
    let delta = f.mul(&half_a, &f.inv(&z.a)?);
    let mut points = vec![];
    for (branch, u) in [f.add(&center, &delta), f.sub(&center, &delta)]
        .into_iter()
        .enumerate()
    {
        if let Some(v) = f.sqrt(&curve.rhs(&u))? {
            points.push((branch as u64, u, v));
        }
    }
    if points.len() != 1 {
        return Err(format!(
            "fixed input has {} rational branches",
            points.len()
        ));
    }
    Ok(points.remove(0))
}
pub fn run() -> Result<Artifacts> {
    let toy = Spec {
        p: 59u32.into(),
        h: Fp2 {
            a: 54u32.into(),
            b: 37u32.into(),
        },
        order_factor: 5,
        q: 3,
        k: 2,
        curve_a: None,
    };
    let tf = Field::new(toy.p.clone())?;
    if toy.h.pow(&15u32.into(), &tf) != Fp2::one()
        || toy.h.pow(&3u32.into(), &tf) == Fp2::one()
        || toy.h.pow(&5u32.into(), &tf) == Fp2::one()
    {
        return Err("fixed Fp59 order certificate failed".into());
    }
    let tc = Circuit::compile(&toy)?;
    let mut members = BTreeMap::new();
    for j in 1..=toy.k {
        for t in 0..toy.q {
            let z = toy.h.pow(&(j + toy.order_factor * t).into(), &tf);
            if members.insert(key(&z), j).is_some() {
                return Err("toy member collision".into());
            }
        }
    }
    let mut trace = vec![];
    let mut accepted = vec![];
    for a in 0u64..59 {
        for b in 0u64..59 {
            for j in 1..=toy.k {
                let z = Fp2 {
                    a: a.into(),
                    b: b.into(),
                };
                let w = tc.witness(&input(z.clone(), j - 1, toy.k - j))?;
                let actual = tc.system.check(&w).is_ok();
                let expected = members.get(&key(&z)) == Some(&j);
                if actual != expected {
                    return Err(format!("toy projection mismatch at ({a},{b}),j{j}"));
                }
                trace.push(u8::from(actual));
                if actual {
                    accepted.push(json!({"a":a,"b":b,"j":j}));
                }
            }
        }
    }

    let old = TorusParameters::actual_four()?;
    let spec = Spec {
        p: old.f.p.clone(),
        h: old.h.clone(),
        order_factor: 60824497,
        q: 213156431,
        k: 20633,
        curve_a: None,
    };
    let membership = Circuit::compile(&spec)?;
    let mut point_spec = spec.clone();
    point_spec.curve_a = Some(486662u32.into());
    let point = Circuit::compile(&point_spec)?;
    let curve = Curve {
        f: old.f.clone(),
        a: 486662u32.into(),
    };
    let mut actual_cases = vec![];
    let mut first = None;
    for j in [1, spec.k / 2, spec.k] {
        for t in [0, 1, spec.q - 1] {
            let z = spec.h.pow(&(j + spec.order_factor * t).into(), &old.f);
            let (branch, u, v) = raw_point(&z, &curve)?;
            for sign in [0, 1] {
                let raw_v = if sign == 0 {
                    v.clone()
                } else {
                    old.f.sub(&0u32.into(), &v)
                };
                let mut wi = input(z.clone(), j - 1, spec.k - j);
                wi.branch = branch.into();
                wi.raw_v = raw_v.clone();
                let w = point.witness(&wi)?;
                point.system.check(&w)?;
                let expected = curve.times(&Some((u.clone(), raw_v.clone())), 8)?;
                if expected.is_none() {
                    return Err(format!("fixed input j{j},t{t} has identity cofactor image"));
                }
                let image = point.image(&w)?;
                if image != expected || !curve.contains(&image) {
                    return Err("cofactor image mismatch".into());
                }
                let (iu, iv) = image.unwrap();
                actual_cases.push(json!({"j":j,"t":t,"sign":sign,"branch":branch,
                "z":[z.a.to_string(),z.b.to_string()],"raw_point":[u.to_string(),raw_v.to_string()],
                "cofactor_image":[iu.to_string(),iv.to_string()]}));
                if first.is_none() {
                    first = Some(w);
                }
            }
        }
    }
    let first = first.unwrap();
    let mut rejected_mutations = 0;
    for i in 0..first.len() {
        let mut changed = first.clone();
        changed[i] = old.f.add(&changed[i], &BigUint::one());
        if point.system.check(&changed).is_ok() {
            return Err(format!("accepted variable mutation {i}"));
        }
        rejected_mutations += 1;
    }
    if point.system.check(&first[..first.len() - 1]).is_ok() {
        return Err("short witness accepted".into());
    }
    let mut extra = first.clone();
    extra.push(BigUint::zero());
    if point.system.check(&extra).is_ok() {
        return Err("extra witness value accepted".into());
    }
    let mut noncanonical = first.clone();
    noncanonical[0] = old.f.p.clone();
    if point.system.check(&noncanonical).is_ok() {
        return Err("noncanonical field value accepted".into());
    }

    let z = spec.h.clone();
    let good = membership.witness(&input(z, 0, spec.k - 1))?;
    membership.system.check(&good)?;
    let mut nonbinary = good.clone();
    nonbinary[membership.first_selector().unwrap()] = 2u32.into();
    if membership.system.check(&nonbinary).is_ok() {
        return Err("nonbinary selector accepted".into());
    }
    let mask = (1u64 << membership.width) - 1;
    let overflow_z = spec.h.pow(&(spec.k + 1).into(), &old.f);
    let overflow = membership.witness(&input(
        overflow_z,
        spec.k,
        (spec.k - 1).wrapping_sub(spec.k) & mask,
    ))?;
    if membership.system.check(&overflow).is_ok() {
        return Err("overflow selector accepted".into());
    }
    let mut small = spec.clone();
    small.p = 17u32.into();
    small.k = 18;
    small.order_factor = 37;
    small.q = 1;
    small.h = Fp2::one();
    if Circuit::compile(&small).is_ok() {
        return Err("unsafe selector modulus accepted".into());
    }
    // This is the concrete range-only counterexample when the p bound is omitted.
    let small_field = Field::new(17u32.into())?;
    let invalid_selector = BigUint::from(31u32);
    let small_slack = BigUint::from(3u32);
    let small_cap = BigUint::from(17u32);
    if small_field.add(&invalid_selector, &small_slack)
        != small_field.add(&small_cap, &BigUint::zero())
        || invalid_selector <= small_cap
    {
        return Err("small-modulus counterexample invalid".into());
    }
    let mut pole = input(Fp2::one(), 0, spec.k - 1);
    pole.raw_v = BigUint::zero();
    if point.witness(&pole).is_ok() {
        return Err("zero doubling denominator was not rejected".into());
    }
    Ok(Artifacts {
        report: json!({"schema":"torus-coset-circuit.controls/v1",
        "scope":"fixed circuit correctness only; no solver or timing",
        "toy":{"p":59,"h":[54,37],"order":15,"factor":5,"q":3,"k":2,
            "candidate_selector_cases":trace.len(),"accepted":accepted,"stats":tc.stats(),
            "trace_layout":"((a*59+b)*2+(j-1)), a,b in 0..58; j in 1..2; bytes 0/1"},
        "actual":{"p":spec.p.to_string(),"h":[spec.h.a.to_string(),spec.h.b.to_string()],
            "factor":spec.order_factor,"q":spec.q,"k":spec.k,"cases":actual_cases,
            "membership_stats":membership.stats(),"point_stats":point.stats()},
        "single_variable_mutations_rejected":rejected_mutations,
        "rejections":["short witness","extra value","noncanonical value","nonbinary selector",
            "overflow selector","unsafe small modulus","zero affine-doubling denominator"],
        "compiler_formally_verified":false,"actual_field_and_map_bridge_formalized":false,
        "target_decomposition_implemented":false,"growing_family_certified":false,
        "performance_result":false,"pcc_standing":"unverified"}),
        toy_trace: trace,
        membership_system: membership.system.canonical_json(),
        point_system: point.system.canonical_json(),
        first_witness: first.iter().map(ToString::to_string).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finite_projection_actual_images_and_adversarial_assignments() {
        let a = run().unwrap();
        assert_eq!(a.report["toy"]["candidate_selector_cases"], 6962);
        assert_eq!(a.report["toy"]["accepted"].as_array().unwrap().len(), 6);
        assert_eq!(a.report["actual"]["cases"].as_array().unwrap().len(), 18);
        assert_eq!(a.report["actual"]["point_stats"]["max_degree"], 2);
        assert_eq!(a.toy_trace.iter().filter(|x| **x == 1).count(), 6);
    }
}
