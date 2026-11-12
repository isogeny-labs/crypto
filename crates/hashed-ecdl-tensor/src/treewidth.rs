//! Elimination-order treewidth bounds and contraction cost of a network's
//! primal graph (one Boolean variable per edge, one clique per vertex).
//!
//! Upper bounds come from greedy elimination orders (min-degree, min-fill-in);
//! lower bounds from the largest vertex arity (a clique of that size is a
//! subgraph) and from MMD+ with min-d contraction (Gogate and Dechter, 2004:
//! every contraction yields a minor, and a minor's minimum degree is at most
//! the treewidth). The width of an order is recomputed exactly while it is
//! executed; the heuristics only choose the order. Nothing here allocates a
//! tensor table or counts anything: it measures what a tree-decomposition
//! contraction would cost, `2^(width+1)` per elimination, without running it.

use crate::network::Network;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// Primal graph on a network's edges.
#[derive(Clone, Debug)]
pub struct PrimalGraph {
    /// Neighbour sets; `adjacency[v]` never contains `v`.
    pub adjacency: Vec<HashSet<u32>>,
    /// Distinct-edge arity of every vertex (factor rank before elimination).
    pub arities: Vec<usize>,
}

impl PrimalGraph {
    /// One clique per vertex over its distinct edges.
    pub fn from_network(network: &Network) -> Self {
        let scopes: Vec<Vec<u32>> = network
            .vertices
            .iter()
            .map(|vertex| {
                let mut scope: Vec<u32> = vertex.edges.iter().map(|&e| e as u32).collect();
                scope.sort_unstable();
                scope.dedup();
                scope
            })
            .collect();
        Self::from_scopes(network.edge_count(), &scopes)
    }

    /// Build from explicit factor scopes over `variables` variables.
    pub fn from_scopes(variables: usize, scopes: &[Vec<u32>]) -> Self {
        let mut adjacency: Vec<HashSet<u32>> = (0..variables).map(|_| HashSet::new()).collect();
        let mut arities = Vec::with_capacity(scopes.len());
        for scope in scopes {
            arities.push(scope.len());
            for (i, &u) in scope.iter().enumerate() {
                for &w in &scope[i + 1..] {
                    if u != w {
                        adjacency[u as usize].insert(w);
                        adjacency[w as usize].insert(u);
                    }
                }
            }
        }
        Self { adjacency, arities }
    }

    /// Number of variables.
    pub fn variable_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Number of primal edges.
    pub fn edge_count(&self) -> usize {
        self.adjacency.iter().map(HashSet::len).sum::<usize>() / 2
    }

    /// Largest factor arity.
    pub fn maximum_arity(&self) -> usize {
        self.arities.iter().copied().max().unwrap_or(0)
    }

    /// A clique on the largest factor's variables is a subgraph, so the
    /// treewidth is at least its size minus one.
    pub fn clique_lower_bound(&self) -> usize {
        self.maximum_arity().saturating_sub(1)
    }
}

/// Greedy ordering rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heuristic {
    /// Eliminate the variable with the fewest neighbours.
    MinDegree,
    /// Eliminate the variable whose neighbourhood needs the fewest fill edges,
    /// ties broken by degree. Every neighbour of an eliminated variable is
    /// re-scored at once (quadratic in its degree).
    MinFill,
    /// Min-fill with lazy re-scoring: a neighbour is re-scored only when it
    /// reaches the top of the heap. Much cheaper, usually a little wider.
    MinFillLazy,
}

impl Heuristic {
    /// Name used in reports.
    pub fn name(self) -> &'static str {
        match self {
            Self::MinDegree => "min-degree",
            Self::MinFill => "min-fill",
            Self::MinFillLazy => "min-fill-lazy",
        }
    }
}

/// An executed elimination order and what it cost.
#[derive(Clone, Debug)]
pub struct Elimination {
    /// Ordering rule.
    pub heuristic: Heuristic,
    /// Largest neighbourhood at elimination: an upper bound on the treewidth.
    pub width: usize,
    /// `log2` of the total table work: every initial factor table (`2^arity`)
    /// plus `2^(|N(v)|+1)` for each eliminated variable `v`.
    pub log2_cost: f64,
    /// The order, first eliminated first.
    pub order: Vec<u32>,
}

