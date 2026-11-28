//! Bounded producer-side P-61 four-sum scalar-norm diagnostic.
//!
//! Derived from the public exact S5 routines of PR #5287. The row-wise norm
//! candidate snapshots a leaf-scale n^2 S5-call budget and then continues a
//! full diagnostic for planted and empty controls. It is neither a fast
//! oracle nor an independent PCC verifier, and it checks rational four-sums
//! rather than generic chord-only seven-sums.
//!
//! Replay: rustc --edition=2021 -D warnings -O p61_four_sum_norm_preflight.rs \
//!           -o /tmp/p61_four_sum_norm_preflight
//!         /tmp/p61_four_sum_norm_preflight > four_sum_norm_preflight.jsonl

#![allow(dead_code)]
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

static ADD_OPS: AtomicU64 = AtomicU64::new(0);
static SUB_OPS: AtomicU64 = AtomicU64::new(0);
static NEG_OPS: AtomicU64 = AtomicU64::new(0);
static MUL_OPS: AtomicU64 = AtomicU64::new(0);
static INV_OPS: AtomicU64 = AtomicU64::new(0);

fn reset_ops() {
    for counter in [&ADD_OPS, &SUB_OPS, &NEG_OPS, &MUL_OPS, &INV_OPS] {
        counter.store(0, Ordering::Relaxed);
    }
}

fn ops() -> [u64; 5] {
    [&ADD_OPS, &SUB_OPS, &NEG_OPS, &MUL_OPS, &INV_OPS]
        .map(|counter| counter.load(Ordering::Relaxed))
}

const P: u64 = 2_305_843_009_213_693_951;
const CURVE_A: u64 = P - 3;
const CURVE_B: u64 = 0x54aa66b458b77ec;
const GENERATOR: Point = Point {
    infinity: false,
    x: 0x112c917afe3d4fd3,
    y: 0x0ef721929b08bfaa,
};
const MAX_DEPTH: usize = 6;

type Poly = Vec<u64>;

fn add(a: u64, b: u64) -> u64 {
    ADD_OPS.fetch_add(1, Ordering::Relaxed);
    let z = a + b;
    if z >= P {
        z - P
    } else {
        z
    }
}

fn sub(a: u64, b: u64) -> u64 {
    SUB_OPS.fetch_add(1, Ordering::Relaxed);
    if a >= b {
        a - b
    } else {
        P - (b - a)
    }
}

fn neg(a: u64) -> u64 {
    NEG_OPS.fetch_add(1, Ordering::Relaxed);
    if a == 0 {
        0
    } else {
        P - a
    }
}

fn mul(a: u64, b: u64) -> u64 {
    MUL_OPS.fetch_add(1, Ordering::Relaxed);
    ((a as u128 * b as u128) % P as u128) as u64
}

fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut z = 1;
    while e != 0 {
        if e & 1 == 1 {
            z = mul(z, a);
        }
        e >>= 1;
        if e != 0 {
            a = mul(a, a);
        }
    }
    z
}

fn inv(a: u64) -> u64 {
    INV_OPS.fetch_add(1, Ordering::Relaxed);
    assert_ne!(a, 0);
    pow(a, P - 2)
}

fn sqrt(a: u64) -> Option<u64> {
    let z = pow(a, (P + 1) / 4);
    (mul(z, z) == a).then_some(z)
}

fn trim(mut f: Poly) -> Poly {
    while f.last() == Some(&0) {
        f.pop();
    }
    f
}

fn poly_sub(a: &[u64], b: &[u64]) -> Poly {
    let mut z = vec![0; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        z[i] = x;
    }
    for (i, &x) in b.iter().enumerate() {
        z[i] = sub(z[i], x);
    }
    trim(z)
}

fn poly_scale(a: &[u64], c: u64) -> Poly {
    if c == 0 {
        return Vec::new();
    }
    trim(a.iter().map(|&x| mul(x, c)).collect())
}

fn poly_mul(a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut z = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            z[i + j] = add(z[i + j], mul(x, y));
        }
    }
    trim(z)
}

fn poly_divrem(f: &[u64], g: &[u64]) -> (Poly, Poly) {
    assert!(!g.is_empty());
    let mut r = trim(f.to_vec());
    let mut q = vec![0; f.len().saturating_sub(g.len()) + 1];
    let lead_inv = inv(*g.last().unwrap());
    while r.len() >= g.len() {
        let shift = r.len() - g.len();
        let c = mul(*r.last().unwrap(), lead_inv);
        q[shift] = c;
        for (i, &x) in g.iter().enumerate() {
            r[i + shift] = sub(r[i + shift], mul(c, x));
        }
        r = trim(r);
    }
    (trim(q), r)
}

