//! Report section 4, "Directed shortest paths": single-source shortest paths
//! with nonnegative integer weights.
//!
//! * [`dijkstra`]: binary-heap Dijkstra, `O((v + e) log v)`; the reference.
//! * [`bmssp_sssp`]: the bounded multi-source shortest path recursion of
//!   Duan, Mao, Mao, Shu and Yin ("Breaking the sorting barrier", 2025):
//!   `FindPivots` with `k` Bellman-Ford rounds, a base-case mini-Dijkstra,
//!   and the partial-sorting frontier structure with `Insert`,
//!   `BatchPrepend` and `Pull`. Its `O(e log^{2/3} v)` bound assumes a
//!   constant-degree graph, distinct path lengths and the paper's linked
//!   block structure; here the frontier is an ordered set (`O(log)` per
//!   operation), distinct lengths are obtained by a seeded 128-bit weight
//!   perturbation that preserves exact distances, and no degree reduction
//!   is applied. The bound is
//!   therefore not claimed for this implementation; outputs are checked
//!   against [`dijkstra`]. The 2026 Duan-Mao-Shu-Yin refinement quoted in
//!   the report (`O(e sqrt(log v) + sqrt(e v log v log log v))`) is a
//!   different, newer algorithm and is not implemented.

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap};

/// Directed graph with nonnegative integer weights.
#[derive(Clone, Debug)]
pub struct Digraph {
    pub adj: Vec<Vec<(usize, u64)>>,
}

impl Digraph {
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        self.adj[u].push((v, w));
    }
    pub fn n(&self) -> usize {
        self.adj.len()
    }
}

pub const UNREACHABLE: u64 = u64::MAX;

/// Dijkstra from `s`.
pub fn dijkstra(g: &Digraph, s: usize) -> Vec<u64> {
    let mut d = vec![UNREACHABLE; g.n()];
    d[s] = 0;
    let mut h = BinaryHeap::from([Reverse((0u64, s))]);
    while let Some(Reverse((du, u))) = h.pop() {
        if du > d[u] {
            continue;
        }
        for &(v, w) in &g.adj[u] {
            let nd = du.saturating_add(w);
            if nd < d[v] {
                d[v] = nd;
                h.push(Reverse((nd, v)));
            }
        }
    }
    d
}

/// Frontier structure `D` of the BMSSP recursion.
type Dist = u128;
const INF: Dist = Dist::MAX;

struct Frontier {
    m: usize,
    bound: Dist,
    set: BTreeSet<(Dist, usize)>,
    val: HashMap<usize, Dist>,
}

impl Frontier {
    fn new(m: usize, bound: Dist) -> Self {
        Self {
            m,
            bound,
            set: BTreeSet::new(),
            val: HashMap::new(),
        }
    }
    fn insert(&mut self, key: usize, value: Dist) {
        match self.val.get(&key) {
            Some(&old) if old <= value => {}
            Some(&old) => {
                self.set.remove(&(old, key));
                self.set.insert((value, key));
                self.val.insert(key, value);
            }
            None => {
                self.set.insert((value, key));
                self.val.insert(key, value);
            }
        }
    }
    fn batch_prepend(&mut self, items: &[(usize, Dist)]) {
        for &(k, v) in items {
            self.insert(k, v);
        }
    }
    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
    /// Remove up to `m` smallest keys (extended over equal values so the
    /// returned bound strictly separates); returns `(bound, keys)`.
    fn pull(&mut self) -> (Dist, Vec<usize>) {
        let mut out = Vec::new();
        let mut last = None;
        while let Some(&(v, k)) = self.set.first() {
            if out.len() >= self.m && last != Some(v) {
                break;
            }
            self.set.pop_first();
            self.val.remove(&k);
            out.push(k);
            last = Some(v);
        }
        let b = self.set.first().map_or(self.bound, |&(v, _)| v);
        (b, out)
    }
}

struct Bmssp<'a> {
    adj: &'a [Vec<(usize, Dist)>],
    budget: u64,
    pub aborted: bool,
    d: Vec<Dist>,
    pred: Vec<usize>,
    k: usize,
    t: u32,
    pub relaxations: u64,
}