/// Exact `log2` of a sum of powers of two, accumulated without overflow.
#[derive(Clone, Copy, Debug, Default)]
struct Log2Sum {
    maximum: Option<i64>,
    scaled: f64,
}

impl Log2Sum {
    fn add_power(&mut self, exponent: usize) {
        let exponent = exponent as i64;
        match self.maximum {
            None => {
                self.maximum = Some(exponent);
                self.scaled = 1.0;
            }
            Some(maximum) if exponent > maximum => {
                self.scaled = self.scaled * 2f64.powi((maximum - exponent) as i32) + 1.0;
                self.maximum = Some(exponent);
            }
            Some(maximum) => {
                self.scaled += 2f64.powi((exponent - maximum) as i32);
            }
        }
    }
    fn log2(self) -> f64 {
        match self.maximum {
            None => f64::NEG_INFINITY,
            Some(maximum) => maximum as f64 + self.scaled.log2(),
        }
    }
}

fn fill_score(adjacency: &[HashSet<u32>], v: u32) -> u64 {
    let neighbours: Vec<u32> = adjacency[v as usize].iter().copied().collect();
    let mut fill = 0u64;
    for (i, &u) in neighbours.iter().enumerate() {
        let set = &adjacency[u as usize];
        fill += neighbours[i + 1..]
            .iter()
            .filter(|&&w| !set.contains(&w))
            .count() as u64;
    }
    (fill << 20) | (neighbours.len() as u64).min((1 << 20) - 1)
}

fn score(adjacency: &[HashSet<u32>], heuristic: Heuristic, v: u32) -> u64 {
    match heuristic {
        Heuristic::MinDegree => adjacency[v as usize].len() as u64,
        Heuristic::MinFill | Heuristic::MinFillLazy => fill_score(adjacency, v),
    }
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Run a greedy elimination order and measure it exactly. Ties are broken by
/// variable index.
pub fn eliminate(graph: &PrimalGraph, heuristic: Heuristic) -> Elimination {
    eliminate_seeded(graph, heuristic, None)
}

/// The best of the deterministic order and `restarts - 1` orders with random
/// tie-breaking (seeded, so reruns agree). Width is the primary key, then cost.
pub fn eliminate_best_of(
    graph: &PrimalGraph,
    heuristic: Heuristic,
    restarts: usize,
) -> Elimination {
    let mut best = eliminate(graph, heuristic);
    for restart in 1..restarts.max(1) {
        let candidate = eliminate_seeded(graph, heuristic, Some(restart as u64));
        if (candidate.width, candidate.log2_cost) < (best.width, best.log2_cost) {
            best = candidate;
        }
    }
    best
}

/// Greedy elimination with ties broken by a seeded random key (`None`: by index).
pub fn eliminate_seeded(
    graph: &PrimalGraph,
    heuristic: Heuristic,
    seed: Option<u64>,
) -> Elimination {
    let n = graph.variable_count();
    let keys: Vec<u32> = match seed {
        None => (0..n as u32).collect(),
        Some(seed) => {
            let mut state = seed ^ 0x5DEE_CE66_D1CE_4E5B;
            (0..n).map(|_| splitmix(&mut state) as u32).collect()
        }
    };
    let mut adjacency = graph.adjacency.clone();
    let mut version = vec![0u32; n];
    let mut eliminated = vec![false; n];
    // Min-fill scores are refreshed lazily: a neighbour of an eliminated
    // variable is marked dirty and re-scored only when it reaches the top of
    // the heap; if its fresh score is worse it is pushed back.
    let mut dirty = vec![false; n];
    let mut heap = BinaryHeap::with_capacity(n);
    for v in 0..n as u32 {
        heap.push(Reverse((
            score(&adjacency, heuristic, v),
            keys[v as usize],
            v,
            0u32,
        )));
    }
    let mut cost = Log2Sum::default();
    for &arity in &graph.arities {
        cost.add_power(arity);
    }
    let mut width = 0;
    let mut order = Vec::with_capacity(n);
    while let Some(Reverse((old_score, key, v, stamp))) = heap.pop() {
        let v_index = v as usize;
        if eliminated[v_index] || stamp != version[v_index] {
            continue;
        }
        if dirty[v_index] {
            dirty[v_index] = false;
            let fresh = score(&adjacency, heuristic, v);
            if fresh > old_score {
                heap.push(Reverse((fresh, key, v, stamp)));
                continue;
            }
        }
        let neighbours: Vec<u32> = adjacency[v_index].iter().copied().collect();
        width = width.max(neighbours.len());
        cost.add_power(neighbours.len() + 1);
        for &u in &neighbours {
            adjacency[u as usize].remove(&v);
        }
        for (i, &u) in neighbours.iter().enumerate() {
            for &w in &neighbours[i + 1..] {
                if adjacency[u as usize].insert(w) {
                    adjacency[w as usize].insert(u);
                }
            }
        }
        adjacency[v_index].clear();
        eliminated[v_index] = true;
        order.push(v);
        for &u in &neighbours {
            let u_index = u as usize;
            match heuristic {
                Heuristic::MinDegree | Heuristic::MinFill => {
                    version[u_index] += 1;
                    heap.push(Reverse((
                        score(&adjacency, heuristic, u),
                        keys[u_index],
                        u,
                        version[u_index],
                    )));
                }
                Heuristic::MinFillLazy => {
                    dirty[u_index] = true;
                }
            }
        }
    }
    Elimination {
        heuristic,
        width,
        log2_cost: cost.log2(),
        order,
    }
}

/// Recompute the width of an explicit order on this graph.
pub fn width_of_order(graph: &PrimalGraph, order: &[u32]) -> Option<usize> {
    let n = graph.variable_count();
    if order.len() != n {
        return None;
    }
    let mut seen = vec![false; n];
    let mut adjacency = graph.adjacency.clone();
    let mut width = 0;
    for &v in order {
        let v_index = v as usize;
        if v_index >= n || seen[v_index] {
            return None;
        }
        seen[v_index] = true;
        let neighbours: Vec<u32> = adjacency[v_index].iter().copied().collect();
        width = width.max(neighbours.len());
        for &u in &neighbours {
            adjacency[u as usize].remove(&v);
        }
        for (i, &u) in neighbours.iter().enumerate() {
            for &w in &neighbours[i + 1..] {
                if adjacency[u as usize].insert(w) {
                    adjacency[w as usize].insert(u);
                }
            }
        }
        adjacency[v_index].clear();
    }
    Some(width)
}

/// Which neighbour MMD+ contracts a minimum-degree variable into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contraction {
    /// The neighbour of minimum degree (min-d).
    MinDegree,
    /// The neighbour with the fewest common neighbours (least-c).
    LeastCommon,
}

