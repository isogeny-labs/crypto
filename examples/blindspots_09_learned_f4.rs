//! Blind-spot item 09: learned critical-pair selection and pivot-order
//! policies for Boolean F4 on chained Semaev `S_3` descent systems, and
//! per-shape trace transfer.
//!
//! Standard library only.  At the branch base the `crypto` library does not
//! compile (see `research/blindspots_20261018/09-learned-f4-policy/
//! PROTOCOL.md`, Deviation 1), so the field arithmetic, the symbolic `S_3`
//! chain system and the exact solution counter are copied from
//! `blindspots_07_blocked_closure.rs` (itself checked against the committed
//! solution counts) and a Boolean F4 engine is re-implemented here, following
//! `src/cryptanalysis/pq_f4_f2.rs` (Gebauer-Moeller update, field pairs,
//! symbolic preprocessing, row-by-row forward elimination counted in 64-bit
//! word XORs).
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
//!     -o target/blindspots-09/b09 examples/blindspots_09_learned_f4.rs
//! b09 selftest
//! b09 train  --n 17 --ell 4 --m 3 --out PREFIX
//! b09 eval   --n 17 --ell 4 --m 3 --model PREFIX.model --trace PREFIX.trace --arms a,b,c,d
//! b09 report FILE...
//! ```
//!
//! Arms: `a` incumbent (all minimal-degree pairs, lead-descending row order),
//! `b` learned pair subset, `c` learned pivot (row) order, `d` trace transfer
//! (the pairs target A's recorded run found useful, replayed on target B),
//! `o` oracle subset (a bound, not an arm: labels computed on the same state,
//! unpriced), `r` random subset (negative control).

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::time::Instant;

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
    let span: Vec<u64> = (0..1u64 << l)
        .map(|c| {
            (0..l)
                .filter(|t| (c >> t) & 1 == 1)
                .fold(0, |a, t| a ^ basis[t])
        })
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
    (0..n)
        .map(|k| if (v >> k) & 1 == 1 { vec![0] } else { vec![] })
        .collect()
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
    (0..n as usize)
        .map(|k| vec![1u64 << (offset + k)])
        .collect()
}

/// The chained system `build_decomposition_system` builds, for `m >= 3`:
/// variables `m*l` subspace coordinates, then `(m-2)*n` intermediate bits.
fn build_system(gf: &Gf, m: usize, basis: &[u64], xr: u64) -> (Vec<Poly>, usize) {
    let l = basis.len();
    let n = gf.n;
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

// ---------------------------------------------------------------- column index

// ---------------------------------------------------------------- hashing

#[derive(Default, Clone, Copy)]
struct Fx(u64);
impl Fx {
    #[inline]
    fn add(&mut self, x: u64) {
        self.0 = (self.0.rotate_left(5) ^ x).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
}
impl Hasher for Fx {
    #[inline]
    fn finish(&self) -> u64 {
        self.0 ^ (self.0 >> 29)
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.add(b as u64);
        }
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64)
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(i as u64)
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i)
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64)
    }
}
type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<Fx>>;
type FxSet<K> = HashSet<K, BuildHasherDefault<Fx>>;

// ---------------------------------------------------------------- monomials

/// Sort key: smaller key = greater monomial in DegRevLex with x_0 > x_1 > ...
/// (degree descending, then numerically smaller mask first).
#[inline]
fn mkey(m: u64) -> u64 {
    ((63 - m.count_ones() as u64) << 48) | m
}

/// A Boolean polynomial: distinct masks sorted by `mkey` ascending, so the
/// leading monomial is first.
type KPoly = Vec<u64>;

fn to_kpoly(p: &Poly) -> KPoly {
    let mut v = p.clone();
    v.sort_unstable_by_key(|&m| mkey(m));
    v
}

