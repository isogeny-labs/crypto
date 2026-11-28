//! Project synthetic coset relations to an odd prime subgroup and recover a toy scalar.
use super::*;

fn add(c: &Coset, p: Option<Point>, q: Option<Point>) -> Option<Point> {
    match (p, q) {
        (Some(p), Some(q)) => c.add(p, q, None),
        _ => p.or(q),
    }
}

fn scale(c: &Coset, mut p: Option<Point>, mut scalar: u64) -> Option<Point> {
    let mut result = None;
    while scalar != 0 {
        if scalar & 1 != 0 {
            result = add(c, result, p);
        }
        scalar >>= 1;
        if scalar != 0 {
            p = add(c, p, p);
        }
    }
    result
}

fn frobenius(c: &Coset, p: Point, k: u32) -> Point {
    (c.f.sqr_k(p.0, k), c.f.sqr_k(p.1, k))
}

fn ordinary_point(c: &Coset, seed: &mut u64) -> Point {
    loop {
        if let Some(point) = c.lift(random(seed) as u64 & ((1 << c.f.n) - 1)) {
            return point;
        }
    }
}

struct Group {
    order: u64,
    prime: u64,
    generator: Point,
    shift: Point,
    split_order: u64,
    split_negative: bool,
    eigenvalue: u64,
}

impl Group {
    fn new(c: &Coset, seed: &mut u64) -> Self {
        let order = curve_order(c.f.n);
        let prime = largest_prime_factor(order);
        assert_ne!(order / prime % prime, 0);
        let generator = loop {
            let point = ordinary_point(c, seed);
            if let Some(g) = scale(c, Some(point), order / prime) {
                break g;
            }
        };
        assert!(scale(c, Some(generator), prime).is_none());
        // Use an element of maximal 2-power order to enter the odd-sum coset.
        let two_power = 1 << order.trailing_zeros();
        let shift = loop {
            let point = ordinary_point(c, seed);
            let torsion = scale(c, Some(point), order / two_power);
            if scale(c, torsion, two_power / 2).is_some() {
                break torsion.unwrap();
            }
        };
        assert!(scale(c, Some(shift), order / prime).is_none());
        let split_image = frobenius(c, generator, c.k);
        let split_negative = split_image != generator;
        if split_negative {
            assert_eq!(split_image, negate(generator));
        }
        let positive_order = curve_order(c.k);
        let split_order = if split_negative {
            order / positive_order
        } else {
            positive_order
        };
        assert_eq!(split_order % prime, 0);
        let eigenvalue = (0..prime)
            .filter(|&m| (m * m + prime + 2 - m) % prime == 0)
            .find(|&m| scale(c, Some(generator), m) == Some(frobenius(c, generator, 1)))
            .unwrap();
        Self {
            order,
            prime,
            generator,
            shift,
            split_order,
            split_negative,
            eigenvalue,
        }
    }

    fn project(&self, c: &Coset, point: Point, split: bool) -> Option<Point> {
        if !split {
            return scale(c, Some(point), self.order / self.prime);
        }
        let other = frobenius(c, point, c.k);
        let component = c.add(
            point,
            if self.split_negative {
                negate(other)
            } else {
                other
            },
            None,
        );
        scale(c, component, self.split_order / self.prime)
    }
}

type Label = Option<(usize, u64)>;

fn columns(
    c: &Coset,
    group: &Group,
    points: &[Point],
    orbit: bool,
    split: bool,
) -> (Vec<Point>, HashMap<Point, Label>) {
    let mut reps = Vec::new();
    let mut found = HashMap::new();
    let mut labels = HashMap::new();
    for &point in points.iter().step_by(2) {
        let Some(image) = group.project(c, point, split) else {
            labels.insert(point, None);
            labels.insert(negate(point), None);
            continue;
        };
        let (column, weight) = *found.entry(image).or_insert((reps.len(), 1));
        if column == reps.len() {
            reps.push(image);
            let mut current = image;
            let mut factor = 1;
            loop {
                found.insert(current, (column, factor));
                found.insert(negate(current), (column, group.prime - factor));
                if !orbit {
                    break;
                }
                current = frobenius(c, current, 1);
                factor = factor * group.eigenvalue % group.prime;
                if current == image {
                    assert_eq!(factor, 1);
                    break;
                }
            }
        }
        labels.insert(point, Some((column, weight)));
        labels.insert(negate(point), Some((column, group.prime - weight)));
    }
    (reps, labels)
}

fn coefficients(
    relation: &Triple,
    labels: &HashMap<Point, Label>,
    width: usize,
    prime: u64,
) -> Vec<u64> {
    let mut row = vec![0; width];
    for point in relation {
        if let Some((i, weight)) = labels[point] {
            row[i] = (row[i] + weight) % prime;
        }
    }
    row
}

struct System {
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    prime: u64,
}

impl System {
    fn add(&mut self, mut row: Vec<u64>, rhs: u64) {
        let p = self.prime;
        row.push(rhs);
        for i in 0..self.pivots.len() {
            let v = row[i];
            if v == 0 {
                continue;
            }
            if let Some(prev) = &self.pivots[i] {
                for j in i..row.len() {
                    row[j] = (row[j] + (p - v) * prev[j]) % p;
                }
            } else {
                let inv = pow_mod(v, p - 2, p);
                for item in &mut row[i..] {
                    *item = *item * inv % p;
                }
                self.pivots[i] = Some(row);
                self.rank += 1;
                return;
            }
        }
        assert_eq!(*row.last().unwrap(), 0);
    }

