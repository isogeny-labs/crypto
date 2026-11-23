//! Fixed development controls. Point tuples are given candidates, not roots
//! discovered in a torus base. Existing curve addition is the comparison.
use crate::{IndexedPoint, Operator, Result, ARITY};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use torus_s4::{curve25519_prime, Curve, Point};

fn point(p: &Point) -> Value {
    p.as_ref()
        .map_or(Value::Null, |(x, y)| json!([x.to_string(), y.to_string()]))
}
fn negative(c: &Curve, p: &Point) -> Point {
    p.as_ref()
        .map(|(x, y)| (x.clone(), c.f.sub(&BigUint::zero(), y)))
}
fn total(c: &Curve, ts: &[IndexedPoint]) -> Result<Point> {
    ts.iter().try_fold(None, |a, t| c.add(&a, &t.point))
}
fn terms(c: &Curve, g: &Point, multipliers: &[i32]) -> Result<Vec<IndexedPoint>> {
    multipliers
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            let p = c.times(g, u64::from(n.unsigned_abs()))?;
            Ok(IndexedPoint {
                column: (i + 1) as u64,
                point: if n < 0 { negative(c, &p) } else { p },
            })
        })
        .collect()
}

fn check_case(op: &Operator, ts: &[IndexedPoint], target: &Point, id: String) -> Value {
    let expected = total(&op.curve, ts).map(|sum| sum == *target);
    let result = op.analyze(ts, target);
    let record = match (expected, result) {
        (Ok(expected), Ok(a)) => {
            let observed = a.section.is_some();
            let cert_ok = a
                .section
                .as_ref()
                .is_none_or(|s| op.verify_section(ts, target, s).is_ok());
            let mut mutation_refusals = 0;
            if let Some(section) = &a.section {
                for i in 0..ARITY {
                    let mut changed = section.clone();
                    changed[i] = op.curve.f.add(&changed[i], &BigUint::one());
                    mutation_refusals +=
                        usize::from(op.verify_section(ts, target, &changed).is_err());
                }
            }
            let matched = observed == expected
                && cert_ok
                && a.rank == if expected { 5 } else { 6 }
                && mutation_refusals == if expected { 6 } else { 0 };
            json!({"id":id,"expected_group_sum":expected,"section_exists":observed,"rank":a.rank,
                "matched":matched,"certificate_ok":cert_ok,"coefficient_mutations_refused":mutation_refusals,
                "target":point(target),"candidate_points":ts.iter().map(|t| json!({"column":t.column,"point":point(&t.point)})).collect::<Vec<_>>(),
                "multiplicities":a.evaluation.multiplicities,"charts":a.evaluation.charts,
                "matrix":a.evaluation.rows.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),
                "section":a.section.map(|s|s.iter().map(ToString::to_string).collect::<Vec<_>>())})
        }
        (a, b) => {
            json!({"id":id,"matched":false,"group_error":a.err(),"operator_error":b.err(),"target":point(target)})
        }
    };
    record
}

fn targets(op: &Operator, g: &Point, ts: &[IndexedPoint], toy: bool) -> Result<Vec<Point>> {
    if toy {
        return (0..13).map(|n| op.curve.times(g, n)).collect();
    }
    let sum = total(&op.curve, ts)?;
    Ok(vec![
        None,
        g.clone(),
        negative(&op.curve, g),
        op.curve.times(g, 2)?,
        op.curve.times(g, 3)?,
        sum.clone(),
        op.curve.add(&sum, g)?,
    ])
}

fn naive_examples(op: &Operator, g: &Point, label: &str) -> Result<Vec<Value>> {
    let repeated = terms(&op.curve, g, &[1; 6])?;
    let wrong = op.curve.times(g, 7)?;
    let distinct = terms(&op.curve, g, &[1, 2, 3, 4, 5, 6])?;
    let negative_target = negative(&op.curve, g);
    let mut out = vec![];
    for (kind, ts, target, cleared) in [
        ("repeated-value-alias", repeated, wrong, false),
        (
            "unguarded-denominator-clearing",
            distinct,
            negative_target,
            true,
        ),
    ] {
        let naive_rank = op.naive_rank(&ts, &target, cleared)?;
        let corrected = op.analyze(&ts, &target)?;
        let group_sum = total(&op.curve, &ts)?;
        out.push(json!({"field":label,"kind":kind,"naive_rank":naive_rank,"corrected_rank":corrected.rank,
            "target":point(&target),"group_sum":point(&group_sum),
            "candidate_points":ts.iter().map(|t|json!({"column":t.column,"point":point(&t.point)})).collect::<Vec<_>>(),
            "reproduced":naive_rank<6 && corrected.rank==6 && group_sum!=target}));
    }
    Ok(out)
}

