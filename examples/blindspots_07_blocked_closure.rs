//! Blind-spot item 07: a memory-blocked Boolean closure (F4-equivalent) that
//! measures the solving degree `D` of the chained Semaev `S_3` descent system
//! at the deciding `(n, l, m)` cells.
//!
//! Standard library only.  At the branch base the `crypto` library does not
//! compile (see `research/blindspots_20261018/07-blocked-f4-degree-cells/
//! PROTOCOL.md`, Deviation 2), so the field arithmetic, the symbolic `S_3`
//! chain system and the exact solution counter are re-implemented here and
//! checked against the committed solution counts and closure ranks.
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
//!     -o target/blindspots-07/b07 examples/blindspots_07_blocked_closure.rs
//! b07 selftest
//! b07 count    --m 4 --n 11 --ell 4 --basis 493,1083,1298,447 --xr 33
//! b07 closure  --m 4 --n 11 --ell 4 --basis 493,1083,1298,447 --xr 33 --to 4
//! b07 find     --m 4 --n 15 --ell 5 --seed 7 --want 2
//! b07 plan     --m 4 --n 17 --ell 5
//! b07 control-table controls.tsv [--max-vars 32]
//! b07 random-control --m 4 --n 6 --ell 2 --seeds 1,2,3,4 --to 6
//! ```
//!
//! # The object computed
//!
//! The MutantXL fixed-degree closure `V_D` of the system in the Boolean
//! ring: the least subspace of polynomials of degree `<= D` that contains the
//! equations and is closed under multiplying any element of degree `< D` by a
//! variable.  `1 in V_D` is the refutation and `D` is the least such degree.
//! This is the same space the committed dense engine
//! (`ReducedBasis::from_system_closed_full`) saturates: its Macaulay rows
//! `t * f_i` (`deg t <= D - deg f_i`) are reached here by stepping one
//! variable at a time through the falls, and its "fall times monomial"
//! products are the same repeated single-variable products.
//!
//! # The engine
//!
//! * Columns are the squarefree monomials of degree `<= D`, in descending
//!   DegRevLex order, indexed by the combinatorial rank of the variable set
//!   (degree descending, then colex rank ascending).  No hash map, no column
//!   list is searched.
//! * The basis is kept in reduced row echelon form.  Row `r` stores only the
//!   words from its pivot word on (the row is zero before its pivot), which
//!   halves resident memory against a square dense layout.  Every pivot
//!   column is zero in every other row, so reducing a sparse product row is
//!   one XOR per pivot bit it carries and never cascades.
//! * Products `x_j * f` of the pending falls are formed in **row blocks** of
//!   `K` dense rows.  A block is reduced against the basis, row-echelonised
//!   against itself, and its new pivots are folded into the basis in a
//!   single pass over the basis (the blocked panel pass), so the basis is
//!   streamed once per block and not once per row.
//! * Nothing is written to disk.

use std::collections::{HashMap, VecDeque};
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

// minimal JSON field extraction for the committed JSONL files
fn jfind<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    Some(line[i..].trim_start())
}
fn jnum(line: &str, key: &str) -> Option<i64> {
    let s = jfind(line, key)?;
    let end = s
        .find(|c: char| !(c.is_ascii_digit() || c == '-'))
        .unwrap_or(s.len());
    s[..end].parse().ok()
}
fn jarr(line: &str, key: &str) -> Option<Vec<u64>> {
    let s = jfind(line, key)?;
    let s = s.strip_prefix('[')?;
    let end = s.find(']')?;
    s[..end]
        .split(',')
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().parse().ok())
        .collect()
}
fn jstr(line: &str, key: &str) -> Option<String> {
    let s = jfind(line, key)?.strip_prefix('"')?;
    let end = s.find('"')?;
    Some(s[..end].to_string())
}

fn binom_table() -> Vec<[u64; 10]> {
    let mut t = vec![[0u64; 10]; 70];
    for n in 0..70 {
        t[n][0] = 1;
        for k in 1..10 {
            if n > 0 {
                t[n][k] = t[n - 1][k - 1].saturating_add(t[n - 1][k]);
            }
        }
    }
    t
}

