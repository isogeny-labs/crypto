//! Exact deferred affine labels. This prototype is not a timed solver arm.
use hec_native_ic::{fp::Fp, solve::Row};
use std::collections::BTreeMap;

enum Expr {
    Leaf(Row),
    Sum(Vec<(usize, u64)>),
}

#[derive(Default, Debug)]
pub struct Counts {
    pub links: u64,
    pub nodes: u64,
    pub arcs: u64,
    pub leaf_terms: u64,
    pub expanded_nodes: u64,
    pub expanded_arcs: u64,
    pub expanded_terms: u64,
    pub emissions: u64,
}

pub struct LazyForest {
    f: Fp,
    h: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
    weight: Vec<u64>,
    beta: Vec<usize>,
    arena: Vec<Expr>,
    pub unions: u64,
    pub pins: u64,
    pub counts: Counts,
}

fn accumulate(map: &mut BTreeMap<usize, u64>, id: usize, c: u64, f: &Fp) {
    if c == 0 {
        return;
    }
    let next = f.add(map.get(&id).copied().unwrap_or(0), c);
    if next == 0 {
        map.remove(&id);
    } else {
        map.insert(id, next);
    }
}

impl LazyForest {
    pub fn new(nodes: usize, anchors: usize, ell: u64) -> Self {
        Self {
            f: Fp::new(ell),
            h: anchors,
            parent: (0..=nodes).collect(),
            size: vec![1; nodes + 1],
            weight: vec![1; nodes + 1],
            beta: vec![0; nodes + 1],
            arena: vec![Expr::Leaf(Vec::new())],
            unions: 0,
            pins: 0,
            counts: Counts::default(),
        }
    }

    fn leaf(&mut self, row: &Row) -> usize {
        for &(j, c) in row {
            assert!((j as usize) < self.h && c < self.f.p);
        }
        if row.is_empty() {
            return 0;
        }
        self.counts.nodes += 1;
        self.counts.leaf_terms += row.len() as u64;
        let id = self.arena.len();
        self.arena.push(Expr::Leaf(row.clone()));
        id
    }

    fn sum(&mut self, terms: &[(usize, u64)]) -> usize {
        let mut combined = BTreeMap::new();
        for &(id, c) in terms {
            assert!(id < self.arena.len() && c < self.f.p);
            if id != 0 {
                accumulate(&mut combined, id, c, &self.f);
            }
        }
        if combined.is_empty() {
            return 0;
        }
        if combined.len() == 1 {
            let (&id, &c) = combined.first_key_value().unwrap();
            if c == 1 {
                return id;
            }
        }
        let terms: Vec<_> = combined.into_iter().collect();
        self.counts.nodes += 1;
        self.counts.arcs += terms.len() as u64;
        let id = self.arena.len();
        self.arena.push(Expr::Sum(terms));
        id
    }

    fn find(&mut self, start: usize) -> (usize, u64, usize) {
        let (mut u, mut alpha) = (start, 1);
        let mut terms = Vec::new();
        while self.parent[u] != u {
            self.counts.links += 1;
            terms.push((self.beta[u], alpha));
            alpha = self.f.mul(alpha, self.weight[u]);
            u = self.parent[u];
        }
        let beta = self.sum(&terms);
        if start != u {
            self.parent[start] = u;
            self.weight[start] = alpha;
            self.beta[start] = beta;
        }
        (u, alpha, beta)
    }

    fn expand(&mut self, id: usize) -> Row {
        let mut pending = BTreeMap::new();
        let mut columns = BTreeMap::new();
        if id != 0 {
            pending.insert(id, 1);
        }
        while let Some((id, c)) = pending.pop_last() {
            self.counts.expanded_nodes += 1;
            match &self.arena[id] {
                Expr::Leaf(row) => {
                    self.counts.expanded_terms += row.len() as u64;
                    for &(j, v) in row {
                        accumulate(&mut columns, j as usize, self.f.mul(c, v), &self.f);
                    }
                }
                Expr::Sum(terms) => {
                    self.counts.expanded_arcs += terms.len() as u64;
                    for &(child, v) in terms {
                        assert!(child < id);
                        accumulate(&mut pending, child, self.f.mul(c, v), &self.f);
                    }
                }
            }
        }
        self.counts.emissions += 1;
        columns.into_iter().map(|(j, c)| (j as u32, c)).collect()
    }

    pub fn insert(&mut self, off: &[(usize, u64)], on: &Row) -> Option<Row> {
        assert!(off.len() <= 2);
        let mut terms = Vec::new();
        let mut roots: Vec<(usize, u64)> = Vec::new();
        for &(node, c) in off {
            assert!(node < self.parent.len() && c < self.f.p);
            let (root, a, beta) = self.find(node);
            terms.push((beta, c));
            if root != 0 {
                let v = self.f.mul(c, a);
                if let Some((_, old)) = roots.iter_mut().find(|(r, _)| *r == root) {
                    *old = self.f.add(*old, v);
                } else {
                    roots.push((root, v));
                }
            }
        }
        roots.retain(|&(_, c)| c != 0);
        // Find compression may publish expression IDs into beta. Everything
        // created after this watermark is temporary if this insertion emits.
        let watermark = self.arena.len();
        let leaf = self.leaf(on);
        terms.push((leaf, 1));
        let hot = self.sum(&terms);
        if roots.is_empty() {
            let row = self.expand(hot);
            self.arena.truncate(watermark);
            return Some(row);
        }
        if roots.len() == 2 && self.size[roots[0].0] > self.size[roots[1].0] {
            roots.swap(0, 1);
        }
        let (u, a) = roots[0];
        let s = self.f.neg(self.f.inv(a));
        if roots.len() == 1 {
            self.parent[u] = 0;
            self.weight[u] = 0;
            self.pins += 1;
        } else {
            let (v, b) = roots[1];
            self.parent[u] = v;
            self.weight[u] = self.f.mul(s, b);
            self.size[v] += self.size[u];
        }
        self.beta[u] = self.sum(&[(hot, s)]);
        self.unions += 1;
        None
    }

