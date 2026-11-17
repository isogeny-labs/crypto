//! Independent replay for the existing, tiny public APS fixture transcripts.
//!
//! Uses the separate BigUint EC implementation, never the producer's fast
//! arithmetic, polynomial solver, sampler or recovery routines. Complete sets
//! are checked by point enumeration; partial enumeration stays partial. Shared
//! conventions are serde, rand 0.8 StdRng and ICV1 identity construction.
//! Solver diagnostics and execution costs are explicitly NOT attested.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    fs::File,
    io::Read,
    path::Path,
};

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{de, Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::ecc::{field::FieldElement, point::Point};

/// The producer only emits the three existing public fixtures.
pub const MAX_INPUT_BYTES: usize = 2 * 1024 * 1024;
const MAX_SETS_PER_ATTEMPT: usize = 2048;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Fixture {
    Toy101,
    Toy211,
    Toy103J0,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Arm {
    Partitioned3,
    Unordered3,
    Line3,
    Expanded4,
    Split4,
    BlockSymmetric4,
    Hybrid3,
    Hybrid4,
    Cover3,
    Orbit3,
    OrbitLine3,
    OrbitPrunedLine3,
}
impl Arm {
    fn arity(self) -> usize {
        match self {
            Self::Expanded4 | Self::Split4 | Self::BlockSymmetric4 | Self::Hybrid4 => 4,
            _ => 3,
        }
    }
    fn orbit(self) -> bool {
        matches!(
            self,
            Self::Orbit3 | Self::OrbitLine3 | Self::OrbitPrunedLine3
        )
    }
    fn partitioned(self) -> bool {
        matches!(
            self,
            Self::Partitioned3
                | Self::Expanded4
                | Self::Split4
                | Self::Hybrid3
                | Self::Hybrid4
                | Self::Orbit3
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Labels {
    Random,
    Guaranteed,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Verified,
    Incomplete,
    NoInformativeRelation,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Transcript {
    schema: String,
    fixture: Fixture,
    arm: Arm,
    labels: Labels,
    subset_size: u8,
    seed: u64,
    attempt_cap: u8,
    solver_budget_ms_per_attempt: u64,
    #[serde(default)]
    first_informative: bool,
    curve_icv1: String,
    curve_model: Model,
    generator: (u64, u64),
    target: (u64, u64),
    expected_public_logarithm: u64,
    status: Outcome,
    attempts: Vec<Attempt>,
    limits: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Model {
    a: String,
    b: String,
    field: String,
    form: String,
    p: String,
    v: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Entry {
    x: u64,
    y: u64,
    u: u64,
    v: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Base {
    parts: Vec<Vec<u64>>,
    #[serde(deserialize_with = "unique_points")]
    points: BTreeMap<u64, Entry>,
    draws: usize,
}

// Ordinary serde maps overwrite duplicate JSON keys. A proof input must not
// have two interpretations, including aliases such as "01" and "1".
fn unique_points<'de, D: Deserializer<'de>>(d: D) -> Result<BTreeMap<u64, Entry>, D::Error> {
    struct Points;
    impl<'de> de::Visitor<'de> for Points {
        type Value = BTreeMap<u64, Entry>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("distinct canonical decimal coordinate keys")
        }
        fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut points = BTreeMap::new();
            while let Some(key) = map.next_key::<String>()? {
                let x = key.parse::<u64>().map_err(de::Error::custom)?;
                if key != x.to_string() || points.contains_key(&x) || points.len() >= 32 {
                    return Err(de::Error::custom(
                        "duplicate/noncanonical key or oversized base",
                    ));
                }
                points.insert(x, map.next_value()?);
            }
            Ok(points)
        }
    }
    d.deserialize_map(Points)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Attempt {
    base: Base,
    algebra: Algebra,
    recovery: Option<Recovery>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Algebra {
    complete: bool,
    #[serde(default)]
    stopped_early: bool,
    coordinate_sets: Vec<Vec<u64>>,
    timed_out: bool,
    // These fields are parsed strictly for schema compatibility, not trusted
    // as a cost certificate. Their underscore names make that boundary clear.
    #[serde(rename = "solver_calls")]
    _solver_calls: usize,
    #[serde(rename = "solving_degree")]
    _solving_degree: u32,
    #[serde(rename = "max_rows")]
    _max_rows: usize,
    #[serde(rename = "max_columns")]
    _max_columns: usize,
    #[serde(rename = "f4_field_multiplications")]
    _f4_field_multiplications: u64,
    #[serde(default, rename = "roots")]
    _roots: Option<Roots>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Roots {
    #[serde(rename = "equation_roots_visited")]
    _equation_roots_visited: usize,
    #[serde(rename = "candidates_rejected")]
    _candidates_rejected: usize,
    #[serde(rename = "domain_evaluations")]
    _domain_evaluations: u64,
    #[serde(rename = "factored_univariates")]
    _factored_univariates: usize,
    #[serde(rename = "root_field_multiplications")]
    _root_field_multiplications: u64,
    #[serde(rename = "full_field_branches")]
    _full_field_branches: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Recovery {
    logarithm: u64,
    coordinates: Vec<u64>,
    signs: Vec<i8>,
    a: u64,
    b: u64,
}

/// A hash-bound arithmetic replay receipt, not a timing/cost or execution attestation.
#[derive(Debug, Serialize)]
pub struct ReplayReport {
    pub schema: &'static str,
    pub input_sha256: String,
    pub outcome: Outcome,
    pub attempts_checked: usize,
    pub sampled_draws_checked: usize,
    pub labeled_points_checked: usize,
    pub coordinate_sets_checked: usize,
    pub complete_attempts_checked: usize,
    pub verified_logarithm: Option<u64>,
    pub arithmetic: &'static str,
    pub costs_attested: bool,
    pub execution_history_attested: bool,
}

struct Reference {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    d: u64,
    g: Point,
    q: Point,
    a_fe: FieldElement,
}
impl Reference {
    fn new(fixture: Fixture) -> Self {
        let (p, a, b, n, gx, gy, d): (u64, u64, u64, u64, u64, u64, u64) = match fixture {
            Fixture::Toy101 => (101, 59, 6, 107, 78, 4, 37),
            Fixture::Toy211 => (211, 166, 158, 227, 155, 121, 73),
            Fixture::Toy103J0 => (103, 0, 5, 97, 2, 42, 17),
        };
        let a_fe = FieldElement::new(BigUint::from(a), BigUint::from(p));
        let g = Point::Affine {
            x: FieldElement::new(BigUint::from(gx), BigUint::from(p)),
            y: FieldElement::new(BigUint::from(gy), BigUint::from(p)),
        };
        let q = reference_mul(&g, d, &a_fe);
        Self {
            p,
            a,
            b,
            n,
            d,
            g,
            q,
            a_fe,
        }
    }
    fn point(&self, x: u64, y: u64) -> Result<Point, String> {
        if x >= self.p || y >= self.p {
            return Err("noncanonical point coordinate".into());
        }
        let f = |v| FieldElement::new(BigUint::from(v), BigUint::from(self.p));
        let (x, y) = (f(x), f(y));
        if y.mul(&y) != x.mul(&x).mul(&x).add(&self.a_fe.mul(&x)).add(&f(self.b)) {
            return Err("point is not on the fixture curve".into());
        }
        Ok(Point::Affine { x, y })
    }
    fn check_instance(&self, t: &Transcript) -> Result<(), String> {
        let m = &t.curve_model;
        if m.p != self.p.to_string()
            || m.a != self.a.to_string()
            || m.b != self.b.to_string()
            || m.field != format!("fp-{}", self.p)
            || m.form != "y^2=x^3+a*x+b"
            || m.v != "1"
            || t.expected_public_logarithm != self.d
        {
            return Err("fixture/model/public-answer mismatch".into());
        }
        let g = self.point(t.generator.0, t.generator.1)?;
        let q = self.point(t.target.0, t.target.1)?;
        if g != self.g
            || q != self.q
            || reference_mul(&g, self.n, &self.a_fe) != Point::Infinity
            || reference_mul(&q, self.n, &self.a_fe) != Point::Infinity
        {
            return Err("fixture point/subgroup mismatch".into());
        }
        let id = crate::cryptanalysis::curve_id::prime(
            &BigUint::from(self.p),
            &BigUint::from(self.a),
            &BigUint::from(self.b),
            &BigUint::from(self.n),
        )
        .ok_or("fixture identity unavailable")?;
        if t.curve_icv1 != id.icv1 {
            return Err("fixture curve identity mismatch".into());
        }
        Ok(())
    }
    fn entry(&self, point: &Point, u: u64, v: u64) -> Entry {
        let (x, mut y) = coordinates(point).expect("only finite sampled points are labeled");
        // The producer compares the Montgomery *representation* of y with
        // (p-1)/2, not the affine integer y. Recompute that convention with
        // BigUint, without importing Fp64 or FastCurve.
        let mont_y = (BigUint::from(y) * (BigUint::from(1u8) << 64usize)) % BigUint::from(self.p);
        let flip = mont_y > BigUint::from(self.p / 2);
        if flip {
            y = (self.p - y) % self.p;
        }
        Entry {
            x,
            y,
            u: if flip { (self.n - u) % self.n } else { u },
            v: if flip { (self.n - v) % self.n } else { v },
        }
    }
    fn check_labels(&self, base: &Base) -> Result<(), String> {
        for (&key, entry) in &base.points {
            if key != entry.x || entry.u >= self.n || entry.v >= self.n {
                return Err("noncanonical labeled point".into());
            }
            let point = self.point(entry.x, entry.y)?;
            let represented = reference_mul(&self.g, entry.u, &self.a_fe)
                .add(&reference_mul(&self.q, entry.v, &self.a_fe), &self.a_fe);
            if represented != point {
                return Err("independent point-label equation failed".into());
            }
        }
        Ok(())
    }
    fn sample(&self, t: &Transcript, index: usize, draw_cap: usize) -> Result<Base, String> {
        let mut rng = StdRng::seed_from_u64(t.seed.wrapping_add(index as u64));
        let mut base = Base {
            parts: vec![vec![]; t.arm.arity()],
            points: BTreeMap::new(),
            draws: 0,
        };
        for slot in 0..t.arm.arity() {
            while base.parts[slot].len() < usize::from(t.subset_size) {
                if base.draws >= draw_cap || base.draws >= 10_000 {
                    return Err("sampling/draw count mismatch".into());
                }
                let u = rng.gen_range(0..self.n);
                let v = match t.labels {
                    Labels::Random => rng.gen_range(0..self.n),
                    Labels::Guaranteed if t.arm.arity() == 3 => 1,
                    Labels::Guaranteed => 1 << slot,
                };
                let point = reference_mul(&self.g, u, &self.a_fe)
                    .add(&reference_mul(&self.q, v, &self.a_fe), &self.a_fe);
                base.draws += 1;
                if point == Point::Infinity {
                    continue;
                }
                let entry = self.entry(&point, u, v);
                if base.points.contains_key(&entry.x) {
                    continue;
                }
                base.parts[slot].push(entry.x);
                base.points.insert(entry.x, entry);
            }
        }
        if !t.arm.orbit() {
            return Ok(base);
        }
        let mut expanded = Base {
            parts: vec![vec![]; base.parts.len()],
            points: BTreeMap::new(),
            draws: base.draws,
        };
        // Only the existing public j=0 fixture is admitted by header checks.
        for (slot, part) in base.parts.iter().enumerate() {
            for x in part {
                let entry = &base.points[x];
                let (mut point, mut u, mut v) = (self.point(entry.x, entry.y)?, entry.u, entry.v);
                for _ in 0..3 {
                    let entry = self.entry(&point, u, v);
                    if expanded.points.contains_key(&entry.x) {
                        return Err("short/overlapping sampled orbit".into());
                    }
                    expanded.parts[slot].push(entry.x);
                    expanded.points.insert(entry.x, entry);
                    let (x, y) = coordinates(&point).ok_or("orbit reached infinity")?;
                    point = self.point((56 * x) % self.p, y)?;
                    u = (35 * u) % self.n;
                    v = (35 * v) % self.n;
                }
            }
        }
        Ok(expanded)
    }
}

// Affine double-and-add using the existing BigUint add/double formulas. This
// deliberately avoids both FastCurve and the optimized word/Jacobian paths.
fn reference_mul(point: &Point, mut scalar: u64, a: &FieldElement) -> Point {
    let mut result = Point::Infinity;
    let mut power = point.clone();
    while scalar > 0 {
        if scalar & 1 == 1 {
            result = result.add(&power, a);
        }
        scalar >>= 1;
        if scalar > 0 {
            power = power.double(a);
        }
    }
    result
}

fn coordinates(point: &Point) -> Option<(u64, u64)> {
    match point {
        Point::Infinity => None,
        Point::Affine { x, y } => Some((x.value.to_u64()?, y.value.to_u64()?)),
    }
}

fn allowed(arm: Arm, base: &Base, xs: &[u64]) -> bool {
    let counts: Vec<_> = base
        .parts
        .iter()
        .map(|part| xs.iter().filter(|x| part.contains(x)).count())
        .collect();
    if arm.partitioned() {
        return counts.iter().all(|&count| count == 1);
    }
    if arm == Arm::BlockSymmetric4 {
        return counts[..2].iter().sum::<usize>() == 2 && counts[2..].iter().sum::<usize>() == 2;
    }
    true
}

struct Relation {
    informative: bool,
}

fn signed_sum(
    r: &Reference,
    base: &Base,
    xs: &[u64],
    signs: &[i8],
) -> Result<(Point, u64, u64, bool), String> {
    if signs.len() != xs.len()
        || signs.first() != Some(&1)
        || signs.iter().any(|&s| s != 1 && s != -1)
    {
        return Err("invalid signed-relation orientation".into());
    }
    let (mut sum, mut a, mut b) = (Point::Infinity, 0, 0);
    let mut ys = BTreeSet::new();
    for (x, sign) in xs.iter().zip(signs) {
        let entry = base
            .points
            .get(x)
            .ok_or("relation coordinate outside base")?;
        let point = r.point(entry.x, entry.y)?;
        let point = if *sign == 1 { point } else { point.neg() };
        ys.insert(
            coordinates(&point)
                .ok_or("finite relation point required")?
                .1,
        );
        sum = sum.add(&point, &r.a_fe);
        a = (a + if *sign == 1 {
            entry.u
        } else {
            (r.n - entry.u) % r.n
        }) % r.n;
        b = (b + if *sign == 1 {
            entry.v
        } else {
            (r.n - entry.v) % r.n
        }) % r.n;
    }
    Ok((sum, a, b, ys.len() == 1))
}

fn relation(r: &Reference, arm: Arm, base: &Base, xs: &[u64]) -> Result<Option<Relation>, String> {
    if xs.len() != arm.arity()
        || xs.windows(2).any(|w| w[0] >= w[1])
        || xs.iter().any(|x| !base.points.contains_key(x))
        || !allowed(arm, base, xs)
    {
        return Err("invalid coordinate tuple/partition membership".into());
    }
    let mut found = None;
    for mask in 0..(1usize << (xs.len() - 1)) {
        let signs: Vec<_> = (0..xs.len())
            .map(|i| {
                if i == 0 || mask & (1 << (i - 1)) == 0 {
                    1
                } else {
                    -1
                }
            })
            .collect();
        let (sum, _, b, horizontal) = signed_sum(r, base, xs, &signs)?;
        if sum == Point::Infinity && !(arm == Arm::OrbitPrunedLine3 && horizontal) {
            let result = found.get_or_insert(Relation { informative: false });
            result.informative |= b != 0;
        }
    }
    Ok(found)
}

fn combinations(values: &[u64], arity: usize) -> Vec<Vec<u64>> {
    fn extend(values: &[u64], arity: usize, prefix: &mut Vec<u64>, out: &mut Vec<Vec<u64>>) {
        if prefix.len() == arity {
            out.push(prefix.clone());
            return;
        }
        if values.len() < arity - prefix.len() {
            return;
        }
        for (i, value) in values.iter().enumerate() {
            prefix.push(*value);
            extend(&values[i + 1..], arity, prefix, out);
            prefix.pop();
        }
    }
    let mut result = vec![];
    extend(values, arity, &mut vec![], &mut result);
    result
}

fn check_recovery(r: &Reference, arm: Arm, a: &Attempt) -> Result<Option<u64>, String> {
    let Some(w) = &a.recovery else {
        return Ok(None);
    };
    if !a.algebra.coordinate_sets.contains(&w.coordinates)
        || w.logarithm >= r.n
        || w.a >= r.n
        || w.b >= r.n
    {
        return Err("recovery does not bind to recorded coordinates/canonical scalars".into());
    }
    let (sum, aa, bb, horizontal) = signed_sum(r, &a.base, &w.coordinates, &w.signs)?;
    if sum != Point::Infinity
        || aa != w.a
        || bb != w.b
        || bb == 0
        || (arm == Arm::OrbitPrunedLine3 && horizontal)
    {
        return Err("invalid or uninformative signed recovery".into());
    }
    let n = BigUint::from(r.n);
    let quotient = FieldElement::new(BigUint::from(bb), n.clone())
        .inv()
        .ok_or("noninvertible relation coefficient")?;
    let d = ((&n - BigUint::from(aa)) * quotient.value) % &n;
    if d != BigUint::from(w.logarithm) || reference_mul(&r.g, w.logarithm, &r.a_fe) != r.q {
        return Err("recovered logarithm failed independent verification".into());
    }
    Ok(Some(w.logarithm))
}

/// Strictly parse and independently replay a bounded public-fixture transcript.
pub fn verify_bytes(bytes: &[u8]) -> Result<ReplayReport, String> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("transcript exceeds input byte limit".into());
    }
    let t: Transcript = serde_json::from_slice(bytes).map_err(|e| {
        format!(
            "invalid transcript: {}",
            e.to_string().chars().take(512).collect::<String>()
        )
    })?;
    if t.schema != "aps.single-relation.correctness/v1"
        || !(1..=4).contains(&t.subset_size)
        || !(1..=128).contains(&t.attempt_cap)
        || !(1..=60_000).contains(&t.solver_budget_ms_per_attempt)
        || t.attempts.is_empty()
        || t.attempts.len() > usize::from(t.attempt_cap)
        || t.limits.len() > 4096
        || (t.arm.orbit() && (t.fixture != Fixture::Toy103J0 || t.subset_size != 1))
    {
        return Err("unsupported schema/fixture bounds".into());
    }
    let r = Reference::new(t.fixture);
    r.check_instance(&t)?;
    let mut report = ReplayReport {
        schema: "aps.single-relation.replay/v1",
        input_sha256: format!("{:x}", Sha256::digest(bytes)),
        outcome: t.status,
        attempts_checked: 0,
        sampled_draws_checked: 0,
        labeled_points_checked: 0,
        coordinate_sets_checked: 0,
        complete_attempts_checked: 0,
        verified_logarithm: None,
        arithmetic: "existing BigUint affine add/double; independent of FastCurve/F4/recovery",
        costs_attested: false,
        execution_history_attested: false,
    };
    for (i, attempt) in t.attempts.iter().enumerate() {
        let check = || -> Result<(usize, bool, Option<u64>), String> {
            let expected = r.sample(&t, i, attempt.base.draws)?;
            if expected != attempt.base {
                return Err("sampled base/labels/draws mismatch".into());
            }
            r.check_labels(&attempt.base)?;
            let a = &attempt.algebra;
            if (a.complete && (a.stopped_early || a.timed_out))
                || (a.stopped_early && a.timed_out)
                || a.stopped_early != (t.first_informative && attempt.recovery.is_some())
                || a.coordinate_sets.len() > MAX_SETS_PER_ATTEMPT
                || a.coordinate_sets.windows(2).any(|w| w[0] >= w[1])
            {
                return Err("inconsistent completion/stop/timeout or coordinate ordering".into());
            }
            let mut informative = false;
            for xs in &a.coordinate_sets {
                let rel = relation(&r, t.arm, &attempt.base, xs)?
                    .ok_or("reported tuple is not a point relation")?;
                informative |= rel.informative;
            }
            if a.complete {
                let xs: Vec<_> = attempt.base.points.keys().copied().collect();
                let mut exact = BTreeSet::new();
                for tuple in combinations(&xs, t.arm.arity()) {
                    if allowed(t.arm, &attempt.base, &tuple)
                        && relation(&r, t.arm, &attempt.base, &tuple)?.is_some()
                    {
                        exact.insert(tuple);
                    }
                }
                if exact != a.coordinate_sets.iter().cloned().collect() {
                    return Err(
                        "complete solution set differs from independent point enumeration".into(),
                    );
                }
            }
            let recovered = check_recovery(&r, t.arm, attempt)?;
            if informative != recovered.is_some() {
                return Err("informative recorded relations/recovery disagree".into());
            }
            if recovered.is_some() && i + 1 != t.attempts.len() {
                return Err("attempts continue after a verified recovery".into());
            }
            Ok((a.coordinate_sets.len(), a.complete, recovered))
        };
        let (sets, complete, recovered) = check().map_err(|e| format!("attempt {i}: {e}"))?;
        report.attempts_checked += 1;
        report.sampled_draws_checked += attempt.base.draws;
        report.labeled_points_checked += attempt.base.points.len();
        report.coordinate_sets_checked += sets;
        report.complete_attempts_checked += usize::from(complete);
        report.verified_logarithm = recovered.or(report.verified_logarithm);
    }
    let actual = if report.verified_logarithm.is_some() {
        Outcome::Verified
    } else if t.attempts.iter().any(|a| !a.algebra.complete) {
        Outcome::Incomplete
    } else {
        Outcome::NoInformativeRelation
    };
    if actual != t.status
        || (actual != Outcome::Verified && t.attempts.len() != usize::from(t.attempt_cap))
    {
        return Err("status or unsuccessful attempt count disagrees with replay".into());
    }
    Ok(report)
}

/// Read at most the accepted byte limit, including when a file grows during reading.
pub fn verify_file(path: &Path) -> Result<ReplayReport, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = vec![];
    file.take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    verify_bytes(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::{aps_single_relation as producer, curve_id, prime_fast::FastCurve};
    use serde_json::{json, Value};
    use std::{sync::OnceLock, time::Duration};

    fn producer_curve(
        which: Fixture,
    ) -> (
        FastCurve,
        crate::cryptanalysis::prime_fast::Pt,
        crate::cryptanalysis::prime_fast::Pt,
    ) {
        let r = Reference::new(which);
        let curve = FastCurve::new("aps-replay-public-fixture", r.p, r.a, r.b, r.n);
        let (gx, gy) = coordinates(&r.g).unwrap();
        let g = curve.point(gx, gy);
        let q = curve.mul(Some(g), r.d).unwrap();
        (curve, g, q)
    }

    fn header(
        which: Fixture,
        arm: &str,
        labels: &str,
        seed: u64,
        cap: u8,
        attempts: Value,
    ) -> Value {
        let r = Reference::new(which);
        let id = curve_id::prime(&r.p.into(), &r.a.into(), &r.b.into(), &r.n.into()).unwrap();
        let recovered = attempts
            .as_array()
            .unwrap()
            .iter()
            .any(|a| !a["recovery"].is_null());
        let status = if recovered {
            "verified"
        } else {
            "no_informative_relation"
        };
        let fixture = match which {
            Fixture::Toy101 => "toy101",
            Fixture::Toy211 => "toy211",
            Fixture::Toy103J0 => "toy103_j0",
        };
        json!({
            "schema":"aps.single-relation.correctness/v1", "fixture":fixture, "arm":arm,
            "labels":labels, "subset_size":2, "seed":seed, "attempt_cap":cap,
            "solver_budget_ms_per_attempt":10000, "first_informative":false,
            "curve_icv1":id.icv1, "curve_model":serde_json::from_str::<Value>(&id.model_json).unwrap(),
            "generator":coordinates(&r.g).unwrap(), "target":coordinates(&r.q).unwrap(),
            "expected_public_logarithm":r.d, "status":status, "attempts":attempts,
            "limits":"Fixed public test fixture, not a performance sample."
        })
    }

    fn valid() -> Value {
        static VALUE: OnceLock<Value> = OnceLock::new();
        VALUE
            .get_or_init(|| {
                let (curve, g, q) = producer_curve(Fixture::Toy101);
                let attempts = producer::solve_dlp(
                    &curve,
                    g,
                    q,
                    producer::Encoding::Line3,
                    producer::Labels::Guaranteed,
                    2,
                    20261026,
                    32,
                    Duration::from_secs(10),
                )
                .unwrap();
                assert!(attempts.last().unwrap().recovery.is_some());
                header(
                    Fixture::Toy101,
                    "line3",
                    "guaranteed",
                    20261026,
                    32,
                    serde_json::to_value(attempts).unwrap(),
                )
            })
            .clone()
    }

    fn check(value: &Value) -> Result<ReplayReport, String> {
        verify_bytes(&serde_json::to_vec(value).unwrap())
    }

    #[test]
    fn replays_all_public_fixtures_with_both_label_policies_and_wrapping_seeds() {
        for which in [Fixture::Toy101, Fixture::Toy211, Fixture::Toy103J0] {
            let (curve, g, q) = producer_curve(which);
            let r = Reference::new(which);
            for labels in [Labels::Random, Labels::Guaranteed] {
                for seed in [0, 20261026, u64::MAX] {
                    let mut t: Transcript = serde_json::from_value(valid()).unwrap();
                    t.fixture = which;
                    t.labels = labels;
                    t.seed = seed;
                    let policy = match labels {
                        Labels::Random => producer::Labels::Random,
                        Labels::Guaranteed => producer::Labels::Guaranteed,
                    };
                    for i in 0..2 {
                        let produced = producer::sample_base(
                            &curve,
                            g,
                            q,
                            &[2, 2, 2],
                            policy,
                            seed.wrapping_add(i as u64),
                            10_000,
                        )
                        .unwrap();
                        let expected: Base =
                            serde_json::from_value(serde_json::to_value(&produced).unwrap())
                                .unwrap();
                        assert_eq!(r.sample(&t, i, expected.draws).unwrap(), expected);
                        r.check_labels(&expected).unwrap();
                    }
                }
            }
        }
    }

    #[test]
    fn complete_producer_transcript_replays_with_exact_byte_binding() {
        let bytes = serde_json::to_vec_pretty(&valid()).unwrap();
        let report = verify_bytes(&bytes).unwrap();
        assert_eq!(report.outcome, Outcome::Verified);
        assert_eq!(report.verified_logarithm, Some(37));
        assert_eq!(report.complete_attempts_checked, report.attempts_checked);
        assert!(report.coordinate_sets_checked > 0);
        assert_eq!(report.input_sha256, format!("{:x}", Sha256::digest(&bytes)));
        assert!(!report.costs_attested);
        assert!(!report.execution_history_attested);
        let compact = serde_json::to_vec(&valid()).unwrap();
        assert_ne!(
            report.input_sha256,
            verify_bytes(&compact).unwrap().input_sha256
        );
    }

    #[test]
    fn rejects_label_point_seed_and_draw_corruption() {
        for field in ["u", "v", "x", "y"] {
            let mut value = valid();
            let entry = value["attempts"][0]["base"]["points"]
                .as_object_mut()
                .unwrap()
                .values_mut()
                .next()
                .unwrap();
            entry[field] = json!(entry[field].as_u64().unwrap() + 1);
            assert!(check(&value).is_err(), "{field}");
        }
        let mut value = valid();
        value["seed"] = json!(20261027);
        assert!(check(&value).unwrap_err().contains("sampled base"));
        let mut value = valid();
        let draws = value["attempts"][0]["base"]["draws"].as_u64().unwrap();
        value["attempts"][0]["base"]["draws"] = json!(draws + 1);
        assert!(check(&value).unwrap_err().contains("draws mismatch"));
    }

    #[test]
    fn rejects_fixture_identity_model_point_and_expected_answer_substitution() {
        for pointer in [
            "/curve_icv1",
            "/curve_model/p",
            "/curve_model/a",
            "/curve_model/b",
            "/curve_model/v",
        ] {
            let mut value = valid();
            *value.pointer_mut(pointer).unwrap() = json!("wrong");
            assert!(check(&value).is_err(), "{pointer}");
        }
        for pointer in ["/generator/0", "/target/1", "/expected_public_logarithm"] {
            let mut value = valid();
            *value.pointer_mut(pointer).unwrap() = json!(999999);
            assert!(check(&value).is_err(), "{pointer}");
        }
    }

    #[test]
    fn incomplete_coordinate_prefix_stays_incomplete() {
        let mut value = valid();
        let count = value["attempts"].as_array().unwrap().len();
        let last = &mut value["attempts"][count - 1];
        last["algebra"]["coordinate_sets"] = json!([]);
        last["algebra"]["complete"] = json!(false);
        last["recovery"] = Value::Null;
        value["status"] = json!("incomplete");
        value["attempt_cap"] = json!(count);
        let report = check(&value).unwrap();
        assert_eq!(report.outcome, Outcome::Incomplete);
        assert_eq!(report.verified_logarithm, None);
        assert_eq!(report.complete_attempts_checked + 1, count);
    }

    #[test]
    fn omitted_solutions_cannot_be_reported_as_complete_even_without_a_witness() {
        let mut value = valid();
        let count = value["attempts"].as_array().unwrap().len();
        let last = &mut value["attempts"][count - 1];
        last["algebra"]["coordinate_sets"] = json!([]);
        last["recovery"] = Value::Null;
        value["status"] = json!("no_informative_relation");
        value["attempt_cap"] = json!(count);
        assert!(check(&value)
            .unwrap_err()
            .contains("complete solution set differs"));
    }

    #[test]
    fn rejects_bad_recovery_even_when_the_public_answer_is_unchanged() {
        for field in ["a", "b", "logarithm"] {
            let mut value = valid();
            let witness = value["attempts"]
                .as_array_mut()
                .unwrap()
                .last_mut()
                .unwrap()
                .get_mut("recovery")
                .unwrap();
            witness[field] = json!((witness[field].as_u64().unwrap() + 1) % 107);
            assert!(check(&value).is_err(), "{field}");
        }
        for sign in [0, 2, -2] {
            let mut value = valid();
            value["attempts"]
                .as_array_mut()
                .unwrap()
                .last_mut()
                .unwrap()["recovery"]["signs"][1] = json!(sign);
            assert!(check(&value).unwrap_err().contains("orientation"));
        }
        let mut value = valid();
        value["attempts"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap()["recovery"]["signs"] = json!([1]);
        assert!(check(&value).unwrap_err().contains("orientation"));
    }

    #[test]
    fn rejects_false_status_truncated_retries_and_conflicting_completion_flags() {
        let mut value = valid();
        value["status"] = json!("no_informative_relation");
        assert!(check(&value).is_err());
        let mut value = valid();
        value["attempts"][0]["algebra"]["timed_out"] = json!(true);
        assert!(check(&value).unwrap_err().contains("completion"));
        let mut value = valid();
        value["attempts"][0]["algebra"]["stopped_early"] = json!(true);
        assert!(check(&value).unwrap_err().contains("completion"));
        let mut value = valid();
        let first = value["attempts"][0].clone();
        value["attempts"] = json!([first]);
        value["attempts"][0]["algebra"]["coordinate_sets"] = json!([]);
        value["attempts"][0]["algebra"]["complete"] = json!(false);
        value["attempts"][0]["recovery"] = Value::Null;
        value["status"] = json!("incomplete");
        assert!(check(&value).unwrap_err().contains("attempt count"));
    }

    #[test]
    fn rejects_unknown_fields_duplicate_fields_and_coordinate_key_aliases() {
        let mut value = valid();
        value["hidden_fallback"] = json!(true);
        assert!(check(&value).unwrap_err().contains("unknown field"));
        let mut value = valid();
        value["attempts"][0]["base"]["extra"] = json!(1);
        assert!(check(&value).unwrap_err().contains("unknown field"));
        let bytes = serde_json::to_string(&valid()).unwrap().replacen(
            "\"status\":",
            "\"status\":\"verified\",\"status\":",
            1,
        );
        assert!(verify_bytes(bytes.as_bytes())
            .unwrap_err()
            .contains("duplicate field"));
        let mut value = valid();
        let points = value["attempts"][0]["base"]["points"]
            .as_object_mut()
            .unwrap();
        let key = points.keys().next().unwrap().clone();
        let entry = points.remove(&key).unwrap();
        points.insert(format!("0{key}"), entry);
        assert!(check(&value).unwrap_err().contains("noncanonical key"));
        let value = valid();
        let points = value["attempts"][0]["base"]["points"].as_object().unwrap();
        let (key, entry) = points.iter().next().unwrap();
        let bytes = serde_json::to_string(&value).unwrap().replacen(
            "\"points\":{",
            &format!("\"points\":{{\"{key}\":{entry},"),
            1,
        );
        assert!(verify_bytes(bytes.as_bytes())
            .unwrap_err()
            .contains("duplicate/noncanonical"));
    }

    #[test]
    fn rejects_unsupported_schema_limits_and_oversized_input_before_arithmetic() {
        assert!(verify_bytes(&vec![b' '; MAX_INPUT_BYTES + 1])
            .unwrap_err()
            .contains("byte limit"));
        for (pointer, replacement) in [
            ("/schema", json!("other/v1")),
            ("/attempt_cap", json!(0)),
            ("/subset_size", json!(5)),
            ("/solver_budget_ms_per_attempt", json!(0)),
        ] {
            let mut value = valid();
            *value.pointer_mut(pointer).unwrap() = replacement;
            assert!(check(&value).is_err(), "{pointer}");
        }
    }

    #[test]
    fn solver_diagnostics_are_explicitly_not_cost_attestations() {
        let mut value = valid();
        for attempt in value["attempts"].as_array_mut().unwrap() {
            attempt["algebra"]["f4_field_multiplications"] = json!(0);
            attempt["algebra"]["solver_calls"] = json!(0);
        }
        let report = check(&value).unwrap();
        assert!(!report.costs_attested);
        assert!(!report.execution_history_attested);
    }

    #[test]
    fn complete_unsuccessful_attempt_is_checked_as_no_informative_relation() {
        let (curve, g, q) = producer_curve(Fixture::Toy101);
        let attempts = producer::solve_dlp(
            &curve,
            g,
            q,
            producer::Encoding::Partitioned3,
            producer::Labels::Guaranteed,
            2,
            717,
            1,
            Duration::from_secs(10),
        )
        .unwrap();
        assert!(attempts[0].recovery.is_none());
        let value = header(
            Fixture::Toy101,
            "partitioned3",
            "guaranteed",
            717,
            1,
            serde_json::to_value(attempts).unwrap(),
        );
        let report = check(&value).unwrap();
        assert_eq!(report.outcome, Outcome::NoInformativeRelation);
        assert_eq!(report.complete_attempts_checked, 1);
    }

    #[test]
    fn repeated_coordinates_and_partition_corruption_are_rejected() {
        let mut value = valid();
        let sets = &mut value["attempts"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap()["algebra"]["coordinate_sets"];
        sets[0][1] = sets[0][0].clone();
        assert!(check(&value).is_err());
        let mut value = valid();
        let x = value["attempts"][0]["base"]["parts"][0][0].clone();
        value["attempts"][0]["base"]["parts"][1][0] = x;
        assert!(check(&value).unwrap_err().contains("sampled base"));
    }

    #[test]
    fn horizontal_b_zero_orbit_is_rejected_even_with_the_correct_logarithm() {
        let (curve, g, q) = producer_curve(Fixture::Toy103J0);
        let seeds = producer::sample_base(
            &curve,
            g,
            q,
            &[1, 1, 1],
            producer::Labels::Random,
            20261026,
            10_000,
        )
        .unwrap();
        let base = producer::orbit_base(&curve, g, q, &seeds, 56, 35).unwrap();
        let values = base.points.keys().copied().collect::<Vec<_>>();
        let system = producer::build_system(&curve, &[values], producer::Encoding::Line3).unwrap();
        let algebra = producer::solve_system(&system, 103, 24, Duration::from_secs(10)).unwrap();
        let recovery = producer::recover(&curve, g, q, &base, &algebra.coordinate_sets).unwrap();
        let mut orbit = base.parts[0].clone();
        orbit.sort_unstable();
        let mut value = header(
            Fixture::Toy103J0,
            "orbit_line3",
            "random",
            20261026,
            1,
            serde_json::to_value([producer::Attempt {
                base,
                algebra,
                recovery,
            }])
            .unwrap(),
        );
        value["subset_size"] = json!(1);
        check(&value).unwrap();
        value["attempts"][0]["recovery"] =
            json!({"logarithm":17,"coordinates":orbit,"signs":[1,1,1],"a":0,"b":0});
        assert!(check(&value)
            .unwrap_err()
            .contains("uninformative signed recovery"));
    }
}
