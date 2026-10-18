//! Blind-spot item 06: a signature-based (GVW/F5) Boolean Groebner engine and
//! Joux-Vitse trace replay (F4Remake), measured against the Boolean-ring F4 of
//! item 05 on the descended chained Semaev `S_3` systems.
//!
//! Standard library only (see `research/blindspots_20261018/06-signature-gb-trace-replay/PROTOCOL.md`).
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
//!     -o target/blindspots-06/bin/b06 examples/blindspots_06_signature_gb.rs
//! b06 selftest
//! b06 xcheck  --draws <item05 draws.tsv> --ref <item05 committed grevlex lines> --cell c17-4-3
//! b06 gen     --m 3 --n 17 --ell 4 --seed 20261020 --want 17 --label c17-4-3
//! b06 run     --draws draws.tsv --cell c17-4-3 --targets 0-15 --trace-target 16 \
//!             --arms f4,sig,f4r,f4n,sigr [--time-limit S] [--total-limit S] [--touch-ratio K]
//! b06 summarize --dir raw
//! ```
//!
//! Arms (all on the same system, same refutation test):
//! * `f4`    item 05's Boolean-ring F4, grevlex (the incumbent, arm a);
//! * `sig`   the signature-based engine with criteria K0-K5 (arm b), generators in
//!           ascending degree;
//! * `sig4`  the same without the rewritable criterion K5; `sig0` with K1-K5 off (squares and
//!           singular pairs only); `sigc` the construction order of the generators;
//! * `f4r`   F4 with replay of a trace recorded on the held-out target, keeping only the pairs
//!           the trace found useful and the reducers it chose (exact F4Remake, arm c);
//!           `f4n` the same with a negative filter (drop only the pairs the trace found
//!           to reduce to zero), descriptive;
//! * `sigr`  `sig` with the same replay (arm d); `sign` its negative-filter variant;
//! * `f4rs`/`sigrs` replay of the target's own trace (the ceiling); `f4rx:<cell>` replay of
//!           another cell's trace (the fallback control).
//! `--touch-ratio K` stops a signature arm (status `touchcap`) once its touches exceed K times
//! those of `f4` on the same target; the ratio R is then bounded above by touches(f4)/touches.
//! (`sigs`, selection by signature degree, was a development arm: it needs degree-5 matrices.)

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
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

// ---------------------------------------------------------------- GF(2^n) and the chained system

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


// ---------------------------------------------------------------- hashing

#[derive(Default, Clone, Copy)]
struct Fx(u64);
impl Hasher for Fx {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x517c_c1b7_2722_0a95);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }
    fn finish(&self) -> u64 {
        let mut z = self.0;
        z ^= z >> 32;
        z = z.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        z ^ (z >> 29)
    }
}
type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<Fx>>;



#[derive(Clone, Copy)]
struct Cell {
    m: usize,
    n: u32,
    l: usize,
}
impl Cell {
    fn nv(&self) -> usize {
        self.m * self.l + (self.m - 2) * self.n as usize
    }
    fn nleaf(&self) -> usize {
        self.m * self.l
    }
}

fn standard_monomials(lm: &[u64], nv: usize) -> u64 {
    let mut c = 0u64;
    for m in 0..(1u64 << nv) {
        if !lm.iter().any(|&l| l & !m == 0) {
            c += 1;
        }
    }
    c
}

/// The leading monomials not divisible by another one.
fn minimal_lms(lm: &[u64]) -> Vec<u64> {
    let mut v: Vec<u64> = lm.to_vec();
    v.sort_unstable_by_key(|&m| key(m));
    v.dedup();
    let mut out: Vec<u64> = Vec::new();
    for &m in &v {
        if !out.iter().any(|&l| l & !m == 0) {
            out.push(m);
        }
    }
    out
}

fn brute_solutions(eqs: &[Poly], nv: usize) -> u64 {
    let mut c = 0u64;
    for pt in 0..(1u64 << nv) {
        if eqs.iter().all(|p| eval_poly(p, pt) == 0) {
            c += 1;
        }
    }
    c
}

struct Draw {
    label: String,
    idx: usize,
    cell: Cell,
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
            cell: Cell {
                m: f[2].parse().unwrap(),
                n: f[3].parse().unwrap(),
                l: f[4].parse().unwrap(),
            },
            basis: f[5].split(',').map(|x| x.parse().unwrap()).collect(),
            xr: f[6].parse().unwrap(),
            solutions: f[7].parse().unwrap(),
        });
    }
    out
}

