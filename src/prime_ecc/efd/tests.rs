use super::program::{equality, Expr};
use super::*;
use crate::ecc::curve::CurveParams;
use crate::ecc::point::Point;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::BTreeSet;

/// Small prime used for exhaustive model checks. `p = 1 mod 12` keeps square
/// roots of -1 and primitive cube roots of unity available.
const P: u64 = 109;

/// Two upstream `.op3` files compute `-(r1+l3)` where the pinned record says
/// `r2 = -r1+l3`. They are kept byte for byte and excluded from the
/// affine-law comparison; everything else is compared strictly.
const KNOWN_PROGRAM_DEFECTS: &[(&str, &str)] = &[
    (
        "jintersect/standard/doubling/dbl-2001-ls",
        "op3 computes -(r1+l3); record says -r1+l3",
    ),
    (
        "jintersect/extended/doubling/dbl-2001-ls",
        "op3 computes -(r1+l3); record says -r1+l3",
    ),
];

type Point2 = Vec<FieldElement>;

fn modulus() -> BigUint {
    BigUint::from(P)
}
fn fe(v: u64) -> FieldElement {
    FieldElement::new(BigUint::from(v % P), modulus())
}
fn fe_i(v: i64) -> FieldElement {
    let p = P as i64;
    fe((((v % p) + p) % p) as u64)
}
fn one() -> FieldElement {
    fe(1)
}
fn zero() -> FieldElement {
    fe(0)
}

fn coordinates(model: &Model) -> Vec<String> {
    model
        .source
        .text
        .lines()
        .filter_map(|l| l.strip_prefix("coordinate "))
        .map(str::to_owned)
        .collect()
}

fn relations(model: &Model) -> Vec<(Expr, Expr)> {
    model
        .source
        .text
        .lines()
        .filter_map(|l| l.strip_prefix("satisfying "))
        .map(|s| equality(s).unwrap())
        .collect()
}

/// Square root via exhaustive search; only used on the small test field.
fn small_sqrt(v: &FieldElement) -> Option<FieldElement> {
    if v.is_zero() {
        return Some(zero());
    }
    (0..P).map(fe).find(|x| x.mul(x) == *v)
}

fn model_points(model: &Model, params: &Bindings) -> Vec<Point2> {
    let coords = coordinates(model);
    let rels = relations(model);
    let mut out = Vec::new();
    if coords.len() == 2 {
        for a in 0..P {
            for b in 0..P {
                let point = vec![fe(a), fe(b)];
                if satisfies(&coords, &rels, params, &point) {
                    out.push(point);
                }
            }
        }
    } else {
        // Jacobi intersections: s^2+c^2=1 and a s^2+d^2=1.
        let a = params["a"].clone();
        for s in (0..P).map(fe) {
            for c in (0..P).map(fe) {
                if s.mul(&s).add(&c.mul(&c)) != one() {
                    continue;
                }
                let d2 = one().sub(&a.mul(&s.mul(&s)));
                if let Some(d) = small_sqrt(&d2) {
                    out.push(vec![s.clone(), c.clone(), d.clone()]);
                    if !d.is_zero() {
                        out.push(vec![s.clone(), c.clone(), zero().sub(&d)]);
                    }
                }
            }
        }
    }
    out
}

fn satisfies(coords: &[String], rels: &[(Expr, Expr)], params: &Bindings, point: &Point2) -> bool {
    let mut v = params.clone();
    for (name, value) in coords.iter().zip(point) {
        v.insert(name.clone(), value.clone());
    }
    rels.iter()
        .all(|(a, b)| a.eval(&modulus(), &v).unwrap() == b.eval(&modulus(), &v).unwrap())
}

