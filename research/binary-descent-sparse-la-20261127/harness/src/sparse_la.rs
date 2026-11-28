//! Arm D-S linear algebra: structured Gaussian elimination (SGE) followed by
//! Wiedemann mod ℓ on the remainder.  Study `bdsl-20261127/v1`.
//!
//! Input: the relation rows of arm D (#5253 `Row`: columns = factor-base
//! classes with small signed coefficients, right-hand side `a + b·d`).
//! Output: `d` with `Σ c_j (a_j + b_j d) = 0` for a non-zero left-kernel
//! vector `c` of the column matrix, or `None` (no useful dependency yet).
//!
//! Every row carries its right-hand side `(a, b)` through every row
//! operation, so no back-substitution is ever needed: a left-kernel vector of
//! the reduced matrix gives `d` directly.
//!
//! Counting (PROTOCOL.md "Cost unit"): through #5253's counted field `CF`,
//! every product mod ℓ is one `z`, every addition mod ℓ one `za`, an
//! inversion mod ℓ `2·bits(ℓ)` `z` (the #5253 `inv_mod` charge).  A product
//! with an operand equal to ±1 is a sign change, not a product, and is not
//! charged (the rule of #5253's own Wiedemann `kernel()`); every such skipped
//! product is counted in `Cnt::pm1_skipped` so the conservative reading that
//! charges them can be reported.  Index bookkeeping (adjacency lists, weight
//! counters, heap operations, the peeling check) is not mod-ℓ arithmetic;
//! #5253's echelon leaves its own bookkeeping (BTreeMap/HashMap pivots)
//! uncharged, and so does this module in the primary reading, but every
//! such operation is counted in `Cnt::structural` and reported.
use crate::field::CF;
use crate::ic::Row;
use crate::seeds::Sm64;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cnt {
    /// Index bookkeeping operations (not charged in the primary reading).
    pub structural: u64,
    /// Products with a ±1 operand (not charged in the primary reading).
    pub pm1_skipped: u64,
}

#[derive(Clone, Debug, Default)]
pub struct LaReport {
    pub rows_in: usize,
    pub cols_in: usize,
    pub core_rows: usize,
    pub core_cols: usize,
    pub sge_pivots: u64,
    pub sge_singletons: u64,
    pub sge_empty_useless: u64,
    pub sge_empty_useful: bool,
    pub rows_out: usize,
    pub cols_out: usize,
    pub nnz_out: usize,
    pub excess_pruned: u64,
    pub wiedemann_attempts: u32,
    pub wiedemann_den_zero: u32,
    pub wiedemann_fail: u32,
    pub minpoly_deg: usize,
    pub z_sge: u64,
    pub za_sge: u64,
    pub z_wied: u64,
    pub za_wied: u64,
    /// Track mode only: the left-kernel vector over the *input* rows
    /// (sparse, (row index, coefficient)), for the uncounted cross-check.
    pub orig_combination: Option<Vec<(u32, u64)>>,
}

#[inline(always)]
fn mm(a: u64, b: u64, ell: u64) -> u64 {
    ((a as u128 * b as u128) % ell as u128) as u64
}
#[inline(always)]
fn pm1(v: u64, ell: u64) -> bool {
    v == 1 || v == ell - 1
}

/// Counted product: free (sign change) if either operand is ±1.
#[inline(always)]
fn cmul(cf: &CF, cnt: &mut Cnt, a: u64, b: u64, ell: u64) -> u64 {
    if pm1(a, ell) || pm1(b, ell) {
        cnt.pm1_skipped += 1;
    } else {
        cf.zmul(1);
    }
    mm(a, b, ell)
}
#[inline(always)]
fn cadd(cf: &CF, a: u64, b: u64, ell: u64) -> u64 {
    cf.zadd(1);
    let s = a + b;
    if s >= ell {
        s - ell
    } else {
        s
    }
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mm(r, a, m);
        }
        a = mm(a, a, m);
        e >>= 1;
    }
    r
}
/// Inversion mod prime ℓ, charged `2·bits(ℓ)` products (#5253 `inv_mod`).
fn inv_mod(cf: &CF, a: u64, ell: u64) -> u64 {
    cf.zmul(2 * (64 - ell.leading_zeros()) as u64);
    powmod(a, ell - 2, ell)
}

