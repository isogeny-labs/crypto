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
const M7_RESIDUAL_QUERIES: u64 = 27_370;

#[derive(Clone, Copy, Debug, Default)]
struct FpOps {
    adds: u64,
    subs: u64,
    muls: u64,
    squares: u64,
    inversions: u64,
}

impl FpOps {
    fn plus(self, rhs: Self) -> Self {
        Self {
            adds: self.adds + rhs.adds,
            subs: self.subs + rhs.subs,
            muls: self.muls + rhs.muls,
            squares: self.squares + rhs.squares,
            inversions: self.inversions + rhs.inversions,
        }
    }

    fn scaled(self, scale: u64) -> Self {
        Self {
            adds: self.adds * scale,
            subs: self.subs * scale,
            muls: self.muls * scale,
            squares: self.squares * scale,
            inversions: self.inversions * scale,
        }
    }
}

fn fadd(a: u64, b: u64, ops: &mut FpOps) -> u64 {
    ops.adds += 1;
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
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

fn fmul(a: u64, b: u64, ops: &mut FpOps) -> u64 {
    ops.muls += 1;
    (a * b) % P
}

fn fsquare(a: u64, ops: &mut FpOps) -> u64 {
    ops.squares += 1;
    (a * a) % P
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
    if fsquare(root, ops) == value {
        Some(root)
    } else {
        None
    }
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
        let y_sum = fadd(left.y, right.y, ops);
        if y_sum == 0 {
            return Point::INFINITY;
        }
        assert_eq!(left.y, right.y, "equal x must be equal or inverse");
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

    let lambda2 = fsquare(lambda, ops);
    let x3 = fsub(fsub(lambda2, left.x, ops), right.x, ops);
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

#[derive(Clone, Debug)]
struct Leaf {
    index: u16,
    point: Point,
}

#[derive(Clone, Debug)]
struct FactorBase {
    roots: Vec<u64>,
    leaves_by_path: Vec<Vec<Leaf>>,
    leaves: Vec<Leaf>,
    ops: FpOps,
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
        assert_eq!(value, 0, "Dickson tower replay failed");
    }
    states
}

fn factor_base(depth: usize) -> FactorBase {
    let mut ops = FpOps::default();
    let roots = dickson_roots(depth, &mut ops);
    let mut leaves_by_path = vec![Vec::new(); roots.len()];
    let mut leaves = Vec::new();

    for (path, &x) in roots.iter().enumerate() {
        let rhs = curve_rhs(x, &mut ops);
        if rhs == 0 {
            continue;
        }
        if let Some(y) = fsqrt(rhs, &mut ops) {
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
                leaves_by_path[path].push(leaf.clone());
                leaves.push(leaf);
            }
        }
    }

    FactorBase {
        roots,
        leaves_by_path,
        leaves,
        ops,
    }
}

#[derive(Clone, Debug)]
struct SupportEntry {
    point: Point,
    multiplicity: u32,
    witness: [u16; 2],
}

#[derive(Clone, Debug)]
struct State {
    entries: Vec<SupportEntry>,
    semantic_id: usize,
}

#[derive(Clone, Debug)]
struct Level {
    node_count: usize,
    states: Vec<State>,
}

#[derive(Clone, Copy, Debug, Default)]
struct CircuitCounters {
    point_additions: u64,
    map_probes: u64,
    collision_merges: u64,
    support_merges: u64,
    actual_union_edges: u64,
}

#[derive(Default)]
struct Interner {
    ids: BTreeMap<Vec<(Point, u32)>, usize>,
    entry_counts: Vec<usize>,
    first_children: Vec<Vec<usize>>,
}

impl Interner {
    fn intern(&mut self, entries: &[SupportEntry], children: Vec<usize>) -> usize {
        let key: Vec<_> = entries
            .iter()
            .map(|entry| (entry.point, entry.multiplicity))
            .collect();
        if let Some(&id) = self.ids.get(&key) {
            return id;
        }
        let id = self.ids.len();
        self.ids.insert(key, id);
        self.entry_counts.push(entries.len());
        self.first_children.push(children);
        id
    }

    fn semantic_entries(&self) -> usize {
        self.entry_counts.iter().sum()
    }

