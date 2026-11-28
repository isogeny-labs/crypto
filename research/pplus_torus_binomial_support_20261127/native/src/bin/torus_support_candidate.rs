use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;

const P: u64 = 2_147_483_647;
const A: u64 = 2_147_483_644;
const B: u64 = 21;
const G_X: u64 = 1_391_443_431;
const G17_X: u64 = 1_960_828_035;
const TERM_CAP: usize = 262_144;

fn add(a: u64, b: u64) -> u64 {
    let value = a + b;
    if value >= P {
        value - P
    } else {
        value
    }
}

fn neg(a: u64) -> u64 {
    if a == 0 {
        0
    } else {
        P - a
    }
}

fn sub(a: u64, b: u64) -> u64 {
    add(a, neg(b))
}

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn power(mut base: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul(result, base);
        }
        base = mul(base, base);
        exponent >>= 1;
    }
    result
}

fn inverse(value: u64) -> u64 {
    assert_ne!(value, 0);
    power(value, P - 2)
}

#[derive(Clone, Copy, Debug, Default)]
struct Work {
    additions: u128,
    multiplications: u128,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Poly<const N: usize> {
    terms: BTreeMap<[i16; N], u64>,
}

impl<const N: usize> Poly<N> {
    fn zero() -> Self {
        Self::default()
    }

    fn constant(value: u64) -> Self {
        let mut result = Self::zero();
        if value != 0 {
            result.terms.insert([0; N], value);
        }
        result
    }

    fn variable(index: usize) -> Self {
        let mut exponent = [0; N];
        exponent[index] = 1;
        let mut result = Self::zero();
        result.terms.insert(exponent, 1);
        result
    }

    fn plus(&self, other: &Self, work: &mut Work) -> Result<Self, String> {
        let mut result = self.clone();
        for (&exponent, &coefficient) in &other.terms {
            work.additions += 1;
            let value = add(
                result.terms.get(&exponent).copied().unwrap_or(0),
                coefficient,
            );
            if value == 0 {
                result.terms.remove(&exponent);
            } else {
                result.terms.insert(exponent, value);
                if result.terms.len() > TERM_CAP {
                    return Err("term cap exceeded".to_string());
                }
            }
        }
        Ok(result)
    }

    fn negative(&self) -> Self {
        Self {
            terms: self.terms.iter().map(|(&e, &c)| (e, neg(c))).collect(),
        }
    }

    fn minus(&self, other: &Self, work: &mut Work) -> Result<Self, String> {
        self.plus(&other.negative(), work)
    }

    fn scaled(&self, scalar: u64, work: &mut Work) -> Self {
        if scalar == 0 {
            return Self::zero();
        }
        if scalar == 1 {
            return self.clone();
        }
        Self {
            terms: self
                .terms
                .iter()
                .filter_map(|(&e, &c)| {
                    work.multiplications += 1;
                    let value = mul(c, scalar);
                    (value != 0).then_some((e, value))
                })
                .collect(),
        }
    }

    fn times(&self, other: &Self, work: &mut Work) -> Result<Self, String> {
        if self.terms.is_empty() || other.terms.is_empty() {
            return Ok(Self::zero());
        }
        let mut result = BTreeMap::new();
        for (&left_e, &left_c) in &self.terms {
            for (&right_e, &right_c) in &other.terms {
                let mut exponent = [0; N];
                for i in 0..N {
                    exponent[i] = left_e[i]
                        .checked_add(right_e[i])
                        .ok_or_else(|| "exponent overflow".to_string())?;
                }
                work.multiplications += 1;
                let product = mul(left_c, right_c);
                let old = result.get(&exponent).copied().unwrap_or(0);
                if old != 0 {
                    work.additions += 1;
                }
                let value = add(old, product);
                if value == 0 {
                    result.remove(&exponent);
                } else {
                    result.insert(exponent, value);
                    if result.len() > TERM_CAP {
                        return Err("term cap exceeded".to_string());
                    }
                }
            }
        }
        Ok(Self { terms: result })
    }

