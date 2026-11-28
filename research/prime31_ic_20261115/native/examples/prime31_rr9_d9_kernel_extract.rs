//! EXP-PRIME31-RR9-D9-KERNEL-001 + 4-Variable Triangular Elimination & Remainder-Tower Comparison.
//!
//! Public-only sparse Krylov / Wiedemann right-kernel extractor on the degree-9 affine Macaulay
//! matrix M (7-variable RR9 chart) and on the 4-variable triangular-reduced (a0, a1, a2, a3)
//! Macaulay operators (both unweighted total degree and canonical (7,5,3,1)-weighted / D9 pullback
//! support).
//!
//! Strictly enforces the public-only custody boundary: candidate extraction receives ONLY
//! `(curve_a, curve_b, p, h_b, public_residual, seed)` and never accesses factor-base y-lifts,
//! sign masks, group-sum tables, or planted witnesses.

#[allow(dead_code)]
#[path = "prime31_rr9_public_macaulay.rs"]
mod public_macaulay;

use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::mem;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const NV7: usize = 7;
const NV4: usize = 4;

type Exp7 = [u8; NV7];
type Exp4 = [u8; NV4];
type Poly7 = BTreeMap<Exp7, u64>;
type Poly4 = BTreeMap<Exp4, u64>;

/// Standalone SHA-256 implementation (FIPS 180-4) so digests require no external crate or Python.
mod sha256 {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    pub fn hex_digest(data: &[u8]) -> String {
        let mut h: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
            0x5be0cd19,
        ];
        let bit_len = (data.len() as u64) * 8;
        let mut padded = data.to_vec();
        padded.push(0x80);
        while (padded.len() % 64) != 56 {
            padded.push(0);
        }
        padded.extend_from_slice(&bit_len.to_be_bytes());
        for chunk in padded.chunks_exact(64) {
            let mut w = [0u32; 64];
            for i in 0..16 {
                w[i] = u32::from_be_bytes([
                    chunk[4 * i],
                    chunk[4 * i + 1],
                    chunk[4 * i + 2],
                    chunk[4 * i + 3],
                ]);
            }
            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1);
            }
            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let temp1 = hh
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(K[i])
                    .wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let temp2 = s0.wrapping_add(maj);
                hh = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }
            h[0] = h[0].wrapping_add(a);
            h[1] = h[1].wrapping_add(b);
            h[2] = h[2].wrapping_add(c);
            h[3] = h[3].wrapping_add(d);
            h[4] = h[4].wrapping_add(e);
            h[5] = h[5].wrapping_add(f);
            h[6] = h[6].wrapping_add(g);
            h[7] = h[7].wrapping_add(hh);
        }
        let mut out = String::with_capacity(64);
        for word in h {
            out.push_str(&format!("{word:08x}"));
        }
        out
    }

    pub fn vector_u64_le_hex(v: &[u64]) -> String {
        let mut bytes = Vec::with_capacity(v.len() * 8);
        for &x in v {
            bytes.extend_from_slice(&x.to_le_bytes());
        }
        hex_digest(&bytes)
    }
}

/// Public-only candidate extractor module.
/// Receives ONLY `(curve_a, h_b, residual, seed)` — zero access to `Arithmetic`, group-sum
/// tables, factor-base `y`-coordinates, sign masks, or known witnesses.
pub mod candidate {
    use super::*;

    #[inline(always)]
    pub fn add(a: u64, b: u64) -> u64 {
        let z = a + b;
        if z >= P { z - P } else { z }
    }

