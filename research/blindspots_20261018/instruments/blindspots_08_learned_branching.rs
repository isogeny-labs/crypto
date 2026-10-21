//! Blind-spot item 08: learned branching for a WDSat-style XOR/AND solver on the
//! symmetrised fourth-summation-polynomial systems `S'_4` (m = 3 summands).
//!
//! Standard library only (the `crypto` library does not compile at this branch
//! base, see `research/blindspots_20261018/08-learned-sat-branching/PROTOCOL.md`).
//! Build (each card does this):
//!   rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native \
//!         -o target/blindspots-08/<dir>/b08 examples/blindspots_08_learned_branching.rs
//!
//! Subcommands: selftest | corpus | pilot | samples | fit | cell | cnf | summarize
//! (see PROTOCOL.md for what each card runs).
//!
//! # The object
//!
//! `F = GF(2^n)`, the curve `y^2 + xy = x^3 + x^2 + 1`, the factor base
//! `V = span{1, z, .., z^(l-1)}`, a target abscissa `x_R`. The decomposition
//! question "is `x_R = x(P1 + P2 + P3)` for factor-base points" is the system
//! `S_4(x1, x2, x3, x_R) = 0` over the `3l` coordinates of `x1, x2, x3 in V`,
//! written as the WDSat corpus does (Trimoska-Ionica-Dequen,
//! `EC-Index-Calculus-Benchmarks`): the coordinates of `e1, e2, e3` (the
//! elementary symmetric functions of `x1, x2, x3`) are `9l - 3` unknowns with
//! their definition rows, and the `n` field-coordinate equations of `S_4`
//! are quadratic in them. `corpus` regenerates the committed 60 corpus
//! instances from their (modulus, x_R) and reproduces the stock WDSat
//! decision counts exactly.
//!
//! # The solver
//!
//! A native replica of Trimoska's WDSat search (`wdsat.c`, no Gaussian
//! elimination): the ANF system becomes AND-atoms (one per monomial of degree
//! >= 2, defined by `a -> x_i` and `x_1 & .. & x_d -> a`) and XOR rows over
//! variables and atoms; unit propagation runs on both to a fixpoint; the search
//! is a depth-first walk over a branching order, value 0 first, one *decision*
//! counted per node whose left branch is tried (WDSat's `conf[0]++`). The
//! candidates are the `3l` subspace coordinates (WDSat's `-g 1,..,3l`, the
//! vertex cover); the `e` variables are determined by propagation. The branching
//! rule is the only thing an arm changes. Counted: decisions `D`, propagated
//! assignments `P` (atoms included, decision literals excluded), probe
//! assignments `Q` (lookahead done by a scorer, charged in full), and `W`, the
//! unpriced native scoring work (incidence scans, model arithmetic, order
//! scans). The cost of an arm at weight `k` per unit of `W` is `D + P + Q + k*W`.

use std::collections::HashMap;
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

// ---------------------------------------------------------------- the system

const NONE: u32 = u32::MAX;

/// AND-atoms and XOR rows of one ANF system (WDSat's `dimacs_read_formula`
/// ANF reader: one atom per distinct monomial of degree >= 2).
struct Sys {
    nv: usize,
    ncore: usize, // branching candidates: the 3l subspace coordinates (the vertex cover)
    atom_vars: Vec<Vec<u32>>, // by atom index (global id = nv + index)
    var_atoms: Vec<Vec<u32>>, // by variable: global atom ids
    rows: Vec<Vec<u32>>,      // XOR rows: global ids
    rc: Vec<u8>,              // row constant: XOR of the terms equals rc
    rows_of: Vec<Vec<u32>>,   // by global id
    trivially_unsat: bool,
    kind: Vec<u8>, // 0..2: subspace block, 3: intermediate u
    pos: Vec<u16>,
    block: Vec<u16>, // size of the variable's block
}

impl Sys {
    fn nid(&self) -> usize {
        self.nv + self.atom_vars.len()
    }
    fn from_polys(eqs: &[Poly], nv: usize, l: usize, n: u32) -> Sys {
        let _ = n;
        let mut atom_of: HashMap<u64, u32> = HashMap::new();
        let mut atom_vars: Vec<Vec<u32>> = Vec::new();
        let mut var_atoms: Vec<Vec<u32>> = vec![Vec::new(); nv];
        let mut rows: Vec<Vec<u32>> = Vec::new();
        let mut rc: Vec<u8> = Vec::new();
        let mut trivially_unsat = false;
        // pass 1: atoms in order of first appearance
        for p in eqs {
            for &m in p {
                if m.count_ones() >= 2 && !atom_of.contains_key(&m) {
                    let id = (nv + atom_vars.len()) as u32;
                    atom_of.insert(m, id);
                    let vs: Vec<u32> = (0..64u32).filter(|b| (m >> b) & 1 == 1).collect();
                    for &v in &vs {
                        var_atoms[v as usize].push(id);
                    }
                    atom_vars.push(vs);
                }
            }
        }
        for p in eqs {
            let mut terms: Vec<u32> = Vec::new();
            let mut c = 0u8;
            for &m in p {
                if m == 0 {
                    c ^= 1;
                } else if m.count_ones() == 1 {
                    terms.push(m.trailing_zeros());
                } else {
                    terms.push(atom_of[&m]);
                }
            }
            if terms.is_empty() {
                if c == 1 {
                    trivially_unsat = true;
                }
                continue;
            }
            rows.push(terms);
            rc.push(c);
        }
        let nid = nv + atom_vars.len();
        let mut rows_of: Vec<Vec<u32>> = vec![Vec::new(); nid];
        for (r, t) in rows.iter().enumerate() {
            for &x in t {
                rows_of[x as usize].push(r as u32);
            }
        }
        let mut kind = vec![3u8; nv];
        let mut pos = vec![0u16; nv];
        let mut block = vec![0u16; nv];
        for v in 0..nv {
            if v < 3 * l {
                kind[v] = (v / l) as u8;
                pos[v] = (v % l) as u16;
                block[v] = l as u16;
            } else {
                pos[v] = (v - 3 * l) as u16;
                block[v] = n as u16;
            }
        }
        Sys { nv, ncore: 3 * l, atom_vars, var_atoms, rows, rc, rows_of, trivially_unsat, kind, pos, block }
    }
    fn degree_of_atom(&self, a: u32) -> usize {
        self.atom_vars[a as usize - self.nv].len()
    }
}

// ---------------------------------------------------------------- the engine

#[derive(Clone, Copy, Default, Debug)]
struct Cnt {
    dec: u64,         // decisions D (nodes whose left branch is tried)
    assigns: u64,     // every assignment in the search (decision literals included)
    dec_assigns: u64, // decision literals applied (left and right)
    q: u64,           // probe assignments Q (lookahead by a scorer)
    w: u64,           // unpriced native work W
    visits: u64,      // row/atom incidence visits during search propagation
    nsol: u64,        // models found in find-all mode
}
impl Cnt {
    fn prop(&self) -> u64 {
        self.assigns - self.dec_assigns
    }
    /// `D + P + Q`, the counted unit at weight 0 for `W`.
    fn cost0(&self) -> u64 {
        self.dec + self.prop() + self.q
    }
    fn cost(&self, k: f64) -> f64 {
        self.cost0() as f64 + k * self.w as f64
    }
}

struct St<'a> {
    s: &'a Sys,
    val: Vec<i8>,
    trail: Vec<u32>,
    un: Vec<u16>,
    par: Vec<u8>,
    queue: Vec<u32>,
    qh: usize,
    c: Cnt,
    root_ok: bool,
}