    fn unique_edges(&self) -> usize {
        self.first_children.iter().map(Vec::len).sum()
    }
}

fn pair_index(node_count: usize, left: usize, right: usize) -> usize {
    assert!(left <= right && right < node_count);
    left * node_count - left * left.saturating_sub(1) / 2 + (right - left)
}

fn canonical_pair(left: u16, right: u16) -> [u16; 2] {
    if left <= right {
        [left, right]
    } else {
        [right, left]
    }
}

fn insert_entry(
    map: &mut BTreeMap<Point, (u32, [u16; 2])>,
    point: Point,
    multiplicity: u32,
    witness: [u16; 2],
    counters: &mut CircuitCounters,
) {
    counters.map_probes += 1;
    match map.get_mut(&point) {
        Some((old_multiplicity, old_witness)) => {
            counters.collision_merges += 1;
            *old_multiplicity += multiplicity;
            if witness < *old_witness {
                *old_witness = witness;
            }
        }
        None => {
            map.insert(point, (multiplicity, witness));
        }
    }
}

fn finish_state(
    map: BTreeMap<Point, (u32, [u16; 2])>,
    children: Vec<usize>,
    interner: &mut Interner,
) -> State {
    let entries: Vec<_> = map
        .into_iter()
        .map(|(point, (multiplicity, witness))| SupportEntry {
            point,
            multiplicity,
            witness,
        })
        .collect();
    let semantic_id = interner.intern(&entries, children);
    State {
        entries,
        semantic_id,
    }
}

fn build_transducer(base: &FactorBase) -> (Vec<Level>, Interner, CircuitCounters, FpOps) {
    let mut interner = Interner::default();
    let mut counters = CircuitCounters::default();
    let mut field_ops = FpOps::default();
    let leaf_nodes = base.leaves_by_path.len();
    let mut leaf_states = Vec::with_capacity(leaf_nodes * (leaf_nodes + 1) / 2);

    for left_node in 0..leaf_nodes {
        for right_node in left_node..leaf_nodes {
            let mut map = BTreeMap::new();
            let left = &base.leaves_by_path[left_node];
            let right = &base.leaves_by_path[right_node];
            if left_node == right_node {
                for (left_index, left_leaf) in left.iter().enumerate() {
                    for right_leaf in right.iter().skip(left_index) {
                        let point = point_add(left_leaf.point, right_leaf.point, &mut field_ops);
                        counters.point_additions += 1;
                        insert_entry(
                            &mut map,
                            point,
                            1,
                            canonical_pair(left_leaf.index, right_leaf.index),
                            &mut counters,
                        );
                    }
                }
            } else {
                for left_leaf in left {
                    for right_leaf in right {
                        let point = point_add(left_leaf.point, right_leaf.point, &mut field_ops);
                        counters.point_additions += 1;
                        insert_entry(
                            &mut map,
                            point,
                            1,
                            canonical_pair(left_leaf.index, right_leaf.index),
                            &mut counters,
                        );
                    }
                }
            }
            leaf_states.push(finish_state(map, Vec::new(), &mut interner));
        }
    }

    let mut levels = vec![Level {
        node_count: leaf_nodes,
        states: leaf_states,
    }];

    while levels.last().unwrap().node_count > 1 {
        let child_level = levels.last().unwrap();
        let parent_count = child_level.node_count / 2;
        let mut parent_states = Vec::with_capacity(parent_count * (parent_count + 1) / 2);

        for left_parent in 0..parent_count {
            for right_parent in left_parent..parent_count {
                let left_children = [2 * left_parent, 2 * left_parent + 1];
                let right_children = [2 * right_parent, 2 * right_parent + 1];
                let child_pairs: Vec<(usize, usize)> = if left_parent == right_parent {
                    vec![
                        (left_children[0], left_children[0]),
                        (left_children[0], left_children[1]),
                        (left_children[1], left_children[1]),
                    ]
                } else {
                    vec![
                        (left_children[0], right_children[0]),
                        (left_children[0], right_children[1]),
                        (left_children[1], right_children[0]),
                        (left_children[1], right_children[1]),
                    ]
                };

                let mut map = BTreeMap::new();
                let mut semantic_children = Vec::with_capacity(child_pairs.len());
                counters.actual_union_edges += child_pairs.len() as u64;
                for (mut left_child, mut right_child) in child_pairs {
                    if left_child > right_child {
                        std::mem::swap(&mut left_child, &mut right_child);
                    }
                    let state = &child_level.states
                        [pair_index(child_level.node_count, left_child, right_child)];
                    semantic_children.push(state.semantic_id);
                    for entry in &state.entries {
                        counters.support_merges += 1;
                        insert_entry(
                            &mut map,
                            entry.point,
                            entry.multiplicity,
                            entry.witness,
                            &mut counters,
                        );
                    }
                }
                semantic_children.sort_unstable();
                parent_states.push(finish_state(map, semantic_children, &mut interner));
            }
        }
        levels.push(Level {
            node_count: parent_count,
            states: parent_states,
        });
    }

    (levels, interner, counters, field_ops)
}

