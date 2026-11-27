//! Independent finite-control verifier for the cold m=7 Dickson/Semaev run.
//!
//! This deliberately does not call the production Dickson factor-base or
//! elliptic-curve helpers.  It reconstructs the tiny public carriers by
//! scanning the base field, uses a separate affine group law, and computes
//! exact seven-leaf reachability by dynamic programming.

use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const ARITY: usize = 7;
const DEFAULT_DIR: &str = "research/pplus_dickson_m7_elimination_20261127";

#[derive(Clone, Copy, Debug, Eq)]
struct Point {
    x: u64,
    y: u64,
    infinity: bool,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.infinity == other.infinity
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.infinity.hash(state);
    }
}

impl Point {
    const INFINITY: Self = Self {
        x: 0,
        y: 0,
        infinity: true,
    };
}

#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}

fn addm(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(p)) as u64
}

fn subm(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) + u128::from(p) - u128::from(b)) % u128::from(p)) as u64
}

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}

fn invm(a: u64, p: u64) -> u64 {
    (1..p)
        .find(|&candidate| mulm(a, candidate, p) == 1)
        .expect("nonzero field element must have an inverse")
}

impl Curve {
    fn rhs(self, x: u64) -> u64 {
        addm(
            addm(
                mulm(mulm(x, x, self.p), x, self.p),
                mulm(self.a, x, self.p),
                self.p,
            ),
            self.b,
            self.p,
        )
    }

    fn add(self, left: Point, right: Point) -> Point {
        if left.infinity {
            return right;
        }
        if right.infinity {
            return left;
        }
        if left.x == right.x && addm(left.y, right.y, self.p) == 0 {
            return Point::INFINITY;
        }
        let slope = if left == right {
            let numerator = addm(
                mulm(3, mulm(left.x, left.x, self.p), self.p),
                self.a,
                self.p,
            );
            mulm(numerator, invm(mulm(2, left.y, self.p), self.p), self.p)
        } else {
            mulm(
                subm(right.y, left.y, self.p),
                invm(subm(right.x, left.x, self.p), self.p),
                self.p,
            )
        };
        let x = subm(
            subm(mulm(slope, slope, self.p), left.x, self.p),
            right.x,
            self.p,
        );
        let y = subm(mulm(slope, subm(left.x, x, self.p), self.p), left.y, self.p);
        Point {
            x,
            y,
            infinity: false,
        }
    }
}

fn roots(value: u64, p: u64) -> Vec<u64> {
    (0..p).filter(|&x| mulm(x, x, p) == value % p).collect()
}

/// Reconstruct the ordered production carrier using exhaustive root scans.
fn carrier(curve: Curve, depth: u32) -> Vec<Point> {
    let mut layer = vec![0u64];
    for _ in 0..depth {
        let mut next = Vec::new();
        for parent in layer {
            let rs = roots(addm(parent, 2 % curve.p, curve.p), curve.p);
            assert_eq!(rs.len(), 2, "unramified Dickson node must have two roots");
            next.extend(rs);
        }
        layer = next;
    }
    let mut points = Vec::new();
    for x in layer {
        let rs = roots(curve.rhs(x), curve.p);
        if let Some(&y) = rs.first() {
            points.push(Point {
                x,
                y,
                infinity: false,
            });
            if rs.len() == 2 {
                points.push(Point {
                    x,
                    y: rs[1],
                    infinity: false,
                });
            }
        }
    }
    points
}

fn exact_reachability(curve: Curve, base: &[Point]) -> HashMap<Point, Vec<usize>> {
    let mut layer = HashMap::from([(Point::INFINITY, Vec::new())]);
    for _ in 0..ARITY {
        let mut next = HashMap::new();
        for (sum, witness) in layer {
            for (index, &leaf) in base.iter().enumerate() {
                let result = curve.add(sum, leaf);
                next.entry(result).or_insert_with(|| {
                    let mut extended = witness.clone();
                    extended.push(index);
                    extended
                });
            }
        }
        layer = next;
    }
    layer
}

fn parse_point(value: &Value) -> Point {
    Point {
        x: value["x"].as_u64().expect("point x"),
        y: value["y"].as_u64().expect("point y"),
        infinity: value["infinity"].as_bool().expect("point infinity flag"),
    }
}

fn parse_witness(value: Option<&Value>) -> Option<Vec<usize>> {
    value.and_then(Value::as_array).map(|items| {
        items
            .iter()
            .map(|item| item.as_u64().expect("witness index") as usize)
            .collect()
    })
}

fn replay(curve: Curve, base: &[Point], witness: &[usize]) -> Point {
    assert_eq!(witness.len(), ARITY, "witness must have seven leaves");
    witness.iter().fold(Point::INFINITY, |sum, &index| {
        assert!(index < base.len(), "witness index outside carrier");
        curve.add(sum, base[index])
    })
}

fn sha256(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    format!("{:x}", Sha256::digest(bytes))
}

