//! Dense exact linear algebra over `F_p` (report section 3, "Dense
//! elimination"): `O(N^3)` classical Gauss-Jordan for rank, solve, right and
//! left nullspaces and inverse. Used for residual cores and as the exact
//! oracle in tests of every sparse method.

use super::field::Fp;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DenseMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<u64>,
}

impl DenseMatrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1);
        }
        m
    }
    pub fn from_rows(rows: &[Vec<u64>]) -> Self {
        let r = rows.len();
        let c = rows.first().map_or(0, |x| x.len());
        let mut m = Self::zeros(r, c);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row.len(), c);
            m.data[i * c..(i + 1) * c].copy_from_slice(row);
        }
        m
    }
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> u64 {
        self.data[i * self.cols + j]
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: u64) {
        self.data[i * self.cols + j] = v;
    }
    pub fn row(&self, i: usize) -> &[u64] {
        &self.data[i * self.cols..(i + 1) * self.cols]
    }
    pub fn transpose(&self) -> Self {
        let mut t = Self::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                t.set(j, i, self.get(i, j));
            }
        }
        t
    }
    pub fn mul_vec(&self, f: Fp, x: &[u64]) -> Vec<u64> {
        (0..self.rows).map(|i| f.dot(self.row(i), x)).collect()
    }
    pub fn mul(&self, f: Fp, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows);
        let ot = other.transpose();
        let mut out = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                out.set(i, j, f.dot(self.row(i), ot.row(j)));
            }
        }
        out
    }
    fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        for j in 0..self.cols {
            self.data.swap(a * self.cols + j, b * self.cols + j);
        }
    }

    /// In-place reduced row echelon form; returns pivot columns.
    pub fn rref(&mut self, f: Fp) -> Vec<usize> {
        let mut pivots = Vec::new();
        let mut r = 0;
        for c in 0..self.cols {
            if r == self.rows {
                break;
            }
            let Some(pr) = (r..self.rows).find(|&i| self.get(i, c) != 0) else {
                continue;
            };
            self.swap_rows(r, pr);
            let inv = f.inv(self.get(r, c)).unwrap();
            for j in c..self.cols {
                let v = f.mul(self.get(r, j), inv);
                self.set(r, j, v);
            }
            for i in 0..self.rows {
                if i == r {
                    continue;
                }
                let factor = self.get(i, c);
                if factor == 0 {
                    continue;
                }
                let nf = f.neg(factor);
                for j in c..self.cols {
                    let v = f.mul_add(self.get(i, j), nf, self.get(r, j));
                    self.set(i, j, v);
                }
            }
            pivots.push(c);
            r += 1;
        }
        pivots
    }

    pub fn rank(&self, f: Fp) -> usize {
        self.clone().rref(f).len()
    }

    /// Basis of the right nullspace `{x : M x = 0}`.
    pub fn right_nullspace(&self, f: Fp) -> Vec<Vec<u64>> {
        let mut m = self.clone();
        let piv = m.rref(f);
        let mut is_piv = vec![false; self.cols];
        for &c in &piv {
            is_piv[c] = true;
        }
        let mut basis = Vec::new();
        for free in (0..self.cols).filter(|&c| !is_piv[c]) {
            let mut x = vec![0u64; self.cols];
            x[free] = 1;
            for (r, &pc) in piv.iter().enumerate() {
                x[pc] = f.neg(m.get(r, free));
            }
            basis.push(x);
        }
        basis
    }

    /// Basis of the left nullspace `{y : y^T M = 0}`.
    pub fn left_nullspace(&self, f: Fp) -> Vec<Vec<u64>> {
        self.transpose().right_nullspace(f)
    }

    /// Some solution of `M x = b`, or `None` when inconsistent.
    pub fn solve(&self, f: Fp, b: &[u64]) -> Option<Vec<u64>> {
        assert_eq!(b.len(), self.rows);
        let mut aug = Self::zeros(self.rows, self.cols + 1);
        for i in 0..self.rows {
            for j in 0..self.cols {
                aug.set(i, j, self.get(i, j));
            }
            aug.set(i, self.cols, f.reduce(b[i]));
        }
        let rows: Vec<Vec<u64>> = (0..aug.rows).map(|i| aug.row(i).to_vec()).collect();
        solve_augmented(f, rows, self.cols)
    }

    /// Inverse of a square matrix, `None` if singular.
    pub fn inverse(&self, f: Fp) -> Option<Self> {
        assert_eq!(self.rows, self.cols);
        let n = self.rows;
        let mut aug = Self::zeros(n, 2 * n);
        for i in 0..n {
            for j in 0..n {
                aug.set(i, j, self.get(i, j));
            }
            aug.set(i, n + i, 1);
        }
        let piv = aug.rref(f);
        if piv.len() < n || piv[n - 1] != n - 1 {
            return None;
        }
        let mut inv = Self::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                inv.set(i, j, aug.get(i, n + j));
            }
        }
        Some(inv)
    }

    pub fn determinant(&self, f: Fp) -> u64 {
        assert_eq!(self.rows, self.cols);
        let mut m = self.clone();
        let n = self.rows;
        let mut det = 1u64;
        for c in 0..n {
            let Some(pr) = (c..n).find(|&i| m.get(i, c) != 0) else {
                return 0;
            };
            if pr != c {
                m.swap_rows(pr, c);
                det = f.neg(det);
            }
            let pv = m.get(c, c);
            det = f.mul(det, pv);
            let inv = f.inv(pv).unwrap();
            for i in c + 1..n {
                let factor = f.mul(m.get(i, c), inv);
                if factor == 0 {
                    continue;
                }
                let nf = f.neg(factor);
                for j in c..n {
                    let v = f.mul_add(m.get(i, j), nf, m.get(c, j));
                    m.set(i, j, v);
                }
            }
        }
        det
    }
}

