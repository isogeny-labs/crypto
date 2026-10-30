//! Linear algebra over F_r (Go: linalg.go).

use std::collections::HashMap;

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
