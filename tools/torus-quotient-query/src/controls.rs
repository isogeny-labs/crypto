//! Fixed prospective known-answer controls, not a sampling campaign.
use crate::arithmetic::{c, encoded, is_zero, z, Alg, Engine, Ring, P};
use crate::query::{admitted_moduli, point_json, Query};
use crate::Result;
use num_bigint::BigUint;
use num_traits::Zero;
use serde_json::{json, Value};
use torus_coset_circuit::{index::IndexTable, Spec};
use torus_s4::{Curve, Point};
use torus_s4_tree::{Fp2, TorusParameters};
use torus_section_operator::{IndexedPoint, Operator};
const WORK: u64 = 100_000_000;
fn engine(p: &BigUint) -> Result<Engine> {
    Engine::new(p.clone(), WORK)
}
fn power_poly(e: &Engine, x: u64, d: usize) -> Result<P> {
    let line = [e.neg(&c(x)), c(1)];
    let mut s = vec![c(1)];
    for _ in 0..d {
        s = e.school(&s, &line)?;
    }
    Ok(s)
}
fn coeffs(n: usize, seed: u64) -> P {
    (0..n)
        .map(|i| Fp2 {
            a: ((i as u64 * 7 + seed) % 97).into(),
            b: ((i as u64 * 11 + seed * 3) % 97).into(),
        })
        .collect()
}
fn alg_json(a: &Alg) -> Value {
    json!({"a":encoded(&a.a),"b":encoded(&a.b)})
}
fn compare_product(p: &BigUint, d: usize, n: u64, start: u64) -> Result<Value> {
    let e = engine(p)?;
    let mut r = Ring::new(&e, power_poly(&e, 2, d)?)?;
    let u = r.x()?;
    r.tau = r.sub(&r.scale(&r.mul(&u, &u)?, &c(4))?, &r.constant(c(2)));
    let t = r.aadd(
        &r.v(),
        &Alg {
            a: r.x()?,
            b: r.constant(z()),
        },
    );
    let rho = Fp2 {
        a: 3u32.into(),
        b: 2u32.into(),
    };
    let fast = r.geometric(&t, &rho, start, n)?;
    let direct = r.direct(&t, &rho, start, n)?;
    if fast != direct {
        return Err("full geometric remainder mismatch".into());
    }
    Ok(
        json!({"p":p.to_string(),"degree":d,"n":n,"start":start,"fast":alg_json(&fast),"direct":alg_json(&direct),"counts":e.stats()}),
    )
}
pub fn spec(actual: bool, k: u64) -> Result<Spec> {
    if actual {
        let t = TorusParameters::actual_four()?;
        Ok(Spec {
            p: t.f.p,
            h: t.h,
            order_factor: 60824497,
            q: 213156431,
            k,
            curve_a: Some(486662u32.into()),
        })
    } else {
        Ok(Spec {
            p: 101u32.into(),
            h: Fp2 {
                a: 17u32.into(),
                b: 12u32.into(),
            },
            order_factor: 17,
            q: 3,
            k,
            curve_a: Some(95u32.into()),
        })
    }
}
fn raw_point(s: &Spec, j: u64, t: u64) -> Result<Point> {
    let e = engine(&s.p)?;
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
    let mut points = vec![];
    for x in [f.add(&center, &delta), f.sub(&center, &delta)] {
        if let Some(y) = f.sqrt(&curve.rhs(&x))? {
            if !y.is_zero() {
                let ny = f.sub(&0u32.into(), &y);
                points.push(Some((x, y.min(ny))));
            }
        }
    }
    if points.len() != 1 {
        return Err("raw map branch not unique".into());
    }
    let p = points.remove(0);
    if curve.times(&p, 8)?.is_none() {
        return Err("excluded identity image".into());
    }
    Ok(p)
}
struct Recipe {
    r: Point,
    cs: Vec<BigUint>,
    labels: Vec<(u64, u64)>,
    attempts: Value,
}
fn make_recipe(s: &Spec, preferred: Option<Vec<(u64, u64)>>) -> Result<Recipe> {
    let op = Operator::new(s.p.clone(), s.curve_a.clone().unwrap())?;
    let labels = preferred.unwrap_or_else(|| (1..=s.k.min(40)).map(|j| (j, 0)).collect());
    let mut candidates = vec![];
    let mut exclusions = vec![];
    for (j, t) in labels {
        match raw_point(s, j, t) {
            Ok(point) => candidates.push((j, t, point)),
            Err(err) => exclusions.push(json!({"j":j,"t":t,"error":err})),
        }
    }
    let mut attempts = vec![];
    for sixth in 5..candidates.len().min(37) {
        let selected: Vec<_> = candidates[..5]
            .iter()
            .chain(std::iter::once(&candidates[sixth]))
            .cloned()
            .collect();
        let mut r = None;
        for (_, _, p) in &selected {
            r = op.curve.add(&r, p)?;
        }
        let terms: Vec<_> = selected
            .iter()
            .enumerate()
            .map(|(i, (_, _, p))| IndexedPoint {
                column: i as u64 + 1,
                point: p.clone(),
            })
            .collect();
        let analysis = op.analyze(&terms, &r);
        let cs = match analysis {
            Ok(a) => a.section,
            Err(err) => {
                attempts.push(json!({"sixth":sixth,"error":err}));
                continue;
            }
        };
        let Some(cs) = cs else {
            attempts.push(json!({"sixth":sixth,"error":"no section for known divisor"}));
            continue;
        };
        if (r.is_some() && cs[4].is_zero()) || (r.is_none() && cs[5].is_zero()) {
            attempts.push(json!({"sixth":sixth,"error":"unsupported leading chart"}));
            continue;
        }
        op.verify_section(&terms, &r, &cs)?;
        attempts.push(json!({"sixth":sixth,"selected":selected.iter().map(|(j,t,p)|json!({"j":j,"t":t,"point":point_json(p)})).collect::<Vec<_>>(),"R":point_json(&r)}));
        return Ok(Recipe {
            r,
            cs,
            labels: selected.iter().map(|(j, t, _)| (*j, *t)).collect(),
            attempts: json!({"attempts":attempts,"excluded":exclusions}),
        });
    }
    Err(format!(
        "known-section chart construction incomplete: {}",
        json!({"attempts":attempts,"excluded":exclusions})
    ))
}
fn pipeline(s: Spec, preferred: Option<Vec<(u64, u64)>>) -> Result<Value> {
    let recipe = make_recipe(&s, preferred)?;
    let e = engine(&s.p)?;
    let op = Operator::new(s.p.clone(), s.curve_a.clone().unwrap())?;
    let q = op.curve.times(&recipe.r, 8)?;
    let query = Query::new(&e, &s, recipe.r, q, recipe.cs)?;
    let table = IndexTable::new(&s)?;
    let result = query.extract(&table)?;
    let got: std::collections::BTreeSet<_> = result["row"]
        .as_array()
        .ok_or("no row")?
        .iter()
        .map(|v| (v["j"].as_u64().unwrap(), v["t"].as_u64().unwrap()))
        .collect();
    if got != recipe.labels.iter().copied().collect() {
        return Err("supplied-section labels changed".into());
    }
    let mut changed = query.coefficients.clone();
    changed[0] = e.f.add(&changed[0], &1u32.into());
    let mutation = match Query::new(&e, &s, query.r.clone(), query.target.clone(), changed)
        .and_then(|m| m.extract(&table))
    {
        Ok(v) => json!({"kind":"different original checked row","value":v}),
        Err(err) => json!({"kind":"refusal","error":err}),
    };
    Ok(
        json!({"k":s.k,"q":s.q,"order_factor":s.order_factor,"construction":recipe.attempts,"result":result,"coefficient_mutation":mutation}),
    )
}
fn alias() -> Result<Value> {
    let s = spec(false, 8)?;
    let op = Operator::new(s.p.clone(), 95u32.into())?;
    let mut points = vec![];
    for j in 1..=8 {
        for t in 0..3 {
            if let Ok(p) = raw_point(&s, j, t) {
                points.push((j, t, op.curve.times(&p, 8)?));
            }
        }
    }
    let pair = (0..points.len())
        .find_map(|i| {
            (i + 1..points.len())
                .find(|&j| points[i].2 == points[j].2)
                .map(|j| (i, j))
        })
        .ok_or("no toy alias pair")?;
    let mut labels = vec![
        (points[pair.0].0, points[pair.0].1),
        (points[pair.1].0, points[pair.1].1),
    ];
    for (j, t, _) in &points {
        if !labels.contains(&(*j, *t)) {
            labels.push((*j, *t));
        }
    }
    let result = pipeline(s, Some(labels))?;
    Ok(
        json!({"alias_pair":[[points[pair.0].0,points[pair.0].1],[points[pair.1].0,points[pair.1].1]],"same_image":point_json(&points[pair.0].2),"pipeline":result}),
    )
}
fn repeated() -> Result<Value> {
    let s = spec(false, 8)?;
    let op = Operator::new(s.p.clone(), 95u32.into())?;
    let e = engine(&s.p)?;
    let table = IndexTable::new(&s)?;
    let mut attempts = vec![];
    for j in 1..=8 {
        let Ok(point) = raw_point(&s, j, 0) else {
            continue;
        };
        let terms: Vec<_> = (0..6)
            .map(|i| IndexedPoint {
                column: i + 1,
                point: point.clone(),
            })
            .collect();
        let r = op.curve.times(&point, 6)?;
        let a = op.analyze(&terms, &r)?;
        let Some(cs) = a.section else {
            return Err("missing repeated section".into());
        };
        if r.is_some() && cs[4].is_zero() {
            attempts.push(j);
            continue;
        }
        let query = Query::new(&e, &s, r.clone(), op.curve.times(&r, 8)?, cs)?;
        let support = query.support()?;
        let error = query
            .extract(&table)
            .err()
            .ok_or("repeated label accepted")?;
        if !error.contains("repeated original raw label") {
            return Err(error);
        }
        if support["H_remainder"]
            .as_array()
            .unwrap()
            .iter()
            .all(|x| x[0] == "0" && x[1] == "0")
        {
            return Err("repeated control did not exhibit nonzero nilpotent H".into());
        }
        return Ok(
            json!({"j":j,"earlier_unsupported":attempts,"point":point_json(&point),"support":support,"original_refusal":error}),
        );
    }
    Err("repeated-section chart incomplete".into())
}
fn nilpotent() -> Result<Value> {
    let e = engine(&101u32.into())?;
    let mut r = Ring::new(&e, power_poly(&e, 1, 6)?)?;
    r.tau = r.constant(c(2));
    let epsilon = r.sub(&r.x()?, &r.constant(c(1)));
    let t = Alg {
        a: r.add(&r.constant(c(1)), &epsilon),
        b: r.constant(z()),
    };
    let fifth = r.pow(&epsilon, &5u32.into())?;
    let sixth = r.pow(&epsilon, &6u32.into())?;
    if fifth.iter().all(is_zero) || sixth.iter().any(|x| !is_zero(x)) {
        return Err("nilpotent stress failed".into());
    }
    let p = r.geometric(&t, &Fp2::one(), 0, 6)?;
    let direct = r.direct(&t, &Fp2::one(), 0, 6)?;
    if p != r.zero() || p != direct {
        return Err("rho1 zero divisor product mismatch".into());
    }
    Ok(
        json!({"epsilon":encoded(&epsilon),"fifth":encoded(&fifth),"sixth":encoded(&sixth),"product":alg_json(&p)}),
    )
}
fn refusals() -> Result<Value> {
    let e = engine(&101u32.into())?;
    let mut rows = vec![];
    let mut record = |name: &str, err: Option<String>| -> Result<()> {
        let error = err.ok_or_else(|| format!("missing refusal {name}"))?;
        rows.push(json!({"case":name,"error":error}));
        Ok(())
    };
    record("modulus", Engine::new(103u32.into(), WORK).err())?;
    record("degree0", Ring::new(&e, vec![c(1)]).err())?;
    record("degree7", Ring::new(&e, power_poly(&e, 1, 7)?).err())?;
    record("nonmonic", Ring::new(&e, vec![c(1), c(2)]).err())?;
    record("noncanonical", Ring::new(&e, vec![c(101), c(1)]).err())?;
    let r = Ring::new(&e, vec![c(1), c(1)])?;
    record("productcap", r.geometric(&r.v(), &c(3), 0, 65537).err())?;
    record(
        "intervaloverflow",
        r.geometric(&r.v(), &c(3), u64::MAX, 1).err(),
    )?;
    record("zerorho", r.geometric(&r.v(), &z(), 0, 2).err())?;
    let nonunit = Ring::new(&e, vec![c(95), c(2)])
        .err()
        .ok_or("nonmonic chart stress missing refusal")?;
    record("chart_nonmonic", Some(nonunit))?;
    let root = e.neg(&e.mul(&c(95), &e.inv(&c(2))?)?);
    let chart = Ring::new(&e, vec![e.neg(&root), c(1)])?;
    record(
        "nonunit_map",
        crate::query::lambda(&chart, &95u32.into()).err(),
    )?;
    let malformed = Alg {
        a: vec![],
        b: vec![],
    };
    record(
        "malformed_algebra",
        r.geometric(&malformed, &c(3), 0, 2).err(),
    )?;
    let zero = Engine::new(101u32.into(), 0)?;
    record("workcap", zero.mul(&c(1), &c(1)).err())?;
    let s = spec(false, 8)?;
    let recipe = make_recipe(&s, None)?;
    let op = Operator::new(s.p.clone(), 95u32.into())?;
    let q = op.curve.times(&recipe.r, 8)?;
    let mut bad = recipe.cs.clone();
    bad[0] = 101u32.into();
    record(
        "noncanonical_section",
        Query::new(&e, &s, recipe.r.clone(), q.clone(), bad).err(),
    )?;
    let mut badspec = s.clone();
    badspec.q = 1;
    record(
        "original_q",
        Query::new(&e, &badspec, recipe.r.clone(), q.clone(), recipe.cs.clone()).err(),
    )?;
    badspec = s.clone();
    badspec.k = 0;
    record(
        "original_k",
        Query::new(&e, &badspec, recipe.r.clone(), q, recipe.cs.clone()).err(),
    )?;
    record(
        "target_mismatch",
        Query::new(&e, &s, recipe.r, None, recipe.cs).err(),
    )?;
    Ok(json!({"refusals":rows}))
}
fn emit_case<F>(
    name: &str,
    run: F,
    emit: &mut dyn FnMut(Value) -> Result<()>,
    passed: &mut bool,
) -> Result<()>
where
    F: FnOnce() -> Result<Value>,
{
    let result = run();
    if result.is_err() {
        *passed = false;
    }
    emit(match result {
        Ok(data) => json!({"case":name,"passed":true,"data":data}),
        Err(err) => json!({"case":name,"passed":false,"error":err}),
    })
}
pub fn run(emit: &mut dyn FnMut(Value) -> Result<()>) -> Result<bool> {
    let mut passed = true;
    for p in admitted_moduli() {
        for n in [0, 1, 3, 8, 9, 17, 33] {
            emit_case(
                &format!("scalar-{p}-{n}"),
                || {
                    let e = engine(&p)?;
                    let a = coeffs(n, 3);
                    let b = coeffs(n, 5);
                    let fast = e.product(&a, &b)?;
                    let direct = e.school(&a, &b)?;
                    if fast != direct {
                        return Err("scalar convolution mismatch".into());
                    }
                    Ok(
                        json!({"length":n,"p":p.to_string(),"fast":encoded(&fast),"direct":encoded(&direct),"counts":e.stats()}),
                    )
                },
                emit,
                &mut passed,
            )?;
        }
    }
    for p in admitted_moduli() {
        for d in [1, 2, 6] {
            for n in [0, 1, 2, 3, 8, 9, 17, 31] {
                for start in [0, 3] {
                    emit_case(
                        &format!("product-{p}-{d}-{n}-{start}"),
                        || compare_product(&p, d, n, start),
                        emit,
                        &mut passed,
                    )?;
                }
            }
        }
    }
    emit_case("nilpotent-rho1", nilpotent, emit, &mut passed)?;
    emit_case("refusals", refusals, emit, &mut passed)?;
    emit_case(
        "toy-original-witness",
        || pipeline(spec(false, 8)?, None),
        emit,
        &mut passed,
    )?;
    emit_case("toy-geometric-alias", alias, emit, &mut passed)?;
    emit_case("toy-repeated-section", repeated, emit, &mut passed)?;
    emit_case(
        "actual-original-q-k31",
        || pipeline(spec(true, 31)?, None),
        emit,
        &mut passed,
    )?;
    emit_case(
        "actual-full-length-degree1",
        || compare_product(&torus_s4::curve25519_prime(), 1, 20633, 3),
        emit,
        &mut passed,
    )?;
    emit_case(
        "actual-original-q-k20633",
        || pipeline(spec(true, 20633)?, None),
        emit,
        &mut passed,
    )?;
    Ok(passed)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_backend() -> Result<()> {
        let e = engine(&101u32.into())?;
        let a = coeffs(17, 3);
        let b = coeffs(33, 5);
        assert_eq!(e.product(&a, &b)?, e.school(&a, &b)?);
        Ok(())
    }
    #[test]
    fn repeated_algebra() -> Result<()> {
        nilpotent()?;
        Ok(())
    }
    #[test]
    fn geometric_small() -> Result<()> {
        compare_product(&101u32.into(), 6, 31, 3)?;
        Ok(())
    }
    #[test]
    fn original_lift() -> Result<()> {
        pipeline(spec(false, 8)?, None)?;
        Ok(())
    }
}
