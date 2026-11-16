//! Native Rust port of the exploratory lower-bound scripts for the HKY last fall degree
//! `d_F(1, t)` of a width-1 generator on the Kummer `m = 2` tower
//! (`research/prime_ic_leads_20261110/growth_law/lower_bound/README.md`).
//!
//! The ring is `R = F_p[z1, z2] / (z1^K - 1, z2^K - 1)`, `K = 2^t`, `p = 786433 = 3*2^18 + 1`,
//! with the popcount degree on normal-form monomials. `V_c(f)` is the smallest subspace of
//! `R_{<=c}` that contains `f` and is closed under `g -> y g` for every tower variable
//! `y = z_i^(2^j)` and every `g` of degree `< c`.
//!
//! This file re-implements, in Rust and over exact modular arithmetic, every computation
//! of the archived Python scripts (`closure.py`, `filt.py`, `lowmult.py`, `lowsupp.py`,
//! `twist.py`, `residue_check.py`, `topfun.py`, `topfun_w.py`, `hsupp2.py`, `s0pat.py`,
//! `thmB.py`, `mult2.py`, `gridcheck.py`), with the same parameters, and prints text lines
//! in the Python format so that the outputs can be diffed.
//!
//! Differences in method, none of which changes a printed value (see the README comparison):
//!
//! - the closure keeps an echelon basis (not a fully reduced one); every printed quantity is a
//!   basis-independent property of the subspace `V_c(f)`;
//! - the support of the multipliers `V f^{-1}` uses one pair of exact 2-D number-theoretic
//!   transforms per basis row instead of one dense float64 matrix product;
//! - the `thmB` kernel dimensions are computed exactly (streamed Gaussian elimination), not by a
//!   random projection that can only overestimate;
//! - random draws mirror CPython's `random` (MT19937, `seed(int)`, `randrange`), so the width-2/3
//!   generators of `topfun_w.py` and the random `g` of `residue_check.py` are the same
//!   polynomials; evaluation points of `lowmult`/`mult2` are drawn the same way, but the kernel of
//!   the evaluation map does not depend on them once there are enough points.
//!
//! Usage (stdout, Python-style text):
//!
//! ```text
//! growth_lower_bound all OUTDIR
//! growth_lower_bound closure T CMAX C00 C10 C01 C11 [ZETA1 ZETA2]
//! growth_lower_bound filt | twist | residue | lowmult D SMAX | mult2 D S
//! growth_lower_bound topfun T C [C00 C10 C01 C11] | topfun_w W T C SEED
//! growth_lower_bound lowsupp T C | hsupp2 T C | thmb T C | s0pat T C [DMAX..]
//! growth_lower_bound gridcheck T C00 C10 C01 C11
//! ```
//!
//! Every claim that this program's output supports is empirical and `[pcc:unverified]` until a
//! pcc claim is registered.

#![allow(clippy::needless_range_loop)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

const P: u64 = 786_433;
const NONE: u32 = u32::MAX;

type Coeffs = Vec<((u32, u32), u64)>;

const F1: [u64; 4] = [3, 1234, 5678, 9012];
const F2: [u64; 4] = [17, 40000, 123_457, 654_321];
const F3: [u64; 4] = [101, 7, 99991, 31337];

fn pop(x: u32) -> u32 {
    x.count_ones()
}

fn powm(mut b: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    b %= P;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e >>= 1;
    }
    r
}

fn invm(a: u64) -> u64 {
    powm(a % P, P - 2)
}

fn pybool(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}

