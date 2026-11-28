use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const MODULUS: u64 = 2_147_483_647;
const COEFFICIENT_A: u64 = 2_147_483_644;
const COEFFICIENT_B: u64 = 21;
const SUBGROUP_ORDER: u64 = 2_147_487_637;
const BASE_POINT: Affine = Affine {
    infinity: false,
    x: 1_391_443_431,
    y: 177_979_044,
};
const EXPECTED_SUPPORT: [(usize, usize, &str); 6] = [
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

fn add_mod(left: u64, right: u64) -> u64 {
    let value = left + right;
    if value >= MODULUS {
        value - MODULUS
    } else {
        value
    }
}

fn sub_mod(left: u64, right: u64) -> u64 {
    if left >= right {
        left - right
    } else {
        MODULUS - (right - left)
    }
}

fn neg_mod(value: u64) -> u64 {
    if value == 0 {
        0
    } else {
        MODULUS - value
    }
}

fn mul_mod(left: u64, right: u64) -> u64 {
    (left * right) % MODULUS
}

fn square_mod(value: u64) -> u64 {
    mul_mod(value, value)
}

fn power_mod(mut base: u64, mut exponent: u64) -> u64 {
    let mut accumulator = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            accumulator = mul_mod(accumulator, base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = square_mod(base);
        }
    }
    accumulator
}

fn inverse_mod(value: u64) -> u64 {
    assert_ne!(value, 0);
    power_mod(value, MODULUS - 2)
}

fn square_root(value: u64) -> Option<u64> {
    let root = power_mod(value, (MODULUS + 1) / 4);
    (square_mod(root) == value).then_some(root)
}

