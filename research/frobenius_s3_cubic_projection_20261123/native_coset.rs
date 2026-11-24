//! Matched toy S3 and signed triple experiments using the upstream word-field kernel.
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

#[allow(dead_code)]
mod upstream {
    pub struct IrreduciblePoly {
        pub degree: u32,
        pub low_terms: Vec<u32>,
    }
    include!(env!("S3_FIELD_KERNEL"));
}
use upstream::{Gf2, IrreduciblePoly};
mod higher_degree;
mod native_recovery;

fn span(basis: &[u64]) -> Vec<u64> {
    let mut values = vec![0];
    for &b in basis {
        for i in 0..values.len() {
            values.push(values[i] ^ b);
        }
    }
    values
}

fn apply(basis: &[u64], mut bits: u64) -> u64 {
    let mut result = 0;
    while bits != 0 {
        result ^= basis[bits.trailing_zeros() as usize];
        bits &= bits - 1;
    }
    result
}

struct BinaryMap {
    pivots: [(u64, u64); 64],
    free: Vec<u64>,
}
impl BinaryMap {
    fn new(columns: impl IntoIterator<Item = u64>) -> Self {
        let mut result = Self {
            pivots: [(0, 0); 64],
            free: Vec::new(),
        };
        for (i, value) in columns.into_iter().enumerate() {
            let (image, preimage) = result.reduce(value, 1 << i);
            if image == 0 {
                result.free.push(preimage);
            } else {
                result.pivots[image.ilog2() as usize] = (image, preimage);
            }
        }
        result
    }
    fn reduce(&self, mut image: u64, mut preimage: u64) -> (u64, u64) {
        while image != 0 {
            let (row, recipe) = self.pivots[image.ilog2() as usize];
            if row == 0 {
                break;
            }
            image ^= row;
            preimage ^= recipe;
        }
        (image, preimage)
    }
    fn solve(&self, image: u64) -> Option<u64> {
        let (left, preimage) = self.reduce(image, 0);
        (left == 0).then_some(preimage)
    }
}

type Point = (u64, u64);
type Triple = [Point; 3];
type Pairs = BTreeSet<(u64, u64)>;

struct Coset {
    f: Gf2,
    k: u32,
    basis: Vec<u64>,
    squares: Vec<u64>,
    shift: u64,
    field_as: BinaryMap,
    subfield_as: BinaryMap,
}
impl Coset {
    fn new(n: u32, k: u32, polynomial: u64) -> Self {
        let f = Gf2::new(&IrreduciblePoly {
            degree: n,
            low_terms: (0..n).filter(|j| polynomial >> j & 1 != 0).collect(),
        });
        let powers = (0..n).map(|j| 1 << j);
        let difference = BinaryMap::new(powers.clone().map(|v| f.sqr_k(v, k) ^ v));
        let shift = if n == 2 * k {
            difference.solve(1).unwrap()
        } else {
            powers.clone().find(|&v| f.sqr_k(v, k) != v).unwrap()
        };
        let basis = difference.free;
        assert_eq!(basis.len(), k as usize);
        let squares = basis.iter().map(|&v| f.sqr(v)).collect();
        let field_as = BinaryMap::new(powers.map(|v| f.sqr(v) ^ v));
        let subfield_as = BinaryMap::new(basis.iter().map(|&v| f.sqr(v) ^ v));
        Self {
            f,
            k,
            basis,
            squares,
            shift,
            field_as,
            subfield_as,
        }
    }

    fn s3(&self, x: u64, y: u64, t: u64) -> u64 {
        let f = &self.f;
        let p = f.mul(x, y);
        f.sqr(p) ^ f.mul(t, p) ^ f.sqr(f.mul(t, x ^ y)) ^ 1
    }

