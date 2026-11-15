// N4 v2 linear algebra mod ell: B0's Lanczos (overflow-safe adds), Lanczos with the +-1 skip,
// structured Gaussian elimination (SGE) with back substitution, and block Lanczos (width b,
// narrowing when a block loses rank). Every multiplication mod ell is counted (lmul); additions
// are counted (ladd). Inversions mod ell in the new code are charged 60 lmul each (ic2::linv).
#![allow(dead_code)]
use crate::common::*;
use crate::ic::Row;
use crate::ic2::{la2, linv, lm2, ls2};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// ------------------------------------------------------------------ B0's Lanczos, verbatim logic
fn matvec_b0(rows: &[Row], d: &[u64], v: &[u64], ell: u64, out: &mut Vec<u64>, tmp: &mut Vec<u64>) {
    for (i, r) in rows.iter().enumerate() {
        let mut s = 0u64;
        for &(c, a) in r {
            s = la2(s, lm2(a, v[c as usize], ell), ell);
        }
        tmp[i] = lm2(s, d[i], ell);
    }
    for o in out.iter_mut() {
        *o = 0;
    }
    for (i, r) in rows.iter().enumerate() {
        let t = tmp[i];
        if t == 0 {
            continue;
        }
        for &(c, a) in r {
            out[c as usize] = la2(out[c as usize], lm2(a, t, ell), ell);
        }
    }
}
fn dot(a: &[u64], b: &[u64], ell: u64) -> u64 {
    let mut s = 0u64;
    for i in 0..a.len() {
        s = la2(s, lm2(a[i], b[i], ell), ell);
    }
    s
}

/// N4's lanczos_kernel (one kernel vector of A via Lanczos on A^T D A), with overflow-safe adds;
/// `pm1` replaces multiplications by matrix coefficients +-1 with additions/subtractions.
pub fn lanczos(rows: &[Row], n: usize, ell: u64, rng: &mut Rng, pm1: bool) -> Option<Vec<u64>> {
    let d: Vec<u64> = (0..rows.len()).map(|_| 1 + rng.below(ell - 1)).collect();
    let b: Vec<u64> = (0..n).map(|_| rng.below(ell)).collect();
    let mut tmp = vec![0u64; rows.len()];
    let mv = |v: &[u64], out: &mut Vec<u64>, tmp: &mut Vec<u64>| {
        if pm1 {
            matvec_pm1(rows, &d, v, ell, out, tmp)
        } else {
            matvec_b0(rows, &d, v, ell, out, tmp)
        }
    };
    let mut y = vec![0u64; n];
    mv(&b, &mut y, &mut tmp);
    let mut x = vec![0u64; n];
    let mut w = y.clone();
    let mut w_prev: Vec<u64> = vec![0; n];
    let mut v = vec![0u64; n];
    let mut v_prev: Vec<u64> = vec![0; n];
    let mut s_prev = 0u64;
    let mut first = true;
    let mut iters = 0;
    loop {
        if w.iter().all(|&e| e == 0) {
            break;
        }
        mv(&w, &mut v, &mut tmp);
        let s = dot(&w, &v, ell);
        if s == 0 {
            return None;
        }
        let si = if pm1 { linv(s, ell) } else { inv_mod_u64(s, ell) };
        let cy = lm2(dot(&w, &y, ell), si, ell);
        for i in 0..n {
            x[i] = la2(x[i], lm2(cy, w[i], ell), ell);
        }
        let a1 = lm2(dot(&v, &v, ell), si, ell);
        let mut wn = vec![0u64; n];
        if first {
            for i in 0..n {
                wn[i] = ls2(v[i], lm2(a1, w[i], ell), ell);
            }
        } else {
            let sp = if pm1 { linv(s_prev, ell) } else { inv_mod_u64(s_prev, ell) };
            let b1 = lm2(dot(&v, &v_prev, ell), sp, ell);
            for i in 0..n {
                let t = ls2(v[i], lm2(a1, w[i], ell), ell);
                wn[i] = ls2(t, lm2(b1, w_prev[i], ell), ell);
            }
        }
        first = false;
        w_prev = w;
        w = wn;
        v_prev = v.clone();
        s_prev = s;
        iters += 1;
        if iters > n + 5 {
            return None;
        }
    }
    let z: Vec<u64> = (0..n).map(|i| ls2(x[i], b[i], ell)).collect();
    if z.iter().all(|&e| e == 0) {
        return None;
    }
    Some(z)
}

