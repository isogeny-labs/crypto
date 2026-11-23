//! Report section 4: 2-SAT through implication SCCs (Aspvall-Plass-Tarjan,
//! `O(s + c)`) and #SAT by dynamic programming over a tree decomposition of
//! the incidence graph.
//!
//! The #SAT routine keeps, per bag, an assignment of the bag's variables and
//! a "satisfied so far" bit per bag clause; join nodes combine clause bits by
//! OR-convolution computed with zeta/Moebius transforms, which is the
//! transform used by Slivovsky-Szeider for their `O*(2^t)` bound. The table
//! here is indexed by all `2^{|bag|}` states, so this implementation's
//! counted cost is `O*(2^t)` per node with a supplied width-`t` incidence
//! decomposition. Every encoded variable and clause, auxiliaries included,
//! must be counted in that width.

use super::decomposition::{min_degree_decomposition, AdjSets, TreeDecomposition};
use super::matching::tarjan_scc;
use std::collections::BTreeSet;

/// A literal: variable index and polarity (`true` = positive).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Lit {
    pub var: usize,
    pub positive: bool,
}

impl Lit {
    pub fn pos(var: usize) -> Self {
        Self {
            var,
            positive: true,
        }
    }
    pub fn neg(var: usize) -> Self {
        Self {
            var,
            positive: false,
        }
    }
    fn node(self) -> usize {
        2 * self.var + usize::from(!self.positive)
    }
    fn negate(self) -> Self {
        Self {
            var: self.var,
            positive: !self.positive,
        }
    }
    pub fn holds(self, value: bool) -> bool {
        value == self.positive
    }
}

/// Solve 2-SAT; returns a satisfying assignment or `None`.
pub fn two_sat(vars: usize, clauses: &[(Lit, Lit)]) -> Option<Vec<bool>> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); 2 * vars];
    for &(a, b) in clauses {
        // (a or b): not a -> b, not b -> a.
        adj[a.negate().node()].push(b.node());
        adj[b.negate().node()].push(a.node());
    }
    let comps = tarjan_scc(&adj);
    let mut comp = vec![0usize; 2 * vars];
    for (i, c) in comps.iter().enumerate() {
        for &v in c {
            comp[v] = i;
        }
    }
    let mut out = vec![false; vars];
    for x in 0..vars {
        let (p, n) = (Lit::pos(x).node(), Lit::neg(x).node());
        if comp[p] == comp[n] {
            return None;
        }
        // Tarjan numbers components in reverse topological order.
        out[x] = comp[p] < comp[n];
    }
    Some(out)
}

/// CNF formula.
#[derive(Clone, Debug)]
pub struct Cnf {
    pub vars: usize,
    pub clauses: Vec<Vec<Lit>>,
}

impl Cnf {
    pub fn satisfied_by(&self, a: &[bool]) -> bool {
        self.clauses
            .iter()
            .all(|c| c.iter().any(|l| l.holds(a[l.var])))
    }
    /// Incidence graph: variables `0..n`, clauses `n..n+m`.
    pub fn incidence_graph(&self) -> AdjSets {
        let n = self.vars;
        let mut g = vec![BTreeSet::new(); n + self.clauses.len()];
        for (j, c) in self.clauses.iter().enumerate() {
            for l in c {
                g[l.var].insert(n + j);
                g[n + j].insert(l.var);
            }
        }
        g
    }
}

#[derive(Clone, Debug)]
enum Nice {
    Leaf,
    Introduce(usize, usize), // (vertex, child)
    Forget(usize, usize),
    Join(usize, usize),
}

struct NiceTd {
    nodes: Vec<(Nice, Vec<usize>)>, // (kind, sorted bag); children precede parents
}

