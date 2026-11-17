//! Independent native replay of public positive PDP and scalar certificates.
//!
//! No producer arithmetic, private witness, solver, search, or sampling is used.
//! A replay proves a finite group equation and the declared domain membership;
//! it does not prove the declared group orders or how a solver found its answer.
//!
//! References read for this implementation:
//! - SEC 1 v2, sections 2.2.1--2.2.2: https://www.secg.org/sec1-v2.pdf
//! - Sorenson--Webster, Theorem 1.1: https://arxiv.org/pdf/1509.00864
//! - Rabin's exact irreducibility criterion, Theorem 1:
//!   https://fredrikj.net/fdoc/html/algorithms/polynomials/irreducibility.html

use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Value};
use std::collections::BTreeSet;

const SCALAR: &str = "restricted-independent-scalar-label";
const PDP: &str = "independently-verified-pdp";
const PRIME_BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
pub(crate) type Point = Option<(u64, u64)>;

pub(crate) fn decimal(value: &Value, name: &str) -> Result<u64> {
    let text = value
        .as_str()
        .with_context(|| format!("{name} must be a decimal string"))?;
    ensure!(
        !text.is_empty()
            && text.len() <= 20
            && text.bytes().all(|b| b.is_ascii_digit())
            && (text.len() == 1 || !text.starts_with('0')),
        "noncanonical {name}"
    );
    text.parse().with_context(|| format!("{name} exceeds u64"))
}

fn element(value: &Value, name: &str, q: u64) -> Result<u64> {
    let result = decimal(value, name)?;
    ensure!(result < q, "{name} outside field");
    Ok(result)
}

fn integer(value: &Value, name: &str, min: u64, max: u64) -> Result<u64> {
    let result = value
        .as_u64()
        .with_context(|| format!("{name} must be an unsigned integer"))?;
    ensure!((min..=max).contains(&result), "invalid {name}");
    Ok(result)
}

fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    ((u128::from(left) * u128::from(right)) % u128::from(modulus)) as u64
}

fn add_mod(left: u64, right: u64, modulus: u64) -> u64 {
    ((u128::from(left) + u128::from(right)) % u128::from(modulus)) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, base, modulus);
        }
        exponent >>= 1;
        base = mul_mod(base, base, modulus);
    }
    result
}