/// Solve an augmented system given as rows of length `k + 1` (last entry is
/// the right-hand side). Free variables are set to zero.
pub fn solve_augmented(f: Fp, rows: Vec<Vec<u64>>, k: usize) -> Option<Vec<u64>> {
    let mut m = DenseMatrix::from_rows(&rows);
    if rows.is_empty() {
        return Some(vec![0; k]);
    }
    let piv = m.rref(f);
    if piv.last() == Some(&k) {
        return None;
    }
    let mut x = vec![0u64; k];
    for (r, &c) in piv.iter().enumerate() {
        x[c] = m.get(r, k);
    }
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    #[test]
    fn inverse_solve_nullspace() {
        let f = Fp::new(1_000_003).unwrap();
        let mut rng = SplitMix64::new(5);
        for n in 1..12 {
            let mut m = DenseMatrix::zeros(n, n);
            for x in m.data.iter_mut() {
                *x = rng.element(f);
            }
            if let Some(inv) = m.inverse(f) {
                assert_eq!(m.mul(f, &inv), DenseMatrix::identity(n));
                assert_ne!(m.determinant(f), 0);
            } else {
                assert_eq!(m.determinant(f), 0);
            }
            let b: Vec<u64> = (0..n).map(|_| rng.element(f)).collect();
            if let Some(x) = m.solve(f, &b) {
                assert_eq!(m.mul_vec(f, &x), b);
            }
        }
        // Rank-deficient: report's structural-rank example [[1,1],[1,1]].
        let m = DenseMatrix::from_rows(&[vec![1, 1], vec![1, 1]]);
        assert_eq!(m.rank(f), 1);
        let ns = m.right_nullspace(f);
        assert_eq!(ns.len(), 1);
        assert_eq!(m.mul_vec(f, &ns[0]), vec![0, 0]);
        assert_eq!(m.left_nullspace(f).len(), 1);
    }

    #[test]
    fn triangle_determinant_is_two() {
        let f = Fp::new(1_000_003).unwrap();
        let m = DenseMatrix::from_rows(&[vec![1, 1, 0], vec![0, 1, 1], vec![1, 0, 1]]);
        assert_eq!(m.determinant(f), 2);
    }
}
