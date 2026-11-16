//! Blind-spot item 03: polynomial-method solving (Dinur 2021; Lokshtanov et
//! al. 2017) on descended Semaev systems, against FES and F4.
//!
//! Standard library only (the `crypto` library does not compile at this branch
//! base, see `research/blindspots_20261018/03-polynomial-method/PROTOCOL.md`).
//! The field arithmetic, the symbolic `S_3` chain builder and the exact
//! solution counters are copied from the item 05 instrument
//! (`examples/blindspots_05_elimination_orders.rs`) so that one `draws.tsv`
//! serves both instruments; the selftest checks the builder against brute force.
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
//!     -o target/blindspots-03/b03 examples/blindspots_03_polynomial_method.rs
//! b03 gen --m 2 --n 13 --ell 6 --seed S --want 8 --sat-want 4
//! b03 selftest
//! b03 fes --draws draws.tsv --cell c13-6-2 --targets 0-7 --random 8
//! b03 poly --draws draws.tsv --cell c13-6-2 --targets 0-7 --rand 8 [--real ...]
//! b03 asym | sens | crossover | summarize --dir raw
//! ```
//!
//! # Unit
//!
//! One bit operation is one two-input Boolean gate (XOR or AND) on single bits,
//! one single-bit counter update, or one single-bit test.  A XOR of two k-bit
//! vectors costs k.  FES: two XORs of the equation vector per Gray step plus the
//! zero test up to the first set bit.  Dinur / Lokshtanov arms: one call of the
//! parity-count core; the real run increments the counter at each charged
//! site, the model evaluates the same charges in closed form, and the selftest
//! requires them to agree to 1e-9 (relative).

#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

// ---------------------------------------------------------------- utilities

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn rss_peak_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

fn vm_bytes() -> u64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|t| t.split_whitespace().next().and_then(|v| v.parse::<u64>().ok()))
        .map(|pages| pages * 4096)
        .unwrap_or(0)
}

/// The soft address-space limit of this process (`prlimit --as`), if any.
fn as_limit_bytes() -> Option<u64> {
    let t = std::fs::read_to_string("/proc/self/limits").ok()?;
    let l = t.lines().find(|l| l.starts_with("Max address space"))?;
    l.split_whitespace().nth(3)?.parse().ok()
}

struct Args {
    v: Vec<String>,
}
impl Args {
    fn flag(&self, name: &str) -> Option<String> {
        self.v
            .iter()
            .position(|a| a == name)
            .and_then(|i| self.v.get(i + 1))
            .cloned()
    }
    fn has(&self, name: &str) -> bool {
        self.v.iter().any(|a| a == name)
    }
    fn num<T: std::str::FromStr>(&self, name: &str, default: T) -> T {
        self.flag(name)
            .map(|s| s.parse().unwrap_or_else(|_| panic!("bad value for {name}")))
            .unwrap_or(default)
    }
    fn list(&self, name: &str) -> Vec<u64> {
        self.flag(name)
            .map(|s| {
                s.split(',')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().parse().expect("number list"))
                    .collect()
            })
            .unwrap_or_default()
    }
}


// ---------------------------------------------------------------- GF(2^n)


struct Gf {
    n: u32,
    low: u64,
    mask: u64,
    pow: Vec<u64>, // z^e mod f, e in 0..2n
}

fn pmod(mut a: u64, b: u64) -> u64 {
    let db = 63 - b.leading_zeros();
    while a != 0 && 63 - a.leading_zeros() >= db {
        a ^= b << (63 - a.leading_zeros() - db);
    }
    a
}

fn is_irreducible(f: u64) -> bool {
    let n = 63 - f.leading_zeros();
    if n == 1 {
        return true;
    }
    for g in 2..(1u64 << (n / 2 + 1)) {
        if pmod(f, g) == 0 {
            return false;
        }
    }
    true
}

/// Same candidate order as `find_irreducible_sparse`: the least `low` among
/// constant + up to three further terms for which `z^n + low` is irreducible.
fn find_irreducible_sparse(n: u32) -> u64 {
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
    let hi = 1u64 << n;
    for low in cands {
        if is_irreducible(hi | low) {
            return low;
        }
    }
    panic!("no sparse irreducible of degree {n}");
}

impl Gf {
    fn new(n: u32) -> Gf {
        let low = find_irreducible_sparse(n);
        let mask = (1u64 << n) - 1;
        let mut g = Gf {
            n,
            low,
            mask,
            pow: Vec::new(),
        };
        let mut p = 1u64;
        for _ in 0..(2 * n) {
            g.pow.push(p);
            p = g.mulx(p);
        }
        g
    }
    #[inline]
    fn mulx(&self, a: u64) -> u64 {
        let hi = (a >> (self.n - 1)) & 1;
        ((a << 1) & self.mask) ^ if hi == 1 { self.low } else { 0 }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        let (mut r, mut aa, mut bb) = (0u64, a, b);
        while bb != 0 {
            if bb & 1 == 1 {
                r ^= aa;
            }
            aa = self.mulx(aa);
            bb >>= 1;
        }
        r
    }
    fn sqr(&self, a: u64) -> u64 {
        let mut r = 0u64;
        let mut bits = a;
        while bits != 0 {
            let k = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            r ^= self.pow[2 * k];
        }
        r
    }
    /// `S_3(a, c, d) = (a+c)^2 d^2 + a c d + (a c)^2 + 1`.
    fn s3(&self, a: u64, c: u64, d: u64) -> u64 {
        let ac = self.mul(a, c);
        self.mul(self.sqr(a ^ c), self.sqr(d)) ^ self.mul(ac, d) ^ self.sqr(ac) ^ 1
    }
    /// Every `d` with `S_3(a, c, d) = 0`.  The `d`-dependent part is
    /// additive, so the roots are an affine subspace.
    fn roots(&self, a: u64, c: u64) -> Vec<u64> {
        let lead = self.sqr(a ^ c);
        let mid = self.mul(a, c);
        let tail = self.sqr(mid) ^ 1;
        let n = self.n as usize;
        // image of each basis vector under d -> lead d^2 + mid d
        let mut piv: Vec<(u64, u64)> = Vec::new(); // (vector, combination)
        let mut kernel: Vec<u64> = Vec::new();
        for i in 0..n {
            let e = 1u64 << i;
            let mut v = self.mul(lead, self.sqr(e)) ^ self.mul(mid, e);
            let mut comb = e;
            for &(pv, pc) in &piv {
                if v & (1u64 << (63 - pv.leading_zeros())) != 0 {
                    v ^= pv;
                    comb ^= pc;
                }
            }
            if v == 0 {
                kernel.push(comb);
            } else {
                piv.push((v, comb));
                piv.sort_unstable_by(|x, y| y.0.cmp(&x.0));
            }
        }
        // solve additive(d) = tail
        let mut v = tail;
        let mut comb = 0u64;
        for &(pv, pc) in &piv {
            if v & (1u64 << (63 - pv.leading_zeros())) != 0 {
                v ^= pv;
                comb ^= pc;
            }
        }
        if v != 0 {
            return Vec::new();
        }
        let mut out = vec![comb];
        for &k in &kernel {
            let cur = out.clone();
            out.extend(cur.iter().map(|x| x ^ k));
        }
        out
    }
}

/// Exact number of solutions of the chained system (tuples of subspace
/// elements and intermediate field elements), by dynamic programming over
/// the chain.  Same quantity as `chained_solution_count`.
fn count_solutions(gf: &Gf, m: usize, basis: &[u64], xr: u64) -> u64 {
    let l = basis.len();
    if m == 2 {
        return count_solutions_m2(gf, basis, xr);
    }
    let span: Vec<u64> = (0..1u64 << l)
        .map(|c| (0..l).filter(|t| (c >> t) & 1 == 1).fold(0, |a, t| a ^ basis[t]))
        .collect();
    let mut state: HashMap<u64, u64> = HashMap::new();
    for &x1 in &span {
        for &x2 in &span {
            for u in gf.roots(x1, x2) {
                *state.entry(u).or_insert(0) += 1;
            }
        }
    }
    for _ in 0..(m - 3) {
        let mut next: HashMap<u64, u64> = HashMap::new();
        for (&u, &cnt) in &state {
            for &x in &span {
                for u2 in gf.roots(u, x) {
                    *next.entry(u2).or_insert(0) += cnt;
                }
            }
        }
        state = next;
    }
    let mut total = 0u64;
    for (&u, &cnt) in &state {
        for &x in &span {
            if gf.s3(u, x, xr) == 0 {
                total += cnt;
            }
        }
    }
    total
}

// ---------------------------------------------------------------- polynomials

type Poly = Vec<u64>; // ascending, unique masks

fn norm(mut v: Vec<u64>) -> Poly {
    v.sort_unstable();
    let mut out = Vec::with_capacity(v.len());
    let mut i = 0;
    while i < v.len() {
        let mut j = i;
        while j < v.len() && v[j] == v[i] {
            j += 1;
        }
        if (j - i) % 2 == 1 {
            out.push(v[i]);
        }
        i = j;
    }
    out
}
fn padd(a: &Poly, b: &Poly) -> Poly {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
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
    let mut v = Vec::with_capacity(a.len() * b.len());
    for &x in a {
        for &y in b {
            v.push(x | y);
        }
    }
    norm(v)
}

type Sym = Vec<Poly>;

fn sym_const(v: u64, n: u32) -> Sym {
    (0..n).map(|k| if (v >> k) & 1 == 1 { vec![0] } else { vec![] }).collect()
}
fn sym_add(a: &Sym, b: &Sym) -> Sym {
    a.iter().zip(b).map(|(x, y)| padd(x, y)).collect()
}
fn sym_sq(a: &Sym, gf: &Gf) -> Sym {
    let n = gf.n as usize;
    let mut raw: Vec<Vec<u64>> = vec![Vec::new(); n];
    for (k, c) in a.iter().enumerate() {
        if c.is_empty() {
            continue;
        }
        let mut bits = gf.pow[2 * k];
        while bits != 0 {
            let t = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            raw[t].extend_from_slice(c);
        }
    }
    raw.into_iter().map(norm).collect()
}
fn sym_mul(a: &Sym, b: &Sym, gf: &Gf) -> Sym {
    let n = gf.n as usize;
    let mut raw: Vec<Vec<u64>> = vec![Vec::new(); n];
    for i in 0..n {
        if a[i].is_empty() {
            continue;
        }
        for j in 0..n {
            if b[j].is_empty() {
                continue;
            }
            let prod = pmul(&a[i], &b[j]);
            if prod.is_empty() {
                continue;
            }
            let mut bits = gf.pow[i + j];
            while bits != 0 {
                let k = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                raw[k].extend_from_slice(&prod);
            }
        }
    }
    raw.into_iter().map(norm).collect()
}
fn sym_s3(x1: &Sym, x2: &Sym, x3: &Sym, gf: &Gf) -> Vec<Poly> {
    let sum_sq = sym_sq(&sym_add(x1, x2), gf);
    let prod = sym_mul(x1, x2, gf);
    let mut acc = sym_mul(&sum_sq, &sym_sq(x3, gf), gf);
    acc = sym_add(&acc, &sym_mul(&prod, x3, gf));
    acc = sym_add(&acc, &sym_sq(&prod, gf));
    acc = sym_add(&acc, &sym_const(1, gf.n));
    acc
}
fn sym_subspace(basis: &[u64], offset: usize, n: u32) -> Sym {
    let mut c: Vec<Vec<u64>> = vec![Vec::new(); n as usize];
    for (t, &b) in basis.iter().enumerate() {
        for k in 0..n as usize {
            if (b >> k) & 1 == 1 {
                c[k].push(1u64 << (offset + t));
            }
        }
    }
    c.into_iter().map(norm).collect()
}
fn sym_free(offset: usize, n: u32) -> Sym {
    (0..n as usize).map(|k| vec![1u64 << (offset + k)]).collect()
}