fn curve_value(x: u64) -> u64 {
    add_mod(
        add_mod(mul_mod(square_mod(x), x), mul_mod(COEFFICIENT_A, x)),
        COEFFICIENT_B,
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Affine {
    infinity: bool,
    x: u64,
    y: u64,
}

impl Affine {
    const IDENTITY: Self = Self {
        infinity: true,
        x: 0,
        y: 0,
    };

    fn negative(self) -> Self {
        if self.infinity || self.y == 0 {
            self
        } else {
            Self {
                infinity: false,
                x: self.x,
                y: MODULUS - self.y,
            }
        }
    }
}

fn elliptic_sum(left: Affine, right: Affine) -> Affine {
    if left.infinity {
        return right;
    }
    if right.infinity {
        return left;
    }
    let slope = if left.x == right.x {
        if add_mod(left.y, right.y) == 0 {
            return Affine::IDENTITY;
        }
        assert_eq!(left.y, right.y);
        let numerator = add_mod(mul_mod(3, square_mod(left.x)), COEFFICIENT_A);
        mul_mod(numerator, inverse_mod(add_mod(left.y, left.y)))
    } else {
        mul_mod(
            sub_mod(right.y, left.y),
            inverse_mod(sub_mod(right.x, left.x)),
        )
    };
    let x = sub_mod(sub_mod(square_mod(slope), left.x), right.x);
    let y = sub_mod(mul_mod(slope, sub_mod(left.x, x)), left.y);
    Affine {
        infinity: false,
        x,
        y,
    }
}

fn multiply_point(mut point: Affine, mut scalar: u64) -> Affine {
    let mut result = Affine::IDENTITY;
    while scalar != 0 {
        if scalar & 1 == 1 {
            result = elliptic_sum(result, point);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = elliptic_sum(point, point);
        }
    }
    result
}

fn is_on_curve(point: Affine) -> bool {
    point.infinity || square_mod(point.y) == curve_value(point.x)
}

#[derive(Clone, Copy, Debug)]
struct SignedLeaf {
    index: u16,
    point: Affine,
}

#[derive(Clone, Debug)]
struct IndependentBase {
    roots: Vec<u64>,
    leaves: Vec<SignedLeaf>,
    leaves_by_x: BTreeMap<u64, Vec<SignedLeaf>>,
    indices: BTreeMap<Affine, u16>,
}

fn independent_roots(depth: usize) -> Vec<u64> {
    let mut layer = vec![0];
    for _ in 0..depth {
        let mut next = Vec::with_capacity(2 * layer.len());
        for value in layer {
            let root = square_root(add_mod(value, 2)).expect("independent tower does not split");
            let low = root.min(MODULUS - root);
            next.push(low);
            next.push(if low == 0 { 0 } else { MODULUS - low });
        }
        layer = next;
    }
    let distinct: BTreeSet<_> = layer.iter().copied().collect();
    assert_eq!(distinct.len(), 1_usize << depth);
    for &root in &layer {
        let mut value = root;
        for _ in 0..depth {
            value = sub_mod(square_mod(value), 2);
        }
        assert_eq!(value, 0);
    }
    layer
}

fn independent_base(depth: usize) -> IndependentBase {
    let roots = independent_roots(depth);
    let mut leaves = Vec::new();
    let mut leaves_by_x = BTreeMap::<u64, Vec<SignedLeaf>>::new();
    let mut indices = BTreeMap::new();
    for &x in &roots {
        let rhs = curve_value(x);
        if rhs == 0 {
            continue;
        }
        let Some(root) = square_root(rhs) else {
            continue;
        };
        let low = root.min(MODULUS - root);
        for y in [low, MODULUS - low] {
            let leaf = SignedLeaf {
                index: leaves.len() as u16,
                point: Affine {
                    infinity: false,
                    x,
                    y,
                },
            };
            assert!(is_on_curve(leaf.point));
            indices.insert(leaf.point, leaf.index);
            leaves_by_x.entry(x).or_default().push(leaf);
            leaves.push(leaf);
        }
    }
    IndependentBase {
        roots,
        leaves,
        leaves_by_x,
        indices,
    }
}

type Polynomial = Vec<u64>;

fn normalize(mut polynomial: Polynomial) -> Polynomial {
    while polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    polynomial
}

fn polynomial_multiply(left: &[u64], right: &[u64]) -> Polynomial {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let mut output = vec![0; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            output[i + j] = add_mod(output[i + j], mul_mod(a, b));
        }
    }
    normalize(output)
}

fn polynomial_eval(polynomial: &[u64], value: u64) -> u64 {
    polynomial
        .iter()
        .rev()
        .fold(0, |accumulator, &coefficient| {
            add_mod(mul_mod(accumulator, value), coefficient)
        })
}

fn polynomial_from_roots(roots: &[u64], indices: &[usize]) -> Polynomial {
    let mut result = vec![1];
    for &index in indices {
        result = polynomial_multiply(&result, &[neg_mod(roots[index]), 1]);
    }
    result
}

fn dickson_selector(depth: usize) -> Polynomial {
    let mut result = vec![0, 1];
    for _ in 0..depth {
        result = polynomial_multiply(&result, &result);
        result[0] = sub_mod(result[0], 2);
        result = normalize(result);
    }
    result
}

#[derive(Clone, Debug)]
struct DepthRecord {
    depth: usize,
    full_roots: usize,
    lifting_abscissae: usize,
    signed_leaves: usize,
    support: usize,
    support_sha256: String,
    selector_degree: usize,
    roots: Vec<u64>,
    selector: Polynomial,
}

#[derive(Clone, Debug)]
struct QueryRecord {
    depth: usize,
    label: String,
    target: Affine,
    found: bool,
    witness: Option<[u16; 2]>,
    direct_found: bool,
    direct_witness: Option<[u16; 2]>,
    algebraic_roots: usize,
    accepted_roots: usize,
    nonlifting_roots: usize,
    translated_nonlifting_roots: usize,
    denominator_zero_roots: usize,
    b_zero_roots: usize,
    ambiguous_roots: usize,
    norm_degree: usize,
    gcd_degree: usize,
    zero_indices: Vec<usize>,
    a: Polynomial,
    b: Polynomial,
    norm: Polynomial,
    gcd: Polynomial,
    values: Vec<u64>,
}

fn fields(line: &str) -> BTreeMap<&str, &str> {
    line.split_whitespace()
        .skip(1)
        .map(|field| field.split_once('=').expect("malformed key=value field"))
        .collect()
}

fn required<'a>(map: &'a BTreeMap<&str, &str>, name: &str) -> &'a str {
    map.get(name)
        .copied()
        .unwrap_or_else(|| panic!("missing transcript field {name}"))
}