fn oracle(model: &Model, operation: &str, params: &Bindings, args: &[&Point2]) -> Result<Point2> {
    let coords = coordinates(model);
    let mut values = params.clone();
    for (i, point) in args.iter().enumerate() {
        for (name, value) in coords.iter().zip(point.iter()) {
            values.insert(format!("{name}{}", i + 1), value.clone());
        }
    }
    let prefix = format!("{operation} ");
    let mut out = Bindings::new();
    for line in model.source.text.lines() {
        let Some(body) = line.strip_prefix(&prefix) else {
            continue;
        };
        let (name, expression) = body.split_once('=').unwrap();
        out.insert(
            name.trim().into(),
            Expr::parse(expression)?.eval(&modulus(), &values)?,
        );
    }
    Ok(coords.iter().map(|c| out[c].clone()).collect())
}

fn jquartic_params() -> Bindings {
    for c in 1..P {
        let rhs = one().sub(&fe(c).mul(&fe(c)));
        if let Some(a) = small_sqrt(&rhs) {
            return Bindings::from([("a".into(), a), ("c".into(), fe(c))]);
        }
    }
    unreachable!("no Jacobi-quartic parameter pair")
}

fn cube_root_of_unity() -> Option<FieldElement> {
    (2..P)
        .map(fe)
        .find(|w| w.mul(w).add(w).add(&one()).is_zero())
}

fn params_for(rep: &Representation) -> Bindings {
    let model = rep.id.split('/').next().unwrap();
    let mut v = Bindings::new();
    match model {
        "shortw" => {
            let a = if rep.id.ends_with("-0") || rep.id == "shortw/w12-0" {
                fe(0)
            } else if rep.id.ends_with("-1") {
                fe_i(-1)
            } else if rep.id.ends_with("-3") {
                fe_i(-3)
            } else {
                fe(2)
            };
            let b = if rep.id == "shortw/w12-0" {
                fe(0)
            } else {
                fe(3)
            };
            v.insert("a".into(), a);
            v.insert("b".into(), b);
        }
        "montgom" => {
            v.insert("a".into(), fe(2));
            v.insert("b".into(), fe(3));
        }
        "edwards" => {
            v.insert("c".into(), fe(1));
            if rep.id.ends_with("/yz") || rep.id.ends_with("/yzsquared") {
                v.insert("r".into(), fe(6));
                v.insert("d".into(), fe(36));
            } else {
                v.insert("d".into(), fe(3));
            }
        }
        "twisted" => {
            v.insert(
                "a".into(),
                if rep.id.ends_with("-1") {
                    fe_i(-1)
                } else {
                    fe(1)
                },
            );
            v.insert("d".into(), fe(3));
        }
        "twistedhessian" => {
            v.insert("a".into(), fe(1));
            v.insert("d".into(), fe(3));
        }
        "hessian" => {
            // xmadd charts require a point with X=1; solving
            // y^3 - 3 d y + 2 = 0 at x=1 gives d = (y^3+2)/(3y).
            let y = fe(3);
            let num = y.mul(&y).mul(&y).add(&fe(2));
            let den = fe(3).mul(&y);
            v.insert("d".into(), num.mul(&den.inv().unwrap()));
        }
        "jquartic" => {
            if rep.id.contains("/2") {
                v.extend(jquartic_params());
            } else {
                v.insert("a".into(), fe(2));
            }
        }
        "jintersect" => {
            v.insert("a".into(), fe(2));
        }
        "2dik" | "3dik" => {
            v.insert("a".into(), fe(2));
        }
        other => unreachable!("unknown model {other}"),
    }
    v
}

/// Formula-level cache parameters that cannot be derived from a definition.
fn special_parameters(formula: &Formula, values: &mut Bindings) {
    if formula.assumptions.iter().any(|s| s.contains('w')) {
        if let Some(w) = cube_root_of_unity() {
            values.entry("w".into()).or_insert(w);
        }
    }
}

fn encode(
    rep: &Representation,
    params: &Bindings,
    point: &Point2,
    z: &FieldElement,
) -> Result<Bindings> {
    if rep.id.starts_with("jintersect/") {
        rep.encode_jintersection(&modulus(), params, &point[0], &point[1], &point[2], z)
    } else {
        rep.encode_affine(&modulus(), params, &point[0], &point[1], z)
    }
}

