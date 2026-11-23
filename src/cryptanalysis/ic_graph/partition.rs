//! Report section 4 and section 8 ("communication model"): hypergraph
//! partitioning for distributed sparse products and independent-set
//! scheduling.
//!
//! * [`fiduccia_mattheyses`]: FM bipartition refinement with gain buckets,
//!   `O(I)` per pass in the original net model (pins scanned a bounded number
//!   of times per pass; bucket scans add `O(max degree)` per move here).
//!   Multilevel coarsening is not included.
//! * [`communication_volume`]: the column-net metric `sum_j (lambda_j - 1)`.
//! * [`greedy_mis`]: sequential greedy maximal independent set, `O(v + e)`.
//! * [`luby_mis`]: Luby's randomized rounds (parallel background).

use super::field::SplitMix64;
use super::graph::Graph;
use super::sparse::SparseMatrix;

/// Hypergraph: `nets[j]` lists the vertices (rows) on net `j` (a column).
#[derive(Clone, Debug)]
pub struct Hypergraph {
    pub vertices: usize,
    pub nets: Vec<Vec<usize>>,
}

impl Hypergraph {
    /// Column-net model of a sparse matrix: vertices are rows, nets columns.
    pub fn column_net(a: &SparseMatrix) -> Self {
        let mut nets = vec![Vec::new(); a.ncols];
        for (i, r) in a.rows.iter().enumerate() {
            for &(c, _) in r {
                nets[c].push(i);
            }
        }
        Self {
            vertices: a.nrows,
            nets,
        }
    }
    pub fn pins(&self) -> usize {
        self.nets.iter().map(|n| n.len()).sum()
    }
    fn vertex_nets(&self) -> Vec<Vec<usize>> {
        let mut vn = vec![Vec::new(); self.vertices];
        for (j, net) in self.nets.iter().enumerate() {
            for &v in net {
                vn[v].push(j);
            }
        }
        vn
    }
}

/// `sum_j (lambda_j - 1)` over nets touching at least one part, where
/// `lambda_j` is the number of distinct parts on net `j`.
pub fn communication_volume(h: &Hypergraph, part: &[usize]) -> usize {
    let mut total = 0;
    let mut seen: Vec<usize> = Vec::new();
    for net in &h.nets {
        seen.clear();
        for &v in net {
            if !seen.contains(&part[v]) {
                seen.push(part[v]);
            }
        }
        total += seen.len().saturating_sub(1);
    }
    total
}

/// Number of nets with pins on both sides of a bipartition.
pub fn cut_nets(h: &Hypergraph, side: &[usize]) -> usize {
    communication_volume(h, side)
}

/// Result of FM refinement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FmResult {
    pub side: Vec<usize>,
    pub cut: usize,
    pub passes: usize,
    pub moves: usize,
}