    #[inline(always)]
    pub fn sub(a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + P - b }
    }

    #[inline(always)]
    pub fn mul(a: u64, b: u64) -> u64 {
        let x = a * b;
        let y = (x & P) + (x >> 31);
        let z = (y & P) + (y >> 31);
        if z >= P { z - P } else { z }
    }

    pub fn pow(mut x: u64, mut e: u64) -> u64 {
        let mut v = 1u64;
        while e != 0 {
            if e & 1 == 1 {
                v = mul(v, x);
            }
            x = mul(x, x);
            e >>= 1;
        }
        v
    }

    #[inline(always)]
    pub fn inv(x: u64) -> u64 {
        assert!(x > 0 && x < P, "division by zero in Fp");
        pow(x, P - 2)
    }

    pub fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn random_nonzero(seed: &mut u64) -> u64 {
        1 + splitmix64(seed) % (P - 1)
    }

    /// Flat CSR sparse matrix over Fp. Identical mathematical entries and row/column ordering
    /// to `Vec<Vec<(usize, u64)>>` without per-row heap pointer chasing.
    #[derive(Clone, Debug)]
    pub struct CsrMatrix {
        pub rows: usize,
        pub cols: usize,
        pub row_offsets: Vec<u32>,
        pub col_idx: Vec<u32>,
        pub vals: Vec<u64>,
    }

    impl CsrMatrix {
        pub fn nnz(&self) -> usize {
            self.vals.len()
        }

        pub fn from_sorted_rows(cols: usize, rows_vec: Vec<Vec<(usize, u64)>>) -> Self {
            let total_nnz: usize = rows_vec.iter().map(Vec::len).sum();
            let mut row_offsets = Vec::with_capacity(rows_vec.len() + 1);
            let mut col_idx = Vec::with_capacity(total_nnz);
            let mut vals = Vec::with_capacity(total_nnz);
            row_offsets.push(0);
            for row in rows_vec {
                for (c, v) in row {
                    assert!(c < cols);
                    assert!(v > 0 && v < P);
                    col_idx.push(c as u32);
                    vals.push(v);
                }
                row_offsets.push(vals.len() as u32);
            }
            Self {
                rows: row_offsets.len() - 1,
                cols,
                row_offsets,
                col_idx,
                vals,
            }
        }

        /// y = M^T D M x. Mathematical field multiplications = 2 * nnz + rows.
        /// Uses exact u128 row & column accumulation (bit-for-bit identical in Fp to eager per-term mod P).
        pub fn gram_apply_with_scratch(&self, diag: &[u64], x: &[u64], y: &mut [u64], y_acc: &mut [u128]) {
            assert_eq!(self.rows, diag.len());
            assert_eq!(self.cols, x.len());
            assert_eq!(self.cols, y.len());
            assert_eq!(self.cols, y_acc.len());
            y_acc.fill(0);
            for r in 0..self.rows {
                let start = self.row_offsets[r] as usize;
                let end = self.row_offsets[r + 1] as usize;
                let mut acc = 0u128;
                for idx in start..end {
                    let c = unsafe { *self.col_idx.get_unchecked(idx) as usize };
                    let coeff = unsafe { *self.vals.get_unchecked(idx) as u128 };
                    let xi = unsafe { *x.get_unchecked(c) as u128 };
                    acc += coeff * xi;
                }
                let z = (acc % (P as u128)) as u64;
                let scaled = mul(diag[r], z) as u128;
                if scaled != 0 {
                    for idx in start..end {
                        let c = unsafe { *self.col_idx.get_unchecked(idx) as usize };
                        let coeff = unsafe { *self.vals.get_unchecked(idx) as u128 };
                        let yc = unsafe { y_acc.get_unchecked_mut(c) };
                        *yc += coeff * scaled;
                    }
                }
            }
            for (dst, &acc) in y.iter_mut().zip(y_acc.iter()) {
                *dst = (acc % (P as u128)) as u64;
            }
        }

        pub fn gram_apply(&self, diag: &[u64], x: &[u64], y: &mut [u64]) {
            let mut y_acc = vec![0u128; self.cols];
            self.gram_apply_with_scratch(diag, x, y, &mut y_acc);
        }

        /// Reference eager per-term modular reduction `y = M^T D M x` for exact equivalence check.
        pub fn gram_apply_eager(&self, diag: &[u64], x: &[u64], y: &mut [u64]) {
            assert_eq!(self.rows, diag.len());
            assert_eq!(self.cols, x.len());
            assert_eq!(self.cols, y.len());
            y.fill(0);
            for r in 0..self.rows {
                let start = self.row_offsets[r] as usize;
                let end = self.row_offsets[r + 1] as usize;
                let mut z = 0u64;
                for idx in start..end {
                    z = add(z, mul(self.vals[idx], x[self.col_idx[idx] as usize]));
                }
                let scaled = mul(diag[r], z);
                for idx in start..end {
                    let c = self.col_idx[idx] as usize;
                    y[c] = add(y[c], mul(self.vals[idx], scaled));
                }
            }
        }

        /// Direct forward multiplication `out = M x` (`rows` length).
        /// Exact field multiplications = nnz.
        pub fn forward_apply(&self, x: &[u64]) -> Vec<u64> {
            assert_eq!(self.cols, x.len());
            let mut out = vec![0u64; self.rows];
            for r in 0..self.rows {
                let start = self.row_offsets[r] as usize;
                let end = self.row_offsets[r + 1] as usize;
                let mut z = 0u64;
                for idx in start..end {
                    z = add(z, mul(self.vals[idx], x[self.col_idx[idx] as usize]));
                }
                out[r] = z;
            }
            out
        }
    }

    pub fn dot(x: &[u64], y: &[u64]) -> u64 {
        assert_eq!(x.len(), y.len());
        let mut acc = 0u128;
        for (&a, &b) in x.iter().zip(y) {
            acc += (a as u128) * (b as u128);
        }
        (acc % (P as u128)) as u64
    }

    /// Classic scalar Berlekamp-Massey with exact counted field muls and inversions.
    /// Returns `(L, C, bm_muls, bm_invs)` where `C[0] = 1` and
    /// `s_t + sum_{i=1}^L C[i] s_{t-i} = 0` for all `t >= L`.
    pub fn berlekamp_massey_counted(s: &[u64]) -> (usize, Vec<u64>, u64, u64) {
        let mut c = vec![1u64];
        let mut b = vec![1u64];
        let mut l = 0usize;
        let mut shift = 1usize;
        let mut prev_discrepancy = 1u64;
        let mut bm_muls = 0u64;
        let mut bm_invs = 0u64;
        for (t, &value) in s.iter().enumerate() {
            let mut discrepancy = value;
            for i in 1..=l {
                discrepancy = add(discrepancy, mul(c[i], s[t - i]));
                bm_muls += 1;
            }
            if discrepancy == 0 {
                shift += 1;
                continue;
            }
            let old_c = c.clone();
            let inv_prev = inv(prev_discrepancy);
            bm_invs += 1;
            let scale = mul(discrepancy, inv_prev);
            bm_muls += 1;
            if c.len() < b.len() + shift {
                c.resize(b.len() + shift, 0);
            }
            for (j, &bj) in b.iter().enumerate() {
                c[j + shift] = sub(c[j + shift], mul(scale, bj));
                bm_muls += 1;
            }
            if 2 * l <= t {
                l = t + 1 - l;
                b = old_c;
                prev_discrepancy = discrepancy;
                shift = 1;
            } else {
                shift += 1;
            }
        }
        c.resize(l + 1, 0);
        (l, c, bm_muls, bm_invs)
    }

    /// Evaluate `W = (P(A)/A) v = (A^(L-1) + c[1] A^(L-2) + ... + c[L-1] I) v` by Horner.
    /// Drops exactly one trailing factor `X` from `P(X)` when `c[L] == 0`.
    /// Returns `(W, gram_apps, horner_field_muls)`.
    pub fn evaluate_kernel_horner(
        m: &CsrMatrix,
        diag: &[u64],
        v: &[u64],
        c: &[u64],
        l: usize,
    ) -> (Vec<u64>, usize, u64) {
        assert!(l >= 1 && c.len() > l && c[l] == 0);
        let mut w = v.to_vec();
        let mut next = vec![0u64; m.cols];
        let mut y_acc = vec![0u128; m.cols];
        let mut gram_apps = 0usize;
        for j in 1..l {
            m.gram_apply_with_scratch(diag, &w, &mut next, &mut y_acc);
            gram_apps += 1;
            let cj = c[j];
            for i in 0..m.cols {
                next[i] = add(next[i], mul(cj, v[i]));
            }
            mem::swap(&mut w, &mut next);
        }
        let per_step = (2 * m.nnz() + m.rows + m.cols) as u64;
        (w, gram_apps, per_step * gram_apps as u64)
    }

    // -------------------------------------------------------------------------
    // 7-variable public RR9 quadrics & Macaulay builder
    // -------------------------------------------------------------------------
    fn term7(q: &mut Poly7, coeff: u64, first: Option<usize>, second: Option<usize>) {
        if coeff == 0 {
            return;
        }
        let mut e = [0u8; NV7];
        for i in [first, second].into_iter().flatten() {
            e[i] += 1;
        }
        let prev = q.get(&e).copied().unwrap_or(0);
        let next = add(prev, coeff);
        if next == 0 {
            q.remove(&e);
        } else {
            q.insert(e, next);
        }
    }

    pub fn poly_mul_univariate(a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut out = vec![0u64; a.len() + b.len() - 1];
        for (i, &ai) in a.iter().enumerate() {
            for (j, &bj) in b.iter().enumerate() {
                out[i + j] = add(out[i + j], mul(ai, bj));
            }
        }
        out
    }

    pub fn factor_polynomial_b8(xs: &[u64; 8]) -> [u64; 9] {
        let mut h = vec![1u64];
        for &x in xs {
            h = poly_mul_univariate(&h, &[sub(0, x), 1]);
        }
        h.try_into().unwrap()
    }

    pub fn equations_7var(curve_a: u64, r: u64, s: u64, h: &[u64; 9]) -> [Poly7; 8] {
        let mut q: [Poly7; 8] = std::array::from_fn(|_| BTreeMap::new());
        let d = [add(mul(r, r), curve_a), r, 1];
        let vvar = |i: usize| if i == 3 { None } else { Some(i + 4) };
        for i in 0..4 {
            for j in 0..4 {
                for (k, &dk) in d.iter().enumerate() {
                    if i + j + k < 8 {
                        term7(&mut q[i + j + k], dk, vvar(i), vvar(j));
                    }
                }
                term7(&mut q[i + j], r, Some(i), Some(j));
                if i + j + 1 < 8 {
                    term7(&mut q[i + j + 1], P - 1, Some(i), Some(j));
                }
                term7(&mut q[i + j], sub(0, mul(2, s)), Some(i), vvar(j));
            }
        }
        for i in 0..8 {
            term7(&mut q[i], sub(0, h[i]), None, None);
        }
        q
    }

    fn exact7(n: usize, rem: u8, e: &mut Exp7, out: &mut Vec<Exp7>) {
        if n == NV7 {
            if rem == 0 {
                out.push(*e);
            }
            return;
        }
        for v in 0..=rem {
            e[n] = v;
            exact7(n + 1, rem - v, e, out);
        }
    }

    pub fn monomials7(max_degree: u8) -> Vec<Exp7> {
        let mut out = Vec::new();
        let mut e = [0u8; NV7];
        for d in 0..=max_degree {
            exact7(0, d, &mut e, &mut out);
        }
        out
    }

    pub fn add_exp7(a: &Exp7, b: &Exp7) -> Exp7 {
        std::array::from_fn(|i| a[i] + b[i])
    }

    pub fn eval_poly7(q: &Poly7, v: &[u64; NV7]) -> u64 {
        let mut total = 0u64;
        for (e, &coeff) in q {
            let mut z = coeff;
            for j in 0..NV7 {
                for _ in 0..e[j] {
                    z = mul(z, v[j]);
                }
            }
            total = add(total, z);
        }
        total
    }

    pub fn direct_m_from_7var(curve_a: u64, v: &[u64; NV7], r: u64, s: u64) -> [u64; 9] {
        let a = [v[0], v[1], v[2], v[3]];
        let vv = [v[4], v[5], v[6], 1];
        let d = [add(mul(r, r), curve_a), r, 1];
        let dvv = poly_mul_univariate(&d, &poly_mul_univariate(&vv, &vv));
        let xaa = poly_mul_univariate(&[sub(0, r), 1], &poly_mul_univariate(&a, &a));
        let av = poly_mul_univariate(&a, &vv);
        std::array::from_fn(|i| {
            let mut z = dvv[i];
            if i < xaa.len() {
                z = sub(z, xaa[i]);
            }
            if i < av.len() {
                z = sub(z, mul(mul(2, s), av[i]));
            }
            z
        })
    }

    pub fn build_macaulay_7var(
        curve_a: u64,
        r: u64,
        s: u64,
        h: &[u64; 9],
        degree: u8,
    ) -> (Vec<Exp7>, HashMap<Exp7, usize>, [Poly7; 8], CsrMatrix) {
        assert!(degree >= 2);
        let qs = equations_7var(curve_a, r, s, h);
        let basis = monomials7(degree);
        let mult_basis = monomials7(degree - 2);
        let index: HashMap<Exp7, usize> =
            basis.iter().copied().enumerate().map(|(i, e)| (e, i)).collect();
        let mut rows = Vec::with_capacity(8 * mult_basis.len());
        for q in &qs {
            for m in &mult_basis {
                let mut row = Vec::with_capacity(q.len());
                for (e, &val) in q {
                    if val != 0 {
                        let col = *index.get(&add_exp7(e, m)).expect("Macaulay degree overflow");
                        row.push((col, val));
                    }
                }
                row.sort_unstable_by_key(|x| x.0);
                rows.push(row);
            }
        }
        let csr = CsrMatrix::from_sorted_rows(basis.len(), rows);
        (basis, index, qs, csr)
    }

    // -------------------------------------------------------------------------
    // 4-variable exact triangular elimination at x^7, x^6, x^5
    // -------------------------------------------------------------------------
    fn accum4(dst: &mut Poly4, exp: Exp4, val: u64) {
        if val == 0 {
            return;
        }
        let prev = dst.get(&exp).copied().unwrap_or(0);
        let next = add(prev, val);
        if next == 0 {
            dst.remove(&exp);
        } else {
            dst.insert(exp, next);
        }
    }

    fn const4(val: u64) -> Poly4 {
        let mut p = BTreeMap::new();
        accum4(&mut p, [0, 0, 0, 0], val % P);
        p
    }

    fn var4(j: usize) -> Poly4 {
        let mut e = [0u8; NV4];
        e[j] = 1;
        let mut p = BTreeMap::new();
        p.insert(e, 1);
        p
    }

    fn scaled_add4(dst: &mut Poly4, src: &Poly4, scale: u64) {
        if scale == 0 {
            return;
        }
        for (&e, &c) in src {
            accum4(dst, e, mul(c, scale));
        }
    }

    pub fn mul_poly4(left: &Poly4, right: &Poly4) -> Poly4 {
        let mut out = BTreeMap::new();
        for (&e, &x) in left {
            for (&f, &y) in right {
                let prod_exp: Exp4 = std::array::from_fn(|i| e[i].checked_add(f[i]).unwrap());
                accum4(&mut out, prod_exp, mul(x, y));
            }
        }
        out
    }

    fn convolve4(left: &[Poly4], right: &[Poly4]) -> Vec<Poly4> {
        let mut out = vec![BTreeMap::new(); left.len() + right.len() - 1];
        for (i, li) in left.iter().enumerate() {
            if li.is_empty() {
                continue;
            }
            for (j, rj) in right.iter().enumerate() {
                if rj.is_empty() {
                    continue;
                }
                let prod = mul_poly4(li, rj);
                scaled_add4(&mut out[i + j], &prod, 1);
            }
        }
        out
    }

    fn norm_univariate4(
        curve_a: u64,
        r: u64,
        s: u64,
        h: &[u64; 9],
        ca: &[Poly4; 4],
        cv: &[Poly4; 4],
    ) -> Vec<Poly4> {
        let d = [const4(add(mul(r, r), curve_a)), const4(r), const4(1)];
        let mut out = convolve4(&d, &convolve4(cv, cv));
        let xaa = convolve4(&[const4(sub(0, r)), const4(1)], &convolve4(ca, ca));
        let av = convolve4(ca, cv);
        let neg_two_s = sub(0, mul(2, s));
        for (i, p) in xaa.iter().enumerate() {
            scaled_add4(&mut out[i], p, P - 1);
        }
        for (i, p) in av.iter().enumerate() {
            scaled_add4(&mut out[i], p, neg_two_s);
        }
        for i in 0..9 {
            if h[i] != 0 {
                scaled_add4(&mut out[i], &const4(h[i]), P - 1);
            }
        }
        assert!(out[8].is_empty(), "leading X^8 coefficient must vanish");
        out.pop();
        out
    }

    #[derive(Clone, Debug)]
    pub struct TriangularReduced4 {
        pub equations: [Poly4; 5],
        /// `[recon0, recon1, recon2]` expressing `(v0, v1, v2)` as polynomials in `(a0, a1, a2, a3)`.
        pub reconstruction: [Poly4; 3],
    }

    pub fn build_triangular_4var(
        curve_a: u64,
        r: u64,
        s: u64,
        h: &[u64; 9],
    ) -> TriangularReduced4 {
        let ca = [var4(0), var4(1), var4(2), var4(3)];
        let mut cv: [Poly4; 4] = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), const4(1)];
        let neg_half = (P - 1) / 2; // -1/2 mod P (since (P-1)/2 * 2 = P-1 = -1 mod P)
        for j in (0..3).rev() {
            let polys = norm_univariate4(curve_a, r, s, h, &ca, &cv);
            let pivot_rhs = &polys[5 + j];
            let mut replacement = BTreeMap::new();
            scaled_add4(&mut replacement, pivot_rhs, neg_half);
            cv[j] = replacement;
        }
        let full = norm_univariate4(curve_a, r, s, h, &ca, &cv);
        for k in 5..8 {
            assert!(
                full[k].is_empty(),
                "triangular elimination failed to zero X^{k}"
            );
        }
        let equations: [Poly4; 5] = [
            full[0].clone(),
            full[1].clone(),
            full[2].clone(),
            full[3].clone(),
            full[4].clone(),
        ];
        let reconstruction = [cv[0].clone(), cv[1].clone(), cv[2].clone()];
        TriangularReduced4 {
            equations,
            reconstruction,
        }
    }

    pub fn eval_poly4(q: &Poly4, a: &[u64; NV4]) -> u64 {
        let mut total = 0u64;
        for (e, &coeff) in q {
            let mut z = coeff;
            for j in 0..NV4 {
                for _ in 0..e[j] {
                    z = mul(z, a[j]);
                }
            }
            total = add(total, z);
        }
        total
    }

    pub fn reconstruct_7var_from_4var(red: &TriangularReduced4, a: &[u64; NV4]) -> [u64; NV7] {
        [
            a[0],
            a[1],
            a[2],
            a[3],
            eval_poly4(&red.reconstruction[0], a),
            eval_poly4(&red.reconstruction[1], a),
            eval_poly4(&red.reconstruction[2], a),
        ]
    }

    #[inline]
    pub fn weighted_deg4(e: &Exp4, weights: &[u16; 4]) -> u16 {
        (e[0] as u16) * weights[0]
            + (e[1] as u16) * weights[1]
            + (e[2] as u16) * weights[2]
            + (e[3] as u16) * weights[3]
    }

    pub fn poly4_max_weighted_deg(p: &Poly4, weights: &[u16; 4]) -> u16 {
        p.keys()
            .map(|e| weighted_deg4(e, weights))
            .max()
            .unwrap_or(0)
    }

    /// Generate all 4-variable monomials `e` with `weighted_deg4(&e, weights) <= max_w`,
    /// ordered by ascending weighted degree then lex `[e0, e1, e2, e3]` (constant `[0,0,0,0]` first).
    pub fn monomials4_weighted(weights: &[u16; 4], max_w: u16) -> Vec<Exp4> {
        let mut items: Vec<(u16, Exp4)> = Vec::new();
        let m0 = (max_w / weights[0]) as u8;
        for e0 in 0..=m0 {
            let rem0 = max_w - (e0 as u16) * weights[0];
            let m1 = (rem0 / weights[1]) as u8;
            for e1 in 0..=m1 {
                let rem1 = rem0 - (e1 as u16) * weights[1];
                let m2 = (rem1 / weights[2]) as u8;
                for e2 in 0..=m2 {
                    let rem2 = rem1 - (e2 as u16) * weights[2];
                    let m3 = (rem2 / weights[3]) as u8;
                    for e3 in 0..=m3 {
                        let e = [e0, e1, e2, e3];
                        let w = weighted_deg4(&e, weights);
                        items.push((w, e));
                    }
                }
            }
        }
        items.sort_unstable();
        items.into_iter().map(|(_, e)| e).collect()
    }

    /// Build a 4-variable weighted (or unweighted when `weights = [1,1,1,1]`) Macaulay matrix.
    /// Optionally replaces `E_0` by `E_0 + r * E_1` when `cancel_a0_sq` is true (cancels `r a0^2`,
    /// dropping `deg_w(E_0)` from 14 to 13 under `w = [7,5,3,1]`).
    pub fn build_macaulay_4var_weighted(
        red: &TriangularReduced4,
        r: u64,
        weights: [u16; 4],
        max_w: u16,
        cancel_a0_sq: bool,
    ) -> (Vec<Exp4>, HashMap<Exp4, usize>, [Poly4; 5], CsrMatrix) {
        let mut eqs = red.equations.clone();
        if cancel_a0_sq {
            let e1 = eqs[1].clone();
            scaled_add4(&mut eqs[0], &e1, r);
        }
        let basis = monomials4_weighted(&weights, max_w);
        let index: HashMap<Exp4, usize> =
            basis.iter().copied().enumerate().map(|(i, e)| (e, i)).collect();
        let mut rows = Vec::new();
        for eq in &eqs {
            let deg_eq = poly4_max_weighted_deg(eq, &weights);
            if deg_eq > max_w {
                continue;
            }
            let mult_cap = max_w - deg_eq;
            let mults = monomials4_weighted(&weights, mult_cap);
            for m in &mults {
                let mut row = Vec::with_capacity(eq.len());
                for (e, &val) in eq {
                    if val != 0 {
                        let combined: Exp4 = std::array::from_fn(|k| e[k] + m[k]);
                        let col = *index
                            .get(&combined)
                            .expect("weighted Macaulay degree overflow");
                        row.push((col, val));
                    }
                }
                row.sort_unstable_by_key(|x| x.0);
                rows.push(row);
            }
        }
        let csr = CsrMatrix::from_sorted_rows(basis.len(), rows);
        (basis, index, eqs, csr)
    }

    /// Exact Minkowski pullback support `\pi(M_{<=d})` of 7-variable degree `<= d` under
    /// `(v0, v1, v2) -> (recon0, recon1, recon2)`!
    /// Notice the structural generator supports (independent of curve/residual):
    /// - `a0`: `(1,0,0,0)`
    /// - `a1`: `(0,1,0,0)`
    /// - `a2`: `(0,0,1,0)`
    /// - `a3`: `(0,0,0,1)`
    /// - `v2`: `(0,0,0,2), (0,0,0,0)`
    /// - `v1`: `(0,0,0,4), (0,0,1,1), (0,0,0,2), (0,0,0,1), (0,0,0,0)`
    /// - `v0`: `(0,0,0,6), (0,0,1,3), (0,0,0,4), (0,0,2,0), (0,1,0,1), (0,0,1,1), (0,0,0,2), (0,0,1,0), (0,0,0,1), (0,0,0,0)`
    pub fn pullback_support_4var(max_7var_deg: u8) -> Vec<Exp4> {
        const GEN_EXPS: &[Exp4] = &[
            [0, 0, 0, 0],
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
            [0, 0, 0, 2],
            [0, 0, 0, 4],
            [0, 0, 1, 1],
            [0, 0, 0, 6],
            [0, 0, 1, 3],
            [0, 0, 2, 0],
            [0, 1, 0, 1],
        ];
        let mut current: BTreeSet<Exp4> = BTreeSet::new();
        current.insert([0, 0, 0, 0]);
        for _ in 0..max_7var_deg {
            let mut next = current.clone();
            for &a in &current {
                for &g in GEN_EXPS {
                    next.insert([a[0] + g[0], a[1] + g[1], a[2] + g[2], a[3] + g[3]]);
                }
            }
            current = next;
        }
        let weights = [7u16, 5, 3, 1];
        let mut items: Vec<(u16, Exp4)> = current
            .into_iter()
            .map(|e| (weighted_deg4(&e, &weights), e))
            .collect();
        items.sort_unstable();
        items.into_iter().map(|(_, e)| e).collect()
    }

    pub fn build_macaulay_4var_pullback(
        red: &TriangularReduced4,
        d_7var: u8,
    ) -> (Vec<Exp4>, HashMap<Exp4, usize>, CsrMatrix) {
        assert!(d_7var >= 2);
        let mults = pullback_support_4var(d_7var - 2);
        let cols_basis = pullback_support_4var(d_7var);
        let index: HashMap<Exp4, usize> = cols_basis
            .iter()
            .copied()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect();
        let mut rows = Vec::with_capacity(5 * mults.len());
        for eq in &red.equations {
            for m in &mults {
                let mut row = Vec::with_capacity(eq.len());
                for (e, &val) in eq {
                    if val != 0 {
                        let combined = [e[0] + m[0], e[1] + m[1], e[2] + m[2], e[3] + m[3]];
                        let col = *index
                            .get(&combined)
                            .expect("pullback support must contain pi(m)*E_j");
                        row.push((col, val));
                    }
                }
                row.sort_unstable_by_key(|x| x.0);
                rows.push(row);
            }
        }
        let csr = CsrMatrix::from_sorted_rows(cols_basis.len(), rows);
        (cols_basis, index, csr)
    }

    /// Run scalar Krylov / Wiedemann right-kernel attempt on any `CsrMatrix`.
    #[derive(Clone, Debug, Serialize)]
    pub struct ScalarKrylovOutcome {
        pub rows: usize,
        pub cols: usize,
        pub nnz: usize,
        pub seed: u64,
        pub krylov_moments: usize,
        pub moment_gram_applications: usize,
        pub moment_field_multiplications: u64,
        pub bm_linear_complexity: usize,
        pub bm_terminal_coefficient: u64,
        pub bm_field_multiplications: u64,
        pub bm_field_inversions: u64,
        pub one_sided_full_rank_certified: bool,
        pub horner_gram_applications: usize,
        pub horner_field_multiplications: u64,
        pub direct_mw_field_multiplications: u64,
        pub kernel_vector_nonzero: bool,
        pub direct_mw_zero: bool,
        pub constant_coord_nonzero: bool,
        pub normalized_kernel_sha256: Option<String>,
        pub raw_kernel_sha256: Option<String>,
        pub moment_wall_ns: u128,
        pub total_wall_ns: u128,
    }

    pub fn run_scalar_krylov(m: &CsrMatrix, seed: u64) -> (ScalarKrylovOutcome, Option<Vec<u64>>) {
        let total_start = Instant::now();
        let mut rng = seed;
        let diag: Vec<u64> = (0..m.rows).map(|_| random_nonzero(&mut rng)).collect();
        let u: Vec<u64> = (0..m.cols).map(|_| random_nonzero(&mut rng)).collect();
        let v: Vec<u64> = (0..m.cols).map(|_| random_nonzero(&mut rng)).collect();

        let count = 2 * m.cols;
        let mut cur = v.clone();
        let mut next = vec![0u64; m.cols];
        let mut y_acc = vec![0u128; m.cols];
        let mut seq = Vec::with_capacity(count);
        let mom_start = Instant::now();
        for _ in 0..count {
            seq.push(dot(&u, &cur));
            m.gram_apply_with_scratch(&diag, &cur, &mut next, &mut y_acc);
            mem::swap(&mut cur, &mut next);
        }
        let moment_wall_ns = mom_start.elapsed().as_nanos();
        let per_iter_muls = (2 * m.nnz() + m.rows + m.cols) as u64;
        let moment_field_multiplications = per_iter_muls * count as u64;

        let (l, c, bm_muls, bm_invs) = berlekamp_massey_counted(&seq);
        let term = c[l];
        let full_rank = l == m.cols && term != 0;

        let mut horner_gram_apps = 0usize;
        let mut horner_muls = 0u64;
        let mut direct_mw_muls = 0u64;
        let mut w_nonzero = false;
        let mut mw_zero = false;
        let mut c0_nonzero = false;
        let mut raw_sha = None;
        let mut norm_sha = None;
        let mut normalized_vec = None;

        if l >= 1 && term == 0 {
            let (w, g_apps, h_muls) = evaluate_kernel_horner(m, &diag, &v, &c, l);
            horner_gram_apps = g_apps;
            horner_muls = h_muls;
            w_nonzero = w.iter().any(|&x| x != 0);
            let mw = m.forward_apply(&w);
            direct_mw_muls = m.nnz() as u64;
            mw_zero = w_nonzero && mw.iter().all(|&x| x == 0);
            if mw_zero {
                raw_sha = Some(sha256::vector_u64_le_hex(&w));
                if w[0] != 0 {
                    c0_nonzero = true;
                    let inv_w0 = inv(w[0]);
                    let nw: Vec<u64> = w.iter().map(|&x| mul(x, inv_w0)).collect();
                    norm_sha = Some(sha256::vector_u64_le_hex(&nw));
                    normalized_vec = Some(nw);
                } else {
                    normalized_vec = Some(w);
                }
            }
        }

        let outcome = ScalarKrylovOutcome {
            rows: m.rows,
            cols: m.cols,
            nnz: m.nnz(),
            seed,
            krylov_moments: count,
            moment_gram_applications: count,
            moment_field_multiplications,
            bm_linear_complexity: l,
            bm_terminal_coefficient: term,
            bm_field_multiplications: bm_muls,
            bm_field_inversions: bm_invs,
            one_sided_full_rank_certified: full_rank,
            horner_gram_applications: horner_gram_apps,
            horner_field_multiplications: horner_muls,
            direct_mw_field_multiplications: direct_mw_muls,
            kernel_vector_nonzero: w_nonzero,
            direct_mw_zero: mw_zero,
            constant_coord_nonzero: c0_nonzero,
            normalized_kernel_sha256: norm_sha,
            raw_kernel_sha256: raw_sha,
            moment_wall_ns,
            total_wall_ns: total_start.elapsed().as_nanos(),
        };
        (outcome, normalized_vec)
    }

    /// Check rank-1 monomial consistency on a normalized 7-variable D9 vector (`w[0] == 1`).
    /// Returns `(consistent, candidate_z, checked_monomials, field_muls)`.
    pub fn check_rank_one_7var(
        basis: &[Exp7],
        index: &HashMap<Exp7, usize>,
        nw: &[u64],
    ) -> (bool, [u64; NV7], usize, u64) {
        assert_eq!(basis.len(), nw.len());
        if nw[0] != 1 {
            return (false, [0; NV7], 0, 0);
        }
        let mut z = [0u64; NV7];
        for j in 0..NV7 {
            let mut e = [0u8; NV7];
            e[j] = 1;
            z[j] = nw[index[&e]];
        }
        let mut powers = [[1u64; 10]; NV7];
        let mut muls = 0u64;
        for j in 0..NV7 {
            for d in 1..=9 {
                powers[j][d] = mul(powers[j][d - 1], z[j]);
                muls += 1;
            }
        }
        for (col, exp) in basis.iter().enumerate() {
            let mut expected = 1u64;
            for j in 0..NV7 {
                let d = exp[j] as usize;
                if d > 0 {
                    expected = mul(expected, powers[j][d]);
                    muls += 1;
                }
            }
            if nw[col] != expected {
                return (false, z, col + 1, muls);
            }
        }
        (true, z, basis.len(), muls)
    }

    /// Check rank-1 monomial consistency on a normalized 4-variable vector (`w[0] == 1`).
    pub fn check_rank_one_4var(
        basis: &[Exp4],
        index: &HashMap<Exp4, usize>,
        nw: &[u64],
    ) -> (bool, [u64; NV4], usize, u64) {
        assert_eq!(basis.len(), nw.len());
        if nw[0] != 1 {
            return (false, [0; NV4], 0, 0);
        }
        let mut a = [0u64; NV4];
        for j in 0..NV4 {
            let mut e = [0u8; NV4];
            e[j] = 1;
            a[j] = nw[index[&e]];
        }
        let max_exp = basis
            .iter()
            .flat_map(|e| e.iter().copied())
            .max()
            .unwrap_or(1) as usize;
        let mut powers: Vec<Vec<u64>> = vec![vec![1u64; max_exp + 1]; NV4];
        let mut muls = 0u64;
        for j in 0..NV4 {
            for d in 1..=max_exp {
                powers[j][d] = mul(powers[j][d - 1], a[j]);
                muls += 1;
            }
        }
        for (col, exp) in basis.iter().enumerate() {
            let mut expected = 1u64;
            for j in 0..NV4 {
                let d = exp[j] as usize;
                if d > 0 {
                    expected = mul(expected, powers[j][d]);
                    muls += 1;
                }
            }
            if nw[col] != expected {
                return (false, a, col + 1, muls);
            }
        }
        (true, a, basis.len(), muls)
    }
}