fn projection(rep: &Representation, point: &Point2) -> Projection {
    if rep.id.starts_with("jintersect/") {
        return Projection::Triple {
            s: point[0].clone(),
            c: point[1].clone(),
            d: point[2].clone(),
        };
    }
    if rep.id == "edwards/yz" {
        return Projection::Y(point[1].clone());
    }
    if rep.id == "edwards/yzsquared" {
        return Projection::Y2(point[1].mul(&point[1]));
    }
    if rep.id == "shortw/xz" || rep.id == "montgom/xz" {
        return Projection::X(point[0].clone());
    }
    Projection::Full {
        x: point[0].clone(),
        y: point[1].clone(),
    }
}

fn source_digest(s: &Source) {
    let h = crate::hash::sha256::sha256(s.text.as_bytes());
    let hex: String = h.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(s.sha256, hex, "{}", s.url);
}

#[test]
fn complete_pinned_catalogue_and_source_hashes() {
    let c = catalogue();
    assert_eq!(c.schema, "efd-prime-catalogue/v1");
    assert_eq!(c.characteristic, "p");
    assert_eq!(c.formulas.len(), 471);
    assert_eq!(c.representations.len(), 33);
    assert_eq!(c.models.len(), 10);
    let mut ids = BTreeSet::new();
    for model in &c.models {
        source_digest(&model.source);
    }
    for rep in &c.representations {
        source_digest(&rep.source);
    }
    let mut incomplete = 0;
    for f in &c.formulas {
        assert!(ids.insert(&f.id));
        source_digest(&f.record);
        source_digest(&f.program);
        match f.compile() {
            Ok(_) => {}
            Err(EfdError::Unsupported(message)) if f.program.text.contains("error:") => {
                incomplete += 1;
                assert!(message.contains("upstream program incomplete"), "{message}");
            }
            Err(e) => panic!("{}: {e}", f.id),
        }
    }
    // Six `add-1986-cc*` programs were never emitted by the EFD generator.
    assert_eq!(incomplete, 6);
}

#[test]
fn all_33_formats_roundtrip_scaled_finite_points() {
    let params = |rep: &Representation| params_for(rep);
    for rep in &catalogue().representations {
        let params = params(rep);
        let point: Point2 = if rep.id.starts_with("jintersect/") {
            vec![fe(3), fe(5), fe(7)]
        } else {
            vec![fe(3), fe(5)]
        };
        for z in [1u64, 2, 7] {
            let encoded = encode(rep, &params, &point, &fe(z)).unwrap();
            assert_eq!(
                rep.project(&params, &rep.bind_point(&encoded, 3), 3)
                    .unwrap(),
                projection(rep, &point),
                "{} z={z}",
                rep.id
            );
        }
    }
}