fn signed(e: i64, ell: u64) -> u64 {
    ((e % ell as i64) + ell as i64) as u64 % ell
}

/// Rows/columns of the 2-core (iterated removal of rows that hold a
/// column of weight 1) and its excess `core_rows − core_cols`.  Structural
/// only; every adjacency write and weight update is one `structural` op.
pub fn core_excess(rows: &[Row], ncol_bound: usize, cnt: &mut Cnt) -> (i64, usize, usize) {
    let mut w = vec![0u32; ncol_bound];
    let mut start = vec![0u32; ncol_bound + 1];
    for r in rows {
        for &(c, e) in &r.cols {
            if e != 0 {
                w[c] += 1;
                cnt.structural += 1;
            }
        }
    }
    for c in 0..ncol_bound {
        start[c + 1] = start[c] + w[c];
    }
    let mut fill = start.clone();
    let mut adj = vec![0u32; start[ncol_bound] as usize];
    for (j, r) in rows.iter().enumerate() {
        for &(c, e) in &r.cols {
            if e != 0 {
                adj[fill[c] as usize] = j as u32;
                fill[c] += 1;
                cnt.structural += 1;
            }
        }
    }
    let mut alive = vec![true; rows.len()];
    let mut stack: Vec<usize> = (0..ncol_bound).filter(|&c| w[c] == 1).collect();
    cnt.structural += ncol_bound as u64;
    while let Some(c) = stack.pop() {
        if w[c] != 1 {
            continue;
        }
        // the one live row on c
        let mut rr = None;
        for k in start[c]..start[c + 1] {
            cnt.structural += 1;
            let j = adj[k as usize] as usize;
            if alive[j] {
                rr = Some(j);
                break;
            }
        }
        let Some(j) = rr else { continue };
        alive[j] = false;
        for &(c2, e) in &rows[j].cols {
            if e != 0 {
                w[c2] -= 1;
                cnt.structural += 1;
                if w[c2] == 1 {
                    stack.push(c2);
                }
            }
        }
    }
    let core_rows = alive.iter().filter(|&&a| a).count();
    let core_cols = w.iter().filter(|&&x| x > 0).count();
    cnt.structural += (rows.len() + ncol_bound) as u64;
    (core_rows as i64 - core_cols as i64, core_rows, core_cols)
}

#[derive(Clone)]
struct SR {
    c: Vec<(u32, u64)>, // sorted by column, non-zero coefficients
    a: u64,
    b: u64,
    src: Vec<(u32, u64)>, // track mode: combination of input rows
}

