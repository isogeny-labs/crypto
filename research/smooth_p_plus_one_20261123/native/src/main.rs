//! Native, bounded development solver for the regular four-summand chart.
mod polynomial;
#[cfg(feature = "production-controls")]
mod production;
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

#[derive(Clone, Debug)]
struct Refinement {
    images: [i64; 4],
    polynomial: [i64; 4],
}

impl Refinement {
    fn contains(&self, base: &Base, row: &Relation) -> bool {
        let mut images =
            row.map(|label| dickson(base.orient(label).x, base.depth - 3, base.curve.p));
        images.sort_unstable();
        let mut expected = self.images;
        expected.sort_unstable();
        images == expected
    }
}

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

    fn refinements(&self) -> Vec<Refinement> {
        assert!(self.depth >= 3);
        let mut pools = BTreeMap::<i64, BTreeSet<i64>>::new();
        for point in &self.unsigned {
            let fine = dickson(point.x, self.depth - 3, self.curve.p);
            let coarse = dickson(fine, 1, self.curve.p);
            pools.entry(coarse).or_default().insert(fine);
        }
        if pools.len() != 4 {
            return vec![];
        }
        let mut choices = vec![vec![]];
        for pool in pools.values() {
            assert!(pool.len() <= 2);
            choices = choices
                .iter()
                .flat_map(|choice| {
                    pool.iter().map(move |image| {
                        let mut extended = choice.clone();
                        extended.push(*image);
                        extended
                    })
                })
                .collect();
        }
        choices
            .into_iter()
            .map(|images| {
                let mut polynomial = vec![1];
                for image in &images {
                    polynomial = polynomial::product(&polynomial, &[-image, 1], self.curve.p);
                }
                Refinement {
                    images: images.try_into().expect("four coarse image pools"),
                    polynomial: polynomial[..4].try_into().unwrap(),
                }
            })
            .collect()
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
    let eliminant = coefficients(&data[0])?;
    let dimension = block[2].as_u64().ok_or("missing quotient dimension")?;
    if eliminant.len().checked_sub(1).map(|n| n as u64) != Some(dimension) {
        return Err(format!(
            "nonseparating RUR: eliminant degree {} differs from quotient dimension {dimension}",
            eliminant.len().saturating_sub(1)
        ));
    }
    let denominator = coefficients(&data[1])?;
    let numerators = &data[2];
    let mut result = BTreeSet::new();
    for root in field_roots(eliminant, p)? {
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
    deadline: Instant,
    scratch: &Scratch,
    extra_args: &[&str],
) -> Result<String, String> {
    if Instant::now() >= deadline {
        return Err("oracle budget exhausted".into());
    }
    let input_path = scratch.0.join("input.ms");
    let output_path = scratch.0.join("output.txt");
    let stderr_path = scratch.0.join("stderr.txt");
    if output_path.exists() {
        fs::remove_file(&output_path).map_err(|error| error.to_string())?;
    }
    fs::write(&input_path, input).map_err(|error| error.to_string())?;
    let stderr = File::create(&stderr_path).map_err(|error| error.to_string())?;
    if Instant::now() >= deadline {
        return Err("oracle budget exhausted".into());
    }
    let mut child = Command::new(binary)
        .arg("-f")
        .arg(input_path)
        .arg("-o")
        .arg(&output_path)
        .args(["-t", "1"])
        .args(extra_args)
        .stdout(Stdio::null())
        .stderr(stderr)
        .spawn()
        .map_err(|error| error.to_string())?;
    loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
            if !status.success() {
                return Err(format!("solver exit {status}"));
            }
            let output = fs::read_to_string(output_path).map_err(|error| error.to_string())?;
            if output.trim().is_empty() && extra_args == ["-c", "0"] {
                let message = fs::read_to_string(&stderr_path).unwrap_or_default();
                if message.contains("=====> Computation failed <=====")
                    && message.contains("Try to add a random linear form with a new variable")
                {
                    return Err("nonseparating staircase (msolve -c 0)".into());
                }
            }
            return Ok(output);
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("msolve timeout".into());
        }
        thread::sleep(Duration::from_micros(250));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Method {
    Single(Mode),
    Refined,
    #[cfg(feature = "production-controls")]
    Cofactor,
    #[cfg(feature = "production-controls")]
    CofactorMsolve,
}

impl Method {
    fn name(self) -> String {
        match self {
            Self::Single(mode) => format!("{mode:?}"),
            Self::Refined => "Refined".into(),
            #[cfg(feature = "production-controls")]
            Self::Cofactor => "ProductionCofactor".into(),
            #[cfg(feature = "production-controls")]
            Self::CofactorMsolve => "ProductionCofactorMsolve".into(),
        }
    }

    fn encoding(self) -> Mode {
        match self {
            Self::Single(mode) => mode,
            Self::Refined => Mode::Distinct,
            // The cofactor solver searches the full distinct-x RR5 chart.
            #[cfg(feature = "production-controls")]
            Self::Cofactor | Self::CofactorMsolve => Mode::Quadratic,
        }
    }
}

fn mode(value: &str) -> Option<Method> {
    match value {
        "distinct" => Some(Method::Single(Mode::Distinct)),
        "refined" => Some(Method::Refined),
        "expanded" => Some(Method::Single(Mode::Expanded)),
        "quadratic" => Some(Method::Single(Mode::Quadratic)),
        #[cfg(feature = "production-controls")]
        "cofactor" => Some(Method::Cofactor),
        #[cfg(feature = "production-controls")]
        "cofactor-msolve" => Some(Method::CofactorMsolve),
        _ => None,
    }
}

struct Discovery {
    result: Result<BTreeSet<Relation>, String>,
    planned: usize,
    completed: usize,
    systems: Vec<Value>,
}

fn discover(
    base: &Base,
    target: Point,
    method: Method,
    solver: &Path,
    scratch: &Scratch,
    budget: Duration,
) -> Discovery {
    let deadline = Instant::now() + budget;
    #[cfg(feature = "production-controls")]
    if method == Method::Cofactor {
        return production::discover(base, target, deadline, budget);
    }
    #[cfg(feature = "production-controls")]
    if method == Method::CofactorMsolve {
        return production::discover_msolve(base, target, solver, scratch, deadline, budget);
    }
    let mut outcome = Discovery {
        result: Err("oracle budget exhausted".into()),
        planned: 0,
        completed: 0,
        systems: vec![],
    };
    if Instant::now() >= deadline {
        return outcome;
    }
    let charts = (method == Method::Refined).then(|| base.refinements());
    outcome.planned = charts.as_ref().map_or(1, Vec::len);
    let mut union = BTreeSet::new();
    for index in 0..outcome.planned {
        if Instant::now() >= deadline {
            return outcome;
        }
        let chart = charts.as_ref().map(|charts| &charts[index]);
        let system = if let Some(chart) = chart {
            System::at_cutoff(base.curve, target, base.depth - 3, chart.polynomial)
        } else {
            System::build(
                base.curve,
                target,
                base.depth,
                base.terminal,
                method.encoding(),
            )
        };
        let (variables, equations, terms, degree) = system.metrics();
        outcome.systems.push(json!({
            "variables":variables,"equations":equations,"terms":terms,"degree":degree,
            "fine_images":chart.map(|chart| chart.images),
        }));
        let result = run_solver(solver, &system.render(base.curve.p), deadline, scratch, &[])
            .and_then(|raw| extract(&raw, base, target, method.encoding()));
        let rows = match result {
            Ok(rows) => rows,
            Err(error) => {
                outcome.result = Err(error);
                return outcome;
            }
        };
        if Instant::now() >= deadline {
            return outcome;
        }
        for row in rows {
            if chart.is_some_and(|chart| !chart.contains(base, &row)) || !union.insert(row) {
                outcome.result = Err("overlapping or incorrect refinement chart".into());
                return outcome;
            }
        }
        outcome.completed += 1;
    }
    if Instant::now() < deadline {
        outcome.result = Ok(union);
    }
    outcome
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
    method: Option<Method>,
    mut seed: u64,
    max_draws: usize,
) {
    let base = Base::new(curve, depth);
    let pair_index = method.is_none().then(|| base.pair_index());
    let mut rows = Recovery {
        width: base.unsigned.len(),
        order: curve.n,
        pivots: BTreeMap::new(),
    };
    let mut visited = BTreeSet::from([None]);
    let label = method.map_or("Direct".into(), Method::name);
    let original_seed = seed;
    let mut scalar = None;
    let mut used = 0;
    let mut oracle_calls = 0;
    for attempt in 1..=max_draws {
        let a = next_mask(&mut seed, curve.n);
        let b = next_mask(&mut seed, curve.n);
        let residual = curve.add(curve.multiply(a, curve.g), curve.multiply(b, curve.q));
        if !visited.insert(residual) {
            continue;
        }
        let target = residual.expect("identity is excluded");
        oracle_calls += 1;
        let mut chart_counts = None;
        let relations = if let Some(method) = method {
            let discovery = discover(
                &base,
                target,
                method,
                solver,
                scratch,
                Duration::from_secs(3),
            );
            chart_counts = Some((discovery.planned, discovery.completed));
            discovery.result
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
                "a":a,"b":b,"target":[target.x,target.y],"result":result,"chart_counts":chart_counts,
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
            "base_size":base.unsigned.len(),"max_draws":max_draws,
        })
    );
}