impl<'a> St<'a> {
    fn new(s: &'a Sys) -> St<'a> {
        let nid = s.nid();
        let mut st = St {
            s,
            val: vec![-1; nid],
            trail: Vec::new(),
            un: s.rows.iter().map(|r| r.len() as u16).collect(),
            par: vec![0; s.rows.len()],
            queue: Vec::new(),
            qh: 0,
            c: Cnt::default(),
            root_ok: !s.trivially_unsat,
        };
        if st.root_ok {
            for r in 0..s.rows.len() {
                if s.rows[r].len() == 1 && !st.assign(s.rows[r][0], s.rc[r] as i8) {
                    st.root_ok = false;
                    break;
                }
            }
        }
        if st.root_ok {
            st.root_ok = st.propagate();
        }
        // root propagation is common to every arm: not charged
        st.c = Cnt::default();
        st
    }
    #[inline]
    fn assign(&mut self, id: u32, v: i8) -> bool {
        let x = self.val[id as usize];
        if x >= 0 {
            return x == v;
        }
        self.val[id as usize] = v;
        self.trail.push(id);
        self.c.assigns += 1;
        let vb = v as u8;
        let s = self.s;
        for &r in &s.rows_of[id as usize] {
            self.un[r as usize] -= 1;
            self.par[r as usize] ^= vb;
        }
        self.queue.push(id);
        true
    }
    fn undo(&mut self, cp: usize) {
        let s = self.s;
        while self.trail.len() > cp {
            let id = self.trail.pop().unwrap();
            let vb = self.val[id as usize] as u8;
            for &r in &s.rows_of[id as usize] {
                self.un[r as usize] += 1;
                self.par[r as usize] ^= vb;
            }
            self.val[id as usize] = -1;
        }
        self.queue.clear();
        self.qh = 0;
    }
    fn atom_rule(&mut self, a: u32) -> bool {
        let s = self.s;
        let vs = &s.atom_vars[a as usize - s.nv];
        let mut zero = false;
        let mut unas = 0usize;
        let mut last = NONE;
        for &x in vs {
            match self.val[x as usize] {
                0 => zero = true,
                -1 => {
                    unas += 1;
                    last = x;
                }
                _ => {}
            }
        }
        if zero {
            return self.assign(a, 0);
        }
        if unas == 0 {
            return self.assign(a, 1);
        }
        let av = self.val[a as usize];
        if av == 1 {
            for &x in vs {
                if self.val[x as usize] < 0 && !self.assign(x, 1) {
                    return false;
                }
            }
        } else if av == 0 && unas == 1 {
            return self.assign(last, 0);
        }
        true
    }
    fn propagate(&mut self) -> bool {
        let s = self.s;
        while self.qh < self.queue.len() {
            let id = self.queue[self.qh];
            self.qh += 1;
            for &r in &s.rows_of[id as usize] {
                self.c.visits += 1;
                let ru = r as usize;
                let u = self.un[ru];
                if u == 0 {
                    if self.par[ru] != s.rc[ru] {
                        return false;
                    }
                } else if u == 1 {
                    let want = (s.rc[ru] ^ self.par[ru]) as i8;
                    let mut t = NONE;
                    for &x in &s.rows[ru] {
                        if self.val[x as usize] < 0 {
                            t = x;
                            break;
                        }
                    }
                    if !self.assign(t, want) {
                        return false;
                    }
                }
            }
            if (id as usize) < s.nv {
                for &a in &s.var_atoms[id as usize] {
                    self.c.visits += 1;
                    if !self.atom_rule(a) {
                        return false;
                    }
                }
            } else {
                self.c.visits += 1;
                if !self.atom_rule(id) {
                    return false;
                }
            }
        }
        self.queue.clear();
        self.qh = 0;
        true
    }
    fn decide(&mut self, v: u32, val: i8) -> bool {
        self.c.dec_assigns += 1;
        self.assign(v, val) && self.propagate()
    }
    /// Lookahead: set `v = val`, propagate, undo. Returns (conflict, assignments
    /// made including the literal). Charged to `q`, not to `assigns`.
    fn probe(&mut self, v: u32, val: i8) -> (bool, u64) {
        let cp = self.trail.len();
        let a0 = self.c.assigns;
        let vis0 = self.c.visits;
        let ok = self.assign(v, val) && self.propagate();
        let cnt = self.c.assigns - a0;
        self.undo(cp);
        self.c.assigns = a0;
        self.c.visits = vis0;
        self.c.q += cnt;
        (!ok, cnt)
    }
    fn free_vars(&self) -> usize {
        (0..self.s.ncore).filter(|&v| self.val[v] < 0).count()
    }
    fn model(&self) -> u64 {
        let mut m = 0u64;
        for v in 0..self.s.nv {
            if self.val[v] == 1 {
                m |= 1u64 << v;
            }
        }
        m
    }
}

// ---------------------------------------------------------------- features

const NF: usize = 16;
type Feat = [f64; NF];
/// Feature slots: 0..2 block one-hot, 3 unused (the `e` variables are never
/// candidates), 4 position fraction, 5/6 open quadratic/cubic atoms, 7 rows
/// holding the variable, 8 rows reached through its open atoms, 9 smallest open
/// row, 10/11 probe propagation counts, 12/13 probe conflicts, 14 free fraction,
/// 15 bias.

/// Features of the unassigned variable `v` in the current state. `full` adds
/// the two probe counts (lookahead propagation), charged to `Q`.
fn features(st: &mut St, v: u32, full: bool) -> Feat {
    let s = st.s;
    let mut f = [0.0f64; NF];
    f[s.kind[v as usize] as usize] = 1.0;
    f[4] = s.pos[v as usize] as f64 / s.block[v as usize] as f64;
    let (mut a2, mut a3, mut via) = (0u32, 0u32, 0u32);
    for &a in &s.var_atoms[v as usize] {
        st.c.w += 1;
        if st.val[a as usize] < 0 {
            if s.degree_of_atom(a) == 2 {
                a2 += 1;
            } else {
                a3 += 1;
            }
            via += s.rows_of[a as usize].len() as u32;
            st.c.w += s.rows_of[a as usize].len() as u64;
        }
    }
    f[5] = l2(a2 as f64);
    f[6] = l2(a3 as f64);
    f[7] = l2(s.rows_of[v as usize].len() as f64);
    f[8] = l2(via as f64);
    let mut minu = 99u16;
    for &r in &s.rows_of[v as usize] {
        st.c.w += 1;
        minu = minu.min(st.un[r as usize]);
    }
    f[9] = (minu.min(40)) as f64 / 10.0;
    if full {
        let (c0, n0) = st.probe(v, 0);
        let (c1, n1) = st.probe(v, 1);
        f[10] = l2(n0 as f64);
        f[11] = l2(n1 as f64);
        f[12] = c0 as u8 as f64;
        f[13] = c1 as u8 as f64;
    }
    f[14] = 0.0; // filled by the caller (needs the free count)
    f[15] = 1.0;
    f
}

// ---------------------------------------------------------------- models

#[derive(Clone, Debug)]
struct Node {
    feat: u8, // 255 = leaf
    thr: f64,
    left: u32,
    right: u32,
    val: f64,
}

#[derive(Clone, Debug)]
enum Model {
    Lin { full: bool, mean: Vec<f64>, sd: Vec<f64>, w: Vec<f64> },
    Gbt { full: bool, base: f64, lr: f64, trees: Vec<Vec<Node>> },
}

impl Model {
    fn full(&self) -> bool {
        match self {
            Model::Lin { full, .. } | Model::Gbt { full, .. } => *full,
        }
    }
    /// Predicted log2 cost of branching on the candidate first; and the number
    /// of native operations the evaluation took.
    fn predict(&self, x: &Feat) -> (f64, u64) {
        match self {
            Model::Lin { mean, sd, w, .. } => {
                let mut acc = 0.0;
                for i in 0..NF {
                    acc += w[i] * (x[i] - mean[i]) / sd[i];
                }
                (acc, NF as u64)
            }
            Model::Gbt { base, lr, trees, .. } => {
                let mut acc = *base;
                let mut ops = 0u64;
                for t in trees {
                    let mut i = 0usize;
                    loop {
                        let nd = &t[i];
                        ops += 1;
                        if nd.feat == 255 {
                            acc += lr * nd.val;
                            break;
                        }
                        i = if x[nd.feat as usize] <= nd.thr { nd.left as usize } else { nd.right as usize };
                    }
                }
                (acc, ops)
            }
        }
    }
    fn to_text(&self) -> String {
        let mut s = String::new();
        match self {
            Model::Lin { full, mean, sd, w } => {
                s += &format!("lin full={}\n", *full as u8);
                let j = |v: &Vec<f64>| v.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(",");
                s += &format!("mean {}\nsd {}\nw {}\n", j(mean), j(sd), j(w));
            }
            Model::Gbt { full, base, lr, trees } => {
                s += &format!("gbt full={} base={base:?} lr={lr:?} trees={}\n", *full as u8, trees.len());
                for t in trees {
                    s += &format!("tree {}\n", t.len());
                    for nd in t {
                        s += &format!("n {} {:?} {} {} {:?}\n", nd.feat, nd.thr, nd.left, nd.right, nd.val);
                    }
                }
            }
        }
        s
    }
    fn from_text(lines: &mut std::iter::Peekable<std::str::Lines>) -> Model {
        let head = lines.next().expect("model head");
        let kv = |h: &str, k: &str| -> String {
            h.split_whitespace()
                .find_map(|t| t.strip_prefix(&format!("{k}=")))
                .unwrap_or_else(|| panic!("model key {k}"))
                .to_string()
        };
        let vec = |l: &str| -> Vec<f64> { l.split_whitespace().nth(1).unwrap().split(',').map(|x| x.parse().unwrap()).collect() };
        if head.starts_with("lin ") {
            let full = kv(head, "full") == "1";
            let mean = vec(lines.next().unwrap());
            let sd = vec(lines.next().unwrap());
            let w = vec(lines.next().unwrap());
            Model::Lin { full, mean, sd, w }
        } else {
            let full = kv(head, "full") == "1";
            let base: f64 = kv(head, "base").parse().unwrap();
            let lr: f64 = kv(head, "lr").parse().unwrap();
            let nt: usize = kv(head, "trees").parse().unwrap();
            let mut trees = Vec::new();
            for _ in 0..nt {
                let th = lines.next().unwrap();
                let len: usize = th.split_whitespace().nth(1).unwrap().parse().unwrap();
                let mut t = Vec::new();
                for _ in 0..len {
                    let p: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
                    t.push(Node {
                        feat: p[1].parse().unwrap(),
                        thr: p[2].parse().unwrap(),
                        left: p[3].parse().unwrap(),
                        right: p[4].parse().unwrap(),
                        val: p[5].parse().unwrap(),
                    });
                }
                trees.push(t);
            }
            Model::Gbt { full, base, lr, trees }
        }
    }
}

// ---------------------------------------------------------------- policies

enum Pol<'a> {
    /// A fixed branching order (WDSat `-g`); `from` walks it like WDSat's index `l`.
    Order(Vec<u32>),
    /// Order computed once at the root from a learned scorer.
    LearnedStatic(&'a Model),
    /// Learned scorer re-evaluated at every decision (structural features).
    Dyn(&'a Model),
    /// Learned cube selection: for `depth < dcut` shortlist `k` candidates with
    /// the structural scorer, score the shortlist with the probe-feature
    /// scorer, split on the best; below the cutoff, the identity order.
    Cube { short: &'a Model, full: &'a Model, k: usize, dcut: usize },
}

fn identity(nv: usize) -> Vec<u32> {
    (0..nv as u32).collect()
}

fn feats_for(st: &mut St, v: u32, full: bool) -> Feat {
    let mut f = features(st, v, full);
    f[14] = st.free_vars() as f64 / st.s.ncore as f64;
    f
}

impl<'a> Pol<'a> {
    /// Called once on the root state.
    fn prepare(&mut self, st: &mut St) {
        if let Pol::LearnedStatic(m) = self {
            let m = *m;
            let nv = st.s.ncore;
            let mut sc: Vec<(f64, u32)> = Vec::new();
            for v in 0..nv as u32 {
                if st.val[v as usize] < 0 {
                    let f = feats_for(st, v, m.full());
                    let (p, ops) = m.predict(&f);
                    st.c.w += ops + NF as u64;
                    sc.push((p, v));
                }
            }
            sc.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));
            st.c.w += (sc.len() as f64 * (sc.len() as f64).log2().max(1.0)) as u64;
            let order: Vec<u32> = sc.into_iter().map(|x| x.1).collect();
            *self = Pol::Order(order);
        }
    }
    /// The next branching variable, or None if every variable is assigned.
    fn choose(&mut self, st: &mut St, depth: usize, from: &mut usize) -> Option<u32> {
        match self {
            Pol::Order(ord) => {
                while *from < ord.len() {
                    st.c.w += 1;
                    let v = ord[*from];
                    *from += 1;
                    if st.val[v as usize] < 0 {
                        return Some(v);
                    }
                }
                None
            }
            Pol::LearnedStatic(_) => panic!("prepare first"),
            Pol::Dyn(m) => {
                let m = *m;
                let mut best: Option<(f64, u32)> = None;
                for v in 0..st.s.ncore as u32 {
                    if st.val[v as usize] < 0 {
                        let f = feats_for(st, v, m.full());
                        let (p, ops) = m.predict(&f);
                        st.c.w += ops + NF as u64 + 1;
                        if best.map_or(true, |b| p < b.0) {
                            best = Some((p, v));
                        }
                    }
                }
                best.map(|b| b.1)
            }
            Pol::Cube { short, full, k, dcut } => {
                if depth >= *dcut {
                    let ord = identity(st.s.ncore);
                    // identity walk (same cost model as Order)
                    while *from < ord.len() {
                        st.c.w += 1;
                        let v = ord[*from];
                        *from += 1;
                        if st.val[v as usize] < 0 {
                            return Some(v);
                        }
                    }
                    return None;
                }
                let (short, full, k) = (*short, *full, *k);
                let mut sc: Vec<(f64, u32)> = Vec::new();
                for v in 0..st.s.ncore as u32 {
                    if st.val[v as usize] < 0 {
                        let f = feats_for(st, v, false);
                        let (p, ops) = short.predict(&f);
                        st.c.w += ops + NF as u64 + 1;
                        sc.push((p, v));
                    }
                }
                if sc.is_empty() {
                    return None;
                }
                sc.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));
                let mut best: Option<(f64, u32)> = None;
                for &(_, v) in sc.iter().take(k) {
                    let f = feats_for(st, v, true);
                    let (p, ops) = full.predict(&f);
                    st.c.w += ops + NF as u64 + 1;
                    if best.map_or(true, |b| p < b.0) {
                        best = Some((p, v));
                    }
                }
                best.map(|b| b.1)
            }
        }
    }
}

