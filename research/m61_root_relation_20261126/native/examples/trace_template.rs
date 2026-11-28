//! Two-parameter Dickson trace template: a bounded toy negative test.
//! The O(m²) support scan does not locate enough M61 relations.
use std::collections::{BTreeSet, HashMap};

const P: u64 = 131_071;
const R: u64 = 3_541;
const COFACTOR: u64 = 37;
const M: usize = 771;
const GENERATOR: u64 = 40_181;
const CONTROL_COUNT: usize = 16;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Point {
    Zero,
    Affine(u64, u64),
}

fn mul_field(a: u64, b: u64) -> u64 {
    a * b % P
}
fn power(mut a: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = result * a % modulus;
        }
        a = a * a % modulus;
        exponent >>= 1;
    }
    result
}
fn neg(point: Point) -> Point {
    match point {
        Point::Zero => Point::Zero,
        Point::Affine(x, y) => Point::Affine(x, if y == 0 { 0 } else { P - y }),
    }
}
fn add(left: Point, right: Point, additions: &mut u64) -> Point {
    *additions += 1;
    match (left, right) {
        (Point::Zero, point) | (point, Point::Zero) => point,
        (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
            if x1 == x2 && (y1 + y2) % P == 0 {
                return Point::Zero;
            }
            let (numerator, denominator) = if x1 == x2 {
                ((3 * mul_field(x1, x1) + P - 3) % P, 2 * y1 % P)
            } else {
                ((y2 + P - y1) % P, (x2 + P - x1) % P)
            };
            let slope = mul_field(numerator, power(denominator, P - 2, P));
            let x3 = (mul_field(slope, slope) + 2 * P - x1 - x2) % P;
            Point::Affine(x3, (mul_field(slope, (x1 + P - x3) % P) + P - y1) % P)
        }
    }
}
fn multiply(mut scalar: u64, mut point: Point, additions: &mut u64) -> Point {
    let mut result = Point::Zero;
    while scalar != 0 {
        if scalar & 1 != 0 {
            result = add(result, point, additions);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = add(point, point, additions);
        }
    }
    result
}
fn lift(x: u64) -> Option<Point> {
    let rhs = (mul_field(mul_field(x, x), x) + P - 3 * x % P + 40) % P;
    let y = power(rhs, (P + 1) / 4, P);
    (mul_field(y, y) == rhs).then_some(Point::Affine(x, y.min(P - y)))
}
fn orbit(exponent: usize) -> usize {
    let mut minimum = usize::MAX;
    for sign in [1_isize, -1] {
        for j in 0..3 {
            minimum = minimum.min(
                (sign * exponent as isize + j * (M / 3) as isize).rem_euclid(M as isize) as usize,
            );
        }
    }
    minimum
}
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

