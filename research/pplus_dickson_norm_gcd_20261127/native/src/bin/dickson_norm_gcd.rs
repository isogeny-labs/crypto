use std::collections::{BTreeMap, BTreeSet};

const P: u64 = 2_147_483_647;
const CURVE_A: u64 = 2_147_483_644;
const CURVE_B: u64 = 21;
const GROUP_ORDER: u64 = 2_147_487_637;
const GENERATOR: Point = Point {
    infinity: false,
    x: 1_391_443_431,
    y: 177_979_044,
};
const MAX_DEPTH: usize = 6;
const KARATSUBA_THRESHOLD: usize = 4;

const SEALED_SUPPORT: [(usize, usize, &str); MAX_DEPTH] = [
    (
        2,
        3,
        "14fe07ba12f8aed983d023adbb9e5c9570e8ad3dcb8618e174b6f47d2c224cc1",
    ),
    (
        4,
        9,
        "be147b1a4f2593925cc5f4228495eef0c81319914e6bdfa38b2514ccaed60427",
    ),
    (
        8,
        33,
        "b584bde8931ae3db613a74455de12620b022e73f4b3f91668d6151b1ea14ac1c",
    ),
    (
        16,
        129,
        "57205f062368d4877c3bc980ccf22c2958efaa9149992e38ccbbd6360ab5a4c0",
    ),
    (
        30,
        451,
        "42135e2a95d7b28d5902cfff2b96d92b7c88efac17d9693843f6944c29eb3132",
    ),
    (
        68,
        2_313,
        "ff67c668a68bcc407531add6028b55f706fc9a027991f1f40a1af2dcd5b0adcb",
    ),
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct FpOps {
    adds: u64,
    subs: u64,
    muls: u64,
    squares: u64,
    inversions: u64,
}

fn fadd(a: u64, b: u64, ops: &mut FpOps) -> u64 {
    ops.adds += 1;
    let value = a + b;
    if value >= P {
        value - P
    } else {
        value
    }
}

fn fsub(a: u64, b: u64, ops: &mut FpOps) -> u64 {
    ops.subs += 1;
    if a >= b {
        a - b
    } else {
        P - (b - a)
    }
}

fn fneg(value: u64, ops: &mut FpOps) -> u64 {
    fsub(0, value, ops)
}

fn fmul(a: u64, b: u64, ops: &mut FpOps) -> u64 {
    ops.muls += 1;
    (a * b) % P
}

fn fsquare(value: u64, ops: &mut FpOps) -> u64 {
    ops.squares += 1;
    (value * value) % P
}

fn fpow(mut base: u64, mut exponent: u64, ops: &mut FpOps) -> u64 {
    let mut accumulator = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            accumulator = fmul(accumulator, base, ops);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = fsquare(base, ops);
        }
    }
    accumulator
}

fn finv(value: u64, ops: &mut FpOps) -> u64 {
    assert_ne!(value, 0, "attempted inversion of zero");
    ops.inversions += 1;
    fpow(value, P - 2, ops)
}

fn fsqrt(value: u64, ops: &mut FpOps) -> Option<u64> {
    let root = fpow(value, (P + 1) / 4, ops);
    (fsquare(root, ops) == value).then_some(root)
}