    fn pow(&self, exponent: usize, work: &mut Work) -> Result<Self, String> {
        let mut result = Self::constant(1);
        for _ in 0..exponent {
            result = result.times(self, work)?;
        }
        Ok(result)
    }
}

type P5 = Poly<5>;
type P4 = Poly<4>;
type P2 = Poly<2>;

fn pair_coefficients(s: &P5, q: &P5, work: &mut Work) -> Result<[P5; 3], String> {
    let c2 = s.times(s, work)?.minus(&q.scaled(4, work), work)?;
    let q_plus_a = q.plus(&P5::constant(A), work)?;
    let inner = s
        .times(&q_plus_a, work)?
        .plus(&P5::constant(mul(2, B)), work)?;
    let c1 = inner.scaled(neg(2), work);
    let q_minus_a = q.minus(&P5::constant(A), work)?;
    let c0 = q_minus_a
        .times(&q_minus_a, work)?
        .minus(&s.scaled(mul(4, B), work), work)?;
    Ok([c0, c1, c2])
}

fn vector_sub(left: &[P5], right: &[P5], work: &mut Work) -> Result<Vec<P5>, String> {
    let n = left.len().max(right.len());
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        result.push(
            left.get(i)
                .cloned()
                .unwrap_or_else(P5::zero)
                .minus(right.get(i).unwrap_or(&P5::zero()), work)?,
        );
    }
    Ok(result)
}

fn vector_scale(values: &[P5], scalar: &P5, work: &mut Work) -> Result<Vec<P5>, String> {
    values
        .iter()
        .map(|value| value.times(scalar, work))
        .collect()
}

fn convolution(left: &[P5], right: &[P5], work: &mut Work) -> Result<Vec<P5>, String> {
    let mut result = vec![P5::zero(); left.len() + right.len() - 1];
    for (i, x) in left.iter().enumerate() {
        for (j, y) in right.iter().enumerate() {
            result[i + j] = result[i + j].plus(&x.times(y, work)?, work)?;
        }
    }
    Ok(result)
}

fn s4_coefficients(pair: &[P5; 3], r: &P5, work: &mut Work) -> Result<Vec<P5>, String> {
    let r2 = r.times(r, work)?;
    let b2 = vec![r2.clone(), r.scaled(neg(2), work), P5::constant(1)];
    let b1 = vec![
        r.scaled(A, work)
            .plus(&P5::constant(mul(2, B)), work)?
            .scaled(neg(2), work),
        r2.plus(&P5::constant(A), work)?.scaled(neg(2), work),
        r.scaled(neg(2), work),
    ];
    let b0 = vec![
        P5::constant(mul(A, A)).minus(&r.scaled(mul(4, B), work), work)?,
        r.scaled(mul(2, A), work)
            .plus(&P5::constant(mul(4, B)), work)?
            .negative(),
        r2,
    ];
    let at = vector_sub(
        &vector_scale(&b0, &pair[2], work)?,
        &vector_scale(&b2, &pair[0], work)?,
        work,
    )?;
    let bt = vector_sub(
        &vector_scale(&b1, &pair[2], work)?,
        &vector_scale(&b2, &pair[1], work)?,
        work,
    )?;
    let ct = vector_sub(
        &vector_scale(&b0, &pair[1], work)?,
        &vector_scale(&b1, &pair[0], work)?,
        work,
    )?;
    vector_sub(
        &convolution(&at, &at, work)?,
        &convolution(&bt, &ct, work)?,
        work,
    )
}

fn sylvester(quartic: &[P5], quadratic: &[P5]) -> Vec<Vec<P5>> {
    let mut matrix = vec![vec![P5::zero(); 6]; 6];
    for shift in 0..2 {
        for i in 0..5 {
            matrix[shift][shift + i] = quartic[4 - i].clone();
        }
    }
    for shift in 0..4 {
        for i in 0..3 {
            matrix[2 + shift][shift + i] = quadratic[2 - i].clone();
        }
    }
    matrix
}