fn main() {
    let mut inputs: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if inputs.is_empty() {
        inputs.push(Path::new(DEFAULT_DIR).join("depth1.jsonl"));
        let depth2 = Path::new(DEFAULT_DIR).join("depth2.jsonl");
        if depth2.exists() {
            inputs.push(depth2);
        }
    }

    let mut exact_by_fixture: HashMap<String, (HashMap<Point, Vec<usize>>, Point)> = HashMap::new();
    let mut hashes = serde_json::Map::new();
    let mut solver_rows = 0u64;
    let mut chart_rows = 0u64;
    let mut resolved_rows = 0u64;
    let mut unresolved_rows = 0u64;
    let mut candidate_witnesses = 0u64;

    for path in &inputs {
        hashes.insert(path.display().to_string(), Value::String(sha256(path)));
        let raw = fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for line in raw.lines().filter(|line| !line.trim().is_empty()) {
            let row: Value = serde_json::from_str(line).expect("valid JSONL row");
            let schema = row["schema"].as_str().expect("row schema");
            let fixture = row["fixture"].as_str().expect("fixture label").to_owned();
            if schema == "isogeny-labs/pplus-dickson-m7-chart-control/v1" {
                chart_rows += 1;
                let (reachable, infinity) = exact_by_fixture
                    .get(&fixture)
                    .expect("solver row must precede its chart control");
                assert_eq!(parse_point(&row["target"]), *infinity);
                assert_eq!(row["candidate_run"].as_bool(), Some(false));
                assert_eq!(
                    row["exact_affine_reachable"].as_bool(),
                    Some(reachable.contains_key(infinity))
                );
                continue;
            }
            assert_eq!(schema, "isogeny-labs/pplus-dickson-m7-elimination/v1");
            solver_rows += 1;
            assert_eq!(row["arity"].as_u64(), Some(ARITY as u64));
            assert_eq!(row["cross_target_state_reused"].as_bool(), Some(false));

            let curve = Curve {
                p: row["p"].as_u64().expect("p"),
                a: row["curve_a"].as_u64().expect("curve a"),
                b: row["curve_b"].as_u64().expect("curve b"),
            };
            let depth = row["depth"].as_u64().expect("Dickson depth") as u32;
            let base = carrier(curve, depth);
            assert_eq!(
                base.len() as u64,
                row["factor_base_points"].as_u64().expect("carrier size")
            );
            let mut xs: Vec<u64> = base.iter().map(|point| point.x).collect();
            xs.sort_unstable();
            xs.dedup();
            assert_eq!(
                xs.len() as u64,
                row["factor_base_abscissae"].as_u64().expect("abscissae")
            );

            let reachable = exact_reachability(curve, &base);
            exact_by_fixture.insert(fixture, (reachable.clone(), Point::INFINITY));
            let target = parse_point(&row["target"]);
            let exact_hit = reachable.contains_key(&target);
            assert_eq!(row["expected_hit"].as_bool(), Some(exact_hit));

            if let Some(control) = parse_witness(row.get("control_witness")) {
                assert_eq!(
                    replay(curve, &base, &control),
                    target,
                    "control witness replay"
                );
            } else {
                assert!(!exact_hit, "reachable row lacks its control witness");
            }

            let candidate = parse_witness(row.get("candidate_witness"));
            let candidate_valid = candidate
                .as_ref()
                .is_none_or(|witness| replay(curve, &base, witness) == target);
            assert!(candidate_valid, "candidate witness replay");
            if let Some(stored) = row.get("witness_valid").and_then(Value::as_bool) {
                assert_eq!(stored, candidate_valid);
            }
            candidate_witnesses += u64::from(candidate.is_some());

            let budget_exceeded = row["solver_totals"]["budget_exceeded"]
                .as_u64()
                .expect("budget-exceeded counter");
            // The first archived row predates the explicit `resolved` field;
            // its zero budget counter is the equivalent legacy encoding.
            let resolved = row
                .get("resolved")
                .and_then(Value::as_bool)
                .unwrap_or(budget_exceeded == 0);
            if resolved {
                resolved_rows += 1;
                assert_eq!(candidate.is_some(), exact_hit, "resolved truth agreement");
                if let Some(stored) = row.get("truth_agreement").and_then(Value::as_bool) {
                    assert!(stored);
                }
                assert_eq!(budget_exceeded, 0);
            } else {
                unresolved_rows += 1;
                assert_ne!(budget_exceeded, 0);
            }
        }
    }

    println!(
        "{}",
        serde_json::to_string(&json!({
            "schema":"isogeny-labs/pplus-dickson-m7-independent-verifier/v1",
            "inputs_sha256":hashes,
            "solver_rows":solver_rows,
            "chart_rows":chart_rows,
            "resolved_rows":resolved_rows,
            "unresolved_rows":unresolved_rows,
            "candidate_witnesses_replayed":candidate_witnesses,
            "cross_target_amortization":false,
            "verdict":"all finite controls and available candidate witnesses replayed",
        }))
        .expect("serialize verifier result")
    );
}
