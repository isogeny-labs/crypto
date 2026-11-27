use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;

const MODULUS: u64 = 2_147_483_647;
const COEFFICIENT_A: u64 = 2_147_483_644;
const COEFFICIENT_B: u64 = 21;
const SUBGROUP_ORDER: u64 = 2_147_487_637;
const BASE_GENERATOR: Affine = Affine {
    infinity: false,
    x: 1_391_443_431,
    y: 177_979_044,
};

fn add_mod(a: u64, b: u64) -> u64 {
    let value = a + b;
    if value >= MODULUS {
        value - MODULUS
    } else {
        value
    }
}

fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        MODULUS - (b - a)
    }
}

fn mul_mod(a: u64, b: u64) -> u64 {
    (a * b) % MODULUS
}

fn power_mod(mut value: u64, mut exponent: u64) -> u64 {
    let mut answer = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            answer = mul_mod(answer, value);
        }
        value = mul_mod(value, value);
        exponent /= 2;
    }
    answer
}

fn inverse_mod(value: u64) -> u64 {
    assert_ne!(value, 0);
    power_mod(value, MODULUS - 2)
}

fn square_root(value: u64) -> Option<u64> {
    let root = power_mod(value, (MODULUS + 1) / 4);
    (mul_mod(root, root) == value).then_some(root)
}

