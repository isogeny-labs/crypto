//! Genus-4 non-hyperelliptic curves as smooth (3,3) curves on P^1 x P^1 over F_p
//! (the canonical model on a split quadric), with Pic^0 arithmetic by linear
//! algebra in the bigraded coordinate ring.
//!
//! Canonical class K = O(1,1) (degree 6).  S_n denotes the (n,n)-forms modulo
//! G, a basis of H^0(nK) for n >= 1.  An effective degree-4 divisor D is carried
//! by W_D = H^0(2K - D), a 5-dimensional subspace of S_2, in reduced row echelon
//! form.  `Pic^0` elements are `D - D0` for a fixed reference divisor D0.
//!
//! combine(Da, Db, Dc) returns D ~ Da + Db - Dc:
//!   V  = Wa*Wb              = H^0(4K - Da - Db)          (products)
//!   f  in S_2 with f*S_2 in V                              (div f = Da+Db+R)
//!   g  in H^0(2K - Dc) with g*V in f*S_4                   (div g = R+Dc+D)
//!   V' = {w : f w = g v, v in V} = H^0(4K - Dc - D)
//!   W_D = {e in S_2 : e*Wc in V'}.
//! If W_a*W_b fails to span (rank check) the same steps run at level 3.

// Items used only by tests or kept for completeness.
#![allow(dead_code)]

use std::cell::Cell;

pub const MAXN: usize = 8;
pub type Sp = Vec<Vec<u32>>;

pub fn pw(p: u64, mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    r
}
pub fn inv(p: u64, a: u64) -> u64 {
    pw(p, a, p - 2)
}
pub fn next_u64(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *s = x;
    x.wrapping_mul(0x2545F4914F6CDD1D)
}
pub fn rnd(s: &mut u64, m: u64) -> u64 {
    (next_u64(s) >> 11) % m
}

/// Row-echelon accumulator with optional combination tracking.
#[derive(Clone)]
pub struct Ech {
    pub p: u64,
    pub piv: Vec<usize>,
    pub rows: Vec<Vec<u32>>,
    pub tr: Vec<Vec<u32>>,
    pub ncols: usize,
}

impl Ech {
    pub fn new(p: u64, ncols: usize) -> Ech {
        Ech {
            p,
            piv: vec![],
            rows: vec![],
            tr: vec![],
            ncols,
        }
    }
    /// Reduce `v` (and the tracker `t`, possibly empty) by the stored rows.
    pub fn reduce(&self, v: &mut [u32], t: &mut [u32], cnt: &Cell<u64>) {
        let p = self.p;
        let mut c = 0u64;
        for (r, &pc) in self.piv.iter().enumerate() {
            let a = v[pc] as u64;
            if a == 0 {
                continue;
            }
            let na = p - a;
            for (x, &y) in v.iter_mut().zip(self.rows[r].iter()) {
                *x = ((*x as u64 + na * y as u64) % p) as u32;
            }
            c += v.len() as u64;
            if !t.is_empty() {
                for (x, &y) in t.iter_mut().zip(self.tr[r].iter()) {
                    *x = ((*x as u64 + na * y as u64) % p) as u32;
                }
                c += t.len() as u64;
            }
        }
        cnt.set(cnt.get() + c);
    }
    /// Insert; returns Some(tracker) if the vector was dependent (tracker is
    /// the vanishing combination), None if it extended the span.
    pub fn insert(
        &mut self,
        mut v: Vec<u32>,
        mut t: Vec<u32>,
        cnt: &Cell<u64>,
    ) -> Option<Vec<u32>> {
        self.reduce(&mut v, &mut t, cnt);
        if let Some(c) = v.iter().position(|&x| x != 0) {
            let iv = inv(self.p, v[c] as u64);
            for x in v.iter_mut() {
                *x = ((*x as u64 * iv) % self.p) as u32;
            }
            for x in t.iter_mut() {
                *x = ((*x as u64 * iv) % self.p) as u32;
            }
            cnt.set(cnt.get() + (v.len() + t.len()) as u64);
            self.piv.push(c);
            self.rows.push(v);
            self.tr.push(t);
            None
        } else {
            Some(t)
        }
    }
    pub fn rank(&self) -> usize {
        self.rows.len()
    }
    pub fn free_cols(&self) -> Vec<usize> {
        (0..self.ncols).filter(|c| !self.piv.contains(c)).collect()
    }
}

pub fn rref(p: u64, mut rows: Vec<Vec<u32>>, cnt: &Cell<u64>) -> Vec<Vec<u32>> {
    let ncol = rows.first().map_or(0, |r| r.len());
    let nrows = rows.len();
    let mut r = 0;
    for c in 0..ncol {
        let Some(pi) = (r..nrows).find(|&i| rows[i][c] != 0) else {
            continue;
        };
        rows.swap(r, pi);
        let iv = inv(p, rows[r][c] as u64);
        for x in rows[r].iter_mut() {
            *x = ((*x as u64 * iv) % p) as u32;
        }
        let pr = rows[r].clone();
        for i in 0..nrows {
            if i != r && rows[i][c] != 0 {
                let a = p - rows[i][c] as u64;
                for (x, &y) in rows[i].iter_mut().zip(pr.iter()) {
                    *x = ((*x as u64 + a * y as u64) % p) as u32;
                }
            }
        }
        cnt.set(cnt.get() + (nrows * ncol) as u64);
        r += 1;
        if r == nrows {
            break;
        }
    }
    rows.truncate(r);
    rows
}