/// The chained system `build_decomposition_system` builds, for `m >= 3`:
/// variables `m*l` subspace coordinates, then `(m-2)*n` intermediate bits.
fn build_system(gf: &Gf, m: usize, basis: &[u64], xr: u64) -> (Vec<Poly>, usize) {
    let l = basis.len();
    let n = gf.n;
    if m == 2 {
        // one link S_3(x1, x2, x_R): 2*l unknowns
        let s0 = sym_subspace(basis, 0, n);
        let s1 = sym_subspace(basis, l, n);
        let mut eqs = sym_s3(&s0, &s1, &sym_const(xr, n), gf);
        eqs.retain(|p| !p.is_empty());
        return (eqs, 2 * l);
    }
    let nv = m * l + (m - 2) * n as usize;
    assert!(nv <= 62, "at most 62 unknowns in this build");
    let s: Vec<Sym> = (0..m).map(|i| sym_subspace(basis, i * l, n)).collect();
    let target = sym_const(xr, n);
    let inter: Vec<Sym> = (0..m - 2)
        .map(|i| sym_free(m * l + i * n as usize, n))
        .collect();
    let mut eqs: Vec<Poly> = Vec::new();
    eqs.extend(sym_s3(&s[0], &s[1], &inter[0], gf));
    for i in 0..m - 3 {
        eqs.extend(sym_s3(&inter[i], &s[i + 2], &inter[i + 1], gf));
    }
    eqs.extend(sym_s3(&inter[m - 3], &s[m - 1], &target, gf));
    eqs.retain(|p| !p.is_empty());
    (eqs, nv)
}

fn poly_degree(p: &Poly) -> u32 {
    p.iter().map(|m| m.count_ones()).max().unwrap_or(0)
}

fn eval_poly(p: &Poly, point: u64) -> u64 {
    let mut acc = 0u64;
    for &m in p {
        if m & !point == 0 {
            acc ^= 1;
        }
    }
    acc
}

fn gf2_rank(vs: &[u64]) -> usize {
    let mut piv: Vec<u64> = Vec::new();
    for &v0 in vs {
        let mut v = v0;
        for &p in &piv {
            v = v.min(v ^ p);
        }
        if v != 0 {
            piv.push(v);
        }
    }
    piv.len()
}

/// `m = 2`: `S_3(a, c, x_R)` is affine-linear in `c`, so for each `a` in the
/// subspace the solutions `c` of the subspace are an affine solve over `F_2`.
fn count_solutions_m2(gf: &Gf, basis: &[u64], xr: u64) -> u64 {
    let l = basis.len();
    let d = xr;
    let d2 = gf.sqr(d);
    let mut total = 0u64;
    for code in 0..(1u64 << l) {
        let a = (0..l).filter(|t| (code >> t) & 1 == 1).fold(0u64, |s, t| s ^ basis[t]);
        let a2 = gf.sqr(a);
        let lead = d2 ^ a2;
        let mid = gf.mul(a, d);
        let img: Vec<u64> = basis
            .iter()
            .map(|&b| gf.mul(lead, gf.sqr(b)) ^ gf.mul(mid, b))
            .collect();
        let target = gf.mul(a2, d2) ^ 1;
        let r = gf2_rank(&img);
        let mut ext = img.clone();
        ext.push(target);
        if gf2_rank(&ext) == r {
            total += 1u64 << (l - r);
        }
    }
    total
}

// ---------------------------------------------------------------- bit tools

fn pop(x: u64) -> usize {
    x.count_ones() as usize
}

fn lowmask(m: usize) -> u64 {
    if m >= 64 {
        u64::MAX
    } else {
        (1u64 << m) - 1
    }
}

// ---------------------------------------------------------------- packed systems

/// `E <= 64` equations over `n` Boolean variables, degree `<= d`.  `coef[i]`
/// is a word whose bit `j` is the coefficient of monomial `monos[i]` in
/// equation `j`.  `monos` lists every mask of weight `<= d`, ascending.
#[derive(Clone)]
struct Sys {
    n: usize,
    d: usize,
    m: usize,
    monos: Rc<Vec<u64>>,
    coef: Vec<u64>,
}

fn all_monos(n: usize, d: usize) -> Vec<u64> {
    assert!(n <= 40);
    let mut out = Vec::new();
    fn rec(n: usize, d: usize, start: usize, cur: u64, out: &mut Vec<u64>) {
        out.push(cur);
        if pop(cur) == d {
            return;
        }
        for v in start..n {
            rec(n, d, v + 1, cur | (1u64 << v), out);
        }
    }
    rec(n, d, 0, 0, &mut out);
    out.sort_unstable();
    out
}

impl Sys {
    fn new(n: usize, d: usize, m: usize) -> Sys {
        assert!(m >= 1 && m <= 64);
        let monos = Rc::new(all_monos(n, d));
        let coef = vec![0u64; monos.len()];
        Sys { n, d, m, monos, coef }
    }
    fn from_polys(polys: &[Poly], n: usize, d: usize) -> Sys {
        let mut s = Sys::new(n, d, polys.len());
        for (j, p) in polys.iter().enumerate() {
            for &mono in p {
                assert!(pop(mono) <= d, "monomial above the declared degree");
                let i = s.monos.binary_search(&mono).expect("monomial");
                s.coef[i] ^= 1u64 << j;
            }
        }
        s
    }
    fn random(n: usize, d: usize, m: usize, rng: &mut Rng) -> Sys {
        let mut s = Sys::new(n, d, m);
        let mk = lowmask(m);
        for c in s.coef.iter_mut() {
            *c = rng.next() & mk;
        }
        s
    }
    /// Append `k` random affine equations (the Valiant-Vazirani hash).
    fn with_hash(&self, k: usize, rng: &mut Rng) -> Sys {
        let mut s = self.clone();
        assert!(self.m + k <= 64);
        for j in self.m..self.m + k {
            for (i, &mono) in self.monos.iter().enumerate() {
                if pop(mono) <= 1 && rng.next() & 1 == 1 {
                    s.coef[i] ^= 1u64 << j;
                }
            }
        }
        s.m = self.m + k;
        s
    }
    /// Vector of equation values at a point (bit `j` = equation `j`).
    fn eval(&self, point: u64) -> u64 {
        let mut acc = 0u64;
        for (i, &mono) in self.monos.iter().enumerate() {
            if mono & !point == 0 {
                acc ^= self.coef[i];
            }
        }
        acc
    }
    fn brute_count(&self) -> u64 {
        assert!(self.n <= 20);
        (0..1u64 << self.n).filter(|&p| self.eval(p) == 0).count() as u64
    }
    /// (c, r, z): `c` = dimension of the space of equation combinations with no
    /// monomial of degree >= 2 (affine combinations); `r` = rank of their
    /// linear parts (variables the polynomial method could eliminate for free);
    /// `z` = number of independent combinations that are the constant 1 (a
    /// refutation by linear algebra alone).
    fn affine_info(&self) -> (usize, usize, usize) {
        let mut order: Vec<usize> = (0..self.monos.len()).filter(|&i| pop(self.monos[i]) >= 2).collect();
        let nhi = order.len();
        order.extend((0..self.monos.len()).filter(|&i| pop(self.monos[i]) == 1));
        let nlin_end = order.len();
        order.extend((0..self.monos.len()).filter(|&i| pop(self.monos[i]) == 0));
        let words = (order.len() + 63) / 64;
        let mut rows: Vec<Vec<u64>> = (0..self.m)
            .map(|j| {
                let mut r = vec![0u64; words];
                for (t, &i) in order.iter().enumerate() {
                    if (self.coef[i] >> j) & 1 == 1 {
                        r[t / 64] |= 1u64 << (t % 64);
                    }
                }
                r
            })
            .collect();
        let mut first = 0usize;
        let elim = |rows: &mut Vec<Vec<u64>>, from: usize, cols: std::ops::Range<usize>| -> usize {
            let mut rank = from;
            for col in cols {
                let (w, b) = (col / 64, col % 64);
                if let Some(p) = (rank..rows.len()).find(|&r| (rows[r][w] >> b) & 1 == 1) {
                    rows.swap(rank, p);
                    let pr = rows[rank].clone();
                    for r in 0..rows.len() {
                        if r != rank && (rows[r][w] >> b) & 1 == 1 {
                            for (x, y) in rows[r].iter_mut().zip(&pr) {
                                *x ^= *y;
                            }
                        }
                    }
                    rank += 1;
                }
            }
            rank
        };
        let r_hi = elim(&mut rows, first, 0..nhi);
        first = r_hi;
        let combos = self.m - r_hi;
        let r_lin = elim(&mut rows, first, nhi..nlin_end);
        let lin_rank = r_lin - r_hi;
        let zero = rows.len() - r_lin; // rows with only a constant left
        let incons = (r_lin..rows.len()).filter(|&r| (rows[r][nlin_end / 64] >> (nlin_end % 64)) & 1 == 1).count().min(zero);
        (combos, lin_rank, incons)
    }
}

// ---------------------------------------------------------------- FES (quadratic)

/// Gray-order fast exhaustive search, the repository's `mq_fes` scheme: two
/// XORs of the equation vector per step.  Returns (solutions, bit operations).
fn fes_scan(sys: &Sys) -> (u64, u128) {
    assert!(sys.d <= 2 && sys.n <= 40);
    let n = sys.n;
    let e = sys.m;
    let mk = lowmask(e);
    let mut c0 = 0u64;
    let mut lin = vec![0u64; n];
    let mut q = vec![vec![0u64; n]; n];
    for (i, &mono) in sys.monos.iter().enumerate() {
        let c = sys.coef[i];
        match pop(mono) {
            0 => c0 = c,
            1 => lin[mono.trailing_zeros() as usize] = c,
            _ => {
                let a = mono.trailing_zeros() as usize;
                let b = 63 - mono.leading_zeros() as usize;
                q[a][b] = c;
                q[b][a] = c;
            }
        }
    }
    let mut d1: Vec<u64> = (0..n)
        .map(|i| lin[i] ^ if i >= 1 { q[i][i - 1] } else { 0 })
        .collect();
    let mut f = c0;
    let mut count = 0u64;
    let mut ops: u128 = (n * e) as u128;
    // x = 0 itself: the test inspects up to the first set bit
    if f & mk == 0 {
        count += 1;
        ops += e as u128;
    } else {
        ops += (f.trailing_zeros() as u128 + 1).min(e as u128);
    }
    let steps = (1u64 << n) - 1;
    let mut test_ops: u128 = 0;
    for s in 1..=steps {
        let i = s.trailing_zeros() as usize;
        if s != (1u64 << i) {
            let j = (s - (1u64 << i)).trailing_zeros() as usize;
            d1[i] ^= q[i][j];
        }
        f ^= d1[i];
        if f == 0 {
            count += 1;
            test_ops += e as u128;
        } else {
            test_ops += ((f.trailing_zeros() + 1) as u128).min(e as u128);
        }
    }
    let xors = (steps as u128) + (steps as u128 - n as u128);
    ops += xors * e as u128 + test_ops;
    (count, ops)
}

// ---------------------------------------------------------------- down-closed sets

struct DownSet {
    bits: usize,
    list: Vec<u64>, // ascending
    idx: Vec<u32>,  // mask -> position, u32::MAX if absent
}

const ABSENT: u32 = u32::MAX;

impl DownSet {
    fn build(bits: usize, pred: impl Fn(u64) -> bool) -> DownSet {
        assert!(bits <= 25, "real runs are limited to 25 variables per set");
        let mut idx = vec![ABSENT; 1usize << bits];
        let mut list = Vec::new();
        for x in 0..(1u64 << bits) {
            if pred(x) {
                idx[x as usize] = list.len() as u32;
                list.push(x);
            }
        }
        DownSet { bits, list, idx }
    }
}

