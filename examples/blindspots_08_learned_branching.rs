//! Blind-spot item 08: learned branching for a WDSat-style XOR/AND solver on
//! chained Semaev `S_3` systems (m = 3 summands) over `GF(2^n)`.
//!
//! Standard library only (the `crypto` library does not compile at this branch
//! base, see `research/blindspots_20261018/08-learned-sat-branching/PROTOCOL.md`).
//! Build (each card does this):
//!   rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native \
//!         -o target/blindspots-08/<dir>/b08 examples/blindspots_08_learned_branching.rs
//!
//! Subcommands: selftest | pilot | train | cell | cnf | cmsrun | summarize
//! (see PROTOCOL.md).
//!
//! # The solver
//!
//! A native replica of Trimoska's WDSat search (`wdsat.c`, no Gaussian
//! elimination): the ANF system is turned into AND-atoms (one per monomial of
//! degree >= 2, defined by the clauses `a -> x_i` and `x_1 & .. & x_d -> a`)
//! and XOR rows over variables and atoms; unit propagation runs on both until
//! a fixpoint; the search is a depth-first walk over a branching order,
//! trying the value 0 first (one *decision* is counted per node whose left
//! branch is tried, exactly WDSat's `conf[0]++`). The branching order is the
//! only thing an arm changes. Counted: decisions `D`, propagated assignments
//! `P` (atoms included, decision literals excluded), probe assignments `Q`
//! (lookahead done by a scorer, charged in full), and `W`, the unpriced
//! native scoring work (incidence scans, model arithmetic, order scans).
//! The cost of an arm at weight `k` for one unit of `W` is `D + P + Q + k*W`.

use std::collections::HashMap;
use std::io::Write as _;
use std::process::{Command, Stdio};

// ---------------------------------------------------------------- utilities

#[derive(Clone)]
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % (n as u64)) as usize
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
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

fn fail(msg: &str) -> ! {
    eprintln!("FAIL: {msg}");
    std::process::exit(2);
}