fn build_nice(td: &TreeDecomposition, n_vertices: usize) -> NiceTd {
    let nb = td.bags.len();
    let mut nodes: Vec<(Nice, Vec<usize>)> = Vec::new();
    if nb == 0 {
        nodes.push((Nice::Leaf, Vec::new()));
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); nb];
    for &(a, b) in &td.edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let bags: Vec<Vec<usize>> = td
        .bags
        .iter()
        .map(|b| {
            let mut s = b.clone();
            s.sort_unstable();
            s.dedup();
            s
        })
        .collect();
    // Change the bag of node `cur` (with bag `from`) to `to` by forgets then introduces.
    fn morph(
        nodes: &mut Vec<(Nice, Vec<usize>)>,
        mut cur: usize,
        from: &[usize],
        to: &[usize],
    ) -> usize {
        let mut bag: Vec<usize> = from.to_vec();
        for &v in from {
            if to.binary_search(&v).is_err() {
                bag.retain(|&x| x != v);
                nodes.push((Nice::Forget(v, cur), bag.clone()));
                cur = nodes.len() - 1;
            }
        }
        for &v in to {
            if bag.binary_search(&v).is_err() {
                let p = bag.partition_point(|&x| x < v);
                bag.insert(p, v);
                nodes.push((Nice::Introduce(v, cur), bag.clone()));
                cur = nodes.len() - 1;
            }
        }
        cur
    }
    let mut visited = vec![false; nb];
    let mut roots: Vec<usize> = Vec::new();
    for root in 0..nb {
        if visited[root] {
            continue;
        }
        // Iterative post-order over the bag tree.
        let mut order = Vec::new();
        let mut parent = vec![usize::MAX; nb];
        let mut stack = vec![root];
        visited[root] = true;
        while let Some(b) = stack.pop() {
            order.push(b);
            for &c in &adj[b] {
                if !visited[c] {
                    visited[c] = true;
                    parent[c] = b;
                    stack.push(c);
                }
            }
        }
        let mut built: Vec<usize> = vec![usize::MAX; nb];
        for &b in order.iter().rev() {
            let children: Vec<usize> = adj[b].iter().copied().filter(|&c| parent[c] == b).collect();
            let mut subs: Vec<usize> = Vec::new();
            for c in children {
                subs.push(morph(&mut nodes, built[c], &bags[c], &bags[b]));
            }
            let node = if subs.is_empty() {
                nodes.push((Nice::Leaf, Vec::new()));
                let leaf = nodes.len() - 1;
                morph(&mut nodes, leaf, &[], &bags[b])
            } else {
                let mut cur = subs[0];
                for &s in &subs[1..] {
                    nodes.push((Nice::Join(cur, s), bags[b].clone()));
                    cur = nodes.len() - 1;
                }
                cur
            };
            built[b] = node;
        }
        let top = morph(&mut nodes, built[root], &bags[root], &[]);
        roots.push(top);
    }
    // Combine several roots (a forest decomposition) with empty-bag joins;
    // the final node is always the overall root.
    let _ = n_vertices;
    if let Some((&first, rest)) = roots.split_first() {
        let mut cur = first;
        for &r in rest {
            nodes.push((Nice::Join(cur, r), Vec::new()));
            cur = nodes.len() - 1;
        }
        debug_assert_eq!(cur, nodes.len() - 1);
    }
    NiceTd { nodes }
}

