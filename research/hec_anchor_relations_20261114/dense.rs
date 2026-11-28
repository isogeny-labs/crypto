//! Incremental elimination for the bounded anchor core, target column last.
use hec_native_ic::{fp::Fp, solve::Row};

/// Affine union-find with bounded flat anchor potentials. Scratch vectors are
/// reused; allocation occurs only when emitting a nonempty pure-anchor row.
pub struct DenseForest {
    f: Fp,
    h: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
    weight: Vec<u64>,
    beta: Vec<u64>,
    found: Vec<u64>,
    hot: Vec<u64>,
    pub unions: u64,
    pub pins: u64,
}

impl DenseForest {
    pub fn new(nodes: usize, anchors: usize, ell: u64) -> Self {
        let words = (nodes + 1)
            .checked_mul(anchors)
            .expect("forest size overflow");
        assert!(words <= 16_000_000, "dense forest storage cap");
        Self {
            f: Fp::new(ell),
            h: anchors,
            parent: (0..=nodes).collect(),
            size: vec![1; nodes + 1],
            weight: vec![1; nodes + 1],
            beta: vec![0; words],
            found: vec![0; anchors],
            hot: vec![0; anchors],
            unions: 0,
            pins: 0,
        }
    }

    fn product(&self, a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            0
        } else if a == 1 {
            b
        } else if a == self.f.p - 1 {
            self.f.neg(b)
        } else {
            self.f.mul(a, b)
        }
    }

    /// Writes the accumulated potential, then compresses the starting node.
    fn find(&mut self, start: usize) -> (usize, u64) {
        self.found.fill(0);
        let (mut u, mut alpha) = (start, 1);
        while self.parent[u] != u {
            for j in 0..self.h {
                self.found[j] = self.f.add(
                    self.found[j],
                    self.product(alpha, self.beta[u * self.h + j]),
                );
            }
            alpha = self.product(alpha, self.weight[u]);
            u = self.parent[u];
        }
        if start != u {
            self.parent[start] = u;
            self.weight[start] = alpha;
            self.beta[start * self.h..(start + 1) * self.h].copy_from_slice(&self.found);
        }
        (u, alpha)
    }

    pub fn bytes(&self) -> usize {
        (self.beta.len() + self.weight.len() + self.found.len() + self.hot.len()) * 8
            + (self.parent.len() + self.size.len()) * std::mem::size_of::<usize>()
    }

    pub fn insert(&mut self, off: &[(usize, u64)], on: &Row) -> Option<Row> {
        assert!(off.len() <= 2);
        self.hot.fill(0);
        for &(j, v) in on {
            self.hot[j as usize] = self.f.add(self.hot[j as usize], v);
        }
        let mut roots = [(0usize, 0u64); 2];
        let mut count = 0;
        for &(node, c) in off {
            let (root, a) = self.find(node);
            for j in 0..self.h {
                self.hot[j] = self.f.add(self.hot[j], self.product(c, self.found[j]));
            }
            if root == 0 {
                continue;
            }
            let v = self.product(c, a);
            if let Some(index) = (0..count).find(|&j| roots[j].0 == root) {
                roots[index].1 = self.f.add(roots[index].1, v);
            } else {
                roots[count] = (root, v);
                count += 1;
            }
        }
        let mut nonzero = 0;
        for j in 0..count {
            if roots[j].1 != 0 {
                roots[nonzero] = roots[j];
                nonzero += 1;
            }
        }
        if nonzero == 0 {
            return Some(
                self.hot
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &v)| (v != 0).then_some((j as u32, v)))
                    .collect(),
            );
        }
        if nonzero == 2 && self.size[roots[0].0] > self.size[roots[1].0] {
            roots.swap(0, 1);
        }
        let (u, a) = roots[0];
        let s = self.f.neg(self.f.inv(a));
        if nonzero == 1 {
            self.parent[u] = 0;
            self.weight[u] = 0;
            self.pins += 1;
        } else {
            let (v, b) = roots[1];
            self.parent[u] = v;
            self.weight[u] = self.product(s, b);
            self.size[v] += self.size[u];
        }
        for j in 0..self.h {
            self.beta[u * self.h + j] = self.product(s, self.hot[j]);
        }
        self.unions += 1;
        None
    }
}

pub struct Echelon {
    f: Fp,
    columns: usize,
    pivots: Vec<Option<Vec<u64>>>,
    pub rank: usize,
    pub inserted: usize,
}