fn determinant_subset(matrix: &[Vec<P5>], work: &mut Work) -> Result<P5, String> {
    let n = matrix.len();
    let mut states = vec![P5::zero(); 1 << n];
    states[0] = P5::constant(1);
    for mask in 0usize..(1usize << n) {
        let row = mask.count_ones() as usize;
        if row == n || states[mask].terms.is_empty() {
            continue;
        }
        for (column, entry) in matrix[row].iter().enumerate().take(n) {
            if mask & (1 << column) != 0 || entry.terms.is_empty() {
                continue;
            }
            let mut term = states[mask].times(entry, work)?;
            if (mask >> (column + 1)).count_ones() & 1 == 1 {
                term = term.negative();
            }
            let next = mask | (1 << column);
            states[next] = states[next].plus(&term, work)?;
        }
    }
    Ok(states[(1 << n) - 1].clone())
}

fn build_pair_polar(work: &mut Work) -> Result<P5, String> {
    let left = pair_coefficients(&P5::variable(0), &P5::variable(1), work)?;
    let right = pair_coefficients(&P5::variable(2), &P5::variable(3), work)?;
    let quartic = s4_coefficients(&right, &P5::variable(4), work)?;
    determinant_subset(&sylvester(&quartic, &left), work)
}

fn specialize_target(source: &P5, target: u64, work: &mut Work) -> Result<P4, String> {
    let mut result = P4::zero();
    for (&exponent, &coefficient) in &source.terms {
        if exponent.iter().any(|&e| e < 0) {
            return Err("negative exponent in ordinary polar".to_string());
        }
        let scalar = power(target, exponent[4] as u64);
        work.multiplications += 1;
        let value = mul(coefficient, scalar);
        let key = [exponent[0], exponent[1], exponent[2], exponent[3]];
        let old = result.terms.get(&key).copied().unwrap_or(0);
        if old != 0 {
            work.additions += 1;
        }
        let sum = add(old, value);
        if sum == 0 {
            result.terms.remove(&key);
        } else {
            result.terms.insert(key, sum);
        }
    }
    Ok(result)
}

fn pair_generators() -> (P2, P2) {
    let s = P2 {
        terms: [([-1, 0], 1), ([0, -1], 1), ([0, 1], 1), ([1, 0], 1)]
            .into_iter()
            .collect(),
    };
    let q = P2 {
        terms: [([-1, -1], 1), ([-1, 1], 1), ([1, -1], 1), ([1, 1], 1)]
            .into_iter()
            .collect(),
    };
    (s, q)
}

fn pair_basis(keys: &[(i16, i16)], work: &mut Work) -> Result<Vec<P2>, String> {
    let (s, q) = pair_generators();
    keys.iter()
        .map(|&(a, b)| {
            let left = s.pow(a as usize, work)?;
            left.times(&q.pow(b as usize, work)?, work)
        })
        .collect()
}

fn accumulate<const N: usize>(
    map: &mut BTreeMap<[i16; N], u64>,
    exponent: [i16; N],
    coefficient: u64,
    work: &mut Work,
) -> Result<(), String> {
    let old = map.get(&exponent).copied().unwrap_or(0);
    if old != 0 {
        work.additions += 1;
    }
    let value = add(old, coefficient);
    if value == 0 {
        map.remove(&exponent);
    } else {
        map.insert(exponent, value);
        if map.len() > TERM_CAP {
            return Err("term cap exceeded".to_string());
        }
    }
    Ok(())
}

