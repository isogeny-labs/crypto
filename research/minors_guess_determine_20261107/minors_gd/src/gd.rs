//! Guess-and-determine (arXiv:2607.09814, Mahalanobis 2026), prime-field reading.
//!  guess:     a random l' = l/2 subset `a` of the dense columns (C[:,a] is l x l');
//!             its transpose is reduced to anti-diagonal/identity form, K' = [A' | I]  (l' x l)
//!  determine: for each b, a random (l'-d)-subset of the dense columns of K':
//!             T = left kernel of K'[:,b] (dim d); the signature of every other column
//!             c is s_c = T*c in F^d (sparse columns: free, a column of T);
//!             for every (d-1)-subset x of the l'+d signature rows the right kernel
//!             w_x (normalised) is hashed; two equal lines give d dependent
//!             signature rows = a zero l' x l' minor of K' = a zero l' x l' minor of C.
//! Every F_p multiplication/inversion is charged. The enumeration of the
//! (d-1)-subsets shares echelon prefixes depth-first (the 'rank-one update'
//! idea the paper suggests), with sibling-batched inversions.
use crate::field::*;
use crate::kernel::Kernel;
use crate::rng::*;
use crate::scan::*;

pub fn lnbinom(n: usize, k: usize) -> f64 { let mut s = 0.0; for i in 0..k { s += ((n - i) as f64).ln() - ((i + 1) as f64).ln(); } s }

/// Paper Eq. (4): smallest defect d >= 2 with success probability >= target for one `a`.
pub fn choose_d(l: usize, n: u64, target: f64) -> usize {
    let lp = l / 2;
    for d in 2..=lp {
        let nd = d as f64;
        let per_b = (1.0 - (n as f64 - 2.0 * nd + 1.0) / (n as f64 * (n as f64 - nd + 1.0))).ln();
        let ln_lambda_total = lnbinom(lp + d, d).exp() * lnbinom(lp, d).exp() * per_b;
        let p = 1.0 - ln_lambda_total.exp();
        if p >= target { return d; }
    }
    lp
}

/// Direct right-kernel line of a (d-1) x d matrix, w[d-1] = 1 (counted). None if a pivot vanishes.
fn line_of(f: &Fp, rows: &[Vec<u64>], d: usize) -> Option<Vec<u64>> {
    let mut m: Vec<Vec<u64>> = rows.to_vec();
    for k in 0..d - 1 {
        if m[k][k] == 0 { return None; }
        let iv = f.inv(m[k][k]);
        for j in k..d { m[k][j] = f.mul(m[k][j], iv); }
        for r in 0..d - 1 { if r != k { let fac = m[r][k]; if fac != 0 { for j in k..d { let t = f.mul(fac, m[k][j]); m[r][j] = f.sub(m[r][j], t); } } } }
    }
    // reduced: identity in first d-1 columns; w_k = -m[k][d-1]
    let mut w = vec![0u64; d]; w[d - 1] = 1;
    for k in 0..d - 1 { w[k] = f.neg(m[k][d - 1]); }
    Some(w)
}

struct Dfs<'a> {
    f: &'a Fp, d: usize, ll: usize, sig: &'a [u64],
    piv: Vec<Vec<u64>>, chosen: Vec<usize>,
    probe: Probe2, subs: Vec<u8>, leaves: u64,
    over: &'a dyn Fn() -> bool, aborted: bool,
}

pub struct Probe2 { keys: Vec<u64>, vals: Vec<u32>, used: Vec<bool>, mask: usize }
impl Probe2 {
    pub fn new(n: usize) -> Probe2 { let mut sz = 16; while sz < 2 * n { sz *= 2; } Probe2 { keys: vec![0; sz], vals: vec![0; sz], used: vec![false; sz], mask: sz - 1 } }
    pub fn insert(&mut self, key: u64, val: u32) -> Option<u32> {
        let mut h = (key.wrapping_mul(0x9E3779B97F4A7C15) >> 24) as usize & self.mask;
        loop {
            if !self.used[h] { self.used[h] = true; self.keys[h] = key; self.vals[h] = val; return None; }
            if self.keys[h] == key { return Some(self.vals[h]); }
            h = (h + 1) & self.mask;
        }
    }
}

