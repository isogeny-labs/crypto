use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs;

const P: u64 = 2_147_483_647;

fn add(a: u64, b: u64) -> u64 {
    let value = a + b;
    if value >= P {
        value - P
    } else {
        value
    }
}

fn neg(value: u64) -> u64 {
    if value == 0 {
        0
    } else {
        P - value
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
    power(value, P - 2)
}

#[derive(Clone)]
struct Candidate {
    target: String,
    target_x: u64,
    recorded_summary: String,
    terms: BTreeMap<[i16; 4], u64>,
    orbits: BTreeMap<u32, String>,
}

#[derive(Clone, Debug)]
struct Metrics {
    terms: usize,
    minimum: [i16; 4],
    maximum: [i16; 4],
    projection_terms: usize,
    affine_rank: usize,
    lattice_index: i64,
    hull_area2: i64,
    matrix_rows: usize,
    matrix_columns: usize,
    matrix_rank: usize,
    stabilizer_count: usize,
    permutation_count: usize,
    digest: u128,
}

fn field<'a>(line: &'a str, key: &str) -> Result<&'a str, String> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&format!("{key}=")))
        .ok_or_else(|| format!("missing {key} in {line}"))
}

fn parse_candidate(path: &str) -> Result<Candidate, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut target = None;
    let mut target_x = None;
    let mut summary = None;
    let mut terms = BTreeMap::new();
    let mut orbits = BTreeMap::new();
    for line in text.lines() {
        if line.starts_with("META ") {
            if field(line, "schema")? != "pplus-torus-support-v1"
                || field(line, "role")? != "candidate"
                || field(line, "seed_manifest")? != "null"
                || field(line, "cross_target_cache")? != "0"
                || field(line, "root_grid_materialized")? != "0"
                || field(line, "pair_list_materialized")? != "0"
                || field(line, "fft_used")? != "0"
            {
                return Err(format!("protocol metadata violation in {path}"));
            }
            target = Some(field(line, "target")?.to_string());
            target_x = Some(
                field(line, "target_x")?
                    .parse()
                    .map_err(|_| "bad target_x")?,
            );
        } else if line.starts_with("SUMMARY ") {
            summary = Some(line.to_string());
        } else if line.starts_with("ORBIT ") {
            let t = field(line, "t")?.parse().map_err(|_| "bad t")?;
            orbits.insert(t, line.to_string());
        } else if let Some(rest) = line.strip_prefix("TERM ") {
            let items: Vec<_> = rest.split_whitespace().collect();
            if items.len() != 5 {
                return Err("malformed term".to_string());
            }
            let exponent = [
                items[0].parse().map_err(|_| "bad exponent")?,
                items[1].parse().map_err(|_| "bad exponent")?,
                items[2].parse().map_err(|_| "bad exponent")?,
                items[3].parse().map_err(|_| "bad exponent")?,
            ];
            let coefficient = items[4].parse().map_err(|_| "bad coefficient")?;
            if coefficient == 0 || terms.insert(exponent, coefficient).is_some() {
                return Err("zero or duplicate term".to_string());
            }
        }
    }
    Ok(Candidate {
        target: target.ok_or_else(|| "missing target".to_string())?,
        target_x: target_x.ok_or_else(|| "missing target_x".to_string())?,
        recorded_summary: summary.ok_or_else(|| "missing summary".to_string())?,
        terms,
        orbits,
    })
}

fn digest(terms: &BTreeMap<[i16; 4], u64>) -> u128 {
    let mut state = 0x6a09e667f3bcc908bb67ae8584caa73bu128;
    for (exponent, coefficient) in terms {
        state ^= *coefficient as u128;
        state = state.rotate_left(17).wrapping_mul(0x100000001b3);
        for value in exponent {
            state ^= (*value as i32 as u32) as u128;
            state = state.rotate_left(11).wrapping_mul(0x9e3779b185ebca87);
        }
    }
    state
}

fn cross(o: (i16, i16), a: (i16, i16), b: (i16, i16)) -> i64 {
    (a.0 as i64 - o.0 as i64) * (b.1 as i64 - o.1 as i64)
        - (a.1 as i64 - o.1 as i64) * (b.0 as i64 - o.0 as i64)
}

fn convex_hull(mut points: Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    points.sort_unstable();
    points.dedup();
    if points.len() <= 1 {
        return points;
    }
    let mut low = Vec::new();
    for &point in &points {
        while low.len() > 1 && cross(low[low.len() - 2], low[low.len() - 1], point) <= 0 {
            low.pop();
        }
        low.push(point);
    }
    let mut high = Vec::new();
    for &point in points.iter().rev() {
        while high.len() > 1 && cross(high[high.len() - 2], high[high.len() - 1], point) <= 0 {
            high.pop();
        }
        high.push(point);
    }
    low.pop();
    high.pop();
    low.extend(high);
    low
}

