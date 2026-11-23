//! ROUND-2 COPY of research/quartic-diem-sparse_20261115/src/qds_core.rs (sha256 in research/isogeny-walk-quartic_20261121/REPORT.md):
//! `Instance` gains `extra` targets, `run_sparse` descends for them after the shared relation/LA phase,
//! and a driver `smith_ic` is appended at the end. Everything else is unchanged.
//! MODIFIED COPY of examples/quartic_ic.rs (sha256 d24680aa84e6149b6e919f40791ee950aaf7ee0eb23a54acde469561eeb15bb6)
//! for research/quartic-diem-sparse_20261115/: group law, instance builder, rho and the dense IC arm are
//! unchanged; the sparse arm (`run_sparse`) and `cli` are appended at the end of the file.
//!
//! Double-large-prime index calculus on the Jacobian of a plane quartic over F_q, against
//! Pollard rho in the same group, with one set of counters.
//! (`research/tian_n3_quartic_route_20261024/PROTOCOL.md`, item (b).)
//!
//! Group. `Jac(C)(F_q)`, `C : F(x, y) = 0` a smooth plane quartic with a rational non-flex point
//! `O`.  A class is `[D - 3 O]`, `D` effective of degree 3, stored as a Mumford-like pair
//! `(a(x), b(x))` (ideal `(a, y - b)`).  Addition uses two cubics: a cubic `G0` through
//! `D1 + D2` cuts a residual `R` of degree 6; a cubic `G1` through `R + 3 O` cuts the residual
//! `D3`; `D1 + D2 + R ~ 3H ~ R + 3 O + D3` gives `D3 ~ D1 + D2 - 3 O`.  Subtraction takes the
//! cubics through `D1 + 3 O` and then `R + D2`.  Residual divisors are obtained from
//! `Res_y(G, F)` (evaluated at 13 points and interpolated) divided by the known divisor, and
//! their `y`-coordinates from a Euclid gcd over `F_q[x]/(r)`.
//!
//! Index calculus (Diem 2006; Diem-Thome 2008, implemented from the published idea): factor base
//! of rational points, relations from lines through two factor-base points whose residual pair is
//! rational, double large primes handled as a graph with the factor base contracted to one
//! ground vertex, dense linear algebra modulo the subgroup order `ell`, target introduced by
//! walking `a P + b Q` until the degree-3 divisor splits into points of known logarithm.
//!
//! Counters: `M` (F_q multiplications/squarings), `I` (inversions), `NAT` (native operations:
//! hash probes, table lookups, union-find steps, comparisons), `LA` (modular multiply-adds in the
//! linear algebra modulo `ell`).
//!
//! std-only; `rustc --edition 2021 -O examples/quartic_ic.rs`.
//! Usage: `quartic_ic selftest | run <out-dir> <q> <n> <seed> <B-mult-x100> | summarize <out-dir> <q>...`
#![allow(
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil,
    clippy::many_single_char_names,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::doc_lazy_continuation,
    clippy::match_like_matches_macro,
    clippy::new_without_default,
    clippy::large_enum_variant,
    clippy::manual_memcpy,
    dead_code
)]

use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Write as _;

// ---------------------------------------------------------------- counters

thread_local! {
    static C_M: Cell<u64> = const { Cell::new(0) };
    static C_I: Cell<u64> = const { Cell::new(0) };
    static C_NAT: Cell<u64> = const { Cell::new(0) };
    static C_LA: Cell<u64> = const { Cell::new(0) };
}

#[inline]
fn cm(n: u64) {
    C_M.with(|c| c.set(c.get() + n));
}
#[inline]
fn ci(n: u64) {
    C_I.with(|c| c.set(c.get() + n));
}
#[inline]
fn cn(n: u64) {
    C_NAT.with(|c| c.set(c.get() + n));
}
#[inline]
fn cl(n: u64) {
    C_LA.with(|c| c.set(c.get() + n));
}

#[derive(Clone, Copy, Default, Debug)]
struct Cnt {
    m: u64,
    i: u64,
    nat: u64,
    la: u64,
}

fn snap() -> Cnt {
    Cnt {
        m: C_M.with(|c| c.get()),
        i: C_I.with(|c| c.get()),
        nat: C_NAT.with(|c| c.get()),
        la: C_LA.with(|c| c.get()),
    }
}

fn diff(a: Cnt, b: Cnt) -> Cnt {
    Cnt {
        m: a.m - b.m,
        i: a.i - b.i,
        nat: a.nat - b.nat,
        la: a.la - b.la,
    }
}

fn addc(a: Cnt, b: Cnt) -> Cnt {
    Cnt {
        m: a.m + b.m,
        i: a.i + b.i,
        nat: a.nat + b.nat,
        la: a.la + b.la,
    }
}

// -------------------------------------------------------------------- RNG

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xA076_1D64_78BD_642F)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
}

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// --------------------------------------------------------------- field F_q

#[derive(Clone, Copy)]
struct Fq(u64);

impl Fq {
    #[inline]
    fn mul(self, a: u64, b: u64) -> u64 {
        cm(1);
        a * b % self.0
    }
    #[inline]
    fn add(self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.0 {
            s - self.0
        } else {
            s
        }
    }
    #[inline]
    fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.0 - b
        }
    }
    #[inline]
    fn neg(self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.0 - a
        }
    }
    fn pow(self, a: u64, mut e: u64) -> u64 {
        let mut r = 1;
        let mut b = a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    /// Inverse by the extended Euclidean algorithm; counted as one inversion.
    fn inv(self, a: u64) -> u64 {
        ci(1);
        assert!(a != 0, "inverse of zero");
        let (mut r0, mut r1) = (self.0 as i64, a as i64);
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
        t0.rem_euclid(self.0 as i64) as u64
    }
}

// ------------------------------------------------------- polynomials F_q[x]

type Poly = Vec<u64>;

fn trim(p: &mut Poly) {
    while let Some(&l) = p.last() {
        if l == 0 {
            p.pop();
        } else {
            break;
        }
    }
}

fn padd(f: Fq, a: &[u64], b: &[u64]) -> Poly {
    let n = a.len().max(b.len());
    let mut r = vec![0; n];
    for i in 0..n {
        let x = if i < a.len() { a[i] } else { 0 };
        let y = if i < b.len() { b[i] } else { 0 };
        r[i] = f.add(x, y);
    }
    trim(&mut r);
    r
}

fn psub(f: Fq, a: &[u64], b: &[u64]) -> Poly {
    let n = a.len().max(b.len());
    let mut r = vec![0; n];
    for i in 0..n {
        let x = if i < a.len() { a[i] } else { 0 };
        let y = if i < b.len() { b[i] } else { 0 };
        r[i] = f.sub(x, y);
    }
    trim(&mut r);
    r
}

fn pmul(f: Fq, a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for i in 0..a.len() {
        if a[i] == 0 {
            continue;
        }
        for j in 0..b.len() {
            r[i + j] = (r[i + j] + a[i] * b[j]) % f.0;
        }
    }
    cm((a.len() * b.len()) as u64);
    trim(&mut r);
    r
}

fn pscale(f: Fq, a: &[u64], c: u64) -> Poly {
    let mut r: Poly = a.iter().map(|&x| f.mul(x, c)).collect();
    trim(&mut r);
    r
}

/// Quotient and remainder of `a` by the non-zero polynomial `b`.
fn pdivrem(f: Fq, a: &[u64], b: &[u64]) -> (Poly, Poly) {
    let mut rem = a.to_vec();
    trim(&mut rem);
    if rem.len() < b.len() {
        return (vec![], rem);
    }
    let lead = *b.last().unwrap();
    let li = if lead == 1 { 1 } else { f.inv(lead) };
    let mut quo = vec![0u64; rem.len() - b.len() + 1];
    while rem.len() >= b.len() {
        let top = *rem.last().unwrap();
        let c = if lead == 1 { top } else { f.mul(top, li) };
        let sh = rem.len() - b.len();
        quo[sh] = c;
        if c != 0 {
            for i in 0..b.len() {
                let t = f.mul(c, b[i]);
                rem[sh + i] = f.sub(rem[sh + i], t);
            }
        }
        rem.pop();
        trim(&mut rem);
        if rem.is_empty() {
            break;
        }
    }
    trim(&mut quo);
    (quo, rem)
}

fn pmod(f: Fq, a: &[u64], m: &[u64]) -> Poly {
    pdivrem(f, a, m).1
}

fn pmulmod(f: Fq, a: &[u64], b: &[u64], m: &[u64]) -> Poly {
    pmod(f, &pmul(f, a, b), m)
}

/// Inverse of `a` modulo `m` (None if not coprime).
fn pinv(f: Fq, a: &[u64], m: &[u64]) -> Option<Poly> {
    let (mut r0, mut r1) = (m.to_vec(), pmod(f, a, m));
    let (mut t0, mut t1): (Poly, Poly) = (vec![], vec![1]);
    while !r1.is_empty() {
        let (qt, rem) = pdivrem(f, &r0, &r1);
        let t2 = psub(f, &t0, &pmul(f, &qt, &t1));
        r0 = r1;
        r1 = rem;
        t0 = t1;
        t1 = t2;
    }
    if r0.len() != 1 {
        return None;
    }
    let c = f.inv(r0[0]);
    Some(pmod(f, &pscale(f, &t0, c), m))
}

fn pgcd(f: Fq, a: &[u64], b: &[u64]) -> Poly {
    let (mut p, mut s) = (a.to_vec(), b.to_vec());
    trim(&mut p);
    trim(&mut s);
    while !s.is_empty() {
        let r = pmod(f, &p, &s);
        p = s;
        s = r;
    }
    p
}

fn pmonic(f: Fq, a: &[u64]) -> Poly {
    if a.is_empty() {
        return vec![];
    }
    let l = *a.last().unwrap();
    if l == 1 {
        return a.to_vec();
    }
    pscale(f, a, f.inv(l))
}

fn ppowmod(f: Fq, base: &[u64], mut e: u64, m: &[u64]) -> Poly {
    let mut r: Poly = vec![1];
    let mut b = pmod(f, base, m);
    while e > 0 {
        if e & 1 == 1 {
            r = pmulmod(f, &r, &b, m);
        }
        e >>= 1;
        if e > 0 {
            b = pmulmod(f, &b, &b, m);
        }
    }
    r
}

/// Rational roots of a polynomial (distinct), by gcd with `x^q - x` and Cantor-Zassenhaus.
fn poly_roots(f: Fq, p: &[u64], rng: &mut Rng) -> Vec<u64> {
    let mut p = p.to_vec();
    trim(&mut p);
    if p.len() < 2 {
        return vec![];
    }
    let p = pmonic(f, &p);
    let xq = ppowmod(f, &[0, 1], f.0, &p);
    let g = pgcd(f, &p, &psub(f, &xq, &[0, 1]));
    let mut out = Vec::new();
    split_roots(f, &pmonic(f, &g), rng, &mut out);
    out
}

fn split_roots(f: Fq, g: &[u64], rng: &mut Rng, out: &mut Vec<u64>) {
    match g.len() {
        0 | 1 => {}
        2 => out.push(f.neg(g[0])),
        _ => loop {
            let d = rng.below(f.0);
            let h = ppowmod(f, &[d, 1], (f.0 - 1) / 2, g);
            let h = psub(f, &h, &[1]);
            let g1 = pgcd(f, g, &h);
            if g1.len() > 1 && g1.len() < g.len() {
                let g1 = pmonic(f, &g1);
                let (g2, _) = pdivrem(f, g, &g1);
                split_roots(f, &g1, rng, out);
                split_roots(f, &pmonic(f, &g2), rng, out);
                return;
            }
        },
    }
}

fn peval(f: Fq, p: &[u64], x: u64) -> u64 {
    let mut r = 0;
    for &c in p.iter().rev() {
        r = f.add(f.mul(r, x), c);
    }
    r
}

// ------------------------------------------------------------- the curve

const MON3: [(usize, usize); 10] = [
    (0, 0),
    (1, 0),
    (0, 1),
    (2, 0),
    (1, 1),
    (0, 2),
    (3, 0),
    (2, 1),
    (1, 2),
    (0, 3),
];