fn main() {
    println!("stub: run cargo test first");
}

#[cfg(test)]
mod tests {
    use super::candidate::*;
    use super::*;

    #[test]
    fn probe_4var_degrees_and_7var_positive_arm0() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let xs: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];
        let h = factor_polynomial_b8(&xs);
        let (pos_r, pos_s) = (1_911_579_686u64, 548_820_096u64);
        let (neg0_r, neg0_s) = (625_037_715u64, 2_100_627_251u64);
        let (neg1_r, neg1_s) = (1_923_055_775u64, 307_731_458u64);

        // 1. Check 4-var triangular reconstruction on known positive values from README.md
        let red_pos = build_triangular_4var(PRIME31.a, pos_r, pos_s, &h);
        let known_a = [455_835_358u64, 518_652_820, 754_716_693, 932_383_545];
        let full7 = reconstruct_7var_from_4var(&red_pos, &known_a);
        assert_eq!(
            full7,
            [
                455_835_358,
                518_652_820,
                754_716_693,
                932_383_545,
                62_510_441,
                1_884_059_825,
                1_142_049_404
            ]
        );
        for eq in &red_pos.equations {
            assert_eq!(eval_poly4(eq, &known_a), 0);
        }

        // 2. Check weighted [7,5,3,1] degrees FIRST to find the exact minimal separating W!
        let red_neg0 = build_triangular_4var(PRIME31.a, neg0_r, neg0_s, &h);
        let red_neg1 = build_triangular_4var(PRIME31.a, neg1_r, neg1_s, &h);
        for (cancel, label) in [(false, "raw_E0"), (true, "cancel_a0_sq")] {
            for max_w in (30u16..=56).step_by(2) {
                let (_, _, _, w_csr_neg0) =
                    build_macaulay_4var_weighted(&red_neg0, neg0_r, [7, 5, 3, 1], max_w, cancel);
                let (w_out_neg0, _) = run_scalar_krylov(&w_csr_neg0, 0x0d99_0001);
                eprintln!(
                    "4var weighted[7,5,3,1] ({label}) W={max_w}: rows={}, cols={}, nnz={}, L={}/{}, full_rank={}, wall_ms={:.2}",
                    w_csr_neg0.rows,
                    w_csr_neg0.cols,
                    w_csr_neg0.nnz(),
                    w_out_neg0.bm_linear_complexity,
                    w_csr_neg0.cols,
                    w_out_neg0.one_sided_full_rank_certified,
                    w_out_neg0.total_wall_ns as f64 / 1e6
                );
                if w_out_neg0.one_sided_full_rank_certified {
                    // Check fine-grained W-1 as well!
                    let prev_w = max_w - 1;
                    let (_, _, _, w_prev_neg0) =
                        build_macaulay_4var_weighted(&red_neg0, neg0_r, [7, 5, 3, 1], prev_w, cancel);
                    let (w_prev_out, _) = run_scalar_krylov(&w_prev_neg0, 0x0d99_0001);
                    eprintln!(
                        "  check W={prev_w}: rows={}, cols={}, nnz={}, L={}/{}, full_rank={}",
                        w_prev_neg0.rows,
                        w_prev_neg0.cols,
                        w_prev_neg0.nnz(),
                        w_prev_out.bm_linear_complexity,
                        w_prev_neg0.cols,
                        w_prev_out.one_sided_full_rank_certified
                    );
                    let chosen_w = if w_prev_out.one_sided_full_rank_certified { prev_w } else { max_w };
                    let (_, _, _, w_csr_neg1) =
                        build_macaulay_4var_weighted(&red_neg1, neg1_r, [7, 5, 3, 1], chosen_w, cancel);
                    let (w_out_neg1, _) = run_scalar_krylov(&w_csr_neg1, 0x0d99_0002);
                    let (w_basis_p, w_idx_p, _, w_csr_p) =
                        build_macaulay_4var_weighted(&red_pos, pos_r, [7, 5, 3, 1], chosen_w, cancel);
                    let (w_out_p, w_vec_p) = run_scalar_krylov(&w_csr_p, 0x0d99_1001);
                    eprintln!(
                        "  -> CHOSEN W={chosen_w} ({label}): neg1 full_rank={}, pos L={}/{}, term={}, mw_zero={}, wall_ms={:.2}",
                        w_out_neg1.one_sided_full_rank_certified,
                        w_out_p.bm_linear_complexity,
                        w_csr_p.cols,
                        w_out_p.bm_terminal_coefficient,
                        w_out_p.direct_mw_zero,
                        w_out_p.total_wall_ns as f64 / 1e6
                    );
                    if let Some(v) = w_vec_p {
                        let (ok, a_got, _, _) = check_rank_one_4var(&w_basis_p, &w_idx_p, &v);
                        eprintln!("  -> rank1={ok}, matches={}", a_got == known_a);
                        assert!(ok);
                        assert_eq!(a_got, known_a);
                    }
                    break;
                }
            }
        }

        // 3. Check unweighted [1,1,1,1] degrees on 4var!
        for d_std in [15u16, 17, 19, 21, 23, 25] {
            let (_, _, _, s_csr_neg0) =
                build_macaulay_4var_weighted(&red_neg0, neg0_r, [1, 1, 1, 1], d_std, false);
            let (s_out_neg0, _) = run_scalar_krylov(&s_csr_neg0, 0x0d99_0001);
            eprintln!(
                "4var unweighted D={d_std}: rows={}, cols={}, nnz={}, L={}/{}, full_rank={}, wall_ms={:.2}",
                s_csr_neg0.rows,
                s_csr_neg0.cols,
                s_csr_neg0.nnz(),
                s_out_neg0.bm_linear_complexity,
                s_csr_neg0.cols,
                s_out_neg0.one_sided_full_rank_certified,
                s_out_neg0.total_wall_ns as f64 / 1e6
            );
            if s_out_neg0.one_sided_full_rank_certified {
                let (s_basis_p, s_idx_p, _, s_csr_p) =
                    build_macaulay_4var_weighted(&red_pos, pos_r, [1, 1, 1, 1], d_std, false);
                let (s_out_p, s_vec_p) = run_scalar_krylov(&s_csr_p, 0x0d99_1001);
                eprintln!(
                    "  -> unweighted D={d_std} pos: L={}/{}, term={}, mw_zero={}",
                    s_out_p.bm_linear_complexity,
                    s_csr_p.cols,
                    s_out_p.bm_terminal_coefficient,
                    s_out_p.direct_mw_zero
                );
                if let Some(v) = s_vec_p {
                    let (ok, a_got, _, _) = check_rank_one_4var(&s_basis_p, &s_idx_p, &v);
                    eprintln!("  -> unweighted D={d_std} rank1={ok}, matches={}", a_got == known_a);
                }
                break;
            }
        }

        // 4. Check 7var D9 positive arm 0 (`0x0d991001`)!
        let (b7, idx7, _, csr7) = build_macaulay_7var(PRIME31.a, pos_r, pos_s, &h, 9);
        assert_eq!(csr7.rows, 27_456);
        assert_eq!(csr7.cols, 11_440);
        assert_eq!(csr7.nnz(), 240_240);
        let (out7, vec7) = run_scalar_krylov(&csr7, 0x0d99_1001);
        eprintln!(
            "7var D9 pos arm0: L={}, term={}, mw_zero={}, wall_s={:.3}",
            out7.bm_linear_complexity,
            out7.bm_terminal_coefficient,
            out7.direct_mw_zero,
            out7.total_wall_ns as f64 / 1e9
        );
        let nw7 = vec7.expect("7var D9 pos arm0 must extract kernel");
        let (ok7, z7, checked7, _) = check_rank_one_7var(&b7, &idx7, &nw7);
        assert!(ok7);
        assert_eq!(checked7, 11_440);
        assert_eq!(z7, full7);
    }
}