fn right_hand_side(x: u64) -> u64 {
    add_mod(
        add_mod(mul_mod(mul_mod(x, x), x), mul_mod(COEFFICIENT_A, x)),
        COEFFICIENT_B,
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
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

fn affine_add(p: Affine, q: Affine) -> Affine {
    if p.infinity {
        return q;
    }
    if q.infinity {
        return p;
    }
    let slope = if p.x == q.x {
        if add_mod(p.y, q.y) == 0 {
            return Affine::IDENTITY;
        }
        assert_eq!(p.y, q.y);
        let numerator = add_mod(
            add_mod(
                add_mod(mul_mod(p.x, p.x), mul_mod(p.x, p.x)),
                mul_mod(p.x, p.x),
            ),
            COEFFICIENT_A,
        );
        mul_mod(numerator, inverse_mod(add_mod(p.y, p.y)))
    } else {
        mul_mod(sub_mod(q.y, p.y), inverse_mod(sub_mod(q.x, p.x)))
    };
    let x = sub_mod(sub_mod(mul_mod(slope, slope), p.x), q.x);
    let y = sub_mod(mul_mod(slope, sub_mod(p.x, x)), p.y);
    Affine {
        infinity: false,
        x,
        y,
    }
}

fn affine_scalar(mut point: Affine, mut scalar: u64) -> Affine {
    let mut answer = Affine::IDENTITY;
    while scalar != 0 {
        if scalar & 1 == 1 {
            answer = affine_add(answer, point);
        }
        point = affine_add(point, point);
        scalar >>= 1;
    }
    answer
}

fn valid_point(point: Affine) -> bool {
    point.infinity || mul_mod(point.y, point.y) == right_hand_side(point.x)
}

fn independent_roots(depth: usize) -> Vec<u64> {
    fn extend(level: usize, depth: usize, value: u64, output: &mut Vec<u64>) {
        if level == depth {
            output.push(value);
            return;
        }
        let radicand = add_mod(value, 2);
        let root = square_root(radicand).expect("tower radicand is not a square");
        let low = root.min(MODULUS - root);
        extend(level + 1, depth, low, output);
        extend(level + 1, depth, MODULUS - low, output);
    }

    let mut roots = Vec::with_capacity(1_usize << depth);
    extend(0, depth, 0, &mut roots);
    roots
}

#[derive(Clone, Copy, Debug)]
struct IndexedLeaf {
    index: u16,
    point: Affine,
}

#[derive(Clone, Debug)]
struct IndependentBase {
    roots: Vec<u64>,
    leaves_by_path: Vec<Vec<IndexedLeaf>>,
    leaves: Vec<Affine>,
}

fn independent_base(depth: usize) -> IndependentBase {
    let roots = independent_roots(depth);
    let mut leaves_by_path = vec![Vec::new(); roots.len()];
    let mut leaves = Vec::new();
    for (path, &x) in roots.iter().enumerate() {
        let rhs = right_hand_side(x);
        if rhs == 0 {
            continue;
        }
        if let Some(root) = square_root(rhs) {
            let low = root.min(MODULUS - root);
            for y in [low, MODULUS - low] {
                let point = Affine {
                    infinity: false,
                    x,
                    y,
                };
                assert!(valid_point(point));
                leaves_by_path[path].push(IndexedLeaf {
                    index: leaves.len() as u16,
                    point,
                });
                leaves.push(point);
            }
        }
    }
    IndependentBase {
        roots,
        leaves_by_path,
        leaves,
    }
}

#[derive(Clone, Copy, Debug)]
struct FlatEntry {
    multiplicity: u32,
    witness: [u16; 2],
}

fn flat_support(leaves: &[Affine]) -> BTreeMap<Affine, FlatEntry> {
    let mut support = BTreeMap::new();
    for left in 0..leaves.len() {
        for right in left..leaves.len() {
            let sum = affine_add(leaves[left], leaves[right]);
            let witness = [left as u16, right as u16];
            support
                .entry(sum)
                .and_modify(|entry: &mut FlatEntry| {
                    entry.multiplicity += 1;
                    if witness < entry.witness {
                        entry.witness = witness;
                    }
                })
                .or_insert(FlatEntry {
                    multiplicity: 1,
                    witness,
                });
        }
    }
    support
}

#[derive(Clone, Debug)]
struct AuditState {
    support: BTreeMap<Affine, FlatEntry>,
    semantic_id: usize,
}

#[derive(Clone, Debug)]
struct AuditLevel {
    node_count: usize,
    states: Vec<AuditState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AuditLevelMetrics {
    node_count: usize,
    rows: usize,
    nonempty_rows: usize,
    columns: usize,
    rank_gf2: usize,
    entries: usize,
}

#[derive(Clone, Debug)]
struct AuditSummary {
    levels: Vec<AuditLevelMetrics>,
    actual_states: usize,
    unique_states: usize,
    actual_entries: usize,
    semantic_entries: usize,
    actual_union_edges: usize,
    unique_union_edges: usize,
    root_support: usize,
    leaf_rank_gf2: usize,
    max_rank_gf2: usize,
    peak_live_entries: usize,
}

#[derive(Default)]
struct AuditInterner {
    ids: BTreeMap<Vec<(Affine, u32)>, usize>,
    entry_counts: Vec<usize>,
    first_children: Vec<Vec<usize>>,
}

impl AuditInterner {
    fn intern(&mut self, support: &BTreeMap<Affine, FlatEntry>, children: Vec<usize>) -> usize {
        let key: Vec<_> = support
            .iter()
            .map(|(point, entry)| (*point, entry.multiplicity))
            .collect();
        if let Some(&id) = self.ids.get(&key) {
            return id;
        }
        let id = self.ids.len();
        self.ids.insert(key, id);
        self.entry_counts.push(support.len());
        self.first_children.push(children);
        id
    }
}

fn audit_pair_index(node_count: usize, left: usize, right: usize) -> usize {
    assert!(left <= right && right < node_count);
    (0..left).map(|row| node_count - row).sum::<usize>() + right - left
}

fn leaves_under(base: &IndependentBase, tree_level: usize, node: usize) -> Vec<IndexedLeaf> {
    let path_span = 1_usize << tree_level;
    base.leaves_by_path[node * path_span..(node + 1) * path_span]
        .iter()
        .flatten()
        .copied()
        .collect()
}

fn insert_direct(support: &mut BTreeMap<Affine, FlatEntry>, left: IndexedLeaf, right: IndexedLeaf) {
    let sum = affine_add(left.point, right.point);
    let witness = if left.index <= right.index {
        [left.index, right.index]
    } else {
        [right.index, left.index]
    };
    support
        .entry(sum)
        .and_modify(|entry| {
            entry.multiplicity += 1;
            if witness < entry.witness {
                entry.witness = witness;
            }
        })
        .or_insert(FlatEntry {
            multiplicity: 1,
            witness,
        });
}

fn direct_node_pair_support(
    base: &IndependentBase,
    tree_level: usize,
    left_node: usize,
    right_node: usize,
) -> BTreeMap<Affine, FlatEntry> {
    let left = leaves_under(base, tree_level, left_node);
    let right = leaves_under(base, tree_level, right_node);
    let mut support = BTreeMap::new();
    if left_node == right_node {
        for (left_index, &left_leaf) in left.iter().enumerate() {
            for &right_leaf in right.iter().skip(left_index) {
                insert_direct(&mut support, left_leaf, right_leaf);
            }
        }
    } else {
        for &left_leaf in &left {
            for &right_leaf in &right {
                insert_direct(&mut support, left_leaf, right_leaf);
            }
        }
    }
    support
}

fn audit_level_metrics(level: &AuditLevel) -> AuditLevelMetrics {
    let points: BTreeSet<_> = level
        .states
        .iter()
        .flat_map(|state| state.support.keys().copied())
        .collect();
    let columns: BTreeMap<_, _> = points
        .into_iter()
        .enumerate()
        .map(|(index, point)| (point, index))
        .collect();
    let word_count = columns.len().div_ceil(64);
    let mut pivots: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    let mut nonempty_rows = 0;
    let mut entries = 0;

    for state in &level.states {
        entries += state.support.len();
        if state.support.is_empty() {
            continue;
        }
        nonempty_rows += 1;
        let mut row = vec![0_u64; word_count];
        for point in state.support.keys() {
            let column = columns[point];
            row[column / 64] |= 1_u64 << (column % 64);
        }
        loop {
            let pivot = row
                .iter()
                .enumerate()
                .rev()
                .find_map(|(word_index, &word)| {
                    (word != 0).then(|| word_index * 64 + 63 - word.leading_zeros() as usize)
                });
            let Some(pivot) = pivot else { break };
            if let Some(basis) = pivots.get(&pivot) {
                for (value, &other) in row.iter_mut().zip(basis) {
                    *value ^= other;
                }
            } else {
                pivots.insert(pivot, row);
                break;
            }
        }
    }

    AuditLevelMetrics {
        node_count: level.node_count,
        rows: level.states.len(),
        nonempty_rows,
        columns: columns.len(),
        rank_gf2: pivots.len(),
        entries,
    }
}

fn child_semantic_ids(level: &AuditLevel, left_parent: usize, right_parent: usize) -> Vec<usize> {
    let left = [2 * left_parent, 2 * left_parent + 1];
    let right = [2 * right_parent, 2 * right_parent + 1];
    let pairs: Vec<_> = if left_parent == right_parent {
        vec![(left[0], left[0]), (left[0], left[1]), (left[1], left[1])]
    } else {
        vec![
            (left[0], right[0]),
            (left[0], right[1]),
            (left[1], right[0]),
            (left[1], right[1]),
        ]
    };
    let mut ids: Vec<_> = pairs
        .into_iter()
        .map(|(mut a, mut b)| {
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            level.states[audit_pair_index(level.node_count, a, b)].semantic_id
        })
        .collect();
    ids.sort_unstable();
    ids
}

fn independent_metric_audit(base: &IndependentBase) -> AuditSummary {
    let depth = base.roots.len().ilog2() as usize;
    assert_eq!(1_usize << depth, base.roots.len());
    let mut interner = AuditInterner::default();
    let mut levels = Vec::with_capacity(depth + 1);
    let mut actual_union_edges = 0;

    for tree_level in 0..=depth {
        let node_count = base.roots.len() >> tree_level;
        let mut states = Vec::with_capacity(node_count * (node_count + 1) / 2);
        for left_node in 0..node_count {
            for right_node in left_node..node_count {
                let support = direct_node_pair_support(base, tree_level, left_node, right_node);
                let children = if tree_level == 0 {
                    Vec::new()
                } else {
                    let child_ids = child_semantic_ids(
                        levels.last().expect("missing child audit level"),
                        left_node,
                        right_node,
                    );
                    actual_union_edges += child_ids.len();
                    child_ids
                };
                let semantic_id = interner.intern(&support, children);
                states.push(AuditState {
                    support,
                    semantic_id,
                });
            }
        }
        levels.push(AuditLevel { node_count, states });
    }

    let metrics: Vec<_> = levels.iter().map(audit_level_metrics).collect();
    AuditSummary {
        actual_states: levels.iter().map(|level| level.states.len()).sum(),
        unique_states: interner.ids.len(),
        actual_entries: metrics.iter().map(|metric| metric.entries).sum(),
        semantic_entries: interner.entry_counts.iter().sum(),
        actual_union_edges,
        unique_union_edges: interner.first_children.iter().map(Vec::len).sum(),
        root_support: levels.last().unwrap().states[0].support.len(),
        leaf_rank_gf2: metrics[0].rank_gf2,
        max_rank_gf2: metrics
            .iter()
            .map(|metric| metric.rank_gf2)
            .max()
            .unwrap_or(0),
        peak_live_entries: metrics
            .iter()
            .map(|metric| metric.entries)
            .max()
            .unwrap_or(0),
        levels: metrics,
    }
}

fn support_digest(support: &BTreeMap<Affine, FlatEntry>) -> String {
    let mut bytes = Vec::with_capacity(support.len() * 32);
    for (point, entry) in support {
        bytes.push(u8::from(point.infinity));
        bytes.extend_from_slice(&point.x.to_be_bytes());
        bytes.extend_from_slice(&point.y.to_be_bytes());
        bytes.extend_from_slice(&entry.multiplicity.to_be_bytes());
    }
    encode_hex(&sha256(&bytes))
}

fn canonical_four(left: [u16; 2], right: [u16; 2]) -> [u16; 4] {
    let mut values = [left[0], left[1], right[0], right[1]];
    values.sort_unstable();
    values
}

fn flat_query(
    support: &BTreeMap<Affine, FlatEntry>,
    leaves: &[Affine],
    target: Affine,
) -> Option<[u16; 4]> {
    let mut answer = None;
    for (sum, entry) in support {
        let complement = affine_add(target, sum.negative());
        if let Some(other) = support.get(&complement) {
            let witness = canonical_four(entry.witness, other.witness);
            if answer.is_none_or(|old| witness < old) {
                answer = Some(witness);
            }
        }
    }
    if let Some(witness) = answer {
        assert_eq!(replay(leaves, witness), target);
    }
    answer
}

fn replay(leaves: &[Affine], witness: [u16; 4]) -> Affine {
    witness.into_iter().fold(Affine::IDENTITY, |sum, index| {
        affine_add(sum, leaves[index as usize])
    })
}

fn first_planted(leaves: &[Affine]) -> (Affine, [u16; 4]) {
    for a in 0..leaves.len() {
        for b in a..leaves.len() {
            for c in b..leaves.len() {
                for d in c..leaves.len() {
                    let witness = [a as u16, b as u16, c as u16, d as u16];
                    let target = replay(leaves, witness);
                    if !target.infinity {
                        return (target, witness);
                    }
                }
            }
        }
    }
    panic!("no finite planted target");
}

#[derive(Clone, Debug)]
struct CandidateDepth {
    roots: usize,
    lifting_abscissae: usize,
    signed_leaves: usize,
    actual_states: usize,
    unique_states: usize,
    actual_entries: usize,
    semantic_entries: usize,
    actual_union_edges: usize,
    unique_union_edges: usize,
    root_support: usize,
    leaf_rank_gf2: usize,
    max_rank_gf2: usize,
    peak_live_entries: usize,
    support_sha256: String,
    levels: BTreeMap<usize, CandidateLevel>,
    queries: BTreeMap<String, CandidateQuery>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CandidateLevel {
    node_count: usize,
    rows: usize,
    nonempty_rows: usize,
    columns: usize,
    rank_gf2: usize,
    entries: usize,
}

#[derive(Clone, Debug)]
struct CandidateQuery {
    target: Affine,
    found: bool,
    replay_ok: bool,
    witness: Option<[u16; 4]>,
}

fn fields(line: &str) -> BTreeMap<&str, &str> {
    line.split_whitespace()
        .skip(1)
        .map(|token| token.split_once('=').expect("malformed key=value token"))
        .collect()
}

fn number<T: std::str::FromStr>(map: &BTreeMap<&str, &str>, key: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    map[key].parse().unwrap()
}

fn parse_witness(text: &str) -> Option<[u16; 4]> {
    if text == "-" {
        return None;
    }
    let values: Vec<_> = text
        .split(',')
        .map(|value| value.parse::<u16>().unwrap())
        .collect();
    Some(values.try_into().expect("witness must have four indices"))
}

fn parse_candidate(text: &str) -> BTreeMap<usize, CandidateDepth> {
    let mut depths = BTreeMap::new();
    for line in text.lines() {
        if line.starts_with("DEPTH ") {
            let map = fields(line);
            let depth = number(&map, "depth");
            depths.insert(
                depth,
                CandidateDepth {
                    roots: number(&map, "full_roots"),
                    lifting_abscissae: number(&map, "lifting_abscissae"),
                    signed_leaves: number(&map, "signed_leaves"),
                    actual_states: number(&map, "actual_states"),
                    unique_states: number(&map, "unique_states"),
                    actual_entries: number(&map, "actual_entries"),
                    semantic_entries: number(&map, "semantic_entries"),
                    actual_union_edges: number(&map, "actual_union_edges"),
                    unique_union_edges: number(&map, "unique_union_edges"),
                    root_support: number(&map, "root_support"),
                    leaf_rank_gf2: number(&map, "leaf_rank_gf2"),
                    max_rank_gf2: number(&map, "max_rank_gf2"),
                    peak_live_entries: number(&map, "peak_live_entries"),
                    support_sha256: map["support_sha256"].to_owned(),
                    levels: BTreeMap::new(),
                    queries: BTreeMap::new(),
                },
            );
        } else if line.starts_with("LEVEL ") {
            let map = fields(line);
            let depth: usize = number(&map, "depth");
            let tree_level = number(&map, "tree_level");
            depths
                .get_mut(&depth)
                .expect("LEVEL preceded DEPTH")
                .levels
                .insert(
                    tree_level,
                    CandidateLevel {
                        node_count: number(&map, "node_count"),
                        rows: number(&map, "rows"),
                        nonempty_rows: number(&map, "nonempty_rows"),
                        columns: number(&map, "columns"),
                        rank_gf2: number(&map, "rank_gf2"),
                        entries: number(&map, "entries"),
                    },
                );
        } else if line.starts_with("QUERY ") {
            let map = fields(line);
            let depth: usize = number(&map, "depth");
            let query = CandidateQuery {
                target: Affine {
                    infinity: number::<u8>(&map, "target_inf") == 1,
                    x: number(&map, "target_x"),
                    y: number(&map, "target_y"),
                },
                found: number::<u8>(&map, "found") == 1,
                replay_ok: number::<u8>(&map, "replay_ok") == 1,
                witness: parse_witness(map["witness"]),
            };
            depths
                .get_mut(&depth)
                .expect("QUERY preceded DEPTH")
                .queries
                .insert(map["label"].to_owned(), query);
        }
    }
    depths
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("usage: pair-transducer-verify OUTPUT");
    let candidate_text = fs::read_to_string(path).expect("cannot read candidate output");
    let candidate = parse_candidate(&candidate_text);
    assert_eq!(candidate.len(), 6);
    assert!(valid_point(BASE_GENERATOR));
    assert_eq!(
        affine_scalar(BASE_GENERATOR, SUBGROUP_ORDER),
        Affine::IDENTITY
    );
    println!("SCHEMA isogeny-labs/pplus-oriented-pair-transducer-verify/v1");

    let mut verified_queries = 0;
    for depth in 1..=6 {
        let observed = &candidate[&depth];
        let base = independent_base(depth);
        let support = flat_support(&base.leaves);
        let digest = support_digest(&support);
        let audit = independent_metric_audit(&base);
        assert_eq!(observed.roots, base.roots.len());
        assert_eq!(observed.lifting_abscissae, base.leaves.len() / 2);
        assert_eq!(observed.signed_leaves, base.leaves.len());
        assert_eq!(observed.actual_states, audit.actual_states);
        assert_eq!(observed.unique_states, audit.unique_states);
        assert_eq!(observed.actual_entries, audit.actual_entries);
        assert_eq!(observed.semantic_entries, audit.semantic_entries);
        assert_eq!(observed.actual_union_edges, audit.actual_union_edges);
        assert_eq!(observed.unique_union_edges, audit.unique_union_edges);
        assert_eq!(observed.root_support, support.len());
        assert_eq!(observed.root_support, audit.root_support);
        assert_eq!(observed.leaf_rank_gf2, audit.leaf_rank_gf2);
        assert_eq!(observed.max_rank_gf2, audit.max_rank_gf2);
        assert_eq!(observed.peak_live_entries, audit.peak_live_entries);
        assert_eq!(observed.support_sha256, digest);
        assert_eq!(observed.levels.len(), audit.levels.len());
        for (tree_level, expected) in audit.levels.iter().enumerate() {
            let actual = observed
                .levels
                .get(&tree_level)
                .expect("missing candidate LEVEL row");
            assert_eq!(actual.node_count, expected.node_count);
            assert_eq!(actual.rows, expected.rows);
            assert_eq!(actual.nonempty_rows, expected.nonempty_rows);
            assert_eq!(actual.columns, expected.columns);
            assert_eq!(actual.rank_gf2, expected.rank_gf2);
            assert_eq!(actual.entries, expected.entries);
        }
        let (planted_target, _) = first_planted(&base.leaves);

        for (label, query) in &observed.queries {
            let expected_target = if label == "planted" {
                planted_target
            } else if label == "infinity" {
                Affine::IDENTITY
            } else {
                let scalar: u64 = label
                    .strip_prefix('g')
                    .expect("unknown query label")
                    .parse()
                    .unwrap();
                assert!((1..=8).contains(&scalar));
                affine_scalar(BASE_GENERATOR, scalar)
            };
            assert_eq!(query.target, expected_target);
            assert!(query.replay_ok);
            let expected = flat_query(&support, &base.leaves, expected_target);
            assert_eq!(query.found, expected.is_some());
            assert_eq!(query.witness, expected);
            verified_queries += 1;
        }
        assert_eq!(observed.queries.len(), 10);
        println!(
            "VERIFY depth={} roots={} signed_leaves={} states={} unique_states={} semantic_entries={} leaf_rank_gf2={} root_support={} support_sha256={} queries=10 metrics=ok status=ok",
            depth,
            base.roots.len(),
            base.leaves.len(),
            audit.actual_states,
            audit.unique_states,
            audit.semantic_entries,
            audit.leaf_rank_gf2,
            support.len(),
            digest
        );
    }
    println!("SUMMARY depths=6 queries={} status=ok", verified_queries);
}

fn encode_hex(bytes: &[u8]) -> String {
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
    let bit_length = data.len() as u64 * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_length.to_be_bytes());
    let mut hash = [
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
        let mut words = [0_u32; 64];
        for (index, word) in chunk.chunks_exact(4).enumerate() {
            words[index] = u32::from_be_bytes(word.try_into().unwrap());
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = hash;
        for index in 0..64 {
            let first = h
                .wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25))
                .wrapping_add((e & f) ^ ((!e) & g))
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
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
        for (slot, value) in hash.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut output = [0_u8; 32];
    for (bytes, word) in output.chunks_exact_mut(4).zip(hash) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_arithmetic_controls() {
        assert!(valid_point(BASE_GENERATOR));
        assert_eq!(
            affine_add(BASE_GENERATOR, BASE_GENERATOR.negative()),
            Affine::IDENTITY
        );
        assert_eq!(
            affine_scalar(BASE_GENERATOR, SUBGROUP_ORDER),
            Affine::IDENTITY
        );
        assert_eq!(independent_base(6).leaves.len(), 68);
    }

    #[test]
    fn direct_metric_audit_reaches_flat_root() {
        let base = independent_base(2);
        let audit = independent_metric_audit(&base);
        assert_eq!(audit.levels.len(), 3);
        assert_eq!(audit.levels[0].node_count, 4);
        assert_eq!(audit.levels[2].node_count, 1);
        assert_eq!(audit.root_support, flat_support(&base.leaves).len());
    }

    #[test]
    fn independent_sha_control() {
        assert_eq!(
            encode_hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