    fn pairs(&self, t: u64, cached: bool) -> (Pairs, usize, usize) {
        let f = &self.f;
        let a2 = f.sqr(self.shift);
        let s2_coeff = a2 ^ f.sqr(t);
        let s_coeff = f.mul(t, self.shift);
        let sums = self
            .basis
            .iter()
            .zip(&self.squares)
            .map(|(&b, &b2)| f.mul(s2_coeff, b2) ^ f.mul(s_coeff, b));
        let products = self
            .basis
            .iter()
            .zip(&self.squares)
            .map(|(&b, &b2)| b2 ^ f.mul(t, b));
        let matrix = BinaryMap::new(sums.chain(products));
        let nullity = matrix.free.len();
        assert!(nullity <= 2);
        let rhs = f.sqr(a2) ^ f.mul(t, a2) ^ 1;
        let Some(point) = matrix.solve(rhs) else {
            return (Pairs::new(), 0, nullity);
        };
        let offsets = span(&matrix.free);
        let mut output = Pairs::new();
        for &offset in &offsets {
            let state = point ^ offset;
            let s = apply(&self.basis, state & ((1 << self.k) - 1));
            let p = apply(&self.basis, state >> self.k);
            let root = if !cached {
                BinaryMap::new(
                    self.basis
                        .iter()
                        .zip(&self.squares)
                        .map(|(&b, &b2)| b2 ^ f.mul(s, b)),
                )
                .solve(p)
                .map(|bits| apply(&self.basis, bits))
            } else if s == 0 {
                Some(f.sqr_k(p, self.k - 1))
            } else {
                self.subfield_as
                    .solve(f.mul(p, f.inv(f.sqr(s))))
                    .map(|bits| f.mul(s, apply(&self.basis, bits)))
            };
            if let Some(u) = root {
                let x = u ^ self.shift;
                let y = x ^ s;
                output.insert((x.min(y), x.max(y)));
            }
        }
        (output, offsets.len(), nullity)
    }

    fn lift(&self, x: u64) -> Option<Point> {
        if x == 0 {
            return Some((0, 1));
        }
        let f = &self.f;
        let z = self.field_as.solve(x ^ 1 ^ f.sqr(f.inv(x)))?;
        Some((x, f.mul(x, z)))
    }

    fn add(&self, p: Point, q: Point, inverse: Option<u64>) -> Option<Point> {
        let ((x, y), (u, v)) = (p, q);
        let f = &self.f;
        if x == u {
            if y != v || x == 0 {
                return None;
            }
            let slope = x ^ f.mul(y, f.inv(x));
            let a = f.sqr(slope) ^ slope ^ 1;
            return Some((a, f.sqr(x) ^ f.mul(slope ^ 1, a)));
        }
        let slope = f.mul(y ^ v, inverse.unwrap_or_else(|| f.inv(x ^ u)));
        let a = f.sqr(slope) ^ slope ^ x ^ u ^ 1;
        Some((a, f.mul(slope, x ^ a) ^ a ^ y))
    }
}

fn negate((x, y): Point) -> Point {
    (x, y ^ x)
}

fn triple(p: Point, q: Point, r: Point) -> Option<Triple> {
    if p.0 == q.0 || p.0 == r.0 || q.0 == r.0 {
        return None;
    }
    let mut result = [p, q, r];
    result.sort_unstable();
    Some(result)
}

fn direct(
    c: &Coset,
    points: &[Point],
    base: &HashMap<u64, [Point; 2]>,
    jobs: &[(Point, Point)],
) -> (BTreeSet<Triple>, usize) {
    let mut found = BTreeSet::new();
    let mut scratch = Vec::new();
    for &(w, third) in jobs {
        let mut reciprocals: Vec<_> = points.iter().map(|p| p.0 ^ w.0).collect();
        c.f.batch_inv(&mut reciprocals, &mut scratch);
        for (&p, inverse) in points.iter().zip(reciprocals) {
            if let Some(q) = c.add(w, negate(p), Some(inverse)) {
                if base.contains_key(&q.0) {
                    found.extend(triple(p, q, third));
                }
            }
        }
    }
    (found, points.len() * jobs.len())
}

fn joined(
    c: &Coset,
    base: &HashMap<u64, [Point; 2]>,
    jobs: &[(Point, Point)],
    cached: bool,
) -> (BTreeSet<Triple>, usize, usize) {
    let mut found = BTreeSet::new();
    let mut targets = HashMap::new();
    let mut candidates = 0;
    for &(w, third) in jobs {
        let pairs = targets.entry(w.0).or_insert_with(|| {
            let (pairs, count, _) = c.pairs(w.0, cached);
            candidates += count;
            pairs
        });
        for &(x, y) in pairs.iter() {
            if let (Some(left), Some(right)) = (base.get(&x), base.get(&y)) {
                for (&p, &q) in left.iter().flat_map(|p| right.iter().map(move |q| (p, q))) {
                    if let Some(relation) = triple(p, q, third) {
                        if c.add(p, q, None) == Some(w) {
                            found.insert(relation);
                        }
                    }
                }
            }
        }
    }
    (found, targets.len(), candidates)
}

