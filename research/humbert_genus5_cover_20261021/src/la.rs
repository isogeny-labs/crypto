//! Dense Gaussian elimination mod a prime r < 2^62, skipping zero entries
//! (relation rows start with at most 9 non-zeros).
use crate::curve::{invr, mulr};

pub struct Solve {
    pub kernel_dim: usize,
    /// one kernel vector when kernel_dim == 1
    pub x: Option<Vec<u64>>,
    pub rank: usize,
    pub mulmods: u64,
}

pub fn kernel_mod_r(rows: &[Vec<(usize, u64)>], ncols: usize, r: u64) -> Solve {
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|row| {
            let mut v = vec![0u64; ncols];
            for &(c, a) in row {
                v[c] = (v[c] + a) % r;
            }
            v
        })
        .collect();
    let mut mulmods = 0u64;
    let mut piv_col = Vec::new();
    let mut rk = 0;
    for c in 0..ncols {
        let Some(pr) = (rk..m.len()).find(|&i| m[i][c] != 0) else { continue };
        m.swap(rk, pr);
        let inv = invr(m[rk][c], r);
        let nz: Vec<usize> = (0..ncols).filter(|&j| m[rk][j] != 0).collect();
        for &j in &nz {
            m[rk][j] = mulr(m[rk][j], inv, r);
        }
        mulmods += nz.len() as u64;
        let pivot_row = m[rk].clone();
        for i in 0..m.len() {
            if i == rk || m[i][c] == 0 {
                continue;
            }
            let f = m[i][c];
            for &j in &nz {
                let t = mulr(f, pivot_row[j], r);
                m[i][j] = if m[i][j] >= t { m[i][j] - t } else { m[i][j] + r - t };
            }
            mulmods += nz.len() as u64;
        }
        piv_col.push(c);
        rk += 1;
        if rk == m.len() {
            break;
        }
    }
    let free: Vec<usize> = (0..ncols).filter(|c| !piv_col.contains(c)).collect();
    let kernel_dim = free.len();
    let x = if kernel_dim == 1 {
        let f = free[0];
        let mut v = vec![0u64; ncols];
        v[f] = 1;
        for (ri, &pc) in piv_col.iter().enumerate() {
            v[pc] = (r - m[ri][f]) % r;
        }
        Some(v)
    } else {
        None
    };
    Solve { kernel_dim, x, rank: rk, mulmods }
}

