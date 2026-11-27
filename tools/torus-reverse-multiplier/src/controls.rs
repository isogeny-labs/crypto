//! Fixed native correctness controls; known multipliers never enter solve().
use crate::*;
use std::io::Write;
use torus_quotient_query::controls::spec;
use torus_s4_tree::identity::{self, Expr as E, IdentityStatement, Verdict};

fn emit(out: &mut impl Write, v: &Value) -> Result<()> {
    writeln!(out, "{v}").map_err(|e| e.to_string())
}
fn write_query(out: &mut impl Write, name: &str, s: &Spec, r: &Point) -> Result<Answer> {
    let answer = solve(s, r, WORK)?;
    emit(
        out,
        &json!({"kind":"query","name":name,"value":answer.record}),
    )?;
    Ok(answer)
}
fn reference_key(sigma: &Fp2, p1: &Point, p2: &Point) -> String {
    json!({"sigma":fp2_json(sigma),"raw":[point_json(p1),point_json(p2)]}).to_string()
}
fn known_target(s: &Spec) -> Result<Point> {
    let cv = curve(s)?;
    let a = [s.h.clone(), s.h.mul(&s.h, &cv.f)];
    for (_, p) in raw_choices(&cv, &a[0])? {
        for (_, q) in raw_choices(&cv, &a[1])? {
            let r = cv.add(&p, &q)?;
            if r.as_ref().is_some_and(|(_, y)| !y.is_zero()) && cv.times(&r, 8)?.is_some() {
                return Ok(r);
            }
        }
    }
    Err("no admitted known-consistent target; control construction failed".into())
}
fn identity_control(out: &mut impl Write) -> Result<()> {
    let n1 = E::var("N1");
    let d1 = E::var("D1");
    let n2 = E::var("N2");
    let d2 = E::var("D2");
    let a = E::var("A");
    let r = E::var("r");
    let sub = |x: E, y: E| E::plus(x, E::minus(y));
    let sq = |x: E| E::times(x.clone(), x);
    let x = E::times(n1.clone(), d2.clone());
    let y = E::times(n2.clone(), d1.clone());
    let z = E::times(d1, d2);
    let nn = E::times(n1, n2);
    let middle = E::plus(
        E::times(
            E::plus(x.clone(), y.clone()),
            E::plus(nn.clone(), z.clone()),
        ),
        E::product(vec![E::Const(2), a.clone(), nn.clone(), z.clone()]),
    );
    let p = E::plus(
        sub(
            E::times(sq(sub(x.clone(), y.clone())), sq(r.clone())),
            E::product(vec![E::Const(2), middle, r.clone()]),
        ),
        sq(sub(nn, z.clone())),
    );
    let xy = E::times(x.clone(), y.clone());
    let rz = E::times(r, z.clone());
    let homogeneous = E::plus(
        sub(
            E::times(sq(sub(x.clone(), y.clone())), sq(rz.clone())),
            E::product(vec![
                E::Const(2),
                E::plus(
                    E::times(E::plus(x, y), E::plus(xy.clone(), sq(z.clone()))),
                    E::product(vec![E::Const(2), a, xy.clone(), z.clone()]),
                ),
                rz,
            ]),
        ),
        sq(sub(xy, sq(z.clone()))),
    );
    let statement = IdentityStatement {
        name: "Montgomery pair numerator homogeneous denominator clearing".into(),
        source: "reverse-multiplier-v1 DERIVATION; classical Montgomery S3".into(),
        variables: ["N1", "D1", "N2", "D2", "A", "r"]
            .map(str::to_string)
            .to_vec(),
        degree_bound: 10,
        lhs: E::times(sq(z), p),
        rhs: homogeneous,
    };
    let cert = identity::issue(
        statement.clone(),
        2026112402,
        32,
        "same producer; maintained native checker, not independently signed",
        None,
    )
    .map_err(|e| e.to_string())?;
    let verdict =
        identity::check_with_extra(&cert, Some((2026112403, 32))).map_err(|e| e.to_string())?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err("clearing identity refused".into());
    }
    let mut wrong = statement;
    wrong.rhs = E::plus(wrong.rhs, E::Const(1));
    if identity::issue(wrong, 2026112402, 32, "plus-one control", None).is_ok() {
        return Err("false clearing identity issued".into());
    }
    emit(
        out,
        &json!({"kind":"identity","certificate":cert,"check":format!("{verdict:?}"),"plus_one_refused":true,"scope":"evaluated format field2^61-1 only; no formal proof pointer"}),
    )
}
fn toy_control(out: &mut impl Write) -> Result<()> {
    let s = spec(false, 8)?;
    let cv = curve(&s)?;
    let mut reference: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut all_sigma: Vec<_> = (0u64..101)
        .map(|n| cayley(&cv.f, &BigUint::from(n)))
        .collect::<Result<_>>()?;
    all_sigma.push(Fp2 {
        a: 100u32.into(),
        b: 0u32.into(),
    });
    let alpha = [s.h.clone(), s.h.mul(&s.h, &cv.f)];
    let mut enumerated_pairs = 0;
    for sigma in &all_sigma {
        let p = raw_choices(&cv, &sigma.mul(&alpha[0], &cv.f))?;
        let q = raw_choices(&cv, &sigma.mul(&alpha[1], &cv.f))?;
        for (_, p1) in &p {
            for (_, p2) in &q {
                let r = cv.add(p1, p2)?;
                if r.as_ref().is_none_or(|(_, y)| y.is_zero()) || cv.times(&r, 8)?.is_none() {
                    continue;
                }
                reference
                    .entry(point_json(&r).to_string())
                    .or_default()
                    .insert(reference_key(sigma, p1, p2));
                enumerated_pairs += 1;
            }
        }
    }
    let mut targets = vec![];
    let mut excluded = 0;
    for x in 0u64..101 {
        let x = BigUint::from(x);
        if let Some(y) = cv.f.sqrt(&cv.rhs(&x))? {
            let ys = if y.is_zero() {
                vec![y]
            } else {
                vec![y.clone(), cv.f.sub(&0u32.into(), &y)]
            };
            for y in ys {
                let r = Some((x.clone(), y));
                if r.as_ref().unwrap().1.is_zero() || cv.times(&r, 8)?.is_none() {
                    excluded += 1;
                    continue;
                }
                targets.push(r);
            }
        }
    }
    emit(
        out,
        &json!({"kind":"toy_declared","multipliers":all_sigma.len(),"admitted_finite_targets":targets.len(),"excluded_finite_torsion_targets":excluded,"direct_signed_pairs":enumerated_pairs,"reference_method":"direct full torus/group enumeration, no S3/root factorization; shares field/group primitives"}),
    )?;
    let mut lifts_count = 0;
    let mut zeros = 0;
    let mut infinity = 0;
    let mut rows = BTreeSet::new();
    for (i, r) in targets.iter().enumerate() {
        let answer = write_query(out, &format!("toy-public-{i}"), &s, r)?;
        let actual: BTreeSet<_> = answer.lifts.iter().map(Lift::key).collect();
        let expected = reference
            .get(&point_json(r).to_string())
            .cloned()
            .unwrap_or_default();
        if actual != expected {
            emit(
                out,
                &json!({"kind":"toy_mismatch","R":point_json(r),"expected":expected,"actual":actual}),
            )?;
            return Err("toy complete pair set mismatch".into());
        }
        if actual.is_empty() {
            zeros += 1;
        }
        lifts_count += actual.len();
        for value in &answer.lifts {
            if value.sigma.a == 100u32.into() && value.sigma.b.is_zero() {
                infinity += 1;
            }
            rows.insert(value.row_key(&cv.f)?);
        }
    }
    emit(
        out,
        &json!({"kind":"toy_summary","targets":targets.len(),"zero_result_targets":zeros,"ordered_multiplier_lifts":lifts_count,"direct_pairs":enumerated_pairs,"infinity_lifts":infinity,"distinct_geometric_signed_rows":rows.len(),"all_pair_sets_equal":true,"scope":"full declared toy correctness, no scalar-DLP recovery"}),
    )
}
fn actual_control(out: &mut impl Write) -> Result<()> {
    let s = spec(true, 20633)?;
    let cv = curve(&s)?;
    let y =
        cv.f.sqrt(&cv.rhs(&9u32.into()))?
            .ok_or("public u9 is not rational")?;
    let r = Some((9u32.into(), y.clone()));
    let nr = Some((9u32.into(), cv.f.sub(&0u32.into(), &y)));
    write_query(out, "actual-public-u9", &s, &r)?;
    write_query(out, "actual-public-minus-u9", &s, &nr)?;
    let known = known_target(&s)?;
    let answer = write_query(out, "actual-known-consistent-public-R", &s, &known)?;
    let one = Fp2::one();
    let negative = one.neg(&cv.f);
    let reciprocal = s.h.pow(&3u32.into(), &cv.f).inverse(&cv.f)?;
    let keys = |sigma: &Fp2| {
        answer
            .lifts
            .iter()
            .filter(|x| x.sigma == *sigma)
            .map(|x| x.row_key(&cv.f))
            .collect::<Result<BTreeSet<_>>>()
    };
    let base = keys(&one)?;
    if base.is_empty() || base != keys(&negative)? || base != keys(&reciprocal)? {
        return Err("known multiplier sign/reciprocal row symmetry".into());
    }
    emit(
        out,
        &json!({"kind":"actual_summary","public_queries":3,"known_sigma_one_is_validator_only":true,"one_negation_reciprocal_same_rows":true,"known_geometric_rows":base,"scope":"original-field adapted pair roots only; not original full-base solve, shared-base rank or DLP recovery"}),
    )
}
fn guards(out: &mut impl Write) -> Result<()> {
    let s = spec(false, 8)?;
    let cv = curve(&s)?;
    let r = known_target(&s)?;
    let mut checks = 0;
    if solve(&s, &None, WORK).is_ok() {
        return Err("identity target admitted".into());
    }
    checks += 1;
    if solve(&s, &Some((0u32.into(), 1u32.into())), WORK).is_ok() {
        return Err("offcurve target admitted".into());
    }
    checks += 1;
    if solve(&s, &Some((101u32.into(), 0u32.into())), WORK).is_ok() {
        return Err("noncanonical target admitted".into());
    }
    checks += 1;
    for kind in 0..3 {
        let mut bad = s.clone();
        match kind {
            0 => bad.q = 0,
            1 => bad.k = 1,
            _ => {
                bad.h = Fp2 {
                    a: 0u32.into(),
                    b: 0u32.into(),
                }
            }
        };
        if solve(&bad, &r, WORK).is_ok() {
            return Err("malformed spec admitted".into());
        }
        checks += 1;
    }
    if solve(&s, &r, 0).is_ok() {
        return Err("work0 cap ignored".into());
    }
    checks += 1;
    let e = Engine::new(s.p.clone(), WORK)?;
    for p in [
        vec![],
        vec![c(101)],
        vec![Fp2 {
            a: 0u32.into(),
            b: 1u32.into(),
        }],
        vec![c(1); 10],
    ] {
        if poly::roots(&e, &p).is_ok() {
            return Err("root polynomial admission ignored".into());
        }
        checks += 1;
    }
    let answer = solve(&s, &r, WORK)?;
    let v = answer.lifts.first().ok_or("known toy lift absent")?;
    for kind in 0..3 {
        let mut bad = v.clone();
        match kind {
            0 => bad.raw[0] = None,
            1 => bad.branches[0] = 2,
            _ => {
                bad.base_key = Fp2 {
                    a: 0u32.into(),
                    b: 0u32.into(),
                }
            }
        };
        if verify_lift(&s, &cv, &r, &bad).is_ok() {
            return Err("changed lift accepted".into());
        }
        checks += 1;
    }
    emit(
        out,
        &json!({"kind":"guard_summary","refusals":checks,"caps_are_incomplete":true,"changed_lifts_refused":3}),
    )
}
pub fn run(out: &mut impl Write) -> Result<()> {
    let mut errors = 0;
    for name in ["identity", "guards", "toy", "actual"] {
        emit(out, &json!({"kind":"group_start","name":name}))?;
        let result = match name {
            "identity" => identity_control(out),
            "guards" => guards(out),
            "toy" => toy_control(out),
            _ => actual_control(out),
        };
        if let Err(error) = result {
            emit(
                out,
                &json!({"kind":"group_error","name":name,"error":error,"scope":"this group stopped; its remaining arms unattempted; other planned groups continue"}),
            )?;
            errors += 1;
        }
    }
    emit(
        out,
        &json!({"kind":"complete","groups_attempted":4,"groups_failed":errors,"decoder_calls":0,"scalar_recovery":false,"shared_base_gate":"unimplemented","scope":"fixed correctness only; adapted bases; no timing/exponent result"}),
    )?;
    if errors == 0 {
        Ok(())
    } else {
        Err(format!("{errors} fixed groups failed; raw retained"))
    }
}
