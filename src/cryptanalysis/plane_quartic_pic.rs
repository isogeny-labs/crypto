//! Canonical `Pic^0` arithmetic on a smooth plane quartic over `F_q` (W1 of the
//! plane-quartic index-calculus program).
//!
//! Study: `research/quartic_pic_canonical_20261018/` (frozen `PROTOCOL.md`).
//!
//! # Representation
//!
//! `C : F(X,Y,Z) = 0` is a smooth plane quartic over `F_q` (`q` an odd prime,
//! `q < 2^31`), `K` its canonical class (the line section), `P0` a fixed
//! rational point and `D_inf = (tangent line at P0) - P0 ~ K - P0`. A class
//! `x` in `Pic^0(C)(F_q)` is stored through `x + D_inf in Pic^3`.
//!
//! By Riemann-Roch every degree-3 class has `h^0 = 1` (a unique effective
//! divisor `D`, not collinear) or `h^0 = 2` (the class is `K - P` for a unique
//! rational point `P`). Hence the canonical forms:
//!
//! * [`PicClass::Typ`]: the reduced row-echelon basis of the 3-dimensional
//!   space of conics through `D` (it determines `D`);
//! * [`PicClass::Exc`]: the normalised coordinates of `P`. The zero class is
//!   `Exc(P0)`.
//!
//! Divisors are never stored as point lists. An effective divisor is carried by
//! a *generating space* of forms in the coordinate ring `R = F_q[X,Y,Z]/(F)`
//! whose common zero scheme on `C` is exactly the divisor; residuals use the
//! ideal-quotient rule `e in I(R) iff e*v in (c) for all v in I(Z)`. Everything
//! is `F_q` linear algebra (no extension fields, no root finding).
//!
//! # Group law
//!
//! `add`: `Z6 = D_x + D_y`; the 4-dimensional space of cubics through `Z6`;
//! impose `P0` to get the 3-dimensional space `V` of cubics through
//! `E7 = Z6 + P0`. A conic through `E7` means `E7 ~ 2K - P`, so the sum is
//! `Exc(P)`. Otherwise, for a cubic `c in V` the unique conic `q'` through the
//! residual `R5 = div(c) - E7` satisfies `q' v = c g_v` for `v in V`, and the
//! `g_v` span the conics through the sum divisor `D_z` (`D_z + K ~ E7`).
//!
//! `neg`: the conic through `D_x + 2 P0` (a pencil gives `Exc(P)`), residual.
//!
//! # Cost accounting
//!
//! Every `F_q` multiplication and inversion is counted in [`FqCounter`]
//! (additions are counted but not priced). Fallbacks (generating spaces that
//! needed lifting to a higher degree, `P0` in the support of the sum, the
//! exceptional paths) are counted in [`PicStats`] and are never silent.

use std::cell::Cell;
use std::collections::HashMap;

/// A projective point of `P^2(F_q)`, normalised so that its first nonzero
/// coordinate is 1 (see [`PlaneQuartic::normalize_point`]).
pub type Pt = [u32; 3];

/// Maximum degree of the coordinate ring tables.
const MAXD: usize = 8;

/// Counters of `F_q` operations. `mul` and `inv` are priced; `add` is only
/// counted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FqCounter {
    pub mul: u64,
    pub inv: u64,
    pub add: u64,
}

impl FqCounter {
    /// Componentwise difference `self - earlier`.
    pub fn since(&self, earlier: &FqCounter) -> FqCounter {
        FqCounter {
            mul: self.mul - earlier.mul,
            inv: self.inv - earlier.inv,
            add: self.add - earlier.add,
        }
    }
}

/// Counters of the non-generic paths taken (never silent; the fraction of
/// operations that took each is reported by the study).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PicStats {
    /// Operations started (`add`, `neg`, `reduce_deg6`).
    pub ops: u64,
    /// Generating spaces lifted to a higher degree to become complete.
    pub lifts: u64,
    /// `P0` was in the support of the degree-6 divisor (general E7 path).
    pub p0_in_support: u64,
    /// A conic through `E7` existed (exceptional result of `add`).
    pub exc_results: u64,
    /// A pencil of conics through `D_x + 2 P0` (exceptional result of `neg`).
    pub neg_pencil: u64,
    /// An exceptional operand had to be expanded by a line section.
    pub exc_operands: u64,
    /// `F_q` operations spent normalising results into canonical form
    /// (the "key encoding" share of the totals).
    pub norm_mul: u64,
    pub norm_inv: u64,
}

/// Canonical class key. `key(x) == key(y)` iff `x == y` in `Pic^0`.
/// `[tag, data...]` with tag 0 = exceptional (`data[0..3]` = point), tag 1 =
/// typical (`data[0..18]` = the echelon matrix of conics, row-major).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PicKey(pub [u32; 19]);

/// A class of `Pic^0(C)(F_q)` in canonical form.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PicClass {
    /// `h^0 = 2`: the class `x` with `x + D_inf = K - P`; `P` normalised.
    Exc(Pt),
    /// `h^0 = 1`: reduced echelon basis of the conics through the unique
    /// effective degree-3 divisor in `x + D_inf`.
    Typ([[u32; 6]; 3]),
}

impl PicClass {
    /// The canonical key (a function of the class only).
    pub fn key(&self) -> PicKey {
        let mut k = [0u32; 19];
        match self {
            PicClass::Exc(p) => {
                k[1..4].copy_from_slice(p);
            }
            PicClass::Typ(m) => {
                k[0] = 1;
                for (i, row) in m.iter().enumerate() {
                    k[1 + 6 * i..7 + 6 * i].copy_from_slice(row);
                }
            }
        }
        PicKey(k)
    }
}

/// A generating space: forms of degree `k` (vectors in `R_k`) whose common zero
/// scheme on `C` is the divisor. Not necessarily complete.
#[derive(Clone, Debug)]
pub struct Gen {
    pub k: usize,
    pub rows: Vec<Vec<u32>>,
}

/// Errors from constructing a curve.
#[derive(Debug, PartialEq, Eq)]
pub enum PicError {
    /// `q` is not an odd prime below `2^31`.
    BadField,
    /// The smoothness certificate (`(F_X,F_Y,F_Z)_7 = S_7`) failed.
    NotSmooth,
    /// No rational point was found / supplied point not on the curve.
    NoPoint,
}

struct Fq {
    q: u64,
    c_mul: Cell<u64>,
    c_inv: Cell<u64>,
    c_add: Cell<u64>,
}

impl Fq {
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        self.c_mul.set(self.c_mul.get() + 1);
        ((a as u64 * b as u64) % self.q) as u32
    }
    #[inline]
    fn mulu(&self, a: u32, b: u32) -> u32 {
        ((a as u64 * b as u64) % self.q) as u32
    }
    #[inline]
    fn add(&self, a: u32, b: u32) -> u32 {
        self.c_add.set(self.c_add.get() + 1);
        let s = a as u64 + b as u64;
        (if s >= self.q { s - self.q } else { s }) as u32
    }
    #[inline]
    fn sub(&self, a: u32, b: u32) -> u32 {
        self.c_add.set(self.c_add.get() + 1);
        if a >= b {
            a - b
        } else {
            (a as u64 + self.q - b as u64) as u32
        }
    }
    #[inline]
    fn neg(&self, a: u32) -> u32 {
        if a == 0 {
            0
        } else {
            (self.q - a as u64) as u32
        }
    }
    fn invu(&self, a: u32) -> u32 {
        // extended Euclid on (q, a)
        let (mut r0, mut r1) = (self.q as i64, a as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let qt = r0 / r1;
            let r2 = r0 - qt * r1;
            r0 = r1;
            r1 = r2;
            let t2 = t0 - qt * t1;
            t0 = t1;
            t1 = t2;
        }
        debug_assert_eq!(r0, 1);
        (((t0 % self.q as i64) + self.q as i64) % self.q as i64) as u32
    }
    fn inv(&self, a: u32) -> u32 {
        assert!(a != 0, "inversion of zero");
        self.c_inv.set(self.c_inv.get() + 1);
        self.invu(a)
    }
    fn powu(&self, mut a: u32, mut e: u64) -> u32 {
        let mut r = 1u32;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulu(r, a);
            }
            a = self.mulu(a, a);
            e >>= 1;
        }
        r
    }
}

/// A subspace of `R_n` in reduced echelon form together with the quotient map
/// `R_n -> R_n / W` (unit vectors sent to quotient coordinates).
struct Quot {
    rows: Vec<Vec<u32>>,
    free_rank: Vec<Option<usize>>,
    /// For a pivot position `p` (row index `r`): coordinates of `e_p` mod `W`.
    red: Vec<Vec<u32>>,
    pivrow: Vec<usize>,
    qdim: usize,
}

/// Monomials of degree `n` in the fixed order (`a` descending, then `b`).
pub fn mons_of_degree(n: usize) -> Vec<(u8, u8, u8)> {
    let mut v = Vec::new();
    for a in (0..=n).rev() {
        for b in (0..=n - a).rev() {
            v.push((a as u8, b as u8, (n - a - b) as u8));
        }
    }
    v
}

/// A smooth plane quartic with `Pic^0` arithmetic.
pub struct PlaneQuartic {
    fq: Fq,
    q: u32,
    /// Coefficients of `F` in the order of `mons_of_degree(4)`.
    f: [u32; 15],
    mons: Vec<Vec<(u8, u8, u8)>>,
    midx: Vec<Vec<usize>>,
    dim: Vec<usize>,
    std_mon: Vec<Vec<usize>>,
    nf: Vec<Vec<Vec<(usize, u32)>>>,
    prod: Vec<Vec<Vec<usize>>>,
    p0: Pt,
    p0_vals: Vec<Vec<u32>>,
    lines_p0: Gen,
    cons2p0: Gen,
    stats: Cell<PicStats>,
    setup_cost: FqCounter,
}

/// Is the quartic `f` smooth over `\bar F_q`? Certificate: the partials
/// `F_X,F_Y,F_Z` (a regular sequence iff smooth, `q` odd) satisfy
/// `(F_X,F_Y,F_Z)_7 = S_7`, since `S/(F_X,F_Y,F_Z)` has Hilbert series
/// `(1+t+t^2)^3` (top degree 6) when smooth and never vanishes in a degree when
/// the partials have a common zero.
pub fn smoothness_certificate(q: u32, f: &[u32; 15]) -> bool {
    let fq = Fq {
        q: q as u64,
        c_mul: Cell::new(0),
        c_inv: Cell::new(0),
        c_add: Cell::new(0),
    };
    let m4 = mons_of_degree(4);
    let m3 = mons_of_degree(3);
    let m7 = mons_of_degree(7);
    let m7i: HashMap<(u8, u8), usize> = m7.iter().enumerate().map(|(i, m)| ((m.0, m.1), i)).collect();
    let m3i: HashMap<(u8, u8), usize> = m3.iter().enumerate().map(|(i, m)| ((m.0, m.1), i)).collect();
    // partials
    let mut part = vec![vec![0u32; 10]; 3];
    for (i, &(a, b, c)) in m4.iter().enumerate() {
        let co = f[i];
        if co == 0 {
            continue;
        }
        if a > 0 {
            let v = fq.mulu(co, a as u32);
            part[0][m3i[&(a - 1, b)]] = v;
        }
        if b > 0 {
            let v = fq.mulu(co, b as u32);
            part[1][m3i[&(a, b - 1)]] = v;
        }
        if c > 0 {
            let v = fq.mulu(co, c as u32);
            part[2][m3i[&(a, b)]] = v;
        }
    }
    let mut rows: Vec<Vec<u32>> = Vec::new();
    for p in &part {
        for &(a2, b2, _c2) in &m4 {
            let mut r = vec![0u32; 36];
            for (j, &(a, b, _)) in m3.iter().enumerate() {
                if p[j] != 0 {
                    let idx = m7i[&(a + a2, b + b2)];
                    r[idx] = p[j];
                }
            }
            rows.push(r);
        }
    }
    let rank = rank_mod(&fq, rows, 36);
    rank == 36
}

