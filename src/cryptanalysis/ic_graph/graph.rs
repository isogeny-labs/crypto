//! Report section 1: connectivity and cycle handling on ordinary graphs.
//!
//! Undirected multigraphs with stable edge ids. In a double-large-prime graph
//! a vertex is a large prime (or the "1" vertex) and an edge is a partial
//! relation. Everything here is topology only: section 5 of the report shows
//! that topological cycles need not cancel modular coefficients, so these
//! routines never certify a factor-base dependency on their own.
//!
//! | routine | bound | literature |
//! |---|---|---|
//! | [`Graph::bfs`], [`Graph::dfs_preorder`], [`Graph::components`], [`Graph::spanning_forest`] | `O(v+e)` | Tarjan 1972 |
//! | [`UnionFind`] | `O((v+e) alpha(v))` amortized | Tarjan 1975 |
//! | [`fundamental_cycle_basis`] | `O(v+e+L)` | forest + chords |
//! | [`biconnectivity`] (bridges, articulation points, blocks) | `O(v+e)` | Tarjan 1972 |
//! | [`core_numbers`] | `O(v+e)` | Batagelj-Zaversnik |
//! | [`low_stretch_spanning_tree`] | `O(e log v)` | see function doc |
//! | [`minimum_cycle_basis_gf2`] | polynomial | Horton candidates, exact over GF(2) |

use std::collections::VecDeque;

/// Undirected multigraph. Edge `i` joins `edges[i].0` and `edges[i].1`;
/// self-loops are allowed and are cycles of length one.
#[derive(Clone, Debug, Default)]
pub struct Graph {
    n: usize,
    edges: Vec<(usize, usize)>,
    adj: Vec<Vec<(usize, usize)>>, // (neighbour, edge id)
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: Vec::new(),
            adj: vec![Vec::new(); n],
        }
    }
    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = Self::new(n);
        for &(u, v) in edges {
            g.add_edge(u, v);
        }
        g
    }
    /// Add an edge and return its id.
    pub fn add_edge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.n && v < self.n, "edge endpoint out of range");
        let id = self.edges.len();
        self.edges.push((u, v));
        self.adj[u].push((v, id));
        if u != v {
            self.adj[v].push((u, id));
        }
        id
    }
    pub fn vertex_count(&self) -> usize {
        self.n
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
    pub fn neighbors(&self, u: usize) -> &[(usize, usize)] {
        &self.adj[u]
    }
    pub fn degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }

    /// Breadth-first search from `s`: `(dist, parent_edge)`; unreachable
    /// vertices have `dist = usize::MAX`.
    pub fn bfs(&self, s: usize) -> (Vec<usize>, Vec<Option<usize>>) {
        let mut dist = vec![usize::MAX; self.n];
        let mut pe = vec![None; self.n];
        let mut q = VecDeque::new();
        dist[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &(v, e) in &self.adj[u] {
                if dist[v] == usize::MAX {
                    dist[v] = dist[u] + 1;
                    pe[v] = Some(e);
                    q.push_back(v);
                }
            }
        }
        (dist, pe)
    }

    /// Iterative depth-first preorder of every vertex (all components).
    pub fn dfs_preorder(&self) -> Vec<usize> {
        let mut seen = vec![false; self.n];
        let mut order = Vec::with_capacity(self.n);
        let mut stack = Vec::new();
        for s in 0..self.n {
            if seen[s] {
                continue;
            }
            stack.push(s);
            while let Some(u) = stack.pop() {
                if seen[u] {
                    continue;
                }
                seen[u] = true;
                order.push(u);
                for &(v, _) in self.adj[u].iter().rev() {
                    if !seen[v] {
                        stack.push(v);
                    }
                }
            }
        }
        order
    }

    /// Connected-component label per vertex and the number of components.
    pub fn components(&self) -> (Vec<usize>, usize) {
        let mut comp = vec![usize::MAX; self.n];
        let mut c = 0;
        let mut q = VecDeque::new();
        for s in 0..self.n {
            if comp[s] != usize::MAX {
                continue;
            }
            comp[s] = c;
            q.push_back(s);
            while let Some(u) = q.pop_front() {
                for &(v, _) in &self.adj[u] {
                    if comp[v] == usize::MAX {
                        comp[v] = c;
                        q.push_back(v);
                    }
                }
            }
            c += 1;
        }
        (comp, c)
    }

    /// BFS spanning forest: `parent_edge[v]` (None at roots) and BFS depth.
    pub fn spanning_forest(&self) -> SpanningForest {
        let mut depth = vec![usize::MAX; self.n];
        let mut parent_edge = vec![None; self.n];
        let mut q = VecDeque::new();
        for s in 0..self.n {
            if depth[s] != usize::MAX {
                continue;
            }
            depth[s] = 0;
            q.push_back(s);
            while let Some(u) = q.pop_front() {
                for &(v, e) in &self.adj[u] {
                    if depth[v] == usize::MAX {
                        depth[v] = depth[u] + 1;
                        parent_edge[v] = Some(e);
                        q.push_back(v);
                    }
                }
            }
        }
        SpanningForest { parent_edge, depth }
    }
}