#[inline(always)]
fn acc_coef(s: u64, a: u64, v: u64, ell: u64) -> u64 {
    if a == 1 {
        la2(s, v, ell)
    } else if a == ell - 1 {
        ls2(s, v, ell)
    } else {
        la2(s, lm2(a, v, ell), ell)
    }
}

fn matvec_pm1(rows: &[Row], d: &[u64], v: &[u64], ell: u64, out: &mut Vec<u64>, tmp: &mut Vec<u64>) {
    for (i, r) in rows.iter().enumerate() {
        let mut s = 0u64;
        for &(c, a) in r {
            s = acc_coef(s, a, v[c as usize], ell);
        }
        tmp[i] = lm2(s, d[i], ell);
    }
    for o in out.iter_mut() {
        *o = 0;
    }
    for (i, r) in rows.iter().enumerate() {
        let t = tmp[i];
        if t == 0 {
            continue;
        }
        for &(c, a) in r {
            out[c as usize] = acc_coef(out[c as usize], a, t, ell);
        }
    }
}

pub fn row_apply2(r: &Row, z: &[u64], ell: u64) -> u64 {
    let mut s = 0u64;
    for &(c, a) in r {
        s = acc_coef(s, a, z[c as usize], ell);
    }
    s
}

// ------------------------------------------------------------------ SGE
pub struct Sge {
    pub rows: Vec<Row>,      // reduced rows over reduced columns 0..ncols_red
    pub keep: Vec<u32>,      // reduced column -> original column
    pub elim: Vec<(u32, Row)>, // (original column, pivot row over original columns), in elimination order
    pub free: Vec<u32>,      // original columns left undetermined
    pub n_in: usize,
    pub rows_in: usize,
    pub nnz_in: usize,
    pub singletons: u64,
    pub merges: u64,
    pub dropped: u64,
}

