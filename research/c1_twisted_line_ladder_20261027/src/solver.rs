//! The descended Gaudry system and a counted weighted Macaulay solver.
//!
//! x_i = u y_i, y_i in F_Q (u = 1: standard base). S_5(x1..x4, x_R) = 0 is
//! symmetric in x1..x4 and (char 2, measured in semaev.rs) a polynomial in
//! e1^2, e2^2, e3^2, e4 of the x_i. With E_j = e_j(y)^2 (j <= 3), E_4 = e_4(y):
//!   e_j(x)^2 = u^{2j} E_j,  e_4(x) = u^4 E_4.
//! Each S_5 term c e1^i1 e2^i2 e3^i3 e4^i4 x_R^j b^l becomes
//!   c u^{i1+2 i2+3 i3+4 i4} x_R^j b^l  E1^{i1/2} E2^{i2/2} E3^{i3/2} E4^{i4},
//! with coefficient in F_{Q^4}; its four coordinates on 1,g,g^2,g^3 give four
//! equations over F_Q in (E1,E2,E3,E4), weights (2,2,2,1), degree <= 8.
use crate::gf::*;
use crate::semaev::ETerm;
use std::collections::HashMap;

pub const W: [u8; 4] = [2, 2, 2, 1];
pub type Mono4 = [u8; 4];

pub fn wdeg(m: &Mono4) -> u32 {
    (0..4).map(|i| m[i] as u32 * W[i] as u32).sum()
}
fn cmp_mono(a: &Mono4, b: &Mono4) -> std::cmp::Ordering {
    let (da, db) = (wdeg(a), wdeg(b));
    if da != db {
        return da.cmp(&db);
    }
    for i in (0..4).rev() {
        if a[i] != b[i] {
            return b[i].cmp(&a[i]);
        }
    }
    std::cmp::Ordering::Equal
}
pub fn monomials_desc(d: u32) -> Vec<Mono4> {
    let mut out = vec![];
    for a in 0..=d / 2 {
        for b in 0..=(d - 2 * a) / 2 {
            for c in 0..=(d - 2 * a - 2 * b) / 2 {
                for e in 0..=(d - 2 * a - 2 * b - 2 * c) {
                    out.push([a as u8, b as u8, c as u8, e as u8]);
                }
            }
        }
    }
    out.sort_by(|x, y| cmp_mono(y, x));
    out
}

pub struct System {
    /// four F_Q-polynomials in E1..E4
    pub polys: Vec<Vec<(Mono4, u16)>>,
    pub setup_f4_muls: u64,
}

/// Build the four equations. `b` is the curve constant in F_{Q^4}.
pub fn build_system(f: &Fq4, eterms: &[ETerm], b: &E4, u: &E4, xr: &E4) -> System {
    let mut cnt = 0u64;
    let mut upow = vec![ONE];
    for w in 1..=32 {
        let p = f.mul(&upow[w - 1], u);
        cnt += 1;
        upow.push(p);
    }
    let mut xp = vec![ONE];
    let mut bp = vec![ONE];
    for j in 1..=8 {
        let p = f.mul(&xp[j - 1], xr);
        let q = f.mul(&bp[j - 1], b);
        cnt += 2;
        xp.push(p);
        bp.push(q);
    }
    let mut coef: HashMap<Mono4, E4> = HashMap::new();
    for t in eterms {
        assert!(t.i[0] % 2 == 0 && t.i[1] % 2 == 0 && t.i[2] % 2 == 0);
        let w = t.i[0] as usize + 2 * t.i[1] as usize + 3 * t.i[2] as usize + 4 * t.i[3] as usize;
        let c = f.mul(&f.mul(&upow[w], &xp[t.j as usize]), &bp[t.l as usize]);
        cnt += 2;
        let key = [t.i[0] / 2, t.i[1] / 2, t.i[2] / 2, t.i[3]];
        let e = coef.entry(key).or_insert(ZERO);
        *e = add(e, &c);
    }
    let mut polys: Vec<Vec<(Mono4, u16)>> = vec![vec![]; 4];
    let mut keys: Vec<Mono4> = coef.keys().cloned().collect();
    keys.sort_by(|a, b| cmp_mono(b, a));
    for m in keys {
        let c = coef[&m];
        for r in 0..4 {
            if c[r] != 0 {
                polys[r].push((m, c[r]));
            }
        }
    }
    System { polys, setup_f4_muls: cnt }
}

/// Evaluate polynomial at (E1..E4) in F_Q.
pub fn eval(f: &Fq, p: &[(Mono4, u16)], e: &[u16; 4]) -> u16 {
    let mut s = 0u16;
    for (m, c) in p {
        let mut v = *c;
        for i in 0..4 {
            if m[i] > 0 {
                v = f.mul(v, f.pow(e[i], m[i] as u64));
            }
        }
        s ^= v;
    }
    s
}

/// Elementary symmetric functions of y1..y4 in F_Q, squared for the first three.
pub fn e_coords(f: &Fq, y: &[u16; 4]) -> [u16; 4] {
    let mut e = [0u16; 5];
    e[0] = 1;
    for &yi in y {
        for j in (1..5).rev() {
            e[j] ^= f.mul(e[j - 1], yi);
        }
    }
    [f.sq(e[1]), f.sq(e[2]), f.sq(e[3]), e[4]]
}