/// Count satisfying assignments with a supplied tree decomposition of the
/// incidence graph (validated first), or a min-degree heuristic one when
/// `td` is `None`. Counts are exact while below `2^128`.
pub fn count_models(cnf: &Cnf, td: Option<&TreeDecomposition>) -> Result<u128, String> {
    let g = cnf.incidence_graph();
    let owned;
    let td = match td {
        Some(t) => t,
        None => {
            owned = min_degree_decomposition(&g);
            &owned
        }
    };
    td.validate(&g)?;
    if td.width() >= 40 {
        return Err(format!(
            "incidence width {} too large for the dense table",
            td.width()
        ));
    }
    let n = cnf.vars;
    let nice = build_nice(td, g.len());
    let is_var = |v: usize| v < n;
    // Literal lookup: polarity of var x in clause j (Some(true/false/both)).
    let lit_of = |j: usize, x: usize| -> (bool, bool) {
        let mut pos = false;
        let mut neg = false;
        for l in &cnf.clauses[j] {
            if l.var == x {
                if l.positive {
                    pos = true;
                } else {
                    neg = true;
                }
            }
        }
        (pos, neg)
    };
    let satisfies = |j: usize, x: usize, val: bool| -> bool {
        let (p, q) = lit_of(j, x);
        (val && p) || (!val && q)
    };
    // Tables: dense Vec<u128> of length 2^{|bag|}, bit i <-> bag position i.
    let mut tables: Vec<Vec<u128>> = Vec::with_capacity(nice.nodes.len());
    for (kind, bag) in &nice.nodes {
        let t: Vec<u128> = match *kind {
            Nice::Leaf => vec![1],
            Nice::Introduce(v, c) => {
                let child = &tables[c];
                let cbag = &nice.nodes[c].1;
                let pos_v = bag.binary_search(&v).unwrap();
                let mut t = vec![0u128; 1 << bag.len()];
                for (s, &cnt) in child.iter().enumerate() {
                    if cnt == 0 {
                        continue;
                    }
                    // Spread child bits into new positions (skip pos_v).
                    let lo = s & ((1 << pos_v) - 1);
                    let hi = (s >> pos_v) << (pos_v + 1);
                    let base = lo | hi;
                    if is_var(v) {
                        for val in [false, true] {
                            let mut key = base | (usize::from(val) << pos_v);
                            for (i, &u) in bag.iter().enumerate() {
                                if !is_var(u) && satisfies(u - n, v, val) {
                                    key |= 1 << i;
                                }
                            }
                            t[key] = t[key].wrapping_add(cnt);
                        }
                    } else {
                        let j = v - n;
                        let mut sat = false;
                        for (i, &u) in cbag.iter().enumerate() {
                            if is_var(u) && satisfies(j, u, s >> i & 1 == 1) {
                                sat = true;
                            }
                        }
                        let key = base | (usize::from(sat) << pos_v);
                        t[key] = t[key].wrapping_add(cnt);
                    }
                }
                t
            }
            Nice::Forget(v, c) => {
                let child = &tables[c];
                let cbag = &nice.nodes[c].1;
                let pos_v = cbag.binary_search(&v).unwrap();
                let mut t = vec![0u128; 1 << bag.len()];
                for (s, &cnt) in child.iter().enumerate() {
                    if cnt == 0 {
                        continue;
                    }
                    let bit = s >> pos_v & 1 == 1;
                    if !is_var(v) && !bit {
                        continue; // clause forgotten unsatisfied
                    }
                    let key = (s & ((1 << pos_v) - 1)) | ((s >> (pos_v + 1)) << pos_v);
                    t[key] = t[key].wrapping_add(cnt);
                }
                t
            }
            Nice::Join(a, b) => {
                let (ta, tb) = (&tables[a], &tables[b]);
                let clause_mask: usize = bag
                    .iter()
                    .enumerate()
                    .filter(|&(_, &u)| !is_var(u))
                    .fold(0, |m, (i, _)| m | (1 << i));
                let var_mask = ((1usize << bag.len()) - 1) & !clause_mask;
                let mut t = vec![0u128; 1 << bag.len()];
                // For each variable assignment, OR-convolve over clause bits
                // via zeta (subset sums) and Moebius transforms.
                let clause_bits: Vec<usize> = (0..bag.len())
                    .filter(|&i| clause_mask >> i & 1 == 1)
                    .collect();
                let k = clause_bits.len();
                let expand = |sub: usize| -> usize {
                    clause_bits
                        .iter()
                        .enumerate()
                        .fold(0, |m, (bi, &pos)| m | (((sub >> bi) & 1) << pos))
                };
                let mut fa = vec![0u128; 1 << k];
                let mut fb = vec![0u128; 1 << k];
                let mut asg = var_mask;
                loop {
                    let vars_key = asg & var_mask;
                    for sub in 0..(1usize << k) {
                        let key = vars_key | expand(sub);
                        fa[sub] = ta[key];
                        fb[sub] = tb[key];
                    }
                    for bi in 0..k {
                        for sub in 0..(1usize << k) {
                            if sub >> bi & 1 == 1 {
                                fa[sub] = fa[sub].wrapping_add(fa[sub ^ (1 << bi)]);
                                fb[sub] = fb[sub].wrapping_add(fb[sub ^ (1 << bi)]);
                            }
                        }
                    }
                    for sub in 0..(1usize << k) {
                        fa[sub] = fa[sub].wrapping_mul(fb[sub]);
                    }
                    for bi in 0..k {
                        for sub in 0..(1usize << k) {
                            if sub >> bi & 1 == 1 {
                                fa[sub] = fa[sub].wrapping_sub(fa[sub ^ (1 << bi)]);
                            }
                        }
                    }
                    for sub in 0..(1usize << k) {
                        t[vars_key | expand(sub)] = fa[sub];
                    }
                    if asg == 0 {
                        break;
                    }
                    asg = (asg - 1) & var_mask;
                }
                t
            }
        };
        tables.push(t);
    }
    let last = tables.last().ok_or("empty decomposition")?;
    Ok(last[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    fn random_cnf(rng: &mut SplitMix64, vars: usize, clauses: usize, width: usize) -> Cnf {
        Cnf {
            vars,
            clauses: (0..clauses)
                .map(|_| {
                    (0..1 + rng.below(width as u64) as usize)
                        .map(|_| Lit {
                            var: rng.below(vars as u64) as usize,
                            positive: rng.below(2) == 1,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn brute_count(cnf: &Cnf) -> u128 {
        (0u32..(1 << cnf.vars))
            .filter(|&m| {
                let a: Vec<bool> = (0..cnf.vars).map(|i| m >> i & 1 == 1).collect();
                cnf.satisfied_by(&a)
            })
            .count() as u128
    }

    #[test]
    fn two_sat_matches_brute_force() {
        let mut rng = SplitMix64::new(3);
        for _ in 0..300 {
            let vars = 1 + rng.below(8) as usize;
            let m = rng.below(20) as usize;
            let clauses: Vec<(Lit, Lit)> = (0..m)
                .map(|_| {
                    let a = Lit {
                        var: rng.below(vars as u64) as usize,
                        positive: rng.below(2) == 1,
                    };
                    let b = Lit {
                        var: rng.below(vars as u64) as usize,
                        positive: rng.below(2) == 1,
                    };
                    (a, b)
                })
                .collect();
            let cnf = Cnf {
                vars,
                clauses: clauses.iter().map(|&(a, b)| vec![a, b]).collect(),
            };
            let sol = two_sat(vars, &clauses);
            let sat = brute_count(&cnf) > 0;
            assert_eq!(sol.is_some(), sat);
            if let Some(a) = sol {
                assert!(cnf.satisfied_by(&a));
            }
        }
    }

    #[test]
    fn model_count_matches_brute_force() {
        let mut rng = SplitMix64::new(17);
        for trial in 0..150 {
            let vars = 1 + rng.below(11) as usize;
            let m = rng.below(14) as usize;
            let cnf = random_cnf(&mut rng, vars, m, 3);
            let got = count_models(&cnf, None).unwrap();
            assert_eq!(got, brute_count(&cnf), "trial {trial}");
        }
        // Empty clause: zero models.
        let cnf = Cnf {
            vars: 2,
            clauses: vec![vec![]],
        };
        assert_eq!(count_models(&cnf, None).unwrap(), 0);
        // No clauses: 2^n.
        let cnf = Cnf {
            vars: 5,
            clauses: vec![],
        };
        assert_eq!(count_models(&cnf, None).unwrap(), 32);
    }

    #[test]
    fn model_count_on_long_chain_uses_small_width() {
        // (x_i or not x_{i+1}) chain of 60 variables: count is 61.
        let n = 60;
        let cnf = Cnf {
            vars: n,
            clauses: (0..n - 1)
                .map(|i| vec![Lit::pos(i), Lit::neg(i + 1)])
                .collect(),
        };
        let g = cnf.incidence_graph();
        let td = min_degree_decomposition(&g);
        assert!(td.width() <= 2);
        assert_eq!(count_models(&cnf, Some(&td)).unwrap(), 61);
    }
}
