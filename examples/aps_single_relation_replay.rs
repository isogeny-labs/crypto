//! Independent witness replay for the existing bounded APS fixture JSON.
//! Ordinary-residue affine arithmetic; no producer arithmetic or solver calls.
//! Completeness, sampling, deadlines and work counters are not certified.
use clap::Parser;
use serde::{de, Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::PathBuf;

const MAX_INPUT: usize = 8 * 1024 * 1024;
const MAX_SETS: usize = 4096;
const MAX_TOTAL_SETS: usize = 32768;

#[derive(Parser)]
#[command(about = "Independently check tiny public APS witnesses; no performance claims")]
struct Args {
    transcript: PathBuf,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Labels {
    Random,
    Guaranteed,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Model {
    a: String,
    b: String,
    field: String,
    form: String,
    p: String,
    v: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Transcript {
    schema: String,
    fixture: Fixture,
    arm: Arm,
    labels: Labels,
    subset_size: usize,
    #[serde(rename = "seed")]
    _seed: u64,
    attempt_cap: usize,
    solver_budget_ms_per_attempt: u64,
    first_informative: bool,
    curve_icv1: String,
    curve_model: Model,
    generator: [u64; 2],
    target: [u64; 2],
    expected_public_logarithm: u64,
    status: String,
    attempts: Vec<Attempt>,
    #[serde(rename = "limits")]
    _limits: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attempt {
    base: Base,
    algebra: Algebra,
    recovery: Option<Recovery>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Base {
    parts: Vec<Vec<u64>>,
    #[serde(deserialize_with = "unique_points")]
    points: BTreeMap<u64, LabelledPoint>,
    #[serde(rename = "draws")]
    _draws: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LabelledPoint {
    x: u64,
    y: u64,
    u: u64,
    v: u64,
}

// A normal BTreeMap deserializer silently replaces duplicate JSON keys.
fn unique_points<'de, D>(d: D) -> Result<BTreeMap<u64, LabelledPoint>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = BTreeMap<u64, LabelledPoint>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("unique canonical decimal abscissa keys and labelled points")
        }
        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut result = BTreeMap::new();
            while let Some((key, point)) = map.next_entry::<String, LabelledPoint>()? {
                let x = key.parse::<u64>().map_err(de::Error::custom)?;
                if key != x.to_string() || x != point.x || result.insert(x, point).is_some() {
                    return Err(de::Error::custom(
                        "duplicate or noncanonical labelled-point key",
                    ));
                }
                if result.len() > 32 {
                    return Err(de::Error::custom("factor base exceeds the producer cap"));
                }
            }
            Ok(result)
        }
    }
    d.deserialize_map(Visitor)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Algebra {
    complete: bool,
    stopped_early: bool,
    coordinate_sets: Vec<Vec<u64>>,
    timed_out: bool,
    #[serde(rename = "solver_calls")]
    _solver_calls: u64,
    #[serde(rename = "solving_degree")]
    _solving_degree: u64,
    #[serde(rename = "max_rows")]
    _max_rows: u64,
    #[serde(rename = "max_columns")]
    _max_columns: u64,
    #[serde(rename = "f4_field_multiplications")]
    _f4_field_multiplications: u64,
    #[serde(rename = "roots")]
    _roots: Option<RootDiagnostics>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RootDiagnostics {
    #[serde(rename = "equation_roots_visited")]
    _equation_roots_visited: u64,
    #[serde(rename = "candidates_rejected")]
    _candidates_rejected: u64,
    #[serde(rename = "domain_evaluations")]
    _domain_evaluations: u64,
    #[serde(rename = "factored_univariates")]
    _factored_univariates: u64,
    #[serde(rename = "root_field_multiplications")]
    _root_field_multiplications: u64,
    #[serde(rename = "full_field_branches")]
    _full_field_branches: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Recovery {
    logarithm: u64,
    coordinates: Vec<u64>,
    signs: Vec<i8>,
    a: u64,
    b: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}
type GroupPoint = Option<Point>;

#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    g: Point,
    public_log: u64,
}

fn product(a: u64, b: u64, modulus: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(modulus)) as u64
}
fn negative(a: u64, modulus: u64) -> u64 {
    (modulus - a % modulus) % modulus
}
fn power(mut a: u64, mut e: u64, modulus: u64) -> u64 {
    let mut value = 1;
    while e != 0 {
        if e & 1 != 0 {
            value = product(value, a, modulus);
        }
        a = product(a, a, modulus);
        e >>= 1;
    }
    value
}

impl Curve {
    fn fixture(fixture: Fixture) -> Self {
        let (p, a, b, n, x, y, public_log) = match fixture {
            Fixture::Toy101 => (101, 59, 6, 107, 78, 4, 37),
            Fixture::Toy211 => (211, 166, 158, 227, 155, 121, 73),
            Fixture::Toy103J0 => (103, 0, 5, 97, 2, 42, 17),
        };
        Self {
            p,
            a,
            b,
            n,
            g: Point { x, y },
            public_log,
        }
    }
    fn on_curve(self, q: Point) -> bool {
        q.x < self.p
            && q.y < self.p
            && product(q.y, q.y, self.p)
                == (product(product(q.x, q.x, self.p), q.x, self.p)
                    + product(self.a, q.x, self.p)
                    + self.b)
                    % self.p
    }
    fn negate(self, q: Point) -> Point {
        Point {
            x: q.x,
            y: negative(q.y, self.p),
        }
    }
    fn add(self, left: GroupPoint, right: GroupPoint) -> GroupPoint {
        let (Some(left), Some(right)) = (left, right) else {
            return left.or(right);
        };
        if left.x == right.x && (left.y + right.y) % self.p == 0 {
            return None;
        }
        let (numerator, denominator) = if left.x == right.x {
            (
                (3 * product(left.x, left.x, self.p) + self.a) % self.p,
                2 * left.y % self.p,
            )
        } else {
            (
                (right.y + self.p - left.y) % self.p,
                (right.x + self.p - left.x) % self.p,
            )
        };
        let slope = product(numerator, power(denominator, self.p - 2, self.p), self.p);
        let x = (product(slope, slope, self.p) + 2 * self.p - left.x - right.x) % self.p;
        let y = (product(slope, (left.x + self.p - x) % self.p, self.p) + self.p - left.y) % self.p;
        Some(Point { x, y })
    }
    fn scale(self, q: GroupPoint, mut k: u64) -> GroupPoint {
        let mut result = None;
        let mut current = q;
        while k != 0 {
            if k & 1 != 0 {
                result = self.add(result, current);
            }
            current = self.add(current, current);
            k >>= 1;
        }
        result
    }
    fn group_order(self) -> u64 {
        1 + (0..self.p)
            .map(|x| {
                (0..self.p)
                    .filter(|&y| self.on_curve(Point { x, y }))
                    .count() as u64
            })
            .sum::<u64>()
    }
    fn model(self) -> Model {
        Model {
            a: self.a.to_string(),
            b: self.b.to_string(),
            field: format!("fp-{}", self.p),
            form: "y^2=x^3+a*x+b".into(),
            p: self.p.to_string(),
            v: "1".into(),
        }
    }
    fn identity(self) -> String {
        let a3 = product(product(self.a, self.a, self.p), self.a, self.p);
        let disc = (4 * a3 + 27 * product(self.b, self.b, self.p)) % self.p;
        let j = product(
            product(1728 * 4 % self.p, a3, self.p),
            power(disc, self.p - 2, self.p),
            self.p,
        );
        let model = serde_json::to_string(&self.model()).expect("fixed string model");
        let digest = format!("{:x}", Sha256::digest(model.as_bytes()));
        let trace = self.p as i64 + 1 - self.n as i64;
        format!(
            "ICV1:fp-{}:{trace}:{}:{j}:unk:unk:r:{}",
            self.p,
            self.n,
            &digest[..12]
        )
    }
}

fn require(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn check_base(t: &Transcript, c: Curve, q: Point, base: &Base) -> Result<(), String> {
    let arity = t.arm.arity();
    let part_size = if t.arm.orbit() { 3 } else { t.subset_size };
    require(
        base.parts.len() == arity && base.parts.iter().all(|part| part.len() == part_size),
        "factor-base partition sizes disagree with the fixture request",
    )?;
    let coordinates: BTreeSet<_> = base.parts.iter().flatten().copied().collect();
    require(
        coordinates.len() == arity * part_size
            && coordinates.len() == base.points.len()
            && coordinates.iter().all(|x| base.points.contains_key(x)),
        "factor-base partitions and labelled points differ or repeat",
    )?;
    for (part_index, part) in base.parts.iter().enumerate() {
        for x in part {
            let entry = &base.points[x];
            let point = Point {
                x: entry.x,
                y: entry.y,
            };
            require(
                c.on_curve(point) && entry.u < c.n && entry.v < c.n,
                "noncanonical, off-curve or out-of-range labelled point",
            )?;
            require(
                c.add(c.scale(Some(c.g), entry.u), c.scale(Some(q), entry.v)) == Some(point),
                "label does not represent uG+vQ",
            )?;
            if t.labels == Labels::Guaranteed {
                let weights: Vec<_> = if t.arm.orbit() {
                    vec![1, 35, product(35, 35, c.n)]
                } else {
                    vec![if arity % 2 == 1 { 1 } else { 1 << part_index }]
                };
                require(
                    weights
                        .iter()
                        .any(|&w| entry.v == w || entry.v == negative(w, c.n)),
                    "guaranteed-label weight is invalid",
                )?;
            }
        }
    }
    if t.arm.orbit() {
        require(
            c.a == 0
                && power(56, 3, c.p) == 1
                && (product(35, 35, c.n) + 35 + 1).is_multiple_of(c.n),
            "invalid fixed orbit parameters",
        )?;
        for point in [c.g, q] {
            require(
                c.scale(Some(point), 35)
                    == Some(Point {
                        x: product(56, point.x, c.p),
                        y: point.y,
                    }),
                "orbit eigenvalue does not act on generator and target",
            )?;
        }
        for part in &base.parts {
            for x in part {
                let entry = &base.points[x];
                let next_x = product(56, *x, c.p);
                require(
                    next_x != *x && part.contains(&next_x),
                    "short or unclosed orbit",
                )?;
                let next = &base.points[&next_x];
                let flip = if next.y == entry.y {
                    false
                } else if next.y == negative(entry.y, c.p) {
                    true
                } else {
                    return Err("orbit ordinate mismatch".into());
                };
                let mut u = product(35, entry.u, c.n);
                let mut v = product(35, entry.v, c.n);
                if flip {
                    u = negative(u, c.n);
                    v = negative(v, c.n);
                }
                require(
                    next.u == u && next.v == v,
                    "orbit labels do not certify propagation or horizontal pruning",
                )?;
            }
        }
    }
    Ok(())
}

fn check_membership(arm: Arm, base: &Base, xs: &[u64]) -> Result<(), String> {
    require(
        xs.len() == arm.arity()
            && xs.windows(2).all(|w| w[0] < w[1])
            && xs.iter().all(|x| base.points.contains_key(x)),
        "reported coordinates are not distinct ordered factor-base members",
    )?;
    let per_part: Vec<_> = base
        .parts
        .iter()
        .map(|part| xs.iter().filter(|x| part.contains(x)).count())
        .collect();
    match arm {
        Arm::Partitioned3
        | Arm::Hybrid3
        | Arm::Orbit3
        | Arm::Expanded4
        | Arm::Split4
        | Arm::Hybrid4 => require(
            per_part.iter().all(|&count| count == 1),
            "coordinate violates partition membership",
        ),
        Arm::BlockSymmetric4 => require(
            per_part[0] + per_part[1] == 2 && per_part[2] + per_part[3] == 2,
            "coordinate violates two-block membership",
        ),
        _ => Ok(()),
    }
}

#[derive(Debug)]
struct Relation {
    signs: Vec<i8>,
    a: u64,
    b: u64,
}

fn relations(c: Curve, base: &Base, xs: &[u64], pruned: bool) -> Vec<Relation> {
    let mut found = Vec::new();
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
        let (mut sum, mut a, mut b) = (None, 0, 0);
        let mut ordinates = BTreeSet::new();
        for (&x, &sign) in xs.iter().zip(&signs) {
            let e = &base.points[&x];
            let point = Point { x, y: e.y };
            let point = if sign == 1 { point } else { c.negate(point) };
            ordinates.insert(point.y);
            sum = c.add(sum, Some(point));
            a = (a + if sign == 1 { e.u } else { negative(e.u, c.n) }) % c.n;
            b = (b + if sign == 1 { e.v } else { negative(e.v, c.n) }) % c.n;
        }
        if sum.is_none() && (!pruned || ordinates.len() > 1) {
            found.push(Relation { signs, a, b });
        }
    }
    found
}

fn check_attempt(t: &Transcript, c: Curve, q: Point, a: &Attempt) -> Result<usize, String> {
    check_base(t, c, q, &a.base)?;
    let alg = &a.algebra;
    require(
        !(alg.complete && (alg.stopped_early || alg.timed_out)
            || alg.stopped_early && alg.timed_out),
        "contradictory solver outcome flags",
    )?;
    require(
        !alg.stopped_early || t.first_informative && a.recovery.is_some(),
        "early-stop flag lacks a requested informative witness",
    )?;
    require(
        !(t.first_informative && a.recovery.is_some()) || alg.stopped_early,
        "streaming witness lacks its early-stop flag",
    )?;
    require(
        alg.coordinate_sets.len() <= MAX_SETS,
        "too many reported coordinate sets",
    )?;
    let mut seen = BTreeSet::new();
    let mut informative = false;
    let mut selected = None;
    for xs in &alg.coordinate_sets {
        check_membership(t.arm, &a.base, xs)?;
        require(seen.insert(xs), "duplicate coordinate set")?;
        let valid = relations(c, &a.base, xs, t.arm == Arm::OrbitPrunedLine3);
        require(
            !valid.is_empty(),
            "coordinate set has no permitted signed zero-sum relation",
        )?;
        informative |= valid.iter().any(|r| r.b != 0);
        if a.recovery.as_ref().is_some_and(|r| r.coordinates == *xs) {
            selected = Some(valid);
        }
    }
    if let Some(r) = &a.recovery {
        let valid = selected.ok_or("recovery coordinates are absent from solver output")?;
        let relation = valid
            .iter()
            .find(|v| v.signs == r.signs)
            .ok_or("recovery signs do not give a permitted zero-sum relation")?;
        require(
            relation.a == r.a && relation.b == r.b && r.b != 0,
            "recovery coefficients differ from signed labels or B is zero",
        )?;
        require(
            r.logarithm < c.n
                && r.logarithm == product(negative(r.a, c.n), power(r.b, c.n - 2, c.n), c.n),
            "recovered scalar does not solve A+B*d=0",
        )?;
        require(
            c.scale(Some(c.g), r.logarithm) == Some(q),
            "recovered scalar fails dG=Q",
        )?;
    } else {
        require(
            !informative,
            "an informative reported relation has no recovery",
        )?;
    }
    Ok(alg.coordinate_sets.len())
}

#[derive(Debug, Serialize)]
struct ReplayReport {
    schema: &'static str,
    input_sha256: String,
    fixture: Fixture,
    curve_icv1: String,
    attempts_checked: usize,
    labelled_points_checked: usize,
    coordinate_sets_checked: usize,
    recoveries_verified: usize,
    producer_status: String,
    independent_affine_arithmetic: bool,
    enumeration_completeness_verified: bool,
    sampling_deadlines_and_costs_verified: bool,
}

fn replay_bytes(bytes: &[u8]) -> Result<ReplayReport, String> {
    require(bytes.len() <= MAX_INPUT, "transcript exceeds 8 MiB")?;
    let t: Transcript =
        serde_json::from_slice(bytes).map_err(|e| format!("transcript JSON: {e}"))?;
    require(
        t.schema == "aps.single-relation.correctness/v1",
        "unsupported transcript schema",
    )?;
    require(
        (1..=4).contains(&t.subset_size)
            && (1..=128).contains(&t.attempt_cap)
            && (1..=60000).contains(&t.solver_budget_ms_per_attempt)
            && !t.attempts.is_empty()
            && t.attempts.len() <= t.attempt_cap,
        "fixture request exceeds producer bounds",
    )?;
    require(
        !t.arm.orbit() || t.fixture == Fixture::Toy103J0 && t.subset_size == 1,
        "orbit arms require the fixed j=0 fixture and subset size one",
    )?;
    let c = Curve::fixture(t.fixture);
    require(
        c.model() == t.curve_model && c.identity() == t.curve_icv1,
        "curve model or ICV1 identity does not match the fixed fixture",
    )?;
    require(
        c.group_order() == c.n && c.scale(Some(c.g), c.n).is_none(),
        "independent fixture group-order check failed",
    )?;
    let q = Point {
        x: t.target[0],
        y: t.target[1],
    };
    require(
        t.generator == [c.g.x, c.g.y]
            && c.on_curve(q)
            && c.scale(Some(q), c.n).is_none()
            && t.expected_public_logarithm == c.public_log
            && c.scale(Some(c.g), c.public_log) == Some(q),
        "generator, target or public known-answer metadata differs from the fixture",
    )?;
    let mut sets = 0;
    let mut points = 0;
    let mut witnesses = 0;
    for (i, attempt) in t.attempts.iter().enumerate() {
        sets += check_attempt(&t, c, q, attempt).map_err(|e| format!("attempt {i}: {e}"))?;
        require(sets <= MAX_TOTAL_SETS, "too many total coordinate sets")?;
        points += attempt.base.points.len();
        if attempt.recovery.is_some() {
            witnesses += 1;
            require(
                i + 1 == t.attempts.len(),
                "producer continued after recovering a witness",
            )?;
        }
    }
    let expected_status = if witnesses != 0 {
        "verified"
    } else if t.attempts.iter().any(|a| !a.algebra.complete) {
        "incomplete"
    } else {
        "no_informative_relation"
    };
    require(
        t.status == expected_status,
        "producer status contradicts its attempts",
    )?;
    Ok(ReplayReport {
        schema: "aps.single-relation.replay/v1",
        input_sha256: format!("{:x}", Sha256::digest(bytes)),
        fixture: t.fixture,
        curve_icv1: t.curve_icv1,
        attempts_checked: t.attempts.len(),
        labelled_points_checked: points,
        coordinate_sets_checked: sets,
        recoveries_verified: witnesses,
        producer_status: t.status,
        independent_affine_arithmetic: true,
        enumeration_completeness_verified: false,
        sampling_deadlines_and_costs_verified: false,
    })
}

fn execute(args: Args) -> Result<(), String> {
    let mut input = Vec::new();
    std::fs::File::open(args.transcript)
        .map_err(|e| e.to_string())?
        .take(MAX_INPUT as u64 + 1)
        .read_to_end(&mut input)
        .map_err(|e| e.to_string())?;
    let report = replay_bytes(&input)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(error) = execute(Args::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    const CASES: [&str; 14] = [
        "partitioned3",
        "unordered3",
        "line3",
        "expanded4",
        "split4",
        "block-symmetric4",
        "hybrid3",
        "hybrid4",
        "cover3",
        "orbit3",
        "orbit-line3",
        "orbit-pruned-line3",
        "line3-toy211",
        "split4-toy211",
    ];

    fn fixture_bytes(name: &str) -> Vec<u8> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = if env!("CARGO_PKG_NAME") == "aps-independent-replay" {
            root.join(format!("../fixtures/{name}.json"))
        } else {
            root.join(format!(
                "research/aps_independent_replay_20261029/fixtures/{name}.json"
            ))
        };
        std::fs::read(path).expect("committed native producer fixture")
    }

    fn fixture(name: &str) -> Value {
        serde_json::from_slice(&fixture_bytes(name)).unwrap()
    }

    fn replay(value: &Value) -> Result<ReplayReport, String> {
        replay_bytes(&serde_json::to_vec(value).unwrap())
    }

    fn last(value: &mut Value) -> &mut Value {
        value["attempts"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap()
    }

    #[test]
    fn actual_native_producer_corpus_replays_across_all_arms_and_three_curves() {
        let mut identities = BTreeSet::new();
        for name in CASES {
            let report = replay_bytes(&fixture_bytes(name))
                .unwrap_or_else(|error| panic!("{name}: {error}"));
            assert_eq!(report.recoveries_verified, 1, "{name}");
            assert!(report.attempts_checked > 0 && report.coordinate_sets_checked > 0);
            assert!(!report.enumeration_completeness_verified);
            assert!(!report.sampling_deadlines_and_costs_verified);
            identities.insert(report.curve_icv1);
        }
        assert_eq!(identities.len(), 3);
    }

    #[test]
    fn affine_group_law_handles_infinity_inverses_doubling_and_fixture_orders() {
        for name in [Fixture::Toy101, Fixture::Toy211, Fixture::Toy103J0] {
            let c = Curve::fixture(name);
            assert!(c.on_curve(c.g));
            assert_eq!(c.add(None, Some(c.g)), Some(c.g));
            assert_eq!(c.add(Some(c.g), None), Some(c.g));
            assert_eq!(c.add(Some(c.g), Some(c.negate(c.g))), None);
            assert_eq!(c.add(Some(c.g), Some(c.g)), c.scale(Some(c.g), 2));
            assert_eq!(c.scale(Some(c.g), c.n), None);
            assert_eq!(c.group_order(), c.n);
        }
        // A rational 2-torsion point on a separate nonsingular toy curve.
        let c = Curve {
            p: 5,
            a: 1,
            b: 0,
            n: 4,
            g: Point { x: 0, y: 0 },
            public_log: 1,
        };
        assert!(c.on_curve(c.g));
        assert_eq!(c.add(Some(c.g), Some(c.g)), None);
    }

    #[test]
    fn corrupt_labels_points_and_noncanonical_scalar_representatives_fail() {
        let original = fixture("line3");
        for field in ["u", "v", "x", "y"] {
            let mut value = original.clone();
            let points = last(&mut value)["base"]["points"].as_object_mut().unwrap();
            let point = points.values_mut().next().unwrap();
            point[field] = json!(point[field].as_u64().unwrap() + 1);
            assert!(replay(&value).is_err(), "changed point field {field}");
        }
        let mut value = original;
        let points = last(&mut value)["base"]["points"].as_object_mut().unwrap();
        let point = points.values_mut().next().unwrap();
        point["u"] = json!(point["u"].as_u64().unwrap() + 107);
        assert!(replay(&value).is_err());
    }

    #[test]
    fn recovered_scalar_coefficients_signs_and_membership_are_not_trusted() {
        let original = fixture("line3");
        for field in ["logarithm", "a", "b"] {
            let mut value = original.clone();
            let r = &mut last(&mut value)["recovery"];
            r[field] = json!((r[field].as_u64().unwrap() + 1) % 107);
            assert!(replay(&value).is_err(), "changed recovery {field}");
        }
        let mut zero_b = original.clone();
        last(&mut zero_b)["recovery"]["b"] = json!(0);
        assert!(replay(&zero_b).is_err());
        let mut signs = original.clone();
        last(&mut signs)["recovery"]["signs"][0] = json!(0);
        assert!(replay(&signs).is_err());
        let mut missing = original.clone();
        last(&mut missing)["algebra"]["coordinate_sets"] = json!([]);
        assert!(replay(&missing).is_err());
        let mut omitted_recovery = original;
        last(&mut omitted_recovery)["recovery"] = Value::Null;
        last(&mut omitted_recovery)["algebra"]["stopped_early"] = json!(false);
        omitted_recovery["status"] = json!("incomplete");
        let error = replay(&omitted_recovery).unwrap_err();
        assert!(error.contains("informative reported relation"), "{error}");
    }

    #[test]
    fn model_identity_target_and_expected_answer_are_bound_to_the_fixture() {
        let original = fixture("line3");
        for field in ["a", "b", "p", "field", "form", "v"] {
            let mut value = original.clone();
            value["curve_model"][field] = json!("forged");
            assert!(replay(&value).is_err(), "changed model {field}");
        }
        for field in ["curve_icv1", "fixture", "schema", "status"] {
            let mut value = original.clone();
            value[field] = json!("forged");
            assert!(replay(&value).is_err(), "changed header {field}");
        }
        let mut target = original.clone();
        target["target"][0] = json!(101);
        assert!(replay(&target).is_err());
        let mut expected = original;
        expected["expected_public_logarithm"] = json!(38);
        assert!(replay(&expected).is_err());
    }

    #[test]
    fn duplicate_keys_coordinates_and_new_unrecognised_fields_are_rejected() {
        let original = fixture("line3");
        let mut repeated = original.clone();
        let xs = last(&mut repeated)["algebra"]["coordinate_sets"]
            .as_array_mut()
            .unwrap();
        xs.push(xs[0].clone());
        assert!(replay(&repeated).is_err());
        let mut coordinate = original.clone();
        let xs = &mut last(&mut coordinate)["algebra"]["coordinate_sets"][0];
        xs[1] = xs[0].clone();
        assert!(replay(&coordinate).is_err());
        let mut extension = original.clone();
        extension["unrecognised_claim"] = json!(true);
        assert!(replay(&extension).is_err());
        let mut renamed = original.clone();
        let points = last(&mut renamed)["base"]["points"]
            .as_object_mut()
            .unwrap();
        let key = points.keys().next().unwrap().clone();
        let point = points.remove(&key).unwrap();
        points.insert(format!("0{key}"), point);
        assert!(replay(&renamed).is_err());
        let compact = serde_json::to_string(&original).unwrap();
        let point_map = original["attempts"][0]["base"]["points"]
            .as_object()
            .unwrap();
        let (key, point) = point_map.iter().next().unwrap();
        let duplicate = compact.replacen(
            "\"points\":{",
            &format!("\"points\":{{\"{key}\":{point},"),
            1,
        );
        assert!(replay_bytes(duplicate.as_bytes()).is_err());
        let duplicate_header = format!("{{\"schema\":{},{}", original["schema"], &compact[1..]);
        assert!(replay_bytes(duplicate_header.as_bytes()).is_err());
    }

    #[test]
    fn outcome_flags_and_continued_attempts_cannot_upgrade_an_early_witness() {
        let original = fixture("line3");
        for field in ["complete", "timed_out"] {
            let mut value = original.clone();
            last(&mut value)["algebra"][field] = json!(true);
            assert!(replay(&value).is_err());
        }
        let mut no_stop = original.clone();
        last(&mut no_stop)["algebra"]["stopped_early"] = json!(false);
        assert!(replay(&no_stop).is_err());
        let mut continuing = original;
        let attempts = continuing["attempts"].as_array_mut().unwrap();
        attempts.push(attempts.last().unwrap().clone());
        assert!(replay(&continuing).is_err());
    }

    #[test]
    fn no_witness_is_reported_separately_from_unproved_exhaustion_or_timeout() {
        let mut value = fixture("line3");
        value["attempts"] = json!([value["attempts"].as_array().unwrap().last().unwrap()]);
        let attempt = last(&mut value);
        attempt["recovery"] = Value::Null;
        attempt["algebra"]["coordinate_sets"] = json!([]);
        attempt["algebra"]["stopped_early"] = json!(false);
        attempt["algebra"]["timed_out"] = json!(true);
        attempt["algebra"]["complete"] = json!(false);
        value["status"] = json!("incomplete");
        let report = replay(&value).unwrap();
        assert_eq!(report.recoveries_verified, 0);
        assert!(!report.enumeration_completeness_verified);
        last(&mut value)["algebra"]["timed_out"] = json!(false);
        last(&mut value)["algebra"]["complete"] = json!(true);
        value["status"] = json!("no_informative_relation");
        let report = replay(&value).unwrap();
        assert_eq!(report.recoveries_verified, 0);
        assert!(!report.enumeration_completeness_verified);
        value["status"] = json!("verified");
        assert!(replay(&value).is_err());
    }

    #[test]
    fn pruning_refuses_independent_labels_even_when_points_and_witness_still_match() {
        let mut value = fixture("orbit-pruned-line3");
        value["labels"] = json!("random");
        let attempt = last(&mut value);
        let used = attempt["recovery"]["coordinates"]
            .as_array()
            .unwrap()
            .clone();
        let points = attempt["base"]["points"].as_object_mut().unwrap();
        let (key, point) = points
            .iter_mut()
            .find(|(x, _)| !used.contains(&json!(x.parse::<u64>().unwrap())))
            .unwrap();
        let _ = key;
        // Q=17G: changing (u,v) to (u-17,v+1) preserves this point.
        point["u"] = json!((point["u"].as_u64().unwrap() + 97 - 17) % 97);
        point["v"] = json!((point["v"].as_u64().unwrap() + 1) % 97);
        let error = replay(&value).unwrap_err();
        assert!(error.contains("orbit labels"), "{error}");
    }
}