#[derive(Clone, Copy, Debug, Default)]
struct RankMetrics {
    rows: usize,
    nonempty_rows: usize,
    columns: usize,
    rank: usize,
    word_xors: u64,
    entries: usize,
}

fn incidence_rank(level: &Level) -> RankMetrics {
    let mut columns = BTreeMap::new();
    let mut entries = 0;
    for state in &level.states {
        entries += state.entries.len();
        for entry in &state.entries {
            let next = columns.len();
            columns.entry(entry.point).or_insert(next);
        }
    }
    let column_count = columns.len();
    let words = column_count.div_ceil(64);
    let mut basis: Vec<Option<Vec<u64>>> = vec![None; column_count];
    let mut rank = 0;
    let mut word_xors = 0;
    let mut nonempty_rows = 0;

    for state in &level.states {
        if state.entries.is_empty() {
            continue;
        }
        nonempty_rows += 1;
        let mut row = vec![0_u64; words];
        for entry in &state.entries {
            let column = columns[&entry.point];
            row[column / 64] |= 1_u64 << (column % 64);
        }
        loop {
            let pivot = row
                .iter()
                .enumerate()
                .rev()
                .find_map(|(word_index, &word)| {
                    (word != 0).then(|| word_index * 64 + (63 - word.leading_zeros() as usize))
                });
            let Some(pivot) = pivot else { break };
            if let Some(existing) = &basis[pivot] {
                for (value, &other) in row.iter_mut().zip(existing) {
                    *value ^= other;
                    word_xors += 1;
                }
            } else {
                basis[pivot] = Some(row);
                rank += 1;
                break;
            }
        }
    }

    RankMetrics {
        rows: level.states.len(),
        nonempty_rows,
        columns: column_count,
        rank,
        word_xors,
        entries,
    }
}

#[derive(Clone, Debug)]
struct QueryOutcome {
    found: bool,
    witness: Option<[u16; 4]>,
    probes: u64,
    matches: u64,
    field_ops: FpOps,
    group_additions: u64,
    replay_ok: bool,
}

fn sorted_witness(left: [u16; 2], right: [u16; 2]) -> [u16; 4] {
    let mut values = [left[0], left[1], right[0], right[1]];
    values.sort_unstable();
    values
}

fn replay_witness(base: &FactorBase, witness: [u16; 4], ops: &mut FpOps) -> Point {
    let mut sum = Point::INFINITY;
    for index in witness {
        sum = point_add(sum, base.leaves[index as usize].point, ops);
    }
    sum
}

fn query(root: &State, base: &FactorBase, target: Point) -> QueryOutcome {
    let mut field_ops = FpOps::default();
    let mut probes = 0;
    let mut matches = 0;
    let mut group_additions = 0;
    let mut best_witness = None;

    for entry in &root.entries {
        let complement = point_add(target, entry.point.negate(), &mut field_ops);
        group_additions += 1;
        probes += 1;
        if let Ok(index) = root
            .entries
            .binary_search_by_key(&complement, |candidate| candidate.point)
        {
            matches += 1;
            let candidate = sorted_witness(entry.witness, root.entries[index].witness);
            if best_witness.is_none_or(|old| candidate < old) {
                best_witness = Some(candidate);
            }
        }
    }

    let replay_ok = if let Some(witness) = best_witness {
        replay_witness(base, witness, &mut field_ops) == target
    } else {
        true
    };

    QueryOutcome {
        found: best_witness.is_some(),
        witness: best_witness,
        probes,
        matches,
        field_ops,
        group_additions,
        replay_ok,
    }
}