fn random_basis(n: u32, l: usize, rng: &mut Rng) -> Vec<u64> {
    loop {
        let b: Vec<u64> = (0..l).map(|_| (rng.next() & ((1u64 << n) - 1)) | 0).collect();
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

fn mode_gen(a: &Args) {
    let m: usize = a.num("--m", 3);
    let n: u32 = a.num("--n", 17);
    let l: usize = a.num("--ell", 4);
    let seed: u64 = a.num("--seed", 1);
    let want: usize = a.num("--want", 8);
    let sat_want: usize = a.num("--sat-want", 0);
    let label = a.flag("--label").unwrap_or_else(|| format!("c{n}-{l}-{m}"));
    let gf = Gf::new(n);
    let mut rng = Rng(seed ^ ((m as u64) << 56) ^ ((n as u64) << 48) ^ ((l as u64) << 40));
    let basis = random_basis(n, l, &mut rng);
    let bs = basis.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    println!("#label\tidx\tm\tn\tell\tbasis\txr\tsolutions");
    let (mut have, mut have_sat, mut tried) = (0usize, 0usize, 0u64);
    let mut idx = 0usize;
    while have < want || have_sat < sat_want {
        tried += 1;
        let xr = rng.next() & gf.mask;
        if xr == 0 {
            continue;
        }
        let sols = count_solutions(&gf, m, &basis, xr);
        if (sols == 0 && have < want) || (sols > 0 && have_sat < sat_want) {
            println!("{label}\t{idx}\t{m}\t{n}\t{l}\t{bs}\t{xr}\t{sols}");
            idx += 1;
            if sols == 0 {
                have += 1;
            } else {
                have_sat += 1;
            }
        }
        if tried > 2_000_000 {
            eprintln!("gen: gave up after {tried} draws (unsat {have}, sat {have_sat})");
            std::process::exit(2);
        }
    }
    eprintln!("gen {label}: {tried} draws tried, {have} unsat, {have_sat} sat kept");
}


// ---------------------------------------------------------------- monomial order

/// Degree, then grevlex with variable 0 largest (the committed DegRevLex
/// convention: among equal degree the numerically smaller mask is larger).
/// `key(a) > key(b)` iff `a > b`.  At most 56 variables.
const KM: u64 = (1u64 << 56) - 1;
#[inline]
fn key(m: u64) -> u64 {
    ((m.count_ones() as u64) << 56) | (!m & KM)
}

fn grevlex_greater(a: u64, b: u64) -> bool {
    let (da, db) = (a.count_ones(), b.count_ones());
    if da != db {
        return da > db;
    }
    a < b
}

type FxSet<K> = HashSet<K, BuildHasherDefault<Fx>>;

fn sort_cancel(v: Vec<u64>) -> Vec<u64> {
    let mut kv: Vec<(u64, u64)> = v.into_iter().map(|m| (key(m), m)).collect();
    kv.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let mut out = Vec::with_capacity(kv.len());
    let mut i = 0;
    while i < kv.len() {
        let mut j = i;
        while j < kv.len() && kv[j].0 == kv[i].0 {
            j += 1;
        }
        if (j - i) & 1 == 1 {
            out.push(kv[i].1);
        }
        i = j;
    }
    out
}

/// `t * p` in the Boolean ring (descending by the order).
fn mul_mono(p: &[u64], t: u64) -> Vec<u64> {
    if t == 0 {
        return p.to_vec();
    }
    if p.iter().all(|&m| m & t == 0) {
        return p.iter().map(|&m| m | t).collect();
    }
    sort_cancel(p.iter().map(|&m| m | t).collect())
}

fn xor_merge(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    let mut ka = if a.is_empty() { 0 } else { key(a[0]) };
    let mut kb = if b.is_empty() { 0 } else { key(b[0]) };
    while i < a.len() && j < b.len() {
        if ka > kb {
            out.push(a[i]);
            i += 1;
            if i < a.len() {
                ka = key(a[i]);
            }
        } else if ka < kb {
            out.push(b[j]);
            j += 1;
            if j < b.len() {
                kb = key(b[j]);
            }
        } else {
            i += 1;
            j += 1;
            if i < a.len() {
                ka = key(a[i]);
            }
            if j < b.len() {
                kb = key(b[j]);
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

// ---------------------------------------------------------------- common engine types

/// Criteria K1-K5 on (bits 0-4), generators in ascending degree.  Bit 5 (32) keeps the construction order.
const FULL: u32 = 31;
const NONE: u32 = u32::MAX;
const NOPIV: u32 = u32::MAX;

fn next_set(acc: &[u64], pos: usize) -> Option<usize> {
    let mut w = pos >> 6;
    if w >= acc.len() {
        return None;
    }
    let mut x = acc[w] & (!0u64 << (pos & 63));
    loop {
        if x != 0 {
            return Some(w * 64 + x.trailing_zeros() as usize);
        }
        w += 1;
        if w >= acc.len() {
            return None;
        }
        x = acc[w];
    }
}

struct Limits {
    deadline: Option<Instant>,
    max_cols: usize,
    vm_cap: u64, // bytes of virtual size at which a run stops, 0 = none
    touch_cap: u64, // stop (status touchcap) when the touches exceed this, 0 = none
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Status {
    Refuted,
    Gb,
    TimeLimit,
    MemCap,
    ColsCap,
    TouchCap,
}
impl Status {
    fn name(self) -> &'static str {
        match self {
            Status::Refuted => "refuted",
            Status::Gb => "gb",
            Status::TimeLimit => "timelimit",
            Status::MemCap => "memcap",
            Status::ColsCap => "colscap",
            Status::TouchCap => "touchcap",
        }
    }
}

#[derive(Default, Clone)]
struct Totals {
    steps: u32,
    pairs_cand: u64,
    pairs_sel: u64,
    pairs_gen: u64,
    pairs_drop: u64,
    d_k0: u64,
    d_sing: u64,
    d_dup: u64,
    d_k1: u64,
    d_k2: u64,
    d_k3: u64,
    d_k4: u64,
    d_k5: u64,
    rows_s: u64,
    zero_build: u64,
    zero_elim: u64,
    super_top: u64,
    rows_max: u64,
    cols_max: u64,
    rc_max: u128,
    nnz_max: u64,
    cols_sum: u64,
    rows_sum: u64,
    build: u64,
    elim: u64,
    back: u64,
    new_elems: u64,
    d_mat: u32,
    d_sel: u32,
    g_size: usize,
    sym_tests: u64,
    mem_est_kb: u64,
    ms_sym: f64,
    ms_elim: f64,
    ms_back: f64,
    ms_add: f64,
    ms_pairs: f64,
    log_d: Vec<u32>,
    log_rows: Vec<u64>,
    log_touch: Vec<u64>,
    log_zero: Vec<u64>,
    log_new: Vec<u64>,
}
impl Totals {
    fn touches(&self) -> u64 {
        self.build + self.elim + self.back
    }
    fn zero_red(&self) -> u64 {
        self.zero_build + self.zero_elim
    }
}

/// A recorded trace (F4Remake): the pairs whose row became a new element, and
/// the reducer chosen for every monomial.
#[derive(Default, Clone)]
struct Trace {
    useful: FxSet<u64>,
    zero: FxSet<u64>,
    red: Vec<FxMap<u64, u64>>, // per matrix step: monomial -> reducer key
}

#[inline]
fn mix(a: u64, b: u64) -> u64 {
    let mut z = a.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ b.rotate_left(29).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= z >> 31;
    z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 29)
}

fn reset_hwm() -> bool {
    std::fs::write("/proc/self/clear_refs", "5").is_ok()
}

// ---------------------------------------------------------------- arm (a): the F4 of item 05

#[derive(Clone, Copy)]
struct Pair {
    a: u32,
    b: u32, // NONE: the field-equation pair of `a` with variable `var`
    lcm: u64,
    sel: u32,
    var: u8,
}

fn f4_pair_key(lm_a: u64, lm_b: Option<u64>, var: u8) -> u64 {
    match lm_b {
        None => mix(mix(lm_a, u64::MAX), var as u64),
        Some(lb) => mix(mix(lm_a.min(lb), lm_a.max(lb)), 0x1000),
    }
}

struct F4<'a> {
    lim: &'a Limits,
    g: Vec<Vec<u64>>,
    lm: Vec<u64>,
    lm_idx: FxMap<u64, u32>,
    pairs: Vec<Pair>,
    g_terms: u64,
    step_no: usize,
    t: Totals,
    rec: Option<Trace>,
    rep: Option<&'a Trace>,
    neg: bool,
}

impl<'a> F4<'a> {
    fn new(lim: &'a Limits) -> Self {
        F4 {
            lim,
            g: Vec::new(),
            lm: Vec::new(),
            lm_idx: FxMap::default(),
            pairs: Vec::new(),
            g_terms: 0,
            step_no: 0,
            t: Totals::default(),
            rec: None,
            rep: None,
            neg: false,
        }
    }

    fn over_time(&self) -> bool {
        self.lim.deadline.map_or(false, |d| Instant::now() >= d)
    }
    fn over_mem(&self) -> bool {
        self.lim.vm_cap != 0 && vm_bytes() > self.lim.vm_cap
    }

    /// Gebauer-Moeller update with the Boolean field-equation pairs.
    fn add(&mut self, p: Vec<u64>) {
        let h = self.g.len();
        let lmh = p[0];
        let mut cand: Vec<(u32, u8, u64, u32)> = (0..h)
            .map(|g| {
                let l = self.lm[g] | lmh;
                (
                    l.count_ones(),
                    if self.lm[g] & lmh == 0 { 0 } else { 1 },
                    l,
                    g as u32,
                )
            })
            .collect();
        cand.sort_unstable();
        {
            let lm = &self.lm;
            self.pairs.retain(|p| {
                if p.b == NONE {
                    return true;
                }
                let l = p.lcm;
                !(lmh & !l == 0
                    && (lm[p.a as usize] | lmh) != l
                    && (lm[p.b as usize] | lmh) != l)
            });
        }
        let mut kept: Vec<(u64, u32, bool)> = Vec::new();
        for &(_, nc, l, g) in &cand {
            let coprime = nc == 0;
            if coprime || !kept.iter().any(|&(kl, _, _)| kl & !l == 0) {
                kept.push((l, g, coprime));
            }
        }
        for &(l, g, coprime) in &kept {
            if !coprime {
                self.pairs.push(Pair {
                    a: g,
                    b: h as u32,
                    lcm: l,
                    sel: l.count_ones(),
                    var: 0,
                });
                self.t.pairs_gen += 1;
            }
        }
        let base = lmh.count_ones();
        let mut bits = lmh;
        while bits != 0 {
            let v = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            self.pairs.push(Pair {
                a: h as u32,
                b: NONE,
                lcm: lmh,
                sel: base + 1,
                var: v as u8,
            });
            self.t.pairs_gen += 1;
        }
        self.g_terms += p.len() as u64;
        self.g.push(p);
        self.lm.push(lmh);
        self.lm_idx.insert(lmh, h as u32);
    }

    fn step(&mut self) -> Result<bool, Status> {
        let min_sel = self.pairs.iter().map(|p| p.sel).min().unwrap();
        let mut selected: Vec<Pair> = Vec::new();
        self.pairs.retain(|p| {
            if p.sel == min_sel {
                selected.push(*p);
                false
            } else {
                true
            }
        });
        selected.sort_unstable_by(|x, y| {
            key(x.lcm)
                .cmp(&key(y.lcm))
                .then(x.a.cmp(&y.a))
                .then(x.b.cmp(&y.b))
                .then(x.var.cmp(&y.var))
        });
        self.t.pairs_cand += selected.len() as u64;
        let mut build = 0u64;
        let mut d_sel = self.t.d_sel;
        let mut srows: Vec<Vec<u64>> = Vec::with_capacity(selected.len());
        let mut skeys: Vec<u64> = Vec::new();
        let track = self.rec.is_some() || self.rep.is_some();
        let mut zero_at_build = 0u64;
        let mut sel_n = 0u64;
        for p in &selected {
            let pk = if track {
                if p.b == NONE {
                    f4_pair_key(self.lm[p.a as usize], None, p.var)
                } else {
                    f4_pair_key(self.lm[p.a as usize], Some(self.lm[p.b as usize]), 0)
                }
            } else {
                0
            };
            if let Some(rep) = self.rep {
                let drop = if self.neg { rep.zero.contains(&pk) } else { !rep.useful.contains(&pk) };
                if drop {
                    self.t.pairs_drop += 1;
                    continue;
                }
            }
            sel_n += 1;
            if p.b == NONE {
                d_sel = d_sel.max(p.lcm.count_ones() + 1);
                let r = mul_mono(&self.g[p.a as usize], 1u64 << p.var);
                build += r.len() as u64;
                if r.is_empty() {
                    zero_at_build += 1;
                    if let Some(rec) = self.rec.as_mut() {
                        rec.zero.insert(pk);
                    }
                } else {
                    srows.push(r);
                    skeys.push(pk);
                }
            } else {
                d_sel = d_sel.max(p.lcm.count_ones());
                let ta = p.lcm & !self.lm[p.a as usize];
                let tb = p.lcm & !self.lm[p.b as usize];
                let ra = mul_mono(&self.g[p.a as usize], ta);
                let rb = mul_mono(&self.g[p.b as usize], tb);
                build += (ra.len() + rb.len()) as u64;
                let r = xor_merge(&ra, &rb);
                if r.is_empty() {
                    zero_at_build += 1;
                    if let Some(rec) = self.rec.as_mut() {
                        rec.zero.insert(pk);
                    }
                } else {
                    srows.push(r);
                    skeys.push(pk);
                }
            }
        }
        self.t.pairs_sel += sel_n;
        self.t.rows_s += srows.len() as u64;
        self.t.d_sel = d_sel;
        self.t.zero_build += zero_at_build;
        if srows.is_empty() && self.rep.is_some() {
            return Ok(false);
        }
        let sno = self.step_no;
        let mut used_any = false;
        if !srows.is_empty() {
            self.step_no += 1;
            if let Some(rec) = self.rec.as_mut() {
                rec.red.push(FxMap::default());
            }
        }
        // symbolic preprocessing
        let t_sym = Instant::now();
        let mut idx: FxMap<u64, u32> = FxMap::default();
        let mut monos: Vec<u64> = Vec::new();
        for r in &srows {
            for &m in r {
                if !idx.contains_key(&m) {
                    idx.insert(m, monos.len() as u32);
                    monos.push(m);
                }
            }
        }
        let mut reducers: Vec<(u64, Vec<u64>)> = Vec::new();
        let mut qi = 0usize;
        while qi < monos.len() {
            let m = monos[qi];
            qi += 1;
            let mut best = usize::MAX;
            if let Some(rep) = self.rep {
                if let Some(&gi) = rep.red.get(sno).and_then(|mp| mp.get(&m)) {
                    let gi = gi as usize;
                    if gi < self.lm.len() && self.lm[gi] & !m == 0 {
                        best = gi;
                    }
                }
                self.t.sym_tests += 1;
            }
            if best == usize::MAX {
                self.t.sym_tests += self.lm.len() as u64;
                let mut bl = usize::MAX;
                for (gi, &l) in self.lm.iter().enumerate() {
                    if l & !m == 0 {
                        let len = self.g[gi].len();
                        if len < bl {
                            bl = len;
                            best = gi;
                        }
                    }
                }
            }
            if best != usize::MAX {
                if let Some(rec) = self.rec.as_mut() {
                    rec.red.last_mut().unwrap().insert(m, best as u64);
                }
                let t = m & !self.lm[best];
                let row = mul_mono(&self.g[best], t);
                build += row.len() as u64;
                for &x in &row {
                    if x != m && !idx.contains_key(&x) {
                        idx.insert(x, monos.len() as u32);
                        monos.push(x);
                    }
                }
                reducers.push((m, row));
            }
            if qi & 1023 == 0 {
                if self.over_time() {
                    return Err(Status::TimeLimit);
                }
                if self.over_mem() {
                    return Err(Status::MemCap);
                }
                if monos.len() > self.lim.max_cols {
                    return Err(Status::ColsCap);
                }
            }
        }
        let c = monos.len();
        if c > self.lim.max_cols {
            return Err(Status::ColsCap);
        }
        let d_mat = monos.iter().map(|m| m.count_ones()).max().unwrap_or(0);
        self.t.d_mat = self.t.d_mat.max(d_mat);
        let mut ks: Vec<(u64, u64)> = monos.iter().map(|&m| (key(m), m)).collect();
        ks.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let cols: Vec<u64> = ks.iter().map(|x| x.1).collect();
        drop(ks);
        let mut cidx: FxMap<u64, u32> = FxMap::default();
        cidx.reserve(c);
        for (i, &m) in cols.iter().enumerate() {
            cidx.insert(m, i as u32);
        }
        drop(idx);
        drop(monos);
        self.t.ms_sym += t_sym.elapsed().as_secs_f64() * 1000.0;
        let t_elim = Instant::now();
        let rows = (reducers.len() + srows.len()) as u64;
        let mut nnz = 0u64;
        let mut pivot: Vec<u32> = vec![NOPIV; c];
        let mut store: Vec<Vec<u32>> = Vec::with_capacity(reducers.len() + 64);
        for (m, row) in reducers.drain(..) {
            let r: Vec<u32> = row.iter().map(|x| cidx[x]).collect();
            nnz += r.len() as u64;
            pivot[cidx[&m] as usize] = store.len() as u32;
            store.push(r);
        }
        let words = (c + 63) / 64;
        let mut acc = vec![0u64; words];
        let mut elim = 0u64;
        let mut newp: Vec<u32> = Vec::new();
        let mut zero_red = 0u64;
        for (ri, r) in srows.iter().enumerate() {
            for w in acc.iter_mut() {
                *w = 0;
            }
            for &mm in r {
                let ci = cidx[&mm] as usize;
                acc[ci >> 6] ^= 1u64 << (ci & 63);
            }
            nnz += r.len() as u64;
            let mut pos = 0usize;
            let mut kept: Vec<u32> = Vec::new();
            while let Some(cc) = next_set(&acc, pos) {
                let pv = pivot[cc];
                if pv != NOPIV {
                    let prow = &store[pv as usize];
                    for &x in prow {
                        acc[(x >> 6) as usize] ^= 1u64 << (x & 63);
                    }
                    elim += prow.len() as u64;
                    pos = cc;
                } else {
                    kept.push(cc as u32);
                    pos = cc + 1;
                }
            }
            if kept.is_empty() {
                zero_red += 1;
                if let Some(rec) = self.rec.as_mut() {
                    rec.zero.insert(skeys[ri]);
                }
            } else {
                nnz += kept.len() as u64;
                if let Some(rec) = self.rec.as_mut() {
                    rec.useful.insert(skeys[ri]);
                    used_any = true;
                }
                pivot[kept[0] as usize] = store.len() as u32;
                newp.push(store.len() as u32);
                store.push(kept);
            }
            if ri & 15 == 15 {
                if self.over_time() {
                    return Err(Status::TimeLimit);
                }
                if self.over_mem() {
                    return Err(Status::MemCap);
                }
            }
        }
        self.t.ms_elim += t_elim.elapsed().as_secs_f64() * 1000.0;
        let t_back = Instant::now();
        let mut back = 0u64;
        newp.sort_unstable_by(|&x, &y| store[y as usize][0].cmp(&store[x as usize][0]));
        let is_new = {
            let mut v = vec![false; c];
            for &p in &newp {
                v[store[p as usize][0] as usize] = true;
            }
            v
        };
        for (bi, &p) in newp.iter().enumerate() {
            if bi & 63 == 63 && self.over_time() {
                return Err(Status::TimeLimit);
            }
            let row = store[p as usize].clone();
            if !row[1..].iter().any(|&x| is_new[x as usize]) {
                continue;
            }
            for w in acc.iter_mut() {
                *w = 0;
            }
            for &x in &row {
                acc[(x >> 6) as usize] ^= 1u64 << (x & 63);
            }
            let lead = row[0] as usize;
            let mut pos = lead + 1;
            let mut out: Vec<u32> = vec![row[0]];
            while let Some(cc) = next_set(&acc, pos) {
                if is_new[cc] {
                    let prow = &store[pivot[cc] as usize];
                    for &x in prow {
                        acc[(x >> 6) as usize] ^= 1u64 << (x & 63);
                    }
                    back += prow.len() as u64;
                    pos = cc;
                } else {
                    out.push(cc as u32);
                    pos = cc + 1;
                }
            }
            store[p as usize] = out;
        }
        self.t.ms_back += t_back.elapsed().as_secs_f64() * 1000.0;
        self.t.build += build;
        self.t.elim += elim;
        self.t.back += back;
        self.t.zero_elim += zero_red;
        self.t.steps += 1;
        if !used_any && !srows.is_empty() {
            if let Some(rec) = self.rec.as_mut() {
                rec.red.pop();
                self.step_no -= 1;
            }
        }
        self.t.rows_max = self.t.rows_max.max(rows);
        self.t.cols_max = self.t.cols_max.max(c as u64);
        self.t.nnz_max = self.t.nnz_max.max(nnz);
        let rc = rows as u128 * c as u128;
        self.t.rc_max = self.t.rc_max.max(rc);
        self.t.cols_sum += c as u64;
        self.t.rows_sum += rows;
        let mem = nnz * 4 + rows * 32 + c as u64 * 32 + words as u64 * 8 + self.g_terms * 8;
        self.t.mem_est_kb = self.t.mem_est_kb.max(mem / 1024);
        let t_add = Instant::now();
        let mut news: Vec<Vec<u64>> = Vec::new();
        for &p in newp.iter() {
            let row = &store[p as usize];
            news.push(row.iter().map(|&x| cols[x as usize]).collect());
        }
        news.sort_by(|a, b| key(a[0]).cmp(&key(b[0])));
        self.t.new_elems += news.len() as u64;
        let mut refuted = false;
        for p in news {
            if p[0] == 0 {
                refuted = true;
            }
            self.add(p);
        }
        self.t.ms_add += t_add.elapsed().as_secs_f64() * 1000.0;
        if std::env::var("B06_VERBOSE").is_ok() {
            eprintln!("  f4 step sel={min_sel} rows={rows} cols={c} s_rows={} new={} zero_elim={} zero_build={} G={} touches={}", self.t.rows_s, self.t.new_elems, self.t.zero_elim, self.t.zero_build, self.g.len() + 0, self.t.touches());
        }
        self.t.log_d.push(min_sel);
        self.t.log_rows.push(rows);
        self.t.log_touch.push(self.t.touches());
        self.t.log_zero.push(self.t.zero_red());
        self.t.log_new.push(self.t.new_elems);
        Ok(refuted)
    }

    fn run(&mut self, eqs: &[Poly]) -> Status {
        let mut refuted = false;
        for e in eqs {
            if e.is_empty() {
                continue;
            }
            let p = sort_cancel(e.clone());
            if p.is_empty() {
                continue;
            }
            if p[0] == 0 {
                refuted = true;
            }
            self.add(p);
        }
        let status = if refuted {
            Status::Refuted
        } else {
            loop {
                if self.pairs.is_empty() {
                    break Status::Gb;
                }
                if self.over_time() {
                    break Status::TimeLimit;
                }
                match self.step() {
                    Ok(true) => break Status::Refuted,
                    Ok(false) => {}
                    Err(s) => break s,
                }
            }
        };
        self.t.g_size = self.g.len();
        status
    }
}

// ---------------------------------------------------------------- arm (b): the signature-based engine

/// Spread the bits of `x` (below bit 56) to the even bit positions of a u128.
fn spread(x: u64) -> u128 {
    let mut v = x as u128;
    v = (v | (v << 32)) & 0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF;
    v = (v | (v << 16)) & 0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF;
    v = (v | (v << 8)) & 0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF;
    v = (v | (v << 4)) & 0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F;
    v = (v | (v << 2)) & 0x3333_3333_3333_3333_3333_3333_3333_3333;
    v = (v | (v << 1)) & 0x5555_5555_5555_5555_5555_5555_5555_5555;
    v
}

/// Order key of the monomial `m * t` of the polynomial ring (exponent of a
/// variable = [in m] + [in t], so a variable in both is a square): degree
/// first, then grevlex (a smaller exponent of the smallest variable is
/// larger).  On squarefree monomials this is `key` (checked in the selftest).
/// Larger key = larger monomial.
#[inline]
fn rk(m: u64, t: u64, nv: u32) -> u128 {
    let full = (1u64 << nv) - 1;
    let u = m | t;
    let deg = (m.count_ones() + t.count_ones()) as u128;
    let absent = !u & full; // exponent 0: digit 2
    let one = m ^ t; // exponent 1: digit 1
    (deg << 120) | (spread(absent) << 1) | spread(one)
}

struct Elem {
    poly: Vec<u64>,
    lm: u64,
    idx: u32,
    sig: u64,
}

fn ekey(idx: u32, sig: u64, lm: u64) -> u64 {
    mix(mix(idx as u64 + 1, sig), lm)
}

struct Cand {
    a: u32,
    b: u32, // NONE: field pair of `a` with `var`
    var: u8,
}

struct SRow {
    poly: Vec<u64>,
    idx: u32,
    sig: u64,
    pk: u64,
    seq: u32,
}

struct MRow {
    kind: u8, // 0 reducer, 1 S-row
    idx: u32,
    rk: u128,
    seq: u32,
    sig: u64,
    pk: u64,
    poly: Vec<u64>,
}

/// Iterate every submask of `m` (including `m` and 0).
struct Subs {
    m: u64,
    cur: u64,
    done: bool,
}
impl Subs {
    fn new(m: u64) -> Subs {
        Subs { m, cur: m, done: false }
    }
}
impl Iterator for Subs {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.done {
            return None;
        }
        let r = self.cur;
        if self.cur == 0 {
            self.done = true;
        } else {
            self.cur = (self.cur - 1) & self.m;
        }
        Some(r)
    }
}

struct Sig<'a> {
    lim: &'a Limits,
    nv: u32,
    crit: u32, // bit0 K1, bit1 K2, bit2 K3, bit3 K4
    nidx: usize,
    gen_lm: Vec<u64>,
    g: Vec<Elem>,
    un: Vec<Vec<Vec<u64>>>, // un[deg][b]: bitset over a < b of the unprocessed pairs of that selection degree
    ucnt: Vec<u64>,
    fp_by_deg: Vec<Vec<u32>>,
    lm_pc: Vec<(u32, u32)>,
    z_pc: Vec<(u32, u32)>,
    sig_pc: Vec<(u32, u32)>,
    fpl: Vec<u64>,
    lmset: Vec<FxSet<u64>>,
    zset: Vec<FxSet<u64>>,
    lm_ys: Vec<FxMap<u64, Vec<u32>>>,
    sig_ys: Vec<FxMap<u64, Vec<u32>>>,
    ekey_idx: FxMap<u64, u32>,
    lm_all: FxMap<u64, Vec<u32>>,
    seen: FxSet<u64>,
    g_terms: u64,
    step_no: usize,
    t: Totals,
    rec: Option<Trace>,
    rep: Option<&'a Trace>,
    neg: bool,
    seq: u32,
}

impl<'a> Sig<'a> {
    fn new(lim: &'a Limits, nv: u32, crit: u32) -> Self {
        assert!(nv <= 56);
        Sig {
            lim,
            nv,
            crit,
            nidx: 0,
            gen_lm: Vec::new(),
            g: Vec::new(),
            un: (0..65).map(|_| Vec::new()).collect(),
            ucnt: vec![0; 65],
            fp_by_deg: (0..65).map(|_| Vec::new()).collect(),
            lm_pc: Vec::new(),
            z_pc: Vec::new(),
            sig_pc: Vec::new(),
            fpl: Vec::new(),
            lmset: Vec::new(),
            zset: Vec::new(),
            lm_ys: Vec::new(),
            sig_ys: Vec::new(),
            ekey_idx: FxMap::default(),
            lm_all: FxMap::default(),
            seen: FxSet::default(),
            g_terms: 0,
            step_no: 0,
            t: Totals::default(),
            rec: None,
            rep: None,
            neg: false,
            seq: 0,
        }
    }

    fn over_time(&self) -> bool {
        self.lim.deadline.map_or(false, |d| Instant::now() >= d)
    }
    fn over_mem(&self) -> bool {
        self.lim.vm_cap != 0 && vm_bytes() > self.lim.vm_cap
    }

    fn add_elem(&mut self, poly: Vec<u64>, idx: u32, sig: u64) {
        let h = self.g.len();
        let lm = poly[0];
        self.g_terms += poly.len() as u64;
        self.g.push(Elem { poly, lm, idx, sig });
        // the unprocessed pairs of h with every older element, bucketed by selection degree
        let words = (h + 63) / 64;
        let mut local: Vec<Vec<u64>> = (0..65).map(|_| Vec::new()).collect();
        for a in 0..h {
            let d = self.pair_deg(a, h) as usize;
            if local[d].is_empty() {
                local[d] = vec![0u64; words];
            }
            local[d][a >> 6] |= 1u64 << (a & 63);
            self.ucnt[d] += 1;
        }
        for d in 0..65 {
            let v = std::mem::take(&mut local[d]);
            self.un[d].push(v);
        }
        self.fpl.push(lm & !sig);
        if lm & !sig != 0 {
            let fd = self.field_deg(h) as usize;
            self.fp_by_deg[fd].push(h as u32);
        }
        self.t.d_k0 += (lm & sig).count_ones() as u64;
        let pc = lm.count_ones();
        for j in (idx as usize + 1)..self.nidx {
            self.lmset[j].insert(lm);
            let r = &mut self.lm_pc[j];
            *r = (r.0.min(pc), r.1.max(pc));
        }
        self.lm_ys[idx as usize].entry(lm).or_default().push(h as u32);
        self.sig_ys[idx as usize].entry(sig).or_default().push(h as u32);
        let sp = sig.count_ones();
        let r = &mut self.sig_pc[idx as usize];
        *r = (r.0.min(sp), r.1.max(sp));
        self.ekey_idx.insert(ekey(idx, sig, lm), h as u32);
        self.lm_all.entry(lm).or_default().push(h as u32);
    }

    fn add_syz(&mut self, i: usize, s: u64) {
        self.zset[i].insert(s);
        let pc = s.count_ones();
        let r = &mut self.z_pc[i];
        *r = (r.0.min(pc), r.1.max(pc));
    }

    fn pair_key(&self, c: &Cand) -> u64 {
        let ea = &self.g[c.a as usize];
        let ka = ekey(ea.idx, ea.sig, ea.lm);
        if c.b == NONE {
            mix(mix(ka, u64::MAX), c.var as u64)
        } else {
            let eb = &self.g[c.b as usize];
            let kb = ekey(eb.idx, eb.sig, eb.lm);
            mix(mix(ka.min(kb), ka.max(kb)), 0x1000)
        }
    }

    /// K3: the principal syzygy of the element `x` with an element `y` of the
    /// same index has a leading term dividing `s`.
    fn k3_hit(&self, x: usize, i: usize, s: u64) -> bool {
        let nv = self.nv;
        let ex = &self.g[x];
        let (lmx, sx) = (ex.lm, ex.sig);
        // direction 1: leading term LM(y) * sig(x)
        if sx & !s == 0 {
            let u = s & !sx;
            for sub in Subs::new(u) {
                if sub == 0 {
                    continue;
                }
                if let Some(ys) = self.lm_ys[i].get(&sub) {
                    for &y in ys {
                        if y as usize == x {
                            continue;
                        }
                        let ey = &self.g[y as usize];
                        if rk(ey.lm, sx, nv) > rk(lmx, ey.sig, nv) {
                            return true;
                        }
                    }
                }
            }
        }
        // direction 2: leading term LM(x) * sig(y)
        if lmx & !s == 0 {
            let u = s & !lmx;
            for sub in Subs::new(u) {
                if let Some(ys) = self.sig_ys[i].get(&sub) {
                    for &y in ys {
                        if y as usize == x {
                            continue;
                        }
                        let ey = &self.g[y as usize];
                        if rk(lmx, ey.sig, nv) > rk(ey.lm, sx, nv) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Apply K1-K4 to the label `(i, s)` of a pair whose larger side is `x`
    /// and whose other side is `y` (an element index, or none for a field
    /// pair).  Counts the criterion that fired.
    fn discarded(&mut self, i: usize, s: u64, x: usize, y: Option<usize>) -> bool {
        if self.crit & 1 != 0 && self.gen_lm[i] & !s == 0 {
            self.t.d_k1 += 1;
            return true;
        }
        if self.crit & 2 != 0 && i > 0 && {
            let (lo, hi) = self.lm_pc[i];
            Subs::new(s).any(|u| {
                let pc = u.count_ones();
                pc >= lo && pc <= hi && u != 0 && self.lmset[i].contains(&u)
            })
        } {
            self.t.d_k2 += 1;
            return true;
        }
        if self.crit & 8 != 0 && !self.zset[i].is_empty() && {
            let (lo, hi) = self.z_pc[i];
            Subs::new(s).any(|u| {
                let pc = u.count_ones();
                pc >= lo && pc <= hi && self.zset[i].contains(&u)
            })
        } {
            self.t.d_k4 += 1;
            return true;
        }
        if self.crit & 4 != 0 {
            let mut hit = self.k3_hit(x, i, s);
            if !hit {
                if let Some(yy) = y {
                    if self.g[yy].idx as usize == i {
                        hit = self.k3_hit(yy, i, s);
                    }
                }
            }
            if hit {
                self.t.d_k3 += 1;
                return true;
            }
        }
        // K5 (rewritable): a newer element of the same index has a signature dividing s
        if self.crit & 16 != 0 {
            let xx = x as u32;
            let (lo, hi) = self.sig_pc[i];
            if Subs::new(s).any(|u| {
                let pc = u.count_ones();
                pc >= lo && pc <= hi && self.sig_ys[i].get(&u).map_or(false, |ys| *ys.last().unwrap() > xx)
            }) {
                self.t.d_k5 += 1;
                return true;
            }
        }
        false
    }

    /// Selection degree of the pair (a, b): the degree of the lcm (F4 normal strategy), or with
    /// crit bit 6 the degree of the pair signature (as a polynomial-ring monomial).
    #[inline]
    fn pair_deg(&self, a: usize, b: usize) -> u32 {
        let (ea, eb) = (&self.g[a], &self.g[b]);
        let lcm = ea.lm | eb.lm;
        if self.crit & 64 == 0 {
            return lcm.count_ones();
        }
        let da = ea.sig.count_ones() + (lcm & !ea.lm).count_ones();
        let db = eb.sig.count_ones() + (lcm & !eb.lm).count_ones();
        da.max(db)
    }
    #[inline]
    fn field_deg(&self, b: usize) -> u32 {
        if self.crit & 64 == 0 {
            self.g[b].lm.count_ones() + 1
        } else {
            self.g[b].sig.count_ones() + 1
        }
    }

    fn min_degree(&self) -> Option<u32> {
        (0..65u32).find(|&d| self.ucnt[d as usize] > 0 || !self.fp_by_deg[d as usize].is_empty())
    }

    fn make_row(&mut self, c: &Cand) -> Option<SRow> {
        let nv = self.nv;
        if c.b == NONE {
            // field pair: label x * sig(a), squarefree because K0 removed x in sig(a)
            let a = c.a as usize;
            let (i, s) = (self.g[a].idx as usize, self.g[a].sig | (1u64 << c.var));
            if self.discarded(i, s, a, None) {
                return None;
            }
            let r = mul_mono(&self.g[a].poly, 1u64 << c.var);
            self.t.build += r.len() as u64;
            if r.is_empty() {
                self.t.zero_build += 1;
                self.add_syz(i, s);
                if self.rec.is_some() {
                    let pk = self.pair_key(c);
                    self.rec.as_mut().unwrap().zero.insert(pk);
                }
                return None;
            }
            self.seq += 1;
            let pk = if self.rec.is_some() || self.rep.is_some() { self.pair_key(c) } else { 0 };
            return Some(SRow { poly: r, idx: i as u32, sig: s, pk, seq: self.seq });
        }
        let (a, b) = (c.a as usize, c.b as usize);
        let (la, lb, sa, sb, ia, ib) = (
            self.g[a].lm,
            self.g[b].lm,
            self.g[a].sig,
            self.g[b].sig,
            self.g[a].idx,
            self.g[b].idx,
        );
        let lcm = la | lb;
        let ta = lcm & !la;
        let tb = lcm & !lb;
        let ord = if ia != ib {
            ia.cmp(&ib)
        } else {
            rk(sa, ta, nv).cmp(&rk(sb, tb, nv))
        };
        if ord == std::cmp::Ordering::Equal {
            self.t.d_sing += 1;
            return None;
        }
        let (x, tx, y, ty) = if ord == std::cmp::Ordering::Greater { (a, ta, b, tb) } else { (b, tb, a, ta) };
        let (sx, ix) = (self.g[x].sig, self.g[x].idx as usize);
        if sx & tx != 0 {
            self.t.d_k0 += 1;
            return None;
        }
        let s = sx | tx;
        if self.discarded(ix, s, x, Some(y)) {
            return None;
        }
        // F5/GVW formulation: the row is the larger-signature multiple t_x * X alone;
        // the smaller multiple is cancelled in elimination by the reducer of least label.
        if !self.seen.insert(mix(x as u64 + 1, tx)) {
            self.t.d_dup += 1;
            return None;
        }
        let r = mul_mono(&self.g[x].poly, tx);
        let _ = (y, ty);
        self.t.build += r.len() as u64;
        self.seq += 1;
        let pk = if self.rec.is_some() || self.rep.is_some() { self.pair_key(c) } else { 0 };
        Some(SRow { poly: r, idx: ix as u32, sig: s, pk, seq: self.seq })
    }

    /// Repair pass (PROTOCOL.md deviation 15): the candidate list of a step can grow past the
    /// address-space limit in one reallocation (Vec doubling), which aborts the process before
    /// the periodic memory check runs. Before a reallocation, stop with `memcap` when the doubled
    /// buffer would not fit under the cap. Changes no counted quantity of a job that finishes.
    #[inline]
    fn grow_ok(&self, cands: &Vec<Cand>) -> Result<(), Status> {
        if cands.len() == cands.capacity() && self.lim.vm_cap != 0 {
            let need = (cands.capacity().max(4) * 2 * std::mem::size_of::<Cand>()) as u64;
            if vm_bytes() + need > self.lim.vm_cap {
                return Err(Status::MemCap);
            }
        }
        Ok(())
    }

    fn step(&mut self, d: u32) -> Result<bool, Status> {
        let t_pairs = Instant::now();
        self.seen.clear();
        // collect the unprocessed pairs of degree d
        let mut cands: Vec<Cand> = Vec::new();
        let du = d as usize;
        for b in std::mem::take(&mut self.fp_by_deg[du]) {
            let f = self.fpl[b as usize];
            let mut bits = f;
            while bits != 0 {
                let v = bits.trailing_zeros() as u8;
                bits &= bits - 1;
                self.grow_ok(&cands)?;
                cands.push(Cand { a: b, b: NONE, var: v });
            }
            self.fpl[b as usize] = 0;
        }
        for b in 0..self.un[du].len() {
            let words = std::mem::take(&mut self.un[du][b]);
            for (w, &word) in words.iter().enumerate() {
                let mut x = word;
                while x != 0 {
                    let a = w * 64 + x.trailing_zeros() as usize;
                    x &= x - 1;
                    self.grow_ok(&cands)?;
                    cands.push(Cand { a: a as u32, b: b as u32, var: 0 });
                }
            }
        }
        self.ucnt[du] = 0;
        self.t.pairs_cand += cands.len() as u64;
        self.t.d_sel = self.t.d_sel.max(d);
        let mut srows: Vec<SRow> = Vec::new();
        let mut sel_n = 0u64;
        for (ci, c) in cands.iter().enumerate() {
            if let Some(rep) = self.rep {
                let pk = self.pair_key(c);
                let drop = if self.neg { rep.zero.contains(&pk) } else { !rep.useful.contains(&pk) };
                if drop {
                    self.t.pairs_drop += 1;
                    continue;
                }
            }
            sel_n += 1;
            if let Some(r) = self.make_row(c) {
                srows.push(r);
            }
            if ci & 4095 == 4095 {
                if self.over_time() {
                    return Err(Status::TimeLimit);
                }
                if self.over_mem() {
                    return Err(Status::MemCap);
                }
            }
        }
        drop(cands);
        self.t.pairs_sel += sel_n;
        self.t.rows_s += srows.len() as u64;
        self.t.ms_pairs += t_pairs.elapsed().as_secs_f64() * 1000.0;
        if srows.is_empty() {
            return Ok(false);
        }
        // matrix steps that contribute no new element are not numbered in a trace, so that a
        // replay (which skips such steps) stays aligned with its trace
        let sno = self.step_no;
        self.step_no += 1;
        let mut used_any = false;
        if let Some(rec) = self.rec.as_mut() {
            rec.red.push(FxMap::default());
        }
        let nv = self.nv;
        // symbolic preprocessing: the reducer of a monomial is the divisor of least label
        let t_sym = Instant::now();
        let mut build = 0u64;
        let mut idx: FxMap<u64, u32> = FxMap::default();
        let mut monos: Vec<u64> = Vec::new();
        for r in &srows {
            for &m in &r.poly {
                if !idx.contains_key(&m) {
                    idx.insert(m, monos.len() as u32);
                    monos.push(m);
                }
            }
        }
        let mut reducers: Vec<MRow> = Vec::new();
        let mut qi = 0usize;
        while qi < monos.len() {
            let m = monos[qi];
            qi += 1;
            let mut best = usize::MAX;
            if let Some(rep) = self.rep {
                if let Some(&ek) = rep.red.get(sno).and_then(|mp| mp.get(&m)) {
                    if let Some(&gi) = self.ekey_idx.get(&ek) {
                        if self.g[gi as usize].lm & !m == 0 {
                            best = gi as usize;
                        }
                    }
                }
                self.t.sym_tests += 1;
            }
            if best == usize::MAX {
                // every divisor of m among the leading monomials: look up the submasks of m
                let mut bl: (u32, u128, usize) = (u32::MAX, u128::MAX, usize::MAX);
                for u in Subs::new(m) {
                    self.t.sym_tests += 1;
                    if let Some(gs) = self.lm_all.get(&u) {
                        for &gi in gs {
                            let e = &self.g[gi as usize];
                            let t = m & !e.lm;
                            let cand = (e.idx, rk(e.sig, t, nv), e.poly.len());
                            if cand < bl {
                                bl = cand;
                                best = gi as usize;
                            }
                        }
                    }
                }
            }
            if best != usize::MAX {
                let e = &self.g[best];
                if let Some(rec) = self.rec.as_mut() {
                    rec.red.last_mut().unwrap().insert(m, ekey(e.idx, e.sig, e.lm));
                }
                let t = m & !e.lm;
                let row = mul_mono(&e.poly, t);
                build += row.len() as u64;
                for &x in &row {
                    if x != m && !idx.contains_key(&x) {
                        idx.insert(x, monos.len() as u32);
                        monos.push(x);
                    }
                }
                self.seq += 1;
                reducers.push(MRow {
                    kind: 0,
                    idx: e.idx,
                    rk: rk(e.sig, t, nv),
                    seq: self.seq,
                    sig: e.sig | t,
                    pk: 0,
                    poly: row,
                });
            }
            if qi & 1023 == 0 {
                if self.over_time() {
                    return Err(Status::TimeLimit);
                }
                if self.over_mem() {
                    return Err(Status::MemCap);
                }
                if monos.len() > self.lim.max_cols {
                    return Err(Status::ColsCap);
                }
            }
        }
        let c = monos.len();
        if c > self.lim.max_cols {
            return Err(Status::ColsCap);
        }
        let d_mat = monos.iter().map(|m| m.count_ones()).max().unwrap_or(0);
        self.t.d_mat = self.t.d_mat.max(d_mat);
        let mut ks: Vec<(u64, u64)> = monos.iter().map(|&m| (key(m), m)).collect();
        ks.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let cols: Vec<u64> = ks.iter().map(|x| x.1).collect();
        drop(ks);
        let mut cidx: FxMap<u64, u32> = FxMap::default();
        cidx.reserve(c);
        for (i, &m) in cols.iter().enumerate() {
            cidx.insert(m, i as u32);
        }
        drop(idx);
        drop(monos);
        // all rows in increasing label order
        let mut mrows: Vec<MRow> = reducers;
        for r in srows.drain(..) {
            mrows.push(MRow {
                kind: 1,
                idx: r.idx,
                rk: rk(r.sig, 0, nv),
                seq: r.seq,
                sig: r.sig,
                pk: r.pk,
                poly: r.poly,
            });
        }
        mrows.sort_unstable_by(|x, y| {
            x.idx
                .cmp(&y.idx)
                .then(x.rk.cmp(&y.rk))
                .then(x.kind.cmp(&y.kind))
                .then(x.seq.cmp(&y.seq))
        });
        self.t.ms_sym += t_sym.elapsed().as_secs_f64() * 1000.0;
        let t_elim = Instant::now();
        let rows = mrows.len() as u64;
        let mut nnz = 0u64;
        let words = (c + 63) / 64;
        let mut acc = vec![0u64; words];
        // pivots: several per column are allowed (a raw reducer and a reduced S-row can
        // share a leading column); a row uses the sparsest pivot of strictly smaller label
        let mut pivot: Vec<u32> = vec![NOPIV; c];
        let mut chain: Vec<u32> = Vec::with_capacity(mrows.len());
        let mut store: Vec<Vec<u32>> = Vec::with_capacity(mrows.len());
        let mut slab: Vec<(u32, u128)> = Vec::with_capacity(mrows.len());
        let mut newpiv: Vec<u32> = vec![NOPIV; c];
        let mut elim = 0u64;
        // (store index, idx, sig, rk, seq)
        let mut newp: Vec<(u32, u32, u64, u128, u32)> = Vec::new();
        for (ri, row) in mrows.iter().enumerate() {
            let lab = (row.idx, row.rk);
            let r: Vec<u32> = row.poly.iter().map(|x| cidx[x]).collect();
            if row.kind == 0 {
                nnz += r.len() as u64;
                let col = r[0] as usize;
                chain.push(pivot[col]);
                pivot[col] = store.len() as u32;
                store.push(r);
                slab.push(lab);
                continue;
            }
            for w in acc.iter_mut() {
                *w = 0;
            }
            for &ci in &r {
                acc[(ci >> 6) as usize] ^= 1u64 << (ci & 63);
            }
            nnz += r.len() as u64;
            let mut pos = 0usize;
            let mut kept: Vec<u32> = Vec::new();
            let mut dropped = false;
            while let Some(cc) = next_set(&acc, pos) {
                let mut best = NOPIV;
                let mut bl = usize::MAX;
                let mut any = false;
                let mut p = pivot[cc];
                while p != NOPIV {
                    any = true;
                    if slab[p as usize] < lab {
                        let l = store[p as usize].len();
                        if l < bl {
                            bl = l;
                            best = p;
                        }
                    }
                    p = chain[p as usize];
                }
                if best != NOPIV {
                    let prow = &store[best as usize];
                    for &x in prow {
                        acc[(x >> 6) as usize] ^= 1u64 << (x & 63);
                    }
                    elim += prow.len() as u64;
                    pos = cc;
                } else if any && kept.is_empty() {
                    dropped = true;
                    break;
                } else {
                    kept.push(cc as u32);
                    pos = cc + 1;
                }
            }
            if dropped {
                self.t.super_top += 1;
            } else if kept.is_empty() {
                self.t.zero_elim += 1;
                self.add_syz(row.idx as usize, row.sig);
                if let Some(rec) = self.rec.as_mut() {
                    rec.zero.insert(row.pk);
                }
            } else {
                nnz += kept.len() as u64;
                let col = kept[0] as usize;
                chain.push(pivot[col]);
                pivot[col] = store.len() as u32;
                newpiv[col] = store.len() as u32;
                newp.push((store.len() as u32, row.idx, row.sig, row.rk, row.seq));
                if let Some(rec) = self.rec.as_mut() {
                    rec.useful.insert(row.pk);
                    used_any = true;
                }
                store.push(kept);
                slab.push(lab);
            }
            if ri & 15 == 15 {
                // an abort still charges the work of the unfinished matrix
                let abort = if self.over_time() {
                    Some(Status::TimeLimit)
                } else if self.over_mem() {
                    Some(Status::MemCap)
                } else if self.lim.touch_cap != 0 && self.t.touches() + build + elim > self.lim.touch_cap {
                    Some(Status::TouchCap)
                } else {
                    None
                };
                if let Some(st) = abort {
                    self.t.build += build;
                    self.t.elim += elim;
                    return Err(st);
                }
            }
        }
        self.t.ms_elim += t_elim.elapsed().as_secs_f64() * 1000.0;
        let t_back = Instant::now();
        let mut back = 0u64;
        let mut order: Vec<usize> = (0..newp.len()).collect();
        order.sort_unstable_by(|&x, &y| store[newp[y].0 as usize][0].cmp(&store[newp[x].0 as usize][0]));
        for (bi, &oi) in order.iter().enumerate() {
            let p = newp[oi].0 as usize;
            if bi & 63 == 63 && self.over_time() {
                return Err(Status::TimeLimit);
            }
            let lab = slab[p];
            let row = store[p].clone();
            let blocked = |x: u32, slab: &Vec<(u32, u128)>| {
                let np = newpiv[x as usize];
                np != NOPIV && slab[np as usize] < lab
            };
            if !row[1..].iter().any(|&x| blocked(x, &slab)) {
                continue;
            }
            for w in acc.iter_mut() {
                *w = 0;
            }
            for &x in &row {
                acc[(x >> 6) as usize] ^= 1u64 << (x & 63);
            }
            let lead = row[0] as usize;
            let mut pos = lead + 1;
            let mut out: Vec<u32> = vec![row[0]];
            while let Some(cc) = next_set(&acc, pos) {
                if blocked(cc as u32, &slab) {
                    let prow = &store[newpiv[cc] as usize];
                    for &x in prow {
                        acc[(x >> 6) as usize] ^= 1u64 << (x & 63);
                    }
                    back += prow.len() as u64;
                    pos = cc;
                } else {
                    out.push(cc as u32);
                    pos = cc + 1;
                }
            }
            store[p] = out;
        }
        self.t.ms_back += t_back.elapsed().as_secs_f64() * 1000.0;
        self.t.build += build;
        self.t.elim += elim;
        self.t.back += back;
        self.t.steps += 1;
        if !used_any {
            if let Some(rec) = self.rec.as_mut() {
                rec.red.pop();
                self.step_no -= 1;
            }
        }
        self.t.rows_max = self.t.rows_max.max(rows);
        self.t.cols_max = self.t.cols_max.max(c as u64);
        self.t.nnz_max = self.t.nnz_max.max(nnz);
        self.t.rc_max = self.t.rc_max.max(rows as u128 * c as u128);
        self.t.cols_sum += c as u64;
        self.t.rows_sum += rows;
        let mem = nnz * 4 + rows * 48 + c as u64 * 32 + words as u64 * 8 + self.g_terms * 8
            + (self.g.len() as u64) * (self.g.len() as u64) / 16;
        self.t.mem_est_kb = self.t.mem_est_kb.max(mem / 1024);
        let t_add = Instant::now();
        // new elements in increasing label order
        newp.sort_unstable_by(|x, y| x.1.cmp(&y.1).then(x.3.cmp(&y.3)).then(x.4.cmp(&y.4)));
        let mut refuted = false;
        self.t.new_elems += newp.len() as u64;
        for np in &newp {
            let poly: Vec<u64> = store[np.0 as usize].iter().map(|&x| cols[x as usize]).collect();
            if poly[0] == 0 {
                refuted = true;
            }
            self.add_elem(poly, np.1, np.2);
        }
        self.t.ms_add += t_add.elapsed().as_secs_f64() * 1000.0;
        if std::env::var("B06_VERBOSE").is_ok() {
            eprintln!("  sig step d={d} rows={rows} cols={c} s_rows={} new={} zero_elim={} zero_build={} super={} G={} touches={} k0={} k1={} k2={} k3={} k4={} sing={}", self.t.rows_s, newp.len(), self.t.zero_elim, self.t.zero_build, self.t.super_top, self.g.len(), self.t.touches(), self.t.d_k0, self.t.d_k1, self.t.d_k2, self.t.d_k3, self.t.d_k4, self.t.d_sing);
        }
        self.t.log_d.push(d);
        self.t.log_rows.push(rows);
        self.t.log_touch.push(self.t.touches());
        self.t.log_zero.push(self.t.zero_red());
        self.t.log_new.push(self.t.new_elems);
        Ok(refuted)
    }

    fn run(&mut self, eqs: &[Poly]) -> Status {
        let polys: Vec<Vec<u64>> = eqs
            .iter()
            .map(|e| sort_cancel(e.clone()))
            .filter(|p| !p.is_empty())
            .collect();
        let mut polys = polys;
        if self.crit & 32 == 0 {
            polys.sort_by_key(|p| poly_degree(&p.clone()));
        }
        self.nidx = polys.len();
        self.lmset = (0..self.nidx).map(|_| FxSet::default()).collect();
        self.zset = (0..self.nidx).map(|_| FxSet::default()).collect();
        self.lm_ys = (0..self.nidx).map(|_| FxMap::default()).collect();
        self.sig_ys = (0..self.nidx).map(|_| FxMap::default()).collect();
        self.lm_pc = vec![(u32::MAX, 0); self.nidx];
        self.z_pc = vec![(u32::MAX, 0); self.nidx];
        self.sig_pc = vec![(u32::MAX, 0); self.nidx];
        let mut refuted = false;
        for (i, p) in polys.into_iter().enumerate() {
            self.gen_lm.push(p[0]);
            if p[0] == 0 {
                refuted = true;
            }
            self.add_elem(p, i as u32, 0);
        }
        let status = if refuted {
            Status::Refuted
        } else {
            loop {
                if self.over_time() {
                    break Status::TimeLimit;
                }
                if self.lim.touch_cap != 0 && self.t.touches() > self.lim.touch_cap {
                    break Status::TouchCap;
                }
                let d = match self.min_degree() {
                    Some(d) => d,
                    None => break Status::Gb,
                };
                match self.step(d) {
                    Ok(true) => break Status::Refuted,
                    Ok(false) => {}
                    Err(s) => break s,
                }
            }
        };
        self.t.g_size = self.g.len();
        status
    }
}

// ---------------------------------------------------------------- running one job

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    F4,
    Sig(u32),
}

struct JobOut {
    status: Status,
    t: Totals,
    ms: f64,
    rss_kb: u64,
    hwm_ok: bool,
    g: Vec<Vec<u64>>,
    lm: Vec<u64>,
    trace: Option<Trace>,
    replay: &'static str, // "na" | "ok" | "fallback"
    replay_touches: u64,
    fb_touches: u64,
    dropped: u64,
}
impl JobOut {
    fn charged(&self) -> u64 {
        if self.replay == "na" {
            self.t.touches()
        } else {
            self.replay_touches + self.fb_touches
        }
    }
}

fn exec(kind: Kind, eqs: &[Poly], nv: usize, lim: &Limits, record: bool, rep: Option<&Trace>) -> JobOut {
    exec_n(kind, eqs, nv, lim, record, rep, false)
}

fn exec_n(kind: Kind, eqs: &[Poly], nv: usize, lim: &Limits, record: bool, rep: Option<&Trace>, neg: bool) -> JobOut {
    let hwm_ok = reset_hwm();
    let t0 = Instant::now();
    let rec = if record { Some(Trace::default()) } else { None };
    let (status, t, g, lm, trace) = match kind {
        Kind::F4 => {
            let mut e = F4::new(lim);
            e.rec = rec;
            e.rep = rep;
            e.neg = neg;
            let s = e.run(eqs);
            (s, e.t, e.g, e.lm, e.rec)
        }
        Kind::Sig(crit) => {
            let mut e = Sig::new(lim, nv as u32, crit);
            e.rec = rec;
            e.rep = rep;
            e.neg = neg;
            let s = e.run(eqs);
            let g: Vec<Vec<u64>> = e.g.iter().map(|x| x.poly.clone()).collect();
            let lm: Vec<u64> = e.g.iter().map(|x| x.lm).collect();
            (s, e.t, g, lm, e.rec)
        }
    };
    JobOut {
        status,
        ms: t0.elapsed().as_secs_f64() * 1000.0,
        rss_kb: rss_peak_kb(),
        hwm_ok,
        dropped: t.pairs_drop,
        t,
        g,
        lm,
        trace,
        replay: "na",
        replay_touches: 0,
        fb_touches: 0,
    }
}

/// A replay with the trace `tr`; if the replayed pairs run out without `1`,
/// the full engine is run from scratch and both are charged.
fn exec_replay(kind: Kind, eqs: &[Poly], nv: usize, lim: &Limits, tr: &Trace) -> JobOut {
    exec_replay_n(kind, eqs, nv, lim, tr, false)
}

fn exec_replay_n(kind: Kind, eqs: &[Poly], nv: usize, lim: &Limits, tr: &Trace, neg: bool) -> JobOut {
    let mut j = exec_n(kind, eqs, nv, lim, false, Some(tr), neg);
    let rep_touches = j.t.touches();
    let dropped = j.t.pairs_drop;
    let rep_ms = j.ms;
    match j.status {
        Status::Refuted => {
            j.replay = "ok";
            j.replay_touches = rep_touches;
            j.fb_touches = 0;
            j
        }
        Status::Gb => {
            let mut f = exec(kind, eqs, nv, lim, false, None);
            f.replay = "fallback";
            f.replay_touches = rep_touches;
            f.fb_touches = f.t.touches();
            f.dropped = dropped;
            f.ms += rep_ms;
            f
        }
        _ => {
            j.replay = "ok";
            j.replay_touches = rep_touches;
            j
        }
    }
}

struct Traces {
    f4: Option<Trace>,
    sig: Option<Trace>,
    f4x: Option<Trace>,
}

/// Run the arm `arm` once.  `own` holds the traces recorded on this very target
/// (for `f4rs`/`sigrs`).
fn run_arm(arm: &str, eqs: &[Poly], nv: usize, lim: &Limits, tr: &Traces, own: &Traces) -> JobOut {
    match arm {
        "f4" => exec(Kind::F4, eqs, nv, lim, false, None),
        "sig" => exec(Kind::Sig(FULL), eqs, nv, lim, false, None),
        "sig0" => exec(Kind::Sig(0), eqs, nv, lim, false, None),
        "sig4" => exec(Kind::Sig(15), eqs, nv, lim, false, None),
        "sigc" => exec(Kind::Sig(FULL | 32), eqs, nv, lim, false, None),
        "sigs" => exec(Kind::Sig(FULL | 64), eqs, nv, lim, false, None),
        "f4r" => exec_replay(Kind::F4, eqs, nv, lim, tr.f4.as_ref().expect("f4 trace")),
        "f4n" => exec_replay_n(Kind::F4, eqs, nv, lim, tr.f4.as_ref().expect("f4 trace"), true),
        "sigr" => exec_replay(Kind::Sig(FULL), eqs, nv, lim, tr.sig.as_ref().expect("sig trace")),
        "sign" => exec_replay_n(Kind::Sig(FULL), eqs, nv, lim, tr.sig.as_ref().expect("sig trace"), true),
        "f4rs" => exec_replay(Kind::F4, eqs, nv, lim, own.f4.as_ref().expect("own f4 trace")),
        "sigrs" => exec_replay(Kind::Sig(FULL), eqs, nv, lim, own.sig.as_ref().expect("own sig trace")),
        a if a.starts_with("f4rx") => exec_replay(Kind::F4, eqs, nv, lim, tr.f4x.as_ref().expect("cross trace")),
        _ => panic!("unknown arm {arm}"),
    }
}

// ---------------------------------------------------------------- reference reduced basis

fn nf(p: &[u64], red: &[(u64, Vec<u64>)]) -> Vec<u64> {
    let mut cur: Vec<u64> = p.to_vec();
    let mut out: Vec<u64> = Vec::new();
    let mut start = 0usize;
    // `cur[start..]` is the part not yet decided; `out` the irreducible leading terms so far
    let mut work: Vec<u64> = Vec::new();
    std::mem::swap(&mut work, &mut cur);
    while start < work.len() {
        let m = work[start];
        if let Some((lm, r)) = red.iter().find(|(l, _)| l & !m == 0) {
            let t = m & !*lm;
            let prod = mul_mono(r, t);
            let rest = xor_merge(&work[start..], &prod);
            work = rest;
            start = 0;
        } else {
            out.push(m);
            start += 1;
        }
    }
    out
}

/// The reduced Groebner basis of the ideal (plus field equations) generated by
/// `g` (a Groebner basis, not necessarily minimal): sorted by leading monomial.
fn reduced_gb(g: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let mut polys: Vec<Vec<u64>> = g.iter().filter(|p| !p.is_empty()).cloned().collect();
    polys.sort_by_key(|p| key(p[0]));
    let mut red: Vec<(u64, Vec<u64>)> = Vec::new();
    for p in &polys {
        let q = nf(p, &red);
        if !q.is_empty() {
            red.push((q[0], q));
        }
    }
    let mut out: Vec<Vec<u64>> = red.into_iter().map(|x| x.1).collect();
    out.sort_by_key(|p| key(p[0]));
    out
}

// ---------------------------------------------------------------- output

fn arr_u(v: &[u64]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

fn json_job(label: &str, cell: &Cell, target: usize, arm: &str, solutions: u64, j: &JobOut, extra: &str) -> String {
    let t = &j.t;
    format!(
        "{{\"cell\":\"{}\",\"m\":{},\"n\":{},\"ell\":{},\"nv\":{},\"target\":{},\"arm\":\"{}\",\"solutions\":{},\"status\":\"{}\",\"d_mat\":{},\"d_sel\":{},\"steps\":{},\"pairs_cand\":{},\"pairs_sel\":{},\"pairs_gen\":{},\"pairs_drop\":{},\"d_k0\":{},\"d_sing\":{},\"d_dup\":{},\"d_k1\":{},\"d_k2\":{},\"d_k3\":{},\"d_k4\":{},\"d_k5\":{},\"rows_s\":{},\"zero_build\":{},\"zero_elim\":{},\"zero_red\":{},\"super_top\":{},\"rows_max\":{},\"cols_max\":{},\"rc_max\":{},\"nnz_max\":{},\"cols_sum\":{},\"rows_sum\":{},\"touches\":{},\"build\":{},\"elim\":{},\"back\":{},\"new_elems\":{},\"g_size\":{},\"sym_tests\":{},\"replay\":\"{}\",\"replay_touches\":{},\"fb_touches\":{},\"ms\":{:.1},\"ms_pairs\":{:.1},\"ms_sym\":{:.1},\"ms_elim\":{:.1},\"ms_back\":{:.1},\"ms_add\":{:.1},\"rss_kb\":{},\"hwm_reset\":{},\"mem_est_kb\":{},\"log_d\":{},\"log_rows\":{},\"log_touch\":{},\"log_zero\":{},\"log_new\":{}{}}}",
        label, cell.m, cell.n, cell.l, cell.nv(), target, arm, solutions, j.status.name(),
        t.d_mat, t.d_sel, t.steps, t.pairs_cand, t.pairs_sel, t.pairs_gen, j.dropped,
        t.d_k0, t.d_sing, t.d_dup, t.d_k1, t.d_k2, t.d_k3, t.d_k4, t.d_k5, t.rows_s, t.zero_build, t.zero_elim, t.zero_red(), t.super_top,
        t.rows_max, t.cols_max, t.rc_max, t.nnz_max, t.cols_sum, t.rows_sum,
        j.charged(), t.build, t.elim, t.back, t.new_elems, t.g_size, t.sym_tests,
        j.replay, j.replay_touches, j.fb_touches,
        j.ms, t.ms_pairs, t.ms_sym, t.ms_elim, t.ms_back, t.ms_add, j.rss_kb, j.hwm_ok, t.mem_est_kb,
        arr_u(&t.log_d.iter().map(|x| *x as u64).collect::<Vec<_>>()), arr_u(&t.log_rows), arr_u(&t.log_touch), arr_u(&t.log_zero), arr_u(&t.log_new), extra
    )
}

fn limits_from(a: &Args) -> Limits {
    let secs: f64 = a.num("--time-limit", 0.0);
    let cap = as_limit_bytes().map(|b| b / 100 * 88).unwrap_or(0);
    Limits {
        deadline: if secs > 0.0 {
            Some(Instant::now() + std::time::Duration::from_secs_f64(secs))
        } else {
            None
        },
        max_cols: a.num("--max-cols", 400_000usize),
        vm_cap: cap,
        touch_cap: 0,
    }
}

fn fail(msg: &str) -> ! {
    println!("SELFTEST FAILED: {msg}");
    std::process::exit(2);
}

// ---------------------------------------------------------------- run

fn cell_draws<'a>(draws: &'a [Draw], label: &str) -> Vec<&'a Draw> {
    let v: Vec<&Draw> = draws.iter().filter(|d| d.label == label).collect();
    assert!(!v.is_empty(), "no draws for {label}");
    v
}

fn system_of(d: &Draw) -> (Vec<Poly>, usize) {
    let gf = Gf::new(d.cell.n);
    let (eqs, nv) = build_system(&gf, d.cell.m, &d.basis, d.xr);
    assert_eq!(nv, d.cell.nv());
    (eqs, nv)
}

fn is_sig_arm(arm: &str) -> bool {
    arm.starts_with("sig")
}

fn mode_run(a: &Args) {
    let draws = read_draws(&a.flag("--draws").expect("--draws"));
    let label = a.flag("--cell").expect("--cell");
    let cd = cell_draws(&draws, &label);
    let tgts = parse_targets(&a.flag("--targets").unwrap_or("all".into()), cd.len());
    let mut arms: Vec<String> = a
        .flag("--arms")
        .unwrap_or_else(|| "f4".into())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    // f4 first: the signature arms are capped relative to its touches on the same target
    arms.sort_by_key(|x| if x == "f4" { 0 } else { 1 });
    let trace_target: Option<usize> = a.flag("--trace-target").map(|s| s.parse().unwrap());
    let total_limit: f64 = a.num("--total-limit", 0.0);
    let per_job: f64 = a.num("--time-limit", 0.0);
    let touch_ratio: f64 = a.num("--touch-ratio", 0.0);
    let trace_ratio: f64 = a.num("--trace-ratio", touch_ratio);
    let t_start = Instant::now();
    let mut bad = false;
    let want_f4_trace = arms.iter().any(|x| x == "f4r" || x == "f4n");
    let want_sig_trace = arms.iter().any(|x| x == "sigr" || x == "sign");
    let need_x = arms.iter().find(|x| x.starts_with("f4rx")).cloned();
    let mut traces = Traces { f4: None, sig: None, f4x: None };
    let sig_job: f64 = a.num("--sig-time-limit", per_job);
    let job_lim = |t_start: &Instant, cap: u64, sigfam: bool| {
        let mut lim = limits_from(a);
        // a signature arm gets the signature time limit
        let per_job = if sigfam { sig_job } else { per_job };
        if per_job > 0.0 {
            let mut secs = per_job;
            if total_limit > 0.0 {
                secs = secs.min((total_limit - t_start.elapsed().as_secs_f64()).max(1.0));
            }
            lim.deadline = Some(Instant::now() + std::time::Duration::from_secs_f64(secs));
        }
        lim.touch_cap = cap;
        lim
    };
    // the held-out target: record the traces and print their cost (they are charged)
    if let Some(tt) = trace_target {
        let d = cd.iter().find(|d| d.idx == tt).expect("trace target idx");
        let (eqs, nv) = system_of(d);
        let mut f4_touches = 0u64;
        for (arm, kind, slot) in [("f4", Kind::F4, 0), ("sig", Kind::Sig(FULL), 1)] {
            let print = if slot == 0 {
                want_f4_trace || arms.iter().any(|x| x == "f4")
            } else {
                want_sig_trace || arms.iter().any(|x| x == "sig")
            };
            // f4 on the held-out target is also run (unprinted) to cap the signature trace
            let wanted = print || (slot == 0 && want_sig_trace);
            if !wanted {
                continue;
            }
            let cap = if slot == 1 && trace_ratio > 0.0 && f4_touches > 0 { (trace_ratio * f4_touches as f64) as u64 } else { 0 };
            let lim = job_lim(&t_start, cap, slot == 1);
            let j = exec(kind, &eqs, nv, &lim, true, None);
            if print {
                println!("{}", json_job(&label, &d.cell, tt, arm, d.solutions, &j, ",\"trace\":1"));
                eprintln!("{label} t{tt} {arm} (trace): {} touches={} {:.1}s", j.status.name(), j.t.touches(), j.ms / 1000.0);
            }
            if slot == 0 {
                f4_touches = j.t.touches();
                if j.status != Status::Refuted && d.solutions == 0 {
                    eprintln!("f4 trace run did not refute: {}", j.status.name());
                    bad = true;
                }
                traces.f4 = j.trace;
            } else if j.status == Status::Refuted {
                traces.sig = j.trace;
            }
        }
    }
    if let Some(xa) = need_x {
        let xcell = xa.split_once(':').expect("f4rx:<cell>").1.to_string();
        let xd = cell_draws(&draws, &xcell);
        let tt = trace_target.unwrap_or(xd.len() - 1);
        let d = xd.iter().find(|d| d.idx == tt).expect("cross trace target");
        let (eqs, nv) = system_of(d);
        let lim = job_lim(&t_start, 0, false);
        let j = exec(Kind::F4, &eqs, nv, &lim, true, None);
        eprintln!("{xcell} t{tt} f4 (cross trace): {} touches={}", j.status.name(), j.t.touches());
        traces.f4x = j.trace;
    }
    for &ti in &tgts {
        let d = cd.iter().find(|d| d.idx == ti).expect("target idx");
        let (eqs, nv) = system_of(d);
        let sols = d.solutions;
        let mut own = Traces { f4: None, sig: None, f4x: None };
        let mut f4_touches = 0u64;
        for arm in &arms {
            if total_limit > 0.0 && t_start.elapsed().as_secs_f64() > total_limit {
                println!("{{\"cell\":\"{}\",\"target\":{},\"arm\":\"{}\",\"status\":\"skipped\"}}", label, ti, arm);
                continue;
            }
            let sigfam = is_sig_arm(arm);
            let no_trace = (arm == "sigr" || arm == "sign") && traces.sig.is_none();
            if sigfam && touch_ratio > 0.0 && f4_touches == 0 && !no_trace {
                // the f4 reading of this target (unprinted) fixes the cap of the signature arm
                f4_touches = exec(Kind::F4, &eqs, nv, &job_lim(&t_start, 0, false), false, None).t.touches();
            }
            let cap = if sigfam && touch_ratio > 0.0 && f4_touches > 0 { (touch_ratio * f4_touches as f64) as u64 } else { 0 };
            if arm == "f4rs" {
                own.f4 = exec(Kind::F4, &eqs, nv, &job_lim(&t_start, 0, false), true, None).trace;
            }
            if arm == "sigrs" {
                let r = exec(Kind::Sig(FULL), &eqs, nv, &job_lim(&t_start, cap, true), true, None);
                if r.status == Status::Refuted {
                    own.sig = r.trace;
                }
            }
            if ((arm == "sigr" || arm == "sign") && traces.sig.is_none()) || (arm == "sigrs" && own.sig.is_none()) {
                println!("{{\"cell\":\"{}\",\"target\":{},\"arm\":\"{}\",\"status\":\"notrace\"}}", label, ti, arm);
                eprintln!("{label} t{ti} {arm}: no valid trace (the recording run did not refute)");
                continue;
            }
            let lim = job_lim(&t_start, cap, sigfam);
            let j = run_arm(arm, &eqs, nv, &lim, &traces, &own);
            if arm == "f4" {
                f4_touches = j.t.touches();
            }
            let consistent = match j.status {
                Status::Refuted => sols == 0,
                Status::Gb => sols > 0,
                _ => true,
            };
            if !consistent {
                eprintln!("INCONSISTENT {label} target {ti} arm {arm}: status {} with {sols} solutions", j.status.name());
                bad = true;
            }
            println!("{}", json_job(&label, &d.cell, ti, arm, sols, &j, ""));
            eprintln!(
                "{label} t{ti} {arm}: {} D={} steps={} zero={} touches={} {}{:.1}s",
                j.status.name(),
                j.t.d_mat,
                j.t.steps,
                j.t.zero_red(),
                j.charged(),
                if j.replay == "fallback" { "FALLBACK " } else { "" },
                j.ms / 1000.0
            );
        }
    }
    if bad {
        std::process::exit(2);
    }
}

fn main() {
    let args = Args {
        v: std::env::args().skip(1).collect(),
    };
    match args.v.first().map(|s| s.as_str()) {
        Some("gen") => mode_gen(&args),
        Some("run") => mode_run(&args),
        Some("selftest") => mode_selftest(&args),
        Some("xcheck") => mode_xcheck(&args),
        Some("summarize") => mode_summarize(&args),
        _ => {
            eprintln!("usage: b06 selftest|gen|run|xcheck|summarize ...");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------- selftest

fn rand_sys(rng: &mut Rng, nv: usize, k: usize) -> Vec<Poly> {
    let mut eqs = Vec::new();
    for _ in 0..k {
        let nterms = 3 + (rng.next() % 5) as usize;
        let mut v: Vec<u64> = Vec::new();
        for _ in 0..nterms {
            let r = rng.next() % 4;
            let deg = if r == 0 { 1 } else if r == 3 { 3 } else { 2 };
            let mut m = 0u64;
            while (m.count_ones() as usize) < deg {
                m |= 1u64 << (rng.next() % nv as u64);
            }
            v.push(m);
        }
        if rng.next() % 3 == 0 {
            v.push(0);
        }
        eqs.push(norm(v));
    }
    eqs
}

fn selftest_orders() {
    let mut rng = Rng(77);
    let nv = 24usize;
    for _ in 0..20000 {
        let a = rng.next() & ((1 << nv) - 1);
        let b = rng.next() & ((1 << nv) - 1);
        if a == b {
            continue;
        }
        if (key(a) > key(b)) != grevlex_greater(a, b) {
            fail(&format!("key vs grevlex {a:#x} {b:#x}"));
        }
        // the polynomial-ring key agrees with `key` on squarefree monomials
        if (rk(a, 0, nv as u32) > rk(b, 0, nv as u32)) != grevlex_greater(a, b) {
            fail(&format!("rk vs grevlex on squarefree {a:#x} {b:#x}"));
        }
        // multiplicativity on disjoint multipliers
        let t = rng.next() & ((1 << nv) - 1) & !a & !b;
        if (rk(a, t, nv as u32) > rk(b, t, nv as u32)) != grevlex_greater(a, b) {
            fail("rk not multiplicative on a disjoint multiplier");
        }
        // squares: degree first
        let c = rng.next() & ((1 << nv) - 1);
        let d = rng.next() & c;
        let (dc, dd) = (c.count_ones() + d.count_ones(), a.count_ones());
        if dc != dd && (rk(c, d, nv as u32) > rk(a, 0, nv as u32)) != (dc > dd) {
            fail("rk degree-first");
        }
    }
    println!("orders ok: key = plain grevlex; rk = key on squarefree, multiplicative, degree-first with squares");
}

struct SysStats {
    unsat: u32,
    sat: u32,
    k: [u64; 6],
    replays: u32,
    self_ok: u32,
    fallbacks: u32,
}

fn check_system(name: &str, eqs: &[Poly], nv: usize, cross: &Traces, st: &mut SysStats) {
    let brute = brute_solutions(eqs, nv);
    let sols: Vec<u64> = if brute > 0 {
        (0..(1u64 << nv)).filter(|&pt| eqs.iter().all(|p| eval_poly(p, pt) == 0)).collect()
    } else {
        Vec::new()
    };
    if brute == 0 {
        st.unsat += 1;
    } else {
        st.sat += 1;
    }
    let lim = Limits { deadline: None, max_cols: 1 << 22, vm_cap: 0, touch_cap: 0 };
    let kinds: Vec<Kind> = if nv <= 8 {
        vec![Kind::F4, Kind::Sig(FULL), Kind::Sig(0), Kind::Sig(1), Kind::Sig(2), Kind::Sig(4), Kind::Sig(8), Kind::Sig(16), Kind::Sig(3), Kind::Sig(12), Kind::Sig(15), Kind::Sig(FULL | 32)]
    } else if nv <= 12 {
        vec![Kind::F4, Kind::Sig(FULL), Kind::Sig(15), Kind::Sig(8), Kind::Sig(16)]
    } else {
        vec![Kind::F4, Kind::Sig(FULL), Kind::Sig(15)]
    };
    let mut ref_gb: Option<Vec<Vec<u64>>> = None;
    let mut own = Traces { f4: None, sig: None, f4x: None };
    for kind in kinds {
        let j = exec(kind, eqs, nv, &lim, true, None);
        if std::env::var("B06_TIMING").is_ok() {
            eprintln!("  {name} {kind:?}: {} {:.0} ms, touches {}, g {}", j.status.name(), j.ms, j.t.touches(), j.t.g_size);
        }
        if kind == Kind::Sig(FULL) {
            st.k[0] += j.t.d_k0;
            st.k[1] += j.t.d_k1;
            st.k[2] += j.t.d_k2;
            st.k[3] += j.t.d_k3;
            st.k[4] += j.t.d_k4;
            st.k[5] += j.t.d_sing;
        }
        if brute == 0 {
            if j.status != Status::Refuted {
                fail(&format!("{name}: unsat system not refuted by {kind:?}: {}", j.status.name()));
            }
        } else {
            if j.status != Status::Gb {
                fail(&format!("{name}: sat system, {kind:?} ended {}", j.status.name()));
            }
            let sm = standard_monomials(&minimal_lms(&j.lm), nv);
            if sm != brute {
                fail(&format!("{name}: {kind:?} not a Groebner basis: {sm} standard monomials vs {brute} solutions"));
            }
            for gp in &j.g {
                for &pt in &sols {
                    if eval_poly(gp, pt) != 0 {
                        fail(&format!("{name}: {kind:?} element does not vanish on a solution"));
                    }
                }
            }
            let rgb = reduced_gb(&j.g);
            match &ref_gb {
                None => ref_gb = Some(rgb),
                Some(r) => {
                    if *r != rgb {
                        fail(&format!("{name}: reduced basis of {kind:?} differs from the F4 reduced basis"));
                    }
                }
            }
        }
        if kind == Kind::F4 {
            own.f4 = j.trace.clone();
        }
        if kind == Kind::Sig(FULL) {
            own.sig = j.trace.clone();
        }
    }
    // replays: own trace, then a trace of another system
    for (kind, mine, other, neg) in [
        (Kind::F4, &own.f4, &cross.f4, false),
        (Kind::Sig(FULL), &own.sig, &cross.sig, false),
        (Kind::F4, &own.f4, &cross.f4, true),
        (Kind::Sig(FULL), &own.sig, &cross.sig, true),
    ] {
        for (lab, tr) in [("own", mine), ("cross", other)] {
            let tr = match tr {
                Some(t) => t,
                None => continue,
            };
            if brute > 0 && nv > 12 {
                continue; // a replay on a satisfiable system ends in a full run; skipped on the larger toys
            }
            let j = exec_replay_n(kind, eqs, nv, &lim, tr, neg);
            st.replays += 1;
            if j.replay == "fallback" {
                st.fallbacks += 1;
            }
            if brute == 0 {
                if j.status != Status::Refuted {
                    fail(&format!("{name}: {kind:?} {lab}-trace replay did not refute an unsat system: {}", j.status.name()));
                }
                if lab == "own" && !neg {
                    if j.replay != "ok" {
                        std::env::set_var("B06_VERBOSE", "1");
                        eprintln!("--- record");
                        let _ = exec(kind, eqs, nv, &lim, true, None);
                        eprintln!("--- replay");
                        let _ = exec_replay(kind, eqs, nv, &lim, tr);
                        fail(&format!("{name}: {kind:?} replay of its own trace fell back"));
                    }
                    st.self_ok += 1;
                    let full = exec(kind, eqs, nv, &lim, false, None);
                    if !neg && j.t.touches() > full.t.touches() {
                        fail(&format!("{name}: {kind:?} own-trace replay costs more than the full run"));
                    }
                }
            } else {
                if j.status != Status::Gb {
                    fail(&format!("{name}: {kind:?} {lab}-trace replay on a sat system ended {}", j.status.name()));
                }
                if j.replay != "fallback" {
                    fail(&format!("{name}: replay on a sat system must fall back (a finished replay is not a basis)"));
                }
                let rgb = reduced_gb(&j.g);
                if Some(&rgb) != ref_gb.as_ref() {
                    fail(&format!("{name}: {kind:?} {lab} replay (after fallback) has a different reduced basis"));
                }
            }
        }
    }
}

fn selftest_systems() {
    let mut st = SysStats { unsat: 0, sat: 0, k: [0; 6], replays: 0, self_ok: 0, fallbacks: 0 };
    let toys = [
        Cell { m: 3, n: 5, l: 2 },
        Cell { m: 4, n: 4, l: 1 },
        Cell { m: 2, n: 7, l: 3 },
        Cell { m: 3, n: 6, l: 3 },
        Cell { m: 4, n: 5, l: 2 },
        Cell { m: 3, n: 7, l: 3 },
        Cell { m: 2, n: 9, l: 5 },
    ];
    let mut cross = Traces { f4: None, sig: None, f4x: None };
    let lim = Limits { deadline: None, max_cols: 1 << 22, vm_cap: 0, touch_cap: 0 };
    for c in toys {
        let gf = Gf::new(c.n);
        let mut rng = Rng(1000 + c.n as u64);
        let basis = random_basis(c.n, c.l, &mut rng);
        let (mut unsat, mut sat) = (0, 0);
        let mut tries = 0;
        while (unsat < 3 || sat < 3) && tries < 6000 {
            tries += 1;
            let xr = rng.next() & gf.mask;
            if xr == 0 {
                continue;
            }
            let alg = count_solutions(&gf, c.m, &basis, xr);
            if (alg > 0 && sat >= 3) || (alg == 0 && unsat >= 3) {
                continue;
            }
            let (eqs, nv) = build_system(&gf, c.m, &basis, xr);
            if brute_solutions(&eqs, nv) != alg {
                fail("algebraic solution count differs from brute force");
            }
            if alg > 0 {
                sat += 1;
            } else {
                unsat += 1;
            }
            check_system(&format!("toy {:?} xr={xr}", (c.m, c.n, c.l)), &eqs, nv, &cross, &mut st);
            let r = exec(Kind::F4, &eqs, nv, &lim, true, None);
            cross.f4 = r.trace;
            let r = exec(Kind::Sig(FULL), &eqs, nv, &lim, true, None);
            cross.sig = r.trace;
        }
        println!("toy (m,n,l)={:?} nv={}: {unsat} unsat, {sat} sat systems, all engines agree", (c.m, c.n, c.l), c.nv());
    }
    // random Boolean systems of 6 to 11 unknowns, under- and over-determined
    let mut rng = Rng(424242);
    for it in 0..90 {
        let nv = 6 + (rng.next() % 6) as usize;
        let k = (nv as i64 + (rng.next() % 7) as i64 - 3).max(2) as usize;
        let eqs = rand_sys(&mut rng, nv, k);
        check_system(&format!("random #{it} nv={nv} k={k}"), &eqs, nv, &cross, &mut st);
        let r = exec(Kind::F4, &eqs, nv, &lim, true, None);
        cross.f4 = r.trace;
        let r = exec(Kind::Sig(FULL), &eqs, nv, &lim, true, None);
        cross.sig = r.trace;
    }
    println!(
        "systems: {} unsat, {} sat; engines f4, sig, sig0 and each criterion alone agree (refutation, standard monomials = brute force, vanishing, identical reduced bases)",
        st.unsat, st.sat
    );
    println!(
        "criteria fired in sig over the selftest: K0 {} K1 {} K2 {} K3 {} K4 {} singular {}",
        st.k[0], st.k[1], st.k[2], st.k[3], st.k[4], st.k[5]
    );
    println!(
        "replays: {} run, {} own-trace replays of unsat systems refuted without fallback, {} fell back (never a wrong verdict)",
        st.replays, st.self_ok, st.fallbacks
    );
}

fn mode_selftest(_a: &Args) {
    selftest_orders();
    selftest_systems();
    println!("SELFTEST OK");
}

// ---------------------------------------------------------------- xcheck against the committed item 05 rows

fn mode_xcheck(a: &Args) {
    let draws = read_draws(&a.flag("--draws").expect("--draws"));
    let label = a.flag("--cell").expect("--cell");
    let refpath = a.flag("--ref").expect("--ref");
    let txt = std::fs::read_to_string(&refpath).unwrap_or_else(|e| panic!("ref {refpath}: {e}"));
    let cd = cell_draws(&draws, &label);
    let mut n = 0;
    let mut bad = 0;
    for line in txt.lines() {
        if !line.starts_with("{\"cell\"") || jstr_f(line, "cell").as_deref() != Some(label.as_str()) {
            continue;
        }
        if jstr_f(line, "arm").as_deref() != Some("grevlex") || jstr_f(line, "status").as_deref() != Some("refuted") {
            continue;
        }
        let ti = jnum_f(line, "target").unwrap() as usize;
        let d = cd.iter().find(|d| d.idx == ti).expect("target");
        let (eqs, nv) = system_of(d);
        let lim = Limits { deadline: None, max_cols: 400_000, vm_cap: 0, touch_cap: 0 };
        let j = exec(Kind::F4, &eqs, nv, &lim, false, None);
        let mine = [
            ("touches", j.t.touches()),
            ("build", j.t.build),
            ("elim", j.t.elim),
            ("back", j.t.back),
            ("zero_red", j.t.zero_red()),
            ("d_mat", j.t.d_mat as u64),
            ("steps", j.t.steps as u64),
            ("pairs_sel", j.t.pairs_sel),
            ("new_elems", j.t.new_elems),
            ("g_size", j.t.g_size as u64),
        ];
        let mut ok = j.status == Status::Refuted;
        for (k, v) in mine {
            if jnum_f(line, k).map(|x| x as u64) != Some(v) {
                ok = false;
                println!("  MISMATCH {label} t{ti} {k}: committed {:?}, here {v}", jnum_f(line, k));
            }
        }
        n += 1;
        if !ok {
            bad += 1;
        }
        println!("{label} target {ti}: committed touches {} here {} {}", jnum_f(line, "touches").unwrap() as u64, j.t.touches(), if ok { "IDENTICAL" } else { "DIFFERENT" });
    }
    println!("xcheck {label}: {n} committed item 05 grevlex rows, {bad} differ");
    if n == 0 || bad > 0 {
        println!("XCHECK FAILED");
        std::process::exit(2);
    }
    println!("XCHECK OK");
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

#[derive(Clone)]
struct Rec {
    status: String,
    touches: f64,
    rc_max: f64,
    zero_red: f64,
    sym_tests: f64,
    rss_kb: f64,
    mem_est: f64,
    ms: f64,
    replay: String,
    d_mat: f64,
    log_d: Vec<u64>,
    log_touch: Vec<u64>,
    log_zero: Vec<u64>,
}

fn jarr(line: &str, key: &str) -> Vec<u64> {
    let pat = format!("\"{key}\":[");
    match line.find(&pat) {
        None => Vec::new(),
        Some(i) => {
            let rest = &line[i + pat.len()..];
            let end = rest.find(']').unwrap_or(rest.len());
            rest[..end].split(',').filter(|x| !x.is_empty()).filter_map(|x| x.parse().ok()).collect()
        }
    }
}

fn gmean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}

fn is_cap(status: &str) -> bool {
    matches!(status, "timelimit" | "memcap" | "colscap" | "touchcap")
}

struct ArmCell {
    n_both: usize,
    n_cens: usize,
    ratios: Vec<f64>, // exact ratios (both refuted)
    bounds: Vec<f64>, // upper bounds for the censored targets
    gmean_exact: f64,
    gmean_incl: f64,
    n_ge: usize,
    zero_r: f64,
    rc_r: f64,
    mem_r: f64,
    sym_r: f64,
    nc_r: f64,
    fb: usize,
}

/// `summarize --dir <raw dir>`: read every `*.stdout.log`, pair each arm with
/// `f4` target by target and apply the frozen rule of PROTOCOL.md.
fn mode_summarize(a: &Args) {
    let dir = a.flag("--dir").expect("--dir");
    let cells: Vec<String> = a
        .flag("--cells")
        .unwrap_or_else(|| "c13-3-4,c15-3-4,c15-5-3,c17-4-3".into())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let nt: usize = a.num("--targets", 16);
    let min_ge: usize = a.num("--min-ge", 12);
    let mut recs: HashMap<(String, usize, String), Rec> = HashMap::new();
    let mut notrace: Vec<(String, String)> = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".stdout.log"))
        .collect();
    // Repair pass (PROTOCOL.md deviation 17): a re-run (`-r2` in the file name) is read after the
    // first-run files so it replaces a row of the same (cell, target, arm); a descriptive long run
    // (`-long-` in the file name, a different touch cap) never enters the rule's table.
    files.retain(|p| !p.file_name().map(|n| n.to_string_lossy().contains("-long-")).unwrap_or(false));
    files.sort_by_key(|p| {
        let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        (n.contains("-r2"), n)
    });
    for f in files {
        let txt = std::fs::read_to_string(&f).unwrap_or_default();
        for line in txt.lines() {
            if !line.starts_with("{\"cell\"") {
                continue;
            }
            let (cell, arm, status) = match (jstr_f(line, "cell"), jstr_f(line, "arm"), jstr_f(line, "status")) {
                (Some(c), Some(a), Some(s)) => (c, a, s),
                _ => continue,
            };
            if status == "notrace" {
                notrace.push((cell, arm));
                continue;
            }
            if status == "skipped" || status == "na" {
                continue;
            }
            let target = jnum_f(line, "target").unwrap_or(-1.0) as usize;
            recs.insert(
                (cell, target, arm),
                Rec {
                    status,
                    touches: jnum_f(line, "touches").unwrap_or(0.0),
                    rc_max: jnum_f(line, "rc_max").unwrap_or(0.0),
                    zero_red: jnum_f(line, "zero_red").unwrap_or(0.0),
                    sym_tests: jnum_f(line, "sym_tests").unwrap_or(0.0),
                    rss_kb: if line.contains("\"hwm_reset\":true") { jnum_f(line, "rss_kb").unwrap_or(0.0) } else { 0.0 },
                    mem_est: jnum_f(line, "mem_est_kb").unwrap_or(0.0),
                    ms: jnum_f(line, "ms").unwrap_or(0.0),
                    replay: jstr_f(line, "replay").unwrap_or_default(),
                    d_mat: jnum_f(line, "d_mat").unwrap_or(0.0),
                    log_d: jarr(line, "log_d"),
                    log_touch: jarr(line, "log_touch"),
                    log_zero: jarr(line, "log_zero"),
                },
            );
        }
    }
    println!("# item 06 summary: arm X against f4, paired by target (targets 0..{})", nt - 1);
    println!("# cost(sigr,sign) = touches + touches(sig, held-out)/{nt}; cost(f4r,f4n) = touches + touches(f4, held-out)/{nt}; fallbacks are inside touches");
    println!("# R = touches(f4)/cost(X); a capped X (touchcap/timelimit) gives only the bound R <= touches(f4)/touches(X at the cap)");
    println!("cell      arm    refuted capped  gmean_R(exact)  gmean_R(<=,incl.caps)  n>=1.5  R(no trace charge)  zero(f4/X)  rc_max(f4/X)  mem(f4/X)  sym(f4/X)  fallbacks");
    let arms_all = ["sig", "sig4", "sig0", "sigc", "f4r", "f4n", "sigr", "sign", "f4rs", "sigrs", "f4rx"];
    let mut res: HashMap<(String, String), ArmCell> = HashMap::new();
    for cell in &cells {
        let trace_f4 = recs.get(&(cell.clone(), nt, "f4".into())).map(|r| r.touches);
        let trace_sig = recs.get(&(cell.clone(), nt, "sig".into())).map(|r| r.touches);
        for arm in arms_all {
            let charge = match arm {
                "f4r" | "f4n" => trace_f4.unwrap_or(f64::NAN) / nt as f64,
                "sigr" | "sign" => trace_sig.unwrap_or(f64::NAN) / nt as f64,
                _ => 0.0,
            };
            let (mut rs, mut rs_nc, mut ubs) = (vec![], vec![], vec![]);
            let (mut zr, mut rc, mut me, mut sy) = (vec![], vec![], vec![], vec![]);
            let mut n_present = 0;
            let mut fb = 0;
            for t in 0..nt {
                let f = match recs.get(&(cell.clone(), t, "f4".into())) {
                    Some(f) if f.status == "refuted" => f,
                    _ => continue,
                };
                let x = match recs.get(&(cell.clone(), t, arm.to_string())) {
                    Some(x) => x,
                    None => match arm {
                        "f4rx" => match recs.get(&(cell.clone(), t, "f4rx:c13-3-4".to_string())) {
                            Some(x) => x,
                            None => continue,
                        },
                        _ => continue,
                    },
                };
                n_present += 1;
                if x.replay == "fallback" {
                    fb += 1;
                }
                if x.status == "refuted" {
                    rs.push(f.touches / (x.touches + charge));
                    rs_nc.push(f.touches / x.touches);
                    zr.push(f.zero_red.max(1.0) / x.zero_red.max(1.0));
                    rc.push(f.rc_max / x.rc_max.max(1.0));
                    let (mf, mx) = if f.rss_kb > 0.0 && x.rss_kb > 0.0 { (f.rss_kb, x.rss_kb) } else { (f.mem_est, x.mem_est) };
                    me.push(mf / mx.max(1.0));
                    sy.push(f.sym_tests.max(1.0) / x.sym_tests.max(1.0));
                } else if is_cap(&x.status) {
                    ubs.push(f.touches / (x.touches + charge).max(1.0));
                }
            }
            if n_present == 0 {
                continue;
            }
            let ge = rs.iter().filter(|&&r| r >= 1.5).count();
            let all: Vec<f64> = rs.iter().chain(ubs.iter()).cloned().collect();
            println!(
                "{:<9} {:<6} {:>2}/{:<2}   {:>2}      {:>10.3}      {:>12.3}{}        {:>2}/{:<2}   {:>10.3}          {:>8.3}   {:>8.3}      {:>8.3}   {:>8.3}      {}",
                cell, arm, rs.len(), nt, ubs.len(), gmean(&rs), gmean(&all), if ubs.is_empty() { " " } else { "*" },
                ge, rs.len(), gmean(&rs_nc), gmean(&zr), gmean(&rc), gmean(&me), gmean(&sy), fb
            );
            res.insert(
                (cell.clone(), arm.to_string()),
                ArmCell {
                    n_both: rs.len(),
                    n_cens: ubs.len(),
                    gmean_exact: gmean(&rs),
                    gmean_incl: gmean(&all),
                    n_ge: ge,
                    zero_r: gmean(&zr),
                    rc_r: gmean(&rc),
                    mem_r: gmean(&me),
                    sym_r: gmean(&sy),
                    nc_r: gmean(&rs_nc),
                    fb,
                    ratios: rs,
                    bounds: ubs,
                },
            );
        }
    }
    // the first degree-4 matrix: touches and zero reductions up to its end
    println!("# first matrix with selection degree 4 (cumulative touches, cumulative zero reductions at its end)");
    println!("cell      target arm    touches_at_first_d4  zero_at_first_d4  (stage not reached: -)");
    for cell in &cells {
        let (mut tr, mut zr) = (vec![], vec![]);
        for t in 0..nt {
            let line = |arm: &str| {
                recs.get(&(cell.clone(), t, arm.to_string())).map(|r| {
                    r.log_d.iter().position(|&d| d == 4).map(|i| (r.log_touch[i] as f64, r.log_zero[i] as f64))
                })
            };
            let (f, g) = (line("f4").flatten(), line("sig").flatten());
            match (f, g) {
                (Some(f), Some(g)) => {
                    tr.push(f.0 / g.0.max(1.0));
                    zr.push(f.1.max(1.0) / g.1.max(1.0));
                    if t < 3 {
                        println!("{:<9} {:>4}   f4     {:>14.0}      {:>10.0}", cell, t, f.0, f.1);
                        println!("{:<9} {:>4}   sig    {:>14.0}      {:>10.0}", cell, t, g.0, g.1);
                    }
                }
                _ => {}
            }
        }
        if !tr.is_empty() {
            println!("{:<9} first-d4-step gmean ratio f4/sig over {} targets: touches {:.3}, zero reductions {:.3}", cell, tr.len(), gmean(&tr), gmean(&zr));
        } else {
            println!("{:<9} the first degree-4 matrix was not reached by both f4 and sig", cell);
        }
    }
    // the verdict
    let mut verdict = String::new();
    let mut detail = Vec::new();
    let qualifies = |cell: &String, arm: &str| -> (bool, bool) {
        match res.get(&(cell.clone(), arm.to_string())) {
            Some(r) => {
                let q = r.n_both == nt && r.n_cens == 0 && r.gmean_exact >= 1.5 && r.n_ge >= min_ge;
                let g = r.rc_r >= 1.0 / 1.5 && r.mem_r >= 1.0 / 1.5;
                (q, g)
            }
            None => (false, false),
        }
    };
    for arm in ["sig", "sigr"] {
        let q: Vec<&String> = cells.iter().filter(|c| qualifies(c, arm).0).collect();
        let qg: Vec<&String> = q.iter().cloned().filter(|c| qualifies(c, arm).1).collect();
        detail.push(format!("{arm}: qualifies on {} of {} cells {:?}; with the matrix and memory guard {}", q.len(), cells.len(), q, qg.len()));
        if qg.len() >= 3 && verdict.is_empty() {
            verdict = format!("ADVANCES (constant) by {arm}");
        } else if q.len() >= 3 && verdict.is_empty() {
            verdict = format!("TRADES (constant, but the largest matrix or the peak memory is more than 1.5x larger) by {arm}");
        }
    }
    let countable = |cell: &String, arm: &str| res.get(&(cell.clone(), arm.to_string())).map_or(false, |r| r.n_both + r.n_cens >= min_ge);
    let regress_cells: Vec<&String> = cells
        .iter()
        .filter(|c| res.get(&((*c).clone(), "sig".to_string())).map_or(false, |r| r.n_both + r.n_cens == nt && r.gmean_incl <= 1.0 / 1.5))
        .collect();
    let exact_all = cells.iter().all(|c| res.get(&(c.clone(), "sig".to_string())).map_or(false, |r| r.n_both == nt));
    let countable_n = cells.iter().filter(|c| countable(c, "sig")).count();
    detail.push(format!("sig: R <= 1/1.5 certain (all {nt} targets exact or bounded) on {} of {} cells {:?}", regress_cells.len(), cells.len(), regress_cells));
    if verdict.is_empty() {
        verdict = if countable_n < 3 {
            "INCONCLUSIVE (fewer than 3 countable cells for sig)".to_string()
        } else if regress_cells.len() >= 3 {
            "REGRESSES (constant): sig is at least 1.5x worse than f4 in at least 3 cells (censored targets bound the ratio from above)".to_string()
        } else if exact_all {
            "MATCHES (constant)".to_string()
        } else {
            "INCONCLUSIVE (a cell has capped targets that do not bound the ratio below 1/1.5)".to_string()
        };
    }
    for d in detail {
        println!("# {d}");
    }
    let mut seen_nt: Vec<(String, String)> = Vec::new();
    for (c, arm) in &notrace {
        if !seen_nt.contains(&(c.clone(), arm.clone())) {
            seen_nt.push((c.clone(), arm.clone()));
            println!("# {arm} on {c}: BLOCKED for at least one target: the recording run of the trace did not refute (no trace)");
        }
    }
    // control 4: a replay with a trace of another cell may not be cheaper than the full run
    for cell in &cells {
        if let Some(r) = res.get(&(cell.clone(), "f4rx".to_string())) {
            let ok = r.nc_r <= 1.0 + 1e-9;
            println!("# CONTROL f4rx on {cell}: ratio {:.4} ({} fallbacks of {}) {}", r.nc_r, r.fb, r.n_both, if ok { "OK" } else { "FAILED: the accounting gives a replay a free lunch" });
        }
    }
    println!("# arm (e) msolve: BLOCKED (not installed; no GMP/FLINT/MPFR headers; 3.8 GB disk free); would be unblocked by an msolve build with a matrix-entry counter");
    println!("VERDICT: {verdict}");
}
