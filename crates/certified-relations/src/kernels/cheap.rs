use super::*;

#[derive(Clone, Debug)]
pub struct PairCandidate {
    pub inputs: [usize; 2],
    pub variables: [usize; 2],
    pub cubic_difference: Row,
    pub cubic_syzygy: Row,
}
/// Exact f3 recognition without constructing unused cubic consequences.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PairMetadata {
    pub inputs: [usize; 2],
    pub variables: [usize; 2],
}
pub(crate) fn pair_metadata(inputs: &[Poly], n: usize, p: u64) -> Vec<PairMetadata> {
    if p == 2 {
        return Vec::new();
    }
    let recognized: Vec<_> = inputs
        .iter()
        .enumerate()
        .filter_map(|(i, f)| recognize(f, i, n, p))
        .collect();
    let mut used = BTreeSet::new();
    let mut pairs = Vec::new();
    for (i, left) in recognized.iter().enumerate() {
        for right in &recognized[i + 1..] {
            if (left.indices, left.a, left.b) == (right.indices, right.a, right.b)
                && left.z != right.z
                && used.insert((left.indices, left.a, left.b))
            {
                pairs.push(PairMetadata {
                    inputs: [left.input, right.input],
                    variables: left.indices,
                });
            }
        }
    }
    pairs
}
fn plus_row(a: &mut Row, b: &Row, factor: u64, p: u64) {
    axpy(&mut a.poly, &b.poly, factor, p);
    axpy(&mut a.proof, &b.proof, factor, p);
}
fn exponent(n: usize, indices: [usize; 2], x: usize, y: usize) -> Mono {
    let mut result = Mono::zero(n);
    result.set_exp(indices[0], x);
    result.set_exp(indices[1], y);
    result
}
pub fn f3(n: usize, indices: [usize; 2], a: u64, b: u64, z: u64, p: u64) -> Poly {
    let z2 = mul(z, z, p);
    let add = |x: u64, y: u64| ((x as u128 + y as u128) % p as u128) as u64;
    [
        (2, 2, 1),
        (2, 1, minus(mul(2, z, p), p)),
        (1, 2, minus(mul(2, z, p), p)),
        (2, 0, z2),
        (0, 2, z2),
        (1, 1, minus(add(mul(2, z2, p), mul(2, a, p)), p)),
        (1, 0, minus(add(mul(2, mul(a, z, p), p), mul(4, b, p)), p)),
        (0, 1, minus(add(mul(2, mul(a, z, p), p), mul(4, b, p)), p)),
        (0, 0, add(mul(a, a, p), minus(mul(4, mul(b, z, p), p), p))),
    ]
    .into_iter()
    .filter(|(_, _, c)| *c != 0)
    .map(|(x, y, c)| (exponent(n, indices, x, y), c))
    .collect()
}
#[derive(Clone)]
struct Recognized {
    input: usize,
    indices: [usize; 2],
    a: u64,
    b: u64,
    z: u64,
    row: Row,
}
fn recognize(f: &Poly, input: usize, n: usize, p: u64) -> Option<Recognized> {
    let indices: Vec<usize> = (0..n).filter(|i| f.keys().any(|m| m[*i] != 0)).collect();
    if indices.len() != 2 || degree(f) != 4 {
        return None;
    }
    let indices = [indices[0], indices[1]];
    let leading = exponent(n, indices, 2, 2);
    let normalization = inv(*f.get(&leading)?, p);
    let mut row = Row {
        poly: f.clone(),
        proof: BTreeMap::from([((input, Mono::zero(n)), 1)]),
    };
    scale(&mut row, normalization, p);
    let coefficient = |x, y| {
        row.poly
            .get(&exponent(n, indices, x, y))
            .copied()
            .unwrap_or(0)
    };
    let i2 = p.div_ceil(2);
    let i4 = mul(i2, i2, p);
    let z = mul(minus(coefficient(2, 1), p), i2, p);
    let a = mul(
        minus((coefficient(1, 1) + mul(2, mul(z, z, p), p)) % p, p),
        i2,
        p,
    );
    let b = mul(
        minus((coefficient(1, 0) + mul(2, mul(a, z, p), p)) % p, p),
        i4,
        p,
    );
    if row.poly != f3(n, indices, a, b, z, p) {
        return None;
    }
    Some(Recognized {
        input,
        indices,
        a,
        b,
        z,
        row,
    })
}
/// Only structurally recognized f3 pairs are used. Every returned consequence
/// has a checked certificate against the supplied (possibly scaled) equations.
pub fn consequences(inputs: &[Poly], n: usize, p: u64) -> Vec<PairCandidate> {
    if p == 2 {
        return Vec::new();
    }
    let recognized: Vec<_> = inputs
        .iter()
        .enumerate()
        .filter_map(|(i, f)| recognize(f, i, n, p))
        .collect();
    let mut results = Vec::new();
    let mut used = BTreeSet::new();
    for (i, left) in recognized.iter().enumerate() {
        for right in &recognized[i + 1..] {
            if (left.indices, left.a, left.b) != (right.indices, right.a, right.b)
                || left.z == right.z
            {
                continue;
            }
            if !used.insert((left.indices, left.a, left.b)) {
                continue;
            }
            let delta = (left.z + p - right.z) % p;
            let reciprocal = inv(delta, p);
            let k = (left.z + right.z) % p;
            let mut g = left.row.clone();
            subtract_row(&mut g, &right.row, 1, p);
            scale(&mut g, reciprocal, p);
            let mut h = right.row.clone();
            scale(&mut h, left.z, p);
            subtract_row(&mut h, &left.row, right.z, p);
            scale(&mut h, reciprocal, p);
            let xy = exponent(n, left.indices, 1, 1);
            let x = exponent(n, left.indices, 1, 0);
            let y = exponent(n, left.indices, 0, 1);
            let mut cubic = row_shift(&g, &xy);
            plus_row(&mut cubic, &row_shift(&h, &x), 2, p);
            plus_row(&mut cubic, &row_shift(&h, &y), 2, p);
            plus_row(&mut cubic, &h, mul(4, k, p), p);
            let i2 = p.div_ceil(2);
            plus_row(&mut cubic, &row_shift(&g, &x), mul(k, i2, p), p);
            plus_row(&mut cubic, &row_shift(&g, &y), mul(k, i2, p), p);
            scale(&mut g, minus(i2, p), p);
            scale(&mut cubic, mul(2, mul(reciprocal, reciprocal, p), p), p);
            assert_eq!(degree(&cubic.poly), 3);
            assert!(verify(&g, inputs, p) && verify(&cubic, inputs, p));
            results.push(PairCandidate {
                inputs: [left.input, right.input],
                variables: left.indices,
                cubic_difference: g,
                cubic_syzygy: cubic,
            });
        }
    }
    results
}
/// Construction and its exact identity checks are included by the caller's
/// preprocessing timer. Neither reference(), trace() nor analyze() is called.
pub fn augment(inputs: &[Poly], n: usize, p: u64) -> (Vec<Poly>, Vec<PairCandidate>) {
    let candidates = consequences(inputs, n, p);
    let mut augmented = inputs.to_vec();
    for pair in &candidates {
        augmented.push(pair.cubic_difference.poly.clone());
        augmented.push(pair.cubic_syzygy.poly.clone());
    }
    (augmented, candidates)
}