/// Structured Gaussian elimination. `protect` is never eliminated (kappa column).
pub fn sge(rows_in: &[Row], ncols: usize, protect: u32, gamma: f64, wmax: usize, ell: u64) -> Sge {
    let nrows = rows_in.len();
    let mut rows: Vec<Option<Row>> = rows_in.iter().map(|r| Some(r.clone())).collect();
    let mut colr: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    for (ri, r) in rows_in.iter().enumerate() {
        for &(c, _) in r {
            colr[c as usize].push(ri as u32);
        }
    }
    let nnz_in: usize = rows_in.iter().map(|r| r.len()).sum();
    let mut out = Sge { rows: vec![], keep: vec![], elim: vec![], free: vec![], n_in: ncols, rows_in: nrows, nnz_in, singletons: 0, merges: 0, dropped: 0 };
    let mut dead_col = vec![false; ncols]; // eliminated or free
    let mut alive_rows = nrows;
    let mut nnz = nnz_in;
    let mut alive_cols = (0..ncols).filter(|&c| !colr[c].is_empty()).count();
    for c in 0..ncols {
        if colr[c].is_empty() {
            dead_col[c] = true; // never present: not part of the system
        }
    }
    fn remove_row(ri: u32, rows: &mut Vec<Option<Row>>, colr: &mut Vec<Vec<u32>>, nnz: &mut usize, queue: &mut Vec<u32>) {
        if let Some(r) = rows[ri as usize].take() {
            *nnz -= r.len();
            for &(c, _) in &r {
                let l = &mut colr[c as usize];
                if let Some(p) = l.iter().position(|&x| x == ri) {
                    l.swap_remove(p);
                }
                if l.len() <= 1 {
                    queue.push(c);
                }
            }
        }
    }
    let mut queue: Vec<u32> = (0..ncols as u32).filter(|&c| colr[c as usize].len() == 1).collect();
    macro_rules! cascade {
        () => {
            while let Some(c) = queue.pop() {
                let cu = c as usize;
                if dead_col[cu] || c == protect {
                    continue;
                }
                if colr[cu].is_empty() {
                    dead_col[cu] = true;
                    out.free.push(c);
                    alive_cols -= 1;
                    continue;
                }
                if colr[cu].len() != 1 {
                    continue;
                }
                let ri = colr[cu][0];
                let r = rows[ri as usize].clone().unwrap();
                out.elim.push((c, r));
                dead_col[cu] = true;
                alive_cols -= 1;
                out.singletons += 1;
                remove_row(ri, &mut rows, &mut colr, &mut nnz, &mut queue);
                alive_rows -= 1;
            }
        };
    }
    cascade!();
    // excess trimming: heaviest rows first
    let mut order: Vec<u32> = (0..nrows as u32).collect();
    order.sort_by_key(|&r| (Reverse(rows_in[r as usize].len()), Reverse(r)));
    let mut oi = 0;
    loop {
        let target = std::cmp::max(20, (0.02 * alive_cols as f64).ceil() as usize);
        if alive_rows <= alive_cols + target || oi >= order.len() {
            break;
        }
        let ri = order[oi];
        oi += 1;
        if rows[ri as usize].is_none() {
            continue;
        }
        remove_row(ri, &mut rows, &mut colr, &mut nnz, &mut queue);
        alive_rows -= 1;
        out.dropped += 1;
        cascade!();
    }
    // merges, lightest column first
    let mut heap: BinaryHeap<Reverse<(usize, u32)>> = BinaryHeap::new();
    for c in 0..ncols {
        if !dead_col[c] && c as u32 != protect {
            heap.push(Reverse((colr[c].len(), c as u32)));
        }
    }
    while let Some(Reverse((w, c))) = heap.pop() {
        let cu = c as usize;
        if dead_col[cu] || colr[cu].len() != w {
            if !dead_col[cu] && c != protect {
                // stale entry: re-push with current weight if it is lower than before
            }
            continue;
        }
        if w > wmax {
            break;
        }
        if w <= 1 {
            queue.push(c);
            cascade!();
            continue;
        }
        // pivot: lightest row
        let p = *colr[cu].iter().min_by_key(|&&r| (rows[r as usize].as_ref().unwrap().len(), r)).unwrap();
        let plen = rows[p as usize].as_ref().unwrap().len();
        let dn: isize = (w as isize - 1) * (plen as isize - 2) - plen as isize;
        let n0 = alive_cols as f64;
        let j0 = n0 * nnz as f64 + gamma * n0 * n0;
        let j1 = (n0 - 1.0) * (nnz as f64 + dn as f64) + gamma * (n0 - 1.0) * (n0 - 1.0);
        if j1 >= j0 {
            // a heavier column cannot do better under this model only if its fill is larger;
            // greedy stop (protocol: merges while J decreases)
            break;
        }
        let prow = rows[p as usize].clone().unwrap();
        let ap = prow.iter().find(|x| x.0 == c).unwrap().1;
        let (ap_inv, ap_pm) = if ap == 1 { (1, true) } else if ap == ell - 1 { (ell - 1, true) } else { (linv(ap, ell), false) };
        let others: Vec<u32> = colr[cu].iter().copied().filter(|&r| r != p).collect();
        for r in others {
            let old = rows[r as usize].take().unwrap();
            let ar = old.iter().find(|x| x.0 == c).unwrap().1;
            // factor f = ar / ap ; new = old - f * prow
            let fct = if ap_pm { if ap_inv == 1 { ar } else { ell - ar } } else { lm2(ar, ap_inv, ell) };
            let fct_pm = fct == 1 || fct == ell - 1;
            let mut nr: Row = Vec::with_capacity(old.len() + prow.len());
            let (mut i, mut j) = (0, 0);
            while i < old.len() || j < prow.len() {
                if j >= prow.len() || (i < old.len() && old[i].0 < prow[j].0) {
                    nr.push(old[i]);
                    i += 1;
                } else {
                    let (cj, aj) = prow[j];
                    let t = if fct_pm { if fct == 1 { aj } else { if aj == 0 { 0 } else { ell - aj } } } else { lm2(fct, aj, ell) };
                    if i < old.len() && old[i].0 == cj {
                        let v = ls2(old[i].1, t, ell);
                        if v != 0 {
                            nr.push((cj, v));
                        }
                        i += 1;
                    } else {
                        let v = if t == 0 { 0 } else { ell - t };
                        if v != 0 {
                            nr.push((cj, v));
                        }
                    }
                    j += 1;
                }
            }
            // update column lists: removed and added columns
            nnz = nnz - old.len() + nr.len();
            let mut a = 0;
            let mut b2 = 0;
            while a < old.len() || b2 < nr.len() {
                if b2 >= nr.len() || (a < old.len() && old[a].0 < nr[b2].0) {
                    // column removed from row r
                    let l = &mut colr[old[a].0 as usize];
                    if let Some(pp) = l.iter().position(|&x| x == r) {
                        l.swap_remove(pp);
                    }
                    let cc = old[a].0;
                    if !dead_col[cc as usize] && cc != c {
                        heap.push(Reverse((colr[cc as usize].len(), cc)));
                        if colr[cc as usize].len() <= 1 {
                            queue.push(cc);
                        }
                    }
                    a += 1;
                } else if a >= old.len() || nr[b2].0 < old[a].0 {
                    colr[nr[b2].0 as usize].push(r);
                    let cc = nr[b2].0;
                    if !dead_col[cc as usize] && cc != protect {
                        heap.push(Reverse((colr[cc as usize].len(), cc)));
                    }
                    b2 += 1;
                } else {
                    a += 1;
                    b2 += 1;
                }
            }
            rows[r as usize] = Some(nr);
        }
        out.elim.push((c, prow));
        dead_col[cu] = true;
        alive_cols -= 1;
        out.merges += 1;
        remove_row(p, &mut rows, &mut colr, &mut nnz, &mut queue);
        alive_rows -= 1;
        cascade!();
        bump_hash(0);
    }
    // reduced system
    let mut map = vec![u32::MAX; ncols];
    for c in 0..ncols {
        if !dead_col[c] {
            map[c] = out.keep.len() as u32;
            out.keep.push(c as u32);
        }
    }
    for r in rows.into_iter().flatten() {
        let rr: Row = r.iter().map(|&(c, a)| (map[c as usize], a)).collect();
        debug_assert!(rr.iter().all(|x| x.0 != u32::MAX));
        out.rows.push(rr);
    }
    out
}