impl Echelon {
    pub fn new(anchors: usize, ell: u64) -> Self {
        Self {
            f: Fp::new(ell),
            columns: anchors + 1,
            pivots: vec![None; anchors + 1],
            rank: 0,
            inserted: 0,
        }
    }

    pub fn insert(&mut self, row: &Row, rhs: u64) -> Option<u64> {
        let mut dense = vec![0; self.columns + 1];
        for &(j, v) in row {
            assert!((j as usize) < self.columns);
            dense[j as usize] = self.f.add(dense[j as usize], v);
        }
        dense[self.columns] = rhs;
        self.inserted += 1;
        for j in 0..self.columns {
            let v = dense[j];
            if v == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[j] {
                for k in j..=self.columns {
                    dense[k] = self.f.sub(dense[k], self.f.mul(v, pivot[k]));
                }
            } else {
                let inverse = self.f.inv(v);
                for a in &mut dense[j..] {
                    *a = self.f.mul(*a, inverse);
                }
                self.pivots[j] = Some(dense);
                self.rank += 1;
                return self.target();
            }
        }
        assert_eq!(dense[self.columns], 0, "inconsistent anchor system");
        self.target()
    }

    pub fn target(&self) -> Option<u64> {
        self.pivots[self.columns - 1]
            .as_ref()
            .map(|p| p[self.columns])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hec_native_ic::{
        rng::Rng,
        solve::{solve_fast, SolveStats},
    };

    #[test]
    fn dense_affine_forest_preserves_arbitrary_coefficient_invariants() {
        let f = Fp::new(101);
        let anchors = [7, 29, 33];
        let values = [0, 13, 21, 44, 53, 87];
        let mut forest = DenseForest::new(5, 3, f.p);
        let mut rng = Rng::new(7654);
        for _ in 0..200 {
            let u = 1 + rng.below(5) as usize;
            let v = 1 + rng.below(5) as usize;
            let a = 1 + rng.below(100);
            let b = 1 + rng.below(100);
            let rhs = f.neg(f.add(f.mul(a, values[u]), f.mul(b, values[v])));
            if let Some(row) =
                forest.insert(&[(u, a), (v, b)], &vec![(0, f.mul(rhs, f.inv(anchors[0])))])
            {
                assert_eq!(
                    row.iter()
                        .fold(0, |s, &(j, v)| f.add(s, f.mul(v, anchors[j as usize]))),
                    0
                );
            }
            for node in 1..=5 {
                let (root, alpha) = forest.find(node);
                let beta = forest
                    .found
                    .iter()
                    .zip(anchors)
                    .fold(0, |s, (&a, b)| f.add(s, f.mul(a, b)));
                assert_eq!(values[node], f.add(f.mul(alpha, values[root]), beta));
            }
        }
    }

    #[test]
    fn incremental_target_matches_sparse_solver_at_every_prefix() {
        let f = Fp::new(1009);
        let mut rng = Rng::new(20261115);
        for anchors in [2, 7, 19] {
            for _ in 0..20 {
                let values: Vec<_> = (0..=anchors).map(|_| rng.below(f.p)).collect();
                let mut solver = Echelon::new(anchors, f.p);
                let mut rows = Vec::new();
                for step in 0..(anchors + 1) * 3 {
                    let row: Row = if step < anchors + 1 {
                        // Dense random rows exercise cancellation and arbitrary pivots.
                        (0..=anchors)
                            .map(|j| (j as u32, rng.below(f.p)))
                            .filter(|e| e.1 != 0)
                            .collect()
                    } else {
                        // Guaranteed rank completion, including target-only determination.
                        vec![((step % (anchors + 1)) as u32, 1)]
                    };
                    let rhs = row
                        .iter()
                        .fold(0, |s, &(j, v)| f.add(s, f.mul(v, values[j as usize])));
                    rows.push((row.clone(), rhs));
                    let got = solver.insert(&row, rhs);
                    let reference = solve_fast(&rows, anchors, f.p, &mut SolveStats::default());
                    assert_eq!(got, reference, "anchors={anchors} prefix={step}");
                    if let Some(k) = got {
                        assert_eq!(k, values[anchors]);
                    }
                }
                assert_eq!(solver.target(), Some(values[anchors]));
            }
        }
    }
}
