//! Frozen finite controls; targets alone cross the solver query interface.
use crate::*;
use torus_s4_tree::TorusParameters;

pub const FIXTURE: &[u8] = include_bytes!(
    "../../../research/curve25519_torus_native_20261115/target-circuit-v2/native-v2/controls.json"
);
pub struct Artifacts {
    pub report: Value,
    pub trace: Vec<u8>,
    pub tables: Value,
    pub determinant: Value,
}
fn decimal(v: &Value) -> Result<BigUint> {
    v.as_str()
        .ok_or("expected decimal string")?
        .parse()
        .map_err(|_| "bad decimal".into())
}
fn public_point(v: &Value) -> Result<Point> {
    if v.is_null() {
        Ok(None)
    } else {
        Ok(Some((decimal(&v[0])?, decimal(&v[1])?)))
    }
}
fn answer_json(setup: &Setup, target: &Point, a: &Answer) -> Result<Value> {
    let member = Circuit::compile(&setup.base.spec)?;
    let index = IndexTable::new(&setup.base.spec)?;
    let roots = if let Some(inputs) = &a.inputs {
        let mut rows = vec![];
        for wi in inputs {
            let w = member.witness(wi)?;
            let term = index.decode(&setup.base.spec, &member, &w)?;
            rows.push(
                json!({"j":term.j,"t":term.t,"canonical_coefficient":term.coefficient,
                "z":[wi.z.a.to_string(),wi.z.b.to_string()],"branch":wi.branch.to_string(),
                "raw_v":wi.raw_v.to_string()}),
            );
        }
        Some(rows)
    } else {
        None
    };
    Ok(
        json!({"target":point_json(target),"status":if a.inputs.is_some(){"root returned and replayed"}else{"exhausted full declared finite base"},
        "exhausted":a.exhausted,"root_inputs":roots,"witness":a.witness.as_ref().map(|w|w.iter().map(ToString::to_string).collect::<Vec<_>>()),
        "construction_charged_in_full":setup.events,"query_events":a.events,
        "accounting_scope":"selected development events only; no timing, normalized field-work or performance claim"}),
    )
}
fn table_json(s: &Setup) -> Value {
    json!(s.table.iter().map(|(point,family)|json!({"sum":point_json(point),
        "representatives":family.rows.iter().map(|r|json!({"pivot":r.pivot,
            "reduced_minor_vector":r.reduced.iter().map(ToString::to_string).collect::<Vec<_>>(),
            "raw_columns":r.original.columns.iter().map(|i|json!({"raw_id":s.base.columns[*i].raw_id,"j":s.base.columns[*i].j,"t":s.base.columns[*i].t})).collect::<Vec<_>>(),
            "sign_mask":r.original.signs,"raw_mask":r.original.mask.to_string()})).collect::<Vec<_>>()
    })).collect::<Vec<_>>())
}
fn compression_control(s: &Setup, trace: &mut Vec<u8>) -> Result<Value> {
    let mut full = BTreeMap::<Point, Vec<Half>>::new();
    let mut queries = vec![];
    s.base.halves(s.arity / 2, &mut Events::default(), |h| {
        queries.push(h.clone());
        full.entry(h.point.clone()).or_default().push(h);
        Ok(true)
    })?;
    let start = trace.len();
    let mut expected_yes = 0;
    let mut expected_no = 0;
    let mut first_only_misses = 0;
    let mut first_miss = None;
    for q in &queries {
        for (key, family) in &full {
            let expected = family.iter().any(|h| h.mask & q.mask == 0);
            let actual = s
                .table
                .get(key)
                .ok_or("compressed sum key missing")?
                .disjoint(q.mask)
                .is_some();
            if expected != actual {
                return Err("compressed family changed a disjoint-completion answer".into());
            }
            if expected {
                expected_yes += 1;
                if family[0].mask & q.mask != 0 {
                    first_only_misses += 1;
                    if first_miss.is_none() {
                        let disjoint = family.iter().find(|h| h.mask & q.mask == 0).unwrap();
                        first_miss = Some(json!({"elliptic_half_sum":point_json(key),
                            "forbidden_raw_mask":q.mask.to_string(),"first_witness_raw_mask":family[0].mask.to_string(),
                            "valid_completion_raw_mask":disjoint.mask.to_string(),
                            "scope":"one disjoint-completion lookup, not a whole-target solver-failure claim"}));
                    }
                }
            } else {
                expected_no += 1;
            }
            trace.push(u8::from(expected) | (u8::from(actual) << 1));
        }
    }
    for family in s.table.values() {
        if family.rows.len() > s.dimension {
            return Err("representative dimension exceeded".into());
        }
        let mut check = Family::default();
        for row in &family.rows {
            let ids: Vec<_> = row
                .original
                .columns
                .iter()
                .map(|i| s.base.columns[*i].raw_id)
                .collect();
            if !check.insert(
                wedge(&ids, s.arity / 2, &s.base.curve.f),
                row.original.clone(),
                &s.base.curve.f,
            )? {
                return Err("retained original witnesses are linearly dependent".into());
            }
        }
    }
    Ok(
        json!({"queries":queries.len(),"sum_keys":full.len(),"comparison_cases":trace.len()-start,
        "expected_yes":expected_yes,"expected_no":expected_no,"trace_offset":start,"trace_bytes":trace.len()-start,
        "first_witness_only_missed_completions":first_only_misses,"first_missed_completion":first_miss,
        "trace_layout":"signed half tuples in raw-combination/sign-mask order; inner numerical Point/BTreeMap key order; bit0 full-family answer, bit1 compressed answer"}),
    )
}
fn total(curve: &Curve, points: impl IntoIterator<Item = Point>) -> Result<Point> {
    points.into_iter().try_fold(None, |s, p| curve.add(&s, &p))
}