fn curve_rhs(x: u64, ops: &mut FpOps) -> u64 {
    let x2 = fsquare(x, ops);
    let x3 = fmul(x2, x, ops);
    let ax = fmul(CURVE_A, x, ops);
    fadd(fadd(x3, ax, ops), CURVE_B, ops)
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    fn negate(self) -> Self {
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

fn point_add(left: Point, right: Point, ops: &mut FpOps) -> Point {
    if left.infinity {
        return right;
    }
    if right.infinity {
        return left;
    }

    let lambda = if left.x == right.x {
        if fadd(left.y, right.y, ops) == 0 {
            return Point::INFINITY;
        }
        assert_eq!(left.y, right.y, "equal x must mean equal or inverse points");
        let x2 = fsquare(left.x, ops);
        let two_x2 = fadd(x2, x2, ops);
        let three_x2 = fadd(two_x2, x2, ops);
        let numerator = fadd(three_x2, CURVE_A, ops);
        let denominator = fadd(left.y, left.y, ops);
        fmul(numerator, finv(denominator, ops), ops)
    } else {
        let numerator = fsub(right.y, left.y, ops);
        let denominator = fsub(right.x, left.x, ops);
        fmul(numerator, finv(denominator, ops), ops)
    };

    let x3 = fsub(fsub(fsquare(lambda, ops), left.x, ops), right.x, ops);
    let y3 = fsub(fmul(lambda, fsub(left.x, x3, ops), ops), left.y, ops);
    Point {
        infinity: false,
        x: x3,
        y: y3,
    }
}

fn scalar_mul(mut point: Point, mut scalar: u64, ops: &mut FpOps) -> Point {
    let mut result = Point::INFINITY;
    while scalar != 0 {
        if scalar & 1 == 1 {
            result = point_add(result, point, ops);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = point_add(point, point, ops);
        }
    }
    result
}

fn on_curve(point: Point) -> bool {
    if point.infinity {
        return true;
    }
    let mut ops = FpOps::default();
    fsquare(point.y, &mut ops) == curve_rhs(point.x, &mut ops)
}

#[derive(Clone, Copy, Debug)]
struct Leaf {
    index: u16,
    point: Point,
}

#[derive(Clone, Debug)]
struct FactorBase {
    roots: Vec<u64>,
    leaves: Vec<Leaf>,
    leaves_by_x: BTreeMap<u64, Vec<Leaf>>,
    point_indices: BTreeMap<Point, u16>,
    field_ops: FpOps,
}

fn dickson_roots(depth: usize, ops: &mut FpOps) -> Vec<u64> {
    let mut states = vec![0_u64];
    for _ in 0..depth {
        let mut next = Vec::with_capacity(states.len() * 2);
        for value in states {
            let radicand = fadd(value, 2, ops);
            let root = fsqrt(radicand, ops).expect("Dickson tower did not split");
            let canonical = root.min(P - root);
            next.push(canonical);
            next.push(if canonical == 0 { 0 } else { P - canonical });
        }
        states = next;
    }

    let distinct: BTreeSet<_> = states.iter().copied().collect();
    assert_eq!(distinct.len(), 1_usize << depth, "repeated Dickson root");
    for &root in &states {
        let mut value = root;
        for _ in 0..depth {
            value = fsub(fsquare(value, ops), 2, ops);
        }
        assert_eq!(value, 0, "Dickson root failed tower replay");
    }
    states
}

fn factor_base(depth: usize) -> FactorBase {
    let mut field_ops = FpOps::default();
    let roots = dickson_roots(depth, &mut field_ops);
    let mut leaves = Vec::new();
    let mut leaves_by_x = BTreeMap::<u64, Vec<Leaf>>::new();
    let mut point_indices = BTreeMap::new();

    for &x in &roots {
        let rhs = curve_rhs(x, &mut field_ops);
        if rhs == 0 {
            continue;
        }
        let Some(y) = fsqrt(rhs, &mut field_ops) else {
            continue;
        };
        let canonical = y.min(P - y);
        for ordinate in [canonical, P - canonical] {
            let leaf = Leaf {
                index: leaves.len() as u16,
                point: Point {
                    infinity: false,
                    x,
                    y: ordinate,
                },
            };
            assert!(on_curve(leaf.point));
            point_indices.insert(leaf.point, leaf.index);
            leaves_by_x.entry(x).or_default().push(leaf);
            leaves.push(leaf);
        }
    }

    FactorBase {
        roots,
        leaves,
        leaves_by_x,
        point_indices,
        field_ops,
    }
}

type Poly = Vec<u64>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PolyStats {
    multiply_calls: u64,
    square_calls: u64,
    karatsuba_nodes: u64,
    schoolbook_nodes: u64,
    modular_reductions: u64,
    newton_steps: u64,
    remainder_nodes: u64,
    peak_degree: usize,
    peak_slots: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Work {
    fp: FpOps,
    poly: PolyStats,
}

impl Work {
    fn observe(&mut self, poly: &[u64]) {
        self.poly.peak_slots = self.poly.peak_slots.max(poly.len());
        self.poly.peak_degree = self.poly.peak_degree.max(poly.len().saturating_sub(1));
    }
}

fn trim(mut poly: Poly) -> Poly {
    while poly.last() == Some(&0) {
        poly.pop();
    }
    poly
}

fn poly_constant(value: u64) -> Poly {
    if value == 0 {
        Vec::new()
    } else {
        vec![value]
    }
}

fn poly_add(left: &[u64], right: &[u64], work: &mut Work) -> Poly {
    let length = left.len().max(right.len());
    let mut output = vec![0; length];
    for (index, slot) in output.iter_mut().enumerate() {
        *slot = match (left.get(index), right.get(index)) {
            (Some(&a), Some(&b)) => fadd(a, b, &mut work.fp),
            (Some(&a), None) => a,
            (None, Some(&b)) => b,
            (None, None) => unreachable!(),
        };
    }
    let output = trim(output);
    work.observe(&output);
    output
}

fn poly_sub(left: &[u64], right: &[u64], work: &mut Work) -> Poly {
    let length = left.len().max(right.len());
    let mut output = vec![0; length];
    for (index, slot) in output.iter_mut().enumerate() {
        *slot = match (left.get(index), right.get(index)) {
            (Some(&a), Some(&b)) => fsub(a, b, &mut work.fp),
            (Some(&a), None) => a,
            (None, Some(&b)) => fneg(b, &mut work.fp),
            (None, None) => unreachable!(),
        };
    }
    let output = trim(output);
    work.observe(&output);
    output
}

fn poly_scale(poly: &[u64], scalar: u64, work: &mut Work) -> Poly {
    if scalar == 0 || poly.is_empty() {
        return Vec::new();
    }
    if scalar == 1 {
        return poly.to_vec();
    }
    let output = trim(
        poly.iter()
            .map(|&value| fmul(value, scalar, &mut work.fp))
            .collect(),
    );
    work.observe(&output);
    output
}

fn poly_schoolbook_mul(left: &[u64], right: &[u64], work: &mut Work) -> Poly {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    work.poly.schoolbook_nodes += 1;
    let mut output = vec![0; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            let product = fmul(a, b, &mut work.fp);
            output[i + j] = fadd(output[i + j], product, &mut work.fp);
        }
    }
    let output = trim(output);
    work.observe(&output);
    output
}

fn poly_schoolbook_square(poly: &[u64], work: &mut Work) -> Poly {
    if poly.is_empty() {
        return Vec::new();
    }
    work.poly.schoolbook_nodes += 1;
    let mut output = vec![0; 2 * poly.len() - 1];
    for (i, &a) in poly.iter().enumerate() {
        let diagonal = fsquare(a, &mut work.fp);
        output[2 * i] = fadd(output[2 * i], diagonal, &mut work.fp);
        for (j, &b) in poly.iter().enumerate().skip(i + 1) {
            let cross = fmul(a, b, &mut work.fp);
            let twice = fadd(cross, cross, &mut work.fp);
            output[i + j] = fadd(output[i + j], twice, &mut work.fp);
        }
    }
    let output = trim(output);
    work.observe(&output);
    output
}

fn add_slices(left: &[u64], right: &[u64], work: &mut Work) -> Poly {
    poly_add(left, right, work)
}

fn add_shifted(output: &mut Poly, value: &[u64], shift: usize, work: &mut Work) {
    if value.is_empty() {
        return;
    }
    if output.len() < shift + value.len() {
        output.resize(shift + value.len(), 0);
    }
    for (index, &coefficient) in value.iter().enumerate() {
        output[shift + index] = fadd(output[shift + index], coefficient, &mut work.fp);
    }
}

fn poly_mul_recursive(left: &[u64], right: &[u64], work: &mut Work) -> Poly {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    if left.len().min(right.len()) <= KARATSUBA_THRESHOLD {
        return poly_schoolbook_mul(left, right, work);
    }

    work.poly.karatsuba_nodes += 1;
    let split = left.len().max(right.len()).div_ceil(2);
    let left_low = &left[..left.len().min(split)];
    let left_high = left.get(split..).unwrap_or(&[]);
    let right_low = &right[..right.len().min(split)];
    let right_high = right.get(split..).unwrap_or(&[]);

    let z0 = poly_mul_recursive(left_low, right_low, work);
    let z2 = poly_mul_recursive(left_high, right_high, work);
    let left_sum = add_slices(left_low, left_high, work);
    let right_sum = add_slices(right_low, right_high, work);
    let middle = poly_mul_recursive(&left_sum, &right_sum, work);
    let middle = poly_sub(&poly_sub(&middle, &z0, work), &z2, work);

    let mut output = Vec::new();
    add_shifted(&mut output, &z0, 0, work);
    add_shifted(&mut output, &middle, split, work);
    add_shifted(&mut output, &z2, 2 * split, work);
    let output = trim(output);
    work.observe(&output);
    output
}

fn poly_square_recursive(poly: &[u64], work: &mut Work) -> Poly {
    if poly.is_empty() {
        return Vec::new();
    }
    if poly.len() <= KARATSUBA_THRESHOLD {
        return poly_schoolbook_square(poly, work);
    }

    work.poly.karatsuba_nodes += 1;
    let split = poly.len().div_ceil(2);
    let low = &poly[..split];
    let high = &poly[split..];
    let z0 = poly_square_recursive(low, work);
    let z2 = poly_square_recursive(high, work);
    let sum = add_slices(low, high, work);
    let middle = poly_square_recursive(&sum, work);
    let middle = poly_sub(&poly_sub(&middle, &z0, work), &z2, work);

    let mut output = Vec::new();
    add_shifted(&mut output, &z0, 0, work);
    add_shifted(&mut output, &middle, split, work);
    add_shifted(&mut output, &z2, 2 * split, work);
    let output = trim(output);
    work.observe(&output);
    output
}

fn poly_mul(left: &[u64], right: &[u64], work: &mut Work) -> Poly {
    work.poly.multiply_calls += 1;
    poly_mul_recursive(left, right, work)
}

fn poly_square(poly: &[u64], work: &mut Work) -> Poly {
    work.poly.square_calls += 1;
    poly_square_recursive(poly, work)
}

fn poly_mul_trunc(left: &[u64], right: &[u64], length: usize, work: &mut Work) -> Poly {
    let mut output = poly_mul(left, right, work);
    output.truncate(length);
    output.resize(length, 0);
    work.observe(&output);
    output
}

fn poly_div_rem_long(dividend: &[u64], divisor: &[u64], work: &mut Work) -> (Poly, Poly) {
    assert!(!divisor.is_empty(), "polynomial division by zero");
    if dividend.len() < divisor.len() {
        return (Vec::new(), dividend.to_vec());
    }

    let quotient_len = dividend.len() - divisor.len() + 1;
    let mut quotient = vec![0; quotient_len];
    let mut remainder = dividend.to_vec();
    let divisor_lead = *divisor.last().unwrap();
    let divisor_lead_inv = if divisor_lead == 1 {
        1
    } else {
        finv(divisor_lead, &mut work.fp)
    };

    for shift in (0..quotient_len).rev() {
        let coefficient = fmul(
            remainder[shift + divisor.len() - 1],
            divisor_lead_inv,
            &mut work.fp,
        );
        quotient[shift] = coefficient;
        if coefficient == 0 {
            continue;
        }
        for (index, &divisor_coefficient) in divisor.iter().enumerate() {
            let product = fmul(coefficient, divisor_coefficient, &mut work.fp);
            remainder[shift + index] = fsub(remainder[shift + index], product, &mut work.fp);
        }
    }
    remainder.truncate(divisor.len() - 1);
    let quotient = trim(quotient);
    let remainder = trim(remainder);
    work.observe(&quotient);
    work.observe(&remainder);
    (quotient, remainder)
}

fn poly_inverse_series(series: &[u64], length: usize, work: &mut Work) -> Poly {
    assert!(length > 0);
    assert!(!series.is_empty() && series[0] != 0);
    let mut inverse = vec![if series[0] == 1 {
        1
    } else {
        finv(series[0], &mut work.fp)
    }];
    while inverse.len() < length {
        let next_length = (2 * inverse.len()).min(length);
        let source_length = series.len().min(next_length);
        let product = poly_mul_trunc(&series[..source_length], &inverse, next_length, work);
        let mut correction = vec![0; next_length];
        correction[0] = fsub(2, product[0], &mut work.fp);
        for index in 1..next_length {
            correction[index] = fneg(product[index], &mut work.fp);
        }
        inverse = poly_mul_trunc(&inverse, &correction, next_length, work);
        work.poly.newton_steps += 1;
    }
    inverse.truncate(length);
    inverse
}

fn poly_div_rem_fast(dividend: &[u64], divisor: &[u64], work: &mut Work) -> (Poly, Poly) {
    assert!(!divisor.is_empty(), "polynomial division by zero");
    if dividend.len() < divisor.len() {
        return (Vec::new(), dividend.to_vec());
    }
    let quotient_len = dividend.len() - divisor.len() + 1;
    if quotient_len <= KARATSUBA_THRESHOLD || divisor.len() <= KARATSUBA_THRESHOLD {
        return poly_div_rem_long(dividend, divisor, work);
    }

    let mut reversed_dividend = Vec::with_capacity(quotient_len);
    for index in 0..quotient_len {
        reversed_dividend.push(dividend[dividend.len() - 1 - index]);
    }
    let mut reversed_divisor = Vec::with_capacity(quotient_len.min(divisor.len()));
    for index in 0..quotient_len.min(divisor.len()) {
        reversed_divisor.push(divisor[divisor.len() - 1 - index]);
    }
    let inverse = poly_inverse_series(&reversed_divisor, quotient_len, work);
    let mut reversed_quotient = poly_mul_trunc(&reversed_dividend, &inverse, quotient_len, work);
    reversed_quotient.reverse();
    let quotient = trim(reversed_quotient);
    let product = poly_mul(divisor, &quotient, work);
    let mut remainder = poly_sub(dividend, &product, work);
    if remainder.len() >= divisor.len() {
        let (_, repaired) = poly_div_rem_long(&remainder, divisor, work);
        remainder = repaired;
    }
    assert!(remainder.len() < divisor.len());
    (quotient, remainder)
}

fn poly_rem(dividend: &[u64], divisor: &[u64], work: &mut Work) -> Poly {
    work.poly.modular_reductions += 1;
    poly_div_rem_fast(dividend, divisor, work).1
}

fn poly_mod_mul(left: &[u64], right: &[u64], modulus: &[u64], work: &mut Work) -> Poly {
    let product = poly_mul(left, right, work);
    if product.len() < modulus.len() {
        product
    } else {
        poly_rem(&product, modulus, work)
    }
}

fn poly_mod_square(poly: &[u64], modulus: &[u64], work: &mut Work) -> Poly {
    let square = poly_square(poly, work);
    if square.len() < modulus.len() {
        square
    } else {
        poly_rem(&square, modulus, work)
    }
}

fn poly_eval(poly: &[u64], value: u64, ops: &mut FpOps) -> u64 {
    let mut result = 0;
    for &coefficient in poly.iter().rev() {
        result = fadd(fmul(result, value, ops), coefficient, ops);
    }
    result
}

fn dickson_polynomial(depth: usize, work: &mut Work) -> Poly {
    let mut polynomial = vec![0, 1];
    for _ in 0..depth {
        polynomial = poly_square(&polynomial, work);
        if polynomial.is_empty() {
            polynomial.push(fneg(2, &mut work.fp));
        } else {
            polynomial[0] = fsub(polynomial[0], 2, &mut work.fp);
        }
        polynomial = trim(polynomial);
    }
    polynomial
}

#[derive(Clone, Debug)]
struct ProductNode {
    start: usize,
    end: usize,
    polynomial: Poly,
    left: Option<Box<ProductNode>>,
    right: Option<Box<ProductNode>>,
}

fn build_product_tree_range(
    roots: &[u64],
    start: usize,
    end: usize,
    work: &mut Work,
) -> ProductNode {
    assert!(start < end);
    if end - start == 1 {
        let polynomial = vec![fneg(roots[start], &mut work.fp), 1];
        work.observe(&polynomial);
        return ProductNode {
            start,
            end,
            polynomial,
            left: None,
            right: None,
        };
    }
    let middle = (start + end) / 2;
    let left = build_product_tree_range(roots, start, middle, work);
    let right = build_product_tree_range(roots, middle, end, work);
    let polynomial = poly_mul(&left.polynomial, &right.polynomial, work);
    ProductNode {
        start,
        end,
        polynomial,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}

fn build_product_tree(roots: &[u64], work: &mut Work) -> ProductNode {
    build_product_tree_range(roots, 0, roots.len(), work)
}

fn multipoint_recurse(poly: &[u64], node: &ProductNode, work: &mut Work, values: &mut [u64]) {
    work.poly.remainder_nodes += 1;
    let remainder = if poly.len() >= node.polynomial.len() {
        poly_rem(poly, &node.polynomial, work)
    } else {
        poly.to_vec()
    };
    if node.end - node.start == 1 {
        values[node.start] = remainder.first().copied().unwrap_or(0);
        return;
    }
    multipoint_recurse(&remainder, node.left.as_ref().unwrap(), work, values);
    multipoint_recurse(&remainder, node.right.as_ref().unwrap(), work, values);
}

fn multipoint_values(poly: &[u64], tree: &ProductNode, work: &mut Work) -> Vec<u64> {
    let mut values = vec![0; tree.end - tree.start];
    multipoint_recurse(poly, tree, work, &mut values);
    values
}

fn product_from_selected_roots(roots: &[u64], selected: &[usize], work: &mut Work) -> Poly {
    if selected.is_empty() {
        return vec![1];
    }
    let mut level: Vec<Poly> = selected
        .iter()
        .map(|&index| vec![fneg(roots[index], &mut work.fp), 1])
        .collect();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut index = 0;
        while index < level.len() {
            if index + 1 == level.len() {
                next.push(level[index].clone());
            } else {
                next.push(poly_mul(&level[index], &level[index + 1], work));
            }
            index += 2;
        }
        level = next;
    }
    level.pop().unwrap()
}

#[derive(Clone, Debug)]
struct Algebra {
    selector: Poly,
    product_tree: ProductNode,
    setup_work: Work,
}

fn build_algebra(roots: &[u64], depth: usize) -> Algebra {
    let mut setup_work = Work::default();
    let selector = dickson_polynomial(depth, &mut setup_work);
    let product_tree = build_product_tree(roots, &mut setup_work);
    assert_eq!(
        selector, product_tree.polynomial,
        "root product != Dickson selector"
    );
    for &root in roots {
        let mut check_ops = FpOps::default();
        assert_eq!(poly_eval(&selector, root, &mut check_ops), 0);
    }
    Algebra {
        selector,
        product_tree,
        setup_work,
    }
}

#[derive(Clone, Copy, Debug)]
struct SupportEntry {
    point: Point,
    multiplicity: u32,
}

fn support_entries(base: &FactorBase, ops: &mut FpOps) -> Vec<SupportEntry> {
    let mut entries = BTreeMap::<Point, u32>::new();
    for left in 0..base.leaves.len() {
        for right in left..base.leaves.len() {
            let point = point_add(base.leaves[left].point, base.leaves[right].point, ops);
            *entries.entry(point).or_default() += 1;
        }
    }
    entries
        .into_iter()
        .map(|(point, multiplicity)| SupportEntry {
            point,
            multiplicity,
        })
        .collect()
}

fn support_digest(entries: &[SupportEntry]) -> String {
    let mut bytes = Vec::with_capacity(entries.len() * 32);
    for entry in entries {
        bytes.push(u8::from(entry.point.infinity));
        bytes.extend_from_slice(&entry.point.x.to_be_bytes());
        bytes.extend_from_slice(&entry.point.y.to_be_bytes());
        bytes.extend_from_slice(&entry.multiplicity.to_be_bytes());
    }
    hex(&sha256(&bytes))
}

#[derive(Clone, Debug, Default)]
struct DirectOutcome {
    found: bool,
    witness: Option<[u16; 2]>,
    work: FpOps,
    comparisons: u64,
    denominators: usize,
    batch_inversions: u64,
    denominator_zero_leaves: usize,
}

fn canonical_pair(left: u16, right: u16) -> [u16; 2] {
    if left <= right {
        [left, right]
    } else {
        [right, left]
    }
}

fn update_witness(best: &mut Option<[u16; 2]>, candidate: [u16; 2]) {
    if best.is_none_or(|current| candidate < current) {
        *best = Some(candidate);
    }
}

fn direct_query(base: &FactorBase, target: Point) -> DirectOutcome {
    let mut outcome = DirectOutcome::default();
    if target.infinity {
        for leaf in &base.leaves {
            outcome.comparisons += 1;
            if let Some(&other) = base.point_indices.get(&leaf.point.negate()) {
                update_witness(&mut outcome.witness, canonical_pair(leaf.index, other));
            }
        }
        outcome.found = outcome.witness.is_some();
        return outcome;
    }

    let regular_x: Vec<u64> = base
        .leaves_by_x
        .keys()
        .copied()
        .filter(|&x| x != target.x)
        .collect();
    let denominators: Vec<u64> = regular_x
        .iter()
        .map(|&x| fsub(target.x, x, &mut outcome.work))
        .collect();
    outcome.denominators = denominators.len();

    let mut inverses = vec![0; denominators.len()];
    if !denominators.is_empty() {
        let mut prefixes = Vec::with_capacity(denominators.len());
        let mut product = 1;
        for &denominator in &denominators {
            prefixes.push(product);
            product = fmul(product, denominator, &mut outcome.work);
        }
        let mut inverse_product = finv(product, &mut outcome.work);
        outcome.batch_inversions = 1;
        for index in (0..denominators.len()).rev() {
            inverses[index] = fmul(inverse_product, prefixes[index], &mut outcome.work);
            inverse_product = fmul(inverse_product, denominators[index], &mut outcome.work);
        }
    }

    for (&x, &inverse_denominator) in regular_x.iter().zip(&inverses) {
        for leaf in &base.leaves_by_x[&x] {
            let numerator = fadd(target.y, leaf.point.y, &mut outcome.work);
            let lambda = fmul(numerator, inverse_denominator, &mut outcome.work);
            let qx = fsub(
                fsub(
                    fsquare(lambda, &mut outcome.work),
                    target.x,
                    &mut outcome.work,
                ),
                x,
                &mut outcome.work,
            );
            let qy = fsub(
                fmul(
                    lambda,
                    fsub(target.x, qx, &mut outcome.work),
                    &mut outcome.work,
                ),
                target.y,
                &mut outcome.work,
            );
            let complement = Point {
                infinity: false,
                x: qx,
                y: qy,
            };
            outcome.comparisons += 1;
            if let Some(&other) = base.point_indices.get(&complement) {
                update_witness(&mut outcome.witness, canonical_pair(leaf.index, other));
            }
        }
    }

    if let Some(leaves) = base.leaves_by_x.get(&target.x) {
        outcome.denominator_zero_leaves = leaves.len();
        for leaf in leaves {
            let complement = point_add(target, leaf.point.negate(), &mut outcome.work);
            outcome.comparisons += 1;
            if let Some(&other) = base.point_indices.get(&complement) {
                update_witness(&mut outcome.witness, canonical_pair(leaf.index, other));
            }
        }
    }
    outcome.found = outcome.witness.is_some();
    outcome
}

#[derive(Clone, Debug)]
struct NormOutcome {
    found: bool,
    witness: Option<[u16; 2]>,
    a_poly: Poly,
    b_poly: Poly,
    norm_poly: Poly,
    gcd_poly: Poly,
    norm_values: Vec<u64>,
    zero_indices: Vec<usize>,
    work: Work,
    recovery_ops: FpOps,
    algebraic_roots: usize,
    accepted_roots: usize,
    nonlifting_roots: usize,
    translated_nonlifting_roots: usize,
    denominator_zero_roots: usize,
    b_zero_roots: usize,
    ambiguous_roots: usize,
}

impl NormOutcome {
    fn infinity(base: &FactorBase) -> Self {
        let witness = base.leaves.iter().find_map(|leaf| {
            base.point_indices
                .get(&leaf.point.negate())
                .map(|&other| canonical_pair(leaf.index, other))
        });
        Self {
            found: witness.is_some(),
            witness,
            a_poly: Vec::new(),
            b_poly: Vec::new(),
            norm_poly: Vec::new(),
            gcd_poly: vec![1],
            norm_values: Vec::new(),
            zero_indices: Vec::new(),
            work: Work::default(),
            recovery_ops: FpOps::default(),
            algebraic_roots: 0,
            accepted_roots: usize::from(witness.is_some()),
            nonlifting_roots: 0,
            translated_nonlifting_roots: 0,
            denominator_zero_roots: 0,
            b_zero_roots: 0,
            ambiguous_roots: 0,
        }
    }
}

fn curve_polynomial() -> Poly {
    vec![CURVE_B, CURVE_A, 0, 1]
}

fn quotient_reduce(poly: Poly, modulus: &[u64], work: &mut Work) -> Poly {
    if poly.len() < modulus.len() {
        poly
    } else {
        poly_rem(&poly, modulus, work)
    }
}

fn build_query_polynomials(
    depth: usize,
    target: Point,
    algebra: &Algebra,
    work: &mut Work,
) -> (Poly, Poly, Poly) {
    assert!(!target.infinity);
    let f = curve_polynomial();
    let u_minus_x = vec![target.x, P - 1];
    let d = quotient_reduce(poly_square(&u_minus_x, work), &algebra.selector, work);
    let u_plus_x = vec![target.x, 1];
    let product = poly_mod_mul(&u_plus_x, &d, &algebra.selector, work);
    let v2 = fsquare(target.y, &mut work.fp);
    let c = poly_sub(&poly_add(&f, &poly_constant(v2), work), &product, work);
    let mut a_poly = quotient_reduce(c, &algebra.selector, work);
    let mut b_poly = poly_constant(fadd(target.y, target.y, &mut work.fp));
    let mut delta = d;

    for _ in 0..depth {
        let a2 = poly_mod_square(&a_poly, &algebra.selector, work);
        let b2 = poly_mod_square(&b_poly, &algebra.selector, work);
        let f_b2 = poly_mod_mul(&f, &b2, &algebra.selector, work);
        let delta2 = poly_mod_square(&delta, &algebra.selector, work);
        let twice_delta2 = poly_scale(&delta2, 2, work);
        let next_a = poly_sub(&poly_add(&a2, &f_b2, work), &twice_delta2, work);
        let ab = poly_mod_mul(&a_poly, &b_poly, &algebra.selector, work);
        let next_b = poly_scale(&ab, 2, work);
        a_poly = quotient_reduce(next_a, &algebra.selector, work);
        b_poly = quotient_reduce(next_b, &algebra.selector, work);
        delta = delta2;
    }

    let a2 = poly_mod_square(&a_poly, &algebra.selector, work);
    let b2 = poly_mod_square(&b_poly, &algebra.selector, work);
    let f_b2 = poly_mod_mul(&f, &b2, &algebra.selector, work);
    let norm = quotient_reduce(poly_sub(&a2, &f_b2, work), &algebra.selector, work);
    (a_poly, b_poly, norm)
}

fn orientation_candidates(
    leaves: &[Leaf],
    a_value: u64,
    b_value: u64,
    ops: &mut FpOps,
) -> Vec<Leaf> {
    if b_value == 0 {
        return if a_value == 0 {
            leaves.to_vec()
        } else {
            Vec::new()
        };
    }
    let y = fmul(fneg(a_value, ops), finv(b_value, ops), ops);
    leaves
        .iter()
        .copied()
        .filter(|leaf| leaf.point.y == y)
        .collect()
}

fn norm_query(base: &FactorBase, depth: usize, algebra: &Algebra, target: Point) -> NormOutcome {
    if target.infinity {
        return NormOutcome::infinity(base);
    }

    let mut work = Work::default();
    let (a_poly, b_poly, norm_poly) = build_query_polynomials(depth, target, algebra, &mut work);
    let a_values = multipoint_values(&a_poly, &algebra.product_tree, &mut work);
    let b_values = multipoint_values(&b_poly, &algebra.product_tree, &mut work);
    let norm_values = multipoint_values(&norm_poly, &algebra.product_tree, &mut work);
    let zero_indices: Vec<usize> = norm_values
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| (value == 0).then_some(index))
        .collect();
    let gcd_poly = product_from_selected_roots(&base.roots, &zero_indices, &mut work);

    let mut recovery_ops = FpOps::default();
    let mut witness = None;
    let mut accepted_root_set = BTreeSet::new();
    let mut nonlifting_roots = 0;
    let mut translated_nonlifting_roots = 0;
    let mut denominator_zero_roots = 0;
    let mut b_zero_roots = 0;
    let mut ambiguous_roots = 0;

    for &root_index in &zero_indices {
        let x = base.roots[root_index];
        let Some(leaves) = base.leaves_by_x.get(&x) else {
            nonlifting_roots += 1;
            continue;
        };

        let candidates = if x == target.x {
            denominator_zero_roots += 1;
            leaves.to_vec()
        } else {
            if b_values[root_index] == 0 {
                b_zero_roots += 1;
                if a_values[root_index] == 0 {
                    ambiguous_roots += 1;
                }
            }
            orientation_candidates(
                leaves,
                a_values[root_index],
                b_values[root_index],
                &mut recovery_ops,
            )
        };

        let mut accepted_here = false;
        for leaf in candidates {
            let complement = point_add(target, leaf.point.negate(), &mut recovery_ops);
            if let Some(&other_index) = base.point_indices.get(&complement) {
                accepted_here = true;
                update_witness(&mut witness, canonical_pair(leaf.index, other_index));
            }
        }
        if accepted_here {
            accepted_root_set.insert(root_index);
        } else {
            translated_nonlifting_roots += 1;
        }
    }

    NormOutcome {
        found: witness.is_some(),
        witness,
        a_poly,
        b_poly,
        norm_poly,
        gcd_poly,
        norm_values,
        algebraic_roots: zero_indices.len(),
        accepted_roots: accepted_root_set.len(),
        zero_indices,
        work,
        recovery_ops,
        nonlifting_roots,
        translated_nonlifting_roots,
        denominator_zero_roots,
        b_zero_roots,
        ambiguous_roots,
    }
}

fn replay_pair(base: &FactorBase, witness: [u16; 2], ops: &mut FpOps) -> Point {
    point_add(
        base.leaves[witness[0] as usize].point,
        base.leaves[witness[1] as usize].point,
        ops,
    )
}

fn planted_pair(base: &FactorBase) -> (Point, [u16; 2]) {
    let mut ops = FpOps::default();
    for left in 0..base.leaves.len() {
        for right in left..base.leaves.len() {
            let witness = [left as u16, right as u16];
            let target = replay_pair(base, witness, &mut ops);
            if !target.infinity {
                return (target, witness);
            }
        }
    }
    panic!("no finite planted pair target");
}

fn fixed_targets(base: &FactorBase) -> Vec<(String, Point)> {
    let mut targets = Vec::new();
    targets.push(("planted_pair".to_owned(), planted_pair(base).0));
    for scalar in 1..=8 {
        let mut ops = FpOps::default();
        targets.push((
            format!("g{scalar}"),
            scalar_mul(GENERATOR, scalar, &mut ops),
        ));
    }
    targets.push(("first_leaf".to_owned(), base.leaves[0].point));
    targets.push(("infinity".to_owned(), Point::INFINITY));
    targets
}

fn witness_text(witness: Option<[u16; 2]>) -> String {
    witness.map_or_else(
        || "-".to_owned(),
        |value| format!("{},{}", value[0], value[1]),
    )
}

fn poly_text(poly: &[u64]) -> String {
    if poly.is_empty() {
        "-".to_owned()
    } else {
        poly.iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn indices_text(indices: &[usize]) -> String {
    if indices.is_empty() {
        "-".to_owned()
    } else {
        indices
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn print_fp_ops(prefix: &str, ops: FpOps) {
    print!(
        " {prefix}_add={} {prefix}_sub={} {prefix}_mul={} {prefix}_square={} {prefix}_inv={}",
        ops.adds, ops.subs, ops.muls, ops.squares, ops.inversions
    );
}

fn print_poly_stats(prefix: &str, stats: PolyStats) {
    print!(
        " {prefix}_mul_calls={} {prefix}_square_calls={} {prefix}_karatsuba_nodes={} {prefix}_schoolbook_nodes={} {prefix}_reductions={} {prefix}_newton_steps={} {prefix}_remainder_nodes={} {prefix}_peak_degree={} {prefix}_peak_slots={}",
        stats.multiply_calls,
        stats.square_calls,
        stats.karatsuba_nodes,
        stats.schoolbook_nodes,
        stats.modular_reductions,
        stats.newton_steps,
        stats.remainder_nodes,
        stats.peak_degree,
        stats.peak_slots
    );
}

fn run_depth(depth: usize) {
    let base = factor_base(depth);
    assert_eq!(base.leaves.len(), SEALED_SUPPORT[depth - 1].0);
    let mut support_ops = FpOps::default();
    let support = support_entries(&base, &mut support_ops);
    let digest = support_digest(&support);
    assert_eq!(support.len(), SEALED_SUPPORT[depth - 1].1);
    assert_eq!(digest, SEALED_SUPPORT[depth - 1].2);
    let algebra = build_algebra(&base.roots, depth);

    print!(
        "DEPTH depth={depth} full_roots={} lifting_abscissae={} signed_leaves={} support={} support_sha256={} selector_degree={}",
        base.roots.len(),
        base.leaves_by_x.len(),
        base.leaves.len(),
        support.len(),
        digest,
        algebra.selector.len() - 1
    );
    print_fp_ops("root", base.field_ops);
    print_fp_ops("support", support_ops);
    print_fp_ops("algebra_setup", algebra.setup_work.fp);
    print_poly_stats("algebra_setup", algebra.setup_work.poly);
    println!(
        " roots={} selector={}",
        poly_text(&base.roots),
        poly_text(&algebra.selector)
    );

    for (label, target) in fixed_targets(&base) {
        let norm = norm_query(&base, depth, &algebra, target);
        let direct = direct_query(&base, target);
        assert_eq!(norm.found, direct.found, "query classification mismatch");
        if let Some(witness) = norm.witness {
            let mut replay_ops = FpOps::default();
            assert_eq!(replay_pair(&base, witness, &mut replay_ops), target);
        }
        if let Some(witness) = direct.witness {
            let mut replay_ops = FpOps::default();
            assert_eq!(replay_pair(&base, witness, &mut replay_ops), target);
        }

        print!(
            "QUERY depth={depth} label={label} target_inf={} target_x={} target_y={} found={} witness={} direct_found={} direct_witness={} algebraic_roots={} accepted_roots={} nonlifting_roots={} translated_nonlifting_roots={} denominator_zero_roots={} b_zero_roots={} ambiguous_roots={} norm_degree={} gcd_degree={} zero_indices={} a={} b={} norm={} gcd={} values={}",
            u8::from(target.infinity),
            target.x,
            target.y,
            u8::from(norm.found),
            witness_text(norm.witness),
            u8::from(direct.found),
            witness_text(direct.witness),
            norm.algebraic_roots,
            norm.accepted_roots,
            norm.nonlifting_roots,
            norm.translated_nonlifting_roots,
            norm.denominator_zero_roots,
            norm.b_zero_roots,
            norm.ambiguous_roots,
            norm.norm_poly.len().saturating_sub(1),
            norm.gcd_poly.len().saturating_sub(1),
            indices_text(&norm.zero_indices),
            poly_text(&norm.a_poly),
            poly_text(&norm.b_poly),
            poly_text(&norm.norm_poly),
            poly_text(&norm.gcd_poly),
            poly_text(&norm.norm_values),
        );
        print_fp_ops("norm", norm.work.fp);
        print_poly_stats("norm", norm.work.poly);
        print_fp_ops("recovery", norm.recovery_ops);
        print_fp_ops("direct", direct.work);
        println!(
            " direct_comparisons={} direct_denominators={} direct_batch_inversions={} direct_denominator_zero_leaves={}",
            direct.comparisons,
            direct.denominators,
            direct.batch_inversions,
            direct.denominator_zero_leaves
        );
    }
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 15) as usize] as char);
    }
    output
}

#[allow(clippy::chunks_exact_to_as_chunks)]
fn sha256(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut data = input.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = [
        0x6a09e667_u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in data.chunks_exact(64) {
        let mut w = [0_u32; 64];
        for (index, word) in chunk.chunks_exact(4).enumerate() {
            w[index] = u32::from_be_bytes(word.try_into().unwrap());
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for index in 0..64 {
            let sum1 = hh
                .wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25))
                .wrapping_add((e & f) ^ ((!e) & g))
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let sum0 = (a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22))
                .wrapping_add((a & b) ^ (a & c) ^ (b & c));
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(sum1);
            d = c;
            c = b;
            b = a;
            a = sum0.wrapping_add(sum1);
        }
        for (slot, value) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut output = [0_u8; 32];
    for (chunk, word) in output.chunks_exact_mut(4).zip(h) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    output
}

fn parse_max_depth() -> usize {
    let mut args = std::env::args().skip(1);
    let mut max_depth = 2;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--max-depth" => {
                max_depth = args
                    .next()
                    .expect("--max-depth requires a value")
                    .parse()
                    .expect("invalid --max-depth value");
            }
            "--help" | "-h" => {
                println!("usage: dickson-norm-gcd [--max-depth 1..6]");
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {argument}"),
        }
    }
    assert!((1..=MAX_DEPTH).contains(&max_depth));
    if max_depth > 2 {
        assert_eq!(
            std::env::var("PPLUS_NORM_GCD_PRODUCTION").as_deref(),
            Ok("1"),
            "depths above two require the production custody runner"
        );
    }
    max_depth
}

