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
/// With HUMBERT_LA_THREADS=<n> set this runs `kernel_wiedemann_mt` (same x, same count).
pub fn kernel_wiedemann(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64) -> Solve {
    match la_threads() {
        Some(t) => kernel_wiedemann_mt(rows_in, ncols, r, seed, t),
        None => kernel_wiedemann_serial(rows_in, ncols, r, seed),
    }
}

fn kernel_wiedemann_serial(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64) -> Solve {
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

/// Kernel vector by scalar Lanczos on A = M'^T M' (LaMacchia-Odlyzko), same
/// framing as `kernel_wiedemann`: random diagonal column scaling, one coordinate
/// fixed to 1, every returned vector checked against all rows (M x = 0).
/// About n iterations of one A-product each, against about 3n for scalar
/// Wiedemann (2n for the sequence, n to rebuild the solution).
/// Every mulmod is counted: products, dot products and vector updates.
/// With HUMBERT_LA_THREADS=<n> set this runs `kernel_lanczos_mt` (same x, same count).
pub fn kernel_lanczos(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64) -> Solve {
    match la_threads() {
        Some(t) => kernel_lanczos_mt(rows_in, ncols, r, seed, t),
        None => kernel_lanczos_serial(rows_in, ncols, r, seed),
    }
}

fn kernel_lanczos_serial(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64) -> Solve {
    let mut rng = crate::curve::Rng(seed);
    let mut cnt = 0u64;
    let dot = |a: &[u64], b: &[u64], cnt: &mut u64| -> u64 {
        *cnt += a.len() as u64;
        a.iter().zip(b).fold(0u64, |s, (&x, &y)| (s + mulr(x, y, r)) % r)
    };
    let dscale: Vec<u64> = (0..ncols).map(|_| 1 + rng.below(r - 1)).collect();
    let scaled: Vec<Vec<(usize, u64)>> = rows_in.iter().map(|row| row.iter().map(|&(c, a)| (c, mulr(a, dscale[c], r))).collect()).collect();
    cnt += scaled.iter().map(|x| x.len() as u64).sum::<u64>();
    let rows = &scaled;
    for fixed in (0..ncols).rev().take(6) {
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
        let mut x = vec![0u64; n];
        let mut w = b.clone();
        let mut w_prev = vec![0u64; n];
        let mut v_prev = vec![0u64; n];
        let mut d_prev = 0u64;
        let mut ok_run = false;
        for _ in 0..(n + 10) {
            if w.iter().all(|&e| e == 0) {
                ok_run = true;
                break;
            }
            let mv = matvec(&mp, &w, r, &mut cnt);
            let v = matvec_t(&mp, &mv, n, r, &mut cnt);
            let d = dot(&w, &v, &mut cnt);
            if d == 0 {
                break; // breakdown (probability about n / r)
            }
            let dinv = invr(d, r);
            let s = mulr(dot(&w, &b, &mut cnt), dinv, r);
            let a = mulr(dot(&v, &v, &mut cnt), dinv, r);
            let c = if d_prev == 0 { 0 } else { mulr(dot(&v, &v_prev, &mut cnt), invr(d_prev, r), r) };
            let mut w_next = vec![0u64; n];
            for i in 0..n {
                x[i] = (x[i] + mulr(s, w[i], r)) % r;
                let t = (mulr(a, w[i], r) + mulr(c, w_prev[i], r)) % r;
                w_next[i] = (v[i] + r - t) % r;
            }
            cnt += 3 * n as u64;
            w_prev = std::mem::replace(&mut w, w_next);
            v_prev = v;
            d_prev = d;
        }
        if !ok_run {
            continue;
        }
        let mut full = vec![0u64; ncols];
        for c in 0..ncols {
            full[c] = if c == fixed { 1 } else { x[if c > fixed { c - 1 } else { c }] };
        }
        for c in 0..ncols {
            full[c] = mulr(full[c], dscale[c], r);
        }
        let ok = rows_in.iter().all(|row| row.iter().fold(0u64, |s, &(c, a)| (s + mulr(a, full[c], r)) % r) == 0);
        cnt += rows_in.iter().map(|x| x.len() as u64).sum::<u64>();
        if ok {
            return Solve { kernel_dim: 1, x: Some(full), rank: n, mulmods: cnt };
        }
    }
    Solve { kernel_dim: 0, x: None, rank: 0, mulmods: cnt }
}

// ---------------------------------------------------------------------------
// HUMBERT_LA_THREADS=<n>: multi-threaded Krylov solves (off when unset).
//
// The same Lanczos / Wiedemann recurrences as the serial solvers, run SPMD on
// n scoped threads that live for one Krylov pass and meet at a spin barrier.
// Thread k owns a contiguous block of rows of M' (for M' w) and a contiguous
// block of columns (for M'^T y, the dot products and the vector updates); M'^T y
// reads a column-major copy of M' built once per solve, so every output entry
// has exactly one writer and no atomics read-modify-write or locks are needed.
// All arithmetic is exact mod r, so partial sums combined mod r equal the serial
// sums: the solution vector is identical to the serial one, and so is the mulmod
// count (the same counted multiplications are done, only on different threads).
// Scalars (d, s, a, c and their inverses) are computed by thread 0 alone, as in
// the serial code.
// ---------------------------------------------------------------------------

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Value of HUMBERT_LA_THREADS (>= 1), or None when unset: the serial code path.
pub fn la_threads() -> Option<usize> {
    static T: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();
    *T.get_or_init(|| {
        std::env::var("HUMBERT_LA_THREADS").ok().map(|v| {
            let t: usize = v.trim().parse().expect("HUMBERT_LA_THREADS must be a positive integer");
            assert!(t >= 1, "HUMBERT_LA_THREADS must be >= 1");
            t
        })
    })
}

/// Shared vector: plain loads and stores (relaxed atomics compile to ordinary
/// moves on x86); ordering between writers and readers comes from the barrier.
struct Sv(Vec<AtomicU64>);
impl Sv {
    fn zeros(n: usize) -> Sv {
        Sv((0..n).map(|_| AtomicU64::new(0)).collect())
    }
    fn from(v: &[u64]) -> Sv {
        Sv(v.iter().map(|&x| AtomicU64::new(x)).collect())
    }
    #[inline(always)]
    fn get(&self, i: usize) -> u64 {
        self.0[i].load(Ordering::Relaxed)
    }
    #[inline(always)]
    fn set(&self, i: usize, x: u64) {
        self.0[i].store(x, Ordering::Relaxed)
    }
}

/// Sense-counting spin barrier: spins briefly, then yields (the host is shared).
struct SpinBarrier {
    n: usize,
    count: AtomicUsize,
    gen: AtomicUsize,
}
impl SpinBarrier {
    fn new(n: usize) -> SpinBarrier {
        SpinBarrier { n, count: AtomicUsize::new(0), gen: AtomicUsize::new(0) }
    }
    fn wait(&self) {
        if self.n == 1 {
            return;
        }
        let g = self.gen.load(Ordering::Acquire);
        if self.count.fetch_add(1, Ordering::AcqRel) + 1 == self.n {
            self.count.store(0, Ordering::Relaxed);
            self.gen.store(g.wrapping_add(1), Ordering::Release);
        } else {
            let mut spins = 0u32;
            while self.gen.load(Ordering::Acquire) == g {
                if spins < 2000 {
                    std::hint::spin_loop();
                    spins += 1;
                } else {
                    std::thread::yield_now();
                }
            }
        }
    }
}

/// Run f(0..t) on t threads (f(0) on the caller) and return the results in thread order.
fn spmd<R: Send, F: Fn(usize) -> R + Sync>(t: usize, f: F) -> Vec<R> {
    std::thread::scope(|sc| {
        let f = &f;
        let hs: Vec<_> = (1..t).map(|tid| sc.spawn(move || f(tid))).collect();
        let mut out = vec![f(0)];
        out.extend(hs.into_iter().map(|h| h.join().expect("LA worker thread panicked")));
        out
    })
}

/// t + 1 block boundaries over weights, each block holding about 1/t of the total.
fn split(w: &[u64], t: usize) -> Vec<usize> {
    let total: u64 = w.iter().sum();
    let mut b = vec![0usize];
    let mut acc = 0u64;
    for (i, &wi) in w.iter().enumerate() {
        acc += wi;
        while b.len() < t && acc as u128 * t as u128 >= total as u128 * b.len() as u128 {
            b.push(i + 1);
        }
    }
    while b.len() < t {
        b.push(w.len());
    }
    b.push(w.len());
    b
}

/// M' (row-major, as given) plus a column-major copy and the per-thread row/column blocks.
struct ParOp<'a> {
    t: usize,
    mp: &'a [Vec<(usize, u64)>],
    col_ptr: Vec<usize>,
    col_ent: Vec<(usize, u64)>,
    rlo: Vec<usize>,
    clo: Vec<usize>,
}
impl<'a> ParOp<'a> {
    fn new(mp: &'a [Vec<(usize, u64)>], ncols: usize, t: usize) -> ParOp<'a> {
        let mut deg = vec![0usize; ncols];
        for row in mp {
            for &(c, _) in row {
                deg[c] += 1;
            }
        }
        let mut col_ptr = vec![0usize; ncols + 1];
        for c in 0..ncols {
            col_ptr[c + 1] = col_ptr[c] + deg[c];
        }
        let mut fill = col_ptr.clone();
        let mut col_ent = vec![(0usize, 0u64); col_ptr[ncols]];
        for (i, row) in mp.iter().enumerate() {
            for &(c, a) in row {
                col_ent[fill[c]] = (i, a);
                fill[c] += 1;
            }
        }
        // rows: weight = nnz; columns: nnz plus the ~8 mulmods of dots and updates per entry
        let rw: Vec<u64> = mp.iter().map(|row| row.len() as u64 + 1).collect();
        let cw: Vec<u64> = deg.iter().map(|&d| d as u64 + 8).collect();
        ParOp { t, mp, col_ptr, col_ent, rlo: split(&rw, t), clo: split(&cw, t) }
    }
    /// out[i] = (M' x)[i] for the rows of block `tid`; same count as `matvec`.
    fn mv_rows(&self, tid: usize, x: &Sv, out: &Sv, r: u64) -> u64 {
        let mut cnt = 0u64;
        for i in self.rlo[tid]..self.rlo[tid + 1] {
            let row = &self.mp[i];
            let mut s = 0u64;
            for &(c, a) in row {
                s = (s + mulr(a, x.get(c), r)) % r;
            }
            cnt += row.len() as u64;
            out.set(i, s);
        }
        cnt
    }
    /// out = (M'^T y) on the columns of block `tid`; skips y_i = 0 exactly as
    /// `matvec_t` skips zero rows, so the count is the same.
    fn mvt_cols(&self, tid: usize, y: &Sv, out: &mut [u64], r: u64) -> u64 {
        let mut cnt = 0u64;
        let lo = self.clo[tid];
        for c in lo..self.clo[tid + 1] {
            let mut s = 0u64;
            for &(i, a) in &self.col_ent[self.col_ptr[c]..self.col_ptr[c + 1]] {
                let yi = y.get(i);
                if yi != 0 {
                    s = (s + mulr(a, yi, r)) % r;
                    cnt += 1;
                }
            }
            out[c - lo] = s;
        }
        cnt
    }
}

#[inline]
fn dot_mod(a: &[u64], b: &[u64], r: u64) -> u64 {
    a.iter().zip(b).fold(0u64, |s, (&x, &y)| (s + mulr(x, y, r)) % r)
}

/// M' = M without column `fixed` (later columns shifted down), y = -M[:, fixed].
fn strip_column(rows: &[Vec<(usize, u64)>], fixed: usize, r: u64) -> (Vec<Vec<(usize, u64)>>, Vec<u64>) {
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
    (mp, y)
}

/// Lanczos iterations of `kernel_lanczos_serial` on t threads: Some(x) when w reached 0.
fn lanczos_spmd(op: &ParOp, b: &[u64], n: usize, r: u64) -> (Option<Vec<u64>>, u64) {
    let t = op.t;
    let w_sh = Sv::from(b);
    let mv_sh = Sv::zeros(op.mp.len());
    let part = Sv::zeros(4 * t);
    let zero = Sv::zeros(t);
    let ctl = Sv::zeros(4); // s, a, c, go
    let bar = SpinBarrier::new(t);
    let outs = spmd(t, |tid| {
        let (lo, hi) = (op.clo[tid], op.clo[tid + 1]);
        let len = hi - lo;
        let bl = &b[lo..hi];
        let mut x = vec![0u64; len];
        let mut w: Vec<u64> = bl.to_vec();
        let mut w_prev = vec![0u64; len];
        let mut v = vec![0u64; len];
        let mut v_prev = vec![0u64; len];
        let mut cnt = 0u64;
        let mut d_prev = 0u64; // thread 0 only
        let mut ok_run = false;
        zero.set(tid, w.iter().all(|&e| e == 0) as u64);
        for it in 0..(n + 10) {
            bar.wait(); // w and the zero flags are complete
            if (0..t).all(|k| zero.get(k) == 1) {
                ok_run = true;
                break;
            }
            cnt += op.mv_rows(tid, &w_sh, &mv_sh, r);
            bar.wait(); // M' w is complete
            cnt += op.mvt_cols(tid, &mv_sh, &mut v, r);
            part.set(4 * tid, dot_mod(&w, &v, r));
            part.set(4 * tid + 1, dot_mod(&w, bl, r));
            part.set(4 * tid + 2, dot_mod(&v, &v, r));
            cnt += 3 * len as u64;
            if it > 0 {
                // serial: dot(v, v_prev) only when d_prev != 0, i.e. after the first iteration
                part.set(4 * tid + 3, dot_mod(&v, &v_prev, r));
                cnt += len as u64;
            }
            bar.wait(); // partial dot products are complete
            if tid == 0 {
                let sum = |k: usize| (0..t).fold(0u64, |s, j| (s + part.get(4 * j + k)) % r);
                let d = sum(0);
                if d == 0 {
                    ctl.set(3, 0);
                } else {
                    let dinv = invr(d, r);
                    ctl.set(0, mulr(sum(1), dinv, r));
                    ctl.set(1, mulr(sum(2), dinv, r));
                    ctl.set(2, if d_prev == 0 { 0 } else { mulr(sum(3), invr(d_prev, r), r) });
                    ctl.set(3, 1);
                    d_prev = d;
                }
            }
            bar.wait(); // scalars are published
            if ctl.get(3) == 0 {
                break; // breakdown
            }
            let (s, a, c) = (ctl.get(0), ctl.get(1), ctl.get(2));
            for i in 0..len {
                x[i] = (x[i] + mulr(s, w[i], r)) % r;
                let tt = (mulr(a, w[i], r) + mulr(c, w_prev[i], r)) % r;
                let wn = (v[i] + r - tt) % r;
                w_prev[i] = w[i];
                w[i] = wn;
                w_sh.set(lo + i, wn);
            }
            cnt += 3 * len as u64;
            std::mem::swap(&mut v_prev, &mut v);
            zero.set(tid, w.iter().all(|&e| e == 0) as u64);
        }
        (ok_run, x, cnt)
    });
    let cnt = outs.iter().map(|o| o.2).sum();
    if !outs[0].0 {
        return (None, cnt);
    }
    (Some(outs.into_iter().flat_map(|o| o.1).collect()), cnt)
}

/// `kernel_lanczos` on t threads.
pub fn kernel_lanczos_mt(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64, t: usize) -> Solve {
    let mut rng = crate::curve::Rng(seed);
    let mut cnt = 0u64;
    let dscale: Vec<u64> = (0..ncols).map(|_| 1 + rng.below(r - 1)).collect();
    let scaled: Vec<Vec<(usize, u64)>> = rows_in.iter().map(|row| row.iter().map(|&(c, a)| (c, mulr(a, dscale[c], r))).collect()).collect();
    cnt += scaled.iter().map(|x| x.len() as u64).sum::<u64>();
    for fixed in (0..ncols).rev().take(6) {
        let (mp, y) = strip_column(&scaled, fixed, r);
        let n = ncols - 1;
        let b = matvec_t(&mp, &y, n, r, &mut cnt);
        let op = ParOp::new(&mp, n, t);
        let (x, c) = lanczos_spmd(&op, &b, n, r);
        cnt += c;
        let Some(x) = x else { continue };
        let mut full = vec![0u64; ncols];
        for c in 0..ncols {
            full[c] = if c == fixed { 1 } else { x[if c > fixed { c - 1 } else { c }] };
        }
        for c in 0..ncols {
            full[c] = mulr(full[c], dscale[c], r);
        }
        let ok = rows_in.iter().all(|row| row.iter().fold(0u64, |s, &(c, a)| (s + mulr(a, full[c], r)) % r) == 0);
        cnt += rows_in.iter().map(|x| x.len() as u64).sum::<u64>();
        if ok {
            return Solve { kernel_dim: 1, x: Some(full), rank: n, mulmods: cnt };
        }
    }
    Solve { kernel_dim: 0, x: None, rank: 0, mulmods: cnt }
}

/// Krylov sequence u . A^k b, k < steps, of `kernel_wiedemann_serial` on t threads.
/// Like the serial code, the u . v products are not added to the count.
fn wiedemann_seq_spmd(op: &ParOp, b: &[u64], u: &[u64], steps: usize, r: u64) -> (Vec<u64>, u64) {
    let t = op.t;
    let v_sh = Sv::from(b);
    let mv_sh = Sv::zeros(op.mp.len());
    let part = Sv::zeros(t);
    let bar = SpinBarrier::new(t);
    let outs = spmd(t, |tid| {
        let (lo, hi) = (op.clo[tid], op.clo[tid + 1]);
        let ul = &u[lo..hi];
        let mut v: Vec<u64> = b[lo..hi].to_vec();
        let mut seq = Vec::with_capacity(if tid == 0 { steps } else { 0 });
        let mut cnt = 0u64;
        for _ in 0..steps {
            part.set(tid, dot_mod(ul, &v, r));
            bar.wait(); // v and the partial products are complete
            if tid == 0 {
                seq.push((0..t).fold(0u64, |s, j| (s + part.get(j)) % r));
            }
            cnt += op.mv_rows(tid, &v_sh, &mv_sh, r);
            bar.wait(); // M' v is complete
            cnt += op.mvt_cols(tid, &mv_sh, &mut v, r);
            for (i, &e) in v.iter().enumerate() {
                v_sh.set(lo + i, e);
            }
        }
        (seq, cnt)
    });
    let cnt = outs.iter().map(|o| o.1).sum();
    (outs.into_iter().next().unwrap().0, cnt)
}

/// sum_{k=1}^{L} f_k A^{k-1} b of `kernel_wiedemann_serial` on t threads (the
/// f_k kv updates are not counted, as in the serial code).
fn wiedemann_rebuild_spmd(op: &ParOp, b: &[u64], c: &[u64], ll: usize, r: u64) -> (Vec<u64>, u64) {
    let t = op.t;
    let kv_sh = Sv::from(b);
    let mv_sh = Sv::zeros(op.mp.len());
    let bar = SpinBarrier::new(t);
    let outs = spmd(t, |tid| {
        let (lo, hi) = (op.clo[tid], op.clo[tid + 1]);
        let mut kv: Vec<u64> = b[lo..hi].to_vec();
        let mut x = vec![0u64; hi - lo];
        let mut cnt = 0u64;
        for k in 1..=ll {
            let fk = c[ll - k];
            if fk != 0 {
                for i in 0..x.len() {
                    x[i] = (x[i] + mulr(fk, kv[i], r)) % r;
                }
            }
            if k < ll {
                bar.wait(); // kv is complete
                cnt += op.mv_rows(tid, &kv_sh, &mv_sh, r);
                bar.wait(); // M' kv is complete
                cnt += op.mvt_cols(tid, &mv_sh, &mut kv, r);
                for (i, &e) in kv.iter().enumerate() {
                    kv_sh.set(lo + i, e);
                }
            }
        }
        (x, cnt)
    });
    let cnt = outs.iter().map(|o| o.1).sum();
    (outs.into_iter().flat_map(|o| o.0).collect(), cnt)
}

/// `kernel_wiedemann` on t threads.
pub fn kernel_wiedemann_mt(rows_in: &[Vec<(usize, u64)>], ncols: usize, r: u64, seed: u64, t: usize) -> Solve {
    let mut rng = crate::curve::Rng(seed);
    let mut cnt = 0u64;
    let dscale: Vec<u64> = (0..ncols).map(|_| 1 + rng.below(r - 1)).collect();
    let scaled: Vec<Vec<(usize, u64)>> = rows_in.iter().map(|row| row.iter().map(|&(c, a)| (c, mulr(a, dscale[c], r))).collect()).collect();
    for fixed in (0..ncols).rev().take(6) {
        let (mp, y) = strip_column(&scaled, fixed, r);
        let n = ncols - 1;
        let b = matvec_t(&mp, &y, n, r, &mut cnt);
        let op = ParOp::new(&mp, n, t);
        for _attempt in 0..3 {
            let u: Vec<u64> = (0..n).map(|_| rng.below(r)).collect();
            let (seq, c1) = wiedemann_seq_spmd(&op, &b, &u, 2 * n + 2, r);
            cnt += c1;
            let c = berlekamp_massey(&seq, r);
            let ll = c.len() - 1;
            if ll == 0 || c[ll] == 0 {
                continue;
            }
            let f0inv = invr(c[ll], r);
            let (mut x, c2) = wiedemann_rebuild_spmd(&op, &b, &c, ll, r);
            cnt += c2;
            for xi in x.iter_mut() {
                *xi = mulr((r - *xi) % r, f0inv, r);
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::{invr, mulr, Rng};

    /// Random sparse rows orthogonal to a hidden vector (corank 1), some rows with
    /// an extra zero-weighted shape so that M'^T y meets y_i = 0.
    fn corank1(n: usize, extra: usize, seed: u64, r: u64) -> Vec<Vec<(usize, u64)>> {
        let mut rng = Rng(seed);
        let hidden: Vec<u64> = (0..n).map(|_| 1 + rng.below(r - 1)).collect();
        let mut rows = Vec::new();
        for _ in 0..(n + extra) {
            let mut cols: Vec<usize> = Vec::new();
            while cols.len() < 6 {
                let c = rng.below(n as u64 - 1) as usize;
                if !cols.contains(&c) {
                    cols.push(c);
                }
            }
            let mut acc = 0u64;
            let mut row = Vec::new();
            for &c in &cols {
                let a = 1 + rng.below(r - 1);
                acc = (acc + mulr(a, hidden[c], r)) % r;
                row.push((c, a));
            }
            row.push((n - 1, mulr((r - acc) % r, invr(hidden[n - 1], r), r)));
            rows.push(row);
        }
        rows
    }

    #[test]
    fn threaded_krylov_matches_serial_bit_for_bit() {
        let r = 1_000_003u64;
        for &(n, seed) in &[(40usize, 5u64), (257, 9)] {
            let rows = corank1(n, 5, seed, r);
            let ls = kernel_lanczos_serial(&rows, n, r, 13);
            let ws = kernel_wiedemann_serial(&rows, n, r, 11);
            assert!(ls.x.is_some() && ws.x.is_some());
            for t in [1usize, 2, 3, 4, 7] {
                let lm = kernel_lanczos_mt(&rows, n, r, 13, t);
                assert_eq!((&lm.x, lm.mulmods, lm.rank), (&ls.x, ls.mulmods, ls.rank), "lanczos n={n} t={t}");
                let wm = kernel_wiedemann_mt(&rows, n, r, 11, t);
                assert_eq!((&wm.x, wm.mulmods, wm.rank), (&ws.x, ws.mulmods, ws.rank), "wiedemann n={n} t={t}");
            }
        }
    }

    #[test]
    fn par_matvecs_match_serial_values_and_counts() {
        let r = 1_000_003u64;
        let n = 64;
        let rows = corank1(n, 9, 21, r);
        let mut rng = Rng(77);
        let x: Vec<u64> = (0..n).map(|_| rng.below(r)).collect();
        // y with zero entries: the transposed product must skip the same rows
        let y: Vec<u64> = (0..rows.len()).map(|i| if i % 3 == 0 { 0 } else { rng.below(r) }).collect();
        let (mut c1, mut c2) = (0u64, 0u64);
        let mv = matvec(&rows, &x, r, &mut c1);
        let mvt = matvec_t(&rows, &y, n, r, &mut c2);
        for t in [1usize, 2, 4, 5, 100] {
            let op = ParOp::new(&rows, n, t);
            assert_eq!((op.rlo[0], op.rlo[t], op.clo[0], op.clo[t]), (0, rows.len(), 0, n));
            let (xs, ys, out) = (Sv::from(&x), Sv::from(&y), Sv::zeros(rows.len()));
            let mut k1 = 0u64;
            let mut got_t = vec![0u64; n];
            let mut k2 = 0u64;
            for tid in 0..t {
                k1 += op.mv_rows(tid, &xs, &out, r);
                let (lo, hi) = (op.clo[tid], op.clo[tid + 1]);
                k2 += op.mvt_cols(tid, &ys, &mut got_t[lo..hi], r);
            }
            let got: Vec<u64> = (0..rows.len()).map(|i| out.get(i)).collect();
            assert_eq!((got, k1), (mv.clone(), c1), "matvec t={t}");
            assert_eq!((got_t, k2), (mvt.clone(), c2), "matvec_t t={t}");
        }
    }
}
