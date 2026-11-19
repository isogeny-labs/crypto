//! Bounded polynomial-basis pilot for the five smallest odd-extension covers.
//!
//! The live catalog contains five degree-two maps over `GF(5^2)` or
//! `GF(7^2)` with `x=u^2, y=v`.  This binary parses those exact field models,
//! implements their recorded polynomial bases, proves the basis conversion to
//! the existing `Fp2Ctx` representation, and exhaustively checks the source
//! equation and map on every rational affine point. The supplied graph is
//! first regenerated exactly from the raw registry bytes. It intentionally
//! does not claim arbitrary Jacobian-class transfer or subgroup transport.

#[path = "curve_cover_check/checker.rs"]
mod checker;
#[path = "curve_cover_check/links.rs"]
mod links;
#[path = "curve_cover_check/models.rs"]
mod models;

use crypto_lib::prime_hyperelliptic::{Fp2, Fp2Ctx};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, error::Error, fmt, fs};

const SCHEMA: &str = "odd-extension-transfer-pilot/v1";
const EXPECTED_MEMBERSHIP_SHA256: &str =
    "9563f8923d026ce915caf2345e0faca9ffff7636de85aa9f469df46d525ed8e1";
const PRODUCER_SOURCE_DIGEST_ENCODING: &str = "for each listed UTF-8 source in order: u64be(len(path_bytes))||path_bytes||u64be(len(content_bytes))||content_bytes";
const EXPECTED_MAP_UIDS: [&str; 5] = [
    "urn:curve-cover-map:1:sha256:3c21e11b5aca2eabe9f008e622c25f7f3b93ae7283008eddef45554e2972b459",
    "urn:curve-cover-map:1:sha256:6c8ca91da650ee9f515c687e6366c9df169cdfa15bf89f97a64d1072882d22dd",
    "urn:curve-cover-map:1:sha256:98700133de04feadd2862762bd5254cc31153424ec646ba3fa783c7192ab558d",
    "urn:curve-cover-map:1:sha256:a7a6f5a34c40f52427b3ebe28d0f830c3f000597672d8f2b18f472bfe81d59ba",
    "urn:curve-cover-map:1:sha256:ac41cf69a761aefb1ad291d2a3ce72e4d6683a4e44500fe563e387ba2a38af11",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct K2 {
    a: u64,
    b: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FieldError {
    UnsupportedBasePrime(u64),
    InvalidModulus,
    ReducibleModulus,
    UnsupportedLinearTerm,
    MissingReferenceBasisIsomorphism,
    NoncanonicalElement,
}

impl fmt::Display for FieldError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedBasePrime(p) => {
                write!(out, "base prime {p} is outside the p=5,7 pilot")
            }
            Self::InvalidModulus => write!(out, "quadratic modulus is malformed or noncanonical"),
            Self::ReducibleModulus => write!(out, "quadratic modulus is reducible"),
            Self::UnsupportedLinearTerm => {
                write!(out, "pilot requires a modulus t^2+c with zero linear term")
            }
            Self::MissingReferenceBasisIsomorphism => {
                write!(out, "no scalar basis isomorphism to Fp2Ctx was found")
            }
            Self::NoncanonicalElement => write!(out, "extension element is noncanonical"),
        }
    }
}

impl Error for FieldError {}

/// `GF(p)[t]/(t^2+c1*t+c0)` in the exact catalog basis `(1,t)`.
#[derive(Clone, Debug)]
struct CatalogK2 {
    p: u64,
    c0: u64,
    c1: u64,
    reference: Fp2Ctx,
    reference_scale: u64,
    inverse_reference_scale: u64,
}

impl CatalogK2 {
    fn try_new(p: u64, low: &[u64]) -> Result<Self, FieldError> {
        if !matches!(p, 5 | 7) {
            return Err(FieldError::UnsupportedBasePrime(p));
        }
        if low.len() != 2 || low.iter().any(|coefficient| *coefficient >= p) {
            return Err(FieldError::InvalidModulus);
        }
        let (c0, c1) = (low[0], low[1]);
        if (0..p).any(|root| (mul(root, root, p) + mul(c1, root, p) + c0) % p == 0) {
            return Err(FieldError::ReducibleModulus);
        }
        if c1 != 0 {
            return Err(FieldError::UnsupportedLinearTerm);
        }
        let reference = Fp2Ctx::try_new(p).map_err(|_| FieldError::InvalidModulus)?;
        let catalog_t_squared = neg(c0, p);
        let reference_scale = (1..p)
            .find(|scale| {
                mul(mul(*scale, *scale, p), reference.nonresidue(), p) == catalog_t_squared
            })
            .ok_or(FieldError::MissingReferenceBasisIsomorphism)?;
        let inverse_reference_scale =
            inverse(reference_scale, p).ok_or(FieldError::MissingReferenceBasisIsomorphism)?;
        Ok(Self {
            p,
            c0,
            c1,
            reference,
            reference_scale,
            inverse_reference_scale,
        })
    }

    fn zero(&self) -> K2 {
        K2 { a: 0, b: 0 }
    }

    fn one(&self) -> K2 {
        K2 { a: 1, b: 0 }
    }

    fn from_base(&self, value: u64) -> K2 {
        K2 {
            a: value % self.p,
            b: 0,
        }
    }

    fn element(&self, a: u64, b: u64) -> Result<K2, FieldError> {
        if a >= self.p || b >= self.p {
            return Err(FieldError::NoncanonicalElement);
        }
        Ok(K2 { a, b })
    }