fn substitute_pairs(source: &P4, work: &mut Work) -> Result<P4, String> {
    let left_keys: Vec<_> = source
        .terms
        .keys()
        .map(|e| (e[0], e[1]))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let right_keys: Vec<_> = source
        .terms
        .keys()
        .map(|e| (e[2], e[3]))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let left_index: HashMap<_, _> = left_keys
        .iter()
        .copied()
        .enumerate()
        .map(|(i, k)| (k, i))
        .collect();
    let right_index: HashMap<_, _> = right_keys
        .iter()
        .copied()
        .enumerate()
        .map(|(i, k)| (k, i))
        .collect();
    let left_basis = pair_basis(&left_keys, work)?;
    let right_basis = pair_basis(&right_keys, work)?;

    let mut middle = BTreeMap::<([i16; 2], usize), u64>::new();
    for (&e, &coefficient) in &source.terms {
        let row = left_index[&(e[0], e[1])];
        let column = right_index[&(e[2], e[3])];
        for (&left_e, &left_c) in &left_basis[row].terms {
            work.multiplications += 1;
            let value = mul(coefficient, left_c);
            let key = (left_e, column);
            let old = middle.get(&key).copied().unwrap_or(0);
            if old != 0 {
                work.additions += 1;
            }
            let sum = add(old, value);
            if sum == 0 {
                middle.remove(&key);
            } else {
                middle.insert(key, sum);
            }
        }
    }

    let mut output = BTreeMap::new();
    for ((left_e, column), coefficient) in middle {
        for (&right_e, &right_c) in &right_basis[column].terms {
            work.multiplications += 1;
            let value = mul(coefficient, right_c);
            accumulate(
                &mut output,
                [left_e[0], left_e[1], right_e[0], right_e[1]],
                value,
                work,
            )?;
        }
    }
    Ok(P4 { terms: output })
}

fn digest(poly: &P4) -> u128 {
    let mut state = 0x6a09e667f3bcc908bb67ae8584caa73bu128;
    for (exponent, coefficient) in &poly.terms {
        state ^= *coefficient as u128;
        state = state.rotate_left(17).wrapping_mul(0x100000001b3);
        for value in exponent {
            state ^= (*value as i32 as u32) as u128;
            state = state.rotate_left(11).wrapping_mul(0x9e3779b185ebca87);
        }
    }
    state
}

fn bounds(poly: &P4) -> ([i16; 4], [i16; 4]) {
    let mut minimum = [i16::MAX; 4];
    let mut maximum = [i16::MIN; 4];
    for exponent in poly.terms.keys() {
        for i in 0..4 {
            minimum[i] = minimum[i].min(exponent[i]);
            maximum[i] = maximum[i].max(exponent[i]);
        }
    }
    (minimum, maximum)
}

fn cross(origin: (i16, i16), a: (i16, i16), b: (i16, i16)) -> i64 {
    (a.0 as i64 - origin.0 as i64) * (b.1 as i64 - origin.1 as i64)
        - (a.1 as i64 - origin.1 as i64) * (b.0 as i64 - origin.0 as i64)
}