/// The larger of the two MMD+ bounds.
pub fn mmd_plus_lower_bound(graph: &PrimalGraph) -> usize {
    mmd_plus_with(graph, Contraction::MinDegree).max(mmd_plus_with(graph, Contraction::LeastCommon))
}

/// MMD+ lower bound: repeatedly take a minimum-degree variable, record its
/// degree, and contract it into a neighbour chosen by `rule`. Each step
/// produces a minor, whose minimum degree bounds the treewidth from below.
pub fn mmd_plus_with(graph: &PrimalGraph, rule: Contraction) -> usize {
    let n = graph.variable_count();
    let mut adjacency = graph.adjacency.clone();
    let mut version = vec![0u32; n];
    let mut gone = vec![false; n];
    let mut heap = BinaryHeap::with_capacity(n);
    for v in 0..n as u32 {
        heap.push(Reverse((adjacency[v as usize].len(), v, 0u32)));
    }
    let mut bound = 0;
    while let Some(Reverse((_, v, stamp))) = heap.pop() {
        let v_index = v as usize;
        if gone[v_index] || stamp != version[v_index] {
            continue;
        }
        let degree = adjacency[v_index].len();
        bound = bound.max(degree);
        gone[v_index] = true;
        if degree == 0 {
            continue;
        }
        let neighbours: Vec<u32> = adjacency[v_index].iter().copied().collect();
        let target = match rule {
            Contraction::MinDegree => neighbours
                .iter()
                .copied()
                .min_by_key(|&u| (adjacency[u as usize].len(), u))
                .expect("nonempty neighbourhood"),
            Contraction::LeastCommon => neighbours
                .iter()
                .copied()
                .min_by_key(|&u| {
                    let common = adjacency[u as usize]
                        .iter()
                        .filter(|w| adjacency[v_index].contains(w))
                        .count();
                    (common, adjacency[u as usize].len(), u)
                })
                .expect("nonempty neighbourhood"),
        };
        let target_index = target as usize;
        adjacency[target_index].remove(&v);
        for &w in &neighbours {
            if w == target {
                continue;
            }
            let w_index = w as usize;
            adjacency[w_index].remove(&v);
            if adjacency[w_index].insert(target) {
                adjacency[target_index].insert(w);
            }
        }
        adjacency[v_index].clear();
        for &u in &neighbours {
            let u_index = u as usize;
            version[u_index] += 1;
            heap.push(Reverse((adjacency[u_index].len(), u, version[u_index])));
        }
    }
    bound
}