fn poly_rem(f: &[u64], g: &[u64]) -> Poly {
    poly_divrem(f, g).1
}

fn poly_from_roots(xs: &[u64]) -> Poly {
    xs.iter().fold(vec![1], |f, &x| poly_mul(&f, &[neg(x), 1]))
}

fn dickson_polynomials(n: usize) -> Vec<Poly> {
    assert!(n >= 1);
    let mut d = vec![vec![2], vec![0, 1]];
    for k in 1..n {
        let mut xdk = vec![0];
        xdk.extend_from_slice(&d[k]);
        d.push(poly_sub(&xdk, &d[k - 1]));
    }
    d
}

fn folded_sum_index(sum: usize, n: usize) -> Option<(usize, bool)> {
    if sum < n {
        Some((sum, false))
    } else if sum == n {
        None
    } else {
        Some((2 * n - sum, true))
    }
}

fn basis_product(i: usize, j: usize, n: usize) -> Vec<u64> {
    let mut z = vec![0; n];
    z[i.abs_diff(j)] = add(z[i.abs_diff(j)], 1);
    if let Some((k, negative)) = folded_sum_index(i + j, n) {
        z[k] = if negative { sub(z[k], 1) } else { add(z[k], 1) };
    }
    z
}

fn poly_to_coords(f: &[u64], dickson: &[Poly], n: usize) -> Vec<u64> {
    let mut r = f.to_vec();
    r.resize(n, 0);
    let mut out = vec![0; n];
    for k in (1..n).rev() {
        let c = r[k];
        out[k] = c;
        if c != 0 {
            for (j, &v) in dickson[k].iter().enumerate() {
                r[j] = sub(r[j], mul(c, v));
            }
        }
    }
    out[0] = mul(r[0], inv(2));
    out
}

fn quotient_product_table_ok(n: usize) -> bool {
    let d = dickson_polynomials(n);
    let modulus = &d[n];
    for i in 0..n {
        for j in 0..n {
            let direct = basis_product(i, j, n);
            let mono = poly_rem(&poly_mul(&d[i], &d[j]), modulus);
            if direct != poly_to_coords(&mono, &d, n) {
                return false;
            }
        }
    }
    true
}

fn add_matrix_entry(matrix: &mut [u64], n: usize, row: usize, col: usize, value: u64) {
    let index = row * n + col;
    matrix[index] = add(matrix[index], value);
}

fn multiplication_channels(a: &[u64]) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let n = a.len();
    let mut difference = vec![0; n * n];
    let mut sum_fold = vec![0; n * n];
    for (i, &coefficient) in a.iter().enumerate() {
        for j in 0..n {
            add_matrix_entry(&mut difference, n, i.abs_diff(j), j, coefficient);
            if let Some((row, negative)) = folded_sum_index(i + j, n) {
                add_matrix_entry(
                    &mut sum_fold,
                    n,
                    row,
                    j,
                    if negative {
                        neg(coefficient)
                    } else {
                        coefficient
                    },
                );
            }
        }
    }
    let total = difference
        .iter()
        .zip(&sum_fold)
        .map(|(&x, &y)| add(x, y))
        .collect();
    (difference, sum_fold, total)
}

fn multiplication_matrix_direct(a: &[u64]) -> Vec<u64> {
    let n = a.len();
    let mut matrix = vec![0; n * n];
    for (i, &coefficient) in a.iter().enumerate() {
        for j in 0..n {
            for (row, value) in basis_product(i, j, n).into_iter().enumerate() {
                matrix[row * n + j] = add(matrix[row * n + j], mul(coefficient, value));
            }
        }
    }
    matrix
}