/// A rooted spanning forest of a [`Graph`].
#[derive(Clone, Debug)]
pub struct SpanningForest {
    pub parent_edge: Vec<Option<usize>>,
    pub depth: Vec<usize>,
}

impl SpanningForest {
    pub fn is_tree_edge(&self, g: &Graph, e: usize) -> bool {
        let (u, v) = g.edges[e];
        self.parent_edge[u] == Some(e) || self.parent_edge[v] == Some(e)
    }
    fn parent(&self, g: &Graph, v: usize) -> Option<usize> {
        self.parent_edge[v].map(|e| {
            let (a, b) = g.edges[e];
            if a == v {
                b
            } else {
                a
            }
        })
    }
    /// Tree path between `u` and `v` as edge ids (`None` if in different trees).
    pub fn tree_path(&self, g: &Graph, mut u: usize, mut v: usize) -> Option<Vec<usize>> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        while self.depth[u] > self.depth[v] {
            left.push(self.parent_edge[u]?);
            u = self.parent(g, u)?;
        }
        while self.depth[v] > self.depth[u] {
            right.push(self.parent_edge[v]?);
            v = self.parent(g, v)?;
        }
        while u != v {
            left.push(self.parent_edge[u]?);
            right.push(self.parent_edge[v]?);
            u = self.parent(g, u)?;
            v = self.parent(g, v)?;
        }
        right.reverse();
        left.extend(right);
        Some(left)
    }
}

/// Disjoint-set union with path compression (path halving) and union by size.
#[derive(Clone, Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    sets: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            sets: n,
        }
    }
    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
    /// Union; returns `false` when already connected (a topological cycle
    /// closes, which says nothing about coefficient cancellation).
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.sets -= 1;
        true
    }
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    pub fn set_count(&self) -> usize {
        self.sets
    }
    pub fn set_size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        self.size[r]
    }
}

/// A cycle given as edge ids in traversal order.
pub type Cycle = Vec<usize>;

/// Fundamental cycle basis of a BFS spanning forest: one cycle per chord.
/// The basis has `e - v + c` cycles; materializing it costs `O(v + e + L)`
/// where `L` is the total cycle length returned.
pub fn fundamental_cycle_basis(g: &Graph) -> Vec<Cycle> {
    let forest = g.spanning_forest();
    fundamental_cycles_of(g, &forest)
}

/// Fundamental cycles relative to a supplied spanning forest.
pub fn fundamental_cycles_of(g: &Graph, forest: &SpanningForest) -> Vec<Cycle> {
    let mut out = Vec::new();
    for (e, &(u, v)) in g.edges.iter().enumerate() {
        if forest.is_tree_edge(g, e) {
            continue;
        }
        let mut cyc = forest
            .tree_path(g, v, u)
            .expect("chord endpoints lie in one tree");
        cyc.insert(0, e);
        out.push(cyc);
    }
    out
}

/// Bridges, articulation points and biconnected blocks (edge partition).
#[derive(Clone, Debug, Default)]
pub struct Biconnectivity {
    pub bridges: Vec<usize>,
    pub articulation_points: Vec<usize>,
    /// Each block is a set of edge ids; bridges are singleton blocks.
    pub blocks: Vec<Vec<usize>>,
}