/// Kernel of the map c -> sum_j c_j cols[j]: all combinations vanishing.
pub fn kernel(p: u64, cols: Vec<Vec<u32>>, cnt: &Cell<u64>) -> Vec<Vec<u32>> {
    let n = cols.len();
    let len = cols.first().map_or(0, |c| c.len());
    let mut e = Ech::new(p, len);
    let mut out = vec![];
    for (j, c) in cols.into_iter().enumerate() {
        let mut t = vec![0u32; n];
        t[j] = 1;
        if let Some(k) = e.insert(c, t, cnt) {
            out.push(k);
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct Elt {
    pub w: Sp,
    pub key: Vec<u32>,
}

#[derive(Debug)]
pub enum Fail {
    Rank(usize, usize),
    NoF,
    NoG,
    NotDiv,
    W3Dim(usize),
    Lift,
}

pub struct Curve {
    pub p: u32,
    pub g: [u32; 16],
    lead: (usize, usize),
    pub dim: [usize; MAXN + 1],
    std_mon: Vec<Vec<(usize, usize)>>,
    full: Vec<Vec<Vec<(u16, u32)>>>,
    pub cnt: Cell<u64>,
    pub inner: Cell<u64>,
    pub n_ops: Cell<u64>,
    pub n_fallback: Cell<u64>,
    pub n_exc: Cell<u64>,
    pub n_keyfail: Cell<u64>,
    pub pts: Vec<[u32; 4]>,
    pub d0: Vec<usize>,
    pub zero: Option<Elt>,
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

impl Curve {
    /// Build ring tables for `G = sum g[4i+j] s^i t^(3-i) u^j v^(3-j)`.
    /// Fails unless p is a prime in [5, 2^16) and the curve passes the
    /// smoothness certificate.
    pub fn new(p: u32, g: [u32; 16]) -> Result<Curve, String> {
        if !(5..65536).contains(&p) || !is_prime(p as u64) {
            return Err("bad p".into());
        }
        if g.iter().any(|&c| c >= p) {
            return Err("coef range".into());
        }
        let mut lead = None;
        for i in (0..4).rev() {
            for j in (0..4).rev() {
                if g[4 * i + j] != 0 && lead.is_none() {
                    lead = Some((i, j));
                }
            }
        }
        let lead = lead.ok_or("zero G")?;
        let pp = p as u64;
        let ilc = inv(pp, g[4 * lead.0 + lead.1] as u64);
        let mut dim = [0usize; MAXN + 1];
        let mut std_mon = vec![vec![]; MAXN + 1];
        let mut full: Vec<Vec<Vec<(u16, u32)>>> = vec![vec![]; MAXN + 1];
        for n in 0..=MAXN {
            let w = n + 1;
            let is_std = |i: usize, j: usize| {
                !(n >= 3
                    && i >= lead.0
                    && n - i >= 3 - lead.0
                    && j >= lead.1
                    && n - j >= 3 - lead.1)
            };
            let mut pos = vec![usize::MAX; w * w];
            for i in 0..w {
                for j in 0..w {
                    if is_std(i, j) {
                        pos[i * w + j] = std_mon[n].len();
                        std_mon[n].push((i, j));
                    }
                }
            }
            dim[n] = std_mon[n].len();
            let mut tab: Vec<Vec<(u16, u32)>> = vec![vec![]; w * w];
            for i in 0..w {
                for j in 0..w {
                    let idx = i * w + j;
                    if is_std(i, j) {
                        tab[idx] = vec![(pos[idx] as u16, 1)];
                    } else {
                        let (qi, qj) = (i - lead.0, j - lead.1);
                        let mut acc = vec![0u64; dim[n]];
                        for ti in 0..4 {
                            for tj in 0..4 {
                                let c = g[4 * ti + tj] as u64;
                                if c == 0 || (ti, tj) == lead {
                                    continue;
                                }
                                let coef = (pp - c) * ilc % pp;
                                let tgt = (qi + ti) * w + (qj + tj);
                                for &(ps, cf) in &tab[tgt] {
                                    acc[ps as usize] = (acc[ps as usize] + coef * cf as u64) % pp;
                                }
                            }
                        }
                        tab[idx] = acc
                            .iter()
                            .enumerate()
                            .filter(|(_, &c)| c != 0)
                            .map(|(ps, &c)| (ps as u16, c as u32))
                            .collect();
                    }
                }
            }
            full[n] = tab;
        }
        let c = Curve {
            p,
            g,
            lead,
            dim,
            std_mon,
            full,
            cnt: Cell::new(0),
            inner: Cell::new(0),
            n_ops: Cell::new(0),
            n_fallback: Cell::new(0),
            n_exc: Cell::new(0),
            n_keyfail: Cell::new(0),
            pts: vec![],
            d0: vec![],
            zero: None,
        };
        if !c.smooth_certificate() {
            return Err("not smooth (certificate failed)".into());
        }
        Ok(c)
    }

    pub fn pts_ev1(&self, i: usize) -> [u32; 4] {
        let v = self.eval_vec(1, &self.pts[i]);
        [v[0], v[1], v[2], v[3]]
    }

    pub fn pp(&self) -> u64 {
        self.p as u64
    }
    pub fn mulc(&self) -> u64 {
        self.cnt.get()
    }
    fn addc(&self, k: u64) {
        self.cnt.set(self.cnt.get() + k);
    }

    /// Sufficient smoothness certificate: the ideal (G_s,G_t,G_u,G_v) contains
    /// every (8,8)-form, so there is no singular point over the closure.
    fn smooth_certificate(&self) -> bool {
        let p = self.pp();
        let (a, b) = (8usize, 8usize);
        let w = b + 1;
        // derivative polys: (shift ds,dt of s-exponent / u exponent offsets)
        // each as list of (i,j,coef) with its bidegree (da,db)
        let mut gens: Vec<(usize, usize, Vec<(usize, usize, u64)>)> = vec![];
        let mut gs = vec![];
        let mut gt = vec![];
        let mut gu = vec![];
        let mut gv = vec![];
        for i in 0..4usize {
            for j in 0..4usize {
                let c = self.g[4 * i + j] as u64;
                if c == 0 {
                    continue;
                }
                if i > 0 {
                    gs.push((i - 1, j, c * i as u64 % p));
                }
                if i < 3 {
                    gt.push((i, j, c * (3 - i) as u64 % p));
                }
                if j > 0 {
                    gu.push((i, j - 1, c * j as u64 % p));
                }
                if j < 3 {
                    gv.push((i, j, c * (3 - j) as u64 % p));
                }
            }
        }
        gens.push((2, 3, gs));
        gens.push((2, 3, gt));
        gens.push((3, 2, gu));
        gens.push((3, 2, gv));
        let mut e = Ech::new(p, (a + 1) * w);
        let scratch = Cell::new(0u64);
        for (da, db, terms) in &gens {
            for mi in 0..=(a - da) {
                for mj in 0..=(b - db) {
                    let mut v = vec![0u32; (a + 1) * w];
                    for &(i, j, c) in terms {
                        v[(i + mi) * w + (j + mj)] = c as u32;
                    }
                    e.insert(v, vec![], &scratch);
                }
            }
        }
        e.rank() == (a + 1) * w
    }

    pub fn unit(&self, n: usize, x: usize) -> Vec<u32> {
        let mut v = vec![0u32; self.dim[n]];
        v[x] = 1;
        v
    }

    /// Product of an S_n1 vector and an S_n2 vector in S_(n1+n2).
    pub fn mul(&self, n1: usize, a: &[u32], n2: usize, b: &[u32]) -> Vec<u32> {
        let n = n1 + n2;
        let w = n + 1;
        let p = self.pp();
        let mut acc = vec![0u64; self.dim[n]];
        let mut c = 0u64;
        for (x, &ax) in a.iter().enumerate() {
            if ax == 0 {
                continue;
            }
            let (i1, j1) = self.std_mon[n1][x];
            for (y, &by) in b.iter().enumerate() {
                if by == 0 {
                    continue;
                }
                let (i2, j2) = self.std_mon[n2][y];
                let t = ax as u64 * by as u64;
                for &(ps, cf) in &self.full[n][(i1 + i2) * w + (j1 + j2)] {
                    acc[ps as usize] += t * cf as u64;
                    c += 1;
                }
            }
        }
        self.addc(c);
        acc.into_iter().map(|x| (x % p) as u32).collect()
    }

    /// Values of the std monomials of S_n at a point (s,t,u,v).
    pub fn eval_vec(&self, n: usize, pt: &[u32; 4]) -> Vec<u32> {
        let p = self.pp();
        let pows = |b: u64, e: usize| -> Vec<u64> {
            let mut v = vec![1u64; e + 1];
            for k in 1..=e {
                v[k] = v[k - 1] * b % p;
            }
            v
        };
        let (ps, pt_, pu, pv) = (
            pows(pt[0] as u64, n),
            pows(pt[1] as u64, n),
            pows(pt[2] as u64, n),
            pows(pt[3] as u64, n),
        );
        self.std_mon[n]
            .iter()
            .map(|&(i, j)| (ps[i] * pt_[n - i] % p * (pu[j] * pv[n - j] % p) % p) as u32)
            .collect()
    }

    pub fn eval_g(&self, pt: &[u32; 4]) -> u32 {
        let p = self.pp();
        let mut tot = 0u64;
        let pows = |b: u64| -> [u64; 4] {
            let mut v = [1u64; 4];
            for k in 1..4 {
                v[k] = v[k - 1] * b % p;
            }
            v
        };
        let (a, b, c, d) = (
            pows(pt[0] as u64),
            pows(pt[1] as u64),
            pows(pt[2] as u64),
            pows(pt[3] as u64),
        );
        for i in 0..4 {
            for j in 0..4 {
                let cf = self.g[4 * i + j] as u64;
                if cf != 0 {
                    tot += cf * (a[i] * b[3 - i] % p) % p * (c[j] * d[3 - j] % p) % p;
                }
            }
        }
        (tot % p) as u32
    }

    /// All rational points, each coordinate pair normalised (first nonzero = 1).
    /// Fibre by fibre over (s:t): the cubic in (u:v) is formed (16 mulmods) and
    /// tested at every (u:v) by Horner (3 mulmods each); counted as native work.
    pub fn enumerate_points(&self) -> Vec<[u32; 4]> {
        let p = self.pp();
        let mut out = vec![];
        let mut line = vec![[0u32, 1u32]];
        for x in 0..self.p {
            line.push([1, x]);
        }
        for st in &line {
            let (s, t) = (st[0] as u64, st[1] as u64);
            let mut c = [0u64; 4]; // coefficient of u^j v^(3-j)
            for j in 0..4 {
                for i in 0..4 {
                    let gc = self.g[4 * i + j] as u64;
                    if gc != 0 {
                        c[j] = (c[j] + gc * pw(p, s, i as u64) % p * pw(p, t, 3 - i as u64)) % p;
                    }
                }
            }
            // point (u:v) = (1:0): value c[3]
            if c[3] == 0 {
                out.push([st[0], st[1], 1, 0]);
            }
            // (u:v) = (x:1): c0 + c1 x + c2 x^2 + c3 x^3
            for x in 0..p {
                let v = (((c[3] * x + c[2]) % p * x + c[1]) % p * x + c[0]) % p;
                if v == 0 {
                    out.push([st[0], st[1], x as u32, 1]);
                }
            }
        }
        self.addc((self.p as u64 + 1) * (16 + 3 * self.p as u64 + 3));
        out
    }

    /// Forms of S_k vanishing at all the given points (RREF rows).
    pub fn space_of_points(&self, k: usize, pts: &[[u32; 4]]) -> Sp {
        let evs: Vec<Vec<u32>> = pts.iter().map(|q| self.eval_vec(k, q)).collect();
        let cols: Vec<Vec<u32>> = (0..self.dim[k])
            .map(|x| evs.iter().map(|e| e[x]).collect())
            .collect();
        let ker = kernel(self.pp(), cols, &self.cnt);
        rref(self.pp(), ker, &self.cnt)
    }

    fn residual_free(&self, e: &Ech, mut v: Vec<u32>, free: &[usize]) -> Vec<u32> {
        e.reduce(&mut v, &mut [], &self.cnt);
        free.iter().map(|&c| v[c]).collect()
    }

    pub fn combine_k(&self, wa: &Sp, wb: &Sp, wc: &Sp, k: usize) -> Result<Sp, Fail> {
        let p = self.pp();
        let n2k = 2 * k;
        // 1. V = Wa * Wb
        let mut ev = Ech::new(p, self.dim[n2k]);
        for a in wa {
            for b in wb {
                ev.insert(self.mul(k, a, k, b), vec![], &self.cnt);
            }
        }
        let want_v = 12 * k - 11;
        if ev.rank() != want_v {
            return Err(Fail::Rank(ev.rank(), want_v));
        }
        let vfree = ev.free_cols();
        // 2. f in S_2 with f * S_(2k-2) in V
        let hd = self.dim[n2k - 2];
        let mut cols = vec![];
        for x in 0..self.dim[2] {
            let ex = self.unit(2, x);
            let mut col = vec![];
            for y in 0..hd {
                col.extend(self.residual_free(
                    &ev,
                    self.mul(2, &ex, n2k - 2, &self.unit(n2k - 2, y)),
                    &vfree,
                ));
            }
            cols.push(col);
        }
        let fs = kernel(p, cols, &self.cnt);
        if fs.is_empty() {
            return Err(Fail::NoF);
        }
        let f = &fs[0];
        // 3. g in S_2 with g*V in f*Uc, Uc = H^0(2kK - Dc) = Wc*S_k
        //    (so div g >= R + Dc as divisors, multiplicities included)
        let mut euc = Ech::new(p, self.dim[n2k]);
        for r in wc {
            for y in 0..self.dim[k] {
                euc.insert(self.mul(k, r, k, &self.unit(k, y)), vec![], &self.cnt);
            }
        }
        if euc.rank() != 12 * k - 7 {
            return Err(Fail::Rank(euc.rank(), 12 * k - 7));
        }
        let ucfree = euc.free_cols();
        let nd = self.dim[n2k];
        let mut ef = Ech::new(p, self.dim[n2k + 2]);
        for y in 0..nd {
            let mut t = vec![0u32; nd];
            t[y] = 1;
            let prod = self.mul(2, f, n2k, &self.unit(n2k, y));
            if ef.insert(prod, t, &self.cnt).is_some() {
                return Err(Fail::NoG);
            }
        }
        let ffree = ef.free_cols();
        let mut cols = vec![];
        for x in 0..self.dim[2] {
            let ex = self.unit(2, x);
            let mut col = vec![];
            for v in &ev.rows {
                let mut y = self.mul(2, &ex, n2k, v);
                let mut t = vec![0u32; nd];
                ef.reduce(&mut y, &mut t, &self.cnt);
                col.extend(ffree.iter().map(|&c| y[c]));
                euc.reduce(&mut t, &mut [], &self.cnt);
                col.extend(ucfree.iter().map(|&c| t[c]));
            }
            cols.push(col);
        }
        let gs = kernel(p, cols, &self.cnt);
        if gs.is_empty() {
            return Err(Fail::NoG);
        }
        let g = gs[0].clone();
        // 4. V' = {w : f w = g v}
        let mut evp = Ech::new(p, nd);
        for v in &ev.rows {
            let mut y = self.mul(2, &g, n2k, v);
            let mut t = vec![0u32; nd];
            ef.reduce(&mut y, &mut t, &self.cnt);
            if y.iter().any(|&c| c != 0) {
                return Err(Fail::NotDiv);
            }
            evp.insert(t, vec![], &self.cnt);
        }
        if evp.rank() != want_v {
            return Err(Fail::Rank(evp.rank(), want_v));
        }
        let pfree = evp.free_cols();
        // 5. W = {e in S_k : e * Wc in V'}
        let mut cols = vec![];
        for x in 0..self.dim[k] {
            let ex = self.unit(k, x);
            let mut col = vec![];
            for r in wc {
                col.extend(self.residual_free(&evp, self.mul(k, &ex, k, r), &pfree));
            }
            cols.push(col);
        }
        let ker = kernel(p, cols, &self.cnt);
        if ker.len() != 6 * k - 7 {
            return Err(Fail::W3Dim(ker.len()));
        }
        Ok(rref(p, ker, &self.cnt))
    }

    /// Level 2 -> level 3: W3 = {e in S_3 : e*S_1 in W2*S_2}.
    pub fn lift(&self, w2: &Sp) -> Result<Sp, Fail> {
        let p = self.pp();
        let mut u4 = Ech::new(p, self.dim[4]);
        for r in w2 {
            for y in 0..self.dim[2] {
                u4.insert(self.mul(2, r, 2, &self.unit(2, y)), vec![], &self.cnt);
            }
        }
        if u4.rank() != 17 {
            return Err(Fail::Lift);
        }
        let ufree = u4.free_cols();
        let mut cols = vec![];
        for x in 0..self.dim[3] {
            let ex = self.unit(3, x);
            let mut col = vec![];
            for s in 0..self.dim[1] {
                col.extend(self.residual_free(&u4, self.mul(3, &ex, 1, &self.unit(1, s)), &ufree));
            }
            cols.push(col);
        }
        let ker = kernel(p, cols, &self.cnt);
        if ker.len() != 11 {
            return Err(Fail::Lift);
        }
        Ok(rref(p, ker, &self.cnt))
    }

    /// Level 3 -> level 2: {g in S_2 : g*S_1 in W3}.
    fn lower(&self, w3: &Sp) -> Result<Sp, Fail> {
        let p = self.pp();
        let mut e = Ech::new(p, self.dim[3]);
        for r in w3 {
            e.insert(r.clone(), vec![], &self.cnt);
        }
        let fr = e.free_cols();
        let mut cols = vec![];
        for x in 0..self.dim[2] {
            let ex = self.unit(2, x);
            let mut col = vec![];
            for s in 0..self.dim[1] {
                col.extend(self.residual_free(&e, self.mul(2, &ex, 1, &self.unit(1, s)), &fr));
            }
            cols.push(col);
        }
        let ker = kernel(p, cols, &self.cnt);
        if ker.len() != 5 {
            return Err(Fail::Lift);
        }
        Ok(rref(p, ker, &self.cnt))
    }

    /// Canonical key: the RREF of W_D, or, if D is coplanar (h^0(D) = 2, an
    /// exceptional class), the RREF of H^0(2K - E) for E = (plane) - D.
    pub fn mk(&self, w: Sp) -> Elt {
        let p = self.pp();
        let mut e = Ech::new(p, self.dim[2]);
        for r in &w {
            e.insert(r.clone(), vec![], &self.cnt);
        }
        let fr = e.free_cols();
        let mut cols = vec![];
        for x in 0..self.dim[1] {
            let ex = self.unit(1, x);
            let mut col = vec![];
            for s in 0..self.dim[1] {
                col.extend(self.residual_free(&e, self.mul(1, &ex, 1, &self.unit(1, s)), &fr));
            }
            cols.push(col);
        }
        let lk = kernel(p, cols, &self.cnt);
        let flat = |s: &Sp, tag: u32| -> Vec<u32> {
            let mut k = vec![tag];
            for r in s {
                k.extend(r.iter().copied());
            }
            k
        };
        if lk.is_empty() {
            return Elt {
                key: flat(&w, 0),
                w,
            };
        }
        self.n_exc.set(self.n_exc.get() + 1);
        // exceptional: E = div(l) - D, key from H^0(2K - E)
        let l = &lk[0];
        let mut el = Ech::new(p, self.dim[4]);
        for y in 0..self.dim[3] {
            el.insert(self.mul(1, l, 3, &self.unit(3, y)), vec![], &self.cnt);
        }
        let lfree = el.free_cols();
        let mut cols = vec![];
        for x in 0..self.dim[2] {
            let ex = self.unit(2, x);
            let mut col = vec![];
            for r in &w {
                col.extend(self.residual_free(&el, self.mul(2, &ex, 2, r), &lfree));
            }
            cols.push(col);
        }
        let ker = kernel(p, cols, &self.cnt);
        if ker.len() != 7 {
            self.n_keyfail.set(self.n_keyfail.get() + 1);
            return Elt {
                key: flat(&w, 2),
                w,
            };
        }
        let we = rref(p, ker, &self.cnt);
        Elt {
            key: flat(&we, 1),
            w,
        }
    }

    /// D ~ Da + Db - Dc (as spaces); falls back to level 3 if needed.
    pub fn comb(&self, wa: &Sp, wb: &Sp, wc: &Sp) -> Result<Elt, String> {
        let c0 = self.cnt.get();
        let r = self.comb_inner(wa, wb, wc);
        self.inner.set(self.inner.get() + self.cnt.get() - c0);
        r
    }

    fn comb_inner(&self, wa: &Sp, wb: &Sp, wc: &Sp) -> Result<Elt, String> {
        self.n_ops.set(self.n_ops.get() + 1);
        match self.combine_k(wa, wb, wc, 2) {
            Ok(w) => Ok(self.mk(w)),
            Err(_) => {
                self.n_fallback.set(self.n_fallback.get() + 1);
                let l = |w: &Sp| self.lift(w);
                let r = (|| -> Result<Sp, Fail> {
                    let (a, b, c) = (l(wa)?, l(wb)?, l(wc)?);
                    let w3 = self.combine_k(&a, &b, &c, 3)?;
                    self.lower(&w3)
                })();
                match r {
                    Ok(w) => Ok(self.mk(w)),
                    Err(e) => Err(format!("combine failed at level 3: {e:?}")),
                }
            }
        }
    }

    pub fn zero(&self) -> &Elt {
        self.zero.as_ref().expect("base divisor not set")
    }
    pub fn add(&self, x: &Elt, y: &Elt) -> Elt {
        self.comb(&x.w, &y.w, &self.zero().w).expect("add")
    }
    pub fn neg(&self, x: &Elt) -> Elt {
        let z = self.zero();
        self.comb(&z.w, &z.w, &x.w).expect("neg")
    }
    pub fn sub(&self, x: &Elt, y: &Elt) -> Elt {
        self.comb(&x.w, &self.zero().w, &y.w).expect("sub")
    }
    pub fn is_zero(&self, x: &Elt) -> bool {
        x.key == self.zero().key
    }
    pub fn mul_scalar(&self, x: &Elt, mut k: u128) -> Elt {
        let mut acc = self.zero().clone();
        let mut base = x.clone();
        let mut first = true;
        while k > 0 {
            if k & 1 == 1 {
                acc = if first {
                    base.clone()
                } else {
                    self.add(&acc, &base)
                };
                first = false;
            }
            k >>= 1;
            if k > 0 {
                base = self.add(&base, &base);
            }
        }
        acc
    }

    /// Choose the base divisor D0 (four distinct rational points whose class
    /// is not exceptional) and store the rational points.
    pub fn set_base(&mut self, rng: &mut u64) -> Result<(), String> {
        self.pts = self.enumerate_points();
        if self.pts.len() < 8 {
            return Err("fewer than 8 rational points".into());
        }
        for _ in 0..50 {
            let mut idx = vec![];
            while idx.len() < 4 {
                let i = rnd(rng, self.pts.len() as u64) as usize;
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            let ps: Vec<[u32; 4]> = idx.iter().map(|&i| self.pts[i]).collect();
            let w = self.space_of_points(2, &ps);
            if w.len() != 5 {
                continue;
            }
            let e = self.mk(w);
            if e.key[0] == 0 {
                self.d0 = idx;
                self.zero = Some(e);
                self.n_exc.set(0);
                return Ok(());
            }
        }
        Err("no non-exceptional D0".into())
    }

    /// Effective divisor of four distinct rational points as an element
    /// (class of D - D0).
    pub fn elt_of_points(&self, idx: &[usize]) -> Elt {
        let ps: Vec<[u32; 4]> = idx.iter().map(|&i| self.pts[i]).collect();
        self.mk(self.space_of_points(2, &ps))
    }

    /// Class of P - Pa where Pa = pts[d0[0]].
    pub fn point_class(&self, pi: usize) -> Elt {
        let pa = self.d0[0];
        if pi == pa {
            return self.zero().clone();
        }
        let mut t = vec![];
        for i in 0..self.pts.len() {
            if i != pi && i != pa && t.len() < 3 {
                t.push(i);
            }
        }
        let mut dp = vec![pi];
        dp.extend(&t);
        let mut da = vec![pa];
        da.extend(&t);
        let (ep, ea) = (self.elt_of_points(&dp), self.elt_of_points(&da));
        self.comb(&self.zero().w, &ep.w, &ea.w)
            .expect("point_class")
    }

    /// Rational points in the support of the degree-4 divisor with space `w`
    /// (points where all forms vanish).
    pub fn rational_support(&self, w: &Sp) -> Vec<usize> {
        let p = self.pp();
        let mut out = vec![];
        'pt: for (i, q) in self.pts.iter().enumerate() {
            let ev = self.eval_vec(2, q);
            for r in w {
                let mut s = 0u64;
                for (a, b) in r.iter().zip(ev.iter()) {
                    s += *a as u64 * *b as u64;
                }
                if !s.is_multiple_of(p) {
                    continue 'pt;
                }
            }
            out.push(i);
        }
        self.addc((self.pts.len() * w.len() * self.dim[2]) as u64);
        out
    }
}

// ---------------------------------------------------------------------
// Point counting over F_{p^e} (e <= 4) and the zeta function.
// ---------------------------------------------------------------------

#[derive(Clone)]
pub struct Ext {
    pub p: u64,
    pub e: usize,
    pub m: [u64; 4], // x^e = -sum m[i] x^i
}
pub type El = [u64; 4];

impl Ext {
    pub fn new(p: u64, e: usize, rng: &mut u64) -> Ext {
        loop {
            let mut m = [0u64; 4];
            for x in m.iter_mut().take(e) {
                *x = rnd(rng, p);
            }
            if e == 1 || Ext::irreducible(p, e, &m) {
                return Ext { p, e, m };
            }
        }
    }
    fn irreducible(p: u64, e: usize, m: &[u64; 4]) -> bool {
        // trial division by all monic polys of degree 1..=e/2
        for d in 1..=e / 2 {
            let tot = p.pow(d as u32);
            for code in 0..tot {
                let mut dv = vec![0u64; d + 1];
                let mut c = code;
                for x in dv.iter_mut().take(d) {
                    *x = c % p;
                    c /= p;
                }
                dv[d] = 1;
                let mut f: Vec<u64> = m[..e].to_vec();
                f.push(1);
                // f mod dv
                for k in (d..=e).rev() {
                    let lc = f[k];
                    if lc != 0 {
                        for t in 0..=d {
                            f[k - d + t] = (f[k - d + t] + (p - lc) * dv[t]) % p;
                        }
                    }
                }
                if f[..d].iter().all(|&x| x == 0) {
                    return false;
                }
            }
        }
        true
    }
    pub fn zero(&self) -> El {
        [0; 4]
    }
    pub fn one(&self) -> El {
        [1, 0, 0, 0]
    }
    pub fn from(&self, a: u64) -> El {
        [a % self.p, 0, 0, 0]
    }
    pub fn add(&self, a: &El, b: &El) -> El {
        let mut r = [0; 4];
        for i in 0..self.e {
            r[i] = (a[i] + b[i]) % self.p;
        }
        r
    }
    pub fn sub(&self, a: &El, b: &El) -> El {
        let mut r = [0; 4];
        for i in 0..self.e {
            r[i] = (a[i] + self.p - b[i]) % self.p;
        }
        r
    }
    pub fn neg(&self, a: &El) -> El {
        self.sub(&self.zero(), a)
    }
    pub fn mul(&self, a: &El, b: &El) -> El {
        let e = self.e;
        let mut t = [0u64; 8];
        for i in 0..e {
            for j in 0..e {
                t[i + j] += a[i] * b[j];
            }
        }
        for k in (e..2 * e - 1).rev() {
            let c = t[k] % self.p;
            t[k] = 0;
            if c != 0 {
                for i in 0..e {
                    t[k - e + i] += (self.p - self.m[i]) * c;
                }
            }
        }
        let mut r = [0; 4];
        for i in 0..e {
            r[i] = t[i] % self.p;
        }
        r
    }
    pub fn pow(&self, a: &El, mut n: u128) -> El {
        let mut r = self.one();
        let mut b = *a;
        while n > 0 {
            if n & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            n >>= 1;
        }
        r
    }
    pub fn size(&self) -> u128 {
        (self.p as u128).pow(self.e as u32)
    }
    pub fn inv(&self, a: &El) -> El {
        self.pow(a, self.size() - 2)
    }
    pub fn is_zero(&self, a: &El) -> bool {
        a.iter().all(|&x| x == 0)
    }
    pub fn el(&self, mut i: u64) -> El {
        let mut r = [0; 4];
        for x in r.iter_mut().take(self.e) {
            *x = i % self.p;
            i /= self.p;
        }
        r
    }
}

fn ptrim(f: &Ext, a: &mut Vec<El>) {
    while let Some(l) = a.last() {
        if f.is_zero(l) {
            a.pop();
        } else {
            break;
        }
    }
}
fn pmul(f: &Ext, a: &[El], b: &[El]) -> Vec<El> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![f.zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let t = f.mul(x, y);
            r[i + j] = f.add(&r[i + j], &t);
        }
    }
    ptrim(f, &mut r);
    r
}
fn prem(f: &Ext, a: &[El], m: &[El]) -> Vec<El> {
    let mut a = a.to_vec();
    ptrim(f, &mut a);
    let dm = m.len() - 1;
    let il = f.inv(&m[dm]);
    while a.len() > dm {
        let dl = a.len() - 1;
        let c = f.mul(&a[dl], &il);
        for i in 0..=dm {
            let t = f.mul(&c, &m[i]);
            a[dl - dm + i] = f.sub(&a[dl - dm + i], &t);
        }
        a.pop();
        ptrim(f, &mut a);
    }
    a
}
fn pgcd(f: &Ext, a: &[El], b: &[El]) -> Vec<El> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    ptrim(f, &mut a);
    ptrim(f, &mut b);
    while !b.is_empty() {
        let r = prem(f, &a, &b);
        a = b;
        b = r;
    }
    a
}

/// Number of F_q-points of C, q = p^e, counted fibre by fibre over (s:t).
pub fn count_points(g: &[u32; 16], f: &Ext) -> u64 {
    let q = f.size();
    let mut total = 0u64;
    let mut fibres: Vec<(El, El)> = vec![(f.zero(), f.one())];
    for i in 0..q as u64 {
        fibres.push((f.one(), f.el(i)));
    }
    for (s, t) in fibres {
        // coefficients of u^j v^(3-j)
        let mut sp = [f.one(); 4];
        let mut tp = [f.one(); 4];
        for k in 1..4 {
            sp[k] = f.mul(&sp[k - 1], &s);
            tp[k] = f.mul(&tp[k - 1], &t);
        }
        let mut c = vec![f.zero(); 4];
        for j in 0..4 {
            for i in 0..4 {
                let gc = g[4 * i + j] as u64;
                if gc != 0 {
                    let term = f.mul(&f.from(gc), &f.mul(&sp[i], &tp[3 - i]));
                    c[j] = f.add(&c[j], &term);
                }
            }
        }
        // c[j] is the coefficient of u^j v^(3-j); point (1:0) is a root iff c[3]==0
        if f.is_zero(&c[3]) {
            total += 1;
        }
        let mut cp = c.clone();
        ptrim(f, &mut cp);
        if cp.is_empty() {
            panic!("fibre contained in curve");
        }
        if cp.len() == 1 {
            continue;
        }
        // x^q mod cp
        let x = vec![f.zero(), f.one()];
        let mut r = vec![f.one()];
        let mut b = prem(f, &x, &cp);
        let mut n = q;
        while n > 0 {
            if n & 1 == 1 {
                r = prem(f, &pmul(f, &r, &b), &cp);
            }
            b = prem(f, &pmul(f, &b, &b), &cp);
            n >>= 1;
        }
        // r - x
        let mut d = r;
        while d.len() < 2 {
            d.push(f.zero());
        }
        d[1] = f.sub(&d[1], &f.one());
        ptrim(f, &mut d);
        let gg = pgcd(f, &cp, &d);
        total += (gg.len().max(1) - 1) as u64;
    }
    total
}

/// L-polynomial coefficients a1..a4 from N_1..N_k (k <= 4 given).
pub fn l_coeffs(p: i128, ns: &[i128]) -> Vec<i128> {
    let mut s = vec![0i128];
    let mut pk = 1i128;
    for n in ns {
        pk *= p;
        s.push(n - pk - 1);
    }
    let mut a = vec![1i128];
    for n in 1..=ns.len() {
        let mut t = 0i128;
        for i in 1..=n {
            t += a[n - i] * s[i];
        }
        assert!(t % n as i128 == 0);
        a.push(t / n as i128);
    }
    a
}

/// |Pic^0(F_p)| = L(1) from a1..a4 (a[0] = 1).
pub fn order_from_l(p: i128, a: &[i128]) -> i128 {
    let (a1, a2, a3, a4) = (a[1], a[2], a[3], a[4]);
    1 + p.pow(4) + a1 * (1 + p.pow(3)) + a2 * (1 + p * p) + a3 * (1 + p) + a4
}