impl Bmssp<'_> {
    fn relax(&mut self, u: usize, v: usize, w: Dist) -> Option<Dist> {
        self.relaxations += 1;
        if self.relaxations > self.budget {
            self.aborted = true;
        }
        let nd = self.d[u].saturating_add(w);
        if nd <= self.d[v] {
            if nd < self.d[v] || self.pred[v] == usize::MAX {
                self.pred[v] = u;
            }
            self.d[v] = nd;
            Some(nd)
        } else {
            None
        }
    }

    fn find_pivots(&mut self, b: Dist, s: &[usize]) -> (Vec<usize>, Vec<usize>) {
        let mut in_w: HashMap<usize, ()> = s.iter().map(|&x| (x, ())).collect();
        let mut w_all: Vec<usize> = s.to_vec();
        let mut prev: Vec<usize> = s.to_vec();
        for _ in 0..self.k {
            let mut next = Vec::new();
            let mut seen_next: HashMap<usize, ()> = HashMap::new();
            for &u in &prev {
                for i in 0..self.adj[u].len() {
                    let (v, w) = self.adj[u][i];
                    if let Some(nd) = self.relax(u, v, w) {
                        if nd < b && seen_next.insert(v, ()).is_none() {
                            next.push(v);
                        }
                    }
                }
            }
            for &v in &next {
                if in_w.insert(v, ()).is_none() {
                    w_all.push(v);
                }
            }
            if w_all.len() > self.k * s.len() {
                return (s.to_vec(), w_all);
            }
            prev = next;
        }
        // Forest of tight edges inside W; pivots are roots in S whose tree
        // has at least k vertices.
        let in_s: HashMap<usize, ()> = s.iter().map(|&x| (x, ())).collect();
        let mut count: HashMap<usize, usize> = HashMap::new();
        for &v in &w_all {
            let mut x = v;
            let mut steps = 0;
            while !in_s.contains_key(&x) && steps <= w_all.len() {
                let p = self.pred[x];
                if p == usize::MAX || !in_w.contains_key(&p) {
                    break;
                }
                x = p;
                steps += 1;
            }
            if in_s.contains_key(&x) {
                *count.entry(x).or_insert(0) += 1;
            }
        }
        let pivots = s
            .iter()
            .copied()
            .filter(|x| count.get(x).copied().unwrap_or(0) >= self.k)
            .collect();
        (pivots, w_all)
    }

    fn base_case(&mut self, b: Dist, x: usize) -> (Dist, Vec<usize>) {
        let mut u0: Vec<usize> = Vec::new();
        let mut done: HashMap<usize, ()> = HashMap::new();
        let mut h = BinaryHeap::from([Reverse((self.d[x], x))]);
        while let Some(Reverse((du, u))) = h.pop() {
            if du != self.d[u] || done.contains_key(&u) {
                continue;
            }
            if u0.len() > self.k {
                break;
            }
            done.insert(u, ());
            u0.push(u);
            for i in 0..self.adj[u].len() {
                let (v, w) = self.adj[u][i];
                let nd = self.d[u].saturating_add(w);
                if nd < b && nd <= self.d[v] && !done.contains_key(&v) {
                    self.relax(u, v, w);
                    h.push(Reverse((nd, v)));
                }
            }
        }
        if u0.len() <= self.k {
            (b, u0)
        } else {
            let bp = u0.iter().map(|&v| self.d[v]).max().unwrap();
            let u = u0.into_iter().filter(|&v| self.d[v] < bp).collect();
            (bp, u)
        }
    }

    fn run(&mut self, l: u32, b: Dist, s: &[usize]) -> (Dist, Vec<usize>) {
        if l == 0 {
            debug_assert_eq!(s.len(), 1);
            return self.base_case(b, s[0]);
        }
        let (p, w) = self.find_pivots(b, s);
        let m = 1usize << ((l - 1) * self.t).min(40);
        let limit = self.k.saturating_mul(1usize << (l * self.t).min(40));
        let mut dstruct = Frontier::new(m, b);
        for &x in &p {
            dstruct.insert(x, self.d[x]);
        }
        let mut last_bp = p.iter().map(|&x| self.d[x]).min().unwrap_or(b);
        let mut u_all: Vec<usize> = Vec::new();
        let mut in_u: HashMap<usize, ()> = HashMap::new();
        while u_all.len() < limit && !dstruct.is_empty() && !self.aborted {
            let (bi, si) = dstruct.pull();
            // Recurse one vertex at a time at level 0 (base case is single-source).
            let (bpi, ui) = if l == 1 {
                let mut best_b = bi;
                let mut all = Vec::new();
                for &x in &si {
                    let (bb, uu) = self.base_case(bi, x);
                    best_b = best_b.min(bb);
                    all.extend(uu);
                }
                (
                    best_b,
                    all.into_iter()
                        .filter(|&v| self.d[v] < best_b)
                        .collect::<Vec<_>>(),
                )
            } else {
                self.run(l - 1, bi, &si)
            };
            for &u in &ui {
                if in_u.insert(u, ()).is_none() {
                    u_all.push(u);
                }
            }
            let mut kbatch: Vec<(usize, Dist)> = Vec::new();
            for &u in &ui {
                for i in 0..self.adj[u].len() {
                    let (v, w) = self.adj[u][i];
                    if let Some(nd) = self.relax(u, v, w) {
                        if nd >= bi && nd < b {
                            dstruct.insert(v, nd);
                        } else if nd >= bpi && nd < bi {
                            kbatch.push((v, nd));
                        }
                    }
                }
            }
            for &x in &si {
                let dx = self.d[x];
                if dx >= bpi && dx < bi {
                    kbatch.push((x, dx));
                }
            }
            dstruct.batch_prepend(&kbatch);
            last_bp = bpi;
            if u_all.len() >= limit {
                break;
            }
        }
        let bp = if dstruct.is_empty() && u_all.len() < limit {
            b
        } else {
            last_bp.min(b)
        };
        for &x in &w {
            if self.d[x] < bp && in_u.insert(x, ()).is_none() {
                u_all.push(x);
            }
        }
        (bp, u_all)
    }
}