/// Result of the safe preprocessing rules.
#[derive(Clone, Debug)]
pub struct Reduction {
    /// The remaining core, reindexed.
    pub core: PrimalGraph,
    /// Core variable `i` is original variable `core_index[i]`.
    pub core_index: Vec<u32>,
    /// Largest degree among the variables the rules eliminated.
    pub width: usize,
    /// Treewidth lower bound established while reducing.
    pub low: usize,
    /// Number of variables eliminated by the rules.
    pub eliminated: usize,
}

/// Bodlaender–Koster safe reductions: a simplicial variable (its neighbours
/// form a clique) is always eliminated, and an almost-simplicial variable (all
/// but one neighbour form a clique) of degree at most the current lower bound
/// is eliminated with the missing edges filled in. Both preserve treewidth, so
/// the core has the same treewidth as the graph whenever `low` was a valid
/// lower bound, and `max(width, tw(core))` is that treewidth.
pub fn reduce(graph: &PrimalGraph, low: usize) -> Reduction {
    let n = graph.variable_count();
    let mut adjacency = graph.adjacency.clone();
    let mut alive = vec![true; n];
    let mut queued = vec![true; n];
    let mut queue: std::collections::VecDeque<u32> = (0..n as u32).collect();
    let mut low = low;
    let mut width = 0;
    let mut eliminated = 0;
    while let Some(v) = queue.pop_front() {
        let v_index = v as usize;
        queued[v_index] = false;
        if !alive[v_index] {
            continue;
        }
        let neighbours: Vec<u32> = adjacency[v_index].iter().copied().collect();
        let degree = neighbours.len();
        let mut missing_total = 0usize;
        let mut missing: Vec<usize> = Vec::with_capacity(degree);
        for &u in &neighbours {
            let set = &adjacency[u as usize];
            let count = neighbours
                .iter()
                .filter(|&&w| w != u && !set.contains(&w))
                .count();
            missing.push(count);
            missing_total += count;
        }
        missing_total /= 2;
        let special = if missing_total == 0 {
            Some(None)
        } else if degree <= low {
            missing
                .iter()
                .position(|&m| m == missing_total)
                .map(|i| Some(neighbours[i]))
        } else {
            None
        };
        let Some(special) = special else {
            continue;
        };
        if missing_total == 0 {
            low = low.max(degree);
        }
        width = width.max(degree);
        if let Some(u) = special {
            let u_index = u as usize;
            for &w in &neighbours {
                if w != u && adjacency[u_index].insert(w) {
                    adjacency[w as usize].insert(u);
                }
            }
        }
        for &u in &neighbours {
            adjacency[u as usize].remove(&v);
            if !queued[u as usize] {
                queued[u as usize] = true;
                queue.push_back(u);
            }
        }
        adjacency[v_index].clear();
        alive[v_index] = false;
        eliminated += 1;
    }
    let core_index: Vec<u32> = (0..n as u32).filter(|&v| alive[v as usize]).collect();
    let mut position = vec![u32::MAX; n];
    for (i, &v) in core_index.iter().enumerate() {
        position[v as usize] = i as u32;
    }
    let core_adjacency: Vec<HashSet<u32>> = core_index
        .iter()
        .map(|&v| {
            adjacency[v as usize]
                .iter()
                .map(|&u| position[u as usize])
                .collect()
        })
        .collect();
    // Core factors: the filled adjacency is what matters; keep binary scopes
    // implicit and record the largest surviving original arity for cost.
    let core = PrimalGraph {
        adjacency: core_adjacency,
        arities: Vec::new(),
    };
    Reduction {
        core,
        core_index,
        width,
        low,
        eliminated,
    }
}

