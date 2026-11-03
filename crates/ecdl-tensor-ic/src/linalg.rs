//! Linear algebra over F_r (Go: linalg.go).

use std::collections::{BTreeMap, HashMap};

use crate::field::{inv_mod, mul_mod, sub_mod};

/// LeftKernelVector returns a nonzero c with sum_j c_j * rows[j] = 0 (mod r),
/// where each row is a sparse map from column to coefficient. It needs
/// len(rows) > rank, which len(rows) > ncols guarantees; `None` (Go: ok ==
/// false) otherwise. r must be prime.
pub fn left_kernel_vector(rows: &[HashMap<usize, u64>], ncols: usize, r: u64) -> Option<Vec<u64>> {
    let k = rows.len();
    // Augment each row with an identity block that records how it was formed.
    let mut mat: Vec<Vec<u64>> = Vec::with_capacity(k);
    for (j, row) in rows.iter().enumerate() {
        let mut mr = vec![0u64; ncols + k];
        for (&col, &v) in row {
            mr[col] = v % r;
        }
        mr[ncols + j] = 1;
        mat.push(mr);
    }
    let mut used = vec![false; k];
    for col in 0..ncols {
        let mut piv: Option<usize> = None;
        for j in 0..k {
            if !used[j] && mat[j][col] != 0 {
                piv = Some(j);
                break;
            }
        }
        let piv = match piv {
            Some(p) => p,
            None => continue,
        };
        used[piv] = true;
        let inv = inv_mod(mat[piv][col], r);
        for t in 0..mat[piv].len() {
            mat[piv][t] = mul_mod(mat[piv][t], inv, r);
        }
        crate::opcount::bump("linalg_mulmod_uncharged", mat[piv].len() as u64);
        let prow = mat[piv].clone();
        for j in 0..k {
            if used[j] || mat[j][col] == 0 {
                continue;
            }
            let f = mat[j][col];
            let mut muls = 0u64;
            for t in 0..mat[j].len() {
                if prow[t] != 0 {
                    mat[j][t] = sub_mod(mat[j][t], mul_mod(f, prow[t], r), r);
                    muls += 1;
                }
            }
            crate::opcount::bump("linalg_mulmod_uncharged", muls);
        }
    }
    // An unused row now has zeros in every column; its identity block is a
    // dependency, and it is nonzero because the row's own entry stays 1.
    for j in 0..k {
        if !used[j] {
            return Some(mat[j][ncols..].to_vec());
        }
    }
    None
}

/// Sparse left-kernel vector: same contract as [`left_kernel_vector`], but the
/// elimination is done on sorted sparse rows (`BTreeMap`) instead of a dense
/// `k x (ncols + k)` matrix.
///
/// A relation row has only m nonzeros, so the dense version allocates
/// O(k * (ncols + k)) coefficient words while this one allocates O(nnz), and
/// each reduction touches only the pivot row's support. Pivot rows are
/// normalized to a leading coefficient of 1. Deterministic: the leading data
/// column and the column order are both smallest-first.
pub fn left_kernel_vector_sparse(
    rows: &[HashMap<usize, u64>],
    ncols: usize,
    r: u64,
) -> Option<Vec<u64>> {
    let k = rows.len();
    let mut pivots: BTreeMap<usize, BTreeMap<usize, u64>> = BTreeMap::new();
    for (j, row) in rows.iter().enumerate() {
        let mut cur: BTreeMap<usize, u64> = row
            .iter()
            .filter(|(_, &v)| v % r != 0)
            .map(|(&c, &v)| (c, v % r))
            .collect();
        // The identity block records how the row was formed.
        cur.insert(ncols + j, 1);
        let mut installed = false;
        loop {
            let lead = match cur.keys().copied().find(|&c| c < ncols) {
                None => break,
                Some(c) => c,
            };
            match pivots.get(&lead) {
                Some(prow) => {
                    let f = cur[&lead];
                    let mut muls = 0u64;
                    for (&c, &pv) in prow.iter() {
                        let e = cur.entry(c).or_insert(0);
                        *e = sub_mod(*e, mul_mod(f, pv, r), r);
                        muls += 1;
                        if *e == 0 {
                            cur.remove(&c);
                        }
                    }
                    crate::opcount::bump("linalg_mulmod_uncharged", muls);
                }
                None => {
                    let inv = inv_mod(cur[&lead], r);
                    crate::opcount::bump("linalg_mulmod_uncharged", cur.len() as u64);
                    for val in cur.values_mut() {
                        *val = mul_mod(*val, inv, r);
                    }
                    pivots.insert(lead, std::mem::take(&mut cur));
                    installed = true;
                    break;
                }
            }
        }
        if installed || cur.is_empty() {
            continue;
        }
        // No data column is left: the combination of earlier rows recorded in
        // the identity block sums to zero.
        let mut c = vec![0u64; k];
        for (&col, &val) in cur.iter() {
            if col >= ncols {
                c[col - ncols] = val;
            }
        }
        if c.iter().any(|&v| v != 0) {
            return Some(c);
        }
    }
    None
}