fn cols_up_to(nv: usize, d: usize) -> u64 {
    let b = binom_table();
    (0..=d).map(|k| b[nv][k]).sum()
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

struct Cols {
    nv: usize,
    d: usize,
    off: Vec<usize>,
    ncols: usize,
    mask_of: Vec<u64>,
    deg_of: Vec<u8>,
    binom: Vec<[u64; 10]>,
}

impl Cols {
    fn new(nv: usize, d: usize) -> Cols {
        assert!(nv <= 62 && d <= 8);
        let binom = binom_table();
        let mut off = vec![0usize; d + 1];
        for k in (0..d).rev() {
            off[k] = off[k + 1] + binom[nv][k + 1] as usize;
        }
        let ncols = off[0] + 1;
        let mut mask_of = Vec::with_capacity(ncols);
        let mut deg_of = Vec::with_capacity(ncols);
        let limit: u64 = 1u64 << nv;
        for k in (0..=d).rev() {
            if k == 0 {
                mask_of.push(0);
                deg_of.push(0);
                continue;
            }
            if k > nv {
                continue;
            }
            let mut m: u64 = (1u64 << k) - 1;
            while m < limit {
                mask_of.push(m);
                deg_of.push(k as u8);
                let c = m & m.wrapping_neg();
                let r = m + c;
                m = (((r ^ m) >> 2) / c) | r;
            }
        }
        assert_eq!(mask_of.len(), ncols, "column count");
        Cols {
            nv,
            d,
            off,
            ncols,
            mask_of,
            deg_of,
            binom,
        }
    }
    #[inline]
    fn col_of(&self, mask: u64) -> usize {
        let mut m = mask;
        let mut i = 1usize;
        let mut r = 0u64;
        while m != 0 {
            let p = m.trailing_zeros() as usize;
            r += self.binom[p][i];
            i += 1;
            m &= m - 1;
        }
        self.off[(i - 1) as usize] + r as usize
    }
    fn words(&self) -> usize {
        (self.ncols + 63) / 64
    }
}

// ---------------------------------------------------------------- engine

#[inline(always)]
fn xor_into(dst: &mut [u64], src: &[u64]) {
    for (a, b) in dst.iter_mut().zip(src.iter()) {
        *a ^= *b;
    }
}

struct Row {
    #[allow(dead_code)]
    piv: u32,
    start: u32,
    data: Vec<u64>,
}

#[derive(Default, Clone)]
struct Stats {
    products: u64,
    seed_rows: u64,
    chunks: u64,
    ops_fwd: u64,
    ops_local: u64,
    ops_pass: u64,
    bytes_pass: u64,
    hits_fwd: u64,
    new_pivots: u64,
    falls: u64,
    t_fwd: f64,
    t_local: f64,
    t_pass: f64,
}

enum Outcome {
    Refuted,
    Saturated,
    TimeLimit,
    MemCap,
}

struct Engine<'a> {
    cols: &'a Cols,
    w: usize,
    kchunk: usize,
    tile: usize,
    rows: Vec<Row>,
    pivot_row: Vec<u32>,
    pivmask: Vec<u64>,
    loc_of_col: Vec<u32>,
    queue: Vec<VecDeque<u32>>,
    basis_bytes: u64,
    peak_basis_bytes: u64,
    st: Stats,
    refuted: bool,
    mv: Vec<Vec<u32>>,
    low_start: usize,
}

const NONE: u32 = u32::MAX;
static ORDER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(2);
static TILE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(256);