/// Iterative Tarjan low-link biconnectivity in `O(v + e)`.
pub fn biconnectivity(g: &Graph) -> Biconnectivity {
    let n = g.n;
    let mut disc = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut is_art = vec![false; n];
    let mut out = Biconnectivity::default();
    let mut timer = 0;
    let mut edge_stack: Vec<usize> = Vec::new();
    // frame: (vertex, parent edge, next adjacency index, child count)
    let mut stack: Vec<(usize, Option<usize>, usize, usize)> = Vec::new();
    for root in 0..n {
        if disc[root] != usize::MAX {
            continue;
        }
        disc[root] = timer;
        low[root] = timer;
        timer += 1;
        stack.push((root, None, 0, 0));
        while let Some(&mut (u, pe, ref mut idx, _)) = stack.last_mut() {
            if *idx < g.adj[u].len() {
                let (v, e) = g.adj[u][*idx];
                *idx += 1;
                if Some(e) == pe {
                    continue;
                }
                if u == v {
                    out.blocks.push(vec![e]); // self-loop is its own block
                    continue;
                }
                if disc[v] == usize::MAX {
                    edge_stack.push(e);
                    disc[v] = timer;
                    low[v] = timer;
                    timer += 1;
                    stack.last_mut().unwrap().3 += 1;
                    stack.push((v, Some(e), 0, 0));
                } else if disc[v] < disc[u] {
                    edge_stack.push(e);
                    low[u] = low[u].min(disc[v]);
                }
            } else {
                let (u, pe, _, _) = stack.pop().unwrap();
                if let Some(&(p, _, _, _)) = stack.last() {
                    low[p] = low[p].min(low[u]);
                    let pe = pe.unwrap();
                    if low[u] >= disc[p] {
                        // p separates u's subtree: pop one block.
                        let is_root = stack.len() == 1;
                        if !is_root {
                            is_art[p] = true;
                        }
                        let mut block = Vec::new();
                        while let Some(x) = edge_stack.pop() {
                            block.push(x);
                            if x == pe {
                                break;
                            }
                        }
                        if low[u] > disc[p] {
                            out.bridges.push(pe);
                        }
                        out.blocks.push(block);
                    }
                } else if u == root {
                    // root is an articulation point iff it has >= 2 DFS children
                }
            }
        }
    }
    // Root articulation points: two or more DFS children.
    let mut children = vec![0usize; n];
    {
        // Recompute DFS-tree child counts from discovery order cheaply:
        // a vertex v (non-root) has DFS parent p where the tree edge entered v.
        // We reconstruct by a second pass to keep the main loop simple.
        let mut seen = vec![false; n];
        let mut st: Vec<(usize, usize)> = Vec::new();
        for r in 0..n {
            if seen[r] {
                continue;
            }
            seen[r] = true;
            st.push((r, 0));
            while let Some(&mut (u, ref mut i)) = st.last_mut() {
                if *i < g.adj[u].len() {
                    let (v, _) = g.adj[u][*i];
                    *i += 1;
                    if !seen[v] {
                        seen[v] = true;
                        if u == r {
                            children[r] += 1;
                        }
                        st.push((v, 0));
                    }
                } else {
                    st.pop();
                }
            }
            if children[r] >= 2 {
                is_art[r] = true;
            }
        }
    }
    out.articulation_points = (0..n).filter(|&v| is_art[v]).collect();
    out.bridges.sort_unstable();
    out
}

