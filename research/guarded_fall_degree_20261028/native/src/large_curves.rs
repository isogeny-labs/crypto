//! Deterministic public SEC 2 fixtures for the guarded three-abscissa theorem.
//!
//! The only targets emitted here are the public point `16 G`, with factor-base
//! lifts `G, 2 G, 4 G`. Checking the first 28 nonzero multiples proves that no
//! three signed lifts can sum to either sign of the target. This is a fixture
//! provenance check; the independent sparse DAG verifier checks a different
//! layer, the polynomial ideal identity. Field primality is standards-assumed.

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt};

const PARAMETERS: &str = include_str!("../../extensions/large_curves/standard_parameters.json");
const PARAMETER_SHA256: &str = "35f833e4324c39d1295d94688485c07cd3e516951eccf0e92cb24baf4b868885";
const SEC2_SHA256: &str = "87b8f3703364ed5b21ba8582e411cc0cbf477bcaa3f4f45e0d6580d1c00d9952";
const SEC2_URL: &str = "https://www.secg.org/sec2-v2.pdf";
const NAMES: [&str; 4] = ["secp256k1", "secp256r1", "secp384r1", "secp521r1"];
type Poly = BTreeMap<[u32; 5], BigUint>;
type Result<T> = std::result::Result<T, FixtureError>;

#[derive(Debug, Clone)]
pub struct FixtureError(String);
impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for FixtureError {}
fn fail(message: impl Into<String>) -> FixtureError {
    FixtureError(message.into())
}
fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(fail(message))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: BigUint,
    y: BigUint,
}
#[derive(Debug)]
struct Curve {
    p: BigUint,
    a: BigUint,
    b: BigUint,
    g: Point,
    record: ParameterRecord,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParameterRecord {
    standard_name: String,
    icv1_slug: String,
    ec1: String,
    curve_uid: String,
    p: String,
    a: String,
    b: String,
    generator: [String; 2],
    source_declared_order: String,
    cofactor: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ParameterDocument {
    schema: String,
    source: Value,
    curves: Vec<ParameterRecord>,
}

fn decimal(text: &str) -> Result<BigUint> {
    require(
        !text.is_empty()
            && text.bytes().all(|b| b.is_ascii_digit())
            && (text == "0" || !text.starts_with('0')),
        "expected a canonical nonnegative decimal integer",
    )?;
    BigUint::parse_bytes(text.as_bytes(), 10).ok_or_else(|| fail("invalid decimal integer"))
}
fn hex(text: &str) -> Result<BigUint> {
    let digits = text.strip_prefix("0x").unwrap_or(text);
    BigUint::parse_bytes(digits.as_bytes(), 16).ok_or_else(|| fail("invalid hexadecimal parameter"))
}
fn raw_integer(value: &RawValue) -> Result<BigUint> {
    decimal(value.get())
}
fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a + p - b) % p
}
fn inverse(a: &BigUint, p: &BigUint) -> Result<BigUint> {
    let p_int = BigInt::from_biguint(Sign::Plus, p.clone());
    let egcd = BigInt::from_biguint(Sign::Plus, a.clone()).extended_gcd(&p_int);
    require(
        egcd.gcd == BigInt::one(),
        "noninvertible point-addition denominator",
    )?;
    egcd.x
        .mod_floor(&p_int)
        .to_biguint()
        .ok_or_else(|| fail("negative modular inverse"))
}