    #[cfg(test)]
    fn decode_packed(&self, value: u64) -> Result<K2, FieldError> {
        if value >= self.p * self.p {
            return Err(FieldError::NoncanonicalElement);
        }
        self.element(value % self.p, value / self.p)
    }

    fn pack(&self, value: K2) -> u64 {
        value.a + self.p * value.b
    }

    fn add(&self, left: K2, right: K2) -> K2 {
        K2 {
            a: (left.a + right.a) % self.p,
            b: (left.b + right.b) % self.p,
        }
    }

    fn neg(&self, value: K2) -> K2 {
        K2 {
            a: neg(value.a, self.p),
            b: neg(value.b, self.p),
        }
    }

    fn sub(&self, left: K2, right: K2) -> K2 {
        self.add(left, self.neg(right))
    }

    fn mul(&self, left: K2, right: K2) -> K2 {
        let bd = mul(left.b, right.b, self.p);
        K2 {
            a: (mul(left.a, right.a, self.p) + self.p - mul(self.c0, bd, self.p)) % self.p,
            b: (mul(left.a, right.b, self.p) + mul(left.b, right.a, self.p) + self.p
                - mul(self.c1, bd, self.p))
                % self.p,
        }
    }

    fn square(&self, value: K2) -> K2 {
        self.mul(value, value)
    }