impl<'a> Engine<'a> {
    fn new(cols: &'a Cols, kchunk: usize) -> Engine<'a> {
        let w = cols.words();
        let low_start = cols.off[cols.d - 1];
        // multiply-by-variable tables for the columns of degree < D
        let mut mv: Vec<Vec<u32>> = Vec::with_capacity(cols.nv);
        for j in 0..cols.nv {
            let bit = 1u64 << j;
            let mut t = Vec::with_capacity(cols.ncols - low_start);
            for c in low_start..cols.ncols {
                t.push(cols.col_of(cols.mask_of[c] | bit) as u32);
            }
            mv.push(t);
        }
        Engine {
            cols,
            w,
            kchunk,
            tile: TILE.load(std::sync::atomic::Ordering::Relaxed),
            rows: Vec::new(),
            pivot_row: vec![NONE; cols.ncols],
            pivmask: vec![0; w],
            loc_of_col: vec![NONE; cols.ncols],
            queue: (0..=cols.d).map(|_| VecDeque::new()).collect(),
            basis_bytes: 0,
            peak_basis_bytes: 0,
            st: Stats::default(),
            refuted: false,
            mv,
            low_start,
        }
    }

    /// Reduce a dense row against the global basis: one XOR per pivot bit it
    /// carries (the basis is fully reduced, so XORs never create pivot bits).
    fn forward(&mut self, b: &mut [u64]) {
        let w = self.w;
        for wd in 0..w {
            let mut h = b[wd] & self.pivmask[wd];
            while h != 0 {
                let c = wd * 64 + h.trailing_zeros() as usize;
                h &= h - 1;
                let r = &self.rows[self.pivot_row[c] as usize];
                let s = r.start as usize;
                xor_into(&mut b[s..], &r.data);
                self.st.ops_fwd += r.data.len() as u64;
                self.st.hits_fwd += 1;
            }
        }
    }

    /// Reduce one row block and fold it into the basis.  `chunk` is emptied.
    fn process_chunk(&mut self, chunk: &mut Vec<Vec<u64>>) {
        let w = self.w;
        let cols = self.cols;
        self.st.chunks += 1;
        let mut local: Vec<(usize, Vec<u64>)> = Vec::new();
        let mut lmask = vec![0u64; w];
        let t_a = Instant::now();
        let mut t_fwd_acc = 0.0f64;
        for mut b in chunk.drain(..) {
            let t_f = Instant::now();
            self.forward(&mut b);
            t_fwd_acc += t_f.elapsed().as_secs_f64();
            // reduce against this block's own pivots
            for wd in 0..w {
                let mut h = b[wd] & lmask[wd];
                while h != 0 {
                    let c = wd * 64 + h.trailing_zeros() as usize;
                    h &= h - 1;
                    let li = self.loc_of_col[c] as usize;
                    let lr = &local[li].1;
                    xor_into(&mut b[wd..], &lr[wd..]);
                    self.st.ops_local += (w - wd) as u64;
                }
            }
            let first = match b.iter().position(|&x| x != 0) {
                Some(f) => f,
                None => continue,
            };
            let c = first * 64 + b[first].trailing_zeros() as usize;
            // clear column c from the block's earlier rows
            let (cw, cb) = (c / 64, c % 64);
            for (lp, lr) in local.iter_mut() {
                if *lp < c && (lr[cw] >> cb) & 1 == 1 {
                    xor_into(&mut lr[cw..], &b[cw..]);
                    self.st.ops_local += (w - cw) as u64;
                }
            }
            lmask[cw] |= 1u64 << cb;
            self.loc_of_col[c] = local.len() as u32;
            local.push((c, b));
        }
        self.st.t_fwd += t_fwd_acc;
        self.st.t_local += t_a.elapsed().as_secs_f64() - t_fwd_acc;
        if local.is_empty() {
            return;
        }
        let t_p = Instant::now();
        // The blocked pass: the block's new pivot columns are cleared from
        // every old row.  The sweep is tiled over word ranges: for one tile
        // the block's rows contribute a `tile`-word slice each, small enough
        // to stay in cache while every old row streams through once.  A row
        // remembers which block rows it has taken (a list) so the later
        // tiles apply the same rows.
        let mut comb: Vec<Vec<u16>> = vec![Vec::new(); self.rows.len()];
        let tile = self.tile.max(8);
        let minw = local.iter().map(|(c, _)| c / 64).min().unwrap();
        let mut t0 = (minw / tile) * tile;
        while t0 < w {
            let t1 = (t0 + tile).min(w);
            for (ri, r) in self.rows.iter_mut().enumerate() {
                let base = r.start as usize;
                if base >= t1 {
                    continue;
                }
                let lo = base.max(t0);
                self.st.bytes_pass += ((t1 - lo) * 8) as u64;
                let cm = &mut comb[ri];
                for &li in cm.iter() {
                    xor_into(
                        &mut r.data[(lo - base)..(t1 - base)],
                        &local[li as usize].1[lo..t1],
                    );
                    self.st.ops_pass += (t1 - lo) as u64;
                }
                for x in lo..t1 {
                    let mut h = r.data[x - base] & lmask[x];
                    while h != 0 {
                        let c = x * 64 + h.trailing_zeros() as usize;
                        h &= h - 1;
                        let li = self.loc_of_col[c] as usize;
                        xor_into(&mut r.data[(x - base)..(t1 - base)], &local[li].1[x..t1]);
                        self.st.ops_pass += (t1 - x) as u64;
                        cm.push(li as u16);
                    }
                }
            }
            t0 = t1;
        }
        self.st.t_pass += t_p.elapsed().as_secs_f64();
        // append the block's rows
        for (c, b) in local.into_iter() {
            self.loc_of_col[c] = NONE;
            let s = c / 64;
            let b: Vec<u64> = b[s..].to_vec();
            let idx = self.rows.len() as u32;
            self.basis_bytes += (b.len() * 8) as u64;
            self.pivot_row[c] = idx;
            self.pivmask[c / 64] |= 1u64 << (c % 64);
            if (cols.deg_of[c] as usize) < cols.d {
                self.queue[cols.deg_of[c] as usize].push_back(idx);
                self.st.falls += 1;
            }
            if c == cols.ncols - 1 {
                self.refuted = true;
            }
            self.rows.push(Row {
                piv: c as u32,
                start: s as u32,
                data: b,
            });
            self.st.new_pivots += 1;
        }
        if self.basis_bytes > self.peak_basis_bytes {
            self.peak_basis_bytes = self.basis_bytes;
        }
    }

    fn terms_of(&self, r: &Row) -> Vec<u32> {
        let mut out = Vec::new();
        for (i, &x) in r.data.iter().enumerate() {
            let base = (r.start as usize + i) * 64;
            let mut bits = x;
            while bits != 0 {
                out.push((base + bits.trailing_zeros() as usize) as u32);
                bits &= bits - 1;
            }
        }
        out
    }

    fn run(
        &mut self,
        eqs: &[Poly],
        time_limit: f64,
        mem_cap: u64,
        as_cap: u64,
        started: &Instant,
        progress: bool,
    ) -> Outcome {
        let w = self.w;
        let cols = self.cols;
        let d = cols.d as u32;
        let mut chunk: Vec<Vec<u64>> = Vec::new();
        for p in eqs {
            if poly_degree(p) > d {
                continue;
            }
            let mut b = vec![0u64; w];
            for &m in p {
                let c = cols.col_of(m);
                b[c / 64] ^= 1u64 << (c % 64);
            }
            chunk.push(b);
            self.st.seed_rows += 1;
        }
        self.process_chunk(&mut chunk);
        let mut last_print = Instant::now();
        loop {
            if self.refuted {
                return Outcome::Refuted;
            }
            if started.elapsed().as_secs_f64() > time_limit {
                return Outcome::TimeLimit;
            }
            let est = self.basis_bytes + (self.kchunk * w * 8 * 2) as u64;
            if est > mem_cap || vm_bytes() + (self.kchunk * w * 8 * 2) as u64 > as_cap {
                return Outcome::MemCap;
            }
            if progress && last_print.elapsed().as_secs_f64() > 20.0 {
                last_print = Instant::now();
                eprintln!(
                    "  [{:.0}s] rank {} products {} pending {} basis {:.0} MiB vm {:.0} MiB rss-peak {:.0} MiB",
                    started.elapsed().as_secs_f64(),
                    self.rows.len(),
                    self.st.products,
                    self.queue.iter().map(|q| q.len()).sum::<usize>(),
                    self.basis_bytes as f64 / 1048576.0,
                    vm_bytes() as f64 / 1048576.0,
                    rss_peak_kb() as f64 / 1024.0
                );
            }
            let order = ORDER.load(std::sync::atomic::Ordering::Relaxed);
            let popped = if order == 0 {
                // lowest degree first
                self.queue.iter_mut().find_map(|q| q.pop_front())
            } else if order == 1 {
                self.queue.iter_mut().rev().find_map(|q| q.pop_front())
            } else {
                // arrival order across degrees is not tracked: highest first
                self.queue.iter_mut().rev().find_map(|q| q.pop_back())
            };
            let f = match popped {
                Some(f) => f,
                None => {
                    if chunk.is_empty() {
                        return Outcome::Saturated;
                    }
                    self.process_chunk(&mut chunk);
                    continue;
                }
            };
            let terms = self.terms_of(&self.rows[f as usize]);
            let mut andmask = u64::MAX;
            for &c in &terms {
                andmask &= cols.mask_of[c as usize];
            }
            for j in 0..cols.nv {
                if (andmask >> j) & 1 == 1 {
                    continue; // x_j * f = f
                }
                let mv = &self.mv[j];
                let mut b = vec![0u64; w];
                for &c in &terms {
                    let c2 = mv[c as usize - self.low_start] as usize;
                    b[c2 / 64] ^= 1u64 << (c2 % 64);
                }
                self.st.products += 1;
                chunk.push(b);
                if chunk.len() >= self.kchunk {
                    self.process_chunk(&mut chunk);
                    if self.refuted {
                        return Outcome::Refuted;
                    }
                }
            }
        }
    }
}

struct DegreeResult {
    degree: usize,
    columns: usize,
    outcome: &'static str,
    refuted: bool,
    rank: usize,
    stats: Stats,
    peak_basis: u64,
    secs: f64,
    rss_kb: u64,
    kchunk: usize,
}

fn closure_at(
    eqs: &[Poly],
    nv: usize,
    degree: usize,
    kchunk_req: usize,
    time_limit: f64,
    mem_cap: u64,
    progress: bool,
) -> DegreeResult {
    let started = Instant::now();
    let cols = Cols::new(nv, degree);
    let w = cols.words();
    let kchunk = if kchunk_req > 0 {
        kchunk_req
    } else {
        (400_000_000usize / (w * 8)).clamp(64, 4096)
    };
    let mut e = Engine::new(&cols, kchunk);
    // keep a margin under any address-space limit so that a block never
    // fails inside the allocator
    let as_cap = as_limit_bytes().map_or(u64::MAX, |l| l.saturating_sub(256 << 20));
    let out = e.run(eqs, time_limit, mem_cap, as_cap, &started, progress);
    let (name, refuted) = match out {
        Outcome::Refuted => ("refuted", true),
        Outcome::Saturated => ("saturated", false),
        Outcome::TimeLimit => ("timelimit", false),
        Outcome::MemCap => ("memcap", false),
    };
    DegreeResult {
        degree,
        columns: cols.ncols,
        outcome: name,
        refuted,
        rank: e.rows.len(),
        stats: e.st.clone(),
        peak_basis: e.peak_basis_bytes,
        secs: started.elapsed().as_secs_f64(),
        rss_kb: rss_peak_kb(),
        kchunk,
    }
}

fn result_json(label: &str, m: usize, n: u32, ell: usize, nv: usize, r: &DegreeResult) -> String {
    let s = &r.stats;
    format!(
        concat!(
            "{{\"tool\":\"b07\",\"cell\":\"{}\",\"m\":{},\"n\":{},\"ell\":{},\"n_vars\":{},",
            "\"degree\":{},\"columns\":{},\"status\":\"{}\",\"refuted\":{},\"rank\":{},",
            "\"products\":{},\"seed_rows\":{},\"chunks\":{},\"falls\":{},\"new_pivots\":{},",
            "\"word_ops\":{},\"word_ops_forward\":{},\"word_ops_block\":{},\"word_ops_pass\":{},",
            "\"hits_forward\":{},\"bytes_pass\":{},\"bytes_read_forward\":{},",
            "\"basis_bytes_peak\":{},\"rss_peak_kb\":{},\"secs\":{:.3},",
            "\"secs_forward\":{:.3},\"secs_block\":{:.3},\"secs_pass\":{:.3},",
            "\"chunk_rows\":{},\"tile_words\":{},\"order\":{}}}"
        ),
        label,
        m,
        n,
        ell,
        nv,
        r.degree,
        r.columns,
        r.outcome,
        r.refuted,
        r.rank,
        s.products,
        s.seed_rows,
        s.chunks,
        s.falls,
        s.new_pivots,
        s.ops_fwd + s.ops_local + s.ops_pass,
        s.ops_fwd,
        s.ops_local,
        s.ops_pass,
        s.hits_fwd,
        s.bytes_pass,
        s.ops_fwd * 8,
        r.peak_basis,
        r.rss_kb,
        r.secs,
        s.t_fwd,
        s.t_local,
        s.t_pass,
        r.kchunk,
        TILE.load(std::sync::atomic::Ordering::Relaxed),
        ORDER.load(std::sync::atomic::Ordering::Relaxed)
    )
}

/// Scan degrees `from..=to`; stop at the first refutation.  Returns the
/// closure degree if one was found.
fn scan(
    label: &str,
    m: usize,
    n: u32,
    ell: usize,
    eqs: &[Poly],
    nv: usize,
    from: usize,
    to: usize,
    kchunk: usize,
    time_limit: f64,
    mem_cap: u64,
    progress: bool,
    verbose: bool,
) -> (Option<usize>, Vec<DegreeResult>) {
    let t0 = Instant::now();
    let mut results = Vec::new();
    for d in from..=to {
        let left = time_limit - t0.elapsed().as_secs_f64();
        let r = closure_at(eqs, nv, d, kchunk, left.max(1.0), mem_cap, progress);
        if verbose {
            println!("{}", result_json(label, m, n, ell, nv, &r));
        }
        let found = r.refuted;
        let stop = r.outcome == "timelimit" || r.outcome == "memcap";
        results.push(r);
        if found {
            return (Some(d), results);
        }
        if stop {
            break;
        }
    }
    (None, results)
}

// ---------------------------------------------------------------- modes

fn system_for(m: usize, n: u32, basis: &[u64], xr: u64) -> (Vec<Poly>, usize, Gf) {
    let gf = Gf::new(n);
    let (eqs, nv) = build_system(&gf, m, basis, xr);
    (eqs, nv, gf)
}

fn mode_count(a: &Args) {
    let m: usize = a.num("--m", 4);
    let n: u32 = a.num("--n", 11);
    let basis = a.list("--basis");
    let xr: u64 = a.num("--xr", 0);
    let gf = Gf::new(n);
    let c = count_solutions(&gf, m, &basis, xr);
    println!(
        "{{\"tool\":\"b07\",\"mode\":\"count\",\"m\":{m},\"n\":{n},\"ell\":{},\"solutions\":{c}}}",
        basis.len()
    );
    if let Some(want) = a.flag("--expect") {
        let want: u64 = want.parse().unwrap();
        if want != c {
            eprintln!("count {c} != expected {want}");
            std::process::exit(3);
        }
    }
}

fn mode_closure(a: &Args) {
    let m: usize = a.num("--m", 4);
    let n: u32 = a.num("--n", 11);
    let basis = a.list("--basis");
    let xr: u64 = a.num("--xr", 0);
    let from: usize = a.num("--from", 3);
    let to: usize = a.num("--to", 4);
    let kchunk: usize = a.num("--chunk", 0);
    let time_limit: f64 = a.num("--time-limit", 1e9);
    let mem_cap_mb: u64 = a.num("--mem-cap-mb", 6600);
    let label = a.flag("--label").unwrap_or_else(|| format!("n{n}l{}m{m}", basis.len()));
    let (eqs, nv, gf) = system_for(m, n, &basis, xr);
    // expectation on the exact count (unsatisfiable draws only are refutable)
    if let Some(want) = a.flag("--expect-solutions") {
        let c = count_solutions(&gf, m, &basis, xr);
        println!(
            "{{\"tool\":\"b07\",\"mode\":\"count\",\"cell\":\"{label}\",\"solutions\":{c}}}"
        );
        if c != want.parse::<u64>().unwrap() {
            eprintln!("solutions {c} != expected {want}: refusing to run");
            std::process::exit(3);
        }
    }
    let (found, res) = scan(
        &label,
        m,
        n,
        basis.len(),
        &eqs,
        nv,
        from,
        to,
        kchunk,
        time_limit,
        mem_cap_mb * 1024 * 1024,
        true,
        true,
    );
    let last = res.last();
    let status = last.map(|r| r.outcome).unwrap_or("none");
    println!(
        "{{\"tool\":\"b07\",\"mode\":\"closure-summary\",\"cell\":\"{label}\",\"closure_degree\":{},\"last_status\":\"{status}\",\"rss_peak_kb\":{}}}",
        found.map_or("null".to_string(), |d| d.to_string()),
        rss_peak_kb()
    );
    if let Some(expect) = a.flag("--expect-degree") {
        let want: usize = expect.parse().unwrap();
        if found != Some(want) {
            eprintln!("closure degree {found:?} != expected {want}");
            std::process::exit(4);
        }
    }
    if let Some(expect) = a.flag("--expect-rank") {
        let want: usize = expect.parse().unwrap();
        let got = res.last().map(|r| r.rank);
        if got != Some(want) || last.map(|r| r.outcome) != Some("saturated") {
            eprintln!("saturated rank {got:?} != expected {want}");
            std::process::exit(7);
        }
    }
    if a.has("--expect-no-refutation") && found.is_some() {
        eprintln!("a satisfiable system was refuted");
        std::process::exit(5);
    }
}

fn mode_find(a: &Args) {
    let m: usize = a.num("--m", 4);
    let n: u32 = a.num("--n", 15);
    let ell: usize = a.num("--ell", 5);
    let seed: u64 = a.num("--seed", 1);
    let want: usize = a.num("--want", 2);
    let max_draws: usize = a.num("--max-draws", 4096);
    let gf = Gf::new(n);
    let mut rng = Rng(seed ^ ((n as u64) << 40) ^ ((ell as u64) << 32) ^ ((m as u64) << 48));
    let mask = (1u64 << n) - 1;
    let mut found = 0usize;
    for draw in 0..max_draws {
        // l independent vectors
        let mut basis: Vec<u64> = Vec::new();
        let mut ech: Vec<u64> = Vec::new();
        while basis.len() < ell {
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
        let xr = rng.next() & mask;
        let c = count_solutions(&gf, m, &basis, xr);
        println!(
            "{{\"tool\":\"b07\",\"mode\":\"find\",\"m\":{m},\"n\":{n},\"ell\":{ell},\"seed\":{seed},\"draw\":{draw},\"v_basis\":{:?},\"x_r\":{xr},\"solutions\":{c}}}",
            basis
        );
        if c == 0 {
            found += 1;
            if found >= want {
                break;
            }
        }
    }
}

fn mode_plan(a: &Args) {
    let m: usize = a.num("--m", 4);
    let n: u32 = a.num("--n", 15);
    let ell: usize = a.num("--ell", 5);
    let d: usize = a.num("--degree", 4);
    let k: f64 = a.num("--chunk", 4096usize) as f64;
    let mem_cap_mb: f64 = a.num("--mem-cap-mb", 6600usize) as f64;
    let nv = m * ell + (m - 2) * n as usize;
    let cols = cols_up_to(nv, d);
    let low = cols_up_to(nv, d - 1);
    let w = ((cols + 63) / 64) as f64;
    let c = cols as f64;
    // exact-size triangular rows at full rank (the measured basis at
    // refutation is 99.7% of this at (13,4,4))
    let tri = c * c / 16.0;
    let dense = c * w * 8.0;
    // block buffers, per-block combination masks, per-row headers, tables
    let block = k * w * 8.0;
    let comb = c * (k / 64.0).ceil() * 8.0;
    let rowhdr = c * 32.0;
    let tables = (nv as f64) * (low as f64) * 4.0 + c * 12.0;
    let resident = tri + block + comb + rowhdr + tables;
    let as_cap = 7.0 * 1024f64.powi(3);
    let inst_cap = mem_cap_mb * 1048576.0;
    let fits = resident <= inst_cap;
    let disk = (resident - inst_cap).max(0.0);
    // committed dense engine: log2(word ops) / log2(columns) in 2.18..2.33
    let lo = c.log2() * 2.18;
    let hi = c.log2() * 2.33;
    println!(
        concat!(
            "{{\"tool\":\"b07\",\"mode\":\"plan\",\"m\":{},\"n\":{},\"ell\":{},\"n_vars\":{},",
            "\"degree\":{},\"columns\":{},\"columns_below_degree\":{},\"words\":{},",
            "\"basis_tri_upper_bytes\":{:.0},\"dense_square_bytes\":{:.0},\"block_bytes\":{:.0},",
            "\"comb_bytes\":{:.0},\"resident_upper_bytes\":{:.0},\"instrument_cap_bytes\":{:.0},",
            "\"address_space_cap_bytes\":{:.0},\"projected_disk_bytes_if_spilled\":{:.0},",
            "\"fits_instrument_cap\":{},\"word_ops_log2_low\":{:.2},\"word_ops_log2_high\":{:.2},",
            "\"note\":\"bounds are upper bounds at rank = columns; the word-op range is the committed dense engine's log2(ops)/log2(cols) in 2.18..2.33, a projection, not a measurement\"}}"
        ),
        m,
        n,
        ell,
        nv,
        d,
        cols,
        low,
        w as u64,
        tri,
        dense,
        block,
        comb,
        resident,
        inst_cap,
        as_cap,
        disk,
        fits,
        lo,
        hi
    );
}

// --- selftest -----------------------------------------------------------

fn grevlex_greater(a: u64, b: u64) -> bool {
    // exponent-vector grevlex, x0 > x1 > ...: degree, then the LAST differing
    // variable: the monomial with the smaller exponent there is larger.
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

fn selftest(a: &Args) {
    let mut ok = true;
    let mut check = |name: &str, pass: bool, detail: String| {
        println!("{} {name} {detail}", if pass { "PASS" } else { "FAIL" });
        if !pass {
            ok = false;
        }
    };
    // 1. rank/unrank and order
    {
        let cols = Cols::new(10, 4);
        let mut inv = true;
        for c in 0..cols.ncols {
            if cols.col_of(cols.mask_of[c]) != c {
                inv = false;
            }
        }
        let mut order = true;
        for c in 1..cols.ncols {
            if !grevlex_greater(cols.mask_of[c - 1], cols.mask_of[c]) {
                order = false;
            }
        }
        check("column rank inverse", inv, format!("ncols {}", cols.ncols));
        check(
            "columns descend in DegRevLex",
            order,
            "adjacent pairs checked".into(),
        );
        let big = Cols::new(50, 4);
        check(
            "column count N=50 D=4",
            big.ncols as u64 == cols_up_to(50, 4),
            format!("{}", big.ncols),
        );
    }
    // 2. field: irreducible polynomials of the committed choice
    {
        let g = Gf::new(11);
        check(
            "n=11 irreducible is z^11+z^2+1",
            g.low == 0b101,
            format!("low {:#b}", g.low),
        );
        let g = Gf::new(13);
        check(
            "n=13 irreducible is z^13+z^4+z^3+z+1",
            g.low == 0b11011,
            format!("low {:#b}", g.low),
        );
        // multiplication is associative/commutative; roots solve S3
        let g = Gf::new(13);
        let mut rng = Rng(5);
        let mut fine = true;
        for _ in 0..2000 {
            let (x, y, z) = (rng.next() & g.mask, rng.next() & g.mask, rng.next() & g.mask);
            if g.mul(x, g.mul(y, z)) != g.mul(g.mul(x, y), z) || g.mul(x, y) != g.mul(y, x) {
                fine = false;
            }
            if g.sqr(x) != g.mul(x, x) {
                fine = false;
            }
            for d in g.roots(x, y) {
                if g.s3(x, y, d) != 0 {
                    fine = false;
                }
            }
        }
        check("field axioms and S3 roots", fine, "2000 random triples".into());
        // roots complete: brute force at n = 9
        let g = Gf::new(9);
        let mut complete = true;
        for _ in 0..300 {
            let (x, y) = (rng.next() & g.mask, rng.next() & g.mask);
            let brute: Vec<u64> = (0..1u64 << 9).filter(|&d| g.s3(x, y, d) == 0).collect();
            let mut got = g.roots(x, y);
            got.sort_unstable();
            if got != brute {
                complete = false;
            }
        }
        check(
            "additive root solve equals brute force (n=9)",
            complete,
            "300 pairs".into(),
        );
    }
    // 3. the symbolic system has exactly the counted solutions (exhaustive)
    {
        let cases: Vec<(usize, u32, Vec<u64>, u64)> = vec![
            (4, 4, vec![5], 3),
            (4, 5, vec![11, 20], 9),
            (3, 6, vec![5, 13], 17),
            (4, 6, vec![19, 40], 33),
            (3, 7, vec![9, 70], 91),
        ];
        for (m, n, basis, xr) in cases {
            let gf = Gf::new(n);
            let (eqs, nv) = build_system(&gf, m, &basis, xr);
            let cnt = count_solutions(&gf, m, &basis, xr);
            if nv > 22 {
                continue;
            }
            let mut brute = 0u64;
            for point in 0..(1u64 << nv) {
                if eqs.iter().all(|p| eval_poly(p, point) == 0) {
                    brute += 1;
                }
            }
            check(
                &format!("system solutions m={m} n={n} l={} N={nv}", basis.len()),
                brute == cnt,
                format!("enumerated {brute} counted {cnt} eqs {}", eqs.len()),
            );
        }
    }
    // 4. committed draws: solution counts and closure degrees
    if let Some(tsv) = a.flag("--controls") {
        let max_vars: usize = a.num("--max-vars", 30);
        let rank3_vars: usize = a.num("--rank3-vars", 44);
        control_check(&tsv, max_vars, rank3_vars, &mut check);
    }
    println!("{}", if ok { "SELFTEST OK" } else { "SELFTEST FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

#[derive(Clone)]
struct Ctl {
    key: String,
    m: usize,
    n: u32,
    ell: usize,
    basis: Vec<u64>,
    xr: u64,
    solutions: i64,
    ref_closure: i64, // -1 none
    ref_msolve: i64,
    ref_rank3: i64,
}

fn read_controls(path: &str) -> Vec<Ctl> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 10 {
            continue;
        }
        out.push(Ctl {
            key: f[0].to_string(),
            m: f[1].parse().unwrap(),
            n: f[2].parse().unwrap(),
            ell: f[3].parse().unwrap(),
            basis: f[4].split(',').map(|x| x.parse().unwrap()).collect(),
            xr: f[5].parse().unwrap(),
            solutions: f[6].parse().unwrap(),
            ref_closure: f[7].parse().unwrap(),
            ref_msolve: f[8].parse().unwrap(),
            ref_rank3: f[9].parse().unwrap(),
        });
    }
    out
}

fn control_check(
    path: &str,
    max_vars: usize,
    rank3_vars: usize,
    check: &mut dyn FnMut(&str, bool, String),
) {
    let ctls = read_controls(path);
    let (mut n_cnt, mut bad_cnt) = (0, 0);
    let (mut n_deg, mut bad_deg) = (0, 0);
    let (mut n_rank, mut bad_rank) = (0, 0);
    let (mut n_ms, mut bad_ms) = (0, 0);
    for c in &ctls {
        let nv = c.m * c.ell + (c.m - 2) * c.n as usize;
        let gf = Gf::new(c.n);
        if c.solutions >= 0 {
            n_cnt += 1;
            let got = count_solutions(&gf, c.m, &c.basis, c.xr) as i64;
            if got != c.solutions {
                bad_cnt += 1;
                println!("  count mismatch {} got {got} committed {}", c.key, c.solutions);
            }
        }
        if nv > rank3_vars || c.solutions != 0 {
            continue;
        }
        let (eqs, nv2) = build_system(&gf, c.m, &c.basis, c.xr);
        assert_eq!(nv, nv2);
        let deep = nv <= max_vars;
        let top = if deep { 5 } else { 3 };
        let (found, res) = scan(&c.key, c.m, c.n, c.ell, &eqs, nv, 3, top, 0, 600.0, 4 << 30, false, false);
        let got = found.map(|d| d as i64).unwrap_or(-1);
        if deep && c.ref_closure >= 0 {
            n_deg += 1;
            if got != c.ref_closure {
                bad_deg += 1;
                println!("  degree mismatch {} got {got} committed closure {}", c.key, c.ref_closure);
            }
        }
        if deep && c.ref_msolve >= 0 {
            n_ms += 1;
            if got != c.ref_msolve {
                bad_ms += 1;
                println!("  msolve mismatch {} got {got} msolve {}", c.key, c.ref_msolve);
            }
        }
        if c.ref_rank3 >= 0 {
            if let Some(r3) = res.iter().find(|r| r.degree == 3) {
                n_rank += 1;
                if r3.rank as i64 != c.ref_rank3 && !r3.refuted {
                    bad_rank += 1;
                    println!(
                        "  rank3 mismatch {} got {} committed closure_rank {}",
                        c.key, r3.rank, c.ref_rank3
                    );
                }
            }
        }
    }
    check(
        "committed solution counts reproduced",
        bad_cnt == 0,
        format!("{n_cnt} draws, {bad_cnt} mismatches"),
    );
    check(
        "closure degree equals committed incumbent closure",
        bad_deg == 0,
        format!("{n_deg} draws, {bad_deg} mismatches"),
    );
    check(
        "closure degree equals committed msolve F4 step degree",
        bad_ms == 0,
        format!("{n_ms} draws, {bad_ms} mismatches"),
    );
    check(
        "degree-3 closure rank equals committed closure_rank",
        bad_rank == 0,
        format!("{n_rank} draws, {bad_rank} mismatches"),
    );
}

/// Random systems with the term-count-per-degree profile of a real cell.
fn mode_random_control(a: &Args) {
    let m: usize = a.num("--m", 4);
    let n: u32 = a.num("--n", 6);
    let basis = a.list("--basis");
    let xr: u64 = a.num("--xr", 0);
    let seeds = a.list("--seeds");
    let from: usize = a.num("--from", 3);
    let to: usize = a.num("--to", 6);
    let time_limit: f64 = a.num("--time-limit", 1e9);
    let mem_cap_mb: u64 = a.num("--mem-cap-mb", 6600);
    let (eqs, nv, _gf) = system_for(m, n, &basis, xr);
    let mut blind = 0;
    for &seed in &seeds {
        let mut rng = Rng(seed ^ 0xC0FFEE);
        let mut rnd: Vec<Poly> = Vec::new();
        for p in &eqs {
            let mut raw: Vec<u64> = Vec::new();
            for deg in 0..=poly_degree(p) {
                let cnt = p.iter().filter(|x| x.count_ones() == deg).count();
                let mut got: Vec<u64> = Vec::new();
                while got.len() < cnt {
                    // a uniformly random variable subset of size `deg`
                    let mut mask = 0u64;
                    while (mask.count_ones()) < deg {
                        mask |= 1u64 << (rng.next() % nv as u64);
                    }
                    if !got.contains(&mask) {
                        got.push(mask);
                    }
                }
                raw.extend(got);
            }
            rnd.push(norm(raw));
        }
        let label = format!("random-n{n}l{}m{m}-s{seed}", basis.len());
        let (found, _) = scan(
            &label,
            m,
            n,
            basis.len(),
            &rnd,
            nv,
            from,
            to,
            0,
            time_limit,
            mem_cap_mb * 1024 * 1024,
            false,
            true,
        );
        println!(
            "{{\"tool\":\"b07\",\"mode\":\"random-control\",\"cell\":\"{label}\",\"seed\":{seed},\"closure_degree\":{}}}",
            found.map_or("null".to_string(), |d| d.to_string())
        );
        if found.map_or(false, |d| d <= 4) {
            blind += 1;
        }
    }
    println!(
        "{{\"tool\":\"b07\",\"mode\":\"random-control-summary\",\"seeds\":{},\"read_at_most_4\":{blind}}}",
        seeds.len()
    );
    if a.has("--expect-above-4") && blind > 0 {
        eprintln!("the instrument read degree <= 4 on a random system");
        std::process::exit(6);
    }
}

/// Build a controls TSV from committed closure-result files (each line names
/// the draw source file it was run on).
fn mode_control_table(a: &Args) {
    let out = a.v.get(2).cloned().expect("control-table OUT.tsv FILES...");
    let max_n: usize = a.num("--max-vars", 40);
    let mut rows: Vec<String> = Vec::new();
    rows.push(
        "#key\tm\tn\tell\tv_basis\tx_r\tsolutions\tref_closure\tref_msolve\tref_rank3".into(),
    );
    // msolve readings of item 06: (path, u) -> F4 step degree
    let mut ms: HashMap<(String, usize), i64> = HashMap::new();
    if let Some(mf) = a.flag("--msolve") {
        let t = std::fs::read_to_string(&mf).unwrap_or_else(|e| panic!("{mf}: {e}"));
        for l in t.lines() {
            let (Some(i), Some(j)) = (l.find('['), l.find(']')) else { continue };
            let path = l[i + 1..j].to_string();
            let Some(k) = l.find("F4 ") else { continue };
            let Ok(d) = l[k + 3..].trim().parse::<i64>() else { continue };
            let u = l
                .split_whitespace()
                .find(|t| t.starts_with('u') && t.ends_with(':'))
                .and_then(|t| t[1..t.len() - 1].parse::<usize>().ok())
                .unwrap_or(0);
            let per_u = path.contains(".u") && path.matches(".u").count() > 0 && path.rsplit('.').nth(1).map_or(false, |x| x.starts_with('u'));
            ms.insert((path, if per_u { 0 } else { u }), d);
        }
    }
    let mut seen_rank: HashMap<String, usize> = HashMap::new();
    let files: Vec<&String> = a.v[3..]
        .iter()
        .enumerate()
        .filter(|(i, x)| {
            !x.starts_with("--")
                && !(*i > 0 && ["--max-vars", "--msolve"].contains(&a.v[3 + *i - 1].as_str()))
        })
        .map(|(_, x)| x)
        .collect();
    for f in files {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let Some(src) = jstr(line, "source") else { continue };
            let Some(draw) = jnum(line, "draw") else { continue };
            let m = jnum(line, "m").unwrap_or(3) as usize;
            let n = jnum(line, "n").unwrap() as u32;
            let ell = jnum(line, "ell").unwrap() as usize;
            let nv = jnum(line, "n_vars").unwrap() as usize;
            if nv > max_n {
                continue;
            }
            let cd = match jfind(line, "closure_degree") {
                Some(s) if s.starts_with("null") => -1,
                Some(_) => jnum(line, "closure_degree").unwrap_or(-1),
                None => -1,
            };
            let r3 = line
                .find("\"degree\":3,")
                .and_then(|i| jnum(&line[i..], "closure_rank"))
                .unwrap_or(-1);
            let Ok(stext) = std::fs::read_to_string(&src) else { continue };
            let sline = stext
                .lines()
                .find(|l| jnum(l, "draw") == Some(draw) && jarr(l, "v_basis").is_some());
            let per_u = src.rsplit('.').nth(1).map_or(false, |x| {
                x.starts_with('u') && x[1..].chars().all(|c| c.is_ascii_digit())
            });
            let rank = {
                let e = seen_rank.entry(src.clone()).or_insert(0);
                let r = *e;
                *e += 1;
                r
            };
            let kk = if per_u { 0 } else { rank };
            let key_path = src.strip_prefix("research/").unwrap_or(&src).to_string();
            let msd = ms.get(&(key_path, kk)).copied().unwrap_or(-1);
            if let Some(sl) = sline {
                let basis = jarr(sl, "v_basis").unwrap();
                let xr = jnum(sl, "x_r").unwrap();
                let sol = jnum(sl, "solutions").unwrap_or(-1);
                rows.push(format!(
                    "{}#{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    src,
                    draw,
                    m,
                    n,
                    ell,
                    basis.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                    xr,
                    sol,
                    cd,
                    msd,
                    r3
                ));
            }
        }
    }
    let n_rows = rows.len() - 1;
    std::fs::write(&out, rows.join("\n") + "\n").unwrap();
    println!("wrote {n_rows} control rows to {out}");
}

fn main() {
    let a = Args {
        v: std::env::args().collect(),
    };
    ORDER.store(a.num("--order", 2usize), std::sync::atomic::Ordering::Relaxed);
    TILE.store(a.num("--tile", 256usize), std::sync::atomic::Ordering::Relaxed);
    match a.v.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(&a),
        Some("count") => mode_count(&a),
        Some("closure") => mode_closure(&a),
        Some("find") => mode_find(&a),
        Some("plan") => mode_plan(&a),
        Some("random-control") => mode_random_control(&a),
        Some("control-table") => mode_control_table(&a),
        _ => {
            eprintln!("usage: b07 selftest|count|closure|find|plan|random-control|control-table ...");
            std::process::exit(2);
        }
    }
}
