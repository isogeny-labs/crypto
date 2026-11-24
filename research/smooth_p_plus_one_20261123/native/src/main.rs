//! Native, bounded development solver for the regular four-summand chart.
mod polynomial;
mod system;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use polynomial::{evaluate, field_roots, inverse, pow};
use serde_json::{json, Value};
use system::{Mode, System};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Fixture {
    p: i64,
    a: i64,
    b: i64,
    n: i64,
    g: Point,
    q: Point,
}

const FIXTURES: [Fixture; 2] = [
    Fixture {
        p: 127,
        a: 1,
        b: 7,
        n: 109,
        g: Point { x: 1, y: 3 },
        q: Point { x: 11, y: 98 },
    },
    Fixture {
        p: 8191,
        a: 1,
        b: 5,
        n: 8053,
        g: Point { x: 0, y: 789 },
        q: Point { x: 3197, y: 5073 },
    },
];

impl Fixture {
    fn rhs(self, x: i64) -> i64 {
        (((x * x % self.p + self.a) * x) + self.b).rem_euclid(self.p)
    }

    fn negate(self, point: Point) -> Point {
        Point {
            x: point.x,
            y: (-point.y).rem_euclid(self.p),
        }
    }

    fn add(self, left: Option<Point>, right: Option<Point>) -> Option<Point> {
        let (Some(left), Some(right)) = (left, right) else {
            return left.or(right);
        };
        let slope = if left.x == right.x {
            if (left.y + right.y).rem_euclid(self.p) == 0 {
                return None;
            }
            (3 * left.x * left.x + self.a) * inverse(2 * left.y, self.p)
        } else {
            (right.y - left.y) * inverse(right.x - left.x, self.p)
        }
        .rem_euclid(self.p);
        let x = (slope * slope - left.x - right.x).rem_euclid(self.p);
        let y = (slope * (left.x - x) - left.y).rem_euclid(self.p);
        Some(Point { x, y })
    }

    fn multiply(self, mut k: i64, base: Point) -> Option<Point> {
        let (mut result, mut step) = (None, Some(base));
        while k > 0 {
            if k & 1 != 0 {
                result = self.add(result, step);
            }
            step = self.add(step, step);
            k >>= 1;
        }
        result
    }
}

fn dickson(mut x: i64, steps: usize, p: i64) -> i64 {
    for _ in 0..steps {
        x = (x * x - 2).rem_euclid(p);
    }
    x
}

fn torus_mul((x, y): (i64, i64), (u, v): (i64, i64), p: i64) -> (i64, i64) {
    ((x * u - y * v).rem_euclid(p), (x * v + y * u).rem_euclid(p))
}

fn torus_pow(mut base: (i64, i64), mut exponent: i64, p: i64) -> (i64, i64) {
    let mut result = (1, 0);
    while exponent > 0 {
        if exponent & 1 != 0 {
            result = torus_mul(result, base, p);
        }
        base = torus_mul(base, base, p);
        exponent >>= 1;
    }
    result
}

type Relation = [i32; 4];

struct Base {
    curve: Fixture,
    depth: usize,
    terminal: i64,
    unsigned: Vec<Point>,
    signed: BTreeMap<Point, i32>,
}

impl Base {
    fn new(curve: Fixture, depth: usize) -> Self {
        let p = curve.p;
        let size = 1 << depth;
        assert_eq!((p + 1) % size, 0);
        let alpha = (2..p)
            .find_map(|x| {
                let inv = inverse(x * x + 1, p);
                let candidate = (
                    ((x * x - 1) * inv).rem_euclid(p),
                    (2 * x * inv).rem_euclid(p),
                );
                (torus_pow(candidate, (p + 1) / 2, p) != (1, 0)).then_some(candidate)
            })
            .expect("primitive torus representative");
        let step = torus_pow(alpha, (p + 1) / size, p);
        let terminal = (2 * torus_pow(alpha, size, p).0).rem_euclid(p);
        let mut representative = alpha;
        let mut unsigned = vec![];
        for _ in 0..size {
            let x = 2 * representative.0 % p;
            let square = curve.rhs(x);
            let y = pow(square, (p + 1) / 4, p);
            if y * y % p == square {
                assert_ne!(y, 0);
                unsigned.push(Point { x, y: y.min(p - y) });
            }
            representative = torus_mul(representative, step, p);
        }
        unsigned.sort_unstable();
        let signed = unsigned
            .iter()
            .enumerate()
            .flat_map(|(i, point)| {
                let label = i as i32 + 1;
                [(*point, label), (curve.negate(*point), -label)]
            })
            .collect();
        Self {
            curve,
            depth,
            terminal,
            unsigned,
            signed,
        }
    }

