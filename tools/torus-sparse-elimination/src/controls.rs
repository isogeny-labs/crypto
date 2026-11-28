//! Predeclared fixed correctness controls; no performance/solver campaign.
use crate::{solve, Answer, Limits, Report, Stop};
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use std::io::Read;
use torus_coset_circuit::{
    aggregation::TargetCircuit,
    index::{canonical_point, IndexTable},
    Spec,
};
use torus_s4::{curve25519_prime, Curve, Field, Poly, System};
use torus_s4_tree::TorusParameters;

pub const FULL_FIXTURE:&[u8]=include_bytes!("../../../research/curve25519_torus_native_20261115/target-circuit-v2/native-v2/six-member-system.json.gz");
type Result<T> = std::result::Result<T, String>;
fn system(p: &BigUint, n: usize, equations: Vec<Poly>) -> System {
    System {
        p: p.clone(),
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        equations,
    }
}
fn var(p: &BigUint, i: usize) -> Poly {
    Poly::var(p, i)
}
fn c(p: &BigUint, n: u64) -> Poly {
    Poly::small(p, n)
}
fn root(r: &Report, s: &System) -> Result<Option<Vec<BigUint>>> {
    if let Answer::Root(values) = &r.answer {
        let values: Vec<BigUint> = values
            .iter()
            .map(|s| s.parse::<BigUint>().map_err(|e| e.to_string()))
            .collect::<Result<_>>()?;
        s.check(&values)?;
        let at = s
            .equations
            .iter()
            .flat_map(|p| p.terms.keys())
            .find_map(|m| m.first().copied())
            .ok_or("root test has no constrained variable")?;
        let mut bad = values.clone();
        bad[at] = (&bad[at] + 1u32) % &s.p;
        if s.check(&bad).is_ok() {
            return Err("mutated root passed original replay".into());
        }
        if s.check(&values[..values.len() - 1]).is_ok() {
            return Err("truncated assignment accepted".into());
        }
        Ok(Some(values))
    } else {
        Ok(None)
    }
}
fn from_json(v: &Value) -> Result<System> {
    let p = v["p"]
        .as_str()
        .ok_or("missingp")?
        .parse::<BigUint>()
        .map_err(|e| e.to_string())?;
    let variables = v["variables"]
        .as_array()
        .ok_or("variables")?
        .iter()
        .map(|x| {
            x.as_str()
                .map(str::to_string)
                .ok_or_else(|| "variable name".to_string())
        })
        .collect::<Result<Vec<_>>>()?;
    let mut equations = vec![];
    for terms in v["equations"].as_array().ok_or("equations")? {
        let mut poly = Poly::small(&p, 0);
        for t in terms.as_array().ok_or("terms")? {
            let m = t["monomial"]
                .as_array()
                .ok_or("monomial")?
                .iter()
                .map(|x| {
                    x.as_u64()
                        .and_then(|x| usize::try_from(x).ok())
                        .ok_or_else(|| "index".into())
                })
                .collect::<Result<Vec<_>>>()?;
            let c = t["coefficient"]
                .as_str()
                .ok_or("coefficient")?
                .parse::<BigUint>()
                .map_err(|e| e.to_string())?;
            if poly.terms.insert(m, c).is_some() {
                return Err("duplicate serialized term".into());
            }
        }
        equations.push(poly)
    }
    Ok(System {
        p,
        variables,
        equations,
    })
}
pub struct Artifacts {
    pub report: Value,
    pub q1_system: Value,
}
pub fn run() -> Result<Artifacts> {
    let mut sqrt_controls = vec![];
    for p in [BigUint::from(101u32), curve25519_prime()] {
        let f = Field::new(p.clone())?;
        let values = if p == BigUint::from(101u32) {
            (0u32..101).map(BigUint::from).collect::<Vec<_>>()
        } else {
            vec![
                0u32.into(),
                1u32.into(),
                2u32.into(),
                5u32.into(),
                &p - 1u32,
            ]
        };
        for a in values {
            let mut ctx = crate::Context {
                scalar_bytes: p.bits().div_ceil(8),
                f: f.clone(),
                limits: Limits::default(),
                c: crate::Counts::default(),
            };
            let got = ctx.sqrt(&a).map_err(|e| format!("counted sqrt: {e:?}"))?;
            if got != f.sqrt(&a)? {
                return Err("instrumented/unchanged sqrt disagreement".into());
            }
            sqrt_controls.push(json!({"p":p.to_string(),"input":a.to_string(),"root":got.map(|x|x.to_string()),"counted_algorithm":ctx.c}));
        }
    }
    let mut cases = vec![];
    for p in [BigUint::from(101u32), curve25519_prime()] {
        let x = var(&p, 0);
        let y = var(&p, 1);
        let triangular = system(&p, 2, vec![&x - &(&y + &c(&p, 2)), &y - &c(&p, 7)]);
        let r = solve(&triangular, Limits::default());
        if root(&r, &triangular)?.is_none() {
            return Err(format!("triangular root missing: {:?}", r.answer));
        }
        cases.push(json!({"case":"triangular","p":p.to_string(),"report":r}));
        let coupled = system(&p, 2, vec![&(&x * &y) - &c(&p, 1), &(&x + &y) - &c(&p, 3)]);
        let r = solve(&coupled, Limits::default());
        // Whether5 is a square depends on the field; keep the actual rational outcome.
        let expected = Field::new(p.clone())?.sqrt(&5u32.into())?.is_some();
        if root(&r, &coupled)?.is_some() != expected || r.counts.buchberger_calls == 0 {
            return Err(format!("coupled extraction mismatch: {:?}", r.answer));
        }
        cases.push(json!({"case":"coupled quadratic requiring elimination","p":p.to_string(),"rational_expected":expected,"report":r}));
        let inconsistent = system(&p, 1, vec![&x - &c(&p, 1), &x - &c(&p, 2)]);
        let r = solve(&inconsistent, Limits::default());
        if !matches!(r.answer, Answer::NoRootObservation) {
            return Err("contradiction classification".into());
        }
        cases.push(json!({"case":"contradiction observation, not certificate","p":p.to_string(),"report":r}));
        let high = system(
            &p,
            1100,
            vec![&var(&p, 1099) - &Poly::constant(&p, &p - 2u32)],
        );
        let r = solve(&high, Limits::default());
        let values = root(&r, &high)?.ok_or("dynamic root missing")?;
        if values[1099] != &p - 2u32 || values.len() != 1100 {
            return Err("wide dynamic assignment truncated".into());
        }
        cases.push(json!({"case":"1100variables, constrained index1099 and large coefficient","p":p.to_string(),"report":r}));
    }
    let p = BigUint::from(101u32);
    let x = var(&p, 0);
    let y = var(&p, 1);
    for (value, expect_root) in [(1u64, true), (2, false)] {
        let s = system(&p, 1, vec![&(&x * &x) - &c(&p, value)]);
        let r = solve(&s, Limits::default());
        if root(&r, &s)?.is_some() != expect_root {
            return Err("quadratic root control".into());
        }
        if !expect_root && !matches!(r.answer, Answer::NoRootObservation) {
            return Err("nonsquare observation classification".into());
        }
        cases.push(
            json!({"case":"rational/nonsquare quadratic","constant":value,"p":"101","report":r}),
        );
    }
    let mut cube = c(&p, 0);
    cube.terms.insert(vec![0, 0, 0], BigUint::one());
    cube.terms.insert(vec![], &p - 1u32);
    let cubic = system(&p, 1, vec![cube]);
    let r = solve(&cubic, Limits::default());
    if !matches!(r.answer, Answer::Stopped(Stop::Unsupported(_))) {
        return Err("cubic refusal classification".into());
    }
    cases.push(json!({"case":"unsupported cubic extraction","report":r}));
    let pair = system(&p, 2, vec![&(&x * &y) - &c(&p, 1), &(&x + &y) - &c(&p, 3)]);
    let square = system(&p, 1, vec![&(&x * &x) - &c(&p, 1)]);
    let linear = system(&p, 1, vec![&x - &c(&p, 7)]);
    let mut cap_cases = vec![];
    for name in [
        "work",
        "storage",
        "degree",
        "terms",
        "pairs",
        "basis",
        "depth",
        "variables",
        "equations",
    ] {
        let mut l = Limits::default();
        let input = match name {
            "work" => {
                l.work = 0;
                &linear
            }
            "storage" => {
                l.bytes = 0;
                &linear
            }
            "degree" => {
                l.degree = 1;
                &square
            }
            "terms" => {
                l.terms = 0;
                &linear
            }
            "pairs" => {
                l.pairs = 0;
                &pair
            }
            "basis" => {
                l.basis = 1;
                &pair
            }
            "depth" => {
                l.depth = 0;
                &square
            }
            "variables" => {
                l.variables = 0;
                &linear
            }
            "equations" => {
                l.equations = 0;
                &linear
            }
            _ => unreachable!(),
        };
        let r = solve(input, l);
        if !matches!(r.answer, Answer::Stopped(Stop::Cap(_))) {
            return Err(format!("forced {name} cap classified {:?}", r.answer));
        }
        if name == "basis" && r.counts.field_inverse != 0 {
            return Err("basis admission performed normalization before refusing".into());
        }
        cap_cases.push(json!({"cap":name,"report":r}));
    }
    let mut bad = system(&p, 1, vec![var(&p, 2)]);
    let r = solve(&bad, Limits::default());
    if !matches!(r.answer, Answer::Stopped(Stop::Invalid(_))) {
        return Err("invalid index accepted".into());
    }
    bad.p = 100u32.into();
    let even = solve(&bad, Limits::default());
    if !matches!(even.answer, Answer::Stopped(Stop::Invalid(_))) {
        return Err("even modulus accepted".into());
    }
    let mut negative_coefficient = linear.canonical_json();
    negative_coefficient["equations"][0][0]["coefficient"] = json!("-1");
    let mut negative_index = linear.canonical_json();
    negative_index["equations"][0][0]["monomial"] = json!([-1]);
    if from_json(&negative_coefficient).is_ok() || from_json(&negative_index).is_ok() {
        return Err("malformed serialized input accepted".into());
    }
    let mut noncanonical = c(&p, 0);
    noncanonical.terms.insert(vec![0], 203u32.into());
    noncanonical.terms.insert(vec![], 94u32.into());
    let normalized = system(&p, 1, vec![noncanonical]);
    let normalized_report = solve(&normalized, Limits::default());
    if root(&normalized_report, &normalized)?.ok_or("canonicalization root missing")?[0]
        != 7u32.into()
    {
        return Err("coefficient canonicalization changed field".into());
    }
    let old = TorusParameters::actual_four()?;
    let spec = Spec {
        p: old.f.p.clone(),
        h: old.h.pow(&213156431u32.into(), &old.f),
        order_factor: 60824497,
        q: 1,
        k: 2,
        curve_a: Some(486662u32.into()),
    };
    let curve = Curve {
        f: old.f.clone(),
        a: 486662u32.into(),
    };
    let p1 = canonical_point(&spec, 1, 0)?;
    let p2 = canonical_point(&spec, 2, 0)?;
    let target = curve.add(&p1, &p2)?;
    let circuit = TargetCircuit::compile(&spec, 2, target.clone(), true)?;
    let actual = solve(&circuit.system, Limits::default());
    if matches!(
        actual.answer,
        Answer::NoRootObservation | Answer::Stopped(Stop::Invalid(_))
    ) {
        return Err(format!(
            "known-consistent actual control reported invalid/contradiction: {:?}",
            actual.answer
        ));
    }
    let row = if let Some(values) = root(&actual, &circuit.system)? {
        let table = IndexTable::new(&spec)?;
        Some(
            circuit
                .decode_row(&values, &table)?
                .into_iter()
                .map(|r| json!({"j":r.j,"t":r.t,"coefficient":r.coefficient}))
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };
    let mut bytes = vec![];
    GzDecoder::new(FULL_FIXTURE)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let full = from_json(&serde_json::from_slice::<Value>(&bytes).map_err(|e| e.to_string())?)?;
    if full.variables.len() != 1059 || full.equations.len() != 1089 {
        return Err("frozen original input dimensions changed".into());
    }
    let forced = solve(
        &full,
        Limits {
            work: 1,
            ..Limits::default()
        },
    );
    if !matches!(forced.answer, Answer::Stopped(Stop::Cap(_))) {
        return Err("full input forced refusal".into());
    }
    Ok(Artifacts {
        q1_system: circuit.system.canonical_json(),
        report: json!({"schema":"torus.sparse-elimination.control/v1","standing":"producer unsigned/unverified",
        "scope":"fixed ordinary Buchberger/root-extraction correctness; no full-base, absence certificate or performance result",
        "limits":Limits::default(),"sqrt_cross_checks":sqrt_controls,"fixed_cases":cases,"forced_caps":cap_cases,"malformed":[r,even],
        "serialized_refusals":["negative coefficient","negative monomial index"],"noncanonical_coefficient_control":normalized_report,
        "actual_q1_k2":{"system_stats":circuit.stats(),"target":target.as_ref().map(|(u,v)|vec![u.to_string(),v.to_string()]),"report":actual,"indexed_row":row,"public_solver_inputs":"compiled System only; no labels/signs/scalar/witness passed"},
        "original_full_input_forced_refusal":{"variables":full.variables.len(),"equations":full.equations.len(),"work_cap":1,"report":forced},
        "accounting_scope":"counted primitive arithmetic and conservative explicit structural/represented storage reservations; not RSS, timing or normalized full attack cost. Independent original/group replay and serialization are not priced; original replay calls are recorded."}),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn preregistered_fixed_qualification() {
        super::run().expect("fixed qualification");
    }
}