/// Compare every optimized straight-line program with the independent,
/// unoptimized affine group law in the model record. Exercise both unit and
/// nonunit projective scales. Singular charts and vanishing denominators are
/// skipped; every formula must have at least 8 successful comparisons.
#[test]
fn all_471_formulas_match_affine_model_laws() {
    let mut coverage = Vec::new();
    for rep in &catalogue().representations {
        let model = catalogue().representation_model(&rep.id).unwrap();
        let params = params_for(rep);
        let points = model_points(model, &params);
        assert!(points.len() > 8, "{}: {} points", rep.id, points.len());
        for formula in catalogue()
            .formulas
            .iter()
            .filter(|f| f.representation == rep.id)
        {
            if let Some((_, reason)) = KNOWN_PROGRAM_DEFECTS
                .iter()
                .find(|(id, _)| *id == formula.id)
            {
                assert!(
                    formula.program.text.contains("-t7"),
                    "{}: expected defect evidence",
                    formula.id
                );
                coverage.push(serde_json::json!({"formula":formula.id,"prime":P,
                    "executable":false,"defect":reason}));
                continue;
            }
            let program = match formula.compile() {
                Ok(program) => program,
                Err(EfdError::Unsupported(_)) => {
                    assert!(formula.program.text.contains("error:"));
                    coverage.push(serde_json::json!({"formula":formula.id,
                        "prime":P,"executable":false}));
                    continue;
                }
                Err(e) => panic!("{}: {e}", formula.id),
            };
            let mut passed = 0;
            let mut attempted = 0;
            let limit = (points.len() * points.len() * 2).min(600);
            let unit = |name: &str| {
                formula
                    .assumptions
                    .iter()
                    .any(|s| s.replace(' ', "") == format!("{name}=1"))
            };
            let has_unit_x2 = unit("X2") || unit("S2");
            let has_unit_x1 = unit("X1") || unit("S1");
            let inverted = rep.id.ends_with("/inverted");
            for k in 0..limit {
                attempted += 1;
                let mut p_index = k % points.len();
                let mut q_index = (k / points.len() + k + 3) % points.len();
                if !inverted {
                    if has_unit_x2 {
                        if let Some(i) = points.iter().position(|p| p[0] == one()) {
                            q_index = i;
                        }
                    }
                    if has_unit_x1 {
                        if let Some(i) = points.iter().position(|p| p[0] == one()) {
                            p_index = i;
                        }
                    }
                }
                let p = &points[p_index];
                let q = &points[q_index];
                let attempt = (|| -> Result<()> {
                    let mut inputs = params.clone();
                    special_parameters(formula, &mut inputs);
                    let mut encoded_points: Vec<(u32, Point2)> = vec![(1, p.clone())];
                    let expected: Vec<(u32, Point2)> = match formula.operation.as_str() {
                        "addition" => {
                            encoded_points.push((2, q.clone()));
                            vec![(3, oracle(model, "addition", &params, &[p, q])?)]
                        }
                        "doubling" => vec![(3, oracle(model, "doubling", &params, &[p])?)],
                        "tripling" => {
                            let doubled = oracle(model, "doubling", &params, &[p])?;
                            vec![(3, oracle(model, "addition", &params, &[p, &doubled])?)]
                        }
                        "scaling" => vec![(3, p.clone())],
                        "diffadd" | "ladder" => {
                            let negated = oracle(model, "negation", &params, &[p])?;
                            let difference = oracle(model, "addition", &params, &[q, &negated])?;
                            encoded_points =
                                vec![(1, difference.clone()), (2, p.clone()), (3, q.clone())];
                            let sum = oracle(model, "addition", &params, &[p, q])?;
                            if formula.operation == "diffadd" {
                                vec![(5, sum)]
                            } else {
                                vec![(4, oracle(model, "doubling", &params, &[p])?), (5, sum)]
                            }
                        }
                        other => panic!("uncovered operation {other}"),
                    };
                    for (index, point) in &encoded_points {
                        let index = *index;
                        let z_forced = formula
                            .assumptions
                            .iter()
                            .any(|s| s.replace(' ', "") == format!("Z{index}=1"));
                        let x_forced = (index == 1 && has_unit_x1) || (index == 2 && has_unit_x2);
                        let scale = if x_forced && inverted {
                            point[0].clone()
                        } else if x_forced || z_forced || k % 2 == 0 {
                            one()
                        } else {
                            fe(u64::from(index) + 2)
                        };
                        let encoded = encode(rep, &params, point, &scale)?;
                        inputs.extend(rep.bind_point(&encoded, index));
                    }
                    let output = program.evaluate(&modulus(), &inputs)?;
                    for (index, point) in expected {
                        assert_eq!(
                            rep.project(&params, &output, index)?,
                            projection(rep, &point),
                            "{} k={k}",
                            formula.id
                        );
                    }
                    Ok(())
                })();
                match attempt {
                    Ok(()) => passed += 1,
                    Err(
                        EfdError::ZeroDenominator
                        | EfdError::OutsideChart
                        | EfdError::Assumption(_),
                    ) => {}
                    Err(e) => panic!("{} k={k}: {e}", formula.id),
                }
                if passed == 32 {
                    break;
                }
            }
            assert!(
                passed >= 8,
                "{}: only {passed} comparable cases (attempted {attempted})",
                formula.id
            );
            coverage.push(serde_json::json!({"formula":formula.id,"prime":P,
                "passed":passed,"attempted":attempted,
                "excluded_singular_or_conditional":attempted-passed}));
        }
    }
    if let Some(path) = std::env::var_os("EFD_COVERAGE_OUT") {
        std::fs::write(path, serde_json::to_vec_pretty(&coverage).unwrap()).unwrap();
    }
}