/// Core number of every vertex (Batagelj-Zaversnik bucket algorithm,
/// `O(v + e)`). Self-loops count twice toward degree, as in the usual
/// multigraph convention.
pub fn core_numbers(g: &Graph) -> Vec<usize> {
    let n = g.n;
    let mut deg: Vec<usize> = (0..n)
        .map(|u| {
            g.adj[u]
                .iter()
                .map(|&(v, _)| if v == u { 2 } else { 1 })
                .sum()
        })
        .collect();
    let maxd = deg.iter().copied().max().unwrap_or(0);
    let mut bin = vec![0usize; maxd + 1];
    for &d in &deg {
        bin[d] += 1;
    }
    let mut start = 0;
    for b in bin.iter_mut() {
        let c = *b;
        *b = start;
        start += c;
    }
    let mut pos = vec![0usize; n];
    let mut vert = vec![0usize; n];
    for v in 0..n {
        pos[v] = bin[deg[v]];
        vert[pos[v]] = v;
        bin[deg[v]] += 1;
    }
    for d in (1..=maxd).rev() {
        bin[d] = bin[d - 1];
    }
    if maxd > 0 || n > 0 {
        bin[0] = 0;
    }
    for i in 0..n {
        let v = vert[i];
        for &(u, _) in &g.adj[v] {
            if u == v {
                continue;
            }
            if deg[u] > deg[v] {
                let du = deg[u];
                let pu = pos[u];
                let pw = bin[du];
                let w = vert[pw];
                if u != w {
                    pos[u] = pw;
                    vert[pu] = w;
                    pos[w] = pu;
                    vert[pw] = u;
                }
                bin[du] += 1;
                deg[u] -= 1;
            }
        }
    }
    deg
}

/// Vertices of the 2-core (core number >= 2).
pub fn two_core(g: &Graph) -> Vec<usize> {
    core_numbers(g)
        .into_iter()
        .enumerate()
        .filter(|&(_, c)| c >= 2)
        .map(|(v, _)| v)
        .collect()
}

/// Low-stretch spanning forest for unweighted graphs by recursive
/// ball-growing low-diameter decomposition (Alon-Karp-Peleg-West / Bartal
/// style region growing): grow a BFS ball from a centre, stop at the first
/// radius whose boundary edge count is at most `ln(e+1)/r_max` times the
/// interior volume, recurse inside each cluster, then join clusters along
/// BFS tree edges of the contracted graph.
///
/// The report's literature bound is Abraham-Neiman petal decomposition,
/// `O(e log v log log v)` total stretch. This is a different, simpler
/// construction; its stretch is measured by [`total_stretch`], not proved
/// here. The result is always a valid spanning forest.
pub fn low_stretch_spanning_tree(g: &Graph) -> Vec<usize> {
    let n = g.n;
    let mut uf = UnionFind::new(n);
    let mut tree = Vec::new();
    let all: Vec<usize> = (0..n).collect();
    let m = g.edge_count().max(1) as f64;
    lsst_rec(g, &all, &mut uf, &mut tree, (m + 1.0).ln().max(1.0));
    // Any remaining cross-cluster connectivity: BFS-order edges over the
    // contracted graph (Kruskal on edges in BFS order of the whole graph).
    let forest = g.spanning_forest();
    let mut order: Vec<usize> = (0..g.edge_count()).collect();
    order.sort_by_key(|&e| {
        let (u, v) = g.edges[e];
        (
            forest.depth[u].min(forest.depth[v]),
            !forest.is_tree_edge(g, e),
        )
    });
    for e in order {
        let (u, v) = g.edges[e];
        if uf.union(u, v) {
            tree.push(e);
        }
    }
    tree.sort_unstable();
    tree
}

fn lsst_rec(g: &Graph, verts: &[usize], uf: &mut UnionFind, tree: &mut Vec<usize>, log_m: f64) {
    if verts.len() <= 2 {
        // Connect what is directly adjacent.
        if verts.len() == 2 {
            let (a, b) = (verts[0], verts[1]);
            if let Some(&(_, e)) = g.adj[a].iter().find(|&&(v, _)| v == b) {
                if uf.union(a, b) {
                    tree.push(e);
                }
            }
        }
        return;
    }
    let mut inside = vec![false; g.n];
    for &v in verts {
        inside[v] = true;
    }
    let mut assigned = vec![false; g.n];
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    for &c in verts {
        if assigned[c] {
            continue;
        }
        // Region growing from c within unassigned vertices of this level.
        let mut layer = vec![c];
        let mut ball = vec![c];
        assigned[c] = true;
        let mut bfs_edges: Vec<usize> = Vec::new();
        let mut volume = 0usize;
        loop {
            let mut next = Vec::new();
            let mut boundary = 0usize;
            let mut next_edges = Vec::new();
            for &u in &layer {
                for &(v, e) in &g.adj[u] {
                    if !inside[v] {
                        continue;
                    }
                    volume += 1;
                    if !assigned[v] {
                        boundary += 1;
                        assigned[v] = true;
                        next.push(v);
                        next_edges.push(e);
                    }
                }
            }
            let stop =
                next.is_empty() || (boundary as f64) <= (volume.max(1) as f64) * (1.0 / log_m);
            if stop {
                for v in next {
                    assigned[v] = false;
                }
                break;
            }
            ball.extend(next.iter().copied());
            bfs_edges.extend(next_edges);
            layer = next;
        }
        for e in bfs_edges {
            let (u, v) = g.edges[e];
            if uf.union(u, v) {
                tree.push(e);
            }
        }
        clusters.push(ball);
    }
    if clusters.len() == 1 {
        return; // already a BFS tree of the whole set
    }
    for cl in clusters {
        if cl.len() < verts.len() {
            lsst_rec(g, &cl, uf, tree, log_m);
        }
    }
}

