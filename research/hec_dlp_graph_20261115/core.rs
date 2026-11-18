use crate::dense;
use hec_native_ic::{fp::Fp, solve::Row};
use std::collections::HashSet;
pub struct Core {
    pub forest: dense::DenseForest,
    pub echelon: dense::Echelon,
    pub rows: u64,
    pub empty: u64,
    pub unique: u64,
    pub entries: u64,
    pub k: Option<u64>,
    pub solved_at: u64,
    seen: HashSet<Row>,
    f: Fp,
    h: usize,
}
impl Core {
    pub fn new(n: usize, h: usize, ell: u64) -> Self {
        Self {
            forest: dense::DenseForest::new(n, h + 2, ell),
            echelon: dense::Echelon::new(h, ell),
            rows: 0,
            empty: 0,
            unique: 0,
            entries: 0,
            k: None,
            solved_at: 0,
            seen: HashSet::new(),
            f: Fp::new(ell),
            h,
        }
    }
    pub fn feed(&mut self, off: &[(usize, u64)], on: &Row, trial: u64) -> Option<u64> {
        let Some(mut row) = self.forest.insert(off, on) else {
            return None;
        };
        self.rows += 1;
        self.entries += row.len() as u64;
        if row.is_empty() {
            self.empty += 1;
            return None;
        }
        let inv = self.f.inv(row[0].1);
        for (_, v) in &mut row {
            *v = self.f.mul(*v, inv);
        }
        if !self.seen.insert(row.clone()) {
            return None;
        }
        self.unique += 1;
        if self.echelon.rank == self.h + 1 {
            return None;
        }
        let mut rhs = 0;
        let mut eq = Row::new();
        for (j, v) in row {
            if j as usize == self.h {
                rhs = self.f.neg(v);
            } else {
                eq.push((
                    if j as usize == self.h + 1 {
                        self.h as u32
                    } else {
                        j
                    },
                    v,
                ));
            }
        }
        let k = self.echelon.insert(&eq, rhs);
        if let Some(k) = k {
            if let Some(old) = self.k {
                assert_eq!(old, k);
            } else {
                self.k = Some(k);
                self.solved_at = trial;
                return Some(k);
            }
        }
        None
    }
}
