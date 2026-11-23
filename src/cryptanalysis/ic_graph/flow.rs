//! Report section 4: exact max-flow / min-cost flow on integral networks.
//!
//! The report's literature bound is the deterministic almost-linear
//! `e^{1+o(1)}` algorithm of van den Brand et al. (2023); that algorithm is
//! not practical to implement and is not what is here. This module gives the
//! exact classical algorithms with the same outputs:
//!
//! * [`MaxFlow::dinic`]: Dinic blocking flows, `O(v^2 e)` in general and
//!   `O(e sqrt(v))` on unit-capacity bipartite matching networks.
//! * [`MinCostFlow::solve`]: successive shortest paths with Johnson
//!   potentials (Bellman-Ford once, then Dijkstra), exact for integral
//!   capacities and costs.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone, Debug)]
struct Arc {
    to: usize,
    cap: i64,
    cost: i64,
}

/// Residual network shared by both solvers. Arc `2i` is forward, `2i+1` back.
#[derive(Clone, Debug)]
pub struct MaxFlow {
    n: usize,
    arcs: Vec<Arc>,
    head: Vec<Vec<usize>>,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            arcs: Vec::new(),
            head: vec![Vec::new(); n],
        }
    }
    /// Add arc `u -> v` with capacity `cap >= 0`; returns the arc id.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) -> usize {
        assert!(cap >= 0);
        let id = self.arcs.len();
        self.arcs.push(Arc {
            to: v,
            cap,
            cost: 0,
        });
        self.arcs.push(Arc {
            to: u,
            cap: 0,
            cost: 0,
        });
        self.head[u].push(id);
        self.head[v].push(id + 1);
        id
    }
    /// Flow currently on the forward arc `id` (as returned by `add_edge`).
    pub fn flow_on(&self, id: usize) -> i64 {
        self.arcs[id + 1].cap
    }

    /// Dinic's algorithm from `s` to `t`; returns the maximum flow value.
    pub fn dinic(&mut self, s: usize, t: usize) -> i64 {
        let mut total = 0;
        loop {
            let mut level = vec![usize::MAX; self.n];
            level[s] = 0;
            let mut q = VecDeque::from([s]);
            while let Some(u) = q.pop_front() {
                for &a in &self.head[u] {
                    let v = self.arcs[a].to;
                    if self.arcs[a].cap > 0 && level[v] == usize::MAX {
                        level[v] = level[u] + 1;
                        q.push_back(v);
                    }
                }
            }
            if level[t] == usize::MAX {
                return total;
            }
            let mut it = vec![0usize; self.n];
            loop {
                let f = self.augment(s, t, &level, &mut it);
                if f == 0 {
                    break;
                }
                total += f;
            }
        }
    }

    /// One augmenting path in the level graph (iterative DFS).
    fn augment(&mut self, s: usize, t: usize, level: &[usize], it: &mut [usize]) -> i64 {
        let mut path: Vec<usize> = Vec::new();
        let mut u = s;
        loop {
            if u == t {
                let f = path.iter().map(|&a| self.arcs[a].cap).min().unwrap_or(0);
                for &a in &path {
                    self.arcs[a].cap -= f;
                    self.arcs[a ^ 1].cap += f;
                }
                return f;
            }
            let mut advanced = false;
            while it[u] < self.head[u].len() {
                let a = self.head[u][it[u]];
                let v = self.arcs[a].to;
                if self.arcs[a].cap > 0 && level[v] == level[u] + 1 {
                    path.push(a);
                    u = v;
                    advanced = true;
                    break;
                }
                it[u] += 1;
            }
            if !advanced {
                // Dead end: retreat.
                let Some(a) = path.pop() else { return 0 };
                u = self.arcs[a ^ 1].to;
                it[u] += 1;
            }
        }
    }

    /// Vertices on the source side of a minimum cut (after `dinic`).
    pub fn min_cut_side(&self, s: usize) -> Vec<bool> {
        let mut seen = vec![false; self.n];
        seen[s] = true;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &a in &self.head[u] {
                let v = self.arcs[a].to;
                if self.arcs[a].cap > 0 && !seen[v] {
                    seen[v] = true;
                    q.push_back(v);
                }
            }
        }
        seen
    }
}

/// Min-cost flow network.
#[derive(Clone, Debug)]
pub struct MinCostFlow {
    net: MaxFlow,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        Self {
            net: MaxFlow::new(n),
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) -> usize {
        let id = self.net.add_edge(u, v, cap);
        self.net.arcs[id].cost = cost;
        self.net.arcs[id + 1].cost = -cost;
        id
    }
    pub fn flow_on(&self, id: usize) -> i64 {
        self.net.flow_on(id)
    }

