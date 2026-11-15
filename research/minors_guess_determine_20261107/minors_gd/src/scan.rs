//! Zero-minor scanners over C (papers 2 and 3), every F_p operation charged.
//!   all2 : zero entries, then all 2x2 minors (paper 2 Table 1)
//!   gesc : Gaussian-elimination Schur-complement chain, 2x2 minors of each
//!          complement for k' = 0..l/2 (paper 2 sec. 4.1, paper 3 sec. 4)
//!   apm2 : almost principal minors: contiguous 2x2 principal block at every
//!          position t, then two arbitrary row and two arbitrary column deviations
//!          (paper 3 sec. 5; 4x4 minors), via the Schur complement of the block.
//! The 2x2 scan uses slope hashing (a zero 2x2 minor <=> two rows with equal
//! slope A[i][j2]/A[i][j1]); this is the cheapest correct implementation, i.e.
//! the reading most favourable to the method. The papers' naive determinant
//! enumeration is recorded separately as `naive_minor_evals` (not charged).
use crate::field::*;

pub struct Found { pub alpha: Vec<usize>, pub beta: Vec<usize> }
pub enum ScanRes { Found(Found), None, OverBudget }

#[derive(Default, Debug, Clone)]
pub struct ScanStats { pub minors_tested: f64, pub naive_minor_evals: f64, pub hash_inserts: u64 }

struct Probe { keys: Vec<u64>, vals: Vec<u32>, stamp: Vec<u32>, cur: u32, mask: usize }
impl Probe {
    fn new(n: usize) -> Probe {
        let mut sz = 16; while sz < 2 * n { sz *= 2; }
        Probe { keys: vec![0; sz], vals: vec![0; sz], stamp: vec![0; sz], cur: 0, mask: sz - 1 }
    }
    fn clear(&mut self) { self.cur += 1; }
    /// returns Some(previous index) if key already present
    fn insert(&mut self, key: u64, val: u32) -> Option<u32> {
        let mut h = (key.wrapping_mul(0x9E3779B97F4A7C15) >> 20) as usize & self.mask;
        loop {
            if self.stamp[h] != self.cur { self.stamp[h] = self.cur; self.keys[h] = key; self.vals[h] = val; return None; }
            if self.keys[h] == key { return Some(self.vals[h]); }
            h = (h + 1) & self.mask;
        }
    }
}

/// Scan a rows x cols matrix `h` (row-major) for a zero 1x1 or 2x2 minor.
/// Returns (i, i2, j, j2) with i2 == usize::MAX meaning a 1x1 zero at (i, j).
pub fn scan2(f: &Fp, h: &[u64], rows: usize, cols: usize, st: &mut ScanStats, over: &dyn Fn() -> bool) -> Result<Option<(usize, usize, usize, usize)>, ()> {
    for i in 0..rows { for j in 0..cols { if h[i * cols + j] == 0 { return Ok(Some((i, usize::MAX, j, usize::MAX))); } } }
    st.minors_tested += (rows * cols) as f64;
    let mut inv: Vec<u64> = h.to_vec();
    f.batch_inv(&mut inv);
    let mut pr = Probe::new(rows);
    let pairs = (cols * (cols.saturating_sub(1)) / 2) as f64;
    st.minors_tested += pairs * (rows * (rows.saturating_sub(1)) / 2) as f64;
    st.naive_minor_evals += pairs * (rows * (rows.saturating_sub(1)) / 2) as f64;
    for j1 in 0..cols {
        if over() { return Err(()); }
        for j2 in j1 + 1..cols {
            pr.clear();
            for i in 0..rows {
                let s = f.mul(h[i * cols + j2], inv[i * cols + j1]);
                st.hash_inserts += 1;
                if let Some(i0) = pr.insert(s, i as u32) { return Ok(Some((i0 as usize, i, j1, j2))); }
            }
        }
    }
    Ok(None)
}

pub fn all2(f: &Fp, c: &[u64], l: usize, st: &mut ScanStats, over: &dyn Fn() -> bool) -> ScanRes {
    match scan2(f, c, l, l, st, over) {
        Err(()) => ScanRes::OverBudget,
        Ok(None) => ScanRes::None,
        Ok(Some((i, i2, j, j2))) => ScanRes::Found(mk(&[], i, i2, j, j2, 0)),
    }
}

fn mk(lead: &[usize], i: usize, i2: usize, j: usize, j2: usize, off: usize) -> Found {
    let mut alpha = lead.to_vec(); let mut beta = lead.to_vec();
    alpha.push(i + off); beta.push(j + off);
    if i2 != usize::MAX { alpha.push(i2 + off); beta.push(j2 + off); }
    Found { alpha, beta }
}