    fn solve(&self) -> Vec<u64> {
        let n = self.pivots.len();
        let mut values = vec![0; n];
        for i in (0..n).rev() {
            let row = self.pivots[i].as_ref().unwrap();
            let known = (i + 1..n).fold(0, |v, j| (v + row[j] * values[j]) % self.prime);
            values[i] = (row[n] + self.prime - known) % self.prime;
        }
        values
    }
}

fn generic(c: &Coset, generator: Point, challenge: Point, prime: u64) -> u64 {
    let step = prime.isqrt() + 1;
    let mut babies = HashMap::new();
    let mut current = None;
    for i in 0..step {
        babies.insert(current, i);
        current = add(c, current, Some(generator));
    }
    let back = current.map(negate);
    let mut current = Some(challenge);
    for i in 0..step {
        if let Some(j) = babies.get(&current) {
            return (i * step + j) % prime;
        }
        current = add(c, current, back);
    }
    unreachable!();
}

fn solve(n: u32, k: u32, modulus: u64, panel: u64, method: &str) {
    let started = Instant::now();
    let c = Coset::new(n, k, modulus);
    let mut seed = (panel ^ ((n as u64) << 10)) | 1;
    let group = Group::new(&c, &mut seed);
    let prime = group.prime;
    let secret = 1 + random(&mut seed) as u64 % (prime - 1);
    let challenge = scale(&c, Some(group.generator), secret).unwrap();
    let base: HashMap<_, _> = span(&c.basis)
        .into_iter()
        .filter_map(|u| {
            let x = u ^ c.shift;
            c.lift(x).map(|p| (x, [p, negate(p)]))
        })
        .collect();
    let mut points: Vec<_> = base.values().flatten().copied().collect();
    points.sort_unstable();
    let orbit = method.starts_with("orbit");
    let split = method == "orbit-split";
    let (reps, labels) = columns(&c, &group, &points, orbit, split);
    let width = reps.len();
    assert!(width > 0);
    let mut system = System {
        pivots: vec![None; width],
        rank: 0,
        prime,
    };
    let projected_generator = group.project(&c, group.generator, split).unwrap();
    assert!(group.project(&c, group.shift, split).is_none());
    let setup = started.elapsed().as_secs_f64();
    let mut search = 0.0;
    let mut linear_algebra = 0.0;
    let mut verify = 0.0;
    let mut failed = 0;
    let mut triples = 0;
    let mut collection_attempts = 0;
    let mut descent_attempts = 0;
    let mut logs = None;
    let mut recovered = None;
    for _ in 0..128 {
        let started = Instant::now();
        let collecting = logs.is_none();
        let offset = (random(&mut seed) as u64) % prime;
        let shifted = scale(&c, Some(group.generator), offset);
        let full = add(&c, shifted, Some(group.shift));
        let target = if collecting {
            full
        } else {
            add(&c, full, Some(challenge))
        }
        .unwrap();
        let jobs: Vec<_> = points
            .iter()
            .filter_map(|&p| c.add(target, negate(p), None).map(|w| (w, p)))
            .collect();
        let relations = if method == "direct" {
            direct(&c, &points, &base, &jobs).0
        } else {
            joined(&c, &base, &jobs, true).0
        };
        search += started.elapsed().as_secs_f64();
        failed += usize::from(relations.is_empty());
        triples += relations.len();
        let started = Instant::now();
        if collecting {
            collection_attempts += 1;
            for relation in &relations {
                system.add(coefficients(relation, &labels, width, prime), offset);
                if system.rank == width {
                    break;
                }
            }
            if system.rank == width {
                logs = Some(system.solve());
            }
        } else {
            descent_attempts += 1;
            if let (Some(values), Some(relation)) = (&logs, relations.first()) {
                let scalar = coefficients(relation, &labels, width, prime)
                    .iter()
                    .zip(values)
                    .fold(0, |v, (&a, &b)| (v + a * b) % prime);
                recovered = Some((scalar + prime - offset) % prime);
            }
        }
        linear_algebra += started.elapsed().as_secs_f64();
        if recovered.is_some() {
            break;
        }
    }
    let started = Instant::now();
    if let Some(value) = recovered {
        assert_eq!(scale(&c, Some(group.generator), value), Some(challenge));
        for (&point, scalar) in reps.iter().zip(logs.unwrap()) {
            assert_eq!(scale(&c, Some(projected_generator), scalar), Some(point));
        }
        verify = started.elapsed().as_secs_f64();
    }
    let started = Instant::now();
    assert_eq!(generic(&c, group.generator, challenge, prime), secret);
    let generic_time = started.elapsed().as_secs_f64();
    println!(
        concat!(
        "{{\"n\":{},\"k\":{},\"seed\":{},\"method\":\"{}\",\"prime\":{},\"base_abscissae\":{},",
        "\"columns\":{},\"rank\":{},\"collection_attempts\":{},\"descent_attempts\":{},",
        "\"failed_attempts\":{},\"verified_triples\":{},\"recovered\":{},\"setup_seconds\":{:.9},",
        "\"search_seconds\":{:.9},\"linear_algebra_seconds\":{:.9},\"verify_seconds\":{:.9},",
        "\"generic_seconds\":{:.9}}}"
    ),
        n,
        k,
        panel,
        method,
        prime,
        base.len(),
        width,
        system.rank,
        collection_attempts,
        descent_attempts,
        failed,
        triples,
        recovered.map_or_else(|| "null".to_string(), |v| v.to_string()),
        setup,
        search,
        linear_algebra,
        verify,
        generic_time
    );
}

pub(super) fn run() {
    for (n, k, modulus) in [(14, 7, 0x4021), (18, 9, 0x40009), (22, 11, 0x400003)] {
        for panel in [11781661, 12675811] {
            for method in ["direct", "linear", "orbit-cofactor", "orbit-split"] {
                solve(n, k, modulus, panel, method);
            }
        }
    }
}