    fn pow(&self, mut value: K2, mut exponent: u64) -> K2 {
        let mut result = self.one();
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.mul(result, value);
            }
            exponent >>= 1;
            if exponent != 0 {
                value = self.square(value);
            }
        }
        result
    }

    fn inverse(&self, value: K2) -> Option<K2> {
        (value != self.zero()).then(|| self.pow(value, self.p * self.p - 2))
    }

    fn divide(&self, numerator: K2, denominator: K2) -> Option<K2> {
        self.inverse(denominator)
            .map(|inverse| self.mul(numerator, inverse))
    }

    fn elements(&self) -> impl Iterator<Item = K2> + '_ {
        (0..self.p).flat_map(move |a| (0..self.p).map(move |b| K2 { a, b }))
    }

    fn square_roots(&self, value: K2) -> Vec<K2> {
        self.elements()
            .filter(|candidate| self.square(*candidate) == value)
            .collect()
    }

    fn to_reference(&self, value: K2) -> Fp2 {
        self.reference
            .element(value.a, mul(value.b, self.reference_scale, self.p))
    }

    fn from_reference(&self, value: Fp2) -> K2 {
        K2 {
            a: value.a % self.p,
            b: mul(value.b % self.p, self.inverse_reference_scale, self.p),
        }
    }

    fn verify_reference_isomorphism(&self) -> usize {
        let elements: Vec<_> = self.elements().collect();
        assert_eq!(self.to_reference(self.one()), self.reference.one());
        assert_eq!(self.to_reference(self.zero()), self.reference.zero());
        let catalog_t = K2 { a: 0, b: 1 };
        assert_eq!(
            self.reference.square(self.to_reference(catalog_t)),
            self.to_reference(self.square(catalog_t))
        );
        let mut pair_checks = 0usize;
        for left in &elements {
            assert_eq!(self.from_reference(self.to_reference(*left)), *left);
            if *left != self.zero() {
                assert_eq!(
                    self.from_reference(self.reference.inv(self.to_reference(*left)).unwrap()),
                    self.inverse(*left).unwrap()
                );
            }
            for right in &elements {
                pair_checks += 1;
                assert_eq!(
                    self.to_reference(self.add(*left, *right)),
                    self.reference
                        .add(self.to_reference(*left), self.to_reference(*right))
                );
                assert_eq!(
                    self.to_reference(self.mul(*left, *right)),
                    self.reference
                        .mul(self.to_reference(*left), self.to_reference(*right))
                );
            }
        }
        pair_checks
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Point {
    Infinity,
    Affine { x: K2, y: K2 },
}

fn target_is_on_curve(field: &CatalogK2, point: Point, a: K2, b: K2) -> bool {
    match point {
        Point::Infinity => true,
        Point::Affine { x, y } => {
            field.square(y)
                == field.add(field.add(field.mul(field.square(x), x), field.mul(a, x)), b)
        }
    }
}

fn target_add(field: &CatalogK2, left: Point, right: Point, a: K2) -> Point {
    match (left, right) {
        (Point::Infinity, point) | (point, Point::Infinity) => point,
        (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
            if x1 == x2 && field.add(y1, y2) == field.zero() {
                return Point::Infinity;
            }
            let slope = if x1 == x2 {
                let numerator = field.add(field.mul(field.from_base(3), field.square(x1)), a);
                let denominator = field.mul(field.from_base(2), y1);
                field
                    .divide(numerator, denominator)
                    .expect("nonvertical tangent denominator")
            } else {
                field
                    .divide(field.sub(y2, y1), field.sub(x2, x1))
                    .expect("distinct x-coordinates")
            };
            let x3 = field.sub(field.sub(field.square(slope), x1), x2);
            let y3 = field.sub(field.mul(slope, field.sub(x1, x3)), y1);
            Point::Affine { x: x3, y: y3 }
        }
    }
}

fn target_points(field: &CatalogK2, a: K2, b: K2) -> Vec<Point> {
    let mut points = vec![Point::Infinity];
    for x in field.elements() {
        for y in field.elements() {
            let point = Point::Affine { x, y };
            if target_is_on_curve(field, point, a, b) {
                points.push(point);
            }
        }
    }
    points
}

fn verify_target_group(field: &CatalogK2, points: &[Point], a: K2, b: K2) -> (usize, usize) {
    let mut pair_checks = 0usize;
    for left in points {
        assert_eq!(target_add(field, Point::Infinity, *left, a), *left);
        for right in points {
            pair_checks += 1;
            let sum = target_add(field, *left, *right, a);
            assert!(target_is_on_curve(field, sum, a, b));
            assert_eq!(sum, target_add(field, *right, *left, a));
        }
    }
    let mut triples = 0usize;
    for left in points {
        for middle in points {
            for right in points {
                triples += 1;
                assert_eq!(
                    target_add(field, target_add(field, *left, *middle, a), *right, a,),
                    target_add(field, *left, target_add(field, *middle, *right, a), a,)
                );
            }
        }
    }
    (pair_checks, triples)
}

fn source_is_on_curve(field: &CatalogK2, u: K2, v: K2, a: K2, b: K2) -> bool {
    let u2 = field.square(u);
    let u6 = field.mul(field.square(u2), u2);
    field.square(v) == field.add(field.add(u6, field.mul(a, u2)), b)
}

type KPoly = Vec<K2>;

fn polynomial_trim(field: &CatalogK2, polynomial: &mut KPoly) {
    while polynomial.last() == Some(&field.zero()) {
        polynomial.pop();
    }
}

fn polynomial_divrem(
    field: &CatalogK2,
    mut numerator: KPoly,
    denominator: &[K2],
) -> (KPoly, KPoly) {
    let denominator_degree = denominator
        .len()
        .checked_sub(1)
        .expect("nonzero polynomial denominator");
    let denominator_lead = denominator[denominator_degree];
    let mut quotient = if numerator.len() >= denominator.len() {
        vec![field.zero(); numerator.len() - denominator.len() + 1]
    } else {
        Vec::new()
    };
    polynomial_trim(field, &mut numerator);
    while numerator.len() > denominator_degree {
        let numerator_degree = numerator.len() - 1;
        let shift = numerator_degree - denominator_degree;
        let factor = field
            .divide(numerator[numerator_degree], denominator_lead)
            .expect("nonzero leading coefficient");
        quotient[shift] = field.add(quotient[shift], factor);
        for (index, coefficient) in denominator.iter().enumerate() {
            numerator[shift + index] =
                field.sub(numerator[shift + index], field.mul(factor, *coefficient));
        }
        polynomial_trim(field, &mut numerator);
    }
    polynomial_trim(field, &mut quotient);
    (quotient, numerator)
}

fn polynomial_gcd(field: &CatalogK2, mut left: KPoly, mut right: KPoly) -> KPoly {
    polynomial_trim(field, &mut left);
    polynomial_trim(field, &mut right);
    while !right.is_empty() {
        let remainder = polynomial_divrem(field, left, &right).1;
        left = right;
        right = remainder;
    }
    if let Some(lead) = left.last().copied() {
        let inverse = field.inverse(lead).expect("nonzero gcd lead");
        for coefficient in &mut left {
            *coefficient = field.mul(*coefficient, inverse);
        }
    }
    left
}

fn source_squarefree(field: &CatalogK2, a: K2, b: K2) -> bool {
    let zero = field.zero();
    let one = field.one();
    let source = vec![b, zero, a, zero, zero, zero, one];
    let mut derivative = vec![zero; source.len() - 1];
    for (degree, coefficient) in source.iter().copied().enumerate().skip(1) {
        derivative[degree - 1] = field.mul(coefficient, field.from_base(degree as u64));
    }
    polynomial_trim(field, &mut derivative);
    polynomial_gcd(field, source, derivative) == vec![one]
}

fn parse_integer(value: &Value) -> Result<u64, String> {
    let text = value
        .as_str()
        .ok_or_else(|| "integer must be a string".to_string())?;
    let (radix, digits) = text
        .strip_prefix("0x")
        .map_or((10, text), |digits| (16, digits));
    u64::from_str_radix(digits, radix).map_err(|_| "integer does not fit u64".to_string())
}

fn parse_list(value: &Value) -> Result<Vec<u64>, String> {
    value
        .as_array()
        .ok_or_else(|| "coefficient list is missing".to_string())?
        .iter()
        .map(parse_integer)
        .collect()
}

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn framed_source_digest(sources: &[(&str, &str)]) -> String {
    let mut framed = Vec::new();
    for (path, contents) in sources {
        framed.extend_from_slice(&(path.len() as u64).to_be_bytes());
        framed.extend_from_slice(path.as_bytes());
        framed.extend_from_slice(&(contents.len() as u64).to_be_bytes());
        framed.extend_from_slice(contents.as_bytes());
    }
    sha(&framed)
}

/// Exact local source closure whose behavior can affect graph regeneration or
/// this pilot's field/map report. External crate sources are pinned separately
/// by Cargo.lock.
fn producer_sources() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Cargo.lock", include_str!("../../Cargo.lock")),
        ("Cargo.toml", include_str!("../../Cargo.toml")),
        (
            "src/bin/curve_cover_check.rs",
            include_str!("curve_cover_check.rs"),
        ),
        (
            "src/bin/curve_cover_check/checker.rs",
            include_str!("curve_cover_check/checker.rs"),
        ),
        (
            "src/bin/curve_cover_check/links.rs",
            include_str!("curve_cover_check/links.rs"),
        ),
        (
            "src/bin/curve_cover_check/models.rs",
            include_str!("curve_cover_check/models.rs"),
        ),
        (
            "src/bin/odd_extension_transfer_pilot.rs",
            include_str!("odd_extension_transfer_pilot.rs"),
        ),
        (
            "src/binary_ecc/f2m.rs",
            include_str!("../binary_ecc/f2m.rs"),
        ),
        (
            "src/binary_ecc/mod.rs",
            include_str!("../binary_ecc/mod.rs"),
        ),
        (
            "src/binary_ecc/operation_tally.rs",
            include_str!("../binary_ecc/operation_tally.rs"),
        ),
        (
            "src/cryptanalysis/bitops.rs",
            include_str!("../cryptanalysis/bitops.rs"),
        ),
        (
            "src/cryptanalysis/ecc2k130_guard.rs",
            include_str!("../cryptanalysis/ecc2k130_guard.rs"),
        ),
        (
            "src/cryptanalysis/mod.rs",
            include_str!("../cryptanalysis/mod.rs"),
        ),
        ("src/ct_bignum.rs", include_str!("../ct_bignum.rs")),
        ("src/hash/mod.rs", include_str!("../hash/mod.rs")),
        ("src/hash/sha256.rs", include_str!("../hash/sha256.rs")),
        ("src/lib.rs", include_str!("../lib.rs")),
        (
            "src/prime_hyperelliptic/fp2.rs",
            include_str!("../prime_hyperelliptic/fp2.rs"),
        ),
        (
            "src/prime_hyperelliptic/mod.rs",
            include_str!("../prime_hyperelliptic/mod.rs"),
        ),
        ("src/utils/mod.rs", include_str!("../utils/mod.rs")),
    ]
}