/// `cp·r − cr·p` (cp = coefficient of `col` in p, cr = in r), counted.
fn combine(cf: &CF, cnt: &mut Cnt, r: &SR, p: &SR, col: u32, ell: u64, track: bool) -> SR {
    let cp = p.c.iter().find(|e| e.0 == col).unwrap().1;
    let cr = r.c.iter().find(|e| e.0 == col).unwrap().1;
    let ncr = ell - cr;
    let mut out = Vec::with_capacity(r.c.len() + p.c.len());
    let (mut i, mut j) = (0, 0);
    while i < r.c.len() || j < p.c.len() {
        let ci = if i < r.c.len() { r.c[i].0 } else { u32::MAX };
        let cj = if j < p.c.len() { p.c[j].0 } else { u32::MAX };
        if ci == col && cj == col {
            i += 1;
            j += 1;
            continue; // cancels by construction
        }
        if ci < cj {
            out.push((ci, cmul(cf, cnt, cp, r.c[i].1, ell)));
            i += 1;
        } else if cj < ci {
            out.push((cj, cmul(cf, cnt, ncr, p.c[j].1, ell)));
            j += 1;
        } else {
            let x = cmul(cf, cnt, cp, r.c[i].1, ell);
            let y = cmul(cf, cnt, ncr, p.c[j].1, ell);
            let s = cadd(cf, x, y, ell);
            if s != 0 {
                out.push((ci, s));
            }
            i += 1;
            j += 1;
        }
        cnt.structural += 1;
    }
    let a = {
        let x = cmul(cf, cnt, cp, r.a, ell);
        let y = cmul(cf, cnt, ncr, p.a, ell);
        cadd(cf, x, y, ell)
    };
    let b = {
        let x = cmul(cf, cnt, cp, r.b, ell);
        let y = cmul(cf, cnt, ncr, p.b, ell);
        cadd(cf, x, y, ell)
    };
    let src = if track {
        // uncounted (verification only)
        let mut m: std::collections::BTreeMap<u32, u64> = Default::default();
        for &(k, v) in &r.src {
            let e = m.entry(k).or_insert(0);
            *e = (*e + mm(cp, v, ell)) % ell;
        }
        for &(k, v) in &p.src {
            let e = m.entry(k).or_insert(0);
            *e = (*e + mm(ncr, v, ell)) % ell;
        }
        m.into_iter().filter(|e| e.1 != 0).collect()
    } else {
        vec![]
    };
    SR { c: out, a, b, src }
}

/// Wiedemann cost model used only to decide when SGE stops (PROTOCOL.md):
/// `N·(6·nnz + 4·N)` for an `N`-row remainder with `nnz` entries.
fn est(n: usize, nnz: usize) -> f64 {
    let n = n as f64;
    n * (6.0 * nnz as f64 + 4.0 * n)
}

/// Largest excess kept for the Wiedemann step (PROTOCOL.md).
pub const MAX_EXCESS: usize = 2;

pub struct Params {
    /// Largest column weight SGE will pivot on.
    pub kmax: usize,
    /// Wiedemann attempts (fresh random projections) before giving up.
    pub attempts: u32,
    /// Non-zeros of the sparse left projection `u`.
    pub u_weight: usize,
}

