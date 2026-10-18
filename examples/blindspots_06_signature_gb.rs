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
//!             --arms f4,sig,f4r,sigr [--time-limit S] [--total-limit S]
//! b06 summarize --dir raw
//! ```
//!
//! Arms: `f4` (item 05's Boolean-ring F4, grevlex), `sig` (signature-based engine
//! with criteria K0-K4), `sig0` (the ablation: K1-K4 off), `f4r`/`sigr` (replay of
//! a trace recorded on the held-out target), `f4rs`/`sigrs` (replay of the
//! target's own trace: the ceiling), `f4rx:<cell>` (replay of another cell's
//! trace: the fallback control).

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