/// Full solution from the reduced kernel vector: free columns unknown (None), eliminated columns
/// by back substitution in reverse order.
pub fn back_substitute(s: &Sge, zr: &[u64], ncols: usize, ell: u64) -> Vec<Option<u64>> {
    let mut z: Vec<Option<u64>> = vec![None; ncols];
    for (k, &c) in s.keep.iter().enumerate() {
        z[c as usize] = Some(zr[k]);
    }
    for (c, r) in s.elim.iter().rev() {
        let mut acc = 0u64;
        let mut ac = 0u64;
        let mut ok = true;
        for &(cc, a) in r {
            if cc == *c {
                ac = a;
                continue;
            }
            match z[cc as usize] {
                Some(v) => acc = acc_coef(acc, a, v, ell),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        // ac * z_c + acc = 0
        let neg = if acc == 0 { 0 } else { ell - acc };
        let v = if ac == 1 { neg } else if ac == ell - 1 { acc } else { lm2(neg, linv(ac, ell), ell) };
        z[*c as usize] = Some(v);
    }
    z
}

// ------------------------------------------------------------------ block Lanczos
/// n x k dense block, row-major.
#[derive(Clone)]
struct Blk {
    n: usize,
    k: usize,
    a: Vec<u64>,
}
impl Blk {
    fn zero(n: usize, k: usize) -> Blk {
        Blk { n, k, a: vec![0; n * k] }
    }
    fn col(&self, j: usize) -> Vec<u64> {
        (0..self.n).map(|i| self.a[i * self.k + j]).collect()
    }
    fn select(&self, cols: &[usize]) -> Blk {
        let mut b = Blk::zero(self.n, cols.len());
        for i in 0..self.n {
            for (t, &j) in cols.iter().enumerate() {
                b.a[i * cols.len() + t] = self.a[i * self.k + j];
            }
        }
        b
    }
}
/// X^T Y (k1 x k2)
fn gram(x: &Blk, y: &Blk, ell: u64) -> Vec<u64> {
    let mut g = vec![0u64; x.k * y.k];
    for i in 0..x.n {
        for p in 0..x.k {
            let xv = x.a[i * x.k + p];
            if xv == 0 {
                continue;
            }
            for r in 0..y.k {
                g[p * y.k + r] = la2(g[p * y.k + r], lm2(xv, y.a[i * y.k + r], ell), ell);
            }
        }
    }
    g
}
/// out -= X C  (X: n x k1, C: k1 x k2, out: n x k2)
fn sub_mul(out: &mut Blk, x: &Blk, c: &[u64], ell: u64) {
    for i in 0..x.n {
        for r in 0..out.k {
            let mut s = 0u64;
            for p in 0..x.k {
                s = la2(s, lm2(x.a[i * x.k + p], c[p * out.k + r], ell), ell);
            }
            out.a[i * out.k + r] = ls2(out.a[i * out.k + r], s, ell);
        }
    }
}
fn add_mul(out: &mut Blk, x: &Blk, c: &[u64], ell: u64) {
    for i in 0..x.n {
        for r in 0..out.k {
            let mut s = 0u64;
            for p in 0..x.k {
                s = la2(s, lm2(x.a[i * x.k + p], c[p * out.k + r], ell), ell);
            }
            out.a[i * out.k + r] = la2(out.a[i * out.k + r], s, ell);
        }
    }
}
/// small matrix product (k1 x k2) (k2 x k3)
fn smul(a: &[u64], b: &[u64], k1: usize, k2: usize, k3: usize, ell: u64) -> Vec<u64> {
    let mut c = vec![0u64; k1 * k3];
    for i in 0..k1 {
        for j in 0..k3 {
            let mut s = 0u64;
            for t in 0..k2 {
                s = la2(s, lm2(a[i * k2 + t], b[t * k3 + j], ell), ell);
            }
            c[i * k3 + j] = s;
        }
    }
    c
}
/// inverse of a k x k matrix (Gauss-Jordan), None if singular
fn sinv(m: &[u64], k: usize, ell: u64) -> Option<Vec<u64>> {
    let mut a = m.to_vec();
    let mut inv = vec![0u64; k * k];
    for i in 0..k {
        inv[i * k + i] = 1;
    }
    for col in 0..k {
        let piv = (col..k).find(|&r| a[r * k + col] != 0)?;
        if piv != col {
            for j in 0..k {
                a.swap(piv * k + j, col * k + j);
                inv.swap(piv * k + j, col * k + j);
            }
        }
        let pi = linv(a[col * k + col], ell);
        for j in 0..k {
            a[col * k + j] = lm2(a[col * k + j], pi, ell);
            inv[col * k + j] = lm2(inv[col * k + j], pi, ell);
        }
        for r in 0..k {
            if r != col && a[r * k + col] != 0 {
                let f = a[r * k + col];
                for j in 0..k {
                    a[r * k + j] = ls2(a[r * k + j], lm2(f, a[col * k + j], ell), ell);
                    inv[r * k + j] = ls2(inv[r * k + j], lm2(f, inv[col * k + j], ell), ell);
                }
            }
        }
    }
    Some(inv)
}
/// M V = B^T D B V for a block (rows with +-1 skip).
fn mblk(rows: &[Row], d: &[u64], v: &Blk, ell: u64) -> Blk {
    let k = v.k;
    let mut out = Blk::zero(v.n, k);
    let mut t = vec![0u64; k];
    for (ri, r) in rows.iter().enumerate() {
        for x in t.iter_mut() {
            *x = 0;
        }
        for &(c, a) in r {
            let base = c as usize * k;
            for j in 0..k {
                t[j] = acc_coef(t[j], a, v.a[base + j], ell);
            }
        }
        for j in 0..k {
            t[j] = lm2(t[j], d[ri], ell);
        }
        for &(c, a) in r {
            let base = c as usize * k;
            for j in 0..k {
                out.a[base + j] = acc_coef(out.a[base + j], a, t[j], ell);
            }
        }
    }
    out
}

pub struct BlStats {
    pub iters: u64,
    pub narrowed: u64,
}

/// One kernel vector of B (rows over n columns) by block Lanczos of width b on B^T D B.
pub fn block_lanczos(rows: &[Row], n: usize, ell: u64, b: usize, rng: &mut Rng, st: &mut BlStats) -> Option<Vec<u64>> {
    let d: Vec<u64> = (0..rows.len()).map(|_| 1 + rng.below(ell - 1)).collect();
    let mut y = Blk::zero(n, b);
    for x in y.a.iter_mut() {
        *x = rng.below(ell);
    }
    let v0 = mblk(rows, &d, &y, ell);
    let mut x = Blk::zero(n, b);
    let mut v = v0.clone();
    let mut vp: Option<(Blk, Blk, Vec<u64>)> = None; // (V_prev, MV_prev, Winv_prev)
    let mut iters = 0u64;
    loop {
        if v.a.iter().all(|&e| e == 0) || v.k == 0 {
            break;
        }
        let mut mv = mblk(rows, &d, &v, ell);
        let mut w = gram(&v, &mv, ell);
        let mut k = v.k;
        let winv = match sinv(&w, k, ell) {
            Some(wi) => wi,
            None => {
                // keep a maximal set of columns with an invertible principal submatrix
                let mut sel: Vec<usize> = Vec::new();
                for j in 0..k {
                    let mut s2 = sel.clone();
                    s2.push(j);
                    let sub: Vec<u64> = s2.iter().flat_map(|&p| s2.iter().map(move |&r| (p, r))).map(|(p, r)| w[p * k + r]).collect();
                    if sinv(&sub, s2.len(), ell).is_some() {
                        sel = s2;
                    }
                }
                st.narrowed += 1;
                if sel.is_empty() {
                    break;
                }
                v = v.select(&sel);
                mv = mv.select(&sel);
                let k2 = sel.len();
                let sub: Vec<u64> = sel.iter().flat_map(|&p| sel.iter().map(move |&r| (p, r))).map(|(p, r)| w[p * k + r]).collect();
                w = sub;
                k = k2;
                sinv(&w, k, ell)?
            }
        };
        // X += V Winv (V^T V0)
        let vt0 = gram(&v, &v0, ell); // k x b
        let c0 = smul(&winv, &vt0, k, k, b, ell);
        add_mul(&mut x, &v, &c0, ell);
        // V_next = MV - V Winv (MV^T MV) - V_prev Winv_prev (MV_prev^T MV)
        let mm = gram(&mv, &mv, ell);
        let c1 = smul(&winv, &mm, k, k, k, ell);
        let mut vn = mv.clone();
        sub_mul(&mut vn, &v, &c1, ell);
        if let Some((vpr, mvpr, wpi)) = &vp {
            let pm = gram(mvpr, &mv, ell); // kp x k
            let c2 = smul(wpi, &pm, vpr.k, vpr.k, k, ell);
            sub_mul(&mut vn, vpr, &c2, ell);
        }
        vp = Some((v, mv, winv));
        v = vn;
        iters += 1;
        if iters as usize > n + 10 {
            return None;
        }
    }
    st.iters += iters;
    // kernel candidates: columns of X - Y
    for j in 0..b {
        let xj = x.col(j);
        let yj = y.col(j);
        let z: Vec<u64> = (0..n).map(|i| ls2(xj[i], yj[i], ell)).collect();
        if z.iter().all(|&e| e == 0) {
            continue;
        }
        if rows.iter().all(|r| row_apply2(r, &z, ell) == 0) {
            return Some(z);
        }
    }
    None
}