fn producer_source_digest() -> String {
    framed_source_digest(&producer_sources())
}

fn producer_source_paths() -> Vec<&'static str> {
    producer_sources().iter().map(|(path, _)| *path).collect()
}

fn producer_source_files() -> Vec<Value> {
    producer_sources()
        .iter()
        .map(|(path, contents)| json!({"path": path, "sha256": sha(contents.as_bytes())}))
        .collect()
}

fn verify_identity(uid: &str, prefix: &str, record: &Value) -> Result<String, String> {
    let digest = sha(record.to_string().as_bytes());
    if uid != format!("{prefix}{digest}") {
        return Err(format!("identity mismatch for {uid}"));
    }
    Ok(digest)
}

fn verify_row(
    graph: &Value,
    registry: &Value,
    uid: &str,
    wrapper: &Value,
    target: &Value,
) -> Result<Value, String> {
    let record = &wrapper["record"];
    let map_digest = verify_identity(uid, "urn:curve-cover-map:1:sha256:", record)?;
    let expected_map_id = format!("CV1D2h{}", &map_digest[..12]);
    if wrapper["map_uid"].as_str() != Some(uid)
        || wrapper["map_id"].as_str() != Some(expected_map_id.as_str())
    {
        return Err(format!("map wrapper identity mismatch for {uid}"));
    }
    if record["degree"] != 2
        || record["x"] != json!(["0x0", "0x0", "0x1"])
        || record["y_v"] != json!(["0x1"])
        || record["y_0"] != json!([])
        || !record["target_model_map"].is_null()
    {
        return Err(format!("unexpected degree-two map formula in {uid}"));
    }
    let raw_target = record["target_model_json"]
        .as_str()
        .ok_or_else(|| "target model JSON missing".to_string())?;
    let target_digest = sha(raw_target.as_bytes());
    if record["target_model_sha256"].as_str() != Some(target_digest.as_str()) {
        return Err(format!("target model digest mismatch in {uid}"));
    }

    let p = parse_integer(&target["p"])?;
    let low = parse_list(&target["modulus"])?;
    let field = CatalogK2::try_new(p, &low).map_err(|error| error.to_string())?;
    let label_digest = sha(format!("fpk-modulus:{p}:{},{}", low[0], low[1]).as_bytes());
    if target["field"] != format!("fpk-{p}-2-{}", &label_digest[..8]) {
        return Err(format!("field label mismatch in {uid}"));
    }
    let a_digits = parse_list(&target["a"])?;
    let b_digits = parse_list(&target["b"])?;
    if a_digits.len() != 2 || b_digits.len() != 2 {
        return Err(format!("target coefficient length mismatch in {uid}"));
    }
    let a = field
        .element(a_digits[0], a_digits[1])
        .map_err(|e| e.to_string())?;
    let b = field
        .element(b_digits[0], b_digits[1])
        .map_err(|e| e.to_string())?;
    let discriminant = field.add(
        field.mul(field.from_base(4), field.mul(field.square(a), a)),
        field.mul(field.from_base(27), field.square(b)),
    );
    if discriminant == field.zero() {
        return Err(format!("singular target in {uid}"));
    }
    if !source_squarefree(&field, a, b) {
        return Err(format!("singular source sextic in {uid}"));
    }
    if field.from_base(2) == field.zero() {
        return Err(format!("inseparable quadratic coordinate map in {uid}"));
    }

    let source_uid = record["source_cover_uid"]
        .as_str()
        .ok_or_else(|| "source UID missing".to_string())?;
    let source_wrapper = graph["covers"]
        .get(source_uid)
        .ok_or_else(|| format!("source {source_uid} missing"))?;
    let source = &source_wrapper["record"];
    let source_digest = verify_identity(source_uid, "urn:hc-model:1:sha256:", source)?;
    let expected_cover_id = format!("HC1G2h{}", &source_digest[..12]);
    if source_wrapper["cover_uid"].as_str() != Some(source_uid)
        || source_wrapper["cover_id"].as_str() != Some(expected_cover_id.as_str())
    {
        return Err(format!("source wrapper identity mismatch for {source_uid}"));
    }
    if source["schema_version"] != "hyperelliptic-model/v1"
        || source["form"] != "v^2+h(u)*v=f(u)"
        || source["genus"] != 2
        || source["h"] != json!([])
        || source["field"]["p"] != target["p"]
        || source["field"]["modulus"] != target["modulus"]
        || source["field"]["label"] != target["field"]
    {
        return Err(format!("source metadata mismatch in {uid}"));
    }
    let f = parse_list(&source["f"])?;
    let expected_f = vec![
        field.pack(b),
        0,
        field.pack(a),
        0,
        0,
        0,
        field.pack(field.one()),
    ];
    if f != expected_f {
        return Err(format!("source equation mismatch in {uid}"));
    }

    let matching_registry_rows: Vec<_> = registry["curves"]
        .as_array()
        .ok_or_else(|| "registry curves missing".to_string())?
        .iter()
        .filter(|row| row["model_json"].as_str() == Some(raw_target))
        .collect();
    if matching_registry_rows.len() != 1 {
        return Err(format!("target registry join is not unique in {uid}"));
    }
    let representation_count = matching_registry_rows[0]["representations"]
        .as_array()
        .ok_or_else(|| "registry representations missing".to_string())?
        .len();
    if representation_count != 0 {
        return Err(format!(
            "pilot target unexpectedly has an EC1 binding in {uid}"
        ));
    }
    let matching_graph_rows: Vec<_> = graph["curves"]
        .as_array()
        .ok_or_else(|| "cover graph curves missing".to_string())?
        .iter()
        .filter(|row| row["model_json"].as_str() == Some(raw_target))
        .collect();
    if matching_graph_rows.len() != 1 {
        return Err(format!("target graph join is not unique in {uid}"));
    }
    let graph_row = matching_graph_rows[0];
    let graph_links = graph_row["cover_links"]
        .as_array()
        .filter(|links| links.len() == 1)
        .ok_or_else(|| format!("target graph row does not have one cover link in {uid}"))?;
    if graph_row["model_sha256"].as_str() != Some(target_digest.as_str())
        || graph_links[0]["map_uid"].as_str() != Some(uid)
        || graph_links[0]["cover_uid"].as_str() != Some(source_uid)
        || graph_row["representations"] != matching_registry_rows[0]["representations"]
        || graph_row["icv1_identity"]["full"].as_str().is_none()
        || graph_row["icv1_identity"]["slug"].as_str().is_none()
    {
        return Err(format!("target graph/registry identity mismatch in {uid}"));
    }

    let basis_pair_checks = field.verify_reference_isomorphism();
    let points = target_points(&field, a, b);
    let (target_pair_checks, associativity_triples) = verify_target_group(&field, &points, a, b);
    let mut source_affine_points = 0usize;
    let mut mapped_source_points = 0usize;
    for u in field.elements() {
        for v in field.elements() {
            if source_is_on_curve(&field, u, v, a, b) {
                source_affine_points += 1;
                let image = Point::Affine {
                    x: field.square(u),
                    y: v,
                };
                assert!(target_is_on_curve(&field, image, a, b));
                mapped_source_points += 1;
            }
        }
    }
    // With even degree six and square leading coefficient one, the two
    // smooth-projective branches above infinity have leading ratios
    // v/u^3 = +1 and -1.  Derive them in the recorded field rather than
    // merely adding two to the affine count.
    let infinity_branches = field.square_roots(field.one());
    assert_eq!(infinity_branches.len(), 2);
    assert_eq!(infinity_branches[1], field.neg(infinity_branches[0]));
    // For local source parameter s=1/u and target parameter -x/y,
    // phi^*(-x/y)=-(u^2/v)=(-1/w)*s+O(s^2), where w=v/u^3 is either
    // infinity branch.  The nonzero linear coefficient gives e=1.
    for branch in &infinity_branches {
        assert!(field.inverse(*branch).is_some());
    }

    let mut split_fibers = 0usize;
    let mut ramified_fibers = 0usize;
    let mut nonsplit_fibers = 0usize;
    let mut infinity_fibers = 0usize;
    for point in &points {
        match point {
            Point::Infinity => {
                infinity_fibers += 1;
                assert_eq!(target_add(&field, *point, *point, a), Point::Infinity);
            }
            Point::Affine { x, y } => {
                let roots = field.square_roots(*x);
                match roots.len() {
                    0 => {
                        nonsplit_fibers += 1;
                        // In K[u]/(u^2-x), the source equation reduces to the
                        // target equation, so the closed degree-two fiber is
                        // valid even though it has no K-rational support.
                        let rhs = field.add(
                            field.add(field.mul(field.square(*x), *x), field.mul(a, *x)),
                            b,
                        );
                        assert_eq!(field.square(*y), rhs);
                    }
                    1 => {
                        ramified_fibers += 1;
                        assert_eq!(*x, field.zero());
                        assert!(source_is_on_curve(&field, roots[0], *y, a, b));
                    }
                    2 => {
                        split_fibers += 1;
                        assert_eq!(roots[1], field.neg(roots[0]));
                        for root in roots {
                            assert!(source_is_on_curve(&field, root, *y, a, b));
                        }
                    }
                    count => return Err(format!("unexpected fiber size {count} in {uid}")),
                }
                // The formal degree-two fiber pushes to P+P. This checks the
                // target operation independently, but is not a reduced
                // source-Jacobian implementation.
                let pushed_pullback = target_add(&field, *point, *point, a);
                assert!(target_is_on_curve(&field, pushed_pullback, a, b));
            }
        }
    }

    Ok(json!({
        "map_uid": uid,
        "map_id": wrapper["map_id"],
        "source_cover_uid": source_uid,
        "target_model_sha256": record["target_model_sha256"],
        "icv1_identity": graph_row["icv1_identity"],
        "field": {
            "p": p,
            "degree": 2,
            "catalog_modulus_low": low,
            "catalog_relation": format!("t^2={}", neg(field.c0, p)),
            "fp2ctx_nonresidue": field.reference.nonresidue(),
            "catalog_t_to_fp2ctx_scale": field.reference_scale,
            "basis_relation": "proved_exhaustively",
            "basis_pair_checks": basis_pair_checks,
        },
        "counts": {
            "target_points": points.len(),
            "target_group_pairs": target_pair_checks,
            "target_associativity_triples": associativity_triples,
            "source_affine_points": source_affine_points,
            "source_infinity_branches": infinity_branches.len(),
            "source_projective_points": source_affine_points + infinity_branches.len(),
            "mapped_source_affine_points": mapped_source_points,
            "split_target_fibers": split_fibers,
            "ramified_target_fibers": ramified_fibers,
            "nonsplit_target_fibers": nonsplit_fibers,
            "target_infinity_fibers": infinity_fibers,
            "named_ec1_bindings": representation_count,
        },
        "statuses": {
            "field_model": "verified_exact_polynomial_basis",
            "basis_adapter": "verified_exhaustively",
            "source_equation": "verified_exact_coefficients_and_all_rational_points",
            "source_smoothness": "verified_sextic_squarefree_by_polynomial_gcd",
            "source_genus": "verified_genus_two_from_squarefree_degree_six_in_odd_characteristic",
            "curve_map": "verified_exact_formula_and_all_rational_affine_points",
            "curve_map_degree_and_separability": "verified_degree_two_with_nonzero_derivative_in_odd_characteristic",
            "infinity_behavior": "verified_two_rational_source_branches_each_unramified_over_target_infinity",
            "formal_curve_fiber_degree_identity": "verified_formal_degree_two_fibers_and_target_2P_without_source_class_reduction",
            "general_jacobian_arithmetic": "unsupported_not_implemented_by_pilot",
            "arbitrary_class_pullback_pushforward": "unsupported_not_implemented_by_pilot",
            "subgroup_transfer": "not_applicable_no_named_ec1_binding",
        }
    }))
}