pub fn run() -> Result<Artifacts> {
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
    let toy_setup = Setup::build(&toy, 6)?;
    if toy_setup.base.columns.len() != 22
        || toy_setup.base.excluded != vec![(1, 2), (8, 2)]
        || toy_setup.half_tuples != 12320
    {
        return Err("fixed toy population changed".into());
    }
    let frozen: Value = serde_json::from_slice(FIXTURE).map_err(|e| e.to_string())?;
    let targets = frozen["toy"]["image_group_points"]
        .as_array()
        .ok_or("fixed public targets missing")?;
    if targets.len() != 13 {
        return Err("fixed toy target count changed".into());
    }
    let mut toy_answers = vec![];
    for target in targets {
        let p = public_point(target)?;
        let a = toy_setup.solve(&p)?;
        if a.inputs.is_none() {
            return Err("toy target has no returned root".into());
        }
        toy_answers.push(answer_json(&toy_setup, &p, &a)?);
    }
    let mut trace = vec![];
    let toy_compression = compression_control(&toy_setup, &mut trace)?;

    let old = TorusParameters::actual_four()?;
    let actual = Spec {
        p: old.f.p.clone(),
        h: old.h.pow(&213156431u32.into(), &old.f),
        order_factor: 60824497,
        q: 1,
        k: 6,
        curve_a: Some(486662u32.into()),
    };
    let actual_setup = Setup::build(&actual, 6)?;
    if actual_setup.base.columns.len() != 6 || actual_setup.half_tuples != 160 {
        return Err("actual six-column population changed".into());
    }
    let mut actual_answers = vec![];
    for arity in [2, 4, 6] {
        let setup = Setup::build(&actual, arity)?;
        let target = total(
            &setup.base.curve,
            setup.base.columns[..arity].iter().map(|c| c.point.clone()),
        )?;
        let answer = setup.solve(&target)?;
        if answer.inputs.is_none() {
            return Err("known-answer actual target missed".into());
        }
        actual_answers.push(json!({"fixture":"known-answer target; solver receives target only","setup":setup.summary(),
            "answer":answer_json(&setup,&target,&answer)?}));
    }
    let c = &actual_setup.base.curve;
    let u = BigUint::from(9u32);
    let v = c.f.sqrt(&c.rhs(&u))?.ok_or("RFC u9 point missing")?;
    let public_target = Some((u, v));
    let answer = actual_setup.solve(&public_target)?;
    let mut brute_roots = 0;
    for signs in 0usize..64 {
        let sum = total(
            c,
            actual_setup
                .base
                .columns
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    if signs >> i & 1 == 1 {
                        negate(&col.point, &c.f)
                    } else {
                        col.point.clone()
                    }
                }),
        )?;
        if sum == public_target {
            brute_roots += 1;
        }
    }
    if answer.inputs.is_some() != (brute_roots > 0) || answer.exhausted != (brute_roots == 0) {
        return Err("actual full six-column reference disagrees with all64 signs".into());
    }
    let actual_compression = compression_control(&actual_setup, &mut trace)?;
    let mut full_actual = actual.clone();
    full_actual.h = old.h;
    full_actual.q = 213156431;
    full_actual.k = 20633;
    if Setup::build(&full_actual, 6).is_ok() {
        return Err("full-base resource guard ignored".into());
    }
    if Setup::build(&actual, 3).is_ok() {
        return Err("unsupported arity accepted".into());
    }
    let mut small = actual.clone();
    small.p = 5u32.into();
    if Setup::build(&small, 6).is_ok() {
        return Err("unsafe label field accepted".into());
    }
    if actual_setup
        .solve(&Some((actual.p.clone(), BigUint::zero())))
        .is_ok()
    {
        return Err("malformed target accepted".into());
    }

    let f = Field::new(101u32.into())?;
    let mut synthetic = Family::default();
    let left = Half {
        columns: vec![0, 1, 2],
        signs: 0,
        mask: 7,
        point: None,
    };
    let right = Half {
        columns: vec![3, 4, 5],
        signs: 0,
        mask: 56,
        point: None,
    };
    synthetic.insert(wedge(&left.columns, 3, &f), left.clone(), &f)?;
    synthetic.insert(wedge(&right.columns, 3, &f), right, &f)?;
    if left.mask & 7 == 0 || synthetic.disjoint(7).is_none() {
        return Err("synthetic first-witness miss not demonstrated".into());
    }
    let duplicate = Half {
        columns: vec![0, 0, 2],
        signs: 0,
        mask: 5,
        point: None,
    };
    if synthetic.insert(wedge(&duplicate.columns, 3, &f), duplicate, &f)? {
        return Err("duplicate-column minor vector not rejected as dependent".into());
    }
    let certificate = determinant::certificate()?;
    let identity = serde_json::to_value(&certificate).map_err(|e| e.to_string())?;
    Ok(Artifacts {
        report: json!({"schema":"torus-public-reference.controls/v1","scope":"fixed native correctness; no benchmark, sampling or scaling measurement",
            "toy":{"setup":toy_setup.summary(),"answers":toy_answers,"compression":toy_compression},
            "actual_six_column":{"p":actual.p.to_string(),"h":[actual.h.a.to_string(),actual.h.b.to_string()],
                "a":actual.order_factor,"q":1,"k":6,"setup":actual_setup.summary(),"known_answers":actual_answers,
                "unplanted_public_target":answer_json(&actual_setup,&public_target,&answer)?,
                "full_bruteforce_sign_assignments":64,"bruteforce_roots":brute_roots,"compression":actual_compression},
            "synthetic":{"first_witness_only_misses":true,"representative_family_preserves":true,"duplicate_column_vector_rejected":true},
            "rejections":["huge actual base before enumeration","unsupported arity","unsafe label field","noncanonical target"],
            "determinant_identity":certificate.id,"identity_tier":"evaluated Mersenne-field identity, not formal proof",
            "novel_algorithm_claim":false,"gamma_below_two":false,"signed_standing":"unverified"}),
        trace,
        tables: json!({"toy":table_json(&toy_setup),"actual_six_column":table_json(&actual_setup)}),
        determinant: identity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_roots_exhaustion_and_all_disjoint_completion_queries() {
        let a = run().unwrap();
        assert_eq!(a.report["toy"]["answers"].as_array().unwrap().len(), 13);
        assert_eq!(a.report["toy"]["setup"]["signed_half_tuples"], 12320);
        assert!(
            a.report["toy"]["setup"]["max_witnesses_per_sum"]
                .as_u64()
                .unwrap()
                <= 20
        );
        assert_eq!(
            a.report["actual_six_column"]["full_bruteforce_sign_assignments"],
            64
        );
    }
}