pub(crate) fn is_prime(n: u64) -> bool {
    // The first twelve prime bases are deterministic throughout u64: the least
    // composite passing all twelve is 318665857834031151167461 > 2^64
    // (Sorenson--Webster, Theorem 1.1). This is not a probable-prime-only check.
    if n < 2 {
        return false;
    }
    for prime in PRIME_BASES {
        if n.is_multiple_of(prime) {
            return n == prime;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'bases: for base in PRIME_BASES {
        let mut residue = pow_mod(base, d, n);
        if residue == 1 || residue == n - 1 {
            continue;
        }
        for _ in 1..s {
            residue = mul_mod(residue, residue, n);
            if residue == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn polynomial_remainder(mut value: u64, modulus: u64) -> u64 {
    debug_assert!(modulus != 0);
    while value != 0 && value.leading_zeros() <= modulus.leading_zeros() {
        value ^= modulus << (modulus.leading_zeros() - value.leading_zeros());
    }
    value
}

fn polynomial_gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, polynomial_remainder(left, right));
    }
    left
}

fn binary_multiply(mut left: u64, mut right: u64, modulus: u64, degree: u32) -> u64 {
    let mut result = 0;
    while right != 0 {
        if right & 1 != 0 {
            result ^= left;
        }
        right >>= 1;
        // Operands have degree < degree <= 61, so this shift cannot overflow.
        left <<= 1;
        if left & (1u64 << degree) != 0 {
            left ^= modulus;
        }
    }
    result
}

fn is_irreducible(polynomial: u64, degree: u32) -> bool {
    let (mut remaining, mut divisor) = (degree, 2);
    let mut checkpoints = BTreeSet::new();
    while divisor * divisor <= remaining {
        if remaining % divisor == 0 {
            checkpoints.insert(degree / divisor);
            while remaining % divisor == 0 {
                remaining /= divisor;
            }
        }
        divisor += 1;
    }
    if remaining > 1 {
        checkpoints.insert(degree / remaining);
    }
    let x = polynomial_remainder(2, polynomial);
    let mut power = x;
    for exponent in 1..=degree {
        power = binary_multiply(power, power, polynomial, degree);
        if checkpoints.contains(&exponent) && polynomial_gcd(polynomial, power ^ x) != 1 {
            return false;
        }
    }
    power == x
}

#[derive(Clone, Debug)]
pub(crate) enum Field {
    Prime { p: u64 },
    Binary { degree: u32, polynomial: u64 },
}

impl Field {
    pub(crate) fn prime(p: u64) -> Result<Self> {
        ensure!(p > 3 && is_prime(p), "invalid prime-field modulus");
        Ok(Self::Prime { p })
    }

    pub(crate) fn binary(degree: u32, terms: &[u32]) -> Result<Self> {
        ensure!((1..=61).contains(&degree), "invalid binary degree");
        ensure!(terms.len() <= degree as usize, "too many polynomial terms");
        let mut polynomial = 1u64 << degree;
        let mut seen = BTreeSet::new();
        for &term in terms {
            ensure!(
                term < degree && seen.insert(term),
                "invalid polynomial exponent"
            );
            polynomial |= 1u64 << term;
        }
        ensure!(
            is_irreducible(polynomial, degree),
            "reducible binary polynomial"
        );
        Ok(Self::Binary { degree, polynomial })
    }

    pub(crate) fn q(&self) -> u64 {
        match *self {
            Self::Prime { p } => p,
            Self::Binary { degree, .. } => 1u64 << degree,
        }
    }

    pub(crate) fn binary_model(&self) -> bool {
        matches!(self, Self::Binary { .. })
    }

    pub(crate) fn add(&self, left: u64, right: u64) -> u64 {
        match *self {
            Self::Prime { p } => add_mod(left, right, p),
            Self::Binary { .. } => left ^ right,
        }
    }

    pub(crate) fn neg(&self, value: u64) -> u64 {
        match *self {
            Self::Prime { p } if value != 0 => p - value,
            _ => value,
        }
    }

    pub(crate) fn sub(&self, left: u64, right: u64) -> u64 {
        self.add(left, self.neg(right))
    }

    pub(crate) fn mul(&self, left: u64, right: u64) -> u64 {
        debug_assert!(left < self.q() && right < self.q());
        match *self {
            Self::Prime { p } => mul_mod(left, right, p),
            Self::Binary { degree, polynomial } => binary_multiply(left, right, polynomial, degree),
        }
    }

    fn pow(&self, mut value: u64, mut exponent: u64) -> u64 {
        let mut result = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.mul(result, value);
            }
            exponent >>= 1;
            value = self.mul(value, value);
        }
        result
    }

    pub(crate) fn inverse(&self, value: u64) -> Result<u64> {
        ensure!(value != 0 && value < self.q(), "invalid inverse operand");
        // Field validity was established independently before using Fermat.
        Ok(self.pow(value, self.q() - 2))
    }

    fn trace(&self, mut value: u64) -> Result<u64> {
        let Self::Binary { degree, .. } = *self else {
            bail!("absolute binary trace requires a binary field");
        };
        ensure!(value < self.q(), "invalid trace operand");
        let mut result = 0;
        for _ in 0..degree {
            result ^= value;
            value = self.mul(value, value);
        }
        ensure!(result <= 1, "absolute trace is not binary");
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Curve {
    pub(crate) field: Field,
    pub(crate) a: u64,
    pub(crate) b: u64,
}

impl Curve {
    fn new(field: Field, a: u64, b: u64) -> Result<Self> {
        ensure!(
            a < field.q() && b < field.q(),
            "noncanonical curve coefficients"
        );
        if field.binary_model() {
            ensure!(b != 0, "singular binary curve");
        } else {
            // Reduce each term before combining it: a^3 and 27*b^2 need not
            // fit u128 when a,b are word-size, even though a single product does.
            let p = field.q();
            let a3 = field.mul(field.mul(a, a), a);
            let b2 = field.mul(b, b);
            ensure!(
                add_mod(mul_mod(4, a3, p), mul_mod(27, b2, p), p) != 0,
                "singular prime curve"
            );
        }
        Ok(Self { field, a, b })
    }

    fn on_curve(&self, point: Point) -> bool {
        let Some((x, y)) = point else { return true };
        let f = &self.field;
        if x >= f.q() || y >= f.q() {
            return false;
        }
        let (x2, y2) = (f.mul(x, x), f.mul(y, y));
        if f.binary_model() {
            (y2 ^ f.mul(x, y)) == (f.mul(x2, x) ^ f.mul(self.a, x2) ^ self.b)
        } else {
            y2 == f.add(f.add(f.mul(x2, x), f.mul(self.a, x)), self.b)
        }
    }

    pub(crate) fn point(&self, value: &Value, name: &str, finite: bool) -> Result<Point> {
        if value.is_null() {
            ensure!(!finite, "{name} must be finite");
            return Ok(None);
        }
        let fields = value
            .as_object()
            .with_context(|| format!("invalid {name}"))?;
        ensure!(
            fields.len() == 2 && fields.contains_key("x") && fields.contains_key("y"),
            "invalid {name} coordinate fields"
        );
        let point = Some((
            element(&value["x"], "point x", self.field.q())?,
            element(&value["y"], "point y", self.field.q())?,
        ));
        ensure!(self.on_curve(point), "{name} is off curve");
        Ok(point)
    }

    pub(crate) fn neg(&self, point: Point) -> Result<Point> {
        ensure!(self.on_curve(point), "cannot negate off-curve point");
        Ok(point.map(|(x, y)| {
            (
                x,
                if self.field.binary_model() {
                    x ^ y
                } else {
                    self.field.neg(y)
                },
            )
        }))
    }

    pub(crate) fn add(&self, left: Point, right: Point) -> Result<Point> {
        ensure!(
            self.on_curve(left) && self.on_curve(right),
            "cannot add off-curve points"
        );
        let (Some((x1, y1)), Some((x2, y2))) = (left, right) else {
            return Ok(left.or(right));
        };
        let f = &self.field;
        if f.binary_model() {
            if x1 == x2 {
                if y1 ^ y2 == x1 {
                    return Ok(None);
                }
                let slope = x1 ^ f.mul(y1, f.inverse(x1)?);
                let x3 = f.mul(slope, slope) ^ slope ^ self.a;
                return Ok(Some((x3, f.mul(x1, x1) ^ f.mul(slope ^ 1, x3))));
            }
            let slope = f.mul(y1 ^ y2, f.inverse(x1 ^ x2)?);
            let x3 = f.mul(slope, slope) ^ slope ^ x1 ^ x2 ^ self.a;
            return Ok(Some((x3, f.mul(slope, x1 ^ x3) ^ x3 ^ y1)));
        }
        let slope = if x1 == x2 {
            if f.add(y1, y2) == 0 {
                return Ok(None);
            }
            let numerator = f.add(mul_mod(3, f.mul(x1, x1), f.q()), self.a);
            f.mul(numerator, f.inverse(f.add(y1, y1))?)
        } else {
            f.mul(f.sub(y2, y1), f.inverse(f.sub(x2, x1))?)
        };
        let x3 = f.sub(f.sub(f.mul(slope, slope), x1), x2);
        Ok(Some((x3, f.sub(f.mul(slope, f.sub(x1, x3)), y1))))
    }

    pub(crate) fn mul(&self, mut point: Point, mut scalar: u64) -> Result<Point> {
        ensure!(self.on_curve(point), "cannot multiply off-curve point");
        let mut result = None;
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = self.add(result, point)?;
            }
            scalar >>= 1;
            point = self.add(point, point)?;
        }
        Ok(result)
    }
}