fn matrix_rank(mut matrix: Vec<u64>, rows: usize, cols: usize) -> usize {
    let mut rank = 0;
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| matrix[row * cols + col] != 0) else {
            continue;
        };
        if pivot != rank {
            for j in col..cols {
                matrix.swap(rank * cols + j, pivot * cols + j);
            }
        }
        let scale = inv(matrix[rank * cols + col]);
        for j in col..cols {
            matrix[rank * cols + j] = mul(matrix[rank * cols + j], scale);
        }
        for row in 0..rows {
            if row == rank {
                continue;
            }
            let factor = matrix[row * cols + col];
            if factor == 0 {
                continue;
            }
            for j in col..cols {
                matrix[row * cols + j] =
                    sub(matrix[row * cols + j], mul(factor, matrix[rank * cols + j]));
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn displacement_down_right(matrix: &[u64], n: usize) -> Vec<u64> {
    let mut out = vec![0; n * n];
    for row in 0..n {
        for col in 0..n {
            let shifted = if row > 0 && col > 0 {
                matrix[(row - 1) * n + col - 1]
            } else {
                0
            };
            out[row * n + col] = sub(matrix[row * n + col], shifted);
        }
    }
    out
}

fn displacement_down_left(matrix: &[u64], n: usize) -> Vec<u64> {
    let mut out = vec![0; n * n];
    for row in 0..n {
        for col in 0..n {
            let shifted = if row > 0 && col + 1 < n {
                matrix[(row - 1) * n + col + 1]
            } else {
                0
            };
            out[row * n + col] = sub(matrix[row * n + col], shifted);
        }
    }
    out
}

fn dickson_roots(depth: usize) -> Vec<u64> {
    let mut roots = vec![0];
    for _ in 0..depth {
        let mut next = Vec::with_capacity(2 * roots.len());
        for value in roots {
            let root = sqrt(add(value, 2)).expect("Dickson fibre must split");
            let canonical = root.min(P - root);
            next.push(canonical);
            next.push(if canonical == 0 { 0 } else { P - canonical });
        }
        roots = next;
    }
    assert_eq!(
        roots.iter().copied().collect::<BTreeSet<_>>().len(),
        1 << depth
    );
    roots
}

fn s3_coefficients(x1: u64, x2: u64) -> [u64; 3] {
    let product = mul(x1, x2);
    let sum = add(x1, x2);
    let difference = sub(x1, x2);
    let c2 = mul(difference, difference);
    let inner = add(mul(sum, add(product, CURVE_A)), mul(2, CURVE_B));
    let c1 = neg(mul(2, inner));
    let part = sub(product, CURVE_A);
    let c0 = sub(mul(part, part), mul(mul(4, CURVE_B), sum));
    [c0, c1, c2]
}

fn s4_in_last(x1: u64, x2: u64, x3: u64) -> Poly {
    let alpha = s3_coefficients(x1, x2);
    let b2 = vec![mul(x3, x3), neg(mul(2, x3)), 1];
    let b1 = vec![
        neg(mul(2, add(mul(CURVE_A, x3), mul(2, CURVE_B)))),
        neg(mul(2, add(mul(x3, x3), CURVE_A))),
        neg(mul(2, x3)),
    ];
    let b0 = vec![
        sub(mul(CURVE_A, CURVE_A), mul(mul(4, CURVE_B), x3)),
        neg(add(mul(mul(2, CURVE_A), x3), mul(4, CURVE_B))),
        mul(x3, x3),
    ];
    let at = poly_sub(&poly_scale(&b0, alpha[2]), &poly_scale(&b2, alpha[0]));
    let bt = poly_sub(&poly_scale(&b1, alpha[2]), &poly_scale(&b2, alpha[1]));
    let ct = poly_sub(&poly_scale(&b0, alpha[1]), &poly_scale(&b1, alpha[0]));
    poly_sub(&poly_mul(&at, &at), &poly_mul(&bt, &ct))
}

fn sylvester_4_2(alpha: &[u64], beta: &[u64]) -> [[u64; 6]; 6] {
    let mut matrix = [[0; 6]; 6];
    let mut a = [0; 5];
    let mut b = [0; 3];
    for (i, &x) in alpha.iter().take(5).enumerate() {
        a[4 - i] = x;
    }
    for (i, &x) in beta.iter().take(3).enumerate() {
        b[2 - i] = x;
    }
    for shift in 0..2 {
        matrix[shift][shift..shift + 5].copy_from_slice(&a);
    }
    for shift in 0..4 {
        matrix[2 + shift][shift..shift + 3].copy_from_slice(&b);
    }
    matrix
}

fn determinant_gauss(mut matrix: [[u64; 6]; 6]) -> u64 {
    let mut determinant = 1;
    for col in 0..6 {
        let Some(pivot) = (col..6).find(|&row| matrix[row][col] != 0) else {
            return 0;
        };
        if pivot != col {
            matrix.swap(pivot, col);
            determinant = neg(determinant);
        }
        let value = matrix[col][col];
        determinant = mul(determinant, value);
        let inverse = inv(value);
        for row in col + 1..6 {
            if matrix[row][col] == 0 {
                continue;
            }
            let factor = mul(matrix[row][col], inverse);
            for j in col..6 {
                matrix[row][j] = sub(matrix[row][j], mul(factor, matrix[col][j]));
            }
        }
    }
    determinant
}

fn determinant_bareiss(mut matrix: [[u64; 6]; 6]) -> u64 {
    let mut previous = 1;
    let mut sign = 1;
    for col in 0..5 {
        let Some(pivot) = (col..6).find(|&row| matrix[row][col] != 0) else {
            return 0;
        };
        if pivot != col {
            matrix.swap(pivot, col);
            sign = neg(sign);
        }
        let pivot_value = matrix[col][col];
        let previous_inverse = inv(previous);
        for row in col + 1..6 {
            for j in col + 1..6 {
                let numerator = sub(
                    mul(matrix[row][j], pivot_value),
                    mul(matrix[row][col], matrix[col][j]),
                );
                matrix[row][j] = mul(numerator, previous_inverse);
            }
            matrix[row][col] = 0;
        }
        previous = pivot_value;
    }
    mul(sign, matrix[5][5])
}

fn s5_two_ways(x: [u64; 5]) -> (u64, u64) {
    let alpha = s4_in_last(x[0], x[1], x[2]);
    let beta = s3_coefficients(x[3], x[4]);
    let matrix = sylvester_4_2(&alpha, &beta);
    (determinant_gauss(matrix), determinant_bareiss(matrix))
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    infinity: bool,
    x: u64,
    y: u64,
}

impl Point {
    const INFINITY: Self = Self {
        infinity: true,
        x: 0,
        y: 0,
    };

    fn negated(self) -> Self {
        if self.infinity || self.y == 0 {
            self
        } else {
            Self {
                infinity: false,
                x: self.x,
                y: P - self.y,
            }
        }
    }
}

fn curve_rhs(x: u64) -> u64 {
    add(add(mul(mul(x, x), x), mul(CURVE_A, x)), CURVE_B)
}

fn point_add(left: Point, right: Point) -> Point {
    if left.infinity {
        return right;
    }
    if right.infinity {
        return left;
    }
    let slope = if left.x == right.x {
        if add(left.y, right.y) == 0 {
            return Point::INFINITY;
        }
        mul(
            add(mul(3, mul(left.x, left.x)), CURVE_A),
            inv(mul(2, left.y)),
        )
    } else {
        mul(sub(right.y, left.y), inv(sub(right.x, left.x)))
    };
    let x = sub(sub(mul(slope, slope), left.x), right.x);
    let y = sub(mul(slope, sub(left.x, x)), left.y);
    Point {
        infinity: false,
        x,
        y,
    }
}

fn scalar_mul(mut point: Point, mut scalar: u64) -> Point {
    let mut out = Point::INFINITY;
    while scalar != 0 {
        if scalar & 1 == 1 {
            out = point_add(out, point);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = point_add(point, point);
        }
    }
    out
}

#[derive(Clone)]
struct LiftedRoot {
    root_index: usize,
    signed_ids: [usize; 2],
}

#[derive(Clone)]
struct FactorBase {
    lifted: Vec<LiftedRoot>,
    signed: Vec<Point>,
}

fn factor_base(roots: &[u64]) -> FactorBase {
    let mut lifted = Vec::new();
    let mut signed = Vec::new();
    for (root_index, &x) in roots.iter().enumerate() {
        let rhs = curve_rhs(x);
        if rhs == 0 {
            let positive = Point {
                infinity: false,
                x,
                y: 0,
            };
            let id = signed.len();
            signed.push(positive);
            lifted.push(LiftedRoot {
                root_index,
                signed_ids: [id, id],
            });
            continue;
        }
        let Some(y) = sqrt(rhs) else {
            continue;
        };
        let canonical = y.min(P - y);
        let positive = Point {
            infinity: false,
            x,
            y: canonical,
        };
        let ids = [signed.len(), signed.len() + 1];
        signed.push(positive);
        signed.push(positive.negated());
        lifted.push(LiftedRoot {
            root_index,
            signed_ids: ids,
        });
    }
    FactorBase { lifted, signed }
}

fn first_planted_target(base: &FactorBase) -> Point {
    let count = base.signed.len();
    for a in 0..count {
        for b in a..count {
            for c in b..count {
                for d in c..count {
                    let sum = [a, b, c, d]
                        .into_iter()
                        .fold(Point::INFINITY, |z, i| point_add(z, base.signed[i]));
                    if !sum.infinity {
                        return sum;
                    }
                }
            }
        }
    }
    panic!("no finite planted target");
}

fn combinations_with_repetition(count: usize, arity: usize) -> Vec<Vec<usize>> {
    fn visit(
        start: usize,
        count: usize,
        arity: usize,
        current: &mut Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == arity {
            out.push(current.clone());
            return;
        }
        for value in start..count {
            current.push(value);
            visit(value, count, arity, current, out);
            current.pop();
        }
    }
    let mut out = Vec::new();
    visit(0, count, arity, &mut Vec::new(), &mut out);
    out
}

fn direct_signed_witnesses(base: &FactorBase, target: Point) -> BTreeSet<[usize; 4]> {
    let mut out = BTreeSet::new();
    for ids in combinations_with_repetition(base.signed.len(), 4) {
        let sum = ids
            .iter()
            .fold(Point::INFINITY, |z, &i| point_add(z, base.signed[i]));
        if sum == target {
            out.insert([ids[0], ids[1], ids[2], ids[3]]);
        }
    }
    out
}

fn s5_recovered_witnesses(
    roots: &[u64],
    base: &FactorBase,
    target: Point,
) -> (BTreeSet<[usize; 4]>, usize, bool) {
    let mut out = BTreeSet::new();
    let mut zero_multisets = 0;
    let mut evaluators_agree = true;
    for choices in combinations_with_repetition(base.lifted.len(), 4) {
        let xs = [
            roots[base.lifted[choices[0]].root_index],
            roots[base.lifted[choices[1]].root_index],
            roots[base.lifted[choices[2]].root_index],
            roots[base.lifted[choices[3]].root_index],
            target.x,
        ];
        let (left, right) = s5_two_ways(xs);
        evaluators_agree &= left == right;
        if left != 0 {
            continue;
        }
        zero_multisets += 1;
        for signs in 0..16 {
            let mut witness = [0; 4];
            let mut sum = Point::INFINITY;
            for position in 0..4 {
                let item = &base.lifted[choices[position]];
                let sign = (signs >> position) & 1;
                let id = item.signed_ids[sign];
                witness[position] = id;
                sum = point_add(sum, base.signed[id]);
            }
            witness.sort_unstable();
            if sum == target {
                out.insert(witness);
            }
        }
    }
    (out, zero_multisets, evaluators_agree)
}

fn encode_witnesses(witnesses: &BTreeSet<[usize; 4]>) -> String {
    if witnesses.is_empty() {
        return "-".into();
    }
    witnesses
        .iter()
        .map(|w| format!("{},{},{},{}", w[0], w[1], w[2], w[3]))
        .collect::<Vec<_>>()
        .join(";")
}

fn kernel_metrics(
    roots: &[u64],
    target_x: u64,
) -> (usize, usize, usize, usize, usize, bool, Vec<u64>) {
    let n = roots.len();
    let dimension = n * n;
    let mut pair_sum = vec![0; dimension];
    let mut pair_product = vec![0; dimension];
    for i in 0..n {
        for j in 0..n {
            let index = i * n + j;
            pair_sum[index] = add(roots[i], roots[j]);
            pair_product[index] = mul(roots[i], roots[j]);
        }
    }
    let mut kernel = vec![0; dimension * dimension];
    let mut delta_sum = vec![0; dimension * dimension];
    let mut delta_product = vec![0; dimension * dimension];
    let mut zero_count = 0;
    let mut first_zero = usize::MAX;
    let mut agree = true;
    for left in 0..dimension {
        let i = left / n;
        let j = left % n;
        for right in 0..dimension {
            let k = right / n;
            let l = right % n;
            let (value, control) = s5_two_ways([roots[i], roots[j], roots[k], roots[l], target_x]);
            agree &= value == control;
            let index = left * dimension + right;
            kernel[index] = value;
            delta_sum[index] = mul(sub(pair_sum[left], pair_sum[right]), value);
            delta_product[index] = mul(sub(pair_product[left], pair_product[right]), value);
            if value == 0 {
                zero_count += 1;
                first_zero = first_zero.min(index);
            }
        }
    }
    let kernel_rank = matrix_rank(kernel.clone(), dimension, dimension);
    (
        kernel_rank,
        matrix_rank(delta_sum, dimension, dimension),
        matrix_rank(delta_product, dimension, dimension),
        zero_count,
        first_zero,
        agree,
        kernel,
    )
}

fn encode_field_values(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn kernel_zero_multisets(kernel: &[u64], n: usize) -> BTreeSet<[usize; 4]> {
    let pair_dimension = n * n;
    let mut zero_multisets = BTreeSet::new();
    for (position, &value) in kernel.iter().enumerate() {
        if value != 0 {
            continue;
        }
        let left = position / pair_dimension;
        let right = position % pair_dimension;
        let mut roots = [left / n, left % n, right / n, right % n];
        roots.sort_unstable();
        zero_multisets.insert(roots);
    }
    zero_multisets
}

const PARTITIONS: [&str; 5] = ["4", "3+1", "2+2", "2+1+1", "1+1+1+1"];

#[derive(Default)]
struct ChartStats {
    states: usize,
    recurrence_ok: bool,
    evaluators_agree: bool,
    zero_multisets: BTreeSet<[usize; 4]>,
}

fn partition_label(multiset: &[usize]) -> &'static str {
    let mut multiplicities = Vec::new();
    let mut start = 0;
    while start < multiset.len() {
        let mut end = start + 1;
        while end < multiset.len() && multiset[end] == multiset[start] {
            end += 1;
        }
        multiplicities.push(end - start);
        start = end;
    }
    multiplicities.sort_unstable_by(|left, right| right.cmp(left));
    match multiplicities.as_slice() {
        [4] => "4",
        [3, 1] => "3+1",
        [2, 2] => "2+2",
        [2, 1, 1] => "2+1+1",
        [1, 1, 1, 1] => "1+1+1+1",
        _ => panic!("unexpected four-multiset partition"),
    }
}

fn expected_partition_states(n: usize, partition: &str) -> usize {
    match partition {
        "4" => n,
        "3+1" => n * (n - 1),
        "2+2" => binomial(n, 2),
        "2+1+1" => n * binomial(n - 1, 2),
        "1+1+1+1" => binomial(n, 4),
        _ => panic!("unknown partition"),
    }
}

fn chart_metrics(roots: &[u64], depth: usize, target_x: u64) -> Vec<ChartStats> {
    let mut charts: Vec<ChartStats> = PARTITIONS
        .iter()
        .map(|_| ChartStats {
            recurrence_ok: true,
            evaluators_agree: true,
            ..ChartStats::default()
        })
        .collect();
    for multiset in combinations_with_repetition(roots.len(), 4) {
        let label = partition_label(&multiset);
        let chart_index = PARTITIONS
            .iter()
            .position(|&partition| partition == label)
            .expect("known partition");
        let chart = &mut charts[chart_index];
        chart.states += 1;

        let mut distinct_roots = Vec::new();
        for &root_index in &multiset {
            if distinct_roots.last() != Some(&roots[root_index]) {
                distinct_roots.push(roots[root_index]);
            }
        }
        let divisor = poly_from_roots(&distinct_roots);
        chart.recurrence_ok &= divisor_recurrence(&divisor, depth).is_empty();

        let (value, control) = s5_two_ways([
            roots[multiset[0]],
            roots[multiset[1]],
            roots[multiset[2]],
            roots[multiset[3]],
            target_x,
        ]);
        chart.evaluators_agree &= value == control;
        if value == 0 {
            chart
                .zero_multisets
                .insert([multiset[0], multiset[1], multiset[2], multiset[3]]);
        }
    }
    charts
}

fn divisor_recurrence(q: &[u64], depth: usize) -> Poly {
    let mut rho = poly_rem(&[0, 1], q);
    for _ in 0..depth {
        let mut next = poly_mul(&rho, &rho);
        if next.is_empty() {
            next.push(neg(2));
        } else {
            next[0] = sub(next[0], 2);
        }
        rho = poly_rem(&next, q);
    }
    rho
}

fn divisor_counts(roots: &[u64], depth: usize) -> Vec<(usize, usize, bool)> {
    let mut out = Vec::new();
    for degree in 1..=4.min(roots.len()) {
        let mut count = 0;
        let mut ok = true;
        fn visit(
            start: usize,
            need: usize,
            roots: &[u64],
            depth: usize,
            chosen: &mut Vec<u64>,
            count: &mut usize,
            ok: &mut bool,
        ) {
            if chosen.len() == need {
                let q = poly_from_roots(chosen);
                *ok &= divisor_recurrence(&q, depth).is_empty();
                *count += 1;
                return;
            }
            let remaining = need - chosen.len();
            for i in start..=roots.len() - remaining {
                chosen.push(roots[i]);
                visit(i + 1, need, roots, depth, chosen, count, ok);
                chosen.pop();
            }
        }
        visit(
            0,
            degree,
            roots,
            depth,
            &mut Vec::new(),
            &mut count,
            &mut ok,
        );
        out.push((degree, count, ok));
    }
    out
}

fn binomial(n: usize, k: usize) -> usize {
    let k = k.min(n - k);
    (0..k).fold(1usize, |z, i| z * (n - i) / (i + 1))
}

fn target_cells(base: &FactorBase) -> [(String, Point); 3] {
    [
        ("planted".into(), first_planted_target(base)),
        ("g1".into(), GENERATOR),
        ("g17".into(), scalar_mul(GENERATOR, 17)),
    ]
}

// Scratch-only scalar contraction from PR #5287's exact field/S5 routines.
// The candidate streams first-pair norms and their outer product. Its budget
// stops after n^2 target-specific S5 calls; a full diagnostic continues only
// to verify hit/miss truth, and is never charged as a passing candidate.

#[derive(Clone, Copy)]
struct PairChart {
    left: usize,
    right: usize,
    sum: u64,
    product: u64,
}

fn checked_pair_charts(roots: &[u64], depth: usize) -> Vec<PairChart> {
    let mut pairs = Vec::with_capacity(roots.len() * (roots.len() + 1) / 2);
    for i in 0..roots.len() {
        for j in i..roots.len() {
            let sum = add(roots[i], roots[j]);
            let product = mul(roots[i], roots[j]);
            let divisor = if i == j {
                vec![neg(roots[i]), 1]
            } else {
                vec![product, neg(sum), 1]
            };
            assert!(divisor_recurrence(&divisor, depth).is_empty());
            pairs.push(PairChart {
                left: i,
                right: j,
                sum,
                product,
            });
        }
    }
    assert_eq!(pairs.len(), roots.len() * (roots.len() + 1) / 2);
    pairs
}

fn build_rational_pair_index(base: &FactorBase) -> (BTreeMap<Point, [usize; 2]>, [u64; 5]) {
    reset_ops();
    let mut index = BTreeMap::new();
    for left in 0..base.signed.len() {
        for right in left..base.signed.len() {
            let sum = point_add(base.signed[left], base.signed[right]);
            index.entry(sum).or_insert([left, right]);
        }
    }
    (index, ops())
}

fn scan_rational_pair_index(
    index: &BTreeMap<Point, [usize; 2]>,
    base: &FactorBase,
    target: Point,
) -> (bool, usize, [u64; 5]) {
    reset_ops();
    let mut examined = 0;
    let mut found = false;
    for (&first_sum, &first_ids) in index {
        examined += 1;
        let complement = point_add(target, first_sum.negated());
        if let Some(&second_ids) = index.get(&complement) {
            let replay = [first_ids[0], first_ids[1], second_ids[0], second_ids[1]]
                .into_iter()
                .fold(Point::INFINITY, |sum, id| point_add(sum, base.signed[id]));
            assert_eq!(replay, target);
            found = true;
            break;
        }
    }
    (found, examined, ops())
}

fn main() {
    assert_eq!(mul(GENERATOR.y, GENERATOR.y), curve_rhs(GENERATOR.x));
    assert_eq!(
        scalar_mul(GENERATOR, 2_305_843_010_027_268_529),
        Point::INFINITY
    );
    println!(
        r#"{{"type":"header","schema":"implicit-pair-norm-gate-v1","source_pr":5287,"source_commit":"b21448ed4059bf336af946c4254bc35624a5f942","curve":"P-61","curve_record":"icv1-fp61-tm813574577-a3b71255","prime":{P},"seed":null,"scope":"rational_four_sums_not_generic","ops_order":["add","sub","neg","mul","inv"]}}"#
    );
    for depth in 2..=5 {
        let n = 1usize << depth;
        let roots = dickson_roots(depth);
        let base = factor_base(&roots);
        let pairs = checked_pair_charts(&roots, depth);
        let pair_count = pairs.len();
        let budget = n * n;
        let setup_checksum = pairs
            .iter()
            .fold(0u64, |z, pair| add(add(z, pair.sum), pair.product));
        let (rational_pair_index, rational_pair_setup_ops) = build_rational_pair_index(&base);
        println!(
            r#"{{"type":"setup","n":{n},"full_roots":{},"lifted_x":{},"signed_leaves":{},"pair_charts":{pair_count},"pair_invariant_slots":{},"divisor_checks":{pair_count},"pair_checksum":{setup_checksum},"rational_pair_support":{},"rational_pair_setup_ops":{rational_pair_setup_ops:?}}}"#,
            roots.len(),
            base.lifted.len(),
            base.signed.len(),
            2 * pair_count,
            rational_pair_index.len()
        );
        if base.signed.is_empty() {
            println!(
                r#"{{"type":"no_query","n":{n},"reason":"no_rational_lift_in_nested_fibre"}}"#
            );
            continue;
        }
        for (label, target) in target_cells(&base) {
            reset_ops();
            let mut scalar_norm = 1u64;
            let mut zero_entries = 0usize;
            let mut row_zero_count = 0usize;
            let mut s5_calls = 0usize;
            let mut threshold = None;
            let mut budget_ops = None;
            let mut first_row_ops = None;
            // Pair invariants and their divisor checks are target-independent.
            // Target-dependent persistent field slots: row norm and outer norm.
            for (outer_index, outer) in pairs.iter().enumerate() {
                let mut row_norm = 1u64;
                let mut row_has_zero = false;
                for (inner_index, inner) in pairs.iter().enumerate() {
                    let xs = [
                        roots[inner.left],
                        roots[inner.right],
                        roots[outer.left],
                        roots[outer.right],
                        target.x,
                    ];
                    let (value, independent_formula) = s5_two_ways(xs);
                    assert_eq!(value, independent_formula, "S5 determinant disagreement");
                    row_norm = mul(row_norm, value);
                    if value == 0 {
                        zero_entries += 1;
                        row_has_zero = true;
                    }
                    s5_calls += 1;
                    if threshold.is_none() && s5_calls >= budget {
                        threshold = Some((outer_index, inner_index));
                        budget_ops = Some(ops());
                    }
                }
                if row_has_zero {
                    row_zero_count += 1;
                }
                assert_eq!(row_norm == 0, row_has_zero);
                scalar_norm = mul(scalar_norm, row_norm);
                if outer_index == 0 {
                    first_row_ops = Some(ops());
                }
            }
            assert_eq!(s5_calls, pair_count * pair_count);
            assert_eq!(scalar_norm == 0, zero_entries != 0);
            let full_ops = ops();
            let direct = direct_signed_witnesses(&base, target);
            let (recovered, lifted_zero_multisets, evaluators_agree) =
                s5_recovered_witnesses(&roots, &base, target);
            assert!(evaluators_agree);
            assert_eq!(direct, recovered, "rational sign replay mismatch");
            let (direct_pair_found, direct_pair_examined, direct_pair_online_ops) =
                scan_rational_pair_index(&rational_pair_index, &base, target);
            assert_eq!(direct_pair_found, !direct.is_empty());
            if label != "planted" {
                assert!(direct.is_empty(), "frozen miss ceased being empty");
            }
            if label == "planted" {
                assert!(!direct.is_empty());
            }
            let (stop_outer, stop_inner) = threshold.expect("candidate budget not reached");
            println!(
                r#"{{"type":"cell","n":{n},"target":"{label}","target_x":{},"candidate":"STOP_N2","budget_s5_calls":{budget},"stop_outer_pair":{stop_outer},"stop_inner_pair":{stop_inner},"complete_rows_before_stop":{stop_outer},"persistent_norm_slots":2,"first_row_ops":{:?},"budget_ops":{:?},"diagnostic_full_ops":{full_ops:?},"diagnostic_s5_calls":{s5_calls},"diagnostic_norm_mults":{},"diagnostic_outer_mults":{pair_count},"diagnostic_algebraic_zero_entries":{zero_entries},"diagnostic_zero_rows":{row_zero_count},"diagnostic_norm_zero":{},"lifted_zero_multisets":{lifted_zero_multisets},"signed_witnesses":{},"rational_replay_ok":true,"direct_pair_found":{direct_pair_found},"direct_pair_examined":{direct_pair_examined},"direct_pair_online_ops":{direct_pair_online_ops:?}}}"#,
                target.x,
                first_row_ops.unwrap(),
                budget_ops.unwrap(),
                s5_calls,
                scalar_norm == 0,
                direct.len()
            );
        }
    }
}