fn jbool(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

/// Coefficients `c00 + c10 z1 + c01 z2 + c11 z1 z2` from `[c00, c10, c01, c11]`.
fn coeffs_of(cf: [u64; 4]) -> Coeffs {
    vec![
        ((0, 0), cf[0]),
        ((1, 0), cf[1]),
        ((0, 1), cf[2]),
        ((1, 1), cf[3]),
    ]
}

// ---------------------------------------------------------------------------------------------
// CPython-compatible Mersenne Twister
// ---------------------------------------------------------------------------------------------

struct Mt {
    mt: [u32; 624],
    i: usize,
}

impl Mt {
    fn init_genrand(s: u32) -> Mt {
        let mut mt = [0u32; 624];
        mt[0] = s;
        for i in 1..624 {
            mt[i] = 1_812_433_253u32
                .wrapping_mul(mt[i - 1] ^ (mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        Mt { mt, i: 624 }
    }

    fn init_by_array(key: &[u32]) -> Mt {
        let mut m = Mt::init_genrand(19_650_218);
        let n = 624usize;
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..n.max(key.len()) {
            let prev = m.mt[i - 1] ^ (m.mt[i - 1] >> 30);
            m.mt[i] = (m.mt[i] ^ prev.wrapping_mul(1_664_525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= n {
                m.mt[0] = m.mt[n - 1];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..n - 1 {
            let prev = m.mt[i - 1] ^ (m.mt[i - 1] >> 30);
            m.mt[i] = (m.mt[i] ^ prev.wrapping_mul(1_566_083_941)).wrapping_sub(i as u32);
            i += 1;
            if i >= n {
                m.mt[0] = m.mt[n - 1];
                i = 1;
            }
        }
        m.mt[0] = 0x8000_0000;
        m.i = 624;
        m
    }

    /// `random.seed(seed)` for a non-negative integer below 2^32.
    fn from_seed(seed: u32) -> Mt {
        Mt::init_by_array(&[seed])
    }

    fn twist(&mut self) {
        for k in 0..624 {
            let y = (self.mt[k] & 0x8000_0000) | (self.mt[(k + 1) % 624] & 0x7fff_ffff);
            let mut v = self.mt[(k + 397) % 624] ^ (y >> 1);
            if y & 1 == 1 {
                v ^= 0x9908_b0df;
            }
            self.mt[k] = v;
        }
        self.i = 0;
    }

    fn next_u32(&mut self) -> u32 {
        if self.i >= 624 {
            self.twist();
        }
        let mut y = self.mt[self.i];
        self.i += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;
        y
    }

    /// `random.random()` (used by the unit test that pins the port to CPython).
    #[cfg(test)]
    fn random(&mut self) -> f64 {
        let a = (self.next_u32() >> 5) as f64;
        let b = (self.next_u32() >> 6) as f64;
        (a * 67_108_864.0 + b) * (1.0 / 9_007_199_254_740_992.0)
    }

    /// CPython's `_randbelow_with_getrandbits(n)` for `1 <= n < 2^32`.
    fn randbelow(&mut self, n: u64) -> u64 {
        let k = 64 - n.leading_zeros();
        loop {
            let r = (self.next_u32() >> (32 - k)) as u64;
            if r < n {
                return r;
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Number-theoretic transform over F_p, length K = 2^t | p - 1
// ---------------------------------------------------------------------------------------------

fn root_of_unity(k: u64) -> u64 {
    for h in 2..P {
        let g = powm(h, (P - 1) / k);
        if powm(g, k / 2) != 1 {
            return g;
        }
    }
    unreachable!("p - 1 is divisible by K");
}

struct Ntt {
    k: usize,
    w: u64,
    rev: Vec<usize>,
    tw: Vec<u64>,
    itw: Vec<u64>,
}

impl Ntt {
    fn new(t: u32) -> Ntt {
        let k = 1usize << t;
        let w = root_of_unity(k as u64);
        let winv = invm(w);
        let mut rev = vec![0usize; k];
        for i in 1..k {
            rev[i] = (rev[i >> 1] >> 1) | ((i & 1) << (t - 1));
        }
        let mut tw = vec![0u64; k];
        let mut itw = vec![0u64; k];
        let mut half = 1usize;
        while half < k {
            let wl = powm(w, (k / (2 * half)) as u64);
            let wil = powm(winv, (k / (2 * half)) as u64);
            let (mut a, mut b) = (1u64, 1u64);
            for j in 0..half {
                tw[half + j] = a;
                itw[half + j] = b;
                a = a * wl % P;
                b = b * wil % P;
            }
            half *= 2;
        }
        Ntt { k, w, rev, tw, itw }
    }

    /// In-place transform with root `w` (forward) or `w^-1` (inverse, unscaled).
    fn transform(&self, a: &mut [u64], inverse: bool) {
        let k = self.k;
        for i in 0..k {
            let r = self.rev[i];
            if i < r {
                a.swap(i, r);
            }
        }
        let tw = if inverse { &self.itw } else { &self.tw };
        let mut half = 1usize;
        while half < k {
            for start in (0..k).step_by(2 * half) {
                for j in 0..half {
                    let u = a[start + j];
                    let v = a[start + j + half] * tw[half + j] % P;
                    a[start + j] = if u + v >= P { u + v - P } else { u + v };
                    a[start + j + half] = if u >= v { u - v } else { u + P - v };
                }
            }
            half *= 2;
        }
    }

    /// 2-D transform of a `K x K` row-major array.
    fn transform2d(&self, g: &mut [u64], inverse: bool) {
        let k = self.k;
        for row in g.chunks_mut(k) {
            self.transform(row, inverse);
        }
        let mut tmp = vec![0u64; k];
        for c in 0..k {
            for r in 0..k {
                tmp[r] = g[r * k + c];
            }
            self.transform(&mut tmp, inverse);
            for r in 0..k {
                g[r * k + c] = tmp[r];
            }
        }
    }
}

/// Values of `f` on the grid `mu_K^2` (`index x*K + y` is `f(w^x, w^y)`).
fn grid_values(ntt: &Ntt, coeffs: &Coeffs) -> Vec<u64> {
    let k = ntt.k;
    let mut g = vec![0u64; k * k];
    for &((a, b), v) in coeffs {
        let i = (a as usize % k) * k + (b as usize % k);
        g[i] = (g[i] + v % P) % P;
    }
    ntt.transform2d(&mut g, false);
    g
}

/// `f^{-1}` in `R`: pointwise inverses on the grid and the coefficient array `fi[a*K + b]`.
struct FInv {
    inv_vals: Vec<u64>,
    fi: Vec<u64>,
}

fn build_finv(ntt: &Ntt, coeffs: &Coeffs) -> Option<FInv> {
    let k = ntt.k;
    let fv = grid_values(ntt, coeffs);
    if fv.contains(&0) {
        return None;
    }
    let inv_vals: Vec<u64> = fv.iter().map(|&x| invm(x)).collect();
    let mut fi = inv_vals.clone();
    ntt.transform2d(&mut fi, true);
    let scale = invm(k as u64 % P);
    let scale2 = scale * scale % P;
    for x in fi.iter_mut() {
        *x = *x * scale2 % P;
    }
    Some(FInv { inv_vals, fi })
}

// ---------------------------------------------------------------------------------------------
// Ring, echelon basis, closure
// ---------------------------------------------------------------------------------------------

struct Ring {
    c: u32,
    k: u32,
    mons: Vec<(u32, u32)>,
    idx: Vec<u32>,
    deg: Vec<u8>,
    n: usize,
}

impl Ring {
    /// Monomials of popcount degree `<= c`, sorted by (descending degree, a, b).
    fn new(t: u32, c: u32) -> Ring {
        let k = 1u32 << t;
        let mut mons: Vec<(u32, u32)> = Vec::new();
        for a in 0..k {
            for b in 0..k {
                if pop(a) + pop(b) <= c {
                    mons.push((a, b));
                }
            }
        }
        mons.sort_by_key(|&(a, b)| (std::cmp::Reverse(pop(a) + pop(b)), a, b));
        let mut idx = vec![NONE; (k * k) as usize];
        for (i, &(a, b)) in mons.iter().enumerate() {
            idx[(a * k + b) as usize] = i as u32;
        }
        let deg: Vec<u8> = mons.iter().map(|&(a, b)| (pop(a) + pop(b)) as u8).collect();
        let n = mons.len();
        Ring {
            c,
            k,
            mons,
            idx,
            deg,
            n,
        }
    }

    fn by_degree(&self) -> Vec<usize> {
        (0..=self.c)
            .map(|d| self.deg.iter().filter(|&&x| x as u32 == d).count())
            .collect()
    }
}

/// Echelon basis (zeros before each pivot, pivot entry 1), reduced left to right.
struct Echelon {
    n: usize,
    piv_of: Vec<u32>,
    /// `(pivot column, entries from the pivot column on)`.
    rows: Vec<(usize, Vec<u32>)>,
}

impl Echelon {
    fn new(n: usize) -> Echelon {
        Echelon {
            n,
            piv_of: vec![NONE; n],
            rows: Vec::new(),
        }
    }

    /// Reduces `v` (u64 accumulators, entries may exceed `P`) against the basis. Returns the
    /// leading column of the remainder, whose entry `v[col]` is then reduced mod `P` and non-zero.
    fn reduce(&self, v: &mut [u64]) -> Option<usize> {
        for j in 0..self.n {
            let x = v[j] % P;
            if x == 0 {
                continue;
            }
            let pr = self.piv_of[j];
            if pr == NONE {
                v[j] = x;
                return Some(j);
            }
            let coef = P - x;
            let row = &self.rows[pr as usize].1;
            for (vk, &r) in v[j..j + row.len()].iter_mut().zip(row.iter()) {
                *vk += coef * r as u64;
            }
        }
        None
    }

    /// Inserts `v`; returns `(pivot column, row id)` if it was independent.
    fn insert(&mut self, v: &mut [u64]) -> Option<(usize, usize)> {
        let j = self.reduce(v)?;
        let inv = invm(v[j]);
        let row: Vec<u32> = v[j..self.n]
            .iter()
            .map(|&x| ((x % P) * inv % P) as u32)
            .collect();
        let id = self.rows.len();
        self.rows.push((j, row));
        self.piv_of[j] = id as u32;
        Some((j, id))
    }
}

struct Closure {
    ring: Ring,
    ech: Echelon,
}

impl Closure {
    fn has_one(&self) -> bool {
        self.ech.piv_of[self.ring.n - 1] != NONE
    }

    fn dim(&self) -> usize {
        self.ech.rows.len()
    }

    fn pivot_deg(&self, rid: usize) -> u32 {
        self.ring.deg[self.ech.rows[rid].0] as u32
    }

    /// Dimension of `V` by pivot degree, for degrees `0..=upto`.
    fn dims_by_pivot_degree(&self, upto: u32) -> Vec<usize> {
        let mut out = vec![0usize; upto as usize + 1];
        for rid in 0..self.dim() {
            let d = self.pivot_deg(rid) as usize;
            if d < out.len() {
                out[d] += 1;
            }
        }
        out
    }

    fn rows_with_pivot_degree_at_most(&self, dmax: u32) -> Vec<usize> {
        (0..self.dim())
            .filter(|&r| self.pivot_deg(r) <= dmax)
            .collect()
    }

    /// Dense row `rid` on the grid array `g[a*K + b]` (cleared first).
    fn row_to_grid(&self, rid: usize, g: &mut [u64]) {
        g.iter_mut().for_each(|x| *x = 0);
        let k = self.ring.k;
        let (off, row) = &self.ech.rows[rid];
        for (kk, &val) in row.iter().enumerate() {
            if val != 0 {
                let (a, b) = self.ring.mons[off + kk];
                g[(a * k + b) as usize] = val as u64;
            }
        }
    }
}

fn generator(ring: &Ring, coeffs: &Coeffs) -> Vec<u64> {
    let mut f = vec![0u64; ring.n];
    for &((a, b), v) in coeffs {
        let i = ring.idx[((a % ring.k) * ring.k + b % ring.k) as usize];
        assert!(i != NONE, "generator degree exceeds c");
        f[i as usize] = (f[i as usize] + v % P) % P;
    }
    f
}

/// The exact closure `V_c(f)` in the algebra `z_i^K = zeta_i` (`zeta = (1, 1)` is the untwisted
/// algebra).
fn closure_run(t: u32, c: u32, coeffs: &Coeffs, zeta: (u64, u64)) -> Closure {
    let ring = Ring::new(t, c);
    let n = ring.n;
    let k = ring.k;
    let zs = [zeta.0 % P, zeta.1 % P];
    let mut ech = Echelon::new(n);
    let mut queue: Vec<usize> = Vec::new();
    let mut f = generator(&ring, coeffs);
    if let Some((col, rid)) = ech.insert(&mut f) {
        if (ring.deg[col] as u32) < c {
            queue.push(rid);
        }
    }
    let mut qi = 0usize;
    let mut v = vec![0u64; n];
    while qi < queue.len() {
        let rid = queue[qi];
        qi += 1;
        let (off, row) = ech.rows[rid].clone();
        for i in 0..2usize {
            for j in 0..t {
                v.iter_mut().for_each(|x| *x = 0);
                for (kk, &val) in row.iter().enumerate() {
                    if val == 0 {
                        continue;
                    }
                    let (a, b) = ring.mons[off + kk];
                    let mut e = [a, b];
                    let mut fac = 1u64;
                    e[i] += 1 << j;
                    if e[i] >= k {
                        e[i] -= k;
                        fac = zs[i];
                    }
                    let tgt = ring.idx[(e[0] * k + e[1]) as usize];
                    debug_assert!(tgt != NONE);
                    v[tgt as usize] = val as u64 * fac;
                }
                if let Some((col, nrid)) = ech.insert(&mut v) {
                    if (ring.deg[col] as u32) < c {
                        queue.push(nrid);
                    }
                }
            }
        }
    }
    Closure { ring, ech }
}

/// `lambda_f(g) = [z1^{K-1} z2^{K-1}](g f^{-1})` kills every basis row of `V`?
fn lambda_kills(cl: &Closure, fi: &FInv) -> bool {
    let k = cl.ring.k as usize;
    let ell: Vec<u64> = cl
        .ring
        .mons
        .iter()
        .map(|&(a, b)| fi.fi[(k - 1 - a as usize) * k + (k - 1 - b as usize)])
        .collect();
    cl.ech.rows.iter().all(|(off, row)| {
        let s: u64 = row
            .iter()
            .enumerate()
            .map(|(kk, &r)| r as u64 * ell[off + kk])
            .sum();
        s.is_multiple_of(P)
    })
}

/// Marks the monomials `T` with `[T](g f^{-1}) != 0` for some `g` among the given basis rows.
fn used_set(cl: &Closure, fi: &FInv, ntt: &Ntt, rows: &[usize]) -> Vec<bool> {
    let k2 = ntt.k * ntt.k;
    let nthreads = std::thread::available_parallelism()
        .map(|x| x.get())
        .unwrap_or(1)
        .min(8)
        .min(rows.len().max(1));
    let chunk = rows.len().div_ceil(nthreads.max(1)).max(1);
    let mut used = vec![false; k2];
    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for part in rows.chunks(chunk) {
            handles.push(s.spawn(move || {
                let mut local = vec![false; k2];
                let mut g = vec![0u64; k2];
                for &rid in part {
                    cl.row_to_grid(rid, &mut g);
                    ntt.transform2d(&mut g, false);
                    for (x, &iv) in g.iter_mut().zip(fi.inv_vals.iter()) {
                        *x = *x * iv % P;
                    }
                    ntt.transform2d(&mut g, true);
                    for (u, &x) in local.iter_mut().zip(g.iter()) {
                        if x != 0 {
                            *u = true;
                        }
                    }
                }
                local
            }));
        }
        for h in handles {
            let local = h.join().expect("worker panicked");
            for (u, l) in used.iter_mut().zip(local) {
                *u |= l;
            }
        }
    });
    used
}

// ---------------------------------------------------------------------------------------------
// Dense RREF (small matrices) for lowmult / mult2
// ---------------------------------------------------------------------------------------------

fn rref(mut a: Vec<Vec<u64>>, ncols: usize) -> (Vec<Vec<u64>>, Vec<usize>) {
    let m = a.len();
    let mut r = 0usize;
    let mut piv = Vec::new();
    for col in 0..ncols {
        if r >= m {
            break;
        }
        let Some(kk) = (r..m).find(|&i| !a[i][col].is_multiple_of(P)) else {
            continue;
        };
        a.swap(r, kk);
        let inv = invm(a[r][col]);
        for x in a[r].iter_mut() {
            *x = *x * inv % P;
        }
        let pr = a[r].clone();
        for i in 0..m {
            if i != r && a[i][col] != 0 {
                let f = a[i][col];
                for (x, &y) in a[i].iter_mut().zip(pr.iter()) {
                    *x = (*x + (P - f) * y) % P;
                }
            }
        }
        piv.push(col);
        r += 1;
    }
    a.truncate(r);
    (a, piv)
}

/// Exact `dim { u in R_{<= c-1} : f^{-1} u is supported in S }` (the `thmB.py` kernel dimension),
/// by streamed Gaussian elimination of the rows `T not in S` of the matrix `[T](u f^{-1})`.
fn kerdim(ring: &Ring, fi: &FInv, in_s: &[bool]) -> usize {
    let k = ring.k as usize;
    let cols: Vec<(usize, usize)> = ring
        .mons
        .iter()
        .filter(|&&(a, b)| pop(a) + pop(b) < ring.c)
        .map(|&(a, b)| (a as usize, b as usize))
        .collect();
    let mut ech = Echelon::new(cols.len());
    let mut v = vec![0u64; cols.len()];
    for tt in 0..k * k {
        if in_s[tt] {
            continue;
        }
        let (ta, tb) = (tt / k, tt % k);
        for (x, &(a, b)) in v.iter_mut().zip(cols.iter()) {
            *x = fi.fi[((ta + k - a) % k) * k + (tb + k - b) % k];
        }
        ech.insert(&mut v);
        if ech.rows.len() == cols.len() {
            break;
        }
    }
    cols.len() - ech.rows.len()
}

/// Points `x` (CPython `random.seed(seed)`, `randrange(p)`) with `c01 + c11 x != 0`.
fn curve_points(seed: u32, count: usize, c01: u64, c11: u64) -> Vec<u64> {
    let mut mt = Mt::from_seed(seed);
    let mut seen = HashSet::new();
    let mut v = Vec::new();
    while v.len() < count {
        let x = mt.randbelow(P);
        if !(c01 + c11 * x).is_multiple_of(P) && seen.insert(x) {
            v.push(x);
        }
    }
    v
}

/// Evaluation matrix of the monomials `mons` (exponents `< e`) on the curve `f = 0`,
/// `z1 = x`, `z2 = -(c00 + c10 x)/(c01 + c11 x)`, for the generator `F1`.
fn curve_matrix(mons: &[(u32, u32)], xs: &[u64], e: usize) -> Vec<Vec<u64>> {
    let [c00, c10, c01, c11] = F1;
    xs.iter()
        .map(|&x| {
            let y = (P - (c00 + c10 * x) % P) % P * invm(c01 + c11 * x) % P;
            let mut px = vec![1u64; e + 1];
            let mut py = vec![1u64; e + 1];
            for i in 1..=e {
                px[i] = px[i - 1] * x % P;
                py[i] = py[i - 1] * y % P;
            }
            mons.iter()
                .map(|&(a, b)| px[a as usize] * py[b as usize] % P)
                .collect()
        })
        .collect()
}

/// `(kernel dimension, number of monomials, max exponent in the kernel support)` of the polynomials
/// of popcount degree `<= d` with exponents `< 2^s` that vanish on the curve `f1 = 0`.
fn lowmult(d: u32, s: u32) -> (usize, usize, Option<u32>) {
    let e = 1usize << s;
    let mut mons = Vec::new();
    for a in 0..e as u32 {
        for b in 0..e as u32 {
            if pop(a) + pop(b) <= d {
                mons.push((a, b));
            }
        }
    }
    let npts = mons.len() + 2 * e + 10;
    let xs = curve_points(1, npts, F1[2], F1[3]);
    let a = curve_matrix(&mons, &xs, e);
    let (rr, piv) = rref(a, mons.len());
    let pivset: HashSet<usize> = piv.iter().copied().collect();
    let free: Vec<usize> = (0..mons.len()).filter(|j| !pivset.contains(j)).collect();
    let mut supp: HashSet<(u32, u32)> = free.iter().map(|&j| mons[j]).collect();
    for (r, &pc) in piv.iter().enumerate() {
        if free.iter().any(|&j| rr[r][j] != 0) {
            supp.insert(mons[pc]);
        }
    }
    let maxe = supp.iter().map(|&(a, b)| a.max(b)).max();
    (mons.len() - piv.len(), mons.len(), maxe)
}

/// Kernel basis (as sorted monomial supports) of `{g in f P : popdeg <= d, exps < 2^s}`.
fn mult2(d: u32, s: u32) -> Vec<Vec<(u32, u32)>> {
    let e = 1usize << s;
    let mut mons = Vec::new();
    for a in 0..e as u32 {
        for b in 0..e as u32 {
            if pop(a) + pop(b) <= d {
                mons.push((a, b));
            }
        }
    }
    mons.sort_by_key(|&(a, b)| {
        (
            std::cmp::Reverse(a.max(b)),
            std::cmp::Reverse(a + b),
            (a, b),
        )
    });
    let xs = curve_points(2, mons.len() + 2 * e + 10, F1[2], F1[3]);
    let a = curve_matrix(&mons, &xs, e);
    let (rr, piv) = rref(a, mons.len());
    let pivset: HashSet<usize> = piv.iter().copied().collect();
    let mut out = Vec::new();
    for j in (0..mons.len()).filter(|j| !pivset.contains(j)) {
        let mut v = vec![mons[j]];
        for (r, &pc) in piv.iter().enumerate() {
            if rr[r][j] != 0 {
                v.push(mons[pc]);
            }
        }
        v.sort_by_key(|&(a, b)| (std::cmp::Reverse(a.max(b)), (a, b)));
        out.push(v);
    }
    out
}

fn pairs_text(v: &[(u32, u32)]) -> String {
    let items: Vec<String> = v.iter().map(|(a, b)| format!("({a}, {b})")).collect();
    format!("[{}]", items.join(", "))
}

fn list_text(v: &[usize]) -> String {
    let items: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", items.join(", "))
}

fn bits(x: u32, t: u32) -> String {
    format!("{:0width$b}", x, width = t as usize)
}

// ---------------------------------------------------------------------------------------------
// Experiment context: closure cache, Python-format text, JSONL records
// ---------------------------------------------------------------------------------------------

struct Ctx {
    cache: HashMap<String, Rc<Closure>>,
    text: String,
    jl: Vec<String>,
}

impl Ctx {
    fn new() -> Ctx {
        Ctx {
            cache: HashMap::new(),
            text: String::new(),
            jl: Vec::new(),
        }
    }

    fn line(&mut self, s: String) {
        self.text.push_str(&s);
        self.text.push('\n');
    }

    fn closure(&mut self, t: u32, c: u32, coeffs: &Coeffs, zeta: (u64, u64)) -> Rc<Closure> {
        let key = format!("{t}/{c}/{coeffs:?}/{zeta:?}");
        if let Some(x) = self.cache.get(&key) {
            return Rc::clone(x);
        }
        let t0 = Instant::now();
        let cl = Rc::new(closure_run(t, c, coeffs, zeta));
        eprintln!(
            "[closure t={t} c={c} twist={zeta:?} dim={} N={} one={} {:.1}s]",
            cl.dim(),
            cl.ring.n,
            cl.has_one(),
            t0.elapsed().as_secs_f64()
        );
        self.cache.insert(key, Rc::clone(&cl));
        cl
    }

    /// `closure.py`'s `last_fall`: the closure at c = 2, 3, ..., until `1` enters.
    fn last_fall(
        &mut self,
        t: u32,
        cmax: u32,
        coeffs: &Coeffs,
        zeta: (u64, u64),
        echo: bool,
    ) -> Option<u32> {
        for c in 2..=cmax {
            let cl = self.closure(t, c, coeffs, zeta);
            if echo {
                self.line(format!(
                    "c={} dim V={} / {}  one={}",
                    c,
                    cl.dim(),
                    cl.ring.n,
                    pybool(cl.has_one())
                ));
                self.jl.push(format!(
                    r#"{{"script":"closure","t":{t},"c":{c},"dim":{},"N":{},"has_one":{}}}"#,
                    cl.dim(),
                    cl.ring.n,
                    jbool(cl.has_one())
                ));
            }
            if cl.has_one() {
                return Some(c);
            }
        }
        None
    }

    fn closure_cli(&mut self, t: u32, cmax: u32, cf: [u64; 4], zeta: (u64, u64)) {
        let d = self.last_fall(t, cmax, &coeffs_of(cf), zeta, true);
        let ds = d.map_or("None".to_string(), |x| x.to_string());
        self.line(format!("d_F = {ds}"));
        let dj = d.map_or("null".to_string(), |x| x.to_string());
        self.jl.push(format!(
            r#"{{"script":"closure_dF","t":{t},"cmax":{cmax},"coeffs":{cf:?},"zeta":[{},{}],"d_F":{dj}}}"#,
            zeta.0, zeta.1
        ));
    }

    fn filt(&mut self) {
        let coeffs = coeffs_of(F1);
        for c in [3u32, 4] {
            for t in 2..8u32 {
                let cl = self.closure(t, c, &coeffs, (1, 1));
                let pd = cl.dims_by_pivot_degree(c);
                let rd = cl.ring.by_degree();
                self.line(format!(
                    "c {c} t {t} dim V by pivot degree {} R by deg {}",
                    list_text(&pd),
                    list_text(&rd)
                ));
                self.jl.push(format!(
                    r#"{{"script":"filt","t":{t},"c":{c},"dim_by_pivot_degree":{},"R_by_degree":{}}}"#,
                    list_text(&pd),
                    list_text(&rd)
                ));
            }
        }
    }

    fn lowmult_cli(&mut self, d: u32, smax: u32) {
        for s in 2..=smax {
            let (ker, nm, maxe) = lowmult(d, s);
            let me = maxe.map_or("None".to_string(), |x| x.to_string());
            self.line(format!("d {d} s {s} ({ker}, {nm}, {me})"));
            self.jl.push(format!(
                r#"{{"script":"lowmult","d":{d},"s":{s},"kernel_dim":{ker},"num_monomials":{nm},"max_exponent":{}}}"#,
                maxe.map_or("null".to_string(), |x| x.to_string())
            ));
        }
    }

    fn twist(&mut self) {
        let coeffs = coeffs_of(F1);
        let [c00, c10, c01, c11] = F1;
        let x = 777u64;
        let y = (P - (c00 + c10 * x) % P) % P * invm(c01 + c11 * x) % P;
        for t in 2..8u32 {
            let k = 1u64 << t;
            let on_c = (powm(x, k), powm(y, k));
            let cmax = if t < 7 { 5 } else { 4 };
            let r1 = self.last_fall(t, cmax, &coeffs, (5, 7), false);
            let r2 = self.last_fall(t, cmax, &coeffs, on_c, false);
            let s1 = r1.map_or("None".to_string(), |v| v.to_string());
            let s2 = r2.map_or("None".to_string(), |v| v.to_string());
            self.line(format!("t {t} generic zeta d_F {s1} zeta on C' d_F {s2}"));
            self.jl.push(format!(
                r#"{{"script":"twist","t":{t},"cmax":{cmax},"zeta_generic":[5,7],"d_F_generic":{},"zeta_on_curve":[{},{}],"d_F_on_curve":{}}}"#,
                r1.map_or("null".to_string(), |v| v.to_string()),
                on_c.0,
                on_c.1,
                r2.map_or("null".to_string(), |v| v.to_string())
            ));
        }
    }

    fn residue(&mut self) {
        let mut mt = Mt::from_seed(7);
        let [c00, c10, c01, c11] = F1;
        for t in 2..=5u32 {
            let k = 1usize << t;
            let w = root_of_unity(k as u64);
            let mu: Vec<u64> = (0..k).map(|i| powm(w, i as u64)).collect();
            let al = c10 * invm(c11) % P;
            let be = c01 * invm(c11) % P;
            let ga = c00 * invm(c11) % P;
            let f = |x: u64, y: u64| (c00 + c10 * x % P + c01 * y % P + c11 * (x * y % P)) % P;
            for trial in 0..3 {
                let mut g = vec![0u64; k * k];
                for v in g.iter_mut() {
                    *v = mt.randbelow(P);
                }
                let big_g = |x: u64, y: u64| -> u64 {
                    let mut px = vec![1u64; k];
                    let mut py = vec![1u64; k];
                    for i in 1..k {
                        px[i] = px[i - 1] * x % P;
                        py[i] = py[i - 1] * y % P;
                    }
                    let mut s = 0u64;
                    for a in 0..k {
                        for b in 0..k {
                            s = (s + g[a * k + b] * (px[a] * py[b] % P)) % P;
                        }
                    }
                    s
                };
                let mut lhs = 0u64;
                for &x in &mu {
                    for &y in &mu {
                        lhs = (lhs + x * y % P * big_g(x, y) % P * invm(f(x, y))) % P;
                    }
                }
                let mut rhs = 0u64;
                for &s in &mu {
                    let r = (P - (ga + be * s) % P) % P * invm(s + al) % P;
                    let den = (s + al) % P * ((powm(r, k as u64) + P - 1) % P) % P;
                    rhs = (rhs + s * big_g(r, s) % P * invm(den)) % P;
                }
                rhs = (P - (k as u64 % P) * rhs % P * invm(c11) % P) % P;
                self.line(format!(
                    "t {t} trial {trial} lhs==rhs {}",
                    pybool(lhs == rhs)
                ));
                self.jl.push(format!(
                    r#"{{"script":"residue_check","t":{t},"trial":{trial},"lhs":{lhs},"rhs":{rhs},"equal":{}}}"#,
                    jbool(lhs == rhs)
                ));
            }
        }
    }

    fn topfun(&mut self, t: u32, c: u32, cf: [u64; 4]) {
        let coeffs = coeffs_of(cf);
        let ntt = Ntt::new(t);
        let head = format!("['{t}', '{c}']");
        let Some(fi) = build_finv(&ntt, &coeffs) else {
            self.line(format!("{head} nonunit"));
            self.jl.push(format!(
                r#"{{"script":"topfun","t":{t},"c":{c},"coeffs":{cf:?},"result":"nonunit"}}"#
            ));
            return;
        };
        let cl = self.closure(t, c, &coeffs, (1, 1));
        let k = 1usize << t;
        let kills = lambda_kills(&cl, &fi);
        let lam1 = fi.fi[(k - 1) * k + (k - 1)];
        self.line(format!(
            "{head} {{'one': {}, 'lam_kills_V': {}, 'lam1': {lam1}}}",
            pybool(cl.has_one()),
            pybool(kills)
        ));
        self.jl.push(format!(
            r#"{{"script":"topfun","t":{t},"c":{c},"coeffs":{cf:?},"one":{},"lam_kills_V":{},"lam1":{lam1}}}"#,
            jbool(cl.has_one()),
            jbool(kills)
        ));
    }

    fn topfun_w(&mut self, w: u32, t: u32, c: u32, seed: u32) {
        let mut mt = Mt::from_seed(seed);
        let mut coeffs: Coeffs = Vec::new();
        for a in 0..=w {
            for b in 0..=w {
                coeffs.push(((a, b), 1 + mt.randbelow(P - 1)));
            }
        }
        let ntt = Ntt::new(t);
        let Some(fi) = build_finv(&ntt, &coeffs) else {
            self.line("nonunit".to_string());
            self.jl.push(format!(
                r#"{{"script":"topfun_w","W":{w},"t":{t},"c":{c},"seed":{seed},"result":"nonunit"}}"#
            ));
            return;
        };
        let cl = self.closure(t, c, &coeffs, (1, 1));
        let k = 1usize << t;
        let kills = lambda_kills(&cl, &fi);
        let nz = fi.fi[(k - 1) * k + (k - 1)] != 0;
        self.line(format!(
            "W {w} t {t} c {c} seed {seed} one {} lam kills V {} lam(1)!=0 {}",
            pybool(cl.has_one()),
            pybool(kills),
            pybool(nz)
        ));
        self.jl.push(format!(
            r#"{{"script":"topfun_w","W":{w},"t":{t},"c":{c},"seed":{seed},"one":{},"lam_kills_V":{},"lam1_nonzero":{}}}"#,
            jbool(cl.has_one()),
            jbool(kills),
            jbool(nz)
        ));
    }

    fn lowsupp(&mut self, t: u32, c: u32) {
        let coeffs = coeffs_of(F1);
        let cl = self.closure(t, c, &coeffs, (1, 1));
        let pd = cl.dims_by_pivot_degree(c);
        self.line(format!(
            "t {t} c {c} one {} dims by deg {}",
            pybool(cl.has_one()),
            list_text(&pd)
        ));
        let mut supp: HashSet<(u32, u32)> = HashSet::new();
        for rid in cl.rows_with_pivot_degree_at_most(c - 1) {
            let (off, row) = &cl.ech.rows[rid];
            for (kk, &val) in row.iter().enumerate() {
                if val != 0 {
                    supp.insert(cl.ring.mons[off + kk]);
                }
            }
        }
        let maxe = supp.iter().map(|&(a, b)| a.max(b)).max().unwrap_or(0);
        self.line(format!(
            "max exponent in low part {maxe} num monomials {}",
            supp.len()
        ));
        self.jl.push(format!(
            r#"{{"script":"lowsupp","t":{t},"c":{c},"one":{},"dims_by_pivot_degree":{},"max_exponent_low":{maxe},"num_monomials_low":{}}}"#,
            jbool(cl.has_one()),
            list_text(&pd),
            supp.len()
        ));
    }

    fn hsupp2(&mut self, t: u32, c: u32) {
        let coeffs = coeffs_of(F1);
        let ntt = Ntt::new(t);
        let fi = build_finv(&ntt, &coeffs).expect("f1 is a unit");
        let cl = self.closure(t, c, &coeffs, (1, 1));
        self.line(format!("one {} dim {}", pybool(cl.has_one()), cl.dim()));
        let rows: Vec<usize> = (0..cl.dim()).collect();
        let t0 = Instant::now();
        let used = used_set(&cl, &fi, &ntt, &rows);
        eprintln!(
            "[hsupp2 t={t} c={c} used-set {:.1}s]",
            t0.elapsed().as_secs_f64()
        );
        let k2 = used.len();
        let nused = used.iter().filter(|&&u| u).count();
        let inu: Vec<bool> = used.iter().map(|&u| !u).collect();
        // Minimal elements of the unused set under bitwise inclusion of the 2t-bit word
        // (a << t | b): x is minimal iff no proper subset of x is unused.
        let mut has = vec![false; k2];
        let mut mins: Vec<usize> = Vec::new();
        for x in 0..k2 {
            let mut proper = false;
            let mut bs = x;
            while bs != 0 {
                let lb = 1 << bs.trailing_zeros();
                if has[x ^ lb] {
                    proper = true;
                    break;
                }
                bs ^= lb;
            }
            has[x] = proper || inu[x];
            if inu[x] && !proper {
                mins.push(x);
            }
        }
        let mut upset = true;
        for x in 0..k2 {
            if inu[x] {
                for b in 0..2 * t {
                    if (x >> b) & 1 == 0 && !inu[x | (1 << b)] {
                        upset = false;
                    }
                }
            }
        }
        self.line(format!("used {nused} unused {}", k2 - nused));
        self.line(format!("minimal unused: {}", mins.len()));
        let mut minstr = Vec::new();
        for (i, &x) in mins.iter().enumerate() {
            let (a, b) = ((x >> t) as u32, (x & ((1 << t) - 1)) as u32);
            minstr.push(format!("\"{} {}\"", bits(a, t), bits(b, t)));
            if i < 60 {
                self.line(format!("   {} {}", bits(a, t), bits(b, t)));
            }
        }
        self.line(format!("unused is up-set: {}", pybool(upset)));
        self.jl.push(format!(
            r#"{{"script":"hsupp2","t":{t},"c":{c},"one":{},"dim":{},"used":{nused},"unused":{},"num_minimal_unused":{},"minimal_unused":[{}],"unused_is_upset":{}}}"#,
            jbool(cl.has_one()),
            cl.dim(),
            k2 - nused,
            mins.len(),
            minstr.join(","),
            jbool(upset)
        ));
    }

    fn thmb(&mut self, t: u32, c: u32) {
        let coeffs = coeffs_of(F1);
        let ntt = Ntt::new(t);
        let fi = build_finv(&ntt, &coeffs).expect("f1 is a unit");
        let cl = self.closure(t, c, &coeffs, (1, 1));
        let low = cl.rows_with_pivot_degree_at_most(c - 1);
        let used = used_set(&cl, &fi, &ntt, &low);
        let k = 1usize << t;
        let s0 = used;
        let mut s1 = s0.clone();
        for x in 0..k * k {
            if s0[x] {
                let (a, b) = (x / k, x % k);
                for j in 0..t as usize {
                    s1[((a + (1 << j)) % k) * k + b] = true;
                    s1[a * k + (b + (1 << j)) % k] = true;
                }
            }
        }
        let n0 = s0.iter().filter(|&&u| u).count();
        let n1 = s1.iter().filter(|&&u| u).count();
        self.line(format!(
            "t {t} c {c} one {} dim V_<c {} |S0| {n0} |S1| {n1} of {}",
            pybool(cl.has_one()),
            low.len(),
            k * k
        ));
        let top = k * k - 1;
        self.line(format!(
            " topmonomial in S0: {}  in S1: {}",
            pybool(s0[top]),
            pybool(s1[top])
        ));
        let t0 = Instant::now();
        let k0 = kerdim(&cl.ring, &fi, &s0);
        let k1 = kerdim(&cl.ring, &fi, &s1);
        eprintln!(
            "[thmb t={t} c={c} kerdim {:.1}s]",
            t0.elapsed().as_secs_f64()
        );
        self.line(format!(" kerdim S0: {k0}  kerdim S1: {k1}"));
        self.jl.push(format!(
            r#"{{"script":"thmB","t":{t},"c":{c},"one":{},"dim_V_lt_c":{},"S0":{n0},"S1":{n1},"K2":{},"top_in_S0":{},"top_in_S1":{},"kerdim_S0":{k0},"kerdim_S1":{k1}}}"#,
            jbool(cl.has_one()),
            low.len(),
            k * k,
            jbool(s0[top]),
            jbool(s1[top])
        ));
    }

    fn s0pat(&mut self, t: u32, c: u32, dmaxs: &[u32]) {
        let coeffs = coeffs_of(F1);
        let ntt = Ntt::new(t);
        let fi = build_finv(&ntt, &coeffs).expect("f1 is a unit");
        let cl = self.closure(t, c, &coeffs, (1, 1));
        let k = 1usize << t;
        let dm: Vec<u32> = if dmaxs.is_empty() {
            (2..c).collect()
        } else {
            dmaxs.to_vec()
        };
        for dmax in dm {
            let low = cl.rows_with_pivot_degree_at_most(dmax);
            let used = used_set(&cl, &fi, &ntt, &low);
            let nused = used.iter().filter(|&&u| u).count();
            let un = |a: u32, b: u32| !used[(a as usize) * k + b as usize];
            let mut mins: Vec<(u32, u32)> = Vec::new();
            for x in 0..k * k {
                if used[x] {
                    continue;
                }
                let (a, b) = ((x / k) as u32, (x % k) as u32);
                let mut minimal = true;
                for j in 0..t {
                    if (a >> j) & 1 == 1 && un(a & !(1 << j), b) {
                        minimal = false;
                    }
                    if (b >> j) & 1 == 1 && un(a, b & !(1 << j)) {
                        minimal = false;
                    }
                }
                if minimal {
                    mins.push((a, b));
                }
            }
            self.line(format!(
                "t {t} c {c} deg<= {dmax} dim {} |supp mult| {nused} minimal non-members {}",
                low.len(),
                mins.len()
            ));
            let mut minstr = Vec::new();
            for (i, &(a, b)) in mins.iter().enumerate() {
                minstr.push(format!("\"{} {}\"", bits(a, t), bits(b, t)));
                if i < 40 {
                    self.line(format!("    {} {}", bits(a, t), bits(b, t)));
                }
            }
            let maxpop = (0..k * k)
                .filter(|&x| used[x])
                .map(|x| pop((x / k) as u32) + pop((x % k) as u32))
                .max()
                .unwrap_or(0);
            self.line(format!("   max popdeg of multipliers {maxpop}"));
            self.jl.push(format!(
                r#"{{"script":"s0pat","t":{t},"c":{c},"dmax":{dmax},"dim":{},"supp_mult":{nused},"num_minimal_nonmembers":{},"minimal_nonmembers":[{}],"max_popdeg_multipliers":{maxpop}}}"#,
                low.len(),
                mins.len(),
                minstr.join(",")
            ));
        }
    }

    fn mult2_cli(&mut self, d: u32, s: u32) {
        let basis = mult2(d, s);
        let mut parts = Vec::new();
        for v in &basis {
            self.line(pairs_text(v));
            parts.push(pairs_text(v));
        }
        self.jl.push(format!(
            r#"{{"script":"mult2","d":{d},"s":{s},"kernel_dim":{},"kernel_basis_supports":"{}"}}"#,
            basis.len(),
            parts.join(" | ")
        ));
    }

    fn gridcheck(&mut self, t: u32, cf: [u64; 4]) {
        let ntt = Ntt::new(t);
        let fv = grid_values(&ntt, &coeffs_of(cf));
        let k = ntt.k;
        let mut zeros: Vec<(u64, u64)> = Vec::new();
        for (i, &v) in fv.iter().enumerate() {
            if v == 0 {
                zeros.push((powm(ntt.w, (i / k) as u64), powm(ntt.w, (i % k) as u64)));
            }
        }
        let items: Vec<String> = zeros.iter().map(|(x, y)| format!("({x}, {y})")).collect();
        self.line(format!(
            "t {t} coeffs {cf:?} grid zeros [{}]",
            items.join(", ")
        ));
        self.jl.push(format!(
            r#"{{"script":"gridcheck","t":{t},"coeffs":{cf:?},"num_grid_zeros":{}}}"#,
            zeros.len()
        ));
    }
}

// ---------------------------------------------------------------------------------------------
// Drivers
// ---------------------------------------------------------------------------------------------

fn flush_group(ctx: &mut Ctx, dir: &Path, name: &str, all_jl: &mut Vec<String>) {
    let text = std::mem::take(&mut ctx.text);
    fs::write(dir.join(name), text).expect("write text output");
    all_jl.append(&mut ctx.jl);
}

fn run_all(dir: &Path) {
    fs::create_dir_all(dir).expect("create output dir");
    let mut ctx = Ctx::new();
    let mut all_jl: Vec<String> = Vec::new();
    let t_all = Instant::now();

    // run1.txt: closure ladder, filt, lowmult, twist, residue_check
    for t in 2..8u32 {
        let cmax = if t < 7 { 6 } else { 4 };
        ctx.closure_cli(t, cmax, F1, (1, 1));
    }
    ctx.filt();
    ctx.lowmult_cli(2, 7);
    ctx.lowmult_cli(3, 6);
    ctx.twist();
    ctx.residue();
    flush_group(&mut ctx, dir, "run1.txt", &mut all_jl);

    // run2.txt: topfun (three generators), topfun_w
    for (t, c) in [
        (3, 2),
        (3, 3),
        (4, 3),
        (5, 3),
        (6, 3),
        (7, 3),
        (6, 4),
        (7, 4),
        (8, 4),
        (9, 4),
    ] {
        ctx.topfun(t, c, F1);
    }
    for (t, c) in [(3, 2), (3, 3), (4, 3), (5, 3), (6, 3), (7, 4), (8, 4)] {
        ctx.topfun(t, c, F2);
    }
    for (t, c) in [(4, 3), (8, 4)] {
        ctx.topfun(t, c, F3);
    }
    for (w, t, c) in [
        (2, 2, 3),
        (2, 3, 3),
        (2, 5, 3),
        (2, 5, 4),
        (2, 6, 4),
        (2, 7, 4),
        (3, 4, 4),
        (3, 6, 4),
        (3, 7, 4),
    ] {
        ctx.topfun_w(w, t, c, 1);
    }
    flush_group(&mut ctx, dir, "run2.txt", &mut all_jl);

    // run3.txt: lowsupp, hsupp2, thmB, s0pat
    ctx.lowsupp(8, 4);
    ctx.lowsupp(9, 4);
    for (t, c) in [(4, 3), (5, 3), (6, 3), (8, 4)] {
        ctx.hsupp2(t, c);
    }
    for (t, c) in [(4, 3), (5, 3), (6, 3), (8, 4)] {
        ctx.thmb(t, c);
    }
    ctx.s0pat(8, 4, &[]);
    flush_group(&mut ctx, dir, "run3.txt", &mut all_jl);

    // separate outputs
    ctx.hsupp2(9, 4);
    flush_group(&mut ctx, dir, "hsupp2_t9_c4.txt", &mut all_jl);
    ctx.s0pat(9, 4, &[3]);
    flush_group(&mut ctx, dir, "s0pat_t9_c4.txt", &mut all_jl);
    ctx.mult2_cli(2, 6);
    flush_group(&mut ctx, dir, "mult2_d2_s6.txt", &mut all_jl);

    // gridcheck: README claims f1 has no zero on mu_{2^9}^2, f2 and f3 none at t <= 8
    for t in 2..=9 {
        ctx.gridcheck(t, F1);
    }
    for t in 2..=8 {
        ctx.gridcheck(t, F2);
        ctx.gridcheck(t, F3);
    }
    flush_group(&mut ctx, dir, "gridcheck.txt", &mut all_jl);

    let mut body = all_jl.join("\n");
    body.push('\n');
    fs::write(dir.join("results.jsonl"), body).expect("write results.jsonl");
    eprintln!("[all done in {:.1}s]", t_all.elapsed().as_secs_f64());
}

fn num<T: std::str::FromStr>(args: &[String], i: usize) -> T {
    args.get(i)
        .unwrap_or_else(|| panic!("missing argument {i}"))
        .parse()
        .unwrap_or_else(|_| panic!("bad argument {i}"))
}

fn cf_from(args: &[String], i: usize) -> [u64; 4] {
    if args.len() >= i + 4 {
        [
            num(args, i),
            num(args, i + 1),
            num(args, i + 2),
            num(args, i + 3),
        ]
    } else {
        F1
    }
}

fn run_single(cmd: &str, a: &[String]) {
    let mut ctx = Ctx::new();
    match cmd {
        "closure" => {
            let zeta = if a.len() >= 8 {
                (num(a, 6), num(a, 7))
            } else {
                (1, 1)
            };
            ctx.closure_cli(num(a, 0), num(a, 1), cf_from(a, 2), zeta);
        }
        "filt" => ctx.filt(),
        "twist" => ctx.twist(),
        "residue" => ctx.residue(),
        "lowmult" => ctx.lowmult_cli(num(a, 0), num(a, 1)),
        "mult2" => ctx.mult2_cli(num(a, 0), num(a, 1)),
        "topfun" => ctx.topfun(num(a, 0), num(a, 1), cf_from(a, 2)),
        "topfun_w" => ctx.topfun_w(num(a, 0), num(a, 1), num(a, 2), num(a, 3)),
        "lowsupp" => ctx.lowsupp(num(a, 0), num(a, 1)),
        "hsupp2" => ctx.hsupp2(num(a, 0), num(a, 1)),
        "thmb" => ctx.thmb(num(a, 0), num(a, 1)),
        "s0pat" => {
            let d: Vec<u32> = (2..a.len()).map(|i| num(a, i)).collect();
            ctx.s0pat(num(a, 0), num(a, 1), &d);
        }
        "gridcheck" => ctx.gridcheck(num(a, 0), cf_from(a, 1)),
        other => {
            eprintln!("unknown command {other}");
            std::process::exit(2);
        }
    }
    print!("{}", ctx.text);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("all") => {
            let dir = args.get(1).expect("usage: growth_lower_bound all OUTDIR");
            run_all(Path::new(dir));
        }
        Some(cmd) => run_single(cmd, &args[1..]),
        None => {
            eprintln!("usage: growth_lower_bound all OUTDIR | <script> args (see the module docs)");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mt19937_reference_vectors() {
        // mt19937ar.out: init_by_array({0x123, 0x234, 0x345, 0x456})
        let mut m = Mt::init_by_array(&[0x123, 0x234, 0x345, 0x456]);
        let want = [
            1_067_595_299u32,
            955_945_823,
            477_289_528,
            4_107_218_783,
            4_228_976_476,
        ];
        for w in want {
            assert_eq!(m.next_u32(), w);
        }
        // init_genrand(5489): the reference seed of the C implementation.
        let mut m = Mt::init_genrand(5489);
        assert_eq!(m.next_u32(), 3_499_211_612);
        // CPython: random.seed(1); random.random() == 0.13436424411240122
        let mut m = Mt::from_seed(1);
        assert_eq!(m.random(), 0.134_364_244_112_401_22);
    }

    #[test]
    fn ring_sizes_match_binomials() {
        let r = Ring::new(4, 4);
        assert_eq!(r.by_degree(), vec![1, 8, 28, 56, 70]);
        assert_eq!(r.mons[r.n - 1], (0, 0));
        let r = Ring::new(2, 3);
        assert_eq!(r.by_degree(), vec![1, 4, 6, 4]);
    }

    #[test]
    fn ntt_roundtrip_and_convolution() {
        let t = 3u32;
        let ntt = Ntt::new(t);
        let k = ntt.k;
        let mut mt = Mt::from_seed(3);
        let a: Vec<u64> = (0..k * k).map(|_| mt.randbelow(P)).collect();
        let mut b = a.clone();
        ntt.transform2d(&mut b, false);
        ntt.transform2d(&mut b, true);
        let s = invm((k * k) as u64);
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(*x, y * s % P);
        }
    }

    #[test]
    fn finv_is_inverse_in_ring() {
        let t = 3u32;
        let ntt = Ntt::new(t);
        let k = ntt.k;
        let coeffs = coeffs_of(F1);
        let fi = build_finv(&ntt, &coeffs).expect("unit");
        // f * f^{-1} = 1 in R (cyclic convolution)
        let mut prod = vec![0u64; k * k];
        for &((a, b), v) in &coeffs {
            for x in 0..k {
                for y in 0..k {
                    let i = (x + a as usize) % k * k + (y + b as usize) % k;
                    prod[i] = (prod[i] + v * fi.fi[x * k + y]) % P;
                }
            }
        }
        for (i, &v) in prod.iter().enumerate() {
            assert_eq!(v, u64::from(i == 0));
        }
    }

    #[test]
    fn ladder_matches_recorded_python_values() {
        // d_F(f1) = 3, 3, 4, 4 at t = 2..5 (README 6.1) and the recorded closure dimensions.
        let mut ctx = Ctx::new();
        let coeffs = coeffs_of(F1);
        let want = [(2u32, 3u32), (3, 3), (4, 4), (5, 4)];
        for (t, d) in want {
            assert_eq!(ctx.last_fall(t, 6, &coeffs, (1, 1), false), Some(d));
        }
        let cl = ctx.closure(4, 3, &coeffs, (1, 1));
        assert_eq!((cl.dim(), cl.ring.n, cl.has_one()), (70, 93, false));
        assert_eq!(cl.dims_by_pivot_degree(3), vec![0, 0, 14, 56]);
        let cl = ctx.closure(5, 4, &coeffs, (1, 1));
        assert_eq!((cl.dim(), cl.ring.n, cl.has_one()), (386, 386, true));
    }

    #[test]
    fn generic_twist_does_not_change_d_f() {
        // Proposition 1: a generic twist gives the same d_F.
        let mut ctx = Ctx::new();
        let coeffs = coeffs_of(F1);
        for t in [2u32, 3, 4] {
            let a = ctx.last_fall(t, 5, &coeffs, (1, 1), false);
            let b = ctx.last_fall(t, 5, &coeffs, (5, 7), false);
            assert_eq!(a, b);
        }
    }

    #[test]
    fn lambda_kills_exactly_when_one_absent() {
        let mut ctx = Ctx::new();
        let coeffs = coeffs_of(F1);
        for (t, c) in [(3u32, 2u32), (3, 3), (4, 3)] {
            let ntt = Ntt::new(t);
            let fi = build_finv(&ntt, &coeffs).expect("unit");
            let cl = ctx.closure(t, c, &coeffs, (1, 1));
            assert_eq!(lambda_kills(&cl, &fi), !cl.has_one());
        }
    }

    #[test]
    fn used_set_matches_direct_coefficients() {
        let mut ctx = Ctx::new();
        let (t, c) = (3u32, 2u32);
        let coeffs = coeffs_of(F1);
        let ntt = Ntt::new(t);
        let fi = build_finv(&ntt, &coeffs).expect("unit");
        let cl = ctx.closure(t, c, &coeffs, (1, 1));
        let rows: Vec<usize> = (0..cl.dim()).collect();
        let used = used_set(&cl, &fi, &ntt, &rows);
        let k = ntt.k;
        let mut direct = vec![false; k * k];
        let mut g = vec![0u64; k * k];
        for &rid in &rows {
            cl.row_to_grid(rid, &mut g);
            for ta in 0..k {
                for tb in 0..k {
                    let mut s = 0u64;
                    for a in 0..k {
                        for b in 0..k {
                            s = (s + g[a * k + b]
                                * fi.fi[((ta + k - a) % k) * k + (tb + k - b) % k])
                                % P;
                        }
                    }
                    if s != 0 {
                        direct[ta * k + tb] = true;
                    }
                }
            }
        }
        assert_eq!(used, direct);
    }

    #[test]
    fn lowmult_recorded_values() {
        assert_eq!(lowmult(2, 2), (4, 11, Some(2)));
        assert_eq!(lowmult(2, 3), (9, 22, Some(4)));
        assert_eq!(lowmult(3, 2), (8, 15, Some(3)));
    }

    #[test]
    fn rref_kernel_small() {
        let a = vec![vec![1u64, 2, 3], vec![2, 4, 6]];
        let (r, piv) = rref(a, 3);
        assert_eq!(piv, vec![0]);
        assert_eq!(r, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn residue_identity_holds_t2() {
        let mut ctx = Ctx::new();
        ctx.residue();
        assert!(ctx.text.lines().take(3).all(|l| l.ends_with("True")));
    }
}