/// Berlekamp-Massey over F_r: connection polynomial C (C[0] = 1) of minimal length L.
fn berlekamp_massey(s: &[u64], r: u64) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let (mut l, mut m, mut bb) = (0usize, 1usize, 1u64);
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l.min(c.len() - 1) {
            d = (d + mulr(c[i], s[n - i], r)) % r;
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = mulr(d, invr(bb, r), r);
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for i in 0..b.len() {
            let v = mulr(coef, b[i], r);
            c[i + m] = (c[i + m] + r - v) % r;
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    c.truncate(l + 1);
    c
}

fn matvec(rows: &[Vec<(usize, u64)>], x: &[u64], r: u64, cnt: &mut u64) -> Vec<u64> {
    rows.iter()
        .map(|row| {
            let mut s = 0u64;
            for &(c, a) in row {
                s = (s + mulr(a, x[c], r)) % r;
            }
            *cnt += row.len() as u64;
            s
        })
        .collect()
}
fn matvec_t(rows: &[Vec<(usize, u64)>], y: &[u64], ncols: usize, r: u64, cnt: &mut u64) -> Vec<u64> {
    let mut out = vec![0u64; ncols];
    for (row, &yi) in rows.iter().zip(y) {
        if yi == 0 {
            continue;
        }
        for &(c, a) in row {
            out[c] = (out[c] + mulr(a, yi, r)) % r;
        }
        *cnt += row.len() as u64;
    }
    out
}

/// Kernel vector of a sparse (rows x ncols) matrix by scalar Wiedemann on
/// A = M'^T M' with one coordinate fixed to 1, after a random diagonal column
/// scaling D (x = D y).  The Krylov solution is a fixed vector for a given D, so a
/// corank > 1 is only visible by comparing solves under two different D: the
/// caller does that.  Every returned vector is checked against all rows (M x = 0).
pub fn kernel_wiedemann(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64) -> Solve {
    let mut rng = crate::curve::Rng(seed);
    let mut cnt = 0u64;
    let dscale: Vec<u64> = (0..ncols).map(|_| 1 + rng.below(r - 1)).collect();
    let scaled: Vec<Vec<(usize, u64)>> = rows_in.iter().map(|row| row.iter().map(|&(c, a)| (c, mulr(a, dscale[c], r))).collect()).collect();
    let rows = &scaled;
    for fixed in (0..ncols).rev().take(6) {
        // M' = M without column `fixed`, y = -M[:, fixed]
        let mut mp: Vec<Vec<(usize, u64)>> = Vec::with_capacity(rows.len());
        let mut y = vec![0u64; rows.len()];
        for (i, row) in rows.iter().enumerate() {
            let mut nr = Vec::with_capacity(row.len());
            for &(c, a) in row {
                if c == fixed {
                    y[i] = (r - a % r) % r;
                } else {
                    nr.push((if c > fixed { c - 1 } else { c }, a));
                }
            }
            mp.push(nr);
        }
        let n = ncols - 1;
        let b = matvec_t(&mp, &y, n, r, &mut cnt);
        for _attempt in 0..3 {
            let u: Vec<u64> = (0..n).map(|_| rng.below(r)).collect();
            let mut v = b.clone();
            let mut seq = Vec::with_capacity(2 * n + 2);
            for _ in 0..(2 * n + 2) {
                let mut s = 0u64;
                for i in 0..n {
                    s = (s + mulr(u[i], v[i], r)) % r;
                }
                seq.push(s);
                let mv = matvec(&mp, &v, r, &mut cnt);
                v = matvec_t(&mp, &mv, n, r, &mut cnt);
            }
            let c = berlekamp_massey(&seq, r);
            let ll = c.len() - 1;
            // minimal polynomial f(z) = z^L C(1/z): f_k = C[L - k]; f_0 = C[L]
            if ll == 0 || c[ll] == 0 {
                continue;
            }
            let f0inv = invr(c[ll], r);
            // x = -(1/f0) sum_{k=1}^{L} f_k A^{k-1} b
            // second pass recomputes A^{k-1} b (O(n) memory)
            let mut x = vec![0u64; n];
            let mut kv = b.clone();
            for k in 1..=ll {
                let fk = c[ll - k];
                if fk != 0 {
                    for i in 0..n {
                        x[i] = (x[i] + mulr(fk, kv[i], r)) % r;
                    }
                }
                if k < ll {
                    let mv = matvec(&mp, &kv, r, &mut cnt);
                    kv = matvec_t(&mp, &mv, n, r, &mut cnt);
                }
            }
            for xi in x.iter_mut() {
                *xi = mulr((r - *xi) % r, f0inv, r);
            }
            // back to full coordinates and check every row
            let mut full = vec![0u64; ncols];
            for c in 0..ncols {
                full[c] = if c == fixed { 1 } else { x[if c > fixed { c - 1 } else { c }] };
            }
            for c in 0..ncols {
                full[c] = mulr(full[c], dscale[c], r);
            }
            let ok = rows_in.iter().all(|row| row.iter().fold(0u64, |s, &(c, a)| (s + mulr(a, full[c], r)) % r) == 0);
            if ok {
                return Solve { kernel_dim: 1, x: Some(full), rank: n, mulmods: cnt };
            }
        }
    }
    Solve { kernel_dim: 0, x: None, rank: 0, mulmods: cnt }
}