    /// Retained vector capacities, including unreachable expressions. Excludes
    /// allocator metadata and transient maps; never label this peak RSS.
    pub fn retained_bytes(&self) -> usize {
        (self.parent.capacity() + self.size.capacity() + self.beta.capacity())
            * std::mem::size_of::<usize>()
            + self.weight.capacity() * 8
            + self.arena.capacity() * std::mem::size_of::<Expr>()
            + self
                .arena
                .iter()
                .map(|e| match e {
                    Expr::Leaf(r) => r.capacity() * std::mem::size_of::<(u32, u64)>(),
                    Expr::Sum(t) => t.capacity() * std::mem::size_of::<(usize, u64)>(),
                })
                .sum::<usize>()
    }
}

#[cfg(test)]
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod reference;

#[cfg(test)]
mod tests {
    use super::*;
    use hec_native_ic::rng::Rng;

    #[test]
    fn every_prefix_matches_dense() {
        let mut inserts = 0;
        let mut streams = 0;
        for ell in [101, 1009] {
            let f = Fp::new(ell);
            for h in [0, 1, 3, 19, 127] {
                for n in [1, 7, 63] {
                    for seed in 0..4 {
                        for planted in [false, true] {
                            if planted && h == 0 {
                                continue;
                            }
                            streams += 1;
                            let mut rng = Rng::new(seed + 7011);
                            let anchors: Vec<_> = (0..h).map(|_| 1 + rng.below(ell - 1)).collect();
                            let mut values: Vec<_> = (0..=n).map(|_| rng.below(ell)).collect();
                            values[0] = 0;
                            let mut dense = reference::DenseForest::new(n, h, ell);
                            let mut lazy = LazyForest::new(n, h, ell);
                            for step in 0..512 {
                                let off: Vec<_> = (0..rng.below(3))
                                    .map(|_| (rng.below((n + 1) as u64) as usize, rng.below(ell)))
                                    .collect();
                                let mut on: Row = if h == 0 {
                                    Vec::new()
                                } else {
                                    (0..rng.below(5))
                                        .map(|_| (rng.below(h as u64) as u32, rng.below(ell)))
                                        .collect()
                                };
                                if planted {
                                    let mut value = 0;
                                    for &(u, c) in &off {
                                        value = f.add(value, f.mul(c, values[u]));
                                    }
                                    for &(j, c) in &on {
                                        value = f.add(value, f.mul(c, anchors[j as usize]));
                                    }
                                    on.push((0, f.mul(f.neg(value), f.inv(anchors[0]))));
                                }
                                let expected = dense.insert(&off, &on);
                                let got = lazy.insert(&off, &on);
                                assert_eq!(got, expected, "ell={ell} h={h} n={n} seed={seed} step={step} planted={planted}");
                                assert_eq!((lazy.unions, lazy.pins), (dense.unions, dense.pins));
                                if planted {
                                    if let Some(row) = got {
                                        assert_eq!(
                                            row.iter().fold(0, |s, &(j, c)| f
                                                .add(s, f.mul(c, anchors[j as usize]))),
                                            0
                                        );
                                    }
                                }
                                inserts += 1;
                            }
                            println!("COUNTS ell={ell} h={h} n={n} seed={seed} planted={planted} dense_bytes={} lazy_retained_bytes={} {:?}", dense.bytes(), lazy.retained_bytes(), lazy.counts);
                        }
                    }
                }
            }
        }
        println!("GATE streams={streams} inserts={inserts}");
    }

    #[test]
    fn explicit_controls_and_shared_expression_cancellation() {
        let mut lazy = LazyForest::new(3, 2, 101);
        let mut dense = reference::DenseForest::new(3, 2, 101);
        let cases = [
            (vec![], vec![]),
            (vec![], vec![(1, 4), (1, 97)]),
            (vec![(1, 3), (2, 5)], vec![(0, 7)]),
            (vec![(2, 1), (2, 100)], vec![(1, 9)]),
            (vec![(1, 2), (2, 6)], vec![(0, 4)]),
            (vec![(2, 7)], vec![(1, 3)]),
            (vec![(3, 1), (0, 99)], vec![(0, 2)]),
            (vec![(3, 1)], vec![(0, 2)]),
        ];
        for (off, on) in cases {
            assert_eq!(lazy.insert(&off, &on), dense.insert(&off, &on));
            assert_eq!((lazy.unions, lazy.pins), (dense.unions, dense.pins));
        }
        let leaf = lazy.leaf(&vec![(0, 11), (1, 7)]);
        let shared = lazy.sum(&[(leaf, 4)]);
        let a = lazy.sum(&[(shared, 3), (leaf, 2)]);
        let b = lazy.sum(&[(shared, 98), (leaf, 99)]);
        let zero = lazy.sum(&[(a, 1), (b, 1)]);
        assert!(lazy.expand(zero).is_empty());
        assert_eq!(lazy.sum(&[(leaf, 0)]), 0);
    }
}