impl Curve {
    fn load(name: &str) -> Result<Self> {
        require(
            format!("{:x}", Sha256::digest(PARAMETERS.as_bytes())) == PARAMETER_SHA256,
            "embedded standard-parameter snapshot hash mismatch",
        )?;
        let document: ParameterDocument =
            serde_json::from_str(PARAMETERS).map_err(|e| fail(e.to_string()))?;
        require(
            document.schema == "standard-large-curve-parameters/v1",
            "wrong parameter schema",
        )?;
        require(
            document.source["url"] == SEC2_URL && document.source["sha256"] == SEC2_SHA256,
            "wrong primary-source provenance",
        )?;
        let record = document
            .curves
            .into_iter()
            .find(|r| r.standard_name == name)
            .ok_or_else(|| fail("unsupported standard curve"))?;
        let p = decimal(&record.p)?;
        let a = decimal(&record.a)?;
        let b = decimal(&record.b)?;
        let one = BigUint::one();
        let expected_p = match name {
            "secp256k1" => (&one << 256usize) - (&one << 32usize) - BigUint::from(977u32),
            "secp256r1" => {
                (&one << 256usize) - (&one << 224usize) + (&one << 192usize) + (&one << 96usize)
                    - &one
            }
            "secp384r1" => {
                (&one << 384usize) - (&one << 128usize) - (&one << 96usize) + (&one << 32usize)
                    - &one
            }
            "secp521r1" => (&one << 521usize) - &one,
            _ => return Err(fail("unsupported standard curve")),
        };
        require(p == expected_p, "field modulus differs from SEC 2 formula")?;
        require(
            a == if name == "secp256k1" {
                BigUint::zero()
            } else {
                &p - 3u32
            },
            "curve coefficient a differs from SEC 2",
        )?;
        let expected_b = match name {
            "secp256k1" => "7",
            "secp256r1" => "5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B",
            "secp384r1" => "B3312FA7E23EE7E4988E056BE3F82D19181D9C6EFE8141120314088F5013875AC656398D8A2ED19D2A85C8EDD3EC2AEF",
            "secp521r1" => "0051953EB9618E1C9A1F929A21A0B68540EEA2DA725B99B315F3B8B489918EF109E156193951EC7E937B1652C0BD3BB1BF073573DF883D2C34F1EF451FD46B503F00",
            _ => unreachable!(),
        };
        require(
            b == hex(expected_b)?,
            "curve coefficient b differs from SEC 2",
        )?;
        require(
            record.cofactor == 1 && decimal(&record.source_declared_order)? > BigUint::from(28u32),
            "invalid source-declared group metadata",
        )?;
        let g = Point {
            x: hex(&record.generator[0])?,
            y: hex(&record.generator[1])?,
        };
        let curve = Self { p, a, b, g, record };
        require(
            curve.a < curve.p && curve.b < curve.p,
            "noncanonical curve coefficients",
        )?;
        require(
            (BigUint::from(4u32) * curve.a.modpow(&BigUint::from(3u32), &curve.p)
                + BigUint::from(27u32) * &curve.b * &curve.b)
                % &curve.p
                != BigUint::zero(),
            "singular curve",
        )?;
        require(
            curve.on_curve(&curve.g),
            "standard generator is not on the curve",
        )?;
        Ok(curve)
    }
    fn on_curve(&self, point: &Point) -> bool {
        point.x < self.p
            && point.y < self.p
            && (&point.y * &point.y) % &self.p
                == (&point.x * &point.x * &point.x + &self.a * &point.x + &self.b) % &self.p
    }
    fn add(&self, left: Option<&Point>, right: Option<&Point>) -> Result<Option<Point>> {
        let (left, right) = match (left, right) {
            (None, q) => return Ok(q.cloned()),
            (p, None) => return Ok(p.cloned()),
            (Some(p), Some(q)) => (p, q),
        };
        require(
            self.on_curve(left) && self.on_curve(right),
            "point-addition input is off curve",
        )?;
        if left.x == right.x && (&left.y + &right.y) % &self.p == BigUint::zero() {
            return Ok(None);
        }
        let slope = if left.x == right.x {
            require(left.y == right.y, "incompatible equal-abscissa points")?;
            ((BigUint::from(3u32) * &left.x * &left.x + &self.a)
                * inverse(&((BigUint::from(2u32) * &left.y) % &self.p), &self.p)?)
                % &self.p
        } else {
            (sub(&right.y, &left.y, &self.p) * inverse(&sub(&right.x, &left.x, &self.p), &self.p)?)
                % &self.p
        };
        let x = sub(
            &sub(&((&slope * &slope) % &self.p), &left.x, &self.p),
            &right.x,
            &self.p,
        );
        let y = sub(
            &((&slope * sub(&left.x, &x, &self.p)) % &self.p),
            &left.y,
            &self.p,
        );
        let point = Point { x, y };
        require(self.on_curve(&point), "point-addition output is off curve")?;
        Ok(Some(point))
    }
    fn multiples(&self) -> Result<Vec<Point>> {
        let mut previous = None;
        let mut multiples = Vec::new();
        for _ in 1..=28 {
            previous = self.add(previous.as_ref(), Some(&self.g))?;
            multiples.push(
                previous
                    .clone()
                    .ok_or_else(|| fail("a multiple jG with 1<=j<=28 is infinity"))?,
            );
        }
        Ok(multiples)
    }
}