fn small_curve(a: u64) -> CurveParams {
    CurveParams {
        name: "efd-test",
        p: modulus(),
        a: BigUint::from(a),
        b: BigUint::from(3u32),
        gx: BigUint::from(0u32),
        gy: BigUint::from(0u32),
        n: BigUint::from(0u32),
        h: 1,
    }
}

fn shortw_points(curve: &CurveParams) -> Vec<Point> {
    let model = catalogue().model("shortw").unwrap();
    let params = Bindings::from([
        ("a".into(), FieldElement::new(curve.a.clone(), modulus())),
        ("b".into(), FieldElement::new(curve.b.clone(), modulus())),
    ]);
    model_points(model, &params)
        .into_iter()
        .map(|p| Point::Affine {
            x: p[0].clone(),
            y: p[1].clone(),
        })
        .collect()
}

fn applies(formula: &Formula, curve: &CurveParams) -> bool {
    let rep = &formula.representation;
    if !rep.starts_with("shortw/")
        || rep == "shortw/xz"
        || !matches!(formula.operation.as_str(), "addition" | "doubling")
    {
        return false;
    }
    let p = &curve.p;
    let a = FieldElement::new(curve.a.clone(), p.clone());
    if rep.ends_with("-0") && !a.is_zero() {
        return false;
    }
    if rep.ends_with("-1") && a != FieldElement::new(p - BigUint::from(1u32), p.clone()) {
        return false;
    }
    if rep.ends_with("-3") && a != FieldElement::new(p - BigUint::from(3u32), p.clone()) {
        return false;
    }
    if rep.ends_with("w12-0") && !curve.b.is_zero() {
        return false;
    }
    true
}

#[test]
fn checked_weierstrass_formulas_match_existing_independent_arithmetic() {
    for a in [0u64, P - 3] {
        let curve = small_curve(a);
        let a_fe = curve.a_fe();
        let points = shortw_points(&curve);
        assert!(points.len() > 8);
        for formula in catalogue().formulas.iter().filter(|f| applies(f, &curve)) {
            if formula.program.text.contains("error:") {
                continue;
            }
            let mut actual_formula = 0;
            for i in 0..points.len() {
                let p = &points[i];
                let q = &points[(i + 5) % points.len()];
                let result = match weierstrass_operation(
                    &curve,
                    &formula.id,
                    p,
                    if formula.operation == "addition" {
                        Some(q)
                    } else {
                        None
                    },
                ) {
                    Ok(result) => result,
                    // xmadd charts require the fixed point X2=1.
                    Err(EfdError::Assumption(_)) => continue,
                    Err(e) => panic!("{}: {e}", formula.id),
                };
                let expected = if formula.operation == "addition" {
                    p.add(q, &a_fe)
                } else {
                    p.double(&a_fe)
                };
                assert_eq!(result.point, expected, "{} a={a}", formula.id);
                actual_formula += usize::from(result.path == ExecutionPath::Formula);
            }
            assert!(
                actual_formula >= 4,
                "{}: did not exercise formula",
                formula.id
            );
        }
    }
}

