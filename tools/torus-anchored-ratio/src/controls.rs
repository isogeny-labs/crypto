//! Fixed declared correctness domains; known labels/tags stay outside query.
use crate::*;
use num_traits::Zero;
use std::io::Write;
use torus_column_transport::{linear, point_key, scalar};
use torus_quotient_query::controls::spec;
use torus_s4::Field;

fn emit(out: &mut dyn Write, v: Value) -> Result<()> {
    writeln!(out, "{v}").map_err(|e| e.to_string())
}
#[derive(Clone)]
struct Leaf {
    z: Fp2,
    raw: Point,
    image: Point,
    branch: usize,
    j: u64,
    t: u64,
}
fn leaves(c: &Context, j: u64, t: u64) -> Result<Vec<Leaf>> {
    let f = &c.family;
    let z = f.s.h.pow(
        &(BigUint::from(j) + BigUint::from(f.s.order_factor) * t),
        &f.cv.f,
    );
    raw_choices(&f.cv, &z)?
        .into_iter()
        .map(|(branch, raw)| {
            let image = f.cv.times(&raw, 8)?;
            Ok(Leaf {
                z: z.clone(),
                raw,
                image,
                branch,
                j,
                t,
            })
        })
        .collect()
}
fn witness_key(a: &Leaf, b: &Leaf) -> String {
    json!({"z":[fp2_json(&a.z),fp2_json(&b.z)],
        "raw":[point_json(&a.raw),point_json(&b.raw)]})
    .to_string()
}
fn keys(a: &Answer) -> BTreeSet<String> {
    a.roots.iter().map(Witness::key).collect()
}
fn numeric(c: &Context, t: &Point, a: &Answer, table: &IndexTable) -> Result<Vec<Value>> {
    a.roots
        .iter()
        .map(|w| {
            let r = numeric_replay(c, t, w, table)?;
            for term in r["rows"].as_array().ok_or("numeric row shape")? {
                let j = term["j"].as_u64().ok_or("numeric j")?;
                let tt = term["t"].as_u64().ok_or("numeric t")?;
                let z = c.family.s.h.pow(
                    &(BigUint::from(j) + BigUint::from(c.family.s.order_factor) * tt),
                    &c.family.cv.f,
                );
                if !w.z.contains(&z) {
                    return Err("numeric replay returned a different raw parameter".into());
                }
            }
            Ok(r)
        })
        .collect()
}
fn toy(out: &mut dyn Write) -> Result<()> {
    let c = Context::new(&spec(false, 8)?)?;
    let f = &c.family;
    let mut all = vec![];
    for j in 1..=8 {
        for t in 0..3 {
            all.extend(leaves(&c, j, t)?);
        }
    }
    let mut old = BTreeMap::new();
    for l in &all {
        let (k, _) = column(&f.cv, &l.image)?;
        old.insert(k.clone(), Some(k));
    }
    let old = old.into_values().collect::<Vec<_>>();
    if all.len() != 44 || old.len() != 6 {
        return Err("declared toy signed44/physical6 mismatch".into());
    }
    let mut reference: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut direct_rows = vec![];
    for a in &all {
        for b in &all {
            if a.j + 1 != b.j || a.z == b.z {
                continue;
            }
            let target = f.cv.add(&a.image, &b.image)?;
            if target.is_none() {
                continue;
            }
            let ratio = b.z.mul(&a.z.inverse(&f.cv.f)?, &f.cv.f);
            if ratio.pow(&f.s.q.into(), &f.cv.f) != c.omega {
                return Err("direct ratio guard".into());
            }
            reference
                .entry(point_key(&target))
                .or_default()
                .insert(witness_key(a, b));
            direct_rows.push(
                json!({"j":[a.j,b.j],"t":[a.t,b.t],"branch":[a.branch,b.branch],
            "target":point_json(&target),"key":witness_key(a,b)}),
            );
        }
    }
    emit(
        out,
        json!({"kind":"toy-reference","spec":{"p":"101","q":3,"k":8},
        "signed_leaves":44,"old":old.iter().map(point_json).collect::<Vec<_>>(),
        "direct_rows":direct_rows,"context":c.stats()}),
    )?;
    let table = IndexTable::new(&f.s)?;
    let mut total = 0usize;
    for i in 1..13u64 {
        let t = f.cv.times(&f.g, i)?;
        let a = query(&c, &t, &old, Mode::All, Limits::default())?;
        let expected = reference.get(&point_key(&t)).cloned().unwrap_or_default();
        if !a.exhausted || keys(&a) != expected {
            return Err(format!("toy complete inverse/direct mismatch target{i}"));
        }
        let first = query(&c, &t, &old, Mode::First, Limits::default())?;
        if !keys(&first).is_subset(&expected)
            || first.roots.len() != usize::from(!expected.is_empty())
            || first.exhausted != expected.is_empty()
        {
            return Err("FIRST versus ALL contract".into());
        }
        for w in &a.roots {
            verify(&c, &t, &old, w)?;
        }
        let rows = numeric(&c, &t, &a, &table)?;
        total += a.roots.len();
        emit(
            out,
            json!({"kind":"toy-query","target_index":i,"target":point_json(&t),
            "all":a.json(),"first":first.json(),"numeric_rows":rows,
            "numeric_decode_calls":2*a.roots.len(),"all_direct_keys_equal":true}),
        )?;
    }
    emit(
        out,
        json!({"kind":"toy-summary","targets":12,"complete_root_records":total,
        "reference_shared_primitives":"field/group/raw map; bypasses inverse and BSGS coset search",
        "index_table":table.stats(),"not_a_uniform_yield_or_cost_measurement":true}),
    )
}
fn actual(out: &mut dyn Write) -> Result<()> {
    let c = Context::new(&spec(true, 20633)?)?;
    let f = &c.family;
    let p = leaves(&c, 1, 0)?
        .first()
        .ok_or("actual j1 fixture")?
        .image
        .clone();
    let q = leaves(&c, 2, 0)?
        .first()
        .ok_or("actual j2 fixture")?
        .image
        .clone();
    let t = f.cv.add(&p, &q)?;
    f.check_image(&t)?;
    let table = IndexTable::new(&f.s)?;
    for (name, target, old, mode) in [
        (
            "actual-known-all-prefix",
            t.clone(),
            vec![f.g.clone(), p.clone(), q.clone()],
            Mode::All,
        ),
        (
            "actual-known-first-prefix",
            t.clone(),
            vec![f.g.clone(), p.clone(), q.clone()],
            Mode::First,
        ),
        ("actual-one-old", t.clone(), vec![p.clone()], Mode::All),
        ("actual-minus", neg(&f.cv, &t), vec![p.clone()], Mode::All),
        ("actual-rfc-target", f.g.clone(), vec![p.clone()], Mode::All),
    ] {
        let a = query(&c, &target, &old, mode, Limits::default())?;
        if name != "actual-rfc-target" && a.roots.is_empty() {
            return Err("known-consistent actual root missing".into());
        }
        for w in &a.roots {
            verify(&c, &target, &old, w)?;
        }
        let rows = numeric(&c, &target, &a, &table)?;
        emit(
            out,
            json!({"kind":"actual-query","name":name,"target":point_json(&target),
            "old":old.iter().map(point_json).collect::<Vec<_>>(),"context":c.stats(),
            "answer":a.json(),"numeric_rows":rows,"numeric_decode_calls":2*a.roots.len(),
            "index_setup_once_for_correctness":table.stats(),
            "scope":"selected original-parameter two-member fixture, no full-base yield or DLP recovery"}),
        )?;
    }
    Ok(())
}
fn fv(f: &Field, n: i64) -> BigUint {
    let z = BigUint::from(n.unsigned_abs()) % &f.p;
    if n < 0 {
        f.sub(&BigUint::zero(), &z)
    } else {
        z
    }
}
fn peel(m: &linear::Matrix) -> Value {
    let mut active = (0..m.len()).collect::<BTreeSet<_>>();
    let mut steps = vec![];
    loop {
        let mut next = None;
        for (c, _) in m.first().into_iter().flatten().enumerate() {
            let r = active
                .iter()
                .copied()
                .filter(|i| !m[*i][c].is_zero())
                .collect::<Vec<_>>();
            if r.len() == 1 {
                next = Some((r[0], c));
                break;
            }
        }
        let Some((r, c)) = next else {
            break;
        };
        // The complete active set makes the singleton certificate replayable.
        steps.push(
            json!({"active_rows":active.iter().copied().collect::<Vec<_>>(),"row":r,"column":c,
            "nonzero":m[r][c].to_string()}),
        );
        active.remove(&r);
    }
    json!({"steps":steps,"core_rows":active.into_iter().collect::<Vec<_>>()})
}
fn check_tags(c: &Context, m: &linear::Matrix, a: &[BigUint], b: &[BigUint]) -> Result<()> {
    let f = &c.family;
    let q = f.cv.times(&f.g, 5)?;
    if m.len() != a.len() || a.len() != b.len() {
        return Err("tag shape".into());
    }
    for (i, row) in m.iter().enumerate() {
        let mut sum = None;
        for (j, n) in row.iter().enumerate() {
            sum =
                f.cv.add(&sum, &scalar(&f.cv, &f.cv.times(&f.g, (j + 2) as u64)?, n)?)?;
        }
        let t =
            f.cv.add(&scalar(&f.cv, &f.g, &a[i])?, &scalar(&f.cv, &q, &b[i])?)?;
        if sum != t {
            return Err("synthetic public group-tag mismatch before rank".into());
        }
    }
    Ok(())
}
fn row_models(out: &mut dyn Write) -> Result<()> {
    let c = Context::new(&spec(false, 8)?)?;
    let f = Field::new(13u32.into())?;
    let make = |r: &[i64]| r.iter().map(|n| fv(&f, *n)).collect::<Vec<_>>();
    let tree = vec![make(&[1, 1, 0]), make(&[0, 1, 1])];
    let mut cycle = tree.clone();
    cycle.push(make(&[1, 0, -1]));
    for (name, m, a, b, want) in [
        ("synthetic-tree", tree, make(&[0, 7]), make(&[1, 0]), false),
        (
            "synthetic-closure-zero-tag",
            cycle.clone(),
            make(&[0, 7, 6]),
            make(&[1, 0, 1]),
            false,
        ),
        (
            "synthetic-closure-informative",
            cycle,
            make(&[0, 7, 11]),
            make(&[1, 0, 0]),
            true,
        ),
    ] {
        check_tags(&c, &m, &a, &b)?;
        let mut bad = b.clone();
        bad[0] = f.add(&bad[0], &1u32.into());
        if check_tags(&c, &m, &a, &bad).is_ok() {
            return Err("wrong synthetic tag accepted".into());
        }
        let audit = linear::analyze(&f, &m, &a, &b)?;
        if !audit.recovered.is_empty() != want {
            return Err("synthetic useful coefficient mismatch".into());
        }
        for x in &audit.recovered {
            if scalar(&c.family.cv, &c.family.g, x)? != c.family.cv.times(&c.family.g, 5)? {
                return Err("synthetic scalar group replay".into());
            }
        }
        emit(
            out,
            json!({"kind":"row-model","name":name,"peeling":peel(&m),"audit":audit.record,
            "scope":"synthetic chosen row/tag correctness; not query-discovered recovery"}),
        )?;
    }
    Ok(())
}
fn guards(out: &mut dyn Write) -> Result<()> {
    let c = Context::new(&spec(false, 8)?)?;
    let f = &c.family;
    let p = leaves(&c, 1, 0)?.first().ok_or("guard p")?.image.clone();
    let mut q = leaves(&c, 2, 0)?.first().ok_or("guard q")?.image.clone();
    let mut t = f.cv.add(&p, &q)?;
    // This is a validator fixture, not query selection: avoid the excluded
    // identity sum while retaining both original raw-label positions.
    if t.is_none() {
        q = neg(&f.cv, &q);
        t = f.cv.add(&p, &q)?;
    }
    let old = vec![p.clone()];
    let l = Limits::default();
    let base = query(&c, &t, &old, Mode::First, l)?;
    let w = base.roots.first().ok_or("guard baseline")?;
    let mut count = 0;
    for (name, result) in [
        ("zero-target", query(&c, &None, &old, Mode::All, l)),
        ("empty-old", query(&c, &t, &[], Mode::All, l)),
        ("identity-old", query(&c, &t, &[None], Mode::All, l)),
        (
            "malformed-old",
            query(&c, &t, &[Some((1u32.into(), 1u32.into()))], Mode::All, l),
        ),
        (
            "torsion-old",
            query(&c, &t, &[Some((0u32.into(), 0u32.into()))], Mode::All, l),
        ),
        (
            "duplicate-sign-old",
            query(&c, &t, &[p.clone(), neg(&f.cv, &p)], Mode::All, l),
        ),
        (
            "pair-cap",
            query(&c, &t, &old, Mode::All, Limits { pairs: 1, ..l }),
        ),
        (
            "output-cap-admission",
            query(&c, &t, &old, Mode::All, Limits { outputs: 0, ..l }),
        ),
        (
            "old-cap",
            query(&c, &t, &[p.clone(), q], Mode::All, Limits { old: 1, ..l }),
        ),
    ] {
        let err = result.err().ok_or(format!("guard {name} accepted"))?;
        emit(out, json!({"kind":"guard","name":name,"refusal":err}))?;
        count += 1;
    }
    let mut badspec = f.s.clone();
    badspec.q = 1;
    if Context::new(&badspec).is_ok() {
        return Err("q1 spec accepted".into());
    }
    count += 1;
    for i in 0..w.values.len() {
        let mut bad = w.clone();
        bad.values[i] = f.cv.f.add(&bad.values[i], &1u32.into());
        if verify(&c, &t, &old, &bad).is_ok() {
            return Err("mutated original witness accepted".into());
        }
    }
    for change in 0..4 {
        let mut bad = w.clone();
        match change {
            0 => bad.j[0] += 1,
            1 => bad.z[1] = bad.z[0].clone(),
            2 => bad.raw[0] = None,
            _ => bad.closes_on_old = !bad.closes_on_old,
        }
        if verify(&c, &t, &old, &bad).is_ok() {
            return Err("mutated derived root accepted".into());
        }
    }
    if verify(&c, &f.g, &old, w).is_ok() {
        return Err("wrong target accepted".into());
    }
    emit(
        out,
        json!({"kind":"guard-summary","admission_refusals":count,
        "original_single_variable_mutations":w.values.len(),"root_mutations":4,"wrong_target_rejected":true}),
    )
}
pub fn run(out: &mut dyn Write) -> Result<()> {
    let mut failed = 0;
    for (name, group) in [
        ("toy", toy as fn(&mut dyn Write) -> Result<()>),
        ("actual", actual),
        ("row-models", row_models),
        ("guards", guards),
    ] {
        emit(
            out,
            json!({"kind":"group","name":name,"status":"attempted"}),
        )?;
        match group(out) {
            Ok(()) => emit(out, json!({"kind":"group","name":name,"status":"complete"}))?,
            Err(e) => {
                failed += 1;
                emit(
                    out,
                    json!({"kind":"group","name":name,"status":"failed","error":e}),
                )?;
            }
        }
    }
    emit(out, json!({"kind":"completion","groups":4,"failed":failed}))?;
    if failed == 0 {
        Ok(())
    } else {
        Err("fixed control groups failed".into())
    }
}