fn main() -> Result<(), String> {
    let argv: Vec<_> = std::env::args().skip(1).collect();
    let [prime, depth, solver, modes @ ..] = argv.as_slice() else {
        return Err(
            "usage: dickson-four-summand PRIME DEPTH MSOLVE [recover=SEED [draws=1..256]] [distinct refined expanded quadratic cofactor(feature) cofactor-msolve(feature)]".into(),
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
    let (max_draws, modes) =
        if let Some(limit) = modes.first().and_then(|a| a.strip_prefix("draws=")) {
            if recovery_seed.is_none() {
                return Err("draws requires a recovery seed".into());
            }
            let limit: usize = limit.parse().map_err(|_| "invalid draw limit")?;
            if !(1..=256).contains(&limit) {
                return Err("draw limit outside 1..=256".into());
            }
            (limit, &modes[1..])
        } else {
            (30, modes)
        };
    let modes: Vec<_> = if modes.is_empty() {
        vec![Mode::Distinct, Mode::Expanded, Mode::Quadratic]
            .into_iter()
            .map(Method::Single)
            .collect()
    } else {
        modes
            .iter()
            .map(|label| mode(label).ok_or("unknown mode"))
            .collect::<Result<_, _>>()?
    };
    if depth > 4 && modes.contains(&Method::Single(Mode::Expanded)) {
        return Err("expanded control is limited to depths of at most four".into());
    }
    if depth < 3 && modes.contains(&Method::Refined) {
        return Err("refinement needs depth at least three".into());
    }
    if let Some(seed) = recovery_seed {
        recover(
            curve,
            depth,
            Path::new(solver),
            &scratch,
            None,
            seed,
            max_draws,
        );
        for mode in modes {
            recover(
                curve,
                depth,
                Path::new(solver),
                &scratch,
                Some(mode),
                seed,
                max_draws,
            );
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
            let want: BTreeSet<_> = expected
                .iter()
                .filter(|row| mode.encoding() != Mode::Distinct || base.separated(row))
                .copied()
                .collect();
            let discovery = discover(
                &base,
                target,
                *mode,
                Path::new(solver),
                &scratch,
                Duration::from_secs(3),
            );
            let mismatch = match &discovery.result {
                Ok(recovered) if recovered != &want => Some(format!(
                    "{mode:?} mismatch for {target:?}: {recovered:?} != {want:?}"
                )),
                _ => None,
            };
            let record = match discovery.result {
                Ok(recovered) if recovered == want => {
                    json!({"status":"exact","relations":recovered.len(),"rows":recovered})
                }
                Ok(recovered) => {
                    json!({"status":"mismatch","relations":recovered.len(),"rows":recovered,"expected_rows":want})
                }
                Err(error) => json!({"status":"undetermined","reason":error}),
            };
            println!(
                "{}",
                json!({
                    "prime":curve.p,"depth":depth,"target":[target.x,target.y],
                    "mode":mode.name(),"systems":discovery.systems,
                    "charts_planned":discovery.planned,"charts_completed":discovery.completed,
                    "query_budget_ms":3000,"expected":want.len(),"result":record,
                    "planted":trial>=unplanted,
                })
            );
            if let Some(error) = mismatch {
                return Err(error);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refinements_partition_all_regular_unsigned_tuples() {
        for (curve, depth) in [
            (FIXTURES[0], 3),
            (FIXTURES[0], 4),
            (FIXTURES[1], 4),
            (FIXTURES[1], 5),
            (FIXTURES[1], 6),
        ] {
            let base = Base::new(curve, depth);
            let charts = base.refinements();
            assert!(charts.len() <= 16);
            for chart in &charts {
                let mut polynomial = chart.polynomial.to_vec();
                polynomial.push(1);
                assert_eq!(
                    field_roots(polynomial, curve.p)
                        .unwrap()
                        .into_iter()
                        .collect::<BTreeSet<_>>(),
                    chart.images.into_iter().collect(),
                );
            }
            let n = base.unsigned.len() as i32;
            let mut regular = 0;
            for a in 1..=n {
                for b in a + 1..=n {
                    for c in b + 1..=n {
                        for d in c + 1..=n {
                            let row = [a, -b, c, -d];
                            let expected = usize::from(base.separated(&row));
                            let count = charts
                                .iter()
                                .filter(|chart| chart.contains(&base, &row))
                                .count();
                            assert_eq!(count, expected, "p={} depth={depth}, {row:?}", curve.p);
                            regular += expected;
                        }
                    }
                }
            }
            assert_eq!(regular == 0, charts.is_empty());
        }
    }

    #[test]
    fn exhausted_budget_never_calls_solver() {
        let base = Base::new(FIXTURES[0], 4);
        let scratch = Scratch::new().unwrap();
        for method in [Method::Single(Mode::Distinct), Method::Refined] {
            let result = discover(
                &base,
                base.curve.q,
                method,
                Path::new("/nonexistent-msolve"),
                &scratch,
                Duration::ZERO,
            );
            assert_eq!(result.completed, 0);
            assert_eq!(result.result.unwrap_err(), "oracle budget exhausted");
        }
    }

    #[cfg(unix)]
    #[test]
    fn failed_refinement_keeps_partial_union_undetermined() {
        use std::os::unix::fs::PermissionsExt;
        let scratch = Scratch::new().unwrap();
        let solver = scratch.0.join("partial-solver.sh");
        fs::write(&solver, "#!/bin/sh\nif [ -e \"$2.seen\" ]; then exit 9; fi\n: > \"$2.seen\"\nprintf '[-1]:' > \"$4\"\n").unwrap();
        fs::set_permissions(&solver, fs::Permissions::from_mode(0o700)).unwrap();
        let base = Base::new(FIXTURES[1], 5);
        let outcome = discover(
            &base,
            base.curve.q,
            Method::Refined,
            &solver,
            &scratch,
            Duration::from_secs(3),
        );
        assert!(outcome.planned > 1);
        assert_eq!(outcome.completed, 1);
        assert!(outcome.result.unwrap_err().starts_with("solver exit"));
    }
}