/// Fiduccia-Mattheyses bipartition refinement. Each side holds at most
/// `ceil((1 + imbalance) * V / 2)` vertices. Starts from `initial` (or a
/// seeded random balanced split) and runs passes until no improvement.
pub fn fiduccia_mattheyses(
    h: &Hypergraph,
    initial: Option<Vec<usize>>,
    imbalance: f64,
    seed: u64,
    max_passes: usize,
) -> FmResult {
    let n = h.vertices;
    let cap = (((1.0 + imbalance) * n as f64) / 2.0).ceil() as usize;
    let cap = cap.max(n.div_ceil(2));
    let mut side = initial.unwrap_or_else(|| {
        let mut idx: Vec<usize> = (0..n).collect();
        SplitMix64::new(seed).shuffle(&mut idx);
        let mut s = vec![0; n];
        for &v in idx.iter().skip(n / 2) {
            s[v] = 1;
        }
        s
    });
    let vn = h.vertex_nets();
    let maxdeg = vn.iter().map(|x| x.len()).max().unwrap_or(0) as i64;
    let off = maxdeg;
    let mut cut = cut_nets(h, &side);
    let mut passes = 0;
    let mut total_moves = 0;
    while passes < max_passes {
        passes += 1;
        let mut cnt = vec![[0usize; 2]; h.nets.len()];
        for (j, net) in h.nets.iter().enumerate() {
            for &v in net {
                cnt[j][side[v]] += 1;
            }
        }
        let mut gain = vec![0i64; n];
        for v in 0..n {
            let s = side[v];
            for &j in &vn[v] {
                if cnt[j][s] == 1 {
                    gain[v] += 1;
                }
                if cnt[j][1 - s] == 0 {
                    gain[v] -= 1;
                }
            }
        }
        // Buckets per side, lazy deletion.
        let nb = (2 * maxdeg + 1) as usize;
        let mut buckets: [Vec<Vec<usize>>; 2] = [vec![Vec::new(); nb], vec![Vec::new(); nb]];
        for v in 0..n {
            buckets[side[v]][(gain[v] + off) as usize].push(v);
        }
        let mut locked = vec![false; n];
        let mut sizes = [side.iter().filter(|&&s| s == 0).count(), 0];
        sizes[1] = n - sizes[0];
        let mut moves: Vec<usize> = Vec::new();
        let mut cur_cut = cut as i64;
        let mut best_cut = cur_cut;
        let mut best_len = 0;
        loop {
            // Pick best feasible move over both sides.
            let mut pick: Option<(i64, usize)> = None;
            for from in 0..2 {
                if sizes[1 - from] + 1 > cap {
                    continue;
                }
                'scan: for gi in (0..nb).rev() {
                    while let Some(&v) = buckets[from][gi].last() {
                        if locked[v] || side[v] != from || gain[v] + off != gi as i64 {
                            buckets[from][gi].pop();
                            continue;
                        }
                        let g = gi as i64 - off;
                        if pick.is_none_or(|(pg, _)| g > pg) {
                            pick = Some((g, v));
                        }
                        break 'scan;
                    }
                }
            }
            let Some((g, v)) = pick else { break };
            let from = side[v];
            let to = 1 - from;
            locked[v] = true;
            // FM gain updates.
            for &j in &vn[v] {
                if cnt[j][to] == 0 {
                    for &u in &h.nets[j] {
                        if !locked[u] {
                            gain[u] += 1;
                            buckets[side[u]][(gain[u] + off) as usize].push(u);
                        }
                    }
                } else if cnt[j][to] == 1 {
                    for &u in &h.nets[j] {
                        if side[u] == to && !locked[u] {
                            gain[u] -= 1;
                            buckets[side[u]][(gain[u] + off) as usize].push(u);
                        }
                    }
                }
                cnt[j][from] -= 1;
                cnt[j][to] += 1;
                if cnt[j][from] == 0 {
                    for &u in &h.nets[j] {
                        if !locked[u] {
                            gain[u] -= 1;
                            buckets[side[u]][(gain[u] + off) as usize].push(u);
                        }
                    }
                } else if cnt[j][from] == 1 {
                    for &u in &h.nets[j] {
                        if side[u] == from && u != v && !locked[u] {
                            gain[u] += 1;
                            buckets[side[u]][(gain[u] + off) as usize].push(u);
                        }
                    }
                }
            }
            side[v] = to;
            sizes[from] -= 1;
            sizes[to] += 1;
            cur_cut -= g;
            moves.push(v);
            if cur_cut < best_cut {
                best_cut = cur_cut;
                best_len = moves.len();
            }
        }
        // Roll back moves after the best prefix.
        for &v in &moves[best_len..] {
            side[v] = 1 - side[v];
        }
        total_moves += best_len;
        let new_cut = best_cut as usize;
        debug_assert_eq!(new_cut, cut_nets(h, &side));
        if new_cut >= cut {
            cut = new_cut;
            break;
        }
        cut = new_cut;
    }
    FmResult {
        side,
        cut,
        passes,
        moves: total_moves,
    }
}

/// Sequential greedy maximal independent set in the given vertex order.
pub fn greedy_mis(g: &Graph, order: &[usize]) -> Vec<usize> {
    let mut blocked = vec![false; g.vertex_count()];
    let mut out = Vec::new();
    for &v in order {
        if blocked[v] {
            continue;
        }
        out.push(v);
        blocked[v] = true;
        for &(u, _) in g.neighbors(v) {
            blocked[u] = true;
        }
    }
    out.sort_unstable();
    out
}