fn random(state: &mut u64) -> usize {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state as usize
}

fn largest_prime_factor(mut order: u64) -> u64 {
    let mut largest = 1;
    let mut p = 2;
    while p * p <= order {
        while order % p == 0 {
            largest = p;
            order /= p;
        }
        p += 1;
    }
    largest.max(order)
}

fn curve_order(n: u32) -> u64 {
    let (mut previous, mut trace) = (2i64, 1i64);
    for _ in 1..n {
        (previous, trace) = (trace, trace - 2 * previous);
    }
    ((1i64 << n) + 1 - trace) as u64
}

fn subgroup_prime(n: u32) -> u64 {
    largest_prime_factor(curve_order(n))
}

fn pow_mod(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut value = 1;
    while b != 0 {
        if b & 1 != 0 {
            value = value * a % p;
        }
        a = a * a % p;
        b >>= 1;
    }
    value
}

fn add_rows(
    relations: &BTreeSet<Triple>,
    columns: &HashMap<u64, usize>,
    pivots: &mut [Option<Vec<u64>>],
    prime: u64,
    rank: &mut usize,
) {
    for relation in relations {
        if *rank == pivots.len() {
            break;
        }
        let mut row = vec![0; columns.len()];
        for &point @ (x, y) in relation {
            row[columns[&x]] = if y < negate(point).1 { 1 } else { prime - 1 };
        }
        for i in 0..row.len() {
            let coefficient = row[i];
            if coefficient == 0 {
                continue;
            }
            if let Some(ref pivot) = pivots[i] {
                for j in i..row.len() {
                    row[j] = (row[j] + (prime - coefficient) * pivot[j]) % prime;
                }
            } else {
                let inverse = pow_mod(coefficient, prime - 2, prime);
                for v in &mut row[i..] {
                    *v = *v * inverse % prime;
                }
                pivots[i] = Some(row);
                *rank += 1;
                break;
            }
        }
    }
}

fn exhaustive(c: &Coset, values: &[u64]) {
    for t in 0..(1 << c.f.n) {
        let mut reference = Pairs::new();
        for (i, &x) in values.iter().enumerate() {
            for &y in &values[i..] {
                if c.s3(x, y, t) == 0 {
                    reference.insert((x.min(y), x.max(y)));
                }
            }
        }
        assert_eq!(reference, c.pairs(t, false).0);
        assert_eq!(reference, c.pairs(t, true).0);
    }
}