    /// Send up to `limit` units from `s` to `t` at minimum cost. Returns
    /// `(flow, cost)`. Requires no negative-cost cycle in the input.
    pub fn solve(&mut self, s: usize, t: usize, limit: i64) -> (i64, i64) {
        let n = self.net.n;
        let arcs = &mut self.net.arcs;
        let head = &self.net.head;
        // Bellman-Ford potentials (handles negative arc costs).
        let mut pot = vec![i64::MAX; n];
        pot[s] = 0;
        for _ in 0..n {
            let mut changed = false;
            for u in 0..n {
                if pot[u] == i64::MAX {
                    continue;
                }
                for &a in &head[u] {
                    if arcs[a].cap > 0 && pot[u] + arcs[a].cost < pot[arcs[a].to] {
                        pot[arcs[a].to] = pot[u] + arcs[a].cost;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        for p in pot.iter_mut() {
            if *p == i64::MAX {
                *p = 0;
            }
        }
        let (mut flow, mut cost) = (0i64, 0i64);
        while flow < limit {
            let mut dist = vec![i64::MAX; n];
            let mut prev = vec![usize::MAX; n];
            dist[s] = 0;
            let mut heap = BinaryHeap::from([Reverse((0i64, s))]);
            while let Some(Reverse((d, u))) = heap.pop() {
                if d > dist[u] {
                    continue;
                }
                for &a in &head[u] {
                    let v = arcs[a].to;
                    if arcs[a].cap <= 0 {
                        continue;
                    }
                    let nd = d + arcs[a].cost + pot[u] - pot[v];
                    if nd < dist[v] {
                        dist[v] = nd;
                        prev[v] = a;
                        heap.push(Reverse((nd, v)));
                    }
                }
            }
            if dist[t] == i64::MAX {
                break;
            }
            for v in 0..n {
                if dist[v] != i64::MAX {
                    pot[v] += dist[v];
                }
            }
            let mut push = limit - flow;
            let mut v = t;
            while v != s {
                let a = prev[v];
                push = push.min(arcs[a].cap);
                v = arcs[a ^ 1].to;
            }
            let mut v = t;
            while v != s {
                let a = prev[v];
                arcs[a].cap -= push;
                arcs[a ^ 1].cap += push;
                cost += push * arcs[a].cost;
                v = arcs[a ^ 1].to;
            }
            flow += push;
        }
        (flow, cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    #[test]
    fn max_flow_equals_brute_force_min_cut() {
        let mut rng = SplitMix64::new(8);
        for _ in 0..60 {
            let n = 2 + rng.below(8) as usize;
            let mut g = MaxFlow::new(n);
            let mut edges = Vec::new();
            for _ in 0..rng.below(20) {
                let (u, v) = (rng.below(n as u64) as usize, rng.below(n as u64) as usize);
                let c = rng.below(10) as i64;
                g.add_edge(u, v, c);
                edges.push((u, v, c));
            }
            let f = g.dinic(0, n - 1);
            // Brute-force minimum s-t cut over vertex subsets.
            let mut best = i64::MAX;
            for mask in 0u32..(1 << n) {
                if mask & 1 == 0 || mask >> (n - 1) & 1 == 1 {
                    continue;
                }
                let cut: i64 = edges
                    .iter()
                    .filter(|&&(u, v, _)| mask >> u & 1 == 1 && mask >> v & 1 == 0)
                    .map(|&(_, _, c)| c)
                    .sum();
                best = best.min(cut);
            }
            assert_eq!(f, best);
            let side = g.min_cut_side(0);
            let cut: i64 = edges
                .iter()
                .filter(|&&(u, v, _)| side[u] && !side[v])
                .map(|&(_, _, c)| c)
                .sum();
            assert_eq!(cut, f);
        }
    }

    #[test]
    fn min_cost_assignment_matches_permutations() {
        let mut rng = SplitMix64::new(21);
        for _ in 0..30 {
            let k = 1 + rng.below(5) as usize;
            let cost: Vec<Vec<i64>> = (0..k)
                .map(|_| (0..k).map(|_| rng.below(50) as i64 - 10).collect())
                .collect();
            let (s, t) = (2 * k, 2 * k + 1);
            let mut g = MinCostFlow::new(2 * k + 2);
            for i in 0..k {
                g.add_edge(s, i, 1, 0);
                g.add_edge(k + i, t, 1, 0);
                for j in 0..k {
                    g.add_edge(i, k + j, 1, cost[i][j]);
                }
            }
            let (fl, c) = g.solve(s, t, k as i64);
            assert_eq!(fl, k as i64);
            // Brute force over permutations.
            let mut perm: Vec<usize> = (0..k).collect();
            let mut best = i64::MAX;
            permute(&mut perm, 0, &mut |p| {
                best = best.min((0..k).map(|i| cost[i][p[i]]).sum());
            });
            assert_eq!(c, best);
        }
    }

    fn permute(p: &mut Vec<usize>, i: usize, f: &mut dyn FnMut(&[usize])) {
        if i == p.len() {
            f(p);
            return;
        }
        for j in i..p.len() {
            p.swap(i, j);
            permute(p, i + 1, f);
            p.swap(i, j);
        }
    }
}