struct Base {
    projected: Vec<Point>,
    orbits: Vec<usize>,
    exponent_index: Vec<Option<usize>>,
}
fn make_base() -> Base {
    assert_eq!(power(GENERATOR, M as u64, P), 1);
    for divisor in [3, 257] {
        assert_ne!(power(GENERATOR, (M / divisor) as u64, P), 1);
    }
    let mut x_by_exponent = Vec::with_capacity(M);
    let mut x = 1;
    for _ in 0..M {
        x_by_exponent.push((x + power(x, P - 2, P)) % P);
        x = mul_field(x, GENERATOR);
    }
    assert_eq!(x, 1);
    let mut points_by_x = HashMap::new();
    let mut orbit_by_x = HashMap::new();
    for (exponent, &x) in x_by_exponent.iter().enumerate() {
        if let Some(point) = lift(x) {
            let projected = multiply(COFACTOR, point, &mut 0);
            if projected != Point::Zero {
                points_by_x.insert(x, projected);
                orbit_by_x.insert(x, orbit(exponent));
            }
        }
    }
    let mut sorted_x = points_by_x.keys().copied().collect::<Vec<_>>();
    sorted_x.sort_unstable();
    let x_index = sorted_x
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect::<HashMap<_, _>>();
    let exponent_index = x_by_exponent
        .iter()
        .map(|x| x_index.get(x).copied())
        .collect();
    Base {
        projected: sorted_x.iter().map(|x| points_by_x[x]).collect(),
        orbits: sorted_x.iter().map(|x| orbit_by_x[x]).collect(),
        exponent_index,
    }
}
fn template_supports(base: &Base) -> (BTreeSet<[usize; 7]>, usize) {
    let mut supports = BTreeSet::new();
    let mut admissible_pairs = 0;
    for i in 0..M {
        for j in 0..M {
            // Trace identities: x(tu)+x(t/u)=x(t)x(u),
            // x(t²)=x(t)²-2. All seven entries use these products/squares.
            let exponents = [i, j, i + j, (i + M - j) % M, 2 * i, 2 * j, 2 * (i + j)];
            let mut columns = [0_usize; 7];
            let mut valid = true;
            for (slot, exponent) in exponents.iter().enumerate() {
                if let Some(index) = base.exponent_index[exponent % M] {
                    columns[slot] = index;
                } else {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }
            let mut orbit_ids = columns.map(|column| base.orbits[column]);
            orbit_ids.sort_unstable();
            if orbit_ids.windows(2).any(|pair| pair[0] == pair[1]) {
                continue;
            }
            admissible_pairs += 1;
            columns.sort_unstable();
            assert!(columns.windows(2).all(|pair| pair[0] != pair[1]));
            supports.insert(columns);
        }
    }
    (supports, admissible_pairs)
}
fn random_supports(base: &Base, size: usize, seed: u64) -> BTreeSet<[usize; 7]> {
    let mut state = seed;
    let mut supports = BTreeSet::new();
    while supports.len() < size {
        let mut columns = [0_usize; 7];
        for entry in &mut columns {
            *entry = (splitmix64(&mut state) as usize) % base.projected.len();
        }
        columns.sort_unstable();
        if columns.windows(2).any(|pair| pair[0] == pair[1]) {
            continue;
        }
        let mut orbit_ids = columns.map(|column| base.orbits[column]);
        orbit_ids.sort_unstable();
        if orbit_ids.windows(2).any(|pair| pair[0] == pair[1]) {
            continue;
        }
        supports.insert(columns);
    }
    supports
}
fn signed_three(a: Point, b: Point, c: Point, additions: &mut u64) -> [Point; 8] {
    std::array::from_fn(|mask| {
        let first = if mask & 1 == 0 { a } else { neg(a) };
        let second = if mask & 2 == 0 { b } else { neg(b) };
        let third = if mask & 4 == 0 { c } else { neg(c) };
        add(add(first, second, additions), third, additions)
    })
}
fn insert_rank(mut row: Vec<u64>, basis: &mut [Option<Vec<u64>>]) -> bool {
    for pivot in 0..row.len() {
        if row[pivot] == 0 {
            continue;
        }
        if let Some(existing) = &basis[pivot] {
            let factor = row[pivot];
            for j in pivot..row.len() {
                row[j] = (row[j] + R - factor * existing[j] % R) % R;
            }
        } else {
            let inverse = power(row[pivot], R - 2, R);
            for entry in &mut row[pivot..] {
                *entry = *entry * inverse % R;
            }
            basis[pivot] = Some(row);
            return true;
        }
    }
    false
}
fn measure(supports: &BTreeSet<[usize; 7]>, base: &Base) -> (usize, usize, u64) {
    let mut zero_rows = 0;
    let mut additions = 0;
    let mut rank = 0;
    let mut basis = vec![None; base.projected.len()];
    for support in supports {
        let points = support.map(|column| base.projected[column]);
        let first_half = signed_three(points[1], points[2], points[3], &mut additions);
        let second_half = signed_three(points[4], points[5], points[6], &mut additions);
        for (first_mask, &first) in first_half.iter().enumerate() {
            for (second_mask, &second) in second_half.iter().enumerate() {
                if add(
                    add(points[0], first, &mut additions),
                    second,
                    &mut additions,
                ) != Point::Zero
                {
                    continue;
                }
                zero_rows += 1;
                let mut row = vec![0; base.projected.len()];
                row[support[0]] = 1;
                for bit in 0..3 {
                    row[support[bit + 1]] = if first_mask & (1 << bit) == 0 {
                        1
                    } else {
                        R - 1
                    };
                    row[support[bit + 4]] = if second_mask & (1 << bit) == 0 {
                        1
                    } else {
                        R - 1
                    };
                }
                // Replay the row independently of the 3+3 partial sums.
                let replay = row
                    .iter()
                    .enumerate()
                    .filter(|&(_, &coefficient)| coefficient != 0)
                    .fold(Point::Zero, |sum, (i, &coefficient)| {
                        add(
                            sum,
                            if coefficient == 1 {
                                base.projected[i]
                            } else {
                                neg(base.projected[i])
                            },
                            &mut additions,
                        )
                    });
                assert_eq!(replay, Point::Zero);
                rank += usize::from(insert_rank(row, &mut basis));
            }
        }
    }
    (zero_rows, rank, additions)
}

fn main() {
    let base = make_base();
    let (supports, admissible_pairs) = template_supports(&base);
    let (rows, rank, additions) = measure(&supports, &base);
    let controls = (0..CONTROL_COUNT)
        .map(|control| {
            let sample = random_supports(&base, supports.len(), 2026 + control as u64);
            measure(&sample, &base).0
        })
        .collect::<Vec<_>>();
    println!(
        "p={P} subgroup_order={R} torus_order={M} generator={GENERATOR} projected_columns={}",
        base.projected.len()
    );
    println!("parameter_pairs={} admissible_pairs={admissible_pairs} unique_seven_orbit_supports={} signed_candidates={} verified_zero_rows={rows} rank={rank} group_additions={additions}", M * M, supports.len(), supports.len() * 64);
    println!("uniform_zero_expectation={:.6} control_count={CONTROL_COUNT} control_zero_mean={:.3} control_zero_range={}..={} control_zero_counts={controls:?}", supports.len() as f64 * 64.0 / R as f64, controls.iter().sum::<usize>() as f64 / CONTROL_COUNT as f64, controls.iter().min().unwrap(), controls.iter().max().unwrap());
}
