use crate::arithmetic::inv;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
static SPARSE: AtomicBool = AtomicBool::new(false);
pub fn set_sparse(x: bool) {
    SPARSE.store(x, Ordering::Relaxed)
}
pub fn sparse() -> bool {
    SPARSE.load(Ordering::Relaxed)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fiber {
    pub rank: usize,
    pub particular: Vec<u64>,
    pub kernel: Vec<Vec<u64>>,
    pub inconsistent: bool,
}
#[derive(Clone)]
pub struct Rref {
    pub p: u64,
    pub n: usize,
    pub pivots: Vec<Option<Vec<u64>>>,
    pub inconsistent: bool,
}
impl Rref {
    pub fn new(p: u64, n: usize) -> Self {
        Self {
            p,
            n,
            pivots: vec![None; n],
            inconsistent: false,
        }
    }
    pub fn add(&mut self, mut r: Vec<u64>) {
        assert_eq!(r.len(), self.n + 1);
        let p = self.p;
        for c in 0..self.n {
            if r[c] != 0 {
                if let Some(a) = &self.pivots[c] {
                    let m = r[c];
                    for j in c..=self.n {
                        r[j] = (r[j] + p - m * a[j] % p) % p;
                    }
                }
            }
        }
        let Some(c) = (0..self.n).find(|i| r[*i] != 0) else {
            if r[self.n] != 0 {
                self.inconsistent = true;
            }
            return;
        };
        let s = inv(r[c], p);
        for a in &mut r[c..] {
            *a = *a * s % p;
        }
        for a in self.pivots.iter_mut().flatten() {
            let m = a[c];
            if m > 0 {
                for j in c..=self.n {
                    a[j] = (a[j] + p - m * r[j] % p) % p;
                }
            }
        }
        self.pivots[c] = Some(r);
    }
    pub fn fiber(&self) -> Fiber {
        let mut x = vec![0; self.n];
        let mut rank = 0;
        for (i, row) in self.pivots.iter().enumerate() {
            if let Some(r) = row {
                x[i] = r[self.n];
                rank += 1;
            }
        }
        let mut kernel = vec![];
        for c in 0..self.n {
            if self.pivots[c].is_some() {
                continue;
            }
            let mut v = vec![0; self.n];
            v[c] = 1;
            for (i, row) in self.pivots.iter().enumerate() {
                if let Some(r) = row {
                    v[i] = (self.p - r[c]) % self.p;
                }
            }
            kernel.push(v);
        }
        Fiber {
            rank,
            particular: x,
            kernel,
            inconsistent: self.inconsistent,
        }
    }
    pub fn target(&self) -> Option<u64> {
        if self.inconsistent {
            None
        } else {
            self.pivots[self.n - 1].as_ref().map(|r| r[self.n])
        }
    }
}
pub fn satisfies(row: &[u64], x: &[u64], p: u64) -> bool {
    row[..x.len()]
        .iter()
        .zip(x)
        .fold(0, |s, (a, b)| (s + a * b) % p)
        == row[x.len()]
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compression {
    pub p: u64,
    pub base: Vec<u64>,
    pub columns: Vec<Vec<u64>>,
    #[serde(skip)]
    support: Option<Vec<Vec<(usize, u64)>>>,
}
impl Compression {
    pub fn new(p: u64, b: usize, known: &[Vec<u64>]) -> Self {
        Self::new_mode(p, b, known, sparse())
    }
    pub fn new_mode(p: u64, b: usize, known: &[Vec<u64>], enabled: bool) -> Self {
        let mut m = Rref::new(p, b);
        for r in known {
            m.add(r.clone());
        }
        let f = m.fiber();
        assert!(!f.inconsistent);
        let support = enabled.then(|| {
            let mut z = vec![vec![]; b];
            for (j, col) in f.kernel.iter().enumerate() {
                for (i, c) in col.iter().enumerate() {
                    if *c != 0 {
                        z[i].push((j, *c));
                    }
                }
            }
            z
        });
        Self {
            p,
            base: f.particular,
            columns: f.kernel,
            support,
        }
    }
    pub fn row(&self, row: &[u64]) -> Vec<u64> {
        let b = self.base.len();
        assert_eq!(row.len(), b + 2);
        if let Some(support) = &self.support {
            let k = self.columns.len();
            let mut r = vec![0; k + 2];
            let mut c = 0;
            for (i, a) in row[..b].iter().enumerate() {
                if *a == 0 {
                    continue;
                }
                for (j, z) in &support[i] {
                    r[*j] = (r[*j] + a * z) % self.p;
                }
                if self.base[i] != 0 {
                    c = (c + a * self.base[i]) % self.p;
                }
            }
            r[k] = row[b];
            r[k + 1] = (row[b + 1] + self.p - c) % self.p;
            return r;
        }
        let mut r = Vec::with_capacity(self.columns.len() + 2);
        for v in &self.columns {
            r.push((0..b).fold(0, |s, i| (s + row[i] * v[i]) % self.p));
        }
        r.push(row[b]);
        let c = (0..b).fold(0, |s, i| (s + row[i] * self.base[i]) % self.p);
        r.push((row[b + 1] + self.p - c) % self.p);
        r
    }
    pub fn expand(&self, x: &[u64], affine: bool) -> Vec<u64> {
        assert_eq!(x.len(), self.columns.len() + 1);
        let mut v = if affine {
            self.base.clone()
        } else {
            vec![0; self.base.len()]
        };
        for (c, a) in self.columns.iter().zip(x) {
            for (i, z) in c.iter().enumerate() {
                v[i] = (v[i] + a * z) % self.p;
            }
        }
        v.push(*x.last().unwrap());
        v
    }
    pub fn fiber(&self, f: &Fiber, known_rank: usize) -> Fiber {
        Fiber {
            rank: f.rank + known_rank,
            particular: self.expand(&f.particular, true),
            kernel: f.kernel.iter().map(|x| self.expand(x, false)).collect(),
            inconsistent: f.inconsistent,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deficient_target_and_ambiguity() {
        let mut a = Rref::new(101, 3);
        a.add(vec![0, 0, 1, 37]);
        assert_eq!(a.target(), Some(37));
        assert_eq!(a.fiber().kernel.len(), 2);
        let mut b = Rref::new(101, 3);
        b.add(vec![1, 0, 1, 37]);
        assert_eq!(b.target(), None);
        assert!(b.fiber().kernel.iter().any(|v| v[2] != 0));
    }
    #[test]
    fn compression_is_full_affine_parameterization() {
        let known = vec![vec![1, 0, 0, 1], vec![0, 1, 99, 0]];
        let c = Compression::new(101, 3, &known);
        let rows = [vec![1, 3, 2, 100, 9], vec![0, 4, 5, 100, 17]];
        let mut a = Rref::new(101, 4);
        for r in &known {
            let mut q = r[..3].to_vec();
            q.push(0);
            q.push(r[3]);
            a.add(q);
        }
        let mut b = Rref::new(101, c.columns.len() + 1);
        for r in &rows {
            a.add(r.clone());
            b.add(c.row(r));
        }
        let bf = c.fiber(&b.fiber(), 2);
        assert_eq!(a.target(), b.target());
        assert_eq!(a.fiber().rank, bf.rank);
        for r in &rows {
            assert!(satisfies(r, &bf.particular, 101));
            for k in &bf.kernel {
                let mut z = r.clone();
                *z.last_mut().unwrap() = 0;
                assert!(satisfies(&z, k, 101));
            }
        }
    }
    #[test]
    fn contradictory_cycle_is_rejected() {
        let mut a = Rref::new(101, 2);
        for r in [vec![1, 0, 1], vec![99, 1, 0], vec![1, 100, 0]] {
            a.add(r);
        }
        assert!(a.fiber().inconsistent);
    }
    #[test]
    fn out_of_order_pivots_recover_the_same_solution() {
        let rows = [vec![0, 1, 1, 9], vec![1, 1, 0, 4], vec![1, 0, 1, 7]];
        for order in [[0, 1, 2], [2, 0, 1], [1, 2, 0]] {
            let mut a = Rref::new(101, 3);
            for i in order {
                a.add(rows[i].clone());
            }
            let f = a.fiber();
            assert_eq!(f.particular, vec![1, 3, 6]);
            assert!(f.kernel.is_empty());
            for row in &rows {
                assert!(satisfies(row, &f.particular, 101));
            }
        }
    }
}

#[cfg(test)]
mod sparse_tests {
    use super::*;
    #[test]
    fn sparse_and_dense_substitution_equal_for_arbitrary_rows_and_zero_dimensional_fibers() {
        for b in [1, 2, 3, 8, 16] {
            let mut known = vec![];
            for i in 0..b / 2 {
                let mut z = vec![0; b + 1];
                z[i] = 1;
                z[b - 1] = 2 * (i as u64 + 1);
                z[b] = i as u64;
                known.push(z);
            }
            let dense = Compression::new_mode(101, b, &known, false);
            let fast = Compression::new_mode(101, b, &known, true);
            assert_eq!(dense.base, fast.base);
            assert_eq!(dense.columns, fast.columns);
            for k in 0..40 {
                let row: Vec<u64> = (0..b + 2)
                    .map(|i| {
                        if (i + k) % 3 == 0 {
                            0
                        } else {
                            ((i * 17 + k * 29) % 101) as u64
                        }
                    })
                    .collect();
                assert_eq!(dense.row(&row), fast.row(&row));
            }
        }
        let c = Compression::new_mode(101, 1, &[vec![1, 37]], true);
        assert!(c.columns.is_empty());
        assert_eq!(c.row(&[2, 100, 81]), vec![100, 7]);
    }
}