/// Total stretch of `g`'s edges in a spanning forest given by edge ids:
/// the sum over every edge of its endpoints' distance in the forest.
/// Returns `None` if an edge joins two different trees.
pub fn total_stretch(g: &Graph, tree_edges: &[usize]) -> Option<usize> {
    let t = Graph::from_edges(
        g.n,
        &tree_edges.iter().map(|&e| g.edges[e]).collect::<Vec<_>>(),
    );
    let f = t.spanning_forest();
    let mut total = 0;
    for &(u, v) in &g.edges {
        total += f.tree_path(&t, u, v)?.len();
    }
    Some(total)
}

/// Exact minimum-length cycle basis over GF(2) by Horton's algorithm:
/// candidate cycles `P(x,u) + uv + P(v,x)` for every vertex `x` and edge
/// `uv` from BFS trees, sorted by length, then greedily kept when
/// independent over GF(2) (bit-packed elimination). This is the exact
/// output of the report's minimum-cycle-basis row for ordinary
/// incidence-cycle spaces; Amaldi et al.'s Monte Carlo `O(e^omega)`
/// algorithm computes the same object faster asymptotically and is not
/// what is implemented here (Horton is `O(v e^3 / 64)` here).
pub fn minimum_cycle_basis_gf2(g: &Graph) -> Vec<Cycle> {
    let m = g.edge_count();
    let (_, ncomp) = g.components();
    let dim = m + ncomp - g.n;
    if dim == 0 {
        return Vec::new();
    }
    let words = m.div_ceil(64);
    let mut cands: Vec<(usize, Vec<u64>, Cycle)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for x in 0..g.n {
        let (dist, pe) = g.bfs(x);
        let path_to = |mut v: usize| -> Vec<usize> {
            let mut p = Vec::new();
            while let Some(e) = pe[v] {
                p.push(e);
                let (a, b) = g.edges[e];
                v = if a == v { b } else { a };
            }
            p
        };
        for (e, &(u, v)) in g.edges.iter().enumerate() {
            if dist[u] == usize::MAX {
                continue;
            }
            if pe[u] == Some(e) || pe[v] == Some(e) {
                continue;
            }
            let pu = path_to(u);
            let pv = path_to(v);
            // Simple only if the two tree paths share no edge.
            let mut bits = vec![0u64; words];
            let mut ok = true;
            for &f in pu.iter().chain(pv.iter()) {
                if bits[f / 64] >> (f % 64) & 1 == 1 {
                    ok = false;
                    break;
                }
                bits[f / 64] |= 1 << (f % 64);
            }
            if !ok || bits[e / 64] >> (e % 64) & 1 == 1 {
                continue;
            }
            bits[e / 64] |= 1 << (e % 64);
            if !seen.insert(bits.clone()) {
                continue;
            }
            let l = pu.len() + pv.len() + 1;
            cands.push((l, bits, ordered_cycle(e, &pv, &pu)));
        }
    }
    cands.sort_by_key(|c| c.0);
    // Greedy GF(2) independence with an echelon basis keyed by pivot bit.
    let mut basis: Vec<(usize, Vec<u64>)> = Vec::new();
    let mut out = Vec::new();
    for (_, bits, cyc) in cands {
        let mut r = bits.clone();
        for (piv, b) in &basis {
            if r[piv / 64] >> (piv % 64) & 1 == 1 {
                for (x, y) in r.iter_mut().zip(b) {
                    *x ^= *y;
                }
            }
        }
        if let Some(piv) = first_bit(&r) {
            // Keep basis fully reduced on pivots for correctness of the scan.
            for (_, b) in basis.iter_mut() {
                if b[piv / 64] >> (piv % 64) & 1 == 1 {
                    for (x, y) in b.iter_mut().zip(&r) {
                        *x ^= *y;
                    }
                }
            }
            basis.push((piv, r));
            out.push(cyc);
            if out.len() == dim {
                break;
            }
        }
    }
    out
}