fn benchmark(n: u32, k: u32, polynomial: u64, trials: usize, panel: u64) {
    let started = Instant::now();
    let c = Coset::new(n, k, polynomial);
    let values: Vec<_> = span(&c.basis).iter().map(|v| v ^ c.shift).collect();
    let base: HashMap<_, _> = values
        .iter()
        .filter_map(|&x| c.lift(x).map(|p| (x, [p, negate(p)])))
        .collect();
    let mut points: Vec<_> = base.values().flatten().copied().collect();
    points.sort_unstable();
    let columns: HashMap<_, _> = points
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(i, p)| (p.0, i))
        .collect();
    let setup = started.elapsed().as_secs_f64();
    if n <= 10 {
        exhaustive(&c, &values);
        println!(
            "{{\"kind\":\"exhaustive\",\"n\":{n},\"k\":{k},\"targets\":{}}}",
            1u64 << n
        );
    }
    if base.len() < 3 {
        return;
    }
    let prime = subgroup_prime(n);
    let mut seed = (panel ^ ((n as u64) << 10) ^ k as u64) | 1;
    for stratum in ["uniform", "planted"] {
        let mut direct_checks = 0;
        let mut candidates = 0;
        let mut intermediate_count = 0;
        let mut triples = 0;
        let mut empty = 0;
        let mut pivots = vec![None; base.len()];
        let mut rank = 0;
        let mut rank_after = Vec::new();
        let mut full_rank = None;
        let (mut shared, mut direct_time, mut linear_time, mut cached_time, mut rank_time) =
            (0.0, 0.0, 0.0, 0.0, 0.0);
        for attempt in 0..trials {
            let started = Instant::now();
            let target = if stratum == "uniform" {
                loop {
                    let x = random(&mut seed) as u64 & ((1 << n) - 1);
                    if let Some(p) = c.lift(x) {
                        let opposite = random(&mut seed) & 1 != 0;
                        if x == 0 && opposite {
                            continue;
                        }
                        break if opposite { negate(p) } else { p };
                    }
                }
            } else {
                loop {
                    let [p, q, r] = [(); 3].map(|()| points[random(&mut seed) % points.len()]);
                    if triple(p, q, r).is_some() {
                        let sum = c.add(p, q, None).and_then(|w| c.add(w, r, None));
                        if let Some(w) = sum {
                            break w;
                        }
                    }
                }
            };
            let jobs: Vec<_> = points
                .iter()
                .filter_map(|&p| c.add(target, negate(p), None).map(|w| (w, p)))
                .collect();
            shared += started.elapsed().as_secs_f64();

            let arms = if attempt % 2 == 0 {
                [0, 1, 2]
            } else {
                [2, 1, 0]
            };
            let mut verified = None;
            for arm in arms {
                let elapsed = Instant::now();
                let found = if arm == 0 {
                    let (found, count) = direct(&c, &points, &base, &jobs);
                    direct_time += elapsed.elapsed().as_secs_f64();
                    direct_checks += count;
                    found
                } else {
                    let (found, intermediates, parameters) = joined(&c, &base, &jobs, arm == 2);
                    if arm == 1 {
                        linear_time += elapsed.elapsed().as_secs_f64();
                        intermediate_count += intermediates;
                        candidates += parameters;
                    } else {
                        cached_time += elapsed.elapsed().as_secs_f64();
                    }
                    found
                };
                match &verified {
                    None => verified = Some(found),
                    Some(reference) => assert_eq!(*reference, found),
                }
            }
            let verified = verified.unwrap();
            if stratum == "planted" {
                assert!(!verified.is_empty());
            }
            empty += usize::from(verified.is_empty());
            triples += verified.len();
            let started = Instant::now();
            add_rows(&verified, &columns, &mut pivots, prime, &mut rank);
            rank_time += started.elapsed().as_secs_f64();
            rank_after.push(rank);
            if rank == base.len() && full_rank.is_none() {
                let common = setup + shared + rank_time;
                full_rank = Some((
                    attempt + 1,
                    common + direct_time,
                    common + linear_time,
                    common + cached_time,
                ));
            }
        }
        let to_rank = match full_rank {
            Some((attempts, direct, linear, cached)) => {
                format!("[{attempts},{direct:.9},{linear:.9},{cached:.9}]")
            }
            None => "null".to_string(),
        };
        let base_size = base.len();
        let kernel = c.f.kernel_name();
        println!(
            concat!(
                "{{\"kind\":\"signed_join\",\"n\":{},\"k\":{},\"seed\":{},\"stratum\":\"{}\",\"targets\":{},",
                "\"base_abscissae\":{},\"prime\":{},\"kernel\":\"{}\",\"empty\":{},",
                "\"triples\":{},\"rank\":{},\"direct_checks\":{},\"intermediate_abscissae\":{},",
                "\"parameter_candidates\":{},\"setup_seconds\":{:.9},\"shared_seconds\":{:.9},",
                "\"direct_seconds\":{:.9},\"linear_seconds\":{:.9},\"cached_seconds\":{:.9},",
                "\"rank_seconds\":{:.9},\"rank_after\":{:?},\"full_rank_prefix\":{}}}"
            ),
            n,
            k,
            panel,
            stratum,
            trials,
            base_size,
            prime,
            kernel,
            empty,
            triples,
            rank,
            direct_checks,
            intermediate_count,
            candidates,
            setup,
            shared,
            direct_time,
            linear_time,
            cached_time,
            rank_time,
            rank_after,
            to_rank
        );
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("recovery") => return native_recovery::run(),
        Some("higher-degree") => return higher_degree::run(),
        _ => {}
    }
    let trials = std::env::args()
        .nth(1)
        .map(|v| v.parse().unwrap())
        .unwrap_or(6);
    assert!((1..=64).contains(&trials));
    let panel = std::env::args()
        .nth(2)
        .map(|v| v.parse().unwrap())
        .unwrap_or(0xb3c61d);
    for (n, k, modulus) in [
        (6, 3, 0x43),
        (9, 3, 0x203),
        (10, 5, 0x409),
        (12, 4, 0x1009),
        (14, 7, 0x4021),
        (15, 5, 0x8003),
        (18, 6, 0x40009),
        (18, 9, 0x40009),
        (22, 11, 0x400003),
    ] {
        benchmark(n, k, modulus, trials, panel);
    }
}