fn rank_mod(fq: &Fq, mut rows: Vec<Vec<u32>>, ncols: usize) -> usize {
    let mut r = 0;
    for col in 0..ncols {
        let Some(p) = (r..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(r, p);
        let iv = fq.invu(rows[r][col]);
        for c in col..ncols {
            rows[r][c] = fq.mulu(rows[r][c], iv);
        }
        let prow = rows[r].clone();
        for i in r + 1..rows.len() {
            let fct = rows[i][col];
            if fct != 0 {
                for c in col..ncols {
                    let t = fq.mulu(fct, prow[c]);
                    rows[i][c] = fq.sub_nc(rows[i][c], t);
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    r
}

impl Fq {
    fn sub_nc(&self, a: u32, b: u32) -> u32 {
        if a >= b {
            a - b
        } else {
            (a as u64 + self.q - b as u64) as u32
        }
    }
}

/// Is `q` an odd prime below `2^31`?
pub fn is_odd_prime_below_2_31(q: u64) -> bool {
    if q < 3 || q >= (1u64 << 31) || q % 2 == 0 {
        return false;
    }
    let mut d = 3u64;
    while d * d <= q {
        if q % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

impl PlaneQuartic {
    /// Build the arithmetic for `F = sum f[i] * mons_of_degree(4)[i]` with the
    /// rational point `p0` as the base point. Fails unless `q` is an odd prime
    /// `< 2^31`, the smoothness certificate passes and `p0` lies on `C`.
    pub fn new(q: u32, f: [u32; 15], p0: Pt) -> Result<PlaneQuartic, PicError> {
        if !is_odd_prime_below_2_31(q as u64) {
            return Err(PicError::BadField);
        }
        if f.iter().any(|&c| c >= q) {
            return Err(PicError::BadField);
        }
        if !smoothness_certificate(q, &f) {
            return Err(PicError::NotSmooth);
        }
        let fq = Fq {
            q: q as u64,
            c_mul: Cell::new(0),
            c_inv: Cell::new(0),
            c_add: Cell::new(0),
        };
        // monomial tables
        let mons: Vec<Vec<(u8, u8, u8)>> = (0..=MAXD).map(mons_of_degree).collect();
        let midx: Vec<Vec<usize>> = (0..=MAXD)
            .map(|n| {
                let mut t = vec![usize::MAX; (n + 1) * (n + 1)];
                for (i, m) in mons[n].iter().enumerate() {
                    t[m.0 as usize * (n + 1) + m.1 as usize] = i;
                }
                t
            })
            .collect();
        let m4 = &mons[4];
        let lead_i = (0..15).find(|&i| f[i] != 0).ok_or(PicError::NotSmooth)?;
        let lead = m4[lead_i];
        let lcoef = f[lead_i];
        let ilc = fq.invu(lcoef);
        let mut dim = vec![0usize; MAXD + 1];
        let mut std_mon: Vec<Vec<usize>> = vec![vec![]; MAXD + 1];
        let mut nf: Vec<Vec<Vec<(usize, u32)>>> = vec![vec![]; MAXD + 1];
        for n in 0..=MAXD {
            let ms = &mons[n];
            let is_std = |m: &(u8, u8, u8)| !(n >= 4 && m.0 >= lead.0 && m.1 >= lead.1 && m.2 >= lead.2);
            let mut pos = vec![usize::MAX; ms.len()];
            for (i, m) in ms.iter().enumerate() {
                if is_std(m) {
                    pos[i] = std_mon[n].len();
                    std_mon[n].push(i);
                }
            }
            dim[n] = std_mon[n].len();
            let mut tab: Vec<Vec<(usize, u32)>> = vec![vec![]; ms.len()];
            for i in (0..ms.len()).rev() {
                let m = ms[i];
                if is_std(&m) {
                    tab[i] = vec![(pos[i], 1)];
                } else {
                    let t = (m.0 - lead.0, m.1 - lead.1, m.2 - lead.2);
                    let mut acc = vec![0u32; dim[n]];
                    for (fi, &fm) in m4.iter().enumerate() {
                        if fi == lead_i || f[fi] == 0 {
                            continue;
                        }
                        let mm = (fm.0 + t.0, fm.1 + t.1);
                        let mi = midx[n][mm.0 as usize * (n + 1) + mm.1 as usize];
                        let coef = fq.mulu(fq.neg(f[fi]), ilc);
                        for &(p, cf) in &tab[mi] {
                            let add = fq.mulu(coef, cf);
                            acc[p] = fq.sub_nc(acc[p], fq.neg(add));
                        }
                    }
                    tab[i] = acc
                        .iter()
                        .enumerate()
                        .filter(|(_, &c)| c != 0)
                        .map(|(p, &c)| (p, c))
                        .collect();
                }
            }
            nf[n] = tab;
        }
        // product index tables prod[i][j][x*dim[j]+y] = monomial index in S_{i+j}
        let mut prod: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; MAXD + 1]; MAXD + 1];
        for i in 0..=MAXD {
            for j in 0..=MAXD - i {
                let n = i + j;
                let mut t = Vec::with_capacity(dim[i] * dim[j]);
                for &xi in &std_mon[i] {
                    let a = mons[i][xi];
                    for &yj in &std_mon[j] {
                        let b = mons[j][yj];
                        t.push(midx[n][(a.0 + b.0) as usize * (n + 1) + (b.1 + a.1) as usize]);
                    }
                }
                prod[i][j] = t;
            }
        }
        let mut s = PlaneQuartic {
            fq,
            q,
            f,
            mons,
            midx,
            dim,
            std_mon,
            nf,
            prod,
            p0: [0; 3],
            p0_vals: vec![],
            lines_p0: Gen { k: 1, rows: vec![] },
            cons2p0: Gen { k: 2, rows: vec![] },
            stats: Cell::new(PicStats::default()),
            setup_cost: FqCounter::default(),
        };
        let p0 = s.normalize_point(p0);
        if p0 == [0, 0, 0] || s.eval_f(p0) != 0 {
            return Err(PicError::NoPoint);
        }
        s.p0 = p0;
        s.setup_base_point();
        Ok(s)
    }

    /// Order of the base field.
    pub fn q(&self) -> u32 {
        self.q
    }
    /// The quartic's coefficients in the order of `mons_of_degree(4)`.
    pub fn coefficients(&self) -> &[u32; 15] {
        &self.f
    }
    /// The fixed rational base point `P0`.
    pub fn base_point(&self) -> Pt {
        self.p0
    }
    /// Snapshot of the `F_q` operation counters.
    pub fn counter(&self) -> FqCounter {
        FqCounter {
            mul: self.fq.c_mul.get(),
            inv: self.fq.c_inv.get(),
            add: self.fq.c_add.get(),
        }
    }
    /// Reset the counters and the path statistics.
    pub fn reset_counter(&self) {
        self.fq.c_mul.set(0);
        self.fq.c_inv.set(0);
        self.fq.c_add.set(0);
        self.stats.set(PicStats::default());
    }
    /// Snapshot of the path statistics.
    pub fn stats(&self) -> PicStats {
        self.stats.get()
    }
    /// Cost of the one-time base-point setup (not charged to operations).
    pub fn setup_cost(&self) -> FqCounter {
        self.setup_cost
    }

    fn bump<F: FnOnce(&mut PicStats)>(&self, f: F) {
        let mut s = self.stats.get();
        f(&mut s);
        self.stats.set(s);
    }

    /// Scale a projective point so that its first nonzero coordinate is 1.
    pub fn normalize_point(&self, p: Pt) -> Pt {
        let Some(i) = (0..3).find(|&i| p[i] != 0) else {
            return [0, 0, 0];
        };
        let iv = self.fq.invu(p[i]);
        [
            self.fq.mulu(p[0], iv),
            self.fq.mulu(p[1], iv),
            self.fq.mulu(p[2], iv),
        ]
    }

    /// `F(p)` (uncounted helper).
    pub fn eval_f(&self, p: Pt) -> u32 {
        let mut s = 0u32;
        for (i, &(a, b, c)) in self.mons[4].iter().enumerate() {
            if self.f[i] == 0 {
                continue;
            }
            let t = self.fq.mulu(
                self.fq.mulu(self.fq.powu(p[0], a as u64), self.fq.powu(p[1], b as u64)),
                self.fq.powu(p[2], c as u64),
            );
            s = (s as u64 + self.fq.mulu(t, self.f[i]) as u64).rem_euclid(self.q as u64) as u32;
        }
        s
    }

    /// Dimension of `R_n = S_n / F S_{n-4}`.
    pub fn dim_r(&self, n: usize) -> usize {
        self.dim[n]
    }

    /// Value of the standard monomials of `R_n` at `p` (uncounted).
    pub fn std_values(&self, n: usize, p: Pt) -> Vec<u32> {
        self.std_mon[n]
            .iter()
            .map(|&mi| {
                let (a, b, c) = self.mons[n][mi];
                self.fq.mulu(
                    self.fq.mulu(self.fq.powu(p[0], a as u64), self.fq.powu(p[1], b as u64)),
                    self.fq.powu(p[2], c as u64),
                )
            })
            .collect()
    }

    /// Standard monomials of `R_n`.
    pub fn std_monomials(&self, n: usize) -> Vec<(u8, u8, u8)> {
        self.std_mon[n].iter().map(|&i| self.mons[n][i]).collect()
    }

    // ---------------------------------------------------------------------
    // linear algebra over F_q (counted)
    // ---------------------------------------------------------------------

    fn rref(&self, rows: &mut Vec<Vec<u32>>, ncols: usize) -> Vec<usize> {
        let mut piv = Vec::new();
        let mut r = 0;
        for col in 0..ncols {
            if r == rows.len() {
                break;
            }
            let Some(p) = (r..rows.len()).find(|&i| rows[i][col] != 0) else {
                continue;
            };
            rows.swap(r, p);
            if rows[r][col] != 1 {
                let iv = self.fq.inv(rows[r][col]);
                rows[r][col] = 1;
                for c in col + 1..ncols {
                    if rows[r][c] != 0 {
                        rows[r][c] = self.fq.mul(rows[r][c], iv);
                    }
                }
            }
            let prow = rows[r].clone();
            for i in 0..rows.len() {
                if i == r {
                    continue;
                }
                let fct = rows[i][col];
                if fct == 0 {
                    continue;
                }
                rows[i][col] = 0;
                for c in col + 1..ncols {
                    if prow[c] != 0 {
                        let t = self.fq.mul(fct, prow[c]);
                        rows[i][c] = self.fq.sub(rows[i][c], t);
                    }
                }
            }
            piv.push(col);
            r += 1;
        }
        rows.truncate(r);
        piv
    }

    /// Basis of `{x : A x = 0}` for the equation rows `eqs` (each of length `n`).
    fn nullspace(&self, mut eqs: Vec<Vec<u32>>, n: usize) -> Vec<Vec<u32>> {
        let piv = self.rref(&mut eqs, n);
        let mut is_piv = vec![false; n];
        for &p in &piv {
            is_piv[p] = true;
        }
        let mut out = Vec::new();
        for f in 0..n {
            if is_piv[f] {
                continue;
            }
            let mut x = vec![0u32; n];
            x[f] = 1;
            for (r, &p) in piv.iter().enumerate() {
                x[p] = self.fq.neg(eqs[r][f]);
            }
            out.push(x);
        }
        out
    }

    fn quot_from(&self, mut rows: Vec<Vec<u32>>, n: usize) -> Quot {
        let piv = self.rref(&mut rows, n);
        let mut free_rank = vec![None; n];
        let mut free = Vec::new();
        let mut is_piv = vec![false; n];
        for &p in &piv {
            is_piv[p] = true;
        }
        for i in 0..n {
            if !is_piv[i] {
                free_rank[i] = Some(free.len());
                free.push(i);
            }
        }
        let mut red = vec![vec![]; n];
        let mut pivrow = vec![usize::MAX; n];
        for (r, &p) in piv.iter().enumerate() {
            pivrow[p] = r;
            red[p] = free.iter().map(|&f| self.fq.neg(rows[r][f])).collect();
        }
        Quot {
            rows,
            free_rank,
            red,
            pivrow,
            qdim: free.len(),
        }
    }

    fn quot_apply(&self, qt: &Quot, v: &[u32]) -> Vec<u32> {
        let mut out = vec![0u32; qt.qdim];
        for (i, &vi) in v.iter().enumerate() {
            if vi == 0 {
                continue;
            }
            if let Some(k) = qt.free_rank[i] {
                out[k] = self.fq.add(out[k], vi);
            } else {
                for (k, &rk) in qt.red[i].iter().enumerate() {
                    if rk != 0 {
                        let t = self.fq.mul(vi, rk);
                        out[k] = self.fq.add(out[k], t);
                    }
                }
            }
        }
        out
    }

    fn quot_apply_sparse(&self, qt: &Quot, v: &[(usize, u32)]) -> Vec<u32> {
        let mut out = vec![0u32; qt.qdim];
        for &(i, vi) in v {
            if let Some(k) = qt.free_rank[i] {
                out[k] = if vi == 1 { self.fq.add(out[k], 1) } else { self.fq.add(out[k], vi) };
            } else {
                for (k, &rk) in qt.red[i].iter().enumerate() {
                    if rk != 0 {
                        let t = if vi == 1 { rk } else { self.fq.mul(vi, rk) };
                        out[k] = self.fq.add(out[k], t);
                    }
                }
            }
        }
        out
    }

    /// Product in the coordinate ring: `a in R_i`, `b in R_j` gives `R_{i+j}`.
    pub fn mul_forms(&self, i: usize, a: &[u32], j: usize, b: &[u32]) -> Vec<u32> {
        let n = i + j;
        let mut res = vec![0u32; self.dim[n]];
        let pt = &self.prod[i][j];
        let dj = self.dim[j];
        for (x, &ax) in a.iter().enumerate() {
            if ax == 0 {
                continue;
            }
            for (y, &by) in b.iter().enumerate() {
                if by == 0 {
                    continue;
                }
                let t = if ax == 1 {
                    by
                } else if by == 1 {
                    ax
                } else {
                    self.fq.mul(ax, by)
                };
                for &(pos, cf) in &self.nf[n][pt[x * dj + y]] {
                    let u = if cf == 1 { t } else { self.fq.mul(t, cf) };
                    res[pos] = self.fq.add(res[pos], u);
                }
            }
        }
        res
    }

    fn unit(&self, n: usize, i: usize) -> Vec<u32> {
        let mut v = vec![0u32; self.dim[n]];
        v[i] = 1;
        v
    }

    /// `e in R_m` with `e * x in W` for every `x in R_{k-m}`, where `W` is a
    /// complete space of degree-`k` forms through a divisor `Z` (all of
    /// `H^0(kK - Z)`, with `deg(kK-Z) >= 6`). Equivalent to `div(e) >= Z`.
    fn forms_through(&self, w: &Quot, k: usize, m: usize) -> Vec<Vec<u32>> {
        let t = k - m;
        let nm = self.dim[m];
        let nt = self.dim[t];
        let qd = w.qdim;
        let pt = &self.prod[m][t];
        let mut eqs = vec![vec![0u32; nm]; nt * qd];
        for j in 0..nm {
            for s in 0..nt {
                let img = self.quot_apply_sparse(w, &self.nf[k][pt[j * nt + s]]);
                for (r, &v) in img.iter().enumerate() {
                    eqs[s * qd + r][j] = v;
                }
            }
        }
        self.nullspace(eqs, nm)
    }

    /// `e in R_m` with `e * v in c * R_{m+vk-cn}` for every `v` in `vs`
    /// (`vs` generate the ideal of `Z`): `div(e) >= div(c) - Z`.
    fn colon(&self, c: &[u32], cn: usize, vs: &[Vec<u32>], vk: usize, m: usize) -> Vec<Vec<u32>> {
        let n = m + vk;
        let t = n - cn;
        let crows: Vec<Vec<u32>> = (0..self.dim[t])
            .map(|i| self.mul_forms(cn, c, t, &self.unit(t, i)))
            .collect();
        let qt = self.quot_from(crows, self.dim[n]);
        assert_eq!(qt.rows.len(), self.dim[t], "multiplication by c not injective");
        let nm = self.dim[m];
        let qd = qt.qdim;
        let mut eqs = vec![vec![0u32; nm]; vs.len() * qd];
        for j in 0..nm {
            let uj = self.unit(m, j);
            for (s, v) in vs.iter().enumerate() {
                let prod = self.mul_forms(m, &uj, vk, v);
                let img = self.quot_apply(&qt, &prod);
                for (r, &val) in img.iter().enumerate() {
                    eqs[s * qd + r][j] = val;
                }
            }
        }
        self.nullspace(eqs, nm)
    }

    /// Solve `sum_j x_j cols[j] = rhs[s]` for each `s`; `None` unless unique.
    fn solve_multi(&self, cols: &[Vec<u32>], rhs: &[Vec<u32>]) -> Option<Vec<Vec<u32>>> {
        let u = cols.len();
        let nr = cols[0].len();
        let width = u + rhs.len();
        let mut rows = vec![vec![0u32; width]; nr];
        for r in 0..nr {
            for (j, c) in cols.iter().enumerate() {
                rows[r][j] = c[r];
            }
            for (s, b) in rhs.iter().enumerate() {
                rows[r][u + s] = b[r];
            }
        }
        let piv = self.rref(&mut rows, width);
        if piv.iter().any(|&p| p >= u) || piv.len() != u {
            return None;
        }
        let mut out = vec![vec![0u32; u]; rhs.len()];
        for (r, &p) in piv.iter().enumerate() {
            for s in 0..rhs.len() {
                out[s][p] = rows[r][u + s];
            }
        }
        Some(out)
    }

    // ---------------------------------------------------------------------
    // generating spaces
    // ---------------------------------------------------------------------

    /// Product of two generating spaces (generates the ideal of `Z_a + Z_b`).
    pub fn gen_mul(&self, a: &Gen, b: &Gen) -> Gen {
        let k = a.k + b.k;
        let mut rows = Vec::with_capacity(a.rows.len() * b.rows.len());
        for x in &a.rows {
            for y in &b.rows {
                rows.push(self.mul_forms(a.k, x, b.k, y));
            }
        }
        let n = self.dim[k];
        self.rref(&mut rows, n);
        Gen { k, rows }
    }

    /// Lift a generating space of a divisor of degree `dz` to the lowest
    /// degree `k` where it spans all of `H^0(kK - Z)` (and that system is base
    /// point free: `4k - dz >= 6`). Returns the degree and the quotient data.
    fn complete(&self, g: Gen, dz: usize) -> (usize, Quot) {
        let mut g = g;
        let mut first = true;
        loop {
            let k = g.k;
            let expect = self.dim[k] as isize - dz as isize;
            let n = self.dim[k];
            let mut rows = g.rows.clone();
            let r = self.rref(&mut rows, n).len();
            if 4 * k as isize - dz as isize >= 6 && r as isize == expect {
                return (k, self.quot_from(rows, n));
            }
            assert!(
                (r as isize) <= expect.max(0),
                "generating space too large: rank {} > {} at degree {} for deg-{} divisor",
                r,
                expect,
                k,
                dz
            );
            assert!(k + 1 < MAXD, "generating space did not complete by degree {}", MAXD - 1);
            if first {
                self.bump(|s| s.lifts += 1);
                first = false;
            }
            let r1 = Gen {
                k: 1,
                rows: (0..3).map(|i| self.unit(1, i)).collect(),
            };
            g = self.gen_mul(&Gen { k, rows }, &r1);
        }
    }

    fn eval_p0(&self, n: usize, v: &[u32]) -> u32 {
        let mut s = 0u32;
        for (i, &vi) in v.iter().enumerate() {
            if vi != 0 {
                let t = self.fq.mul(vi, self.p0_vals[n][i]);
                s = self.fq.add(s, t);
            }
        }
        s
    }

    fn setup_base_point(&mut self) {
        let p0 = self.p0;
        self.p0_vals = (0..=MAXD).map(|n| self.std_values(n, p0)).collect();
        // lines through P0
        let eq = vec![self.p0_vals[1].clone()];
        let ls = self.nullspace(eq, 3);
        assert_eq!(ls.len(), 2);
        self.lines_p0 = Gen { k: 1, rows: ls };
        // gradient of F at P0
        let mut grad = [0u32; 3];
        for (i, &(a, b, c)) in self.mons[4].iter().enumerate() {
            let co = self.f[i];
            if co == 0 {
                continue;
            }
            let pw = |e: u8, x: u32| self.fq.powu(x, e as u64);
            if a > 0 {
                let t = self.fq.mulu(
                    self.fq.mulu(co, a as u32),
                    self.fq.mulu(self.fq.mulu(pw(a - 1, p0[0]), pw(b, p0[1])), pw(c, p0[2])),
                );
                grad[0] = ((grad[0] as u64 + t as u64) % self.q as u64) as u32;
            }
            if b > 0 {
                let t = self.fq.mulu(
                    self.fq.mulu(co, b as u32),
                    self.fq.mulu(self.fq.mulu(pw(a, p0[0]), pw(b - 1, p0[1])), pw(c, p0[2])),
                );
                grad[1] = ((grad[1] as u64 + t as u64) % self.q as u64) as u32;
            }
            if c > 0 {
                let t = self.fq.mulu(
                    self.fq.mulu(co, c as u32),
                    self.fq.mulu(self.fq.mulu(pw(a, p0[0]), pw(b, p0[1])), pw(c - 1, p0[2])),
                );
                grad[2] = ((grad[2] as u64 + t as u64) % self.q as u64) as u32;
            }
        }
        assert!(grad != [0, 0, 0], "singular base point");
        // tangent vector tau: grad . tau = 0, tau not proportional to P0
        let tau = {
            let ns = self.nullspace(vec![grad.to_vec()], 3);
            let mut pick = None;
            for v in &ns {
                let n = self.normalize_point([v[0], v[1], v[2]]);
                if n != p0 {
                    pick = Some([v[0], v[1], v[2]]);
                    break;
                }
            }
            pick.expect("tangent direction")
        };
        // conics e with e(P0)=0 and grad e(P0) . tau = 0
        let m2 = &self.mons[2];
        let mut r1 = vec![0u32; 6];
        let mut r2 = vec![0u32; 6];
        for (i, &(a, b, c)) in m2.iter().enumerate() {
            r1[i] = self.p0_vals[2][i];
            let pw = |e: u8, x: u32| self.fq.powu(x, e as u64);
            let mut d = 0u64;
            let parts = [(a, 0usize), (b, 1), (c, 2)];
            for &(e, v) in &parts {
                if e == 0 {
                    continue;
                }
                let ex = [a, b, c];
                let mut t = self.fq.mulu(e as u32, 1);
                for w in 0..3 {
                    let ee = if w == v { ex[w] - 1 } else { ex[w] };
                    t = self.fq.mulu(t, pw(ee, p0[w]));
                }
                t = self.fq.mulu(t, tau[v]);
                d += t as u64;
            }
            r2[i] = (d % self.q as u64) as u32;
        }
        let ns = self.nullspace(vec![r1, r2], 6);
        assert_eq!(ns.len(), 4);
        self.cons2p0 = Gen { k: 2, rows: ns };
        self.setup_cost = self.counter();
        self.fq.c_mul.set(0);
        self.fq.c_inv.set(0);
        self.fq.c_add.set(0);
    }

    // ---------------------------------------------------------------------
    // classes
    // ---------------------------------------------------------------------

    /// The zero class `Exc(P0)`.
    pub fn zero(&self) -> PicClass {
        PicClass::Exc(self.p0)
    }

    /// Is `x` the zero class?
    pub fn is_zero(&self, x: &PicClass) -> bool {
        matches!(x, PicClass::Exc(p) if *p == self.p0)
    }

    /// The exceptional class `K - P - D_inf = [P0 - P]`.
    pub fn exceptional(&self, p: Pt) -> PicClass {
        PicClass::Exc(self.normalize_point(p))
    }

    /// The class of the point difference `[P - P0] = -Exc(P)`.
    pub fn point_class(&self, p: Pt) -> PicClass {
        self.neg(&self.exceptional(p))
    }

    /// Canonical class from three independent conics through a non-collinear
    /// effective degree-3 divisor.
    pub fn class_from_conics(&self, rows: Vec<Vec<u32>>) -> PicClass {
        let c0 = self.counter();
        let mut r = rows;
        let n = self.dim[2];
        let piv = self.rref(&mut r, n);
        assert_eq!(piv.len(), 3, "conic space of a degree-3 divisor must have dimension 3");
        let mut m = [[0u32; 6]; 3];
        for i in 0..3 {
            m[i].copy_from_slice(&r[i]);
        }
        let c1 = self.counter().since(&c0);
        self.bump(|s| {
            s.norm_mul += c1.mul;
            s.norm_inv += c1.inv;
        });
        PicClass::Typ(m)
    }

    /// Generating space of an effective degree-3 divisor in the class
    /// `x + D_inf`. `variant` picks the line section used for an exceptional
    /// class (any value gives a valid representative).
    pub fn gen_of_with(&self, x: &PicClass, variant: u32) -> Gen {
        match x {
            PicClass::Typ(m) => Gen {
                k: 2,
                rows: m.iter().map(|r| r.to_vec()).collect(),
            },
            PicClass::Exc(p) => {
                self.bump(|s| s.exc_operands += 1);
                let vals = self.std_values(1, *p);
                let ls = self.nullspace(vec![vals], 3);
                // l = l0 + variant * l1
                let mut l = ls[0].clone();
                if variant != 0 {
                    for i in 0..3 {
                        let t = self.fq.mul(variant, ls[1][i]);
                        l[i] = self.fq.add(l[i], t);
                    }
                }
                let cubics = self.colon(&l, 1, &ls, 1, 3);
                debug_assert_eq!(cubics.len(), self.dim[3] - 3);
                let mut rows = cubics;
                let n = self.dim[3];
                self.rref(&mut rows, n);
                assert_eq!(rows.len(), 7, "cubics through a line residual");
                Gen { k: 3, rows }
            }
        }
    }

    /// [`gen_of_with`](Self::gen_of_with) with the default line section.
    pub fn gen_of(&self, x: &PicClass) -> Gen {
        self.gen_of_with(x, 0)
    }

    /// Group addition.
    pub fn add(&self, x: &PicClass, y: &PicClass) -> PicClass {
        self.add_with(x, y, 0, 0)
    }

    /// Group addition with explicit line-section variants for exceptional
    /// operands (results do not depend on them).
    pub fn add_with(&self, x: &PicClass, y: &PicClass, vx: u32, vy: u32) -> PicClass {
        if self.is_zero(x) {
            return y.clone();
        }
        if self.is_zero(y) {
            return x.clone();
        }
        let gx = self.gen_of_with(x, vx);
        let z6 = if x == y && vx == vy && gx.k == 2 {
            // doubling: the 6 products q_i q_j of a conic space span only a
            // 6-dimensional space of quartics (8 are needed), so go straight
            // to degree 5 (designed path, not counted as a lift)
            let g2 = self.gen_mul(&gx, &gx);
            let r1 = Gen {
                k: 1,
                rows: (0..3).map(|i| self.unit(1, i)).collect(),
            };
            self.gen_mul(&g2, &r1)
        } else {
            let gy = self.gen_of_with(y, vy);
            self.gen_mul(&gx, &gy)
        };
        self.reduce_deg6(&z6)
    }

    /// Doubling.
    pub fn double(&self, x: &PicClass) -> PicClass {
        self.add(x, x)
    }

    /// Subtraction.
    pub fn sub(&self, x: &PicClass, y: &PicClass) -> PicClass {
        let ny = self.neg(y);
        self.add(x, &ny)
    }

    /// Scalar multiplication (left-to-right double and add).
    pub fn mul_scalar(&self, x: &PicClass, k: u128) -> PicClass {
        let mut r = self.zero();
        if k == 0 {
            return r;
        }
        let top = 127 - k.leading_zeros();
        for i in (0..=top).rev() {
            r = self.double(&r);
            if (k >> i) & 1 == 1 {
                r = self.add(&r, x);
            }
        }
        r
    }

    /// Class (in `Pic^0`) of an effective divisor `Z` of degree 6 minus
    /// `2 D_inf`: the reduction core of `add`. `z6` must generate the ideal of
    /// `Z` (e.g. a product of two degree-3 generating spaces).
    pub fn reduce_deg6(&self, z6: &Gen) -> PicClass {
        self.reduce_deg6_aux(z6, 0)
    }

    /// [`reduce_deg6`](Self::reduce_deg6) using the `aux`-th (0, 1 or 2) echelon
    /// cubic through `E7` as the auxiliary cubic (the result does not depend on
    /// it; exposed so that the independence is tested).
    pub fn reduce_deg6_aux(&self, z6: &Gen, aux: usize) -> PicClass {
        self.bump(|s| s.ops += 1);
        let (k, w) = self.complete(z6.clone(), 6);
        let u4 = self.forms_through(&w, k, 3);
        assert_eq!(u4.len(), 4, "cubics through a degree-6 divisor");
        let in_supp = w.rows.iter().all(|r| self.eval_p0(k, r) == 0);
        let mut e7: Option<(usize, Quot)> = None;
        let v3: Vec<Vec<u32>> = if !in_supp {
            let vals: Vec<u32> = u4.iter().map(|u| self.eval_p0(3, u)).collect();
            let ker = self.nullspace(vec![vals], 4);
            ker.iter()
                .map(|c| {
                    let mut v = vec![0u32; self.dim[3]];
                    for (j, &cj) in c.iter().enumerate() {
                        if cj != 0 {
                            for (t, &ut) in u4[j].iter().enumerate() {
                                if ut != 0 {
                                    let m = self.fq.mul(cj, ut);
                                    v[t] = self.fq.add(v[t], m);
                                }
                            }
                        }
                    }
                    v
                })
                .collect()
        } else {
            self.bump(|s| s.p0_in_support += 1);
            let g = self.gen_mul(&Gen { k, rows: w.rows.clone() }, &self.lines_p0);
            let (k2, w2) = self.complete(g, 7);
            let v = self.forms_through(&w2, k2, 3);
            e7 = Some((k2, w2));
            v
        };
        assert_eq!(v3.len(), 3, "cubics through E7");
        let v3q = self.quot_from(v3, self.dim[3]);
        assert_eq!(v3q.rows.len(), 3);
        let conics = self.forms_through(&v3q, 3, 2);
        if conics.len() == 1 {
            // E7 ~ 2K - P
            self.bump(|s| s.exc_results += 1);
            let (k2, w2) = match e7 {
                Some(x) => x,
                None => {
                    let g = self.gen_mul(&Gen { k, rows: w.rows.clone() }, &self.lines_p0);
                    self.complete(g, 7)
                }
            };
            let ls = self.colon(&conics[0], 2, &w2.rows, k2, 1);
            assert_eq!(ls.len(), 2, "lines through the residual point");
            let p = self.cross(&ls[0], &ls[1]);
            let p = self.normalize_counted(p);
            debug_assert_eq!(self.eval_f(p), 0);
            return PicClass::Exc(p);
        }
        assert_eq!(conics.len(), 0, "more than one conic through E7");
        let c = v3q.rows[aux].clone();
        let others: Vec<Vec<u32>> = (0..3).filter(|&i| i != aux).map(|i| v3q.rows[i].clone()).collect();
        let qp = self.colon(&c, 3, &others, 3, 2);
        assert_eq!(qp.len(), 1, "conic through R5");
        let q1 = qp[0].clone();
        let cols: Vec<Vec<u32>> = (0..self.dim[2])
            .map(|j| self.mul_forms(3, &c, 2, &self.unit(2, j)))
            .collect();
        let rhs: Vec<Vec<u32>> = others
            .iter()
            .map(|v| self.mul_forms(2, &q1, 3, v))
            .collect();
        let gs = self.solve_multi(&cols, &rhs).expect("g_v exist and are unique");
        let mut rows = vec![q1];
        rows.extend(gs);
        self.class_from_conics(rows)
    }

    /// Cross product of two lines (counted).
    fn cross(&self, a: &[u32], b: &[u32]) -> Pt {
        let m = |x: u32, y: u32| self.fq.mul(x, y);
        let s = |x: u32, y: u32| self.fq.sub(x, y);
        [
            s(m(a[1], b[2]), m(a[2], b[1])),
            s(m(a[2], b[0]), m(a[0], b[2])),
            s(m(a[0], b[1]), m(a[1], b[0])),
        ]
    }

    fn normalize_counted(&self, p: Pt) -> Pt {
        let c0 = self.counter();
        let Some(i) = (0..3).find(|&i| p[i] != 0) else {
            panic!("zero point");
        };
        let iv = self.fq.inv(p[i]);
        let mut o = [0u32; 3];
        for j in 0..3 {
            o[j] = if j == i { 1 } else { self.fq.mul(p[j], iv) };
        }
        let c1 = self.counter().since(&c0);
        self.bump(|s| {
            s.norm_mul += c1.mul;
            s.norm_inv += c1.inv;
        });
        o
    }

    /// Negation.
    pub fn neg(&self, x: &PicClass) -> PicClass {
        self.neg_with(x, 0)
    }

    /// Negation with an explicit line-section variant for an exceptional
    /// operand.
    pub fn neg_with(&self, x: &PicClass, vx: u32) -> PicClass {
        if self.is_zero(x) {
            return x.clone();
        }
        self.bump(|s| s.ops += 1);
        let gx = self.gen_of_with(x, vx);
        let g = self.gen_mul(&gx, &self.cons2p0);
        let (k, w) = self.complete(g, 5);
        let conics = self.forms_through(&w, k, 2);
        match conics.len() {
            1 => {
                let q1 = conics[0].clone();
                let v = self.forms_through(&w, k, 3);
                assert_eq!(v.len(), 5, "cubics through D_x + 2 P0");
                let rows = self.colon(&q1, 2, &v, 3, 2);
                self.class_from_conics(rows)
            }
            2 => {
                // pencil of conics L * l_i through D_x + 2P0: P = l_1 ^ l_2
                self.bump(|s| s.neg_pencil += 1);
                let (q1, q2) = (&conics[0], &conics[1]);
                let mut cols: Vec<Vec<u32>> = Vec::new();
                for j in 0..3 {
                    let u = self.unit(1, j);
                    let col = self.mul_forms(2, q2, 1, &u);
                    cols.push(col.iter().map(|&c| self.fq.neg(c)).collect());
                }
                for j in 0..3 {
                    let u = self.unit(1, j);
                    cols.push(self.mul_forms(2, q1, 1, &u));
                }
                // nullspace of the 10 x 6 matrix with the given columns
                let nr = self.dim[3];
                let mut eqs = vec![vec![0u32; 6]; nr];
                for (j, c) in cols.iter().enumerate() {
                    for r in 0..nr {
                        eqs[r][j] = c[r];
                    }
                }
                let ns = self.nullspace(eqs, 6);
                assert_eq!(ns.len(), 1, "common line factor of a pencil");
                let p = self.cross(&ns[0][0..3], &ns[0][3..6]);
                let p = self.normalize_counted(p);
                debug_assert_eq!(self.eval_f(p), 0);
                PicClass::Exc(p)
            }
            n => panic!("{} conics through D_x + 2 P0", n),
        }
    }

    /// Class equality (canonical forms are unique, so structural equality).
    pub fn eq(&self, x: &PicClass, y: &PicClass) -> bool {
        x == y
    }

    /// All rational points of `C` by brute force (small `q` only).
    pub fn rational_points_bruteforce(&self) -> Vec<Pt> {
        let q = self.q;
        let mut v = Vec::new();
        for x in 0..q {
            for y in 0..q {
                let p = [1, x, y];
                if self.eval_f(p) == 0 {
                    v.push(p);
                }
            }
        }
        for y in 0..q {
            let p = [0, 1, y];
            if self.eval_f(p) == 0 {
                v.push(p);
            }
        }
        if self.eval_f([0, 0, 1]) == 0 {
            v.push([0, 0, 1]);
        }
        v
    }
}

// -------------------------------------------------------------------------
// instance generation (setup; not charged to the group operations)
// -------------------------------------------------------------------------

/// xorshift64* step (the generator named in the W1 protocol).
pub fn xorshift_next(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *s = x;
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

/// The largest prime below `2^k`.
pub fn largest_prime_below_pow2(k: u32) -> u32 {
    let mut n = (1u64 << k) - 1;
    while !is_odd_prime_below_2_31(n) {
        n -= 1;
    }
    n as u32
}

fn pmod_trim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn pmod_mul(q: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y % q) % q;
        }
    }
    pmod_trim(&mut r);
    r
}

fn pinv(q: u64, a: u64) -> u64 {
    let (mut r0, mut r1, mut t0, mut t1) = (q as i128, a as i128, 0i128, 1i128);
    while r1 != 0 {
        let d = r0 / r1;
        (r0, r1) = (r1, r0 - d * r1);
        (t0, t1) = (t1, t0 - d * t1);
    }
    t0.rem_euclid(q as i128) as u64
}

fn pmod_rem(q: u64, a: &[u64], g: &[u64]) -> Vec<u64> {
    let mut r = a.to_vec();
    pmod_trim(&mut r);
    let ig = pinv(q, *g.last().unwrap());
    while r.len() >= g.len() {
        let c = r.last().copied().unwrap() * ig % q;
        let sh = r.len() - g.len();
        for (i, &gi) in g.iter().enumerate() {
            r[sh + i] = (r[sh + i] + q - c * gi % q) % q;
        }
        pmod_trim(&mut r);
    }
    r
}

fn pmod_gcd(q: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    pmod_trim(&mut a);
    pmod_trim(&mut b);
    while !b.is_empty() {
        let r = pmod_rem(q, &a, &b);
        a = b;
        b = r;
    }
    if let Some(&l) = a.last() {
        let il = pinv(q, l);
        for c in a.iter_mut() {
            *c = *c * il % q;
        }
    }
    a
}

fn pmod_powx(q: u64, base: &[u64], mut e: u64, g: &[u64]) -> Vec<u64> {
    let mut r = vec![1u64];
    let mut b = pmod_rem(q, base, g);
    while e > 0 {
        if e & 1 == 1 {
            r = pmod_rem(q, &pmod_mul(q, &r, &b), g);
        }
        b = pmod_rem(q, &pmod_mul(q, &b, &b), g);
        e >>= 1;
    }
    r
}

/// All roots in `F_q` of the polynomial with ascending coefficients `c`
/// (Cantor-Zassenhaus; setup only).
pub fn poly_roots(q: u32, c: &[u32], rng: &mut u64) -> Vec<u32> {
    let q = q as u64;
    let mut g: Vec<u64> = c.iter().map(|&x| x as u64).collect();
    pmod_trim(&mut g);
    if g.len() <= 1 {
        return vec![];
    }
    // g1 = gcd(x^q - x, g)
    let xq = pmod_powx(q, &[0, 1], q, &g);
    let mut diff = xq.clone();
    if diff.len() < 2 {
        diff.resize(2, 0);
    }
    diff[1] = (diff[1] + q - 1) % q;
    pmod_trim(&mut diff);
    let g1 = if diff.is_empty() { g.clone() } else { pmod_gcd(q, &g, &diff) };
    let mut roots = Vec::new();
    let mut stack = vec![g1];
    while let Some(h) = stack.pop() {
        if h.len() <= 1 {
            continue;
        }
        if h.len() == 2 {
            roots.push(((q - h[0]) % q * pinv(q, h[1]) % q) as u32);
            continue;
        }
        loop {
            let a = xorshift_next(rng) % q;
            let t = pmod_powx(q, &[a, 1], (q - 1) / 2, &h);
            let mut t1 = t.clone();
            if t1.is_empty() {
                t1.push(0);
            }
            t1[0] = (t1[0] + q - 1) % q;
            pmod_trim(&mut t1);
            let d = if t1.is_empty() { h.clone() } else { pmod_gcd(q, &h, &t1) };
            if d.len() > 1 && d.len() < h.len() {
                // divide h by d
                let mut quo = vec![0u64; h.len() - d.len() + 1];
                let mut r = h.clone();
                let id = pinv(q, *d.last().unwrap());
                while r.len() >= d.len() {
                    let cf = r.last().copied().unwrap() * id % q;
                    let sh = r.len() - d.len();
                    quo[sh] = cf;
                    for (i, &di) in d.iter().enumerate() {
                        r[sh + i] = (r[sh + i] + q - cf * di % q) % q;
                    }
                    pmod_trim(&mut r);
                }
                stack.push(d);
                stack.push(quo);
                break;
            }
        }
    }
    roots.sort();
    roots
}

/// Find a rational point of the quartic `f` by random search on lines
/// `(1 : y : z)`, `z` a root of `F(1,y,z)` (setup only).
pub fn find_rational_point(q: u32, f: &[u32; 15], rng: &mut u64, max_tries: usize) -> Option<Pt> {
    let m4 = mons_of_degree(4);
    let qq = q as u64;
    for _ in 0..max_tries {
        let y = xorshift_next(rng) % qq;
        // F(1,y,z) = sum_c z^c * (sum coef * y^b)
        let mut coef = vec![0u32; 5];
        for (i, &(_a, b, c)) in m4.iter().enumerate() {
            if f[i] == 0 {
                continue;
            }
            let mut yp = 1u64;
            for _ in 0..b {
                yp = yp * y % qq;
            }
            coef[c as usize] = ((coef[c as usize] as u64 + f[i] as u64 * yp) % qq) as u32;
        }
        let roots = poly_roots(q, &coef, rng);
        if let Some(&z) = roots.first() {
            return Some([1, y as u32, z]);
        }
    }
    None
}


/// First rational point in the canonical order `[1,x,y]` (by `x`, then `y`),
/// then `[0,1,y]`, then `[0,0,1]` (brute force; small `q`).
pub fn first_rational_point_canonical(q: u32, f: &[u32; 15]) -> Option<Pt> {
    let fq = Fq {
        q: q as u64,
        c_mul: Cell::new(0),
        c_inv: Cell::new(0),
        c_add: Cell::new(0),
    };
    let m4 = mons_of_degree(4);
    let eval = |p: Pt| -> bool {
        let mut s = 0u64;
        for (i, &(a, b, c)) in m4.iter().enumerate() {
            let t = fq.mulu(
                fq.mulu(fq.powu(p[0], a as u64), fq.powu(p[1], b as u64)),
                fq.powu(p[2], c as u64),
            );
            s += fq.mulu(t, f[i]) as u64;
        }
        s % q as u64 == 0
    };
    for x in 0..q {
        for y in 0..q {
            if eval([1, x, y]) {
                return Some([1, x, y]);
            }
        }
    }
    for y in 0..q {
        if eval([0, 1, y]) {
            return Some([0, 1, y]);
        }
    }
    if eval([0, 0, 1]) {
        return Some([0, 0, 1]);
    }
    None
}

/// The first smooth quartic with a rational point from the xorshift64* stream
/// seeded at `seed`: coefficients are consecutive outputs mod `q` in the order of
/// `mons_of_degree(4)`; rejected quartics continue the same stream. Returns
/// the coefficients and the base point (the first point of
/// `rational_points_bruteforce` for `q <= 64`, else the first point found by
/// `find_rational_point`).
pub fn next_smooth_quartic_with_point(q: u32, rng: &mut u64) -> ([u32; 15], Pt) {
    loop {
        let mut f = [0u32; 15];
        for c in f.iter_mut() {
            *c = (xorshift_next(rng) % q as u64) as u32;
        }
        if !smoothness_certificate(q, &f) {
            continue;
        }
        if q <= 64 {
            if let Some(p) = first_rational_point_canonical(q, &f) {
                return (f, p);
            }
        } else if let Some(p) = find_rational_point(q, &f, rng, 4 * 64) {
            return (f, p);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::collections::{BTreeMap, HashMap, HashSet};

    // ------------------------------------------------------------------
    // independent small-field helpers (deliberately not using the library's
    // linear algebra or ring tables)
    // ------------------------------------------------------------------

    pub fn pw(q: u64, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        a %= q;
        while e > 0 {
            if e & 1 == 1 {
                r = r * a % q;
            }
            a = a * a % q;
            e >>= 1;
        }
        r
    }

    /// Independent nullspace (plain Gauss-Jordan mod q).
    pub fn nullspace_ind(q: u64, mut rows: Vec<Vec<u64>>, n: usize) -> Vec<Vec<u64>> {
        let mut piv = vec![];
        let mut r = 0;
        for c in 0..n {
            let Some(p) = (r..rows.len()).find(|&i| rows[i][c] % q != 0) else {
                continue;
            };
            rows.swap(r, p);
            let iv = pw(q, rows[r][c], q - 2);
            for x in rows[r].iter_mut() {
                *x = *x * iv % q;
            }
            for i in 0..rows.len() {
                if i != r && rows[i][c] != 0 {
                    let fct = rows[i][c];
                    for j in 0..n {
                        rows[i][j] = (rows[i][j] + q * q - fct * rows[r][j] % q) % q;
                    }
                }
            }
            piv.push(c);
            r += 1;
        }
        let mut out = vec![];
        for f in 0..n {
            if piv.contains(&f) {
                continue;
            }
            let mut v = vec![0u64; n];
            v[f] = 1;
            for (ri, &pc) in piv.iter().enumerate() {
                v[pc] = (q - rows[ri][f]) % q;
            }
            out.push(v);
        }
        out
    }

    #[derive(Clone)]
    pub struct Ext {
        pub q: u64,
        pub k: usize,
        pub m: [u64; 3],
    }
    pub type E = [u32; 3];

    impl Ext {
        pub fn new(q: u32, k: usize) -> Ext {
            let qq = q as u64;
            let m = match k {
                1 => [0, 0, 0],
                2 => {
                    let n = (2..qq).find(|&n| pw(qq, n, (qq - 1) / 2) == qq - 1).unwrap();
                    [qq - n, 0, 0]
                }
                3 => {
                    let mut found = None;
                    'o: for a in 0..qq {
                        for b in 1..qq {
                            if (0..qq).all(|x| (x * x % qq * x + a * x + b) % qq != 0) {
                                found = Some([b, a, 0]);
                                break 'o;
                            }
                        }
                    }
                    found.unwrap()
                }
                _ => panic!(),
            };
            Ext { q: qq, k, m }
        }
        pub fn zero(&self) -> E {
            [0; 3]
        }
        pub fn one(&self) -> E {
            [1, 0, 0]
        }
        pub fn add(&self, a: &E, b: &E) -> E {
            let mut r = [0u32; 3];
            for i in 0..self.k {
                r[i] = ((a[i] as u64 + b[i] as u64) % self.q) as u32;
            }
            r
        }
        pub fn mul(&self, a: &E, b: &E) -> E {
            let k = self.k;
            let mut t = [0u64; 5];
            for i in 0..k {
                for j in 0..k {
                    t[i + j] = (t[i + j] + a[i] as u64 * b[j] as u64) % self.q;
                }
            }
            if k > 1 {
                for d in (k..=2 * k - 2).rev() {
                    let c = t[d];
                    t[d] = 0;
                    for i in 0..k {
                        t[d - k + i] = (t[d - k + i] + (self.q - self.m[i]) % self.q * c) % self.q;
                    }
                }
            }
            let mut r = [0u32; 3];
            for i in 0..k {
                r[i] = t[i] as u32;
            }
            r
        }
        pub fn pow(&self, a: &E, mut e: u64) -> E {
            let mut r = self.one();
            let mut b = *a;
            while e > 0 {
                if e & 1 == 1 {
                    r = self.mul(&r, &b);
                }
                b = self.mul(&b, &b);
                e >>= 1;
            }
            r
        }
        pub fn frob(&self, a: &E) -> E {
            self.pow(a, self.q)
        }
        pub fn from_fq(&self, c: u32) -> E {
            [c, 0, 0]
        }
        pub fn is_zero(&self, a: &E) -> bool {
            a.iter().all(|&x| x == 0)
        }
        pub fn size(&self) -> u32 {
            (self.q as u32).pow(self.k as u32)
        }
        pub fn el(&self, mut i: u32) -> E {
            let mut r = [0u32; 3];
            for d in 0..self.k {
                r[d] = i % self.q as u32;
                i /= self.q as u32;
            }
            r
        }
        pub fn in_fq(&self, a: &E) -> bool {
            a[1] == 0 && a[2] == 0
        }
    }

    pub type EP = [E; 3];

    /// All normalised points of `C(F_{q^k})` (brute force with Horner in z).
    pub fn ext_points(ext: &Ext, f: &[u32; 15]) -> Vec<EP> {
        let m4 = mons_of_degree(4);
        let zero = ext.zero();
        let one = ext.one();
        let mut out = Vec::new();
        let n = ext.size();
        // x = 1
        for yi in 0..n {
            let y = ext.el(yi);
            let mut yp = [one; 5];
            for b in 1..5 {
                yp[b] = ext.mul(&yp[b - 1], &y);
            }
            let mut a_c = [zero; 5];
            for (i, &(a, b, c)) in m4.iter().enumerate() {
                let _ = a;
                if f[i] != 0 {
                    let t = ext.mul(&ext.from_fq(f[i]), &yp[b as usize]);
                    a_c[c as usize] = ext.add(&a_c[c as usize], &t);
                }
            }
            for zi in 0..n {
                let z = ext.el(zi);
                let mut acc = a_c[4];
                for c in (0..4).rev() {
                    acc = ext.add(&ext.mul(&acc, &z), &a_c[c]);
                }
                if ext.is_zero(&acc) {
                    out.push([one, y, z]);
                }
            }
        }
        // x = 0, y = 1
        for zi in 0..n {
            let z = ext.el(zi);
            let mut acc = zero;
            let mut zp = one;
            let mut zpows = [one; 5];
            for c in 1..5 {
                zp = ext.mul(&zp, &z);
                zpows[c] = zp;
            }
            for (i, &(a, _b, c)) in m4.iter().enumerate() {
                if a == 0 && f[i] != 0 {
                    acc = ext.add(&acc, &ext.mul(&ext.from_fq(f[i]), &zpows[c as usize]));
                }
            }
            if ext.is_zero(&acc) {
                out.push([zero, one, z]);
            }
        }
        // (0:0:1)
        let i00 = m4.iter().position(|&m| m == (0, 0, 4)).unwrap();
        if f[i00] == 0 {
            out.push([zero, zero, one]);
        }
        out
    }

    pub fn jac_order(q: u32, n1: u64, n2: u64, n3: u64) -> i128 {
        let q = q as i128;
        let (t1, t2, t3) = (q + 1 - n1 as i128, q * q + 1 - n2 as i128, q * q * q + 1 - n3 as i128);
        let e1 = t1;
        let e2 = (t1 * t1 - t2) / 2;
        let e3 = (t1 * t1 * t1 - 3 * t1 * t2 + 2 * t3) / 6;
        1 - e1 + e2 - e3 + q * e2 - q * q * e1 + q * q * q
    }

    /// Effective degree-3 F_q-rational divisor, stored as Galois-stable point data.
    #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
    pub struct Div {
        pub rat: Vec<(Pt, u8)>,
        pub pair: Option<[E; 3]>,
        pub triple: Option<[E; 3]>,
    }

    pub struct Enum {
        pub q: u32,
        pub f: [u32; 15],
        pub pts1: Vec<Pt>,
        pub pairs: Vec<EP>,
        pub triples: Vec<EP>,
        pub n1: u64,
        pub n2: u64,
        pub n3: u64,
        pub e2: Ext,
        pub e3: Ext,
        pub jac: i128,
    }

    pub fn enumerate_points(q: u32, f: &[u32; 15]) -> Enum {
        let e1 = Ext::new(q, 1);
        let e2 = Ext::new(q, 2);
        let e3 = Ext::new(q, 3);
        let p1: Vec<EP> = ext_points(&e1, f);
        let pts1: Vec<Pt> = p1.iter().map(|p| [p[0][0], p[1][0], p[2][0]]).collect();
        let p2 = ext_points(&e2, f);
        let p3 = ext_points(&e3, f);
        let canon = |e: &Ext, p: &EP, k: usize| -> EP {
            let mut best = *p;
            let mut cur = *p;
            for _ in 1..k {
                cur = [e.frob(&cur[0]), e.frob(&cur[1]), e.frob(&cur[2])];
                if cur < best {
                    best = cur;
                }
            }
            best
        };
        let mut pairs: Vec<EP> = p2
            .iter()
            .filter(|p| !(e2.in_fq(&p[0]) && e2.in_fq(&p[1]) && e2.in_fq(&p[2])))
            .map(|p| canon(&e2, p, 2))
            .collect();
        pairs.sort();
        pairs.dedup();
        let mut triples: Vec<EP> = p3
            .iter()
            .filter(|p| !(e3.in_fq(&p[0]) && e3.in_fq(&p[1]) && e3.in_fq(&p[2])))
            .map(|p| canon(&e3, p, 3))
            .collect();
        triples.sort();
        triples.dedup();
        let n1 = pts1.len() as u64;
        let n2 = p2.len() as u64;
        let n3 = p3.len() as u64;
        assert_eq!(n2, n1 + 2 * pairs.len() as u64);
        assert_eq!(n3, n1 + 3 * triples.len() as u64);
        let jac = jac_order(q, n1, n2, n3);
        Enum { q, f: *f, pts1, pairs, triples, n1, n2, n3, e2, e3, jac }
    }

    impl Enum {
        pub fn all_divisors(&self) -> Vec<Div> {
            let mut out = Vec::new();
            let n = self.pts1.len();
            for i in 0..n {
                for j in i..n {
                    for k in j..n {
                        let mut m: BTreeMap<Pt, u8> = BTreeMap::new();
                        for &x in &[i, j, k] {
                            *m.entry(self.pts1[x]).or_insert(0) += 1;
                        }
                        out.push(Div { rat: m.into_iter().collect(), pair: None, triple: None });
                    }
                }
            }
            for p in &self.pts1 {
                for pr in &self.pairs {
                    out.push(Div { rat: vec![(*p, 1)], pair: Some(*pr), triple: None });
                }
            }
            for t in &self.triples {
                out.push(Div { rat: vec![], pair: None, triple: Some(*t) });
            }
            out
        }
        /// Zeta-function coefficient of T^3: number of effective rational divisors of degree 3.
        pub fn zeta_c3(&self) -> u64 {
            let (a, b, c) = (self.n1 as u128, self.n2 as u128, self.n3 as u128);
            ((a * a * a + 3 * a * b + 2 * c) / 6) as u64
        }
    }

    /// Jet conditions: rows over the monomials of degree `deg` expressing
    /// `ord_p(form|C) >= m` at the smooth rational point `p`.
    pub fn jet_rows(q: u32, f: &[u32; 15], p: Pt, m: usize, deg: usize) -> Vec<Vec<u64>> {
        let qq = q as u64;
        let m4 = mons_of_degree(4);
        let pp = [p[0] as u64, p[1] as u64, p[2] as u64];
        let mons = mons_of_degree(deg);
        if m == 1 {
            return vec![mons
                .iter()
                .map(|&(a, b, c)| pw(qq, pp[0], a as u64) * pw(qq, pp[1], b as u64) % qq * pw(qq, pp[2], c as u64) % qq)
                .collect()];
        }
        // gradient and Hessian of F at p
        let mut g = [0u64; 3];
        let mut h = [[0u64; 3]; 3];
        for (i, &(a, b, c)) in m4.iter().enumerate() {
            let co = f[i] as u64;
            if co == 0 {
                continue;
            }
            let ex = [a as i64, b as i64, c as i64];
            for u in 0..3 {
                // d/dx_u
                if ex[u] >= 1 {
                    let mut t = co * (ex[u] as u64) % qq;
                    for w in 0..3 {
                        let e = if w == u { ex[w] - 1 } else { ex[w] };
                        t = t * pw(qq, pp[w], e as u64) % qq;
                    }
                    g[u] = (g[u] + t) % qq;
                }
                for v in 0..3 {
                    let mut e2 = ex;
                    let mut coef = co;
                    // d/dx_u d/dx_v
                    let ok1 = e2[u] >= 1;
                    if !ok1 {
                        continue;
                    }
                    coef = coef * (e2[u] as u64) % qq;
                    e2[u] -= 1;
                    if e2[v] < 1 {
                        continue;
                    }
                    coef = coef * (e2[v] as u64) % qq;
                    e2[v] -= 1;
                    let mut t = coef;
                    for w in 0..3 {
                        t = t * pw(qq, pp[w], e2[w] as u64) % qq;
                    }
                    h[u][v] = (h[u][v] + t) % qq;
                }
            }
        }
        assert!(g != [0, 0, 0], "singular point");
        let ns = nullspace_ind(qq, vec![g.to_vec()], 3);
        let tau1 = ns
            .iter()
            .find(|v| {
                // not proportional to p
                let c0 = (v[1] * pp[2] + qq * qq - v[2] * pp[1] % qq) % qq;
                let c1 = (v[2] * pp[0] + qq * qq - v[0] * pp[2] % qq) % qq;
                let c2 = (v[0] * pp[1] + qq * qq - v[1] * pp[0] % qq) % qq;
                (c0, c1, c2) != (0, 0, 0)
            })
            .unwrap()
            .clone();
        let mut hq = 0u64;
        for u in 0..3 {
            for v in 0..3 {
                hq = (hq + tau1[u] * tau1[v] % qq * h[u][v]) % qq;
            }
        }
        // g . tau2 = -hq / 2
        let rhs = (qq - hq * pw(qq, 2, qq - 2) % qq) % qq;
        let i = (0..3).find(|&i| g[i] != 0).unwrap();
        let mut tau2 = [0u64; 3];
        tau2[i] = rhs * pw(qq, g[i], qq - 2) % qq;
        let ser: Vec<[u64; 3]> = (0..3).map(|c| [pp[c], tau1[c], tau2[c]]).collect();
        let mulser = |a: &[u64], b: &[u64]| -> Vec<u64> {
            let mut r = vec![0u64; m];
            for i in 0..m.min(a.len()) {
                for j in 0..m.min(b.len()) {
                    if i + j < m {
                        r[i + j] = (r[i + j] + a[i] * b[j]) % qq;
                    }
                }
            }
            r
        };
        let mut rows = vec![vec![0u64; mons.len()]; m];
        for (mi, &(a, b, c)) in mons.iter().enumerate() {
            let mut s = vec![0u64; m];
            s[0] = 1;
            for (var, e) in [(0usize, a), (1, b), (2, c)] {
                for _ in 0..e {
                    s = mulser(&s, &ser[var]);
                }
            }
            for r in 0..m {
                rows[r][mi] = s[r];
            }
        }
        rows
    }

    /// Linear conditions on the coefficients of a degree-`deg` form for it to
    /// vanish on the divisor `d`.
    pub fn div_conditions(en: &Enum, d: &Div, deg: usize) -> Vec<Vec<u64>> {
        let mons = mons_of_degree(deg);
        let mut rows = Vec::new();
        for &(p, m) in &d.rat {
            rows.extend(jet_rows(en.q, &en.f, p, m as usize, deg));
        }
        let ev = |ext: &Ext, pt: &[E; 3], rows: &mut Vec<Vec<u64>>| {
            let mut comp = vec![vec![0u64; mons.len()]; ext.k];
            for (mi, &(a, b, c)) in mons.iter().enumerate() {
                let mut v = ext.one();
                for _ in 0..a {
                    v = ext.mul(&v, &pt[0]);
                }
                for _ in 0..b {
                    v = ext.mul(&v, &pt[1]);
                }
                for _ in 0..c {
                    v = ext.mul(&v, &pt[2]);
                }
                for t in 0..ext.k {
                    comp[t][mi] = v[t] as u64;
                }
            }
            rows.extend(comp);
        };
        if let Some(pr) = &d.pair {
            ev(&en.e2, pr, &mut rows);
        }
        if let Some(tr) = &d.triple {
            ev(&en.e3, tr, &mut rows);
        }
        rows
    }

    /// F restricted to the line through `p` in direction `u`: polynomial in s.
    pub fn restrict_line(q: u32, f: &[u32; 15], p: Pt, u: [u64; 3]) -> [u64; 5] {
        let qq = q as u64;
        let m4 = mons_of_degree(4);
        let lin = |c: usize| [p[c] as u64, u[c] % qq];
        let mulp = |a: &Vec<u64>, b: &[u64; 2]| -> Vec<u64> {
            let mut r = vec![0u64; a.len() + 1];
            for (i, &x) in a.iter().enumerate() {
                r[i] = (r[i] + x * b[0]) % qq;
                r[i + 1] = (r[i + 1] + x * b[1]) % qq;
            }
            r
        };
        let mut out = [0u64; 5];
        for (i, &(a, b, c)) in m4.iter().enumerate() {
            if f[i] == 0 {
                continue;
            }
            let mut s = vec![1u64];
            for (var, e) in [(0usize, a), (1, b), (2, c)] {
                for _ in 0..e {
                    s = mulp(&s, &lin(var));
                }
            }
            for (j, &x) in s.iter().enumerate() {
                out[j] = (out[j] + x * f[i] as u64) % qq;
            }
        }
        out
    }

    /// Key (canonical class) of an effective degree-3 divisor computed from its
    /// point data only; returns also whether it is collinear.
    pub fn class_of_div(en: &Enum, c: &PlaneQuartic, d: &Div) -> (PicClass, bool) {
        let q = en.q as u64;
        let lines = nullspace_ind(q, div_conditions(en, d, 1), 3);
        if !lines.is_empty() {
            assert_eq!(lines.len(), 1);
            let l = &lines[0];
            // residual point on the line
            let mut found = vec![];
            for &p in &en.pts1 {
                let lv = (l[0] * p[0] as u64 + l[1] * p[1] as u64 + l[2] * p[2] as u64) % q;
                if lv != 0 {
                    continue;
                }
                // another point on the line, not proportional to p
                let basis = nullspace_ind(q, vec![l.clone()], 3);
                let pu = basis
                    .iter()
                    .find(|v| {
                        let pp = [p[0] as u64, p[1] as u64, p[2] as u64];
                        let c0 = (v[1] * pp[2] + q * q - v[2] * pp[1] % q) % q;
                        let c1 = (v[2] * pp[0] + q * q - v[0] * pp[2] % q) % q;
                        let c2 = (v[0] * pp[1] + q * q - v[1] * pp[0] % q) % q;
                        (c0, c1, c2) != (0, 0, 0)
                    })
                    .unwrap();
                let poly = restrict_line(en.q, &en.f, p, [pu[0], pu[1], pu[2]]);
                let ml = poly.iter().position(|&x| x != 0).unwrap() as u8;
                let md = d.rat.iter().find(|(pt, _)| *pt == p).map(|x| x.1).unwrap_or(0);
                assert!(ml >= md, "divisor not contained in line section");
                if ml > md {
                    assert_eq!(ml, md + 1);
                    found.push(p);
                }
            }
            assert_eq!(found.len(), 1, "residual point of a collinear divisor");
            return (c.exceptional(found[0]), true);
        }
        let conics = nullspace_ind(q, div_conditions(en, d, 2), 6);
        assert_eq!(conics.len(), 3);
        let rows: Vec<Vec<u32>> = conics.iter().map(|r| r.iter().map(|&x| x as u32).collect()).collect();
        (c.class_from_conics(rows), false)
    }

    // ------------------------------------------------------------------
    // instances
    // ------------------------------------------------------------------

    pub struct Inst {
        pub name: String,
        pub q: u32,
        pub f: [u32; 15],
        pub p0: Pt,
    }

    pub fn ghs_instances() -> Vec<Inst> {
        let text = include_str!("../../research/ghs_cubic_genus3_build_20261012/results/run-20261013T0320Z.jsonl");
        let m4 = mons_of_degree(4);
        let mut out = Vec::new();
        for line in text.lines() {
            let v: serde_json::Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if v["rec"] != "case" || v["quartic_Fq"].is_null() {
                continue;
            }
            let q = v["q"].as_u64().unwrap() as u32;
            let mut f = [0u32; 15];
            for t in v["quartic_Fq"].as_array().unwrap() {
                let t = t.as_array().unwrap();
                let mon = (
                    t[0].as_u64().unwrap() as u8,
                    t[1].as_u64().unwrap() as u8,
                    t[2].as_u64().unwrap() as u8,
                );
                let idx = m4.iter().position(|&m| m == mon).unwrap();
                f[idx] = (t[3].as_u64().unwrap() % q as u64) as u32;
            }
            let p0 = first_rational_point_canonical(q, &f).expect("GHS quartic has a rational point");
            out.push(Inst {
                name: format!("ghs-q{}-{}", q, v["kind"].as_str().unwrap()),
                q,
                f,
                p0,
            });
        }
        out
    }

    pub fn random_instances(q: u32, count: usize) -> Vec<Inst> {
        let mut rng = 20261018u64 + q as u64;
        (0..count)
            .map(|i| {
                let (f, p0) = next_smooth_quartic_with_point(q, &mut rng);
                Inst { name: format!("rand-q{}-{}", q, i), q, f, p0 }
            })
            .collect()
    }

    pub fn small_instances() -> Vec<Inst> {
        let mut v = ghs_instances();
        for &q in &[5u32, 7, 11, 13] {
            v.extend(random_instances(q, 3));
        }
        v
    }

    pub fn curve_of(i: &Inst) -> PlaneQuartic {
        PlaneQuartic::new(i.q, i.f, i.p0).expect("instance builds")
    }

    /// Everything about an instance used by several tests.
    pub struct Full {
        pub inst: Inst,
        pub c: PlaneQuartic,
        pub en: Enum,
        pub divs: Vec<Div>,
        pub classes: Vec<PicClass>,
        pub collinear: Vec<bool>,
        pub class_of: Vec<PicClass>,
    }

    pub fn full(inst: Inst) -> Full {
        let c = curve_of(&inst);
        let en = enumerate_points(inst.q, &inst.f);
        let divs = en.all_divisors();
        let mut class_of = Vec::new();
        let mut collinear = Vec::new();
        for d in &divs {
            let (cl, col) = class_of_div(&en, &c, d);
            class_of.push(cl);
            collinear.push(col);
        }
        let mut seen: HashSet<PicKey> = HashSet::new();
        let mut classes = Vec::new();
        for cl in &class_of {
            if seen.insert(cl.key()) {
                classes.push(cl.clone());
            }
        }
        Full { inst, c, en, divs, classes, collinear, class_of }
    }

    // ------------------------------------------------------------------
    // V5: smoothness certificate
    // ------------------------------------------------------------------

    fn poly_from_terms(q: u32, terms: &[((u8, u8, u8), u32)]) -> [u32; 15] {
        let m4 = mons_of_degree(4);
        let mut f = [0u32; 15];
        for &(m, c) in terms {
            let i = m4.iter().position(|&x| x == m).unwrap();
            f[i] = (f[i] + c % q) % q;
        }
        f
    }

    #[test]
    fn v5_smoothness_certificate() {
        for inst in small_instances() {
            assert!(smoothness_certificate(inst.q, &inst.f), "{} should be smooth", inst.name);
        }
        for &q in &[5u32, 7, 11, 13, 101] {
            // node at (0:0:1): Z^2 X Y + X^4 + Y^4
            let node = poly_from_terms(q, &[((1, 1, 2), 1), ((4, 0, 0), 1), ((0, 4, 0), 1)]);
            assert!(!smoothness_certificate(q, &node), "node accepted at q={}", q);
            // cusp at (0:0:1): Y^2 Z^2 + X^3 Z + X^4
            let cusp = poly_from_terms(q, &[((0, 2, 2), 1), ((3, 0, 1), 1), ((4, 0, 0), 1)]);
            assert!(!smoothness_certificate(q, &cusp), "cusp accepted at q={}", q);
            // two conics: (X^2+Y^2+Z^2)(X^2 + 2 Y^2 + 3 Z^2) expanded
            let mut tc = vec![];
            let a = [(2u8, 0u8, 0u8, 1u32), (0, 2, 0, 1), (0, 0, 2, 1)];
            let b = [(2u8, 0u8, 0u8, 1u32), (0, 2, 0, 2), (0, 0, 2, 3)];
            for x in a {
                for y in b {
                    tc.push(((x.0 + y.0, x.1 + y.1, x.2 + y.2), x.3 * y.3));
                }
            }
            let two = poly_from_terms(q, &tc);
            // reducible, possibly with intersection points: must be singular (4 nodes over closure)
            assert!(!smoothness_certificate(q, &two), "reducible accepted at q={}", q);
            // quadruple line
            let line4 = poly_from_terms(q, &[((4, 0, 0), 1)]);
            assert!(!smoothness_certificate(q, &line4));
            // Fermat quartic is smooth for q odd, q != 2
            let fermat = poly_from_terms(q, &[((4, 0, 0), 1), ((0, 4, 0), 1), ((0, 0, 4), 1)]);
            assert!(smoothness_certificate(q, &fermat), "Fermat rejected at q={}", q);
        }
    }

    #[test]
    fn constructor_errors() {
        let f = poly_from_terms(7, &[((1, 1, 2), 1), ((4, 0, 0), 1), ((0, 4, 0), 1)]);
        assert_eq!(PlaneQuartic::new(7, f, [0, 0, 1]).err(), Some(PicError::NotSmooth));
        assert_eq!(PlaneQuartic::new(8, f, [0, 0, 1]).err(), Some(PicError::BadField));
        let fermat = poly_from_terms(7, &[((4, 0, 0), 1), ((0, 4, 0), 1), ((0, 0, 4), 1)]);
        assert_eq!(PlaneQuartic::new(7, fermat, [1, 1, 1]).err(), Some(PicError::NoPoint));
    }

    // ------------------------------------------------------------------
    // V1: class count
    // ------------------------------------------------------------------

    #[test]
    fn v1_class_count() {
        let mut rows = vec![];
        for inst in small_instances() {
            let name = inst.name.clone();
            let fu = full(inst);
            let en = &fu.en;
            assert_eq!(fu.divs.len() as u64, en.zeta_c3(), "{}: divisors enumerated vs zeta coefficient", name);
            assert_eq!(
                fu.classes.len() as i128,
                en.jac,
                "{}: distinct keys {} vs #Jac {} (N1,N2,N3 = {},{},{})",
                name,
                fu.classes.len(),
                en.jac,
                en.n1,
                en.n2,
                en.n3
            );
            let exc = fu.classes.iter().filter(|c| matches!(c, PicClass::Exc(_))).count();
            assert_eq!(exc as u64, en.n1, "{}: exceptional classes = #C(F_q)", name);
            rows.push(format!(
                "{} q={} N=({},{},{}) #Jac={} divisors={} keys={} exceptional={}",
                name,
                fu.inst.q,
                en.n1,
                en.n2,
                en.n3,
                en.jac,
                fu.divs.len(),
                fu.classes.len(),
                exc
            ));
        }
        eprintln!("V1 PASS\n{}", rows.join("\n"));
    }

    // ------------------------------------------------------------------
    // V3: key uniqueness against the geometric (extension-field) equality test
    // ------------------------------------------------------------------

    #[test]
    fn v3_key_uniqueness() {
        let mut total_pairs = 0u64;
        for inst in small_instances() {
            let name = inst.name.clone();
            let fu = full(inst);
            // group by key
            let mut by_key: HashMap<PicKey, Vec<usize>> = HashMap::new();
            for (i, cl) in fu.class_of.iter().enumerate() {
                by_key.entry(cl.key()).or_default().push(i);
            }
            // key equal => equal divisors or both collinear with same residual point
            for (_k, idxs) in &by_key {
                if idxs.len() > 1 {
                    assert!(idxs.iter().all(|&i| fu.collinear[i]), "{}: non-collinear divisors share a key", name);
                }
                // all in a group give the same exceptional point (key equality)
            }
            // equal divisors or same residual => key equal (the other direction)
            let mut by_point: HashMap<Pt, Vec<usize>> = HashMap::new();
            for (i, cl) in fu.class_of.iter().enumerate() {
                if let PicClass::Exc(p) = cl {
                    by_point.entry(*p).or_default().push(i);
                    assert!(fu.collinear[i]);
                } else {
                    assert!(!fu.collinear[i]);
                }
            }
            for (_p, idxs) in &by_point {
                let k0 = fu.class_of[idxs[0]].key();
                assert!(idxs.iter().all(|&i| fu.class_of[i].key() == k0));
                total_pairs += (idxs.len() * idxs.len()) as u64;
            }
            // distinct divisors, non-collinear => distinct keys
            let distinct: HashSet<&Div> = fu.divs.iter().collect();
            assert_eq!(distinct.len(), fu.divs.len(), "{}: enumeration produced duplicates", name);
            let noncol = fu.collinear.iter().filter(|&&c| !c).count();
            let typ_keys = by_key.iter().filter(|(k, _)| k.0[0] == 1).count();
            assert_eq!(noncol, typ_keys, "{}", name);
            // every point of C has a collinear divisor (the line section minus the point) hence an Exc key
            assert_eq!(by_point.len() as u64, fu.en.n1, "{}: Exc keys vs rational points", name);
        }
        eprintln!("V3 PASS ({} same-residual pairs checked)", total_pairs);
    }

    // ------------------------------------------------------------------
    // V2: group law
    // ------------------------------------------------------------------

    /// Independent second reduction path: cubic residual `R6` of `Z6`, then the
    /// cubic through `R6 + D_inf`, then the residual. Returns `None` on a
    /// degenerate case (not a failure; counted by the caller).
    pub fn add_second_path(c: &PlaneQuartic, x: &PicClass, y: &PicClass) -> Option<PicClass> {
        if c.is_zero(x) || c.is_zero(y) {
            return None;
        }
        let gx = c.gen_of_with(x, 3);
        let gy = c.gen_of_with(y, 5);
        let z6 = c.gen_mul(&gx, &gy);
        let (k, w) = c.complete(z6, 6);
        let u4 = c.forms_through(&w, k, 3);
        assert_eq!(u4.len(), 4);
        // a cubic c0 through Z6 (second basis vector: different from the first path's choice)
        let c0 = u4[1].clone();
        // generating space of R6 = div(c0) - Z6: cubics e with e * w in c0 * R_k
        let r6 = c.colon(&c0, 3, &w.rows, k, 3);
        if r6.len() != 4 {
            return None;
        }
        let dinf = c.gen_of_with(&c.zero(), 0);
        let g = c.gen_mul(&Gen { k: 3, rows: r6 }, &dinf);
        let (k2, w2) = c.complete(g, 9);
        let c2s = c.forms_through(&w2, k2, 3);
        if c2s.len() != 1 {
            return None;
        }
        let rows = c.colon(&c2s[0], 3, &w2.rows, k2, 2);
        if rows.len() != 3 {
            return None;
        }
        // must be a non-collinear divisor to be typical
        Some(c.class_from_conics(rows))
    }

    #[test]
    fn v2_group_law() {
        let mut report = vec![];
        let mut second_ok = 0u64;
        let mut second_skip = 0u64;
        for inst in small_instances() {
            let name = inst.name.clone();
            let q = inst.q;
            let fu = full(inst);
            let c = &fu.c;
            let n = fu.classes.len();
            let keyset: HashSet<PicKey> = fu.classes.iter().map(|x| x.key()).collect();
            let zero = c.zero();
            assert!(keyset.contains(&zero.key()));
            let mut rng = 0x9E3779B97F4A7C15u64 ^ (q as u64);
            let mut pick = |rng: &mut u64| -> &PicClass { &fu.classes[(xorshift_next(rng) % n as u64) as usize] };
            let mut n_exc_ops = 0u64;
            for t in 0..300 {
                let (a, b, d) = (pick(&mut rng).clone(), pick(&mut rng).clone(), pick(&mut rng).clone());
                let ab = c.add(&a, &b);
                let ba = c.add(&b, &a);
                assert_eq!(ab, ba, "{}: commutativity", name);
                assert!(keyset.contains(&ab.key()), "{}: closure", name);
                let l = c.add(&ab, &d);
                let r = c.add(&a, &c.add(&b, &d));
                assert_eq!(l, r, "{}: associativity at trial {}", name, t);
                // inverse
                let na = c.neg(&a);
                assert!(keyset.contains(&na.key()));
                assert_eq!(c.add(&a, &na), zero, "{}: x + (-x)", name);
                assert_eq!(c.neg(&na), a, "{}: -(-x)", name);
                assert_eq!(c.sub(&ab, &b), a, "{}: (a+b)-b", name);
                // zero
                assert_eq!(c.add(&a, &zero), a);
                // doubling vs add
                assert_eq!(c.double(&a), c.add(&a, &a));
                // representative independence for exceptional operands
                let (vx, vy) = ((xorshift_next(&mut rng) % q as u64) as u32, (xorshift_next(&mut rng) % q as u64) as u32);
                assert_eq!(c.add_with(&a, &b, vx, vy), ab, "{}: representative independence", name);
                assert_eq!(c.neg_with(&a, vx), na, "{}: neg representative independence", name);
                if matches!(a, PicClass::Exc(_)) || matches!(b, PicClass::Exc(_)) {
                    n_exc_ops += 1;
                }
                // auxiliary cubic independence (all three echelon cubics give the same class)
                if !c.is_zero(&a) && !c.is_zero(&b) {
                    let z6 = c.gen_mul(&c.gen_of_with(&a, 0), &c.gen_of_with(&b, 0));
                    for aux in 1..3 {
                        assert_eq!(c.reduce_deg6_aux(&z6, aux), ab, "{}: auxiliary cubic {} changes the class", name, aux);
                    }
                }
                // second reduction path
                match add_second_path(c, &a, &b) {
                    Some(s2) => {
                        if matches!(ab, PicClass::Typ(_)) {
                            assert_eq!(s2, ab, "{}: second reduction path disagrees", name);
                            second_ok += 1;
                        } else {
                            second_skip += 1;
                        }
                    }
                    None => second_skip += 1,
                }
            }
            // #Jac * x = 0
            let jac = fu.en.jac as u128;
            for _ in 0..100 {
                let a = pick(&mut rng).clone();
                assert_eq!(c.mul_scalar(&a, jac), zero, "{}: #Jac * x != 0", name);
                let k = 1 + xorshift_next(&mut rng) % 9;
                let mut acc = zero.clone();
                for _ in 0..k {
                    acc = c.add(&acc, &a);
                }
                assert_eq!(c.mul_scalar(&a, k as u128), acc);
            }
            // exact order of the group is not smaller for a generic element: sanity (some element has order > 1)
            assert!(fu.classes.iter().any(|x| *x != zero));
            // line sections with four distinct rational points: sum of point classes constant
            let mut sums: HashSet<PicKey> = HashSet::new();
            let mut nlines = 0u64;
            let qn = q as u64;
            let mut lines: Vec<[u64; 3]> = vec![];
            for a in 0..qn {
                for b in 0..qn {
                    lines.push([1, a, b]);
                }
            }
            for b in 0..qn {
                lines.push([0, 1, b]);
            }
            lines.push([0, 0, 1]);
            for l in &lines {
                let on: Vec<Pt> = fu
                    .en
                    .pts1
                    .iter()
                    .filter(|p| (l[0] * p[0] as u64 + l[1] * p[1] as u64 + l[2] * p[2] as u64) % qn == 0)
                    .cloned()
                    .collect();
                if on.len() == 4 {
                    // four distinct rational points on the line: they are all of L cap C
                    let mut s = zero.clone();
                    for p in &on {
                        s = c.add(&s, &c.point_class(*p));
                    }
                    sums.insert(s.key());
                    nlines += 1;
                }
            }
            assert!(sums.len() <= 1, "{}: line sections not all equivalent", name);
            // point classes: [P - P0] + Exc(P) = 0
            for p in &fu.en.pts1 {
                assert_eq!(c.add(&c.point_class(*p), &c.exceptional(*p)), zero);
            }
            report.push(format!(
                "{}: #Jac={} classes={} exceptional-operand trials={} lines-with-4-points={}",
                name, fu.en.jac, n, n_exc_ops, nlines
            ));
        }
        eprintln!(
            "V2 PASS (second path agrees on {} generic results, {} skipped)\n{}",
            second_ok,
            second_skip,
            report.join("\n")
        );
    }


    #[test]
    fn v2_exceptional_and_paths() {
        let mut tot = PicStats::default();
        let mut exc_sum_results = 0u64;
        let mut pairs_checked = 0u64;
        for inst in small_instances() {
            let name = inst.name.clone();
            let q = inst.q;
            let fu = full(inst);
            let c = &fu.c;
            let n = fu.classes.len();
            let keyset: HashSet<PicKey> = fu.classes.iter().map(|x| x.key()).collect();
            let zero = c.zero();
            c.reset_counter();
            for x in &fu.classes {
                let nx = c.neg(x);
                assert!(keyset.contains(&nx.key()), "{}: neg closure", name);
                assert_eq!(c.neg(&nx), *x, "{}: -(-x)", name);
                assert_eq!(c.add(x, &nx), zero, "{}: x+(-x)", name);
            }
            let mut rng = 77u64 + q as u64;
            let excs: Vec<&PicClass> = fu.classes.iter().filter(|x| matches!(x, PicClass::Exc(_))).collect();
            for e in &excs {
                for _ in 0..60.min(n) {
                    let y = &fu.classes[(xorshift_next(&mut rng) % n as u64) as usize];
                    let z = &fu.classes[(xorshift_next(&mut rng) % n as u64) as usize];
                    let s = c.add(e, y);
                    assert!(keyset.contains(&s.key()));
                    assert_eq!(s, c.add(y, e));
                    assert_eq!(c.add(&s, z), c.add(e, &c.add(y, z)), "{}: assoc with exceptional", name);
                    if matches!(s, PicClass::Exc(_)) {
                        exc_sum_results += 1;
                    }
                    let (vx, vy) = ((xorshift_next(&mut rng) % q as u64) as u32, (xorshift_next(&mut rng) % q as u64) as u32);
                    assert_eq!(c.add_with(e, y, vx, vy), s);
                    pairs_checked += 1;
                }
                for e2 in &excs {
                    let s = c.add(e, e2);
                    assert!(keyset.contains(&s.key()));
                    assert_eq!(s, c.add(e2, e));
                    pairs_checked += 1;
                }
            }
            let st = c.stats();
            tot.ops += st.ops;
            tot.lifts += st.lifts;
            tot.p0_in_support += st.p0_in_support;
            tot.exc_results += st.exc_results;
            tot.neg_pencil += st.neg_pencil;
            tot.exc_operands += st.exc_operands;
        }
        eprintln!("V2 exceptional PASS: pairs {} exceptional results {} ; path counters {:?}", pairs_checked, exc_sum_results, tot);
        assert!(tot.exc_results > 0 && tot.neg_pencil > 0 && tot.p0_in_support > 0 && tot.exc_operands > 0);
    }

    #[test]
    fn v2_cayley_table_q5() {
        let mut insts = vec![random_instances(5, 1).pop().unwrap()];
        insts.push(ghs_instances().into_iter().find(|i| i.q == 5 && i.name.ends_with("split")).unwrap());
        for inst in insts {
            cayley_table_check(inst);
        }
    }

    fn cayley_table_check(inst: Inst) {
        let fu = full(inst);
        let c = &fu.c;
        let n = fu.classes.len();
        let idx: HashMap<PicKey, usize> = fu.classes.iter().enumerate().map(|(i, x)| (x.key(), i)).collect();
        let mut t = vec![vec![0usize; n]; n];
        for i in 0..n {
            for j in 0..n {
                let s = c.add(&fu.classes[i], &fu.classes[j]);
                t[i][j] = *idx.get(&s.key()).expect("closure");
            }
        }
        let z = idx[&c.zero().key()];
        for i in 0..n {
            assert_eq!(t[i][z], i);
            assert_eq!(t[z][i], i);
            let mut seen = vec![false; n];
            for j in 0..n {
                assert_eq!(t[i][j], t[j][i], "commutativity");
                assert!(!seen[t[i][j]], "row {} is not a permutation", i);
                seen[t[i][j]] = true;
            }
            assert!((0..n).any(|j| t[i][j] == z), "inverse");
        }
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    assert_eq!(t[t[i][j]][k], t[i][t[j][k]], "associativity ({},{},{})", i, j, k);
                }
            }
        }
        // order statistics
        let mut orders = BTreeMap::new();
        for i in 0..n {
            let mut o = 1;
            let mut cur = i;
            while cur != z {
                cur = t[cur][i];
                o += 1;
            }
            *orders.entry(o).or_insert(0) += 1;
        }
        eprintln!("Cayley table {}: #Jac={} full associativity ({} triples) PASS; element orders {:?}", fu.inst.name, n, n * n * n, orders);
    }

    #[test]
    fn scaling_group_law_sanity() {
        // no brute-force class count is possible here; check the group axioms on
        // classes built from random rational points, at q up to 2^31
        for &k in &[8u32, 12, 16, 24, 31] {
            let q = largest_prime_below_pow2(k);
            let mut rng = 20261018u64 + k as u64;
            let (f, p0) = next_smooth_quartic_with_point(q, &mut rng);
            let c = PlaneQuartic::new(q, f, p0).unwrap();
            let mut prng = 0xC0FFEEu64 + k as u64;
            let mut rand_class = |prng: &mut u64| -> PicClass {
                let mut x = c.zero();
                for _ in 0..4 {
                    let p = find_rational_point(q, &f, prng, 100_000).unwrap();
                    x = c.add(&x, &c.point_class(p));
                }
                x
            };
            for _ in 0..25 {
                let (a, b, d) = (rand_class(&mut prng), rand_class(&mut prng), rand_class(&mut prng));
                assert_eq!(c.add(&a, &b), c.add(&b, &a));
                assert_eq!(c.add(&c.add(&a, &b), &d), c.add(&a, &c.add(&b, &d)), "assoc q={}", q);
                assert_eq!(c.add(&a, &c.neg(&a)), c.zero());
                assert_eq!(c.sub(&c.add(&a, &b), &b), a);
                assert_eq!(c.mul_scalar(&a, 7), {
                    let a2 = c.double(&a);
                    let a4 = c.double(&a2);
                    c.add(&c.add(&a4, &a2), &a)
                });
                assert_eq!(c.mul_scalar(&c.add(&a, &b), 12345), c.add(&c.mul_scalar(&a, 12345), &c.mul_scalar(&b, 12345)));
            }
            let p = find_rational_point(q, &f, &mut prng, 100_000).unwrap();
            assert_eq!(c.add(&c.point_class(p), &c.exceptional(p)), c.zero());
        }
    }

    // ------------------------------------------------------------------
    // M1-M3: operation cost sweep (ignored by default; run explicitly)
    // ------------------------------------------------------------------

    fn median_iqr(v: &mut Vec<f64>) -> (f64, f64, f64) {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = v.len();
        let at = |p: f64| -> f64 {
            let x = p * (n - 1) as f64;
            let lo = x.floor() as usize;
            let hi = x.ceil() as usize;
            v[lo] + (v[hi] - v[lo]) * (x - lo as f64)
        };
        (at(0.5), at(0.25), at(0.75))
    }

    /// Time one `mul` (one 64-bit modular reduction per product, dependent
    /// chain) against one `inv` (extended Euclid) at this `q`, in nanoseconds,
    /// median of 9 repetitions.
    fn inv_mul_ratio(q: u32) -> (f64, f64) {
        let qq = q as u64;
        let fq = Fq { q: qq, c_mul: Cell::new(0), c_inv: Cell::new(0), c_add: Cell::new(0) };
        let med = |mut v: Vec<f64>| -> f64 {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        };
        let mut tm = vec![];
        let mut ti = vec![];
        for rep in 0..9u64 {
            let reps = 2_000_000u64;
            let b = std::hint::black_box(qq - 2 - rep);
            let mut x = std::hint::black_box(123456789u64 % qq);
            let t0 = std::time::Instant::now();
            for _ in 0..reps {
                x = x * b % qq;
            }
            tm.push(t0.elapsed().as_nanos() as f64 / reps as f64);
            std::hint::black_box(x);
            let reps2 = 200_000u64;
            let mut y = std::hint::black_box(987654321u32 % q);
            let t1 = std::time::Instant::now();
            for _ in 0..reps2 {
                // chain: invert, then perturb to stay nonzero
                y = fq.invu(y);
                y = if y + 1 >= q { 1 } else { y + 1 };
            }
            ti.push(t1.elapsed().as_nanos() as f64 / reps2 as f64);
            std::hint::black_box(y);
        }
        (med(tm), med(ti))
    }

    #[test]
    #[ignore]
    fn m1_cost_sweep() {
        use std::io::Write;
        let out_dir = std::env::var("W1_OUT").unwrap_or_else(|_| "research/quartic_pic_canonical_20261018/results".to_string());
        std::fs::create_dir_all(&out_dir).unwrap();
        let ks: Vec<u32> = match std::env::var("W1_KS") {
            Ok(v) => v.split(',').map(|x| x.parse().unwrap()).collect(),
            Err(_) => vec![8, 12, 16, 20, 24, 28, 31],
        };
        let npairs: usize = std::env::var("W1_PAIRS").ok().and_then(|v| v.parse().ok()).unwrap_or(200);
        let mut raw = std::fs::File::create(format!("{}/m1_raw.jsonl", out_dir)).unwrap();
        let mut summ = std::fs::File::create(format!("{}/m1_summary.jsonl", out_dir)).unwrap();
        // (op, log2 q, median mul, median priced mul-equivalents, median ns)
        let mut series: Vec<(String, f64, f64, f64, f64)> = Vec::new();
        for &k in &ks {
            let q = largest_prime_below_pow2(k);
            // ---- M3 setup ----
            let t_setup = std::time::Instant::now();
            let mut rng = 20261018u64 + k as u64;
            let t_pt = std::time::Instant::now();
            let (f, p0) = next_smooth_quartic_with_point(q, &mut rng);
            let ns_find = t_pt.elapsed().as_nanos();
            let t_c = std::time::Instant::now();
            let ok_smooth = smoothness_certificate(q, &f);
            let ns_cert = t_c.elapsed().as_nanos();
            assert!(ok_smooth);
            let t_n = std::time::Instant::now();
            let c = PlaneQuartic::new(q, f, p0).unwrap();
            let ns_new = t_n.elapsed().as_nanos();
            let setup = c.setup_cost();
            let ns_setup_total = t_setup.elapsed().as_nanos();
            let (tmul, tinv) = inv_mul_ratio(q);
            let inv_equiv = tinv / tmul;
            writeln!(
                summ,
                "{{\"rec\":\"setup\",\"k\":{},\"q\":{},\"f\":{:?},\"p0\":{:?},\"ns_find_instance_incl_smoothness\":{},\"ns_smoothness_cert\":{},\"ns_new_incl_base_point\":{},\"ns_setup_total\":{},\"setup_mul\":{},\"setup_inv\":{},\"ns_per_mul\":{:.3},\"ns_per_inv\":{:.3},\"inv_mul_equiv\":{:.3}}}",
                k, q, f, p0, ns_find, ns_cert, ns_new, ns_setup_total, setup.mul, setup.inv, tmul, tinv, inv_equiv
            )
            .unwrap();
            // ---- classes from random rational points ----
            let mut prng = 0xDEADBEEFu64 ^ (k as u64 * 7919);
            let mut random_pt = |prng: &mut u64| -> Pt {
                find_rational_point(q, &f, prng, 100_000).expect("rational point")
            };
            let mut typical = |prng: &mut u64| -> PicClass {
                let mut x = c.zero();
                for _ in 0..6 {
                    let p = random_pt(prng);
                    x = c.add(&x, &c.point_class(p));
                }
                x
            };
            let mut xs: Vec<PicClass> = Vec::new();
            let mut ys: Vec<PicClass> = Vec::new();
            let mut es: Vec<PicClass> = Vec::new();
            while xs.len() < npairs {
                let x = typical(&mut prng);
                if matches!(x, PicClass::Typ(_)) {
                    xs.push(x);
                }
            }
            while ys.len() < npairs {
                let y = typical(&mut prng);
                if matches!(y, PicClass::Typ(_)) {
                    ys.push(y);
                }
            }
            for _ in 0..npairs {
                es.push(c.exceptional(random_pt(&mut prng)));
            }
            let scalars: Vec<u64> = (0..npairs).map(|_| xorshift_next(&mut prng) | (1u64 << 63)).collect();
            // helper to time and record one operation
            let mut record = |op: &str, i: usize, f: &mut dyn FnMut() -> PicClass, raw: &mut std::fs::File| -> (f64, f64, f64, f64, f64, PicStats) {
                c.reset_counter();
                let t = std::time::Instant::now();
                let r = f();
                let ns = t.elapsed().as_nanos() as f64;
                let cnt = c.counter();
                let st = c.stats();
                std::hint::black_box(&r);
                writeln!(
                    raw,
                    "{{\"k\":{},\"q\":{},\"op\":\"{}\",\"i\":{},\"mul\":{},\"inv\":{},\"add\":{},\"ns\":{},\"lifts\":{},\"p0_in_support\":{},\"exc_results\":{},\"neg_pencil\":{},\"exc_operands\":{},\"norm_mul\":{},\"norm_inv\":{},\"ops\":{}}}",
                    k, q, op, i, cnt.mul, cnt.inv, cnt.add, ns as u64, st.lifts, st.p0_in_support, st.exc_results, st.neg_pencil, st.exc_operands, st.norm_mul, st.norm_inv, st.ops
                )
                .unwrap();
                (cnt.mul as f64, cnt.inv as f64, cnt.add as f64, ns, (st.lifts > 0) as u8 as f64, st)
            };
            let mut results: Vec<(String, Vec<(f64, f64, f64, f64, f64, PicStats)>)> = Vec::new();
            let mut run_op = |name: &str, f: &mut dyn FnMut(usize) -> PicClass, raw: &mut std::fs::File| {
                let mut v = Vec::new();
                for i in 0..npairs {
                    let mut g = || f(i);
                    v.push(record(name, i, &mut g, raw));
                }
                results.push((name.to_string(), v));
            };
            run_op("add_generic", &mut |i| c.add(&xs[i], &ys[i]), &mut raw);
            run_op("add_exceptional_operand", &mut |i| c.add(&es[i], &ys[i]), &mut raw);
            run_op("double", &mut |i| c.double(&xs[i]), &mut raw);
            run_op("negate", &mut |i| c.neg(&xs[i]), &mut raw);
            run_op(
                "key_encoding",
                &mut |i| {
                    // re-encode a scrambled conic basis (random invertible 3x3 combination)
                    let PicClass::Typ(m) = &xs[i] else { unreachable!() };
                    let mut st = 0xABCDEFu64 ^ i as u64;
                    let rows: Vec<Vec<u32>> = loop {
                        let mut mix = [[0u32; 3]; 3];
                        for r in mix.iter_mut() {
                            for e in r.iter_mut() {
                                *e = (xorshift_next(&mut st) % q as u64) as u32;
                            }
                        }
                        let mut out = vec![vec![0u32; 6]; 3];
                        for a in 0..3 {
                            for b in 0..3 {
                                for t in 0..6 {
                                    out[a][t] = ((out[a][t] as u64 + mix[a][b] as u64 * m[b][t] as u64) % q as u64) as u32;
                                }
                            }
                        }
                        let det = {
                            let m = &mix;
                            let qq = q as u128;
                            let t = |a: u32, b: u32| a as u128 * b as u128 % qq;
                            let d = (t(m[0][0], ((t(m[1][1], m[2][2]) + qq - t(m[1][2], m[2][1])) % qq) as u32)
                                + qq
                                - t(m[0][1], ((t(m[1][0], m[2][2]) + qq - t(m[1][2], m[2][0])) % qq) as u32)
                                + t(m[0][2], ((t(m[1][0], m[2][1]) + qq - t(m[1][1], m[2][0])) % qq) as u32))
                                % qq;
                            d
                        };
                        if det != 0 {
                            break out;
                        }
                    };
                    c.class_from_conics(rows)
                },
                &mut raw,
            );
            run_op(
                "scalar_mul_64bit",
                &mut |i| c.mul_scalar(&xs[i], scalars[i] as u128),
                &mut raw,
            );
            for (name, v) in &results {
                let col = |j: usize| -> Vec<f64> {
                    v.iter()
                        .map(|t| match j {
                            0 => t.0,
                            1 => t.1,
                            2 => t.2,
                            _ => t.3,
                        })
                        .collect()
                };
                let (mm, m1, m3) = median_iqr(&mut col(0));
                let (im, i1, i3) = median_iqr(&mut col(1));
                let (am, a1, a3) = median_iqr(&mut col(2));
                let (nm, n1, n3) = median_iqr(&mut col(3));
                let used_lift = v.iter().filter(|t| t.5.lifts > 0).count();
                let used_p0 = v.iter().filter(|t| t.5.p0_in_support > 0).count();
                let used_exc = v.iter().filter(|t| t.5.exc_results > 0 || t.5.neg_pencil > 0).count();
                let used_excop = v.iter().filter(|t| t.5.exc_operands > 0).count();
                writeln!(
                    summ,
                    "{{\"rec\":\"op\",\"k\":{},\"q\":{},\"op\":\"{}\",\"n\":{},\"mul\":{{\"median\":{},\"q1\":{},\"q3\":{}}},\"inv\":{{\"median\":{},\"q1\":{},\"q3\":{}}},\"add\":{{\"median\":{},\"q1\":{},\"q3\":{}}},\"ns\":{{\"median\":{},\"q1\":{},\"q3\":{}}},\"ops_with_lift\":{},\"ops_with_p0_in_support\":{},\"ops_with_exceptional_result\":{},\"ops_with_exceptional_operand\":{},\"priced_mul_equiv_median\":{:.1}}}",
                    k, q, name, v.len(), mm, m1, m3, im, i1, i3, am, a1, a3, nm, n1, n3, used_lift, used_p0, used_exc, used_excop, mm + im * inv_equiv
                )
                .unwrap();
                series.push((name.clone(), (q as f64).log2(), mm, mm + im * inv_equiv, nm));
            }
            eprintln!("k={} q={} done", k, q);
        }
        // M2: OLS fits against log2 q with a 95% t-interval (n-2 degrees of freedom)
        let tcrit = |df: usize| -> f64 {
            // two-sided 95% critical values
            [12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228][df.saturating_sub(1).min(9)]
        };
        let mut fits = std::fs::File::create(format!("{}/m2_fits.jsonl", out_dir)).unwrap();
        let mut ops: Vec<String> = series.iter().map(|t| t.0.clone()).collect();
        ops.sort();
        ops.dedup();
        for op in &ops {
            let pts: Vec<&(String, f64, f64, f64, f64)> = series.iter().filter(|t| &t.0 == op).collect();
            let n = pts.len();
            if n < 3 {
                continue;
            }
            for (metric, sel) in [("mul_median", 2usize), ("priced_mul_equiv_median", 3), ("ns_median", 4)] {
                let xs: Vec<f64> = pts.iter().map(|t| t.1).collect();
                let ys: Vec<f64> = pts
                    .iter()
                    .map(|t| match sel {
                        2 => t.2,
                        3 => t.3,
                        _ => t.4,
                    })
                    .collect();
                let mx = xs.iter().sum::<f64>() / n as f64;
                let my = ys.iter().sum::<f64>() / n as f64;
                let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
                let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
                let slope = sxy / sxx;
                let icpt = my - slope * mx;
                let rss: f64 = xs.iter().zip(&ys).map(|(x, y)| (y - icpt - slope * x).powi(2)).sum();
                let se = (rss / (n as f64 - 2.0) / sxx).sqrt();
                let hw = tcrit(n - 2) * se;
                writeln!(
                    fits,
                    "{{\"op\":\"{}\",\"metric\":\"{}\",\"n_points\":{},\"slope_per_log2q\":{},\"slope_lo95\":{},\"slope_hi95\":{},\"intercept\":{},\"mean\":{}}}",
                    op, metric, n, slope, slope - hw, slope + hw, icpt, my
                )
                .unwrap();
            }
        }
    }

    // ------------------------------------------------------------------
    // V4: agreement with the degree-6 keys of quartic_ic.rs (GHS toys)
    // ------------------------------------------------------------------

    struct V4Rec {
        n: u64,
        deg6_key: String,
        pic3_deg: u64,
        pic3_key: String,
        cubics: Vec<Vec<u32>>,
    }

    pub struct V4Report {
        pub q: u32,
        pub ell: u64,
        pub usable: usize,
        pub distinct_deg6_keys: usize,
        pub distinct_pic3_keys: usize,
        pub distinct_canonical_keys: usize,
        pub triples_tested: usize,
        pub hom_failures: usize,
        pub order_ell_failures: usize,
        pub scalar_relation_failures: usize,
        pub canonical_collisions_unexplained: usize,
        pub pic3_collisions_resolved: usize,
        pub canonical_equal_deg6_differ: usize,
        pub nonzero_images: bool,
    }

    fn v4_run(q: u32, text: &str) -> V4Report {
        let mut header = None;
        let mut recs: Vec<V4Rec> = Vec::new();
        for line in text.lines() {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            match v["rec"].as_str().unwrap() {
                "header" => header = Some(v),
                "transfer" => recs.push(V4Rec {
                    n: v["n"].as_u64().unwrap(),
                    deg6_key: v["deg6_key"].as_str().unwrap().to_string(),
                    pic3_deg: v["pic3_deg"].as_u64().unwrap(),
                    pic3_key: v["pic3_key"].as_str().unwrap().to_string(),
                    cubics: v["cubics"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|r| r.as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u32).collect())
                        .collect(),
                }),
                _ => {}
            }
        }
        let h = header.unwrap();
        assert_eq!(h["q"].as_u64().unwrap() as u32, q);
        let ell = h["ell"].as_u64().unwrap();
        let m4 = mons_of_degree(4);
        let mut f = [0u32; 15];
        for t in h["quartic"].as_array().unwrap() {
            let t = t.as_array().unwrap();
            let m = (t[0].as_u64().unwrap() as u8, t[1].as_u64().unwrap() as u8, t[2].as_u64().unwrap() as u8);
            f[m4.iter().position(|&x| x == m).unwrap()] = t[3].as_u64().unwrap() as u32 % q;
        }
        let p0 = first_rational_point_canonical(q, &f).unwrap();
        let c = PlaneQuartic::new(q, f, p0).unwrap();
        // their cubic monomial order: i ascending, j ascending
        let theirs: Vec<(u8, u8, u8)> = h["cubic_monomials"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| {
                let m = m.as_array().unwrap();
                (m[0].as_u64().unwrap() as u8, m[1].as_u64().unwrap() as u8, m[2].as_u64().unwrap() as u8)
            })
            .collect();
        let mine = mons_of_degree(3);
        let perm: Vec<usize> = theirs.iter().map(|m| mine.iter().position(|x| x == m).unwrap()).collect();
        let hn: Vec<PicClass> = recs
            .iter()
            .map(|r| {
                let rows: Vec<Vec<u32>> = r
                    .cubics
                    .iter()
                    .map(|row| {
                        let mut v = vec![0u32; 10];
                        for (j, &x) in row.iter().enumerate() {
                            v[perm[j]] = x % q;
                        }
                        v
                    })
                    .collect();
                c.reduce_deg6(&Gen { k: 3, rows })
            })
            .collect();
        let idx_of_n: HashMap<u64, usize> = recs.iter().enumerate().map(|(i, r)| (r.n, i)).collect();
        // base shift B = h(a) + h(b) - h(a+b), constant over all triples
        let mut bshift: Option<PicClass> = None;
        let mut triples = 0usize;
        let mut hom_fail = 0usize;
        let mut triples_list = vec![];
        for i in 0..recs.len() {
            for j in i..recs.len() {
                let s = (recs[i].n + recs[j].n) % ell;
                if s == 0 {
                    continue;
                }
                if let Some(&k) = idx_of_n.get(&s) {
                    let b = c.sub(&c.add(&hn[i], &hn[j]), &hn[k]);
                    match &bshift {
                        None => bshift = Some(b),
                        Some(b0) => {
                            if *b0 != b {
                                hom_fail += 1;
                            }
                        }
                    }
                    triples += 1;
                    triples_list.push((i, j, k));
                }
            }
        }
        let b = bshift.expect("no addable triple among the usable transfers");
        let x: Vec<PicClass> = hn.iter().map(|y| c.sub(y, &b)).collect();
        // (iv) key(phi((a+b)G)) = add(key(phi(aG)), key(phi(bG)))
        for &(i, j, k) in &triples_list {
            if c.add(&x[i], &x[j]) != x[k] {
                hom_fail += 1;
            }
        }
        // l * x = 0 and x(n) = n * x(n0) / n0
        let mut ord_fail = 0usize;
        let mut scal_fail = 0usize;
        let n0 = recs[0].n;
        let inv_n0 = {
            // modular inverse of n0 mod ell (ell prime)
            let mut r = 1u64;
            let mut base = n0 % ell;
            let mut e = ell - 2;
            while e > 0 {
                if e & 1 == 1 {
                    r = r * base % ell;
                }
                base = base * base % ell;
                e >>= 1;
            }
            r
        };
        for (i, r) in recs.iter().enumerate() {
            if c.mul_scalar(&x[i], ell as u128) != c.zero() {
                ord_fail += 1;
            }
            let k = (r.n % ell) * inv_n0 % ell;
            if c.mul_scalar(&x[0], k as u128) != x[i] {
                scal_fail += 1;
            }
        }
        let nonzero = x.iter().all(|y| *y != c.zero());
        // key statistics and collisions
        let d6: HashSet<&str> = recs.iter().map(|r| r.deg6_key.as_str()).collect();
        let p3: HashSet<&str> = recs.iter().map(|r| r.pic3_key.as_str()).collect();
        let canon: HashSet<PicKey> = x.iter().map(|y| y.key()).collect();
        let mut unexplained = 0usize;
        let mut pic3_resolved = 0usize;
        let mut canon_eq_d6_differ = 0usize;
        for i in 0..recs.len() {
            for j in i + 1..recs.len() {
                let same_c = x[i].key() == x[j].key();
                if same_c && recs[i].n % ell != recs[j].n % ell {
                    unexplained += 1;
                }
                if same_c && recs[i].deg6_key != recs[j].deg6_key {
                    canon_eq_d6_differ += 1;
                }
                if !same_c && recs[i].pic3_deg == 3 && recs[j].pic3_deg == 3 && recs[i].pic3_key == recs[j].pic3_key {
                    pic3_resolved += 1;
                }
            }
        }
        V4Report {
            q,
            ell,
            usable: recs.len(),
            distinct_deg6_keys: d6.len(),
            distinct_pic3_keys: p3.len(),
            distinct_canonical_keys: canon.len(),
            triples_tested: triples,
            hom_failures: hom_fail,
            order_ell_failures: ord_fail,
            scalar_relation_failures: scal_fail,
            canonical_collisions_unexplained: unexplained,
            pic3_collisions_resolved: pic3_resolved,
            canonical_equal_deg6_differ: canon_eq_d6_differ,
            nonzero_images: nonzero,
        }
    }

    #[test]
    fn v4_ghs_transfer_agreement() {
        let dumps: [(u32, &str); 3] = [
            (5, include_str!("../../research/quartic_pic_canonical_20261018/results/v4_dump_q5.jsonl")),
            (7, include_str!("../../research/quartic_pic_canonical_20261018/results/v4_dump_q7.jsonl")),
            (11, include_str!("../../research/quartic_pic_canonical_20261018/results/v4_dump_q11.jsonl")),
        ];
        for (q, text) in dumps {
            let r = v4_run(q, text);
            eprintln!(
                "V4 q={} ell={} usable={} deg6_keys={} pic3_keys={} canonical_keys={} triples={} hom_fail={} ord_fail={} scalar_fail={} unexplained_canon_collisions={} pic3_collisions_resolved={} canon_equal_but_deg6_differs={} nonzero={}",
                r.q, r.ell, r.usable, r.distinct_deg6_keys, r.distinct_pic3_keys, r.distinct_canonical_keys, r.triples_tested,
                r.hom_failures, r.order_ell_failures, r.scalar_relation_failures, r.canonical_collisions_unexplained,
                r.pic3_collisions_resolved, r.canonical_equal_deg6_differ, r.nonzero_images
            );
            assert_eq!(r.hom_failures, 0);
            assert_eq!(r.order_ell_failures, 0);
            assert_eq!(r.scalar_relation_failures, 0);
            assert!(r.nonzero_images);
            assert_eq!(r.canonical_collisions_unexplained, 0);
            assert_eq!(r.distinct_canonical_keys, r.usable);
        }
    }
}