// ---------------------------------------------------------------- search

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Verdict {
    Sat,
    Unsat,
    Cap,
}
impl Verdict {
    fn name(&self) -> &'static str {
        match self {
            Verdict::Sat => "SAT",
            Verdict::Unsat => "UNSAT",
            Verdict::Cap => "CAP",
        }
    }
}

fn dfs(st: &mut St, pol: &mut Pol, depth: usize, from: usize, cap: u64, all: bool) -> Result<bool, ()> {
    if st.c.cost0() > cap {
        return Err(());
    }
    let mut from = from;
    let v = match pol.choose(st, depth, &mut from) {
        Some(v) => v,
        None => {
            if all {
                st.c.nsol += 1;
                return Ok(false);
            }
            return Ok(true);
        }
    };
    st.c.dec += 1;
    let cp = st.trail.len();
    if st.decide(v, 0) && dfs(st, pol, depth + 1, from, cap, all)? {
        return Ok(true);
    }
    st.undo(cp);
    if !st.decide(v, 1) {
        return Ok(false);
    }
    dfs(st, pol, depth + 1, from, cap, all)
}

struct Out {
    verdict: Verdict,
    c: Cnt,
    model: Option<u64>,
}

fn solve(sys: &Sys, pol: &mut Pol, cap: u64, all: bool) -> Out {
    let mut st = St::new(sys);
    if !st.root_ok {
        return Out { verdict: Verdict::Unsat, c: st.c, model: None };
    }
    pol.prepare(&mut st);
    let r = dfs(&mut st, pol, 0, 0, cap, all);
    match r {
        Err(()) => Out { verdict: Verdict::Cap, c: st.c, model: None },
        Ok(true) => Out { verdict: Verdict::Sat, c: st.c, model: Some(st.model()) },
        Ok(false) => {
            let v = if all && st.c.nsol > 0 { Verdict::Sat } else { Verdict::Unsat };
            Out { verdict: v, c: st.c, model: None }
        }
    }
}

// ---------------------------------------------------------------- instances

#[derive(Clone)]
struct Inst {
    n: u32,
    l: usize,
    idx: usize,
    kind: char, // 'P' planted (a decomposition exists by construction), 'R' uniform random target
    xr: u64,
    chain_sols: u64, // solutions with the intermediate point rational (DP); direct count may be larger
}

fn poly_basis(l: usize) -> Vec<u64> {
    (0..l).map(|i| 1u64 << i).collect()
}

fn rng_for(seed: u64, n: u32, l: usize, role: u64) -> Rng {
    Rng(seed ^ ((n as u64) << 48) ^ ((l as u64) << 40) ^ (role << 56) ^ 0xB08)
}

/// `np` planted targets (x_R = x(P1+P2+P3) for factor-base points, built through
/// the chain roots) then `nr` uniform random targets. One factor base per `(n, l)`.
fn gen_insts(gf: &Gf, n: u32, l: usize, seed: u64, role: u64, np: usize, nr: usize) -> Vec<Inst> {
    let mut rng = rng_for(seed, n, l, role);
    let basis = poly_basis(l);
    let span = span_of(&basis);
    let mut out = Vec::new();
    let mut idx = 0usize;
    while out.len() < np {
        let x1 = span[rng.below(span.len())];
        let x2 = span[rng.below(span.len())];
        let us = gf.roots(x1, x2);
        if us.is_empty() {
            continue;
        }
        let u = us[rng.below(us.len())];
        let x3 = span[rng.below(span.len())];
        let rs = gf.roots(u, x3);
        if rs.is_empty() {
            continue;
        }
        let xr = rs[rng.below(rs.len())];
        if xr == 0 {
            continue;
        }
        let sols = count_solutions(gf, &basis, xr);
        assert!(sols >= 1, "planted target without a solution");
        out.push(Inst { n, l, idx, kind: 'P', xr, chain_sols: sols });
        idx += 1;
    }
    while out.len() < np + nr {
        let xr = rng.next() & gf.mask;
        if xr == 0 {
            continue;
        }
        let sols = count_solutions(gf, &basis, xr);
        out.push(Inst { n, l, idx, kind: 'R', xr, chain_sols: sols });
        idx += 1;
    }
    out
}

fn build_sys(gf: &Gf, it: &Inst) -> (Sys, Vec<Poly>) {
    let (eqs, nv) = build_system(gf, &poly_basis(it.l), it.xr);
    (Sys::from_polys(&eqs, nv, it.l, it.n), eqs)
}

/// WDSat's default order restricted to the vertex cover: `-g 1,..,3l`.

/// Structure-aware static order (not learned): coordinate `t` of `e1,e2,e3`
/// depends only on bits `<= t` of the three blocks, so take bit `t` of all three
/// blocks before bit `t+1`.
fn order_interleave(l: usize) -> Vec<u32> {
    let mut o = Vec::new();
    for t in 0..l {
        for blk in 0..3 {
            o.push((blk * l + t) as u32);
        }
    }
    o
}

fn random_order(ncore: usize, seed: u64) -> Vec<u32> {
    let mut o = identity(ncore);
    Rng(seed ^ 0xD00D).shuffle(&mut o);
    o
}

fn verify_model(eqs: &[Poly], m: u64) -> bool {
    eqs.iter().all(|p| eval_poly(p, m) == 0)
}

/// Complete a core assignment (the `3l` subspace bits) with the `e` bits its
/// definition rows imply: the first `l + (2l-1) + (3l-2)` rows of the system.
fn expand_core(eqs: &[Poly], l: usize, core: u64) -> u64 {
    let mut m = core;
    for (k, row) in eqs.iter().take(6 * l - 3).enumerate() {
        if eval_poly(row, core) == 1 {
            m |= 1u64 << (3 * l + k);
        }
    }
    m
}

fn brute_core_count(eqs: &[Poly], l: usize) -> u64 {
    let mut c = 0u64;
    for core in 0..(1u64 << (3 * l)) {
        if verify_model(eqs, expand_core(eqs, l, core)) {
            c += 1;
        }
    }
    c
}

/// Number of coordinate triples with `Res = 0`, by field arithmetic.
fn direct_count(gf: &Gf, l: usize, xr: u64) -> u64 {
    let span = span_of(&poly_basis(l));
    let mut c = 0u64;
    for &x1 in &span {
        for &x2 in &span {
            for &x3 in &span {
                if s4_eval(gf, x1, x2, x3, xr) == 0 {
                    c += 1;
                }
            }
        }
    }
    c
}

/// The first-solution search with the incumbent order, for the corpus check
/// and the pilot; returns (verdict, counters, model verified).
fn run_order(sys: &Sys, eqs: &[Poly], order: Vec<u32>, cap: u64) -> (Verdict, Cnt, bool) {
    let mut p = Pol::Order(order);
    let o = solve(sys, &mut p, cap, false);
    let ok = o.model.map_or(true, |m| verify_model(eqs, m));
    (o.verdict, o.c, ok)
}

// ---------------------------------------------------------------- selftest

