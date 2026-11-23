//! Report section 3: Montgomery block Lanczos over `F_2` (Montgomery 1995),
//! with 64-bit blocks. `O(N Z / w + N^2)` machine-word operations in the
//! cited model; this is **not** an odd-prime-field bound and does not apply
//! to ECDLP relation algebra over `F_ell` (the report's own caveat). The
//! repository's `koblitz_sparse_la::block_lanczos_solve` is the odd-field
//! block solver.
//!
//! The kernel routine follows the usual practice: run Lanczos on
//! `B = A^T A` for `B X = B Y` with random `Y`, then combine `X - Y` and the
//! last Krylov block and keep the combinations killed by `A` itself
//! (verified).

use super::field::SplitMix64;

/// Sparse matrix over `F_2`: each row is a sorted list of column indices.
#[derive(Clone, Debug)]
pub struct Gf2Sparse {
    pub nrows: usize,
    pub ncols: usize,
    pub rows: Vec<Vec<u32>>,
}

impl Gf2Sparse {
    /// Build with duplicate columns in a row cancelling (mod 2).
    pub fn new(ncols: usize, rows: Vec<Vec<u32>>) -> Self {
        let rows: Vec<Vec<u32>> = rows
            .into_iter()
            .map(|mut r| {
                r.sort_unstable();
                let mut out: Vec<u32> = Vec::with_capacity(r.len());
                for c in r {
                    assert!((c as usize) < ncols);
                    if out.last() == Some(&c) {
                        out.pop();
                    } else {
                        out.push(c);
                    }
                }
                out
            })
            .collect();
        Self {
            nrows: rows.len(),
            ncols,
            rows,
        }
    }
    /// `A V` for a block of 64 vectors (`v.len() == ncols`).
    pub fn mul_block(&self, v: &[u64]) -> Vec<u64> {
        self.rows
            .iter()
            .map(|r| r.iter().fold(0u64, |a, &c| a ^ v[c as usize]))
            .collect()
    }
    /// `A^T W` (`w.len() == nrows`).
    pub fn mul_block_transpose(&self, w: &[u64]) -> Vec<u64> {
        let mut out = vec![0u64; self.ncols];
        for (r, &x) in self.rows.iter().zip(w) {
            if x != 0 {
                for &c in r {
                    out[c as usize] ^= x;
                }
            }
        }
        out
    }
    /// Apply to a single bit vector given as a set of columns.
    pub fn mul_set(&self, x: &[usize]) -> Vec<bool> {
        let mut xb = vec![false; self.ncols];
        for &c in x {
            xb[c] ^= true;
        }
        self.rows
            .iter()
            .map(|r| r.iter().fold(false, |a, &c| a ^ xb[c as usize]))
            .collect()
    }
}

type M64 = [u64; 64];

/// `V^T W` as a 64x64 matrix (row i = bits j with sum_k V[k]_i W[k]_j).
fn inner(v: &[u64], w: &[u64]) -> M64 {
    let mut out = [0u64; 64];
    for (&a, &b) in v.iter().zip(w) {
        let mut a = a;
        while a != 0 {
            let i = a.trailing_zeros() as usize;
            out[i] ^= b;
            a &= a - 1;
        }
    }
    out
}

/// `V M` for an `N x 64` block and a 64x64 matrix.
fn mul_vm(v: &[u64], m: &M64) -> Vec<u64> {
    v.iter()
        .map(|&a| {
            let mut a = a;
            let mut acc = 0u64;
            while a != 0 {
                let i = a.trailing_zeros() as usize;
                acc ^= m[i];
                a &= a - 1;
            }
            acc
        })
        .collect()
}

fn mul_mm(a: &M64, b: &M64) -> M64 {
    let mut out = [0u64; 64];
    for i in 0..64 {
        let mut x = a[i];
        let mut acc = 0;
        while x != 0 {
            let k = x.trailing_zeros() as usize;
            acc ^= b[k];
            x &= x - 1;
        }
        out[i] = acc;
    }
    out
}