#[derive(Clone, Debug, Default)]
pub struct MacRes {
    pub d: u32,
    pub rows: usize,
    pub cols: usize,
    pub rank: usize,
    pub hf: usize,
    pub mults: u64,
    pub invs: u64,
    pub std_max_deg: u32,
    pub std_by_deg: Vec<usize>,
    pub max_row_nnz_after: usize,
}

/// Weighted Macaulay matrix at degree d, forward elimination by leading
/// column with sparsest pivots. Counted: every F_Q multiplication of two
/// nonzero entries (row ops and pivot normalisation) and every inversion.
pub fn macaulay(f: &Fq, polys: &[Vec<(Mono4, u16)>], d: u32, threads: usize) -> (MacRes, Vec<u32>, Vec<Vec<u16>>, Vec<Mono4>) {
    let cols = monomials_desc(d);
    let ncols = cols.len();
    let index: HashMap<Mono4, usize> = cols.iter().cloned().enumerate().map(|(i, m)| (m, i)).collect();
    let mut rows: Vec<Vec<u16>> = vec![];
    for p in polys {
        let dp = p.iter().map(|(m, _)| wdeg(m)).max().unwrap_or(0);
        if dp > d {
            continue;
        }
        for sh in monomials_desc(d - dp) {
            let mut row = vec![0u16; ncols];
            for (m, c) in p {
                let mm = [m[0] + sh[0], m[1] + sh[1], m[2] + sh[2], m[3] + sh[3]];
                row[index[&mm]] ^= *c;
            }
            rows.push(row);
        }
    }
    let nrows = rows.len();
    let mut mults = 0u64;
    let mut invs = 0u64;
    let mut buckets: Vec<Vec<u32>> = vec![vec![]; ncols];
    let mut nnz: Vec<u32> = rows.iter().map(|r| r.iter().filter(|&&x| x != 0).count() as u32).collect();
    for (r, row) in rows.iter().enumerate() {
        if let Some(c) = row.iter().position(|&x| x != 0) {
            buckets[c].push(r as u32);
        }
    }
    let mut pivot_row = vec![u32::MAX; ncols];
    let mut max_nnz = 0usize;
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        let best = (0..b.len()).min_by_key(|&i| (nnz[b[i] as usize], b[i])).unwrap();
        b.swap(0, best);
        let pr = b[0] as usize;
        let mut piv = std::mem::take(&mut rows[pr]);
        let inv = f.inv(piv[c]);
        invs += 1;
        let mut nzj: Vec<(u32, u16)> = vec![]; // (col, log of normalised entry), cols > c
        for j in c..ncols {
            if piv[j] != 0 {
                piv[j] = f.mul(piv[j], inv);
                mults += 1;
            }
        }
        for j in c + 1..ncols {
            if piv[j] != 0 {
                nzj.push((j as u32, f.log[piv[j] as usize]));
            }
        }
        // the leading-column entry itself is cleared exactly, not multiplied
        let mut others: Vec<(usize, Vec<u16>)> = b[1..].iter().map(|&r| (r as usize, std::mem::take(&mut rows[r as usize]))).collect();
        mults += (others.len() * nzj.len()) as u64;
        let work = |chunk: &mut [(usize, Vec<u16>)]| -> Vec<(usize, Option<usize>, u32)> {
            let mut out = Vec::with_capacity(chunk.len());
            for (r, row) in chunk.iter_mut() {
                let fc = row[c];
                let lf = f.log[fc as usize] as usize;
                row[c] = 0;
                for &(j, lg) in &nzj {
                    row[j as usize] ^= f.exp[lf + lg as usize];
                }
                let ld = row[c + 1..].iter().position(|&x| x != 0).map(|k| k + c + 1);
                let cnt = match ld {
                    Some(l) => row[l..].iter().filter(|&&x| x != 0).count() as u32,
                    None => 0,
                };
                out.push((*r, ld, cnt));
            }
            out
        };
        let results: Vec<(usize, Option<usize>, u32)> = if threads > 1 && others.len() * nzj.len() > 20000 {
            let chunk = others.len().div_ceil(threads);
            let mut res = vec![];
            std::thread::scope(|s| {
                let hs: Vec<_> = others.chunks_mut(chunk).map(|ch| s.spawn(|| work(ch))).collect();
                for h in hs {
                    res.extend(h.join().unwrap());
                }
            });
            res
        } else {
            work(&mut others)
        };
        for ((r, row), (_, ld, cnt)) in others.into_iter().zip(results) {
            rows[r] = row;
            nnz[r] = cnt;
            max_nnz = max_nnz.max(cnt as usize);
            if let Some(l) = ld {
                buckets[l].push(r as u32);
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    let rank = pivot_row.iter().filter(|&&p| p != u32::MAX).count();
    let mut std_by_deg = vec![0usize; d as usize + 1];
    let mut std_max = 0;
    for c in 0..ncols {
        if pivot_row[c] == u32::MAX {
            let dg = wdeg(&cols[c]);
            std_by_deg[dg as usize] += 1;
            std_max = std_max.max(dg);
        }
    }
    let res = MacRes { d, rows: nrows, cols: ncols, rank, hf: ncols - rank, mults, invs, std_max_deg: std_max, std_by_deg, max_row_nnz_after: max_nnz };
    (res, pivot_row, rows, cols)
}