fn mon3_index(i: usize, j: usize) -> usize {
    MON3.iter().position(|&m| m == (i, j)).unwrap()
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Div {
    a: Poly,
    b: Poly,
}

impl Div {
    fn key(&self) -> [u64; 6] {
        let mut k = [0u64; 6];
        for i in 0..self.a.len().min(3) {
            k[i] = self.a[i];
        }
        for i in 0..self.b.len().min(3) {
            k[3 + i] = self.b[i];
        }
        k
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Fail {
    Rank,
    Lead,
    Infinity,
    NotDivisible,
    Ycoord,
}

struct Curve {
    fq: Fq,
    f: [[u64; 5]; 5], // f[i][j]: coefficient of x^i y^j
    fpoly: Vec<Poly>, // F as a polynomial in y: fpoly[j] is a polynomial in x
    o: (u64, u64),
    id3: Div,
    id6: Div,
    xpow: Vec<[u64; 5]>, // powers of the 13 sample points
    fat: Vec<[u64; 5]>,  // F's y-coefficients at the 13 sample points
    lag: Vec<Poly>,      // Lagrange basis for the sample points
}

fn series_at_o(fq: Fq, f: &[[u64; 5]; 5], o: (u64, u64), prec: usize) -> Poly {
    // y(t) with x = xo + t, F(x, y(t)) = 0, mod t^prec; returns coefficients in t
    let fy = {
        let mut s = 0;
        for i in 0..5 {
            for j in 1..5 {
                if f[i][j] != 0 {
                    let t = fq.mul(f[i][j], j as u64 % fq.0);
                    let xp = fq.pow(o.0, i as u64);
                    let yp = fq.pow(o.1, (j - 1) as u64);
                    s = fq.add(s, fq.mul(t, fq.mul(xp, yp)));
                }
            }
        }
        s
    };
    let fyi = fq.inv(fy);
    let mut y: Poly = vec![o.1];
    for k in 1..prec {
        y.resize(prec, 0);
        // coefficient of t^k in F(xo + t, y(t)) with s_k = 0
        let c = coeff_t(fq, f, o.0, &y, k, prec);
        y[k] = fq.mul(fq.neg(c), fyi);
    }
    y.truncate(prec);
    y
}

fn tmul(fq: Fq, a: &[u64], b: &[u64], prec: usize) -> Poly {
    let mut r = vec![0u64; prec];
    for i in 0..a.len().min(prec) {
        for j in 0..b.len().min(prec - i) {
            r[i + j] = (r[i + j] + a[i] * b[j]) % fq.0;
        }
    }
    cm((a.len() * b.len()) as u64);
    r
}

fn coeff_t(fq: Fq, f: &[[u64; 5]; 5], xo: u64, y: &[u64], k: usize, prec: usize) -> u64 {
    // x = xo + t
    let xt: Poly = vec![xo, 1];
    let mut xp: Vec<Poly> = vec![vec![1]];
    let mut yp: Vec<Poly> = vec![vec![1]];
    for e in 1..5 {
        xp.push(tmul(fq, &xp[e - 1], &xt, prec));
        yp.push(tmul(fq, &yp[e - 1], y, prec));
    }
    let mut c = 0;
    for i in 0..5 {
        for j in 0..(5 - i) {
            if f[i][j] == 0 {
                continue;
            }
            let t = tmul(fq, &xp[i], &yp[j], prec);
            c = fq.add(c, fq.mul(f[i][j], t[k]));
        }
    }
    c
}

impl Curve {
    fn new(fq: Fq, f: [[u64; 5]; 5], o: (u64, u64)) -> Curve {
        let mut fpoly = Vec::new();
        for j in 0..5 {
            let mut p: Poly = (0..(5 - j)).map(|i| f[i][j]).collect();
            trim(&mut p);
            fpoly.push(p);
        }
        // identities 3O and 6O
        let mk = |m: usize| -> Div {
            let ys = series_at_o(fq, &f, o, m);
            // b(x) = sum s_k (x - xo)^k
            let mut b: Poly = vec![];
            let mut pw: Poly = vec![1];
            for k in 0..m {
                b = padd(fq, &b, &pscale(fq, &pw, ys[k]));
                pw = pmul(fq, &pw, &[fq.neg(o.0), 1]);
            }
            let mut a: Poly = vec![1];
            for _ in 0..m {
                a = pmul(fq, &a, &[fq.neg(o.0), 1]);
            }
            let b = pmod(fq, &b, &a);
            Div { a, b }
        };
        let id3 = mk(3);
        let id6 = mk(6);
        // sample points 0..12
        let mut xpow = Vec::new();
        let mut fat = Vec::new();
        for t in 0..13u64 {
            let mut xp = [1u64; 5];
            for e in 1..5 {
                xp[e] = xp[e - 1] * t % fq.0;
            }
            xpow.push(xp);
            let mut fa = [0u64; 5];
            for j in 0..5 {
                let mut s = 0;
                for i in 0..(5 - j) {
                    s = (s + f[i][j] * xp[i]) % fq.0;
                }
                fa[j] = s;
            }
            fat.push(fa);
        }
        let mut lag = Vec::new();
        for t in 0..13u64 {
            let mut num: Poly = vec![1];
            let mut den = 1u64;
            for s in 0..13u64 {
                if s == t {
                    continue;
                }
                num = pmul(fq, &num, &[fq.neg(s), 1]);
                den = fq.mul(den, fq.sub(t, s));
            }
            lag.push(pscale(fq, &num, fq.inv(den)));
        }
        Curve {
            fq,
            f,
            fpoly,
            o,
            id3,
            id6,
            xpow,
            fat,
            lag,
        }
    }
}

fn det_mod(f: Fq, m: &mut [Vec<u64>]) -> u64 {
    let n = m.len();
    let mut det = 1u64;
    for col in 0..n {
        let Some(p) = (col..n).find(|&r| m[r][col] != 0) else {
            return 0;
        };
        if p != col {
            m.swap(p, col);
            det = f.neg(det);
        }
        det = f.mul(det, m[col][col]);
        let inv = f.inv(m[col][col]);
        for r in (col + 1)..n {
            if m[r][col] == 0 {
                continue;
            }
            let fac = f.mul(m[r][col], inv);
            for c2 in col..n {
                let t = f.mul(fac, m[col][c2]);
                m[r][c2] = f.sub(m[r][c2], t);
            }
        }
    }
    det
}

/// Kernel of the row space (basis of `{g : rows * g = 0}`), 10 columns.
fn kernel10(f: Fq, mut rows: Vec<[u64; 10]>) -> Vec<[u64; 10]> {
    let n = rows.len();
    let mut pivcols = Vec::new();
    let mut r = 0;
    for col in 0..10 {
        if r >= n {
            break;
        }
        let Some(p) = (r..n).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(p, r);
        let inv = f.inv(rows[r][col]);
        for c in 0..10 {
            rows[r][c] = f.mul(rows[r][c], inv);
        }
        for i in 0..n {
            if i != r && rows[i][col] != 0 {
                let fac = rows[i][col];
                for c in 0..10 {
                    let t = f.mul(fac, rows[r][c]);
                    rows[i][c] = f.sub(rows[i][c], t);
                }
            }
        }
        pivcols.push(col);
        r += 1;
    }
    let mut basis = Vec::new();
    for fc in 0..10 {
        if pivcols.contains(&fc) {
            continue;
        }
        let mut v = [0u64; 10];
        v[fc] = 1;
        for (ri, &pc) in pivcols.iter().enumerate() {
            v[pc] = f.neg(rows[ri][fc]);
        }
        basis.push(v);
    }
    basis
}

// ------------------------------------------------------------ group law

struct Ctx {
    c: Curve,
    rng: Rng,
    pool: Vec<Div>,
    fails: u64,
    ops: u64,
}

impl Ctx {
    fn new(c: Curve, seed: u64) -> Ctx {
        Ctx {
            c,
            rng: Rng::new(seed),
            pool: vec![],
            fails: 0,
            ops: 0,
        }
    }

    fn id(&self) -> Div {
        self.c.id3.clone()
    }

    /// Linear conditions on the cubic's 10 coefficients to vanish on `d`.
    fn rows_for(&self, d: &Div) -> Vec<[u64; 10]> {
        let f = self.c.fq;
        let n = d.a.len() - 1;
        let mut bj: Vec<Poly> = vec![vec![1]];
        let bm = pmod(f, &d.b, &d.a);
        for j in 1..4 {
            let prev = bj[j - 1].clone();
            bj.push(pmulmod(f, &prev, &bm, &d.a));
        }
        let mut rows = vec![[0u64; 10]; n];
        for j in 0..4 {
            let mut cur = bj[j].clone();
            for i in 0..(4 - j) {
                let col = mon3_index(i, j);
                for r in 0..n {
                    rows[r][col] = if r < cur.len() { cur[r] } else { 0 };
                }
                cur = pmod(f, &pmul(f, &cur, &[0, 1]), &d.a);
            }
        }
        rows
    }

    fn through(&mut self, ds: &[&Div]) -> Result<[u64; 10], Fail> {
        let f = self.c.fq;
        let mut rows = Vec::new();
        for d in ds {
            rows.extend(self.rows_for(d));
        }
        let want = 10 - rows.len();
        let ker = kernel10(f, rows);
        if ker.len() != want {
            return Err(Fail::Rank);
        }
        for _ in 0..6 {
            let mut g = [0u64; 10];
            for kv in &ker {
                let cf = self.rng.below(f.0);
                for t in 0..10 {
                    g[t] = f.add(g[t], f.mul(cf, kv[t]));
                }
            }
            if g[9] != 0 {
                return Ok(g);
            }
        }
        Err(Fail::Lead)
    }

    /// Residual divisor of `G` on `C` after removing the known divisor (product polynomial `known`).
    fn residual(&mut self, g: &[u64; 10], known: &Poly) -> Result<Div, Fail> {
        let f = self.c.fq;
        let mut vals = [0u64; 13];
        for t in 0..13 {
            let xp = &self.c.xpow[t];
            let mut gj = [0u64; 4];
            for j in 0..4 {
                let mut s = 0;
                for i in 0..(4 - j) {
                    s = f.add(s, f.mul(g[mon3_index(i, j)], xp[i]));
                }
                gj[j] = s;
            }
            let fj = &self.c.fat[t];
            let mut m = vec![vec![0u64; 7]; 7];
            for r in 0..3 {
                for k in 0..5 {
                    m[r][r + k] = fj[4 - k];
                }
            }
            for r in 0..4 {
                for k in 0..4 {
                    m[3 + r][r + k] = gj[3 - k];
                }
            }
            vals[t] = det_mod(f, &mut m);
        }
        let mut rp: Poly = vec![];
        for t in 0..13 {
            if vals[t] != 0 {
                rp = padd(f, &rp, &pscale(f, &self.c.lag[t], vals[t]));
            }
        }
        if rp.len() != 13 {
            return Err(Fail::Infinity);
        }
        let (qt, rem) = pdivrem(f, &rp, known);
        if !rem.is_empty() {
            return Err(Fail::NotDivisible);
        }
        let r = pmonic(f, &qt);
        if r == self.c.id3.a {
            return Ok(self.c.id3.clone());
        }
        // the new points must have x-coordinates different from the known ones, otherwise the
        // gcd below could return a known point's y-coordinate
        if pgcd(f, &r, known).len() > 1 {
            return Err(Fail::Ycoord);
        }
        let b = self.ycoord(g, &r)?;
        Ok(Div { a: r, b })
    }

    /// y-coordinates of the common zeros of `G` and `F` over the roots of `r`.
    fn ycoord(&self, g: &[u64; 10], r: &Poly) -> Result<Poly, Fail> {
        let f = self.c.fq;
        // y-polynomials with coefficients in F_q[x]/(r)
        let mut fy: Vec<Poly> = self.c.fpoly.iter().map(|p| pmod(f, p, r)).collect();
        let mut gy: Vec<Poly> = Vec::new();
        for j in 0..4 {
            let mut p: Poly = (0..(4 - j)).map(|i| g[mon3_index(i, j)]).collect();
            trim(&mut p);
            gy.push(pmod(f, &p, r));
        }
        let inv3 = f.inv(g[9]);
        // fy = fy mod gy (g has constant leading coefficient)
        for d in (3..5).rev() {
            if fy.len() <= d {
                continue;
            }
            let c = pscale(f, &fy[d], inv3);
            if c.is_empty() {
                continue;
            }
            for k in 0..4 {
                let t = pmulmod(f, &c, &gy[k], r);
                let idx = d - 3 + k;
                if idx < fy.len() {
                    fy[idx] = psub(f, &fy[idx], &t);
                }
            }
        }
        fy.truncate(3);
        while fy.len() < 3 {
            fy.push(vec![]);
        }
        let lead = fy[2].clone();
        let Some(li) = pinv(f, &lead, r) else {
            return Err(Fail::Ycoord);
        };
        // gy mod fy
        let mut gg = gy.clone();
        for d in (2..4).rev() {
            let c = pmulmod(f, &gg[d], &li, r);
            if c.is_empty() {
                continue;
            }
            for k in 0..3 {
                let t = pmulmod(f, &c, &fy[k], r);
                let idx = d - 2 + k;
                gg[idx] = psub(f, &gg[idx], &t);
            }
        }
        // gg = u y + v
        let u = gg[1].clone();
        let v = gg[0].clone();
        let Some(ui) = pinv(f, &u, r) else {
            return Err(Fail::Ycoord);
        };
        let b = pmulmod(f, &psub(f, &[], &v), &ui, r);
        // check F(x, b) = 0 mod r
        let mut acc: Poly = vec![];
        for j in (0..5).rev() {
            acc = pmod(f, &padd(f, &pmulmod(f, &acc, &b, r), &self.c.fpoly[j]), r);
        }
        if !acc.is_empty() {
            return Err(Fail::Ycoord);
        }
        Ok(b)
    }

    fn lift(&self, d: &Div) -> Result<Div, Fail> {
        let f = self.c.fq;
        let a2 = pmul(f, &d.a, &d.a);
        let b = &d.b;
        let mut e: Poly = vec![];
        for j in (0..5).rev() {
            e = pmod(f, &padd(f, &pmulmod(f, &e, b, &a2), &self.c.fpoly[j]), &a2);
        }
        let mut fyv: Poly = vec![];
        for j in (1..5).rev() {
            let cj = pscale(f, &self.c.fpoly[j], j as u64 % f.0);
            fyv = pmod(f, &padd(f, &pmulmod(f, &fyv, b, &d.a), &cj), &d.a);
        }
        let Some(fi) = pinv(f, &fyv, &d.a) else {
            return Err(Fail::Ycoord);
        };
        let corr = pmulmod(f, &e, &fi, &a2);
        let bt = pmod(f, &psub(f, b, &corr), &a2);
        Ok(Div { a: a2, b: bt })
    }

    fn retry<F>(&mut self, mut op: F) -> Result<Div, Fail>
    where
        F: FnMut(&mut Ctx) -> Result<Div, Fail>,
    {
        let mut last = Fail::Rank;
        for _ in 0..4 {
            match op(self) {
                Ok(r) => return Ok(r),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    fn add(&mut self, d1: &Div, d2: &Div) -> Result<Div, Fail> {
        self.ops += 1;
        if *d1 == self.c.id3 {
            return Ok(d2.clone());
        }
        if *d2 == self.c.id3 {
            return Ok(d1.clone());
        }
        if d1 == d2 {
            self.ops -= 1;
            return self.dbl(d1);
        }
        let (d1, d2) = (d1.clone(), d2.clone());
        self.retry(|s| {
            let f = s.c.fq;
            let known = pmul(f, &d1.a, &d2.a);
            let g0 = s.through(&[&d1, &d2])?;
            let r = s.residual(&g0, &known)?;
            let id3 = s.c.id3.clone();
            let g1 = s.through(&[&r, &id3])?;
            let known2 = pmul(f, &r.a, &id3.a);
            s.residual(&g1, &known2)
        })
    }

    fn dbl(&mut self, d: &Div) -> Result<Div, Fail> {
        self.ops += 1;
        if *d == self.c.id3 {
            return Ok(d.clone());
        }
        let d = d.clone();
        let dl = self.lift(&d)?;
        self.retry(|s| {
            let f = s.c.fq;
            let g0 = s.through(&[&dl])?;
            let r = s.residual(&g0, &dl.a)?;
            let id3 = s.c.id3.clone();
            let g1 = s.through(&[&r, &id3])?;
            let known2 = pmul(f, &r.a, &id3.a);
            s.residual(&g1, &known2)
        })
    }

    fn sub(&mut self, d1: &Div, d2: &Div) -> Result<Div, Fail> {
        self.ops += 1;
        if *d2 == self.c.id3 {
            return Ok(d1.clone());
        }
        if d1 == d2 {
            return Ok(self.c.id3.clone());
        }
        if *d1 == self.c.id3 {
            self.ops -= 1;
            return self.neg(d2);
        }
        let (d1, d2) = (d1.clone(), d2.clone());
        self.retry(|s| {
            let f = s.c.fq;
            let id3 = s.c.id3.clone();
            let g0 = s.through(&[&d1, &id3])?;
            let known = pmul(f, &d1.a, &id3.a);
            let r = s.residual(&g0, &known)?;
            let g1 = s.through(&[&r, &d2])?;
            let known2 = pmul(f, &r.a, &d2.a);
            s.residual(&g1, &known2)
        })
    }

    fn neg(&mut self, d: &Div) -> Result<Div, Fail> {
        self.ops += 1;
        if *d == self.c.id3 {
            return Ok(d.clone());
        }
        let d = d.clone();
        self.retry(|s| {
            let f = s.c.fq;
            let id6 = s.c.id6.clone();
            let g0 = s.through(&[&id6])?;
            let r = s.residual(&g0, &id6.a)?;
            let g1 = s.through(&[&r, &d])?;
            let known2 = pmul(f, &r.a, &d.a);
            s.residual(&g1, &known2)
        })
    }

    fn pool_elem(&mut self) -> (Div, Div) {
        let i = self.rng.below(self.pool.len() as u64) as usize;
        let c = self.pool[i].clone();
        let nc = self.pool[i ^ 1].clone();
        (c, nc)
    }

    /// Group operations with a randomising fallback on degenerate configurations.
    fn add_r(&mut self, d1: &Div, d2: &Div) -> Option<Div> {
        if let Ok(r) = self.add(d1, d2) {
            return Some(r);
        }
        self.fails += 1;
        for _ in 0..12 {
            let (c, nc) = self.pool_elem();
            let Ok(s1) = self.add(d1, &c) else { continue };
            let Ok(s2) = self.add(d2, &nc) else { continue };
            if let Ok(r) = self.add(&s1, &s2) {
                return Some(r);
            }
        }
        None
    }

    fn sub_r(&mut self, d1: &Div, d2: &Div) -> Option<Div> {
        if let Ok(r) = self.sub(d1, d2) {
            return Some(r);
        }
        self.fails += 1;
        let n = self.neg_r(d2)?;
        self.add_r(d1, &n)
    }

    fn dbl_r(&mut self, d: &Div) -> Option<Div> {
        if let Ok(r) = self.dbl(d) {
            return Some(r);
        }
        self.fails += 1;
        for _ in 0..12 {
            let (c, nc) = self.pool_elem();
            let Ok(s1) = self.add(d, &c) else { continue };
            let Ok(s2) = self.add(d, &nc) else { continue };
            if let Ok(r) = self.add(&s1, &s2) {
                return Some(r);
            }
        }
        None
    }

    fn neg_r(&mut self, d: &Div) -> Option<Div> {
        if let Ok(r) = self.neg(d) {
            return Some(r);
        }
        self.fails += 1;
        let id = self.id();
        // -d = (pool c) - (d + c)
        for _ in 0..12 {
            let (c, _) = self.pool_elem();
            let Ok(s) = self.add(d, &c) else { continue };
            if let Ok(r) = self.sub(&id, &s) {
                if let Ok(r2) = self.add(&r, &c) {
                    return Some(r2);
                }
            }
        }
        None
    }

    /// Scalar multiplication; `None` if the result or a chain element cannot be represented.
    fn smul(&mut self, k: u64, d: &Div) -> Option<Div> {
        let mut r = self.id();
        if k == 0 {
            return Some(r);
        }
        let nb = 64 - k.leading_zeros();
        for bit in (0..nb).rev() {
            r = self.dbl_r(&r)?;
            if (k >> bit) & 1 == 1 {
                r = self.add_r(&r, d)?;
            }
        }
        Some(r)
    }
}

// ------------------------------------------------------- curve generation

fn eval_f(c: &Curve, x: u64, y: u64) -> u64 {
    let f = c.fq;
    let mut r = 0;
    for j in (0..5).rev() {
        r = f.add(f.mul(r, y), peval(f, &c.fpoly[j], x));
    }
    r
}

fn count_affine_roots(f: Fq, fy: &[u64; 5], rng: &mut Rng) -> Vec<u64> {
    let mut p = fy.to_vec();
    trim(&mut p);
    poly_roots(f, &p, rng)
}

fn rational_point_count(f: Fq, fcoef: &[[u64; 5]; 5], rng: &mut Rng) -> u64 {
    let mut n = 0u64;
    for x in 0..f.0 {
        let mut fy = [0u64; 5];
        let mut xp = 1;
        let mut xpows = [1u64; 5];
        for e in 1..5 {
            xp = f.mul(xp, x);
            xpows[e] = xp;
        }
        for j in 0..5 {
            let mut s = 0;
            for i in 0..(5 - j) {
                s = f.add(s, f.mul(fcoef[i][j], xpows[i]));
            }
            fy[j] = s;
        }
        n += count_affine_roots(f, &fy, rng).len() as u64;
    }
    // points at infinity: [1:y:0] roots of the top form, and [0:1:0]
    let mut top = [0u64; 5];
    for j in 0..5 {
        top[j] = fcoef[4 - j][j];
    }
    if fcoef[0][4] == 0 {
        n += 1;
    }
    n += count_affine_roots(f, &top, rng).len() as u64;
    n
}

/// Smoothness certificate (sound): the three pairwise resultants of the partials have gcd 1 in the
/// chart z = 1, and a pair of the binary cubics at z = 0 has non-zero resultant.
fn is_smooth(f: Fq, fc: &[[u64; 5]; 5]) -> bool {
    // partials in the chart z = 1 as cubic forms P[i][j] (x^i y^j)
    let mut px = [[0u64; 4]; 4];
    let mut py = [[0u64; 4]; 4];
    let mut pz = [[0u64; 4]; 4];
    for i in 0..5 {
        for j in 0..(5 - i) {
            let c = fc[i][j];
            if c == 0 {
                continue;
            }
            if i > 0 {
                px[i - 1][j] = f.add(px[i - 1][j], f.mul(c, i as u64));
            }
            if j > 0 {
                py[i][j - 1] = f.add(py[i][j - 1], f.mul(c, j as u64));
            }
            let k = 4 - i - j;
            if k > 0 {
                pz[i][j] = f.add(pz[i][j], f.mul(c, k as u64));
            }
        }
    }
    let ps = [px, py, pz];
    let res33 = |a: &[u64; 4], b: &[u64; 4]| -> u64 {
        let mut m = vec![vec![0u64; 6]; 6];
        for r in 0..3 {
            for j in 0..4 {
                m[r][r + j] = a[3 - j];
                m[r + 3][r + j] = b[3 - j];
            }
        }
        det_mod(f, &mut m)
    };
    // infinity
    let mut inf_ok = false;
    for (a, b) in [(0, 1), (0, 2), (1, 2)] {
        let mut ca = [0u64; 4];
        let mut cb = [0u64; 4];
        for j in 0..4 {
            ca[j] = ps[a][3 - j][j];
            cb[j] = ps[b][3 - j][j];
        }
        if res33(&ca, &cb) != 0 {
            inf_ok = true;
            break;
        }
    }
    if !inf_ok {
        return false;
    }
    // affine: resultant polynomials via 13 evaluations at 0..12 and Lagrange interpolation
    let mut lag: Vec<Poly> = Vec::new();
    for t in 0..13u64 {
        let mut num: Poly = vec![1];
        let mut den = 1u64;
        for s in 0..13u64 {
            if s != t {
                num = pmul(f, &num, &[f.neg(s), 1]);
                den = f.mul(den, f.sub(t, s));
            }
        }
        lag.push(pscale(f, &num, f.inv(den)));
    }
    let respoly = |a: &[[u64; 4]; 4], b: &[[u64; 4]; 4]| -> Poly {
        let mut rp: Poly = vec![];
        for t in 0..13u64 {
            let mut ca = [0u64; 4];
            let mut cb = [0u64; 4];
            for j in 0..4 {
                for i in 0..(4 - j) {
                    let xp = f.pow(t, i as u64);
                    ca[j] = f.add(ca[j], f.mul(a[i][j], xp));
                    cb[j] = f.add(cb[j], f.mul(b[i][j], xp));
                }
            }
            let v = res33(&ca, &cb);
            if v != 0 {
                rp = padd(f, &rp, &pscale(f, &lag[t as usize], v));
            }
        }
        rp
    };
    let r01 = respoly(&ps[0], &ps[1]);
    let r02 = respoly(&ps[0], &ps[2]);
    let r12 = respoly(&ps[1], &ps[2]);
    let g = pgcd(f, &pgcd(f, &r01, &r02), &r12);
    g.len() == 1
}

fn rand_quartic(f: Fq, rng: &mut Rng) -> [[u64; 5]; 5] {
    let mut c = [[0u64; 5]; 5];
    for i in 0..5 {
        for j in 0..(5 - i) {
            c[i][j] = rng.below(f.0);
        }
    }
    while c[0][4] == 0 {
        c[0][4] = rng.below(f.0);
    }
    c
}

fn random_point(f: Fq, fc: &[[u64; 5]; 5], rng: &mut Rng) -> (u64, u64) {
    loop {
        let x = rng.below(f.0);
        let mut fy = [0u64; 5];
        for j in 0..5 {
            let mut s = 0;
            for i in 0..(5 - j) {
                s = f.add(s, f.mul(fc[i][j], f.pow(x, i as u64)));
            }
            fy[j] = s;
        }
        let rts = count_affine_roots(f, &fy, rng);
        if !rts.is_empty() {
            return (x, rts[rng.below(rts.len() as u64) as usize]);
        }
    }
}

/// Partial derivatives at an affine point: (Fx, Fy, Fxx/2, Fxy, Fyy/2).
fn derivs(f: Fq, fc: &[[u64; 5]; 5], x: u64, y: u64) -> [u64; 5] {
    let mut xp = [1u64; 5];
    let mut yp = [1u64; 5];
    for e in 1..5 {
        xp[e] = f.mul(xp[e - 1], x);
        yp[e] = f.mul(yp[e - 1], y);
    }
    let mut d = [0u64; 5];
    let h = f.inv(2);
    for i in 0..5 {
        for j in 0..(5 - i) {
            let c = fc[i][j];
            if c == 0 {
                continue;
            }
            if i >= 1 {
                d[0] = f.add(d[0], f.mul(f.mul(c, i as u64), f.mul(xp[i - 1], yp[j])));
            }
            if j >= 1 {
                d[1] = f.add(d[1], f.mul(f.mul(c, j as u64), f.mul(xp[i], yp[j - 1])));
            }
            if i >= 2 {
                let co = f.mul(c, (i * (i - 1)) as u64 % f.0);
                d[2] = f.add(d[2], f.mul(f.mul(co, h), f.mul(xp[i - 2], yp[j])));
            }
            if i >= 1 && j >= 1 {
                let co = f.mul(c, (i * j) as u64 % f.0);
                d[3] = f.add(d[3], f.mul(co, f.mul(xp[i - 1], yp[j - 1])));
            }
            if j >= 2 {
                let co = f.mul(c, (j * (j - 1)) as u64 % f.0);
                d[4] = f.add(d[4], f.mul(f.mul(co, h), f.mul(xp[i], yp[j - 2])));
            }
        }
    }
    d
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

// ---------------------------------------------------------------- instance

struct Instance {
    ctx: Ctx,
    q: u64,
    n_group: u64,
    ell: u64,
    cof: u64,
    p: Div,
    qq: Div,
    k: u64,
    n1: u64,
    curve_tries: u32,
    extra: Vec<Div>,
}

fn div_from_points(f: Fq, pts: &[(u64, u64)]) -> Option<Div> {
    // distinct x required
    let mut a: Poly = vec![1];
    for &(x, _) in pts {
        a = pmul(f, &a, &[f.neg(x), 1]);
    }
    let mut b: Poly = vec![];
    for (i, &(xi, yi)) in pts.iter().enumerate() {
        let mut num: Poly = vec![1];
        let mut den = 1;
        for (j, &(xj, _)) in pts.iter().enumerate() {
            if i != j {
                num = pmul(f, &num, &[f.neg(xj), 1]);
                let d = f.sub(xi, xj);
                if d == 0 {
                    return None;
                }
                den = f.mul(den, d);
            }
        }
        b = padd(f, &b, &pscale(f, &num, f.mul(yi, f.inv(den))));
    }
    Some(Div { a, b })
}

fn random_class(ctx: &mut Ctx, fc: &[[u64; 5]; 5]) -> Div {
    let f = ctx.c.fq;
    loop {
        let pts: Vec<(u64, u64)> = (0..3).map(|_| random_point(f, fc, &mut ctx.rng)).collect();
        if let Some(d) = div_from_points(f, &pts) {
            return d;
        }
    }
}

/// Group order from `N_1` and a baby-step giant-step search of the Weil interval.
fn find_order(ctx: &mut Ctx, fc: &[[u64; 5]; 5], n1: u64) -> Option<u64> {
    let q = ctx.c.fq.0 as i128;
    let c1 = n1 as i128 - q - 1;
    let center = q * q * q + 1 + (1 + q * q) * c1;
    let w = (15.0 * (q as f64) * (q as f64 + 1.0) + 20.0 * (q as f64).powf(1.5)) as i128;
    let lo = (center - w).max(1);
    let width = (2 * w + 1) as u64;
    let s = (width as f64).sqrt().ceil() as u64 + 1;
    for _try in 0..5 {
        let x = random_class(ctx, fc);
        let Some(x2) = ctx.dbl_r(&x) else { continue };
        // baby steps j X, j in [0, s), skipping unrepresentable elements
        let mut baby: HashMap<[u64; 6], u64> = HashMap::new();
        let mut prev: Option<Div> = None;
        let mut cur: Option<Div> = Some(ctx.id());
        for j in 0..s {
            if let Some(c) = &cur {
                baby.entry(c.key()).or_insert(j);
            }
            let next = match &cur {
                Some(c) => ctx.add_r(c, &x),
                None => None,
            };
            let next = match next {
                Some(n) => Some(n),
                None => match &prev {
                    Some(p) => ctx.add_r(p, &x2),
                    None => None,
                },
            };
            prev = cur;
            cur = next;
        }
        let Some(g) = ctx.smul(s, &x) else { continue };
        let Some(g2) = ctx.dbl_r(&g) else { continue };
        let Some(y) = ctx.smul(lo as u64, &x) else {
            continue;
        };
        let Some(z0) = ctx.neg_r(&y) else { continue };
        let giants = width / s + 2;
        let mut zprev: Option<Div> = None;
        let mut z: Option<Div> = Some(z0);
        let mut found = Vec::new();
        for i in 0..giants {
            if let Some(zz) = &z {
                if let Some(&j) = baby.get(&zz.key()) {
                    let t = i * s + j;
                    if t < width {
                        found.push((lo as u64) + t);
                    }
                }
            }
            let next = match &z {
                Some(c) => ctx.sub_r(c, &g),
                None => None,
            };
            let next = match next {
                Some(n) => Some(n),
                None => match &zprev {
                    Some(p) => ctx.sub_r(p, &g2),
                    None => None,
                },
            };
            zprev = z;
            z = next;
        }
        found.sort();
        found.dedup();
        for &nn in &found {
            let mut good = true;
            for _ in 0..2 {
                let xb = random_class(ctx, fc);
                match ctx.smul(nn, &xb) {
                    Some(r) if r == ctx.id() => {}
                    _ => {
                        good = false;
                        break;
                    }
                }
            }
            if good {
                return Some(nn);
            }
        }
    }
    None
}

fn build_instance(q: u64, seed: u64, max_cof: u64, verbose: bool) -> Option<Instance> {
    let f = Fq(q);
    let mut rng = Rng::new(seed);
    let mut tries = 0;
    loop {
        tries += 1;
        if tries > 400 {
            return None;
        }
        let fc = rand_quartic(f, &mut rng);
        if !is_smooth(f, &fc) {
            continue;
        }
        // point O: non-flex, F_y != 0
        let mut o = None;
        for _ in 0..30 {
            let (x, y) = random_point(f, &fc, &mut rng);
            let d = derivs(f, &fc, x, y);
            if d[1] == 0 {
                continue;
            }
            // tangent direction (Fy, -Fx); second-order term g2 = hxx dx^2 + hxy dx dy + hyy dy^2
            let dx = d[1];
            let dy = f.neg(d[0]);
            let g2 = f.add(
                f.add(f.mul(d[2], f.mul(dx, dx)), f.mul(d[3], f.mul(dx, dy))),
                f.mul(d[4], f.mul(dy, dy)),
            );
            if g2 != 0 {
                o = Some((x, y));
                break;
            }
        }
        let Some(o) = o else { continue };
        let n1 = rational_point_count(f, &fc, &mut rng);
        let curve = Curve::new(f, fc, o);
        let mut ctx = Ctx::new(curve, seed ^ (tries as u64 * 7919));
        // pool of random classes with their negatives, for the degenerate-case fallback
        let mut ok = true;
        for _ in 0..4 {
            let c = random_class(&mut ctx, &fc);
            let Ok(nc) = ctx.neg(&c) else {
                ok = false;
                break;
            };
            ctx.pool.push(c);
            ctx.pool.push(nc);
        }
        if !ok {
            continue;
        }
        let Some(n) = find_order(&mut ctx, &fc, n1) else {
            if verbose {
                eprintln!("order search failed (try {tries})");
            }
            continue;
        };
        let fac = factor(n);
        // largest prime factor with exponent 1 and small cofactor
        let mut best = None;
        for &(p, e) in &fac {
            if e == 1 && n / p <= max_cof {
                best = Some(p);
            }
        }
        let Some(ell) = best else {
            if verbose {
                eprintln!("curve try {tries}: N = {n} = {fac:?}: no suitable prime");
            }
            continue;
        };
        let cof = n / ell;
        let x = random_class(&mut ctx, &fc);
        let Some(p) = ctx.smul(cof, &x) else { continue };
        if p == ctx.id() {
            continue;
        }
        // check order: ell * P = 0
        if ctx.smul(ell, &p) != Some(ctx.id()) {
            if verbose {
                eprintln!("order check failed");
            }
            continue;
        }
        // P must be usable: a few small multiples of P and of Q = kP must be computable
        let mut kq = None;
        for _ in 0..8 {
            let k = 1 + rng.below(ell - 1);
            if let Some(qq) = ctx.smul(k, &p) {
                let usable = [3u64, 5, 7, 11, 13, 17]
                    .iter()
                    .all(|&t| ctx.smul(t, &p).is_some() && ctx.smul(t * 1009, &qq).is_some());
                if usable {
                    kq = Some((k, qq));
                    break;
                }
            }
        }
        let Some((k, qq)) = kq else { continue };
        return Some(Instance {
            ctx,
            q,
            n_group: n,
            ell,
            cof,
            p,
            qq,
            k,
            n1,
            curve_tries: tries,
            extra: Vec::new(),
        });
    }
}

// -------------------------------------------------------------------- rho

#[derive(Debug, Default, Clone)]
struct ArmResult {
    found: bool,
    verified: bool,
    cnt: Cnt,
    adds: u64,
    steps: u64,
    restarts: u64,
    extra: String,
    rel: Cnt,
    tgt: Cnt,
    lin: Cnt,
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn invmod(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1) = (m as i128, a as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let qt = r0 / r1;
        let r2 = r0 - qt * r1;
        r0 = r1;
        r1 = r2;
        let t2 = t0 - qt * t1;
        t0 = t1;
        t1 = t2;
    }
    t0.rem_euclid(m as i128) as u64
}

fn run_rho(inst: &mut Instance, neg: bool, seed: u64) -> ArmResult {
    let ell = inst.ell;
    let p = inst.p.clone();
    let qq = inst.qq.clone();
    let mut rng = Rng::new(seed);
    let c0 = snap();
    let ops0 = inst.ctx.ops;
    let r = 32usize;
    // multipliers T_j = m_j P + n_j Q with 16-bit exponents (setup counted)
    let mut tj: Vec<(Div, u64, u64)> = Vec::new();
    let mut guard = 0;
    while tj.len() < r {
        guard += 1;
        assert!(guard < 4000, "rho: multipliers unavailable");
        let m = 1 + rng.below(1 << 16);
        let n = 1 + rng.below(1 << 16);
        let (Some(a), Some(b)) = (inst.ctx.smul(m, &p), inst.ctx.smul(n, &qq)) else {
            continue;
        };
        if let Some(t) = inst.ctx.add_r(&a, &b) {
            tj.push((t, m, n));
        }
    }
    let theta_bits = 5u32; // distinguished with probability 2^-5
    let mask = (1u64 << theta_bits) - 1;
    let mut table: HashMap<[u64; 6], (u64, u64)> = HashMap::new();
    let max_steps = 60 * ((ell as f64).sqrt() as u64 + 100) + 100_000;
    let new_start = |inst: &mut Instance, rng: &mut Rng| -> (Div, u64, u64) {
        for _ in 0..2000 {
            let m = 1 + rng.below(ell - 1);
            let n = 1 + rng.below(ell - 1);
            let (Some(a), Some(b)) = (inst.ctx.smul(m, &p), inst.ctx.smul(n, &qq)) else {
                continue;
            };
            if let Some(w) = inst.ctx.add_r(&a, &b) {
                return (w, m, n);
            }
        }
        panic!("rho: no usable start point after 2000 attempts");
    };
    let (mut w, mut u, mut v) = new_start(inst, &mut rng);
    let mut steps = 0u64;
    let mut walk_len = 0u64;
    let mut restarts = 0u64;
    let mut dead = 0u64;
    let mut cycles_escaped = 0u64;
    let mut hist: [Option<[u64; 6]>; 2] = [None, None];
    let mut found = None;
    while steps < max_steps {
        if neg {
            match inst.ctx.neg_r(&w) {
                Some(nw) => {
                    cn(1);
                    if nw.key() < w.key() {
                        w = nw;
                        u = (ell - u) % ell;
                        v = (ell - v) % ell;
                    }
                }
                None => {
                    let (nw, nu, nv) = new_start(inst, &mut rng);
                    w = nw;
                    u = nu;
                    v = nv;
                    dead += 1;
                    continue;
                }
            }
        }
        if neg {
            // two-cycle escape for the walk on classes modulo sign: the state before the previous
            // one equals the current one; jump with a random multiplier
            let kcur = w.key();
            if hist[0] == Some(kcur) {
                let j = rng.below(r as u64) as usize;
                if let Some(nw) = inst.ctx.add_r(&w, &tj[j].0) {
                    w = nw;
                    u = (u + tj[j].1) % ell;
                    v = (v + tj[j].2) % ell;
                    cycles_escaped += 1;
                    hist = [None, None];
                    continue;
                }
            }
            hist[0] = hist[1];
            hist[1] = Some(kcur);
        }
        let h = mix(w.key()[0] ^ mix(w.key()[1] ^ mix(w.key()[2])));
        cn(2);
        if (h >> 20) & mask == 0 && walk_len > 0 {
            let key = w.key();
            cn(1);
            if let Some(&(u2, v2)) = table.get(&key) {
                if (u2, v2) != (u, v) && v2 != v {
                    let num = (u + ell - u2) % ell;
                    let den = (v2 + ell - v) % ell;
                    if den != 0 {
                        found = Some(mulmod(num, invmod(den, ell), ell));
                        break;
                    }
                }
            } else {
                table.insert(key, (u, v));
            }
            // continue from this distinguished point plus a random multiplier
            let j = rng.below(r as u64) as usize;
            match inst.ctx.add_r(&w, &tj[j].0) {
                Some(nw) => {
                    w = nw;
                    u = (u + tj[j].1) % ell;
                    v = (v + tj[j].2) % ell;
                }
                None => {
                    let (nw, nu, nv) = new_start(inst, &mut rng);
                    w = nw;
                    u = nu;
                    v = nv;
                    dead += 1;
                }
            }
            restarts += 1;
            walk_len = 0;
            steps += 1;
            continue;
        }
        let j = (h % r as u64) as usize;
        let (ju, next) = match inst.ctx.add(&w, &tj[j].0) {
            Ok(x) => (j, Some(x)),
            Err(_) => {
                inst.ctx.fails += 1;
                let j2 = (j + 1) % r;
                (j2, inst.ctx.add_r(&w, &tj[j2].0))
            }
        };
        match next {
            Some(nw) => {
                u = (u + tj[ju].1) % ell;
                v = (v + tj[ju].2) % ell;
                w = nw;
            }
            None => {
                let (nw, nu, nv) = new_start(inst, &mut rng);
                w = nw;
                u = nu;
                v = nv;
                dead += 1;
                walk_len = 0;
                restarts += 1;
            }
        }
        steps += 1;
        walk_len += 1;
        // restart on a suspected short cycle: far longer than the distinguished spacing
        if walk_len > 40 * (1u64 << theta_bits) {
            let (nw, nu, nv) = new_start(inst, &mut rng);
            w = nw;
            u = nu;
            v = nv;
            walk_len = 0;
            restarts += 1;
        }
    }
    let cnt = diff(snap(), c0);
    let mut res = ArmResult {
        found: found.is_some(),
        cnt,
        adds: inst.ctx.ops - ops0,
        steps,
        restarts,
        extra: format!("dead={dead};cycle_escapes={cycles_escaped}"),
        ..Default::default()
    };
    if let Some(k) = found {
        res.verified = inst.ctx.smul(k, &p) == Some(qq);
    }
    res
}

// --------------------------------------------------------- index calculus

type Form = Vec<(u32, i32)>;

fn form_comb(a: &Form, sa: i32, b: &Form, sb: i32) -> Form {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if j >= b.len() || (i < a.len() && a[i].0 < b[j].0) {
            out.push((a[i].0, sa * a[i].1));
            i += 1;
        } else if i >= a.len() || b[j].0 < a[i].0 {
            out.push((b[j].0, sb * b[j].1));
            j += 1;
        } else {
            let c = sa * a[i].1 + sb * b[j].1;
            if c != 0 {
                out.push((a[i].0, c));
            }
            i += 1;
            j += 1;
        }
    }
    cn(out.len() as u64);
    out
}

fn uf_find(p: &mut [u32], mut x: u32) -> u32 {
    while p[x as usize] != x {
        p[x as usize] = p[p[x as usize] as usize];
        x = p[x as usize];
        cn(1);
    }
    x
}

fn nullspace(mut rows: Vec<Vec<u64>>, ncols: usize, ell: u64) -> Result<Vec<u64>, Vec<usize>> {
    let n = rows.len();
    let mut pivcols = Vec::new();
    let mut r = 0;
    let mut free = Vec::new();
    for col in 0..ncols {
        if r >= n {
            free.push(col);
            continue;
        }
        let Some(p) = (r..n).find(|&i| rows[i][col] != 0) else {
            free.push(col);
            continue;
        };
        rows.swap(p, r);
        let inv = invmod(rows[r][col], ell);
        for c in col..ncols {
            rows[r][c] = mulmod(rows[r][c], inv, ell);
        }
        cl((ncols - col) as u64);
        let pr = rows[r].clone();
        for i in 0..n {
            if i != r && rows[i][col] != 0 {
                let fac = rows[i][col];
                for c in col..ncols {
                    let t = mulmod(fac, pr[c], ell);
                    rows[i][c] = (rows[i][c] + ell - t) % ell;
                }
                cl((ncols - col) as u64);
            }
        }
        pivcols.push(col);
        r += 1;
    }
    if free.len() != 1 {
        return Err(free);
    }
    let fc = free[0];
    let mut sol = vec![0u64; ncols];
    sol[fc] = 1;
    for (ri, &pc) in pivcols.iter().enumerate() {
        sol[pc] = (ell - rows[ri][fc]) % ell;
    }
    Ok(sol)
}

fn run_ic(inst: &mut Instance, bmult100: u64, seed: u64) -> ArmResult {
    let f = inst.ctx.c.fq;
    let q = f.0;
    let ell = inst.ell;
    let fc = inst.ctx.c.f;
    let o = inst.ctx.c.o;
    let okey = o.0 * q + o.1;
    let mut rng = Rng::new(seed);
    let ops0 = inst.ctx.ops;
    let c_start = snap();
    // square-root table (q multiplications)
    const NONE: u32 = u32::MAX;
    let mut sqt = vec![NONE; q as usize];
    for x in 0..q {
        sqt[f.mul(x, x) as usize] = x as u32;
    }
    // factor base
    let b = ((bmult100 as f64 / 100.0) * (q as f64).sqrt()).ceil() as usize;
    let mut fbmap: HashMap<u64, u32> = HashMap::new();
    let mut fbp: Vec<(u64, u64, [u64; 5])> = Vec::new();
    while fbp.len() < b {
        let (x, y) = random_point(f, &fc, &mut rng);
        let key = x * q + y;
        if key == okey || fbmap.contains_key(&key) {
            continue;
        }
        fbmap.insert(key, fbp.len() as u32);
        fbp.push((x, y, derivs(f, &fc, x, y)));
    }
    // the base point O joins the pairs with the pinned value 0 (it removes the trivial solution
    // x_P = c, x_h = 4c of the homogeneous system)
    fbp.push((o.0, o.1, derivs(f, &fc, o.0, o.1)));
    let mpairs = ((b + 1) * b / 2) as u64;
    let mut order: Vec<u32> = (0..mpairs as u32).collect();
    for k in (1..order.len()).rev() {
        let r = rng.below(k as u64 + 1) as usize;
        order.swap(k, r);
    }
    cn(mpairs);
    // graph
    let mut vmap: HashMap<u64, u32> = HashMap::new();
    let mut vkeys: Vec<u64> = Vec::new();
    let mut edges: Vec<(i32, i32, u32, u32)> = Vec::new();
    let mut uf: Vec<u32> = vec![0];
    let mut cedges: Vec<u64> = vec![0];
    let mut csize: Vec<u64> = vec![1];
    let target_cycles = (2 * b + 30) as i64;
    let mut pairs_done = 0u64;
    let mut cycles_ground = 0i64;
    let h2 = f.inv(2);
    let _ = h2;
    let mut t = 0u64;
    while t < mpairs && cycles_ground < target_cycles {
        let pidx = order[t as usize] as u64;
        t += 1;
        let i = ((1.0 + (1.0 + 8.0 * pidx as f64).sqrt()) / 2.0) as u64;
        let mut i = i;
        while i * (i - 1) / 2 > pidx {
            i -= 1;
        }
        while (i + 1) * i / 2 <= pidx {
            i += 1;
        }
        let j = pidx - i * (i - 1) / 2;
        let (i, j) = (j as usize, i as usize); // i < j
        pairs_done += 1;
        let (x1, y1, d1) = (fbp[i].0, fbp[i].1, fbp[i].2);
        let (x2, y2, _) = (fbp[j].0, fbp[j].1, 0);
        let dx = f.sub(x2, x1);
        let dy = f.sub(y2, y1);
        let g1 = f.add(f.mul(d1[0], dx), f.mul(d1[1], dy));
        let dx2 = f.mul(dx, dx);
        let dxy = f.mul(dx, dy);
        let dy2 = f.mul(dy, dy);
        let g2 = f.add(
            f.add(f.mul(d1[2], dx2), f.mul(d1[3], dxy)),
            f.mul(d1[4], dy2),
        );
        // top form at d
        let mut g4 = 0;
        {
            let mut dxp = [1u64; 5];
            let mut dyp = [1u64; 5];
            for e in 1..5 {
                dxp[e] = f.mul(dxp[e - 1], dx);
                dyp[e] = f.mul(dyp[e - 1], dy);
            }
            for ii in 0..5 {
                let c = fc[ii][4 - ii];
                if c != 0 {
                    g4 = f.add(g4, f.mul(c, f.mul(dxp[ii], dyp[4 - ii])));
                }
            }
        }
        let a0 = f.neg(g1);
        let a1 = f.sub(a0, g2);
        let a2 = g4;
        if a2 == 0 {
            continue;
        }
        let disc = f.sub(f.mul(a1, a1), f.mul(4, f.mul(a0, a2)));
        cn(1);
        let sq = sqt[disc as usize];
        if sq == NONE || disc == 0 {
            continue;
        }
        let ia = f.inv(f.add(a2, a2));
        let ua = f.mul(f.sub(f.neg(a1), sq as u64), ia);
        let ub = f.mul(f.add(f.neg(a1), sq as u64), ia);
        if ua == 0 || ua == 1 || ub == 0 || ub == 1 {
            continue;
        }
        let mut ends = [0i32; 2];
        for (e, u) in [ua, ub].iter().enumerate() {
            let x = f.add(x1, f.mul(*u, dx));
            let y = f.add(y1, f.mul(*u, dy));
            let key = x * q + y;
            cn(1);
            ends[e] = if key == okey {
                -1
            } else if let Some(&g) = fbmap.get(&key) {
                -(2 + g as i32)
            } else if let Some(&v) = vmap.get(&key) {
                v as i32
            } else {
                let v = vkeys.len() as u32;
                vmap.insert(key, v);
                vkeys.push(key);
                uf.push(uf.len() as u32);
                cedges.push(0);
                csize.push(1);
                v as i32
            };
        }
        edges.push((ends[0], ends[1], i as u32, j as u32));
        let node = |e: i32| -> u32 {
            if e >= 0 {
                e as u32 + 1
            } else {
                0
            }
        };
        let (n1, n2) = (node(ends[0]), node(ends[1]));
        let (r1, r2) = (uf_find(&mut uf, n1), uf_find(&mut uf, n2));
        if r1 == r2 {
            cedges[r1 as usize] += 1;
        } else {
            uf[r2 as usize] = r1;
            cedges[r1 as usize] += cedges[r2 as usize] + 1;
            csize[r1 as usize] += csize[r2 as usize];
        }
        if pairs_done % 32 == 0 || t == mpairs {
            let rg = uf_find(&mut uf, 0);
            cycles_ground = cedges[rg as usize] as i64 - (csize[rg as usize] as i64 - 1);
        }
    }
    {
        let rg = uf_find(&mut uf, 0);
        cycles_ground = cedges[rg as usize] as i64 - (csize[rg as usize] as i64 - 1);
    }
    let c_rel = diff(snap(), c_start);
    let mut res = ArmResult {
        steps: pairs_done,
        rel: c_rel,
        ..Default::default()
    };
    if cycles_ground < (b + 1) as i64 {
        res.extra = format!("fail=cycles({cycles_ground})");
        res.cnt = diff(snap(), c_start);
        return res;
    }
    // ---- linear algebra stage: tree potentials and relation rows
    let c_la0 = snap();
    let nv = vkeys.len();
    let lform = |i: u32, j: u32| -> Form {
        let mut v: Form = Vec::new();
        if (i as usize) < b {
            v.push((i, -1));
        }
        if (j as usize) < b {
            v.push((j, -1));
        }
        v.push((b as u32, 1));
        v
    };
    let phi = |e: i32| -> Form {
        if e == -1 {
            vec![]
        } else {
            vec![((-(e + 2)) as u32, 1)]
        }
    };
    let mut inc: Vec<Vec<u32>> = vec![vec![]; nv];
    for (ei, e) in edges.iter().enumerate() {
        if e.0 >= 0 {
            inc[e.0 as usize].push(ei as u32);
        }
        if e.1 >= 0 {
            inc[e.1 as usize].push(ei as u32);
        }
    }
    let mut pot: Vec<Option<Form>> = vec![None; nv];
    let mut tree = vec![false; edges.len()];
    let mut queue: Vec<u32> = Vec::new();
    for (ei, e) in edges.iter().enumerate() {
        let (a, bb) = (e.0, e.1);
        if (a >= 0) != (bb >= 0) {
            let (v, w) = if a >= 0 { (a, bb) } else { (bb, a) };
            if pot[v as usize].is_none() {
                let l = lform(e.2, e.3);
                pot[v as usize] = Some(form_comb(&l, 1, &phi(w), -1));
                tree[ei] = true;
                queue.push(v as u32);
            }
        }
    }
    let mut qi = 0;
    while qi < queue.len() {
        let v = queue[qi] as usize;
        qi += 1;
        for &ei in &inc[v] {
            let e = edges[ei as usize];
            if e.0 < 0 || e.1 < 0 {
                continue;
            }
            let u = if e.0 as usize == v { e.1 } else { e.0 } as usize;
            if pot[u].is_none() {
                let l = lform(e.2, e.3);
                let pv = pot[v].clone().unwrap();
                pot[u] = Some(form_comb(&l, 1, &pv, -1));
                tree[ei as usize] = true;
                queue.push(u as u32);
            }
        }
    }
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for (ei, e) in edges.iter().enumerate() {
        if tree[ei] {
            continue;
        }
        let fa = if e.0 >= 0 {
            match &pot[e.0 as usize] {
                Some(p) => p.clone(),
                None => continue,
            }
        } else {
            phi(e.0)
        };
        let fb = if e.1 >= 0 {
            match &pot[e.1 as usize] {
                Some(p) => p.clone(),
                None => continue,
            }
        } else {
            phi(e.1)
        };
        let sum = form_comb(&fa, 1, &fb, 1);
        let row = form_comb(&sum, 1, &lform(e.2, e.3), -1);
        let mut dense = vec![0u64; b + 1];
        for &(var, c) in &row {
            dense[var as usize] = (c as i64).rem_euclid(ell as i64) as u64;
        }
        rows.push(dense);
    }
    let nrows = rows.len();
    let mut colcount = vec![0u32; b + 1];
    for r in &rows {
        for (c, v) in r.iter().enumerate() {
            if *v != 0 {
                colcount[c] += 1;
            }
        }
    }
    let used_cols = colcount.iter().filter(|&&c| c > 0).count();
    let mut ep = vec![0u32; b + 1];
    for e in &edges {
        ep[e.2 as usize] += 1;
        ep[e.3 as usize] += 1;
    }
    let edge_cols = ep.iter().filter(|&&c| c > 0).count();
    let sol = nullspace(rows, b + 1, ell);
    let lin = diff(snap(), c_la0);
    res.lin = lin;
    let sol = match sol {
        Ok(v) => v,
        Err(free) => {
            res.extra = format!("fail=nullspace(rows={nrows},B={b},used_cols={used_cols},edge_cols={edge_cols},free={}))", free.len());
            res.cnt = diff(snap(), c_start);
            return res;
        }
    };
    // potentials of vertices
    let mut known: HashMap<u64, u64> = HashMap::new();
    known.insert(okey, 0);
    for (key, &g) in &fbmap {
        known.insert(*key, sol[g as usize]);
    }
    for v in 0..nv {
        if let Some(p) = &pot[v] {
            let mut acc = 0u64;
            for &(var, c) in p {
                let cc = (c as i64).rem_euclid(ell as i64) as u64;
                acc = (acc + mulmod(cc, sol[var as usize], ell)) % ell;
            }
            cl(p.len() as u64);
            known.insert(vkeys[v], acc);
        }
    }
    let known_frac = known.len() as f64 / (inst.n1 as f64);
    // ---- target
    let c_t0 = snap();
    let p = inst.p.clone();
    let qd = inst.qq.clone();
    let mut rels: Vec<(u64, u64, u64)> = Vec::new();
    let mut trials = 0u64;
    let mut found = None;
    let mut outer_iters = 0u64;
    'outer: while trials < 40_000 && outer_iters < 3000 {
        outer_iters += 1;
        let ea0 = 1 + rng.below(ell - 1);
        let eb0 = 1 + rng.below(ell - 1);
        let (Some(a), Some(bq)) = (inst.ctx.smul(ea0, &p), inst.ctx.smul(eb0, &qd)) else {
            continue;
        };
        let Some(mut xi) = inst.ctx.add_r(&a, &bq) else {
            continue;
        };
        let (mut ea, mut eb) = (ea0, eb0);
        for _ in 0..4000 {
            trials += 1;
            // does the cubic split into three rational points with known logarithms?
            let ff = inst.ctx.c.fq;
            let xq = ppowmod(ff, &[0, 1], q, &xi.a);
            cn(1);
            if xq == vec![0, 1] {
                let rts = poly_roots(ff, &xi.a, &mut rng);
                if rts.len() == 3 {
                    let mut sum = 0u64;
                    let mut ok = true;
                    for &xr in &rts {
                        let yr = peval(ff, &xi.b, xr);
                        cn(1);
                        match known.get(&(xr * q + yr)) {
                            Some(&lv) => sum = (sum + lv) % ell,
                            None => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok {
                        let indep = rels.iter().all(|r| {
                            let det = (mulmod(r.0, eb, ell) + ell - mulmod(r.1, ea, ell)) % ell;
                            det != 0
                        });
                        if indep {
                            rels.push((ea, eb, sum));
                        }
                        if rels.len() == 2 {
                            let (r1, r2) = (rels[0], rels[1]);
                            let det =
                                (mulmod(r1.0, r2.1, ell) + ell - mulmod(r1.1, r2.0, ell)) % ell;
                            if det != 0 {
                                let di = invmod(det, ell);
                                let lp = mulmod(
                                    (mulmod(r1.2, r2.1, ell) + ell - mulmod(r2.2, r1.1, ell)) % ell,
                                    di,
                                    ell,
                                );
                                let lq = mulmod(
                                    (mulmod(r1.0, r2.2, ell) + ell - mulmod(r2.0, r1.2, ell)) % ell,
                                    di,
                                    ell,
                                );
                                if lp != 0 {
                                    found = Some(mulmod(lq, invmod(lp, ell), ell));
                                    break 'outer;
                                }
                            }
                            rels.clear();
                        }
                    }
                }
            }
            // next class: add P or Q
            let stepq = rng.below(2) == 1;
            let nx = if stepq {
                inst.ctx.add_r(&xi, &qd)
            } else {
                inst.ctx.add_r(&xi, &p)
            };
            match nx {
                Some(n) => {
                    xi = n;
                    if stepq {
                        eb = (eb + 1) % ell;
                    } else {
                        ea = (ea + 1) % ell;
                    }
                }
                None => break,
            }
        }
    }
    res.tgt = diff(snap(), c_t0);
    res.cnt = diff(snap(), c_start);
    res.found = found.is_some();
    res.adds = inst.ctx.ops - ops0;
    res.extra = format!("B={b};pairs={pairs_done};edges={};cycles={cycles_ground};rows={nrows};known_frac={known_frac:.3};trials={trials}", edges.len());
    if let Some(k) = found {
        res.verified = inst.ctx.smul(k, &p) == Some(qd);
    }
    res
}

pub fn legacy_main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(a.get(2).map(|v| v.parse().unwrap()).unwrap_or(127)),
        Some("probe") => {
            let (q, cseed): (u64, u64) = (a[2].parse().unwrap(), a[3].parse().unwrap());
            let mut inst = build_instance(q, cseed, 1000, false).expect("instance");
            let p = inst.p.clone();
            let qq = inst.qq.clone();
            let (mut np, mut nq) = (0, 0);
            for t in 1..200u64 {
                if inst.ctx.smul(t * 7919 % inst.ell, &p).is_none() {
                    np += 1;
                }
                if inst.ctx.smul(t * 104729 % inst.ell, &qq).is_none() {
                    nq += 1;
                }
            }
            println!(
                "ell={} cof={} P={:?} fails: P {np}/199, Q {nq}/199, ctx.fails {}",
                inst.ell, inst.cof, p, inst.ctx.fails
            );
        }
        Some("run") => cmd_run(
            &a[2],
            a[3].parse().unwrap(),
            a[4].parse().unwrap(),
            a[5].parse().unwrap(),
            a[6].parse().unwrap(),
            a.get(7).map(|s| s.as_str()).unwrap_or("ic,rho,rhoneg"),
            a.get(8).map(|v| v.parse().unwrap()).unwrap_or(64),
        ),
        Some("summarize") => {
            let qs: Vec<u64> = a[3..].iter().map(|v| v.parse().unwrap()).collect();
            cmd_summarize(&a[2], &qs)
        }
        _ => {
            eprintln!("usage: quartic_ic selftest [q] | run <out-dir> <q> <n> <seed> <B-mult-x100> [arms] [max-cofactor] | summarize <out-dir> <q>...");
            std::process::exit(2);
        }
    }
}

/// Mean M and I of a group addition on this curve (the GAE of the pricing).
fn measure_gae(ctx: &mut Ctx, fc: &[[u64; 5]; 5]) -> (f64, f64) {
    let c0 = snap();
    let mut n = 0u64;
    while n < 100 {
        let d1 = random_class(ctx, fc);
        let d2 = random_class(ctx, fc);
        let b = snap();
        if ctx.add(&d1, &d2).is_ok() {
            let d = diff(snap(), b);
            let _ = d;
            n += 1;
        }
    }
    // second pass over successful adds only
    let _ = c0;
    let mut tm = 0u64;
    let mut ti = 0u64;
    let mut cnt = 0u64;
    while cnt < 100 {
        let d1 = random_class(ctx, fc);
        let d2 = random_class(ctx, fc);
        let b = snap();
        if ctx.add(&d1, &d2).is_ok() {
            let d = diff(snap(), b);
            tm += d.m;
            ti += d.i;
            cnt += 1;
        }
    }
    (tm as f64 / cnt as f64, ti as f64 / cnt as f64)
}

fn cmd_run(out: &str, q: u64, n: u64, seed: u64, bmult: u64, arms: &str, max_cof: u64) {
    std::fs::create_dir_all(out).unwrap();
    let path = format!("{out}/rows_q{q}.csv");
    let mut csv = if std::path::Path::new(&path).exists() {
        String::new()
    } else {
        String::from("q,inst,curve_seed,N,ell,cof,k,arm,found,verified,steps,adds,M,I,NAT,LA,rel_M,rel_I,rel_NAT,lin_LA,lin_M,tgt_M,tgt_I,tgt_NAT,gae_M,gae_I,fails,extra\n")
    };
    let t0 = std::time::Instant::now();
    for inst_i in 0..n {
        let cseed = seed * 1000 + inst_i;
        let Some(mut inst) = build_instance(q, cseed, max_cof, false) else {
            eprintln!("instance {inst_i}: generation failed");
            continue;
        };
        let fc = inst.ctx.c.f;
        let (gm, gi) = measure_gae(&mut inst.ctx, &fc);
        for arm in arms.split(',') {
            let f0 = inst.ctx.fails;
            let r = match arm {
                "ic" => {
                    let mut tot = run_ic(&mut inst, bmult, cseed ^ 0x1C);
                    let mut bm = bmult;
                    // a failed attempt (too few independent relations) is repeated with a larger
                    // factor base; the costs of every attempt are summed
                    let mut attempts = 1;
                    while !tot.found && tot.extra.starts_with("fail=") && attempts < 4 {
                        bm += 100;
                        attempts += 1;
                        let nxt = run_ic(&mut inst, bm, cseed ^ 0x1C ^ attempts);
                        let prev = tot.clone();
                        tot = ArmResult {
                            rel: addc(prev.rel, nxt.rel),
                            lin: addc(prev.lin, nxt.lin),
                            tgt: addc(prev.tgt, nxt.tgt),
                            cnt: addc(prev.cnt, nxt.cnt),
                            steps: prev.steps + nxt.steps,
                            adds: prev.adds + nxt.adds,
                            ..nxt
                        };
                    }
                    tot.extra = format!("{};attempts={attempts};bmult_final={bm}", tot.extra);
                    tot
                }
                "rho" => run_rho(&mut inst, false, cseed ^ 0x2D),
                "rhoneg" => run_rho(&mut inst, true, cseed ^ 0x3E),
                _ => panic!("arm"),
            };
            writeln!(
                csv,
                "{q},{inst_i},{cseed},{},{},{},{},{arm},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{gm:.1},{gi:.1},{},{}",
                inst.n_group,
                inst.ell,
                inst.cof,
                inst.k,
                r.found as u8,
                r.verified as u8,
                r.steps,
                r.adds,
                r.cnt.m,
                r.cnt.i,
                r.cnt.nat,
                r.cnt.la,
                r.rel.m,
                r.rel.i,
                r.rel.nat,
                r.lin.la,
                r.lin.m,
                r.tgt.m,
                r.tgt.i,
                r.tgt.nat,
                inst.ctx.fails - f0,
                r.extra
            )
            .unwrap();
            eprintln!(
                "q={q} inst={inst_i} arm={arm} found={} verified={} steps={} M={} I={} nat={} la={} [{:.0}s] {}",
                r.found, r.verified, r.steps, r.cnt.m, r.cnt.i, r.cnt.nat, r.cnt.la, t0.elapsed().as_secs_f64(), r.extra
            );
        }
        // append this instance's rows at once (a crash later loses nothing already measured)
        let mut fh = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();
        std::io::Write::write_all(&mut fh, csv.as_bytes()).unwrap();
        csv.clear();
    }
}

fn lsq(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    sxy / sxx
}

fn center(v: &[f64], median: bool) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    if median {
        let mut w = v.to_vec();
        w.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = w.len();
        if n % 2 == 1 {
            w[n / 2]
        } else {
            (w[n / 2 - 1] + w[n / 2]) / 2.0
        }
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

fn cmd_summarize(out: &str, qs: &[u64]) {
    // QIC_STAT=median switches every cost centre from the mean to the median over instances
    let use_median = std::env::var("QIC_STAT")
        .map(|v| v == "median")
        .unwrap_or(false);
    let sfx = if use_median { "_median" } else { "" };
    // rows: per (q, arm): list of (cost cells...)
    struct Row {
        q: u64,
        arm: String,
        verified: bool,
        steps: f64,
        ell: f64,
        m: f64,
        i: f64,
        nat: f64,
        la: f64,
        rel_m: f64,
        rel_i: f64,
        rel_nat: f64,
        gm: f64,
        gi: f64,
        tgt_m: f64,
        tgt_i: f64,
        lin_la: f64,
        adds: f64,
    }
    let mut rows: Vec<Row> = Vec::new();
    for &q in qs {
        let txt = std::fs::read_to_string(format!("{out}/rows_q{q}.csv")).expect("rows csv");
        for line in txt.lines().skip(1) {
            let f: Vec<&str> = line.split(',').collect();
            let g = |i: usize| -> f64 { f[i].parse().unwrap_or(0.0) };
            rows.push(Row {
                q,
                arm: f[7].to_string(),
                verified: f[9] == "1",
                steps: g(10),
                adds: g(11),
                ell: g(4),
                m: g(12),
                i: g(13),
                nat: g(14),
                la: g(15),
                rel_m: g(16),
                rel_i: g(17),
                rel_nat: g(18),
                lin_la: g(19),
                tgt_m: g(21),
                tgt_i: g(22),
                gm: g(24),
                gi: g(25),
            });
        }
    }
    let mut s = String::from("q,arm,n_total,n_verified,mean_steps,mean_steps_over_sqrt_ell,mean_adds,mean_M,mean_I,mean_NAT,mean_LA\n");
    let mut cells = String::from("q,arm,w,cI_rule,mean_cost_GAE,n\n");
    let mut fits = String::from("arm,quantity,slope,lo95,hi95,n_q\n");
    let mut per_arm_q: HashMap<(String, u64, usize), Vec<f64>> = HashMap::new();
    let arms = ["ic", "rho", "rhoneg"];
    for &q in qs {
        let ci_rules = [3.0, 2.0 * (q as f64).log2()];
        for arm in arms {
            let sel: Vec<&Row> = rows.iter().filter(|r| r.q == q && r.arm == arm).collect();
            if sel.is_empty() {
                continue;
            }
            let ver: Vec<&&Row> = sel.iter().filter(|r| r.verified).collect();
            let n = ver.len().max(1) as f64;
            let mean = |f: &dyn Fn(&Row) -> f64| ver.iter().map(|r| f(r)).sum::<f64>() / n;
            writeln!(
                s,
                "{q},{arm},{},{},{:.1},{:.3},{:.1},{:.4e},{:.4e},{:.4e},{:.4e}",
                sel.len(),
                ver.len(),
                mean(&|r| r.steps),
                mean(&|r| r.steps / r.ell.sqrt()),
                mean(&|r| r.adds),
                mean(&|r| r.m),
                mean(&|r| r.i),
                mean(&|r| r.nat),
                mean(&|r| r.la)
            )
            .unwrap();
            for (ri, &cirule) in ci_rules.iter().enumerate() {
                for (wi, w) in [0.0, 0.1, 1.0].iter().enumerate() {
                    let costs: Vec<f64> = ver
                        .iter()
                        .map(|r| {
                            let gae = r.gm + cirule * r.gi;
                            (r.m + cirule * r.i) / gae + w * (r.nat + r.la)
                        })
                        .collect();
                    let mc = center(&costs, use_median);
                    writeln!(
                        cells,
                        "{q},{arm},{w},{},{mc:.2},{}",
                        if ri == 0 { "3" } else { "2log2q" },
                        costs.len()
                    )
                    .unwrap();
                    per_arm_q.insert((arm.to_string(), q, ri * 3 + wi), costs);
                }
            }
            // stage diagnostic for IC: relation phase only
            if arm == "ic" {
                let costs: Vec<f64> = ver
                    .iter()
                    .map(|r| {
                        let gae = r.gm + 3.0 * r.gi;
                        (r.rel_m + 3.0 * r.rel_i) / gae
                    })
                    .collect();
                per_arm_q.insert(("ic_rel_stage".to_string(), q, 0), costs);
            }
        }
    }
    // fits: log-log slope over q with bootstrap over instances
    let mut rng = Rng::new(5);
    let mut fit =
        |key: &dyn Fn(u64) -> (String, usize), name: &str, label: &str, fits: &mut String| {
            let series: Vec<(f64, &Vec<f64>)> = qs
                .iter()
                .filter_map(|&q| {
                    let (a, c) = key(q);
                    per_arm_q
                        .get(&(a, q, c))
                        .filter(|v| !v.is_empty())
                        .map(|v| ((q as f64).ln(), v))
                })
                .collect();
            if series.len() < 2 {
                return;
            }
            let xs: Vec<f64> = series.iter().map(|x| x.0).collect();
            let ys: Vec<f64> = series
                .iter()
                .map(|x| center(x.1, use_median).ln())
                .collect();
            let sl = lsq(&xs, &ys);
            let mut bs = Vec::new();
            for _ in 0..2000 {
                let ys2: Vec<f64> = series
                    .iter()
                    .map(|x| {
                        let n = x.1.len();
                        let draw: Vec<f64> =
                            (0..n).map(|_| x.1[rng.below(n as u64) as usize]).collect();
                        center(&draw, use_median).ln()
                    })
                    .collect();
                bs.push(lsq(&xs, &ys2));
            }
            bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            writeln!(
                fits,
                "{name},{label},{sl:.3},{:.3},{:.3},{}",
                bs[50],
                bs[1949],
                series.len()
            )
            .unwrap();
        };
    for (ri, rname) in ["cI=3", "cI=2log2q"].iter().enumerate() {
        for (wi, w) in ["0", "0.1", "1"].iter().enumerate() {
            let lab = format!("cost_GAE w={w} {rname}");
            for arm in arms {
                fit(&|_q| (arm.to_string(), ri * 3 + wi), arm, &lab, &mut fits);
            }
        }
    }
    fit(
        &|_q| ("ic_rel_stage".to_string(), 0),
        "ic_relation_stage_only (stage diagnostic)",
        "cost_GAE w=0 cI=3",
        &mut fits,
    );
    // rho steps
    for arm in ["rho", "rhoneg"] {
        let series: Vec<(f64, Vec<f64>)> = qs
            .iter()
            .map(|&q| {
                (
                    (q as f64).ln(),
                    rows.iter()
                        .filter(|r| r.q == q && r.arm == arm && r.verified)
                        .map(|r| r.steps)
                        .collect::<Vec<f64>>(),
                )
            })
            .filter(|x| !x.1.is_empty())
            .collect();
        if series.len() >= 2 {
            let xs: Vec<f64> = series.iter().map(|x| x.0).collect();
            let ys: Vec<f64> = series
                .iter()
                .map(|x| (x.1.iter().sum::<f64>() / x.1.len() as f64).ln())
                .collect();
            writeln!(fits, "{arm},steps,{:.3},,,{}", lsq(&xs, &ys), series.len()).unwrap();
        }
    }
    // ranking per cell and q
    let mut rank = String::from("q,cell,ic_cost,rho_cost,ic_over_rho\n");
    for &q in qs {
        for c in 0..6 {
            if let (Some(a), Some(b)) = (
                per_arm_q.get(&("ic".to_string(), q, c)),
                per_arm_q.get(&("rho".to_string(), q, c)),
            ) {
                if a.is_empty() || b.is_empty() {
                    continue;
                }
                let ma = center(a, use_median);
                let mb = center(b, use_median);
                writeln!(
                    rank,
                    "{q},{},{ma:.1},{mb:.1},{:.3}",
                    [
                        "w0_cI3",
                        "w0.1_cI3",
                        "w1_cI3",
                        "w0_cI2log",
                        "w0.1_cI2log",
                        "w1_cI2log"
                    ][c],
                    ma / mb
                )
                .unwrap();
            }
        }
    }
    std::fs::write(format!("{out}/summary_arms{sfx}.csv"), &s).unwrap();
    std::fs::write(format!("{out}/summary_cells{sfx}.csv"), &cells).unwrap();
    std::fs::write(format!("{out}/summary_fits{sfx}.csv"), &fits).unwrap();
    std::fs::write(format!("{out}/summary_rank{sfx}.csv"), &rank).unwrap();
    print!("{s}\n{fits}\n{rank}");
}

fn selftest(q: u64) {
    let inst = build_instance(q, 12345, 100_000, true).expect("instance");
    println!(
        "q={q} N1={} N={} ell={} cof={} k={} curve tries {} fails {}",
        inst.n1, inst.n_group, inst.ell, inst.cof, inst.k, inst.curve_tries, inst.ctx.fails
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_ctx(q: u64, seed: u64) -> (Ctx, [[u64; 5]; 5]) {
        let f = Fq(q);
        let mut rng = Rng::new(seed);
        let fc = loop {
            let fc = rand_quartic(f, &mut rng);
            if is_smooth(f, &fc) {
                break fc;
            }
        };
        let o = loop {
            let (x, y) = random_point(f, &fc, &mut rng);
            let d = derivs(f, &fc, x, y);
            if d[1] != 0 {
                let dx = d[1];
                let dy = f.neg(d[0]);
                let g2 = f.add(
                    f.add(f.mul(d[2], f.mul(dx, dx)), f.mul(d[3], f.mul(dx, dy))),
                    f.mul(d[4], f.mul(dy, dy)),
                );
                if g2 != 0 {
                    break (x, y);
                }
            }
        };
        let mut ctx = Ctx::new(Curve::new(f, fc, o), seed);
        for _ in 0..4 {
            let c = random_class(&mut ctx, &fc);
            let nc = ctx.neg(&c).unwrap();
            ctx.pool.push(c);
            ctx.pool.push(nc);
        }
        (ctx, fc)
    }

    #[test]
    fn poly_roots_and_inverse() {
        let f = Fq(251);
        let mut rng = Rng::new(3);
        // (x - 3)(x - 100)(x^2 + 1 irreducible mod 251 since 251 = 3 mod 4)
        let p = pmul(f, &pmul(f, &[f.neg(3), 1], &[f.neg(100), 1]), &[1, 0, 1]);
        let mut r = poly_roots(f, &p, &mut rng);
        r.sort();
        assert_eq!(r, vec![3, 100]);
        let m = vec![5, 0, 0, 1];
        let a = vec![7, 2];
        let inv = pinv(f, &a, &m).unwrap();
        assert_eq!(pmulmod(f, &a, &inv, &m), vec![1]);
    }

    #[test]
    fn group_law_axioms() {
        let (mut ctx, fc) = small_ctx(251, 11);
        let id = ctx.id();
        let mut checked = 0;
        for _ in 0..25 {
            let a = random_class(&mut ctx, &fc);
            let b = random_class(&mut ctx, &fc);
            let c = random_class(&mut ctx, &fc);
            let (Some(ab), Some(ba)) = (ctx.add_r(&a, &b), ctx.add_r(&b, &a)) else {
                continue;
            };
            assert_eq!(ab, ba, "commutativity");
            let (Some(ab_c), Some(bc)) = (ctx.add_r(&ab, &c), ctx.add_r(&b, &c)) else {
                continue;
            };
            let Some(a_bc) = ctx.add_r(&a, &bc) else {
                continue;
            };
            assert_eq!(ab_c, a_bc, "associativity");
            let Some(back) = ctx.sub_r(&ab, &b) else {
                continue;
            };
            assert_eq!(back, a, "(a+b)-b = a");
            let Some(na) = ctx.neg_r(&a) else { continue };
            assert_eq!(ctx.add_r(&a, &na), Some(id.clone()), "a + (-a) = 0");
            let Some(d2) = ctx.dbl_r(&a) else { continue };
            assert_eq!(
                Some(d2),
                ctx.add_r(&a, &a).or(None).or_else(|| ctx.dbl_r(&a))
            );
            checked += 1;
        }
        assert!(checked >= 12, "{checked}");
    }

    #[test]
    fn order_annihilates_random_classes() {
        let inst = build_instance(251, 777, 100_000, false).expect("instance");
        let mut ctx = inst.ctx;
        let fc = ctx.c.f;
        let mut n = 0;
        for _ in 0..6 {
            let x = random_class(&mut ctx, &fc);
            if let Some(r) = ctx.smul(inst.n_group, &x) {
                assert_eq!(r, ctx.id());
                n += 1;
            }
        }
        assert!(n >= 4);
        // the planted answer is consistent
        assert_eq!(ctx.smul(inst.k, &inst.p), Some(inst.qq.clone()));
    }

    #[test]
    fn nullspace_small() {
        let ell = 101;
        // x, y, z with 2x = y and 3y = 2z
        let rows = vec![vec![2, 100, 0], vec![0, 3, 99]];
        let sol = nullspace(rows, 3, ell).unwrap();
        assert_eq!((2 * sol[0] + 100 * sol[1]) % ell, 0);
        assert_eq!((3 * sol[1] + 99 * sol[2]) % ell, 0);
        assert!(sol.iter().any(|&v| v != 0));
    }

    #[test]
    fn small_instance_both_arms() {
        let mut inst = build_instance(251, 31337, 100_000, false).expect("instance");
        let r = run_rho(&mut inst, false, 5);
        assert!(r.found && r.verified);
        let mut ok = false;
        for bm in [300, 400, 500] {
            let r = run_ic(&mut inst, bm, 6);
            if r.found {
                assert!(r.verified);
                ok = true;
                break;
            }
        }
        assert!(ok);
    }
}

// =====================================================================================
// quartic-diem-sparse (2026-11-15): Diem index calculus on a generic plane quartic with a
// factor base of about c * sqrt(q) points, double large primes kept as sparse columns
// (rows of at most four points plus the hyperplane unknown), large-prime columns removed by
// minimum-weight elimination (`dq::eliminate_lp`) and the factor-base system solved by
// Wiedemann (`dq::wiedemann_solve`). The dense Gauss-Jordan of the base tool runs on the same
// rows as an A/B arm. Everything below is new code of this study.
// =====================================================================================

use crate::dq::{self, Mont, SRow};
use std::time::Instant;

/// Counter triple used for phase accounting (the LA operations are tallied by the caller).
#[derive(Clone, Copy, Default, Debug)]
pub struct Tally {
    pub m: u64,
    pub i: u64,
    pub nat: u64,
}

fn tally_diff(a: Cnt, b: Cnt) -> Tally {
    Tally {
        m: a.m - b.m,
        i: a.i - b.i,
        nat: a.nat - b.nat,
    }
}

impl Tally {
    fn add(&mut self, o: Tally) {
        self.m += o.m;
        self.i += o.i;
        self.nat += o.nat;
    }
}

/// Everything recorded about one sparse-IC unit.
#[derive(Clone, Debug, Default)]
pub struct SparseRes {
    pub status: String,
    pub found: bool,
    pub verified: bool,
    pub b: usize,
    pub pairs: u64,
    pub edges: usize,
    pub kept: usize,
    pub cycles: i64,
    pub rounds: u32,
    pub w_raw_mean: f64,
    pub w_raw_max: usize,
    pub rows_fb: usize,
    pub w_fb_mean: f64,
    pub nnz_fb: usize,
    pub rel: Tally,
    pub setup: Tally,
    pub tgt: Tally,
    pub total: Tally,
    pub elim_ops: u64,
    pub elim_entries: u64,
    pub wied_ops: u64,
    pub bm_ops: u64,
    pub matvecs: u64,
    pub deg: usize,
    pub dense_la: u64,
    pub dense_ok: bool,
    pub agree: Option<bool>,
    pub tgt_adds: u64,
    pub known_frac: f64,
    pub wall_rel: f64,
    pub wall_elim: f64,
    pub wall_wied: f64,
    pub wall_dense: f64,
    pub wall_tgt: f64,
    /// Log found for the primary target (round 2).
    pub found_k: Option<u64>,
    /// Extra targets (round 2): (log found, tally, group additions, verified on C).
    pub extra_tgts: Vec<(Option<u64>, Tally, u64, bool)>,
}

/// Collection state: factor base, shuffled pair order, large-prime graph.
struct Coll {
    f: Fq,
    fc: [[u64; 5]; 5],
    q: u64,
    okey: u64,
    b: usize,
    fbp: Vec<(u64, u64, [u64; 5])>,
    fbmap: HashMap<u64, u32>,
    sqt: Vec<u32>,
    order: Vec<u32>,
    t: u64,
    mpairs: u64,
    vmap: HashMap<u64, u32>,
    vkeys: Vec<u64>,
    edges: Vec<(i32, i32, u32, u32)>,
    uf: Vec<u32>,
    cedges: Vec<u64>,
    csize: Vec<u64>,
    pairs_done: u64,
    cycles_ground: i64,
}

const NONE32: u32 = u32::MAX;

/// Decode a pair index into `(i, j)` with `i < j` (the enumeration of the base tool).
fn pair_decode(pidx: u64) -> (usize, usize) {
    let mut i = ((1.0 + (1.0 + 8.0 * pidx as f64).sqrt()) / 2.0) as u64;
    while i * (i - 1) / 2 > pidx {
        i -= 1;
    }
    while (i + 1) * i / 2 <= pidx {
        i += 1;
    }
    let j = pidx - i * (i - 1) / 2;
    (j as usize, i as usize)
}

impl Coll {
    fn new(inst: &Instance, c100: u64, rng: &mut Rng) -> Coll {
        let f = inst.ctx.c.fq;
        let q = f.0;
        let fc = inst.ctx.c.f;
        let o = inst.ctx.c.o;
        let okey = o.0 * q + o.1;
        let mut sqt = vec![NONE32; q as usize];
        for x in 0..q {
            sqt[f.mul(x, x) as usize] = x as u32;
        }
        let b = ((c100 as f64 / 100.0) * (q as f64).sqrt()).ceil() as usize;
        let mut fbmap: HashMap<u64, u32> = HashMap::new();
        let mut fbp: Vec<(u64, u64, [u64; 5])> = Vec::new();
        while fbp.len() < b {
            let (x, y) = random_point(f, &fc, rng);
            let key = x * q + y;
            if key == okey || fbmap.contains_key(&key) {
                continue;
            }
            fbmap.insert(key, fbp.len() as u32);
            fbp.push((x, y, derivs(f, &fc, x, y)));
        }
        // the base point O is index b: its logarithm is pinned to 0 and its column dropped
        fbp.push((o.0, o.1, derivs(f, &fc, o.0, o.1)));
        let mpairs = ((b + 1) * b / 2) as u64;
        let mut order: Vec<u32> = (0..mpairs as u32).collect();
        for k in (1..order.len()).rev() {
            let r = rng.below(k as u64 + 1) as usize;
            order.swap(k, r);
        }
        cn(mpairs);
        Coll {
            f,
            fc,
            q,
            okey,
            b,
            fbp,
            fbmap,
            sqt,
            order,
            t: 0,
            mpairs,
            vmap: HashMap::new(),
            vkeys: Vec::new(),
            edges: Vec::new(),
            uf: vec![0],
            cedges: vec![0],
            csize: vec![1],
            pairs_done: 0,
            cycles_ground: 0,
        }
    }

    fn ground_cycles(&mut self) -> i64 {
        let rg = uf_find(&mut self.uf, 0);
        self.cedges[rg as usize] as i64 - (self.csize[rg as usize] as i64 - 1)
    }

    /// One pair trial: line through factor-base points `i < j`, residual quadratic in the line
    /// parameter. Returns the two residual points as end codes (`-1` = O, `<= -2` factor-base
    /// point `-(e+2)`, `>= 0` large-prime vertex) when both are rational and distinct.
    fn pair_edge(&mut self, i: usize, j: usize) -> Option<[i32; 2]> {
        let f = self.f;
        let q = self.q;
        let (x1, y1, d1) = (self.fbp[i].0, self.fbp[i].1, self.fbp[i].2);
        let (x2, y2) = (self.fbp[j].0, self.fbp[j].1);
        let dx = f.sub(x2, x1);
        let dy = f.sub(y2, y1);
        let g1 = f.add(f.mul(d1[0], dx), f.mul(d1[1], dy));
        let dx2 = f.mul(dx, dx);
        let dxy = f.mul(dx, dy);
        let dy2 = f.mul(dy, dy);
        let g2 = f.add(
            f.add(f.mul(d1[2], dx2), f.mul(d1[3], dxy)),
            f.mul(d1[4], dy2),
        );
        let mut g4 = 0;
        let mut dxp = [1u64; 5];
        let mut dyp = [1u64; 5];
        for e in 1..5 {
            dxp[e] = f.mul(dxp[e - 1], dx);
            dyp[e] = f.mul(dyp[e - 1], dy);
        }
        for ii in 0..5 {
            let c = self.fc[ii][4 - ii];
            if c != 0 {
                g4 = f.add(g4, f.mul(c, f.mul(dxp[ii], dyp[4 - ii])));
            }
        }
        let a0 = f.neg(g1);
        let a1 = f.sub(a0, g2);
        let a2 = g4;
        if a2 == 0 {
            return None;
        }
        let disc = f.sub(f.mul(a1, a1), f.mul(4, f.mul(a0, a2)));
        cn(1);
        let sq = self.sqt[disc as usize];
        if sq == NONE32 || disc == 0 {
            return None;
        }
        let ia = f.inv(f.add(a2, a2));
        let ua = f.mul(f.sub(f.neg(a1), sq as u64), ia);
        let ub = f.mul(f.add(f.neg(a1), sq as u64), ia);
        if ua == 0 || ua == 1 || ub == 0 || ub == 1 {
            return None;
        }
        let mut ends = [0i32; 2];
        for (e, u) in [ua, ub].iter().enumerate() {
            let x = f.add(x1, f.mul(*u, dx));
            let y = f.add(y1, f.mul(*u, dy));
            let key = x * q + y;
            cn(1);
            ends[e] = if key == self.okey {
                -1
            } else if let Some(&g) = self.fbmap.get(&key) {
                -(2 + g as i32)
            } else if let Some(&v) = self.vmap.get(&key) {
                v as i32
            } else {
                let v = self.vkeys.len() as u32;
                self.vmap.insert(key, v);
                self.vkeys.push(key);
                self.uf.push(self.uf.len() as u32);
                self.cedges.push(0);
                self.csize.push(1);
                v as i32
            };
        }
        // A line through three or four base points is enumerated by every pair among them and would
        // give the same row several times (parallel edges counted as independent cycles); keep it
        // only for the pair of its two smallest base-point indices (`O` has index `b`).
        let mut smallest = [i as i64, j as i64, i64::MAX, i64::MAX];
        let mut extra = 0;
        for &e in &ends {
            let idx = if e == -1 {
                Some(self.b as i64)
            } else if e <= -2 {
                Some((-(e + 2)) as i64)
            } else {
                None
            };
            if let Some(k) = idx {
                smallest[2 + extra] = k;
                extra += 1;
            }
        }
        if extra > 0 {
            let mut sorted = smallest;
            sorted.sort_unstable();
            if sorted[0] != i as i64 || sorted[1] != j as i64 {
                cn(1);
                return None;
            }
        }
        Some(ends)
    }

    /// Collect pairs until the ground component carries `target` independent cycles.
    fn extend(&mut self, target: i64) {
        while self.t < self.mpairs && self.cycles_ground < target {
            let pidx = self.order[self.t as usize] as u64;
            self.t += 1;
            let (i, j) = pair_decode(pidx);
            self.pairs_done += 1;
            let Some(ends) = self.pair_edge(i, j) else {
                continue;
            };
            self.edges.push((ends[0], ends[1], i as u32, j as u32));
            let node = |e: i32| -> u32 {
                if e >= 0 {
                    e as u32 + 1
                } else {
                    0
                }
            };
            let (n1, n2) = (node(ends[0]), node(ends[1]));
            let (r1, r2) = (uf_find(&mut self.uf, n1), uf_find(&mut self.uf, n2));
            if r1 == r2 {
                self.cedges[r1 as usize] += 1;
            } else {
                self.uf[r2 as usize] = r1;
                self.cedges[r1 as usize] += self.cedges[r2 as usize] + 1;
                self.csize[r1 as usize] += self.csize[r2 as usize];
            }
            if self.pairs_done % 32 == 0 || self.t == self.mpairs {
                self.cycles_ground = self.ground_cycles();
            }
        }
        self.cycles_ground = self.ground_cycles();
    }
}

/// Sparse row of one edge `P_i + P_j + X + Y = h` over the columns `0 = h`, `1..=b` factor-base
/// points, `b + 1 + v` large-prime vertex `v`. Coefficients of equal columns are merged, zero
/// entries dropped, the point `O` (index `b`, end code `-1`) is omitted.
pub fn edge_row(mont: &Mont, b: usize, edge: (i32, i32, u32, u32)) -> SRow {
    let mut ent: Vec<(u32, i64)> = vec![(0, -1)];
    for &k in &[edge.2, edge.3] {
        if (k as usize) < b {
            ent.push((k + 1, 1));
        }
    }
    for &e in &[edge.0, edge.1] {
        if e >= 0 {
            ent.push((b as u32 + 1 + e as u32, 1));
        } else if e <= -2 {
            ent.push(((-(e + 2)) as u32 + 1, 1));
        }
    }
    ent.sort_unstable_by_key(|x| x.0);
    let mut out: SRow = Vec::with_capacity(ent.len());
    let mut idx = 0;
    while idx < ent.len() {
        let c = ent[idx].0;
        let mut s = 0i64;
        while idx < ent.len() && ent[idx].0 == c {
            s += ent[idx].1;
            idx += 1;
        }
        if s != 0 {
            out.push((c, mont.from_i(s)));
        }
    }
    out
}

fn addmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}

fn submod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + m as u128 - b as u128) % m as u128) as u64
}

/// Logarithms (in units of `h`) of every large-prime vertex reachable from the factor base by
/// edge propagation `x_X + x_Y = 1 - x_i - x_j`. `fb[k]` is the log of factor-base point `k`
/// (`k = b` is `O`, value 0). Returns `vertex -> log`.
pub fn propagate(
    edges: &[(i32, i32, u32, u32)],
    nv: usize,
    fb: &[u64],
    ell: u64,
) -> Vec<Option<u64>> {
    let mut val: Vec<Option<u64>> = vec![None; nv];
    let mut inc: Vec<Vec<u32>> = vec![vec![]; nv];
    for (ei, e) in edges.iter().enumerate() {
        if e.0 >= 0 {
            inc[e.0 as usize].push(ei as u32);
        }
        if e.1 >= 0 {
            inc[e.1 as usize].push(ei as u32);
        }
    }
    // value of an end code when it is a ground point
    let ground = |e: i32| -> Option<u64> {
        if e == -1 {
            Some(0)
        } else if e <= -2 {
            Some(fb[(-(e + 2)) as usize])
        } else {
            None
        }
    };
    // rhs of an edge: 1 - x_i - x_j
    let rhs = |e: &(i32, i32, u32, u32)| -> u64 {
        let xi = fb[e.2 as usize];
        let xj = fb[e.3 as usize];
        submod(submod(1 % ell, xi, ell), xj, ell)
    };
    let mut queue: Vec<u32> = Vec::new();
    for e in edges {
        let (a, bb) = (e.0, e.1);
        if (a >= 0) != (bb >= 0) {
            let (v, w) = if a >= 0 { (a, bb) } else { (bb, a) };
            if val[v as usize].is_none() {
                let g = ground(w).unwrap();
                val[v as usize] = Some(submod(rhs(e), g, ell));
                queue.push(v as u32);
            }
        }
    }
    cn(edges.len() as u64);
    let mut qi = 0;
    while qi < queue.len() {
        let v = queue[qi] as usize;
        qi += 1;
        let xv = val[v].unwrap();
        for &ei in &inc[v] {
            let e = &edges[ei as usize];
            if e.0 < 0 || e.1 < 0 {
                continue;
            }
            let u = if e.0 as usize == v { e.1 } else { e.0 } as usize;
            if val[u].is_none() {
                val[u] = Some(submod(rhs(e), xv, ell));
                queue.push(u as u32);
            }
            cn(1);
        }
    }
    val
}

/// Target stage of the base tool (walk `aP + bQ + t` until the degree-3 divisor splits into
/// three points of known logarithm, two independent hits give `log_P Q`).
fn descend(inst: &mut Instance, known: &HashMap<u64, u64>, rng: &mut Rng) -> Option<u64> {
    let ell = inst.ell;
    let q = inst.q;
    let p = inst.p.clone();
    let qd = inst.qq.clone();
    let mut rels: Vec<(u64, u64, u64)> = Vec::new();
    let mut trials = 0u64;
    let mut outer_iters = 0u64;
    while trials < 40_000 && outer_iters < 3000 {
        outer_iters += 1;
        let ea0 = 1 + rng.below(ell - 1);
        let eb0 = 1 + rng.below(ell - 1);
        let (Some(a), Some(bq)) = (inst.ctx.smul(ea0, &p), inst.ctx.smul(eb0, &qd)) else {
            continue;
        };
        let Some(mut xi) = inst.ctx.add_r(&a, &bq) else {
            continue;
        };
        let (mut ea, mut eb) = (ea0, eb0);
        for _ in 0..4000 {
            trials += 1;
            let ff = inst.ctx.c.fq;
            let xq = ppowmod(ff, &[0, 1], q, &xi.a);
            cn(1);
            if xq == vec![0, 1] {
                let rts = poly_roots(ff, &xi.a, rng);
                if rts.len() == 3 {
                    let mut sum = 0u64;
                    let mut ok = true;
                    for &xr in &rts {
                        let yr = peval(ff, &xi.b, xr);
                        cn(1);
                        match known.get(&(xr * q + yr)) {
                            Some(&lv) => sum = (sum + lv) % ell,
                            None => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok {
                        let indep = rels.iter().all(|r| {
                            let det = (mulmod(r.0, eb, ell) + ell - mulmod(r.1, ea, ell)) % ell;
                            det != 0
                        });
                        if indep {
                            rels.push((ea, eb, sum));
                        }
                        if rels.len() == 2 {
                            let (r1, r2) = (rels[0], rels[1]);
                            let det =
                                (mulmod(r1.0, r2.1, ell) + ell - mulmod(r1.1, r2.0, ell)) % ell;
                            if det != 0 {
                                let di = invmod(det, ell);
                                let lp = mulmod(
                                    (mulmod(r1.2, r2.1, ell) + ell - mulmod(r2.2, r1.1, ell)) % ell,
                                    di,
                                    ell,
                                );
                                let lq = mulmod(
                                    (mulmod(r1.0, r2.2, ell) + ell - mulmod(r2.0, r1.2, ell)) % ell,
                                    di,
                                    ell,
                                );
                                if lp != 0 {
                                    return Some(mulmod(lq, invmod(lp, ell), ell));
                                }
                            }
                            rels.clear();
                        }
                    }
                }
            }
            let stepq = rng.below(2) == 1;
            let nx = if stepq {
                inst.ctx.add_r(&xi, &qd)
            } else {
                inst.ctx.add_r(&xi, &p)
            };
            match nx {
                Some(n) => {
                    xi = n;
                    if stepq {
                        eb = (eb + 1) % ell;
                    } else {
                        ea = (ea + 1) % ell;
                    }
                }
                None => break,
            }
        }
    }
    None
}

/// Dense rows (base-tool layout: factor-base columns `0..b`, `h` at column `b`) from sparse rows
/// over `0 = h`, `1..=b`.
pub fn densify(mont: &Mont, b: usize, rows: &[SRow]) -> Vec<Vec<u64>> {
    rows.iter()
        .map(|r| {
            let mut d = vec![0u64; b + 1];
            for &(c, v) in r {
                let plain = mont.from(v);
                if c == 0 {
                    d[b] = plain;
                } else {
                    d[c as usize - 1] = plain;
                }
            }
            d
        })
        .collect()
}

/// One sparse-IC unit. `rows_mult`: target rows are `rows_mult * b + 30 + holdout`.
fn run_sparse(
    inst: &mut Instance,
    c100: u64,
    seed: u64,
    rows_mult: f64,
    holdout: usize,
    with_dense: bool,
    rowmode: u8,
) -> SparseRes {
    let ell = inst.ell;
    let mut rng = Rng::new(seed);
    let ops0 = inst.ctx.ops;
    let c_start = snap();
    let mont = Mont::new(ell);
    let mut res = SparseRes::default();
    let c_setup = snap();
    let t_setup = Instant::now();
    let mut col = Coll::new(inst, c100, &mut rng);
    let setup = tally_diff(snap(), c_setup);
    let setup_wall = t_setup.elapsed().as_secs_f64();
    let b = col.b;
    res.b = b;
    let mut rel = Tally::default();
    let mut target = (rows_mult * b as f64).ceil() as i64 + 30 + holdout as i64;
    // rows handed to the solver in mode `light`; collection runs to `light_mult * b` cycles first
    let mut m_take = target as usize;
    if rowmode == 2 {
        let lm: f64 = std::env::var("QDS_LIGHTMULT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3.0);
        target = (lm * b as f64).ceil() as i64 + 30 + holdout as i64;
    }
    let mut sol_v: Option<Vec<u64>> = None;
    let mut dense_rows: Vec<Vec<u64>> = Vec::new();
    let mut elim_wall = 0.0;
    let mut wied_wall = 0.0;
    let mut rel_wall = 0.0;
    for round in 1..=8u32 {
        res.rounds = round;
        let c0 = snap();
        let t0 = Instant::now();
        col.extend(target);
        rel_wall += t0.elapsed().as_secs_f64();
        rel.add(tally_diff(snap(), c0));
        res.cycles = col.cycles_ground;
        res.pairs = col.pairs_done;
        res.edges = col.edges.len();
        // mode `light` may proceed with fewer cycles than its collection target once every pair is
        // used, provided the rows it hands to the solver are available
        let exhausted = col.t >= col.mpairs;
        if col.cycles_ground < target
            && !(rowmode == 2 && exhausted && col.cycles_ground >= m_take as i64)
        {
            res.status = format!("fail=cycles({},{target})", col.cycles_ground);
            break;
        }
        // rows of the ground component
        let rg = uf_find(&mut col.uf, 0);
        let mut rows: Vec<SRow> = Vec::new();
        let mut wsum = 0usize;
        let mut wmax = 0usize;
        for e in &col.edges {
            let n1 = if e.0 >= 0 { e.0 as u32 + 1 } else { 0 };
            if uf_find(&mut col.uf, n1) != rg {
                continue;
            }
            let r = edge_row(&mont, b, *e);
            wsum += r.len();
            wmax = wmax.max(r.len());
            rows.push(r);
        }
        cn(col.edges.len() as u64);
        res.kept = rows.len();
        res.w_raw_mean = wsum as f64 / rows.len().max(1) as f64;
        res.w_raw_max = wmax;
        // elimination of the large-prime columns
        let t1 = Instant::now();
        let (mut eo, mut ee) = (0u64, 0u64);
        let fbrows = if rowmode >= 1 {
            let (mut r, ent) = bfs_rows(&mont, &col.edges, col.vkeys.len(), b);
            ee = ent;
            if rowmode == 2 {
                if r.len() < m_take {
                    target += (b as i64 / 2).max(20);
                    res.status = "short_cycles".into();
                    continue;
                }
                r = select_light(r, m_take, b);
            }
            r
        } else {
            dq::eliminate_lp(&mont, b + 1, rows, &mut eo, &mut ee)
        };
        elim_wall += t1.elapsed().as_secs_f64();
        res.elim_ops += eo;
        res.elim_entries += ee;
        res.rows_fb = fbrows.len();
        let nnz: usize = fbrows.iter().map(|r| r.len()).sum();
        res.nnz_fb = nnz;
        res.w_fb_mean = nnz as f64 / fbrows.len().max(1) as f64;
        if fbrows.len() < b + holdout {
            res.status = format!("short_rows({},{})", fbrows.len(), b + holdout);
            if rowmode == 2 {
                m_take += (b / 8).max(10);
            } else {
                target += (b as i64 / 8).max(10);
            }
            continue;
        }
        let t2 = Instant::now();
        let la = dq::wiedemann_solve(&mont, &fbrows, b + 1, holdout, seed ^ 0x57, None);
        wied_wall += t2.elapsed().as_secs_f64();
        res.wied_ops += la.wied_ops;
        res.bm_ops += la.bm_ops;
        res.matvecs += la.matvecs;
        res.deg = la.deg;
        if !la.ok {
            res.status = format!("wied_{}", la.fail);
            if rowmode == 2 {
                m_take += (b / 8).max(10);
            } else {
                target += (b as i64 / 8).max(10);
            }
            continue;
        }
        res.status = "ok".into();
        sol_v = Some(la.v);
        if with_dense {
            let m = fbrows.len() - holdout;
            dense_rows = densify(&mont, b, &fbrows[..m]);
        }
        break;
    }
    res.rel = rel;
    res.setup = setup;
    res.wall_rel = rel_wall + setup_wall;
    res.wall_elim = elim_wall;
    res.wall_wied = wied_wall;
    let Some(v) = sol_v else {
        res.total = Tally::default();
        res.total.add(res.setup);
        res.total.add(res.rel);
        return res;
    };
    // dense A/B on the same rows
    if with_dense {
        let t3 = Instant::now();
        let la0 = snap().la;
        let sol = nullspace(dense_rows, b + 1, ell);
        res.dense_la = snap().la - la0;
        res.wall_dense = t3.elapsed().as_secs_f64();
        if let Ok(s) = sol {
            res.dense_ok = true;
            let hinv = invmod(s[b], ell);
            let agree = (0..b).all(|k| mulmod(s[k], hinv, ell) == v[k + 1]);
            res.agree = Some(agree);
        } else {
            res.agree = Some(false);
        }
    }
    // logarithms: factor base, O, large primes by propagation
    let mut fb: Vec<u64> = (0..b).map(|k| v[k + 1]).collect();
    fb.push(0);
    let t4 = Instant::now();
    let lv = propagate(&col.edges, col.vkeys.len(), &fb, ell);
    let mut known: HashMap<u64, u64> = HashMap::new();
    known.insert(col.okey, 0);
    for (key, &g) in &col.fbmap {
        known.insert(*key, fb[g as usize]);
    }
    for (vi, val) in lv.iter().enumerate() {
        if let Some(x) = val {
            known.insert(col.vkeys[vi], *x);
        }
    }
    res.known_frac = known.len() as f64 / inst.n1 as f64;
    let ctg = snap();
    let ops_t = inst.ctx.ops;
    let found = descend(inst, &known, &mut rng);
    res.tgt = tally_diff(snap(), ctg);
    res.tgt_adds = inst.ctx.ops - ops_t;
    res.wall_tgt = t4.elapsed().as_secs_f64();
    res.found = found.is_some();
    res.found_k = found;
    if let Some(k) = found {
        let pp = inst.p.clone();
        res.verified = inst.ctx.smul(k, &pp) == Some(inst.qq.clone());
    }
    let extras = std::mem::take(&mut inst.extra);
    for qx in extras {
        let cx = snap();
        let ox = inst.ctx.ops;
        inst.qq = qx.clone();
        let fk = descend(inst, &known, &mut rng);
        let t = tally_diff(snap(), cx);
        let pp = inst.p.clone();
        let ver = fk.is_some_and(|k| inst.ctx.smul(k, &pp) == Some(qx.clone()));
        if std::env::var("QDS_DEBUG").is_ok() {
            if let Some(k) = fk {
                let r = inst.ctx.smul(k, &pp);
                eprintln!(
                    "extra: k={k} ver={ver} smul={:?} qx=({:?},{:?})",
                    r.as_ref().map(|d| (d.a.clone(), d.b.clone())),
                    qx.a,
                    qx.b
                );
            }
        }
        res.extra_tgts.push((fk, t, inst.ctx.ops - ox, ver));
    }
    res.total = Tally::default();
    res.total.add(res.setup);
    res.total.add(res.rel);
    res.total.add(res.tgt);
    let _ = (ops0, c_start);
    res
}

/// Exploratory row mode (Amendment 1): breadth-first potentials from the base points, as in the base
/// tool, but returned as sparse rows over the columns `0 = h`, `1..=b`. Every edge outside the
/// breadth-first forest of the ground component gives one row `p_X + p_Y - (h - x_i - x_j) = 0`.
/// Returns the rows and the number of form entries produced (counted by the caller as native work).
pub fn bfs_rows(
    mont: &Mont,
    edges: &[(i32, i32, u32, u32)],
    nv: usize,
    b: usize,
) -> (Vec<SRow>, u64) {
    let lform = |i: u32, j: u32| -> Form {
        let mut v: Form = Vec::new();
        if (i as usize) < b {
            v.push((i, -1));
        }
        if (j as usize) < b {
            v.push((j, -1));
        }
        v.push((b as u32, 1));
        v.sort_unstable_by_key(|e| e.0);
        // merge equal variables (i == j cannot occur; kept for safety)
        v
    };
    let phi = |e: i32| -> Form {
        if e == -1 {
            vec![]
        } else {
            vec![((-(e + 2)) as u32, 1)]
        }
    };
    let mut entries = 0u64;
    let mut inc: Vec<Vec<u32>> = vec![vec![]; nv];
    for (ei, e) in edges.iter().enumerate() {
        if e.0 >= 0 {
            inc[e.0 as usize].push(ei as u32);
        }
        if e.1 >= 0 {
            inc[e.1 as usize].push(ei as u32);
        }
    }
    let mut pot: Vec<Option<Form>> = vec![None; nv];
    let mut tree = vec![false; edges.len()];
    let mut queue: Vec<u32> = Vec::new();
    for (ei, e) in edges.iter().enumerate() {
        let (a, bb) = (e.0, e.1);
        if (a >= 0) != (bb >= 0) {
            let (v, w) = if a >= 0 { (a, bb) } else { (bb, a) };
            if pot[v as usize].is_none() {
                let f = form_comb(&lform(e.2, e.3), 1, &phi(w), -1);
                entries += f.len() as u64;
                pot[v as usize] = Some(f);
                tree[ei] = true;
                queue.push(v as u32);
            }
        }
    }
    let mut qi = 0;
    while qi < queue.len() {
        let v = queue[qi] as usize;
        qi += 1;
        for &ei in &inc[v] {
            let e = edges[ei as usize];
            if e.0 < 0 || e.1 < 0 {
                continue;
            }
            let u = if e.0 as usize == v { e.1 } else { e.0 } as usize;
            if pot[u].is_none() {
                let f = form_comb(&lform(e.2, e.3), 1, pot[v].as_ref().unwrap(), -1);
                entries += f.len() as u64;
                pot[u] = Some(f);
                tree[ei as usize] = true;
                queue.push(u as u32);
            }
        }
    }
    let mut rows: Vec<SRow> = Vec::new();
    for (ei, e) in edges.iter().enumerate() {
        if tree[ei] {
            continue;
        }
        let side = |end: i32| -> Option<Form> {
            if end >= 0 {
                pot[end as usize].clone()
            } else {
                Some(phi(end))
            }
        };
        let (Some(fa), Some(fb)) = (side(e.0), side(e.1)) else {
            continue;
        };
        let sum = form_comb(&fa, 1, &fb, 1);
        let row = form_comb(&sum, 1, &lform(e.2, e.3), -1);
        entries += (sum.len() + row.len()) as u64;
        if row.is_empty() {
            continue;
        }
        let mut out: SRow = row
            .iter()
            .map(|&(var, c)| {
                let col = if var as usize == b { 0 } else { var + 1 };
                (col, mont.from_i(c as i64))
            })
            .collect();
        out.sort_unstable_by_key(|e| e.0);
        rows.push(out);
    }
    (rows, entries)
}

/// Exploratory row selection (Amendment 1, mode `light`): the `want` lightest rows, then a coverage
/// guard that adds the lightest unused rows until every factor-base column occurs in at least two
/// chosen rows. (The N4 study found lightest-first without a guard rank deficient.)
pub fn select_light(rows: Vec<SRow>, want: usize, b: usize) -> Vec<SRow> {
    let mut idx: Vec<usize> = (0..rows.len()).collect();
    idx.sort_by_key(|&i| (rows[i].len(), i));
    let take = want.min(rows.len());
    let mut chosen = vec![false; rows.len()];
    let mut count = vec![0u32; b + 1];
    for &i in &idx[..take] {
        chosen[i] = true;
        for &(c, _) in &rows[i] {
            count[c as usize] += 1;
        }
    }
    for col in 1..=b {
        if count[col] >= 2 {
            continue;
        }
        for &i in &idx[take..] {
            if count[col] >= 2 {
                break;
            }
            if !chosen[i] && rows[i].binary_search_by_key(&(col as u32), |e| e.0).is_ok() {
                chosen[i] = true;
                for &(c, _) in &rows[i] {
                    count[c as usize] += 1;
                }
            }
        }
    }
    cn(rows.len() as u64);
    rows.into_iter()
        .zip(chosen)
        .filter_map(|(r, c)| c.then_some(r))
        .collect()
}

// ------------------------------------------------------------------------- CLI

fn jnum(v: f64) -> String {
    if v.is_finite() {
        format!("{v}")
    } else {
        "null".into()
    }
}

fn record_ic(
    git: &str,
    inst: &Instance,
    inst_i: u64,
    cseed: u64,
    ic_seed: u64,
    c100: u64,
    tag: &str,
    gae: (f64, f64),
    r: &SparseRes,
) -> String {
    format!(
        concat!(
            "{{\"kind\":\"ic\",\"tag\":\"{}\",\"git\":\"{}\",\"q\":{},\"inst\":{},\"curve_seed\":{},\"ic_seed\":{},",
            "\"c\":{},\"N\":{},\"ell\":{},\"cof\":{},\"b\":{},\"status\":\"{}\",\"found\":{},\"verified\":{},",
            "\"pairs\":{},\"edges\":{},\"kept\":{},\"cycles\":{},\"rounds\":{},\"w_raw_mean\":{},\"w_raw_max\":{},",
            "\"rows_fb\":{},\"w_fb_mean\":{},\"nnz_fb\":{},",
            "\"setup_M\":{},\"setup_I\":{},\"setup_NAT\":{},\"rel_M\":{},\"rel_I\":{},\"rel_NAT\":{},",
            "\"elim_ops\":{},\"elim_entries\":{},\"wied_ops\":{},\"bm_ops\":{},\"matvecs\":{},\"deg\":{},",
            "\"dense_la\":{},\"dense_ok\":{},\"agree\":{},",
            "\"tgt_M\":{},\"tgt_I\":{},\"tgt_NAT\":{},\"tgt_adds\":{},\"known_frac\":{},",
            "\"M\":{},\"I\":{},\"NAT\":{},\"gae_M\":{},\"gae_I\":{},",
            "\"wall_rel\":{},\"wall_elim\":{},\"wall_wied\":{},\"wall_dense\":{},\"wall_tgt\":{}}}"
        ),
        tag,
        git,
        inst.q,
        inst_i,
        cseed,
        ic_seed,
        c100,
        inst.n_group,
        inst.ell,
        inst.cof,
        r.b,
        r.status,
        r.found,
        r.verified,
        r.pairs,
        r.edges,
        r.kept,
        r.cycles,
        r.rounds,
        jnum(r.w_raw_mean),
        r.w_raw_max,
        r.rows_fb,
        jnum(r.w_fb_mean),
        r.nnz_fb,
        r.setup.m,
        r.setup.i,
        r.setup.nat,
        r.rel.m,
        r.rel.i,
        r.rel.nat,
        r.elim_ops,
        r.elim_entries,
        r.wied_ops,
        r.bm_ops,
        r.matvecs,
        r.deg,
        r.dense_la,
        r.dense_ok,
        match r.agree {
            Some(true) => "true",
            Some(false) => "false",
            None => "null",
        },
        r.tgt.m,
        r.tgt.i,
        r.tgt.nat,
        r.tgt_adds,
        jnum(r.known_frac),
        r.total.m,
        r.total.i,
        r.total.nat,
        jnum(gae.0),
        jnum(gae.1),
        jnum(r.wall_rel),
        jnum(r.wall_elim),
        jnum(r.wall_wied),
        jnum(r.wall_dense),
        jnum(r.wall_tgt)
    )
}

fn record_rho(
    git: &str,
    inst: &Instance,
    inst_i: u64,
    cseed: u64,
    seed: u64,
    neg: bool,
    gae: (f64, f64),
    r: &ArmResult,
    wall: f64,
) -> String {
    format!(
        concat!(
            "{{\"kind\":\"{}\",\"git\":\"{}\",\"q\":{},\"inst\":{},\"curve_seed\":{},\"rho_seed\":{},\"N\":{},\"ell\":{},",
            "\"found\":{},\"verified\":{},\"steps\":{},\"adds\":{},\"M\":{},\"I\":{},\"NAT\":{},",
            "\"gae_M\":{},\"gae_I\":{},\"wall\":{}}}"
        ),
        if neg { "rhoneg" } else { "rho" },
        git,
        inst.q,
        inst_i,
        cseed,
        seed,
        inst.n_group,
        inst.ell,
        r.found,
        r.verified,
        r.steps,
        r.adds,
        r.cnt.m,
        r.cnt.i,
        r.cnt.nat,
        jnum(gae.0),
        jnum(gae.1),
        jnum(wall)
    )
}

/// `run <out.jsonl> <q> <n_inst> <seed0> <c-list x100> <rho: none|plain|both> <dense_max_b> [rho_runs]`
fn cmd_run_sparse(a: &[String]) {
    let out = &a[0];
    let q: u64 = a[1].parse().unwrap();
    let n: u64 = a[2].parse().unwrap();
    let seed0: u64 = a[3].parse().unwrap();
    let cs: Vec<u64> = if a[4] == "-" {
        Vec::new()
    } else {
        a[4].split(',').map(|v| v.parse().unwrap()).collect()
    };
    let rho_mode = a[5].as_str();
    let dense_max_b: usize = a[6].parse().unwrap();
    let rho_runs: u64 = a.get(7).map(|v| v.parse().unwrap()).unwrap_or(1);
    let git = std::env::var("QDS_GIT").unwrap_or_else(|_| "unknown".into());
    // exploratory row mode (PLAN Amendment 1): QDS_ROWMODE=bfs
    let rowmode: u8 = match std::env::var("QDS_ROWMODE").as_deref() {
        Ok("bfs") => 1,
        Ok("light") => 2,
        _ => 0,
    };
    let t0 = Instant::now();
    // QDS_INST0: first instance index (run the instances of one cell in parallel processes)
    let inst0: u64 = std::env::var("QDS_INST0")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for inst_i in inst0..inst0 + n {
        let cseed = seed0 * 1000 + inst_i;
        let Some(mut inst) = build_instance(q, cseed, 64, false) else {
            eprintln!("instance {inst_i}: generation failed");
            continue;
        };
        let fc = inst.ctx.c.f;
        let gae = measure_gae(&mut inst.ctx, &fc);
        let mut lines = String::new();
        for &c in &cs {
            // A/A: the factor-base size c = 2.0 is run with two IC seeds
            let seeds: Vec<(u64, &str)> = if c == 200 {
                vec![(cseed ^ 0x1C, "main"), (cseed ^ 0x1D, "aa")]
            } else {
                vec![(cseed ^ 0x1C, "main")]
            };
            let seeds: Vec<(u64, &str)> = match rowmode {
                1 => seeds
                    .into_iter()
                    .map(|(s, t)| (s, if t == "main" { "bfs" } else { "bfs_aa" }))
                    .collect(),
                2 => seeds
                    .into_iter()
                    .map(|(s, t)| (s, if t == "main" { "light" } else { "light_aa" }))
                    .collect(),
                _ => seeds,
            };
            for (s, tag) in seeds {
                let b_est = (c as f64 / 100.0 * (q as f64).sqrt()).ceil() as usize;
                let r = run_sparse(&mut inst, c, s, 1.25, 8, b_est <= dense_max_b, rowmode);
                lines.push_str(&record_ic(&git, &inst, inst_i, cseed, s, c, tag, gae, &r));
                lines.push('\n');
                eprintln!(
                    "q={q} inst={inst_i} c={c} {tag} status={} verified={} b={} pairs={} edges={} rows_fb={} w_fb={:.1} elim={} wied={} bm={} dense={} adds_tgt={} [{:.0}s]",
                    r.status, r.verified, r.b, r.pairs, r.edges, r.rows_fb, r.w_fb_mean,
                    r.elim_ops, r.wied_ops, r.bm_ops, r.dense_la, r.tgt_adds, t0.elapsed().as_secs_f64()
                );
            }
        }
        if rho_mode != "none" {
            // QDS_RHO_RUN0: first rho run index (run indices select the seeds)
            let run0: u64 = std::env::var("QDS_RHO_RUN0")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            for run in run0..run0 + rho_runs {
                for neg in [false, true] {
                    if neg && rho_mode != "both" {
                        continue;
                    }
                    let s = cseed ^ (if neg { 0x3E } else { 0x2D }) ^ (run << 20);
                    let tw = Instant::now();
                    let r = run_rho(&mut inst, neg, s);
                    let wall = tw.elapsed().as_secs_f64();
                    lines.push_str(&record_rho(
                        &git, &inst, inst_i, cseed, s, neg, gae, &r, wall,
                    ));
                    lines.push('\n');
                    eprintln!(
                        "q={q} inst={inst_i} rho{} verified={} steps={} adds={} [{:.0}s]",
                        if neg { "neg" } else { "" },
                        r.verified,
                        r.steps,
                        r.adds,
                        t0.elapsed().as_secs_f64()
                    );
                }
            }
        }
        let mut fh = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(out)
            .unwrap();
        std::io::Write::write_all(&mut fh, lines.as_bytes()).unwrap();
    }
}

/// Calibrated weight of one mod-ell operation in group additions (AGENTS.md section 5a, 2026-11-12):
/// best-of-N wall time of one operation counted by the linear algebra (`Mont` multiply-add) and of
/// one plain `u128 %` mulmod, divided by the best-of-N wall time of one `Pic^0` addition on the same
/// curve and host. Shared VM, not isolated; labelled best-of-N.
fn cmd_calib(q: u64, cseed: u64) {
    use std::hint::black_box;
    let Some(mut inst) = build_instance(q, cseed, 64, false) else {
        eprintln!("instance generation failed");
        return;
    };
    let fc = inst.ctx.c.f;
    let mut best_add = f64::MAX;
    for _ in 0..5 {
        let d1 = random_class(&mut inst.ctx, &fc);
        let d2 = random_class(&mut inst.ctx, &fc);
        let t = Instant::now();
        let mut n = 0u32;
        while n < 200 {
            let a = random_class(&mut inst.ctx, &fc);
            if inst.ctx.add(&a, &d2).is_ok() {
                n += 1;
            }
            black_box(&d1);
        }
        // subtract the random_class cost by timing it separately
        let tot = t.elapsed().as_secs_f64();
        let t2 = Instant::now();
        for _ in 0..200 {
            black_box(random_class(&mut inst.ctx, &fc));
        }
        let rc = t2.elapsed().as_secs_f64();
        best_add = best_add.min((tot - rc).max(0.0) / 200.0);
    }
    let ell = inst.ell;
    let mont = Mont::new(ell);
    let n = 2_000_000u64;
    let (mut best_mont, mut best_mm) = (f64::MAX, f64::MAX);
    for _ in 0..7 {
        let t = Instant::now();
        let mut x = mont.to(12345 % ell);
        let y = mont.to(54321 % ell);
        let z = mont.to(777 % ell);
        for _ in 0..n {
            x = black_box(mont.add(mont.mul(x, y), z));
        }
        black_box(x);
        best_mont = best_mont.min(t.elapsed().as_secs_f64() / n as f64);
        let t = Instant::now();
        let mut x = 12345 % ell;
        let y = 54321 % ell;
        for _ in 0..n {
            x = black_box(mulmod(x, y, ell)) ^ 1;
        }
        black_box(x);
        best_mm = best_mm.min(t.elapsed().as_secs_f64() / n as f64);
    }
    println!(
        "{{\"kind\":\"calib\",\"q\":{q},\"curve_seed\":{cseed},\"ell\":{ell},\"t_add_s\":{best_add:e},\"t_mont_op_s\":{best_mont:e},\"t_mulmod_s\":{best_mm:e},\"w_cal_mont\":{},\"w_cal_mulmod\":{},\"method\":\"best-of-5 (add) and best-of-7 (mod-ell) on a shared 4-CPU VM, not isolated\"}}",
        best_mont / best_add,
        best_mm / best_add
    );
}

pub fn cli() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("run") => cmd_run_sparse(&a[2..]),
        Some("calib") => cmd_calib(a[2].parse().unwrap(), a[3].parse().unwrap()),
        Some("selftest") => selftest(a.get(2).map(|v| v.parse().unwrap()).unwrap_or(127)),
        _ => {
            eprintln!("usage: quartic_diem_sparse calib <q> <cseed> | run <out.jsonl> <q> <n_inst> <seed0> <c-list x100> <rho: none|plain|both> <dense_max_b> [rho_runs] | selftest [q]");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod sparse_tests {
    use super::*;

    #[test]
    fn pair_decode_enumerates_each_pair_once() {
        let b = 12usize;
        let n = ((b + 1) * b / 2) as u64;
        let mut seen = std::collections::HashSet::new();
        for p in 0..n {
            let (i, j) = pair_decode(p);
            assert!(i < j && j <= b);
            assert!(seen.insert((i, j)));
        }
        assert_eq!(seen.len() as u64, n);
    }

    #[test]
    fn edge_row_merges_drops_o_and_numbers_columns() {
        let mont = Mont::new(1_000_003);
        let b = 10usize;
        // i = 2, j = 10 (= O, dropped), X = large prime 3, Y = base point 2 (end code -(2 + 2) = -4):
        // x_2 + 0 + x_{LP3} + x_2 - h  ->  2 x_2 (column 3), LP column b + 1 + 3 = 14, h = column 0
        let r = edge_row(&mont, b, (3, -4, 2, 10));
        let plain: Vec<(u32, i64)> = r
            .iter()
            .map(|&(c, v)| {
                let x = mont.from(v);
                (
                    c,
                    if x > 500_000 {
                        x as i64 - 1_000_003
                    } else {
                        x as i64
                    },
                )
            })
            .collect();
        assert_eq!(plain, vec![(0, -1), (3, 2), (14, 1)]);
        // O as an end code and a cancelling pair of equal coefficients is impossible, but a point
        // seen both as i and as X merges: i = 1, j = 4, X = base point 1 -> 2 on column 2
        let r2 = edge_row(&mont, b, (-3, -1, 1, 4));
        let cols: Vec<u32> = r2.iter().map(|e| e.0).collect();
        assert_eq!(cols, vec![0, 2, 5]);
        let c2 = r2.iter().find(|e| e.0 == 2).unwrap().1;
        assert_eq!(mont.from(c2), 2);
    }

    #[test]
    fn propagate_recovers_planted_logs() {
        // planted logs mod ell; base points 0..3, O = index 3 (value 0); vertices 0..3 are large primes
        let ell = 1_000_003u64;
        let b = 3usize;
        let fb = vec![11u64, 22, 33, 0];
        let lp = [100u64, 200, 300, 400];
        let h = 1u64; // h = 1 in these units: build rhs consistently
        let edge = |x: i32, y: i32, i: u32, j: u32| -> (i32, i32, u32, u32) { (x, y, i, j) };
        let val_of = |e: i32| -> u64 {
            if e == -1 {
                0
            } else if e <= -2 {
                fb[(-(e + 2)) as usize]
            } else {
                lp[e as usize]
            }
        };
        // choose the logs of large primes so that every edge relation holds with h = 1
        // x_X + x_Y = 1 - x_i - x_j; build a path v0 - v1 - v2 grounded at v0 by the base point 1
        let mut edges = Vec::new();
        let mut lpv = [0u64; 4];
        // edge A: (base 1, v0; i=0, j=2): x_v0 = 1 - 11 - 33 - 22
        let e_a = edge(-3, 0, 0, 2);
        lpv[0] = (h + 3 * ell - 11 - 33 - 22) % ell;
        edges.push(e_a);
        // edge B: (v0, v1; i=1, j=3): x_v1 = 1 - 22 - 0 - x_v0
        let e_b = edge(0, 1, 1, 3);
        lpv[1] = (h + 3 * ell - 22 - lpv[0]) % ell;
        edges.push(e_b);
        // edge C: (v1, v2; i=0, j=1)
        let e_c = edge(1, 2, 0, 1);
        lpv[2] = (h + 3 * ell - 11 - 22 - lpv[1]) % ell;
        edges.push(e_c);
        let _ = (val_of, b);
        let got = propagate(&edges, 4, &fb, ell);
        assert_eq!(got[0], Some(lpv[0]));
        assert_eq!(got[1], Some(lpv[1]));
        assert_eq!(got[2], Some(lpv[2]));
        assert_eq!(got[3], None);
    }

    #[test]
    fn bfs_rows_and_light_selection_annihilate_planted_vector() {
        // planted logs mod 11 with h = 1; every quadruple (i, j, X, Y) satisfying the relation is an edge
        let ell = 11u64;
        let b = 5usize;
        let mont = Mont::new(ell);
        let mut st = 12345u64;
        let mut nxt = || {
            st = st
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (st >> 33) % ell
        };
        let mut fb: Vec<u64> = (0..b).map(|_| nxt()).collect();
        fb.push(0); // O
        let nlp = 6usize;
        let lp: Vec<u64> = (0..nlp).map(|_| nxt()).collect();
        // end codes of every point: O, base points, large primes
        let mut ends: Vec<(i32, u64)> = vec![(-1, 0)];
        for k in 0..b {
            ends.push((-(k as i32 + 2), fb[k]));
        }
        for v in 0..nlp {
            ends.push((v as i32, lp[v]));
        }
        let mut edges = Vec::new();
        for i in 0..b {
            for j in (i + 1)..=b {
                for (a, &(ea, va)) in ends.iter().enumerate() {
                    for &(eb, vb) in ends.iter().skip(a + 1) {
                        if (fb[i] + fb[j] + va + vb) % ell == 1 {
                            edges.push((ea, eb, i as u32, j as u32));
                        }
                    }
                }
            }
        }
        assert!(edges.len() > 40);
        let x = |col: u32| -> u64 {
            if col == 0 {
                1
            } else {
                fb[col as usize - 1]
            }
        };
        let (rows, entries) = bfs_rows(&mont, &edges, nlp, b);
        assert!(entries > 0 && !rows.is_empty());
        for r in &rows {
            let mut sum = 0u64;
            for &(c, v) in r {
                assert!((c as usize) <= b, "large-prime column left in a row");
                sum = (sum + mont.from(v) * x(c)) % ell;
            }
            assert_eq!(sum, 0, "row does not annihilate the planted vector");
        }
        let light = select_light(rows.clone(), 4, b);
        assert!(light.len() >= 4 && light.len() <= rows.len());
        let mut cover = vec![0u32; b + 1];
        for r in &light {
            for &(c, _) in r {
                cover[c as usize] += 1;
            }
        }
        // the guard gives every column at least two rows whenever the full set has two
        let mut full = vec![0u32; b + 1];
        for r in &rows {
            for &(c, _) in r {
                full[c as usize] += 1;
            }
        }
        for c in 1..=b {
            assert!(cover[c] >= full[c].min(2));
        }
    }

    #[test]
    fn end_to_end_small_q_verifies_and_dense_agrees() {
        // q = 251: 3 curves tried, at least one unit must verify with dense == Wiedemann
        let mut verified = 0;
        for seed in 0..6u64 {
            let Some(mut inst) = build_instance(251, 7000 + seed, 64, false) else {
                continue;
            };
            let r = run_sparse(&mut inst, 400, 99 + seed, 1.25, 8, true, 0);
            if r.status == "ok" && r.verified {
                verified += 1;
                assert_eq!(r.agree, Some(true), "dense and Wiedemann kernels differ");
                assert!(r.w_raw_max <= 5);
            }
        }
        assert!(verified >= 2, "only {verified} verified units");
    }
}

// ======================================================================================
// Round-2 driver (isogeny-walk-quartic_20261121): the sparse Diem arm on a GIVEN quartic with a
// GIVEN subgroup, instead of a random generated one. Inputs are plain numbers; the group law,
// factor base, relation collection, elimination, Wiedemann and descent are the code above.
// ======================================================================================

/// One pushed-forward point pair of the quartic. `rational`: two F_q points given as
/// `pts[0]`, `pts[1]` with zero `b` parts; otherwise `pts[0]` is one of two F_{q^2}-conjugate
/// points, each as `[u_a, u_b, v_a, v_b]` (`u = u_a + u_b w`, `w^2 = nr`).
#[derive(Clone, Debug)]
pub struct PhiIn {
    pub rational: bool,
    pub pts: Vec<[u64; 4]>,
}

/// Input of one curve.
#[derive(Clone, Debug)]
pub struct IcIn {
    pub q: u64,
    pub nr: u64,
    pub fc: [[u64; 5]; 5],
    pub n_group: u64,
    pub ell: u64,
    /// Candidate points; the first eight whose divisor is representable are used.
    pub cands: Vec<PhiIn>,
    pub seed: u64,
    pub ic_seed: u64,
    pub c100: u64,
}

/// (M, I, NAT) counted on a phase.
pub type Mt = [u64; 3];

/// Everything measured for one curve.
#[derive(Clone, Debug, Default)]
pub struct IcOut {
    pub status: String,
    pub chosen: Vec<usize>,
    pub o: (u64, u64),
    pub n1_c: u64,
    pub b: usize,
    pub pairs: u64,
    pub rounds: u32,
    pub rows_fb: usize,
    pub w_fb_mean: f64,
    pub known_frac: f64,
    pub c_setup: Mt,
    pub phi_build: Mt,
    pub fb_setup: Mt,
    pub rel: Mt,
    pub elim_ops: u64,
    pub elim_entries: u64,
    pub wied_ops: u64,
    pub bm_ops: u64,
    pub matvecs: u64,
    /// per target: (log of Q' wrt G' if found, tally, additions, verified on C, Q' is zero)
    pub tgts: Vec<(Option<u64>, Mt, u64, bool, bool)>,
    pub gae_c: (f64, f64),
    pub order_check_c: bool,
    pub wall_rel: f64,
    pub wall_elim: f64,
    pub wall_wied: f64,
}

fn mt(t: Tally) -> Mt {
    [t.m, t.i, t.nat]
}

fn div_conj(f: Fq, nr: u64, o: (u64, u64), pt: [u64; 4]) -> Option<Div> {
    let [ua, ub, va, vb] = pt;
    if ub == 0 {
        return None;
    }
    let s = f.add(ua, ua);
    let n = f.sub(f.mul(ua, ua), f.mul(nr, f.mul(ub, ub)));
    let m: Poly = vec![n, f.neg(s), 1];
    let b1 = f.mul(vb, f.inv(ub));
    let b0 = f.sub(va, f.mul(b1, ua));
    let mut bm: Poly = vec![b0, b1];
    trim(&mut bm);
    let mxo = peval(f, &m, o.0);
    if mxo == 0 {
        return None;
    }
    let t = f.mul(f.sub(o.1, peval(f, &bm, o.0)), f.inv(mxo));
    let b = padd(f, &bm, &pscale(f, &m, t));
    let a = pmul(f, &m, &[f.neg(o.0), 1]);
    Some(Div { a, b })
}

fn div_phi(f: Fq, nr: u64, o: (u64, u64), ph: &PhiIn) -> Option<Div> {
    if ph.rational {
        let pts = [
            (ph.pts[0][0], ph.pts[0][2]),
            (ph.pts[1][0], ph.pts[1][2]),
            o,
        ];
        div_from_points(f, &pts)
    } else {
        div_conj(f, nr, o, ph.pts[0])
    }
}

/// Run the Diem arm on the quartic of `inp`: classes `Phi(P) = [Q1 + Q2 + O - 3 O]`, the generator
/// `G = Phi(P1) - Phi(P2)` and three targets `Phi(P3) - Phi(P4)`, `Phi(P5) - Phi(P6)`,
/// `Phi(P7) - Phi(P8)` of the chosen candidates, all multiplied by `N / ell`.
pub fn smith_ic(inp: &IcIn) -> IcOut {
    let mut out = IcOut::default();
    let f = Fq(inp.q);
    let fc = inp.fc;
    let mut rng = Rng::new(inp.seed);
    // diagnostic only (not charged): number of rational points of C, for `known_frac`
    let n1 = rational_point_count(f, &fc, &mut rng);
    out.n1_c = n1;
    let s0 = snap();
    if !is_smooth(f, &fc) {
        out.status = "quartic not smooth".into();
        return out;
    }
    let mut o = None;
    for _ in 0..400 {
        let (x, y) = random_point(f, &fc, &mut rng);
        let d = derivs(f, &fc, x, y);
        if d[1] == 0 {
            continue;
        }
        let dx = d[1];
        let dy = f.neg(d[0]);
        let g2 = f.add(
            f.add(f.mul(d[2], f.mul(dx, dx)), f.mul(d[3], f.mul(dx, dy))),
            f.mul(d[4], f.mul(dy, dy)),
        );
        if g2 != 0 {
            o = Some((x, y));
            break;
        }
    }
    let Some(o) = o else {
        out.status = "no non-flex base point".into();
        return out;
    };
    out.o = o;
    let curve = Curve::new(f, fc, o);
    let mut ctx = Ctx::new(curve, inp.seed ^ 0x5151);
    for _ in 0..4 {
        let c = random_class(&mut ctx, &fc);
        let Ok(nc) = ctx.neg(&c) else {
            out.status = "pool negation failed".into();
            return out;
        };
        ctx.pool.push(c);
        ctx.pool.push(nc);
    }
    out.c_setup = mt(tally_diff(snap(), s0));
    // divisors of the chosen candidates
    let s1 = snap();
    let mut cls: Vec<Div> = Vec::new();
    for (i, ph) in inp.cands.iter().enumerate() {
        if cls.len() >= 8 {
            break;
        }
        if let Some(d) = div_phi(f, inp.nr, o, ph) {
            out.chosen.push(i);
            cls.push(d);
        }
    }
    if cls.len() < 8 {
        out.status = format!("only {} representable candidates", cls.len());
        return out;
    }
    let cof = inp.n_group / inp.ell;
    let mut diffs: Vec<Div> = Vec::new();
    for j in 0..4 {
        let Some(d) = ctx.sub_r(&cls[2 * j], &cls[2 * j + 1]) else {
            out.status = "class difference failed".into();
            return out;
        };
        let Some(d) = ctx.smul(cof, &d) else {
            out.status = "cofactor clearing failed".into();
            return out;
        };
        diffs.push(d);
    }
    out.phi_build = mt(tally_diff(snap(), s1));
    let id = ctx.id();
    let gp = diffs[0].clone();
    if gp == id {
        out.status = "G' trivial".into();
        return out;
    }
    // check N * G_C = 0 on C, i.e. ell * G' = 0 (validation, not charged)
    out.order_check_c = ctx.smul(inp.ell, &gp) == Some(id.clone());
    if !out.order_check_c {
        out.status = "ell * G' != 0 on C (group order wrong)".into();
        return out;
    }
    let mut targets: Vec<Div> = Vec::new();
    let mut zero_flags: Vec<bool> = Vec::new();
    for d in &diffs[1..] {
        zero_flags.push(*d == id);
        targets.push(d.clone());
    }
    // usable first target (non-zero) is the primary; the others ride along
    let Some(first) = targets.iter().position(|t| *t != id) else {
        out.status = "all targets trivial".into();
        return out;
    };
    let primary = targets[first].clone();
    let extras: Vec<Div> = targets
        .iter()
        .enumerate()
        .filter(|(i, t)| *i != first && **t != id)
        .map(|(_, t)| t.clone())
        .collect();
    let order: Vec<usize> = std::iter::once(first)
        .chain((0..targets.len()).filter(|i| *i != first && targets[*i] != id))
        .collect();
    let mut inst = Instance {
        ctx,
        q: inp.q,
        n_group: inp.n_group,
        ell: inp.ell,
        cof,
        p: gp,
        qq: primary,
        k: 0,
        n1,
        curve_tries: 0,
        extra: extras,
    };
    let r = run_sparse(&mut inst, inp.c100, inp.ic_seed, 1.25, 8, false, 2);
    out.status = r.status.clone();
    out.b = r.b;
    out.pairs = r.pairs;
    out.rounds = r.rounds;
    out.rows_fb = r.rows_fb;
    out.w_fb_mean = r.w_fb_mean;
    out.known_frac = r.known_frac;
    out.fb_setup = mt(r.setup);
    out.rel = mt(r.rel);
    out.elim_ops = r.elim_ops;
    out.elim_entries = r.elim_entries;
    out.wied_ops = r.wied_ops;
    out.bm_ops = r.bm_ops;
    out.matvecs = r.matvecs;
    out.wall_rel = r.wall_rel;
    out.wall_elim = r.wall_elim;
    out.wall_wied = r.wall_wied;
    // targets in the order of the original three; trivial ones flagged
    let mut by_orig: Vec<(Option<u64>, Mt, u64, bool, bool)> =
        vec![(None, [0; 3], 0, false, true); targets.len()];
    for (slot, &orig) in order.iter().enumerate() {
        if slot == 0 {
            by_orig[orig] = (r.found_k, mt(r.tgt), r.tgt_adds, r.verified, false);
        } else {
            if let Some(e) = r.extra_tgts.get(slot - 1) {
                by_orig[orig] = (e.0, mt(e.1), e.2, e.3, false);
            } else {
                by_orig[orig].4 = false;
            }
        }
    }
    out.tgts = by_orig;
    let _ = zero_flags;
    let fcc = inst.ctx.c.f;
    out.gae_c = measure_gae(&mut inst.ctx, &fcc);
    out
}