fn mode_selftest(_a: &Args) {
    let mut ok = true;
    let mut check = |name: &str, cond: bool| {
        println!("selftest {name}: {}", if cond { "PASS" } else { "FAIL" });
        if !cond {
            ok = false;
        }
    };
    let gf = Gf::new(31);
    let mut rng = Rng(7);
    let mut fld = true;
    let mut ident = true;
    for _ in 0..300 {
        let (a, b, c, t) = (rng.next() & gf.mask, rng.next() & gf.mask, rng.next() & gf.mask, rng.next() & gf.mask);
        fld &= gf.mul(a, b) == gf.mul(b, a) && gf.mul(a, a) == gf.sqr(a);
        fld &= gf.mul(a, b ^ c) == gf.mul(a, b) ^ gf.mul(a, c);
        fld &= gf.s3(a, b, c) == gf.s3(c, a, b);
        // the e-form of S_4 equals the resultant, and S_4 is symmetric in all four arguments
        let e1 = a ^ b ^ c;
        let e2 = gf.mul(a, b) ^ gf.mul(a, c) ^ gf.mul(b, c);
        let e3 = gf.mul(gf.mul(a, b), c);
        let pw = |k: u32| (0..k).fold(1u64, |r, _| gf.mul(r, t));
        let e3s = gf.sqr(e3);
        let f = pw(4) ^ gf.mul(pw(3), e3) ^ gf.mul(1 ^ pw(4), e3s) ^ gf.mul(pw(1), gf.mul(e3s, e3)) ^ gf.sqr(e3s)
            ^ gf.mul(pw(2), gf.sqr(e2)) ^ gf.mul(pw(3), gf.mul(gf.sqr(e2), e3)) ^ gf.mul(pw(4), gf.sqr(gf.sqr(e2)))
            ^ gf.mul(pw(1), gf.mul(gf.sqr(e1), e3)) ^ gf.mul(pw(2), gf.sqr(gf.mul(e1, e3))) ^ gf.sqr(gf.sqr(e1));
        ident &= f == s4_eval(&gf, a, b, c, t);
        ident &= s4_eval(&gf, a, b, c, t) == s4_eval(&gf, t, b, c, a);
    }
    check("field-axioms-and-s3-symmetry", fld);
    check("e-form of S_4 == resultant == symmetric in 4 arguments (300 random points, n=31)", ident);
    let mut exh = true; // engine find-all == brute force over the core == field evaluation, three orders
    let mut first_ok = true;
    let mut chain_le = true;
    let (mut sat_seen, mut unsat_seen, mut total_models) = (0, 0, 0u64);
    for &(n, l) in &[(7u32, 2usize), (9, 2), (9, 3), (11, 3), (13, 4), (15, 4)] {
        let gf = Gf::new(n);
        let insts = gen_insts(&gf, n, l, 20261018, 9, 3, 4);
        for it in &insts {
            let (sys, eqs) = build_sys(&gf, it);
            let ncore = sys.ncore;
            let brute = brute_core_count(&eqs, l);
            let direct = direct_count(&gf, l, it.xr);
            chain_le &= it.chain_sols <= direct;
            let mut counts = Vec::new();
            for mut p in [Pol::Order(identity(ncore)), Pol::Order(order_interleave(l)), Pol::Order(random_order(ncore, it.xr))] {
                counts.push(solve(&sys, &mut p, u64::MAX, true).c.nsol);
            }
            let good = counts.iter().all(|&c| c == brute) && brute == direct;
            if !good {
                println!("  n={n} l={l} idx={} brute={brute} direct={direct} chain={} engine={counts:?}", it.idx, it.chain_sols);
            }
            exh &= good;
            total_models += brute;
            if brute > 0 { sat_seen += 1 } else { unsat_seen += 1 }
            for ord in [identity(ncore), order_interleave(l)] {
                let (v, _, mok) = run_order(&sys, &eqs, ord, u64::MAX);
                first_ok &= (v == Verdict::Sat) == (brute > 0) && mok;
            }
        }
    }
    println!("  small cells: {sat_seen} satisfiable, {unsat_seen} unsatisfiable draws, {total_models} models in all");
    check("engine find-all == brute force over the core == field evaluation (3 orders)", exh);
    check("first-solution verdicts agree with the exact count and models verify (2 orders)", first_ok);
    check("chain-DP count <= direct count (degenerate solutions only add)", chain_le);
    check("both satisfiable and unsatisfiable draws seen", sat_seen > 0 && unsat_seen > 0);
    let mut rr = Rng(99);
    let xs: Vec<Feat> = (0..400)
        .map(|_| {
            let mut f = [0.0; NF];
            for i in 0..NF - 1 {
                f[i] = rr.unit();
            }
            f[15] = 1.0;
            f
        })
        .collect();
    let ys: Vec<f64> = xs.iter().map(|f| 3.0 * f[5] - 2.0 * f[7] + if f[10] > 0.5 { 1.0 } else { 0.0 }).collect();
    let lin = fit_linear(&xs, &ys, false, 1.0);
    let gbt = fit_gbt(&xs, &ys, false, 80, 3, 0.2);
    let rmse = |m: &Model| (xs.iter().zip(&ys).map(|(x, y)| (m.predict(x).0 - y).powi(2)).sum::<f64>() / xs.len() as f64).sqrt();
    let (rl, rg) = (rmse(&lin), rmse(&gbt));
    println!("  learner rmse on a synthetic law: linear {rl:.4}, gbt {rg:.4}");
    check("ridge fits a linear law", rl < 0.35);
    check("gbt fits a law with a step", rg < 0.15);
    let mut text = String::new();
    text += &lin.to_text();
    text += &gbt.to_text();
    let mut lines = text.lines().peekable();
    let lin2 = Model::from_text(&mut lines);
    let gbt2 = Model::from_text(&mut lines);
    let rt = xs.iter().all(|x| (lin.predict(x).0 - lin2.predict(x).0).abs() < 1e-9 && (gbt.predict(x).0 - gbt2.predict(x).0).abs() < 1e-9);
    check("model text round trip", rt);
    println!("selftest rss_peak_kb {}", rss_peak_kb());
    if !ok {
        fail("selftest");
    }
    println!("SELFTEST OK");
}

// ---------------------------------------------------------------- learners

fn solve_linear_system(a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for col in 0..n {
        let mut piv = col;
        for r in col + 1..n {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        a.swap(col, piv);
        b.swap(col, piv);
        let d = a[col][col];
        if d.abs() < 1e-12 {
            continue;
        }
        for r in col + 1..n {
            let f = a[r][col] / d;
            if f != 0.0 {
                for c in col..n {
                    a[r][c] -= f * a[col][c];
                }
                b[r] -= f * b[col];
            }
        }
    }
    let mut x = vec![0.0; n];
    for r in (0..n).rev() {
        let mut s = b[r];
        for c in r + 1..n {
            s -= a[r][c] * x[c];
        }
        x[r] = if a[r][r].abs() < 1e-12 { 0.0 } else { s / a[r][r] };
    }
    x
}

/// Ridge regression on standardised features (constant features keep their raw
/// value, so feature 15 acts as the intercept).
fn fit_linear(xs: &[Feat], ys: &[f64], full: bool, lambda: f64) -> Model {
    let m = xs.len().max(1) as f64;
    let mut mean = vec![0.0; NF];
    let mut sd = vec![1.0; NF];
    for i in 0..NF {
        let mu = xs.iter().map(|x| x[i]).sum::<f64>() / m;
        let var = xs.iter().map(|x| (x[i] - mu).powi(2)).sum::<f64>() / m;
        if var > 1e-12 {
            mean[i] = mu;
            sd[i] = var.sqrt();
        }
    }
    let z = |x: &Feat| -> Vec<f64> { (0..NF).map(|i| (x[i] - mean[i]) / sd[i]).collect() };
    let mut a = vec![vec![0.0; NF]; NF];
    let mut b = vec![0.0; NF];
    for (x, &y) in xs.iter().zip(ys) {
        let zx = z(x);
        for i in 0..NF {
            b[i] += zx[i] * y;
            for j in 0..NF {
                a[i][j] += zx[i] * zx[j];
            }
        }
    }
    for i in 0..NF {
        a[i][i] += lambda;
    }
    let w = solve_linear_system(&mut a, &mut b);
    Model::Lin { full, mean, sd, w }
}

fn build_tree(xs: &[Feat], res: &[f64], idx: &mut Vec<usize>, depth: usize, nodes: &mut Vec<Node>) -> u32 {
    let me = nodes.len() as u32;
    let mean = idx.iter().map(|&i| res[i]).sum::<f64>() / idx.len().max(1) as f64;
    nodes.push(Node { feat: 255, thr: 0.0, left: 0, right: 0, val: mean });
    if depth == 0 || idx.len() < 10 {
        return me;
    }
    let total: f64 = idx.iter().map(|&i| res[i]).sum();
    let n = idx.len() as f64;
    let mut best: Option<(f64, usize, f64)> = None;
    for f in 0..NF {
        let mut vals: Vec<(f64, f64)> = idx.iter().map(|&i| (xs[i][f], res[i])).collect();
        vals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if vals[0].0 == vals[vals.len() - 1].0 {
            continue;
        }
        let mut left_sum = 0.0;
        let step = (vals.len() / 24).max(1);
        for k in 0..vals.len() - 1 {
            left_sum += vals[k].1;
            if vals[k].0 == vals[k + 1].0 || (k + 1) % step != 0 {
                continue;
            }
            let nl = (k + 1) as f64;
            if nl < 4.0 || n - nl < 4.0 {
                continue;
            }
            let gain = left_sum * left_sum / nl + (total - left_sum).powi(2) / (n - nl);
            if best.map_or(true, |b| gain > b.0) {
                best = Some((gain, f, 0.5 * (vals[k].0 + vals[k + 1].0)));
            }
        }
    }
    let Some((gain, f, thr)) = best else { return me };
    if gain <= total * total / n + 1e-12 {
        return me;
    }
    let (mut li, mut ri): (Vec<usize>, Vec<usize>) = idx.iter().partition(|&&i| xs[i][f] <= thr);
    let l = build_tree(xs, res, &mut li, depth - 1, nodes);
    let r = build_tree(xs, res, &mut ri, depth - 1, nodes);
    nodes[me as usize] = Node { feat: f as u8, thr, left: l, right: r, val: 0.0 };
    me
}

fn fit_gbt(xs: &[Feat], ys: &[f64], full: bool, ntrees: usize, depth: usize, lr: f64) -> Model {
    let base = ys.iter().sum::<f64>() / ys.len().max(1) as f64;
    let mut pred = vec![base; ys.len()];
    let mut trees = Vec::new();
    for _ in 0..ntrees {
        let res: Vec<f64> = ys.iter().zip(&pred).map(|(y, p)| y - p).collect();
        let mut idx: Vec<usize> = (0..xs.len()).collect();
        let mut nodes = Vec::new();
        build_tree(xs, &res, &mut idx, depth, &mut nodes);
        for (i, x) in xs.iter().enumerate() {
            let mut k = 0usize;
            loop {
                let nd = &nodes[k];
                if nd.feat == 255 {
                    pred[i] += lr * nd.val;
                    break;
                }
                k = if x[nd.feat as usize] <= nd.thr { nd.left as usize } else { nd.right as usize };
            }
        }
        trees.push(nodes);
    }
    Model::Gbt { full, base, lr, trees }
}

// ---------------------------------------------------------------- corpus control

fn bits_le(s: &str) -> u64 {
    let mut v = 0u64;
    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            v |= 1u64 << i;
        }
    }
    v
}