    fn orient(&self, label: i32) -> Point {
        let point = self.unsigned[label.unsigned_abs() as usize - 1];
        if label < 0 {
            self.curve.negate(point)
        } else {
            point
        }
    }

    fn separated(&self, row: &Relation) -> bool {
        row.iter()
            .map(|label| dickson(self.orient(*label).x, self.depth - 2, self.curve.p))
            .collect::<BTreeSet<_>>()
            .len()
            == 4
    }

    fn pair_index(&self) -> BTreeMap<Option<Point>, Vec<[i32; 2]>> {
        let points: Vec<_> = self.signed.iter().collect();
        let mut result = BTreeMap::<_, Vec<_>>::new();
        for (i, (left, left_label)) in points.iter().enumerate() {
            for (right, right_label) in &points[i + 1..] {
                if left.x != right.x {
                    result
                        .entry(self.curve.add(Some(**left), Some(**right)))
                        .or_default()
                        .push([**left_label, **right_label]);
                }
            }
        }
        result
    }

    fn reference(
        &self,
        pairs: &BTreeMap<Option<Point>, Vec<[i32; 2]>>,
        target: Point,
    ) -> BTreeSet<Relation> {
        let mut result = BTreeSet::new();
        for (sum, lefts) in pairs {
            let complement = self
                .curve
                .add(Some(target), sum.map(|point| self.curve.negate(point)));
            if let Some(rights) = pairs.get(&complement) {
                for [a, b] in lefts {
                    for [c, d] in rights {
                        let mut row = [*a, *b, *c, *d];
                        row.sort_unstable();
                        let abscissae: BTreeSet<_> =
                            row.iter().map(|label| self.orient(*label).x).collect();
                        if abscissae.len() == 4 && !abscissae.contains(&target.x) {
                            result.insert(row);
                        }
                    }
                }
            }
        }
        result
    }
}

fn coefficients(encoded: &Value) -> Result<Vec<i64>, String> {
    encoded
        .get(1)
        .and_then(Value::as_array)
        .ok_or("invalid polynomial in RUR")?
        .iter()
        .map(|value| {
            value
                .as_i64()
                .ok_or_else(|| "invalid coefficient in RUR".into())
        })
        .collect()
}

