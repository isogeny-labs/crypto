//! Counted Wiedemann (Krylov, BM, reconstruction); dense GE mod r (uncounted).
use crate::lines::Row;
use crate::plane_quartic_pic::xorshift_next;

pub const ELL: u64 = (1u64 << 61) - 1;
pub const LAMBDA_PROXY: u64 = 1669582390241348315;

#[derive(Clone, Copy)]
pub struct Modulus {
    pub m: u64,
    fast: bool,
}
impl Modulus {
    pub fn new(m: u64) -> Modulus {
        Modulus { m, fast: m == ELL }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        let x = (a as u128) * (b as u128);
        if self.fast {
            let lo = (x as u64) & ELL;
            let hi = (x >> 61) as u64;
            let mut s = lo + hi;
            if s >= ELL {
                s -= ELL;
            }
            if s >= ELL {
                s -= ELL;
            }
            s
        } else {
            (x % self.m as u128) as u64
        }
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.m {
            s - self.m
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.m - b
        }
    }
    pub fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    pub fn inv(&self, a: u64) -> u64 {
        // prime modulus
        self.pow(a, self.m - 2)
    }
}

/// Sparse square matrix in CSR form.
pub struct Csr {
    pub n: usize,
    pub rowptr: Vec<usize>,
    pub col: Vec<u32>,
    pub val: Vec<u64>,
}
impl Csr {
    pub fn w(&self) -> usize {
        self.val.len()
    }
}

/// Build the n x n matrix from the first n accepted rows (padding with zero rows if fewer).
/// `lam` is the cube root of 1 for orbit rows. Entries that reduce to zero are dropped.
pub fn build_matrix(rows: &[Row], n: usize, md: &Modulus, lam: u64) -> Csr {
    let lam2 = md.mul(lam, lam);
    let mut rowptr = vec![0usize];
    let mut col = Vec::new();
    let mut val = Vec::new();
    for r in 0..n {
        if r < rows.len() {
            match &rows[r] {
                Row::Full(v) => {
                    for &(c, k) in v.iter() {
                        let kk = if k >= 0 {
                            (k as u64) % md.m
                        } else {
                            md.m - ((-k) as u64 % md.m)
                        };
                        if kk % md.m != 0 {
                            col.push(c);
                            val.push(kk % md.m);
                        }
                    }
                }
                Row::Orb(v) => {
                    for &(c, t) in v.iter() {
                        let a = (t[0] as u64) % md.m;
                        let b = md.mul(t[1] as u64, lam);
                        let d = md.mul(t[2] as u64, lam2);
                        let x = md.add(md.add(a, b), d);
                        if x != 0 {
                            col.push(c);
                            val.push(x);
                        }
                    }
                }
            }
        }
        rowptr.push(col.len());
    }
    Csr { n, rowptr, col, val }
}

fn matvec(a: &Csr, v: &[u64], out: &mut [u64], md: &Modulus) {
    for i in 0..a.n {
        let mut acc = 0u64;
        for k in a.rowptr[i]..a.rowptr[i + 1] {
            acc = md.add(acc, md.mul(a.val[k], v[a.col[k] as usize]));
        }
        out[i] = acc;
    }
}

#[derive(Clone, Debug, Default)]
pub struct LaCount {
    pub n: usize,
    pub w: usize,
    pub krylov_mul: u64,
    pub bm_mul: u64,
    pub bm_inv: u64,
    pub l_bm: usize,
    pub recon_mul: u64,
    pub recon_inv: u64,
}
impl LaCount {
    pub fn la_mul(&self) -> u64 {
        self.krylov_mul + self.bm_mul + self.recon_mul
    }
    pub fn la_inv(&self) -> u64 {
        self.bm_inv + self.recon_inv
    }
    pub fn fpm_unw(&self) -> u64 {
        self.la_mul() + 10 * self.la_inv()
    }
    pub fn fpm_w(&self) -> u64 {
        9 * self.fpm_unw()
    }
}

