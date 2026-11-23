//! Fixed known-answer qualification; coefficients are constructed, not discovered.
use crate::{predicate, Result};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use torus_coset_circuit::{index::IndexTable, Spec};
use torus_quotient_query::{
    arithmetic::{c, encoded, Engine, P},
    controls::spec,
    query::{point_json, Query},
};
use torus_s4::{Curve, Point};
use torus_s4_tree::Fp2;
use torus_section_operator::{IndexedPoint, Operator};
const WORK: u64 = 100_000_000;
fn engine(s: &Spec) -> Result<Engine> {
    Engine::new(s.p.clone(), WORK)
}
fn real(x: BigUint) -> Fp2 {
    Fp2 {
        a: x,
        b: 0u32.into(),
    }
}
#[derive(Clone)]
struct Raw {
    j: u64,
    t: u64,
    point: Point,
    xs: [BigUint; 2],
    retained: bool,
}
fn raw(s: &Spec, j: u64, t: u64) -> Result<Raw> {
    let e = engine(s)?;
    let f = &e.f;
    let a = s.curve_a.clone().ok_or("missing A")?;
    let curve = Curve {
        f: f.clone(),
        a: a.clone(),
    };
    let z =
        s.h.pow(&(BigUint::from(j) + BigUint::from(s.order_factor) * t), f);
    let half = f.mul(&a, &f.inv(&2u32.into())?);
    let center = f.sub(&0u32.into(), &half);
    let delta = f.mul(&half, &f.inv(&z.a)?);
    let xs = [f.add(&center, &delta), f.sub(&center, &delta)];
    let mut points = vec![];
    for x in &xs {
        if let Some(y) = f.sqrt(&curve.rhs(x))? {
            if y.is_zero() {
                return Err("unexpected torsion map root".into());
            }
            let negative = f.sub(&0u32.into(), &y);
            points.push(Some((x.clone(), y.min(negative))));
        }
    }
    if points.len() != 1 {
        return Err("map branches do not have exactly one curve point".into());
    }
    let point = points.remove(0);
    let retained = curve.times(&point, 8)?.is_some();
    Ok(Raw {
        j,
        t,
        point,
        xs,
        retained,
    })
}
fn toy_raw(k: u64) -> Result<Vec<Raw>> {
    let s = spec(false, k)?;
    (1..=k)
        .flat_map(|j| (0..s.q).map(move |t| (j, t)))
        .map(|(j, t)| raw(&s, j, t))
        .collect()
}
fn dense(e: &Engine, rows: &[Raw]) -> Result<P> {
    let mut p = vec![c(1)];
    for row in rows {
        for x in &row.xs {
            p = e.school(&p, &[e.neg(&real(x.clone())), c(1)])?;
        }
    }
    Ok(p)
}
fn normalize(op: &Operator, r: &Point, cs: &[BigUint]) -> Result<Vec<BigUint>> {
    let pivot = if r.is_some() { 4 } else { 5 };
    let inverse = op
        .curve
        .f
        .inv(&cs[pivot])
        .map_err(|_| "unsupported leading chart")?;
    Ok(cs.iter().map(|v| op.curve.f.mul(v, &inverse)).collect())
}
fn recipe(s: &Spec, rows: &[Raw], signs: u64) -> Result<(Point, Vec<BigUint>)> {
    let op = Operator::new(s.p.clone(), s.curve_a.clone().unwrap())?;
    let mut r = None;
    let mut terms = vec![];
    for (i, row) in rows.iter().enumerate() {
        let mut point = row.point.clone();
        if signs >> i & 1 != 0 {
            if let Some((_, y)) = &mut point {
                *y = op.curve.f.sub(&0u32.into(), y);
            }
        }
        r = op.curve.add(&r, &point)?;
        terms.push(IndexedPoint {
            column: i as u64 + 1,
            point,
        });
    }
    let result = op.analyze(&terms, &r)?;
    let cs = result
        .section
        .ok_or("no section in supported known-divisor chart")?;
    op.verify_section(&terms, &r, &cs)?;
    Ok((r.clone(), normalize(&op, &r, &cs)?))
}
fn compact(mut v: Value, full: bool) -> Result<Value> {
    let bytes = serde_json::to_vec(&v["original_witness"]).map_err(|e| e.to_string())?;
    v["original_witness_sha256"] = json!(format!("{:x}", Sha256::digest(&bytes)));
    v["original_witness_canonical_json_bytes"] = json!(bytes.len());
    if !full {
        v.as_object_mut()
            .ok_or("original result not object")?
            .remove("original_witness");
    }
    Ok(v)
}
fn checked_row(s: &Spec, rows: &[Raw], signs: u64, full: bool) -> Result<Value> {
    let (r, cs) = recipe(s, rows, signs)?;
    let e = engine(s)?;
    let curve = Curve {
        f: e.f.clone(),
        a: s.curve_a.clone().unwrap(),
    };
    let q = Query::new(&e, s, r.clone(), curve.times(&r, 8)?, cs.clone())?;
    let new = predicate(&q)?;
    if new["accept"] != true {
        return Err(format!(
            "known distinct-row section failed new predicate: {new}"
        ));
    }
    let original = q.extract(&IndexTable::new(s)?)?;
    let got: BTreeSet<_> = original["row"]
        .as_array()
        .ok_or("missing row")?
        .iter()
        .map(|v| (v["j"].as_u64().unwrap(), v["t"].as_u64().unwrap()))
        .collect();
    let expected: BTreeSet<_> = rows.iter().map(|v| (v.j, v.t)).collect();
    if expected.len() != 6 || got != expected {
        return Err("extracted raw labels differ".into());
    }
    Ok(
        json!({"R":point_json(&r),"coefficients":cs.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "labels":rows.iter().map(|x|json!([x.j,x.t])).collect::<Vec<_>>(),"sign_bits":signs,
        "new_predicate":new,"original":compact(original,full)?}),
    )
}
fn split_control() -> Result<Value> {
    let s = spec(false, 8)?;
    let rows = toy_raw(8)?;
    let e = engine(&s)?;
    let p = dense(&e, &rows)?;
    let unique: BTreeSet<_> = rows.iter().flat_map(|v| v.xs.iter().cloned()).collect();
    if unique.len() != 48 || rows.len() != 24 || p.len() != 49 {
        return Err("split/raw injectivity mismatch".into());
    }
    let derivative: P = p
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, v)| e.mul(v, &c(i as u64)))
        .collect::<Result<_>>()?;
    if e.gcd(&p, &derivative)?.len() != 1 {
        return Err("dense E is not squarefree".into());
    }
    let mut out = vec![];
    for row in &rows {
        let x = row.point.as_ref().unwrap().0.clone();
        // Use a degree-one quotient solely to compare the XZ denominator with group multiplication.
        let op = Operator::new(s.p.clone(), s.curve_a.clone().unwrap())?;
        let curve = &op.curve;
        let mut n = x;
        let mut d = BigUint::one();
        for _ in 0..3 {
            let nn = e.f.square(&n);
            let dd = e.f.square(&d);
            let nd = e.f.mul(&n, &d);
            n = e.f.square(&e.f.sub(&nn, &dd));
            d = e.f.scale(
                &e.f.mul(&nd, &e.f.add(&e.f.add(&nn, &e.f.mul(&curve.a, &nd)), &dd)),
                4,
            );
        }
        if d.is_zero() == row.retained {
            return Err("XZ identity classification mismatch".into());
        }
        out.push(json!({"j":row.j,"t":row.t,"point":point_json(&row.point),"two_x":row.xs.iter().map(ToString::to_string).collect::<Vec<_>>(),"retained":row.retained,"D3":d.to_string()}));
    }
    let retained = rows.iter().filter(|v| v.retained).count();
    if retained != 22 {
        return Err("toy retained count mismatch".into());
    }
    Ok(
        json!({"columns":out,"dense_E":encoded(&p),"degree":48,"distinct_Fp_roots":unique.len(),"retained":retained}),
    )
}
fn degree_control(k: u64, r: &Point, cs: &[BigUint]) -> Result<Value> {
    let (rx, ry) = r.as_ref().ok_or("degree audit needs finite R")?;
    let e = engine(&spec(false, k)?)?;
    let p = dense(&e, &toy_raw(k)?)?;
    let ee: Vec<_> = p
        .iter()
        .map(|v| {
            v.a.to_u64()
                .ok_or_else(|| "not toy coefficient".to_string())
        })
        .collect::<Result<_>>()?;
    let cc: Vec<_> = cs
        .iter()
        .map(|v| v.to_u64().ok_or_else(|| "not toy coefficient".to_string()))
        .collect::<Result<_>>()?;
    let pencil = crate::symbolic::norm_pencil(95, rx.to_u64().unwrap(), ry.to_u64().unwrap(), &cc)?;
    let mut result = crate::symbolic::audit(&ee, &pencil)?;
    result["k"] = json!(k);
    Ok(result)
}
fn mutation_control(rows: &[Raw]) -> Result<Value> {
    let s = spec(false, 8)?;
    let (r, cs) = recipe(&s, rows, 0)?;
    let e = engine(&s)?;
    let curve = Curve {
        f: e.f.clone(),
        a: s.curve_a.clone().unwrap(),
    };
    let target = curve.times(&r, 8)?;
    let dense_e = dense(&e, &toy_raw(8)?)?;
    let table = IndexTable::new(&s)?;
    let mut out = vec![];
    for t in 0u32..101 {
        let mut cc = cs.clone();
        cc[3] = t.into();
        let q = match Query::new(&e, &s, r.clone(), target.clone(), cc) {
            Ok(q) => q,
            Err(err) => {
                out.push(json!({"c3":t,"chart_refusal":err}));
                continue;
            }
        };
        let value = predicate(&q)?;
        let (_, rem) = e.divrem(&dense_e, &q.ring.s)?;
        let dense_zero = rem.is_empty();
        if dense_zero != (value["H_zero"] == true) {
            return Err("fast versus dense predicate mismatch".into());
        }
        let accepted = value["accept"] == true;
        let original = if accepted {
            compact(q.extract(&table)?, false)?
        } else {
            Value::Null
        };
        out.push(json!({"c3":t,"new":value,"dense_remainder":encoded(&rem),"original":original}));
    }
    Ok(
        json!({"R":point_json(&r),"other_coefficients":cs.iter().map(ToString::to_string).collect::<Vec<_>>(),"arms":out}),
    )
}
fn repeated_control(rows: &[Raw]) -> Result<Value> {
    let s = spec(false, 8)?;
    let repeated = vec![rows[0].clone(); 6];
    let (r, cs) = recipe(&s, &repeated, 0)?;
    let e = engine(&s)?;
    let curve = Curve {
        f: e.f.clone(),
        a: s.curve_a.clone().unwrap(),
    };
    let q = Query::new(&e, &s, r.clone(), curve.times(&r, 8)?, cs)?;
    let old = q.support()?;
    let new = predicate(&q)?;
    if new["H_zero"] != false {
        return Err("repeated support unexpectedly squarefree".into());
    }
    let error = q
        .extract(&IndexTable::new(&s)?)
        .err()
        .ok_or("original repeated label accepted")?;
    Ok(json!({"old_support":old,"new":new,"original_refusal":error}))
}
fn identity_control(rows: &[Raw]) -> Result<Value> {
    let s = spec(false, 8)?;
    let excluded = toy_raw(8)?
        .into_iter()
        .find(|x| !x.retained)
        .ok_or("no excluded raw point")?;
    let mut selected = rows[..5].to_vec();
    selected.push(excluded);
    let (r, cs) = recipe(&s, &selected, 0)?;
    let e = engine(&s)?;
    let curve = Curve {
        f: e.f.clone(),
        a: s.curve_a.clone().unwrap(),
    };
    let q = Query::new(&e, &s, r.clone(), curve.times(&r, 8)?, cs)?;
    let new = predicate(&q)?;
    if new["H_zero"] != true || new["eightfold_unit"] != false {
        return Err("identity-image guard mismatch".into());
    }
    let error = q
        .extract(&IndexTable::new(&s)?)
        .err()
        .ok_or("identity-image original row accepted")?;
    Ok(json!({"new":new,"original_refusal":error}))
}

