use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs;

const MODULUS: u64 = 2_147_483_647;
const CURVE_A: u64 = 2_147_483_644;
const CURVE_B: u64 = 21;
const GENERATOR_X: u64 = 1_391_443_431;
const GENERATOR_17_X: u64 = 1_960_828_035;
const LIMIT: usize = 262_144;

fn fp_add(left: u64, right: u64) -> u64 {
    let result = left + right;
    if result >= MODULUS {
        result - MODULUS
    } else {
        result
    }
}

fn fp_neg(value: u64) -> u64 {
    if value == 0 {
        0
    } else {
        MODULUS - value
    }
}

fn fp_sub(left: u64, right: u64) -> u64 {
    fp_add(left, fp_neg(right))
}

fn fp_mul(left: u64, right: u64) -> u64 {
    ((left as u128 * right as u128) % MODULUS as u128) as u64
}

fn fp_pow(mut base: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = fp_mul(result, base);
        }
        base = fp_mul(base, base);
        exponent /= 2;
    }
    result
}

fn fp_inv(value: u64) -> u64 {
    assert_ne!(value, 0);
    fp_pow(value, MODULUS - 2)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Polynomial<const N: usize>(BTreeMap<[i16; N], u64>);

impl<const N: usize> Polynomial<N> {
    fn zero() -> Self {
        Self::default()
    }

    fn one() -> Self {
        Self::constant(1)
    }

    fn constant(value: u64) -> Self {
        let mut result = BTreeMap::new();
        if value != 0 {
            result.insert([0; N], value);
        }
        Self(result)
    }

    fn indeterminate(index: usize) -> Self {
        let mut exponent = [0; N];
        exponent[index] = 1;
        let mut result = BTreeMap::new();
        result.insert(exponent, 1);
        Self(result)
    }

    fn sum(&self, other: &Self) -> Result<Self, String> {
        let mut result = self.0.clone();
        for (&exponent, &coefficient) in &other.0 {
            let value = fp_add(result.get(&exponent).copied().unwrap_or(0), coefficient);
            if value == 0 {
                result.remove(&exponent);
            } else {
                result.insert(exponent, value);
                if result.len() > LIMIT {
                    return Err("verifier term cap exceeded".to_string());
                }
            }
        }
        Ok(Self(result))
    }

    fn opposite(&self) -> Self {
        Self(self.0.iter().map(|(&e, &c)| (e, fp_neg(c))).collect())
    }

    fn difference(&self, other: &Self) -> Result<Self, String> {
        self.sum(&other.opposite())
    }

    fn scalar_product(&self, scalar: u64) -> Self {
        if scalar == 0 {
            return Self::zero();
        }
        Self(
            self.0
                .iter()
                .filter_map(|(&e, &c)| {
                    let value = fp_mul(c, scalar);
                    (value != 0).then_some((e, value))
                })
                .collect(),
        )
    }

    fn product(&self, other: &Self) -> Result<Self, String> {
        if self.0.is_empty() || other.0.is_empty() {
            return Ok(Self::zero());
        }
        let mut result = BTreeMap::new();
        for (&left_e, &left_c) in &self.0 {
            for (&right_e, &right_c) in &other.0 {
                let mut exponent = [0; N];
                for i in 0..N {
                    exponent[i] = left_e[i]
                        .checked_add(right_e[i])
                        .ok_or_else(|| "verifier exponent overflow".to_string())?;
                }
                let value = fp_add(
                    result.get(&exponent).copied().unwrap_or(0),
                    fp_mul(left_c, right_c),
                );
                if value == 0 {
                    result.remove(&exponent);
                } else {
                    result.insert(exponent, value);
                    if result.len() > LIMIT {
                        return Err("verifier term cap exceeded".to_string());
                    }
                }
            }
        }
        Ok(Self(result))
    }
}

type Ordinary5 = Polynomial<5>;
type Ordinary4 = Polynomial<4>;

fn s3_coefficients(x: &Ordinary5, y: &Ordinary5) -> Result<[Ordinary5; 3], String> {
    let s = x.sum(y)?;
    let q = x.product(y)?;
    let c2 = s.product(&s)?.difference(&q.scalar_product(4))?;
    let c1 = s
        .product(&q.sum(&Ordinary5::constant(CURVE_A))?)?
        .sum(&Ordinary5::constant(fp_mul(2, CURVE_B)))?
        .scalar_product(fp_neg(2));
    let shifted_q = q.difference(&Ordinary5::constant(CURVE_A))?;
    let c0 = shifted_q
        .product(&shifted_q)?
        .difference(&s.scalar_product(fp_mul(4, CURVE_B)))?;
    Ok([c0, c1, c2])
}

fn scale_vector(values: &[Ordinary5], scalar: &Ordinary5) -> Result<Vec<Ordinary5>, String> {
    values.iter().map(|value| value.product(scalar)).collect()
}

fn subtract_vectors(left: &[Ordinary5], right: &[Ordinary5]) -> Result<Vec<Ordinary5>, String> {
    let size = left.len().max(right.len());
    (0..size)
        .map(|i| {
            left.get(i)
                .cloned()
                .unwrap_or_else(Ordinary5::zero)
                .difference(right.get(i).unwrap_or(&Ordinary5::zero()))
        })
        .collect()
}

fn multiply_vectors(left: &[Ordinary5], right: &[Ordinary5]) -> Result<Vec<Ordinary5>, String> {
    let mut result = vec![Ordinary5::zero(); left.len() + right.len() - 1];
    for (i, x) in left.iter().enumerate() {
        for (j, y) in right.iter().enumerate() {
            result[i + j] = result[i + j].sum(&x.product(y)?)?;
        }
    }
    Ok(result)
}

fn direct_s4(pair: &[Ordinary5; 3], r: &Ordinary5) -> Result<Vec<Ordinary5>, String> {
    let r_squared = r.product(r)?;
    let b2 = vec![
        r_squared.clone(),
        r.scalar_product(fp_neg(2)),
        Ordinary5::one(),
    ];
    let b1 = vec![
        r.scalar_product(CURVE_A)
            .sum(&Ordinary5::constant(fp_mul(2, CURVE_B)))?
            .scalar_product(fp_neg(2)),
        r_squared
            .sum(&Ordinary5::constant(CURVE_A))?
            .scalar_product(fp_neg(2)),
        r.scalar_product(fp_neg(2)),
    ];
    let b0 = vec![
        Ordinary5::constant(fp_mul(CURVE_A, CURVE_A))
            .difference(&r.scalar_product(fp_mul(4, CURVE_B)))?,
        r.scalar_product(fp_mul(2, CURVE_A))
            .sum(&Ordinary5::constant(fp_mul(4, CURVE_B)))?
            .opposite(),
        r_squared,
    ];
    let at = subtract_vectors(&scale_vector(&b0, &pair[2])?, &scale_vector(&b2, &pair[0])?)?;
    let bt = subtract_vectors(&scale_vector(&b1, &pair[2])?, &scale_vector(&b2, &pair[1])?)?;
    let ct = subtract_vectors(&scale_vector(&b0, &pair[1])?, &scale_vector(&b1, &pair[0])?)?;
    subtract_vectors(&multiply_vectors(&at, &at)?, &multiply_vectors(&bt, &ct)?)
}

fn permutation_is_odd(permutation: &[usize]) -> bool {
    let mut odd = false;
    for i in 0..permutation.len() {
        for j in i + 1..permutation.len() {
            if permutation[i] > permutation[j] {
                odd = !odd;
            }
        }
    }
    odd
}

fn generate_permutations(values: &mut [usize], offset: usize, output: &mut Vec<Vec<usize>>) {
    if offset == values.len() {
        output.push(values.to_vec());
        return;
    }
    for i in offset..values.len() {
        values.swap(offset, i);
        generate_permutations(values, offset + 1, output);
        values.swap(offset, i);
    }
}

fn determinant_by_permutations(matrix: &[Vec<Ordinary5>]) -> Result<Ordinary5, String> {
    let mut base: Vec<_> = (0..matrix.len()).collect();
    let mut permutations = Vec::with_capacity(720);
    generate_permutations(&mut base, 0, &mut permutations);
    let mut determinant = Ordinary5::zero();
    for columns in permutations {
        if columns
            .iter()
            .enumerate()
            .any(|(row, &column)| matrix[row][column].0.is_empty())
        {
            continue;
        }
        let mut term = Ordinary5::one();
        for (row, &column) in columns.iter().enumerate() {
            term = term.product(&matrix[row][column])?;
        }
        if permutation_is_odd(&columns) {
            term = term.opposite();
        }
        determinant = determinant.sum(&term)?;
    }
    Ok(determinant)
}

fn build_s5_in_x() -> Result<Ordinary5, String> {
    let x1 = Ordinary5::indeterminate(0);
    let x2 = Ordinary5::indeterminate(1);
    let x3 = Ordinary5::indeterminate(2);
    let x4 = Ordinary5::indeterminate(3);
    let r = Ordinary5::indeterminate(4);
    let left = s3_coefficients(&x1, &x2)?;
    let right = s3_coefficients(&x3, &x4)?;
    let quartic = direct_s4(&right, &r)?;
    let mut matrix = vec![vec![Ordinary5::zero(); 6]; 6];
    for shift in 0..2 {
        for i in 0..5 {
            matrix[shift][shift + i] = quartic[4 - i].clone();
        }
    }
    for shift in 0..4 {
        for i in 0..3 {
            matrix[2 + shift][shift + i] = left[2 - i].clone();
        }
    }
    determinant_by_permutations(&matrix)
}

fn fix_target(source: &Ordinary5, target: u64) -> Result<Ordinary4, String> {
    let mut result = BTreeMap::new();
    for (&e, &coefficient) in &source.0 {
        if e.iter().any(|&value| value < 0) {
            return Err("ordinary verifier polynomial had a negative exponent".to_string());
        }
        let key = [e[0], e[1], e[2], e[3]];
        let value = fp_mul(coefficient, fp_pow(target, e[4] as u64));
        let sum = fp_add(result.get(&key).copied().unwrap_or(0), value);
        if sum == 0 {
            result.remove(&key);
        } else {
            result.insert(key, sum);
        }
    }
    Ok(Polynomial::<4>(result))
}

fn binomial_expansion(degree: usize) -> Vec<(i16, u64)> {
    let mut result = Vec::with_capacity(degree + 1);
    let mut coefficient = 1u64;
    for k in 0..=degree {
        result.push((degree as i16 - 2 * k as i16, coefficient));
        if k < degree {
            coefficient = fp_mul(coefficient, (degree - k) as u64);
            coefficient = fp_mul(coefficient, fp_inv((k + 1) as u64));
        }
    }
    result
}

fn laurent_from_x(source: &Ordinary4) -> Result<Ordinary4, String> {
    let maximum = source.0.keys().fold([0usize; 4], |mut current, e| {
        for i in 0..4 {
            current[i] = current[i].max(e[i] as usize);
        }
        current
    });
    let expansions: Vec<Vec<Vec<(i16, u64)>>> = maximum
        .iter()
        .map(|&bound| (0..=bound).map(binomial_expansion).collect())
        .collect();
    let mut result = BTreeMap::new();
    for (&ordinary_e, &ordinary_c) in &source.0 {
        for &(e0, c0) in &expansions[0][ordinary_e[0] as usize] {
            for &(e1, c1) in &expansions[1][ordinary_e[1] as usize] {
                for &(e2, c2) in &expansions[2][ordinary_e[2] as usize] {
                    for &(e3, c3) in &expansions[3][ordinary_e[3] as usize] {
                        let exponent = [e0, e1, e2, e3];
                        let contribution =
                            fp_mul(ordinary_c, fp_mul(fp_mul(c0, c1), fp_mul(c2, c3)));
                        let value =
                            fp_add(result.get(&exponent).copied().unwrap_or(0), contribution);
                        if value == 0 {
                            result.remove(&exponent);
                        } else {
                            result.insert(exponent, value);
                            if result.len() > LIMIT {
                                return Err("verifier Laurent term cap exceeded".to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Polynomial::<4>(result))
}

fn map_digest(poly: &Ordinary4) -> u128 {
    let mut state = 0x6a09e667f3bcc908bb67ae8584caa73bu128;
    for (exponent, coefficient) in &poly.0 {
        state ^= *coefficient as u128;
        state = state.rotate_left(17).wrapping_mul(0x100000001b3);
        for value in exponent {
            state ^= (*value as i32 as u32) as u128;
            state = state.rotate_left(11).wrapping_mul(0x9e3779b185ebca87);
        }
    }
    state
}

fn extrema(poly: &Ordinary4) -> ([i16; 4], [i16; 4]) {
    let mut low = [i16::MAX; 4];
    let mut high = [i16::MIN; 4];
    for e in poly.0.keys() {
        for i in 0..4 {
            low[i] = low[i].min(e[i]);
            high[i] = high[i].max(e[i]);
        }
    }
    (low, high)
}

fn orientation(o: (i16, i16), a: (i16, i16), b: (i16, i16)) -> i64 {
    (a.0 as i64 - o.0 as i64) * (b.1 as i64 - o.1 as i64)
        - (a.1 as i64 - o.1 as i64) * (b.0 as i64 - o.0 as i64)
}

fn hull(mut points: Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    points.sort_unstable();
    points.dedup();
    if points.len() < 2 {
        return points;
    }
    let mut lower = Vec::new();
    for &point in &points {
        while lower.len() > 1
            && orientation(lower[lower.len() - 2], lower[lower.len() - 1], point) <= 0
        {
            lower.pop();
        }
        lower.push(point);
    }
    let mut upper = Vec::new();
    for &point in points.iter().rev() {
        while upper.len() > 1
            && orientation(upper[upper.len() - 2], upper[upper.len() - 1], point) <= 0
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

fn twice_area(vertices: &[(i16, i16)]) -> i64 {
    if vertices.len() < 3 {
        return 0;
    }
    (0..vertices.len())
        .map(|i| {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % vertices.len()];
            x1 as i64 * y2 as i64 - y1 as i64 * x2 as i64
        })
        .sum::<i64>()
        .abs()
}

fn integer_gcd(mut left: i64, mut right: i64) -> i64 {
    left = left.abs();
    right = right.abs();
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn lattice_data(points: &[(i16, i16)]) -> (usize, i64) {
    if points.len() <= 1 {
        return (0, 0);
    }
    let anchor = points[0];
    let differences: Vec<_> = points
        .iter()
        .map(|&(x, y)| (x as i64 - anchor.0 as i64, y as i64 - anchor.1 as i64))
        .collect();
    let mut index = 0;
    for &u in &differences {
        for &v in &differences {
            index = integer_gcd(index, u.0 * v.1 - u.1 * v.0);
        }
    }
    if index != 0 {
        (2, index)
    } else if differences.iter().any(|&(x, y)| x != 0 || y != 0) {
        (1, 0)
    } else {
        (0, 0)
    }
}

fn coefficient_rank(poly: &Ordinary4) -> (usize, usize, usize) {
    let rows: Vec<_> = poly
        .0
        .keys()
        .map(|e| [e[0], e[1]])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let columns: Vec<_> = poly
        .0
        .keys()
        .map(|e| [e[2], e[3]])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let row_positions: HashMap<_, _> = rows
        .iter()
        .copied()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let column_positions: HashMap<_, _> = columns
        .iter()
        .copied()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let mut values = vec![vec![0u64; columns.len()]; rows.len()];
    for (&e, &coefficient) in &poly.0 {
        values[row_positions[&[e[0], e[1]]]][column_positions[&[e[2], e[3]]]] = coefficient;
    }
    let mut rank = 0;
    for column in 0..columns.len() {
        let Some(pivot) = (rank..rows.len()).find(|&row| values[row][column] != 0) else {
            continue;
        };
        values.swap(rank, pivot);
        let factor = fp_inv(values[rank][column]);
        for value in &mut values[rank][column..] {
            *value = fp_mul(*value, factor);
        }
        for row in 0..rows.len() {
            if row == rank || values[row][column] == 0 {
                continue;
            }
            let scale = values[row][column];
            for c in column..columns.len() {
                values[row][c] = fp_sub(values[row][c], fp_mul(scale, values[rank][c]));
            }
        }
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    (rows.len(), columns.len(), rank)
}

fn exact_gl_stabilizers(poly: &Ordinary4) -> Vec<(i16, i16, i16, i16)> {
    let mut answer = Vec::new();
    for a in -2i16..=2 {
        for b in -2i16..=2 {
            for c in -2i16..=2 {
                for d in -2i16..=2 {
                    if (a as i32 * d as i32 - b as i32 * c as i32).abs() != 1 {
                        continue;
                    }
                    if poly.0.iter().all(|(e, coefficient)| {
                        let transformed = [e[0], e[1], a * e[2] + b * e[3], c * e[2] + d * e[3]];
                        poly.0.get(&transformed) == Some(coefficient)
                    }) {
                        answer.push((a, b, c, d));
                    }
                }
            }
        }
    }
    answer
}

fn visit_permutations<F: FnMut(&[usize])>(values: &mut [usize], offset: usize, callback: &mut F) {
    if offset == values.len() {
        callback(values);
        return;
    }
    for i in offset..values.len() {
        values.swap(offset, i);
        visit_permutations(values, offset + 1, callback);
        values.swap(offset, i);
    }
}

fn symmetric_permutation_count(poly: &Ordinary4) -> usize {
    let mut values = [0usize, 1, 2, 3];
    let mut count = 0;
    visit_permutations(&mut values, 0, &mut |order| {
        count += usize::from(poly.0.iter().all(|(e, coefficient)| {
            let transformed = [e[order[0]], e[order[1]], e[order[2]], e[order[3]]];
            poly.0.get(&transformed) == Some(coefficient)
        }));
    });
    count
}

fn array_text(values: &[i16; 4]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn metric_lines(poly: &Ordinary4) -> (String, String, String) {
    let (minimum, maximum) = extrema(poly);
    let width = std::array::from_fn(|i| maximum[i] - minimum[i]);
    let clear = std::array::from_fn(|i| -minimum[i]);
    let projection: Vec<_> = poly
        .0
        .keys()
        .map(|e| (e[2], e[3]))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let vertices = hull(projection.clone());
    let (affine_rank, lattice_index) = lattice_data(&projection);
    let (rows, columns, rank) = coefficient_rank(poly);
    let stabilizers = exact_gl_stabilizers(poly);
    let permutations = symmetric_permutation_count(poly);
    let summary = format!(
        "SUMMARY terms={} min={} max={} width={} clear={} projection_terms={} affine_rank={} lattice_index={} hull_area2={} matrix_rows={} matrix_columns={} matrix_rank={} stabilizer_count={} permutation_count={} rank_one={} digest={:032x}",
        poly.0.len(), array_text(&minimum), array_text(&maximum), array_text(&width), array_text(&clear),
        projection.len(), affine_rank, lattice_index, twice_area(&vertices), rows, columns, rank,
        stabilizers.len(), permutations, usize::from(rank == 1), map_digest(poly)
    );
    let hull_line = format!(
        "HULL points={}",
        vertices
            .iter()
            .map(|(x, y)| format!("{x}:{y}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let stabilizer_line = format!(
        "STABILIZERS matrices={}",
        stabilizers
            .iter()
            .map(|(a, b, c, d)| format!("{a},{b},{c},{d}"))
            .collect::<Vec<_>>()
            .join(";")
    );
    (summary, hull_line, stabilizer_line)
}

fn parse_field<'a>(line: &'a str, name: &str) -> Result<&'a str, String> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&format!("{name}=")))
        .ok_or_else(|| format!("missing {name} in {line}"))
}

struct CandidateData {
    target: String,
    target_x: u64,
    summary: String,
    hull: String,
    stabilizers: String,
    evaluations: BTreeMap<usize, (String, u64)>,
    orbits: BTreeMap<u32, String>,
    map: Ordinary4,
}

fn parse_candidate(text: &str) -> Result<CandidateData, String> {
    let mut target = None;
    let mut target_x = None;
    let mut summary = None;
    let mut hull_line = None;
    let mut stabilizers = None;
    let mut evaluations = BTreeMap::new();
    let mut orbits = BTreeMap::new();
    let mut terms = BTreeMap::new();
    for line in text.lines() {
        if line.starts_with("META ") {
            if parse_field(line, "schema")? != "pplus-torus-support-v1"
                || parse_field(line, "role")? != "candidate"
                || parse_field(line, "seed_manifest")? != "null"
                || parse_field(line, "cross_target_cache")? != "0"
                || parse_field(line, "root_grid_materialized")? != "0"
                || parse_field(line, "pair_list_materialized")? != "0"
                || parse_field(line, "fft_used")? != "0"
            {
                return Err("candidate metadata violates protocol".to_string());
            }
            target = Some(parse_field(line, "target")?.to_string());
            target_x = Some(
                parse_field(line, "target_x")?
                    .parse()
                    .map_err(|_| "bad target_x")?,
            );
        } else if line.starts_with("SUMMARY ") {
            summary = Some(line.to_string());
        } else if line.starts_with("HULL ") {
            hull_line = Some(line.to_string());
        } else if line.starts_with("STABILIZERS ") {
            stabilizers = Some(line.to_string());
        } else if line.starts_with("EVAL ") {
            let index: usize = parse_field(line, "index")?
                .parse()
                .map_err(|_| "bad eval index")?;
            let value: u64 = parse_field(line, "value")?
                .parse()
                .map_err(|_| "bad eval value")?;
            evaluations.insert(index, (parse_field(line, "z")?.to_string(), value));
        } else if line.starts_with("ORBIT ") {
            let t: u32 = parse_field(line, "t")?
                .parse()
                .map_err(|_| "bad orbit depth")?;
            orbits.insert(t, line.to_string());
        } else if let Some(rest) = line.strip_prefix("TERM ") {
            let values: Vec<_> = rest.split_whitespace().collect();
            if values.len() != 5 {
                return Err("bad TERM field count".to_string());
            }
            let exponent = [
                values[0].parse().map_err(|_| "bad term exponent")?,
                values[1].parse().map_err(|_| "bad term exponent")?,
                values[2].parse().map_err(|_| "bad term exponent")?,
                values[3].parse().map_err(|_| "bad term exponent")?,
            ];
            let coefficient: u64 = values[4].parse().map_err(|_| "bad term coefficient")?;
            if coefficient == 0 || terms.insert(exponent, coefficient).is_some() {
                return Err("zero or duplicate candidate term".to_string());
            }
        }
    }
    Ok(CandidateData {
        target: target.ok_or_else(|| "missing META".to_string())?,
        target_x: target_x.ok_or_else(|| "missing target_x".to_string())?,
        summary: summary.ok_or_else(|| "missing SUMMARY".to_string())?,
        hull: hull_line.ok_or_else(|| "missing HULL".to_string())?,
        stabilizers: stabilizers.ok_or_else(|| "missing STABILIZERS".to_string())?,
        evaluations,
        orbits,
        map: Polynomial::<4>(terms),
    })
}

fn evaluate_laurent(poly: &Ordinary4, inputs: [u64; 4]) -> u64 {
    poly.0.iter().fold(0, |total, (exponent, coefficient)| {
        let term = (0..4).fold(*coefficient, |value, i| {
            let factor = if exponent[i] >= 0 {
                fp_pow(inputs[i], exponent[i] as u64)
            } else {
                fp_pow(fp_inv(inputs[i]), (-exponent[i]) as u64)
            };
            fp_mul(value, factor)
        });
        fp_add(total, term)
    })
}

fn test_tuple(index: usize) -> [u64; 4] {
    let i = index as u64;
    [2 + i, 101 + 3 * i, 1_009 + 5 * i, 10_007 + 7 * i]
}

fn orbit_line(t: u32) -> String {
    let n = 1u128 << t;
    let m = 2 * n;
    let torus = m * m;
    let inversion = n * n;
    let unordered = n * (n + 1) / 2;
    let off_diagonal = n * (n - 1) / 2;
    format!(
        "ORBIT t={t} n={n} m={m} resultant_degree={torus} torus_ordered={torus} inversion_ordered={inversion} unordered_with_diagonal={unordered} off_diagonal={off_diagonal}"
    )
}

fn main() -> Result<(), String> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 3 || arguments[1] != "--candidate" {
        return Err("usage: torus-support-verify --candidate PATH".to_string());
    }
    let candidate_text = fs::read_to_string(&arguments[2]).map_err(|error| error.to_string())?;
    let candidate = parse_candidate(&candidate_text)?;
    let expected_x = match candidate.target.as_str() {
        "g" => GENERATOR_X,
        "g17" => GENERATOR_17_X,
        _ => return Err("unknown candidate target".to_string()),
    };
    if candidate.target_x != expected_x {
        return Err("candidate target coordinate mismatch".to_string());
    }

    let direct_s5 = build_s5_in_x()?;
    let fixed_s5 = fix_target(&direct_s5, candidate.target_x)?;
    let expected_map = laurent_from_x(&fixed_s5)?;
    if candidate.map != expected_map {
        let first = candidate
            .map
            .0
            .iter()
            .chain(expected_map.0.iter())
            .find(|(e, _)| candidate.map.0.get(*e) != expected_map.0.get(*e));
        return Err(format!("full Laurent map mismatch at {first:?}"));
    }
    let (summary, hull_line, stabilizer_line) = metric_lines(&expected_map);
    if candidate.summary != summary
        || candidate.hull != hull_line
        || candidate.stabilizers != stabilizer_line
    {
        return Err("candidate metric record mismatch".to_string());
    }
    if candidate.evaluations.len() != 32 {
        return Err("candidate did not emit 32 evaluations".to_string());
    }
    for index in 0..32 {
        let z = test_tuple(index);
        let z_text = format!("{},{},{},{}", z[0], z[1], z[2], z[3]);
        let value = evaluate_laurent(&expected_map, z);
        if candidate.evaluations.get(&index) != Some(&(z_text, value)) {
            return Err(format!("evaluation mismatch at {index}"));
        }
    }
    if candidate.orbits.len() != 29 {
        return Err("candidate did not emit 29 orbit rows".to_string());
    }
    for t in 1..=29 {
        if candidate.orbits.get(&t) != Some(&orbit_line(t)) {
            return Err(format!("orbit row mismatch at depth {t}"));
        }
    }
    println!(
        "VERIFY schema=pplus-torus-support-v1 target={} target_x={} map_terms={} map_match=1 summary_match=1 eval_match=1 orbit_match=1 independent_permutation_determinant=1 direct_x_substitution=1 errors=0 exactness_ok=1 digest={:032x}",
        candidate.target,
        candidate.target_x,
        expected_map.0.len(),
        map_digest(&expected_map),
    );
    Ok(())
}