/// Luby's randomized maximal independent set; returns the set and the
/// number of synchronous rounds.
pub fn luby_mis(g: &Graph, seed: u64) -> (Vec<usize>, usize) {
    let n = g.vertex_count();
    let mut rng = SplitMix64::new(seed);
    let mut alive = vec![true; n];
    let mut in_set = vec![false; n];
    let mut rounds = 0;
    while alive.iter().any(|&a| a) {
        rounds += 1;
        let pri: Vec<u64> = (0..n).map(|_| rng.next_u64()).collect();
        let winners: Vec<usize> = (0..n)
            .filter(|&v| {
                alive[v]
                    && g.neighbors(v)
                        .iter()
                        .all(|&(u, _)| u == v || !alive[u] || (pri[v], v) < (pri[u], u))
            })
            .collect();
        for &v in &winners {
            in_set[v] = true;
        }
        for &v in &winners {
            alive[v] = false;
            for &(u, _) in g.neighbors(v) {
                alive[u] = false;
            }
        }
    }
    ((0..n).filter(|&v| in_set[v]).collect(), rounds)
}

/// Check independence and maximality.
pub fn is_maximal_independent(g: &Graph, set: &[usize]) -> bool {
    let n = g.vertex_count();
    let mut s = vec![false; n];
    for &v in set {
        s[v] = true;
    }
    for &(u, v) in g.edges() {
        if u != v && s[u] && s[v] {
            return false;
        }
    }
    (0..n).all(|v| s[v] || g.neighbors(v).iter().any(|&(u, _)| s[u]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::Fp;

    #[test]
    fn fm_never_worsens_and_respects_balance() {
        let f = Fp::new(101).unwrap();
        for seed in 0..10 {
            let mut rng = SplitMix64::new(seed);
            let nr = 120;
            let rows: Vec<Vec<(usize, u64)>> = (0..nr)
                .map(|i| {
                    // Two planted clusters with light cross traffic.
                    let base = if i < nr / 2 { 0 } else { 40 };
                    let mut r: Vec<(usize, u64)> =
                        (0..4).map(|_| (base + rng.below(40) as usize, 1)).collect();
                    if rng.below(10) == 0 {
                        r.push((rng.below(80) as usize, 1));
                    }
                    r
                })
                .collect();
            let a = SparseMatrix::new(f, 80, rows);
            let h = Hypergraph::column_net(&a);
            let start = {
                let mut idx: Vec<usize> = (0..nr).collect();
                SplitMix64::new(seed + 1000).shuffle(&mut idx);
                let mut s = vec![0; nr];
                for &v in idx.iter().skip(nr / 2) {
                    s[v] = 1;
                }
                s
            };
            let c0 = cut_nets(&h, &start);
            let res = fiduccia_mattheyses(&h, Some(start), 0.1, seed, 20);
            assert_eq!(res.cut, cut_nets(&h, &res.side));
            assert!(res.cut <= c0);
            let s0 = res.side.iter().filter(|&&s| s == 0).count();
            let cap = ((1.1 * nr as f64) / 2.0).ceil() as usize;
            assert!(s0 <= cap && nr - s0 <= cap);
            assert!(
                res.cut < c0,
                "seed {seed}: FM should improve a random split ({c0} -> {})",
                res.cut
            );
        }
    }

    #[test]
    fn communication_volume_counts_lambda_minus_one() {
        let h = Hypergraph {
            vertices: 4,
            nets: vec![vec![0, 1, 2, 3], vec![0, 1], vec![2]],
        };
        assert_eq!(communication_volume(&h, &[0, 1, 2, 2]), 2 + 1);
    }

    #[test]
    fn mis_variants_are_maximal() {
        for seed in 0..20 {
            let mut rng = SplitMix64::new(seed);
            let n = 50;
            let mut g = Graph::new(n);
            for _ in 0..120 {
                let (u, v) = (rng.below(n as u64) as usize, rng.below(n as u64) as usize);
                if u != v {
                    g.add_edge(u, v);
                }
            }
            let order: Vec<usize> = (0..n).collect();
            assert!(is_maximal_independent(&g, &greedy_mis(&g, &order)));
            let (s, rounds) = luby_mis(&g, seed);
            assert!(is_maximal_independent(&g, &s));
            assert!(rounds >= 1);
        }
    }
}