fn ordered_cycle(e: usize, pv: &[usize], pu: &[usize]) -> Cycle {
    // e (u->v), then v up to x (pv in order v->x), then x down to u.
    let mut c = vec![e];
    c.extend(pv.iter().copied());
    c.extend(pu.iter().rev().copied());
    c
}

fn first_bit(r: &[u64]) -> Option<usize> {
    for (i, &w) in r.iter().enumerate() {
        if w != 0 {
            return Some(i * 64 + w.trailing_zeros() as usize);
        }
    }
    None
}

/// Check that `cycle` (edge ids) is a closed walk with every vertex of even
/// degree in the edge multiset - i.e. an element of the GF(2) cycle space.
pub fn is_cycle_space_element(g: &Graph, cycle: &[usize]) -> bool {
    let mut deg = vec![0usize; g.n];
    for &e in cycle {
        let (u, v) = g.edges[e];
        deg[u] += 1;
        deg[v] += 1;
    }
    deg.iter().all(|d| d % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    fn random_graph(n: usize, m: usize, seed: u64) -> Graph {
        let mut r = SplitMix64::new(seed);
        let mut g = Graph::new(n);
        for _ in 0..m {
            let u = r.below(n as u64) as usize;
            let v = r.below(n as u64) as usize;
            if u != v {
                g.add_edge(u, v);
            }
        }
        g
    }

    #[test]
    fn components_and_forest() {
        let g = Graph::from_edges(6, &[(0, 1), (1, 2), (3, 4)]);
        let (_, c) = g.components();
        assert_eq!(c, 3);
        let mut uf = UnionFind::new(6);
        for &(u, v) in g.edges() {
            uf.union(u, v);
        }
        assert_eq!(uf.set_count(), 3);
        assert_eq!(g.dfs_preorder().len(), 6);
        assert_eq!(fundamental_cycle_basis(&g).len(), 0);
    }

    #[test]
    fn fundamental_cycles_are_cycles() {
        for seed in 0..20 {
            let g = random_graph(30, 60, seed);
            let (_, c) = g.components();
            let basis = fundamental_cycle_basis(&g);
            assert_eq!(basis.len(), g.edge_count() + c - g.vertex_count());
            for cyc in &basis {
                assert!(is_cycle_space_element(&g, cyc));
            }
        }
    }

    #[test]
    fn bridges_and_articulation_on_handcuff() {
        // Triangles abc and def joined by bridge cd (report section 5).
        let g = Graph::from_edges(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3), (2, 3)]);
        let b = biconnectivity(&g);
        assert_eq!(b.bridges, vec![6]);
        assert_eq!(b.articulation_points, vec![2, 3]);
        assert_eq!(b.blocks.len(), 3);
        let cores = core_numbers(&g);
        assert!(cores.iter().all(|&c| c == 2));
    }

    #[test]
    fn bridges_match_brute_force() {
        for seed in 0..30 {
            let g = random_graph(15, 20, 100 + seed);
            let b = biconnectivity(&g);
            let (_, c0) = g.components();
            let mut brute = Vec::new();
            for e in 0..g.edge_count() {
                let rest: Vec<_> = (0..g.edge_count())
                    .filter(|&f| f != e)
                    .map(|f| g.edges()[f])
                    .collect();
                let h = Graph::from_edges(g.vertex_count(), &rest);
                if h.components().1 > c0 {
                    brute.push(e);
                }
            }
            assert_eq!(b.bridges, brute, "seed {seed}");
            let total: usize = b.blocks.iter().map(|x| x.len()).sum();
            assert_eq!(total, g.edge_count());
            // Articulation brute force.
            let mut art = Vec::new();
            for v in 0..g.vertex_count() {
                let rest: Vec<_> = g
                    .edges()
                    .iter()
                    .copied()
                    .filter(|&(a, b)| a != v && b != v)
                    .collect();
                let h = Graph::from_edges(g.vertex_count(), &rest);
                // removing v adds isolated v itself: compare minus that one
                if h.components().1 - 1 > c0 - usize::from(g.degree(v) == 0) {
                    art.push(v);
                }
            }
            assert_eq!(b.articulation_points, art, "seed {seed}");
        }
    }

    #[test]
    fn core_numbers_match_peeling() {
        for seed in 0..20 {
            let g = random_graph(25, 70, 200 + seed);
            let cores = core_numbers(&g);
            for k in 0..8 {
                // Brute-force k-core by peeling.
                let mut alive = vec![true; g.vertex_count()];
                loop {
                    let mut changed = false;
                    for v in 0..g.vertex_count() {
                        if alive[v] && g.neighbors(v).iter().filter(|&&(u, _)| alive[u]).count() < k
                        {
                            alive[v] = false;
                            changed = true;
                        }
                    }
                    if !changed {
                        break;
                    }
                }
                for v in 0..g.vertex_count() {
                    assert_eq!(alive[v], cores[v] >= k, "seed {seed} k {k} v {v}");
                }
            }
        }
    }

    #[test]
    fn low_stretch_tree_is_spanning_forest() {
        for seed in 0..10 {
            let g = random_graph(60, 150, 300 + seed);
            let t = low_stretch_spanning_tree(&g);
            let (_, c) = g.components();
            assert_eq!(t.len(), g.vertex_count() - c);
            assert!(total_stretch(&g, &t).is_some());
        }
    }

    #[test]
    fn minimum_cycle_basis_matches_bruteforce_on_small() {
        // K4: minimum cycle basis is three triangles, total length 9.
        let g = Graph::from_edges(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        let b = minimum_cycle_basis_gf2(&g);
        assert_eq!(b.len(), 3);
        assert_eq!(b.iter().map(|c| c.len()).sum::<usize>(), 9);
        for c in &b {
            assert!(is_cycle_space_element(&g, c));
        }
        // Brute force on random small graphs: compare total length with the
        // optimum over all subsets of fundamental-cycle-space elements.
        for seed in 0..15 {
            let g = random_graph(7, 10, 400 + seed);
            let b = minimum_cycle_basis_gf2(&g);
            let fb = fundamental_cycle_basis(&g);
            assert_eq!(b.len(), fb.len());
            let d = fb.len();
            if d == 0 || d > 6 {
                continue;
            }
            let m = g.edge_count();
            let vec_of = |c: &Cycle| -> u64 { c.iter().fold(0u64, |a, &e| a ^ (1 << e)) };
            let basis_bits: Vec<u64> = fb.iter().map(vec_of).collect();
            let mut elems: Vec<u64> = Vec::new();
            for mask in 1u64..(1 << d) {
                let mut x = 0;
                for i in 0..d {
                    if mask >> i & 1 == 1 {
                        x ^= basis_bits[i];
                    }
                }
                elems.push(x);
            }
            elems.sort_by_key(|x| x.count_ones());
            // Greedy over all cycle-space elements gives the optimum (matroid).
            let mut piv: Vec<u64> = Vec::new();
            let mut best = 0usize;
            for x in elems {
                let mut r = x;
                for &p in &piv {
                    r = r.min(r ^ p);
                }
                if r != 0 {
                    piv.push(r);
                    piv.sort_unstable_by(|a, b| b.cmp(a));
                    best += x.count_ones() as usize;
                }
            }
            let got: usize = b.iter().map(|c| c.len()).sum();
            assert_eq!(got, best, "seed {seed} m {m}");
        }
    }
}