/// Solve for `d` from `rows`; `ncol_bound` bounds the column ids.
/// `rng` drives Wiedemann's projections (a frozen stream per call).
pub fn solve(cf: &CF, cnt: &mut Cnt, rows: &[Row], ncol_bound: usize, ell: u64, rng: &mut Sm64, prm: &Params, track: bool, rep: &mut LaReport) -> Option<u64> {
    let z0 = cf.ops();
    rep.rows_in = rows.len();
    // ── load (sign conversion only: no products) ──
    let mut rs: Vec<Option<SR>> = Vec::with_capacity(rows.len());
    for (j, r) in rows.iter().enumerate() {
        let mut c: Vec<(u32, u64)> = r.cols.iter().filter(|e| e.1 != 0).map(|&(c, e)| (c as u32, signed(e, ell))).filter(|e| e.1 != 0).collect();
        c.sort_unstable();
        cnt.structural += c.len() as u64;
        rs.push(Some(SR { c, a: r.a, b: r.b, src: if track { vec![(j as u32, 1)] } else { vec![] } }));
    }
    // ── column adjacency (exact: every list holds exactly the live rows that contain the column) ──
    let mut col: Vec<Vec<u32>> = vec![vec![]; ncol_bound];
    for (j, r) in rs.iter().enumerate() {
        for &(c, _) in &r.as_ref().unwrap().c {
            col[c as usize].push(j as u32);
            cnt.structural += 1;
        }
    }
    rep.cols_in = col.iter().filter(|v| !v.is_empty()).count();
    let mut n_alive = rs.len();
    let mut nnz: usize = rs.iter().map(|r| r.as_ref().unwrap().c.len()).sum();
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = BinaryHeap::new();
    for (c, v) in col.iter().enumerate() {
        if !v.is_empty() {
            heap.push(Reverse((v.len() as u32, c as u32)));
            cnt.structural += 1;
        }
    }
    let mut found: Option<(u64, Vec<(u32, u64)>)> = None;
    // remove row j from every column list
    fn unlink(col: &mut [Vec<u32>], r: &SR, j: u32, cnt: &mut Cnt) {
        for &(c, _) in &r.c {
            let v = &mut col[c as usize];
            let k = v.iter().position(|&x| x == j).unwrap();
            cnt.structural += k as u64 + 1;
            v.swap_remove(k);
        }
    }
    let mut core_done = false;
    while let Some(Reverse((w, c))) = heap.pop() {
        cnt.structural += 1;
        let cu = c as usize;
        if col[cu].len() as u32 != w || w == 0 {
            if !col[cu].is_empty() && col[cu].len() as u32 != w {
                // stale key: re-file under its current weight
                heap.push(Reverse((col[cu].len() as u32, c)));
                cnt.structural += 1;
            }
            continue;
        }
        if w >= 2 && !core_done {
            // first time the smallest weight is >= 2: the 2-core is reached
            core_done = true;
            rep.core_rows = n_alive;
            rep.core_cols = col.iter().filter(|v| !v.is_empty()).count();
        }
        if w == 1 {
            // singleton column: its row cannot be in any dependency
            let j = col[cu][0];
            let r = rs[j as usize].take().unwrap();
            unlink(&mut col, &r, j, cnt);
            nnz -= r.c.len();
            n_alive -= 1;
            rep.sge_singletons += 1;
            for &(c2, _) in &r.c {
                let l = col[c2 as usize].len();
                if l > 0 {
                    heap.push(Reverse((l as u32, c2)));
                    cnt.structural += 1;
                }
            }
            continue;
        }
        if w as usize > prm.kmax {
            continue; // weight bound: never pivot on this column at this weight
        }
        // pivot row: the shortest live row on c
        let (mut pj, mut pl) = (u32::MAX, usize::MAX);
        for &j in &col[cu] {
            cnt.structural += 1;
            let l = rs[j as usize].as_ref().unwrap().c.len();
            if l < pl || (l == pl && j < pj) {
                pl = l;
                pj = j;
            }
        }
        let others: Vec<u32> = col[cu].iter().copied().filter(|&j| j != pj).collect();
        let sum_other: usize = others.iter().map(|&j| rs[j as usize].as_ref().unwrap().c.len()).sum();
        let k = w as usize;
        let delta = (k - 1) as i64 * (pl as i64 - 2) - pl as i64;
        let opcost = 2.0 * ((k - 1) * pl + sum_other) as f64;
        let after = est(n_alive - 1, (nnz as i64 + delta).max(0) as usize) + opcost;
        if after >= est(n_alive, nnz) {
            continue; // not beneficial now; re-filed if its weight changes
        }
        // eliminate c from every other row with the pivot row, then drop the pivot row
        let p = rs[pj as usize].take().unwrap();
        unlink(&mut col, &p, pj, cnt);
        nnz -= p.c.len();
        n_alive -= 1;
        rep.sge_pivots += 1;
        let mut touched: Vec<u32> = p.c.iter().map(|e| e.0).collect();
        for j in others {
            let r = rs[j as usize].take().unwrap();
            unlink(&mut col, &r, j, cnt);
            nnz -= r.c.len();
            let nr = combine(cf, cnt, &r, &p, c, ell, track);
            if nr.c.is_empty() {
                n_alive -= 1;
                if nr.b != 0 && found.is_none() {
                    // a dependency found by elimination: d = −a/b
                    let iv = inv_mod(cf, nr.b, ell);
                    cf.zmul(1);
                    found = Some((mm((ell - nr.a) % ell, iv, ell), nr.src.clone()));
                } else if nr.b == 0 {
                    rep.sge_empty_useless += 1;
                }
                continue;
            }
            for &(c2, _) in &nr.c {
                col[c2 as usize].push(j);
                cnt.structural += 1;
                touched.push(c2);
            }
            nnz += nr.c.len();
            rs[j as usize] = Some(nr);
        }
        if found.is_some() {
            break;
        }
        touched.sort_unstable();
        touched.dedup();
        for c2 in touched {
            let l = col[c2 as usize].len();
            if l > 0 {
                heap.push(Reverse((l as u32, c2)));
                cnt.structural += 1;
            }
        }
    }
    if !core_done {
        rep.core_rows = n_alive;
        rep.core_cols = col.iter().filter(|v| !v.is_empty()).count();
    }
    let z1 = cf.ops();
    rep.z_sge = z1.z - z0.z;
    rep.za_sge = z1.za - z0.za;
    if let Some((d, src)) = found {
        rep.sge_empty_useful = true;
        if track {
            rep.orig_combination = Some(src);
        }
        return Some(d);
    }
    // ── excess pruning: drop the heaviest rows until the excess is at most
    //    MAX_EXCESS (each dense row of the Wiedemann step costs R' per product) ──
    let mut live: Vec<usize> = (0..rs.len()).filter(|&j| rs[j].is_some()).collect();
    loop {
        let ncols_now = col.iter().filter(|v| !v.is_empty()).count();
        if live.len() <= ncols_now + MAX_EXCESS {
            break;
        }
        let (pos, _) = live.iter().enumerate().max_by_key(|(_, &j)| (rs[j].as_ref().unwrap().c.len(), std::cmp::Reverse(j))).unwrap();
        let j = live.remove(pos);
        let r = rs[j].take().unwrap();
        unlink(&mut col, &r, j as u32, cnt);
        nnz -= r.c.len();
        rep.excess_pruned += 1;
        cnt.structural += live.len() as u64;
    }
    // ── remainder: compact rows and columns ──
    let mut cmap = vec![u32::MAX; ncol_bound];
    let mut nc = 0u32;
    for c in 0..ncol_bound {
        if !col[c].is_empty() {
            cmap[c] = nc;
            nc += 1;
        }
    }
    let m: Vec<SR> = live
        .iter()
        .map(|&j| {
            let r = rs[j].as_ref().unwrap();
            SR { c: r.c.iter().map(|&(c, v)| (cmap[c as usize], v)).collect(), a: r.a, b: r.b, src: r.src.clone() }
        })
        .collect();
    cnt.structural += (ncol_bound + nnz) as u64;
    rep.rows_out = m.len();
    rep.cols_out = nc as usize;
    rep.nnz_out = nnz;
    let res = if m.len() > nc as usize { wiedemann(cf, cnt, &m, nc as usize, ell, rng, prm, track, rep) } else { None };
    let z2 = cf.ops();
    rep.z_wied = z2.z - z1.z;
    rep.za_wied = z2.za - z1.za;
    res
}