pub(crate) struct PublicCurve {
    pub(crate) curve: Curve,
    generator: Point,
    pub(crate) target: Point,
    declared_order: u64,
    arity: usize,
    twist_validated: bool,
}

pub(crate) fn public_curve(public: &Value, config: &Value) -> Result<PublicCurve> {
    ensure!(
        public.is_object() && config.is_object(),
        "public/config must be objects"
    );
    ensure!(
        public["schema"] == "ml-ecdlp-instance/v1",
        "unknown public schema"
    );
    let n = integer(&public["n"], "field size n", 3, 61)?;
    let arity = integer(&public["m"], "public arity", 2, 15)? as usize;
    let twist = public["twist"].as_bool().context("invalid twist flag")?;
    for key in ["family", "n", "m", "twist"] {
        ensure!(public[key] == config[key], "public/config {key} mismatch");
    }
    let family = public["family"].as_str().context("invalid family")?;
    let field = &public["field"];
    let model = &public["curve"];
    ensure!(
        field.is_object() && model.is_object(),
        "invalid field/curve objects"
    );
    let arithmetic = match family {
        "prime" => {
            ensure!(
                model["model"] == "short-weierstrass",
                "prime model mismatch"
            );
            let p = decimal(&field["modulus"], "modulus")?;
            ensure!(
                field["characteristic"].as_str() == Some(p.to_string().as_str())
                    && field["degree"].as_u64() == Some(1)
                    && field["basis"] == "canonical-residue"
                    && u64::from(64 - p.leading_zeros()) == n,
                "prime field metadata mismatch"
            );
            Field::prime(p)?
        }
        "binary" | "koblitz" => {
            ensure!(model["model"] == "binary-ordinary", "binary model mismatch");
            ensure!(
                field["characteristic"] == "2"
                    && field["degree"].as_u64() == Some(n)
                    && field["basis"] == "polynomial-lsb-first",
                "binary field metadata mismatch"
            );
            let rows = field["irreducible_low_terms"]
                .as_array()
                .context("missing field polynomial")?;
            ensure!(rows.len() <= n as usize, "too many polynomial exponents");
            let terms = rows
                .iter()
                .map(|v| integer(v, "polynomial exponent", 0, n - 1).map(|x| x as u32))
                .collect::<Result<Vec<_>>>()?;
            Field::binary(n as u32, &terms)?
        }
        _ => bail!("unknown curve family"),
    };
    let q = arithmetic.q();
    let curve = Curve::new(
        arithmetic,
        element(&model["a"], "curve a", q)?,
        element(&model["b"], "curve b", q)?,
    )?;
    let generator = curve.point(&public["generator"], "generator", true)?;
    let target = curve.point(&public["target"], "target", true)?;
    let declared_order = decimal(&model["subgroup_order"], "declared subgroup order")?;
    ensure!(declared_order > 1, "invalid declared subgroup bound");
    let present = ["base_a", "base_b", "twist_parameter"]
        .iter()
        .filter(|key| model.get(**key).is_some())
        .count();
    ensure!(present == 0 || present == 3, "partial twist definition");
    if present == 3 {
        let base_a = element(&model["base_a"], "base a", q)?;
        let base_b = element(&model["base_b"], "base b", q)?;
        let d = element(&model["twist_parameter"], "twist parameter", q)?;
        Curve::new(curve.field.clone(), base_a, base_b)?;
        let expected = if curve.field.binary_model() {
            ensure!(
                curve.field.trace(d)? == 1,
                "binary twist parameter must have trace one"
            );
            (if twist { base_a ^ d } else { base_a }, base_b)
        } else {
            ensure!(
                d != 0 && curve.field.pow(d, (q - 1) / 2) == q - 1,
                "prime twist parameter must be a nonsquare"
            );
            if twist {
                let d2 = curve.field.mul(d, d);
                (
                    curve.field.mul(base_a, d2),
                    curve.field.mul(base_b, curve.field.mul(d2, d)),
                )
            } else {
                (base_a, base_b)
            }
        };
        ensure!((curve.a, curve.b) == expected, "twist coefficient mismatch");
        if family == "koblitz" {
            ensure!(base_a <= 1 && base_b == 1, "invalid Koblitz base model");
        }
    } else if family == "koblitz" {
        ensure!(
            curve.b == 1 && (twist || curve.a <= 1),
            "invalid Koblitz model"
        );
    }
    Ok(PublicCurve {
        curve,
        generator,
        target,
        declared_order,
        arity,
        twist_validated: present == 3,
    })
}