/// In-place zeta (subset-sum) transform over a down-closed set.  Returns the
/// number of word XORs executed (= sum of the weights of the elements).
fn zeta<T: Copy + std::ops::BitXorAssign>(vals: &mut [T], ds: &DownSet) -> u128 {
    let mut cnt = 0u128;
    for b in 0..ds.bits {
        let bit = 1u64 << b;
        for (i, &x) in ds.list.iter().enumerate() {
            if x & bit != 0 {
                let j = ds.idx[(x ^ bit) as usize] as usize;
                debug_assert!(j != ABSENT as usize);
                let v = vals[j];
                vals[i] ^= v;
                cnt += 1;
            }
        }
    }
    cnt
}

// ---------------------------------------------------------------- the real parity-count core

struct Ctx {
    n: usize,
    t: usize,
    cancel: bool,
    rng: Rng,
    ops: u128,
    sets: HashMap<(u8, usize, usize, usize), Rc<DownSet>>,
    mpos: Vec<u32>,
}

impl Ctx {
    fn new(n: usize, t: usize, cancel: bool, seed: u64, d: usize) -> Ctx {
        let monos = all_monos(n, d);
        let mut mpos = vec![ABSENT; 1usize << n];
        for (i, &m) in monos.iter().enumerate() {
            mpos[m as usize] = i as u32;
        }
        Ctx { n, t, cancel, rng: Rng(seed), ops: 0, sets: HashMap::new(), mpos }
    }
    /// `{x over bits : |x| <= w}`
    fn ball(&mut self, bits: usize, w: usize) -> Rc<DownSet> {
        self.sets
            .entry((0, bits, w, 0))
            .or_insert_with(|| Rc::new(DownSet::build(bits, |x| pop(x) <= w)))
            .clone()
    }
    /// `{x over bits : |x & low(k)| <= w}` (the points `P`, `P2`)
    fn points(&mut self, bits: usize, k: usize, w: usize) -> Rc<DownSet> {
        self.sets
            .entry((1, bits, k, w))
            .or_insert_with(|| {
                let ym = lowmask(k);
                Rc::new(DownSet::build(bits, |x| pop(x & ym) <= w))
            })
            .clone()
    }
    /// `P u {weight <= wd}` over `bits`, with `P` as in `points`.
    fn union_pts(&mut self, bits: usize, k: usize, w: usize, wd: usize) -> Rc<DownSet> {
        let key = (2, bits, k, w * 1000 + wd);
        self.sets
            .entry(key)
            .or_insert_with(|| {
                let ym = lowmask(k);
                Rc::new(DownSet::build(bits, |x| pop(x & ym) <= w || pop(x) <= wd))
            })
            .clone()
    }
}

/// Algorithm 3 with the Moebius evaluation: parities `V[y]` for `|y| <= w`.
fn leaf(ctx: &mut Ctx, sys: &Sys, n1: usize, w: usize) -> Vec<u8> {
    let n = ctx.n;
    let k = n - n1;
    let ym = lowmask(k);
    let u = ctx.union_pts(n, k, w, sys.d);
    let wy = ctx.ball(k, w);
    let mut vals: Vec<u64> = u
        .list
        .iter()
        .map(|&x| {
            let p = ctx.mpos[x as usize];
            if p == ABSENT {
                0
            } else {
                sys.coef[p as usize]
            }
        })
        .collect();
    let cnt = zeta(&mut vals, &u);
    ctx.ops += cnt * sys.m as u128;
    let mut v = vec![0u8; wy.list.len()];
    for (i, &x) in u.list.iter().enumerate() {
        if pop(x & ym) <= w {
            let f = (vals[i] == 0) as u8;
            ctx.ops += sys.m as u128; // (m - 1) ANDs and one parity accumulation
            v[wy.idx[(x & ym) as usize] as usize] ^= f;
        }
    }
    v
}

/// Algorithm 2: `MultParityCount`.  `plan[0]` is this node's `n2`.
fn mpc(ctx: &mut Ctx, sys: &Sys, n1: usize, w: usize, plan: &[usize]) -> Vec<u8> {
    if plan.is_empty() {
        return leaf(ctx, sys, n1, w);
    }
    let n = ctx.n;
    let d = sys.d;
    let n2 = plan[0];
    assert!(n2 <= n1 && n2 >= 1);
    let l = n2 + 2;
    let wprime = d * l - if ctx.cancel { n2 } else { 0 };
    let k = n - n1;
    let uu = n1 - n2;
    let wy = ctx.ball(k, w);
    let wp = ctx.ball(n - n2, wprime);
    let u2 = ctx.union_pts(n - n2, k, w, wprime);
    let ymk = lowmask(k);
    let mut sb = vec![0u16; wy.list.len() << uu];
    let mut pts: Vec<(u32, usize)> = Vec::new(); // (position in u2, scoreboard slot)
    for (i, &x) in u2.list.iter().enumerate() {
        if pop(x & ymk) <= w {
            let iy = wy.idx[(x & ymk) as usize] as usize;
            pts.push((i as u32, (iy << uu) | (x >> k) as usize));
        }
    }
    let mm = sys.monos.len() as u128;
    let mk = lowmask(sys.m);
    for _ in 0..ctx.t {
        let alpha: Vec<u64> = (0..l).map(|_| ctx.rng.next() & mk).collect();
        let mut child = Sys { n, d, m: l, monos: sys.monos.clone(), coef: vec![0u64; sys.coef.len()] };
        for (i, &c) in sys.coef.iter().enumerate() {
            let mut word = 0u64;
            for (r, &a) in alpha.iter().enumerate() {
                word |= (((a & c).count_ones() & 1) as u64) << r;
            }
            child.coef[i] = word;
        }
        ctx.ops += mm * l as u128 * sys.m as u128;
        let v1 = mpc(ctx, &child, n2, wprime, &plan[1..]);
        let mut g = v1;
        let c1 = zeta(&mut g, &wp);
        ctx.ops += c1;
        let mut vals = vec![0u8; u2.list.len()];
        for (j, &x) in wp.list.iter().enumerate() {
            vals[u2.idx[x as usize] as usize] = g[j];
        }
        let c2 = zeta(&mut vals, &u2);
        ctx.ops += c2;
        for &(pi, slot) in &pts {
            sb[slot] += vals[pi as usize] as u16;
        }
        ctx.ops += pts.len() as u128;
    }
    let mut v = vec![0u8; wy.list.len()];
    for (slot, &s) in sb.iter().enumerate() {
        if 2 * s as usize > ctx.t {
            v[slot >> uu] ^= 1;
        }
    }
    ctx.ops += pts.len() as u128;
    v
}

/// Algorithm 1: parity of the number of solutions.
fn parity_count(ctx: &mut Ctx, sys: &Sys, n1: usize, plan: &[usize]) -> u8 {
    let v = mpc(ctx, sys, n1, ctx.n - n1, plan);
    ctx.ops += 1u128 << (ctx.n - n1);
    v.iter().fold(0u8, |a, &b| a ^ b)
}

// ---------------------------------------------------------------- the counted model (log2 domain)

struct Lg {
    lf: Vec<f64>, // log2 n!
}
const NEG: f64 = f64::NEG_INFINITY;

fn lg_add(a: f64, b: f64) -> f64 {
    if a == NEG {
        return b;
    }
    if b == NEG {
        return a;
    }
    let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
    hi + (1.0 + (lo - hi).exp2()).log2()
}