struct CorpusRow {
    stem: String,
    n: u32,
    l: usize,
    low: u64,
    xr: u64,
    label: String,
    cert: Option<[u64; 3]>,
    wdsat_status: String,
    wdsat_decisions: u64,
    cms: (String, u64, u64, u64),
}

fn read_corpus(path: &str) -> Vec<CorpusRow> {
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| fail(&format!("corpus {path}: {e}")));
    let mut rows = Vec::new();
    let mut head: Vec<String> = Vec::new();
    for line in txt.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if head.is_empty() {
            head = f.iter().map(|s| s.to_string()).collect();
            continue;
        }
        let col = |name: &str| -> &str { f[head.iter().position(|h| h == name).unwrap_or_else(|| panic!("column {name}"))] };
        let n: u32 = col("n").parse().unwrap();
        let modulus = col("modulus");
        assert_eq!(modulus.len() as u32, n + 1);
        assert!(modulus.ends_with('1'));
        let cert = col("cert");
        let cert = if cert == "UNSAT" {
            None
        } else {
            let p: Vec<u64> = cert.split('-').map(bits_le).collect();
            Some([p[0], p[1], p[2]])
        };
        rows.push(CorpusRow {
            stem: col("stem").to_string(),
            n,
            l: col("l").parse().unwrap(),
            low: bits_le(&modulus[..n as usize]),
            xr: bits_le(col("xR")),
            label: col("label").to_string(),
            cert,
            wdsat_status: col("wdsat_status").to_string(),
            wdsat_decisions: col("wdsat_decisions").parse().unwrap(),
            cms: (
                col("cms_status").to_string(),
                col("cms_conflicts").parse().unwrap_or(0),
                col("cms_decisions").parse().unwrap_or(0),
                col("cms_propagations").parse().unwrap_or(0),
            ),
        });
    }
    rows
}

/// Reproduce the committed stock-WDSat decision counts of the 60 corpus
/// instances with the replica under WDSat's default order (the fidelity control).
fn mode_corpus(a: &Args) {
    let path = a.flag("--tsv").unwrap_or_else(|| fail("--tsv"));
    let only: Vec<String> = a.flag("--cells").map(|s| s.split(',').map(|x| x.to_string()).collect()).unwrap_or_default();
    let cap: u64 = a.num("--cap", 4_000_000_000);
    let rows = read_corpus(&path);
    let (mut tot, mut exact, mut verdict_ok, mut cert_ok, mut cert_n) = (0, 0, 0, 0, 0);
    for r in &rows {
        let cell = format!("n{}l{}", r.n, r.l);
        if !only.is_empty() && !only.contains(&cell) {
            continue;
        }
        let gf = Gf::with_low(r.n, r.low);
        let (eqs, nv) = build_system(&gf, &poly_basis(r.l), r.xr);
        let sys = Sys::from_polys(&eqs, nv, r.l, r.n);
        let t = std::time::Instant::now();
        let (v, c, mok) = run_order(&sys, &eqs, identity(sys.ncore), cap);
        let want_sat = r.wdsat_status == "SAT";
        let vok = (v == Verdict::Sat) == want_sat && v != Verdict::Cap;
        let dm = c.dec == r.wdsat_decisions;
        let mut cert_field = String::from("null");
        if let Some([x1, x2, x3]) = r.cert {
            cert_n += 1;
            let direct = s4_eval(&gf, x1, x2, x3, r.xr) == 0;
            let core = x1 | (x2 << r.l) | (x3 << (2 * r.l));
            let full = verify_model(&eqs, expand_core(&eqs, r.l, core));
            cert_field = format!("{}", direct && full);
            if direct && full {
                cert_ok += 1;
            }
        }
        tot += 1;
        exact += dm as usize;
        verdict_ok += vok as usize;
        println!(
            "{{\"stem\":\"{}\",\"n\":{},\"l\":{},\"label\":\"{}\",\"replica_verdict\":\"{}\",\"stock_status\":\"{}\",\"replica_decisions\":{},\"stock_decisions\":{},\"decisions_equal\":{},\"verdict_equal\":{},\"model_ok\":{},\"cert_ok\":{},\"replica_prop\":{},\"cms_decisions\":{},\"cms_conflicts\":{},\"cms_propagations\":{},\"ms\":{:.0}}}",
            r.stem, r.n, r.l, r.label, v.name(), r.wdsat_status, c.dec, r.wdsat_decisions, dm, vok, mok, cert_field,
            c.prop(), r.cms.2, r.cms.1, r.cms.3, t.elapsed().as_secs_f64() * 1000.0
        );
    }
    println!("CORPUS tot={tot} decisions_equal={exact} verdict_equal={verdict_ok} certificates_ok={cert_ok}/{cert_n}");
    if exact != tot || verdict_ok != tot || cert_ok != cert_n {
        fail("corpus control: the replica does not reproduce the stock counts");
    }
    println!("CORPUS OK");
}

// ---------------------------------------------------------------- training data

/// Complete-enumeration cost (`D + P`) of the subtree below "branch on `v`
/// first, then the incumbent order": the label of candidate `v` in this state.
fn rollout_cost(st: &mut St, v: u32, cap: u64) -> f64 {
    let ord = identity(st.s.ncore);
    let mut pol = Pol::Order(ord);
    let base = st.c;
    let cp = st.trail.len();
    st.c.dec += 1;
    let mut capped = false;
    for val in 0..2i8 {
        if st.decide(v, val) {
            if dfs(st, &mut pol, 0, 0, base.cost0() + cap, true).is_err() {
                capped = true;
            }
        }
        st.undo(cp);
        if capped {
            break;
        }
    }
    let spent = (st.c.dec - base.dec) + (st.c.prop() - base.prop());
    st.c = base;
    st.c.nsol = 0;
    (1.0 + spent as f64).log2()
}

/// Random walk of `depth` decisions from the root; None if it hits a conflict.
fn random_state<'a>(sys: &'a Sys, depth: usize, rng: &mut Rng) -> Option<St<'a>> {
    let mut st = St::new(sys);
    if !st.root_ok {
        return None;
    }
    for _ in 0..depth {
        let free: Vec<u32> = (0..sys.ncore as u32).filter(|&v| st.val[v as usize] < 0).collect();
        if free.len() <= 3 {
            return None;
        }
        let v = free[rng.below(free.len())];
        if !st.decide(v, rng.below(2) as i8) {
            return None;
        }
    }
    Some(st)
}

/// One line per (state, candidate): `state depth root label f0 .. f15` (features
/// with the probe slots filled; the structural scorers zero them at use).
fn mode_samples(a: &Args) {
    let n: u32 = a.num("--n", 23);
    let l: usize = a.num("--ell", 5);
    let np: usize = a.num("--np", 2);
    let nr: usize = a.num("--nr", 2);
    let seed: u64 = a.num("--seed", 1);
    let role: u64 = a.num("--role", 3);
    let depths: Vec<u64> = if a.flag("--depths").is_some() { a.list("--depths") } else { vec![0, 1, 2, 3, 4, 6, 8] };
    let cap: u64 = a.num("--roll-cap", 400_000_000);
    let gf = Gf::new(n);
    let insts = gen_insts(&gf, n, l, seed, role, np, nr);
    let mut rng = Rng(seed ^ 0x5A5A ^ ((n as u64) << 20));
    let mut state_id = 0usize;
    let t0 = std::time::Instant::now();
    for it in &insts {
        let (sys, _) = build_sys(&gf, it);
        for &d in &depths {
            let mut tries = 0;
            let mut st = loop {
                tries += 1;
                if let Some(s) = random_state(&sys, d as usize, &mut rng) {
                    break Some(s);
                }
                if tries > 50 {
                    break None;
                }
            };
            let Some(st) = st.as_mut() else { continue };
            let free: Vec<u32> = (0..sys.ncore as u32).filter(|&v| st.val[v as usize] < 0).collect();
            let feats: Vec<Feat> = free.iter().map(|&v| feats_for(st, v, true)).collect();
            for (k, &v) in free.iter().enumerate() {
                let y = rollout_cost(st, v, cap);
                let f = feats[k];
                let fs: Vec<String> = f.iter().map(|x| format!("{x:.6}")).collect();
                println!("{state_id} {d} {} {y:.6} {}", (d == 0) as u8, fs.join(" "));
            }
            state_id += 1;
        }
        eprintln!("samples n={n} inst={} kind={} states={} elapsed={:.1}s", it.idx, it.kind, state_id, t0.elapsed().as_secs_f64());
    }
}

struct Samples {
    // (state id, depth, root flag, label, features)
    rows: Vec<(usize, usize, bool, f64, Feat)>,
}

fn read_samples(paths: &[String]) -> Samples {
    let mut rows = Vec::new();
    let mut offset = 0usize;
    for p in paths {
        let txt = std::fs::read_to_string(p).unwrap_or_else(|e| fail(&format!("samples {p}: {e}")));
        let mut maxid = 0usize;
        for line in txt.lines() {
            let t: Vec<&str> = line.split_whitespace().collect();
            if t.len() != 4 + NF {
                continue;
            }
            let sid: usize = t[0].parse().unwrap();
            maxid = maxid.max(sid + 1);
            let mut f = [0.0; NF];
            for i in 0..NF {
                f[i] = t[4 + i].parse().unwrap();
            }
            rows.push((offset + sid, t[1].parse().unwrap(), t[2] == "1", t[3].parse().unwrap(), f));
        }
        offset += maxid;
    }
    Samples { rows }
}