fn mul_mono(p: &KPoly, m: u64) -> KPoly {
    let mut v: Vec<u64> = p.iter().map(|&t| t | m).collect();
    v.sort_unstable_by_key(|&t| mkey(t));
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

fn kpoly_hash(p: &KPoly) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &t in p {
        h = (h ^ t).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// ---------------------------------------------------------------- matrices

const NONE: u32 = u32::MAX;

struct MRow {
    bits: Vec<u64>,
    start: usize,
    end: usize,
    /// Pivots whose non-zero words are few are stored as (word, value)
    /// lists: the same row, XORed faster; the counted unit is unchanged.
    sp: Option<(Vec<u32>, Vec<u64>)>,
}
impl MRow {
    fn lead(&mut self) -> Option<usize> {
        while self.start < self.end {
            let w = self.bits[self.start];
            if w != 0 {
                return Some(self.start * 64 + w.trailing_zeros() as usize);
            }
            self.start += 1;
        }
        None
    }
    fn nnz(&self) -> u32 {
        self.bits[self.start..self.end]
            .iter()
            .map(|w| w.count_ones())
            .sum()
    }
    fn clone_row(&self) -> MRow {
        MRow {
            bits: self.bits.clone(),
            start: self.start,
            end: self.end,
            sp: self.sp.clone(),
        }
    }
}

struct Columns {
    monos: Vec<u64>,
    index: FxMap<u64, u32>,
}
impl Columns {
    fn from_monomials(mut v: Vec<u64>) -> Columns {
        v.sort_unstable_by_key(|&m| mkey(m));
        v.dedup();
        let mut index: FxMap<u64, u32> = FxMap::default();
        index.reserve(v.len());
        for (i, &m) in v.iter().enumerate() {
            index.insert(m, i as u32);
        }
        Columns { monos: v, index }
    }
    fn words(&self) -> usize {
        (self.monos.len() + 63) / 64
    }
    fn pack(&self, p: &KPoly) -> MRow {
        let words = self.words();
        let mut bits = vec![0u64; words];
        let (mut lo, mut hi) = (usize::MAX, 0usize);
        for &t in p {
            let c = self.index[&t] as usize;
            bits[c / 64] |= 1u64 << (c % 64);
            lo = lo.min(c / 64);
            hi = hi.max(c / 64 + 1);
        }
        if lo == usize::MAX {
            lo = 0;
            hi = 0;
        }
        MRow {
            bits,
            start: lo,
            end: hi,
            sp: None,
        }
    }
    fn unpack(&self, r: &MRow) -> KPoly {
        let mut out = Vec::new();
        let mut emit = |w: usize, mut b: u64| {
            while b != 0 {
                let k = b.trailing_zeros() as usize;
                b &= b - 1;
                out.push(self.monos[w * 64 + k]);
            }
        };
        if let Some((ix, vs)) = &r.sp {
            for (i, v) in ix.iter().zip(vs) {
                emit(*i as usize, *v);
            }
        } else {
            for w in r.start..r.end {
                emit(w, r.bits[w]);
            }
        }
        out
    }
}

/// Row-by-row forward elimination.  `xors` counts the 64-bit words between a
/// pivot's lead word and its last non-zero word, once per row-add: the
/// convention of `pq_f4_f2::F4Stats::word_xors`.
struct Elim {
    piv: Vec<u32>,
    rows: Vec<MRow>,
    leads: Vec<usize>,
    xors: u64,
}
impl Elim {
    fn new(ncols: usize) -> Elim {
        Elim {
            piv: vec![NONE; ncols],
            rows: Vec::new(),
            leads: Vec::new(),
            xors: 0,
        }
    }
    fn clone_state(&self) -> Elim {
        Elim {
            piv: self.piv.clone(),
            rows: self.rows.iter().map(|r| r.clone_row()).collect(),
            leads: self.leads.clone(),
            xors: self.xors,
        }
    }
    /// Reduce `r`; if it survives, store it as a pivot and return its lead.
    fn reduce(&mut self, mut r: MRow) -> Option<usize> {
        loop {
            let lead = r.lead()?;
            let pi = self.piv[lead];
            if pi == NONE {
                while r.end > r.start && r.bits[r.end - 1] == 0 {
                    r.end -= 1;
                }
                let nzw = r.bits[r.start..r.end].iter().filter(|&&w| w != 0).count();
                if nzw * 4 < r.end - r.start {
                    let mut ix = Vec::with_capacity(nzw);
                    let mut vs = Vec::with_capacity(nzw);
                    for w in r.start..r.end {
                        if r.bits[w] != 0 {
                            ix.push(w as u32);
                            vs.push(r.bits[w]);
                        }
                    }
                    r.sp = Some((ix, vs));
                    r.bits = Vec::new();
                }
                self.piv[lead] = self.rows.len() as u32;
                self.rows.push(r);
                self.leads.push(lead);
                return Some(lead);
            }
            let p = &self.rows[pi as usize];
            let (ps, pe) = (p.start, p.end);
            if let Some((ix, vs)) = &p.sp {
                for (i, v) in ix.iter().zip(vs.iter()) {
                    r.bits[*i as usize] ^= *v;
                }
            } else {
                let dst = &mut r.bits[ps..pe];
                let src = &p.bits[ps..pe];
                for (d, s) in dst.iter_mut().zip(src.iter()) {
                    *d ^= *s;
                }
            }
            if pe > r.end {
                r.end = pe;
            }
            self.xors += (pe - ps) as u64;
        }
    }
}

// ---------------------------------------------------------------- statistics

#[derive(Default, Clone)]
struct Stats {
    xors: u64,
    mat_words: u64,
    rows_built: u64,
    steps: u64,
    pairs_reduced: u64,
    field_pairs_reduced: u64,
    pairs_deferred: u64,
    reducer_rows: u64,
    divisor_tests: u64,
    rows_max: u64,
    cols_max: u64,
    solving_degree: u32,
    degree_reached: u32,
    policy_ops: u64,
    new_elements: u64,
    peak_matrix_bytes: u64,
    pairs_left: u64,
    // offline (training / oracle) work, never in the arm's cost
    label_xors: u64,
}
impl Stats {
    fn touches(&self) -> u64 {
        self.xors + self.mat_words
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Outcome {
    Unsat,
    Sat,
    CapTouch,
    CapWall,
    CapMatrix,
}
impl Outcome {
    fn name(self) -> &'static str {
        match self {
            Outcome::Unsat => "unsat",
            Outcome::Sat => "sat",
            Outcome::CapTouch => "cap_touch",
            Outcome::CapWall => "cap_wall",
            Outcome::CapMatrix => "cap_matrix",
        }
    }
}

// ---------------------------------------------------------------- pairs and state

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pair {
    field: bool,
    a: u32,
    /// Critical: the second element.  Field: the variable.
    b: u32,
    lcm: u64,
    deg: u32,
}

struct St {
    polys: Vec<KPoly>,
    lm: Vec<u64>,
    active: Vec<bool>,
    pairs: Vec<Pair>,
}

impl St {
    fn new() -> St {
        St {
            polys: Vec::new(),
            lm: Vec::new(),
            active: Vec::new(),
            pairs: Vec::new(),
        }
    }

    /// Becker-Weispfenning UPDATE for the critical pairs, plus the field
    /// pairs of the new element (same logic as `pq_f4_f2::State`).
    fn insert_core(&mut self, p: KPoly, immediate_filter: bool) {
        let h = self.polys.len();
        let lh = p[0];
        self.polys.push(p);
        self.lm.push(lh);
        self.active.push(false);
        let dh = lh.count_ones();
        let mut bits = lh;
        while bits != 0 {
            let v = bits.trailing_zeros();
            bits &= bits - 1;
            self.pairs.push(Pair {
                field: true,
                a: h as u32,
                b: v,
                lcm: lh,
                deg: dh + 1,
            });
        }
        let active: Vec<usize> = (0..h).filter(|&g| self.active[g]).collect();
        let mut candidates: Vec<(usize, u64)> =
            active.iter().map(|&g| (g, lh | self.lm[g])).collect();
        let mut kept: Vec<(usize, u64)> = Vec::with_capacity(candidates.len());
        while let Some((g1, l1)) = candidates.pop() {
            let coprime = lh & self.lm[g1] == 0;
            let covered = candidates
                .iter()
                .chain(kept.iter())
                .any(|&(_, l2)| l2 & !l1 == 0);
            if coprime || !covered {
                kept.push((g1, l1));
            }
        }
        let selected: Vec<(usize, u64)> = kept
            .into_iter()
            .filter(|&(g, _)| lh & self.lm[g] != 0)
            .collect();
        if immediate_filter {
            self.filter_pairs_since(h);
        }
        for (g, l) in selected {
            self.pairs.push(Pair {
                field: false,
                a: g as u32,
                b: h as u32,
                lcm: l,
                deg: l.count_ones(),
            });
        }
        for g in 0..h {
            if self.active[g] && lh & !self.lm[g] == 0 {
                self.active[g] = false;
            }
        }
        self.active[h] = true;
    }

    fn insert_batch(&mut self, polys: Vec<KPoly>) {
        let first = self.polys.len();
        for p in polys {
            self.insert_core(p, false);
        }
        if self.polys.len() > first {
            self.filter_pairs_since(first);
        }
    }

    fn filter_pairs_since(&mut self, first: usize) {
        let lm = &self.lm;
        self.pairs.retain(|p| {
            if p.field {
                return true;
            }
            let (i, j) = (p.a as usize, p.b as usize);
            (first.max(i.max(j) + 1)..lm.len()).all(|h| {
                let lh = lm[h];
                lh & !p.lcm != 0 || (lm[i] | lh) == p.lcm || (lm[j] | lh) == p.lcm
            })
        });
    }

    /// The active element with the fewest terms whose leading monomial
    /// divides `m` (first on a tie).
    fn reducer_among(&self, m: u64, active: &[usize]) -> Option<usize> {
        let mut best: Option<usize> = None;
        for &g in active {
            if self.lm[g] & !m == 0
                && best.map_or(true, |b| self.polys[g].len() < self.polys[b].len())
            {
                best = Some(g);
            }
        }
        best
    }
}

// ---------------------------------------------------------------- features and models

const NF: usize = 15; // pair features
const NR: usize = 9; // S-row features
/// Native policy work priced per scored pair (feature construction plus the
/// dot product, counted generously) and per set-membership test.
const OPS_PER_PAIR_SCORE: u64 = 60;
const OPS_PER_TRACE_TEST: u64 = 8;

struct Ctx {
    nv: usize,
    coord_mask: u64,
    n_start: usize,
}

fn pair_feats(st: &St, p: &Pair, step: usize, cx: &Ctx) -> [f64; NF] {
    let (a, b) = (p.a as usize, p.b as usize);
    let d = p.deg as f64;
    let la = st.lm[a];
    let da = la.count_ones() as f64;
    let ln1 = |x: usize| ((1 + x) as f64).ln();
    let (db, ta, tb, ov, derived, lcm, mult, mn, recent) = if p.field {
        (
            0.0,
            ln1(st.polys[a].len()),
            0.0,
            da,
            (a >= cx.n_start) as u32 as f64,
            la,
            1.0,
            da,
            a,
        )
    } else {
        let lb = st.lm[b];
        let db = lb.count_ones() as f64;
        (
            db,
            ln1(st.polys[a].len()),
            ln1(st.polys[b].len()),
            (la & lb).count_ones() as f64,
            ((a >= cx.n_start) as u32 + (b >= cx.n_start) as u32) as f64,
            p.lcm,
            (d - da) + (d - db),
            da.min(db),
            a.max(b),
        )
    };
    [
        1.0,
        d,
        p.field as u32 as f64,
        da,
        db,
        ta,
        tb,
        ov,
        derived,
        recent as f64 / st.polys.len() as f64,
        ((1 + step) as f64).ln(),
        mult,
        (lcm & cx.coord_mask).count_ones() as f64,
        (lcm & !cx.coord_mask).count_ones() as f64,
        mn,
    ]
}

#[derive(Clone, Default)]
struct Model {
    sel_mu: Vec<f64>,
    sel_sd: Vec<f64>,
    sel_w: Vec<f64>,
    sel_tau: f64, // logit threshold
    sel_rate: f64,
    ord_mu: Vec<f64>,
    ord_sd: Vec<f64>,
    ord_w: Vec<f64>,
}

impl Model {
    fn score(&self, x: &[f64; NF]) -> f64 {
        let mut s = 0.0;
        for i in 0..NF {
            let z = if self.sel_sd[i] > 0.0 {
                (x[i] - self.sel_mu[i]) / self.sel_sd[i]
            } else {
                0.0
            };
            s += self.sel_w[i] * z;
        }
        s
    }
    fn row_score(&self, x: &[f64; NR]) -> f64 {
        let mut s = 0.0;
        for i in 0..NR {
            let z = if self.ord_sd[i] > 0.0 {
                (x[i] - self.ord_mu[i]) / self.ord_sd[i]
            } else {
                0.0
            };
            s += self.ord_w[i] * z;
        }
        s
    }
    fn to_text(&self) -> String {
        let f = |v: &Vec<f64>| {
            v.iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        format!(
            "b09-model-v1\nsel_mu {}\nsel_sd {}\nsel_w {}\nsel_tau {}\nsel_rate {}\nord_mu {}\nord_sd {}\nord_w {}\n",
            f(&self.sel_mu), f(&self.sel_sd), f(&self.sel_w), self.sel_tau, self.sel_rate,
            f(&self.ord_mu), f(&self.ord_sd), f(&self.ord_w)
        )
    }
    fn from_text(t: &str) -> Model {
        let mut m = Model::default();
        for line in t.lines() {
            let mut it = line.split_whitespace();
            let k = it.next().unwrap_or("");
            let v: Vec<f64> = it.map(|x| x.parse().expect("model number")).collect();
            match k {
                "sel_mu" => m.sel_mu = v,
                "sel_sd" => m.sel_sd = v,
                "sel_w" => m.sel_w = v,
                "sel_tau" => m.sel_tau = v[0],
                "sel_rate" => m.sel_rate = v[0],
                "ord_mu" => m.ord_mu = v,
                "ord_sd" => m.ord_sd = v,
                "ord_w" => m.ord_w = v,
                _ => {}
            }
        }
        assert!(
            m.sel_w.len() == NF && m.ord_w.len() == NR,
            "model file has wrong dimensions"
        );
        m
    }
}

type TraceKey = (u64, u64, bool);

enum PairPol {
    All,
    /// The top `q` fraction (at least one pair) of the minimal-degree pairs
    /// by the learned score.
    Learned(f64),
    Trace(FxSet<TraceKey>),
    /// A uniformly random `q` fraction (at least one pair): the null of `Learned(q)`.
    Random(f64),
    Oracle,
}
struct Policy {
    pair: PairPol,
    learned_order: bool,
    random_order: bool,
    model: Option<Model>,
}
impl Policy {
    fn incumbent() -> Policy {
        Policy {
            pair: PairPol::All,
            learned_order: false,
            random_order: false,
            model: None,
        }
    }
}

struct Caps {
    max_touch: u64,
    max_matrix_words: u64,
    wall_s: f64,
    /// cap on a matrix (rows*words) for cloning-based (c) training samples
    train_clone_words: u64,
}

/// Training samples gathered while the incumbent runs a train target.
#[derive(Default)]
struct Collector {
    px: Vec<[f64; NF]>,
    py: Vec<u8>,
    rx: Vec<[f64; NR]>,
    ry: Vec<f64>,
    trace: Vec<TraceKey>,
    // how often each candidate order won on the sampled matrices
    order_wins: [u64; 5],
    order_matrices: u64,
    order_best_xors: u64,
    order_default_xors: u64,
    pair_samples_cap: usize,
    row_samples_cap: usize,
}

fn trace_key(st: &St, p: &Pair) -> TraceKey {
    if p.field {
        (st.lm[p.a as usize], p.b as u64, true)
    } else {
        let (x, y) = (st.lm[p.a as usize], st.lm[p.b as usize]);
        (x.min(y), x.max(y), false)
    }
}

// ---------------------------------------------------------------- one F4 step's matrix

struct StepMat {
    cols: Columns,
    reducers: Vec<KPoly>,
    srows: Vec<KPoly>,
    srow_field: Vec<bool>,
    /// per selected pair: its (first, second) S-row ids, `usize::MAX` = none
    pair_rows: Vec<(usize, usize)>,
    no_div: FxSet<u64>,
    divisor_tests: u64,
}

fn build_step(st: &St, selected: &[Pair]) -> StepMat {
    let mut row_of: FxMap<(u64, u32), usize> = FxMap::default();
    let mut keys: Vec<(u64, u32, bool)> = Vec::new();
    let mut pair_rows: Vec<(usize, usize)> = Vec::with_capacity(selected.len());
    let mut lcm_cols: FxSet<u64> = FxSet::default();
    for p in selected {
        if p.field {
            let key = (1u64 << p.b, p.a);
            let id = *row_of.entry(key).or_insert_with(|| {
                keys.push((key.0, key.1, true));
                keys.len() - 1
            });
            pair_rows.push((id, usize::MAX));
        } else {
            lcm_cols.insert(p.lcm);
            let mut ids = [0usize; 2];
            for (k, g) in [p.a, p.b].into_iter().enumerate() {
                let mult = p.lcm & !st.lm[g as usize];
                let key = (mult, g);
                ids[k] = *row_of.entry(key).or_insert_with(|| {
                    keys.push((key.0, key.1, false));
                    keys.len() - 1
                });
            }
            pair_rows.push((ids[0], ids[1]));
        }
    }
    // products; a zero field product is dropped (as pq_f4_f2 does)
    let mut srows: Vec<KPoly> = Vec::with_capacity(keys.len());
    let mut srow_field: Vec<bool> = Vec::with_capacity(keys.len());
    let mut remap: Vec<usize> = vec![usize::MAX; keys.len()];
    for (i, &(mult, g, fld)) in keys.iter().enumerate() {
        let prod = mul_mono(&st.polys[g as usize], mult);
        if prod.is_empty() {
            continue;
        }
        remap[i] = srows.len();
        srows.push(prod);
        srow_field.push(fld);
    }
    for pr in pair_rows.iter_mut() {
        if pr.0 != usize::MAX {
            pr.0 = remap[pr.0];
        }
        if pr.1 != usize::MAX {
            pr.1 = remap[pr.1];
        }
    }
    // symbolic preprocessing
    let active: Vec<usize> = (0..st.polys.len()).filter(|&g| st.active[g]).collect();
    let mut examined: FxSet<u64> = FxSet::default();
    let mut queue: Vec<u64> = Vec::new();
    for r in &srows {
        for &t in r {
            if !lcm_cols.contains(&t) && examined.insert(t) {
                queue.push(t);
            }
        }
    }
    for &c in lcm_cols.iter() {
        examined.insert(c);
    }
    queue.sort_unstable();
    let mut reducers: Vec<KPoly> = Vec::new();
    let mut no_div: FxSet<u64> = FxSet::default();
    let mut tests = 0u64;
    let mut frontier = queue;
    while !frontier.is_empty() {
        let mut next: Vec<u64> = Vec::new();
        tests += (active.len() * frontier.len()) as u64;
        for &m in &frontier {
            match st.reducer_among(m, &active) {
                Some(g) => {
                    let row = mul_mono(&st.polys[g], m & !st.lm[g]);
                    for &t in &row[1..] {
                        if examined.insert(t) {
                            next.push(t);
                        }
                    }
                    reducers.push(row);
                }
                None => {
                    no_div.insert(m);
                }
            }
        }
        next.sort_unstable();
        frontier = next;
    }
    let cols = Columns::from_monomials(examined.into_iter().collect());
    StepMat {
        cols,
        reducers,
        srows,
        srow_field,
        pair_rows,
        no_div,
        divisor_tests: tests,
    }
}

/// Which selected pairs are useful: process the pairs one at a time (by
/// degree, lcm, then index) against the reducers and count a pair useful when
/// its rows add a pivot on a leading monomial no active element divides.
/// Offline work: counted in `label_xors`, never in the arm's cost.
fn label_pass(sm: &StepMat, selected: &[Pair], stats: &mut Stats) -> Vec<bool> {
    let mut el = Elim::new(sm.cols.monos.len());
    for r in &sm.reducers {
        el.reduce(sm.cols.pack(r));
    }
    let mut idx: Vec<usize> = (0..selected.len()).collect();
    idx.sort_by_key(|&i| {
        (
            selected[i].deg,
            mkey(selected[i].lcm),
            selected[i].a,
            selected[i].b,
        )
    });
    let mut labels = vec![false; selected.len()];
    for i in idx {
        let (r0, r1) = sm.pair_rows[i];
        for rid in [r0, r1] {
            if rid == usize::MAX {
                continue;
            }
            if let Some(lead) = el.reduce(sm.cols.pack(&sm.srows[rid])) {
                if sm.no_div.contains(&sm.cols.monos[lead]) {
                    labels[i] = true;
                }
            }
        }
    }
    stats.label_xors += el.xors;
    labels
}

fn row_feats(
    r: &MRow,
    lead: usize,
    ncols: usize,
    field: bool,
    group: usize,
    basepos: f64,
    monos: &[u64],
) -> [f64; NR] {
    let nnz = r.nnz() as f64;
    let span = (r.end - r.start) as f64;
    [
        1.0,
        (1.0 + nnz).ln(),
        (1.0 + span).ln(),
        lead as f64 / ncols.max(1) as f64,
        field as u32 as f64,
        (group as f64).ln_1p(),
        basepos,
        nnz / (span * 64.0).max(1.0),
        monos[lead].count_ones() as f64,
    ]
}

/// Eliminate a step's matrix; return the fresh (new-element) polynomials.
/// `order_model`: use the learned row order; `col`: gather (c) training data.
fn eliminate_step(
    sm: &StepMat,
    model: Option<&Model>,
    learned_order: bool,
    random_order: bool,
    rng: &mut Rng,
    col: &mut Option<Collector>,
    stats: &mut Stats,
    deadline: &Instant,
    wall_s: f64,
    max_touch: u64,
) -> Result<Vec<KPoly>, Outcome> {
    let ncols = sm.cols.monos.len();
    let mut el = Elim::new(ncols);
    for r in &sm.reducers {
        el.reduce(sm.cols.pack(r));
    }
    // pack the S-rows and find their leads
    let mut packed: Vec<Option<MRow>> = sm.srows.iter().map(|p| Some(sm.cols.pack(p))).collect();
    let nsr = packed.len();
    let leads: Vec<usize> = packed
        .iter_mut()
        .map(|r| r.as_mut().unwrap().lead().unwrap_or(usize::MAX))
        .collect();
    let mut base: Vec<usize> = (0..nsr).collect();
    base.sort_by_key(|&i| (leads[i], i)); // lead column ascending = greater monomial first
    let mut group: FxMap<usize, usize> = FxMap::default();
    for &l in &leads {
        *group.entry(l).or_insert(0) += 1;
    }
    let needs_feats = learned_order
        || col.as_ref().map_or(false, |c| {
            nsr >= 500 && c.rx.len() < c.row_samples_cap && c.order_matrices < 2
        });
    let mut feats: Vec<[f64; NR]> = Vec::new();
    if needs_feats && nsr > 0 {
        let mut basepos = vec![0.0f64; nsr];
        for (pos, &i) in base.iter().enumerate() {
            basepos[i] = pos as f64 / nsr as f64;
        }
        for i in 0..nsr {
            let l = leads[i];
            let g = *group.get(&l).unwrap_or(&1);
            feats.push(row_feats(
                packed[i].as_ref().unwrap(),
                l.min(ncols.saturating_sub(1)),
                ncols,
                sm.srow_field[i],
                g,
                basepos[i],
                &sm.cols.monos,
            ));
        }
    }
    // (c) training samples: the cheapest of five candidate orders
    if let Some(c) = col.as_mut() {
        let words = sm.cols.words() as u64;
        if nsr >= 500
            && c.rx.len() < c.row_samples_cap
            && c.order_matrices < 2
            && (sm.reducers.len() + nsr) as u64 * words <= 1u64 << 26
        {
            let nnz: Vec<f64> = (0..nsr)
                .map(|i| packed[i].as_ref().unwrap().nnz() as f64)
                .collect();
            let span: Vec<f64> = (0..nsr)
                .map(|i| {
                    let r = packed[i].as_ref().unwrap();
                    (r.end - r.start) as f64
                })
                .collect();
            let mut orders: Vec<Vec<usize>> = Vec::new();
            orders.push(base.clone());
            let mut o = base.clone();
            o.sort_by(|&x, &y| nnz[x].partial_cmp(&nnz[y]).unwrap().then(x.cmp(&y)));
            orders.push(o);
            let mut o = base.clone();
            o.sort_by(|&x, &y| span[x].partial_cmp(&span[y]).unwrap().then(x.cmp(&y)));
            orders.push(o);
            let mut o = base.clone();
            o.reverse();
            orders.push(o);
            let mut o = base.clone();
            o.sort_by(|&x, &y| {
                (nnz[x] * span[x])
                    .partial_cmp(&(nnz[y] * span[y]))
                    .unwrap()
                    .then(x.cmp(&y))
            });
            orders.push(o);
            let mut best = 0usize;
            let mut best_x = u64::MAX;
            let mut def_x = 0u64;
            for (k, ord) in orders.iter().enumerate() {
                let mut e2 = el.clone_state();
                let x0 = e2.xors;
                for &i in ord {
                    e2.reduce(packed[i].as_ref().unwrap().clone_row());
                }
                let x = e2.xors - x0;
                stats.label_xors += x;
                if k == 0 {
                    def_x = x;
                }
                if x < best_x {
                    best_x = x;
                    best = k;
                }
            }
            c.order_wins[best] += 1;
            c.order_matrices += 1;
            c.order_best_xors += best_x;
            c.order_default_xors += def_x;
            let mut pos = vec![0.0f64; nsr];
            for (p, &i) in orders[best].iter().enumerate() {
                pos[i] = p as f64 / nsr as f64;
            }
            let stride = (nsr / 400).max(1);
            for i in (0..nsr).step_by(stride) {
                if c.rx.len() < c.row_samples_cap {
                    c.rx.push(feats[i]);
                    c.ry.push(pos[i]);
                }
            }
        }
    }
    // the order this arm uses
    let order: Vec<usize> = if learned_order && nsr > 0 {
        let m = model.expect("learned order needs a model");
        let mut sc: Vec<(f64, usize)> = (0..nsr).map(|i| (m.row_score(&feats[i]), i)).collect();
        sc.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap().then(x.1.cmp(&y.1)));
        stats.policy_ops += (nsr as u64) * (2 * NR as u64)
            + (nsr as u64) * 2 * ((nsr as f64).log2().ceil() as u64).max(1);
        sc.into_iter().map(|x| x.1).collect()
    } else if random_order && nsr > 0 {
        stats.policy_ops += nsr as u64 * (1 + 2 * ((nsr as f64).log2().ceil() as u64).max(1));
        let mut sc: Vec<(u64, usize)> = (0..nsr).map(|i| (rng.next(), i)).collect();
        sc.sort();
        sc.into_iter().map(|x| x.1).collect()
    } else {
        base
    };
    let x0 = el.xors;
    for (k, &i) in order.iter().enumerate() {
        if k % 256 == 0 {
            if deadline.elapsed().as_secs_f64() > wall_s {
                stats.xors += el.xors - x0;
                return Err(Outcome::CapWall);
            }
            if stats.touches() + (el.xors - x0) > max_touch {
                stats.xors += el.xors - x0;
                return Err(Outcome::CapTouch);
            }
        }
        el.reduce(packed[i].take().unwrap());
    }
    stats.xors += el.xors - x0;
    let mut fresh = Vec::new();
    for (k, row) in el.rows.iter().enumerate() {
        let lead = el.leads[k];
        if sm.no_div.contains(&sm.cols.monos[lead]) {
            fresh.push(sm.cols.unpack(row));
        }
    }
    Ok(fresh)
}

// ---------------------------------------------------------------- the F4 run

struct RunOut {
    outcome: Outcome,
    stats: Stats,
    basis_hash: u64,
    basis_len: usize,
    nstd: u64,
    useful_trace: Vec<TraceKey>,
    wall_ms: u128,
}

/// Choose the pairs of the next step among `cand` (indices into `st.pairs`,
/// all of the minimal degree).  Returns a keep-mask over `cand`.
fn select_pairs(
    pol: &Policy,
    st: &St,
    cand: &[usize],
    step: usize,
    cx: &Ctx,
    stats: &mut Stats,
    rng: &mut Rng,
) -> Vec<bool> {
    let all = vec![true; cand.len()];
    match &pol.pair {
        PairPol::All | PairPol::Oracle => all,
        PairPol::Learned(q) => {
            let m = pol.model.as_ref().expect("learned selection needs a model");
            let n = cand.len();
            stats.policy_ops += n as u64 * OPS_PER_PAIR_SCORE
                + 2 * n as u64 * ((n as f64).log2().ceil() as u64).max(1);
            let mut sc: Vec<(f64, usize)> = (0..n)
                .map(|k| (m.score(&pair_feats(st, &st.pairs[cand[k]], step, cx)), k))
                .collect();
            sc.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap().then(x.1.cmp(&y.1)));
            let take = ((*q * n as f64).ceil() as usize).clamp(1, n);
            let mut keep = vec![false; n];
            for &(_, k) in sc.iter().take(take) {
                keep[k] = true;
            }
            keep
        }
        PairPol::Trace(set) => {
            stats.policy_ops += cand.len() as u64 * OPS_PER_TRACE_TEST;
            let keep: Vec<bool> = cand
                .iter()
                .map(|&i| set.contains(&trace_key(st, &st.pairs[i])))
                .collect();
            if keep.iter().any(|&k| k) {
                keep
            } else {
                all
            }
        }
        PairPol::Random(q) => {
            let n = cand.len();
            stats.policy_ops += n as u64 + 2 * n as u64 * ((n as f64).log2().ceil() as u64).max(1);
            let mut sc: Vec<(u64, usize)> = (0..n).map(|k| (rng.next(), k)).collect();
            sc.sort();
            let take = ((*q * n as f64).ceil() as usize).clamp(1, n);
            let mut keep = vec![false; n];
            for &(_, k) in sc.iter().take(take) {
                keep[k] = true;
            }
            keep
        }
    }
}

struct Draw {
    nv: usize,
    ncoord: usize,
    eqs: Vec<KPoly>,
}

fn run_f4(
    draw: &Draw,
    pol: &Policy,
    caps: &Caps,
    col: &mut Option<Collector>,
    rng: &mut Rng,
) -> RunOut {
    let t0 = Instant::now();
    let mut stats = Stats::default();
    let mut trace: Vec<TraceKey> = Vec::new();
    let coord_mask = if draw.ncoord >= 64 {
        u64::MAX
    } else {
        (1u64 << draw.ncoord) - 1
    };
    let finish =
        |outcome: Outcome, stats: Stats, h: u64, bl: usize, nstd: u64, trace: Vec<TraceKey>| {
            RunOut {
                outcome,
                stats,
                basis_hash: h,
                basis_len: bl,
                nstd,
                useful_trace: trace,
                wall_ms: t0.elapsed().as_millis(),
            }
        };
    // initial echelon of the equations
    let cols = Columns::from_monomials(draw.eqs.iter().flat_map(|p| p.iter().copied()).collect());
    let mut el = Elim::new(cols.monos.len());
    for p in &draw.eqs {
        el.reduce(cols.pack(p));
    }
    stats.xors += el.xors;
    stats.rows_built += draw.eqs.len() as u64;
    stats.mat_words += draw.eqs.len() as u64 * cols.words() as u64;
    let mut start: Vec<KPoly> = el.rows.iter().map(|r| cols.unpack(r)).collect();
    if start.iter().any(|p| p.len() == 1 && p[0] == 0) {
        return finish(Outcome::Unsat, stats, 1, 1, 0, trace);
    }
    start.sort_by_key(|p| mkey(p[0]));
    let mut st = St::new();
    st.insert_batch(start);
    let cx = Ctx {
        nv: draw.nv,
        coord_mask,
        n_start: st.polys.len(),
    };
    let _ = cx.nv;
    let mut step = 0usize;
    while !st.pairs.is_empty() {
        if t0.elapsed().as_secs_f64() > caps.wall_s {
            stats.pairs_left = st.pairs.len() as u64;
            return finish(Outcome::CapWall, stats, 0, 0, 0, trace);
        }
        if stats.touches() > caps.max_touch {
            stats.pairs_left = st.pairs.len() as u64;
            return finish(Outcome::CapTouch, stats, 0, 0, 0, trace);
        }
        let d = st.pairs.iter().map(|p| p.deg).min().unwrap();
        let cand: Vec<usize> = (0..st.pairs.len())
            .filter(|&i| st.pairs[i].deg == d)
            .collect();
        let mut keep = select_pairs(pol, &st, &cand, step, &cx, &mut stats, rng);
        let mut oracle_rows: Option<()> = None;
        let _ = &mut oracle_rows;
        // the oracle arm: label the full step offline, keep the useful pairs
        if matches!(pol.pair, PairPol::Oracle) {
            let sel_all: Vec<Pair> = cand.iter().map(|&i| st.pairs[i]).collect();
            let sm_all = build_step(&st, &sel_all);
            if (sm_all.reducers.len() + sm_all.srows.len()) as u64 * sm_all.cols.words() as u64
                > caps.max_matrix_words
            {
                stats.pairs_left = st.pairs.len() as u64;
                return finish(Outcome::CapMatrix, stats, 0, 0, 0, trace);
            }
            let labels = label_pass(&sm_all, &sel_all, &mut stats);
            if labels.iter().any(|&l| l) {
                keep = labels;
            }
        }
        let selected: Vec<Pair> = cand
            .iter()
            .zip(&keep)
            .filter(|(_, &k)| k)
            .map(|(&i, _)| st.pairs[i])
            .collect();
        stats.pairs_deferred += (cand.len() - selected.len()) as u64;
        // remove the selected pairs from the queue
        let mut remove = vec![false; st.pairs.len()];
        for (&i, &k) in cand.iter().zip(&keep) {
            if k {
                remove[i] = true;
            }
        }
        let mut pi = 0usize;
        st.pairs.retain(|_| {
            let r = remove[pi];
            pi += 1;
            !r
        });
        step += 1;
        stats.steps += 1;
        stats.degree_reached = stats.degree_reached.max(d);
        for p in &selected {
            if p.field {
                stats.field_pairs_reduced += 1;
            } else {
                stats.pairs_reduced += 1;
            }
        }
        let tb = Instant::now();
        let sm = build_step(&st, &selected);
        if std::env::var("B09_TIMING").is_ok() {
            eprintln!(
                "step {step} deg {d} pairs {} build {:.2}s rows {} cols {}",
                selected.len(),
                tb.elapsed().as_secs_f64(),
                sm.reducers.len() + sm.srows.len(),
                sm.cols.monos.len()
            );
        }
        stats.divisor_tests += sm.divisor_tests;
        let n_rows = sm.reducers.len() + sm.srows.len();
        let words = sm.cols.words() as u64;
        if n_rows as u64 * words > caps.max_matrix_words {
            stats.pairs_left = st.pairs.len() as u64 + selected.len() as u64;
            return finish(Outcome::CapMatrix, stats, 0, 0, 0, trace);
        }
        stats.reducer_rows += sm.reducers.len() as u64;
        stats.rows_built += n_rows as u64;
        stats.mat_words += n_rows as u64 * words;
        stats.rows_max = stats.rows_max.max(n_rows as u64);
        stats.cols_max = stats.cols_max.max(sm.cols.monos.len() as u64);
        stats.peak_matrix_bytes = stats.peak_matrix_bytes.max(n_rows as u64 * words * 8);
        // training labels for the incumbent's run
        if col.is_some() {
            let labels = label_pass(&sm, &selected, &mut stats);
            let c = col.as_mut().unwrap();
            let stride = if selected.len() > c.pair_samples_cap {
                selected.len() / c.pair_samples_cap + 1
            } else {
                1
            };
            for (i, p) in selected.iter().enumerate() {
                if labels[i] {
                    trace.push(trace_key(&st, p));
                }
                if i % stride == 0 {
                    c.px.push(pair_feats(&st, p, step - 1, &cx));
                    c.py.push(labels[i] as u8);
                }
            }
        }
        let started = t0;
        let fresh = match eliminate_step(
            &sm,
            pol.model.as_ref(),
            pol.learned_order,
            pol.random_order,
            rng,
            col,
            &mut stats,
            &started,
            caps.wall_s,
            caps.max_touch,
        ) {
            Ok(f) => f,
            Err(o) => {
                stats.pairs_left = st.pairs.len() as u64;
                return finish(o, stats, 0, 0, 0, trace);
            }
        };
        if fresh.iter().any(|p| p.len() == 1 && p[0] == 0) {
            stats.solving_degree = stats.solving_degree.max(d);
            stats.pairs_left = st.pairs.len() as u64;
            return finish(Outcome::Unsat, stats, 1, 1, 0, trace);
        }
        if !fresh.is_empty() {
            stats.solving_degree = stats.solving_degree.max(d);
        }
        let mut fresh = fresh;
        fresh.sort_by_key(|p| mkey(p[0]));
        stats.new_elements += fresh.len() as u64;
        st.insert_batch(fresh);
    }
    stats.pairs_left = 0;
    // complete: reduced basis and the standard-monomial count
    let active: Vec<KPoly> = (0..st.polys.len())
        .filter(|&g| st.active[g])
        .map(|g| st.polys[g].clone())
        .collect();
    let basis = reduce_basis(active);
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for p in &basis {
        h = (h ^ kpoly_hash(p)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    let lms: Vec<u64> = basis.iter().map(|p| p[0]).collect();
    let nstd = count_standard(&lms, draw.nv);
    finish(Outcome::Sat, stats, h, basis.len(), nstd, trace)
}

/// The reduced Groebner basis from an antichain of leading monomials: every
/// tail term divisible by another leading monomial is rewritten away.
fn reduce_basis(mut elems: Vec<KPoly>) -> Vec<KPoly> {
    elems.sort_by_key(|p| mkey(p[0]));
    elems.reverse(); // ascending monomial order: smallest leading monomial first
    let mut done: Vec<KPoly> = Vec::new();
    for g in elems {
        let lm = g[0];
        let mut terms: FxSet<u64> = g[1..].iter().copied().collect();
        loop {
            let mut hit: Option<(u64, usize)> = None;
            for &t in terms.iter() {
                for (hi, h) in done.iter().enumerate() {
                    if h[0] & !t == 0 {
                        hit = Some((t, hi));
                        break;
                    }
                }
                if hit.is_some() {
                    break;
                }
            }
            let Some((t, hi)) = hit else { break };
            // g += (t \ LM_h) * h  (cancels t, adds smaller terms)
            let mult = t & !done[hi][0];
            for &s in &done[hi] {
                let u = s | mult;
                if !terms.remove(&u) {
                    terms.insert(u);
                }
            }
        }
        let mut v: Vec<u64> = terms.into_iter().collect();
        v.sort_unstable_by_key(|&m| mkey(m));
        v.insert(0, lm);
        done.push(v);
    }
    done.sort_by_key(|p| mkey(p[0]));
    done
}

/// Number of squarefree monomials in `nv` variables divisible by no leading
/// monomial (= the number of solutions of the boolean ideal).
fn count_standard(lms: &[u64], nv: usize) -> u64 {
    fn rec(cur: u64, next: usize, nv: usize, lms: &[u64], count: &mut u64) {
        *count += 1;
        for v in next..nv {
            let m = cur | (1u64 << v);
            if lms.iter().all(|&l| l & !m != 0) {
                rec(m, v + 1, nv, lms, count);
            }
        }
    }
    let mut c = 0u64;
    if lms.iter().all(|&l| l != 0) {
        rec(0, 0, nv, lms, &mut c);
    }
    c
}

// ---------------------------------------------------------------- cells and draws

const N_TRAIN: usize = 32;
const N_TEST: usize = 32;
/// Test targets generated per cell (the first 32 are the design's; more only
/// where many targets are satisfiable and so censored at the incumbent).
const N_TEST_POOL: usize = 64;
const N_PLANTED: usize = 8;

struct Cell {
    n: u32,
    l: usize,
    m: usize,
    gf: Gf,
    basis: Vec<u64>,
    targets: Vec<u64>,
    planted: Vec<u64>,
}

fn make_cell(n: u32, l: usize, m: usize, seed: u64) -> Cell {
    let gf = Gf::new(n);
    let mut rng = Rng(seed ^ ((n as u64) << 40) ^ ((l as u64) << 32) ^ ((m as u64) << 48));
    let mask = (1u64 << n) - 1;
    let mut basis: Vec<u64> = Vec::new();
    let mut ech: Vec<u64> = Vec::new();
    while basis.len() < l {
        let v = rng.next() & mask;
        let mut r = v;
        for &e in &ech {
            r = r.min(r ^ e);
        }
        if r == 0 {
            continue;
        }
        ech.push(r);
        ech.sort_unstable_by(|x, y| y.cmp(x));
        basis.push(v);
    }
    let mut targets: Vec<u64> = Vec::new();
    while targets.len() < N_TRAIN + N_TEST_POOL {
        let x = rng.next() & mask;
        if x != 0 && !targets.contains(&x) {
            targets.push(x);
        }
    }
    // planted: a decomposable target (x_1..x_m in the span, the chain solved)
    let span_elem = |rng: &mut Rng| -> u64 {
        let c = rng.next() & ((1u64 << l) - 1);
        (0..l)
            .filter(|t| (c >> t) & 1 == 1)
            .fold(0u64, |a, t| a ^ basis[t])
    };
    let mut planted: Vec<u64> = Vec::new();
    let mut guard = 0;
    while planted.len() < N_PLANTED && guard < 100000 {
        guard += 1;
        let x: Vec<u64> = (0..m).map(|_| span_elem(&mut rng)).collect();
        if x.iter().any(|&v| v == 0) {
            continue;
        }
        let mut u = x[0];
        let mut ok = true;
        // u = root of S3(x0, x1, .), then S3(u, x_i, .) for i = 2..m-1
        for xi in x.iter().skip(1) {
            let rs = gf.roots(u, *xi);
            if rs.is_empty() {
                ok = false;
                break;
            }
            u = rs[(rng.next() as usize) % rs.len()];
        }
        if ok && u != 0 && !targets.contains(&u) && !planted.contains(&u) {
            planted.push(u);
        }
    }
    Cell {
        n,
        l,
        m,
        gf,
        basis,
        targets,
        planted,
    }
}

fn cell_target(c: &Cell, role: &str, idx: usize) -> u64 {
    match role {
        "train" => c.targets[idx],
        "test" => c.targets[N_TRAIN + idx],
        "planted" => c.planted[idx],
        _ => panic!("role must be train, test or planted"),
    }
}

fn draw_for(c: &Cell, xr: u64) -> Draw {
    let (eqs, nv) = build_system(&c.gf, c.m, &c.basis, xr);
    Draw {
        nv,
        ncoord: c.m * c.l,
        eqs: eqs.iter().map(to_kpoly).collect(),
    }
}

fn caps_from(a: &Args) -> Caps {
    Caps {
        max_touch: a.num("--cap", 20_000_000_000u64),
        max_matrix_words: a.num("--max-matrix-words", 1u64 << 27),
        wall_s: a.num("--wall", 150.0f64),
        train_clone_words: 1u64 << 25,
    }
}

// ---------------------------------------------------------------- linear algebra for the fits

fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for c in 0..n {
        let mut p = c;
        for r in c + 1..n {
            if a[r][c].abs() > a[p][c].abs() {
                p = r;
            }
        }
        a.swap(c, p);
        b.swap(c, p);
        let d = a[c][c];
        if d.abs() < 1e-14 {
            continue;
        }
        for r in 0..n {
            if r != c {
                let f = a[r][c] / d;
                if f != 0.0 {
                    for k in c..n {
                        a[r][k] -= f * a[c][k];
                    }
                    b[r] -= f * b[c];
                }
            }
        }
    }
    (0..n)
        .map(|i| {
            if a[i][i].abs() < 1e-14 {
                0.0
            } else {
                b[i] / a[i][i]
            }
        })
        .collect()
}

fn standardize<const D: usize>(xs: &[[f64; D]]) -> (Vec<f64>, Vec<f64>) {
    let n = xs.len().max(1) as f64;
    let mut mu = vec![0.0; D];
    for x in xs {
        for i in 0..D {
            mu[i] += x[i];
        }
    }
    for v in mu.iter_mut() {
        *v /= n;
    }
    let mut sd = vec![0.0; D];
    for x in xs {
        for i in 0..D {
            sd[i] += (x[i] - mu[i]) * (x[i] - mu[i]);
        }
    }
    for v in sd.iter_mut() {
        *v = (*v / n).sqrt();
        if *v < 1e-9 {
            *v = 0.0;
        }
    }
    (mu, sd)
}

fn zrow<const D: usize>(x: &[f64; D], mu: &[f64], sd: &[f64]) -> [f64; D] {
    let mut z = [0.0; D];
    for i in 0..D {
        z[i] = if sd[i] > 0.0 {
            (x[i] - mu[i]) / sd[i]
        } else {
            1.0
        };
    }
    z
}

struct FitReport {
    n_pairs: usize,
    pos_rate: f64,
    recall: f64,
    sel_rate: f64,
    precision: f64,
    n_rows: usize,
    ridge_r2: f64,
    order_wins: [u64; 5],
    order_matrices: u64,
    order_best_xors: u64,
    order_default_xors: u64,
}

fn fit_model(c: &Collector) -> (Model, FitReport) {
    let mut m = Model::default();
    // (b) logistic regression by Newton steps, ridge 1e-3
    let (mu, sd) = standardize(&c.px);
    let zs: Vec<[f64; NF]> = c.px.iter().map(|x| zrow(x, &mu, &sd)).collect();
    let npos = c.py.iter().filter(|&&y| y == 1).count();
    let mut w = vec![0.0f64; NF];
    if npos > 0 && npos < c.py.len() {
        let lam = 1e-3 * zs.len() as f64;
        for _ in 0..12 {
            let mut g = vec![0.0; NF];
            let mut h = vec![vec![0.0; NF]; NF];
            for (z, &y) in zs.iter().zip(&c.py) {
                let s: f64 = (0..NF).map(|i| w[i] * z[i]).sum();
                let p = 1.0 / (1.0 + (-s).exp());
                let wt = (p * (1.0 - p)).max(1e-6);
                for i in 0..NF {
                    g[i] += (y as f64 - p) * z[i];
                    for j in 0..NF {
                        h[i][j] += wt * z[i] * z[j];
                    }
                }
            }
            for i in 0..NF {
                g[i] -= lam * w[i];
                h[i][i] += lam;
            }
            let step = solve_linear(h, g);
            for i in 0..NF {
                w[i] += step[i];
            }
        }
    }
    m.sel_mu = mu;
    m.sel_sd = sd;
    m.sel_w = w.clone();
    // threshold: keep 98 percent of the useful pairs
    let scores: Vec<f64> = zs
        .iter()
        .map(|z| (0..NF).map(|i| w[i] * z[i]).sum())
        .collect();
    let mut pos_scores: Vec<f64> = scores
        .iter()
        .zip(&c.py)
        .filter(|(_, &y)| y == 1)
        .map(|(s, _)| *s)
        .collect();
    pos_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    m.sel_tau = if pos_scores.is_empty() {
        f64::NEG_INFINITY
    } else {
        pos_scores[(pos_scores.len() as f64 * 0.02) as usize]
    };
    let sel: Vec<bool> = scores.iter().map(|&s| s >= m.sel_tau).collect();
    let nsel = sel.iter().filter(|&&b| b).count();
    let tp = sel.iter().zip(&c.py).filter(|(&s, &y)| s && y == 1).count();
    m.sel_rate = nsel as f64 / sel.len().max(1) as f64;
    // (c) ridge regression of the position in the cheapest order
    let (rmu, rsd) = standardize(&c.rx);
    let rz: Vec<[f64; NR]> = c.rx.iter().map(|x| zrow(x, &rmu, &rsd)).collect();
    let mut ata = vec![vec![0.0; NR]; NR];
    let mut aty = vec![0.0; NR];
    for (z, &y) in rz.iter().zip(&c.ry) {
        for i in 0..NR {
            aty[i] += z[i] * y;
            for j in 0..NR {
                ata[i][j] += z[i] * z[j];
            }
        }
    }
    for i in 0..NR {
        ata[i][i] += 1e-3 * rz.len().max(1) as f64;
    }
    let rw = if rz.is_empty() {
        vec![0.0; NR]
    } else {
        solve_linear(ata, aty)
    };
    let ymean = c.ry.iter().sum::<f64>() / c.ry.len().max(1) as f64;
    let (mut sse, mut sst) = (0.0, 0.0);
    for (z, &y) in rz.iter().zip(&c.ry) {
        let p: f64 = (0..NR).map(|i| rw[i] * z[i]).sum();
        sse += (y - p) * (y - p);
        sst += (y - ymean) * (y - ymean);
    }
    m.ord_mu = rmu;
    m.ord_sd = rsd;
    m.ord_w = rw;
    let rep = FitReport {
        n_pairs: c.py.len(),
        pos_rate: npos as f64 / c.py.len().max(1) as f64,
        recall: if npos > 0 {
            tp as f64 / npos as f64
        } else {
            0.0
        },
        sel_rate: m.sel_rate,
        precision: if nsel > 0 {
            tp as f64 / nsel as f64
        } else {
            0.0
        },
        n_rows: c.ry.len(),
        ridge_r2: if sst > 0.0 { 1.0 - sse / sst } else { 0.0 },
        order_wins: c.order_wins,
        order_matrices: c.order_matrices,
        order_best_xors: c.order_best_xors,
        order_default_xors: c.order_default_xors,
    };
    (m, rep)
}

// ---------------------------------------------------------------- output lines

fn run_line(
    cell: &Cell,
    role: &str,
    idx: usize,
    arm: &str,
    xr: u64,
    sols: u64,
    r: &RunOut,
) -> String {
    let s = &r.stats;
    let ok = match r.outcome {
        Outcome::Unsat => sols == 0,
        Outcome::Sat => r.nstd == sols,
        _ => true,
    };
    format!(
        "{{\"tool\":\"b09\",\"cell\":\"{}-{}-{}\",\"role\":\"{}\",\"idx\":{},\"arm\":\"{}\",\"x_r\":{},\"solutions\":{},\"outcome\":\"{}\",\"check\":{},\"touches\":{},\"xors\":{},\"mat_words\":{},\"rows\":{},\"steps\":{},\"pairs\":{},\"field_pairs\":{},\"deferred\":{},\"policy_ops\":{},\"solving_degree\":{},\"degree_reached\":{},\"rows_max\":{},\"cols_max\":{},\"divisor_tests\":{},\"label_xors\":{},\"basis_hash\":{},\"basis_len\":{},\"nstd\":{},\"pairs_left\":{},\"peak_matrix_mb\":{},\"wall_ms\":{},\"rss_kb\":{}}}",
        cell.n, cell.l, cell.m, role, idx, arm, xr, sols, r.outcome.name(), ok,
        s.touches(), s.xors, s.mat_words, s.rows_built, s.steps, s.pairs_reduced, s.field_pairs_reduced,
        s.pairs_deferred, s.policy_ops, s.solving_degree, s.degree_reached, s.rows_max, s.cols_max,
        s.divisor_tests, s.label_xors, r.basis_hash, r.basis_len, r.nstd, s.pairs_left,
        s.peak_matrix_bytes / (1 << 20), r.wall_ms, rss_peak_kb()
    )
}

fn cell_from(a: &Args) -> Cell {
    let n: u32 = a.num("--n", 17);
    let l: usize = a.num("--ell", 4);
    let m: usize = a.num("--m", 3);
    let seed: u64 = a.num("--seed", 20261018);
    make_cell(n, l, m, seed)
}

fn trace_to_text(idx: usize, t: &[TraceKey]) -> String {
    let mut v: Vec<String> = t
        .iter()
        .map(|k| format!("{},{},{}", k.0, k.1, k.2 as u8))
        .collect();
    v.sort();
    format!("T {} {}\n", idx, v.join(" "))
}

fn read_traces(path: &str) -> Vec<FxSet<TraceKey>> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut out: Vec<FxSet<TraceKey>> = Vec::new();
    for line in text.lines() {
        let mut it = line.split_whitespace();
        if it.next() != Some("T") {
            continue;
        }
        let idx: usize = it.next().unwrap().parse().unwrap();
        let mut set: FxSet<TraceKey> = FxSet::default();
        for tok in it {
            let f: Vec<&str> = tok.split(',').collect();
            set.insert((f[0].parse().unwrap(), f[1].parse().unwrap(), f[2] == "1"));
        }
        while out.len() <= idx {
            out.push(FxSet::default());
        }
        out[idx] = set;
    }
    out
}

fn parse_arm(
    arm: &str,
    model: &Option<Model>,
    traces: &Option<Vec<FxSet<TraceKey>>>,
    i: usize,
) -> Policy {
    let q =
        |name: &str| -> f64 { name[1..].parse::<f64>().expect("arm fraction in percent") / 100.0 };
    match arm {
        "a" => Policy::incumbent(),
        "c" => Policy {
            pair: PairPol::All,
            learned_order: true,
            random_order: false,
            model: model.clone(),
        },
        "d" => {
            let t = traces.as_ref().expect("arm d needs --trace");
            Policy {
                pair: PairPol::Trace(t.get(i % N_TRAIN).cloned().unwrap_or_default()),
                learned_order: false,
                random_order: false,
                model: None,
            }
        }
        "p" => Policy {
            pair: PairPol::All,
            learned_order: false,
            random_order: true,
            model: None,
        },
        "o" => Policy {
            pair: PairPol::Oracle,
            learned_order: false,
            random_order: false,
            model: None,
        },
        x if x.starts_with('b') => Policy {
            pair: PairPol::Learned(q(x)),
            learned_order: false,
            random_order: false,
            model: model.clone(),
        },
        x if x.starts_with('r') => Policy {
            pair: PairPol::Random(q(x)),
            learned_order: false,
            random_order: false,
            model: None,
        },
        other => panic!("unknown arm {other}"),
    }
}

fn mode_train(a: &Args) {
    let cell = cell_from(a);
    let caps = caps_from(a);
    let out = a.flag("--out").expect("--out PREFIX");
    let lo: usize = a.num("--from", 0);
    let hi: usize = a.num("--to", N_TRAIN);
    let pair_cap: usize = a.num("--pair-cap", 1000usize);
    let row_cap: usize = a.num("--row-cap", 300usize);
    let mut samples = String::new();
    let mut traces = String::new();
    let mut ow = [0u64; 5];
    let (mut om, mut obx, mut odx) = (0u64, 0u64, 0u64);
    let mut rng = Rng(7);
    for i in lo..hi {
        let xr = cell_target(&cell, "train", i);
        let sols = count_solutions(&cell.gf, cell.m, &cell.basis, xr);
        let draw = draw_for(&cell, xr);
        let mut c = Some(Collector {
            pair_samples_cap: pair_cap,
            row_samples_cap: row_cap,
            ..Collector::default()
        });
        let r = run_f4(&draw, &Policy::incumbent(), &caps, &mut c, &mut rng);
        let c = c.unwrap();
        println!("{}", run_line(&cell, "train", i, "a", xr, sols, &r));
        traces.push_str(&trace_to_text(i, &r.useful_trace));
        for (x, y) in c.px.iter().zip(&c.py) {
            samples.push_str(&format!("P {y}"));
            for v in x {
                samples.push_str(&format!(" {}", *v as f32));
            }
            samples.push('\n');
        }
        for (x, y) in c.rx.iter().zip(&c.ry) {
            samples.push_str(&format!("R {}", *y as f32));
            for v in x {
                samples.push_str(&format!(" {}", *v as f32));
            }
            samples.push('\n');
        }
        for k in 0..5 {
            ow[k] += c.order_wins[k];
        }
        om += c.order_matrices;
        obx += c.order_best_xors;
        odx += c.order_default_xors;
    }
    samples.push_str(&format!(
        "O {} {} {} {} {} {} {} {}\n",
        ow[0], ow[1], ow[2], ow[3], ow[4], om, obx, odx
    ));
    std::fs::write(format!("{out}.samples.{lo}"), samples).expect("write samples");
    std::fs::write(format!("{out}.trace.{lo}"), traces).expect("write trace");
}

fn mode_fit(a: &Args) {
    let cell = cell_from(a);
    let out = a.flag("--out").expect("--out PREFIX");
    let files: Vec<String> = a
        .flag("--samples")
        .expect("--samples f1,f2")
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let mut col = Collector::default();
    for f in &files {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        for line in text.lines() {
            let mut it = line.split_whitespace();
            match it.next() {
                Some("P") => {
                    let y: u8 = it.next().unwrap().parse().unwrap();
                    let mut x = [0.0f64; NF];
                    for k in 0..NF {
                        x[k] = it.next().unwrap().parse().unwrap();
                    }
                    col.px.push(x);
                    col.py.push(y);
                }
                Some("R") => {
                    let y: f64 = it.next().unwrap().parse().unwrap();
                    let mut x = [0.0f64; NR];
                    for k in 0..NR {
                        x[k] = it.next().unwrap().parse().unwrap();
                    }
                    col.rx.push(x);
                    col.ry.push(y);
                }
                Some("O") => {
                    let v: Vec<u64> = it.map(|t| t.parse().unwrap()).collect();
                    for k in 0..5 {
                        col.order_wins[k] += v[k];
                    }
                    col.order_matrices += v[5];
                    col.order_best_xors += v[6];
                    col.order_default_xors += v[7];
                }
                _ => {}
            }
        }
    }
    let (model, rep) = fit_model(&col);
    std::fs::write(format!("{out}.model"), model.to_text()).expect("write model");
    println!(
        "{{\"tool\":\"b09\",\"mode\":\"fit\",\"cell\":\"{}-{}-{}\",\"pair_samples\":{},\"pos_rate\":{:.6},\"recall_at_tau\":{:.4},\"sel_rate_at_tau\":{:.4},\"precision_at_tau\":{:.4},\"row_samples\":{},\"ridge_r2\":{:.4},\"order_wins\":{:?},\"order_matrices\":{},\"order_best_xors\":{},\"order_default_xors\":{}}}",
        cell.n, cell.l, cell.m, rep.n_pairs, rep.pos_rate, rep.recall, rep.sel_rate, rep.precision,
        rep.n_rows, rep.ridge_r2, rep.order_wins, rep.order_matrices, rep.order_best_xors, rep.order_default_xors
    );
}

fn mode_eval(a: &Args) {
    let cell = cell_from(a);
    let mut caps = caps_from(a);
    let role = a.flag("--role").unwrap_or_else(|| "test".to_string());
    let arms: Vec<String> = a
        .flag("--arms")
        .unwrap_or_else(|| "a".to_string())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let lo: usize = a.num("--from", 0);
    let hi: usize = a.num("--to", N_TEST);
    let rel_cap: f64 = a.num("--rel-cap", 1.5f64);
    let model = a.flag("--model").map(|p| {
        Model::from_text(&std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{p}: {e}")))
    });
    let traces: Option<Vec<FxSet<TraceKey>>> = a.flag("--trace").map(|p| {
        let mut all: Vec<FxSet<TraceKey>> = Vec::new();
        for f in p.split(',') {
            let t = read_traces(f);
            for (i, set) in t.into_iter().enumerate() {
                while all.len() <= i {
                    all.push(FxSet::default());
                }
                if !set.is_empty() {
                    all[i] = set;
                }
            }
        }
        all
    });
    let base_cap = caps.max_touch;
    for i in lo..hi {
        let xr = cell_target(&cell, &role, i);
        let sols = count_solutions(&cell.gf, cell.m, &cell.basis, xr);
        let draw = draw_for(&cell, xr);
        let mut a_touches: Option<u64> = None;
        for arm in &arms {
            let pol = parse_arm(arm, &model, &traces, i);
            let mut rng = Rng(0x5EED ^ (i as u64) << 8);
            let mut none: Option<Collector> = None;
            caps.max_touch = match (arm.as_str(), a_touches) {
                ("a", _) | (_, None) => base_cap,
                (_, Some(t)) => ((t as f64 * rel_cap) as u64).min(base_cap),
            };
            let r = run_f4(&draw, &pol, &caps, &mut none, &mut rng);
            if arm == "a" && (r.outcome == Outcome::Unsat || r.outcome == Outcome::Sat) {
                a_touches = Some(r.stats.touches());
            }
            println!("{}", run_line(&cell, &role, i, arm, xr, sols, &r));
            if arm == "a" && a_touches.is_none() {
                break; // the incumbent hit a cap: the target is censored, no arm is run on it
            }
        }
    }
}

// ---------------------------------------------------------------- report

fn jraw<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    Some(line[i..].trim_start())
}
fn jn(line: &str, key: &str) -> Option<f64> {
    let s = jraw(line, key)?;
    let end = s
        .find(|c: char| {
            !(c.is_ascii_digit() || c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+')
        })
        .unwrap_or(s.len());
    s[..end].parse().ok()
}
fn js(line: &str, key: &str) -> Option<String> {
    let s = jraw(line, key)?.strip_prefix('"')?;
    Some(s[..s.find('"')?].to_string())
}

#[derive(Clone)]
struct Rec {
    cell: String,
    role: String,
    idx: usize,
    arm: String,
    outcome: String,
    touches: f64,
    rows: f64,
    ops: f64,
    hash: u64,
    check: bool,
    sols: f64,
}

fn percentile(v: &mut Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() {
        return f64::NAN;
    }
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn ratio_ci(ca: &[f64], cb: &[f64], seed: u64) -> (f64, f64, f64, f64) {
    let sum = |v: &[f64]| v.iter().sum::<f64>();
    let point = sum(ca) / sum(cb).max(1.0);
    let mut rng = Rng(seed);
    let mut bs: Vec<f64> = Vec::new();
    for _ in 0..2000 {
        let (mut sa, mut sb) = (0.0, 0.0);
        for _ in 0..ca.len() {
            let k = (rng.next() as usize) % ca.len().max(1);
            sa += ca[k];
            sb += cb[k];
        }
        bs.push(sa / sb.max(1.0));
    }
    let lo = percentile(&mut bs.clone(), 0.025);
    let hi = percentile(&mut bs, 0.975);
    let mut per: Vec<f64> = ca.iter().zip(cb).map(|(x, y)| x / y.max(1.0)).collect();
    (point, lo, hi, percentile(&mut per, 0.5))
}

/// Cost of an arm run at weight `w`; a run that hit a cap is censored and
/// counted at the cap (1.5 times the incumbent's touches), a lower bound.
fn cens_cost(y: &Rec, a: &Rec, w: f64) -> f64 {
    if y.outcome == "unsat" || y.outcome == "sat" {
        y.touches + w * y.ops
    } else {
        y.touches.max(1.5 * a.touches) + w * y.ops
    }
}

fn mode_report(a: &Args) {
    let mut recs: Vec<Rec> = Vec::new();
    for f in a.v.iter().skip(2).filter(|s| !s.starts_with("--")) {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("skip {f}: {e}");
                continue;
            }
        };
        for line in text.lines() {
            if !line.contains("\"arm\":") {
                continue;
            }
            let Some(h) = jn(line, "basis_hash") else {
                continue;
            };
            recs.push(Rec {
                cell: js(line, "cell").unwrap_or_default(),
                role: js(line, "role").unwrap_or_default(),
                idx: jn(line, "idx").unwrap_or(0.0) as usize,
                arm: js(line, "arm").unwrap_or_default(),
                outcome: js(line, "outcome").unwrap_or_default(),
                touches: jn(line, "touches").unwrap_or(0.0),
                rows: jn(line, "rows").unwrap_or(0.0),
                ops: jn(line, "policy_ops").unwrap_or(0.0),
                hash: h as u64,
                check: jraw(line, "check").map_or(false, |s| s.starts_with("true")),
                sols: jn(line, "solutions").unwrap_or(0.0),
            });
        }
    }
    let bad = recs.iter().filter(|r| !r.check).count();
    println!("records {} ; check failures {}", recs.len(), bad);
    let mut cells: Vec<String> = recs.iter().map(|r| r.cell.clone()).collect();
    cells.sort();
    cells.dedup();
    let weights = [0.0f64, 0.1, 1.0];
    let done = |r: &Rec| r.outcome == "unsat" || r.outcome == "sat";
    let usable = |r: &Rec| done(r) || r.outcome.starts_with("cap");
    // verdict table: (arm, literal win count, tightened win count) over the design cells
    let design_cells: Vec<&String> = cells
        .iter()
        .filter(|c| ["17-4-3", "19-4-3", "21-5-3", "13-3-4"].contains(&c.as_str()))
        .collect();
    let mut lit: HashMap<String, usize> = HashMap::new();
    let mut tight: HashMap<String, usize> = HashMap::new();
    for role in ["test", "planted"] {
        println!("== role {role}");
        for cell in &cells {
            let find = |arm: &str, idx: usize| {
                recs.iter()
                    .find(|r| &r.cell == cell && r.role == role && r.arm == arm && r.idx == idx)
            };
            let mut idxs: Vec<usize> = recs
                .iter()
                .filter(|r| &r.cell == cell && r.role == role && r.arm == "a")
                .map(|r| r.idx)
                .collect();
            idxs.sort();
            idxs.dedup();
            if idxs.is_empty() {
                continue;
            }
            let n_cens = idxs
                .iter()
                .filter(|&&i| find("a", i).map_or(false, |r| !done(r)))
                .count();
            let nsat = idxs
                .iter()
                .filter(|&&i| find("a", i).map_or(false, |r| r.sols > 0.0))
                .count();
            println!(
                "cell {cell}: {} targets, {} satisfiable, {} censored at the incumbent",
                idxs.len(),
                nsat,
                n_cens
            );
            let mut arms: Vec<String> = recs
                .iter()
                .filter(|r| &r.cell == cell && r.role == role && r.arm != "a")
                .map(|r| r.arm.clone())
                .collect();
            arms.sort();
            arms.dedup();
            let mut result: HashMap<String, (bool, bool, bool)> = HashMap::new(); // (touch win, rows ok, null beaten)
            for arm in &arms {
                // pairs against the incumbent
                let pairs: Vec<(&Rec, &Rec)> = idxs
                    .iter()
                    .filter_map(|&i| match (find("a", i), find(arm, i)) {
                        (Some(x), Some(y)) if done(x) && usable(y) => Some((x, y)),
                        _ => None,
                    })
                    .collect();
                if pairs.is_empty() {
                    continue;
                }
                let n_arm_cens = pairs.iter().filter(|(_, y)| !done(y)).count();
                let both: Vec<&(&Rec, &Rec)> = pairs.iter().filter(|(_, y)| done(y)).collect();
                let nident = both.iter().filter(|(x, y)| x.hash == y.hash).count();
                let identical = nident == both.len();
                let mut line = format!("  {arm:>4} vs a: n={} (arm censored at 2x: {n_arm_cens}) identical {nident}/{}", pairs.len(), both.len());
                let mut win = identical;
                let mut all_w_ok = true;
                for (wi, &w) in weights.iter().enumerate() {
                    let ca: Vec<f64> = pairs.iter().map(|(x, _)| x.touches).collect();
                    let cb: Vec<f64> = pairs.iter().map(|(x, y)| cens_cost(y, x, w)).collect();
                    let (pt, lo, hi, med) = ratio_ci(&ca, &cb, 0xB007 + wi as u64);
                    line.push_str(&format!(" | w={w}: {pt:.3} [{lo:.3},{hi:.3}] med {med:.3}"));
                    if !(pt >= 1.3 && lo > 1.0) {
                        all_w_ok = false;
                    }
                }
                let ra: f64 = pairs.iter().map(|(x, _)| x.rows).sum();
                let rb: f64 = pairs.iter().map(|(_, y)| y.rows).sum();
                line.push_str(&format!(" | rows ratio {:.3}", ra / rb.max(1.0)));
                let rows_ok = ra >= rb;
                win &= all_w_ok;
                // matched null: b<q> against r<q>, c against p
                let null = if arm.starts_with('b') {
                    Some(format!("r{}", &arm[1..]))
                } else if arm == "c" {
                    Some("p".to_string())
                } else {
                    None
                };
                let mut beats_null = true;
                if let Some(nm) = &null {
                    let np: Vec<(&Rec, &Rec)> = idxs
                        .iter()
                        .filter_map(|&i| match (find(nm, i), find(arm, i)) {
                            (Some(x), Some(y))
                                if usable(x) && usable(y) && done(find("a", i).unwrap()) =>
                            {
                                Some((x, y))
                            }
                            _ => None,
                        })
                        .collect();
                    if np.is_empty() {
                        beats_null = false;
                    } else {
                        let mut ok_all = true;
                        line.push_str(&format!(" || null {nm} / {arm}:"));
                        for (wi, &w) in weights.iter().enumerate() {
                            let ia: Vec<&Rec> = idxs.iter().filter(|&&i| matches!((find(nm, i), find(arm, i)), (Some(x), Some(y)) if usable(x) && usable(y) && done(find("a", i).unwrap()))).map(|&i| find("a", i).unwrap()).collect();
                            let ca: Vec<f64> = np
                                .iter()
                                .zip(&ia)
                                .map(|((x, _), a0)| cens_cost(x, a0, w))
                                .collect();
                            let cb: Vec<f64> = np
                                .iter()
                                .zip(&ia)
                                .map(|((_, y), a0)| cens_cost(y, a0, w))
                                .collect();
                            let (pt, lo, hi, _) = ratio_ci(&ca, &cb, 0xB107 + wi as u64);
                            line.push_str(&format!(" w={w}: {pt:.3} [{lo:.3},{hi:.3}]"));
                            if !(lo > 1.0) {
                                ok_all = false;
                            }
                        }
                        beats_null = ok_all;
                    }
                }
                println!(
                    "{line} | touch_win {win} | rows_ok {rows_ok}{}",
                    if null.is_some() {
                        format!(" | beats_null {beats_null}")
                    } else {
                        String::new()
                    }
                );
                result.insert(arm.clone(), (win, rows_ok, beats_null));
            }
            if role == "test" {
                for (arm, (w, rok, bn)) in &result {
                    if *w {
                        *lit.entry(arm.clone()).or_insert(0) += 1;
                    }
                    if *w && *rok && *bn {
                        *tight.entry(arm.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    println!(
        "== verdicts over the {} design cells found",
        design_cells.len()
    );
    let mut arms: Vec<String> = lit.keys().chain(tight.keys()).cloned().collect();
    arms.sort();
    arms.dedup();
    for arm in arms {
        let l = lit.get(&arm).copied().unwrap_or(0);
        let t = tight.get(&arm).copied().unwrap_or(0);
        println!("arm {arm}: touch-rule cells {l} ; touch-rule and rows-not-worse and matched-null-beaten cells {t} -> literal rule {} ; frozen verdict {}", if l >= 3 { "met" } else { "not met" }, if t >= 3 { "ADVANCES (constant)" } else if l >= 3 { "TRADES or MATCHES-NULL (constant), see rows and null columns" } else { "not ADVANCES" });
    }
}

// ---------------------------------------------------------------- controls and selftest

fn grevlex_greater(a: u64, b: u64) -> bool {
    let (da, db) = (a.count_ones(), b.count_ones());
    if da != db {
        return da > db;
    }
    let diff = a ^ b;
    if diff == 0 {
        return false;
    }
    let hb = 63 - diff.leading_zeros();
    (a >> hb) & 1 == 0
}

fn points_of(draw: &Draw) -> Vec<u64> {
    // exhaustive solutions, N <= 24
    let mut out = Vec::new();
    for pt in 0..(1u64 << draw.nv) {
        if draw.eqs.iter().all(|p| eval_poly(p, pt) == 0) {
            out.push(pt);
        }
    }
    out
}

fn train_small(cell: &Cell, caps: &Caps, ntrain: usize) -> Model {
    let mut col = Collector {
        pair_samples_cap: 2000,
        row_samples_cap: 1500,
        ..Collector::default()
    };
    let mut rng = Rng(3);
    for i in 0..ntrain {
        let xr = cell_target(cell, "train", i);
        let draw = draw_for(cell, xr);
        let mut c = Some(Collector {
            pair_samples_cap: 2000,
            row_samples_cap: 1500,
            ..Collector::default()
        });
        let _ = run_f4(&draw, &Policy::incumbent(), caps, &mut c, &mut rng);
        let c = c.unwrap();
        col.px.extend(c.px);
        col.py.extend(c.py);
        col.rx.extend(c.rx);
        col.ry.extend(c.ry);
    }
    fit_model(&col).0
}

fn selftest(a: &Args) {
    let mut ok = true;
    let mut check = |name: &str, pass: bool, detail: String| {
        println!("{} {name} {detail}", if pass { "PASS" } else { "FAIL" });
        if !pass {
            ok = false;
        }
    };
    // 1. the monomial key agrees with the grevlex comparison
    let mut rng = Rng(11);
    let mut agree = true;
    for _ in 0..20000 {
        let (x, y) = (rng.next() & 0xFFFFFF, rng.next() & 0xFFFFFF);
        if x != y && (mkey(x) < mkey(y)) != grevlex_greater(x, y) {
            agree = false;
        }
    }
    check(
        "monomial-order",
        agree,
        "mkey vs grevlex_greater on 20000 pairs".into(),
    );
    // 2. small cells: every arm against exhaustive enumeration
    let caps = Caps {
        max_touch: 1 << 40,
        max_matrix_words: 1 << 27,
        wall_s: 120.0,
        train_clone_words: 1 << 25,
    };
    let cells = [(7u32, 3usize, 3usize), (8, 3, 3), (8, 2, 4)];
    let ntrain: usize = a.num("--ntrain", 8);
    for &(n, l, m) in &cells {
        let cell = make_cell(n, l, m, 424242);
        let model = train_small(&cell, &caps, ntrain);
        let traces: Vec<FxSet<TraceKey>> = (0..N_TRAIN)
            .map(|i| {
                let xr = cell_target(&cell, "train", i);
                let mut none: Option<Collector> = Some(Collector {
                    pair_samples_cap: 100,
                    row_samples_cap: 0,
                    ..Collector::default()
                });
                let r = run_f4(
                    &draw_for(&cell, xr),
                    &Policy::incumbent(),
                    &caps,
                    &mut none,
                    &mut Rng(1),
                );
                r.useful_trace.into_iter().collect()
            })
            .collect();
        let (mut n_sat, mut n_unsat, mut bad, mut hash_diff, mut nchk) = (0, 0, 0, 0, 0);
        let mut targets: Vec<(String, usize)> =
            (0..N_TRAIN).map(|i| ("train".to_string(), i)).collect();
        targets.extend((0..cell.planted.len()).map(|i| ("planted".to_string(), i)));
        for (role, i) in &targets {
            let xr = cell_target(&cell, role, *i);
            let draw = draw_for(&cell, xr);
            let sols = count_solutions(&cell.gf, m, &cell.basis, xr);
            // exhaustive count of the polynomial system itself
            if draw.nv <= 20 && *i < 3 {
                let pts = points_of(&draw);
                if pts.len() as u64 != sols {
                    bad += 1;
                    println!(
                        "  exhaustive {} vs counter {} at {role} {i}",
                        pts.len(),
                        sols
                    );
                }
            }
            let mut ref_hash = None;
            for arm in ["a", "b", "c", "d", "o", "r", "p"] {
                let pol = match arm {
                    "a" => Policy::incumbent(),
                    "b" => Policy {
                        pair: PairPol::Learned(0.5),
                        learned_order: false,
                        random_order: false,
                        model: Some(model.clone()),
                    },
                    "c" => Policy {
                        pair: PairPol::All,
                        learned_order: true,
                        random_order: false,
                        model: Some(model.clone()),
                    },
                    "d" => Policy {
                        pair: PairPol::Trace(traces[(*i + 5) % N_TRAIN].clone()),
                        learned_order: false,
                        random_order: false,
                        model: None,
                    },
                    "p" => Policy {
                        pair: PairPol::All,
                        learned_order: false,
                        random_order: true,
                        model: None,
                    },
                    "o" => Policy {
                        pair: PairPol::Oracle,
                        learned_order: false,
                        random_order: false,
                        model: None,
                    },
                    _ => Policy {
                        pair: PairPol::Random(0.5),
                        learned_order: false,
                        random_order: false,
                        model: None,
                    },
                };
                let mut none: Option<Collector> = None;
                let r = run_f4(&draw, &pol, &caps, &mut none, &mut Rng(5 + *i as u64));
                nchk += 1;
                let good = match r.outcome {
                    Outcome::Unsat => sols == 0,
                    Outcome::Sat => r.nstd == sols,
                    _ => false,
                };
                if !good {
                    bad += 1;
                    println!(
                        "  FAIL arm {arm} {role} {i}: outcome {:?} nstd {} sols {}",
                        r.outcome, r.nstd, sols
                    );
                }
                match ref_hash {
                    None => ref_hash = Some(r.basis_hash),
                    Some(h) => {
                        if h != r.basis_hash {
                            hash_diff += 1;
                            println!("  hash differs arm {arm} {role} {i}");
                        }
                    }
                }
                if arm == "a" {
                    if r.outcome == Outcome::Sat {
                        n_sat += 1;
                    } else {
                        n_unsat += 1;
                    }
                }
            }
        }
        check(
            &format!("cell-{n}-{l}-{m}"),
            bad == 0 && hash_diff == 0 && n_sat > 0 && n_unsat > 0,
            format!(
                "{nchk} runs; sat {n_sat} unsat {n_unsat}; wrong {bad}; basis differs {hash_diff}"
            ),
        );
    }
    println!("{}", if ok { "SELFTEST OK" } else { "SELFTEST FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

/// Positive control against the committed degrees: item 07's controls table.
fn mode_control(a: &Args) {
    let path = a.flag("--table").expect("--table controls.tsv");
    let max_vars: usize = a.num("--max-vars", 30);
    let caps = caps_from(a);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let (mut n, mut agree_ms, mut n_ms, mut agree_cl, mut bad) = (0, 0, 0, 0, 0);
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 10 {
            continue;
        }
        let (m, nn, l): (usize, u32, usize) = (
            f[1].parse().unwrap(),
            f[2].parse().unwrap(),
            f[3].parse().unwrap(),
        );
        let basis: Vec<u64> = f[4].split(',').map(|x| x.parse().unwrap()).collect();
        let xr: u64 = f[5].parse().unwrap();
        let sols: i64 = f[6].parse().unwrap();
        let (rc, rm): (i64, i64) = (f[7].parse().unwrap(), f[8].parse().unwrap());
        if m * l + (m - 2) * nn as usize > max_vars || sols != 0 {
            continue;
        }
        let gf = Gf::new(nn);
        let (eqs, nv) = build_system(&gf, m, &basis, xr);
        let draw = Draw {
            nv,
            ncoord: m * l,
            eqs: eqs.iter().map(to_kpoly).collect(),
        };
        let mut none: Option<Collector> = None;
        let r = run_f4(&draw, &Policy::incumbent(), &caps, &mut none, &mut Rng(1));
        let d = r.stats.degree_reached as i64;
        n += 1;
        if r.outcome != Outcome::Unsat {
            bad += 1;
        }
        if rc >= 0 && d == rc {
            agree_cl += 1;
        }
        if rm >= 0 {
            n_ms += 1;
            if d == rm {
                agree_ms += 1;
            }
        }
        println!("{{\"tool\":\"b09\",\"mode\":\"control\",\"key\":\"{}\",\"m\":{m},\"n\":{nn},\"ell\":{l},\"nv\":{nv},\"outcome\":\"{}\",\"degree_reached\":{d},\"ref_closure\":{rc},\"ref_msolve\":{rm},\"touches\":{},\"steps\":{}}}", f[0], r.outcome.name(), r.stats.touches(), r.stats.steps);
    }
    println!("control summary: {n} unsat draws; not refuted {bad}; degree equals committed closure degree {agree_cl}/{n}; equals committed msolve degree {agree_ms}/{n_ms}");
}

fn main() {
    let a = Args {
        v: std::env::args().collect(),
    };
    match a.v.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(&a),
        Some("control") => mode_control(&a),
        Some("train") => mode_train(&a),
        Some("fit") => mode_fit(&a),
        Some("eval") => mode_eval(&a),
        Some("report") => mode_report(&a),
        _ => {
            eprintln!("usage: b09 selftest|control|train|fit|eval|report ...");
            std::process::exit(2);
        }
    }
}