impl Lg {
    fn new(max: usize) -> Lg {
        let mut lf = vec![0.0f64; max + 1];
        for i in 1..=max {
            lf[i] = lf[i - 1] + (i as f64).log2();
        }
        Lg { lf }
    }
    fn lbin(&self, n: usize, k: usize) -> f64 {
        if k > n {
            return NEG;
        }
        self.lf[n] - self.lf[k] - self.lf[n - k]
    }
    /// log2 of prefix sums `sum_{b'<=b} C(z, b')`, for b = 0..=z.
    fn prefix(&self, z: usize) -> Vec<f64> {
        let mut out = Vec::with_capacity(z + 1);
        let mut acc = NEG;
        for b in 0..=z {
            acc = lg_add(acc, self.lbin(z, b));
            out.push(acc);
        }
        out
    }
    /// (log2 |S|, log2 sum of |x| over S) for `S = {x over k + z bits :
    /// a <= amax or a + b <= bmax}` where `a` is the weight on the first `k`
    /// bits and `b` on the last `z`; `bmax = None` drops the second clause.
    fn set_sums(&self, k: usize, z: usize, amax: usize, bmax: Option<usize>) -> (f64, f64) {
        let p0 = self.prefix(z);
        let p1 = if z >= 1 { self.prefix(z - 1) } else { Vec::new() };
        let (mut size, mut wsum) = (NEG, NEG);
        for a in 0..=k {
            let top = if a <= amax {
                z
            } else if let Some(bm) = bmax {
                if a <= bm {
                    (bm - a).min(z)
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let ca = self.lbin(k, a);
            size = lg_add(size, ca + p0[top]);
            // sum_b C(z,b) (a + b) = a 2^p0 + z 2^p1
            let t0 = if a > 0 { (a as f64).log2() + p0[top] } else { NEG };
            let t1 = if top >= 1 && z >= 1 { (z as f64).log2() + p1[top - 1] } else { NEG };
            wsum = lg_add(wsum, ca + lg_add(t0, t1));
        }
        (size, wsum)
    }
}

fn mono_count_log(lg: &Lg, n: usize, d: usize) -> f64 {
    (0..=d).fold(NEG, |acc, s| lg_add(acc, lg.lbin(n, s)))
}

fn lg_mul_u(x: usize) -> f64 {
    if x == 0 {
        NEG
    } else {
        (x as f64).log2()
    }
}

#[derive(Clone, Copy)]
struct Par {
    n: usize,
    d: usize,
    t: f64,
    cancel: bool,
}

fn model_leaf(lg: &Lg, p: &Par, m: usize, n1: usize, w: usize) -> f64 {
    let k = p.n - n1;
    let d = p.d;
    let (szp, _) = lg.set_sums(k, n1, w, None);
    let (_, wsu) = lg.set_sums(k, n1, w, Some(d));
    lg_add(lg_mul_u(m) + wsu, lg_mul_u(m) + szp)
}

fn model_node(lg: &Lg, p: &Par, m: usize, n1: usize, w: usize, plan: &[usize]) -> f64 {
    if plan.is_empty() {
        return model_leaf(lg, p, m, n1, w);
    }
    let n2 = plan[0];
    let l = n2 + 2;
    let wprime = p.d * l - if p.cancel { n2 } else { 0 };
    let k = p.n - n1;
    let uu = n1 - n2;
    let rform = mono_count_log(lg, p.n, p.d) + lg_mul_u(l) + lg_mul_u(m);
    let child = model_node(lg, p, l, n2, wprime, &plan[1..]);
    let (_, wsw) = lg.set_sums(p.n - n2, 0, wprime, None);
    let (szp2, _) = lg.set_sums(k, uu, w, None);
    let (_, wsu2) = lg.set_sums(k, uu, w, Some(wprime));
    let per_rep = lg_add(lg_add(rform, child), lg_add(lg_add(wsw, wsu2), szp2));
    lg_add(p.t.log2() + per_rep, szp2)
}

/// Total counted cost (log2) of one `ParityCount` call.
fn model_top(lg: &Lg, p: &Par, m: usize, n1: usize, plan: &[usize]) -> f64 {
    lg_add(model_node(lg, p, m, n1, p.n - n1, plan), (p.n - n1) as f64)
}

/// Peak working set (log2 of bits) of one call: the packed values of the
/// largest transform, the scoreboard counters and the vote vector.
fn model_mem(lg: &Lg, p: &Par, m: usize, n1: usize, w: usize, plan: &[usize]) -> f64 {
    let k = p.n - n1;
    if plan.is_empty() {
        let (szu, _) = lg.set_sums(k, n1, w, Some(p.d));
        return szu + lg_mul_u(m);
    }
    let n2 = plan[0];
    let l = n2 + 2;
    let wprime = p.d * l - if p.cancel { n2 } else { 0 };
    let uu = n1 - n2;
    let child = model_mem(lg, p, l, n2, wprime, &plan[1..]);
    let (szu2, _) = lg.set_sums(k, uu, w, Some(wprime));
    let (szw, _) = lg.set_sums(p.n - n2, 0, wprime, None);
    let (szp2, _) = lg.set_sums(k, uu, w, None);
    let sb = szp2 + 4.0; // 16-bit scoreboard counters
    lg_add(sb, child.max(lg_add(szu2, szw)))
}

fn model_mem_top(lg: &Lg, p: &Par, m: usize, n1: usize, plan: &[usize]) -> f64 {
    lg_add(model_mem(lg, p, m, n1, p.n - n1, plan), (p.n - n1) as f64)
}

#[derive(Clone)]
struct Best {
    cost: f64,
    mem: f64,
    sched: Vec<usize>,
}

/// Best schedule per memory cap (log2 bits; `INFINITY` = none) among schedules
/// with exactly one of `levels` interior levels.
fn best_schedules(lg: &Lg, p: &Par, m: usize, levels: &[usize], step: usize, caps: &[f64]) -> Vec<Best> {
    let n = p.n;
    let mut best: Vec<Best> = caps.iter().map(|_| Best { cost: f64::INFINITY, mem: f64::INFINITY, sched: Vec::new() }).collect();
    for &lv in levels {
        // schedule s0 >= s1 >= ... >= s_lv >= 1
        let mut s = vec![0usize; lv + 1];
        #[allow(clippy::too_many_arguments)]
        fn rec(lg: &Lg, p: &Par, m: usize, lv: usize, pos: usize, hi: usize, step: usize, s: &mut Vec<usize>, caps: &[f64], best: &mut Vec<Best>) {
            if pos == lv + 1 {
                let c = model_top(lg, p, m, s[0], &s[1..]);
                if best.iter().all(|b| c >= b.cost) {
                    return; // cannot improve any cap
                }
                let mem = model_mem_top(lg, p, m, s[0], &s[1..]);
                for (b, &cap) in best.iter_mut().zip(caps) {
                    if mem <= cap && c < b.cost {
                        *b = Best { cost: c, mem, sched: s.clone() };
                    }
                }
                return;
            }
            let mut v = hi;
            loop {
                s[pos] = v;
                rec(lg, p, m, lv, pos + 1, v, step, s, caps, best);
                if v <= step {
                    if v > 1 {
                        s[pos] = 1;
                        rec(lg, p, m, lv, pos + 1, 1, step, s, caps, best);
                    }
                    break;
                }
                v -= step;
            }
        }
        rec(lg, p, m, lv, 0, n - 1, step, &mut s, caps, &mut best);
    }
    best
}

fn best_schedule(lg: &Lg, p: &Par, m: usize, levels: &[usize], step: usize) -> (f64, Vec<usize>) {
    let b = best_schedules(lg, p, m, levels, step, &[f64::INFINITY]).remove(0);
    (b.cost, b.sched)
}

/// Plain Moebius exhaustive search (the degenerate schedule), best `n1`.
fn exhaustive_cost(lg: &Lg, p: &Par, m: usize) -> f64 {
    (1..p.n).map(|n1| model_top(lg, p, m, n1, &[])).fold(f64::INFINITY, f64::min)
}

fn fes_model_log2(n: usize, e: usize) -> f64 {
    n as f64 + ((2 * e + 2) as f64).log2()
}

fn paper_t(n: usize) -> f64 {
    (48 * n + 1) as f64
}

// ---------------------------------------------------------------- records

enum J {
    S(String),
    F(f64),
    I(i128),
    B(bool),
}
fn js(s: &str) -> J {
    J::S(s.to_string())
}
fn jline(fields: Vec<(&str, J)>) -> String {
    let mut out = String::from("{\"item\":\"03\"");
    for (k, v) in fields {
        out.push_str(&format!(",\"{k}\":"));
        match v {
            J::S(s) => out.push_str(&format!("\"{s}\"")),
            J::F(x) => {
                if x.is_finite() {
                    out.push_str(&format!("{x:.6}"))
                } else {
                    out.push_str("null")
                }
            }
            J::I(i) => out.push_str(&i.to_string()),
            J::B(b) => out.push_str(if b { "true" } else { "false" }),
        }
    }
    out.push('}');
    out
}

fn fail(msg: &str) -> ! {
    println!("CONTROL FAILED: {msg}");
    std::process::exit(2);
}

// ---------------------------------------------------------------- draws

struct Draw {
    label: String,
    idx: usize,
    m: usize,
    n: u32,
    l: usize,
    basis: Vec<u64>,
    xr: u64,
    solutions: u64,
}

fn read_draws(path: &str) -> Vec<Draw> {
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("draws {path}: {e}"));
    let mut out = Vec::new();
    for line in txt.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        assert!(f.len() >= 8, "bad draws line: {line}");
        out.push(Draw {
            label: f[0].to_string(),
            idx: f[1].parse().unwrap(),
            m: f[2].parse().unwrap(),
            n: f[3].parse().unwrap(),
            l: f[4].parse().unwrap(),
            basis: f[5].split(',').map(|x| x.parse().unwrap()).collect(),
            xr: f[6].parse().unwrap(),
            solutions: f[7].parse().unwrap(),
        });
    }
    out
}

fn random_basis(n: u32, l: usize, rng: &mut Rng) -> Vec<u64> {
    loop {
        let b: Vec<u64> = (0..l).map(|_| rng.next() & ((1u64 << n) - 1)).collect();
        if gf2_rank(&b) == l {
            return b;
        }
    }
}

fn parse_targets(s: &str, avail: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for part in s.split(',') {
        if let Some((a, b)) = part.split_once('-') {
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            out.extend(a..=b);
        } else if part == "all" {
            out.extend(0..avail);
        } else if !part.is_empty() {
            out.push(part.parse().unwrap());
        }
    }
    out
}

fn sys_of_draw(d: &Draw) -> Sys {
    let gf = Gf::new(d.n);
    let (eqs, nv) = build_system(&gf, d.m, &d.basis, d.xr);
    let deg = if d.m == 2 { 2 } else { 3 };
    assert!(eqs.iter().all(|p| poly_degree(p) as usize <= deg));
    Sys::from_polys(&eqs, nv, deg)
}

/// `gen --m 2 --n 13 --ell 6 --seed S --want 8 --sat-want 4 [--label L]`
/// Unsatisfiable targets first (idx 0..want), then satisfiable ones.
fn mode_gen(a: &Args) {
    let m: usize = a.num("--m", 2);
    let n: u32 = a.num("--n", 13);
    let l: usize = a.num("--ell", 6);
    let seed: u64 = a.num("--seed", 1);
    let want: usize = a.num("--want", 8);
    let sat_want: usize = a.num("--sat-want", 4);
    let label = a.flag("--label").unwrap_or_else(|| format!("c{n}-{l}-{m}"));
    let gf = Gf::new(n);
    let mut rng = Rng(seed ^ ((m as u64) << 56) ^ ((n as u64) << 48) ^ ((l as u64) << 40));
    let basis = random_basis(n, l, &mut rng);
    let bs = basis.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    let (mut unsat, mut sat) = (Vec::new(), Vec::new());
    let mut tried = 0u64;
    while unsat.len() < want || sat.len() < sat_want {
        tried += 1;
        if tried > 4_000_000 {
            eprintln!("gen: gave up after {tried} draws (unsat {}, sat {})", unsat.len(), sat.len());
            std::process::exit(2);
        }
        let xr = rng.next() & gf.mask;
        if xr == 0 {
            continue;
        }
        let sols = count_solutions(&gf, m, &basis, xr);
        if sols == 0 && unsat.len() < want && !unsat.iter().any(|&(x, _)| x == xr) {
            unsat.push((xr, sols));
        } else if sols > 0 && sat.len() < sat_want && !sat.iter().any(|&(x, _)| x == xr) {
            sat.push((xr, sols));
        }
    }
    println!("#label\tidx\tm\tn\tell\tbasis\txr\tsolutions");
    for (idx, (xr, sols)) in unsat.iter().chain(sat.iter()).enumerate() {
        println!("{label}\t{idx}\t{m}\t{n}\t{l}\t{bs}\t{xr}\t{sols}");
    }
    eprintln!("gen {label}: {tried} draws tried, {} unsat, {} sat kept", unsat.len(), sat.len());
}

// ---------------------------------------------------------------- selftest

fn rand_down_set(bits: usize, rng: &mut Rng) -> DownSet {
    let w = 1 + (rng.next() % 3) as usize;
    let k = 1 + (rng.next() as usize % bits);
    let ym = lowmask(k);
    DownSet::build(bits, |x| pop(x & ym) <= w || pop(x) <= 2)
}

fn selftest_zeta() {
    let mut rng = Rng(11);
    for _ in 0..20 {
        let bits = 4 + (rng.next() % 5) as usize;
        let ds = rand_down_set(bits, &mut rng);
        let vals0: Vec<u64> = ds.list.iter().map(|_| rng.next()).collect();
        let mut v = vals0.clone();
        let cnt = zeta(&mut v, &ds);
        let mut want = 0u128;
        for (i, &x) in ds.list.iter().enumerate() {
            want += pop(x) as u128;
            let mut acc = 0u64;
            for (j, &y) in ds.list.iter().enumerate() {
                if y & !x == 0 {
                    acc ^= vals0[j];
                }
            }
            if acc != v[i] {
                fail("zeta differs from the subset-sum definition");
            }
        }
        if want != cnt {
            fail("zeta operation count differs from sum of weights");
        }
    }
    println!("zeta ok: 20 random down-closed sets equal the subset-sum definition; the count is the sum of weights");
}

fn selftest_fes() {
    let mut rng = Rng(23);
    let mut with_sol = 0;
    for t in 0..40 {
        let n = 6 + (t % 9);
        let e = 3 + (rng.next() % 18) as usize;
        let mut s = Sys::random(n, 2, e, &mut rng);
        if t % 3 == 0 {
            // sparsify so that systems with solutions are common
            for c in s.coef.iter_mut() {
                *c &= rng.next() & rng.next();
            }
        }
        let want = s.brute_count();
        let (got, ops) = fes_scan(&s);
        if want != got {
            fail(&format!("FES found {got} solutions, naive evaluation {want} (n={n}, e={e})"));
        }
        if ops == 0 {
            fail("no operations counted");
        }
        if want > 0 {
            with_sol += 1;
        }
    }
    println!("fes ok: 40 random quadratic systems, N=6..14, equal to naive evaluation ({with_sol} with solutions)");
}

fn selftest_builder() {
    let toys: [(usize, u32, usize); 5] = [(2, 7, 3), (2, 8, 4), (3, 5, 2), (3, 4, 1), (2, 6, 2)];
    let mut rng = Rng(5);
    for &(m, n, l) in &toys {
        let gf = Gf::new(n);
        let basis = random_basis(n, l, &mut rng);
        let mut checked = 0;
        for _ in 0..12 {
            let xr = rng.next() & gf.mask;
            if xr == 0 {
                continue;
            }
            let (eqs, nv) = build_system(&gf, m, &basis, xr);
            let d = if m == 2 { 2 } else { 3 };
            let s = Sys::from_polys(&eqs, nv, d);
            let brute = s.brute_count();
            let dp = count_solutions(&gf, m, &basis, xr);
            if brute != dp {
                fail(&format!("builder: brute force {brute} vs exact count {dp} at m={m} n={n} l={l}"));
            }
            checked += 1;
        }
        println!("builder ok: (m,n,l)=({m},{n},{l}) N={} {checked} targets: brute-force count = exact count", if m == 2 { 2 * l } else { m * l + (m - 2) * n as usize });
    }
}