impl<'a> Dfs<'a> {
    fn row(&self, r: usize) -> &[u64] { &self.sig[r * self.d..(r + 1) * self.d] }
    fn reduce(&self, r: usize, depth: usize) -> Vec<u64> {
        let f = self.f; let d = self.d;
        let mut v = self.row(r).to_vec();
        for k in 0..depth {
            let fac = v[k];
            if fac != 0 { for j in k + 1..d { let t = f.mul(fac, self.piv[k][j]); v[j] = f.sub(v[j], t); } }
            v[k] = 0;
        }
        v
    }
    /// returns Some((idx_a, idx_b-subset)) on a verified duplicate
    fn rec(&mut self, depth: usize, start: usize) -> Option<(Vec<usize>, Vec<usize>)> {
        let d = self.d; let f = self.f;
        let last = d - 2;
        let rem_after = d - 2 - depth; // slots left after this one
        let hi = self.ll - rem_after;  // exclusive
        if start >= hi { return None; }
        if depth == 0 && (self.over)() { self.aborted = true; return None; }
        let mut vs: Vec<(usize, Vec<u64>)> = Vec::with_capacity(hi - start);
        for r in start..hi {
            let v = self.reduce(r, depth);
            if v[depth] != 0 { vs.push((r, v)); }
        }
        if vs.is_empty() { return None; }
        let mut dens: Vec<u64> = vs.iter().map(|(_, v)| v[depth]).collect();
        f.batch_inv(&mut dens);
        if depth < last {
            for (ix, (r, v)) in vs.into_iter().enumerate() {
                let mut pr = v;
                for j in depth..d { pr[j] = f.mul(pr[j], dens[ix]); }
                self.piv.truncate(depth); self.piv.push(pr);
                self.chosen.truncate(depth); self.chosen.push(r);
                if let Some(x) = self.rec(depth + 1, r + 1) { return Some(x); }
                if self.aborted { return None; }
            }
            None
        } else {
            for (ix, (r, v)) in vs.iter().enumerate() {
                let mut w = vec![0u64; d];
                w[d - 1] = 1;
                w[d - 2] = f.neg(f.mul(v[d - 1], dens[ix]));
                for k in (0..d - 2).rev() {
                    let mut s = 0u64;
                    for j in k + 1..d { s = f.add(s, f.mul(self.piv[k][j], w[j])); }
                    w[k] = f.neg(s);
                }
                let mut h = 0xcbf29ce484222325u64;
                for k in 0..d - 1 { h = (h.rotate_left(13) ^ w[k]).wrapping_mul(0x100000001b3).wrapping_add(0x9E3779B97F4A7C15); }
                self.leaves += 1;
                let id = (self.subs.len() / (d - 1)) as u32;
                for k in 0..depth { self.subs.push(self.chosen[k] as u8); }
                self.subs.push(*r as u8);
                if let Some(prev) = self.probe.insert(h, id) {
                    // verify exactly
                    let sa: Vec<usize> = self.subs[prev as usize * (d - 1)..(prev as usize + 1) * (d - 1)].iter().map(|&t| t as usize).collect();
                    let mut sb: Vec<usize> = self.chosen[..depth].to_vec(); sb.push(*r);
                    let ra: Vec<Vec<u64>> = sa.iter().map(|&t| self.row(t).to_vec()).collect();
                    let rb: Vec<Vec<u64>> = sb.iter().map(|&t| self.row(t).to_vec()).collect();
                    if let (Some(wa), Some(wb)) = (line_of(f, &ra, d), line_of(f, &rb, d)) {
                        if wa == wb { return Some((sa, sb)); }
                    }
                }
                if self.leaves & 0xFFFF == 0 && (self.over)() { self.aborted = true; return None; }
            }
            None
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct GdStats { pub b_tried: u64, pub leaves: u64, pub kernels: u64 }

pub fn attempt(f: &Fp, k: &Kernel, d: usize, rng: &mut Rng, st: &mut ScanStats, gs: &mut GdStats, over: &dyn Fn() -> bool) -> ScanRes {
    let l = k.l; assert!(l % 2 == 0);
    let lp = l / 2;
    assert!(d >= 2 && d < lp);
    let a = rng.subset(l, lp);
    let r2 = rng.subset(l, lp);
    let mut in2 = vec![false; l]; for &r in &r2 { in2[r] = true; }
    let r1: Vec<usize> = (0..l).filter(|&r| !in2[r]).collect();
    // [X2 | X1] with X[c][r] = C[r][a_c]; Gauss-Jordan to [I | A']
    let w = 2 * lp;
    let mut g = vec![0u64; lp * w];
    for c in 0..lp { for u in 0..lp { g[c * w + u] = k.at(r2[u], a[c]); g[c * w + lp + u] = k.at(r1[u], a[c]); } }
    for kk in 0..lp {
        let mut pr = kk; while pr < lp && g[pr * w + kk] == 0 { pr += 1; }
        if pr == lp { return ScanRes::None; }
        if pr != kk { for cc in 0..w { g.swap(pr * w + cc, kk * w + cc); } }
        let iv = f.inv(g[kk * w + kk]);
        for cc in kk..w { g[kk * w + cc] = f.mul(g[kk * w + cc], iv); }
        for r in 0..lp { if r != kk { let fac = g[r * w + kk]; if fac != 0 { for cc in kk + 1..w { let t = f.mul(fac, g[kk * w + cc]); g[r * w + cc] = f.sub(g[r * w + cc], t); } g[r * w + kk] = 0; } } }
    }
    let ap: Vec<u64> = (0..lp * lp).map(|i| g[(i / lp) * w + lp + i % lp]).collect(); // A' (lp x lp)
    let nb_total = lnbinom(lp, d).exp().ceil() as u64;
    let ll = lp + d;
    let leaves_per_b = lnbinom(ll, d - 1).exp().ceil() as usize;
    for _ in 0..nb_total {
        if over() { return ScanRes::OverBudget; }
        gs.b_tried += 1;
        // b = (lp-d)-subset of dense columns: choose the d excluded columns
        let excl = rng.subset(lp, d);
        let mut exl = vec![false; lp]; for &e in &excl { exl[e] = true; }
        let b: Vec<usize> = (0..lp).filter(|&c| !exl[c]).collect();
        // T = left kernel of M = A'[:, b]  (elimination on [M | I])
        let mw = lp - d + lp;
        let mut aug = vec![0u64; lp * mw];
        for r in 0..lp { for (ci, &c) in b.iter().enumerate() { aug[r * mw + ci] = ap[r * lp + c]; } aug[r * mw + (lp - d) + r] = 1; }
        let mut ok = true;
        for kk in 0..lp - d {
            let mut pr = kk; while pr < lp && aug[pr * mw + kk] == 0 { pr += 1; }
            if pr == lp { ok = false; break; }
            if pr != kk { for cc in 0..mw { aug.swap(pr * mw + cc, kk * mw + cc); } }
            let iv = f.inv(aug[kk * mw + kk]);
            for r in kk + 1..lp {
                let fac = f.mul(aug[r * mw + kk], iv);
                if fac == 0 { continue; }
                for cc in kk + 1..mw { let t = f.mul(fac, aug[kk * mw + cc]); aug[r * mw + cc] = f.sub(aug[r * mw + cc], t); }
                aug[r * mw + kk] = 0;
            }
        }
        if !ok { continue; }
        gs.kernels += 1;
        let mut tm = vec![0u64; d * lp]; // T: d x lp
        for t in 0..d { for c in 0..lp { tm[t * lp + c] = aug[(lp - d + t) * mw + (lp - d) + c]; } }
        // signatures: rows 0..d-1 dense columns excl[..], rows d.. sparse 0..lp
        let mut sig = vec![0u64; ll * d];
        for (ti, &c) in excl.iter().enumerate() {
            for t in 0..d { let mut s = 0u64; for r in 0..lp { s = f.add(s, f.mul(tm[t * lp + r], ap[r * lp + c])); } sig[ti * d + t] = s; }
        }
        for u in 0..lp { for t in 0..d { sig[(d + u) * d + t] = tm[t * lp + u]; } }
        let mut dfs = Dfs { f, d, ll, sig: &sig, piv: vec![], chosen: vec![], probe: Probe2::new(leaves_per_b), subs: Vec::with_capacity(leaves_per_b * (d - 1)), leaves: 0, over, aborted: false };
        let res = dfs.rec(0, 0);
        gs.leaves += dfs.leaves;
        st.minors_tested += lnbinom(ll, d).exp();
        st.naive_minor_evals += lnbinom(ll, d).exp();
        st.hash_inserts += dfs.leaves;
        if dfs.aborted { return ScanRes::OverBudget; }
        if let Some((sa, sb)) = res {
            let mut un: Vec<usize> = sa.iter().chain(sb.iter()).cloned().collect();
            un.sort(); un.dedup();
            let dset: Vec<usize> = un.into_iter().take(d).collect();
            let mut rows: Vec<usize> = b.iter().map(|&c| r1[c]).collect();
            for &t in &dset { if t < d { rows.push(r1[excl[t]]); } else { rows.push(r2[t - d]); } }
            rows.sort(); rows.dedup();
            if rows.len() != lp { continue; }
            return ScanRes::Found(Found { alpha: rows, beta: a.clone() });
        }
    }
    ScanRes::None
}
