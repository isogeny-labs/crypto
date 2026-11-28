//! Actual seven-column relation rank on the certified p=131071 toy curve.
//! The 3+4 meet in the middle is a toy diagnostic, not an M61 locator.
use std::collections::{HashMap, HashSet};

const P: u64 = 131_071;
const R: u64 = 3_541;
const H: u64 = 37;
const ROOT_ORDER: u64 = 102;
const ROOT_GENERATOR: u64 = 37_206;

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
    if rhs == 0 {
        return Some(Point::Affine(x, 0));
    }
    let y = power(rhs, (P + 1) / 4, P);
    (mul_field(y, y) == rhs).then_some(Point::Affine(x, y.min(P - y)))
}
fn roots() -> Vec<Point> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    let mut x = 1;
    for _ in 0..ROOT_ORDER {
        assert!(seen.insert(x));
        if let Some(Point::Affine(x, y)) = lift(x) {
            if y != 0 {
                result.push(Point::Affine(x, y));
            }
        }
        x = mul_field(x, ROOT_GENERATOR);
    }
    assert_eq!(x, 1);
    result
}
fn trace_roots(order: usize, generator: u64) -> (Vec<Point>, Vec<Vec<usize>>) {
    assert_eq!(order % 3, 0);
    let mut powers = Vec::with_capacity(order);
    let mut t = 1;
    for _ in 0..order {
        powers.push(t);
        t = mul_field(t, generator);
    }
    assert_eq!(t, 1);
    assert_eq!(powers.iter().copied().collect::<HashSet<_>>().len(), order);
    let trace = (0..order)
        .map(|i| (powers[i] + powers[(order - i) % order]) % P)
        .collect::<Vec<_>>();
    let mut seen_parameter = vec![false; order];
    let mut seen_x = HashSet::new();
    let mut base = Vec::new();
    let mut free_rows = Vec::new();
    for i in 0..order {
        if seen_parameter[i] {
            continue;
        }
        let d = order / 3;
        for j in [i, (i + d) % order, (i + 2 * d) % order] {
            seen_parameter[j] = true;
            seen_parameter[(order - j) % order] = true;
        }
        let xs = [trace[i], trace[(i + d) % order], trace[(i + 2 * d) % order]];
        if let Some(Point::Affine(_, y)) = lift(xs[0]) {
            let orbit = xs.map(|x| Point::Affine(x, y));
            assert_eq!(
                orbit
                    .into_iter()
                    .fold(Point::Zero, |acc, p| add(acc, p, &mut 0)),
                Point::Zero
            );
            if orbit
                .iter()
                .all(|&point| multiply(H, point, &mut 0) != Point::Zero)
            {
                let mut orbit_columns = HashMap::new();
                for x in xs.into_iter().collect::<HashSet<_>>() {
                    assert!(seen_x.insert(x));
                    orbit_columns.insert(x, base.len());
                    base.push(Point::Affine(x, y));
                }
                free_rows.push(xs.map(|x| orbit_columns[&x]).to_vec());
            }
        }
    }
    assert!(seen_parameter.iter().all(|&seen| seen));
    assert_eq!(seen_x.len(), base.len());
    (base, free_rows)
}
fn next_random(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn random_base(size: usize, seed: u64) -> Vec<Point> {
    let mut state = seed;
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    while result.len() < size {
        let x = 1 + next_random(&mut state) % (P - 1);
        if seen.insert(x) {
            if let Some(point) = lift(x) {
                if let Point::Affine(_, y) = point {
                    if y != 0 && multiply(H, point, &mut 0) != Point::Zero {
                        result.push(point);
                    }
                }
            }
        }
    }
    result
}

#[derive(Clone, Copy)]
struct Triple {
    ids: [u8; 3],
    signs: u8,
    columns: u32,
}
fn signed(point: Point, mask: u8, bit: u8) -> Point {
    if mask & (1 << bit) == 0 {
        point
    } else {
        neg(point)
    }
}
fn row_codes(triple: Triple, quartet: [usize; 4], quartet_signs: u8, powers: &[u64]) -> (u64, u64) {
    let mut code = 0;
    let mut opposite = 0;
    for (bit, &id) in triple.ids.iter().enumerate() {
        let digit = if triple.signs & (1 << bit) == 0 { 1 } else { 2 };
        code += digit * powers[id as usize];
        opposite += (3 - digit) * powers[id as usize];
    }
    for (bit, &id) in quartet.iter().enumerate() {
        let digit = if quartet_signs & (1 << bit) == 0 {
            1
        } else {
            2
        };
        code += digit * powers[id];
        opposite += (3 - digit) * powers[id];
    }
    (code, opposite)
}
fn row_from_code(mut code: u64, columns: usize) -> Vec<u64> {
    let mut row = vec![0; columns];
    for entry in &mut row {
        *entry = match code % 3 {
            0 => 0,
            1 => 1,
            2 => R - 1,
            _ => unreachable!(),
        };
        code /= 3;
    }
    assert_eq!(code, 0);
    row
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

struct ResultRow {
    triples: usize,
    quartets: usize,
    raw_matches: u64,
    distinct_rows: usize,
    orbit_distinct_rows: usize,
    orbit_distinct_rank: usize,
    rank: usize,
    additions: u64,
    orbit_basis: Vec<Option<Vec<u64>>>,
}
fn measure(base: &[Point], free_rows: &[Vec<usize>], seed_free: bool) -> ResultRow {
    let b = base.len();
    assert!(b <= 21);
    let mut additions = 0;
    let projected = base
        .iter()
        .map(|&point| multiply(H, point, &mut additions))
        .collect::<Vec<_>>();
    assert!(projected.iter().all(|&point| point != Point::Zero));
    assert!(projected
        .iter()
        .all(|&point| multiply(R, point, &mut additions) == Point::Zero));
    let mut triples: HashMap<Point, Vec<Triple>> = HashMap::new();
    let mut triple_count = 0;
    for i in 0..b {
        for j in i + 1..b {
            for k in j + 1..b {
                for signs in 0..8 {
                    let sum = add(
                        add(
                            signed(projected[i], signs, 0),
                            signed(projected[j], signs, 1),
                            &mut additions,
                        ),
                        signed(projected[k], signs, 2),
                        &mut additions,
                    );
                    triples.entry(sum).or_default().push(Triple {
                        ids: [i as u8, j as u8, k as u8],
                        signs,
                        columns: (1 << i) | (1 << j) | (1 << k),
                    });
                    triple_count += 1;
                }
            }
        }
    }
    let mut powers = vec![1; b];
    for i in 1..b {
        powers[i] = powers[i - 1] * 3;
    }
    let mut seen_rows = HashSet::new();
    let mut basis = vec![None; b];
    let mut orbit_basis = vec![None; b];
    let mut orbit_distinct_rows = 0;
    let mut orbit_distinct_rank = 0;
    let mut orbit_ids = vec![usize::MAX; b];
    for (id, indices) in free_rows.iter().enumerate() {
        for &index in indices {
            assert!(orbit_ids[index] == usize::MAX || orbit_ids[index] == id);
            orbit_ids[index] = id;
        }
    }
    if !free_rows.is_empty() {
        assert!(orbit_ids.iter().all(|&id| id != usize::MAX));
    }
    let mut rank = 0;
    for indices in free_rows.iter().filter(|_| seed_free) {
        let mut row = vec![0; b];
        for &index in indices {
            row[index] += 1;
        }
        let sum = indices.iter().fold(Point::Zero, |acc, &index| {
            add(acc, projected[index], &mut additions)
        });
        assert_eq!(sum, Point::Zero);
        assert!(insert_rank(row, &mut basis));
        rank += 1;
    }
    let mut raw_matches = 0;
    let mut quartet_count = 0;
    for i in 0..b {
        for j in i + 1..b {
            for k in j + 1..b {
                for l in k + 1..b {
                    let quartet = [i, j, k, l];
                    let bits = (1 << i) | (1 << j) | (1 << k) | (1 << l);
                    for signs in 0..16 {
                        let left = add(
                            signed(projected[i], signs, 0),
                            signed(projected[j], signs, 1),
                            &mut additions,
                        );
                        let right = add(
                            signed(projected[k], signs, 2),
                            signed(projected[l], signs, 3),
                            &mut additions,
                        );
                        let sum = add(left, right, &mut additions);
                        quartet_count += 1;
                        if let Some(matches) = triples.get(&neg(sum)) {
                            for &triple in matches {
                                if triple.columns & bits != 0 {
                                    continue;
                                }
                                raw_matches += 1;
                                let (code, opposite) = row_codes(triple, quartet, signs, &powers);
                                let canonical = code.min(opposite);
                                if seen_rows.insert(canonical) {
                                    let row = row_from_code(canonical, b);
                                    assert_eq!(row.iter().filter(|&&entry| entry != 0).count(), 7);
                                    let sum = row
                                        .iter()
                                        .enumerate()
                                        .filter(|&(_, &value)| value != 0)
                                        .fold(Point::Zero, |acc, (index, &value)| {
                                            add(
                                                acc,
                                                if value == 1 {
                                                    projected[index]
                                                } else {
                                                    neg(projected[index])
                                                },
                                                &mut additions,
                                            )
                                        });
                                    assert_eq!(sum, Point::Zero);
                                    if !free_rows.is_empty() {
                                        let ids = triple
                                            .ids
                                            .iter()
                                            .map(|&index| orbit_ids[index as usize])
                                            .chain(quartet.iter().map(|&index| orbit_ids[index]))
                                            .collect::<HashSet<_>>();
                                        if ids.len() == 7 {
                                            orbit_distinct_rows += 1;
                                            orbit_distinct_rank += usize::from(insert_rank(
                                                row.clone(),
                                                &mut orbit_basis,
                                            ));
                                        }
                                    }
                                    rank += usize::from(insert_rank(row, &mut basis));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(rank < b);
    ResultRow {
        triples: triple_count,
        quartets: quartet_count,
        raw_matches,
        distinct_rows: seen_rows.len(),
        orbit_distinct_rows,
        orbit_distinct_rank,
        rank,
        additions,
        orbit_basis,
    }
}

fn null_vector(basis: &[Option<Vec<u64>>]) -> Vec<u64> {
    let free = basis.iter().position(Option::is_none).unwrap();
    assert_eq!(
        basis.iter().filter(|row| row.is_some()).count(),
        basis.len() - 1
    );
    let mut vector = vec![0; basis.len()];
    vector[free] = 1;
    for pivot in (0..basis.len()).rev() {
        if let Some(row) = &basis[pivot] {
            let rest = (pivot + 1..basis.len())
                .map(|j| row[j] * vector[j] % R)
                .fold(0, |acc, value| (acc + value) % R);
            vector[pivot] = (R - rest) % R;
        }
    }
    for row in basis.iter().flatten() {
        let dot = row
            .iter()
            .zip(&vector)
            .map(|(&a, &b)| a * b % R)
            .fold(0, |acc, value| (acc + value) % R);
        assert_eq!(dot, 0);
    }
    vector
}

fn six_descent(target: Point, base: &[Point], orbit_ids: &[usize]) -> Vec<(usize, i8)> {
    let mut triples: HashMap<Point, Vec<Triple>> = HashMap::new();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for k in j + 1..base.len() {
                for signs in 0..8 {
                    let sum = add(
                        add(signed(base[i], signs, 0), signed(base[j], signs, 1), &mut 0),
                        signed(base[k], signs, 2),
                        &mut 0,
                    );
                    triples.entry(sum).or_default().push(Triple {
                        ids: [i as u8, j as u8, k as u8],
                        signs,
                        columns: (1 << i) | (1 << j) | (1 << k),
                    });
                }
            }
        }
    }
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for k in j + 1..base.len() {
                for signs in 0..8 {
                    let sum = add(
                        add(signed(base[i], signs, 0), signed(base[j], signs, 1), &mut 0),
                        signed(base[k], signs, 2),
                        &mut 0,
                    );
                    let needed = add(target, neg(sum), &mut 0);
                    if let Some(matches) = triples.get(&needed) {
                        for &first in matches {
                            if first.columns & ((1 << i) | (1 << j) | (1 << k)) != 0 {
                                continue;
                            }
                            let ids = first
                                .ids
                                .iter()
                                .map(|&index| orbit_ids[index as usize])
                                .chain([i, j, k].map(|index| orbit_ids[index]))
                                .collect::<HashSet<_>>();
                            if ids.len() != 6 {
                                continue;
                            }
                            let terms = first
                                .ids
                                .iter()
                                .enumerate()
                                .map(|(bit, &index)| {
                                    (
                                        index as usize,
                                        if first.signs & (1 << bit) == 0 { 1 } else { -1 },
                                    )
                                })
                                .chain([i, j, k].into_iter().enumerate().map(|(bit, index)| {
                                    (index, if signs & (1 << bit) == 0 { 1 } else { -1 })
                                }))
                                .collect::<Vec<_>>();
                            let replay = terms.iter().fold(Point::Zero, |acc, &(index, sign)| {
                                add(
                                    acc,
                                    if sign == 1 {
                                        base[index]
                                    } else {
                                        neg(base[index])
                                    },
                                    &mut 0,
                                )
                            });
                            assert_eq!(replay, target);
                            return terms;
                        }
                    }
                }
            }
        }
    }
    panic!("toy target has no six-column descent");
}

fn dot_terms(terms: &[(usize, i8)], vector: &[u64]) -> u64 {
    terms.iter().fold(0, |acc, &(index, sign)| {
        (acc + if sign == 1 {
            vector[index]
        } else {
            (R - vector[index]) % R
        }) % R
    })
}

fn six_orbit_coverage(base: &[Point], orbit_ids: &[usize], generator: Point) -> (usize, u32, u32) {
    let orbit_count = 1 + *orbit_ids.iter().max().unwrap();
    let mut groups = vec![Vec::new(); orbit_count];
    for (index, &point) in base.iter().enumerate() {
        groups[orbit_ids[index]].extend([point, neg(point)]);
    }
    assert_eq!(orbit_count, 7);
    assert!(groups.iter().all(|group| group.len() == 6));
    fn visit(
        groups: &[Vec<Point>],
        skip: usize,
        at: usize,
        sum: Point,
        counts: &mut HashMap<Point, u32>,
    ) {
        if at == groups.len() {
            *counts.entry(sum).or_default() += 1;
            return;
        }
        if at == skip {
            visit(groups, skip, at + 1, sum, counts);
        } else {
            for &point in &groups[at] {
                visit(groups, skip, at + 1, add(sum, point, &mut 0), counts);
            }
        }
    }
    let mut counts = HashMap::new();
    for skip in 0..orbit_count {
        visit(&groups, skip, 0, Point::Zero, &mut counts);
    }
    assert_eq!(
        counts.values().map(|&count| count as u64).sum::<u64>(),
        7 * 6_u64.pow(6)
    );
    let mut point = Point::Zero;
    for _ in 0..R {
        assert!(counts.contains_key(&point));
        point = add(point, generator, &mut 0);
    }
    assert_eq!(point, Point::Zero);
    (
        counts.len(),
        *counts.values().min().unwrap(),
        *counts.values().max().unwrap(),
    )
}

fn main() {
    let root = roots();
    let root_nonzero = root
        .iter()
        .copied()
        .filter(|&point| multiply(H, point, &mut 0) != Point::Zero)
        .collect::<Vec<_>>();
    println!(
        "p={P} order={} cofactor={H} subgroup_order={R} root_valid={} root_nonzero={}",
        H * R,
        root.len(),
        root_nonzero.len()
    );
    for size in [10, 12, 14, 20] {
        let root_result = measure(&root_nonzero[..size], &[], false);
        let controls = (0..16)
            .map(|control| measure(&random_base(size, 2026 + control), &[], false))
            .collect::<Vec<_>>();
        let min_rank = controls.iter().map(|result| result.rank).min().unwrap();
        let max_rank = controls.iter().map(|result| result.rank).max().unwrap();
        let mean_rows = controls
            .iter()
            .map(|result| result.distinct_rows as f64)
            .sum::<f64>()
            / controls.len() as f64;
        println!("B={size} root triples={} quartets={} raw_matches={} unique_rows={} rank={} additions={} control_16_rank_range={min_rank}..={max_rank} control_16_row_mean={mean_rows:.2}", root_result.triples, root_result.quartets, root_result.raw_matches, root_result.distinct_rows, root_result.rank, root_result.additions);
        if size <= 14 {
            let nonzero_root = root
                .iter()
                .copied()
                .filter(|&point| multiply(H, point, &mut 0) != Point::Zero)
                .collect::<Vec<_>>();
            let windows = (0..nonzero_root.len())
                .map(|start| {
                    let points = (0..size)
                        .map(|offset| nonzero_root[(start + offset) % nonzero_root.len()])
                        .collect::<Vec<_>>();
                    measure(&points, &[], false)
                })
                .collect::<Vec<_>>();
            let min_rank = windows.iter().map(|result| result.rank).min().unwrap();
            let max_rank = windows.iter().map(|result| result.rank).max().unwrap();
            let mean_rows = windows
                .iter()
                .map(|result| result.distinct_rows as f64)
                .sum::<f64>()
                / windows.len() as f64;
            let full_rank = windows
                .iter()
                .filter(|result| result.rank == size - 1)
                .count();
            println!("B={size} nonzero_root_windows={} window_rank_range={min_rank}..={max_rank} window_row_mean={mean_rows:.2} full_rank_windows={full_rank}", windows.len());
        }
    }
    for (order, generator) in [
        (51, mul_field(ROOT_GENERATOR, ROOT_GENERATOR)),
        (102, ROOT_GENERATOR),
    ] {
        let (base, free_rows) = trace_roots(order, generator);
        let result = measure(&base, &free_rows, false);
        let union = measure(&base, &free_rows, true);
        let controls = (0..16)
            .map(|control| measure(&random_base(base.len(), 10_000 + control), &[], false))
            .collect::<Vec<_>>();
        let control_min = controls.iter().map(|row| row.rank).min().unwrap();
        let control_max = controls.iter().map(|row| row.rank).max().unwrap();
        let control_mean = controls
            .iter()
            .map(|row| row.distinct_rows as f64)
            .sum::<f64>()
            / controls.len() as f64;
        println!("trace_order={order} B={} free_rank={} seven_unique_rows={} seven_rank={} union_rank={} orbit_distinct_rows={} orbit_distinct_rank={} control_16_rank_range={control_min}..={control_max} control_16_row_mean={control_mean:.2}", base.len(), free_rows.len(), result.distinct_rows, result.rank, union.rank, result.orbit_distinct_rows, result.orbit_distinct_rank);
        if order == 102 {
            assert_eq!(result.orbit_distinct_rank, base.len() - 1);
            let projected = base
                .iter()
                .map(|&point| multiply(H, point, &mut 0))
                .collect::<Vec<_>>();
            let mut orbit_ids = vec![usize::MAX; base.len()];
            for (id, row) in free_rows.iter().enumerate() {
                for &index in row {
                    orbit_ids[index] = id;
                }
            }
            let vector = null_vector(&result.orbit_basis);
            let generator = multiply(123, projected[0], &mut 0);
            let target = multiply(1_777, generator, &mut 0);
            assert!(!projected.contains(&generator) && !projected.contains(&neg(generator)));
            assert!(!projected.contains(&target) && !projected.contains(&neg(target)));
            let generator_terms = six_descent(generator, &projected, &orbit_ids);
            let target_terms = six_descent(target, &projected, &orbit_ids);
            let coverage = six_orbit_coverage(&projected, &orbit_ids, generator);
            let denominator = dot_terms(&generator_terms, &vector);
            assert_ne!(denominator, 0);
            let recovered = dot_terms(&target_terms, &vector) * power(denominator, R - 2, R) % R;
            assert_eq!(recovered, 1_777);
            assert_eq!(multiply(recovered, generator, &mut 0), target);
            println!("trace_toy_log_recovered={recovered} generator_six_terms={} target_six_terms={} orbit_distinct_rank={}", generator_terms.len(), target_terms.len(), result.orbit_distinct_rank);
            println!(
                "trace_toy_six_orbit_coverage={} min_decompositions={} max_decompositions={}",
                coverage.0, coverage.1, coverage.2
            );
        }
    }
}