fn build_report(graph_bytes: &[u8], registry_bytes: &[u8]) -> Result<Value, String> {
    let registry: Value = serde_json::from_slice(registry_bytes)
        .map_err(|error| format!("parse registry: {error}"))?;
    let geometry = checker::catalog(registry_bytes)?;
    let graph: Value = serde_json::from_slice(graph_bytes)
        .map_err(|error| format!("parse cover graph: {error}"))?;
    if graph["schema_version"] != "curve-cover-graph/v1"
        || graph["registry_sha256"] != geometry["registry_sha256"]
    {
        return Err("registry and cover graph schema/digest do not match".into());
    }
    let expected_graph = links::graph(&registry, &geometry)
        .map_err(|error| format!("rebuild cover graph: {error}"))?;
    if graph != expected_graph {
        return Err("cover graph is not the exact deterministic registry graph".into());
    }
    drop(expected_graph);
    drop(geometry);

    let maps = graph["maps"]
        .as_object()
        .ok_or_else(|| "cover graph maps missing".to_string())?;
    let mut extension_maps = 0usize;
    let mut unsupported_degree = 0usize;
    let mut unsupported_base_prime = 0usize;
    let mut unsupported_records = Vec::new();
    let mut selected: BTreeMap<String, (&Value, Value)> = BTreeMap::new();
    for (uid, wrapper) in maps {
        let Some(raw_target) = wrapper["record"]["target_model_json"].as_str() else {
            continue;
        };
        let target: Value = serde_json::from_str(raw_target)
            .map_err(|error| format!("target JSON for {uid}: {error}"))?;
        let Some(k) = target.get("k").and_then(Value::as_str) else {
            continue;
        };
        extension_maps += 1;
        if k != "2" {
            unsupported_degree += 1;
            unsupported_records.push(json!({
                "map_uid": uid,
                "status": "unsupported_extension_degree",
                "extension_degree": k,
            }));
            continue;
        }
        let p = parse_integer(&target["p"])?;
        if !matches!(p, 5 | 7) {
            unsupported_base_prime += 1;
            unsupported_records.push(json!({
                "map_uid": uid,
                "status": "unsupported_outside_pilot_base_prime",
                "extension_degree": 2,
                "p": p,
            }));
            continue;
        }
        if selected.insert(uid.clone(), (wrapper, target)).is_some() {
            return Err(format!("duplicate map {uid}"));
        }
    }
    let ids: Vec<_> = selected.keys().map(String::as_str).collect();
    if ids != EXPECTED_MAP_UIDS {
        return Err("five-row pilot membership changed".into());
    }
    let membership = format!("{}\n", ids.join("\n"));
    if sha(membership.as_bytes()) != EXPECTED_MEMBERSHIP_SHA256 {
        return Err("pilot membership digest mismatch".into());
    }
    if (
        extension_maps,
        unsupported_degree,
        unsupported_base_prime,
        selected.len(),
    ) != (1_405, 1_342, 58, 5)
    {
        return Err("extension population changed".into());
    }

    let mut records = Vec::new();
    for (uid, (wrapper, target)) in selected {
        records.push(verify_row(&graph, &registry, &uid, wrapper, &target)?);
    }
    Ok(json!({
        "schema": SCHEMA,
        "status": "verified_bounded_field_and_map_pilot",
        "producer": {
            "source_sha256": producer_source_digest(),
            "source_digest_encoding": PRODUCER_SOURCE_DIGEST_ENCODING,
            "source_paths": producer_source_paths(),
            "source_files": producer_source_files(),
        },
        "inputs": {
            "registry_sha256": sha(registry_bytes),
            "cover_graph_sha256": sha(graph_bytes),
            "graph_registry_sha256": graph["registry_sha256"],
            "exact_graph_regenerated": true,
        },
        "scope": {
            "families": ["odd_extension_quadratic_pullback_v1"],
            "base_primes": [5, 7],
            "extension_degree": 2,
            "map": "x=u^2,y=v",
            "membership_sha256": EXPECTED_MEMBERSHIP_SHA256,
        },
        "summary": {
            "odd_extension_maps": extension_maps,
            "verified_field_and_map_pilot_maps": records.len(),
            "unsupported_extension_degree": unsupported_degree,
            "unsupported_outside_pilot_base_prime": unsupported_base_prime,
            "named_ec1_bindings": 0,
            "general_jacobian_transfer_supported": 0,
        },
        "records": records,
        "unsupported_records": unsupported_records,
        "non_claims": [
            "The pilot does not implement general genus-two Jacobian reduction or arbitrary-class transfer.",
            "The five maps have no named EC1 subgroup binding, so no subgroup preservation claim is made.",
            "The other 1400 extension maps remain structured unsupported cases.",
            "No solver, scaling, key-recovery, or computational-advantage claim follows from these checks."
        ]
    }))
}