/// Left kernel of the reduced `R'×C'` matrix `M` (`R' > C'`), i.e. the
/// kernel of `W = Mᵀ` (`C'×R'`).  `k = R' − C'` is the excess.  Each attempt
/// stacks `k − 1` dense random rows under `W` and one zero row, giving a
/// square `R'×R'` matrix `B` whose kernel is (generically) a one-dimensional
/// random subspace of `ker W`, so the vector found is a random kernel element
/// (useful, `Σ c_j b_j ≠ 0`, with probability `1 − 1/ℓ` whenever a useful
/// dependency exists).  Without those rows, a kernel vector found inside the
/// image of a padded `B` is supported on the first `C'` rows only, which can
/// lock onto a sparse useless dependency (seen in pre-freeze self-tests).
/// Then Wiedemann: sparse random projection `u`, Berlekamp–Massey,
/// `y = g(B)v`, and `B^k y` until 0.
fn wiedemann(cf: &CF, cnt: &mut Cnt, m: &[SR], nc: usize, ell: u64, rng: &mut Sm64, prm: &Params, track: bool, rep: &mut LaReport) -> Option<u64> {
    let n = m.len();
    let k1 = n - nc - 1; // dense random rows
    // B v : out[col] = Σ_j coeff_{j,col} v[j] (one za per entry, one z unless ±1);
    //       out[nc+i] = ρ_i · v (n z + n za each)
    let matvec = |cnt: &mut Cnt, v: &[u64], rho: &[Vec<u64>]| -> Vec<u64> {
        let mut out = vec![0u64; n];
        for (j, r) in m.iter().enumerate() {
            let x = v[j];
            if x == 0 {
                continue;
            }
            for &(c, e) in &r.c {
                let t = cmul(cf, cnt, e, x, ell);
                out[c as usize] = cadd(cf, out[c as usize], t, ell);
            }
        }
        for (i, rr) in rho.iter().enumerate() {
            let mut acc = 0u64;
            for (j, &x) in v.iter().enumerate() {
                let t = cmul(cf, cnt, rr[j], x, ell);
                acc = cadd(cf, acc, t, ell);
            }
            out[nc + i] = acc;
        }
        out
    };
    let live = nc + k1; // coordinates of B v that can be non-zero
    for _ in 0..prm.attempts {
        rep.wiedemann_attempts += 1;
        let rho: Vec<Vec<u64>> = (0..k1).map(|_| (0..n).map(|_| rng.below(ell)).collect()).collect();
        let uw = prm.u_weight.min(live);
        let mut uix: Vec<(usize, u64)> = vec![];
        while uix.len() < uw {
            let k = rng.below(live as u64) as usize;
            if uix.iter().all(|e| e.0 != k) {
                uix.push((k, 1 + rng.below(ell - 1)));
            }
        }
        let v: Vec<u64> = (0..n).map(|_| rng.below(ell)).collect();
        let dot = |x: &[u64]| -> u64 {
            let mut acc = 0u64;
            for &(k, uk) in &uix {
                cf.zmul(1);
                cf.zadd(1);
                acc = (acc + mm(uk, x[k], ell)) % ell;
            }
            acc
        };
        // rank B <= live, so its minimal polynomial has degree <= live + 1
        let len = 2 * (live + 1) + 4;
        let mut seq = Vec::with_capacity(len);
        let mut w = v.clone();
        for i in 0..len {
            seq.push(dot(&w));
            if i + 1 < len {
                w = matvec(cnt, &w, &rho);
            }
        }
        // Berlekamp–Massey mod ℓ (every product a z, every addition a za)
        let mut cpoly = vec![1u64];
        let mut bpoly = vec![1u64];
        let (mut l, mut mshift, mut bd) = (0usize, 1usize, 1u64);
        for i in 0..len {
            let mut d = seq[i];
            for k in 1..=l.min(cpoly.len() - 1) {
                d = (d + mm(cpoly[k], seq[i - k], ell)) % ell;
            }
            let lk = l.min(cpoly.len() - 1) as u64;
            cf.zmul(lk);
            cf.zadd(lk);
            if d == 0 {
                mshift += 1;
                continue;
            }
            let coef = mm(d, inv_mod(cf, bd, ell), ell);
            cf.zmul(1);
            let t = cpoly.clone();
            if cpoly.len() < bpoly.len() + mshift {
                cpoly.resize(bpoly.len() + mshift, 0);
            }
            for (k, &bk) in bpoly.iter().enumerate() {
                cpoly[k + mshift] = (cpoly[k + mshift] + ell - mm(coef, bk, ell)) % ell;
            }
            cf.zmul(bpoly.len() as u64);
            cf.zadd(bpoly.len() as u64);
            if 2 * l <= i {
                l = i + 1 - l;
                bpoly = t;
                bd = d;
                mshift = 1;
            } else {
                mshift += 1;
            }
        }
        cpoly.resize(l + 1, 0);
        rep.minpoly_deg = l;
        // connection polynomial C(x) = Σ c_k x^k ; minimal polynomial f(x) = x^l C(1/x)
        // f(x) = Σ_k c_k x^{l−k}.  Lowest non-zero power of f: x^e with e = l − k*,
        // k* = largest k with c_k ≠ 0.  g(x) = f(x)/x^e = Σ_{k<=k*} c_k x^{k*−k}.
        let Some(kstar) = (0..=l).rev().find(|&k| cpoly[k] != 0) else {
            rep.wiedemann_fail += 1;
            continue;
        };
        let e = l - kstar;
        if e == 0 {
            rep.wiedemann_fail += 1;
            continue;
        }
        // y = g(B) v by Horner: y = c_0 v; y = B y + c_k v
        let mut y: Vec<u64> = v.iter().map(|&x| cmul(cf, cnt, cpoly[0], x, ell)).collect();
        for k in 1..=kstar {
            let by = matvec(cnt, &y, &rho);
            let ck = cpoly[k];
            y = by.iter().zip(&v).map(|(&a, &b)| { let t = cmul(cf, cnt, ck, b, ell); cadd(cf, a, t, ell) }).collect();
        }
        // B^e g(B) v = 0: the last non-zero B^k y is a kernel vector of B
        let mut kv = None;
        for _ in 0..=e + 1 {
            if y.iter().all(|&t| t == 0) {
                break;
            }
            let zz = matvec(cnt, &y, &rho);
            if zz.iter().all(|&t| t == 0) {
                kv = Some(y);
                break;
            }
            y = zz;
        }
        let Some(c) = kv else {
            rep.wiedemann_fail += 1;
            continue;
        };
        // exact check of the dependency on the reduced matrix (counted)
        let mut acc = vec![0u64; nc];
        for (j, r) in m.iter().enumerate() {
            if c[j] == 0 {
                continue;
            }
            for &(col, ev) in &r.c {
                let t = cmul(cf, cnt, ev, c[j], ell);
                acc[col as usize] = cadd(cf, acc[col as usize], t, ell);
            }
        }
        if acc.iter().any(|&t| t != 0) {
            rep.wiedemann_fail += 1;
            continue;
        }
        let (mut num, mut den) = (0u64, 0u64);
        for (j, r) in m.iter().enumerate() {
            if c[j] == 0 {
                continue;
            }
            let x = cmul(cf, cnt, c[j], r.a, ell);
            num = cadd(cf, num, x, ell);
            let y2 = cmul(cf, cnt, c[j], r.b, ell);
            den = cadd(cf, den, y2, ell);
        }
        if den == 0 {
            rep.wiedemann_den_zero += 1;
            if k1 == 0 {
                // excess 1: the kernel is (generically) this one line, and it is useless;
                // another projection would find it again, so more relations are needed
                break;
            }
            continue;
        }
        let iv = inv_mod(cf, den, ell);
        cf.zmul(1);
        let d = mm((ell - num) % ell, iv, ell);
        if track {
            let mut comb: std::collections::BTreeMap<u32, u64> = Default::default();
            for (j, r) in m.iter().enumerate() {
                if c[j] == 0 {
                    continue;
                }
                for &(k, v) in &r.src {
                    let e2 = comb.entry(k).or_insert(0);
                    *e2 = (*e2 + mm(c[j], v, ell)) % ell;
                }
            }
            rep.orig_combination = Some(comb.into_iter().filter(|e| e.1 != 0).collect());
        }
        return Some(d);
    }
    None
}

/// Uncounted verification: `comb` is a left-kernel vector of the input rows
/// and gives `d` (`Σ c_j (a_j + b_j d) = 0`).
pub fn check_combination(rows: &[Row], comb: &[(u32, u64)], ell: u64, d: u64) -> bool {
    if comb.is_empty() {
        return false;
    }
    let mut acc: std::collections::HashMap<usize, u64> = Default::default();
    let (mut num, mut den) = (0u64, 0u64);
    for &(j, cj) in comb {
        let r = &rows[j as usize];
        for &(c, e) in &r.cols {
            let t = mm(signed(e, ell), cj, ell);
            let x = acc.entry(c).or_insert(0);
            *x = (*x + t) % ell;
        }
        num = (num + mm(cj, r.a, ell)) % ell;
        den = (den + mm(cj, r.b, ell)) % ell;
    }
    acc.values().all(|&v| v == 0) && den != 0 && (num + mm(den, d, ell)) % ell == 0
}