fn main() {
    let max_depth = parse_max_depth();
    let mut control_ops = FpOps::default();
    assert!(on_curve(GENERATOR));
    assert_eq!(
        scalar_mul(GENERATOR, GROUP_ORDER, &mut control_ops),
        Point::INFINITY
    );
    println!("SCHEMA isogeny-labs/pplus-dickson-norm-gcd/v1");
    println!(
        "CURVE slug=icv1-fp31-tm3989-ae9db972 p={P} a={CURVE_A} b={CURVE_B} order={GROUP_ORDER} generator_x={} generator_y={} max_depth={max_depth} seed=none",
        GENERATOR.x, GENERATOR.y
    );
    for depth in 1..=max_depth {
        run_depth(depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_curve_and_sha_controls() {
        let mut ops = FpOps::default();
        let value = 1_234_567;
        let inverse = finv(value, &mut ops);
        assert_eq!(fmul(value, inverse, &mut ops), 1);
        assert!(on_curve(GENERATOR));
        assert_eq!(
            point_add(GENERATOR, GENERATOR.negate(), &mut ops),
            Point::INFINITY
        );
        assert_eq!(
            scalar_mul(GENERATOR, GROUP_ORDER, &mut ops),
            Point::INFINITY
        );
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn karatsuba_and_square_match_schoolbook() {
        let left: Poly = (0..19).map(|index| (17 * index + 3) as u64).collect();
        let right: Poly = (0..13).map(|index| (29 * index + 5) as u64).collect();
        let mut fast = Work::default();
        let mut slow = Work::default();
        assert_eq!(
            poly_mul(&left, &right, &mut fast),
            poly_schoolbook_mul(&left, &right, &mut slow)
        );
        let mut fast = Work::default();
        let mut slow = Work::default();
        assert_eq!(
            poly_square(&left, &mut fast),
            poly_schoolbook_square(&left, &mut slow)
        );
    }

    #[test]
    fn fast_division_matches_long_division() {
        let divisor: Poly = (0..17)
            .map(|index| (37 * index + 11) as u64)
            .chain(std::iter::once(1))
            .collect();
        let quotient: Poly = (0..21).map(|index| (41 * index + 7) as u64).collect();
        let remainder: Poly = (0..12).map(|index| (13 * index + 2) as u64).collect();
        let mut assembly = Work::default();
        let dividend = poly_add(
            &poly_mul(&divisor, &quotient, &mut assembly),
            &remainder,
            &mut assembly,
        );
        let mut fast = Work::default();
        let mut slow = Work::default();
        assert_eq!(
            poly_div_rem_fast(&dividend, &divisor, &mut fast),
            poly_div_rem_long(&dividend, &divisor, &mut slow)
        );
    }

    #[test]
    fn multipoint_matches_horner_at_depth_two() {
        let base = factor_base(2);
        let algebra = build_algebra(&base.roots, 2);
        let polynomial = vec![7, 11, 13, 17];
        let mut work = Work::default();
        let values = multipoint_values(&polynomial, &algebra.product_tree, &mut work);
        for (&root, &value) in base.roots.iter().zip(&values) {
            let mut ops = FpOps::default();
            assert_eq!(value, poly_eval(&polynomial, root, &mut ops));
        }
    }

    #[test]
    fn sealed_support_controls_through_depth_two() {
        for depth in 1..=2 {
            let base = factor_base(depth);
            let mut ops = FpOps::default();
            let support = support_entries(&base, &mut ops);
            assert_eq!(base.leaves.len(), SEALED_SUPPORT[depth - 1].0);
            assert_eq!(support.len(), SEALED_SUPPORT[depth - 1].1);
            assert_eq!(support_digest(&support), SEALED_SUPPORT[depth - 1].2);
        }
    }

    #[test]
    fn norm_and_batched_direct_agree_through_depth_two() {
        for depth in 1..=2 {
            let base = factor_base(depth);
            let algebra = build_algebra(&base.roots, depth);
            for (_, target) in fixed_targets(&base) {
                let norm = norm_query(&base, depth, &algebra, target);
                let direct = direct_query(&base, target);
                assert_eq!(norm.found, direct.found);
                if let Some(witness) = norm.witness {
                    let mut ops = FpOps::default();
                    assert_eq!(replay_pair(&base, witness, &mut ops), target);
                }
            }
        }
    }

    #[test]
    fn first_leaf_exercises_denominator_zero_chart() {
        let base = factor_base(2);
        let algebra = build_algebra(&base.roots, 2);
        let target = base.leaves[0].point;
        let norm = norm_query(&base, 2, &algebra, target);
        assert!(norm.denominator_zero_roots >= 1);
        assert!(norm
            .zero_indices
            .iter()
            .any(|&index| base.roots[index] == target.x));
    }

    #[test]
    fn ambiguous_orientation_returns_both_signs() {
        let base = factor_base(1);
        let leaves = &base.leaves_by_x[&base.leaves[0].point.x];
        let mut ops = FpOps::default();
        let candidates = orientation_candidates(leaves, 0, 0, &mut ops);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].point, candidates[1].point.negate());
    }
}