fn signed_domain(curve: &Curve, base: &Value) -> Result<BTreeSet<Point>> {
    ensure!(
        base.is_object()
            && matches!(
                base["schema"].as_str(),
                Some("ml-ecdlp-factor-base/v1" | "ml-ecdlp-factor-base/v2")
            ),
        "invalid factor-base schema"
    );
    let rows = base["points"]
        .as_array()
        .context("missing factor-base points")?;
    ensure!(!rows.is_empty(), "empty factor base");
    let mut original = BTreeSet::new();
    let mut signed = BTreeSet::new();
    for row in rows {
        let point = curve.point(row, "factor-base point", true)?;
        ensure!(original.insert(point), "duplicate factor-base point");
        signed.insert(point);
        signed.insert(curve.neg(point)?);
    }
    Ok(signed)
}

fn kummer_domain(
    curve: &Curve,
    original: &BTreeSet<Point>,
    definition: &Value,
    config: &Value,
) -> Result<BTreeSet<Point>> {
    ensure!(
        config["solver"] == "pkm" && !curve.field.binary_model(),
        "alternate factor base requires prime PKM"
    );
    ensure!(
        definition.is_object() && definition["kind"] == "prime-kummer",
        "invalid alternate factor base"
    );
    let t = integer(&definition["t"], "Kummer height", 1, 4)?;
    let p = curve.field.q();
    let g = element(&definition["g"], "Kummer coset", p)?;
    ensure!(
        g > 0 && (p - 1).is_multiple_of(1u64 << t),
        "invalid Kummer parameters"
    );
    let expected_g = original
        .iter()
        .filter_map(|point| match point {
            Some((x, _)) if *x != 0 => Some(*x),
            _ => None,
        })
        .min();
    ensure!(
        expected_g == Some(g),
        "Kummer coset differs from original factor base"
    );
    let rows = definition["points"]
        .as_array()
        .context("missing Kummer points")?;
    ensure!(!rows.is_empty(), "empty Kummer domain");
    let inverse = curve.field.inverse(g)?;
    let mut points = BTreeSet::new();
    for row in rows {
        let point = curve.point(row, "Kummer point", true)?;
        let (x, _) = point.context("infinite Kummer point")?;
        ensure!(
            curve.field.pow(curve.field.mul(x, inverse), 1u64 << t) == 1,
            "point outside defined Kummer base"
        );
        ensure!(points.insert(point), "duplicate Kummer point");
    }
    // Do not add unlisted negatives or claim this list is exhaustive.
    Ok(points)
}