fn add_mm(a: &M64, b: &M64) -> M64 {
    let mut out = [0u64; 64];
    for i in 0..64 {
        out[i] = a[i] ^ b[i];
    }
    out
}

fn mask_cols(a: &M64, s: u64) -> M64 {
    let mut out = *a;
    for x in out.iter_mut() {
        *x &= s;
    }
    out
}

fn identity() -> M64 {
    let mut m = [0u64; 64];
    for (i, x) in m.iter_mut().enumerate() {
        *x = 1u64 << i;
    }
    m
}

/// Montgomery's simultaneous selection of `S_i` and `W_i^{-1}`.
fn find_nonsingular_sub(t: &M64, last_s: u64) -> Option<(M64, u64)> {
    let mut m: Vec<[u64; 2]> = (0..64).map(|i| [t[i], 1u64 << i]).collect();
    let mut cols: Vec<usize> = (0..64).filter(|&c| last_s >> c & 1 == 0).collect();
    cols.extend((0..64).filter(|&c| last_s >> c & 1 == 1));
    let mut s = 0u64;
    for i in 0..64 {
        let c = cols[i];
        let bit = 1u64 << c;
        let mut found = false;
        for j in i..64 {
            if m[cols[j]][0] & bit != 0 {
                m.swap(cols[i], cols[j]);
                found = true;
                break;
            }
        }
        if found {
            let pi = m[cols[i]];
            for j in 0..64 {
                if j != i && m[cols[j]][0] & bit != 0 {
                    m[cols[j]][0] ^= pi[0];
                    m[cols[j]][1] ^= pi[1];
                }
            }
            s |= bit;
            continue;
        }
        for j in i..64 {
            if m[cols[j]][1] & bit != 0 {
                m.swap(cols[i], cols[j]);
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
        let pi = m[cols[i]];
        for j in 0..64 {
            if j != i && m[cols[j]][1] & bit != 0 {
                m[cols[j]][0] ^= pi[0];
                m[cols[j]][1] ^= pi[1];
            }
        }
        m[cols[i]] = [0, 0];
    }
    let mut winv = [0u64; 64];
    for i in 0..64 {
        winv[i] = m[i][1];
    }
    Some((winv, s))
}

/// Statistics of one block Lanczos run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gf2LanczosStats {
    pub iterations: usize,
    pub block_products: usize,
    pub attempts: u32,
}

/// Kernel vectors of `A` over `F_2` (right kernel, as column-index sets),
/// found by Montgomery block Lanczos on `A^T A`. Every returned vector is
/// nonzero, verified `A x = 0`, and the set is linearly independent.
pub fn block_lanczos_gf2_kernel(
    a: &Gf2Sparse,
    seed: u64,
    max_attempts: u32,
) -> (Vec<Vec<usize>>, Gf2LanczosStats) {
    let n = a.ncols;
    let mut st = Gf2LanczosStats::default();
    let mut rng = SplitMix64::new(seed);
    for _ in 0..max_attempts {
        st.attempts += 1;
        let y: Vec<u64> = (0..n).map(|_| rng.next_u64()).collect();
        let apply_b = |v: &[u64], st: &mut Gf2LanczosStats| -> Vec<u64> {
            st.block_products += 2;
            a.mul_block_transpose(&a.mul_block(v))
        };
        let v0 = apply_b(&y, &mut st);
        let mut x = vec![0u64; n];
        let mut v = v0.clone();
        let zero_blk = vec![0u64; n];
        let (mut v1, mut v2) = (zero_blk.clone(), zero_blk.clone());
        let (mut winv1, mut winv2) = ([0u64; 64], [0u64; 64]);
        let (mut vav1, mut va2v1) = ([0u64; 64], [0u64; 64]);
        let mut s1: u64 = u64::MAX;
        let id = identity();
        let max_iter = n / 60 + 20;
        let mut failed = false;
        for _ in 0..max_iter {
            st.iterations += 1;
            let av = apply_b(&v, &mut st);
            let vav = inner(&v, &av);
            let va2v = inner(&av, &av);
            if vav.iter().all(|&r| r == 0) {
                break;
            }
            let Some((winv, s)) = find_nonsingular_sub(&vav, s1) else {
                failed = true;
                break;
            };
            if s == 0 {
                failed = true;
                break;
            }
            // x += V Winv V^T V0
            let d = inner(&v, &v0);
            let xi = mul_vm(&v, &mul_mm(&winv, &d));
            for (a1, b1) in x.iter_mut().zip(&xi) {
                *a1 ^= *b1;
            }
            // Coefficients.
            let dmat = add_mm(&id, &mul_mm(&winv, &add_mm(&mask_cols(&va2v, s), &vav)));
            let emat = mask_cols(&mul_mm(&winv1, &vav), s);
            let fmat = mask_cols(
                &mul_mm(
                    &winv2,
                    &mul_mm(
                        &add_mm(&id, &mul_mm(&vav1, &winv1)),
                        &add_mm(&mask_cols(&va2v1, s1), &vav1),
                    ),
                ),
                s,
            );
            let mut next: Vec<u64> = av.iter().map(|&w| w & s).collect();
            let t1 = mul_vm(&v, &dmat);
            let t2 = mul_vm(&v1, &emat);
            let t3 = mul_vm(&v2, &fmat);
            for i in 0..n {
                next[i] ^= t1[i] ^ t2[i] ^ t3[i];
            }
            v2 = std::mem::replace(&mut v1, std::mem::replace(&mut v, next));
            winv2 = winv1;
            winv1 = winv;
            vav1 = vav;
            va2v1 = va2v;
            s1 = s;
        }
        if failed {
            continue;
        }
        // Combine U = [X + Y | V_m] and keep combinations killed by A.
        let u0: Vec<u64> = x.iter().zip(&y).map(|(a1, b1)| a1 ^ b1).collect();
        let u1 = v;
        let au0 = a.mul_block(&u0);
        let au1 = a.mul_block(&u1);
        st.block_products += 2;
        let kv = combine_kernel(a, &u0, &u1, &au0, &au1);
        if !kv.is_empty() {
            return (kv, st);
        }
    }
    (Vec::new(), st)
}

/// Find combinations `c in F_2^128` of the 128 columns of `[U0 | U1]` with
/// `A [U0|U1] c = 0` and `[U0|U1] c != 0`; return independent kernel vectors.
fn combine_kernel(
    a: &Gf2Sparse,
    u0: &[u64],
    u1: &[u64],
    au0: &[u64],
    au1: &[u64],
) -> Vec<Vec<usize>> {
    let rw = a.nrows.div_ceil(64);
    // Column j of A U as a bitset over rows, with a 128-bit tag.
    let mut vecs: Vec<(Vec<u64>, u128)> = Vec::with_capacity(128);
    for j in 0..128 {
        let mut bits = vec![0u64; rw];
        let src = if j < 64 { au0 } else { au1 };
        let b = j % 64;
        for (r, &w) in src.iter().enumerate() {
            if w >> b & 1 == 1 {
                bits[r / 64] |= 1 << (r % 64);
            }
        }
        vecs.push((bits, 1u128 << j));
    }
    let mut pivots: Vec<(usize, Vec<u64>, u128)> = Vec::new();
    let mut deps: Vec<u128> = Vec::new();
    for (mut bits, mut tag) in vecs {
        for (p, pb, pt) in &pivots {
            if bits[p / 64] >> (p % 64) & 1 == 1 {
                for (x, y) in bits.iter_mut().zip(pb) {
                    *x ^= *y;
                }
                tag ^= *pt;
            }
        }
        if let Some(p) = bits
            .iter()
            .enumerate()
            .find(|(_, &w)| w != 0)
            .map(|(i, &w)| i * 64 + w.trailing_zeros() as usize)
        {
            pivots.push((p, bits, tag));
        } else {
            deps.push(tag);
        }
    }
    // Expand dependencies to kernel vectors; keep an independent set.
    let nw = a.ncols.div_ceil(64);
    let mut basis: Vec<(usize, Vec<u64>)> = Vec::new();
    let mut out = Vec::new();
    for tag in deps {
        let mut xbits = vec![0u64; nw];
        for c in 0..a.ncols {
            let lo = (u0[c] as u128) & tag;
            let hi = (u1[c] as u128) & (tag >> 64);
            if (lo.count_ones() + hi.count_ones()) % 2 == 1 {
                xbits[c / 64] |= 1 << (c % 64);
            }
        }
        let support: Vec<usize> = (0..a.ncols)
            .filter(|&c| xbits[c / 64] >> (c % 64) & 1 == 1)
            .collect();
        if support.is_empty() || a.mul_set(&support).iter().any(|&b| b) {
            continue;
        }
        let mut r = xbits.clone();
        for (p, b) in &basis {
            if r[p / 64] >> (p % 64) & 1 == 1 {
                for (x, y) in r.iter_mut().zip(b) {
                    *x ^= *y;
                }
            }
        }
        if let Some(p) = r
            .iter()
            .enumerate()
            .find(|(_, &w)| w != 0)
            .map(|(i, &w)| i * 64 + w.trailing_zeros() as usize)
        {
            basis.push((p, r));
            out.push(support);
        }
    }
    out
}

/// Dense GF(2) rank (oracle for tests and small residual cores).
pub fn gf2_rank(a: &Gf2Sparse) -> usize {
    let nw = a.ncols.div_ceil(64);
    let mut rows: Vec<Vec<u64>> = a
        .rows
        .iter()
        .map(|r| {
            let mut b = vec![0u64; nw];
            for &c in r {
                b[c as usize / 64] ^= 1 << (c % 64);
            }
            b
        })
        .collect();
    let mut rank = 0;
    for col in 0..a.ncols {
        let Some(p) = (rank..rows.len()).find(|&i| rows[i][col / 64] >> (col % 64) & 1 == 1) else {
            continue;
        };
        rows.swap(rank, p);
        let pr = rows[rank].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i != rank && row[col / 64] >> (col % 64) & 1 == 1 {
                for (x, y) in row.iter_mut().zip(&pr) {
                    *x ^= *y;
                }
            }
        }
        rank += 1;
    }
    rank
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_verified_kernel_vectors() {
        for seed in 0..6u64 {
            let mut rng = SplitMix64::new(seed);
            let (nr, nc) = (300usize, 360usize);
            let rows: Vec<Vec<u32>> = (0..nr)
                .map(|_| (0..6).map(|_| rng.below(nc as u64) as u32).collect())
                .collect();
            let a = Gf2Sparse::new(nc, rows);
            let (kv, st) = block_lanczos_gf2_kernel(&a, seed, 4);
            let dim = nc - gf2_rank(&a);
            assert!(!kv.is_empty(), "seed {seed} stats {st:?}");
            assert!(kv.len() <= dim);
            for x in &kv {
                assert!(!x.is_empty());
                assert!(a.mul_set(x).iter().all(|&b| !b));
            }
        }
    }

    #[test]
    fn nonsingular_sub_inverts_selected_block() {
        let mut rng = SplitMix64::new(77);
        for _ in 0..50 {
            // Random symmetric T.
            let mut t = [0u64; 64];
            for i in 0..64 {
                for j in i..64 {
                    if rng.below(2) == 1 {
                        t[i] |= 1 << j;
                        t[j] |= 1 << i;
                    }
                }
            }
            let (winv, s) = find_nonsingular_sub(&t, u64::MAX).unwrap();
            // Winv restricted to S inverts T restricted to S.
            let ts = {
                let mut m = [0u64; 64];
                for i in 0..64 {
                    if s >> i & 1 == 1 {
                        m[i] = t[i] & s;
                    }
                }
                m
            };
            let prod = mul_mm(&ts, &winv);
            for i in 0..64 {
                if s >> i & 1 == 1 {
                    assert_eq!(prod[i] & s, 1u64 << i);
                }
            }
        }
    }
}