struct CoreTest {
    n: usize,
    d: usize,
    m: usize,
    sched: Vec<usize>,
    cancel: bool,
    t: usize,
    parity: bool,
}

fn selftest_core() {
    let tests = vec![
        CoreTest { n: 8, d: 2, m: 9, sched: vec![3, 3], cancel: true, t: 89, parity: true },
        CoreTest { n: 8, d: 2, m: 9, sched: vec![3, 3], cancel: false, t: 89, parity: true },
        CoreTest { n: 8, d: 2, m: 9, sched: vec![4, 2], cancel: true, t: 89, parity: true },
        CoreTest { n: 8, d: 2, m: 9, sched: vec![4, 3, 2], cancel: true, t: 31, parity: false },
        CoreTest { n: 8, d: 2, m: 9, sched: vec![5, 3, 2, 1], cancel: true, t: 9, parity: false },
        CoreTest { n: 8, d: 3, m: 9, sched: vec![3, 2], cancel: true, t: 89, parity: true },
        CoreTest { n: 8, d: 3, m: 9, sched: vec![4, 3, 2], cancel: true, t: 13, parity: false },
        CoreTest { n: 9, d: 2, m: 10, sched: vec![4, 3], cancel: true, t: 100, parity: true },
        CoreTest { n: 7, d: 2, m: 8, sched: vec![4, 3, 2], cancel: true, t: 78, parity: true },
    ];
    let lg = Lg::new(64);
    let mut rng = Rng(99);
    let (mut checked, mut odd, mut total_ops_checks) = (0, 0, 0);
    for (ti, tc) in tests.iter().enumerate() {
        let tstart = Instant::now();
        for seed in 0..3u64 {
            let base = Sys::random(tc.n, tc.d, 4, &mut rng);
            for k in 0..=6usize {
                // hashed system: 4 random equations plus k random affine ones
                let sys = base.with_hash(k, &mut rng);
                let truth = (sys.brute_count() % 2) as u8;
                let mut ctx = Ctx::new(tc.n, tc.t, tc.cancel, 1000 * ti as u64 + seed, tc.d);
                let got = parity_count(&mut ctx, &sys, tc.sched[0], &tc.sched[1..]);
                let p = Par { n: tc.n, d: tc.d, t: tc.t as f64, cancel: tc.cancel };
                let model = model_top(&lg, &p, sys.m, tc.sched[0], &tc.sched[1..]);
                let real = (ctx.ops as f64).log2();
                if (model - real).abs() > 1e-9 {
                    fail(&format!(
                        "counted model {model:.9} differs from real counter {real:.9} (n={} d={} sched={:?} cancel={} t={})",
                        tc.n, tc.d, tc.sched, tc.cancel, tc.t
                    ));
                }
                total_ops_checks += 1;
                if tc.parity {
                    if got != truth {
                        fail(&format!(
                            "parity {got} vs truth {truth} (n={} d={} sched={:?} cancel={} t={})",
                            tc.n, tc.d, tc.sched, tc.cancel, tc.t
                        ));
                    }
                    checked += 1;
                    odd += truth as usize;
                }
            }
        }
        println!(
            "core ok: n={} d={} sched={:?} cancel={} t={} parity_checked={} ({:.1}s)",
            tc.n, tc.d, tc.sched, tc.cancel, tc.t, tc.parity, tstart.elapsed().as_secs_f64()
        );
    }
    println!(
        "core ok: {total_ops_checks} counted-model == real-counter checks; {checked} parity checks equal brute force ({odd} of them odd)"
    );
    if odd < 20 {
        fail("too few odd-parity systems for the parity check to mean anything");
    }
    // the check can fail: with a single repetition the vote is one noisy sample
    let mut wrong = 0;
    for seed in 0..60u64 {
        let base = Sys::random(8, 2, 4, &mut rng);
        let sys = base.with_hash((seed % 6) as usize, &mut rng);
        let truth = (sys.brute_count() % 2) as u8;
        let mut ctx = Ctx::new(8, 1, true, seed, 2);
        if parity_count(&mut ctx, &sys, 3, &[3]) != truth {
            wrong += 1;
        }
    }
    if wrong == 0 {
        fail("t = 1 never produced a wrong parity: the parity check cannot see errors");
    }
    println!("core ok: with t = 1 the parity check sees {wrong} of 60 wrong answers (it can fail)");
}

fn mode_selftest(_a: &Args) {
    selftest_zeta();
    selftest_fes();
    selftest_builder();
    selftest_core();
    println!("SELFTEST OK");
}

// ---------------------------------------------------------------- asymptotic exponent (positive control 2)

fn h2(x: f64) -> f64 {
    if x <= 0.0 || x >= 1.0 {
        0.0
    } else {
        -x * x.log2() - (1.0 - x) * (1.0 - x).log2()
    }
}

/// exponent of the ball of radius `a n` in `(1-k) n` bits, plus nothing
fn fd(d: usize, k: f64, cancel: bool) -> f64 {
    let a = if cancel { (d as f64 - 1.0) * k } else { d as f64 * k };
    let r = a / (1.0 - k);
    if r >= 0.5 {
        1.0 - k
    } else {
        (1.0 - k) * h2(r)
    }
}

fn feasible(d: usize, e: f64, lmax: usize, cancel: bool) -> bool {
    let k1 = 1.0 - e;
    if k1 <= 0.0 {
        return true;
    }
    if fd(d, k1, cancel) > e {
        return false;
    }
    let mut cur = k1;
    for level in 1..=lmax {
        if fd(d, cur, cancel) + cur <= e {
            return true;
        }
        if !cancel || level == lmax {
            return false;
        }
        let slack = e - fd(d, cur, cancel);
        if slack <= 1e-12 {
            return false;
        }
        let next = cur - slack;
        if next <= 0.0 {
            return true;
        }
        if fd(d, next, cancel) > e {
            return false;
        }
        cur = next;
    }
    false
}