#[test]
fn large_prime_planted_points_match_existing_arithmetic() {
    // Existing public generators and their small multiples; no imported target.
    for curve in [CurveParams::secp256k1(), CurveParams::p256()] {
        let a = curve.a_fe();
        let p = curve.generator();
        let q = p.double(&a);
        let r = p.add(&q, &a);
        for formula in catalogue().formulas.iter().filter(|f| applies(f, &curve)) {
            if formula.program.text.contains("error:") {
                continue;
            }
            let addition = formula.operation == "addition";
            let result = match weierstrass_operation(
                &curve,
                &formula.id,
                &p,
                if addition { Some(&r) } else { None },
            ) {
                Ok(result) => result,
                Err(EfdError::Assumption(_)) => continue,
                Err(e) => panic!("{}: {e}", formula.id),
            };
            assert_eq!(
                result.path,
                ExecutionPath::Formula,
                "{} p={}",
                formula.id,
                curve.p
            );
            assert_eq!(
                result.point,
                if addition {
                    p.add(&r, &a)
                } else {
                    p.double(&a)
                },
                "{} p={}",
                formula.id,
                curve.p
            );
        }
    }
}

/// Every published shape and chart must be present, encodable from model
/// parameters alone, and checked against its own `satisfying` equations.
#[test]
fn all_ten_curve_equations_and_thirty_three_charts_are_covered() {
    let c = catalogue();
    let models: BTreeSet<&str> = c.models.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(
        models,
        [
            "2dik",
            "3dik",
            "edwards",
            "hessian",
            "jintersect",
            "jquartic",
            "montgom",
            "shortw",
            "twisted",
            "twistedhessian",
        ]
        .into_iter()
        .collect()
    );
    let mut seen = BTreeSet::new();
    for rep in &c.representations {
        assert!(seen.insert(rep.id.clone()), "duplicate {}", rep.id);
        let model = c
            .representation_model(&rep.id)
            .unwrap_or_else(|| panic!("{} has no model", rep.id));
        assert!(!rep.variables.is_empty(), "{}", rep.id);
        // Supply model parameters only: `r` and `c` are chart parameters that
        // the representation fixes by `d = r^2` and `a^2+c^2 = 1`.
        let mut params = params_for(rep);
        params.remove("r");
        params.remove("c");
        let point = if rep.id.starts_with("jintersect/") {
            vec![fe(3), fe(5), fe(7)]
        } else {
            vec![fe(3), fe(5)]
        };
        for z in [1u64, 5] {
            let encoded = encode(rep, &params, &point, &fe(z)).unwrap();
            assert_eq!(
                rep.project(&params, &rep.bind_point(&encoded, 3), 3)
                    .unwrap(),
                projection(rep, &point),
                "{} z={z}",
                rep.id
            );
        }
        assert_eq!(
            model.coordinates().len(),
            if rep.id.starts_with("jintersect/") {
                3
            } else {
                2
            }
        );
    }
    // Each curve equation is executable and holds on its own enumerated points.
    for model in &c.models {
        let rep = c
            .representations
            .iter()
            .find(|r| r.id.split('/').next() == Some(model.id.as_str()))
            .unwrap();
        let params = params_for(rep);
        assert!(
            !relations(model).is_empty(),
            "{}: no satisfying equations",
            model.id
        );
        let points = model_points(model, &params);
        assert!(points.len() > 8, "{}: {} points", model.id, points.len());
        // The equations must actually constrain the chart.
        assert!(
            points.len() < P as usize * P as usize,
            "{}: unconstrained",
            model.id
        );
        for point in &points {
            assert!(
                model.on_model(&modulus(), &params, point).unwrap(),
                "{}",
                model.id
            );
        }
        assert!(model.on_model(&modulus(), &params, &points[0]).unwrap());
        assert_eq!(model.coordinates().len(), points[0].len());
    }
    // Unknown charts fail closed; no projection falls through a default.
    let bogus = Representation {
        id: "bogus/chart".into(),
        page_url: String::new(),
        variables: vec!["X".into(), "Y".into(), "Z".into()],
        relations: vec![],
        assumptions: vec![],
        source: Source {
            url: String::new(),
            sha256: String::new(),
            text: String::new(),
        },
    };
    assert!(matches!(
        bogus.encode_affine(&modulus(), &Bindings::new(), &fe(3), &fe(5), &fe(1)),
        Err(EfdError::Unsupported(_))
    ));
    let values = Bindings::from([
        ("X3".into(), fe(1)),
        ("Y3".into(), fe(1)),
        ("Z3".into(), fe(1)),
    ]);
    assert!(matches!(
        bogus.project(&Bindings::new(), &values, 3),
        Err(EfdError::Unsupported(_))
    ));
}