fn number(map: &BTreeMap<&str, &str>, name: &str) -> u64 {
    required(map, name)
        .parse()
        .unwrap_or_else(|_| panic!("invalid numeric transcript field {name}"))
}

fn list_u64(text: &str) -> Vec<u64> {
    if text == "-" {
        Vec::new()
    } else {
        text.split(',')
            .map(|value| value.parse().expect("invalid u64 list"))
            .collect()
    }
}

fn list_usize(text: &str) -> Vec<usize> {
    if text == "-" {
        Vec::new()
    } else {
        text.split(',')
            .map(|value| value.parse().expect("invalid usize list"))
            .collect()
    }
}

fn parse_witness(text: &str) -> Option<[u16; 2]> {
    if text == "-" {
        return None;
    }
    let values: Vec<u16> = text
        .split(',')
        .map(|value| value.parse().expect("invalid witness"))
        .collect();
    assert_eq!(values.len(), 2);
    Some([values[0], values[1]])
}

fn parse_depth(line: &str) -> DepthRecord {
    let map = fields(line);
    DepthRecord {
        depth: number(&map, "depth") as usize,
        full_roots: number(&map, "full_roots") as usize,
        lifting_abscissae: number(&map, "lifting_abscissae") as usize,
        signed_leaves: number(&map, "signed_leaves") as usize,
        support: number(&map, "support") as usize,
        support_sha256: required(&map, "support_sha256").to_owned(),
        selector_degree: number(&map, "selector_degree") as usize,
        roots: list_u64(required(&map, "roots")),
        selector: list_u64(required(&map, "selector")),
    }
}

fn parse_query(line: &str) -> QueryRecord {
    let map = fields(line);
    QueryRecord {
        depth: number(&map, "depth") as usize,
        label: required(&map, "label").to_owned(),
        target: Affine {
            infinity: number(&map, "target_inf") == 1,
            x: number(&map, "target_x"),
            y: number(&map, "target_y"),
        },
        found: number(&map, "found") == 1,
        witness: parse_witness(required(&map, "witness")),
        direct_found: number(&map, "direct_found") == 1,
        direct_witness: parse_witness(required(&map, "direct_witness")),
        algebraic_roots: number(&map, "algebraic_roots") as usize,
        accepted_roots: number(&map, "accepted_roots") as usize,
        nonlifting_roots: number(&map, "nonlifting_roots") as usize,
        translated_nonlifting_roots: number(&map, "translated_nonlifting_roots") as usize,
        denominator_zero_roots: number(&map, "denominator_zero_roots") as usize,
        b_zero_roots: number(&map, "b_zero_roots") as usize,
        ambiguous_roots: number(&map, "ambiguous_roots") as usize,
        norm_degree: number(&map, "norm_degree") as usize,
        gcd_degree: number(&map, "gcd_degree") as usize,
        zero_indices: list_usize(required(&map, "zero_indices")),
        a: list_u64(required(&map, "a")),
        b: list_u64(required(&map, "b")),
        norm: list_u64(required(&map, "norm")),
        gcd: list_u64(required(&map, "gcd")),
        values: list_u64(required(&map, "values")),
    }
}

fn read_transcript(path: &str) -> (usize, Vec<DepthRecord>, Vec<QueryRecord>) {
    let file = File::open(path).expect("cannot open candidate transcript");
    let mut schema_seen = false;
    let mut max_depth = None;
    let mut depths = Vec::new();
    let mut queries = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("cannot read candidate transcript");
        if line.starts_with("SCHEMA ") {
            assert_eq!(line, "SCHEMA isogeny-labs/pplus-dickson-norm-gcd/v1");
            assert!(!schema_seen);
            schema_seen = true;
        } else if line.starts_with("CURVE ") {
            let map = fields(&line);
            assert_eq!(required(&map, "slug"), "icv1-fp31-tm3989-ae9db972");
            assert_eq!(number(&map, "p"), MODULUS);
            assert_eq!(number(&map, "a"), COEFFICIENT_A);
            assert_eq!(number(&map, "b"), COEFFICIENT_B);
            assert_eq!(number(&map, "order"), SUBGROUP_ORDER);
            assert_eq!(number(&map, "generator_x"), BASE_POINT.x);
            assert_eq!(number(&map, "generator_y"), BASE_POINT.y);
            assert_eq!(required(&map, "seed"), "none");
            max_depth = Some(number(&map, "max_depth") as usize);
        } else if line.starts_with("DEPTH ") {
            depths.push(parse_depth(&line));
        } else if line.starts_with("QUERY ") {
            queries.push(parse_query(&line));
        } else if !line.is_empty() {
            panic!("unknown candidate transcript row");
        }
    }
    assert!(schema_seen);
    let max_depth = max_depth.expect("missing curve transcript row");
    assert_eq!(depths.len(), max_depth);
    assert_eq!(queries.len(), 11 * max_depth);
    (max_depth, depths, queries)
}