fn exponent(d: usize, lmax: usize, cancel: bool) -> f64 {
    let (mut lo, mut hi) = (0.3f64, 1.0f64);
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if feasible(d, mid, lmax, cancel) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

fn mode_asym(_a: &Args) {
    let lmaxes = [1usize, 2, 3, 5, 10, 20, 60];
    let mut ok = true;
    for &(d, cancel, name) in &[(2usize, true, "dinur"), (3, true, "dinur"), (2, false, "lptwy")] {
        let mut prev = f64::INFINITY;
        for &lm in &lmaxes {
            if !cancel && lm > 1 {
                continue;
            }
            let e = exponent(d, lm, cancel);
            println!("{}", jline(vec![("kind", js("asym")), ("arm", js(name)), ("d", J::I(d as i128)), ("lmax", J::I(lm as i128)), ("exponent", J::F(e))]));
            if e > prev + 1e-9 {
                println!("CONTROL FAILED: exponent increased with the number of levels ({name} d={d})");
                ok = false;
            }
            prev = e;
        }
        let (lo, hi, pubv) = match (name, d) {
            ("dinur", 2) => (0.6943 - 0.0005, 0.6943 + 0.002, 0.6943),
            ("dinur", 3) => (0.8114 - 0.0005, 0.8114 + 0.003, 0.8114),
            _ => (0.8765 - 0.004, 0.8765 + 0.004, 0.8765),
        };
        let last = exponent(d, if cancel { 60 } else { 1 }, cancel);
        let pass = last >= lo && last <= hi;
        println!(
            "{}",
            jline(vec![
                ("kind", js("control")),
                ("name", js(&format!("published-exponent-{name}-d{d}"))),
                ("value", J::F(last)),
                ("published", J::F(pubv)),
                ("lo", J::F(lo)),
                ("hi", J::F(hi)),
                ("pass", J::B(pass)),
            ])
        );
        ok &= pass;
    }
    // the depth-1 value of the cancelling scheme, for the record (not a target)
    println!("{}", jline(vec![("kind", js("asym-note")), ("arm", js("bkw-form-depth1")), ("exponent", J::F(exponent(2, 1, true)))]));
    if !ok {
        std::process::exit(2);
    }
    println!("ASYM OK");
}

// ---------------------------------------------------------------- measured modes

fn grid_step(n: usize) -> usize {
    if n <= 36 {
        1
    } else {
        (n + 39) / 40
    }
}

struct CellModel {
    dinur: Best,
    levels: Vec<Best>, // best with exactly 1, 2, 3 interior levels
    dinur_cap35: Best, // working set <= 2^35 bits = 4 GiB
    dinur_t11: Best,
    lptwy: Best,
    exh: f64,
}

fn best_lptwy(lg: &Lg, p: &Par, m: usize) -> Best {
    let mut best = Best { cost: f64::INFINITY, mem: f64::INFINITY, sched: Vec::new() };
    for s0 in 1..p.n {
        let c = model_top(lg, p, m, s0, &[s0]);
        if c < best.cost {
            best = Best { cost: c, mem: model_mem_top(lg, p, m, s0, &[s0]), sched: vec![s0, s0] };
        }
    }
    best
}

fn cell_model(lg: &Lg, n: usize, e: usize, d: usize) -> CellModel {
    let step = grid_step(n);
    let p = Par { n, d, t: paper_t(n), cancel: true };
    let levels: Vec<Best> = [1usize, 2, 3]
        .iter()
        .map(|&l| best_schedules(lg, &p, e, &[l], step, &[f64::INFINITY]).remove(0))
        .collect();
    let dinur = levels.iter().cloned().fold(levels[0].clone(), |a, b| if b.cost < a.cost { b } else { a });
    let dinur_cap35 = best_schedules(lg, &p, e, &[1, 2, 3], step, &[35.0]).remove(0);
    let p11 = Par { n, d, t: (11 * n + 1) as f64, cancel: true };
    let dinur_t11 = best_schedules(lg, &p11, e, &[1, 2, 3], step, &[f64::INFINITY]).remove(0);
    let pl = Par { n, d, t: paper_t(n), cancel: false };
    let lptwy = best_lptwy(lg, &pl, e);
    CellModel { dinur, levels, dinur_cap35, dinur_t11, lptwy, exh: exhaustive_cost(lg, &p, e) }
}

fn sched_str(s: &[usize]) -> String {
    s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("-")
}

fn sys_rows(label: &str, kind: &str, target: usize, d: &Draw, sys: &Sys) -> Vec<(&'static str, J)> {
    let _ = d;
    vec![
        ("sys", js(kind)),
        ("cell", js(label)),
        ("target", J::I(target as i128)),
        ("N", J::I(sys.n as i128)),
        ("E", J::I(sys.m as i128)),
        ("d", J::I(sys.d as i128)),
    ]
}

fn mode_fes(a: &Args) {
    let draws = read_draws(&a.flag("--draws").expect("--draws"));
    let label = a.flag("--cell").expect("--cell");
    let cell: Vec<&Draw> = draws.iter().filter(|d| d.label == label).collect();
    assert!(!cell.is_empty(), "no draws for {label}");
    let tgts = parse_targets(&a.flag("--targets").unwrap_or("0-7".into()), cell.len());
    let nrand: usize = a.num("--random", 0);
    let seed: u64 = a.num("--seed", 20261018);
    let mut dims = (0usize, 0usize);
    for &ti in &tgts {
        let d = cell.iter().find(|d| d.idx == ti).expect("target");
        let sys = sys_of_draw(d);
        dims = (sys.n, sys.m);
        let t0 = Instant::now();
        let (sols, ops) = fes_scan(&sys);
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        if sols != d.solutions {
            fail(&format!("FES found {sols} solutions, the draw records {} ({label} t{ti})", d.solutions));
        }
        let mut f = sys_rows(&label, "desc", ti, d, &sys);
        f.insert(0, ("kind", js("fes")));
        f.push(("sols", J::I(sols as i128)));
        f.push(("bitops", J::I(ops as i128)));
        f.push(("log2", J::F((ops as f64).log2())));
        f.push(("ms", J::F(ms)));
        { let (c, r, z) = sys.affine_info(); f.push(("affine_combos", J::I(c as i128))); f.push(("affine_linear_rank", J::I(r as i128))); f.push(("affine_const_one", J::I(z as i128))); }
        println!("{}", jline(f));
    }
    let mut rng = Rng(seed ^ (dims.0 as u64) << 20);
    for r in 0..nrand {
        let sys = Sys::random(dims.0, 2, dims.1, &mut rng);
        let t0 = Instant::now();
        let (sols, ops) = fes_scan(&sys);
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        let dummy = &cell[0];
        let mut f = sys_rows(&label, "rand", r, dummy, &sys);
        f.insert(0, ("kind", js("fes")));
        f.push(("sols", J::I(sols as i128)));
        f.push(("bitops", J::I(ops as i128)));
        f.push(("log2", J::F((ops as f64).log2())));
        f.push(("ms", J::F(ms)));
        { let (c, r, z) = sys.affine_info(); f.push(("affine_combos", J::I(c as i128))); f.push(("affine_linear_rank", J::I(r as i128))); f.push(("affine_const_one", J::I(z as i128))); }
        println!("{}", jline(f));
    }
}

/// `poly --draws F --cell C --targets 0-7 [--rand K] [--real] [--real-targets 8-11]
///  [--sched a,b,c] [--t T] [--max-ops X] [--hash-max H] [--real-rand K] [--seed S] [--no-model]`
fn mode_poly(a: &Args) {
    let draws = read_draws(&a.flag("--draws").expect("--draws"));
    let label = a.flag("--cell").expect("--cell");
    let cell: Vec<&Draw> = draws.iter().filter(|d| d.label == label).collect();
    assert!(!cell.is_empty(), "no draws for {label}");
    let tgts = parse_targets(&a.flag("--targets").unwrap_or("0-7".into()), cell.len());
    let nrand: usize = a.num("--rand", 0);
    let seed: u64 = a.num("--seed", 20261018);
    let lg = Lg::new(512);
    let first = sys_of_draw(cell.iter().find(|d| d.idx == tgts[0]).expect("target"));
    let (n, e, dd) = (first.n, first.m, first.d);
    let cm = cell_model(&lg, n, e, dd);
    let emit = |kind: &str, target: usize, sys: &Sys| {
        let rows = |arm: &str, c: f64, mem: f64, sched: &[usize], tfac: &str| {
            println!(
                "{}",
                jline(vec![
                    ("kind", js("model")),
                    ("sys", js(kind)),
                    ("cell", js(&label)),
                    ("target", J::I(target as i128)),
                    ("N", J::I(sys.n as i128)),
                    ("E", J::I(sys.m as i128)),
                    ("d", J::I(sys.d as i128)),
                    ("arm", js(arm)),
                    ("tfac", js(tfac)),
                    ("sched", js(&sched_str(sched))),
                    ("log2", J::F(c)),
                    ("mem_log2_bits", J::F(mem)),
                    ("affine_combos", J::I(sys.affine_info().0 as i128)),
                ])
            );
        };
        rows("dinur", cm.dinur.cost, cm.dinur.mem, &cm.dinur.sched, "48n+1");
        for (i, b) in cm.levels.iter().enumerate() {
            rows(&format!("dinur-L{}", i + 1), b.cost, b.mem, &b.sched, "48n+1");
        }
        rows("dinur-mem35", cm.dinur_cap35.cost, cm.dinur_cap35.mem, &cm.dinur_cap35.sched, "48n+1");
        rows("dinur-t11", cm.dinur_t11.cost, cm.dinur_t11.mem, &cm.dinur_t11.sched, "11n+1");
        rows("lptwy", cm.lptwy.cost, cm.lptwy.mem, &cm.lptwy.sched, "48n+1");
        rows("exh", cm.exh, f64::NAN, &[], "-");
        if sys.d >= 3 {
            rows("fes-floor", fes_model_log2(sys.n, sys.m), f64::NAN, &[], "-");
        }
    };
    let no_model = a.has("--no-model");
    if !no_model {
        for &ti in &tgts {
            let d = cell.iter().find(|d| d.idx == ti).expect("target");
            emit("desc", ti, &sys_of_draw(d));
        }
        let mut rng = Rng(seed ^ (n as u64) << 24 ^ 0x55);
        for r in 0..nrand {
            emit("rand", r, &Sys::random(n, dd, e, &mut rng));
        }
    }
    if !a.has("--real") {
        return;
    }
    // real runs on hashed systems: the Valiant-Vazirani step, k = 0 .. hash-max
    let rt = parse_targets(&a.flag("--real-targets").unwrap_or("8-11".into()), cell.len());
    let hash_max: usize = a.num("--hash-max", n);
    let max_ops: f64 = a.num("--max-ops", 3e9);
    let forced: Vec<usize> = a.list("--sched").iter().map(|&x| x as usize).collect();
    let t_override: usize = a.num("--t", 0);
    let nrr: usize = a.num("--real-rand", 4);
    let mut targets: Vec<(String, usize, Sys)> = Vec::new();
    for &ti in &rt {
        if let Some(d) = cell.iter().find(|d| d.idx == ti) {
            targets.push(("desc".into(), ti, sys_of_draw(d)));
        }
    }
    let mut rng2 = Rng(seed ^ (n as u64) << 28 ^ 0x77);
    for r in 0..nrr {
        targets.push(("rand".into(), r, Sys::random(n, dd, e, &mut rng2)));
    }
    let (mut checked, mut odd, mut bad, mut skipped) = (0usize, 0usize, 0usize, 0usize);
    let mut hrng = Rng(seed ^ 0x1234);
    for (kind, ti, sys0) in &targets {
        for k in 0..=hash_max.min(sys0.n) {
            if sys0.m + k > 64 {
                break;
            }
            let sys = sys0.with_hash(k, &mut hrng);
            let truth = if sys.d <= 2 { (fes_scan(&sys).0 % 2) as u8 } else { (sys.brute_count() % 2) as u8 };
            let arms: [(&str, bool, Vec<usize>); 2] = [
                ("dinur", true, if forced.is_empty() { best_schedule(&lg, &Par { n, d: dd, t: if t_override > 0 { t_override as f64 } else { paper_t(n) }, cancel: true }, sys.m, &[1, 2, 3], 1).1 } else { forced.clone() }),
                ("lptwy", false, if forced.is_empty() { best_lptwy(&lg, &Par { n, d: dd, t: if t_override > 0 { t_override as f64 } else { paper_t(n) }, cancel: false }, sys.m).sched } else { vec![forced[0], forced[0]] }),
            ];
            for (arm, cancel, sched) in arms.iter() {
                if let Some(list) = a.flag("--arms") {
                    if !list.split(',').any(|x| x == *arm) {
                        continue;
                    }
                }
                let t = if t_override > 0 { t_override } else { 48 * n + 1 };
                let p = Par { n, d: dd, t: t as f64, cancel: *cancel };
                let model = model_top(&lg, &p, sys.m, sched[0], &sched[1..]);
                if model.exp2() > max_ops {
                    skipped += 1;
                    println!(
                        "{}",
                        jline(vec![("kind", js("real")), ("arm", js(arm)), ("cell", js(&label)), ("sys", js(kind)), ("target", J::I(*ti as i128)), ("k", J::I(k as i128)), ("status", js("skipped")), ("model_log2", J::F(model))])
                    );
                    continue;
                }
                let t0 = Instant::now();
                let mut ctx = Ctx::new(n, t, *cancel, seed ^ (k as u64) << 8 ^ *ti as u64, dd);
                let got = parity_count(&mut ctx, &sys, sched[0], &sched[1..]);
                let ms = t0.elapsed().as_secs_f64() * 1000.0;
                let real = (ctx.ops as f64).log2();
                let ok = got == truth;
                let eq = (real - model).abs() < 1e-9;
                checked += 1;
                odd += truth as usize;
                if !ok || !eq {
                    bad += 1;
                }
                println!(
                    "{}",
                    jline(vec![
                        ("kind", js("real")),
                        ("arm", js(arm)),
                        ("cell", js(&label)),
                        ("sys", js(kind)),
                        ("target", J::I(*ti as i128)),
                        ("k", J::I(k as i128)),
                        ("E", J::I(sys.m as i128)),
                        ("status", js("done")),
                        ("sched", js(&sched_str(sched))),
                        ("t", J::I(t as i128)),
                        ("truth", J::I(truth as i128)),
                        ("got", J::I(got as i128)),
                        ("match", J::B(ok)),
                        ("ops_real", J::I(ctx.ops as i128)),
                        ("real_log2", J::F(real)),
                        ("model_log2", J::F(model)),
                        ("model_equal", J::B(eq)),
                        ("ms", J::F(ms)),
                    ])
                );
            }
        }
    }
    println!(
        "{}",
        jline(vec![
            ("kind", js("control")),
            ("name", js("parity-agreement")),
            ("cell", js(&label)),
            ("checked", J::I(checked as i128)),
            ("odd", J::I(odd as i128)),
            ("skipped", J::I(skipped as i128)),
            ("bad", J::I(bad as i128)),
            ("pass", J::B(bad == 0)),
        ])
    );
    if bad > 0 {
        std::process::exit(2);
    }
}

/// Sensitivity control: the counters move with E as they should.
fn mode_sens(_a: &Args) {
    let n = 16usize;
    let lg = Lg::new(64);
    let mut rng = Rng(3);
    let mut base_fes = 0f64;
    let mut ok = true;
    let sched = best_schedule(&lg, &Par { n, d: 2, t: paper_t(n), cancel: true }, 17, &[1, 2, 3], 1).1;
    let base_dinur = model_top(&lg, &Par { n, d: 2, t: paper_t(n), cancel: true }, 17, sched[0], &sched[1..]);
    for &e in &[17usize, 33, 64] {
        let s = Sys::random(n, 2, e, &mut rng);
        let (_, ops) = fes_scan(&s);
        let r_fes = ops as f64;
        if e == 17 {
            base_fes = r_fes;
        }
        let want = (2 * e + 2) as f64 / 36.0;
        let got = r_fes / base_fes;
        let dinur = model_top(&lg, &Par { n, d: 2, t: paper_t(n), cancel: true }, e, sched[0], &sched[1..]);
        let rel_d = (dinur - base_dinur).exp2() - 1.0;
        let pass_f = (got / want - 1.0).abs() <= 0.02;
        let pass_d = rel_d.abs() < 0.01;
        ok &= pass_f && pass_d;
        println!(
            "{}",
            jline(vec![
                ("kind", js("control")),
                ("name", js("sensitivity")),
                ("E", J::I(e as i128)),
                ("fes_ratio", J::F(got)),
                ("fes_expected", J::F(want)),
                ("dinur_rel_change", J::F(rel_d)),
                ("pass", J::B(pass_f && pass_d)),
            ])
        );
    }
    if !ok {
        std::process::exit(2);
    }
    println!("SENS OK");
}

fn mode_crossover(a: &Args) {
    let nmax: usize = a.num("--nmax", 400);
    let lg = Lg::new(nmax + 8);
    let list: Vec<usize> = [32usize, 40, 48, 64, 80, 100, 128, 160, 200, 256, 320, 400]
        .iter()
        .cloned()
        .filter(|&x| x <= nmax)
        .collect();
    // memory caps in log2 bits: 4 GiB (this host's per-job cap), 1 TiB, none
    let caps = [35.0f64, 43.0, f64::INFINITY];
    let mut first_d: [Option<usize>; 3] = [None; 3];
    let mut first_l: Option<usize> = None;
    for &n in &list {
        let e = n + 1;
        let step = grid_step(n);
        let p = Par { n, d: 2, t: paper_t(n), cancel: true };
        let bs = best_schedules(&lg, &p, e, &[1, 2, 3], step, &caps);
        let pl = Par { n, d: 2, t: paper_t(n), cancel: false };
        let lp = best_lptwy(&lg, &pl, e);
        let fes = fes_model_log2(n, e);
        for (i, b) in bs.iter().enumerate() {
            if first_d[i].is_none() && b.cost < fes {
                first_d[i] = Some(n);
            }
        }
        if first_l.is_none() && lp.cost < fes {
            first_l = Some(n);
        }
        println!(
            "{}",
            jline(vec![
                ("kind", js("crossover-grid")),
                ("N", J::I(n as i128)),
                ("fes_log2", J::F(fes)),
                ("dinur_log2", J::F(bs[2].cost)),
                ("dinur_sched", js(&sched_str(&bs[2].sched))),
                ("dinur_mem_log2_bits", J::F(bs[2].mem)),
                ("dinur_cap43_log2", J::F(bs[1].cost)),
                ("dinur_cap35_log2", J::F(bs[0].cost)),
                ("lptwy_log2", J::F(lp.cost)),
                ("lptwy_mem_log2_bits", J::F(lp.mem)),
                ("dinur_c", J::F(bs[2].cost / n as f64)),
            ])
        );
    }
    let g = |x: Option<usize>| J::I(x.map(|v| v as i128).unwrap_or(-1));
    println!(
        "{}",
        jline(vec![
            ("kind", js("crossover")),
            ("dinur_first_grid_N_below_fes", g(first_d[2])),
            ("dinur_first_grid_N_below_fes_mem43", g(first_d[1])),
            ("dinur_first_grid_N_below_fes_mem35", g(first_d[0])),
            ("lptwy_first_grid_N_below_fes", g(first_l)),
            ("nmax", J::I(nmax as i128)),
        ])
    );
}

fn main() {
    let args = Args { v: std::env::args().skip(1).collect() };
    match args.v.first().map(|s| s.as_str()) {
        Some("gen") => mode_gen(&args),
        Some("selftest") => mode_selftest(&args),
        Some("fes") => mode_fes(&args),
        Some("poly") => mode_poly(&args),
        Some("sens") => mode_sens(&args),
        Some("asym") => mode_asym(&args),
        Some("crossover") => mode_crossover(&args),
        Some("summarize") => mode_summarize(&args),
        _ => {
            eprintln!("usage: b03 gen|selftest|fes|poly|sens|asym|crossover|summarize ...");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------- summarize

fn jnum_f(line: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == 'E' || c == '+'))
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}
fn jstr_f(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":\"");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    Some(rest[..rest.find('"')?].to_string())
}
fn jbool_f(line: &str, key: &str) -> Option<bool> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

fn ols(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let num: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let den: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    num / den
}

/// (slope, jackknife min, jackknife max)
fn slope_jack(xs: &[f64], ys: &[f64]) -> (f64, f64, f64) {
    let s = ols(xs, ys);
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for i in 0..xs.len() {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        for j in 0..xs.len() {
            if j != i {
                a.push(xs[j]);
                b.push(ys[j]);
            }
        }
        let t = ols(&a, &b);
        lo = lo.min(t);
        hi = hi.max(t);
    }
    (s, lo, hi)
}

fn gmean_f(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}

#[derive(Default, Clone)]
struct CellAgg {
    label: String,
    n: usize,
    e: usize,
    d: usize,
    fes_desc: Vec<f64>,   // bitops
    fes_rand: Vec<f64>,
    fes_ms: Vec<f64>,
    model: HashMap<String, Vec<f64>>, // arm -> log2 values (desc)
    model_rand: HashMap<String, Vec<f64>>,
    mem: HashMap<String, f64>,
    sched: HashMap<String, String>,
    f4: Vec<(usize, String, f64, f64)>, // target, status, touches, ms
    affine: Vec<(f64, f64, f64)>,
}

fn mode_summarize(a: &Args) {
    let dir = a.flag("--dir").expect("--dir");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("read dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".stdout.log"))
        .collect();
    files.sort();
    let mut cells: HashMap<String, CellAgg> = HashMap::new();
    let mut selftest_ok = false;
    let mut sens_rows = 0usize;
    let mut sens_bad = 0usize;
    let mut asym_ok = false;
    let (mut par_checked, mut par_odd, mut par_bad, mut par_skipped) = (0f64, 0f64, 0f64, 0f64);
    let mut par_cells: Vec<String> = Vec::new();
    let mut control_fail: Vec<String> = Vec::new();
    let mut asym_rows: Vec<(String, usize, usize, f64)> = Vec::new();
    let mut cross: Option<String> = None;
    let mut cross_grid: Vec<String> = Vec::new();
    let mut real_ops_equal = (0usize, 0usize);
    let mut real_sched_depth: HashMap<String, usize> = HashMap::new();
    let mut nlines = 0usize;
    for f in &files {
        let txt = std::fs::read_to_string(f).unwrap_or_default();
        for line in txt.lines() {
            if line.contains("SELFTEST OK") {
                selftest_ok = true;
            }
            if line.contains("ASYM OK") {
                asym_ok = true;
            }
            if line.starts_with("{\"cell\"") {
                // item 05 F4 lines
                let cell = jstr_f(line, "cell").unwrap_or_default();
                let arm = jstr_f(line, "arm").unwrap_or_default();
                if arm != "grevlex" {
                    continue;
                }
                nlines += 1;
                let c = cells.entry(cell.clone()).or_default();
                c.label = cell;
                c.f4.push((
                    jnum_f(line, "target").unwrap_or(0.0) as usize,
                    jstr_f(line, "status").unwrap_or_default(),
                    jnum_f(line, "touches").unwrap_or(f64::NAN),
                    jnum_f(line, "ms").unwrap_or(f64::NAN),
                ));
                continue;
            }
            if !line.starts_with("{\"item\":\"03\"") {
                continue;
            }
            nlines += 1;
            let kind = jstr_f(line, "kind").unwrap_or_default();
            match kind.as_str() {
                "fes" | "model" => {
                    let cell = jstr_f(line, "cell").unwrap_or_default();
                    let c = cells.entry(cell.clone()).or_default();
                    c.label = cell;
                    c.n = jnum_f(line, "N").unwrap_or(0.0) as usize;
                    c.e = jnum_f(line, "E").unwrap_or(0.0) as usize;
                    c.d = jnum_f(line, "d").unwrap_or(0.0) as usize;
                    let sys = jstr_f(line, "sys").unwrap_or_default();
                    if kind == "fes" {
                        let b = jnum_f(line, "bitops").unwrap_or(f64::NAN);
                        if sys == "desc" {
                            c.fes_desc.push(b);
                            c.fes_ms.push(jnum_f(line, "ms").unwrap_or(f64::NAN));
                            c.affine.push((
                                jnum_f(line, "affine_combos").unwrap_or(0.0),
                                jnum_f(line, "affine_linear_rank").unwrap_or(0.0),
                                jnum_f(line, "affine_const_one").unwrap_or(0.0),
                            ));
                        } else {
                            c.fes_rand.push(b);
                        }
                    } else {
                        let arm = jstr_f(line, "arm").unwrap_or_default();
                        let v = jnum_f(line, "log2").unwrap_or(f64::NAN);
                        if sys == "desc" {
                            c.model.entry(arm.clone()).or_default().push(v);
                            if let Some(m) = jnum_f(line, "mem_log2_bits") {
                                c.mem.insert(arm.clone(), m);
                            }
                            c.sched.insert(arm, jstr_f(line, "sched").unwrap_or_default());
                        } else {
                            c.model_rand.entry(arm).or_default().push(v);
                        }
                    }
                }
                "real" => {
                    if jstr_f(line, "status").as_deref() == Some("done") {
                        real_ops_equal.1 += 1;
                        if jbool_f(line, "model_equal") == Some(true) {
                            real_ops_equal.0 += 1;
                        }
                        let sched = jstr_f(line, "sched").unwrap_or_default();
                        let arm = jstr_f(line, "arm").unwrap_or_default();
                        let depth = sched.split('-').count().saturating_sub(1);
                        let key = format!("{arm}-L{depth}");
                        *real_sched_depth.entry(key).or_insert(0) += 1;
                    }
                }
                "control" => {
                    let name = jstr_f(line, "name").unwrap_or_default();
                    let pass = jbool_f(line, "pass").unwrap_or(false);
                    if name == "sensitivity" {
                        // the frozen text: the dinur count moves by less than 1% across E
                        sens_rows += 1;
                        if jnum_f(line, "dinur_rel_change").map(|x| x.abs() >= 0.01).unwrap_or(true) {
                            sens_bad += 1;
                        }
                    }
                    if name == "parity-agreement" {
                        par_checked += jnum_f(line, "checked").unwrap_or(0.0);
                        par_odd += jnum_f(line, "odd").unwrap_or(0.0);
                        par_bad += jnum_f(line, "bad").unwrap_or(0.0);
                        par_skipped += jnum_f(line, "skipped").unwrap_or(0.0);
                        par_cells.push(jstr_f(line, "cell").unwrap_or_default());
                    }
                    if !pass {
                        control_fail.push(name);
                    }
                }
                "asym" => asym_rows.push((
                    jstr_f(line, "arm").unwrap_or_default(),
                    jnum_f(line, "d").unwrap_or(0.0) as usize,
                    jnum_f(line, "lmax").unwrap_or(0.0) as usize,
                    jnum_f(line, "exponent").unwrap_or(f64::NAN),
                )),
                "crossover" => cross = Some(line.to_string()),
                "crossover-grid" => cross_grid.push(line.to_string()),
                _ => {}
            }
        }
    }
    println!("# item 03 summary: {} files, {} records", files.len(), nlines);

    // controls
    println!("\n## Controls");
    println!("selftest line present: {selftest_ok}");
    let sens_ok = sens_rows >= 3 && sens_bad == 0;
    println!("sensitivity control: {sens_ok} ({sens_rows} rows, {sens_bad} outside the frozen 1% bound on the dinur change or failing)");
    println!("published-exponent control (asym): {asym_ok}");
    println!(
        "parity agreement: {par_checked} real calls on hashed systems over cells {:?}, {par_odd} with odd true parity, {par_bad} wrong or model-unequal, {par_skipped} skipped",
        par_cells
    );
    println!("real counter == counted model on {} of {} real calls", real_ops_equal.0, real_ops_equal.1);
    println!("real calls by arm and interior depth: {:?}", { let mut v: Vec<_> = real_sched_depth.iter().collect(); v.sort(); v });
    if !control_fail.is_empty() {
        println!("FAILED CONTROLS: {:?}", control_fail);
    }
    let odd_needed = 400.0; // 200 odd hashed systems, each checked by both polynomial-method arms
    let controls_ok = selftest_ok && sens_ok && asym_ok && control_fail.is_empty() && par_bad == 0.0 && par_odd >= odd_needed;
    println!("controls pass: {controls_ok} (odd-parity real calls {par_odd} of {odd_needed} required)");
    // descriptive only: every control but the sensitivity bound passes (the frozen verdict stays INCONCLUSIVE)
    let controls_ok_but_sens = !controls_ok && selftest_ok && asym_ok && par_bad == 0.0 && par_odd >= odd_needed && control_fail.iter().all(|n| n == "sensitivity");

    let mut s3: Vec<&CellAgg> = cells.values().filter(|c| c.d == 2 && c.n > 0).collect();
    s3.sort_by_key(|c| c.n);
    let mut s4: Vec<&CellAgg> = cells.values().filter(|c| c.d == 3 && c.n > 0).collect();
    s4.sort_by_key(|c| c.n);
    let mean = |v: &Vec<f64>| if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 };
    let m0 = |c: &CellAgg, arm: &str| -> f64 { c.model.get(arm).map(|v| v[0]).unwrap_or(f64::NAN) };

    println!("\n## S_3 family (m = 2, quadratic): counted bit operations, log2");
    println!("cell        N   E  fes(desc)  fes(rand)  d%     dinur   (sched)         lptwy  dinur-t11  dinur-L2  dinur-L3     exh   mem(dinur)  rho_dinur  rho_lptwy");
    let mut xs: Vec<f64> = Vec::new();
    let (mut y_fes, mut y_din, mut y_lp) = (Vec::new(), Vec::new(), Vec::new());
    let mut null_gap = 0f64;
    for c in &s3 {
        let fes = mean(&c.fes_desc).log2();
        let fesr = mean(&c.fes_rand).log2();
        let dpct = (mean(&c.fes_rand) / mean(&c.fes_desc) - 1.0) * 100.0;
        let din = m0(c, "dinur");
        let lp = m0(c, "lptwy");
        for (arm, v) in &c.model {
            if let Some(r) = c.model_rand.get(arm) {
                for x in r {
                    null_gap = null_gap.max((x - v[0]).abs());
                }
            }
        }
        println!(
            "{:<10} {:>3} {:>3}  {:>9.3}  {:>9.3} {:>5.2}  {:>7.3}  {:<14} {:>7.3}  {:>9.3}  {:>8.3}  {:>8.3}  {:>7.3}  {:>9.2}  {:>9.3e}  {:>9.3e}",
            c.label, c.n, c.e, fes, fesr, dpct, din, c.sched.get("dinur").cloned().unwrap_or_default(), lp,
            m0(c, "dinur-t11"), m0(c, "dinur-L2"), m0(c, "dinur-L3"), m0(c, "exh"),
            c.mem.get("dinur").cloned().unwrap_or(f64::NAN), (fes - din).exp2(), (fes - lp).exp2()
        );
        if c.n > 0 && !c.fes_desc.is_empty() {
            xs.push(c.n as f64);
            y_fes.push(fes);
            y_din.push(din);
            y_lp.push(lp);
        }
    }
    println!("negative control: largest |log2 difference| between a descended draw and a random dense system of equal (N,E,d), dinur/lptwy/exh/t11 model rows: {null_gap:.2e}");
    println!("FES cost differs between draws only through the early-exit test (column d% = random mean over descended mean, in percent)");

    println!("\n## Affine relations among the equations of the descended draws (combos, linear rank, constant-one)");
    for c in &s3 {
        if let Some(first) = c.affine.first() {
            let all_same = c.affine.iter().all(|x| x == first);
            println!("{:<10} N={:>2}: {:?} on {} draws (all equal: {all_same})", c.label, c.n, first, c.affine.len());
        }
    }

    let (fit_fes, fit_din, fit_lp);
    println!("\n## Fitted exponent c in 2^(cN) over the S_3 cells (slope of log2 cost against N; jackknife = leave-one-cell-out range, a stability interval)");
    if xs.len() >= 4 {
        fit_fes = slope_jack(&xs, &y_fes);
        fit_din = slope_jack(&xs, &y_din);
        fit_lp = slope_jack(&xs, &y_lp);
        let loc = |ys: &Vec<f64>| (ys[ys.len() - 1] - ys[ys.len() - 2]) / (xs[xs.len() - 1] - xs[xs.len() - 2]);
        println!("fes    c = {:.4}  [{:.4}, {:.4}]  local({}->{}) = {:.4}", fit_fes.0, fit_fes.1, fit_fes.2, xs[xs.len() - 2], xs[xs.len() - 1], loc(&y_fes));
        println!("dinur  c = {:.4}  [{:.4}, {:.4}]  local = {:.4}", fit_din.0, fit_din.1, fit_din.2, loc(&y_din));
        println!("lptwy  c = {:.4}  [{:.4}, {:.4}]  local = {:.4}", fit_lp.0, fit_lp.1, fit_lp.2, loc(&y_lp));
    } else {
        fit_fes = (f64::NAN, f64::NAN, f64::NAN);
        fit_din = fit_fes;
        fit_lp = fit_fes;
        println!("fewer than 4 cells with FES rows: no fit");
    }

    println!("\n## F4 (item 05 Boolean F4, grevlex), touches, per cell");
    println!("cell        N  finished/rows  statuses            geo-mean touches (log2)  range over targets (log2)   F4/FES(desc mean)");
    let mut f4_x = Vec::new();
    let mut f4_y = Vec::new();
    for c in s3.iter().chain(s4.iter()) {
        if c.f4.is_empty() {
            println!("{:<10} {:>3}  no F4 rows", c.label, c.n);
            continue;
        }
        let done: Vec<f64> = c.f4.iter().filter(|r| r.1 == "refuted").map(|r| r.2).collect();
        let mut st: HashMap<String, usize> = HashMap::new();
        for r in &c.f4 {
            *st.entry(r.1.clone()).or_insert(0) += 1;
        }
        let mut stv: Vec<_> = st.into_iter().collect();
        stv.sort();
        let (lo, hi) = (done.iter().cloned().fold(f64::INFINITY, f64::min), done.iter().cloned().fold(0.0, f64::max));
        let base = if c.d == 2 { mean(&c.fes_desc) } else { fes_model_log2(c.n, c.e).exp2() };
        println!(
            "{:<10} {:>3}  {:>3}/{:<3}        {:<18}  {:>10.3}               [{:.3}, {:.3}]          {:.3}{}",
            c.label, c.n, done.len(), c.f4.len(), format!("{:?}", stv),
            gmean_f(&done).log2(), lo.log2(), hi.log2(), gmean_f(&done) / base,
            if c.d == 3 { "  (vs FES floor)" } else { "" }
        );
        if c.d == 2 && done.len() >= 4 {
            f4_x.push(c.n as f64);
            f4_y.push(gmean_f(&done).log2());
        }
    }
    let fit_f4 = if f4_x.len() >= 3 { slope_jack(&f4_x, &f4_y) } else { (f64::NAN, f64::NAN, f64::NAN) };
    println!("F4 fitted c over the S_3 cells with >= 4 finished targets ({} cells): {:.4} [{:.4}, {:.4}]", f4_x.len(), fit_f4.0, fit_f4.1, fit_f4.2);

    println!("\n## S_4 family (m = 3 chain, cubic): log2 counted cost");
    println!("cell        N   E  fes-floor  dinur  (sched)        lptwy  dinur-t11  mem(dinur)  F4(geo)   rho_dinur(vs floor)");
    for c in &s4 {
        let fl = m0(c, "fes-floor");
        let f4d: Vec<f64> = c.f4.iter().filter(|r| r.1 == "refuted").map(|r| r.2).collect();
        println!(
            "{:<10} {:>3} {:>3}  {:>9.3}  {:>6.3}  {:<12} {:>7.3}  {:>9.3}  {:>9.2}  {:>8.3}  {:>9.3e}",
            c.label, c.n, c.e, fl, m0(c, "dinur"), c.sched.get("dinur").cloned().unwrap_or_default(), m0(c, "lptwy"),
            m0(c, "dinur-t11"), c.mem.get("dinur").cloned().unwrap_or(f64::NAN), gmean_f(&f4d).log2(), (fl - m0(c, "dinur")).exp2()
        );
    }

    println!("\n## Crossover (counted model, E = N + 1, d = 2) and working set");
    for g in &cross_grid {
        println!(
            "N={:>3} fes {:>7.2}  dinur {:>7.2} ({})  mem {:>6.2}  cap2^43 {}  cap2^35 {}  lptwy {:>7.2}",
            jnum_f(g, "N").unwrap_or(0.0), jnum_f(g, "fes_log2").unwrap_or(f64::NAN), jnum_f(g, "dinur_log2").unwrap_or(f64::NAN),
            jstr_f(g, "dinur_sched").unwrap_or_default(), jnum_f(g, "dinur_mem_log2_bits").unwrap_or(f64::NAN),
            jnum_f(g, "dinur_cap43_log2").map(|x| format!("{x:.2}")).unwrap_or("none".into()),
            jnum_f(g, "dinur_cap35_log2").map(|x| format!("{x:.2}")).unwrap_or("none".into()),
            jnum_f(g, "lptwy_log2").unwrap_or(f64::NAN)
        );
    }
    if let Some(c) = &cross {
        println!("{c}");
    }

    println!("\n## Exponent statement (asym routine)");
    let get = |arm: &str, d: usize, lm: usize| asym_rows.iter().find(|r| r.0 == arm && r.1 == d && r.2 == lm).map(|r| r.3).unwrap_or(f64::NAN);
    let c2 = get("dinur", 2, 60);
    let c3 = get("dinur", 3, 60);
    let cl = get("lptwy", 2, 1);
    println!("asymptotic exponent, least over schedules with up to L levels: d=2 L=1 {:.4}, L=3 {:.4}, L=60 {:.4}; d=3 L=1 {:.4}, L=60 {:.4}; lptwy d=2 {:.4}", get("dinur", 2, 1), get("dinur", 2, 3), c2, get("dinur", 3, 1), c3, cl);
    for &(m, c) in &[(2usize, c2), (3usize, c3)] {
        let need = 0.5 - 1.0 / m as f64;
        println!(
            "family m={m}: required c < 1/2 - 1/m = {need:.4}; published-form asymptotic c = {c:.4}; distance remaining {:.4}; index-calculus exponent relative to rho 2/m + 2c = {:.4} (above 1 loses)",
            c - need, 2.0 / m as f64 + 2.0 * c
        );
    }
    for (name, fit, m) in [("dinur", fit_din, 2usize), ("lptwy", fit_lp, 2), ("fes", fit_fes, 2)] {
        let need = 0.5 - 1.0 / m as f64;
        println!("fitted c of {name} over the measured S_3 range: {:.4}; distance to required {:.4}; 2/m + 2c = {:.4}", fit.0, fit.0 - need, 2.0 / m as f64 + 2.0 * fit.0);
    }

    // verdict
    println!("\n## Verdict by the frozen rule");
    let big = s3.last();
    let mut verdict = "INCONCLUSIVE".to_string();
    let mut level = "n/a";
    if !controls_ok && !controls_ok_but_sens {
        println!("a control did not pass or is missing: INCONCLUSIVE");
    } else if let Some(c) = big {
        if c.n != 32 || c.fes_desc.is_empty() {
            println!("the N = 32 cell has no FES rows: INCONCLUSIVE");
        } else {
            let fes = mean(&c.fes_desc).log2();
            let rho_d = (fes - m0(c, "dinur")).exp2();
            let rho_l = (fes - m0(c, "lptwy")).exp2();
            let best = rho_d.max(rho_l);
            let jack_fes = fit_fes.2 - fit_fes.1;
            let margin_d = (fit_fes.0 - fit_din.0) - jack_fes.max(fit_din.2 - fit_din.1);
            let margin_l = (fit_fes.0 - fit_lp.0) - jack_fes.max(fit_lp.2 - fit_lp.1);
            if best >= 1.5 {
                verdict = "ADVANCES".into();
                level = "constant";
            } else if best > 2.0 / 3.0 {
                verdict = "MATCHES".into();
                level = "constant";
            } else if margin_d > 0.0 || margin_l > 0.0 {
                verdict = "TRADES".into();
                level = "exponent over constant";
            } else {
                verdict = "REGRESSES".into();
                level = "constant";
            }
            println!(
                "rho_dinur(32) = {rho_d:.3e}, rho_lptwy(32) = {rho_l:.3e}; fitted-exponent margin below FES (after the larger jackknife range): dinur {margin_d:.4}, lptwy {margin_l:.4}"
            );
            if !controls_ok {
                println!("a control did not pass (the sensitivity bound): INCONCLUSIVE by the frozen rule; the branch the rule would give with that one control waived, descriptive only: {verdict} (level: {level})");
                verdict = "INCONCLUSIVE".into();
                level = "n/a";
            }
        }
    }
    println!("arm (e) msolve: BLOCKED (not installed; no GMP/MPFR/FLINT headers; ~4 GB disk); unblock with a prebuilt msolve binary");
    println!("VERDICT: {verdict} (level: {level})");
}