/// Counters of a BMSSP run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BmsspStats {
    pub relaxations: u64,
    pub levels: u32,
    pub k: usize,
    pub t: u32,
    /// Vertices whose distance needed the final Dijkstra completion sweep
    /// (zero when the recursion completed every reachable vertex).
    pub completed_by_sweep: usize,
    /// The relaxation budget guard stopped the recursion (ties or a
    /// violated assumption); the sweep then completed the distances.
    pub aborted: bool,
}

/// Single-source shortest paths by the BMSSP recursion from `s`.
///
/// After the top-level call, a final label-correcting sweep (Dijkstra seeded
/// with every vertex's current label) completes any vertex the recursion
/// left unsettled and the count is reported in `completed_by_sweep`; the
/// returned distances are exact either way.
pub fn bmssp_sssp(g: &Digraph, s: usize) -> (Vec<u64>, BmsspStats) {
    let n = g.n().max(2);
    let logn = (n as f64).log2();
    let k = (logn.powf(1.0 / 3.0).floor() as usize).max(1);
    let t = (logn.powf(2.0 / 3.0).floor() as u32).max(1);
    let levels = ((logn / t as f64).ceil() as u32).max(1);
    // Distinct path lengths (the paper's Assumption): internal weight
    // (w << 64) + 1 + r with seeded r < 2^64 / (2n). Over any simple path the
    // perturbation sum stays below 2^64, so `d >> 64` is the exact distance,
    // zero-weight edges become positive and ties are broken.
    let mut rng = super::field::SplitMix64::new(0x5EED ^ g.n() as u64);
    let span = (u64::MAX / (2 * n as u64)).max(1);
    let adj: Vec<Vec<(usize, Dist)>> = g
        .adj
        .iter()
        .map(|row| {
            row.iter()
                .map(|&(v, w)| (v, ((w as Dist) << 64) + 1 + rng.below(span) as Dist))
                .collect()
        })
        .collect();
    let edges: u64 = g.adj.iter().map(|r| r.len() as u64).sum();
    let mut ctx = Bmssp {
        adj: &adj,
        budget: 64 * (edges + n as u64) * (levels as u64 + 2),
        aborted: false,
        d: vec![INF; g.n()],
        pred: vec![usize::MAX; g.n()],
        k,
        t,
        relaxations: 0,
    };
    ctx.d[s] = 0;
    ctx.run(levels, INF, &[s]);
    let aborted = ctx.aborted;
    let mut d: Vec<u64> = ctx
        .d
        .iter()
        .map(|&x| {
            if x == INF {
                UNREACHABLE
            } else {
                (x >> 64) as u64
            }
        })
        .collect();
    // Completion sweep: Dijkstra seeded with all finite labels.
    let before = d.clone();
    let mut h: BinaryHeap<Reverse<(u64, usize)>> = d
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != UNREACHABLE)
        .map(|(v, &x)| Reverse((x, v)))
        .collect();
    while let Some(Reverse((du, u))) = h.pop() {
        if du > d[u] {
            continue;
        }
        for &(v, w) in &g.adj[u] {
            let nd = du.saturating_add(w);
            if nd < d[v] {
                d[v] = nd;
                h.push(Reverse((nd, v)));
            }
        }
    }
    let completed_by_sweep = d.iter().zip(&before).filter(|(a, b)| a != b).count();
    let stats = BmsspStats {
        relaxations: ctx.relaxations,
        levels,
        k,
        t,
        completed_by_sweep,
        aborted,
    };
    (d, stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    #[test]
    fn bmssp_matches_dijkstra() {
        let mut swept = 0;
        for seed in 0..40 {
            let mut rng = SplitMix64::new(seed);
            let n = 50 + rng.below(400) as usize;
            let m = n * (1 + rng.below(4) as usize);
            let mut g = Digraph::new(n);
            let wmax = if seed % 2 == 0 { 5 } else { 1_000_000 };
            for _ in 0..m {
                let (u, v) = (rng.below(n as u64) as usize, rng.below(n as u64) as usize);
                g.add_edge(u, v, rng.below(wmax));
            }
            let s = rng.below(n as u64) as usize;
            let want = dijkstra(&g, s);
            let (got, st) = bmssp_sssp(&g, s);
            assert_eq!(got, want, "seed {seed}");
            swept += st.completed_by_sweep;
            assert!(!st.aborted, "seed {seed}: budget guard fired");
        }
        // The recursion itself should settle every vertex.
        assert_eq!(swept, 0, "recursion left vertices for the sweep");
    }
}