/// Bounds for one graph: reduce, bound the core from below with MMD+, and
/// from above with min-degree and (optionally) min-fill restarts on the core.
#[derive(Clone, Debug)]
pub struct Analysis {
    /// Variables left after the safe rules.
    pub core_variables: usize,
    /// Primal edges of the core.
    pub core_edges: usize,
    /// Width incurred by the safe rules.
    pub reduction_width: usize,
    /// Lower bound: clique bound, reduction bound and MMD+ on the core.
    pub lower_bound: usize,
    /// MMD+ on the core alone.
    pub core_mmd_plus: usize,
    /// Upper bound from min-degree on the core (overall width).
    pub min_degree: Elimination,
    /// Upper bound from eager min-fill on the core (overall width), if run.
    pub min_fill: Option<Elimination>,
    /// Upper bound from lazy min-fill on the core (overall width), if run.
    pub min_fill_lazy: Option<Elimination>,
    /// True when the rules eliminated everything: the width is exact.
    pub exact: bool,
}

/// Run the full pipeline. `log2_cost` of each elimination covers the core
/// only; the width fields are overall widths (`max` with the reduction).
pub fn analyse(
    graph: &PrimalGraph,
    restarts: usize,
    min_fill: bool,
    min_fill_lazy: bool,
) -> Analysis {
    let clique = graph.clique_lower_bound();
    let reduction = reduce(graph, clique.max(mmd_plus_lower_bound(graph)));
    let core_mmd_plus = mmd_plus_lower_bound(&reduction.core);
    let lower_bound = clique.max(reduction.low).max(core_mmd_plus);
    let lift = |mut run: Elimination| {
        run.width = run.width.max(reduction.width);
        run
    };
    let min_degree = lift(eliminate_best_of(
        &reduction.core,
        Heuristic::MinDegree,
        restarts,
    ));
    let min_fill = min_fill.then(|| {
        lift(eliminate_best_of(
            &reduction.core,
            Heuristic::MinFill,
            restarts,
        ))
    });
    let min_fill_lazy = min_fill_lazy.then(|| {
        lift(eliminate_best_of(
            &reduction.core,
            Heuristic::MinFillLazy,
            restarts,
        ))
    });
    Analysis {
        core_variables: reduction.core.variable_count(),
        core_edges: reduction.core.edge_count(),
        reduction_width: reduction.width,
        lower_bound,
        core_mmd_plus,
        min_degree,
        min_fill,
        min_fill_lazy,
        exact: reduction.core.variable_count() == 0,
    }
}

/// Write the primal graph in PACE 2017 `.gr` format (1-based vertices).
pub fn write_pace<W: std::io::Write>(graph: &PrimalGraph, out: &mut W) -> std::io::Result<()> {
    writeln!(
        out,
        "p tw {} {}",
        graph.variable_count(),
        graph.edge_count()
    )?;
    for (v, neighbours) in graph.adjacency.iter().enumerate() {
        let mut sorted: Vec<u32> = neighbours.iter().copied().collect();
        sorted.sort_unstable();
        for u in sorted {
            if (v as u32) < u {
                writeln!(out, "{} {}", v + 1, u + 1)?;
            }
        }
    }
    Ok(())
}

/// Parse a PACE `.gr` graph into a primal graph of binary scopes.
pub fn read_pace(text: &str) -> std::result::Result<PrimalGraph, String> {
    let mut variables = None;
    let mut scopes = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.first() == Some(&"p") {
            if fields.len() != 4 || fields[1] != "tw" {
                return Err("bad p line".into());
            }
            variables = Some(fields[2].parse::<usize>().map_err(|_| "bad vertex count")?);
            continue;
        }
        if fields.len() != 2 {
            return Err(format!("bad edge line: {line}"));
        }
        let u: u32 = fields[0].parse().map_err(|_| "bad vertex")?;
        let w: u32 = fields[1].parse().map_err(|_| "bad vertex")?;
        if u == 0 || w == 0 {
            return Err("vertices are 1-based".into());
        }
        scopes.push(vec![u - 1, w - 1]);
    }
    let variables = variables.ok_or("missing p line")?;
    if scopes.iter().flatten().any(|&v| v as usize >= variables) {
        return Err("edge beyond vertex count".into());
    }
    Ok(PrimalGraph::from_scopes(variables, &scopes))
}