fn lite(f: &Feat) -> Feat {
    let mut g = *f;
    for i in 10..14 {
        g[i] = 0.0;
    }
    g
}

/// Centre labels within each state; `shuffle` permutes the centred labels among the
/// candidates of the same state (the shuffled-label control).
fn design(s: &Samples, root_only: bool, use_lite: bool, shuffle: Option<u64>) -> (Vec<Feat>, Vec<f64>) {
    let mut by_state: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, r) in s.rows.iter().enumerate() {
        if root_only && !r.2 {
            continue;
        }
        by_state.entry(r.0).or_default().push(i);
    }
    let mut ids: Vec<usize> = by_state.keys().cloned().collect();
    ids.sort();
    let mut rng = Rng(shuffle.unwrap_or(0));
    let (mut xs, mut ys) = (Vec::new(), Vec::new());
    for sid in ids {
        let idx = &by_state[&sid];
        let mean = idx.iter().map(|&i| s.rows[i].3).sum::<f64>() / idx.len() as f64;
        let mut ylist: Vec<f64> = idx.iter().map(|&i| s.rows[i].3 - mean).collect();
        if shuffle.is_some() {
            rng.shuffle(&mut ylist);
        }
        for (k, &i) in idx.iter().enumerate() {
            xs.push(if use_lite { lite(&s.rows[i].4) } else { s.rows[i].4 });
            ys.push(ylist[k]);
        }
    }
    (xs, ys)
}

// ---------------------------------------------------------------- fitting and selection

fn eval_policy_cost(sys_list: &[(Sys, Vec<Poly>)], make: &dyn Fn() -> Pol<'static>, cap: u64, kappa: f64) -> f64 {
    // mean log2 cost over the validation instances
    let mut acc = 0.0;
    for (sys, _) in sys_list {
        let mut p = make();
        let o = solve(sys, &mut p, cap, false);
        acc += (1.0 + o.c.cost(kappa)).log2();
    }
    acc / sys_list.len() as f64
}

fn leak(m: Model) -> &'static Model {
    Box::leak(Box::new(m))
}

fn mode_fit(a: &Args) {
    let paths: Vec<String> = a.flag("--samples").unwrap_or_else(|| fail("--samples")).split(',').map(|s| s.to_string()).collect();
    let out = a.flag("--out").unwrap_or_else(|| fail("--out"));
    let seed: u64 = a.num("--seed", 1);
    let l: usize = a.num("--ell", 5);
    let val_ns: Vec<u64> = a.list("--val-ns");
    let val_p: usize = a.num("--val-p", 2);
    let val_r: usize = a.num("--val-r", 2);
    let kappa: f64 = a.num("--kappa", 0.1);
    let cap: u64 = a.num("--cap", 200_000_000);
    let s = read_samples(&paths);
    println!("fit: {} sample rows from {} files", s.rows.len(), paths.len());
    // validation instances (role 5, disjoint from the sampling roles)
    let mut val: Vec<(Sys, Vec<Poly>)> = Vec::new();
    for &n in &val_ns {
        let gf = Gf::new(n as u32);
        for it in gen_insts(&gf, n as u32, l, seed, 5, val_p, val_r) {
            val.push(build_sys(&gf, &it));
        }
    }
    println!("fit: {} validation instances", val.len());
    let mut report = String::new();
    {
        // upper bound for any static first-variable rule: the best candidate's rollout vs. the incumbent's first
        let mut by: HashMap<usize, (f64, f64, usize)> = HashMap::new(); // state -> (min, label of x1 bit 0, count)
        for r in s.rows.iter().filter(|r| r.2) {
            let e = by.entry(r.0).or_insert((f64::MAX, f64::NAN, 0));
            e.0 = e.0.min(r.3);
            if r.4[0] == 1.0 && r.4[4] == 0.0 {
                e.1 = r.3;
            }
            e.2 += 1;
        }
        let gains: Vec<f64> = by.values().filter(|v| v.1.is_finite()).map(|v| (v.1 - v.0).exp2()).collect();
        let g = gains.iter().sum::<f64>() / gains.len().max(1) as f64;
        println!("fit: root states {}, mean oracle gain of the best first variable over the incumbent's first variable: {g:.4}x", gains.len());
        report += &format!("# root oracle first-variable gain {g:.4}x over {} states\n", gains.len());
    }
    // ---- b: static, full features (root probes), ridge vs gbt
    let (xr, yr) = design(&s, true, false, None);
    let b_lin = fit_linear(&xr, &yr, true, 1.0);
    let b_gbt = fit_gbt(&xr, &yr, true, 40, 2, 0.15);
    let (bl, bg) = (leak(b_lin.clone()), leak(b_gbt.clone()));
    let c_bl = eval_policy_cost(&val, &|| Pol::LearnedStatic(bl), cap, kappa);
    let c_bg = eval_policy_cost(&val, &|| Pol::LearnedStatic(bg), cap, kappa);
    let c_id = eval_policy_cost(&val, &|| Pol::Order(identity(3 * l)), cap, kappa);
    println!("fit: validation mean log2 cost: identity {c_id:.4}  b_lin {c_bl:.4}  b_gbt {c_bg:.4}");
    report += &format!("# val identity {c_id:.4} b_lin {c_bl:.4} b_gbt {c_bg:.4}\n");
    let b = if c_bl <= c_bg { b_lin } else { b_gbt };
    // ---- b2: dynamic, structural features
    let (xd, yd) = design(&s, false, true, None);
    let d_lin = fit_linear(&xd, &yd, false, 1.0);
    let d_gbt = fit_gbt(&xd, &yd, false, 40, 3, 0.15);
    let (dl, dg) = (leak(d_lin.clone()), leak(d_gbt.clone()));
    let c_dl = eval_policy_cost(&val, &|| Pol::Dyn(dl), cap, kappa);
    let c_dg = eval_policy_cost(&val, &|| Pol::Dyn(dg), cap, kappa);
    println!("fit: validation mean log2 cost: b2_lin {c_dl:.4}  b2_gbt {c_dg:.4}");
    report += &format!("# val b2_lin {c_dl:.4} b2_gbt {c_dg:.4}\n");
    let b2 = if c_dl <= c_dg { d_lin } else { d_gbt };
    // ---- c: probe-feature rescoring model on shallow states, shortlist from b2
    let shallow = Samples { rows: s.rows.iter().filter(|r| r.1 <= 8).cloned().collect() };
    let (xf, yf) = design(&shallow, false, false, None);
    let f_lin = fit_linear(&xf, &yf, true, 1.0);
    let f_gbt = fit_gbt(&xf, &yf, true, 40, 3, 0.15);
    let b2l = leak(b2.clone());
    let mut best_c: Option<(f64, Model, usize, usize)> = None;
    for (nm, fm) in [("lin", &f_lin), ("gbt", &f_gbt)] {
        let fl = leak(fm.clone());
        for &dcut in &[2usize, 4, 6] {
            for &k in &[3usize, 5] {
                let c = eval_policy_cost(&val, &|| Pol::Cube { short: b2l, full: fl, k, dcut }, cap, kappa);
                println!("fit: validation mean log2 cost: cube full={nm} dcut={dcut} k={k} {c:.4}");
                if best_c.as_ref().map_or(true, |b| c < b.0) {
                    best_c = Some((c, fm.clone(), dcut, k));
                }
            }
        }
    }
    let (c_best, c_full, c_dcut, c_k) = best_c.unwrap();
    report += &format!("# val cube best {c_best:.4} dcut={c_dcut} k={c_k}\n");
    // ---- shuffled-label controls (same learner family and hyper-parameters as the chosen ones)
    let same_family = |m: &Model, xs: &[Feat], ys: &[f64], full: bool, depth: usize| -> Model {
        match m {
            Model::Lin { .. } => fit_linear(xs, ys, full, 1.0),
            Model::Gbt { .. } => fit_gbt(xs, ys, full, 40, depth, 0.15),
        }
    };
    let (xs_r, ys_r) = design(&s, true, false, Some(seed ^ 0xBAD));
    let bs = same_family(&b, &xs_r, &ys_r, true, 2);
    let (xs_d, ys_d) = design(&s, false, true, Some(seed ^ 0xBAD2));
    let b2s = same_family(&b2, &xs_d, &ys_d, false, 3);
    let mut txt = String::from("# b08 models (item 08)\n");
    txt += &report;
    txt += &format!("param cube_dcut {c_dcut}\nparam cube_k {c_k}\n");
    for (name, m) in [("b", &b), ("b2", &b2), ("cfull", &c_full), ("bs", &bs), ("b2s", &b2s)] {
        txt += &format!("model {name}\n{}", m.to_text());
    }
    std::fs::write(&out, txt).unwrap_or_else(|e| fail(&format!("write {out}: {e}")));
    println!("fit: wrote {out}");
}

struct ModelSet {
    b: Model,
    b2: Model,
    cfull: Model,
    bs: Model,
    b2s: Model,
    dcut: usize,
    k: usize,
}

fn read_models(path: &str) -> ModelSet {
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| fail(&format!("models {path}: {e}")));
    let mut dcut = 4usize;
    let mut k = 4usize;
    let mut map: HashMap<String, Model> = HashMap::new();
    let mut lines = txt.lines().peekable();
    while let Some(line) = lines.peek().cloned() {
        if line.starts_with('#') {
            lines.next();
        } else if let Some(v) = line.strip_prefix("param cube_dcut ") {
            dcut = v.trim().parse().unwrap();
            lines.next();
        } else if let Some(v) = line.strip_prefix("param cube_k ") {
            k = v.trim().parse().unwrap();
            lines.next();
        } else if let Some(name) = line.strip_prefix("model ") {
            let name = name.trim().to_string();
            lines.next();
            map.insert(name, Model::from_text(&mut lines));
        } else {
            lines.next();
        }
    }
    let mut g = |n: &str| map.remove(n).unwrap_or_else(|| fail(&format!("model {n} missing in {path}")));
    ModelSet { b: g("b"), b2: g("b2"), cfull: g("cfull"), bs: g("bs"), b2s: g("b2s"), dcut, k }
}

// ---------------------------------------------------------------- the measured cell