#[derive(Clone, Copy)]
struct SupportValue {
    point: Affine,
    multiplicity: u32,
}

fn independent_support(base: &IndependentBase) -> Vec<SupportValue> {
    let mut map = BTreeMap::<Affine, u32>::new();
    for left in 0..base.leaves.len() {
        for right in left..base.leaves.len() {
            let sum = elliptic_sum(base.leaves[left].point, base.leaves[right].point);
            *map.entry(sum).or_default() += 1;
        }
    }
    map.into_iter()
        .map(|(point, multiplicity)| SupportValue {
            point,
            multiplicity,
        })
        .collect()
}

fn digest_support(entries: &[SupportValue]) -> String {
    let mut input = Vec::with_capacity(32 * entries.len());
    for entry in entries {
        input.push(u8::from(entry.point.infinity));
        input.extend_from_slice(&entry.point.x.to_be_bytes());
        input.extend_from_slice(&entry.point.y.to_be_bytes());
        input.extend_from_slice(&entry.multiplicity.to_be_bytes());
    }
    encode_hex(&sha256(&input))
}

fn canonical_pair(left: u16, right: u16) -> [u16; 2] {
    if left <= right {
        [left, right]
    } else {
        [right, left]
    }
}

fn direct_truth(base: &IndependentBase, target: Affine) -> Option<[u16; 2]> {
    let mut answer = None;
    for left in 0..base.leaves.len() {
        for right in left..base.leaves.len() {
            if elliptic_sum(base.leaves[left].point, base.leaves[right].point) == target {
                let candidate = [left as u16, right as u16];
                if answer.is_none_or(|current| candidate < current) {
                    answer = Some(candidate);
                }
            }
        }
    }
    answer
}

fn replay(base: &IndependentBase, witness: [u16; 2]) -> Affine {
    assert!((witness[0] as usize) < base.leaves.len());
    assert!((witness[1] as usize) < base.leaves.len());
    elliptic_sum(
        base.leaves[witness[0] as usize].point,
        base.leaves[witness[1] as usize].point,
    )
}

fn planted_target(base: &IndependentBase) -> Affine {
    for left in 0..base.leaves.len() {
        for right in left..base.leaves.len() {
            let target = elliptic_sum(base.leaves[left].point, base.leaves[right].point);
            if !target.infinity {
                return target;
            }
        }
    }
    panic!("no independent planted target")
}

fn expected_targets(base: &IndependentBase) -> Vec<(String, Affine)> {
    let mut targets = vec![("planted_pair".to_owned(), planted_target(base))];
    for scalar in 1..=8 {
        targets.push((format!("g{scalar}"), multiply_point(BASE_POINT, scalar)));
    }
    targets.push(("first_leaf".to_owned(), base.leaves[0].point));
    targets.push(("infinity".to_owned(), Affine::IDENTITY));
    targets
}

