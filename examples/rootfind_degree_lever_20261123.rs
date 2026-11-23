//! Degree lever for binary Koblitz summation-polynomial root finding:
//! does the 2-torsion (Artin–Schreier) symmetrised frame change how the
//! **last fall degree** of the Weil-descended decomposition system grows
//! with `n`, against the production `x`-frame systems at matched `F_2`-
//! dimension `ℓ = n/3`?
//!
//! Study: `research/rootfind_degree_lever_20261123/` (PROTOCOL.md is frozen
//! before any arm data). All numbers it prints are `[pcc:unverified]`, L0.
//!
//! **Self-contained (std only).** The shared `target/` could not take a full
//! `crypto_lib` rebuild (1.2 GB free, ~0.7 GB lib artifact, three agents), so
//! this file re-implements — and gates against — the parts of the existing
//! instrument it needs:
//!
//! * the field `F_{2^n}` with the library's `find_irreducible_sparse` choice
//!   (numerically smallest irreducible `x^n + low` with at most 3 extra bits);
//! * the Weil descent of `build_decomposition_system` (`S_3` chain, variable
//!   layout `m·ℓ` summand bits then `(m−2)·n` intermediate bits), symbolic;
//! * the direct `S_4` (`koblitz_symmetrised::plain_terms`, 24 terms, `b = 1`)
//!   and the symmetrised `S_4`/`S_3` (`symmetrised_terms`, 18/5 terms over
//!   `(w_1..w_m, w_R, s)`, frame `u = 1/(x+1)`, `w = u² + u`), descended by
//!   exact truth table + Möbius transform (ANF);
//! * the HKY mutant closure of `examples/hky_par.rs` /
//!   `examples/binary_falldeg_arms.rs` (`closure_par`), single-threaded, same
//!   monomial indexing, same `d_F` = least `c` with `1 ∈ V_{F,c}`.
//!
//! Gate `replay` re-derives every `d_F` and per-degree closure dimension of
//! `research/binary_falldeg_20261118/raw/pilot.jsonl` (72 rows, produced by the
//! library-based instrument) from its `(m, n, V, x_R)`.
//!
//! Usage:
//!   rootfind_degree_lever selftest
//!   rootfind_degree_lever replay <file>      (lines: m n x_r d_F|- c:plain:closed,... basis...)
//!   rootfind_degree_lever posctl --seed S --specs k:e,... --keep K --cmax C
//!   rootfind_degree_lever run --cells m:n:ell,... --seed S --keep K --arms a,b,..
//!        [--cmax-chain C] [--cmax C] [--max-cols N] [--budget SECS]

use std::collections::HashMap;
use std::time::Instant;

// ───────────────────────────── RNG (splitmix64) ─────────────────────────────

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

fn cell_seed(master: u64, tag: &str, n: u32, ell: usize, m: usize, draw: usize) -> u64 {
    let mut h = master
        ^ (u64::from(n) << 40)
        ^ ((ell as u64) << 32)
        ^ ((m as u64) << 48)
        ^ ((draw as u64) << 16);
    for b in tag.bytes() {
        h = h.wrapping_mul(0x100000001b3).wrapping_add(b as u64);
    }
    h
}

// ───────────────────────────── field F_{2^n} ─────────────────────────────

#[derive(Clone)]
struct Field {
    n: u32,
    low: u64, // f = x^n + low
    sq: Vec<u64>, // sq[j] = z^{2j} mod f, j < n
}

fn pmulmod(a: u64, b: u64, n: u32, low: u64) -> u64 {
    // multiply then reduce mod x^n + low
    let mut r: u128 = 0;
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            r ^= (a as u128) << i;
        }
    }
    reduce(r, n, low)
}

fn reduce(mut r: u128, n: u32, low: u64) -> u64 {
    let mut k = 127i32;
    while k >= n as i32 {
        if (r >> k) & 1 == 1 {
            r ^= 1u128 << k;
            r ^= (low as u128) << (k as u32 - n);
        }
        k -= 1;
    }
    r as u64
}

/// `f = x^n + low` irreducible over F_2 (Ben-Or style: no factor of degree ≤ n/2).
fn is_irreducible(n: u32, low: u64) -> bool {
    // x^{2^i} mod f; f irreducible iff gcd(x^{2^i} - x, f) = 1 for i = 1..n/2
    let mut xp: u64 = if n > 1 { 2 } else { 0 };
    for _ in 1..=n / 2 {
        xp = pmulmod(xp, xp, n, low);
        let g = pgcd(xp ^ 2, n, low);
        if g != 1 {
            return false;
        }
    }
    true
}

fn pdeg(a: u128) -> i32 {
    if a == 0 {
        -1
    } else {
        127 - a.leading_zeros() as i32
    }
}

fn pgcd(a: u64, n: u32, low: u64) -> u128 {
    let mut x: u128 = (1u128 << n) | low as u128;
    let mut y: u128 = a as u128;
    while y != 0 {
        // x mod y
        while pdeg(x) >= pdeg(y) {
            x ^= y << (pdeg(x) - pdeg(y));
        }
        std::mem::swap(&mut x, &mut y);
    }
    x
}