const ARMS: [&str; 8] = ["a", "a2", "b", "b2", "c", "d", "bs", "b2s"];

fn make_policy<'a>(arm: &str, sys: &Sys, l: usize, ms: &'a ModelSet, xr: u64) -> Pol<'a> {
    match arm {
        "a" => Pol::Order(identity(sys.ncore)),
        "a2" => Pol::Order(order_interleave(l)),
        "b" => Pol::LearnedStatic(&ms.b),
        "b2" => Pol::Dyn(&ms.b2),
        "c" => Pol::Cube { short: &ms.b2, full: &ms.cfull, k: ms.k, dcut: ms.dcut },
        "d" => Pol::Order(random_order(sys.ncore, xr)),
        "bs" => Pol::LearnedStatic(&ms.bs),
        "b2s" => Pol::Dyn(&ms.b2s),
        _ => fail(&format!("unknown arm {arm}")),
    }
}

fn mode_cell(a: &Args) {
    let n: u32 = a.num("--n", 31);
    let l: usize = a.num("--ell", 5);
    let np: usize = a.num("--np", 16);
    let nr: usize = a.num("--nr", 16);
    let seed: u64 = a.num("--seed", 1);
    let role: u64 = a.num("--role", 7);
    let cap: u64 = a.num("--cap", 100_000_000);
    let only: Vec<u64> = a.list("--only");
    let truth: bool = a.has("--truth");
    let arms: Vec<String> = a.flag("--arms").map(|s| s.split(',').map(|x| x.to_string()).collect()).unwrap_or_else(|| ARMS.iter().map(|s| s.to_string()).collect());
    let ms = read_models(&a.flag("--models").unwrap_or_else(|| fail("--models")));
    let cms_bin = a.flag("--cms");
    let gf = Gf::new(n);
    let insts = gen_insts(&gf, n, l, seed, role, np, nr);
    for it in insts.iter().filter(|it| only.is_empty() || only.contains(&(it.idx as u64))) {
        let (sys, eqs) = build_sys(&gf, it);
        let direct = if truth { Some(direct_count(&gf, l, it.xr)) } else { None };
        let mut verdicts: Vec<Verdict> = Vec::new();
        for arm in &arms {
            if arm == "e" {
                continue;
            }
            let mut pol = make_policy(arm, &sys, l, &ms, it.xr);
            let t = std::time::Instant::now();
            let o = solve(&sys, &mut pol, cap, false);
            let mok = o.model.map_or(true, |m| verify_model(&eqs, m));
            verdicts.push(o.verdict);
            println!(
                "{{\"n\":{n},\"l\":{l},\"idx\":{},\"kind\":\"{}\",\"xr\":{},\"chain_sols\":{},\"direct_sols\":{},\"arm\":\"{arm}\",\"verdict\":\"{}\",\"D\":{},\"P\":{},\"Q\":{},\"W\":{},\"visits\":{},\"model_ok\":{mok},\"ms\":{:.0}}}",
                it.idx, it.kind, it.xr, it.chain_sols, direct.map_or(-1i64, |d| d as i64), o.verdict.name(), o.c.dec, o.c.prop(), o.c.q, o.c.w, o.c.visits,
                t.elapsed().as_secs_f64() * 1000.0
            );
        }
        // every exact arm must agree on SAT/UNSAT (ignoring capped runs); planted targets must be SAT
        let decided: Vec<Verdict> = verdicts.iter().cloned().filter(|v| *v != Verdict::Cap).collect();
        let agree = decided.windows(2).all(|w| w[0] == w[1]);
        let planted_ok = it.kind != 'P' || decided.iter().all(|v| *v == Verdict::Sat);
        let truth_ok = direct.map_or(true, |d| decided.iter().all(|v| (*v == Verdict::Sat) == (d > 0)));
        println!("{{\"n\":{n},\"l\":{l},\"idx\":{},\"check\":\"agreement\",\"arms_agree\":{agree},\"planted_sat\":{planted_ok},\"truth_ok\":{truth_ok}}}", it.idx);
        if !(agree && planted_ok && truth_ok) {
            fail("verdict disagreement between exact arms or with the ground truth");
        }
        if let Some(bin) = &cms_bin {
            cms_run(bin, &sys, &eqs, it, a);
        }
    }
}

// ---------------------------------------------------------------- CryptoMiniSat arm

/// CNF + XOR DIMACS of the system (CryptoMiniSat input): AND-gate clauses for
/// every atom, one `x` line per XOR row (a negated first literal when the row's
/// constant is 0, since an `x` line asserts odd parity).
fn cnf_xor_text(sys: &Sys) -> String {
    let mut clauses: Vec<String> = Vec::new();
    for (ai, vs) in sys.atom_vars.iter().enumerate() {
        let a = sys.nv + ai + 1;
        for &v in vs {
            clauses.push(format!("-{a} {} 0", v + 1));
        }
        let mut c = format!("{a}");
        for &v in vs {
            c += &format!(" -{}", v + 1);
        }
        clauses.push(c + " 0");
    }
    for (r, terms) in sys.rows.iter().enumerate() {
        let mut c = String::from("x");
        for (i, &t) in terms.iter().enumerate() {
            let neg = i == 0 && sys.rc[r] == 0;
            c += &format!("{}{}{}", if i == 0 { "" } else { " " }, if neg { "-" } else { "" }, t + 1);
        }
        clauses.push(c + " 0");
    }
    let mut out = format!("p cnf {} {}\n", sys.nid(), clauses.len());
    for c in clauses {
        out += &c;
        out.push('\n');
    }
    out
}

/// CryptoMiniSat prints large counters with a K/M/G suffix; the value is rounded then.
fn cms_stat(stdout: &str, key: &str) -> u64 {
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("c ") {
            if let Some(r) = rest.trim_start().strip_prefix(key) {
                if let Some(r) = r.trim_start().strip_prefix(':') {
                    if let Some(tok) = r.split_whitespace().next() {
                        let (num, mult) = match tok.chars().last() {
                            Some('K') => (&tok[..tok.len() - 1], 1_000u64),
                            Some('M') => (&tok[..tok.len() - 1], 1_000_000),
                            Some('G') => (&tok[..tok.len() - 1], 1_000_000_000),
                            _ => (tok, 1),
                        };
                        if let Ok(v) = num.parse::<f64>() {
                            return (v * mult as f64).round() as u64;
                        }
                    }
                }
            }
        }
    }
    0
}

fn cms_run(bin: &str, sys: &Sys, eqs: &[Poly], it: &Inst, a: &Args) {
    let dir = a.flag("--tmp").unwrap_or_else(|| "target/blindspots-08/tmp".to_string());
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| fail(&format!("mkdir {dir}: {e}")));
    let path = format!("{dir}/cms-n{}-l{}-{}-{}.cnf", it.n, it.l, it.kind, it.idx);
    std::fs::write(&path, cnf_xor_text(sys)).unwrap_or_else(|e| fail(&format!("write {path}: {e}")));
    let maxtime: u64 = a.num("--cms-maxtime", 60);
    let t = std::time::Instant::now();
    let out = Command::new(bin)
        .args(["--verb", "1", "--threads", "1", "--maxtime", &maxtime.to_string(), &path])
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|e| fail(&format!("spawn {bin}: {e}")));
    let _ = std::fs::remove_file(&path);
    let so = String::from_utf8_lossy(&out.stdout).to_string();
    let verdict = if so.lines().any(|l| l.starts_with("s SATISFIABLE")) {
        "SAT"
    } else if so.lines().any(|l| l.starts_with("s UNSATISFIABLE")) {
        "UNSAT"
    } else {
        "CAP"
    };
    let mut mok = true;
    if verdict == "SAT" {
        let mut core = 0u64;
        for line in so.lines().filter(|l| l.starts_with("v ")) {
            for tok in line[2..].split_whitespace() {
                if let Ok(x) = tok.parse::<i64>() {
                    if x > 0 && (x as usize) <= sys.ncore {
                        core |= 1u64 << (x - 1);
                    }
                }
            }
        }
        mok = verify_model(eqs, expand_core(eqs, it.l, core));
    }
    let (confl, dec, prop) = (cms_stat(&so, "conflicts"), cms_stat(&so, "decisions"), cms_stat(&so, "propagations"));
    println!(
        "{{\"n\":{},\"l\":{},\"idx\":{},\"kind\":\"{}\",\"xr\":{},\"arm\":\"e\",\"verdict\":\"{verdict}\",\"D\":{dec},\"P\":{prop},\"Q\":0,\"W\":0,\"conflicts\":{confl},\"model_ok\":{mok},\"ms\":{:.0}}}",
        it.n, it.l, it.idx, it.kind, it.xr, t.elapsed().as_secs_f64() * 1000.0
    );
}

fn mode_cnf(a: &Args) {
    let n: u32 = a.num("--n", 15);
    let l: usize = a.num("--ell", 5);
    let seed: u64 = a.num("--seed", 1);
    let idx: usize = a.num("--idx", 0);
    let gf = Gf::new(n);
    let insts = gen_insts(&gf, n, l, seed, a.num("--role", 7), a.num("--np", 2), a.num("--nr", 2));
    let it = &insts[idx];
    let (sys, _) = build_sys(&gf, it);
    print!("{}", cnf_xor_text(&sys));
}

// ---------------------------------------------------------------- summary

fn jnum(line: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == '+')).unwrap_or(rest.len());
    rest[..end].parse().ok()
}
fn jstr(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":\"");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    Some(rest[..rest.find('"')?].to_string())
}

#[derive(Clone)]
struct Rec {
    n: u32,
    idx: usize,
    kind: char,
    arm: String,
    verdict: String,
    d: f64,
    p: f64,
    q: f64,
    w: f64,
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len();
    if m == 0 {
        return f64::NAN;
    }
    if m % 2 == 1 { v[m / 2] } else { 0.5 * (v[m / 2 - 1] + v[m / 2]) }
}

fn boot_median_ci(r: &[f64], seed: u64, b: usize) -> (f64, f64) {
    let mut rng = Rng(seed);
    let mut meds: Vec<f64> = Vec::with_capacity(b);
    for _ in 0..b {
        let mut s: Vec<f64> = (0..r.len()).map(|_| r[rng.below(r.len())]).collect();
        meds.push(median(&mut s));
    }
    meds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (meds[(0.025 * b as f64) as usize], meds[((0.975 * b as f64) as usize).min(b - 1)])
}