fn convex_hull(mut points: Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    points.sort_unstable();
    points.dedup();
    if points.len() <= 1 {
        return points;
    }
    let mut lower = Vec::new();
    for &point in &points {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], point) <= 0
        {
            lower.pop();
        }
        lower.push(point);
    }
    let mut upper = Vec::new();
    for &point in points.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], point) <= 0
        {
            upper.pop();
        }
        upper.push(point);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn hull_area2(hull: &[(i16, i16)]) -> i64 {
    if hull.len() < 3 {
        return 0;
    }
    let mut area = 0i64;
    for i in 0..hull.len() {
        let (x1, y1) = hull[i];
        let (x2, y2) = hull[(i + 1) % hull.len()];
        area += x1 as i64 * y2 as i64 - y1 as i64 * x2 as i64;
    }
    area.abs()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn affine_rank_and_index(points: &[(i16, i16)]) -> (usize, i64) {
    if points.len() <= 1 {
        return (0, 0);
    }
    let base = points[0];
    let differences: Vec<_> = points
        .iter()
        .map(|&(x, y)| (x as i64 - base.0 as i64, y as i64 - base.1 as i64))
        .collect();
    if differences.iter().all(|&(x, y)| x == 0 && y == 0) {
        return (0, 0);
    }
    let mut index = 0;
    for &left in &differences {
        for &right in &differences {
            index = gcd(index, left.0 * right.1 - left.1 * right.0);
        }
    }
    if index == 0 {
        (1, 0)
    } else {
        (2, index)
    }
}

fn matrix_rank(poly: &P4) -> (usize, usize, usize) {
    let rows: Vec<_> = poly
        .terms
        .keys()
        .map(|e| [e[0], e[1]])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let columns: Vec<_> = poly
        .terms
        .keys()
        .map(|e| [e[2], e[3]])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let row_index: HashMap<_, _> = rows
        .iter()
        .copied()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let column_index: HashMap<_, _> = columns
        .iter()
        .copied()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let mut matrix = vec![vec![0u64; columns.len()]; rows.len()];
    for (&e, &coefficient) in &poly.terms {
        matrix[row_index[&[e[0], e[1]]]][column_index[&[e[2], e[3]]]] = coefficient;
    }
    let mut rank = 0;
    for column in 0..columns.len() {
        let Some(pivot) = (rank..rows.len()).find(|&row| matrix[row][column] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let scale = inverse(matrix[rank][column]);
        for entry in &mut matrix[rank][column..] {
            *entry = mul(*entry, scale);
        }
        for row in 0..rows.len() {
            if row == rank || matrix[row][column] == 0 {
                continue;
            }
            let factor = matrix[row][column];
            for col in column..columns.len() {
                matrix[row][col] = sub(matrix[row][col], mul(factor, matrix[rank][col]));
            }
        }
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    (rows.len(), columns.len(), rank)
}

fn gl_stabilizers(poly: &P4) -> Vec<(i16, i16, i16, i16)> {
    let mut result = Vec::new();
    for a in -2i16..=2 {
        for b in -2i16..=2 {
            for c in -2i16..=2 {
                for d in -2i16..=2 {
                    let determinant = a as i32 * d as i32 - b as i32 * c as i32;
                    if determinant.abs() != 1 {
                        continue;
                    }
                    let stable = poly.terms.iter().all(|(e, coefficient)| {
                        let mut transformed = *e;
                        transformed[2] = a * e[2] + b * e[3];
                        transformed[3] = c * e[2] + d * e[3];
                        poly.terms.get(&transformed) == Some(coefficient)
                    });
                    if stable {
                        result.push((a, b, c, d));
                    }
                }
            }
        }
    }
    result
}

fn enumerate_permutations<F: FnMut(&[usize])>(values: &mut [usize], at: usize, callback: &mut F) {
    if at == values.len() {
        callback(values);
        return;
    }
    for i in at..values.len() {
        values.swap(at, i);
        enumerate_permutations(values, at + 1, callback);
        values.swap(at, i);
    }
}

fn permutation_stabilizer_count(poly: &P4) -> usize {
    let mut permutation = [0usize, 1, 2, 3];
    let mut count = 0;
    enumerate_permutations(&mut permutation, 0, &mut |order| {
        let stable = poly.terms.iter().all(|(e, coefficient)| {
            let transformed = [e[order[0]], e[order[1]], e[order[2]], e[order[3]]];
            poly.terms.get(&transformed) == Some(coefficient)
        });
        count += usize::from(stable);
    });
    count
}

fn evaluate_laurent(poly: &P4, inputs: [u64; 4]) -> u64 {
    let mut total = 0;
    for (exponent, coefficient) in &poly.terms {
        let mut term = *coefficient;
        for i in 0..4 {
            let value = if exponent[i] >= 0 {
                power(inputs[i], exponent[i] as u64)
            } else {
                power(inverse(inputs[i]), (-exponent[i]) as u64)
            };
            term = mul(term, value);
        }
        total = add(total, term);
    }
    total
}

fn evaluate_ordinary(poly: &P4, inputs: [u64; 4]) -> u64 {
    let mut total = 0;
    for (exponent, coefficient) in &poly.terms {
        let mut term = *coefficient;
        for i in 0..4 {
            assert!(exponent[i] >= 0);
            term = mul(term, power(inputs[i], exponent[i] as u64));
        }
        total = add(total, term);
    }
    total
}

fn fixed_tuple(index: usize) -> [u64; 4] {
    let i = index as u64;
    [2 + i, 101 + 3 * i, 1_009 + 5 * i, 10_007 + 7 * i]
}

fn join_i16(values: &[i16; 4]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn main() -> Result<(), String> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 3 || arguments[1] != "--target" {
        return Err("usage: torus-support-candidate --target g|g17".to_string());
    }
    let (target_name, target_x) = match arguments[2].as_str() {
        "g" => ("g", G_X),
        "g17" => ("g17", G17_X),
        _ => return Err("target must be g or g17".to_string()),
    };

    let mut work = Work::default();
    let formal = build_pair_polar(&mut work)?;
    let specialized = specialize_target(&formal, target_x, &mut work)?;
    let laurent = substitute_pairs(&specialized, &mut work)?;
    let (minimum, maximum) = bounds(&laurent);
    let width = std::array::from_fn(|i| maximum[i] - minimum[i]);
    let clear = std::array::from_fn(|i| -minimum[i]);
    let projection: Vec<_> = laurent
        .terms
        .keys()
        .map(|e| (e[2], e[3]))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let hull = convex_hull(projection.clone());
    let (affine_rank, lattice_index) = affine_rank_and_index(&projection);
    let (matrix_rows, matrix_columns, rank) = matrix_rank(&laurent);
    let stabilizers = gl_stabilizers(&laurent);
    let permutation_count = permutation_stabilizer_count(&laurent);
    let poly_digest = digest(&laurent);

    println!(
        "META schema=pplus-torus-support-v1 role=candidate target={target_name} target_x={target_x} seed_manifest=null cross_target_cache=0 root_grid_materialized=0 pair_list_materialized=0 fft_used=0"
    );
    println!(
        "BUILD formal_terms={} specialized_terms={} additions={} multiplications={}",
        formal.terms.len(),
        specialized.terms.len(),
        work.additions,
        work.multiplications
    );
    println!(
        "SUMMARY terms={} min={} max={} width={} clear={} projection_terms={} affine_rank={} lattice_index={} hull_area2={} matrix_rows={} matrix_columns={} matrix_rank={} stabilizer_count={} permutation_count={} rank_one={} digest={:032x}",
        laurent.terms.len(),
        join_i16(&minimum),
        join_i16(&maximum),
        join_i16(&width),
        join_i16(&clear),
        projection.len(),
        affine_rank,
        lattice_index,
        hull_area2(&hull),
        matrix_rows,
        matrix_columns,
        rank,
        stabilizers.len(),
        permutation_count,
        usize::from(rank == 1),
        poly_digest,
    );
    println!(
        "HULL points={}",
        hull.iter()
            .map(|(x, y)| format!("{x}:{y}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    println!(
        "STABILIZERS matrices={}",
        stabilizers
            .iter()
            .map(|(a, b, c, d)| format!("{a},{b},{c},{d}"))
            .collect::<Vec<_>>()
            .join(";")
    );

    for index in 0..32 {
        let z = fixed_tuple(index);
        let x = z.map(|value| add(value, inverse(value)));
        let pair_values = [
            add(x[0], x[1]),
            mul(x[0], x[1]),
            add(x[2], x[3]),
            mul(x[2], x[3]),
        ];
        let laurent_value = evaluate_laurent(&laurent, z);
        let ordinary_value = evaluate_ordinary(&specialized, pair_values);
        if laurent_value != ordinary_value {
            return Err(format!("numeric substitution mismatch at tuple {index}"));
        }
        println!(
            "EVAL index={index} z={},{},{},{} value={laurent_value}",
            z[0], z[1], z[2], z[3]
        );
    }

    for t in 1u32..=29 {
        let n = 1u128 << t;
        let m = 2 * n;
        let torus = m * m;
        let inversion = n * n;
        let unordered = n * (n + 1) / 2;
        let off_diagonal = n * (n - 1) / 2;
        println!(
            "ORBIT t={t} n={n} m={m} resultant_degree={torus} torus_ordered={torus} inversion_ordered={inversion} unordered_with_diagonal={unordered} off_diagonal={off_diagonal}"
        );
    }

    for (exponent, coefficient) in &laurent.terms {
        println!(
            "TERM {} {} {} {} {}",
            exponent[0], exponent[1], exponent[2], exponent[3], coefficient
        );
    }
    println!("END target={target_name} exact_internal=1");
    Ok(())
}