fn extract(
    raw: &str,
    base: &Base,
    target: Point,
    mode: Mode,
) -> Result<BTreeSet<Relation>, String> {
    let parsed: Value = serde_json::from_str(&raw.trim().trim_end_matches(':').replace('\'', "\""))
        .map_err(|error| error.to_string())?;
    if parsed == json!([-1]) {
        return Ok(BTreeSet::new());
    }
    if parsed[0].as_i64() != Some(0) || parsed[1][0].as_i64() != Some(base.curve.p) {
        return Err("solver did not return a finite parametrization".into());
    }
    let p = base.curve.p;
    let block = &parsed[1];
    let names = block[3]
        .as_array()
        .ok_or("missing parametrization coordinates")?;
    let get = |name| {
        names
            .iter()
            .position(|entry| entry.as_str() == Some(name))
            .ok_or("missing RR5 coordinate")
    };
    let indices = [get("a0")?, get("a1")?, get("t0")?];
    let parametrization = &block[5];
    if parametrization[0].as_i64() != Some(1) || names.is_empty() {
        return Err("unsupported RUR scaling".into());
    }
    let data = &parametrization[1];
    let denominator = coefficients(&data[1])?;
    let numerators = &data[2];
    let mut result = BTreeSet::new();
    for root in field_roots(coefficients(&data[0])?, p)? {
        let scaled = evaluate(&denominator, root, p);
        if scaled == 0 {
            return Err("zero RUR denominator".into());
        }
        let scale = inverse(scaled, p);
        let mut parameters = [0; 3];
        for (value, index) in parameters.iter_mut().zip(indices) {
            *value = if index + 1 == names.len() {
                root
            } else {
                let numerator = coefficients(&numerators[index][0])?;
                (-evaluate(&numerator, root, p) * scale).rem_euclid(p)
            };
        }
        let [alpha, beta, t] = parameters;
        let quartic = system::numeric_quartic(base.curve, target, alpha, beta, t);
        let xs = field_roots(quartic, p)?;
        if xs.len() != 4 || xs.contains(&target.x) {
            continue;
        }
        let mut points = BTreeSet::new();
        let mut sum = None;
        for x in xs {
            if (x + t).rem_euclid(p) == 0 {
                return Err("zero RR5 denominator".into());
            }
            let y =
                (-target.y - (x - target.x) * (alpha + beta * x) * inverse(x + t, p)).rem_euclid(p);
            let leaf = Point { x, y };
            let &label = base.signed.get(&leaf).ok_or("signed lift outside base")?;
            points.insert(label);
            sum = base.curve.add(sum, Some(leaf));
        }
        let row: Relation = points
            .into_iter()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "repeated RR5 label")?;
        if sum != Some(target) || (mode == Mode::Distinct && !base.separated(&row)) {
            return Err("RR5 relation failed independent replay".into());
        }
        result.insert(row);
    }
    Ok(result)
}

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Result<Self, String> {
        let parent = std::env::temp_dir();
        for suffix in 0..512 {
            let path = parent.join(format!("rr5-solve-{}-{suffix}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.to_string()),
            }
        }
        Err("no solver work directory".into())
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run_solver(
    binary: &Path,
    input: &str,
    budget: Duration,
    scratch: &Scratch,
) -> Result<String, String> {
    let input_path = scratch.0.join("input.ms");
    let output_path = scratch.0.join("output.txt");
    let stderr_path = scratch.0.join("stderr.txt");
    if output_path.exists() {
        fs::remove_file(&output_path).map_err(|error| error.to_string())?;
    }
    fs::write(&input_path, input).map_err(|error| error.to_string())?;
    let stderr = File::create(&stderr_path).map_err(|error| error.to_string())?;
    let mut child = Command::new(binary)
        .arg("-f")
        .arg(input_path)
        .arg("-o")
        .arg(&output_path)
        .args(["-t", "1"])
        .stdout(Stdio::null())
        .stderr(stderr)
        .spawn()
        .map_err(|error| error.to_string())?;
    let deadline = Instant::now() + budget;
    loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
            if !status.success() {
                return Err(format!("solver exit {status}"));
            }
            return fs::read_to_string(output_path).map_err(|error| error.to_string());
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("msolve timeout".into());
        }
        thread::sleep(Duration::from_micros(250));
    }
}

fn mode(value: &str) -> Option<Mode> {
    match value {
        "distinct" => Some(Mode::Distinct),
        "expanded" => Some(Mode::Expanded),
        "quadratic" => Some(Mode::Quadratic),
        _ => None,
    }
}

struct Recovery {
    width: usize,
    order: i64,
    pivots: BTreeMap<usize, Vec<i64>>,
}