#[derive(Clone)]
struct Decimal(BigUint);
impl Serialize for Decimal {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        RawValue::from_string(self.0.to_string())
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}
#[derive(Serialize)]
struct CurveCoefficients {
    a: Decimal,
    b: Decimal,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct FixtureMetadata {
    schema: String,
    standard_name: String,
    icv1_slug: String,
    ec1: String,
    curve_uid: String,
    source_url: String,
    source_pdf_sha256: String,
    parameter_snapshot_sha256: String,
    public_point_scalars: [u32; 3],
    public_target_scalar: u32,
}
#[derive(Serialize)]
struct ProblemDocument {
    p: Decimal,
    n: usize,
    polynomials: Vec<Vec<(Decimal, [u32; 5])>>,
    variables: [&'static str; 5],
    curve: CurveCoefficients,
    factor_base_abscissas: Vec<Decimal>,
    target: [Decimal; 2],
    restriction: &'static str,
    implicit_field_equations: bool,
    fixture: FixtureMetadata,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCurve {
    a: Box<RawValue>,
    b: Box<RawValue>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProblem {
    p: Box<RawValue>,
    n: usize,
    polynomials: Vec<Vec<(Box<RawValue>, Vec<u32>)>>,
    variables: Vec<String>,
    curve: RawCurve,
    factor_base_abscissas: Vec<Box<RawValue>>,
    target: Vec<Box<RawValue>>,
    restriction: String,
    implicit_field_equations: bool,
    fixture: FixtureMetadata,
}

fn constant(c: BigUint) -> Poly {
    if c.is_zero() {
        Poly::new()
    } else {
        BTreeMap::from([([0; 5], c)])
    }
}
fn variable(axis: usize) -> Poly {
    let mut e = [0; 5];
    e[axis] = 1;
    BTreeMap::from([(e, BigUint::one())])
}
fn add(left: &Poly, right: &Poly, p: &BigUint) -> Poly {
    let mut result = left.clone();
    for (e, c) in right {
        let value = (result.get(e).cloned().unwrap_or_default() + c) % p;
        if value.is_zero() {
            result.remove(e);
        } else {
            result.insert(*e, value);
        }
    }
    result
}
fn scale(poly: &Poly, coefficient: &BigUint, p: &BigUint) -> Poly {
    poly.iter()
        .filter_map(|(e, c)| {
            let value = c * coefficient % p;
            (!value.is_zero()).then_some((*e, value))
        })
        .collect()
}
fn minus(left: &Poly, right: &Poly, p: &BigUint) -> Poly {
    add(left, &scale(right, &(p - 1u32), p), p)
}
fn mul(left: &Poly, right: &Poly, p: &BigUint) -> Poly {
    let mut result = Poly::new();
    for (e, c) in left {
        for (f, d) in right {
            let powers = std::array::from_fn(|i| e[i] + f[i]);
            result = add(&result, &constant_term(powers, c * d % p), p);
        }
    }
    result
}
fn constant_term(e: [u32; 5], c: BigUint) -> Poly {
    if c.is_zero() {
        Poly::new()
    } else {
        BTreeMap::from([(e, c)])
    }
}
fn s3(curve: &Curve, x: &Poly, y: &Poly, z: &Poly) -> Poly {
    let p = &curve.p;
    let xy = mul(x, y, p);
    let sum = add(x, y, p);
    let difference = minus(x, y, p);
    let tail = minus(&xy, &constant(curve.a.clone()), p);
    let middle = add(
        &mul(&sum, &add(&xy, &constant(curve.a.clone()), p), p),
        &constant((BigUint::from(2u32) * &curve.b) % p),
        p,
    );
    let first = mul(&mul(&difference, &difference, p), &mul(z, z, p), p);
    let second = scale(&mul(&middle, z, p), &(p - 2u32), p);
    let fourth = scale(
        &sum,
        &sub(&BigUint::zero(), &((BigUint::from(4u32) * &curve.b) % p), p),
        p,
    );
    add(
        &add(&first, &second, p),
        &add(&mul(&tail, &tail, p), &fourth, p),
        p,
    )
}
fn inputs(curve: &Curve, base: &[BigUint], target: &Point) -> Vec<Poly> {
    let p = &curve.p;
    let mut result = Vec::new();
    for axis in 0..3 {
        let x = variable(axis);
        let mut g = constant(BigUint::one());
        for root in base {
            g = mul(&g, &minus(&x, &constant(root.clone()), p), p);
        }
        result.push(g);
    }
    result.push(s3(curve, &variable(0), &variable(1), &variable(3)));
    result.push(s3(
        curve,
        &variable(3),
        &variable(2),
        &constant(target.x.clone()),
    ));
    result.push(minus(
        &mul(&variable(4), &minus(&variable(0), &variable(1), p), p),
        &constant(BigUint::one()),
        p,
    ));
    result
}

pub fn curve_names() -> &'static [&'static str] {
    &NAMES
}

/// Build a fixed public known-answer input and its independently replayable
/// curve-provenance witness. All large JSON input integers stay numeric tokens;
/// the witness explicitly labels large coordinates as canonical decimal strings.
pub fn fixture_json(name: &str) -> Result<(String, Value)> {
    let curve = Curve::load(name)?;
    let multiples = curve.multiples()?;
    let base: Vec<_> = [0usize, 1, 3]
        .iter()
        .map(|i| multiples[*i].x.clone())
        .collect();
    require(
        base[0] != base[1] && base[0] != base[2] && base[1] != base[2],
        "factor-base abscissas collide",
    )?;
    let target = &multiples[15];
    require(
        !base.contains(&target.x),
        "target abscissa belongs to the factor base",
    )?;
    let metadata = FixtureMetadata {
        schema: "public-standard-curve-fixture/v1".into(),
        standard_name: name.into(),
        icv1_slug: curve.record.icv1_slug.clone(),
        ec1: curve.record.ec1.clone(),
        curve_uid: curve.record.curve_uid.clone(),
        source_url: SEC2_URL.into(),
        source_pdf_sha256: SEC2_SHA256.into(),
        parameter_snapshot_sha256: PARAMETER_SHA256.into(),
        public_point_scalars: [1, 2, 4],
        public_target_scalar: 16,
    };
    let polynomials = inputs(&curve, &base, target)
        .into_iter()
        .map(|p| p.into_iter().rev().map(|(e, c)| (Decimal(c), e)).collect())
        .collect();
    let document = ProblemDocument {
        p: Decimal(curve.p.clone()),
        n: 5,
        polynomials,
        variables: ["x1", "x2", "x3", "u", "v"],
        curve: CurveCoefficients {
            a: Decimal(curve.a.clone()),
            b: Decimal(curve.b.clone()),
        },
        factor_base_abscissas: base.into_iter().map(Decimal).collect(),
        target: [Decimal(target.x.clone()), Decimal(target.y.clone())],
        restriction: "x1 != x2",
        implicit_field_equations: false,
        fixture: metadata,
    };
    let problem = serde_json::to_string_pretty(&document).map_err(|e| fail(e.to_string()))? + "\n";
    let parsed = crate::bigprime::Problem::from_json(&problem).map_err(|e| fail(e.to_string()))?;
    let witness: Vec<_> = multiples
        .iter()
        .enumerate()
        .map(|(i, point)| {
            json!({
                "multiple": i+1, "x_decimal": point.x.to_string(), "y_decimal": point.y.to_string()
            })
        })
        .collect();
    let receipt = json!({
        "schema": "public-standard-curve-fixture-witness/v1", "standard_name": name,
        "icv1_slug": curve.record.icv1_slug, "ec1": curve.record.ec1, "curve_uid": curve.record.curve_uid,
        "problem_sha256": parsed.digest(), "field_bits": curve.p.bits(), "p_decimal": curve.p.to_string(),
        "source_url": SEC2_URL, "source_pdf_sha256": SEC2_SHA256,
        "parameter_snapshot_sha256": PARAMETER_SHA256,
        "coordinate_encoding": "canonical decimal strings",
        "multiples_1_through_28": witness,
        "checks": {"nonsingular":true,"generator_on_curve":true,"all_28_points_on_curve":true,
            "successive_addition_recurrence":true,"all_28_multiples_nonzero":true,
            "three_base_abscissas_distinct":true,"target_outside_base":true},
        "public_point_scalars": [1,2,4], "public_target_scalar":16,
        "unsat_argument": "Each signed three-summand coefficient s has abs(s)<=12; equality to either sign of16G forces a nonzero kG=O with4<=abs(k)<=28, excluded by the checked public multiples.",
        "unsatisfiable": true,
        "primality_status": "SEC 2 standard field primality is assumed; this fixture checker is not a primality prover.",
        "group_order_status": "Full group order and its primality are not independently proved or needed: checked multiples1..28 suffice.",
        "scope": "Public deterministic ordinary coefficient-field known-answer fixture; not a Boolean subspace fixture, DLP recovery, sweep, or performance measurement."
    });
    Ok((problem, receipt))
}

/// Reconstruct standard parameters, every small multiple and all six inputs
/// from serialized documents, rather than accepting receipt flags as evidence.
pub fn verify_fixture_json(problem: &str, witness: &str) -> Result<Value> {
    let actual: RawProblem = serde_json::from_str(problem).map_err(|e| fail(e.to_string()))?;
    let (expected_json, expected_witness) = fixture_json(&actual.fixture.standard_name)?;
    let expected: RawProblem =
        serde_json::from_str(&expected_json).map_err(|e| fail(e.to_string()))?;
    let actual_problem =
        crate::bigprime::Problem::from_json(problem).map_err(|e| fail(e.to_string()))?;
    let expected_problem =
        crate::bigprime::Problem::from_json(&expected_json).map_err(|e| fail(e.to_string()))?;
    require(
        actual_problem.digest() == expected_problem.digest(),
        "six-input polynomial binding mismatch",
    )?;
    require(
        actual.n == expected.n
            && actual.polynomials.len() == 6
            && actual.variables == expected.variables
            && actual.restriction == expected.restriction
            && !actual.implicit_field_equations
            && actual.fixture == expected.fixture,
        "fixture metadata mismatch",
    )?;
    require(
        raw_integer(&actual.p)? == raw_integer(&expected.p)?
            && raw_integer(&actual.curve.a)? == raw_integer(&expected.curve.a)?
            && raw_integer(&actual.curve.b)? == raw_integer(&expected.curve.b)?,
        "curve metadata mismatch",
    )?;
    let actual_base: Vec<_> = actual
        .factor_base_abscissas
        .iter()
        .map(|x| raw_integer(x))
        .collect::<Result<_>>()?;
    let expected_base: Vec<_> = expected
        .factor_base_abscissas
        .iter()
        .map(|x| raw_integer(x))
        .collect::<Result<_>>()?;
    let actual_target: Vec<_> = actual
        .target
        .iter()
        .map(|x| raw_integer(x))
        .collect::<Result<_>>()?;
    let expected_target: Vec<_> = expected
        .target
        .iter()
        .map(|x| raw_integer(x))
        .collect::<Result<_>>()?;
    require(
        actual_base == expected_base && actual_target == expected_target,
        "factor-base/target metadata mismatch",
    )?;
    let actual_witness: Value = serde_json::from_str(witness).map_err(|e| fail(e.to_string()))?;
    require(
        actual_witness == expected_witness,
        "serialized public point witness mismatch",
    )?;
    Ok(
        json!({"schema":"public-standard-curve-fixture-check/v1", "valid_fixture":true,
        "standard_name":actual.fixture.standard_name,"problem_sha256":actual_problem.digest(),
        "field_bits":actual_problem.modulus.bits(),"small_multiples_checked":28,
        "unsatisfiable_by_public_scalar_bound":true,
        "scope":"Curve/input provenance only; sparse DAG replay and primality assumptions are separate."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn evaluate(poly: &Poly, values: &[BigUint; 5], p: &BigUint) -> BigUint {
        poly.iter().fold(BigUint::zero(), |sum, (e, c)| {
            (sum + e
                .iter()
                .zip(values)
                .fold(c.clone(), |term, (power, value)| {
                    term * value.modpow(&BigUint::from(*power), p) % p
                }))
                % p
        })
    }
    #[test]
    fn standard_large_curve_fixtures_reconstruct_and_check() {
        for name in curve_names() {
            let (problem, witness) = fixture_json(name).unwrap();
            let receipt = verify_fixture_json(&problem, &witness.to_string()).unwrap();
            assert_eq!(receipt["valid_fixture"], true);
            assert!(receipt["field_bits"].as_u64().unwrap() >= 256);
        }
    }
    #[test]
    fn point_recurrences_and_summation_polynomials_agree() {
        for name in curve_names() {
            let curve = Curve::load(name).unwrap();
            let points = curve.multiples().unwrap();
            assert_eq!(
                curve.add(Some(&points[1]), Some(&points[1])).unwrap(),
                Some(points[3].clone())
            );
            assert_eq!(
                curve.add(Some(&points[7]), Some(&points[7])).unwrap(),
                Some(points[15].clone())
            );
            let zero = BigUint::zero();
            let first_values = [
                points[0].x.clone(),
                points[1].x.clone(),
                zero.clone(),
                points[2].x.clone(),
                zero.clone(),
            ];
            let first = s3(&curve, &variable(0), &variable(1), &variable(3));
            assert!(evaluate(&first, &first_values, &curve.p).is_zero());
            let second_values = [
                zero.clone(),
                zero.clone(),
                points[3].x.clone(),
                points[11].x.clone(),
                zero,
            ];
            let second = s3(
                &curve,
                &variable(3),
                &variable(2),
                &constant(points[15].x.clone()),
            );
            assert!(evaluate(&second, &second_values, &curve.p).is_zero());
        }
    }
    #[test]
    fn serialized_geometry_and_witness_tampering_fail_closed() {
        let (problem, mut witness) = fixture_json("secp256k1").unwrap();
        witness["multiples_1_through_28"][27]["x_decimal"] = json!("0");
        assert!(verify_fixture_json(&problem, &witness.to_string()).is_err());
        let (_, witness) = fixture_json("secp256k1").unwrap();
        let changed = problem.replace("\"restriction\": \"x1 != x2\"", "\"restriction\": \"none\"");
        assert!(verify_fixture_json(&changed, &witness.to_string()).is_err());
        let changed = problem.replace("\"a\": 0,", "\"a\": 1,");
        assert!(verify_fixture_json(&changed, &witness.to_string()).is_err());
        let changed = problem.replacen("\n        1,", "\n        2,", 1);
        assert_ne!(changed, problem);
        assert!(verify_fixture_json(&changed, &witness.to_string()).is_err());
        assert!(fixture_json("unknown-curve").is_err());
    }
}
