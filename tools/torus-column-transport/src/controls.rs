//! Fixed source-qualified controls. No fixture scalar enters public query().
use crate::*;
use std::io::Write;
use torus_quotient_query::controls::spec;
use torus_reverse_multiplier::{cayley, WORK};
use torus_s4::{Field, Point};
use torus_s4_tree::identity::{self, Expr as E, IdentityStatement, Verdict};
fn emit(out: &mut dyn Write, v: Value) -> Result<()> {
    writeln!(out, "{v}").map_err(|e| e.to_string())
}
fn audited(out: &mut dyn Write, name: &str, f: &Family, q: &Point) -> Result<Query> {
    let answer = query(f, q, WORK)?;
    let mut rows: BTreeMap<String, Vec<&Lift>> = BTreeMap::new();
    for v in &answer.lifts {
        rows.entry(v.row_key(&f.cv.f)?).or_default().push(v);
    }
    let mut audits = vec![];
    for values in rows.values() {
        let (keys, record) = transport(f, q, values[0])?;
        let expected = values.iter().map(|v| v.key()).collect::<BTreeSet<_>>();
        if keys != expected {
            return Err("same-row transport versus all-sector root mismatch".into());
        }
        audits.push(record);
    }
    emit(
        out,
        json!({"kind":"query","name":name,"value":answer.record,"same_row_transport":audits,"all_transport_sets_equal":true}),
    )?;
    Ok(answer)
}
#[derive(Clone)]
struct TaggedRow {
    name: String,
    entries: BTreeMap<Key, i64>,
    a: BigUint,
    b: BigUint,
    provenance: Vec<String>,
}
fn signed(f: &Field, n: i64) -> BigUint {
    let v = BigUint::from(n.unsigned_abs()) % &f.p;
    if n < 0 {
        f.sub(&BigUint::zero(), &v)
    } else {
        v
    }
}
fn row_sum(cv: &Curve, row: &BTreeMap<Key, i64>) -> Result<Point> {
    let mut out = None;
    for (k, n) in row {
        let p = cv.times(&Some(k.clone()), n.unsigned_abs())?;
        out = cv.add(&out, &if *n < 0 { neg(cv, &p) } else { p })?;
    }
    Ok(out)
}
fn check_tags(f: &Family, q: &Point, rows: &[TaggedRow]) -> Result<()> {
    for row in rows {
        let target =
            f.cv.add(&scalar(&f.cv, &f.g, &row.a)?, &scalar(&f.cv, q, &row.b)?)?;
        if row_sum(&f.cv, &row.entries)? != target {
            return Err("public target-tag group mismatch".into());
        }
    }
    Ok(())
}
struct TaggedQuery<'a> {
    name: &'a str,
    a: BigUint,
    b: BigUint,
    answer: &'a Query,
}
fn analyze_panel(f: &Family, q: &Point, queries: &[TaggedQuery<'_>]) -> Result<Value> {
    f.check_image(q)?;
    let mut rows = vec![];
    for input in queries {
        let mut groups: BTreeMap<String, Vec<&Lift>> = BTreeMap::new();
        for v in &input.answer.lifts {
            groups.entry(v.row_key(&f.cv.f)?).or_default().push(v);
        }
        for values in groups.values() {
            let mut entries = BTreeMap::new();
            for p in &values[0].images {
                let (k, c) = column(&f.cv, p)?;
                *entries.entry(k).or_default() += c;
            }
            entries.retain(|_, n| *n != 0);
            rows.push(TaggedRow {
                name: input.name.into(),
                entries,
                a: input.a.clone(),
                b: input.b.clone(),
                provenance: values.iter().map(|v| v.key()).collect(),
            });
        }
    }
    check_tags(f, q, &rows)?;
    let field = Field::new(f.ell.clone())?;
    // This mutation must fail BEFORE reduction, not just produce a different rank.
    if let Some(first) = rows.first() {
        let mut changed = first.clone();
        changed.a = field.add(&changed.a, &BigUint::one());
        if check_tags(f, q, &[changed]).is_ok() {
            return Err("wrong tag accepted".into());
        }
    }
    let (g, sg) = column(&f.cv, &f.g)?;
    let (qc, sq) = column(&f.cv, q)?;
    if g == qc {
        return Err("panel Q=+/-G is a trivial public-log control".into());
    }
    let before = rows
        .iter()
        .flat_map(|r| r.entries.keys().cloned())
        .collect::<BTreeSet<_>>();
    let mut folded = vec![];
    for r in &rows {
        let mut entries = r.entries.clone();
        let mut a = r.a.clone();
        let mut b = r.b.clone();
        if let Some(c) = entries.remove(&g) {
            a = field.sub(&a, &signed(&field, c * sg));
        }
        if let Some(c) = entries.remove(&qc) {
            b = field.sub(&b, &signed(&field, c * sq));
        }
        folded.push(TaggedRow {
            name: r.name.clone(),
            entries,
            a,
            b,
            provenance: r.provenance.clone(),
        });
    }
    check_tags(f, q, &folded)?;
    let columns = folded
        .iter()
        .flat_map(|r| r.entries.keys().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let matrix = folded
        .iter()
        .map(|r| {
            columns
                .iter()
                .map(|k| signed(&field, *r.entries.get(k).unwrap_or(&0)))
                .collect()
        })
        .collect();
    let a = folded.iter().map(|r| r.a.clone()).collect::<Vec<_>>();
    let b = folded.iter().map(|r| r.b.clone()).collect::<Vec<_>>();
    let analysis = linear::analyze(&field, &matrix, &a, &b)?;
    for x in &analysis.recovered {
        if scalar(&f.cv, &f.g, x)? != *q {
            return Err("kernel scalar group replay".into());
        }
    }
    let private_columns = columns
        .iter()
        .filter(|c| folded.iter().filter(|r| r.entries.contains_key(*c)).count() == 1)
        .count();
    Ok(
        json!({"G":point_json(&f.g),"Q":point_json(q),"rows":folded.len(),"columns_before_known_fold":before.len(),"unknown_columns":columns.iter().map(|(x,y)|json!([x.to_string(),y.to_string()])).collect::<Vec<_>>(),"private_unknown_columns":private_columns,"known_G_Q_columns_folded":before.len()-columns.len(),"tagged_rows":folded.iter().map(|r|json!({"query":r.name,"entries":r.entries.iter().map(|((x,y),n)|json!([x.to_string(),y.to_string(),n])).collect::<Vec<_>>(),"a":r.a.to_string(),"b":r.b.to_string(),"original_multiplier_raw_keys":r.provenance})).collect::<Vec<_>>(),"rank_M":analysis.rank,"rank_M_b":analysis.augmented_rank,"linear_certificate":analysis.record,"recovered_scalars":analysis.recovered.iter().map(ToString::to_string).collect::<Vec<_>>(),"all_returned_scalars_group_replayed":true,"wrong_tag_refused_before_rank":!rows.is_empty(),"scope":"fixed tagged target panel only; duplicates may enlarge an uninformative kernel, not independence or a growing-family result"}),
    )
}
fn panel(f: &Family, q: &Point) -> Result<Vec<(&'static str, BigUint, BigUint, Point)>> {
    Ok(vec![
        ("Q", 0u32.into(), 1u32.into(), q.clone()),
        ("Q+G", 1u32.into(), 1u32.into(), f.cv.add(q, &f.g)?),
        ("-Q", 0u32.into(), &f.ell - 1u32, neg(&f.cv, q)),
        ("G", 1u32.into(), 0u32.into(), f.g.clone()),
    ])
}
fn toy(out: &mut dyn Write) -> Result<()> {
    let s = spec(false, 8)?;
    let f = Family::new(&s)?;
    let mut all_points = vec![None];
    for x in 0u64..101 {
        for y in 0u64..101 {
            let p = Some((x.into(), y.into()));
            if f.cv.contains(&p) {
                all_points.push(p);
            }
        }
    }
    if all_points.len() != 104 {
        return Err("original toy group104 mismatch".into());
    }
    let images = all_points
        .iter()
        .map(|p| f.cv.times(p, 8))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|p| p.is_some())
        .map(|p| (point_key(&p), p))
        .collect::<BTreeMap<_, _>>();
    if images.len() != 12 {
        return Err("toy nonidentity images12 mismatch".into());
    }
    let mut sigmas = (0u64..101)
        .map(|n| cayley(&f.cv.f, &n.into()))
        .collect::<Result<Vec<_>>>()?;
    sigmas.push(Fp2 {
        a: 100u32.into(),
        b: 0u32.into(),
    });
    let mut reference: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for sigma in sigmas {
        let z1 = sigma.mul(&s.h, &f.cv.f);
        let z2 = z1.mul(&s.h, &f.cv.f);
        for (_, p1) in raw_choices(&f.cv, &z1)? {
            for (_, p2) in raw_choices(&f.cv, &z2)? {
                let q = f.cv.times(&f.cv.add(&p1, &p2)?, 8)?;
                if q.is_none() {
                    continue;
                }
                reference.entry(point_key(&q)).or_default().insert(
                    json!({"sigma":fp2_json(&sigma),"raw":[point_json(&p1),point_json(&p2)]})
                        .to_string(),
                );
            }
        }
    }
    let mut answers = BTreeMap::new();
    let mut total = 0;
    for (i, (key, q)) in images.iter().enumerate() {
        let direct = all_points
            .iter()
            .filter(|p| f.cv.times(p, 8).as_ref() == Ok(q))
            .map(point_key)
            .collect::<BTreeSet<_>>();
        let lifted = f.sectors(q)?.iter().map(point_key).collect::<BTreeSet<_>>();
        if direct != lifted || direct.len() != 8 {
            return Err("toy full fibre mismatch".into());
        }
        let answer = audited(out, &format!("toy-image-{i}"), &f, q)?;
        let keys = answer.lifts.iter().map(Lift::key).collect::<BTreeSet<_>>();
        if keys != reference.get(key).cloned().unwrap_or_default() {
            return Err("toy complete pair reference mismatch".into());
        }
        total += keys.len();
        answers.insert(key.clone(), answer);
    }
    let q = scalar(&f.cv, &f.g, &5u32.into())?; // Validator-only fixture scalar, not query input.
    let targets = panel(&f, &q)?;
    let tagged = targets
        .iter()
        .map(|(name, a, b, t)| TaggedQuery {
            name,
            a: a.clone(),
            b: b.clone(),
            answer: &answers[&point_key(t)],
        })
        .collect::<Vec<_>>();
    let analysis = analyze_panel(&f, &q, &tagged)?;
    for x in analysis["recovered_scalars"]
        .as_array()
        .ok_or("toy scalar array")?
    {
        if x.as_str() != Some("5") {
            return Err("toy held-out scalar mismatch".into());
        }
    }
    emit(
        out,
        json!({"kind":"toy_tagged_panel","analysis":analysis,"known_x5_validator_only":true}),
    )?;
    emit(
        out,
        json!({"kind":"toy_summary","prime_images":12,"raw_sectors":96,"direct_points":104,"direct_torus_parameters":102,"ordered_multiplier_raw_lifts":total,"all_fibres_pair_sets_and_transports_equal":true,"scope":"complete declared fixed toy correctness, no actual-field scaling evidence"}),
    )
}
fn actual(out: &mut dyn Write) -> Result<()> {
    let s = spec(true, 20633)?;
    let f = Family::new(&s)?;
    // Public previous known-consistent image; no scalar or original pair enters query().
    let q = Some((
        decimal("48173329619960954078050199146764420134354751059177422565294150542832202044900")?,
        decimal("47328770622582993615148102044202909030051215429879810403631724073618663020849")?,
    ));
    let targets = panel(&f, &q)?;
    let mut answers = vec![];
    for (name, _, _, target) in &targets {
        answers.push(audited(out, &format!("actual-{name}"), &f, target)?);
    }
    let tagged = targets
        .iter()
        .zip(&answers)
        .map(|((name, a, b, _), answer)| TaggedQuery {
            name,
            a: a.clone(),
            b: b.clone(),
            answer,
        })
        .collect::<Vec<_>>();
    let analysis = analyze_panel(&f, &q, &tagged)?;
    emit(
        out,
        json!({"kind":"actual_tagged_panel","analysis":analysis,"group_setup":{"ell":f.ell.to_string(),"torsion":f.torsion.iter().map(point_json).collect::<Vec<_>>(),"inv8":f.inv8.to_string()},"scope":"four fixed public-tag targets on adapted bases; previous known-consistent Q, not random-target yield or full-base solve"}),
    )
}
fn guards(out: &mut dyn Write) -> Result<()> {
    let field = Field::new(13u32.into())?;
    let vecu = |a: &[u32]| a.iter().copied().map(BigUint::from).collect::<Vec<_>>();
    let cases = [
        (
            "private",
            vec![vecu(&[1, 1, 0, 0]), vecu(&[0, 0, 1, 1])],
            vecu(&[0, 0]),
            vecu(&[1, 1]),
            false,
        ),
        (
            "duplicate",
            vec![vecu(&[1, 1]), vecu(&[1, 1])],
            vecu(&[0, 0]),
            vecu(&[1, 1]),
            false,
        ),
        (
            "signed-duplicate",
            vec![vecu(&[1, 1]), vecu(&[12, 12])],
            vecu(&[0, 0]),
            vecu(&[1, 12]),
            false,
        ),
        (
            "synthetic-informative",
            vec![vecu(&[1, 1, 0]), vecu(&[0, 1, 1]), vecu(&[1, 0, 12])],
            vecu(&[0, 10, 8]),
            vecu(&[1, 0, 0]),
            true,
        ),
    ];
    for (name, m, a, b, informative) in cases {
        let result = linear::analyze(&field, &m, &a, &b)?;
        if result.recovered.is_empty() == informative {
            return Err("synthetic informative status mismatch".into());
        }
        if informative && result.recovered != vec![BigUint::from(5u32)] {
            return Err("synthetic scalar mismatch".into());
        }
        emit(
            out,
            json!({"kind":"synthetic_linear","name":name,"analysis":result.record,"scope":"declared synthetic algebra only"}),
        )?;
    }
    let m = vec![vecu(&[1, 1]), vecu(&[1, 1])];
    let mut r = linear::reduce(&field, &m)?;
    r.left[1][0] = field.add(&r.left[1][0], &BigUint::one());
    let mut refusals = 0;
    let mut reject = |v: bool| -> Result<()> {
        if !v {
            return Err("guard unexpectedly accepted".into());
        }
        refusals += 1;
        Ok(())
    };
    reject(linear::verify(&field, &m, &r).is_err())?;
    let mut r = linear::reduce(&field, &m)?;
    r.operations.push(linear::Op::Scale(0, BigUint::zero()));
    reject(linear::verify(&field, &m, &r).is_err())?;
    reject(linear::reduce(&field, &vec![vecu(&[1]), vecu(&[1, 2])]).is_err())?;
    reject(linear::reduce(&field, &vec![vecu(&[13])]).is_err())?;
    reject(linear::reduce(&field, &vec![vec![BigUint::zero(); 513]]).is_err())?;
    let f = Family::new(&spec(false, 8)?)?;
    reject(f.sectors(&None).is_err())?;
    reject(f.sectors(&Some((0u32.into(), 1u32.into()))).is_err())?;
    reject(f.sectors(&f.torsion[1]).is_err())?;
    reject(query(&f, &f.g, 0).is_err())?;
    emit(
        out,
        json!({"kind":"guard_summary","refusals":refusals,"caps_are_incomplete":true,"scope":"fixed malformed/resource/row-operation/kernel controls; wrong-target tag also checked in both panels"}),
    )
}
fn identity_control(out: &mut dyn Write) -> Result<()> {
    let a = E::var("a");
    let ca = E::var("A");
    let eps = E::var("eps");
    let n = E::times(ca.clone(), E::plus(eps.clone(), E::minus(a.clone())));
    let d = E::times(E::Const(2), a.clone());
    let statement = IdentityStatement {
        name: "original inverse torus real-coordinate denominator relation".into(),
        source: "classical rearrangement of x=A(eps-a)/(2a); torsion-column-transport DERIVATION"
            .into(),
        variables: ["a", "A", "eps"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: E::times(
            E::plus(E::times(E::Const(2), n), E::times(ca.clone(), d.clone())),
            a,
        ),
        rhs: E::product(vec![ca, eps, d]),
    };
    let cert = identity::issue(
        statement.clone(),
        2026112404,
        32,
        "producer evaluated; no independently signed formal pointer",
        None,
    )
    .map_err(|e| e.to_string())?;
    let verdict =
        identity::check_with_extra(&cert, Some((2026112405, 32))).map_err(|e| e.to_string())?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err("inverse identity refused".into());
    }
    let mut wrong = statement;
    wrong.rhs = E::plus(wrong.rhs, E::Const(1));
    if identity::issue(wrong, 2026112404, 32, "plus-one control", None).is_ok() {
        return Err("false inverse identity issued".into());
    }
    emit(
        out,
        json!({"kind":"identity","certificate":cert,"check":format!("{verdict:?}"),"plus_one_refused":true,"scope":"evaluated format2^61-1 identity only; not native inverse-map/group proof"}),
    )
}
pub fn run(out: &mut impl Write) -> Result<()> {
    type Group = fn(&mut dyn Write) -> Result<()>;
    let groups: [(&str, Group); 4] = [
        ("toy", toy),
        ("actual", actual),
        ("guards", guards),
        ("identity", identity_control),
    ];
    let mut failed = 0;
    for (name, group) in groups {
        emit(out, json!({"kind":"group_start","name":name}))?;
        match group(out) {
            Ok(()) => emit(out, json!({"kind":"group_complete","name":name}))?,
            Err(error) => {
                failed += 1;
                emit(
                    out,
                    json!({"kind":"group_error","name":name,"error":error,"remaining_group_arms":"unattempted; exact declared scope in PROTOCOL; later groups still attempted"}),
                )?;
            }
        }
    }
    emit(
        out,
        json!({"kind":"complete","groups_planned":4,"groups_attempted":4,"groups_completed":4-failed,"groups_failed":failed,"scope":"fixed correctness and tagged panel only; no timing/family/exponent claim"}),
    )?;
    if failed > 0 {
        Err(format!("{failed} fixed groups failed"))
    } else {
        Ok(())
    }
}