impl Recovery {
    fn insert(&mut self, relation: Relation, a: i64, b: i64) -> Option<i64> {
        let mut row = vec![0; self.width + 2];
        for label in relation {
            row[label.unsigned_abs() as usize - 1] += i64::from(label.signum());
        }
        row[self.width] = a;
        row[self.width + 1] = b;
        let order = self.order;
        for column in 0..self.width {
            let leading = row[column].rem_euclid(order);
            if leading == 0 {
                continue;
            }
            if let Some(pivot) = self.pivots.get(&column) {
                for (entry, coefficient) in row[column..].iter_mut().zip(&pivot[column..]) {
                    *entry = (*entry - leading * coefficient).rem_euclid(order);
                }
            } else {
                let scale = inverse(leading, order);
                for entry in &mut row[column..] {
                    *entry = (*entry * scale).rem_euclid(order);
                }
                self.pivots.insert(column, row);
                return None;
            }
        }
        let a = row[self.width].rem_euclid(order);
        let b = row[self.width + 1].rem_euclid(order);
        if b == 0 {
            assert_eq!(a, 0, "inconsistent signed rows");
            None
        } else {
            Some((-a * inverse(b, order)).rem_euclid(order))
        }
    }
}

fn next_mask(state: &mut u64, order: i64) -> i64 {
    let order = order as u64;
    loop {
        *state = state.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = *state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^= value >> 31;
        if value < u64::MAX - u64::MAX % order {
            return (value % order) as i64;
        }
    }
}

fn recover(
    curve: Fixture,
    depth: usize,
    solver: &Path,
    scratch: &Scratch,
    method: Option<Mode>,
    mut seed: u64,
) {
    let base = Base::new(curve, depth);
    let pair_index = method.is_none().then(|| base.pair_index());
    let mut rows = Recovery {
        width: base.unsigned.len(),
        order: curve.n,
        pivots: BTreeMap::new(),
    };
    let mut visited = BTreeSet::from([None]);
    let label = method.map_or("Direct".into(), |mode| format!("{mode:?}"));
    let original_seed = seed;
    let mut scalar = None;
    let mut used = 0;
    let mut oracle_calls = 0;
    for attempt in 1..=30 {
        let a = next_mask(&mut seed, curve.n);
        let b = next_mask(&mut seed, curve.n);
        let residual = curve.add(curve.multiply(a, curve.g), curve.multiply(b, curve.q));
        if !visited.insert(residual) {
            continue;
        }
        let target = residual.expect("identity is excluded");
        oracle_calls += 1;
        let relations = if let Some(mode) = method {
            let system = System::build(curve, target, depth, base.terminal, mode);
            run_solver(
                solver,
                &system.render(curve.p),
                Duration::from_secs(3),
                scratch,
            )
            .and_then(|raw| extract(&raw, &base, target, mode))
        } else {
            Ok(base.reference(pair_index.as_ref().unwrap(), target))
        };
        let result = match relations {
            Err(reason) => json!({"status":"undetermined","reason":reason}),
            Ok(relations) => {
                if let Some(row) = relations.first() {
                    used += 1;
                    scalar = rows.insert(*row, a, b);
                }
                json!({"status":"checked","relations":relations.len(),"selected":relations.first()})
            }
        };
        println!(
            "{}",
            json!({
                "kind":"attempt","mode":label,"seed":original_seed,"draw":attempt,
                "a":a,"b":b,"target":[target.x,target.y],"result":result,
            })
        );
        if let Some(found) = scalar {
            assert_eq!(curve.multiply(found, curve.g), Some(curve.q));
            break;
        }
    }
    println!(
        "{}",
        json!({
            "kind":"summary","mode":label,"seed":original_seed,"prime":curve.p,"depth":depth,
            "calls":oracle_calls,"selected":used,"base_rank":rows.pivots.len(),"scalar":scalar,
        })
    );
}

