//! Incremental elimination for the bounded anchor core, target column last.
use hec_native_ic::{fp::Fp, solve::Row};

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