/// Every model exposes both directions of its Weierstrass bridge, so model
/// conversion is possible (through the hub) for all ten shapes.
#[test]
fn every_model_has_both_weierstrass_maps() {
    let c = catalogue();
    assert_eq!(c.models.len(), 10);
    let mut identity = BTreeSet::new();
    for model in &c.models {
        assert!(
            !model.to_weierstrass().is_empty(),
            "{}: no toweierstrass",
            model.id
        );
        assert!(
            !model.from_weierstrass().is_empty(),
            "{}: no fromweierstrass",
            model.id
        );
        if model.is_identity_weierstrass() {
            identity.insert(model.id.as_str());
        }
    }
    // These four shapes are already Weierstrass models with the same
    // coordinates; the other six need a rational map.
    assert_eq!(
        identity,
        ["2dik", "3dik", "montgom", "shortw"].into_iter().collect()
    );
}

#[test]
fn rejects_assumption_field_and_chart_mismatches() {
    let c = catalogue();
    let rep = c.representation("shortw/jacobian").unwrap();
    let params = params_for(rep);
    let mut inputs = params.clone();
    inputs.extend(
        rep.bind_point(
            &rep.encode_affine(&modulus(), &params, &fe(1), &fe(2), &fe(2))
                .unwrap(),
            1,
        ),
    );
    inputs.extend(
        rep.bind_point(
            &rep.encode_affine(&modulus(), &params, &fe(3), &fe(4), &fe(2))
                .unwrap(),
            2,
        ),
    );
    let mixed = c.formula("shortw/jacobian/addition/madd-2007-bl").unwrap();
    assert!(matches!(
        mixed.evaluate(&modulus(), &inputs),
        Err(EfdError::Assumption(_))
    ));
    inputs.insert("Z2".into(), one());
    let derived = mixed.evaluate(&modulus(), &inputs).unwrap();
    assert!(derived.contains_key("X3"));

    let other = BigUint::from(107u32);
    assert_eq!(
        rep.encode_affine(&other, &params, &fe(1), &fe(2), &fe(1)),
        Err(EfdError::FieldMismatch)
    );
    assert_eq!(
        rep.encode_affine(&modulus(), &params, &fe(1), &fe(2), &zero()),
        Err(EfdError::OutsideChart)
    );
    assert!(Expr::parse("a; panic!()").is_err());
    assert!(Expr::parse("a ^ 100000000").is_err());
    assert!(matches!(
        Expr::parse("2d").unwrap(),
        Expr::Variable(ref v) if v == "2d"
    ));
    assert_eq!(
        Expr::parse("1/0")
            .unwrap()
            .eval(&modulus(), &Bindings::new()),
        Err(EfdError::ZeroDenominator)
    );
}

#[test]
fn preserves_declared_operation_counts() {
    let f = catalogue()
        .formula("shortw/jacobian/doubling/dbl-2007-bl")
        .unwrap();
    assert_eq!(
        f.compile().unwrap().operation_counts(),
        OperationCounts {
            multiplications: 6,
            squarings: 8,
            inversions: 0,
            additions: 10
        }
    );
    let f = catalogue()
        .formula("montgom/xz/doubling/dbl-1987-m-3")
        .unwrap();
    assert_eq!(
        f.compile().unwrap().operation_counts(),
        OperationCounts {
            multiplications: 3,
            squarings: 2,
            inversions: 0,
            additions: 4
        }
    );
}