fn arm(
    emit: &mut impl FnMut(Value) -> Result<()>,
    name: &str,
    result: Result<Value>,
    all: &mut bool,
) -> Result<()> {
    match result {
        Ok(v) => emit(json!({"case":name,"passed":true,"value":v})),
        Err(e) => {
            *all = false;
            emit(json!({"case":name,"passed":false,"error":e}))
        }
    }
}
pub fn run(emit: &mut impl FnMut(Value) -> Result<()>) -> Result<bool> {
    let mut all = true;
    arm(emit, "toy-split-map-XZ", split_control(), &mut all)?;
    let s = spec(false, 8)?;
    let first: Vec<_> = toy_raw(8)?
        .into_iter()
        .filter(|r| r.retained)
        .take(8)
        .collect();
    let seed: Vec<_> = (1..=6).map(|j| raw(&s, j, 0)).collect::<Result<_>>()?;
    let (r, cs) = recipe(&s, &seed, 0)?;
    for k in 1..=8 {
        arm(
            emit,
            &format!("formal-degree-k{k}"),
            degree_control(k, &r, &cs),
            &mut all,
        )?;
    }
    arm(
        emit,
        "coefficient-pencil-101",
        mutation_control(&seed),
        &mut all,
    )?;
    arm(
        emit,
        "repeated-raw-refusal",
        repeated_control(&first),
        &mut all,
    )?;
    arm(
        emit,
        "identity-image-refusal",
        identity_control(&first),
        &mut all,
    )?;
    let mut ordinal = 0;
    let mut refusals = 0;
    let mut supported = 0;
    let mut first_full = false;
    let mut last_supported = None;
    for mask in 0u64..256 {
        if mask.count_ones() != 6 {
            continue;
        }
        let selected: Vec<_> = first
            .iter()
            .enumerate()
            .filter(|(i, _)| mask >> i & 1 != 0)
            .map(|(_, r)| r.clone())
            .collect();
        for signs in 0..64 {
            ordinal += 1;
            match recipe(&s, &selected, signs) {
                Err(err) if err.contains("two-torsion") || err.contains("leading chart") => {
                    refusals += 1;
                    emit(
                        json!({"case":"toy-signed-support","ordinal":ordinal,"mask":mask,"signs":signs,"supported_chart":false,"refusal":err}),
                    )?;
                }
                Err(err) => {
                    all = false;
                    emit(
                        json!({"case":"toy-signed-support","ordinal":ordinal,"passed":false,"error":err}),
                    )?;
                }
                Ok(_) => {
                    supported += 1;
                    let full = !first_full;
                    first_full = true;
                    let result = checked_row(&s, &selected, signs, full);
                    if result.is_ok() {
                        last_supported = Some((selected.clone(), signs));
                    }
                    arm(
                        emit,
                        &format!("toy-signed-support-{ordinal}"),
                        result,
                        &mut all,
                    )?;
                }
            }
        }
    }
    if ordinal != 1792 || supported == 0 {
        all = false;
    }
    emit(
        json!({"case":"toy-domain-summary","passed":ordinal==1792 && supported>0,"declared":1792,"observed":ordinal,"supported":supported,"chart_refusals":refusals}),
    )?;
    if let Some((selected, signs)) = last_supported {
        arm(
            emit,
            "toy-last-full-witness",
            checked_row(&s, &selected, signs, true),
            &mut all,
        )?;
    }
    let b = 213156431u64 * 20633;
    let d = 2 * b;
    emit(
        json!({"case":"actual-formal-sizes","passed":true,"B":b,"D":d,"pencil_degree":2*(d-5),"method":"integer formula only; actual E not expanded"}),
    )?;
    for k in [31, 20633] {
        let s = spec(true, k)?;
        let result = (|| {
            let mut attempts = vec![];
            for sixth in 6..38 {
                let labels: Vec<_> = (1..=5).chain(std::iter::once(sixth)).collect();
                let selected: Vec<_> = labels
                    .into_iter()
                    .map(|j| raw(&s, j, 0))
                    .collect::<Result<_>>()?;
                match recipe(&s, &selected, 0) {
                    Ok(_) => {
                        return Ok(
                            json!({"attempts":attempts,"result":checked_row(&s,&selected,0,true)?}),
                        )
                    }
                    Err(err) => attempts.push(json!({"sixth":sixth,"error":err})),
                }
            }
            Err(format!(
                "known actual section chart incomplete: {}",
                json!(attempts)
            ))
        })();
        arm(emit, &format!("original-actual-k{k}"), result, &mut all)?;
    }
    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_and_torsion_guard() {
        assert!(split_control().is_ok());
    }
    #[test]
    fn repeated_support_is_not_a_distinct_row() {
        let rows: Vec<_> = toy_raw(8)
            .unwrap()
            .into_iter()
            .filter(|x| x.retained)
            .take(8)
            .collect();
        assert!(repeated_control(&rows).is_ok());
        assert!(identity_control(&rows).is_ok());
    }
}