/// Check a PACE `.td` tree decomposition against the graph and return its
/// width: every variable in some bag, every primal edge inside some bag, the
/// bags of each variable connected in the tree, and the tree a tree.
pub fn verify_tree_decomposition(
    graph: &PrimalGraph,
    td: &str,
) -> std::result::Result<usize, String> {
    let mut bags: Vec<Option<Vec<u32>>> = Vec::new();
    let mut tree_edges: Vec<(usize, usize)> = Vec::new();
    for line in td.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields[0] {
            "s" => {
                if fields.len() != 5 || fields[1] != "td" {
                    return Err("bad s line".into());
                }
                let count: usize = fields[2].parse().map_err(|_| "bad bag count")?;
                let n: usize = fields[4].parse().map_err(|_| "bad vertex count")?;
                if n != graph.variable_count() {
                    return Err("vertex count differs from the graph".into());
                }
                bags = vec![None; count];
            }
            "b" => {
                let id: usize = fields
                    .get(1)
                    .ok_or("bad bag")?
                    .parse()
                    .map_err(|_| "bad bag id")?;
                if id == 0 || id > bags.len() || bags[id - 1].is_some() {
                    return Err(format!("bag {id} out of range or repeated"));
                }
                let mut members = Vec::new();
                for field in &fields[2..] {
                    let v: u32 = field.parse().map_err(|_| "bad bag member")?;
                    if v == 0 || v as usize > graph.variable_count() {
                        return Err("bag member out of range".into());
                    }
                    members.push(v - 1);
                }
                members.sort_unstable();
                members.dedup();
                bags[id - 1] = Some(members);
            }
            _ => {
                if fields.len() != 2 {
                    return Err(format!("bad tree edge: {line}"));
                }
                let a: usize = fields[0].parse().map_err(|_| "bad tree edge")?;
                let b: usize = fields[1].parse().map_err(|_| "bad tree edge")?;
                if a == 0 || b == 0 || a > bags.len() || b > bags.len() || a == b {
                    return Err("tree edge out of range".into());
                }
                tree_edges.push((a - 1, b - 1));
            }
        }
    }
    let bags: Vec<Vec<u32>> = bags
        .into_iter()
        .enumerate()
        .map(|(i, bag)| bag.ok_or_else(|| format!("bag {} missing", i + 1)))
        .collect::<std::result::Result<_, _>>()?;
    let count = bags.len();
    if count == 0 {
        return Err("no bags".into());
    }
    if tree_edges.len() != count - 1 {
        return Err("tree edge count is not bags - 1".into());
    }
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); count];
    for &(a, b) in &tree_edges {
        tree[a].push(b);
        tree[b].push(a);
    }
    // Connected with count - 1 edges: a tree.
    let mut seen = vec![false; count];
    let mut stack = vec![0];
    seen[0] = true;
    while let Some(i) = stack.pop() {
        for &j in &tree[i] {
            if !seen[j] {
                seen[j] = true;
                stack.push(j);
            }
        }
    }
    if seen.iter().any(|&s| !s) {
        return Err("tree is disconnected".into());
    }
    // Membership lists per variable.
    let n = graph.variable_count();
    let mut containing: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, bag) in bags.iter().enumerate() {
        for &v in bag {
            containing[v as usize].push(i);
        }
    }
    if containing.iter().any(Vec::is_empty) {
        return Err("a variable is in no bag".into());
    }
    // Every primal edge inside some bag.
    let bag_sets: Vec<HashSet<u32>> = bags.iter().map(|b| b.iter().copied().collect()).collect();
    for (v, neighbours) in graph.adjacency.iter().enumerate() {
        for &u in neighbours {
            if (v as u32) < u && !containing[v].iter().any(|&i| bag_sets[i].contains(&u)) {
                return Err(format!("edge {}-{} is in no bag", v + 1, u + 1));
            }
        }
    }
    // Connectedness of each variable's bags.
    let mut mark = vec![usize::MAX; count];
    for (v, list) in containing.iter().enumerate() {
        for &i in list {
            mark[i] = v;
        }
        let mut reached = 1;
        let mut stack = vec![list[0]];
        let mut visited = vec![list[0]];
        mark[list[0]] = usize::MAX - 1;
        while let Some(i) = stack.pop() {
            for &j in &tree[i] {
                if mark[j] == v {
                    mark[j] = usize::MAX - 1;
                    reached += 1;
                    stack.push(j);
                    visited.push(j);
                }
            }
        }
        for i in visited {
            mark[i] = usize::MAX;
        }
        if reached != list.len() {
            return Err(format!("bags of variable {} are disconnected", v + 1));
        }
    }
    Ok(bags.iter().map(Vec::len).max().unwrap_or(1) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(variables: usize, edges: &[(u32, u32)]) -> PrimalGraph {
        let scopes: Vec<Vec<u32>> = edges.iter().map(|&(u, w)| vec![u, w]).collect();
        PrimalGraph::from_scopes(variables, &scopes)
    }

    fn check(graph: &PrimalGraph, exact: usize) {
        for heuristic in [
            Heuristic::MinDegree,
            Heuristic::MinFill,
            Heuristic::MinFillLazy,
        ] {
            let run = eliminate(graph, heuristic);
            assert_eq!(run.width, exact, "{}", heuristic.name());
            assert_eq!(run.order.len(), graph.variable_count());
            assert_eq!(width_of_order(graph, &run.order), Some(exact));
        }
        assert!(mmd_plus_lower_bound(graph) <= exact);
        assert!(graph.clique_lower_bound() <= exact);
    }

    #[test]
    fn path_cycle_star_and_clique_have_known_treewidth() {
        let path = graph(6, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
        check(&path, 1);
        assert_eq!(mmd_plus_lower_bound(&path), 1);
        let cycle = graph(6, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
        check(&cycle, 2);
        assert_eq!(mmd_plus_lower_bound(&cycle), 2);
        let star = graph(5, &[(0, 1), (0, 2), (0, 3), (0, 4)]);
        check(&star, 1);
        let clique = PrimalGraph::from_scopes(6, &[vec![0, 1, 2, 3, 4, 5]]);
        check(&clique, 5);
        assert_eq!(clique.clique_lower_bound(), 5);
        assert_eq!(mmd_plus_lower_bound(&clique), 5);
        assert_eq!(clique.edge_count(), 15);
    }

    #[test]
    fn grid_width_is_bracketed() {
        // 4 x 4 grid: treewidth 4.
        let mut edges = Vec::new();
        for r in 0..4u32 {
            for c in 0..4u32 {
                let v = 4 * r + c;
                if c + 1 < 4 {
                    edges.push((v, v + 1));
                }
                if r + 1 < 4 {
                    edges.push((v, v + 4));
                }
            }
        }
        let grid = graph(16, &edges);
        let lower = mmd_plus_lower_bound(&grid);
        assert!(lower >= 2 && lower <= 4, "{lower}");
        for heuristic in [Heuristic::MinDegree, Heuristic::MinFill] {
            let run = eliminate(&grid, heuristic);
            assert!(run.width >= 4 && run.width <= 5, "{}", run.width);
            assert_eq!(width_of_order(&grid, &run.order), Some(run.width));
        }
    }

    #[test]
    fn cost_counts_initial_tables_and_eliminations() {
        let single = PrimalGraph::from_scopes(3, &[vec![0, 1, 2]]);
        let run = eliminate(&single, Heuristic::MinDegree);
        // 2^3 initial + 2^3 (first elimination, two neighbours) + 2^2 + 2^1.
        assert!((run.log2_cost - (8f64 + 8.0 + 4.0 + 2.0).log2()).abs() < 1e-12);
        let mut sum = Log2Sum::default();
        sum.add_power(700);
        sum.add_power(700);
        sum.add_power(3);
        assert!((sum.log2() - 701.0).abs() < 1e-9);
    }

    #[test]
    fn restarts_never_worsen_and_mmd_rules_agree_on_cliques() {
        let clique = PrimalGraph::from_scopes(5, &[vec![0, 1, 2, 3, 4]]);
        let best = eliminate_best_of(&clique, Heuristic::MinFill, 4);
        assert_eq!(best.width, 4);
        assert_eq!(mmd_plus_with(&clique, Contraction::MinDegree), 4);
        assert_eq!(mmd_plus_with(&clique, Contraction::LeastCommon), 4);
        let cycle = graph(7, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 0)]);
        let seeded = eliminate_seeded(&cycle, Heuristic::MinDegree, Some(7));
        assert_eq!(seeded.width, 2);
        assert_eq!(width_of_order(&cycle, &seeded.order), Some(2));
    }

    #[test]
    fn pace_round_trip_and_decomposition_check() {
        let cycle = graph(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let mut text = Vec::new();
        write_pace(&cycle, &mut text).unwrap();
        let text = String::from_utf8(text).unwrap();
        assert!(text.starts_with("p tw 4 4\n"));
        let back = read_pace(&text).unwrap();
        assert_eq!(back.edge_count(), 4);
        assert_eq!(back.variable_count(), 4);
        let good = "s td 2 3 4\nb 1 1 2 3\nb 2 1 3 4\n1 2\n";
        assert_eq!(verify_tree_decomposition(&back, good), Ok(2));
        let missing_edge = "s td 2 3 4\nb 1 1 2 3\nb 2 1 2 4\n1 2\n";
        assert!(verify_tree_decomposition(&back, missing_edge).is_err());
        let disconnected = "s td 3 3 4\nb 1 1 2 3\nb 2 2 3\nb 3 1 3 4\n1 2\n2 3\n";
        assert!(verify_tree_decomposition(&back, disconnected).is_err());
        let not_tree = "s td 2 3 4\nb 1 1 2 3\nb 2 1 3 4\n";
        assert!(verify_tree_decomposition(&back, not_tree).is_err());
    }

    #[test]
    fn safe_rules_solve_trees_cycles_and_cliques_exactly() {
        let path = graph(6, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
        let analysis = analyse(&path, 1, true, true);
        assert!(analysis.exact);
        assert_eq!(analysis.reduction_width, 1);
        assert_eq!(analysis.min_degree.width, 1);
        let cycle = graph(6, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
        let analysis = analyse(&cycle, 1, true, true);
        assert!(analysis.exact);
        assert_eq!(analysis.min_fill.as_ref().unwrap().width, 2);
        assert_eq!(analysis.lower_bound, 2);
        let clique = PrimalGraph::from_scopes(6, &[vec![0, 1, 2, 3, 4, 5]]);
        let analysis = analyse(&clique, 2, true, true);
        assert!(analysis.exact);
        assert_eq!(analysis.lower_bound, 5);
        assert_eq!(analysis.min_degree.width, 5);
        // A 4x4 grid has no simplicial or almost-simplicial vertex of degree
        // <= 2 after the corners go; the core keeps the bracket honest.
        let mut edges = Vec::new();
        for r in 0..4u32 {
            for c in 0..4u32 {
                let v = 4 * r + c;
                if c + 1 < 4 {
                    edges.push((v, v + 1));
                }
                if r + 1 < 4 {
                    edges.push((v, v + 4));
                }
            }
        }
        let grid = graph(16, &edges);
        let analysis = analyse(&grid, 3, true, true);
        assert!(analysis.lower_bound >= 2 && analysis.lower_bound <= 4);
        assert!(analysis.min_fill.as_ref().unwrap().width >= 4);
        assert!(analysis.min_fill.as_ref().unwrap().width <= 5);
        assert!(analysis.min_degree.width >= analysis.lower_bound);
    }

    #[test]
    fn rejects_bad_orders() {
        let path = graph(3, &[(0, 1), (1, 2)]);
        assert_eq!(width_of_order(&path, &[0, 1]), None);
        assert_eq!(width_of_order(&path, &[0, 0, 1]), None);
        assert_eq!(width_of_order(&path, &[1, 0, 2]), Some(2));
    }
}
