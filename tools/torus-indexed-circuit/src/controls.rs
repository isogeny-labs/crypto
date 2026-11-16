//! Fixed construction/extension controls, not a solver or a cost comparison.
use crate::{Augmented, Result};
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use std::{collections::BTreeSet, io::Read};
use torus_coset_circuit::{
    aggregation_controls::FIXTURE,
    index::{canonical_point, negate, IndexTable},
    Circuit, Spec, WitnessInput,
};
use torus_s4::{Curve, Field, Point};
use torus_s4_tree::{Fp2, TorusParameters};

pub const FULL: &[u8] = include_bytes!("../../../research/curve25519_torus_native_20261115/target-circuit-v2/native-v2/six-member-system.json.gz");
pub struct Artifacts {
    pub report: Value,
    pub trace: Vec<u8>,
    pub system: Value,
    pub witness: Vec<String>,
}
fn point(p: &Point) -> Value {
    p.as_ref()
        .map(|(u, v)| json!([u.to_string(), v.to_string()]))
        .unwrap_or(Value::Null)
}
fn toy() -> Spec {
    Spec {
        p: 101u32.into(),
        h: Fp2 {
            a: 17u32.into(),
            b: 12u32.into(),
        },
        order_factor: 17,
        q: 3,
        k: 8,
        curve_a: Some(95u32.into()),
    }
}
fn decimal(v: &Value) -> Result<BigUint> {
    v.as_str()
        .ok_or("expected decimal string")?
        .parse()
        .map_err(|_| "bad decimal".into())
}
fn raw_input(s: &Spec, j: u64, t: u64, sign: bool) -> Result<WitnessInput> {
    let f = Field::new(s.p.clone())?;
    let curve = Curve {
        f: f.clone(),
        a: s.curve_a.clone().ok_or("missing curve")?,
    };
    let z =
        s.h.pow(&(BigUint::from(j) + BigUint::from(s.order_factor) * t), &f);
    let half = f.mul(&curve.a, &f.inv(&2u32.into())?);
    let center = f.sub(&0u32.into(), &half);
    let delta = f.mul(&half, &f.inv(&z.a)?);
    let mut found = vec![];
    for (branch, u) in [f.add(&center, &delta), f.sub(&center, &delta)]
        .into_iter()
        .enumerate()
    {
        if let Some(v) = f.sqrt(&curve.rhs(&u))? {
            let neg = f.sub(&0u32.into(), &v);
            let canonical = v.min(neg);
            found.push((
                branch,
                if sign {
                    f.sub(&0u32.into(), &canonical)
                } else {
                    canonical
                },
            ));
        }
    }
    if found.len() != 1 {
        return Err("fixture raw branch not unique".into());
    }
    let (branch, raw_v) = found.remove(0);
    Ok(WitnessInput {
        z,
        selector: j - 1,
        slack: s.k - j,
        branch: branch.into(),
        raw_v,
    })
}
fn expect_refusal(s: &Spec) -> Result<()> {
    if Augmented::member(s).is_ok() {
        return Err("malformed constructor accepted".into());
    }
    Ok(())
}