fn main() -> Result<(), String> {
    let argv: Vec<_> = std::env::args().skip(1).collect();
    let [prime, depth, solver, modes @ ..] = argv.as_slice() else {
        return Err(
            "usage: dickson-four-summand PRIME DEPTH MSOLVE [recover=SEED] [distinct expanded quadratic]".into(),
        );
    };
    let curve = *FIXTURES
        .iter()
        .find(|curve| curve.p.to_string() == *prime)
        .ok_or("unknown fixture")?;
    let depth: usize = depth.parse().map_err(|_| "invalid depth")?;
    if !(2..=6).contains(&depth) {
        return Err("depth outside 2..=6".into());
    }
    if ((curve.p + 1) >> depth) < 4 {
        return Err("depth leaves no regular fiber for the fixture generator".into());
    }
    assert_eq!(curve.multiply(curve.n, curve.g), None);
    assert_eq!(curve.multiply(curve.n, curve.q), None);
    let scratch = Scratch::new()?;
    let (recovery_seed, modes) =
        if let Some(recovery) = modes.first().and_then(|a| a.strip_prefix("recover=")) {
            let seed = recovery
                .parse::<u64>()
                .map_err(|_| "invalid recovery seed")?;
            (Some(seed), &modes[1..])
        } else {
            (None, modes)
        };
    let modes: Vec<_> = if modes.is_empty() {
        vec![Mode::Distinct, Mode::Expanded, Mode::Quadratic]
    } else {
        modes
            .iter()
            .map(|label| mode(label).ok_or("unknown mode"))
            .collect::<Result<_, _>>()?
    };
    if depth > 4 && modes.contains(&Mode::Expanded) {
        return Err("expanded control is limited to depths of at most four".into());
    }
    if let Some(seed) = recovery_seed {
        recover(curve, depth, Path::new(solver), &scratch, None, seed);
        for mode in modes {
            recover(curve, depth, Path::new(solver), &scratch, Some(mode), seed);
        }
        return Ok(());
    }
    let base = Base::new(curve, depth);
    let pairs = base.pair_index();
    let mut targets = if curve.p == 127 {
        vec![
            Point { x: 20, y: 91 },
            Point { x: 116, y: 69 },
            Point { x: 41, y: 95 },
            Point { x: 41, y: 32 },
        ]
    } else {
        [17, 43, 89, 152]
            .into_iter()
            .map(|k| curve.multiply(k, curve.g).unwrap())
            .collect()
    };
    let unplanted = targets.len();
    if curve.p != 127 {
        let mut pools = BTreeMap::new();
        for point in &base.unsigned {
            pools
                .entry(dickson(point.x, depth - 2, curve.p))
                .or_insert(*point);
        }
        if pools.len() == 4 {
            let sum = pools
                .values()
                .fold(None, |acc, point| curve.add(acc, Some(*point)));
            if let Some(target) = sum.filter(|target| pools.values().all(|leaf| leaf.x != target.x))
            {
                targets.push(target);
            }
        }
    }
    for (trial, target) in targets.into_iter().enumerate() {
        let expected = base.reference(&pairs, target);
        for mode in &modes {
            let system = System::build(curve, target, depth, base.terminal, *mode);
            let (variables, equations, terms, degree) = system.metrics();
            let want: BTreeSet<_> = expected
                .iter()
                .filter(|row| *mode != Mode::Distinct || base.separated(row))
                .copied()
                .collect();
            let result = run_solver(
                Path::new(solver),
                &system.render(curve.p),
                Duration::from_secs(3),
                &scratch,
            )
            .and_then(|raw| extract(&raw, &base, target, *mode));
            let record = match result {
                Ok(recovered) if recovered == want => {
                    json!({"status":"exact","relations":recovered.len()})
                }
                Ok(recovered) => {
                    return Err(format!(
                        "{mode:?} mismatch for {target:?}: {recovered:?} != {want:?}"
                    ))
                }
                Err(error) => json!({"status":"undetermined","reason":error}),
            };
            println!(
                "{}",
                json!({
                    "prime":curve.p,"depth":depth,"target":[target.x,target.y],
                    "mode":format!("{mode:?}"),"variables":variables,"equations":equations,
                    "terms":terms,"degree":degree,"expected":want.len(),"result":record,
                    "planted":trial>=unplanted,
                })
            );
        }
    }
    Ok(())
}