fn mul(left: u64, right: u64, p: u64) -> u64 {
    ((left as u128 * right as u128) % p as u128) as u64
}

fn neg(value: u64, p: u64) -> u64 {
    (p - value % p) % p
}

fn inverse(value: u64, p: u64) -> Option<u64> {
    (value % p != 0).then(|| {
        (1..p)
            .find(|candidate| mul(value, *candidate, p) == 1)
            .expect("nonzero element in a pilot prime field")
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 3 {
        return Err("usage: odd_extension_transfer_pilot [COVER_GRAPH] [REGISTRY]".into());
    }
    let graph_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("docs/curves/cover-links.yaml");
    let registry_path = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("docs/curves/registry.json");
    let graph = fs::read(graph_path)?;
    let registry = fs::read(registry_path)?;
    let report = build_report(&graph, &registry)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_live_five_row_pilot_passes_and_retains_unsupported_population() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let graph_bytes = fs::read(root.join("docs/curves/cover-links.yaml")).unwrap();
        let registry_bytes = fs::read(root.join("docs/curves/registry.json")).unwrap();
        let graph: Value = serde_json::from_slice(&graph_bytes).unwrap();
        let registry: Value = serde_json::from_slice(&registry_bytes).unwrap();
        let report = build_report(&graph_bytes, &registry_bytes).unwrap();
        assert_eq!(report["status"], "verified_bounded_field_and_map_pilot");
        assert_eq!(report["inputs"]["registry_sha256"], sha(&registry_bytes));
        assert_eq!(report["inputs"]["cover_graph_sha256"], sha(&graph_bytes));
        assert_eq!(report["inputs"]["exact_graph_regenerated"], true);
        assert_eq!(
            report["producer"]["source_paths"].as_array().unwrap().len(),
            producer_sources().len()
        );
        assert_eq!(
            report["producer"]["source_files"].as_array().unwrap().len(),
            producer_sources().len()
        );
        assert_eq!(report["summary"]["verified_field_and_map_pilot_maps"], 5);
        assert_eq!(report["summary"]["unsupported_extension_degree"], 1_342);
        assert_eq!(
            report["summary"]["unsupported_outside_pilot_base_prime"],
            58
        );
        assert_eq!(report["summary"]["general_jacobian_transfer_supported"], 0);
        assert_eq!(
            report["unsupported_records"].as_array().unwrap().len(),
            1_400
        );
        assert!(report["unsupported_records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| matches!(
                row["status"].as_str(),
                Some("unsupported_extension_degree") | Some("unsupported_outside_pilot_base_prime")
            )));
        for record in report["records"].as_array().unwrap() {
            assert_eq!(record["counts"]["named_ec1_bindings"], 0);
            assert_eq!(record["counts"]["source_infinity_branches"], 2);
            assert_eq!(
                record["statuses"]["source_smoothness"],
                "verified_sextic_squarefree_by_polynomial_gcd"
            );
            assert_eq!(
                record["statuses"]["source_genus"],
                "verified_genus_two_from_squarefree_degree_six_in_odd_characteristic"
            );
            assert_eq!(
                record["statuses"]["infinity_behavior"],
                "verified_two_rational_source_branches_each_unramified_over_target_infinity"
            );
            assert_eq!(
                record["statuses"]["formal_curve_fiber_degree_identity"],
                "verified_formal_degree_two_fibers_and_target_2P_without_source_class_reduction"
            );
            assert_eq!(
                record["statuses"]["general_jacobian_arithmetic"],
                "unsupported_not_implemented_by_pilot"
            );
        }

        let uid = EXPECTED_MAP_UIDS[0];
        let wrapper = &graph["maps"][uid];
        let target: Value =
            serde_json::from_str(wrapper["record"]["target_model_json"].as_str().unwrap()).unwrap();
        let mut changed_wrapper = wrapper.clone();
        changed_wrapper["record"]["x"] = json!(["0x1"]);
        assert!(
            verify_row(&graph, &registry, uid, &changed_wrapper, &target)
                .unwrap_err()
                .contains("identity mismatch")
        );

        let mut changed_wrapper_id = wrapper.clone();
        changed_wrapper_id["map_id"] = json!("CV1D2h000000000000");
        assert!(
            verify_row(&graph, &registry, uid, &changed_wrapper_id, &target)
                .unwrap_err()
                .contains("map wrapper identity mismatch")
        );

        let source_uid = wrapper["record"]["source_cover_uid"].as_str().unwrap();
        let mut changed_source_wrapper = graph.clone();
        changed_source_wrapper["covers"][source_uid]["cover_id"] = json!("HC1G2h000000000000");
        assert!(
            verify_row(&changed_source_wrapper, &registry, uid, wrapper, &target)
                .unwrap_err()
                .contains("source wrapper identity mismatch")
        );

        let mut stale_registry_bytes = registry_bytes.clone();
        stale_registry_bytes.push(b'\n');
        assert!(build_report(&graph_bytes, &stale_registry_bytes)
            .unwrap_err()
            .contains("schema/digest do not match"));

        let unsupported_uid = graph["maps"]
            .as_object()
            .unwrap()
            .iter()
            .find_map(|(candidate, wrapper)| {
                let target: Value =
                    serde_json::from_str(wrapper["record"]["target_model_json"].as_str()?).ok()?;
                (target.get("k").and_then(Value::as_str).is_some()
                    && !EXPECTED_MAP_UIDS.contains(&candidate.as_str()))
                .then(|| candidate.clone())
            })
            .unwrap();
        let mut changed_unsupported = graph.clone();
        changed_unsupported["maps"][unsupported_uid.as_str()]["map_id"] =
            json!("CV1D2h000000000000");
        let changed_unsupported_bytes = serde_json::to_vec(&changed_unsupported).unwrap();
        assert!(build_report(&changed_unsupported_bytes, &registry_bytes)
            .unwrap_err()
            .contains("not the exact deterministic registry graph"));
    }

    #[test]
    fn producer_manifest_binds_the_exact_transitive_local_source_closure() {
        let expected_paths = vec![
            "Cargo.lock",
            "Cargo.toml",
            "src/bin/curve_cover_check.rs",
            "src/bin/curve_cover_check/checker.rs",
            "src/bin/curve_cover_check/links.rs",
            "src/bin/curve_cover_check/models.rs",
            "src/bin/odd_extension_transfer_pilot.rs",
            "src/binary_ecc/f2m.rs",
            "src/binary_ecc/mod.rs",
            "src/binary_ecc/operation_tally.rs",
            "src/cryptanalysis/bitops.rs",
            "src/cryptanalysis/ecc2k130_guard.rs",
            "src/cryptanalysis/mod.rs",
            "src/ct_bignum.rs",
            "src/hash/mod.rs",
            "src/hash/sha256.rs",
            "src/lib.rs",
            "src/prime_hyperelliptic/fp2.rs",
            "src/prime_hyperelliptic/mod.rs",
            "src/utils/mod.rs",
        ];
        let sources = producer_sources();
        assert_eq!(producer_source_paths(), expected_paths);
        assert!(sources.iter().all(|(_, contents)| !contents.is_empty()));
        let files = producer_source_files();
        for (file, (path, contents)) in files.iter().zip(&sources) {
            assert_eq!(file["path"], *path);
            assert_eq!(file["sha256"], sha(contents.as_bytes()));
        }
        assert_eq!(producer_source_digest(), framed_source_digest(&sources));
    }

    #[test]
    fn basis_mismatch_and_malformed_fields_fail_closed() {
        assert_eq!(
            CatalogK2::try_new(11, &[1, 0]).unwrap_err(),
            FieldError::UnsupportedBasePrime(11)
        );
        assert_eq!(
            CatalogK2::try_new(5, &[5, 0]).unwrap_err(),
            FieldError::InvalidModulus
        );
        assert_eq!(
            CatalogK2::try_new(5, &[1, 0]).unwrap_err(),
            FieldError::ReducibleModulus
        );
        assert_eq!(
            CatalogK2::try_new(5, &[2, 1]).unwrap_err(),
            FieldError::UnsupportedLinearTerm
        );
        let field = CatalogK2::try_new(5, &[2, 0]).unwrap();
        // The catalog uses t^2=3 while Fp2Ctx uses s^2=2: direct coefficient
        // reuse is wrong, and the proved adapter is t -> 2*s.
        assert_eq!(field.reference.nonresidue(), 2);
        assert_eq!(field.reference_scale, 2);
        assert_ne!(
            field.reference.square(Fp2 { a: 0, b: 1 }),
            field.to_reference(field.square(K2 { a: 0, b: 1 }))
        );
        assert_eq!(
            field
                .reference
                .square(field.to_reference(K2 { a: 0, b: 1 })),
            field.to_reference(field.square(K2 { a: 0, b: 1 }))
        );
        assert_eq!(
            field.element(5, 0).unwrap_err(),
            FieldError::NoncanonicalElement
        );
        assert_eq!(
            field.decode_packed(25).unwrap_err(),
            FieldError::NoncanonicalElement
        );
    }
}