fn pointwise_recurrence(
    base: &IndependentBase,
    depth: usize,
    target: Affine,
) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    assert!(!target.infinity);
    let mut a_values = Vec::with_capacity(base.roots.len());
    let mut b_values = Vec::with_capacity(base.roots.len());
    let mut norm_values = Vec::with_capacity(base.roots.len());
    for &x in &base.roots {
        let f = curve_value(x);
        let difference = sub_mod(target.x, x);
        let mut delta = square_mod(difference);
        let mut a = sub_mod(
            add_mod(square_mod(target.y), f),
            mul_mod(add_mod(target.x, x), delta),
        );
        let mut b = add_mod(target.y, target.y);
        for _ in 0..depth {
            let delta_square = square_mod(delta);
            let next_a = sub_mod(
                add_mod(square_mod(a), mul_mod(f, square_mod(b))),
                add_mod(delta_square, delta_square),
            );
            let product = mul_mod(a, b);
            let next_b = add_mod(product, product);
            a = next_a;
            b = next_b;
            delta = delta_square;
        }
        a_values.push(a);
        b_values.push(b);
        norm_values.push(sub_mod(square_mod(a), mul_mod(f, square_mod(b))));
    }
    (a_values, b_values, norm_values)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ChartReplay {
    accepted_roots: usize,
    nonlifting_roots: usize,
    translated_nonlifting_roots: usize,
    denominator_zero_roots: usize,
    b_zero_roots: usize,
    ambiguous_roots: usize,
    witness: Option<[u16; 2]>,
}

fn replay_charts(
    base: &IndependentBase,
    target: Affine,
    a_values: &[u64],
    b_values: &[u64],
    zero_indices: &[usize],
) -> ChartReplay {
    let mut replay = ChartReplay::default();
    let mut accepted = BTreeSet::new();
    for &root_index in zero_indices {
        let x = base.roots[root_index];
        let Some(leaves) = base.leaves_by_x.get(&x) else {
            replay.nonlifting_roots += 1;
            continue;
        };
        let candidates: Vec<SignedLeaf> = if x == target.x {
            replay.denominator_zero_roots += 1;
            leaves.clone()
        } else if b_values[root_index] == 0 {
            replay.b_zero_roots += 1;
            if a_values[root_index] == 0 {
                replay.ambiguous_roots += 1;
                leaves.clone()
            } else {
                Vec::new()
            }
        } else {
            let y = mul_mod(
                neg_mod(a_values[root_index]),
                inverse_mod(b_values[root_index]),
            );
            leaves
                .iter()
                .copied()
                .filter(|leaf| leaf.point.y == y)
                .collect()
        };
        let mut accepted_here = false;
        for leaf in candidates {
            let complement = elliptic_sum(target, leaf.point.negative());
            if let Some(&other) = base.indices.get(&complement) {
                accepted_here = true;
                let candidate = canonical_pair(leaf.index, other);
                if replay.witness.is_none_or(|current| candidate < current) {
                    replay.witness = Some(candidate);
                }
            }
        }
        if accepted_here {
            accepted.insert(root_index);
        } else {
            replay.translated_nonlifting_roots += 1;
        }
    }
    replay.accepted_roots = accepted.len();
    replay
}

#[derive(Default)]
struct VerificationTotals {
    depths: usize,
    queries: usize,
    hits: usize,
    finite_misses: usize,
    coefficient_evaluations: usize,
    root_values: usize,
    denominator_zero_queries: usize,
    ambiguous_queries: usize,
}