fn refusals(op: &Operator, g: &Point, label: &str) -> Result<Vec<Value>> {
    let ts = terms(&op.curve, g, &[1, 2, 3, 4, 5, 6])?;
    let t = total(&op.curve, &ts)?;
    let section = op
        .analyze(&ts, &t)?
        .section
        .ok_or("control section missing")?;
    let mut out = vec![];
    let mut record =
        |name: &str, refused: bool| out.push(json!({"field":label,"kind":name,"refused":refused}));
    record("wrong-arity", op.analyze(&ts[..5], &t).is_err());
    let mut v = ts.clone();
    v[1].column = v[0].column;
    record("repeated-raw-column", op.analyze(&v, &t).is_err());
    let mut v = ts.clone();
    v[0].point = None;
    record("identity-member-unsupported", op.analyze(&v, &t).is_err());
    let mut v = ts.clone();
    v[0].point = Some((BigUint::zero(), BigUint::zero()));
    record(
        "two-torsion-member-unsupported",
        op.analyze(&v, &t).is_err(),
    );
    let mut v = ts.clone();
    v[0].point = Some((BigUint::zero(), BigUint::one()));
    record("off-curve-member", op.analyze(&v, &t).is_err());
    let mut v = ts.clone();
    let (x, y) = v[0].point.as_ref().ok_or("finite member")?;
    v[0].point = Some((x + &op.curve.f.p, y.clone()));
    record("noncanonical-member", op.analyze(&v, &t).is_err());
    record(
        "two-torsion-target-unsupported",
        op.analyze(&ts, &Some((BigUint::zero(), BigUint::zero())))
            .is_err(),
    );
    record(
        "off-curve-target",
        op.analyze(&ts, &Some((BigUint::zero(), BigUint::one())))
            .is_err(),
    );
    let (x, y) = g.as_ref().ok_or("finite G")?;
    record(
        "noncanonical-target",
        op.analyze(&ts, &Some((x + &op.curve.f.p, y.clone())))
            .is_err(),
    );
    record(
        "zero-section",
        op.verify_section(&ts, &t, &vec![BigUint::zero(); 6])
            .is_err(),
    );
    record(
        "truncated-section",
        op.verify_section(&ts, &t, &section[..5]).is_err(),
    );
    let mut changed = section.clone();
    changed[0] += &op.curve.f.p;
    record(
        "noncanonical-section",
        op.verify_section(&ts, &t, &changed).is_err(),
    );
    let changed: Vec<_> = section.iter().map(|c| op.curve.f.scale(c, 2)).collect();
    record(
        "unnormalized-section",
        op.verify_section(&ts, &t, &changed).is_err(),
    );
    record(
        "wrong-target-section",
        op.verify_section(&ts, &op.curve.add(&t, g)?, &section)
            .is_err(),
    );
    record(
        "singular-curve",
        Operator::new(op.curve.f.p.clone(), 2u32.into()).is_err(),
    );
    record(
        "noncanonical-A",
        Operator::new(op.curve.f.p.clone(), &op.curve.a + &op.curve.f.p).is_err(),
    );
    Ok(out)
}

pub fn run() -> Result<Value> {
    let mut cases = vec![];
    let mut counterexamples = vec![];
    let mut invalid = vec![];
    for toy in [true, false] {
        let (p, a, label) = if toy {
            (BigUint::from(101u32), BigUint::from(95u32), "Fp101")
        } else {
            (
                curve25519_prime(),
                BigUint::from(486662u32),
                "Curve25519-field",
            )
        };
        let op = Operator::new(p, a)?;
        let raw = if toy {
            Some((2u32.into(), 17u32.into()))
        } else {
            let x = BigUint::from(9u32);
            let y = op
                .curve
                .f
                .sqrt(&op.curve.rhs(&x))?
                .ok_or("RFC9 has no square root")?;
            Some((x, y))
        };
        if !op.curve.contains(&raw) {
            return Err("bad fixed source point".into());
        }
        let g = op.curve.times(&raw, 8)?;
        if g.is_none() {
            return Err("identity fixed G".into());
        }
        if toy {
            if op.curve.times(&g, 13)?.is_some() {
                return Err("toy13G not identity".into());
            }
            for n in 1..13 {
                if op.curve.times(&g, n)?.is_none() {
                    return Err("small toy order".into());
                }
            }
        }
        for c in 0..=6 {
            for b in 0..=6 - c {
                let a = 6 - b - c;
                let mut multipliers = vec![1; a];
                multipliers.extend(vec![2; b]);
                multipliers.extend(vec![3; c]);
                let ts = terms(&op.curve, &g, &multipliers)?;
                for (j, t) in targets(&op, &g, &ts, toy)?.iter().enumerate() {
                    cases.push(check_case(
                        &op,
                        &ts,
                        t,
                        format!("{label}-multiplicity-{a}-{b}-{c}-target-{j}"),
                    ));
                }
            }
        }
        for (k, ms) in [[1, 2, 3, 4, 5, 6], [1, 2, 3, 4, 5, -1]].iter().enumerate() {
            let ts = terms(&op.curve, &g, ms)?;
            for (j, t) in targets(&op, &g, &ts, toy)?.iter().enumerate() {
                cases.push(check_case(
                    &op,
                    &ts,
                    t,
                    format!("{label}-distinct-{k}-target-{j}"),
                ));
            }
        }
        counterexamples.extend(naive_examples(&op, &g, label)?);
        invalid.extend(refusals(&op, &g, label)?);
    }
    invalid.push(json!({"field":"admission","kind":"unsupported-composite-modulus","refused":Operator::new(15u32.into(),1u32.into()).is_err()}));
    let matched = cases.iter().filter(|v| v["matched"] == true).count();
    let positives = cases.iter().filter(|v| v["section_exists"] == true).count();
    let certificates_mutated: u64 = cases
        .iter()
        .filter_map(|v| v["coefficient_mutations_refused"].as_u64())
        .sum();
    let naive = counterexamples
        .iter()
        .filter(|v| v["reproduced"] == true)
        .count();
    let refused = invalid.iter().filter(|v| v["refused"] == true).count();
    Ok(
        json!({"schema":"torus-section-operator.controls/v1","scope":"fixed given-candidate predicate controls; not base discovery or scaling",
        "planned_comparisons":600,"attempted_comparisons":cases.len(),"matched":matched,"positive_sections":positives,
        "coefficient_mutations_refused":certificates_mutated,"naive_counterexamples_reproduced":naive,
        "refusals":refused,"passed":cases.len()==600 && matched==600 && positives==60 && certificates_mutated==360 && naive==4 && refused==invalid.len(),
        "cases":cases,"naive_counterexamples":counterexamples,"invalid_controls":invalid}),
    )
}