/// Independently check one public positive certificate, without witness access.
///
/// `declared_domain` denotes scalar bounds or the explicit finite PDP domain,
/// never membership in a certified prime-order subgroup. In particular, a
/// declared r is only a bound: neither its primality nor the group order is
/// inferred. A valid equation does not establish how a solver discovered it.
pub fn verify_certificate(
    public: &Value,
    base: &Value,
    config: &Value,
    cert: &Value,
    role: &str,
) -> Result<Value> {
    ensure!(
        role == SCALAR || role == PDP,
        "unsupported certificate role"
    );
    ensure!(cert.is_object(), "certificate must be an object");
    let context = public_curve(public, config)?;
    let curve = &context.curve;
    ensure!(
        cert["target"] == public["target"],
        "certificate target mismatch"
    );
    let domain_kind;
    if role == SCALAR {
        ensure!(
            cert["generator"] == public["generator"],
            "certificate generator mismatch"
        );
        let scalar = decimal(&cert["scalar"], "recovered scalar")?;
        ensure!(
            scalar > 0 && scalar < context.declared_order,
            "scalar outside declared order bound"
        );
        ensure!(
            curve.mul(context.generator, scalar)? == context.target,
            "scalar group relation failed"
        );
        domain_kind = "scalar-interval-under-declared-order";
    } else {
        crate::policy::bind_base(public, base, config)?;
        ensure!(
            base["construction_status"] != "unsupported",
            "positive PDP certificate has unsupported factor-base policy"
        );
        let mut domain = signed_domain(curve, base)?;
        domain_kind = if let Some(definition) = cert.get("factor_base_definition") {
            domain = kummer_domain(curve, &domain, definition, config)?;
            "declared-prime-kummer-domain"
        } else {
            "declared-signed-factor-base"
        };
        let summands = cert["summands"]
            .as_array()
            .context("missing PDP summands")?;
        ensure!(summands.len() == context.arity, "PDP arity mismatch");
        let mut total = None;
        for row in summands {
            let point = curve.point(row, "PDP summand", true)?;
            ensure!(
                domain.contains(&point),
                "PDP summand outside declared domain"
            );
            total = curve.add(total, point)?;
        }
        ensure!(total == context.target, "PDP group relation failed");
        ensure!(
            cert.get("sum") == public.get("target"),
            "certificate sum mismatch"
        );
    }
    Ok(json!({
        "schema":"ml-dataset.independent-arithmetic/v1", "role":role,
        "field_validated":true,"on_curve":true,"group_relation":true,"declared_domain":true,
        "domain_kind":domain_kind,"twist_validated":context.twist_validated,
        "group_order":false,"subgroup_order":false,"factor_base_construction":false,
        "solver_independence":false,"unsat":false,"fall_degree":false,
        "instructions":false,"exhaustiveness":false,"private_witness_read":false,
        "producer_native_called":false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_base_positive_pdp_keeps_construction_and_root_claims_unverified() {
        let (public, base, config) = crate::policy::tests::fixture();
        let certificate = json!({"target":public["target"],"sum":public["target"],
            "summands":[public["generator"],public["generator"]]});
        let result = verify_certificate(&public, &base, &config, &certificate, PDP).unwrap();
        assert_eq!(result["group_relation"], true);
        assert_eq!(result["declared_domain"], true);
        for key in [
            "factor_base_construction",
            "subgroup_order",
            "unsat",
            "fall_degree",
            "instructions",
            "exhaustiveness",
        ] {
            assert_eq!(result[key], false);
        }
        let mut wrong = base.clone();
        wrong["construction_context"]["sampling_seed"] = json!("0");
        assert!(verify_certificate(&public, &wrong, &config, &certificate, PDP).is_err());
        let mut wrong_certificate = certificate;
        wrong_certificate["summands"][0] = public["target"].clone();
        assert!(verify_certificate(&public, &base, &config, &wrong_certificate, PDP).is_err());
    }

    // Fixed public software vectors; no producer calls or campaign cells.
    struct Fixture {
        public: Value,
        base: Value,
        config: Value,
        cert: Value,
    }

    impl Fixture {
        fn verify(&self, role: &str) -> Result<Value> {
            verify_certificate(&self.public, &self.base, &self.config, &self.cert, role)
        }
    }

    fn point(x: u64, y: u64) -> Value {
        json!({"x":x.to_string(),"y":y.to_string()})
    }

    fn prime() -> Fixture {
        Fixture {
            config: json!({"family":"prime","n":5,"m":2,"twist":false,"solver":"enumerate"}),
            public: json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":5,"m":2,"twist":false,
                "field":{"characteristic":"17","modulus":"17","degree":1,"basis":"canonical-residue"},
                "curve":{"model":"short-weierstrass","a":"2","b":"2","subgroup_order":"19"},
                "generator":point(5,1),"target":point(6,3)}),
            base: json!({"schema":"ml-ecdlp-factor-base/v1","points":[point(5,1)]}),
            cert: json!({"scalar":"2","generator":point(5,1),"target":point(6,3)}),
        }
    }

    fn binary() -> Fixture {
        Fixture {
            config: json!({"family":"binary","n":3,"m":2,"twist":false,"solver":"enumerate"}),
            public: json!({"schema":"ml-ecdlp-instance/v1","family":"binary","n":3,"m":2,"twist":false,
                "field":{"characteristic":"2","degree":3,"irreducible_low_terms":[0,1],"basis":"polynomial-lsb-first"},
                "curve":{"model":"binary-ordinary","a":"0","b":"1","subgroup_order":"4"},
                "generator":point(1,0),"target":point(0,1)}),
            base: json!({"schema":"ml-ecdlp-factor-base/v1","points":[point(1,0)]}),
            cert: json!({"scalar":"2","generator":point(1,0),"target":point(0,1)}),
        }
    }

    fn pdp(mut f: Fixture) -> Fixture {
        f.cert = json!({"summands":[f.public["generator"].clone(),f.public["generator"].clone()],
                        "target":f.public["target"].clone(),"sum":f.public["target"].clone()});
        f
    }

    fn pkm() -> Fixture {
        let mut f = pdp(prime());
        f.config["solver"] = json!("pkm");
        f.cert["factor_base_definition"] = json!({"kind":"prime-kummer","t":2,"g":"5",
                                                "points":[point(5,1),point(3,1)]});
        f
    }

    fn bounded(report: Value) {
        for key in [
            "field_validated",
            "on_curve",
            "group_relation",
            "declared_domain",
        ] {
            assert_eq!(report[key], true, "{key}");
        }
        for key in [
            "group_order",
            "subgroup_order",
            "solver_independence",
            "unsat",
            "fall_degree",
            "instructions",
            "exhaustiveness",
            "factor_base_construction",
            "private_witness_read",
            "producer_native_called",
        ] {
            assert_eq!(report[key], false, "{key}");
        }
    }

    #[test]
    fn prime_field_fixed_values() {
        let f = Field::prime(17).unwrap();
        assert_eq!(
            (
                f.q(),
                f.add(16, 2),
                f.sub(1, 2),
                f.neg(3),
                f.mul(13, 11),
                f.inverse(5).unwrap(),
                f.pow(3, 4)
            ),
            (17, 1, 16, 14, 7, 7, 13)
        );
    }

    #[test]
    fn binary_field_fixed_values() {
        let f = Field::binary(3, &[0, 1]).unwrap();
        assert_eq!(
            (f.q(), f.add(3, 5), f.sub(3, 5), f.neg(3), f.mul(2, 4)),
            (8, 6, 6, 3, 3)
        );
        assert_eq!(
            (f.inverse(2).unwrap(), f.inverse(3).unwrap(), f.pow(2, 4)),
            (5, 6, 6)
        );
        assert_eq!((f.trace(1).unwrap(), f.trace(2).unwrap()), (1, 0));
    }

    #[test]
    fn even_binary_trace_is_not_low_bit() {
        let f = Field::binary(4, &[0, 1]).unwrap();
        assert_eq!((f.trace(1).unwrap(), f.trace(8).unwrap()), (0, 1));
    }

    #[test]
    fn zero_has_no_inverse() {
        assert!(Field::prime(17).unwrap().inverse(0).is_err());
        assert!(Field::binary(3, &[0, 1]).unwrap().inverse(0).is_err());
    }

    #[test]
    fn strong_pseudoprimes_are_rejected() {
        for composite in [
            23 * 89,
            829 * 1657,
            151 * 751 * 28351,
            10670053u64 * 32010157,
        ] {
            assert!(Field::prime(composite).is_err(), "{composite}");
        }
    }

    #[test]
    fn invalid_prime_fields_rejected() {
        for p in [0, 1, 2, 3, 4, u64::MAX] {
            assert!(Field::prime(p).is_err());
        }
    }

    #[test]
    fn reducible_binary_polynomials_rejected() {
        assert!(Field::binary(3, &[0]).is_err());
        assert!(Field::binary(4, &[0, 2]).is_err());
    }

    #[test]
    fn malformed_binary_polynomials_rejected() {
        for (n, terms) in [
            (0, vec![0]),
            (62, vec![0, 1]),
            (3, vec![]),
            (3, vec![1]),
            (3, vec![0, 0, 1]),
            (3, vec![0, 3]),
        ] {
            assert!(Field::binary(n, &terms).is_err());
        }
    }

    #[test]
    fn prime_fixed_multiples_and_inverses() {
        let c = Curve::new(Field::prime(17).unwrap(), 2, 2).unwrap();
        let g = Some((5, 1));
        for (k, expected) in [
            (0, None),
            (1, g),
            (2, Some((6, 3))),
            (3, Some((10, 6))),
            (4, Some((3, 1))),
            (18, Some((5, 16))),
            (19, None),
        ] {
            assert_eq!(c.mul(g, k).unwrap(), expected, "{k}");
        }
        assert_eq!(c.add(g, Some((6, 3))).unwrap(), Some((10, 6)));
        assert_eq!(c.neg(g).unwrap(), Some((5, 16)));
        assert_eq!(c.add(g, Some((5, 16))).unwrap(), None);
        assert_eq!(c.add(None, g).unwrap(), g);
        assert_eq!(c.add(g, None).unwrap(), g);
        assert_eq!(c.add(None, None).unwrap(), None);
    }

    #[test]
    fn prime_y_zero_doubling_is_infinity() {
        let c = Curve::new(Field::prime(17).unwrap(), 0, 1).unwrap();
        assert!(c.on_curve(Some((16, 0))));
        assert_eq!(c.add(Some((16, 0)), Some((16, 0))).unwrap(), None);
    }

    #[test]
    fn binary_exceptional_doubling_and_negation() {
        let c = Curve::new(Field::binary(3, &[0, 1]).unwrap(), 0, 1).unwrap();
        let g = Some((1, 0));
        assert_eq!(c.neg(g).unwrap(), Some((1, 1)));
        assert_eq!(c.add(g, g).unwrap(), Some((0, 1)));
        assert_eq!(c.mul(g, 3).unwrap(), Some((1, 1)));
        assert_eq!(c.mul(g, 4).unwrap(), None);
        assert_eq!(c.add(g, Some((1, 1))).unwrap(), None);
        assert_eq!(c.add(Some((0, 1)), Some((0, 1))).unwrap(), None);
        assert_eq!(c.neg(Some((0, 1))).unwrap(), Some((0, 1)));
        assert_eq!(c.add(None, g).unwrap(), g);
    }

    #[test]
    fn singular_and_noncanonical_curves_rejected() {
        assert!(Curve::new(Field::prime(17).unwrap(), 0, 0).is_err());
        assert!(Curve::new(Field::binary(3, &[0, 1]).unwrap(), 0, 0).is_err());
        assert!(Curve::new(Field::prime(17).unwrap(), 17, 2).is_err());
        assert!(Curve::new(Field::binary(3, &[0, 1]).unwrap(), 0, 8).is_err());
    }

    #[test]
    fn p61_arithmetic_boundary() {
        let p = (1u64 << 61) - 1;
        let f = Field::prime(p).unwrap();
        assert_eq!(f.mul(p - 1, p - 1), 1);
        assert_eq!(f.inverse(p - 1).unwrap(), p - 1);
        assert_eq!(f.add(p - 1, p - 1), p - 2);
        let c = Curve::new(f, 0, 1).unwrap();
        assert_eq!(c.mul(Some((0, 1)), 2).unwrap(), Some((0, p - 1)));
        assert_eq!(c.mul(Some((0, 1)), 3).unwrap(), None);
    }

    #[test]
    fn canonical_point_parser_and_infinity() {
        let c = Curve::new(Field::prime(17).unwrap(), 2, 2).unwrap();
        assert_eq!(c.point(&point(5, 1), "point", false).unwrap(), Some((5, 1)));
        assert_eq!(c.point(&Value::Null, "point", false).unwrap(), None);
        assert!(c.point(&Value::Null, "point", true).is_err());
        for malformed in [
            json!({}),
            json!({"infinity":true}),
            json!({"x":"5"}),
            json!({"x":"5","y":"1","infinity":true}),
            json!({"x":5,"y":"1"}),
            json!({"x":"05","y":"1"}),
            json!({"x":"+5","y":"1"}),
            json!({"x":" 5","y":"1"}),
            json!({"x":"-1","y":"1"}),
            point(17, 1),
            point(5, 17),
        ] {
            assert!(c.point(&malformed, "point", false).is_err(), "{malformed}");
        }
    }

    #[test]
    fn prime_scalar_certificate() {
        bounded(prime().verify(SCALAR).unwrap());
    }

    #[test]
    fn binary_scalar_bound_is_not_prime_order_claim() {
        bounded(binary().verify(SCALAR).unwrap());
    }

    #[test]
    fn prime_pdp_certificate() {
        bounded(pdp(prime()).verify(PDP).unwrap());
    }

    #[test]
    fn binary_pdp_certificate() {
        bounded(pdp(binary()).verify(PDP).unwrap());
    }

    #[test]
    fn wrong_or_noncanonical_scalar_rejected() {
        for scalar in [
            json!("1"),
            json!("0"),
            json!("19"),
            json!("-1"),
            json!("02"),
            json!("+2"),
            json!("2 "),
            json!(2),
            json!(true),
        ] {
            let mut f = prime();
            f.cert["scalar"] = scalar;
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn public_and_certificate_targets_bound() {
        let mut f = prime();
        f.public["target"] = point(10, 6);
        assert!(f.verify(SCALAR).is_err());
        let mut f = prime();
        f.cert["target"] = point(10, 6);
        assert!(f.verify(SCALAR).is_err());
    }

    #[test]
    fn certificate_generator_bound() {
        let mut f = prime();
        f.cert["generator"] = point(6, 3);
        assert!(f.verify(SCALAR).is_err());
    }

    #[test]
    fn invalid_public_points_rejected() {
        for (mut f, bad) in [
            (prime(), point(5, 2)),
            (prime(), point(22, 1)),
            (binary(), point(1, 2)),
            (binary(), point(9, 0)),
        ] {
            f.public["generator"] = bad.clone();
            f.cert["generator"] = bad;
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn public_infinity_rejected() {
        for bad in [
            Value::Null,
            json!({}),
            json!({"infinity":true}),
            json!({"x":null,"y":null}),
        ] {
            let mut f = prime();
            f.public["target"] = bad.clone();
            f.cert["target"] = bad;
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn public_config_identity_bound() {
        for (key, value) in [
            ("family", json!("binary")),
            ("n", json!(6)),
            ("m", json!(4)),
            ("twist", json!(true)),
        ] {
            let mut f = prime();
            f.public[key] = value;
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn field_metadata_bound() {
        for (key, value) in [
            ("characteristic", json!("19")),
            ("degree", json!(2)),
            ("degree", json!(true)),
            ("basis", json!("polynomial-lsb-first")),
            ("modulus", json!("017")),
        ] {
            let mut f = prime();
            f.public["field"][key] = value;
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn pdp_sum_target_and_arity_bound() {
        for (key, value) in [
            ("sum", point(10, 6)),
            ("target", point(10, 6)),
            ("summands", json!([point(5, 1)])),
            ("summands", json!([point(5, 1), point(5, 16)])),
        ] {
            let mut f = pdp(prime());
            f.cert[key] = value;
            assert!(f.verify(PDP).is_err());
        }
    }

    #[test]
    fn ordinary_domain_is_signed() {
        let mut f = pdp(prime());
        f.base["points"] = json!([point(5, 16)]);
        bounded(f.verify(PDP).unwrap());
    }

    #[test]
    fn pdp_domain_and_offcurve_summands_rejected() {
        let mut f = pdp(prime());
        f.base["points"] = json!([point(10, 6)]);
        assert!(f.verify(PDP).is_err());
        let mut f = pdp(prime());
        f.base["points"] = json!([point(5, 2)]);
        f.cert["summands"] = json!([point(5, 2), point(5, 2)]);
        assert!(f.verify(PDP).is_err());
    }

    #[test]
    fn prime_twist_coefficients_and_relation() {
        let mut f = prime();
        f.public["twist"] = json!(true);
        f.config["twist"] = json!(true);
        f.public["curve"] = json!({"model":"short-weierstrass","a":"1","b":"3","subgroup_order":"19",
                                   "base_a":"2","base_b":"2","twist_parameter":"3"});
        f.public["generator"] = point(2, 8);
        f.cert["generator"] = point(2, 8);
        f.public["target"] = point(12, 3);
        f.cert["target"] = point(12, 3);
        bounded(f.verify(SCALAR).unwrap());
        f.public["curve"]["twist_parameter"] = json!("2");
        assert!(f.verify(SCALAR).is_err());
    }

    #[test]
    fn untwisted_prime_metadata_checked() {
        let mut f = prime();
        for (key, value) in [("base_a", "2"), ("base_b", "2"), ("twist_parameter", "3")] {
            f.public["curve"][key] = json!(value);
        }
        bounded(f.verify(SCALAR).unwrap());
        f.public["curve"]["base_a"] = json!("1");
        assert!(f.verify(SCALAR).is_err());
    }

    fn twist_binary(n: u64, shift: &str) -> Fixture {
        let mut f = binary();
        f.public["n"] = json!(n);
        f.config["n"] = json!(n);
        f.public["field"]["degree"] = json!(n);
        f.public["twist"] = json!(true);
        f.config["twist"] = json!(true);
        for (key, value) in [
            ("a", shift),
            ("base_a", "0"),
            ("base_b", "1"),
            ("twist_parameter", shift),
        ] {
            f.public["curve"][key] = json!(value);
        }
        f.public["generator"] = point(0, 1);
        f.cert["generator"] = point(0, 1);
        f.cert["scalar"] = json!("1");
        f
    }

    #[test]
    fn binary_twist_requires_trace_one() {
        bounded(twist_binary(3, "1").verify(SCALAR).unwrap());
        assert!(twist_binary(3, "2").verify(SCALAR).is_err());
    }

    #[test]
    fn even_binary_twist_requires_absolute_trace() {
        bounded(twist_binary(4, "8").verify(SCALAR).unwrap());
        assert!(twist_binary(4, "1").verify(SCALAR).is_err());
    }

    #[test]
    fn partial_twist_definition_rejected() {
        for mut f in [prime(), binary()] {
            f.public["curve"]["base_a"] = f.public["curve"]["a"].clone();
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn koblitz_base_coefficients_checked() {
        let mut f = binary();
        f.public["family"] = json!("koblitz");
        f.config["family"] = json!("koblitz");
        for (key, value) in [("base_a", "0"), ("base_b", "1"), ("twist_parameter", "1")] {
            f.public["curve"][key] = json!(value);
        }
        bounded(f.verify(SCALAR).unwrap());
        f.public["curve"]["base_a"] = json!("2");
        assert!(f.verify(SCALAR).is_err());
    }

    #[test]
    fn pkm_alternate_domain_valid() {
        bounded(pkm().verify(PDP).unwrap());
    }

    #[test]
    fn pkm_cannot_override_other_solver() {
        let mut f = pkm();
        f.config["solver"] = json!("enumerate");
        assert!(f.verify(PDP).is_err());
    }

    #[test]
    fn pkm_coset_height_and_points_bound() {
        for (key, value) in [
            ("g", json!("6")),
            ("g", json!("05")),
            ("t", json!(0)),
            ("t", json!(5)),
            ("t", json!(true)),
            ("kind", json!("unchecked")),
            ("points", json!([point(5, 1), point(6, 3)])),
            ("points", json!([point(5, 1), point(5, 1)])),
        ] {
            let mut f = pkm();
            f.cert["factor_base_definition"][key] = value;
            assert!(f.verify(PDP).is_err());
        }
    }

    #[test]
    fn pkm_domain_not_implicitly_closed_under_negation() {
        let mut f = pkm();
        f.cert["factor_base_definition"]["points"] = json!([point(5, 16)]);
        assert!(f.verify(PDP).is_err());
    }

    #[test]
    fn unknown_role_rejected() {
        assert!(prime().verify("producer-success").is_err());
    }

    #[test]
    fn decimal_word_overflow_and_non_ascii_rejected() {
        for value in [
            json!("18446744073709551616"),
            json!("999999999999999999999"),
            json!("２"),
            json!(""),
            json!(2.0),
        ] {
            assert!(decimal(&value, "test").is_err());
        }
        assert_eq!(
            decimal(&json!(u64::MAX.to_string()), "test").unwrap(),
            u64::MAX
        );
    }

    #[test]
    fn full_word_products_and_discriminant_do_not_overflow() {
        let p = u64::MAX - 58; // Fixed largest 64-bit prime, independently tested above.
        let f = Field::prime(p).unwrap();
        assert_eq!(f.mul(p - 1, p - 1), 1);
        assert_eq!(f.add(p - 1, p - 1), p - 2);
        assert!(Curve::new(f.clone(), p - 1, p - 1).is_ok()); // discriminant = 23 mod p
        let c = Curve::new(f, 0, 1).unwrap();
        assert_eq!(c.mul(Some((0, 1)), 2).unwrap(), Some((0, p - 1)));
    }

    #[test]
    fn absent_public_fields_and_wrong_json_types_rejected() {
        for key in [
            "schema",
            "field",
            "curve",
            "generator",
            "target",
            "n",
            "m",
            "family",
            "twist",
        ] {
            let mut f = prime();
            f.public.as_object_mut().unwrap().remove(key);
            assert!(f.verify(SCALAR).is_err(), "{key}");
        }
        for bad in [Value::Null, json!([]), json!(true), json!("not an object")] {
            let f = prime();
            assert!(verify_certificate(&bad, &f.base, &f.config, &f.cert, SCALAR).is_err());
        }
    }

    #[test]
    fn declared_orders_are_not_certified() {
        let mut f = prime();
        f.public["curve"]["subgroup_order"] = json!("20"); // Deliberately composite declaration.
        f.public["curve"]["group_order"] = json!("not-a-count-certificate");
        let report = f.verify(SCALAR).unwrap();
        assert_eq!(report["group_order"], false);
        assert_eq!(report["subgroup_order"], false);
        assert_eq!(report["group_relation"], true);
    }

    #[test]
    fn binary_polynomial_json_types_and_duplicates_rejected() {
        for terms in [
            json!([0, true]),
            json!([0, -1]),
            json!([0, 1.0]),
            json!([0, 0, 1]),
            json!([0, 3]),
            json!(null),
        ] {
            let mut f = binary();
            f.public["field"]["irreducible_low_terms"] = terms;
            assert!(f.verify(SCALAR).is_err());
        }
    }

    #[test]
    fn supported_arity_upper_boundary_and_bound_rejection() {
        let mut f = pdp(prime());
        f.public["m"] = json!(15);
        f.config["m"] = json!(15);
        // Fifteen copies of G = -4G = (3,16) on the fixed order-19 curve.
        f.public["target"] = point(3, 16);
        f.cert["target"] = point(3, 16);
        f.cert["sum"] = point(3, 16);
        f.cert["summands"] = json!(vec![point(5, 1); 15]);
        bounded(f.verify(PDP).unwrap());
        f.public["m"] = json!(16);
        f.config["m"] = json!(16);
        assert!(f.verify(PDP).is_err());
    }

    #[test]
    fn duplicate_empty_or_infinite_factor_bases_rejected() {
        for rows in [json!([]), json!([null]), json!([point(5, 1), point(5, 1)])] {
            let mut f = pdp(prime());
            f.base["points"] = rows;
            assert!(f.verify(PDP).is_err());
        }
    }

    #[test]
    fn missing_pdp_sum_and_null_override_rejected() {
        let mut f = pdp(prime());
        f.cert.as_object_mut().unwrap().remove("sum");
        assert!(f.verify(PDP).is_err());
        let mut f = pdp(prime());
        f.cert["factor_base_definition"] = Value::Null;
        assert!(f.verify(PDP).is_err());
    }
}