/// Berlekamp-Massey as frozen in SPEC 7. Returns (C, L, mul, inv).
pub fn bm(s: &[u64], md: &Modulus) -> (Vec<u64>, usize, u64, u64) {
    let mut c: Vec<u64> = vec![1];
    let mut b: Vec<u64> = vec![1];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut bb = 1u64;
    let (mut mulc, mut invc) = (0u64, 0u64);
    for k in 0..s.len() {
        let mut d = s[k];
        for i in 1..=l {
            let ci = if i < c.len() { c[i] } else { 0 };
            d = md.add(d, md.mul(ci, s[k - i]));
        }
        mulc += l as u64;
        if d == 0 {
            m += 1;
            continue;
        }
        invc += 1;
        let ib = md.inv(bb);
        let coef = md.mul(d, ib);
        mulc += 1;
        let t = c.clone();
        let need = std::cmp::max(c.len(), b.len() + m);
        c.resize(need, 0);
        for i in 0..b.len() {
            c[i + m] = md.sub(c[i + m], md.mul(coef, b[i]));
        }
        mulc += b.len() as u64;
        if 2 * l <= k {
            l = k + 1 - l;
            b = t;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    (c, l, mulc, invc)
}

pub fn wiedemann_count(a: &Csr, md: &Modulus, solver_seed: u64, p: u64) -> LaCount {
    let n = a.n;
    let w = a.w();
    let mut st: u64 = 0xDEADBEEF ^ (solver_seed << 32) ^ p;
    let u: Vec<u64> = (0..n).map(|_| xorshift_next(&mut st) % md.m).collect();
    let b: Vec<u64> = (0..n).map(|_| xorshift_next(&mut st) % md.m).collect();
    // Krylov
    let mut v = b.clone();
    let mut tmp = vec![0u64; n];
    let mut s = vec![0u64; 2 * n];
    for i in 0..2 * n {
        let mut acc = 0u64;
        for j in 0..n {
            acc = md.add(acc, md.mul(u[j], v[j]));
        }
        s[i] = acc;
        matvec(a, &v, &mut tmp, md);
        std::mem::swap(&mut v, &mut tmp);
    }
    let krylov_mul = 2 * (n as u64) * (n as u64 + w as u64);
    let (cvec, l_bm, bm_mul, bm_inv) = bm(&s, md);
    // reconstruction (counted surrogate)
    let c_l = if l_bm < cvec.len() { cvec[l_bm] } else { 0 };
    let _c0inv = if c_l != 0 { md.inv(c_l) } else { md.inv(1) };
    let mut acc = vec![0u64; n];
    let mut v = b.clone();
    for i in 0..l_bm {
        let ci = if i < cvec.len() { cvec[i] } else { 0 };
        for j in 0..n {
            acc[j] = md.add(acc[j], md.mul(ci, v[j]));
        }
        matvec(a, &v, &mut tmp, md);
        std::mem::swap(&mut v, &mut tmp);
    }
    for j in 0..n {
        acc[j] = md.mul(acc[j], _c0inv);
    }
    let recon_mul = (l_bm as u64) * (n as u64 + w as u64) + n as u64;
    LaCount { n, w, krylov_mul, bm_mul, bm_inv, l_bm, recon_mul, recon_inv: 1 }
}

/// Dense GE mod prime r: returns (rank, kernel basis) of the matrix with `ncols` columns
/// given as rows of dense vectors.
pub fn rank_and_kernel(rows: &[Vec<u64>], ncols: usize, r: u64) -> (usize, Vec<Vec<u64>>) {
    let md = Modulus::new(r);
    let mut a: Vec<Vec<u64>> = rows.to_vec();
    let mut pivcols: Vec<usize> = Vec::new();
    let mut rk = 0usize;
    for c in 0..ncols {
        let mut pr = None;
        for i in rk..a.len() {
            if a[i][c] % r != 0 {
                pr = Some(i);
                break;
            }
        }
        let Some(pi) = pr else { continue };
        a.swap(rk, pi);
        let iv = md.inv(a[rk][c] % r);
        for j in 0..ncols {
            a[rk][j] = md.mul(a[rk][j], iv);
        }
        for i in 0..a.len() {
            if i != rk && a[i][c] != 0 {
                let f = a[i][c];
                for j in 0..ncols {
                    let t = md.mul(f, a[rk][j]);
                    a[i][j] = md.sub(a[i][j], t);
                }
            }
        }
        pivcols.push(c);
        rk += 1;
    }
    let free: Vec<usize> = (0..ncols).filter(|c| !pivcols.contains(c)).collect();
    let mut ker = Vec::new();
    for &fc in free.iter() {
        let mut v = vec![0u64; ncols];
        v[fc] = 1;
        for (ri, &pc) in pivcols.iter().enumerate() {
            v[pc] = md.sub(0, a[ri][fc]);
        }
        ker.push(v);
    }
    (rk, ker)
}