fn verify_depth(
    record: &DepthRecord,
    query_records: &[QueryRecord],
    totals: &mut VerificationTotals,
) {
    let depth = record.depth;
    assert!((1..=6).contains(&depth));
    let base = independent_base(depth);
    let support = independent_support(&base);
    let digest = digest_support(&support);
    let expected = EXPECTED_SUPPORT[depth - 1];

    assert_eq!(record.full_roots, base.roots.len());
    assert_eq!(record.lifting_abscissae, base.leaves_by_x.len());
    assert_eq!(record.signed_leaves, base.leaves.len());
    assert_eq!(record.support, support.len());
    assert_eq!(record.roots, base.roots);
    assert_eq!(record.support_sha256, digest);
    assert_eq!(record.signed_leaves, expected.0);
    assert_eq!(record.support, expected.1);
    assert_eq!(record.support_sha256, expected.2);

    let selector = dickson_selector(depth);
    assert_eq!(record.selector, selector);
    assert_eq!(record.selector_degree, selector.len() - 1);
    for &root in &base.roots {
        assert_eq!(polynomial_eval(&selector, root), 0);
    }

    let expected_queries = expected_targets(&base);
    assert_eq!(query_records.len(), expected_queries.len());
    let mut depth_misses = 0;
    for (record, (expected_label, expected_target)) in query_records.iter().zip(&expected_queries) {
        assert_eq!(record.depth, depth);
        assert_eq!(&record.label, expected_label);
        assert_eq!(&record.target, expected_target);
        assert!(is_on_curve(record.target));
        let truth = direct_truth(&base, record.target);
        assert_eq!(record.found, truth.is_some());
        assert_eq!(record.direct_found, truth.is_some());
        assert_eq!(record.direct_witness, truth);
        if let Some(witness) = record.witness {
            assert_eq!(replay(&base, witness), record.target);
        }
        assert_eq!(record.witness.is_some(), record.found);

        if record.target.infinity {
            assert!(record.a.is_empty());
            assert!(record.b.is_empty());
            assert!(record.norm.is_empty());
            assert_eq!(record.gcd, vec![1]);
            assert!(record.values.is_empty());
            assert!(record.zero_indices.is_empty());
            assert_eq!(record.algebraic_roots, 0);
            assert_eq!(record.accepted_roots, usize::from(record.found));
        } else {
            assert!(record.a.len() < selector.len());
            assert!(record.b.len() < selector.len());
            assert!(record.norm.len() < selector.len());
            assert_eq!(record.norm_degree, record.norm.len().saturating_sub(1));
            assert_eq!(record.gcd_degree, record.gcd.len().saturating_sub(1));

            let (a_values, b_values, norm_values) =
                pointwise_recurrence(&base, depth, record.target);
            assert_eq!(record.values, norm_values);
            for (index, &root) in base.roots.iter().enumerate() {
                assert_eq!(polynomial_eval(&record.a, root), a_values[index]);
                assert_eq!(polynomial_eval(&record.b, root), b_values[index]);
                assert_eq!(polynomial_eval(&record.norm, root), norm_values[index]);
                totals.coefficient_evaluations += 3;
            }
            totals.root_values += norm_values.len();
            let zero_indices: Vec<usize> = norm_values
                .iter()
                .enumerate()
                .filter_map(|(index, &value)| (value == 0).then_some(index))
                .collect();
            assert_eq!(record.zero_indices, zero_indices);
            assert_eq!(record.algebraic_roots, zero_indices.len());
            assert_eq!(
                record.gcd,
                polynomial_from_roots(&base.roots, &zero_indices)
            );

            let charts = replay_charts(&base, record.target, &a_values, &b_values, &zero_indices);
            assert_eq!(record.accepted_roots, charts.accepted_roots);
            assert_eq!(record.nonlifting_roots, charts.nonlifting_roots);
            assert_eq!(
                record.translated_nonlifting_roots,
                charts.translated_nonlifting_roots
            );
            assert_eq!(record.denominator_zero_roots, charts.denominator_zero_roots);
            assert_eq!(record.b_zero_roots, charts.b_zero_roots);
            assert_eq!(record.ambiguous_roots, charts.ambiguous_roots);
            assert_eq!(record.witness, charts.witness);
            assert_eq!(charts.witness.is_some(), truth.is_some());
            totals.denominator_zero_queries += usize::from(charts.denominator_zero_roots != 0);
            totals.ambiguous_queries += usize::from(charts.ambiguous_roots != 0);
            if truth.is_none() {
                depth_misses += 1;
                totals.finite_misses += 1;
            }
        }
        totals.queries += 1;
        totals.hits += usize::from(record.found);
    }
    if depth == 6 {
        assert!(depth_misses > 0, "depth six lacks a finite miss");
    }
    totals.depths += 1;
    println!(
        "VERIFY_DEPTH depth={depth} roots={} leaves={} support={} queries={} finite_misses={} support_sha256={} status=pass",
        base.roots.len(),
        base.leaves.len(),
        support.len(),
        query_records.len(),
        depth_misses,
        digest
    );
}

fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(2 * bytes.len());
    for &byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 15) as usize] as char);
    }
    output
}