fn l2(x: f64) -> f64 {
    (1.0 + x).log2()
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
        Gf::with_low(n, find_irreducible_sparse(n))
    }
    /// `GF(2^n)` defined by `z^n + low` (bit `i` of `low` is the coefficient of `z^i`).
    fn with_low(n: u32, low: u64) -> Gf {
        let mask = (1u64 << n) - 1;
        let mut g = Gf { n, low, mask, pow: Vec::new() };
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
    /// `S_3(a, c, d) = (a+c)^2 d^2 + a c d + (a c)^2 + 1` (curve `y^2+xy=x^3+1`).
    fn s3(&self, a: u64, c: u64, d: u64) -> u64 {
        let ac = self.mul(a, c);
        self.mul(self.sqr(a ^ c), self.sqr(d)) ^ self.mul(ac, d) ^ self.sqr(ac) ^ 1
    }
    /// Every `d` with `S_3(a, c, d) = 0`; the roots form an affine subspace.
    fn roots(&self, a: u64, c: u64) -> Vec<u64> {
        let lead = self.sqr(a ^ c);
        let mid = self.mul(a, c);
        let tail = self.sqr(mid) ^ 1;
        let n = self.n as usize;
        let mut piv: Vec<(u64, u64)> = Vec::new();
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

fn span_of(basis: &[u64]) -> Vec<u64> {
    let l = basis.len();
    (0..1u64 << l)
        .map(|c| (0..l).filter(|t| (c >> t) & 1 == 1).fold(0, |a, t| a ^ basis[t]))
        .collect()
}

/// Exact number of solutions of the m = 3 chained system (tuples of subspace
/// coordinates and the intermediate field element), by dynamic programming.
fn count_solutions(gf: &Gf, basis: &[u64], xr: u64) -> u64 {
    let span = span_of(basis);
    let mut state: HashMap<u64, u64> = HashMap::new();
    for &x1 in &span {
        for &x2 in &span {
            for u in gf.roots(x1, x2) {
                *state.entry(u).or_insert(0) += 1;
            }
        }
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

type Poly = Vec<u64>; // ascending, unique monomial masks; mask 0 is the constant 1

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

/// Number of unknowns of the symmetrised system for factor-base dimension `l`:
/// `3l` subspace coordinates, then the coordinates of the elementary symmetric
/// functions `e1` (`l` bits), `e2` (`2l-1`) and `e3` (`3l-2`).
fn nvars_sym(l: usize) -> usize {
    9 * l - 3
}

/// The symmetrised fourth-summation-polynomial system `S'_4` of the WDSat
/// corpus (Trimoska, Ionica, Dequen; `EC-Index-Calculus-Benchmarks`, the
/// committed ICPERF instances `n15l5`, `n17l6`, `n19l6`), for the factor base
/// `V = span{1, z, .., z^(l-1)}` and the curve `y^2 + xy = x^3 + x^2 + 1`.
///
/// Unknowns (the corpus numbering): `x1` bits `0..l`, `x2` bits, `x3` bits,
/// then `e1`, `e2`, `e3`, the coordinates (polynomial basis, no reduction while
/// `n >= 3l - 2`) of `e1 = x1+x2+x3`, `e2 = x1x2+x1x3+x2x3`, `e3 = x1x2x3`.
/// Equations: the `4l-1+3l-2-..` definitions of the `e` bits, and the `n` bit
/// equations of
///   `S_4 = t^4 + t^3 e3 + (1+t^4) e3^2 + t e3^3 + e3^4 + t^2 e2^2 + t^3 e2^2 e3
///          + t^4 e2^4 + t e1^2 e3 + t^2 e1^2 e3^2 + e1^4`,  `t = x_R`,
/// the resultant `Res_u(S_3(x1,x2,u), S_3(u,x3,x_R))` written in `e1,e2,e3`
/// (coefficients found by an exact F_2 fit and re-checked in the selftest
/// against `s4_eval` on random field elements).
fn build_system(gf: &Gf, basis: &[u64], xr: u64) -> (Vec<Poly>, usize) {
    let l = basis.len();
    assert!(basis.iter().enumerate().all(|(i, &b)| b == 1u64 << i), "polynomial-basis factor base");
    let n = gf.n as usize;
    assert!(n >= 3 * l - 2, "n >= 3l-2 keeps e1,e2,e3 unreduced");
    let nv = nvars_sym(l);
    assert!(nv <= 62, "at most 62 unknowns in this build");
    let xv = |blk: usize, t: usize| -> u64 { 1u64 << (blk * l + t) };
    let e1v = |k: usize| -> u64 { 1u64 << (3 * l + k) };
    let e2v = |k: usize| -> u64 { 1u64 << (4 * l + k) };
    let e3v = |k: usize| -> u64 { 1u64 << (6 * l - 1 + k) };
    let mut eqs: Vec<Poly> = Vec::new();
    for k in 0..l {
        eqs.push(norm(vec![e1v(k), xv(0, k), xv(1, k), xv(2, k)]));
    }
    for k in 0..2 * l - 1 {
        let mut t = vec![e2v(k)];
        for a in 0..l {
            if k >= a && k - a < l {
                let b = k - a;
                t.push(xv(0, a) | xv(1, b));
                t.push(xv(0, a) | xv(2, b));
                t.push(xv(1, a) | xv(2, b));
            }
        }
        eqs.push(norm(t));
    }
    for k in 0..3 * l - 2 {
        let mut t = vec![e3v(k)];
        for a in 0..l {
            for b in 0..l {
                if k >= a + b && k - a - b < l {
                    t.push(xv(0, a) | xv(1, b) | xv(2, k - a - b));
                }
            }
        }
        eqs.push(norm(t));
    }
    let coord = |len: usize, var: &dyn Fn(usize) -> u64| -> Sym {
        (0..n).map(|k| if k < len { vec![var(k)] } else { vec![] }).collect()
    };
    let e1 = coord(l, &e1v);
    let e2 = coord(2 * l - 1, &e2v);
    let e3 = coord(3 * l - 2, &e3v);
    let pw = |k: u32| -> u64 {
        let mut r = 1u64;
        for _ in 0..k {
            r = gf.mul(r, xr);
        }
        r
    };
    let cmul = |c: u64, x: &Sym| -> Sym { sym_mul(&sym_const(c, gf.n), x, gf) };
    let e3s = sym_sq(&e3, gf);
    let e3c = sym_mul(&e3s, &e3, gf);
    let e3q = sym_sq(&e3s, gf);
    let e2s = sym_sq(&e2, gf);
    let e2q = sym_sq(&e2s, gf);
    let e1s = sym_sq(&e1, gf);
    let e1q = sym_sq(&e1s, gf);
    let e1s_e3 = sym_mul(&e1s, &e3, gf);
    let e1e3_sq = sym_sq(&sym_mul(&e1, &e3, gf), gf);
    let e2s_e3 = sym_mul(&e2s, &e3, gf);
    let mut f = sym_const(pw(4), gf.n);
    f = sym_add(&f, &cmul(pw(3), &e3));
    f = sym_add(&f, &cmul(1 ^ pw(4), &e3s));
    f = sym_add(&f, &cmul(pw(1), &e3c));
    f = sym_add(&f, &e3q);
    f = sym_add(&f, &cmul(pw(2), &e2s));
    f = sym_add(&f, &cmul(pw(3), &e2s_e3));
    f = sym_add(&f, &cmul(pw(4), &e2q));
    f = sym_add(&f, &cmul(pw(1), &e1s_e3));
    f = sym_add(&f, &cmul(pw(2), &e1e3_sq));
    f = sym_add(&f, &e1q);
    eqs.extend(f);
    eqs.retain(|p| !p.is_empty());
    (eqs, nv)
}

/// `Res(x1, x2, x3, x_R)` evaluated with field arithmetic (zero iff the direct
/// system holds at these field elements).
fn s4_eval(gf: &Gf, x1: u64, x2: u64, x3: u64, xr: u64) -> u64 {
    let a = gf.sqr(x1 ^ x2);
    let b = gf.mul(x1, x2);
    let c = gf.sqr(b) ^ 1;
    let a2 = gf.sqr(x3 ^ xr);
    let b2 = gf.mul(x3, xr);
    let c2 = gf.sqr(b2) ^ 1;
    let t1 = gf.sqr(gf.mul(a, c2) ^ gf.mul(a2, c));
    let t2a = gf.mul(a, b2) ^ gf.mul(a2, b);
    let t2b = gf.mul(b, c2) ^ gf.mul(b2, c);
    t1 ^ gf.mul(t2a, t2b)
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