pub fn run() -> Result<Artifacts> {
    let mut trace = vec![];
    let mut fixtures = vec![];
    let mut base = toy();
    base.curve_a = None;
    fixtures.push(("q3/nonpower range", base.clone()));
    let mut power = base.clone();
    power.h = Fp2 {
        a: 84u32.into(),
        b: 89u32.into(),
    };
    power.order_factor = 51;
    power.q = 2;
    power.k = 2;
    fixtures.push(("q2/power-of-two range", power));
    let mut large = base.clone();
    large.order_factor = 3;
    large.q = 17;
    large.k = 1;
    fixtures.push(("q17/zero selector width", large));
    let mut one = base.clone();
    one.h = base.h.pow(&3u32.into(), &Field::new(base.p.clone())?);
    one.q = 1;
    one.k = 1;
    fixtures.push(("q1/k1/both widths zero", one));
    let mut member_results = vec![];
    let mut overflow_rejections = 0;
    let mut matching_overflow_rejections = 0;
    for (name, s) in fixtures {
        let (c, a) = Augmented::member(&s)?;
        if !a.prefix_retained() {
            return Err("original member prefix lost".into());
        }
        let f = Field::new(s.p.clone())?;
        let start = trace.len();
        let mut original_roots = 0;
        let mut index_checks = 0;
        let mut roots = vec![];
        for j in 1..=s.k {
            let omega = s.h.pow(&(BigUint::from(s.q) * j), &f);
            for real in 0u32..101 {
                for imag in 0u32..101 {
                    let z = Fp2 {
                        a: real.into(),
                        b: imag.into(),
                    };
                    let w = c.witness(&WitnessInput {
                        z: z.clone(),
                        selector: j - 1,
                        slack: s.k - j,
                        branch: 0u32.into(),
                        raw_v: 0u32.into(),
                    })?;
                    let original = c.system.check(&w).is_ok();
                    let direct = z.norm(&f) == BigUint::one() && z.pow(&s.q.into(), &f) == omega;
                    if original != direct {
                        return Err("original/direct member disagreement".into());
                    }
                    let mut accepted = vec![];
                    if original {
                        original_roots += 1;
                        let width = 64 - (s.q - 1).leading_zeros();
                        for t in 0..1u64 << width {
                            index_checks += 1;
                            let ext = a.extension_candidate(&w, &[t])?;
                            let got = a.system.check(&ext).is_ok();
                            let parameter = s
                                .h
                                .pow(&(BigUint::from(j) + BigUint::from(s.order_factor) * t), &f);
                            let expected = t < s.q && parameter == z;
                            if got != expected {
                                return Err("indexed member root disagreement".into());
                            }
                            if got {
                                if a.project_and_check(&ext)? != w || a.indices(&ext)? != vec![t] {
                                    return Err("member projection/index mismatch".into());
                                }
                                accepted.push(t);
                            }
                            if t >= s.q {
                                if got {
                                    return Err("overflow t accepted".into());
                                }
                                overflow_rejections += 1;
                                if parameter == z {
                                    matching_overflow_rejections += 1;
                                }
                            }
                        }
                        if accepted.len() != 1 {
                            return Err("original root lacks unique indexed extension".into());
                        }
                        roots.push(json!({"j":j,"z":[real,imag],"t":accepted[0]}));
                    }
                    let exists = !accepted.is_empty();
                    if original != exists {
                        return Err("member extension mismatch".into());
                    }
                    trace.push(u8::from(original) | (u8::from(exists) << 1));
                }
            }
        }
        member_results.push(json!({"name":name,"a":s.order_factor,"q":s.q,"k":s.k,
            "trace_start":start,"field_pair_checks":trace.len()-start,
            "original_roots":original_roots,"canonical_index_checks":index_checks,
            "roots":roots,"stats":a.stats(),
            "scope":"all field pairs and canonical represented t strings on original roots, not arbitrary auxiliary field assignments"}));
    }
    let mut malformed = vec![];
    for name in [
        "evenp",
        "square2",
        "noncanonicalh",
        "badnorm",
        "q0",
        "a0",
        "qcap",
        "acap",
        "k0",
        "krange",
        "properorder",
    ] {
        let mut s = base.clone();
        match name {
            "evenp" => s.p = 100u32.into(),
            "square2" => s.p = 17u32.into(),
            "noncanonicalh" => s.h.a = s.p.clone(),
            "badnorm" => s.h.b = 13u32.into(),
            "q0" => s.q = 0,
            "a0" => s.order_factor = 0,
            "qcap" => s.q = 1u64 << 32,
            "acap" => s.order_factor = 1u64 << 32,
            "k0" => s.k = 0,
            "krange" => s.k = 9,
            "properorder" => s.h = base.h.pow(&3u32.into(), &Field::new(base.p.clone())?),
            _ => unreachable!(),
        }
        expect_refusal(&s)?;
        malformed.push(name);
    }
    if Augmented::target(&toy(), 1, None, true).is_ok() {
        return Err("arity refusal failed".into());
    }
    malformed.push("targetarity");

    let s = toy();
    let f = Field::new(s.p.clone())?;
    let curve = Curve {
        f: f.clone(),
        a: 95u32.into(),
    };
    let member = Circuit::compile(&s)?;
    let table = IndexTable::new(&s)?;
    let mut group = BTreeSet::new();
    group.insert(None);
    let mut point_count = 1;
    for u in 0u32..101 {
        for v in 0u32..101 {
            let p = Some((u.into(), v.into()));
            if curve.contains(&p) {
                point_count += 1;
                group.insert(curve.times(&p, 8)?);
            }
        }
    }
    if point_count != 104 || group.len() != 13 {
        return Err("toy group fixture changed".into());
    }
    let mut signed = vec![];
    let mut excluded = vec![];
    for j in 1..=s.k {
        for t in 0..s.q {
            if canonical_point(&s, j, t)?.is_none() {
                excluded.push(json!({"j":j,"t":t}));
                continue;
            }
            for sign in [false, true] {
                let input = raw_input(&s, j, t, sign)?;
                let w = member.witness(&input)?;
                member.system.check(&w)?;
                let p = member.image(&w)?;
                if p != if sign {
                    negate(&canonical_point(&s, j, t)?, &f)
                } else {
                    canonical_point(&s, j, t)?
                } {
                    return Err("toy signed leaf replay mismatch".into());
                }
                signed.push((j, t, input, p));
            }
        }
    }
    let mut templates = vec![];
    for q in &group {
        let (original, aug) = Augmented::target(&s, 2, q.clone(), true)?;
        templates.push((q.clone(), original, aug));
    }
    let target_start = trace.len();
    let mut pairs = 0;
    let mut accepted_targets = 0;
    for left in &signed {
        for right in &signed {
            if (left.0, left.1) == (right.0, right.1) {
                continue;
            }
            pairs += 1;
            let expected = curve.add(&left.3, &right.3)?;
            let w = templates[0].1.witness(&[left.2.clone(), right.2.clone()])?;
            let ext = templates[0].2.extension_candidate(&w, &[left.1, right.1])?;
            for (q, c, a) in &templates {
                let old = c.system.check(&w).is_ok();
                let new = a.system.check(&ext).is_ok();
                if old != new || new != (*q == expected) {
                    return Err("toy original/augmented target mismatch".into());
                }
                if new {
                    accepted_targets += 1;
                    c.decode_row(&w, &table)?;
                    if a.project_and_check(&ext)? != w || a.indices(&ext)? != vec![left.1, right.1]
                    {
                        return Err("toy target root projection mismatch".into());
                    }
                }
                trace.push(u8::from(old) | (u8::from(new) << 1) | (u8::from(*q == expected) << 2));
            }
        }
    }
    if pairs != 1848 || accepted_targets != 1848 || trace.len() - target_start != 24024 {
        return Err("toy target coverage changed".into());
    }

    let old = TorusParameters::actual_four()?;
    let s = Spec {
        p: old.f.p.clone(),
        h: old.h,
        order_factor: 60824497,
        q: 213156431,
        k: 20633,
        curve_a: Some(486662u32.into()),
    };
    let f = Field::new(s.p.clone())?;
    let curve = Curve {
        f: f.clone(),
        a: 486662u32.into(),
    };
    let member = Circuit::compile(&s)?;
    let frozen: Value = serde_json::from_slice(FIXTURE).map_err(|e| e.to_string())?;
    let cases = frozen["actual"]["cases"]
        .as_array()
        .ok_or("missing old fixture")?;
    let ids = [0usize, 2, 4, 6, 8, 10];
    let mut inputs = vec![];
    let mut ts = vec![];
    let mut images = vec![];
    let mut target = None;
    for i in ids {
        let r = &cases[i];
        let j = r["j"].as_u64().ok_or("missing j")?;
        let input = WitnessInput {
            z: Fp2 {
                a: decimal(&r["z"][0])?,
                b: decimal(&r["z"][1])?,
            },
            selector: j - 1,
            slack: s.k - j,
            branch: r["branch"].as_u64().ok_or("branch")?.into(),
            raw_v: decimal(&r["raw_point"][1])?,
        };
        let w = member.witness(&input)?;
        member.system.check(&w)?;
        let image = member.image(&w)?;
        target = curve.add(&target, &image)?;
        images.push(image);
        inputs.push(input);
        ts.push([0, 1, s.q - 1][(i / 2) % 3]);
    }
    let (original, aug) = Augmented::target(&s, 6, target.clone(), true)?;
    let mut full = vec![];
    GzDecoder::new(FULL)
        .read_to_end(&mut full)
        .map_err(|e| e.to_string())?;
    let full: Value = serde_json::from_slice(&full).map_err(|e| e.to_string())?;
    if original.system.canonical_json() != full {
        return Err("original actual full system changed".into());
    }
    let w = original.witness(&inputs)?;
    original.system.check(&w)?;
    let ext = aug.extension_candidate(&w, &ts)?;
    if aug.project_and_check(&ext)? != w || aug.indices(&ext)? != ts || !aug.prefix_retained() {
        return Err("actual extension failed".into());
    }
    let table = IndexTable::new(&s)?;
    let row = original.decode_row(&w, &table)?;
    let mut mutations = 0;
    for i in 0..ext.len() {
        let mut changed = ext.clone();
        changed[i] = f.add(&changed[i], &BigUint::one());
        if aug.system.check(&changed).is_ok() {
            return Err(format!("actual augmented mutation {i} passed"));
        }
        mutations += 1;
    }
    let wrong = curve.add(&target, &images[0])?;
    let (_, wrong_aug) = Augmented::target(&s, 6, wrong, true)?;
    if wrong_aug.system.check(&ext).is_ok() {
        return Err("wrong actual target accepted".into());
    }
    let mut bad_ts = ts.clone();
    bad_ts[0] = s.q;
    let overflow = aug.extension_candidate(&w, &bad_ts)?;
    if aug.system.check(&overflow).is_ok() {
        return Err("actual matching-parameter overflow accepted".into());
    }
    if aug.project_and_check(&ext[..ext.len() - 1]).is_ok()
        || aug.extension_candidate(&w[..w.len() - 1], &ts).is_ok()
        || aug.extension_candidate(&w, &ts[..5]).is_ok()
    {
        return Err("malformed assignment shape accepted".into());
    }
    let mut noncanonical = w.clone();
    noncanonical[0] = s.p.clone();
    if aug.extension_candidate(&noncanonical, &ts).is_ok() {
        return Err("noncanonical original assignment accepted".into());
    }
    let row_json: Vec<_> = row
        .iter()
        .map(|r| json!({"j":r.j,"t":r.t,"coefficient":r.coefficient,"point":point(&r.point)}))
        .collect();
    Ok(Artifacts {
        report: json!({"schema":"torus.indexed-circuit.controls/v1",
        "purpose":"fixed construction and original-root extension, not solving or timing",
        "membership":member_results,"toy_overflow_rejections":overflow_rejections,
        "toy_matching_parameter_overflow_rejections":matching_overflow_rejections,
        "malformed_constructor_refusals":malformed,
        "toy_target":{"raw_columns":24,"signed_leaves":signed.len(),"exclusions":excluded,
            "curve_points":point_count,"image_targets":group.len(),"ordered_signed_pairs":pairs,
            "target_checks":trace.len()-target_start,"accepted_targets":accepted_targets,
            "trace_start":target_start,"stats":templates[0].2.stats()},
        "actual":{"fixture_ids":ids,"target":point(&target),"given_extension_t":ts,
            "stats":aug.stats(),"original_stats":original.stats(),"mutation_rejections":mutations,
            "row":row_json,"solver_invocations":0,"root_discovery":false,
            "role":"known original six-member witness extended; labels used only by qualification harness"},
        "standing":"producer unsigned/unverified","primality":"external contract",
        "cost":"not instrumented as complete cold work, timing or memory"}),
        trace,
        system: aug.system.canonical_json(),
        witness: ext.iter().map(ToString::to_string).collect(),
    })
}

#[test]
fn fixed_correspondence_controls() {
    run().expect("indexed original-root correspondence controls");
}