fn area2(vertices: &[(i16, i16)]) -> i64 {
    if vertices.len() < 3 {
        return 0;
    }
    let mut result = 0;
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % vertices.len()];
        result += x1 as i64 * y2 as i64 - y1 as i64 * x2 as i64;
    }
    result.abs()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lattice(points: &[(i16, i16)]) -> (usize, i64) {
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
            index = gcd(index, u.0 * v.1 - u.1 * v.0);
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

fn coefficient_rank(terms: &BTreeMap<[i16; 4], u64>) -> (usize, usize, usize) {
    let rows: Vec<_> = terms
        .keys()
        .map(|e| [e[0], e[1]])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let columns: Vec<_> = terms
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
    for (&e, &coefficient) in terms {
        matrix[row_index[&[e[0], e[1]]]][column_index[&[e[2], e[3]]]] = coefficient;
    }
    let mut rank = 0;
    for column in 0..columns.len() {
        let Some(pivot) = (rank..rows.len()).find(|&row| matrix[row][column] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let scale = inverse(matrix[rank][column]);
        for value in &mut matrix[rank][column..] {
            *value = mul(*value, scale);
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

fn gl_count(terms: &BTreeMap<[i16; 4], u64>) -> usize {
    let mut count = 0;
    for a in -2i16..=2 {
        for b in -2i16..=2 {
            for c in -2i16..=2 {
                for d in -2i16..=2 {
                    if (a as i32 * d as i32 - b as i32 * c as i32).abs() != 1 {
                        continue;
                    }
                    count += usize::from(terms.iter().all(|(e, coefficient)| {
                        let transformed = [e[0], e[1], a * e[2] + b * e[3], c * e[2] + d * e[3]];
                        terms.get(&transformed) == Some(coefficient)
                    }));
                }
            }
        }
    }
    count
}

fn permutations<F: FnMut(&[usize])>(values: &mut [usize], offset: usize, callback: &mut F) {
    if offset == values.len() {
        callback(values);
        return;
    }
    for i in offset..values.len() {
        values.swap(offset, i);
        permutations(values, offset + 1, callback);
        values.swap(offset, i);
    }
}

fn permutation_count(terms: &BTreeMap<[i16; 4], u64>) -> usize {
    let mut values = [0usize, 1, 2, 3];
    let mut count = 0;
    permutations(&mut values, 0, &mut |order| {
        count += usize::from(terms.iter().all(|(e, coefficient)| {
            let transformed = [e[order[0]], e[order[1]], e[order[2]], e[order[3]]];
            terms.get(&transformed) == Some(coefficient)
        }));
    });
    count
}

fn metrics(candidate: &Candidate) -> Metrics {
    let mut minimum = [i16::MAX; 4];
    let mut maximum = [i16::MIN; 4];
    for e in candidate.terms.keys() {
        for i in 0..4 {
            minimum[i] = minimum[i].min(e[i]);
            maximum[i] = maximum[i].max(e[i]);
        }
    }
    let projection: Vec<_> = candidate
        .terms
        .keys()
        .map(|e| (e[2], e[3]))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let (affine_rank, lattice_index) = lattice(&projection);
    let vertices = convex_hull(projection.clone());
    let (matrix_rows, matrix_columns, matrix_rank) = coefficient_rank(&candidate.terms);
    Metrics {
        terms: candidate.terms.len(),
        minimum,
        maximum,
        projection_terms: projection.len(),
        affine_rank,
        lattice_index,
        hull_area2: area2(&vertices),
        matrix_rows,
        matrix_columns,
        matrix_rank,
        stabilizer_count: gl_count(&candidate.terms),
        permutation_count: permutation_count(&candidate.terms),
        digest: digest(&candidate.terms),
    }
}

fn array(values: &[i16; 4]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn summary(metric: &Metrics) -> String {
    let width = std::array::from_fn(|i| metric.maximum[i] - metric.minimum[i]);
    let clear = std::array::from_fn(|i| -metric.minimum[i]);
    format!(
        "SUMMARY terms={} min={} max={} width={} clear={} projection_terms={} affine_rank={} lattice_index={} hull_area2={} matrix_rows={} matrix_columns={} matrix_rank={} stabilizer_count={} permutation_count={} rank_one={} digest={:032x}",
        metric.terms,
        array(&metric.minimum),
        array(&metric.maximum),
        array(&width),
        array(&clear),
        metric.projection_terms,
        metric.affine_rank,
        metric.lattice_index,
        metric.hull_area2,
        metric.matrix_rows,
        metric.matrix_columns,
        metric.matrix_rank,
        metric.stabilizer_count,
        metric.permutation_count,
        usize::from(metric.matrix_rank == 1),
        metric.digest,
    )
}

fn expected_orbit(t: u32) -> String {
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

fn check_orbits(candidate: &Candidate) -> Result<(), String> {
    if candidate.orbits.len() != 29 {
        return Err(format!(
            "{} has {} orbit rows",
            candidate.target,
            candidate.orbits.len()
        ));
    }
    for t in 1..=29 {
        if candidate.orbits.get(&t) != Some(&expected_orbit(t)) {
            return Err(format!("{} orbit mismatch at {t}", candidate.target));
        }
    }
    Ok(())
}

fn check_verifier(path: &str, candidate: &Candidate, metric: &Metrics) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let line = text
        .lines()
        .find(|line| line.starts_with("VERIFY "))
        .ok_or_else(|| format!("missing VERIFY line in {path}"))?;
    if field(line, "schema")? != "pplus-torus-support-v1"
        || field(line, "target")? != candidate.target
        || field(line, "target_x")? != candidate.target_x.to_string()
        || field(line, "map_terms")? != metric.terms.to_string()
        || field(line, "map_match")? != "1"
        || field(line, "summary_match")? != "1"
        || field(line, "eval_match")? != "1"
        || field(line, "orbit_match")? != "1"
        || field(line, "independent_permutation_determinant")? != "1"
        || field(line, "direct_x_substitution")? != "1"
        || field(line, "errors")? != "0"
        || field(line, "exactness_ok")? != "1"
        || field(line, "digest")? != format!("{:032x}", metric.digest)
    {
        return Err(format!("verifier record mismatch for {}", candidate.target));
    }
    Ok(())
}

fn argument(arguments: &[String], name: &str) -> Result<String, String> {
    arguments
        .windows(2)
        .find(|window| window[0] == name)
        .map(|window| window[1].clone())
        .ok_or_else(|| format!("missing {name}"))
}

fn main() -> Result<(), String> {
    let arguments: Vec<_> = env::args().collect();
    let candidate_paths = [
        argument(&arguments, "--candidate-g")?,
        argument(&arguments, "--candidate-g17")?,
    ];
    let verifier_paths = [
        argument(&arguments, "--verify-g")?,
        argument(&arguments, "--verify-g17")?,
    ];
    let mut records = Vec::new();
    for i in 0..2 {
        let candidate = parse_candidate(&candidate_paths[i])?;
        let metric = metrics(&candidate);
        if candidate.recorded_summary != summary(&metric) {
            return Err(format!(
                "analyzer summary mismatch for {}",
                candidate.target
            ));
        }
        check_orbits(&candidate)?;
        check_verifier(&verifier_paths[i], &candidate, &metric)?;
        println!(
            "ANALYZE target={} target_x={} terms={} projection_terms={} affine_rank={} lattice_index={} hull_area2={} matrix_rank={} stabilizer_count={} permutation_count={} digest={:032x} exactness_ok=1",
            candidate.target,
            candidate.target_x,
            metric.terms,
            metric.projection_terms,
            metric.affine_rank,
            metric.lattice_index,
            metric.hull_area2,
            metric.matrix_rank,
            metric.stabilizer_count,
            metric.permutation_count,
            metric.digest,
        );
        records.push((candidate, metric));
    }
    if records[0].0.target != "g" || records[1].0.target != "g17" {
        return Err("targets were not supplied as independent g and g17 records".to_string());
    }
    let structural_stop = records.iter().all(|(_, metric)| {
        metric.affine_rank == 2
            && metric.lattice_index == 1
            && metric.matrix_rank > 1
            && metric.stabilizer_count == 8
            && metric.permutation_count == 24
    });
    let verdict = if structural_stop {
        "TORUS_QUADRATIC_STOP"
    } else {
        "SPECIALIZATION_LEAD"
    };
    println!("RECURRENCE torus_ordered_t=4^(t+1) inversion_ordered_t=4^t unordered_t=(4^t+2^t)/2 unordered_next=4*unordered_t-2^t asymptotic_base=4 explicit_eliminator=0");
    println!(
        "RESULT exactness_ok=1 targets_independent=1 seed_manifest=null cross_target_amortization=0 root_grid_materialized=0 pair_list_materialized=0 fft_used=0 mixed_volume_degree=4*n^2 inversion_quotient_degree=n^2 symmetric_quotient_dimension=n*(n+1)/2 recurrence_rho=4 verdict={verdict}"
    );
    Ok(())
}