impl Field {
    fn new(n: u32) -> Field {
        // library order: candidates 1, 1|1<<i, 1|1<<i|1<<j, 1|..|1<<k sorted
        let mut cands: Vec<u64> = vec![1];
        for i in 1..n {
            cands.push(1 | (1 << i));
            for j in (i + 1)..n {
                cands.push(1 | (1 << i) | (1 << j));
                for k in (j + 1)..n {
                    cands.push(1 | (1 << i) | (1 << j) | (1 << k));
                }
            }
        }
        cands.sort_unstable();
        let low = *cands
            .iter()
            .find(|&&l| is_irreducible(n, l))
            .expect("irreducible");
        let sq = (0..n).map(|j| pmulmod(1 << j, 1 << j, n, low)).collect();
        Field { n, low, sq }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        pmulmod(a, b, self.n, self.low)
    }
    fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn pow(&self, a: u64, mut e: u64) -> u64 {
        let (mut r, mut b) = (1u64, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.sqr(b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        self.pow(a, (1u64 << self.n) - 2)
    }
    fn mask(&self) -> u64 {
        (1u64 << self.n) - 1
    }
}

// ───────────────────── Boolean polynomials (multilinear) ─────────────────────

type Poly = Vec<u64>; // sorted unique monomial masks

fn padd(a: &Poly, b: &Poly) -> Poly {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            out.push(a[i]);
            i += 1;
        } else if a[i] > b[j] {
            out.push(b[j]);
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

fn pmul(a: &Poly, b: &Poly) -> Poly {
    let mut acc: HashMap<u64, bool> = HashMap::new();
    for &x in a {
        for &y in b {
            let e = acc.entry(x | y).or_insert(false);
            *e = !*e;
        }
    }
    let mut out: Vec<u64> = acc.into_iter().filter(|(_, v)| *v).map(|(k, _)| k).collect();
    out.sort_unstable();
    out
}

/// Symbolic field element: `n` Boolean coordinate polynomials.
#[derive(Clone)]
struct Sym(Vec<Poly>);

impl Sym {
    fn constant(v: u64, n: u32) -> Sym {
        Sym((0..n).map(|k| if (v >> k) & 1 == 1 { vec![0] } else { vec![] }).collect())
    }
    fn subspace(basis: &[u64], off: usize, n: u32) -> Sym {
        Sym((0..n)
            .map(|k| {
                let mut p: Poly = basis
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| (*b >> k) & 1 == 1)
                    .map(|(t, _)| 1u64 << (off + t))
                    .collect();
                p.sort_unstable();
                p
            })
            .collect())
    }
    fn free(off: usize, n: u32) -> Sym {
        Sym((0..n as usize).map(|k| vec![1u64 << (off + k)]).collect())
    }
    fn add(&self, o: &Sym) -> Sym {
        Sym(self.0.iter().zip(o.0.iter()).map(|(a, b)| padd(a, b)).collect())
    }
    fn mul(&self, o: &Sym, f: &Field) -> Sym {
        let n = f.n as usize;
        let mut tmp: Vec<Poly> = vec![vec![]; 2 * n];
        for i in 0..n {
            if self.0[i].is_empty() {
                continue;
            }
            for j in 0..n {
                if o.0[j].is_empty() {
                    continue;
                }
                let p = pmul(&self.0[i], &o.0[j]);
                tmp[i + j] = padd(&tmp[i + j], &p);
            }
        }
        for k in (n..2 * n).rev() {
            if tmp[k].is_empty() {
                continue;
            }
            let t = std::mem::take(&mut tmp[k]);
            for b in 0..n {
                if (f.low >> b) & 1 == 1 {
                    tmp[k - n + b] = padd(&tmp[k - n + b], &t);
                }
            }
        }
        tmp.truncate(n);
        Sym(tmp)
    }
    fn square(&self, f: &Field) -> Sym {
        let n = f.n as usize;
        let mut out: Vec<Poly> = vec![vec![]; n];
        for j in 0..n {
            if self.0[j].is_empty() {
                continue;
            }
            for k in 0..n {
                if (f.sq[j] >> k) & 1 == 1 {
                    out[k] = padd(&out[k], &self.0[j]);
                }
            }
        }
        Sym(out)
    }
}

/// `S_3(a,b,c) = (a+b)² c² + a b c + (a b)² + 1` (binary, `b_curve = 1`).
fn sym_s3(a: &Sym, b: &Sym, c: &Sym, f: &Field) -> Vec<Poly> {
    let ab = a.mul(b, f);
    let t1 = a.add(b).square(f).mul(&c.square(f), f);
    let t2 = ab.mul(c, f);
    let t3 = ab.square(f);
    let one = Sym::constant(1, f.n);
    t1.add(&t2).add(&t3).add(&one).0
}

/// The production chain (`build_decomposition_system` layout).
fn chain_system(basis: &[u64], x_r: u64, m: usize, f: &Field) -> (Vec<Poly>, u32) {
    let ell = basis.len();
    let n = f.n;
    let nv = m * ell + (m - 2) * n as usize;
    let s: Vec<Sym> = (0..m).map(|i| Sym::subspace(basis, i * ell, n)).collect();
    let t = Sym::constant(x_r, n);
    let mut eqs = vec![];
    if m == 2 {
        eqs.extend(sym_s3(&s[0], &s[1], &t, f));
    } else {
        let inter: Vec<Sym> = (0..m - 2).map(|i| Sym::free(m * ell + i * n as usize, n)).collect();
        eqs.extend(sym_s3(&s[0], &s[1], &inter[0], f));
        for i in 0..m - 3 {
            eqs.extend(sym_s3(&inter[i], &s[i + 2], &inter[i + 1], f));
        }
        eqs.extend(sym_s3(&inter[m - 3], &s[m - 1], &t, f));
    }
    (eqs, nv as u32)
}

// ─────────────── truth-table descents (direct S4, symmetrised) ───────────────

/// `plain_terms(3)`: S4 in x over (x1,x2,x3,xR), b = 1, all coefficients 1.
const PLAIN3: [[u32; 4]; 24] = [
    [4, 4, 4, 0], [4, 4, 0, 4], [4, 0, 4, 4], [0, 4, 4, 4], [4, 2, 2, 2], [3, 3, 3, 1],
    [3, 3, 1, 3], [3, 1, 3, 3], [2, 4, 2, 2], [2, 2, 4, 2], [2, 2, 2, 4], [1, 3, 3, 3],
    [3, 1, 1, 1], [2, 2, 2, 0], [2, 2, 0, 2], [2, 0, 2, 2], [1, 3, 1, 1], [1, 1, 3, 1],
    [1, 1, 1, 3], [0, 2, 2, 2], [4, 0, 0, 0], [0, 4, 0, 0], [0, 0, 4, 0], [0, 0, 0, 4],
];
/// `symmetrised_terms(2)` over (w1, w2, wR, s).
const SYM2: [[u32; 4]; 5] = [[1, 1, 1, 0], [1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
/// `symmetrised_terms(3)` over (w1, w2, w3, wR, s).
const SYM3: [[u32; 5]; 18] = [
    [2, 2, 2, 0, 0], [2, 2, 0, 2, 0], [2, 0, 2, 2, 0], [0, 2, 2, 2, 0], [2, 1, 1, 1, 0],
    [1, 2, 1, 1, 0], [1, 1, 2, 1, 0], [1, 1, 1, 2, 0], [1, 1, 1, 1, 1], [2, 0, 0, 0, 0],
    [0, 2, 0, 0, 0], [0, 0, 2, 0, 0], [0, 0, 0, 2, 0], [1, 0, 0, 0, 0], [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0], [0, 0, 0, 1, 0], [0, 0, 0, 0, 1],
];

fn eval_terms(f: &Field, vals: &[u64], terms: &[&[u32]]) -> u64 {
    let mut acc = 0u64;
    for t in terms {
        let mut p = 1u64;
        for (i, &e) in t.iter().enumerate() {
            if e > 0 {
                p = f.mul(p, f.pow(vals[i], e as u64));
            }
        }
        acc ^= p;
    }
    acc
}

fn plain3_eval(f: &Field, x: &[u64; 4]) -> u64 {
    let t: Vec<&[u32]> = PLAIN3.iter().map(|r| &r[..]).collect();
    eval_terms(f, x, &t)
}

fn span_values(basis: &[u64]) -> Vec<u64> {
    let mut v = vec![0u64; 1 << basis.len()];
    for mask in 1..v.len() {
        let t = mask.trailing_zeros() as usize;
        v[mask] = v[mask & (mask - 1)] ^ basis[t];
    }
    v
}

/// Truth table → ANF (Möbius); coordinate `k` of the system is the set of
/// masks whose ANF value has bit `k`. Returns equations and the zero count.
fn anf_system(mut tt: Vec<u64>, nv: u32, n: u32) -> (Vec<Poly>, u64) {
    let zeros = tt.iter().filter(|&&v| v == 0).count() as u64;
    for i in 0..nv {
        let bit = 1usize << i;
        for x in 0..tt.len() {
            if x & bit != 0 {
                tt[x] ^= tt[x ^ bit];
            }
        }
    }
    let mut eqs: Vec<Poly> = vec![vec![]; n as usize];
    for (mask, &v) in tt.iter().enumerate() {
        let mut w = v;
        while w != 0 {
            let k = w.trailing_zeros() as usize;
            eqs[k].push(mask as u64);
            w &= w - 1;
        }
    }
    (eqs, zeros)
}

/// Direct (unchained) `S_{m+1}` in x, m ∈ {2, 3}; layout `m·ℓ` bits.
fn direct_system(basis: &[u64], x_r: u64, m: usize, f: &Field) -> (Vec<Poly>, u32, u64) {
    let ell = basis.len();
    let nv = (m * ell) as u32;
    let vals = span_values(basis);
    let size = 1usize << nv;
    let mut tt = vec![0u64; size];
    let xr2 = f.sqr(x_r);
    if m == 2 {
        for a in 0..size {
            let x1 = vals[a & ((1 << ell) - 1)];
            let x2 = vals[a >> ell];
            let p = f.mul(x1, x2);
            tt[a] = f.mul(f.sqr(x1 ^ x2), xr2) ^ f.mul(p, x_r) ^ f.sqr(p) ^ 1;
        }
    } else {
        // precompute: terms factor as monomials; evaluate per point
        let lm = (1usize << ell) - 1;
        let pw: Vec<[u64; 5]> = vals
            .iter()
            .map(|&v| {
                let v2 = f.sqr(v);
                [1, v, v2, f.mul(v2, v), f.sqr(v2)]
            })
            .collect();
        let r = [1, x_r, xr2, f.mul(xr2, x_r), f.sqr(xr2)];
        for a in 0..size {
            let p = [&pw[a & lm], &pw[(a >> ell) & lm], &pw[a >> (2 * ell)]];
            let mut acc = 0u64;
            for t in PLAIN3.iter() {
                let q = f.mul(f.mul(p[0][t[0] as usize], p[1][t[1] as usize]), f.mul(p[2][t[2] as usize], r[t[3] as usize]));
                acc ^= q;
            }
            tt[a] = acc;
        }
    }
    let (eqs, zeros) = anf_system(tt, nv, f.n);
    (eqs, nv, zeros)
}

/// The symmetrised system (`build_symmetrised_system` layout): bits
/// `c_{i,t}` (`i < m`, `t < ℓ−1`) then `ε`; basis[0] must be 1.
fn sym_system(basis: &[u64], x_r: u64, m: usize, f: &Field) -> Option<(Vec<Poly>, u32, u64)> {
    assert_eq!(basis[0], 1);
    let ellw = basis.len() - 1;
    let nv = (m * ellw + 1) as u32;
    let u_r = f.inv(x_r ^ 1);
    if u_r == 0 || x_r == 1 {
        return None;
    }
    let w_r = f.sqr(u_r) ^ u_r;
    let vb: Vec<u64> = basis[1..].to_vec();
    let wb: Vec<u64> = vb.iter().map(|&b| f.sqr(b) ^ b).collect();
    let uvals = span_values(&vb);
    let wvals = span_values(&wb);
    let lm = (1usize << ellw) - 1;
    let size = 1usize << nv;
    let mut tt = vec![0u64; size];
    for a in 0..size {
        let eps = ((a >> (m * ellw)) & 1) as u64;
        let mut s = eps ^ u_r;
        let mut v = vec![0u64; m + 2];
        for i in 0..m {
            let c = (a >> (i * ellw)) & lm;
            s ^= uvals[c];
            v[i] = wvals[c];
        }
        v[m] = w_r;
        v[m + 1] = s;
        tt[a] = if m == 2 {
            let t: Vec<&[u32]> = SYM2.iter().map(|r| &r[..]).collect();
            eval_terms(f, &v, &t)
        } else {
            let t: Vec<&[u32]> = SYM3.iter().map(|r| &r[..]).collect();
            eval_terms(f, &v, &t)
        };
    }
    let (eqs, zeros) = anf_system(tt, nv, f.n);
    Some((eqs, nv, zeros))
}

// ─────────── HKY mutant closure (copied from binary_falldeg_arms.rs) ───────────

struct Space {
    c: u32,
    binom: Vec<Vec<u64>>,
    off: Vec<usize>,
    monos: Vec<u64>,
    words: usize,
}

impl Space {
    fn new(nv: u32, c: u32) -> Space {
        let mut binom = vec![vec![0u64; c as usize + 2]; nv as usize + 1];
        for n in 0..=nv as usize {
            binom[n][0] = 1;
            for k in 1..=c as usize + 1 {
                binom[n][k] = if n == 0 { 0 } else { binom[n - 1][k - 1] + binom[n - 1][k] };
            }
        }
        let mut off = vec![0usize; c as usize + 2];
        for d in 0..=c as usize {
            off[d + 1] = off[d] + binom[nv as usize][d] as usize;
        }
        let total = off[c as usize + 1];
        let mut monos = vec![0u64; total];
        for d in 0..=c {
            let mut m: u64 = if d == 0 { 0 } else { (1u64 << d) - 1 };
            let base = off[d as usize];
            for slot in monos[base..off[d as usize + 1]].iter_mut() {
                *slot = m;
                if d == 0 {
                    break;
                }
                let t = m | (m - 1);
                m = (t + 1) | (((!t & (t + 1)) - 1) >> (m.trailing_zeros() + 1));
            }
        }
        let words = total.div_ceil(64);
        Space { c, binom, off, monos, words }
    }
    #[inline]
    fn col(&self, m: u64) -> usize {
        let d = m.count_ones() as usize;
        let (mut r, mut x, mut j) = (0u64, m, 1usize);
        while x != 0 {
            let b = x.trailing_zeros() as usize;
            r += self.binom[b][j];
            x &= x - 1;
            j += 1;
        }
        self.off[d] + r as usize
    }
    fn deg_of_col(&self, col: usize) -> u32 {
        self.monos[col].count_ones()
    }
    fn row(&self, poly: &[u64]) -> Vec<u64> {
        let mut r = vec![0u64; self.words];
        for &m in poly {
            let i = self.col(m);
            r[i / 64] ^= 1 << (i % 64);
        }
        r
    }
    fn times_var(&self, row: &[u64], i: u32) -> Option<Vec<u64>> {
        let mut out = vec![0u64; self.words];
        let bit = 1u64 << i;
        for (w, &word) in row.iter().enumerate() {
            let mut b = word;
            while b != 0 {
                let m = self.monos[w * 64 + b.trailing_zeros() as usize] | bit;
                b &= b - 1;
                if m.count_ones() > self.c {
                    return None;
                }
                let j = self.col(m);
                out[j / 64] ^= 1 << (j % 64);
            }
        }
        Some(out)
    }
}

fn top(v: &[u64]) -> Option<usize> {
    v.iter()
        .enumerate()
        .rev()
        .find(|(_, &w)| w != 0)
        .map(|(i, &w)| i * 64 + 63 - w.leading_zeros() as usize)
}

struct Basis {
    piv: Vec<Option<Box<[u64]>>>,
    dim: usize,
}

impl Basis {
    fn reduce_full(&self, mut v: Vec<u64>) -> Option<Vec<u64>> {
        loop {
            let t = top(&v)?;
            if let Some(p) = &self.piv[t] {
                for (a, b) in v.iter_mut().zip(p.iter()) {
                    *a ^= *b;
                }
            } else {
                return Some(v);
            }
        }
    }
    fn absorb_one(&mut self, v: Vec<u64>) -> Option<usize> {
        if let Some(mut r) = self.reduce_full(v) {
            if let Some(t) = top(&r) {
                r.truncate(t / 64 + 1);
                self.piv[t] = Some(r.into_boxed_slice());
                self.dim += 1;
                return Some(t);
            }
        }
        None
    }
}

/// `(plain dim after the Macaulay round, closure dim, rounds, timed_out)`.
/// Same algorithm as `closure_par` (rows are absorbed one at a time instead
/// of chunk-reduced in parallel; the spanned space after each round is the
/// same, so `plain`, `closed` and the `1 ∈ V` verdict are identical).
fn closure(eqs: &[Poly], nv: u32, c: u32, deadline: Instant) -> (usize, usize, u32, bool) {
    let sp = Space::new(nv, c);
    let total = sp.monos.len();
    let mut basis = Basis { piv: vec![None; total], dim: 0 };
    enum Src {
        Mac(usize, u64),
        Mul(usize, u32),
    }
    let bases: Vec<Option<Vec<u64>>> = eqs
        .iter()
        .map(|f| (!f.is_empty() && f.iter().map(|m| m.count_ones()).max().unwrap_or(0) <= c).then(|| sp.row(f)))
        .collect();
    let mut srcs: Vec<Src> = vec![];
    for (k, f) in eqs.iter().enumerate() {
        if bases[k].is_none() {
            continue;
        }
        let df = f.iter().map(|m| m.count_ones()).max().unwrap_or(0);
        srcs.extend(sp.monos.iter().filter(|m| m.count_ones() <= c - df).map(|&m| Src::Mac(k, m)));
    }
    let mut plain = None;
    let mut rounds = 0u32;
    let mut cnt = 0u64;
    loop {
        rounds += 1;
        let mut new_low: Vec<usize> = vec![];
        for s in &srcs {
            cnt += 1;
            if cnt % 4096 == 0 && Instant::now() > deadline {
                return (plain.unwrap_or(basis.dim), basis.dim, rounds, true);
            }
            let v = match *s {
                Src::Mac(k, m) => {
                    let mut r = bases[k].clone().unwrap();
                    let mut ok = true;
                    for i in 0..nv {
                        if m >> i & 1 == 1 {
                            match sp.times_var(&r, i) {
                                Some(x) => r = x,
                                None => {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                    }
                    if !ok {
                        continue;
                    }
                    r
                }
                Src::Mul(t, i) => {
                    let mut full = basis.piv[t].as_ref().unwrap().to_vec();
                    full.resize(sp.words, 0);
                    match sp.times_var(&full, i) {
                        Some(x) => x,
                        None => continue,
                    }
                }
            };
            if let Some(t) = basis.absorb_one(v) {
                if sp.deg_of_col(t) < c {
                    new_low.push(t);
                }
            }
            if basis.piv[0].is_some() {
                break;
            }
        }
        if plain.is_none() {
            plain = Some(basis.dim);
        }
        if basis.piv[0].is_some() {
            return (plain.unwrap(), total, rounds, false);
        }
        if new_low.is_empty() {
            return (plain.unwrap(), basis.dim, rounds, false);
        }
        srcs = new_low.iter().flat_map(|&t| (0..nv).map(move |i| Src::Mul(t, i))).collect();
    }
}

fn cols_upto(nv: u32, c: u32) -> u64 {
    let mut s = 0u64;
    let mut b = 1u64;
    for d in 0..=c as u64 {
        if d > 0 {
            b = b * (nv as u64 - d + 1) / d;
        }
        s += b;
    }
    s
}

/// `d_F` with censoring. Returns (d_F, censor_reason, per-degree json).
fn last_fall(eqs: &[Poly], nv: u32, cmax: u32, max_cols: u64, budget: f64) -> (Option<u32>, Option<String>, String) {
    let deadline = Instant::now() + std::time::Duration::from_secs_f64(budget);
    let mut per = vec![];
    for c in 2..=cmax {
        let cols = cols_upto(nv, c);
        if cols > max_cols {
            return (None, Some(format!("cols>{max_cols} at c={c}")), per.join(","));
        }
        let t0 = Instant::now();
        let (plain, closed, rounds, timed_out) = closure(eqs, nv, c, deadline);
        per.push(format!(
            "\"{c}\":{{\"plain\":{plain},\"closed\":{closed},\"of\":{cols},\"rounds\":{rounds},\"secs\":{:.3}}}",
            t0.elapsed().as_secs_f64()
        ));
        if timed_out {
            return (None, Some(format!("budget at c={c}")), per.join(","));
        }
        if closed as u64 == cols {
            return (Some(c), None, per.join(","));
        }
    }
    (None, Some(format!("cmax={cmax}")), per.join(","))
}

/// Boolean semi-regular degree of regularity: first non-positive coefficient
/// of `(1+z)^k / Π (1+z^{d_i})`.
fn semireg_dreg(k: u32, degs: &[u32]) -> u32 {
    let len = (k + 2) as usize;
    let mut s: Vec<i128> = vec![0; len];
    let mut b: i128 = 1;
    for d in 0..len {
        if d as u32 <= k {
            s[d] = b;
            b = b * (k as i128 - d as i128) / (d as i128 + 1);
        }
    }
    for &d in degs {
        // divide by (1 + z^d): s'[i] = s[i] - s'[i-d]
        let d = d as usize;
        for i in d..len {
            s[i] -= s[i - d];
        }
    }
    (0..len).find(|&i| s[i] <= 0).map_or(k + 1, |i| i as u32)
}

fn sys_degree(eqs: &[Poly]) -> u32 {
    eqs.iter().flat_map(|p| p.iter()).map(|m| m.count_ones()).max().unwrap_or(0)
}

// ─────────────────────────────── drivers ───────────────────────────────

fn getflag(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned()
}

fn random_basis_with_one(f: &Field, ell: usize, rng: &mut Rng) -> Vec<u64> {
    // uniformly random ℓ-dim subspace containing 1 (basis with 1 first)
    let mut basis = vec![1u64];
    let mut ech: Vec<u64> = vec![1];
    while basis.len() < ell {
        let v = rng.next() & f.mask();
        let r = ech.iter().fold(v, |r, &x| r.min(r ^ x));
        if r != 0 {
            ech.push(r);
            ech.sort_unstable_by(|a, b| b.cmp(a));
            basis.push(v);
        }
    }
    basis
}

fn window_basis(ell: usize) -> Vec<u64> {
    (0..ell).map(|k| 1u64 << k).collect()
}

fn run(args: &[String]) {
    let cells = getflag(args, "--cells").expect("--cells");
    let seed: u64 = getflag(args, "--seed").map_or(20261123, |v| v.parse().unwrap());
    let keep: usize = getflag(args, "--keep").map_or(4, |v| v.parse().unwrap());
    let cmax: u32 = getflag(args, "--cmax").map_or(8, |v| v.parse().unwrap());
    let cmax_chain: u32 = getflag(args, "--cmax-chain").map_or(5, |v| v.parse().unwrap());
    let max_cols: u64 = getflag(args, "--max-cols").map_or(400_000, |v| v.parse().unwrap());
    let budget: f64 = getflag(args, "--budget").map_or(600.0, |v| v.parse().unwrap());
    let arms: Vec<String> = getflag(args, "--arms")
        .unwrap_or_else(|| "xchain-rand,xchain-win,xdirect-rand,sym-rand,sym-rand2,sym-win".into())
        .split(',')
        .map(String::from)
        .collect();
    for cell in cells.split(',') {
        let p: Vec<usize> = cell.split(':').map(|x| x.parse().unwrap()).collect();
        let (m, n, ell) = (p[0], p[1] as u32, p[2]);
        let f = Field::new(n);
        let (mut kept, mut attempts) = (0usize, 0usize);
        while kept < keep && attempts < 400 {
            let d = attempts;
            attempts += 1;
            let mut r = Rng(cell_seed(seed, "xr", n, ell, m, d));
            let x_r = loop {
                let v = r.next() & f.mask();
                if v > 1 {
                    break v;
                }
            };
            let va = random_basis_with_one(&f, ell, &mut Rng(cell_seed(seed, "VA", n, ell, m, d)));
            let vb = random_basis_with_one(&f, ell, &mut Rng(cell_seed(seed, "VB", n, ell, m, d)));
            let vw = window_basis(ell);
            // Keep the draw only if every arm's system is inconsistent.
            // x-frame consistency is read off the direct system (chain roots ⊆ direct roots).
            let mut sys: Vec<(String, Vec<Poly>, u32, Vec<u64>)> = vec![];
            let mut ok = true;
            let mut cache_direct: HashMap<&str, (Vec<Poly>, u32, u64)> = HashMap::new();
            // `--sym-only` (exploratory, added after the primary runs): skip
            // the x-frame filter so the cheap u-frame arms can go past the
            // sizes where the x-frame systems fit in memory.
            let sym_only = args.iter().any(|a| a == "--sym-only");
            for (tag, b) in [("rand", &va), ("win", &vw)] {
                if sym_only {
                    break;
                }
                let ds = direct_system(b, x_r, m, &f);
                if ds.2 > 0 {
                    ok = false;
                    break;
                }
                cache_direct.insert(tag, ds);
            }
            if !ok {
                continue;
            }
            for arm in &arms {
                let entry = match arm.as_str() {
                    "xchain-rand" => {
                        let (e, nv) = chain_system(&va, x_r, m, &f);
                        Some((e, nv, va.clone()))
                    }
                    "xchain-win" => {
                        let (e, nv) = chain_system(&vw, x_r, m, &f);
                        Some((e, nv, vw.clone()))
                    }
                    "xdirect-rand" => {
                        let (e, nv, _) = cache_direct["rand"].clone();
                        Some((e, nv, va.clone()))
                    }
                    "xdirect-win" => {
                        let (e, nv, _) = cache_direct["win"].clone();
                        Some((e, nv, vw.clone()))
                    }
                    "sym-rand" | "sym-rand2" | "sym-win" => {
                        let b = match arm.as_str() {
                            "sym-rand" => &va,
                            "sym-rand2" => &vb,
                            _ => &vw,
                        };
                        match sym_system(b, x_r, m, &f) {
                            Some((e, nv, z)) if z == 0 => Some((e, nv, b.clone())),
                            _ => None,
                        }
                    }
                    _ => panic!("unknown arm {arm}"),
                };
                match entry {
                    Some((e, nv, b)) => sys.push((arm.clone(), e, nv, b)),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok {
                continue;
            }
            kept += 1;
            for (arm, eqs, nv, b) in &sys {
                let cm = if arm.starts_with("xchain") && m >= 3 { cmax_chain } else { cmax };
                let deg = sys_degree(eqs);
                let neq = eqs.iter().filter(|e| !e.is_empty()).count();
                let degs: Vec<u32> = eqs.iter().filter(|e| !e.is_empty()).map(|e| sys_degree(std::slice::from_ref(e))).collect();
                let sr = semireg_dreg(*nv, &degs);
                let t0 = Instant::now();
                let (d_f, cens, per) = last_fall(eqs, *nv, cm, max_cols, budget);
                let secs = t0.elapsed().as_secs_f64();
                let d_f_s = d_f.map_or("null".into(), |x| x.to_string());
                let cens_s = cens.map_or("null".into(), |x| format!("\"{x}\""));
                let cols = d_f.map_or("null".into(), |x| cols_upto(*nv, x).to_string());
                let bs: Vec<String> = b.iter().map(|x| x.to_string()).collect();
                println!(
                    "{{\"schema\":\"rootfind_degree_lever/v1\",\"arm\":\"{arm}\",\"m\":{m},\"n\":{n},\"ell\":{ell},\"draw\":{d},\"seed\":{seed},\"x_r\":{x_r},\"v_basis\":[{}],\"n_vars\":{nv},\"n_eqs\":{neq},\"input_degree\":{deg},\"semireg_dreg\":{sr},\"d_F\":{d_f_s},\"censored\":{cens_s},\"cols_at_dF\":{cols},\"secs\":{secs:.3},\"per_degree\":{{{per}}}}}",
                    bs.join(",")
                );
            }
        }
        eprintln!("cell {cell}: kept {kept}/{keep} in {attempts} attempts");
    }
}

fn posctl(args: &[String]) {
    // Random dense quadratic Boolean systems (positive control for the detector).
    let seed: u64 = getflag(args, "--seed").map_or(7, |v| v.parse().unwrap());
    let keep: usize = getflag(args, "--keep").map_or(3, |v| v.parse().unwrap());
    let cmax: u32 = getflag(args, "--cmax").map_or(7, |v| v.parse().unwrap());
    let specs = getflag(args, "--specs").expect("--specs k:e,...");
    for spec in specs.split(',') {
        let p: Vec<u32> = spec.split(':').map(|x| x.parse().unwrap()).collect();
        let (k, e) = (p[0], p[1]);
        let mut kept = 0;
        let mut d = 0usize;
        while kept < keep && d < 200 {
            let mut r = Rng(cell_seed(seed, "posctl", k, e as usize, 2, d));
            d += 1;
            let monos: Vec<u64> = (0..k)
                .flat_map(|i| (i..k).map(move |j| (1u64 << i) | (1u64 << j)))
                .chain((0..1).map(|_| 0u64))
                .collect();
            let eqs: Vec<Poly> = (0..e)
                .map(|_| {
                    let mut pl: Poly = monos.iter().copied().filter(|_| r.next() & 1 == 1).collect();
                    pl.sort_unstable();
                    pl.dedup();
                    pl
                })
                .collect();
            if eqs.iter().any(|p| p.iter().all(|m| m.count_ones() < 2)) {
                continue;
            }
            // consistency by exhaustive evaluation
            let consistent = (0u64..(1u64 << k)).any(|a| {
                eqs.iter().all(|p| p.iter().filter(|&&m| m & !a == 0).count() % 2 == 0)
            });
            if consistent {
                continue;
            }
            kept += 1;
            let degs = vec![2u32; e as usize];
            let sr = semireg_dreg(k, &degs);
            let t0 = Instant::now();
            let (d_f, cens, per) = last_fall(&eqs, k, cmax, 2_000_000, 600.0);
            println!(
                "{{\"schema\":\"rootfind_degree_lever.posctl/v1\",\"k\":{k},\"e\":{e},\"draw\":{},\"semireg_dreg\":{sr},\"d_F\":{},\"censored\":{},\"secs\":{:.3},\"per_degree\":{{{per}}}}}",
                d - 1,
                d_f.map_or("null".into(), |x| x.to_string()),
                cens.map_or("null".into(), |x| format!("\"{x}\"")),
                t0.elapsed().as_secs_f64()
            );
        }
    }
}

fn replay(path: &str) {
    // line: m n x_r dF(or -) c:plain:closed;... b0 b1 ...
    let text = std::fs::read_to_string(path).expect("read replay file");
    let (mut ok, mut bad) = (0, 0);
    for line in text.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 6 {
            continue;
        }
        let m: usize = t[0].parse().unwrap();
        let n: u32 = t[1].parse().unwrap();
        let x_r: u64 = t[2].parse().unwrap();
        let want_df: Option<u32> = t[3].parse().ok();
        let want_dims: Vec<(u32, usize, usize)> = t[4]
            .split(';')
            .map(|s| {
                let q: Vec<usize> = s.split(':').map(|x| x.parse().unwrap()).collect();
                (q[0] as u32, q[1], q[2])
            })
            .collect();
        let basis: Vec<u64> = t[5..].iter().map(|x| x.parse().unwrap()).collect();
        let f = Field::new(n);
        let (eqs, nv) = chain_system(&basis, x_r, m, &f);
        let cmax = want_dims.iter().map(|x| x.0).max().unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(3600);
        let mut got = vec![];
        let mut got_df = None;
        for c in 2..=cmax {
            let (plain, closed, _, _) = closure(&eqs, nv, c, deadline);
            got.push((c, plain, closed));
            if closed as u64 == cols_upto(nv, c) {
                got_df = Some(c);
                break;
            }
        }
        let same = got_df == want_df && got == want_dims;
        if same {
            ok += 1;
        } else {
            bad += 1;
            println!("MISMATCH m={m} n={n} x_r={x_r} want {want_df:?} {want_dims:?} got {got_df:?} {got:?}");
        }
    }
    println!("replay: {ok} match, {bad} mismatch");
}

fn selftest() {
    let mut fails = 0;
    // T1: the chosen x^n + low is irreducible by trial division (the
    // equality with the library's choice is gated by `replay`).
    for n in [9u32, 13, 15, 21] {
        let f = Field::new(n);
        let full: u128 = (1u128 << n) | f.low as u128;
        for d in 2u128..(1u128 << (n / 2 + 1)) {
            let mut a = full;
            while a != 0 && pdeg(a) >= pdeg(d) {
                a ^= d << (pdeg(a) - pdeg(d));
            }
            if a == 0 {
                println!("T1 n={n}: low={:#b} has factor {d:#b}", f.low);
                fails += 1;
                break;
            }
        }
    }
    // T2: chain m=2 symbolic == direct m=2 truth table (ANF equality)
    for n in [9u32, 11, 13] {
        let f = Field::new(n);
        let mut r = Rng(n as u64);
        let b = random_basis_with_one(&f, 4, &mut r);
        let x_r = (r.next() & f.mask()) | 2;
        let (e1, _) = chain_system(&b, x_r, 2, &f);
        let (e2, _, _) = direct_system(&b, x_r, 2, &f);
        if e1 != e2 {
            println!("T2 n={n}: chain m=2 != direct m=2");
            fails += 1;
        }
    }
    // T3: S4 (PLAIN3) vanishes on chained roots; SYM3 vanishes on them in the u-frame.
    let f = Field::new(11);
    let mut r = Rng(99);
    let mut checked = 0;
    let s3 = |a: u64, b: u64, c: u64| -> u64 {
        let p = f.mul(a, b);
        f.mul(f.sqr(a ^ b), f.sqr(c)) ^ f.mul(p, c) ^ f.sqr(p) ^ 1
    };
    let mut tries = 0;
    while checked < 30 && tries < 100000 {
        tries += 1;
        let x1 = r.next() & f.mask();
        let x2 = r.next() & f.mask();
        let x3 = r.next() & f.mask();
        let roots: Vec<u64> = (0..(1u64 << 11)).filter(|&x| s3(x1, x2, x) == 0).collect();
        if roots.is_empty() {
            continue;
        }
        let xx = roots[0];
        let x4s: Vec<u64> = (0..(1u64 << 11)).filter(|&x| s3(xx, x3, x) == 0).collect();
        for &x4 in &x4s {
            if [x1, x2, x3, x4].iter().any(|&v| v == 1) {
                continue;
            }
            checked += 1;
            if plain3_eval(&f, &[x1, x2, x3, x4]) != 0 {
                println!("T3 PLAIN3 does not vanish");
                fails += 1;
            }
            let u: Vec<u64> = [x1, x2, x3, x4].iter().map(|&x| f.inv(x ^ 1)).collect();
            let w: Vec<u64> = u.iter().map(|&v| f.sqr(v) ^ v).collect();
            let s = u[0] ^ u[1] ^ u[2] ^ u[3];
            let t: Vec<&[u32]> = SYM3.iter().map(|q| &q[..]).collect();
            let v0 = eval_terms(&f, &[w[0], w[1], w[2], w[3], s], &t);
            if v0 != 0 {
                println!("T3 SYM3 does not vanish on a relation tuple");
                fails += 1;
            }
        }
    }
    // T3b: SYM2 vanishes on S3 roots
    let mut c2 = 0;
    for _ in 0..200 {
        let x1 = r.next() & f.mask();
        let x2 = r.next() & f.mask();
        for x3 in (0..(1u64 << 11)).filter(|&x| s3(x1, x2, x) == 0) {
            if [x1, x2, x3].iter().any(|&v| v == 1) {
                continue;
            }
            c2 += 1;
            let u: Vec<u64> = [x1, x2, x3].iter().map(|&x| f.inv(x ^ 1)).collect();
            let w: Vec<u64> = u.iter().map(|&v| f.sqr(v) ^ v).collect();
            let s = u[0] ^ u[1] ^ u[2];
            let t: Vec<&[u32]> = SYM2.iter().map(|q| &q[..]).collect();
            if eval_terms(&f, &[w[0], w[1], w[2], s], &t) != 0 {
                println!("T3b SYM2 does not vanish");
                fails += 1;
            }
        }
    }
    // T4: semi-regular sanity: 10 quadratics in 10 vars
    let sr = semireg_dreg(10, &[2; 10]);
    println!("selftest: S4 checks {checked}, S3 checks {c2}, semireg(10,10x2)={sr}");
    if fails == 0 {
        println!("[selftest] OK");
    } else {
        println!("[selftest] {fails} FAIL");
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selftest") => selftest(),
        Some("replay") => replay(&args[2]),
        Some("posctl") => posctl(&args),
        Some("run") => run(&args),
        _ => eprintln!("usage: selftest | replay FILE | posctl ... | run ..."),
    }
}