fn mode_summarize(a: &Args) {
    let files: Vec<String> = a.flag("--files").unwrap_or_else(|| fail("--files")).split(',').map(|s| s.to_string()).collect();
    let sizes: Vec<u64> = a.list("--sizes");
    let incumbent = a.flag("--incumbent").unwrap_or_else(|| "a".into());
    let cands: Vec<String> = a.flag("--cands").map(|s| s.split(',').map(|x| x.to_string()).collect()).unwrap_or_else(|| ["b", "b2", "c"].iter().map(|s| s.to_string()).collect());
    let controls: Vec<String> = a.flag("--controls").map(|s| s.split(',').map(|x| x.to_string()).collect()).unwrap_or_else(|| ["bs", "b2s", "d", "a2"].iter().map(|s| s.to_string()).collect());
    let thr: f64 = a.num("--threshold", 1.3);
    let min_pairs: usize = a.num("--min-pairs", 100);
    let boots: usize = a.num("--boots", 10000);
    let kappas = [0.0, 0.1, 1.0];
    let mut recs: Vec<Rec> = Vec::new();
    for f in &files {
        let txt = std::fs::read_to_string(f).unwrap_or_else(|e| fail(&format!("read {f}: {e}")));
        for line in txt.lines() {
            if !line.starts_with('{') || jstr(line, "arm").is_none() {
                continue;
            }
            let n = jnum(line, "n").unwrap() as u32;
            if !sizes.is_empty() && !sizes.contains(&(n as u64)) {
                continue;
            }
            recs.push(Rec {
                n,
                idx: jnum(line, "idx").unwrap() as usize,
                kind: jstr(line, "kind").unwrap().chars().next().unwrap(),
                arm: jstr(line, "arm").unwrap(),
                verdict: jstr(line, "verdict").unwrap(),
                d: jnum(line, "D").unwrap(),
                p: jnum(line, "P").unwrap(),
                q: jnum(line, "Q").unwrap_or(0.0),
                w: jnum(line, "W").unwrap_or(0.0),
            });
        }
    }
    let key = |r: &Rec| (r.n, r.idx);
    let by_arm = |arm: &str| -> HashMap<(u32, usize), &Rec> { recs.iter().filter(|r| r.arm == arm).map(|r| (key(r), r)).collect() };
    let cost = |r: &Rec, k: f64| r.d + r.p + r.q + k * r.w;
    let inc = by_arm(&incumbent);
    println!("summary: {} records, incumbent '{}' has {} targets", recs.len(), incumbent, inc.len());
    let capped = |arm: &str| recs.iter().filter(|r| r.arm == arm && r.verdict == "CAP").count();
    let mut all_arms: Vec<String> = Vec::new();
    for r in &recs {
        if !all_arms.contains(&r.arm) {
            all_arms.push(r.arm.clone());
        }
    }
    for arm in &all_arms {
        let m = by_arm(arm);
        let (sat, uns) = (m.values().filter(|r| r.verdict == "SAT").count(), m.values().filter(|r| r.verdict == "UNSAT").count());
        let mut dd: Vec<f64> = m.values().map(|r| r.d).collect();
        let mut pp: Vec<f64> = m.values().map(|r| r.p).collect();
        let mut ww: Vec<f64> = m.values().map(|r| r.w).collect();
        let mut qq: Vec<f64> = m.values().map(|r| r.q).collect();
        println!("arm {arm}: targets {} SAT {sat} UNSAT {uns} capped {}  median D {:.0} P {:.0} Q {:.0} W {:.0}", m.len(), capped(arm), median(&mut dd), median(&mut pp), median(&mut qq), median(&mut ww));
    }
    let mut verdicts: Vec<(String, String)> = Vec::new();
    let mut all_tested: Vec<(String, String)> = cands.iter().map(|c| (c.clone(), incumbent.clone())).collect();
    for c in &controls {
        all_tested.push((c.clone(), incumbent.clone()));
    }
    if all_arms.contains(&"a2".to_string()) {
        for c in &cands {
            all_tested.push((c.clone(), "a2".to_string()));
        }
    }
    for (arm, base) in &all_tested {
        if !all_arms.contains(arm) || !all_arms.contains(base) {
            continue;
        }
        let mb = by_arm(base);
        let ma = by_arm(arm);
        let mut pass_all = true;
        let mut pass_k0 = false;
        let mut regress_k0 = false;
        let mut npair = 0;
        for (ki, &k) in kappas.iter().enumerate() {
            let mut r: Vec<f64> = Vec::new();
            let mut tags: Vec<(u32, char)> = Vec::new();
            for (kk, rb) in &mb {
                if let Some(ra) = ma.get(kk) {
                    r.push(cost(rb, k) / cost(ra, k));
                    tags.push((rb.n, rb.kind));
                }
            }
            npair = r.len();
            if r.is_empty() {
                continue;
            }
            let mut rs = r.clone();
            let med = median(&mut rs);
            let (lo, hi) = boot_median_ci(&r, 20261018 + ki as u64, boots);
            let pass = med >= thr && lo > 1.0;
            pass_all &= pass;
            if ki == 0 {
                pass_k0 = pass;
                regress_k0 = hi < 1.0 && med <= 1.0 / 1.1;
            }
            let mut per: Vec<String> = Vec::new();
            let mut ns: Vec<u32> = tags.iter().map(|t| t.0).collect();
            ns.sort();
            ns.dedup();
            for n in ns {
                let mut v: Vec<f64> = r.iter().zip(&tags).filter(|(_, t)| t.0 == n).map(|(x, _)| *x).collect();
                per.push(format!("n{n}:{:.3}", median(&mut v)));
            }
            for kind in ['P', 'R'] {
                let mut v: Vec<f64> = r.iter().zip(&tags).filter(|(_, t)| t.1 == kind).map(|(x, _)| *x).collect();
                if !v.is_empty() {
                    per.push(format!("{kind}:{:.3}", median(&mut v)));
                }
            }
            println!("rule {arm} vs {base} kappa={k}: pairs {} median ratio {med:.4} CI [{lo:.4}, {hi:.4}] pass {pass}  ({})", r.len(), per.join(" "));
        }
        let word = if npair < min_pairs {
            "INCONCLUSIVE"
        } else if pass_all {
            "ADVANCES"
        } else if pass_k0 {
            "TRADES"
        } else if regress_k0 {
            "REGRESSES"
        } else {
            "MATCHES"
        };
        let frozen = cands.contains(arm) && *base == incumbent;
        println!(
            "{} {arm} vs {base}: {word} (level: constant; ratio of D+P+Q+k*W at k in 0,0.1,1; ADVANCES needs median >= {thr} with the bootstrap interval above 1 at all three)",
            if frozen { "VERDICT" } else { "DESCRIPTIVE" }
        );
        verdicts.push((format!("{arm} vs {base}"), word.to_string()));
    }
    // attribution: a shuffled-label control that passes the rule at kappa = 0 means the gain is not learning
    for (pair, word) in &verdicts {
        if pair.ends_with(&format!("vs {incumbent}")) && (word == "ADVANCES" || word == "TRADES") {
            let arm = pair.split(' ').next().unwrap();
            if cands.contains(&arm.to_string()) {
                let bad: Vec<&str> = verdicts
                    .iter()
                    .filter(|(p2, w2)| (p2.starts_with("bs ") || p2.starts_with("b2s ")) && p2.ends_with(&format!("vs {incumbent}")) && (w2 == "ADVANCES" || w2 == "TRADES"))
                    .map(|(p2, _)| p2.as_str())
                    .collect();
                println!("ATTRIBUTION {arm}: {}", if bad.is_empty() { "no shuffled-label control passes; the gain is attributable to the learned labels".to_string() } else { format!("shuffled-label control(s) {bad:?} also pass: the gain is NOT attributable to learning") });
            }
        }
    }
    let _ = verdicts;
}

// ---------------------------------------------------------------- pilot

fn mode_pilot(a: &Args) {
    let n: u32 = a.num("--n", 23);
    let l: usize = a.num("--ell", 5);
    let np: usize = a.num("--np", 4);
    let nr: usize = a.num("--nr", 4);
    let seed: u64 = a.num("--seed", 1);
    let cap: u64 = a.num("--cap", 20_000_000);
    let gf = Gf::new(n);
    let insts = gen_insts(&gf, n, l, seed, 8, np, nr);
    for it in &insts {
        let t0 = std::time::Instant::now();
        let (sys, eqs) = build_sys(&gf, it);
        let tb = t0.elapsed().as_secs_f64() * 1000.0;
        let mut line = format!("n={n} l={l} idx={} kind={} chain={} atoms={} rows={} build={tb:.0}ms", it.idx, it.kind, it.chain_sols, sys.atom_vars.len(), sys.rows.len());
        for (nm, ord) in [("core", identity(sys.ncore)), ("ilv", order_interleave(l)), ("rnd", random_order(sys.ncore, it.xr))] {
            let t = std::time::Instant::now();
            let (v, c, mok) = run_order(&sys, &eqs, ord, cap);
            line += &format!(" | {nm} {} D={} P={} W={} ok={mok} {:.0}ms", v.name(), c.dec, c.prop(), c.w, t.elapsed().as_secs_f64() * 1000.0);
        }
        println!("{line}");
    }
}

fn main() {
    let args = Args { v: std::env::args().skip(1).collect() };
    match args.v.first().map(|s| s.as_str()) {
        Some("selftest") => mode_selftest(&args),
        Some("pilot") => mode_pilot(&args),
        Some("corpus") => mode_corpus(&args),
        Some("samples") => mode_samples(&args),
        Some("fit") => mode_fit(&args),
        Some("cell") => mode_cell(&args),
        Some("cnf") => mode_cnf(&args),
        Some("summarize") => mode_summarize(&args),
        _ => fail("usage: b08 selftest|pilot|corpus|samples|fit|cell|cnf|summarize"),
    }
}