#[allow(clippy::chunks_exact_to_as_chunks)]
fn sha256(input: &[u8]) -> [u8; 32] {
    const ROUND: [u32; 64] = [
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
    let mut message = input.to_vec();
    let bit_length = 8 * message.len() as u64;
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_length.to_be_bytes());
    let mut state = [
        0x6a09e667_u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for block in message.chunks_exact(64) {
        let mut schedule = [0_u32; 64];
        for (index, word) in block.chunks_exact(4).enumerate() {
            schedule[index] = u32::from_be_bytes(word.try_into().unwrap());
        }
        for index in 16..64 {
            let low = schedule[index - 15].rotate_right(7)
                ^ schedule[index - 15].rotate_right(18)
                ^ (schedule[index - 15] >> 3);
            let high = schedule[index - 2].rotate_right(17)
                ^ schedule[index - 2].rotate_right(19)
                ^ (schedule[index - 2] >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(low)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(high);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let first = h
                .wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25))
                .wrapping_add((e & f) ^ ((!e) & g))
                .wrapping_add(ROUND[index])
                .wrapping_add(schedule[index]);
            let second = (a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22))
                .wrapping_add((a & b) ^ (a & c) ^ (b & c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(first);
            d = c;
            c = b;
            b = a;
            a = first.wrapping_add(second);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut digest = [0_u8; 32];
    for (chunk, word) in digest.chunks_exact_mut(4).zip(state) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    digest
}

fn candidate_path() -> String {
    let mut arguments = std::env::args().skip(1);
    let mut path = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--candidate" => path = arguments.next(),
            "--help" | "-h" => {
                println!("usage: dickson-norm-gcd-verify --candidate PATH");
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {argument}"),
        }
    }
    path.expect("--candidate is required")
}

fn main() {
    let path = candidate_path();
    assert!(is_on_curve(BASE_POINT));
    assert_eq!(multiply_point(BASE_POINT, SUBGROUP_ORDER), Affine::IDENTITY);
    let (max_depth, depth_records, query_records) = read_transcript(&path);
    let mut totals = VerificationTotals::default();
    for depth in 1..=max_depth {
        let record = depth_records
            .iter()
            .find(|record| record.depth == depth)
            .expect("missing depth record");
        let records: Vec<QueryRecord> = query_records
            .iter()
            .filter(|record| record.depth == depth)
            .cloned()
            .collect();
        verify_depth(record, &records, &mut totals);
    }
    assert_eq!(totals.depths, max_depth);
    assert_eq!(totals.queries, 11 * max_depth);
    assert!(totals.denominator_zero_queries >= max_depth);
    println!(
        "VERIFY_SUMMARY depths={} queries={} hits={} finite_misses={} coefficient_evaluations={} root_values={} denominator_zero_queries={} ambiguous_queries={} seed=none verdict=pass",
        totals.depths,
        totals.queries,
        totals.hits,
        totals.finite_misses,
        totals.coefficient_evaluations,
        totals.root_values,
        totals.denominator_zero_queries,
        totals.ambiguous_queries
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_field_curve_and_sha() {
        assert_eq!(mul_mod(987_654, inverse_mod(987_654)), 1);
        assert!(is_on_curve(BASE_POINT));
        assert_eq!(multiply_point(BASE_POINT, SUBGROUP_ORDER), Affine::IDENTITY);
        assert_eq!(
            encode_hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn independent_sealed_support_through_depth_two() {
        for depth in 1..=2 {
            let base = independent_base(depth);
            let support = independent_support(&base);
            assert_eq!(base.leaves.len(), EXPECTED_SUPPORT[depth - 1].0);
            assert_eq!(support.len(), EXPECTED_SUPPORT[depth - 1].1);
            assert_eq!(digest_support(&support), EXPECTED_SUPPORT[depth - 1].2);
        }
    }

    #[test]
    fn pointwise_norm_detects_planted_pair() {
        let depth = 2;
        let base = independent_base(depth);
        let target = planted_target(&base);
        let (a, b, norm) = pointwise_recurrence(&base, depth, target);
        let zeros: Vec<usize> = norm
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| (value == 0).then_some(index))
            .collect();
        let charts = replay_charts(&base, target, &a, &b, &zeros);
        assert!(charts.witness.is_some());
        assert_eq!(replay(&base, charts.witness.unwrap()), target);
    }
}