pub fn gesc(f: &Fp, c: &[u64], l: usize, st: &mut ScanStats, over: &dyn Fn() -> bool) -> ScanRes {
    let mut h = c.to_vec();
    let mut m = l; // current size
    let mut lead: Vec<usize> = vec![];
    for kp in 0..=l / 2 {
        match scan2(f, &h, m, m, st, over) {
            Err(()) => return ScanRes::OverBudget,
            Ok(Some((i, i2, j, j2))) => return ScanRes::Found(mk(&lead, i, i2, j, j2, kp)),
            Ok(None) => {}
        }
        if kp == l / 2 || m <= 3 { break; }
        // eliminate pivot (0,0) of h
        let piv = h[0];
        if piv == 0 { let mut a = lead.clone(); a.push(kp); return ScanRes::Found(Found { alpha: a.clone(), beta: a }); }
        let ip = f.inv(piv);
        let mut nh = vec![0u64; (m - 1) * (m - 1)];
        for r in 1..m {
            let fac = f.mul(h[r * m], ip);
            for cc in 1..m { let t = f.mul(fac, h[cc]); nh[(r - 1) * (m - 1) + cc - 1] = f.sub(h[r * m + cc], t); }
        }
        h = nh; m -= 1; lead.push(kp);
    }
    ScanRes::None
}

pub fn apm2(f: &Fp, c: &[u64], l: usize, st: &mut ScanStats, over: &dyn Fn() -> bool) -> ScanRes {
    let m = l - 2;
    let mut h = vec![0u64; m * m];
    for t in 0..l - 1 {
        if over() { return ScanRes::OverBudget; }
        let (a, b, cc, d) = (c[t * l + t], c[t * l + t + 1], c[(t + 1) * l + t], c[(t + 1) * l + t + 1]);
        let det = f.sub(f.mul(a, d), f.mul(b, cc));
        if det == 0 { return ScanRes::Found(Found { alpha: vec![t, t + 1], beta: vec![t, t + 1] }); }
        let id = f.inv(det);
        // Einv = id * [[d, -b], [-c, a]]
        let e00 = f.mul(id, d); let e01 = f.mul(id, f.neg(b)); let e10 = f.mul(id, f.neg(cc)); let e11 = f.mul(id, a);
        let cols: Vec<usize> = (0..l).filter(|&s| s != t && s != t + 1).collect();
        let mut w0 = vec![0u64; m]; let mut w1 = vec![0u64; m];
        for (k, &s) in cols.iter().enumerate() {
            let (u, v) = (c[t * l + s], c[(t + 1) * l + s]);
            w0[k] = f.add(f.mul(e00, u), f.mul(e01, v));
            w1[k] = f.add(f.mul(e10, u), f.mul(e11, v));
        }
        for (ri, r) in (0..l).filter(|&r| r != t && r != t + 1).enumerate() {
            let (p0, p1) = (c[r * l + t], c[r * l + t + 1]);
            for (k, &s) in cols.iter().enumerate() {
                let x = f.add(f.mul(p0, w0[k]), f.mul(p1, w1[k]));
                h[ri * m + k] = f.sub(c[r * l + s], x);
            }
        }
        match scan2(f, &h, m, m, st, over) {
            Err(()) => return ScanRes::OverBudget,
            Ok(None) => {}
            Ok(Some((i, i2, j, j2))) => {
                let rmap: Vec<usize> = (0..l).filter(|&r| r != t && r != t + 1).collect();
                let mut alpha = vec![t, t + 1]; let mut beta = vec![t, t + 1];
                alpha.push(rmap[i]); beta.push(rmap[j]);
                if i2 != usize::MAX { alpha.push(rmap[i2]); beta.push(rmap[j2]); }
                return ScanRes::Found(Found { alpha, beta });
            }
        }
    }
    ScanRes::None
}

/// Control: count (not stop at) all zero 1x1 and 2x2 minors of an l x l matrix.
pub fn count_zero_minors(f: &Fp, h: &[u64], l: usize) -> (u64, u64) {
    let mut z1 = 0u64; for &v in h { if v == 0 { z1 += 1; } }
    if z1 > 0 { return (z1, 0); }
    let mut inv = h.to_vec(); f.batch_inv(&mut inv);
    let mut z2 = 0u64;
    let mut pr = Probe::new(l);
    for j1 in 0..l { for j2 in j1 + 1..l {
        // count pairs with equal slope: group by slope
        let mut v: Vec<u64> = (0..l).map(|i| f.mul_nc(h[i * l + j2], inv[i * l + j1])).collect();
        v.sort_unstable();
        let mut k = 0; while k < v.len() { let mut e = k; while e < v.len() && v[e] == v[k] { e += 1; } let r = (e - k) as u64; z2 += r * (r - 1) / 2; k = e; }
    } }
    let _ = &mut pr;
    (z1, z2)
}
