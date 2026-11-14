//! Root extraction: reduce short rows mod an irreducible q of degree D (> degree bounds),
//! resultant in X2 by evaluation/interpolation over F_{2^D}, roots by gcd with X^{2^D}-X
//! and trace splitting; each candidate is lifted back to F_2[t] (unique, deg < D) and
//! verified against the arm polynomial mod f by the caller.
use crate::gf2x::{Rng, F64, P};
use crate::lattice::Bi;

pub type U = Vec<u64>; // univariate over F64, low -> high

fn trim(a: &mut U) { while a.last() == Some(&0) { a.pop(); } }
fn udeg(a: &U) -> i64 { a.len() as i64 - 1 }

pub fn urem(k: &F64, a: &U, m: &U) -> U {
    let mut r = a.clone();
    trim(&mut r);
    let dm = udeg(m);
    assert!(dm >= 0);
    let il = k.inv(m[dm as usize]);
    while udeg(&r) >= dm {
        let dr = udeg(&r) as usize;
        let c = k.mul(r[dr], il);
        let sh = dr - dm as usize;
        for i in 0..=dm as usize { r[sh + i] ^= k.mul(c, m[i]); }
        trim(&mut r);
    }
    r
}
pub fn umul(k: &F64, a: &U, b: &U) -> U {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() { if x == 0 { continue; } for (j, &y) in b.iter().enumerate() { r[i + j] ^= k.mul(x, y); } }
    trim(&mut r);
    r
}
pub fn ugcd(k: &F64, a: &U, b: &U) -> U {
    let (mut x, mut y) = (a.clone(), b.clone());
    trim(&mut x); trim(&mut y);
    while !y.is_empty() { let r = urem(k, &x, &y); x = y; y = r; }
    if !x.is_empty() { let il = k.inv(*x.last().unwrap()); x = x.iter().map(|&c| k.mul(c, il)).collect(); }
    x
}
fn uadd(a: &U, b: &U) -> U {
    let mut r = vec![0u64; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() { r[i] ^= x; }
    for (i, &x) in b.iter().enumerate() { r[i] ^= x; }
    trim(&mut r);
    r
}

/// distinct roots in F_{2^D} of a nonzero polynomial
pub fn roots(k: &F64, a: &U, rng: &mut Rng) -> Vec<u64> {
    let mut a = a.clone();
    trim(&mut a);
    if udeg(&a) <= 0 { return vec![]; }
    // X^{2^D} mod a
    let mut x: U = urem(k, &vec![0, 1], &a);
    for _ in 0..k.d { x = urem(k, &umul(k, &x, &x), &a); }
    let g = ugcd(k, &a, &uadd(&x, &vec![0, 1]));
    let mut out = vec![];
    split(k, &g, rng, &mut out);
    out.sort();
    out
}
fn split(k: &F64, g: &U, rng: &mut Rng, out: &mut Vec<u64>) {
    let d = udeg(g);
    if d <= 0 { return; }
    if d == 1 { out.push(k.mul(g[0], k.inv(g[1]))); return; }
    for _ in 0..200 {
        let beta = rng.next() & ((1u64 << k.d) - 1);
        if beta == 0 { continue; }
        let mut t: U = urem(k, &vec![0, beta], g);
        let mut acc = t.clone();
        for _ in 1..k.d { t = urem(k, &umul(k, &t, &t), g); acc = uadd(&acc, &t); }
        let h = ugcd(k, g, &acc);
        if udeg(&h) > 0 && udeg(&h) < d {
            let mut q = g.clone();
            // g / h by long division
            let mut quo = vec![0u64; (d - udeg(&h) + 1) as usize];
            let il = k.inv(*h.last().unwrap());
            while udeg(&q) >= udeg(&h) {
                let dq = udeg(&q) as usize; let dh = udeg(&h) as usize;
                let c = k.mul(q[dq], il);
                quo[dq - dh] = c;
                for i in 0..=dh { q[dq - dh + i] ^= k.mul(c, h[i]); }
                trim(&mut q);
            }
            trim(&mut quo);
            split(k, &h, rng, out);
            split(k, &quo, rng, out);
            return;
        }
    }
    panic!("split failed");
}

pub fn det(k: &F64, mut m: Vec<Vec<u64>>) -> u64 {
    let n = m.len();
    let mut d = 1u64;
    for c in 0..n {
        let piv = (c..n).find(|&r| m[r][c] != 0);
        let p = match piv { None => return 0, Some(p) => p };
        m.swap(c, p);
        d = k.mul(d, m[c][c]);
        let ip = k.inv(m[c][c]);
        for r in c + 1..n {
            if m[r][c] == 0 { continue; }
            let f = k.mul(m[r][c], ip);
            for j in c..n { let v = k.mul(f, m[c][j]); m[r][j] ^= v; }
        }
    }
    d
}

/// bivariate over F64 as dense [a][b]
pub struct BiF { pub c: Vec<Vec<u64>>, pub da: usize, pub db: usize }
pub fn to_bif(k: &F64, b: &Bi) -> BiF {
    let qp = P::from_u64(k.q);
    let da = b.keys().map(|m| m.0).max().unwrap_or(0);
    let db = b.keys().map(|m| m.1).max().unwrap_or(0);
    let mut c = vec![vec![0u64; db + 1]; da + 1];
    for (m, x) in b { c[m.0][m.1] = x.rem(&qp).to_u64(); }
    BiF { c, da, db }
}
/// evaluate X1 = v -> univariate in X2 of formal length db+1
pub fn eval_x1(k: &F64, h: &BiF, v: u64) -> Vec<u64> {
    let mut out = vec![0u64; h.db + 1];
    for b in 0..=h.db {
        let mut acc = 0u64;
        for a in (0..=h.da).rev() { acc = k.mul(acc, v) ^ h.c[a][b]; }
        out[b] = acc;
    }
    out
}
pub fn eval_x2(k: &F64, h: &BiF, v: u64) -> U {
    let mut out = vec![0u64; h.da + 1];
    for a in 0..=h.da {
        let mut acc = 0u64;
        for b in (0..=h.db).rev() { acc = k.mul(acc, v) ^ h.c[a][b]; }
        out[a] = acc;
    }
    trim(&mut out);
    out
}
fn sylvester(k: &F64, a: &[u64], b: &[u64]) -> u64 {
    let m = a.len() - 1; let n = b.len() - 1;
    let sz = m + n;
    if sz == 0 { return 1; }
    let mut mat = vec![vec![0u64; sz]; sz];
    for i in 0..n { for j in 0..=m { mat[i][i + j] = a[m - j]; } }
    for i in 0..m { for j in 0..=n { mat[n + i][i + j] = b[n - j]; } }
    det(k, mat)
}
/// Res_{X2}(h1, h2) as polynomial in X1 (formal degrees), by evaluation/interpolation.
pub fn resultant_x2(k: &F64, h1: &BiF, h2: &BiF) -> U {
    let deg_bound = h1.da * h2.db + h2.da * h1.db;
    let npts = deg_bound + 1;
    assert!((npts as u128) < (1u128 << k.d));
    let xs: Vec<u64> = (0..npts as u64).map(|i| i + 1).collect();
    let ys: Vec<u64> = xs.iter().map(|&v| sylvester(k, &eval_x1(k, h1, v), &eval_x1(k, h2, v))).collect();
    // Newton divided differences (char 2: subtraction = xor)
    let mut coef = ys.clone();
    for j in 1..npts {
        for i in (j..npts).rev() {
            let num = coef[i] ^ coef[i - 1];
            let den = xs[i] ^ xs[i - j];
            coef[i] = k.mul(num, k.inv(den));
        }
    }
    let mut poly: U = vec![coef[npts - 1]];
    for i in (0..npts - 1).rev() {
        poly = umul(k, &poly, &vec![xs[i], 1]);
        if poly.is_empty() { poly = vec![0]; }
        poly[0] ^= coef[i];
    }
    trim(&mut poly);
    poly
}

pub struct ExtractOut {
    pub pairs_tried: u32,
    pub res_zero: u32,
    pub candidates: Vec<(P, P)>, // F_2[t] pairs with deg bounds satisfied (not yet verified mod f)
    pub resolved: bool,
}

/// h: short rows as bivariate over F_2[t]; bounds: max degree of var1, var2
pub fn extract(k: &F64, h: &[Bi], b1: usize, b2: usize, rng: &mut Rng, max_rows: usize) -> ExtractOut {
    let hs: Vec<BiF> = h.iter().take(max_rows).map(|b| to_bif(k, b)).collect();
    let mut out = ExtractOut { pairs_tried: 0, res_zero: 0, candidates: vec![], resolved: false };
    for i in 0..hs.len() {
        for j in i + 1..hs.len() {
            out.pairs_tried += 1;
            let r = resultant_x2(k, &hs[i], &hs[j]);
            if r.is_empty() { out.res_zero += 1; continue; }
            out.resolved = true;
            for r1 in roots(k, &r, rng) {
                if 64 - r1.leading_zeros() as usize > b1 + 1 { continue; }
                // X2 from gcd over all short rows at X1=r1
                let mut gacc: U = vec![];
                for hh in &hs {
                    let u = { let mut v = eval_x1(k, hh, r1); trim(&mut v); v };
                    if u.is_empty() { continue; }
                    gacc = if gacc.is_empty() { u } else { ugcd(k, &gacc, &u) };
                }
                if gacc.is_empty() { continue; }
                for r2 in roots(k, &gacc, rng) {
                    if 64 - r2.leading_zeros() as usize > b2 + 1 { continue; }
                    out.candidates.push((P::from_u64(r1), P::from_u64(r2)));
                }
            }
            return out;
        }
    }
    out
}
