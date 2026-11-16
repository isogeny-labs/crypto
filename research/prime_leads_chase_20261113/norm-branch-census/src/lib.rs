use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PROTOCOL_ID: &str = "EXP-NORM-BRANCH-CENSUS-20261113-v1";
pub const PRIMES: [u64; 2] = [8_191, 131_071];
pub const SUPPORT_SEEDS: [u64; 4] = [
    0x4e42_4353_0000_0001,
    0x4e42_4353_0000_0002,
    0x4e42_4353_0000_0003,
    0x4e42_4353_0000_0004,
];
pub const K_LADDER: [usize; 5] = [6, 8, 10, 12, 14];

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ops {
    pub additions: u64,
    pub multiplications: u64,
    pub inversions: u64,
    pub comparisons: u64,
}

impl Ops {
    fn merge(&mut self, other: Ops) {
        self.additions += other.additions;
        self.multiplications += other.multiplications;
        self.inversions += other.inversions;
        self.comparisons += other.comparisons;
    }
}

#[inline]
pub fn add(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

#[inline]
pub fn sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

#[inline]
pub fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

pub fn pow_mod(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 == 1 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}

pub fn inv(a: u64, p: u64) -> u64 {
    assert_ne!(a, 0);
    pow_mod(a, p - 2, p)
}

fn neg(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}

#[derive(Clone, Copy, Debug)]
struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn nonzero(&mut self, modulus: u64) -> u64 {
        loop {
            let x = self.next() % modulus;
            if x != 0 {
                return x;
            }
        }
    }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == q {
            return true;
        }
        if n.is_multiple_of(q) {
            return false;
        }
    }
    let mut d = 41;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }
    true
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AffinePoint {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Point {
    Infinity,
    Affine(AffinePoint),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurveFixture {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64,
    pub generator: AffinePoint,
    pub search_attempt: u64,
}

impl CurveFixture {
    fn rhs(&self, x: u64) -> u64 {
        add(
            add(
                mul(mul(x, x, self.p), x, self.p),
                mul(self.a, x, self.p),
                self.p,
            ),
            self.b,
            self.p,
        )
    }

    fn on_curve(&self, q: AffinePoint) -> bool {
        mul(q.y, q.y, self.p) == self.rhs(q.x)
    }

    fn point_neg(&self, q: Point) -> Point {
        match q {
            Point::Infinity => Point::Infinity,
            Point::Affine(q) => Point::Affine(AffinePoint {
                x: q.x,
                y: neg(q.y, self.p),
            }),
        }
    }

    fn point_add(&self, p1: Point, p2: Point) -> Point {
        match (p1, p2) {
            (Point::Infinity, q) | (q, Point::Infinity) => q,
            (Point::Affine(p1), Point::Affine(p2)) => {
                if p1.x == p2.x && add(p1.y, p2.y, self.p) == 0 {
                    return Point::Infinity;
                }
                let slope = if p1 == p2 {
                    let numerator = add(mul(3, mul(p1.x, p1.x, self.p), self.p), self.a, self.p);
                    mul(numerator, inv(mul(2, p1.y, self.p), self.p), self.p)
                } else {
                    mul(
                        sub(p2.y, p1.y, self.p),
                        inv(sub(p2.x, p1.x, self.p), self.p),
                        self.p,
                    )
                };
                let x3 = sub(sub(mul(slope, slope, self.p), p1.x, self.p), p2.x, self.p);
                let y3 = sub(mul(slope, sub(p1.x, x3, self.p), self.p), p1.y, self.p);
                Point::Affine(AffinePoint { x: x3, y: y3 })
            }
        }
    }

    fn scalar_mul(&self, mut n: u64, q: AffinePoint) -> Point {
        let mut acc = Point::Infinity;
        let mut base = Point::Affine(q);
        while n != 0 {
            if n & 1 == 1 {
                acc = self.point_add(acc, base);
            }
            base = self.point_add(base, base);
            n >>= 1;
        }
        acc
    }
}

fn legendre(a: u64, p: u64) -> i64 {
    if a == 0 {
        0
    } else if pow_mod(a, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    }
}

fn curve_order(p: u64, a: u64, b: u64) -> u64 {
    let mut order = p + 1;
    for x in 0..p {
        let rhs = add(add(mul(mul(x, x, p), x, p), mul(a, x, p), p), b, p);
        let chi = legendre(rhs, p);
        if chi > 0 {
            order += 1;
        } else if chi < 0 {
            order -= 1;
        }
    }
    order
}

pub fn find_curve(p: u64) -> CurveFixture {
    assert!(is_prime(p));
    assert_eq!(p % 4, 3);
    let mut rng = SplitMix64::new(0x4e42_4331_0000_0000 ^ p);
    for attempt in 1..=10_000 {
        let a = rng.next() % p;
        let b = rng.next() % p;
        let discriminant = add(
            mul(4, mul(mul(a, a, p), a, p), p),
            mul(27, mul(b, b, p), p),
            p,
        );
        if discriminant == 0 {
            continue;
        }
        let order = curve_order(p, a, b);
        if !is_prime(order) {
            continue;
        }
        for x in 0..p {
            let rhs = add(add(mul(mul(x, x, p), x, p), mul(a, x, p), p), b, p);
            if legendre(rhs, p) == 1 {
                let root = pow_mod(rhs, (p + 1) / 4, p);
                let y = root.min(p - root);
                let curve = CurveFixture {
                    p,
                    a,
                    b,
                    order,
                    generator: AffinePoint { x, y },
                    search_attempt: attempt,
                };
                assert!(curve.on_curve(curve.generator));
                assert_eq!(curve.scalar_mul(order, curve.generator), Point::Infinity);
                return curve;
            }
        }
    }
    panic!("no prime-order curve found under frozen attempt cap for p={p}");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchInstance {
    pub curve: CurveFixture,
    pub seed: u64,
    pub target_label: u64,
    pub target: AffinePoint,
    pub support_labels: Vec<u64>,
    pub support: Vec<AffinePoint>,
}

impl BranchInstance {
    pub fn k(&self) -> usize {
        self.support.len()
    }

    fn xs(&self) -> Vec<u64> {
        self.support
            .iter()
            .map(|q| q.x)
            .chain([self.target.x])
            .collect()
    }

    fn ys(&self) -> Vec<u64> {
        self.support
            .iter()
            .map(|q| q.y)
            .chain([self.target.y])
            .collect()
    }
}

pub fn sample_max_instance(curve: &CurveFixture, seed: u64, max_k: usize) -> BranchInstance {
    let mut rng = SplitMix64::new(seed ^ curve.p.rotate_left(17));
    let target_label = rng.nonzero(curve.order);
    let Point::Affine(target) = curve.scalar_mul(target_label, curve.generator) else {
        unreachable!();
    };
    let mut labels = Vec::with_capacity(max_k);
    let mut support = Vec::with_capacity(max_k);
    let mut fibres = BTreeSet::from([target.x]);
    while support.len() < max_k {
        let label = rng.nonzero(curve.order);
        let Point::Affine(q) = curve.scalar_mul(label, curve.generator) else {
            continue;
        };
        if fibres.insert(q.x) {
            labels.push(label);
            support.push(q);
        }
    }
    BranchInstance {
        curve: curve.clone(),
        seed,
        target_label,
        target,
        support_labels: labels,
        support,
    }
}

pub fn prefix_instance(maximal: &BranchInstance, k: usize) -> BranchInstance {
    assert!(k <= maximal.k());
    BranchInstance {
        curve: maximal.curve.clone(),
        seed: maximal.seed,
        target_label: maximal.target_label,
        target: maximal.target,
        support_labels: maximal.support_labels[..k].to_vec(),
        support: maximal.support[..k].to_vec(),
    }
}

type Poly = Vec<u64>;

fn poly_trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn poly_coeff(a: &Poly, i: usize) -> u64 {
    *a.get(i).unwrap_or(&0)
}

fn poly_sub(a: &Poly, b: &Poly, p: u64) -> Poly {
    poly_trim(
        (0..a.len().max(b.len()))
            .map(|i| sub(poly_coeff(a, i), poly_coeff(b, i), p))
            .collect(),
    )
}

fn poly_mul(a: &Poly, b: &Poly, p: u64) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = add(out[i + j], mul(*x, *y, p), p);
        }
    }
    poly_trim(out)
}

fn poly_scale(a: &Poly, c: u64, p: u64) -> Poly {
    poly_trim(a.iter().map(|x| mul(*x, c, p)).collect())
}

fn poly_eval(a: &Poly, x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |v, c| add(mul(v, x, p), *c, p))
}