fn first_planted_target(base: &FactorBase) -> (Point, [u16; 4]) {
    let mut ops = FpOps::default();
    let count = base.leaves.len();
    for a in 0..count {
        for b in a..count {
            for c in b..count {
                for d in c..count {
                    let witness = [a as u16, b as u16, c as u16, d as u16];
                    let target = replay_witness(base, witness, &mut ops);
                    if !target.infinity {
                        return (target, witness);
                    }
                }
            }
        }
    }
    panic!("no finite planted four-leaf target");
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

fn witness_text(witness: Option<[u16; 4]>) -> String {
    witness.map_or_else(
        || "-".to_owned(),
        |w| format!("{},{},{},{}", w[0], w[1], w[2], w[3]),
    )
}

fn print_ops(prefix: &str, ops: FpOps) {
    print!(
        " {prefix}_add={} {prefix}_sub={} {prefix}_mul={} {prefix}_square={} {prefix}_inv={}",
        ops.adds, ops.subs, ops.muls, ops.squares, ops.inversions
    );
}

fn print_query(depth: usize, label: &str, target: Point, outcome: &QueryOutcome) {
    print!(
        "QUERY depth={depth} label={label} target_inf={} target_x={} target_y={} found={} probes={} matches={} group_additions={} replay_ok={} witness={}",
        u8::from(target.infinity),
        target.x,
        target.y,
        u8::from(outcome.found),
        outcome.probes,
        outcome.matches,
        outcome.group_additions,
        u8::from(outcome.replay_ok),
        witness_text(outcome.witness),
    );
    print_ops("query", outcome.field_ops);
    println!();
}

#[derive(Clone, Copy)]
struct ProjectionInput {
    setup: FpOps,
    miss: FpOps,
    miss_probes: u64,
    root_support: u64,
}

fn print_projection(input: ProjectionInput) {
    let retained_setup = FpOps {
        adds: 0,
        subs: 0,
        muls: 368_098,
        squares: 144_160,
        inversions: 1,
    };
    let projected = retained_setup
        .plus(input.setup)
        .plus(input.miss.scaled(M7_RESIDUAL_QUERIES));
    let optimistic_probes = input.root_support * M7_RESIDUAL_QUERIES;
    print!(
        "PROJECTION residual_queries={} root_support={} miss_probes={} optimistic_probes={}",
        M7_RESIDUAL_QUERIES, input.root_support, input.miss_probes, optimistic_probes
    );
    print_ops("projected", projected);
    for (index, (square_tenths, inversion_price)) in [(8_u64, 50_u64), (10, 100), (10, 200)]
        .into_iter()
        .enumerate()
    {
        let field_equivalent_tenths = 10 * projected.muls
            + square_tenths * projected.squares
            + 10 * inversion_price * projected.inversions;
        print!(
            " panel{}_square_tenths={} panel{}_inv_price={} panel{}_fe_tenths={}",
            index + 1,
            square_tenths,
            index + 1,
            inversion_price,
            index + 1,
            field_equivalent_tenths
        );
    }
    println!();
}

fn main() {
    assert!(on_curve(GENERATOR));
    let mut order_ops = FpOps::default();
    assert_eq!(
        scalar_mul(GENERATOR, GROUP_ORDER, &mut order_ops),
        Point::INFINITY
    );
    println!("SCHEMA isogeny-labs/pplus-oriented-pair-transducer/v1");
    println!("CURVE slug=icv1-fp31-tm3989-ae9db972 p={P} order={GROUP_ORDER}");

    let mut projection = None;
    for depth in 1..=6 {
        let base = factor_base(depth);
        assert!(
            !base.leaves.is_empty(),
            "empty signed factor base at depth {depth}"
        );
        let (levels, interner, counters, state_ops) = build_transducer(&base);
        let metrics: Vec<_> = levels.iter().map(incidence_rank).collect();
        let root = &levels.last().unwrap().states[0];
        let digest = support_digest(&root.entries);
        let actual_states: usize = levels.iter().map(|level| level.states.len()).sum();
        let actual_entries: usize = metrics.iter().map(|metric| metric.entries).sum();
        let peak_live_entries = metrics
            .iter()
            .map(|metric| metric.entries)
            .max()
            .unwrap_or(0);
        let total_rank_xors: u64 = metrics.iter().map(|metric| metric.word_xors).sum();
        let setup_ops = base.ops.plus(state_ops);

        print!(
            "DEPTH depth={} full_roots={} lifting_abscissae={} signed_leaves={} actual_states={} unique_states={} actual_entries={} semantic_entries={} actual_union_edges={} unique_union_edges={} root_support={} support_sha256={} leaf_rank_gf2={} max_rank_gf2={} peak_live_entries={} rank_word_xors={} point_additions={} map_probes={} collision_merges={} support_merges={}",
            depth,
            base.roots.len(),
            base.leaves.len() / 2,
            base.leaves.len(),
            actual_states,
            interner.ids.len(),
            actual_entries,
            interner.semantic_entries(),
            counters.actual_union_edges,
            interner.unique_edges(),
            root.entries.len(),
            digest,
            metrics[0].rank,
            metrics.iter().map(|metric| metric.rank).max().unwrap_or(0),
            peak_live_entries,
            total_rank_xors,
            counters.point_additions,
            counters.map_probes,
            counters.collision_merges,
            counters.support_merges,
        );
        print_ops("base", base.ops);
        print_ops("state", state_ops);
        print_ops("setup", setup_ops);
        println!();

        for (tree_level, metric) in metrics.iter().enumerate() {
            println!(
                "LEVEL depth={} tree_level={} node_count={} rows={} nonempty_rows={} columns={} rank_gf2={} entries={} rank_word_xors={}",
                depth,
                tree_level,
                levels[tree_level].node_count,
                metric.rows,
                metric.nonempty_rows,
                metric.columns,
                metric.rank,
                metric.entries,
                metric.word_xors,
            );
        }

        let (planted_target, planted_source) = first_planted_target(&base);
        let planted = query(root, &base, planted_target);
        assert!(planted.found && planted.replay_ok);
        print_query(depth, "planted", planted_target, &planted);
        println!(
            "PLANTED_SOURCE depth={} witness={}",
            depth,
            witness_text(Some(planted_source))
        );

        let mut first_miss = None;
        for scalar in 1..=8 {
            let mut target_ops = FpOps::default();
            let target = scalar_mul(GENERATOR, scalar, &mut target_ops);
            let outcome = query(root, &base, target);
            assert!(outcome.replay_ok);
            print_query(depth, &format!("g{scalar}"), target, &outcome);
            if !outcome.found && first_miss.is_none() {
                first_miss = Some(outcome.clone());
            }
        }

        let infinity = query(root, &base, Point::INFINITY);
        assert!(infinity.replay_ok);
        print_query(depth, "infinity", Point::INFINITY, &infinity);

        if depth == 6 {
            let miss = first_miss.expect("depth-six panel contains no exact miss");
            projection = Some(ProjectionInput {
                setup: setup_ops,
                miss: miss.field_ops,
                miss_probes: miss.probes,
                root_support: root.entries.len() as u64,
            });
        }
    }

    print_projection(projection.expect("missing depth-six projection"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_and_curve_controls() {
        let mut ops = FpOps::default();
        let value = 1_234_567;
        assert_eq!(fmul(value, finv(value, &mut ops), &mut ops), 1);
        assert!(on_curve(GENERATOR));
        assert_eq!(
            point_add(GENERATOR, GENERATOR.negate(), &mut ops),
            Point::INFINITY
        );
        assert_eq!(
            scalar_mul(GENERATOR, GROUP_ORDER, &mut ops),
            Point::INFINITY
        );
    }

    #[test]
    fn production_depth_six_cardinality_and_query() {
        let base = factor_base(6);
        assert_eq!(base.roots.len(), 64);
        assert_eq!(base.leaves.len(), 68);
        let (levels, _, _, _) = build_transducer(&base);
        let root = &levels.last().unwrap().states[0];
        let (target, _) = first_planted_target(&base);
        let outcome = query(root, &base, target);
        assert!(outcome.found && outcome.replay_ok);
    }

    #[test]
    fn sha256_control() {
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