fn root_poly(xs: &[u64], p: u64) -> Poly {
    xs.iter().fold(vec![1], |acc, x| {
        poly_mul(&acc, &[neg(*x, p), 1].to_vec(), p)
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RankEvidence {
    pub rank: usize,
    pub pivot_rows: Vec<usize>,
    pub pivot_cols: Vec<usize>,
    pub pivot_minor_det: u64,
    pub ops: Ops,
}

fn determinant_gaussian(matrix: &[Vec<u64>], p: u64) -> (u64, Ops) {
    let n = matrix.len();
    assert!(matrix.iter().all(|row| row.len() == n));
    if n == 0 {
        return (1, Ops::default());
    }
    let mut a = matrix.to_vec();
    let mut det = 1;
    let mut ops = Ops::default();
    for col in 0..n {
        let pivot = (col..n).find(|r| {
            ops.comparisons += 1;
            a[*r][col] != 0
        });
        let Some(pivot) = pivot else {
            return (0, ops);
        };
        if pivot != col {
            a.swap(pivot, col);
            det = neg(det, p);
        }
        let pivot_value = a[col][col];
        det = mul(det, pivot_value, p);
        ops.multiplications += 1;
        let pivot_inv = inv(pivot_value, p);
        ops.inversions += 1;
        let pivot_row = a[col].clone();
        for row in a.iter_mut().skip(col + 1) {
            if row[col] == 0 {
                ops.comparisons += 1;
                continue;
            }
            let factor = mul(row[col], pivot_inv, p);
            ops.multiplications += 1;
            for (target, source) in row[col..].iter_mut().zip(&pivot_row[col..]) {
                *target = sub(*target, mul(factor, *source, p), p);
                ops.multiplications += 1;
                ops.additions += 1;
            }
        }
    }
    (det, ops)
}

pub fn rank_rref(matrix: &[Vec<u64>], p: u64) -> RankEvidence {
    if matrix.is_empty() || matrix[0].is_empty() {
        return RankEvidence {
            rank: 0,
            pivot_rows: vec![],
            pivot_cols: vec![],
            pivot_minor_det: 1,
            ops: Ops::default(),
        };
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    assert!(matrix.iter().all(|r| r.len() == cols));
    let mut a = matrix.to_vec();
    let mut row_ids: Vec<_> = (0..rows).collect();
    let mut pivot_rows = vec![];
    let mut pivot_cols = vec![];
    let mut rank = 0;
    let mut ops = Ops::default();
    for col in 0..cols {
        let pivot = (rank..rows).find(|r| {
            ops.comparisons += 1;
            a[*r][col] != 0
        });
        let Some(pivot) = pivot else {
            continue;
        };
        a.swap(rank, pivot);
        row_ids.swap(rank, pivot);
        pivot_rows.push(row_ids[rank]);
        pivot_cols.push(col);
        let scale = inv(a[rank][col], p);
        ops.inversions += 1;
        for x in &mut a[rank][col..] {
            *x = mul(*x, scale, p);
            ops.multiplications += 1;
        }
        let pivot_row = a[rank].clone();
        for (r, row) in a.iter_mut().enumerate() {
            if r == rank || row[col] == 0 {
                ops.comparisons += 1;
                continue;
            }
            let factor = row[col];
            for j in col..cols {
                row[j] = sub(row[j], mul(factor, pivot_row[j], p), p);
                ops.multiplications += 1;
                ops.additions += 1;
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    let minor: Vec<Vec<u64>> = pivot_rows
        .iter()
        .map(|r| pivot_cols.iter().map(|c| matrix[*r][*c]).collect())
        .collect();
    let (pivot_minor_det, det_ops) = determinant_gaussian(&minor, p);
    ops.merge(det_ops);
    assert_ne!(pivot_minor_det, 0);
    RankEvidence {
        rank,
        pivot_rows,
        pivot_cols,
        pivot_minor_det,
        ops,
    }
}

pub fn rank_column_basis(matrix: &[Vec<u64>], p: u64) -> usize {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut basis: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    for mut v in (0..cols).map(|col| (0..rows).map(|r| matrix[r][col]).collect::<Vec<_>>()) {
        for (pivot, b) in &basis {
            let factor = v[*pivot];
            if factor != 0 {
                for i in *pivot..rows {
                    v[i] = sub(v[i], mul(factor, b[i], p), p);
                }
            }
        }
        let Some(pivot) = (0..rows).find(|r| v[*r] != 0) else {
            continue;
        };
        let scale = inv(v[pivot], p);
        for x in &mut v[pivot..] {
            *x = mul(*x, scale, p);
        }
        basis.insert(pivot, v);
    }
    basis.len()
}

fn null_vector(matrix: &[Vec<u64>], p: u64) -> Option<Vec<u64>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return None;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut a = matrix.to_vec();
    let mut pivots = Vec::new();
    for col in 0..cols {
        let row = pivots.len();
        if row == rows {
            break;
        }
        let Some(pivot) = (row..rows).find(|r| a[*r][col] != 0) else {
            continue;
        };
        a.swap(row, pivot);
        let scale = inv(a[row][col], p);
        for x in &mut a[row][col..] {
            *x = mul(*x, scale, p);
        }
        let pivot_row = a[row].clone();
        for (r, target) in a.iter_mut().enumerate() {
            if r == row {
                continue;
            }
            let factor = target[col];
            if factor != 0 {
                for j in col..cols {
                    target[j] = sub(target[j], mul(factor, pivot_row[j], p), p);
                }
            }
        }
        pivots.push(col);
    }
    let free = (0..cols).find(|c| !pivots.contains(c))?;
    let mut out = vec![0; cols];
    out[free] = 1;
    for (row, pivot) in pivots.iter().enumerate() {
        out[*pivot] = neg(a[row][free], p);
    }
    Some(out)
}

fn vandermonde_product(indices: &[usize], xs: &[u64], p: u64, ops: &mut Ops) -> u64 {
    let mut out = 1;
    for i in 0..indices.len() {
        for j in i + 1..indices.len() {
            out = mul(out, sub(xs[indices[j]], xs[indices[i]], p), p);
            ops.additions += 1;
            ops.multiplications += 1;
        }
    }
    out
}

fn laplace_sign(selected_u_rows: &[usize], u: usize) -> bool {
    let one_based_row_sum: usize = selected_u_rows.iter().map(|i| i + 1).sum();
    let column_sum = u * (u + 1) / 2;
    (one_based_row_sum + column_sum) % 2 == 1
}

pub fn coefficients_from_nodes(xs: &[u64], ys: &[u64], p: u64) -> (Vec<u64>, Ops) {
    assert_eq!(xs.len(), ys.len());
    let d = xs.len();
    let k = d - 1;
    let u = d / 2 + 1;
    let v = (d - 1) / 2;
    assert_eq!(u + v, d);
    let mut coeffs = vec![0; 1usize << k];
    let mut ops = Ops::default();
    for (mask, slot) in coeffs.iter_mut().enumerate() {
        let weight = mask.count_ones() as usize;
        if weight > v || v - weight > 1 {
            continue;
        }
        let target_in = v - weight == 1;
        let mut w_rows: Vec<usize> = (0..k).filter(|i| mask >> i & 1 == 1).collect();
        if target_in {
            w_rows.push(k);
        }
        let w_set: BTreeSet<_> = w_rows.iter().copied().collect();
        let u_rows: Vec<_> = (0..d).filter(|i| !w_set.contains(i)).collect();
        assert_eq!(u_rows.len(), u);
        assert_eq!(w_rows.len(), v);
        let mut value = mul(
            vandermonde_product(&u_rows, xs, p, &mut ops),
            vandermonde_product(&w_rows, xs, p, &mut ops),
            p,
        );
        ops.multiplications += 1;
        for i in &w_rows {
            value = mul(value, ys[*i], p);
            ops.multiplications += 1;
        }
        if laplace_sign(&u_rows, u) ^ target_in {
            value = neg(value, p);
        }
        assert_ne!(
            value, 0,
            "distinct nonzero-y guarded nodes have nonzero coefficients"
        );
        *slot = value;
    }
    (coeffs, ops)
}

fn vandermonde_matrix(indices: &[usize], xs: &[u64], p: u64) -> Vec<Vec<u64>> {
    indices
        .iter()
        .map(|i| {
            let mut row = Vec::with_capacity(indices.len());
            let mut power = 1;
            for _ in 0..indices.len() {
                row.push(power);
                power = mul(power, xs[*i], p);
            }
            row
        })
        .collect()
}

pub fn coefficients_direct_minors(xs: &[u64], ys: &[u64], p: u64) -> Vec<u64> {
    assert_eq!(xs.len(), ys.len());
    let d = xs.len();
    let k = d - 1;
    let u = d / 2 + 1;
    let v = (d - 1) / 2;
    let mut coeffs = vec![0; 1usize << k];
    for (mask, slot) in coeffs.iter_mut().enumerate() {
        let weight = mask.count_ones() as usize;
        if weight > v || v - weight > 1 {
            continue;
        }
        let target_in = v - weight == 1;
        let mut w_rows: Vec<usize> = (0..k).filter(|i| mask >> i & 1 == 1).collect();
        if target_in {
            w_rows.push(k);
        }
        let w_set: BTreeSet<_> = w_rows.iter().copied().collect();
        let u_rows: Vec<_> = (0..d).filter(|i| !w_set.contains(i)).collect();
        let (du, _) = determinant_gaussian(&vandermonde_matrix(&u_rows, xs, p), p);
        let mut dw_matrix = vandermonde_matrix(&w_rows, xs, p);
        for (row, index) in dw_matrix.iter_mut().zip(&w_rows) {
            for x in row {
                *x = mul(*x, ys[*index], p);
            }
        }
        let (dw, _) = determinant_gaussian(&dw_matrix, p);
        let mut value = mul(du, dw, p);
        if laplace_sign(&u_rows, u) ^ target_in {
            value = neg(value, p);
        }
        *slot = value;
    }
    coeffs
}

fn matrix_from_coefficients(coeffs: &[u64], left_mask: u64, k: usize) -> Vec<Vec<u64>> {
    let left: Vec<_> = (0..k).filter(|i| left_mask >> i & 1 == 1).collect();
    let right: Vec<_> = (0..k).filter(|i| left_mask >> i & 1 == 0).collect();
    let mut matrix = vec![vec![0; 1usize << right.len()]; 1usize << left.len()];
    for (global, value) in coeffs.iter().enumerate() {
        if *value == 0 {
            continue;
        }
        let mut row = 0;
        for (j, i) in left.iter().enumerate() {
            row |= ((global >> i) & 1) << j;
        }
        let mut col = 0;
        for (j, i) in right.iter().enumerate() {
            col |= ((global >> i) & 1) << j;
        }
        matrix[row][col] = *value;
    }
    matrix
}

fn random_coefficient_null(coeffs: &[u64], p: u64, seed: u64) -> Vec<u64> {
    let mut rng = SplitMix64::new(seed);
    coeffs
        .iter()
        .map(|x| if *x == 0 { 0 } else { rng.nonzero(p) })
        .collect()
}

fn random_ordinate_coefficients(instance: &BranchInstance, seed: u64) -> Vec<u64> {
    let p = instance.curve.p;
    let mut rng = SplitMix64::new(seed);
    let xs = instance.xs();
    let ys: Vec<_> = instance
        .ys()
        .into_iter()
        .map(|y| mul(y, rng.nonzero(p), p))
        .collect();
    coefficients_from_nodes(&xs, &ys, p).0
}

fn second_vandermonde_coefficients(instance: &BranchInstance) -> Vec<u64> {
    let p = instance.curve.p;
    let d = instance.k() + 1;
    let offset = 97 + instance.k() as u64 * 19;
    let xs: Vec<_> = (0..d).map(|i| (offset + 3 * i as u64) % p).collect();
    assert_eq!(xs.iter().copied().collect::<BTreeSet<_>>().len(), d);
    let ys = vec![1; d];
    coefficients_from_nodes(&xs, &ys, p).0
}

fn canonical_split(mask: u64, k: usize) -> u64 {
    let all = (1u64 << k) - 1;
    mask.min(all ^ mask)
}

fn combination_masks(k: usize, weight: usize) -> Vec<u64> {
    (0..1u64 << k)
        .filter(|m| m.count_ones() as usize == weight)
        .collect()
}

pub fn balanced_splits(k: usize) -> Vec<u64> {
    assert_eq!(k % 2, 0);
    let half = k / 2;
    if k <= 10 {
        return combination_masks(k, half)
            .into_iter()
            .map(|m| canonical_split(m, k))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
    }
    let mut splits = BTreeSet::new();
    splits.insert(canonical_split((1u64 << half) - 1, k));
    let alternating = (0..k)
        .filter(|i| i % 2 == 0)
        .fold(0u64, |m, i| m | (1u64 << i));
    splits.insert(canonical_split(alternating, k));
    let mut rng = SplitMix64::new(0x4e42_4350_0000_0000 ^ k as u64);
    while splits.len() < 8 {
        let mut positions: Vec<_> = (0..k).collect();
        for i in (1..k).rev() {
            let j = (rng.next() as usize) % (i + 1);
            positions.swap(i, j);
        }
        let mask = positions[..half].iter().fold(0u64, |m, i| m | (1u64 << i));
        splits.insert(canonical_split(mask, k));
    }
    splits.into_iter().collect()
}

fn fwt_values(coeffs: &[u64], p: u64) -> Vec<u64> {
    assert!(coeffs.len().is_power_of_two());
    let mut values = coeffs.to_vec();
    let mut step = 1;
    while step < values.len() {
        for base in (0..values.len()).step_by(step * 2) {
            for j in 0..step {
                let a = values[base + j];
                let b = values[base + j + step];
                values[base + j] = add(a, b, p);
                values[base + j + step] = sub(a, b, p);
            }
        }
        step *= 2;
    }
    values
}

fn coefficient_models(coeffs: &[u64], p: u64) -> Vec<u64> {
    fwt_values(coeffs, p)
        .into_iter()
        .enumerate()
        .filter_map(|(mask, value)| (value == 0).then_some(mask as u64))
        .collect()
}

fn group_models(instance: &BranchInstance) -> Vec<u64> {
    let curve = &instance.curve;
    let mut models = Vec::new();
    for mask in 0..1u64 << instance.k() {
        let mut sum = Point::Infinity;
        for (i, q) in instance.support.iter().enumerate() {
            let term = if mask >> i & 1 == 1 {
                curve.point_neg(Point::Affine(*q))
            } else {
                Point::Affine(*q)
            };
            sum = curve.point_add(sum, term);
        }
        if sum == Point::Affine(instance.target) {
            models.push(mask);
        }
    }
    models
}

fn evaluation_matrix(
    instance: &BranchInstance,
    assignment_mask: u64,
    target_sign: i8,
) -> Vec<Vec<u64>> {
    let p = instance.curve.p;
    let d = instance.k() + 1;
    let u = d / 2 + 1;
    let v = (d - 1) / 2;
    assert_eq!(u + v, d);
    let xs = instance.xs();
    let ys = instance.ys();
    (0..d)
        .map(|i| {
            let sign_negative = if i < instance.k() {
                assignment_mask >> i & 1 == 1
            } else {
                target_sign < 0
            };
            let signed_y = if sign_negative { neg(ys[i], p) } else { ys[i] };
            let mut row = Vec::with_capacity(d);
            let mut power = 1;
            for _ in 0..u {
                row.push(power);
                power = mul(power, xs[i], p);
            }
            power = 1;
            for _ in 0..v {
                row.push(mul(signed_y, power, p));
                power = mul(power, xs[i], p);
            }
            row
        })
        .collect()
}

fn direct_matrix_models(instance: &BranchInstance, target_sign: i8) -> Vec<u64> {
    (0..1u64 << instance.k())
        .filter(|mask| {
            rank_column_basis(
                &evaluation_matrix(instance, *mask, target_sign),
                instance.curve.p,
            ) < instance.k() + 1
        })
        .collect()
}

fn norm_replay(instance: &BranchInstance, assignment_mask: u64, target_sign: i8) -> bool {
    let p = instance.curve.p;
    let d = instance.k() + 1;
    let u = d / 2 + 1;
    let v = (d - 1) / 2;
    let matrix = evaluation_matrix(instance, assignment_mask, target_sign);
    let Some(kernel) = null_vector(&matrix, p) else {
        return false;
    };
    let a = poly_trim(kernel[..u].to_vec());
    let b = poly_trim(kernel[u..u + v].to_vec());
    if a.is_empty() && b.is_empty() {
        return false;
    }
    let f = vec![instance.curve.b, instance.curve.a, 0, 1];
    let norm = poly_sub(
        &poly_mul(&a, &a, p),
        &poly_mul(&f, &poly_mul(&b, &b, p), p),
        p,
    );
    let g = root_poly(&instance.xs(), p);
    if norm.is_empty() || norm.len() != g.len() {
        return false;
    }
    let c = *norm.last().unwrap();
    if c == 0 || norm != poly_scale(&g, c, p) {
        return false;
    }
    for i in 0..d {
        let q = if i < instance.k() {
            instance.support[i]
        } else {
            instance.target
        };
        let negative = if i < instance.k() {
            assignment_mask >> i & 1 == 1
        } else {
            target_sign < 0
        };
        let y = if negative { neg(q.y, p) } else { q.y };
        if add(poly_eval(&a, q.x, p), mul(y, poly_eval(&b, q.x, p), p), p) != 0 {
            return false;
        }
    }
    true
}

fn validate_instance(instance: &BranchInstance) -> Result<(), String> {
    let curve = &instance.curve;
    if !is_prime(curve.p) || !is_prime(curve.order) {
        return Err("field or group order not prime".into());
    }
    if !curve.on_curve(curve.generator) || !curve.on_curve(instance.target) {
        return Err("generator or target off curve".into());
    }
    if curve.scalar_mul(curve.order, curve.generator) != Point::Infinity {
        return Err("generator order check failed".into());
    }
    let mut fibres = BTreeSet::from([instance.target.x]);
    for (label, q) in instance.support_labels.iter().zip(&instance.support) {
        if *label == 0 || !curve.on_curve(*q) {
            return Err("zero label or support point off curve".into());
        }
        if curve.scalar_mul(*label, curve.generator) != Point::Affine(*q) {
            return Err("support label mismatch".into());
        }
        if q.y == 0 || !fibres.insert(q.x) {
            return Err("zero ordinate or duplicate fibre".into());
        }
    }
    if instance.target_label == 0
        || curve.scalar_mul(instance.target_label, curve.generator)
            != Point::Affine(instance.target)
        || instance.target.y == 0
    {
        return Err("target label mismatch or zero ordinate".into());
    }
    Ok(())
}

fn validate_degree_window(d: usize, u: usize, v: usize) -> Result<(), String> {
    if u + v != d || u != d / 2 + 1 || v != (d - 1) / 2 {
        return Err("degree window does not match the frozen norm-branch chart".into());
    }
    Ok(())
}

fn popcount_masks(bits: usize, weight: usize) -> Vec<usize> {
    (0..1usize << bits)
        .filter(|m| m.count_ones() as usize == weight)
        .collect()
}

fn compose_global_mask(row: usize, col: usize, left: &[usize], right: &[usize]) -> usize {
    let mut out = 0;
    for (j, index) in left.iter().enumerate() {
        out |= ((row >> j) & 1) << index;
    }
    for (j, index) in right.iter().enumerate() {
        out |= ((col >> j) & 1) << index;
    }
    out
}

fn product_kernel(
    instance: &BranchInstance,
    left: &[usize],
    right: &[usize],
    row_masks: &[usize],
    col_masks: &[usize],
) -> Vec<Vec<u64>> {
    let p = instance.curve.p;
    row_masks
        .iter()
        .map(|row| {
            col_masks
                .iter()
                .map(|col| {
                    let mut out = 1;
                    for (li, global_l) in left.iter().enumerate() {
                        if row >> li & 1 == 0 {
                            continue;
                        }
                        for (ri, global_r) in right.iter().enumerate() {
                            if col >> ri & 1 == 1 {
                                let delta = sub(
                                    instance.support[*global_r].x,
                                    instance.support[*global_l].x,
                                    p,
                                );
                                out = mul(out, mul(delta, delta, p), p);
                            }
                        }
                    }
                    out
                })
                .collect()
        })
        .collect()
}

fn block_matrix(
    coeffs: &[u64],
    left: &[usize],
    right: &[usize],
    row_masks: &[usize],
    col_masks: &[usize],
) -> Vec<Vec<u64>> {
    row_masks
        .iter()
        .map(|row| {
            col_masks
                .iter()
                .map(|col| coeffs[compose_global_mask(*row, *col, left, right)])
                .collect()
        })
        .collect()
}

fn scaling_equivalent(c: &[Vec<u64>], k: &[Vec<u64>], p: u64) -> bool {
    if c.is_empty() || c[0].is_empty() {
        return true;
    }
    for i in 0..c.len() {
        for j in 0..c[0].len() {
            if c[i][j] == 0 || k[i][j] == 0 {
                return false;
            }
            let lhs = mul(mul(mul(c[i][j], c[0][0], p), k[i][0], p), k[0][j], p);
            let rhs = mul(mul(mul(c[i][0], c[0][j], p), k[i][j], p), k[0][0], p);
            if lhs != rhs {
                return false;
            }
        }
    }
    true
}

fn permanent_dp(matrix: &[Vec<u64>], p: u64) -> u64 {
    let n = matrix.len();
    assert!(matrix.iter().all(|row| row.len() == n));
    let mut dp = vec![0; 1usize << n];
    dp[0] = 1;
    for mask in 0..1usize << n {
        let row = mask.count_ones() as usize;
        if row == n {
            continue;
        }
        for (col, entry) in matrix[row].iter().enumerate() {
            if mask >> col & 1 == 0 {
                let next = mask | (1 << col);
                dp[next] = add(dp[next], mul(dp[mask], *entry, p), p);
            }
        }
    }
    dp[(1usize << n) - 1]
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockDefect {
    pub split_mask: u64,
    pub target_in: bool,
    pub left_weight: usize,
    pub right_weight: usize,
    pub rows: usize,
    pub cols: usize,
    pub natural_rank: usize,
    pub kernel_rank: usize,
    pub null_rank: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockSweep {
    pub blocks_checked: usize,
    pub scaling_checks: usize,
    pub ordinate_invariance_checks: usize,
    pub mismatches: usize,
    pub defects: Vec<BlockDefect>,
    pub extreme_cauchy_checks: usize,
    pub extreme_permanent_zeroes: usize,
}

fn sweep_blocks(
    instance: &BranchInstance,
    split_mask: u64,
    natural: &[u64],
    null: &[u64],
    random_ordinates: &[u64],
    retain_full: bool,
) -> BlockSweep {
    let k = instance.k();
    let p = instance.curve.p;
    let d = k + 1;
    let v = (d - 1) / 2;
    let left: Vec<_> = (0..k).filter(|i| split_mask >> i & 1 == 1).collect();
    let right: Vec<_> = (0..k).filter(|i| split_mask >> i & 1 == 0).collect();
    let mut out = BlockSweep::default();
    for target_in in [false, true] {
        for a in 0..=left.len() {
            let Some(b) = v.checked_sub(a + usize::from(target_in)) else {
                continue;
            };
            if b > right.len() {
                continue;
            }
            let row_masks = popcount_masks(left.len(), a);
            let col_masks = popcount_masks(right.len(), b);
            if row_masks.is_empty() || col_masks.is_empty() {
                continue;
            }
            let c = block_matrix(natural, &left, &right, &row_masks, &col_masks);
            let random_y = block_matrix(random_ordinates, &left, &right, &row_masks, &col_masks);
            let null_block = block_matrix(null, &left, &right, &row_masks, &col_masks);
            let kernel = product_kernel(instance, &left, &right, &row_masks, &col_masks);
            let c_rank = rank_column_basis(&c, p);
            let y_rank = rank_column_basis(&random_y, p);
            let null_rank = rank_column_basis(&null_block, p);
            let kernel_rank = rank_column_basis(&kernel, p);
            out.blocks_checked += 1;
            out.scaling_checks += 1;
            out.ordinate_invariance_checks += 1;
            let scale_ok = scaling_equivalent(&c, &kernel, p);
            if !scale_ok || c_rank != kernel_rank || c_rank != y_rank {
                out.mismatches += 1;
            }
            let full = row_masks.len().min(col_masks.len());
            if c_rank < full || retain_full {
                out.defects.push(BlockDefect {
                    split_mask,
                    target_in,
                    left_weight: a,
                    right_weight: b,
                    rows: row_masks.len(),
                    cols: col_masks.len(),
                    natural_rank: c_rank,
                    kernel_rank,
                    null_rank,
                });
            }
            if !target_in && a == 1 && b + 1 == right.len() && left.len() == right.len() {
                let cauchy: Vec<Vec<u64>> = left
                    .iter()
                    .map(|i| {
                        right
                            .iter()
                            .map(|j| inv(sub(instance.support[*j].x, instance.support[*i].x, p), p))
                            .collect()
                    })
                    .collect();
                let inverse_square: Vec<Vec<u64>> = cauchy
                    .iter()
                    .map(|row| row.iter().map(|x| mul(*x, *x, p)).collect())
                    .collect();
                let (det_c, _) = determinant_gaussian(&cauchy, p);
                let (det_square, _) = determinant_gaussian(&inverse_square, p);
                let permanent = permanent_dp(&cauchy, p);
                assert_eq!(det_square, mul(det_c, permanent, p));
                assert_ne!(det_c, 0);
                assert_eq!(rank_column_basis(&inverse_square, p), kernel_rank);
                out.extreme_cauchy_checks += 1;
                if permanent == 0 {
                    out.extreme_permanent_zeroes += 1;
                }
            }
        }
    }
    out
}

fn block_rank_paths_agree(
    instance: &BranchInstance,
    split_mask: u64,
    natural: &[u64],
    null: &[u64],
    random_ordinates: &[u64],
) -> bool {
    let k = instance.k();
    let p = instance.curve.p;
    let v = k / 2;
    let left: Vec<_> = (0..k).filter(|i| split_mask >> i & 1 == 1).collect();
    let right: Vec<_> = (0..k).filter(|i| split_mask >> i & 1 == 0).collect();
    for target_in in [false, true] {
        for a in 0..=left.len() {
            let Some(b) = v.checked_sub(a + usize::from(target_in)) else {
                continue;
            };
            if b > right.len() {
                continue;
            }
            let row_masks = popcount_masks(left.len(), a);
            let col_masks = popcount_masks(right.len(), b);
            let matrices = [
                block_matrix(natural, &left, &right, &row_masks, &col_masks),
                block_matrix(null, &left, &right, &row_masks, &col_masks),
                block_matrix(random_ordinates, &left, &right, &row_masks, &col_masks),
                product_kernel(instance, &left, &right, &row_masks, &col_masks),
            ];
            if matrices
                .iter()
                .any(|matrix| rank_rref(matrix, p).rank != rank_column_basis(matrix, p))
            {
                return false;
            }
        }
    }
    true
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SplitRankRecord {
    pub left_mask: u64,
    pub rows: usize,
    pub cols: usize,
    pub natural_rank: usize,
    pub null_rank: usize,
    pub random_ordinate_rank: usize,
    pub second_vandermonde_rank: usize,
    pub natural_pivot_det: u64,
    pub natural_pivot_rows: Vec<usize>,
    pub natural_pivot_cols: Vec<usize>,
    pub null_pivot_det: u64,
    pub null_pivot_rows: Vec<usize>,
    pub null_pivot_cols: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellRecord {
    pub p: u64,
    pub seed: u64,
    pub k: usize,
    pub target_label: u64,
    pub target: AffinePoint,
    pub support_labels: Vec<u64>,
    pub support: Vec<AffinePoint>,
    pub coefficient_nonzero_edges: usize,
    pub coefficient_bit_growth_bound: usize,
    pub model_count: usize,
    pub least_model: Option<u64>,
    pub semantic_model_sets_equal: bool,
    pub least_model_norm_replay: bool,
    pub direct_matrix_assignments_checked: usize,
    pub splits: Vec<SplitRankRecord>,
    pub block_sweep: BlockSweep,
    pub separable_control_rank: usize,
    pub matrix_bytes_max: usize,
    pub ops: Ops,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TinySemanticRecord {
    pub assignments: usize,
    pub group_models: Vec<u64>,
    pub coefficient_models: Vec<u64>,
    pub matrix_models: Vec<u64>,
    pub every_model_norm_replayed: bool,
    pub target_sign_flip_changes_models: bool,
    pub ordinate_flip_changes_models: bool,
    pub duplicate_x_rejected: bool,
    pub curve_mutation_rejected: bool,
    pub changed_degree_budget_rejected: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CounterexampleRecord {
    pub integer_matrix: [[i64; 2]; 2],
    pub canonical_laplace_matrix: [[i64; 2]; 2],
    pub rank_equivalent_by_row_column_signs: bool,
    pub integer_determinant: i64,
    pub modular_ranks: BTreeMap<u64, usize>,
    pub all_entries_nonzero: bool,
    pub cross_factor_mutation_nonsingular: bool,
    pub duplicate_abscissa_rejected: bool,
    pub target_inclusion_mutation_changes_block: bool,
    pub cardinality_mutation_changes_dimensions: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperimentResult {
    pub schema: String,
    pub protocol_id: String,
    pub source_revision: String,
    pub protocol_sha256: String,
    pub curves: Vec<CurveFixture>,
    pub tiny_semantic: TinySemanticRecord,
    pub counterexample: CounterexampleRecord,
    pub cells: Vec<CellRecord>,
    pub heldout_sizes: Vec<usize>,
    pub advance_threshold_rank_ratio_numerator: usize,
    pub advance_threshold_rank_ratio_denominator: usize,
    pub advance_threshold_slope_gap: String,
    pub scope: String,
}

fn lowercase_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn family_complete(result: &ExperimentResult) -> bool {
    let expected_curves: Vec<_> = PRIMES.into_iter().collect();
    let observed_curves: Vec<_> = result.curves.iter().map(|curve| curve.p).collect();
    let expected_cells: Vec<_> = PRIMES
        .into_iter()
        .flat_map(|p| {
            SUPPORT_SEEDS
                .into_iter()
                .flat_map(move |seed| K_LADDER.into_iter().map(move |k| (p, seed, k)))
        })
        .collect();
    let observed_cells: Vec<_> = result
        .cells
        .iter()
        .map(|cell| (cell.p, cell.seed, cell.k))
        .collect();
    observed_curves == expected_curves
        && observed_cells == expected_cells
        && result.heldout_sizes == [12, 14]
        && result.advance_threshold_rank_ratio_numerator == 1
        && result.advance_threshold_rank_ratio_denominator == 2
        && result.advance_threshold_slope_gap == "0.125 log2(rank) per Boolean variable"
        && lowercase_hex(&result.source_revision, 40)
        && lowercase_hex(&result.protocol_sha256, 64)
        && result.cells.iter().all(|cell| {
            cell.support.len() == cell.k
                && cell.support_labels.len() == cell.k
                && cell
                    .splits
                    .iter()
                    .map(|split| split.left_mask)
                    .collect::<Vec<_>>()
                    == balanced_splits(cell.k)
        })
}

fn tiny_fixture() -> BranchInstance {
    BranchInstance {
        curve: CurveFixture {
            p: 17,
            a: 2,
            b: 2,
            order: 19,
            generator: AffinePoint { x: 0, y: 6 },
            search_attempt: 0,
        },
        seed: 0,
        target_label: 1,
        target: AffinePoint { x: 0, y: 6 },
        support_labels: vec![4, 9, 2, 3, 8],
        support: vec![
            AffinePoint { x: 3, y: 1 },
            AffinePoint { x: 5, y: 1 },
            AffinePoint { x: 6, y: 3 },
            AffinePoint { x: 10, y: 11 },
            AffinePoint { x: 13, y: 7 },
        ],
    }
}

fn derive_labels(curve: &CurveFixture, points: &[AffinePoint]) -> Vec<u64> {
    points
        .iter()
        .map(|target| {
            (1..curve.order)
                .find(|n| curve.scalar_mul(*n, curve.generator) == Point::Affine(*target))
                .expect("point belongs to prime-order group")
        })
        .collect()
}

fn run_tiny_semantic() -> TinySemanticRecord {
    let mut fixture = tiny_fixture();
    fixture.support_labels = derive_labels(&fixture.curve, &fixture.support);
    fixture.target_label = derive_labels(&fixture.curve, &[fixture.target])[0];
    validate_instance(&fixture).expect("frozen F17 fixture validates");
    let coeffs = coefficients_from_nodes(&fixture.xs(), &fixture.ys(), 17).0;
    let group = group_models(&fixture);
    let coefficient = coefficient_models(&coeffs, 17);
    let matrix = direct_matrix_models(&fixture, -1);
    let every_model_norm_replayed = group.iter().all(|m| norm_replay(&fixture, *m, -1));
    let target_sign_models = direct_matrix_models(&fixture, 1);
    let mut ordinate_flipped = fixture.clone();
    ordinate_flipped.support[0].y = neg(ordinate_flipped.support[0].y, 17);
    ordinate_flipped.support_labels[0] =
        derive_labels(&ordinate_flipped.curve, &[ordinate_flipped.support[0]])[0];
    let ordinate_models = group_models(&ordinate_flipped);
    let mut duplicate = fixture.clone();
    duplicate.support[1].x = duplicate.support[0].x;
    let mut bad_curve = fixture.clone();
    bad_curve.curve.a = add(bad_curve.curve.a, 1, 17);
    TinySemanticRecord {
        assignments: 1 << fixture.k(),
        group_models: group.clone(),
        coefficient_models: coefficient.clone(),
        matrix_models: matrix.clone(),
        every_model_norm_replayed,
        target_sign_flip_changes_models: target_sign_models != matrix,
        ordinate_flip_changes_models: ordinate_models != group,
        duplicate_x_rejected: validate_instance(&duplicate).is_err(),
        curve_mutation_rejected: validate_instance(&bad_curve).is_err(),
        changed_degree_budget_rejected: validate_degree_window(
            fixture.k() + 1,
            fixture.k().div_ceil(2),
            ((fixture.k() + 1) - 1) / 2,
        )
        .is_err(),
    }
}

fn run_counterexample() -> CounterexampleRecord {
    let integer_matrix = [[-252, 108], [378, -162]];
    let canonical_laplace_matrix = [[252, 108], [378, 162]];
    let integer_determinant =
        integer_matrix[0][0] * integer_matrix[1][1] - integer_matrix[0][1] * integer_matrix[1][0];
    let mut modular_ranks = BTreeMap::new();
    for p in PRIMES {
        let matrix: Vec<Vec<u64>> = integer_matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| ((*x as i128).rem_euclid(p as i128)) as u64)
                    .collect()
            })
            .collect();
        modular_ranks.insert(p, rank_column_basis(&matrix, p));
    }
    let fixture_p = PRIMES[0];
    let fixture_coefficients =
        coefficients_from_nodes(&[0, 3, 6, 2, 9], &[1, 1, 1, 1, 1], fixture_p).0;
    let left = vec![0, 1];
    let right = vec![2, 3];
    let base_block = block_matrix(
        &fixture_coefficients,
        &left,
        &right,
        &popcount_masks(left.len(), 1),
        &popcount_masks(right.len(), 1),
    );
    let expected_modular_block: Vec<Vec<u64>> = canonical_laplace_matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| ((*x as i128).rem_euclid(fixture_p as i128)) as u64)
                .collect()
        })
        .collect();
    assert_eq!(base_block, expected_modular_block);
    let target_in_block = block_matrix(
        &fixture_coefficients,
        &left,
        &right,
        &popcount_masks(left.len(), 1),
        &popcount_masks(right.len(), 0),
    );
    let changed_cardinality_block = block_matrix(
        &fixture_coefficients,
        &left,
        &right,
        &popcount_masks(left.len(), 0),
        &popcount_masks(right.len(), 2),
    );
    let mutated = [[-251i64, 108], [378, -162]];
    CounterexampleRecord {
        integer_matrix,
        canonical_laplace_matrix,
        rank_equivalent_by_row_column_signs: canonical_laplace_matrix
            == [
                [-integer_matrix[0][0], integer_matrix[0][1]],
                [integer_matrix[1][0], -integer_matrix[1][1]],
            ],
        integer_determinant,
        modular_ranks,
        all_entries_nonzero: integer_matrix.iter().flatten().all(|x| *x != 0),
        cross_factor_mutation_nonsingular: mutated[0][0] * mutated[1][1]
            - mutated[0][1] * mutated[1][0]
            != 0,
        duplicate_abscissa_rejected: [0u64, 3, 6, 2, 0]
            .into_iter()
            .collect::<BTreeSet<_>>()
            .len()
            != 5,
        target_inclusion_mutation_changes_block: target_in_block != base_block,
        cardinality_mutation_changes_dimensions: changed_cardinality_block != base_block,
    }
}

fn mutation_controls_pass(
    tiny: &TinySemanticRecord,
    counterexample: &CounterexampleRecord,
) -> bool {
    tiny.target_sign_flip_changes_models
        && tiny.ordinate_flip_changes_models
        && tiny.duplicate_x_rejected
        && tiny.curve_mutation_rejected
        && tiny.changed_degree_budget_rejected
        && counterexample.all_entries_nonzero
        && counterexample.rank_equivalent_by_row_column_signs
        && counterexample.cross_factor_mutation_nonsingular
        && counterexample.duplicate_abscissa_rejected
        && counterexample.target_inclusion_mutation_changes_block
        && counterexample.cardinality_mutation_changes_dimensions
}

fn merge_block_sweep(target: &mut BlockSweep, mut source: BlockSweep) {
    target.blocks_checked += source.blocks_checked;
    target.scaling_checks += source.scaling_checks;
    target.ordinate_invariance_checks += source.ordinate_invariance_checks;
    target.mismatches += source.mismatches;
    target.extreme_cauchy_checks += source.extreme_cauchy_checks;
    target.extreme_permanent_zeroes += source.extreme_permanent_zeroes;
    target.defects.append(&mut source.defects);
}

fn sampled_assignment_masks(k: usize, seed: u64, least: Option<u64>) -> Vec<u64> {
    if k <= 8 {
        return (0..1u64 << k).collect();
    }
    let all = (1u64 << k) - 1;
    let mut set = BTreeSet::from([0, all]);
    if let Some(mask) = least {
        set.insert(mask);
    }
    let mut rng = SplitMix64::new(seed ^ 0x4e42_434d_4154_5249);
    while set.len() < 16 {
        set.insert(rng.next() & all);
    }
    set.into_iter().collect()
}

fn separable_control_rank(k: usize, p: u64) -> usize {
    let half = k / 2;
    let mut separable = vec![vec![0; 1usize << half]; 1usize << half];
    for (i, row) in separable.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            *value = mul((i as u64 + 1) % p, (j as u64 + 1) % p, p);
        }
    }
    rank_column_basis(&separable, p)
}

fn run_cell(instance: BranchInstance) -> CellRecord {
    validate_instance(&instance).expect("generated instance validates");
    let p = instance.curve.p;
    let k = instance.k();
    let xs = instance.xs();
    let ys = instance.ys();
    let (natural, coefficient_ops) = coefficients_from_nodes(&xs, &ys, p);
    assert_eq!(natural, coefficients_direct_minors(&xs, &ys, p));
    let null_seed =
        instance.seed ^ (k as u64).rotate_left(9) ^ p.rotate_left(31) ^ 0x4e42_434e_554c_4c00;
    let ordinate_seed =
        instance.seed ^ (k as u64).rotate_left(21) ^ p.rotate_left(7) ^ 0x4e42_434f_5244_0000;
    let null = random_coefficient_null(&natural, p, null_seed);
    let random_ordinates = random_ordinate_coefficients(&instance, ordinate_seed);
    let second = second_vandermonde_coefficients(&instance);
    let coefficient_model_set = coefficient_models(&natural, p);
    let group_model_set = group_models(&instance);
    let semantic_model_sets_equal = coefficient_model_set == group_model_set;
    let least_model = coefficient_model_set.first().copied();
    let least_model_norm_replay = least_model
        .map(|m| norm_replay(&instance, m, -1))
        .unwrap_or(true);
    let sampled_masks = sampled_assignment_masks(k, instance.seed ^ p, least_model);
    for mask in &sampled_masks {
        let singular = rank_column_basis(&evaluation_matrix(&instance, *mask, -1), p) < k + 1;
        let determinant_zero = fwt_values(&natural, p)[*mask as usize] == 0;
        let group_hit = group_model_set.binary_search(mask).is_ok();
        assert_eq!(singular, determinant_zero);
        assert_eq!(singular, group_hit);
        if singular {
            assert!(norm_replay(&instance, *mask, -1));
        }
    }

    let splits = balanced_splits(k);
    let mut split_records = Vec::with_capacity(splits.len());
    let mut block_sweep = BlockSweep::default();
    let mut ops = coefficient_ops;
    let mut matrix_bytes_max = 0;
    for split_mask in splits {
        let natural_matrix = matrix_from_coefficients(&natural, split_mask, k);
        let null_matrix = matrix_from_coefficients(&null, split_mask, k);
        let ordinate_matrix = matrix_from_coefficients(&random_ordinates, split_mask, k);
        let second_matrix = matrix_from_coefficients(&second, split_mask, k);
        matrix_bytes_max = matrix_bytes_max.max(natural_matrix.len() * natural_matrix[0].len() * 8);
        let natural_rank = rank_rref(&natural_matrix, p);
        let null_rank = rank_rref(&null_matrix, p);
        let ordinate_rank = rank_rref(&ordinate_matrix, p);
        let second_rank = rank_rref(&second_matrix, p);
        ops.merge(natural_rank.ops);
        ops.merge(null_rank.ops);
        ops.merge(ordinate_rank.ops);
        ops.merge(second_rank.ops);
        merge_block_sweep(
            &mut block_sweep,
            sweep_blocks(
                &instance,
                split_mask,
                &natural,
                &null,
                &random_ordinates,
                false,
            ),
        );
        split_records.push(SplitRankRecord {
            left_mask: split_mask,
            rows: natural_matrix.len(),
            cols: natural_matrix[0].len(),
            natural_rank: natural_rank.rank,
            null_rank: null_rank.rank,
            random_ordinate_rank: ordinate_rank.rank,
            second_vandermonde_rank: second_rank.rank,
            natural_pivot_det: natural_rank.pivot_minor_det,
            natural_pivot_rows: natural_rank.pivot_rows,
            natural_pivot_cols: natural_rank.pivot_cols,
            null_pivot_det: null_rank.pivot_minor_det,
            null_pivot_rows: null_rank.pivot_rows,
            null_pivot_cols: null_rank.pivot_cols,
        });
    }
    let d = k + 1;
    let u = d / 2 + 1;
    let v = (d - 1) / 2;
    let field_bits = (64 - (p - 1).leading_zeros()) as usize;
    let coefficient_bit_growth_bound = (u * (u - 1) / 2 + v * (v - 1) / 2 + v) * field_bits;
    CellRecord {
        p,
        seed: instance.seed,
        k,
        target_label: instance.target_label,
        target: instance.target,
        support_labels: instance.support_labels,
        support: instance.support,
        coefficient_nonzero_edges: natural.iter().filter(|x| **x != 0).count(),
        coefficient_bit_growth_bound,
        model_count: coefficient_model_set.len(),
        least_model,
        semantic_model_sets_equal,
        least_model_norm_replay,
        direct_matrix_assignments_checked: sampled_masks.len(),
        splits: split_records,
        block_sweep,
        separable_control_rank: separable_control_rank(k, p),
        matrix_bytes_max,
        ops,
    }
}

pub fn run_experiment(source_revision: String, protocol_sha256: String) -> ExperimentResult {
    let tiny_semantic = run_tiny_semantic();
    assert_eq!(tiny_semantic.group_models, tiny_semantic.coefficient_models);
    assert_eq!(tiny_semantic.group_models, tiny_semantic.matrix_models);
    assert!(tiny_semantic.every_model_norm_replayed);
    assert!(tiny_semantic.target_sign_flip_changes_models);
    assert!(tiny_semantic.ordinate_flip_changes_models);
    assert!(tiny_semantic.duplicate_x_rejected);
    assert!(tiny_semantic.curve_mutation_rejected);
    assert!(tiny_semantic.changed_degree_budget_rejected);
    let counterexample = run_counterexample();
    assert_eq!(counterexample.integer_determinant, 0);
    assert!(counterexample.modular_ranks.values().all(|r| *r == 1));
    assert!(mutation_controls_pass(&tiny_semantic, &counterexample));
    let curves: Vec<_> = PRIMES.into_iter().map(find_curve).collect();
    let mut cells = Vec::new();
    for curve in &curves {
        for seed in SUPPORT_SEEDS {
            let maximal = sample_max_instance(curve, seed, *K_LADDER.last().unwrap());
            for k in K_LADDER {
                cells.push(run_cell(prefix_instance(&maximal, k)));
            }
        }
    }
    ExperimentResult {
        schema: "norm.branch-census/v1".into(),
        protocol_id: PROTOCOL_ID.into(),
        source_revision,
        protocol_sha256,
        curves,
        tiny_semantic,
        counterexample,
        cells,
        heldout_sizes: vec![12, 14],
        advance_threshold_rank_ratio_numerator: 1,
        advance_threshold_rank_ratio_denominator: 2,
        advance_threshold_slope_gap: "0.125 log2(rank) per Boolean variable".into(),
        scope: "exact finite family and registered linear-message flattening only; no timing, SAT, DLP, security, novelty, or universal hardness claim".into(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckReceipt {
    pub schema: String,
    pub protocol_id: String,
    pub source_revision: String,
    pub protocol_sha256: String,
    pub artifact_cells: usize,
    pub curves_recounted: usize,
    pub coefficients_checked: usize,
    pub model_sets_checked: usize,
    pub split_ranks_checked: usize,
    pub block_checks_replayed: usize,
    pub pivot_determinants_nonzero: usize,
    pub errors: Vec<String>,
    pub pass: bool,
    pub independence: String,
}

pub fn check_experiment(result: &ExperimentResult) -> CheckReceipt {
    let mut receipt = CheckReceipt {
        schema: "norm.branch-census-check/v1".into(),
        protocol_id: result.protocol_id.clone(),
        source_revision: result.source_revision.clone(),
        protocol_sha256: result.protocol_sha256.clone(),
        artifact_cells: result.cells.len(),
        curves_recounted: 0,
        coefficients_checked: 0,
        model_sets_checked: 0,
        split_ranks_checked: 0,
        block_checks_replayed: 0,
        pivot_determinants_nonzero: 0,
        errors: vec![],
        pass: false,
        independence: "frozen curves and support instances regenerated from seeds; coefficients regenerated from explicit minor determinants; global ranks regenerated by incremental column bases; every producer pivot minor re-extracted; block ranks cross-checked by RREF and column bases; curve orders recounted; group models replayed independently of producer RREF evidence".into(),
    };
    if result.schema != "norm.branch-census/v1" || result.protocol_id != PROTOCOL_ID {
        receipt.errors.push("schema or protocol id mismatch".into());
    }
    if !family_complete(result) {
        receipt
            .errors
            .push("frozen curve/cell/split family is incomplete or duplicated".into());
        return receipt;
    }
    let expected_curves: Vec<_> = PRIMES.into_iter().map(find_curve).collect();
    if result.curves != expected_curves {
        receipt
            .errors
            .push("curve search does not reproduce the frozen first prime-order curves".into());
        return receipt;
    }
    let tiny = run_tiny_semantic();
    if tiny != result.tiny_semantic {
        receipt.errors.push("tiny semantic record mismatch".into());
    }
    if run_counterexample() != result.counterexample {
        receipt.errors.push("counterexample replay mismatch".into());
    }
    if !mutation_controls_pass(&result.tiny_semantic, &result.counterexample) {
        receipt
            .errors
            .push("one or more mutation controls failed".into());
    }
    let curve_map: BTreeMap<_, _> = result.curves.iter().map(|c| (c.p, c)).collect();
    let mut expected_instances = BTreeMap::new();
    for curve in &expected_curves {
        for seed in SUPPORT_SEEDS {
            let maximal = sample_max_instance(curve, seed, *K_LADDER.last().unwrap());
            for k in K_LADDER {
                expected_instances.insert((curve.p, seed, k), prefix_instance(&maximal, k));
            }
        }
    }
    for curve in &result.curves {
        if curve_order(curve.p, curve.a, curve.b) != curve.order
            || !is_prime(curve.order)
            || !curve.on_curve(curve.generator)
            || curve.scalar_mul(curve.order, curve.generator) != Point::Infinity
        {
            receipt
                .errors
                .push(format!("curve replay failed for p={}", curve.p));
        }
        receipt.curves_recounted += 1;
    }
    for cell in &result.cells {
        let Some(curve) = curve_map.get(&cell.p) else {
            receipt
                .errors
                .push(format!("missing curve for p={}", cell.p));
            continue;
        };
        let instance = BranchInstance {
            curve: (*curve).clone(),
            seed: cell.seed,
            target_label: cell.target_label,
            target: cell.target,
            support_labels: cell.support_labels.clone(),
            support: cell.support.clone(),
        };
        if expected_instances.get(&(cell.p, cell.seed, cell.k)) != Some(&instance) {
            receipt.errors.push(format!(
                "frozen sampler mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
            continue;
        }
        if let Err(error) = validate_instance(&instance) {
            receipt
                .errors
                .push(format!("invalid cell p={} k={}: {error}", cell.p, cell.k));
            continue;
        }
        let xs = instance.xs();
        let ys = instance.ys();
        let direct = coefficients_direct_minors(&xs, &ys, cell.p);
        let product = coefficients_from_nodes(&xs, &ys, cell.p).0;
        if direct != product {
            receipt.errors.push(format!(
                "coefficient rewrite mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
            continue;
        }
        receipt.coefficients_checked += direct.iter().filter(|x| **x != 0).count();
        let coefficient_model_set = coefficient_models(&direct, cell.p);
        let group_model_set = group_models(&instance);
        let expected_least = coefficient_model_set.first().copied();
        let expected_norm_replay = expected_least
            .map(|mask| norm_replay(&instance, mask, -1))
            .unwrap_or(true);
        let expected_sample_count =
            sampled_assignment_masks(cell.k, cell.seed ^ cell.p, expected_least).len();
        if coefficient_model_set != group_model_set
            || coefficient_model_set.len() != cell.model_count
            || expected_least != cell.least_model
            || cell.semantic_model_sets_equal != (coefficient_model_set == group_model_set)
            || cell.least_model_norm_replay != expected_norm_replay
            || cell.direct_matrix_assignments_checked != expected_sample_count
            || cell.coefficient_nonzero_edges != direct.iter().filter(|value| **value != 0).count()
        {
            receipt.errors.push(format!(
                "model set mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
        }
        receipt.model_sets_checked += 1;
        let null_seed = cell.seed
            ^ (cell.k as u64).rotate_left(9)
            ^ cell.p.rotate_left(31)
            ^ 0x4e42_434e_554c_4c00;
        let ordinate_seed = cell.seed
            ^ (cell.k as u64).rotate_left(21)
            ^ cell.p.rotate_left(7)
            ^ 0x4e42_434f_5244_0000;
        let null = random_coefficient_null(&direct, cell.p, null_seed);
        let random_ordinates = random_ordinate_coefficients(&instance, ordinate_seed);
        let second = second_vandermonde_coefficients(&instance);
        let d = cell.k + 1;
        let u = d / 2 + 1;
        let v = (d - 1) / 2;
        let field_bits = (64 - (cell.p - 1).leading_zeros()) as usize;
        let expected_bit_growth = (u * (u - 1) / 2 + v * (v - 1) / 2 + v) * field_bits;
        let expected_matrix_bytes = cell
            .splits
            .iter()
            .map(|split| split.rows * split.cols * 8)
            .max()
            .unwrap_or(0);
        if cell.coefficient_bit_growth_bound != expected_bit_growth
            || cell.matrix_bytes_max != expected_matrix_bytes
        {
            receipt.errors.push(format!(
                "resource invariant mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
        }
        if balanced_splits(cell.k).len() != cell.splits.len() {
            receipt.errors.push(format!(
                "split count mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
        }
        let mut replayed_block_sweep = BlockSweep::default();
        for reported in &cell.splits {
            let natural_matrix = matrix_from_coefficients(&direct, reported.left_mask, cell.k);
            let null_matrix = matrix_from_coefficients(&null, reported.left_mask, cell.k);
            let ordinate_matrix =
                matrix_from_coefficients(&random_ordinates, reported.left_mask, cell.k);
            let second_matrix = matrix_from_coefficients(&second, reported.left_mask, cell.k);
            let got = [
                rank_column_basis(&natural_matrix, cell.p),
                rank_column_basis(&null_matrix, cell.p),
                rank_column_basis(&ordinate_matrix, cell.p),
                rank_column_basis(&second_matrix, cell.p),
            ];
            let expected = [
                reported.natural_rank,
                reported.null_rank,
                reported.random_ordinate_rank,
                reported.second_vandermonde_rank,
            ];
            if got != expected {
                receipt.errors.push(format!(
                    "independent rank mismatch p={} k={} seed={} split={}",
                    cell.p, cell.k, cell.seed, reported.left_mask
                ));
            }
            if reported.rows != natural_matrix.len()
                || reported.cols != natural_matrix.first().map_or(0, Vec::len)
            {
                receipt.errors.push(format!(
                    "matrix dimension mismatch p={} k={} seed={} split={}",
                    cell.p, cell.k, cell.seed, reported.left_mask
                ));
            }
            let pivot_checks = [
                (
                    "natural",
                    &natural_matrix,
                    reported.natural_rank,
                    reported.natural_pivot_det,
                    &reported.natural_pivot_rows,
                    &reported.natural_pivot_cols,
                ),
                (
                    "null",
                    &null_matrix,
                    reported.null_rank,
                    reported.null_pivot_det,
                    &reported.null_pivot_rows,
                    &reported.null_pivot_cols,
                ),
            ];
            for (name, matrix, rank, claimed_det, pivot_rows, pivot_cols) in pivot_checks {
                let indices_valid = pivot_rows.len() == rank
                    && pivot_cols.len() == rank
                    && pivot_rows.iter().all(|row| *row < matrix.len())
                    && pivot_cols
                        .iter()
                        .all(|col| matrix.first().is_some_and(|row| *col < row.len()));
                if !indices_valid {
                    receipt.errors.push(format!(
                        "invalid {name} pivot indices p={} k={} seed={} split={}",
                        cell.p, cell.k, cell.seed, reported.left_mask
                    ));
                    continue;
                }
                let minor: Vec<Vec<u64>> = pivot_rows
                    .iter()
                    .map(|row| pivot_cols.iter().map(|col| matrix[*row][*col]).collect())
                    .collect();
                let (checked_det, _) = determinant_gaussian(&minor, cell.p);
                if claimed_det == 0 || checked_det != claimed_det {
                    receipt.errors.push(format!(
                        "{name} pivot determinant mismatch p={} k={} seed={} split={} claimed={} checked={}",
                        cell.p,
                        cell.k,
                        cell.seed,
                        reported.left_mask,
                        claimed_det,
                        checked_det
                    ));
                } else {
                    receipt.pivot_determinants_nonzero += 1;
                }
            }
            let sweep = sweep_blocks(
                &instance,
                reported.left_mask,
                &direct,
                &null,
                &random_ordinates,
                false,
            );
            receipt.block_checks_replayed += sweep.blocks_checked;
            if sweep.mismatches != 0 {
                receipt.errors.push(format!(
                    "block rewrite/invariance mismatch p={} k={} seed={} split={}",
                    cell.p, cell.k, cell.seed, reported.left_mask
                ));
            }
            if !block_rank_paths_agree(
                &instance,
                reported.left_mask,
                &direct,
                &null,
                &random_ordinates,
            ) {
                receipt.errors.push(format!(
                    "independent block-rank mismatch p={} k={} seed={} split={}",
                    cell.p, cell.k, cell.seed, reported.left_mask
                ));
            }
            merge_block_sweep(&mut replayed_block_sweep, sweep);
            receipt.split_ranks_checked += 1;
        }
        if replayed_block_sweep != cell.block_sweep {
            receipt.errors.push(format!(
                "block-sweep record mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
        }
        if cell.separable_control_rank != separable_control_rank(cell.k, cell.p) {
            receipt.errors.push(format!(
                "separable positive-control mismatch p={} k={} seed={}",
                cell.p, cell.k, cell.seed
            ));
        }
    }
    receipt.pass = receipt.errors.is_empty();
    receipt
}

fn has_growing_block_defect(c12: &CellRecord, c14: &CellRecord) -> bool {
    c12.block_sweep.defects.iter().any(|small| {
        c14.block_sweep.defects.iter().any(|large| {
            let left_growth = large.left_weight.checked_sub(small.left_weight);
            let right_growth = large.right_weight.checked_sub(small.right_weight);
            let small_full = small.rows.min(small.cols);
            let large_full = large.rows.min(large.cols);
            small.target_in == large.target_in
                && matches!(
                    (left_growth, right_growth),
                    (Some(1), Some(0)) | (Some(0), Some(1))
                )
                && large_full >= small_full
                && large_full - large.natural_rank >= small_full - small.natural_rank
        })
    })
}

pub fn analyze_decision(result: &ExperimentResult) -> (&'static str, String) {
    let controls_pass = result.tiny_semantic.group_models
        == result.tiny_semantic.coefficient_models
        && result.tiny_semantic.group_models == result.tiny_semantic.matrix_models
        && result.tiny_semantic.every_model_norm_replayed
        && result.counterexample.integer_determinant == 0
        && result
            .counterexample
            .modular_ranks
            .values()
            .all(|r| *r == 1)
        && mutation_controls_pass(&result.tiny_semantic, &result.counterexample)
        && family_complete(result)
        && result.cells.iter().all(|c| {
            c.semantic_model_sets_equal
                && c.least_model_norm_replay
                && c.block_sweep.mismatches == 0
                && c.separable_control_rank == 1
        });
    if !controls_pass {
        return (
            "INCONCLUSIVE",
            "one or more semantic, mutation, rewrite, or positive controls failed".into(),
        );
    }
    let heldout: Vec<_> = result
        .cells
        .iter()
        .filter(|c| c.k == 12 || c.k == 14)
        .collect();
    let kill = heldout.iter().all(|cell| {
        cell.block_sweep.defects.is_empty()
            && cell
                .splits
                .iter()
                .all(|s| s.natural_rank == s.rows.min(s.cols) && s.natural_rank == s.null_rank)
    });
    if kill {
        return ("KILL_MEASURED_FAMILY", "every held-out global flattening and fixed-cardinality block has the available full rank and matches the support-matched null".into());
    }
    let mut advance = true;
    for p in PRIMES {
        for seed in SUPPORT_SEEDS {
            let c12 = heldout
                .iter()
                .find(|c| c.p == p && c.seed == seed && c.k == 12);
            let c14 = heldout
                .iter()
                .find(|c| c.p == p && c.seed == seed && c.k == 14);
            let (Some(c12), Some(c14)) = (c12, c14) else {
                advance = false;
                continue;
            };
            let pair = |cell: &CellRecord| {
                let selected = cell
                    .splits
                    .iter()
                    .min_by_key(|split| (split.natural_rank, split.left_mask))
                    .unwrap();
                (selected.natural_rank, selected.null_rank)
            };
            let (n12, r12) = pair(c12);
            let (n14, r14) = pair(c14);
            let ratio_gate = 2 * n12 <= r12 && 2 * n14 <= r14;
            let natural_slope = ((n14 as f64).log2() - (n12 as f64).log2()) / 2.0;
            let null_slope = ((r14 as f64).log2() - (r12 as f64).log2()) / 2.0;
            let defect = has_growing_block_defect(c12, c14);
            advance &= ratio_gate && null_slope - natural_slope >= 0.125 && defect;
        }
    }
    if advance {
        ("ADVANCE_B2", "all preregistered held-out ratio, slope, and growing product-kernel defect gates passed".into())
    } else {
        ("INCONCLUSIVE", "held-out cells neither satisfy the strict advance gate nor the strict full-rank kill gate".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f17_semantic_gate() {
        let record = run_tiny_semantic();
        assert_eq!(record.group_models, record.coefficient_models);
        assert_eq!(record.group_models, record.matrix_models);
        assert!(record.every_model_norm_replayed);
        assert!(record.target_sign_flip_changes_models);
        assert!(record.ordinate_flip_changes_models);
        assert!(record.duplicate_x_rejected);
        assert!(record.curve_mutation_rejected);
        assert!(record.changed_degree_budget_rejected);
    }

    #[test]
    fn counterexample_replays() {
        let record = run_counterexample();
        assert_eq!(record.integer_determinant, 0);
        assert!(record.modular_ranks.values().all(|r| *r == 1));
        assert!(record.all_entries_nonzero);
        assert!(record.rank_equivalent_by_row_column_signs);
        assert!(record.cross_factor_mutation_nonsingular);
        assert!(record.duplicate_abscissa_rejected);
        assert!(record.target_inclusion_mutation_changes_block);
        assert!(record.cardinality_mutation_changes_dimensions);
    }

    #[test]
    fn coefficient_paths_agree_on_tiny_fixture() {
        let fixture = tiny_fixture();
        assert_eq!(
            coefficients_from_nodes(&fixture.xs(), &fixture.ys(), 17).0,
            coefficients_direct_minors(&fixture.xs(), &fixture.ys(), 17)
        );
    }

    #[test]
    fn split_counts_are_frozen() {
        assert_eq!(balanced_splits(6).len(), 10);
        assert_eq!(balanced_splits(8).len(), 35);
        assert_eq!(balanced_splits(10).len(), 126);
        assert_eq!(balanced_splits(12).len(), 8);
        assert_eq!(balanced_splits(14).len(), 8);
    }

    #[test]
    fn product_kernel_and_ordinate_invariance_hold_on_all_tiny_blocks() {
        let maximal = tiny_fixture();
        let instance = prefix_instance(&maximal, 4);
        let natural = coefficients_from_nodes(&instance.xs(), &instance.ys(), 17).0;
        let null = random_coefficient_null(&natural, 17, 7);
        let random_y = random_ordinate_coefficients(&instance, 11);
        for split in balanced_splits(4) {
            let sweep = sweep_blocks(&instance, split, &natural, &null, &random_y, false);
            assert_eq!(sweep.mismatches, 0);
        }
    }
}
