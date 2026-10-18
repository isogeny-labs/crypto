//! Blind-spot item 05: monomial, variable and block orders tuned to the descent
//! structure, measured on a native Boolean-ring F4.
//!
//! Standard library only (the `crypto` library does not compile at this branch
//! base, see `research/blindspots_20261018/05-elimination-orders/PROTOCOL.md`).
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
//!     -o target/blindspots-05/b05 examples/blindspots_05_elimination_orders.rs
//! b05 selftest [--controls controls.tsv --max-nv 26]
//! b05 gen  --m 3 --n 17 --ell 4 --seed 20261019 --want 8 --label c17-4-3
//! b05 run  --draws draws.tsv --cell c17-4-3 --targets 0-7 --arms grevlex,wdeg,... \
//!          [--time-limit S] [--total-limit S] [--max-cols N]
//! b05 sens
//! b05 summarize --dir raw [--cells a,b,..] [--min-targets 4] [--min-arm-targets 2]
//! ```
//!
//! # The instrument
//!
//! The chained Semaev `S_3` descent system of a cell `(n, l, m)` (unknowns
//! `m*l` subspace coordinates, then `(m-2)*n` intermediate bits; equations are
//! the bit coordinates of the `m-1` links) is solved by an F4 over the Boolean
//! ring `B = F_2[x]/(x_i^2 + x_i)`.  Monomials are squarefree masks.  The
//! monomial order is a parameter: weighted degree (per-variable positive
//! weights), then grevlex under a variable ranking, optionally as a block
//! (product) order.  Everything else (pair selection by the order's own degree,
//! Gebauer-Moeller update, symbolic preprocessing with the smallest divisor,
//! sparse reducers, dense accumulators, tail reduction) is identical for every
//! order, so a difference between arms is a difference of the order.
//!
//! Pairs: the ordinary `S`-pairs of the leading monomials (lcm = union) and,
//! for every `x` in `LM(g)`, the field-equation pair `(g, x^2 + x)`, whose
//! `S`-polynomial is `x * g` computed in `B`.  A finished basis is a Groebner
//! basis of `I + <x_i^2 + x_i>`; the selftest certifies this by counting
//! standard monomials against a brute-force solution count.
//!
//! Counted: `touches` (matrix entries written while building rows plus entries
//! toggled by every row operation, back-substitution included), matrix shapes,
//! and the total and selection degrees.  Wall time is descriptive.

use std::collections::HashMap;
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

// ---------------------------------------------------------------- monomial orders

/// A monomial order on squarefree monomials (masks over at most 62 variables).
///
/// `key(m)` is an integer such that `a > b` in the order iff `key(a) > key(b)`.
/// Layout (high to low): weighted degree of block A (8 bits), complemented
/// ranked mask of block A, weighted degree of block B (8 bits), complemented
/// ranked mask of block B.  With one block this is weighted degree, then
/// grevlex under the variable ranking (rank 0 is the largest variable).  With
/// two blocks it is the product order: block A first.
struct Order {
    nv: usize,
    tab: Vec<[u128; 256]>,
    flip: u128,
    off_a_w: u32,
    off_b_w: u32,
    w: Vec<u32>,
    blk: Vec<u8>,
    name: String,
}

impl Order {
    fn new(name: &str, rank: &[usize], weight: &[u32], blk: &[u8]) -> Order {
        let nv = rank.len();
        assert!(nv <= 62);
        let mut pos = vec![0u32; nv];
        let mut sizes = [0u32; 2];
        for b in 0..2u8 {
            let mut vs: Vec<usize> = (0..nv).filter(|&v| blk[v] == b).collect();
            vs.sort_by_key(|&v| rank[v]);
            for (i, &v) in vs.iter().enumerate() {
                pos[v] = i as u32;
            }
            sizes[b as usize] = vs.len() as u32;
        }
        let (s0, s1) = (sizes[0], sizes[1]);
        let off_b_w = s1;
        let off_a_p = s1 + 8;
        let off_a_w = s1 + 8 + s0;
        let nch = (nv + 7) / 8;
        let mut tab = vec![[0u128; 256]; nch];
        for (c, row) in tab.iter_mut().enumerate() {
            for b in 0..256usize {
                let mut k = 0u128;
                for bit in 0..8 {
                    if (b >> bit) & 1 == 1 {
                        let v = c * 8 + bit;
                        if v < nv {
                            k += if blk[v] == 0 {
                                (1u128 << (off_a_p + pos[v])) + ((weight[v] as u128) << off_a_w)
                            } else {
                                (1u128 << pos[v]) + ((weight[v] as u128) << off_b_w)
                            };
                        }
                    }
                }
                row[b] = k;
            }
        }
        let flip = ((1u128 << s1) - 1) | (((1u128 << s0) - 1) << off_a_p);
        Order {
            nv,
            tab,
            flip,
            off_a_w,
            off_b_w,
            w: weight.to_vec(),
            blk: blk.to_vec(),
            name: name.to_string(),
        }
    }
    #[inline]
    fn key(&self, mut m: u64) -> u128 {
        let mut k = 0u128;
        let mut c = 0;
        while m != 0 {
            k += self.tab[c][(m & 255) as usize];
            m >>= 8;
            c += 1;
        }
        k ^ self.flip
    }
    /// The order's own degree vector of `m`: (weighted degree of A) * 256 +
    /// (weighted degree of B).  Pairs are selected by the least such value.
    #[inline]
    fn seldeg(&self, m: u64) -> u32 {
        let k = self.key(m);
        let wa = ((k >> self.off_a_w) & 255) as u32;
        let wb = ((k >> self.off_b_w) & 255) as u32;
        (wa << 8) | wb
    }
    fn var_sel(&self, v: usize) -> u32 {
        if self.blk[v] == 0 {
            self.w[v] << 8
        } else {
            self.w[v]
        }
    }
}

/// Plain grevlex comparator on masks, variable 0 largest (the committed
/// DegRevLex convention): used only to test `Order`.
fn grevlex_greater(a: u64, b: u64) -> bool {
    let (da, db) = (a.count_ones(), b.count_ones());
    if da != db {
        return da > db;
    }
    // among equal degree the numerically smaller mask is larger
    a < b
}

fn sort_cancel(ord: &Order, v: Vec<u64>) -> Vec<u64> {
    let mut kv: Vec<(u128, u64)> = v.into_iter().map(|m| (ord.key(m), m)).collect();
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

fn mul_mono(ord: &Order, p: &[u64], t: u64) -> Vec<u64> {
    if t == 0 {
        return p.to_vec();
    }
    if p.iter().all(|&m| m & t == 0) {
        return p.iter().map(|&m| m | t).collect();
    }
    sort_cancel(ord, p.iter().map(|&m| m | t).collect())
}

fn xor_merge(ord: &Order, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    let mut ka = if a.is_empty() { 0 } else { ord.key(a[0]) };
    let mut kb = if b.is_empty() { 0 } else { ord.key(b[0]) };
    while i < a.len() && j < b.len() {
        if ka > kb {
            out.push(a[i]);
            i += 1;
            if i < a.len() {
                ka = ord.key(a[i]);
            }
        } else if ka < kb {
            out.push(b[j]);
            j += 1;
            if j < b.len() {
                kb = ord.key(b[j]);
            }
        } else {
            i += 1;
            j += 1;
            if i < a.len() {
                ka = ord.key(a[i]);
            }
            if j < b.len() {
                kb = ord.key(b[j]);
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

// ---------------------------------------------------------------- the F4

const NONE: u32 = u32::MAX;

#[derive(Clone, Copy)]
struct Pair {
    a: u32,
    b: u32, // NONE: the field-equation pair of `a` with variable `var`
    lcm: u64,
    sel: u32,
    var: u8,
}

struct Limits {
    deadline: Option<Instant>,
    max_cols: usize,
    vm_cap: u64, // bytes of virtual size at which a run stops, 0 = none
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Status {
    Refuted,
    Gb,
    TimeLimit,
    MemCap,
    ColsCap,
}
impl Status {
    fn name(self) -> &'static str {
        match self {
            Status::Refuted => "refuted",
            Status::Gb => "gb",
            Status::TimeLimit => "timelimit",
            Status::MemCap => "memcap",
            Status::ColsCap => "colscap",
        }
    }
}

#[derive(Default, Clone)]
struct Totals {
    steps: u32,
    pairs_sel: u64,
    pairs_gen: u64,
    rows_max: u64,
    cols_max: u64,
    rc_max: u128,
    nnz_max: u64,
    cols_sum: u64,
    rows_sum: u64,
    build: u64,
    elim: u64,
    back: u64,
    zero_red: u64,
    new_elems: u64,
    d_mat: u32,
    d_sel: u32,
    g_size: usize,
    step_rows: Vec<u64>,
    step_cols: Vec<u64>,
    step_sel: Vec<u32>,
    ms_sym: f64,
    ms_elim: f64,
    ms_back: f64,
    ms_add: f64,
}
impl Totals {
    fn touches(&self) -> u64 {
        self.build + self.elim + self.back
    }
}

struct Eng<'a> {
    ord: &'a Order,
    lim: &'a Limits,
    g: Vec<Vec<u64>>,
    lm: Vec<u64>,
    pairs: Vec<Pair>,
    t: Totals,
}

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

impl<'a> Eng<'a> {
    fn new(ord: &'a Order, lim: &'a Limits) -> Self {
        Eng {
            ord,
            lim,
            g: Vec::new(),
            lm: Vec::new(),
            pairs: Vec::new(),
            t: Totals::default(),
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
        // old ordinary pairs dominated by the new element (chain criterion)
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
                    sel: self.ord.seldeg(l),
                    var: 0,
                });
                self.t.pairs_gen += 1;
            }
        }
        let base = self.ord.seldeg(lmh);
        let mut bits = lmh;
        while bits != 0 {
            let v = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            self.pairs.push(Pair {
                a: h as u32,
                b: NONE,
                lcm: lmh,
                sel: base + self.ord.var_sel(v),
                var: v as u8,
            });
            self.t.pairs_gen += 1;
        }
        self.g.push(p);
        self.lm.push(lmh);
    }

    /// One F4 step: select the pairs of least degree, build the matrix,
    /// eliminate, add the new elements.  `Ok(true)` once `1` is in the basis.
    fn step(&mut self) -> Result<bool, Status> {
        let ord = self.ord;
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
            ord.key(x.lcm)
                .cmp(&ord.key(y.lcm))
                .then(x.a.cmp(&y.a))
                .then(x.b.cmp(&y.b))
                .then(x.var.cmp(&y.var))
        });
        self.t.pairs_sel += selected.len() as u64;
        let mut build = 0u64;
        let mut d_sel = self.t.d_sel;
        let mut srows: Vec<Vec<u64>> = Vec::with_capacity(selected.len());
        let mut zero_at_build = 0u64;
        for p in &selected {
            if p.b == NONE {
                d_sel = d_sel.max(p.lcm.count_ones() + 1);
                let r = mul_mono(ord, &self.g[p.a as usize], 1u64 << p.var);
                build += r.len() as u64;
                if r.is_empty() {
                    zero_at_build += 1;
                } else {
                    srows.push(r);
                }
            } else {
                d_sel = d_sel.max(p.lcm.count_ones());
                let ta = p.lcm & !self.lm[p.a as usize];
                let tb = p.lcm & !self.lm[p.b as usize];
                let ra = mul_mono(ord, &self.g[p.a as usize], ta);
                let rb = mul_mono(ord, &self.g[p.b as usize], tb);
                build += (ra.len() + rb.len()) as u64;
                let r = xor_merge(ord, &ra, &rb);
                if r.is_empty() {
                    zero_at_build += 1;
                } else {
                    srows.push(r);
                }
            }
        }
        self.t.d_sel = d_sel;
        self.t.zero_red += zero_at_build;
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
            if best != usize::MAX {
                let t = m & !self.lm[best];
                let row = mul_mono(ord, &self.g[best], t);
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
        let mut ks: Vec<(u128, u64)> = monos.iter().map(|&m| (ord.key(m), m)).collect();
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
        let n_red = store.len();
        // reduce the S rows
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
            } else {
                nnz += kept.len() as u64;
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
        // back-substitution among the new pivots, rightmost lead first
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
        self.t.zero_red += zero_red;
        self.t.steps += 1;
        self.t.rows_max = self.t.rows_max.max(rows);
        self.t.cols_max = self.t.cols_max.max(c as u64);
        self.t.nnz_max = self.t.nnz_max.max(nnz);
        let rc = rows as u128 * c as u128;
        self.t.rc_max = self.t.rc_max.max(rc);
        self.t.cols_sum += c as u64;
        self.t.rows_sum += rows;
        self.t.step_rows.push(rows);
        self.t.step_cols.push(c as u64);
        self.t.step_sel.push(min_sel);
        let _ = n_red;
        let t_add = Instant::now();
        // new elements, smallest leading monomial first
        let mut news: Vec<Vec<u64>> = Vec::new();
        for &p in newp.iter() {
            let row = &store[p as usize];
            news.push(row.iter().map(|&x| cols[x as usize]).collect());
        }
        news.sort_by(|a, b| ord.key(a[0]).cmp(&ord.key(b[0])));
        self.t.new_elems += news.len() as u64;
        let mut refuted = false;
        for p in news {
            if p[0] == 0 {
                refuted = true;
            }
            self.add(p);
        }
        self.t.ms_add += t_add.elapsed().as_secs_f64() * 1000.0;
        Ok(refuted)
    }

    fn run(&mut self, eqs: &[Poly]) -> Status {
        let ord = self.ord;
        let mut refuted = false;
        for e in eqs {
            if e.is_empty() {
                continue;
            }
            let p = sort_cancel(ord, e.clone());
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

// ---------------------------------------------------------------- primal graph and elimination orders

/// Primal graph of the system: one vertex per variable, a clique on the
/// variables of every equation (the construction of `descent_treewidth.rs`).
fn primal(eqs: &[Poly], nv: usize) -> Vec<u64> {
    let mut adj = vec![0u64; nv];
    for e in eqs {
        let occ = e.iter().fold(0u64, |a, &m| a | m);
        let mut bits = occ;
        while bits != 0 {
            let v = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            adj[v] |= occ & !(1u64 << v);
        }
    }
    adj
}

#[derive(Clone, Copy, PartialEq)]
enum Heur {
    MinDegree,
    MinFill,
}

/// Greedy elimination order and its width (the largest degree at elimination).
fn elim_order(adj0: &[u64], heur: Heur) -> (Vec<usize>, usize) {
    let nv = adj0.len();
    let mut adj = adj0.to_vec();
    let mut alive: u64 = if nv == 64 { !0 } else { (1u64 << nv) - 1 };
    let mut order = Vec::with_capacity(nv);
    let mut width = 0usize;
    for _ in 0..nv {
        let mut best = usize::MAX;
        let mut bk = (usize::MAX, usize::MAX);
        let mut bits = alive;
        while bits != 0 {
            let v = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            let nb = adj[v] & alive;
            let deg = nb.count_ones() as usize;
            let key = match heur {
                Heur::MinDegree => (deg, 0),
                Heur::MinFill => {
                    let mut fill = 0usize;
                    let mut nbb = nb;
                    while nbb != 0 {
                        let u = nbb.trailing_zeros() as usize;
                        nbb &= nbb - 1;
                        fill += (nb & !adj[u] & !(1u64 << u)).count_ones() as usize;
                    }
                    (fill / 2, deg)
                }
            };
            if key < bk {
                bk = key;
                best = v;
            }
        }
        let nb = adj[best] & alive;
        width = width.max(nb.count_ones() as usize);
        let mut nbb = nb;
        while nbb != 0 {
            let u = nbb.trailing_zeros() as usize;
            nbb &= nbb - 1;
            adj[u] |= nb & !(1u64 << u);
        }
        alive &= !(1u64 << best);
        order.push(best);
    }
    (order, width)
}

/// Exact treewidth by the Bodlaender-Fomin subset dynamic program, for at most
/// 18 vertices (the cap of `descent_treewidth::exact_treewidth`).
fn exact_treewidth(adj: &[u64]) -> usize {
    let nv = adj.len();
    assert!(nv <= 18);
    let full = (1usize << nv) - 1;
    let mut tw = vec![u8::MAX; full + 1];
    tw[0] = 0;
    for s in 1..=full {
        let mut best = u8::MAX;
        for v in 0..nv {
            if (s >> v) & 1 == 0 {
                continue;
            }
            let t = s & !(1 << v);
            // Q(t, v): vertices outside t + v reachable from v through t
            let mut comp: u64 = 1 << v;
            loop {
                let mut nb = 0u64;
                let mut bits = comp;
                while bits != 0 {
                    let u = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    nb |= adj[u];
                }
                let grow = nb & (t as u64) & !comp;
                if grow == 0 {
                    break;
                }
                comp |= grow;
            }
            let mut nb = 0u64;
            let mut bits = comp;
            while bits != 0 {
                let u = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                nb |= adj[u];
            }
            let q = (nb & !(t as u64) & !(1u64 << v)).count_ones() as u8;
            best = best.min(tw[t].max(q));
        }
        tw[s] = best;
    }
    tw[full] as usize
}

// ---------------------------------------------------------------- cells and arms

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

const FIXED_ARMS: [&str; 6] = ["grevlex", "blkLI", "blkIL", "wdeg", "mindeg", "tw"];

fn perm_from_seed(seed: u64, nv: usize) -> Vec<usize> {
    let mut rng = Rng(seed);
    let mut p: Vec<usize> = (0..nv).collect();
    for i in (1..nv).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        p.swap(i, j);
    }
    p
}

fn rand_seed(cell: &Cell, d: u64) -> u64 {
    0x5EED_0005_0000_0000u64 ^ ((cell.m as u64) << 48) ^ ((cell.n as u64) << 32) ^ ((cell.l as u64) << 24) ^ d
}

/// Variable ranking from an elimination sequence: the variable eliminated
/// first is the largest (rank 0).
fn rank_from_elim(order: &[usize], reverse: bool) -> Vec<usize> {
    let nv = order.len();
    let mut rank = vec![0usize; nv];
    for (pos, &v) in order.iter().enumerate() {
        rank[v] = if reverse { nv - 1 - pos } else { pos };
    }
    rank
}

/// Build the order of an arm for a system.  `None`: the arm is not defined at
/// this cell (block and weighted arms need intermediates, `m >= 3`).
fn make_order(arm: &str, cell: &Cell, eqs: &[Poly]) -> Option<Order> {
    let nv = cell.nv();
    let nl = cell.nleaf();
    let ident: Vec<usize> = (0..nv).collect();
    let ones = vec![1u32; nv];
    let blk0 = vec![0u8; nv];
    match arm {
        "grevlex" => Some(Order::new(arm, &ident, &ones, &blk0)),
        "blkLI" | "blkIL" => {
            if cell.m < 3 {
                return None;
            }
            let leaves_first = arm == "blkLI";
            let blk: Vec<u8> = (0..nv)
                .map(|v| if (v < nl) == leaves_first { 0 } else { 1 })
                .collect();
            Some(Order::new(arm, &ident, &ones, &blk))
        }
        "wdeg" => {
            if cell.m < 3 {
                return None;
            }
            let w: Vec<u32> = (0..nv).map(|v| if v < nl { 1 } else { 2 }).collect();
            Some(Order::new(arm, &ident, &w, &blk0))
        }
        "mindeg" | "mindeg-rev" | "tw" | "tw-rev" => {
            let adj = primal(eqs, nv);
            let heur = if arm.starts_with("mindeg") { Heur::MinDegree } else { Heur::MinFill };
            let (ord, _) = elim_order(&adj, heur);
            let rank = rank_from_elim(&ord, arm.ends_with("-rev"));
            Some(Order::new(arm, &rank, &ones, &blk0))
        }
        _ if arm.starts_with("rand") => {
            let d: u64 = arm[4..].parse().expect("randK");
            let perm = perm_from_seed(rand_seed(cell, d), nv);
            // perm[i] = the variable with rank i
            let mut rank = vec![0usize; nv];
            for (r, &v) in perm.iter().enumerate() {
                rank[v] = r;
            }
            Some(Order::new(arm, &rank, &ones, &blk0))
        }
        _ => panic!("unknown arm {arm}"),
    }
}

// ---------------------------------------------------------------- running one job

struct JobOut {
    status: Status,
    t: Totals,
    ms: f64,
    rss_kb: u64,
    g: Vec<Vec<u64>>,
    lm: Vec<u64>,
}

fn run_job(eqs: &[Poly], ord: &Order, lim: &Limits) -> JobOut {
    let t0 = Instant::now();
    let mut e = Eng::new(ord, lim);
    let status = e.run(eqs);
    JobOut {
        status,
        ms: t0.elapsed().as_secs_f64() * 1000.0,
        rss_kb: rss_peak_kb(),
        t: e.t,
        lm: e.lm,
        g: e.g,
    }
}

/// Number of standard monomials: squarefree masks (of `nv` variables) divisible
/// by no leading monomial.  Equals the number of solutions for a Groebner
/// basis of `I + FE` (selftest, small `nv` only).
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

fn json_job(
    label: &str,
    cell: &Cell,
    target: usize,
    arm: &str,
    solutions: u64,
    j: &JobOut,
    extra: &str,
) -> String {
    let t = &j.t;
    let arr = |v: &Vec<u64>| {
        format!(
            "[{}]",
            v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        )
    };
    format!(
        "{{\"cell\":\"{}\",\"m\":{},\"n\":{},\"ell\":{},\"nv\":{},\"target\":{},\"arm\":\"{}\",\"solutions\":{},\"status\":\"{}\",\"d_mat\":{},\"d_sel\":{},\"steps\":{},\"pairs_sel\":{},\"pairs_gen\":{},\"rows_max\":{},\"cols_max\":{},\"rc_max\":{},\"nnz_max\":{},\"cols_sum\":{},\"rows_sum\":{},\"touches\":{},\"build\":{},\"elim\":{},\"back\":{},\"zero_red\":{},\"new_elems\":{},\"g_size\":{},\"ms\":{:.1},\"ms_sym\":{:.1},\"ms_elim\":{:.1},\"ms_back\":{:.1},\"ms_add\":{:.1},\"rss_kb\":{},\"step_rows\":{},\"step_cols\":{}{}}}",
        label, cell.m, cell.n, cell.l, cell.nv(), target, arm, solutions, j.status.name(),
        t.d_mat, t.d_sel, t.steps, t.pairs_sel, t.pairs_gen, t.rows_max, t.cols_max, t.rc_max,
        t.nnz_max, t.cols_sum, t.rows_sum, t.touches(), t.build, t.elim, t.back, t.zero_red,
        t.new_elems, t.g_size, j.ms, t.ms_sym, t.ms_elim, t.ms_back, t.ms_add, j.rss_kb, arr(&t.step_rows), arr(&t.step_cols), extra
    )
}

// ---------------------------------------------------------------- draws

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
    }
}

fn mode_run(a: &Args) {
    let draws = read_draws(&a.flag("--draws").expect("--draws"));
    let label = a.flag("--cell").expect("--cell");
    let cell_draws: Vec<&Draw> = draws.iter().filter(|d| d.label == label).collect();
    assert!(!cell_draws.is_empty(), "no draws for {label}");
    let tgts = parse_targets(&a.flag("--targets").unwrap_or("all".into()), cell_draws.len());
    let arms: Vec<String> = a
        .flag("--arms")
        .unwrap_or_else(|| "grevlex".into())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let total_limit: f64 = a.num("--total-limit", 0.0);
    let per_job: f64 = a.num("--time-limit", 0.0);
    let t_start = Instant::now();
    let mut bad = false;
    for &ti in &tgts {
        let d = cell_draws.iter().find(|d| d.idx == ti).expect("target idx");
        let gf = Gf::new(d.cell.n);
        let (eqs, nv) = build_system(&gf, d.cell.m, &d.basis, d.xr);
        assert_eq!(nv, d.cell.nv());
        let sols = d.solutions;
        let adj = primal(&eqs, nv);
        let edges: u32 = adj.iter().map(|x| x.count_ones()).sum::<u32>() / 2;
        let (_, w_md) = elim_order(&adj, Heur::MinDegree);
        let (_, w_mf) = elim_order(&adj, Heur::MinFill);
        let extra = format!(
            ",\"pg_edges\":{edges},\"pg_pairs\":{},\"width_mindeg\":{w_md},\"width_minfill\":{w_mf}",
            nv * (nv - 1) / 2
        );
        for arm in &arms {
            let ord = match make_order(arm, &d.cell, &eqs) {
                Some(o) => o,
                None => {
                    println!(
                        "{{\"cell\":\"{}\",\"target\":{},\"arm\":\"{}\",\"status\":\"na\"}}",
                        label, ti, arm
                    );
                    continue;
                }
            };
            if total_limit > 0.0 && t_start.elapsed().as_secs_f64() > total_limit {
                println!(
                    "{{\"cell\":\"{}\",\"target\":{},\"arm\":\"{}\",\"status\":\"skipped\"}}",
                    label, ti, arm
                );
                continue;
            }
            let mut lim = limits_from(a);
            if per_job > 0.0 && total_limit > 0.0 {
                let left = total_limit - t_start.elapsed().as_secs_f64();
                lim.deadline = Some(
                    Instant::now() + std::time::Duration::from_secs_f64(per_job.min(left.max(1.0))),
                );
            }
            let j = run_job(&eqs, &ord, &lim);
            let consistent = match j.status {
                Status::Refuted => sols == 0,
                Status::Gb => sols > 0,
                _ => true,
            };
            if !consistent {
                eprintln!("INCONSISTENT {label} target {ti} arm {arm}: status {} with {sols} solutions", j.status.name());
                bad = true;
            }
            println!("{}", json_job(&label, &d.cell, ti, arm, sols, &j, &extra));
            eprintln!(
                "{label} t{ti} {arm}: {} D={} steps={} touches={} {:.1}s",
                j.status.name(),
                j.t.d_mat,
                j.t.steps,
                j.t.touches(),
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
        Some("sens") => mode_sens(&args),
        Some("summarize") => mode_summarize(&args),
        _ => {
            eprintln!("usage: b05 selftest|gen|run|sens|summarize ...");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------- selftest

fn fail(msg: &str) -> ! {
    println!("SELFTEST FAILED: {msg}");
    std::process::exit(2);
}

fn selftest_orders() {
    let mut rng = Rng(77);
    let nv = 20usize;
    let ident: Vec<usize> = (0..nv).collect();
    let ones = vec![1u32; nv];
    let zero = vec![0u8; nv];
    let g = Order::new("g", &ident, &ones, &zero);
    // identity grevlex agrees with the plain comparator
    for _ in 0..20000 {
        let a = rng.next() & ((1 << nv) - 1);
        let b = rng.next() & ((1 << nv) - 1);
        if a == b {
            continue;
        }
        if (g.key(a) > g.key(b)) != grevlex_greater(a, b) {
            fail(&format!("grevlex mismatch {a:#x} {b:#x}"));
        }
    }
    // multiplicativity and 1 minimal, for several orders
    let blk: Vec<u8> = (0..nv).map(|v| if v < 8 { 0 } else { 1 }).collect();
    let w: Vec<u32> = (0..nv).map(|v| if v < 8 { 1 } else { 2 }).collect();
    let perm = perm_from_seed(5, nv);
    let mut rank = vec![0usize; nv];
    for (r, &v) in perm.iter().enumerate() {
        rank[v] = r;
    }
    let orders = vec![
        Order::new("block", &ident, &ones, &blk),
        Order::new("weighted", &ident, &w, &zero),
        Order::new("perm", &rank, &ones, &zero),
        Order::new("gw", &rank, &w, &blk),
    ];
    for o in &orders {
        for _ in 0..20000 {
            let a = rng.next() & ((1 << nv) - 1);
            let b = rng.next() & ((1 << nv) - 1);
            let t = rng.next() & ((1 << nv) - 1) & !a & !b;
            if a == b {
                continue;
            }
            if (o.key(a) > o.key(b)) != (o.key(a | t) > o.key(b | t)) {
                fail(&format!("{} not multiplicative {a:#x} {b:#x} {t:#x}", o.name));
            }
            if o.key(0) >= o.key(a) && a != 0 {
                fail("1 is not minimal");
            }
        }
    }
    println!("orders ok: grevlex = plain comparator; block/weighted/permuted are multiplicative, 1 minimal");
}

struct Toy {
    cell: Cell,
}

fn selftest_toys() {
    let toys = [
        Toy { cell: Cell { m: 3, n: 5, l: 2 } },
        Toy { cell: Cell { m: 4, n: 4, l: 1 } },
        Toy { cell: Cell { m: 2, n: 7, l: 3 } },
        Toy { cell: Cell { m: 3, n: 6, l: 3 } },
        Toy { cell: Cell { m: 4, n: 5, l: 2 } },
        Toy { cell: Cell { m: 3, n: 7, l: 3 } },
        Toy { cell: Cell { m: 4, n: 6, l: 2 } },
        Toy { cell: Cell { m: 2, n: 9, l: 5 } },
    ];
    let arms: Vec<String> = ["grevlex", "blkLI", "blkIL", "wdeg", "mindeg", "tw", "mindeg-rev", "tw-rev", "rand0", "rand1", "rand2", "rand3"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut total_jobs = 0;
    for toy in &toys {
        let c = toy.cell;
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
            let want_sat = alg > 0;
            if (want_sat && sat >= 3) || (!want_sat && unsat >= 3) {
                continue;
            }
            let (eqs, nv) = build_system(&gf, c.m, &basis, xr);
            if nv != c.nv() {
                fail("nv mismatch");
            }
            let brute = brute_solutions(&eqs, nv);
            if brute != alg {
                fail(&format!("solution count: algebraic {alg} vs brute force {brute} at {:?}", (c.m, c.n, c.l, xr)));
            }
            if want_sat {
                sat += 1;
            } else {
                unsat += 1;
            }
            for arm in &arms {
                let ord = match make_order(arm, &c, &eqs) {
                    Some(o) => o,
                    None => continue,
                };
                let lim = Limits { deadline: None, max_cols: 1 << 22, vm_cap: 0 };
                let j = run_job(&eqs, &ord, &lim);
                total_jobs += 1;
                if alg == 0 {
                    if j.status != Status::Refuted {
                        fail(&format!("unsat system not refuted: {:?} {arm} -> {}", (c.m, c.n, c.l, xr), j.status.name()));
                    }
                } else {
                    if j.status != Status::Gb {
                        fail(&format!("sat system: status {} for {arm}", j.status.name()));
                    }
                    let sm = standard_monomials(&j.lm, nv);
                    if sm != alg {
                        fail(&format!("not a Groebner basis: {sm} standard monomials vs {alg} solutions ({arm}, {:?})", (c.m, c.n, c.l, xr)));
                    }
                    // every input equation reduces to zero: all roots of G are roots of the input
                    // (checked through the count above); also the basis elements vanish on the solutions
                    let sols: Vec<u64> = (0..(1u64 << nv)).filter(|&pt| eqs.iter().all(|p| eval_poly(p, pt) == 0)).collect();
                    for gp in &j.g {
                        for &pt in &sols {
                            if eval_poly(gp, pt) != 0 {
                                fail("basis element does not vanish on a solution");
                            }
                        }
                    }
                }
            }
        }
        if unsat < 3 || sat < 3 {
            println!("toy {:?}: found unsat {unsat} sat {sat} (not both) - continuing", (c.m, c.n, c.l));
        } else {
            println!("toy (m,n,l)={:?} nv={}: {} arms x (3 unsat refuted, 3 sat certified by standard-monomial count)", (c.m, c.n, c.l), c.nv(), arms.len());
        }
    }
    println!("toy jobs: {total_jobs}");
}

fn selftest_treewidth() {
    let cell = Cell { m: 3, n: 4, l: 2 }; // nv = 10
    let gf = Gf::new(cell.n);
    let mut rng = Rng(5);
    let basis = random_basis(cell.n, cell.l, &mut rng);
    let (eqs, nv) = build_system(&gf, cell.m, &basis, 5);
    let adj = primal(&eqs, nv);
    let ex = exact_treewidth(&adj);
    let (_, w_md) = elim_order(&adj, Heur::MinDegree);
    let (_, w_mf) = elim_order(&adj, Heur::MinFill);
    println!("treewidth, descended cell (3,4,2) nv={nv}: exact {ex}, min-degree width {w_md}, min-fill width {w_mf}");
    if w_md < ex || w_mf < ex {
        fail("a greedy width is below the exact treewidth");
    }
    // path graph: tw 1, exact and greedy agree
    let pn = 12;
    let mut padj = vec![0u64; pn];
    for v in 0..pn - 1 {
        padj[v] |= 1 << (v + 1);
        padj[v + 1] |= 1 << v;
    }
    if exact_treewidth(&padj) != 1 || elim_order(&padj, Heur::MinFill).1 != 1 {
        fail("path graph treewidth");
    }
    // complete graph K6: tw 5
    let kn = 6;
    let kadj: Vec<u64> = (0..kn).map(|v| ((1u64 << kn) - 1) & !(1 << v)).collect();
    if exact_treewidth(&kadj) != 5 || elim_order(&kadj, Heur::MinDegree).1 != 5 {
        fail("complete graph treewidth");
    }
    println!("treewidth ok: path = 1, K6 = 5, greedy >= exact on the descended cell");
}

fn selftest_controls(a: &Args) {
    let path = match a.flag("--controls") {
        Some(p) => p,
        None => return,
    };
    let max_nv: usize = a.num("--max-nv", 22);
    let txt = std::fs::read_to_string(&path).expect("controls");
    let (mut rows, mut agree_cl, mut agree_ms, mut have_ms, mut below, mut above) = (0, 0, 0, 0, 0, 0);
    let mut lines_out: Vec<String> = Vec::new();
    for line in txt.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let m: usize = f[1].parse().unwrap();
        let n: u32 = f[2].parse().unwrap();
        let l: usize = f[3].parse().unwrap();
        let basis: Vec<u64> = f[4].split(',').map(|x| x.parse().unwrap()).collect();
        let xr: u64 = f[5].parse().unwrap();
        let sols: u64 = f[6].parse().unwrap();
        let ref_cl: i64 = f[7].parse().unwrap();
        let ref_ms: i64 = f[8].parse().unwrap();
        let cell = Cell { m, n, l };
        if cell.nv() > max_nv {
            continue;
        }
        let gf = Gf::new(n);
        let (eqs, nv) = build_system(&gf, m, &basis, xr);
        let alg = count_solutions(&gf, m, &basis, xr);
        if alg != sols {
            fail(&format!("committed solution count {sols} vs rebuilt {alg} for {line}"));
        }
        let ord = make_order("grevlex", &cell, &eqs).unwrap();
        let lim = Limits { deadline: Some(Instant::now() + std::time::Duration::from_secs(60)), max_cols: 1 << 22, vm_cap: 0 };
        let j = run_job(&eqs, &ord, &lim);
        if sols == 0 && j.status != Status::Refuted {
            fail(&format!("committed unsat draw not refuted ({}) nv={nv}: {line}", j.status.name()));
        }
        rows += 1;
        let d = j.t.d_mat as i64;
        if d == ref_cl {
            agree_cl += 1;
        } else if d < ref_cl {
            below += 1;
        } else {
            above += 1;
        }
        if ref_ms >= 0 {
            have_ms += 1;
            if d == ref_ms {
                agree_ms += 1;
            }
        }
        lines_out.push(format!(
            "  control (m,n,l)=({m},{n},{l}) nv={nv} D_mat(grevlex)={d} D_sel={} committed closure={ref_cl} msolve={ref_ms}",
            j.t.d_sel
        ));
    }
    for l in &lines_out {
        println!("{l}");
    }
    println!("committed draws: {rows} unsat draws refuted by the F4; D_mat(grevlex) equals the committed closure degree on {agree_cl}, is below on {below}, above on {above}; equals msolve on {agree_ms} of {have_ms}");
}

fn mode_selftest(a: &Args) {
    selftest_orders();
    selftest_toys();
    selftest_treewidth();
    selftest_controls(a);
    println!("SELFTEST OK");
}

// ---------------------------------------------------------------- sensitivity control

fn rand_poly(rng: &mut Rng, vars: &[usize], nterms: usize, with_const: bool) -> Poly {
    let mut v: Vec<u64> = Vec::new();
    for _ in 0..nterms {
        let a = vars[(rng.next() % vars.len() as u64) as usize];
        let mask = if rng.next() % 3 == 0 {
            1u64 << a
        } else {
            let b = vars[(rng.next() % vars.len() as u64) as usize];
            (1u64 << a) | (1u64 << b)
        };
        v.push(mask);
    }
    if with_const && rng.next() & 1 == 1 {
        v.push(0);
    }
    norm(v)
}

/// A planted order effect: `ny` definitions `y_j + q_j(x)` plus random
/// quadratic relations in `(x, y)`.  Block order `y >> x` substitutes the
/// definitions at once; `x >> y` cannot.  The control passes when the two block
/// orders differ by at least the rule's 1.5x in touches on every instance and
/// every run agrees with brute force.
fn mode_sens(a: &Args) {
    let nx: usize = a.num("--nx", 10);
    let ny: usize = a.num("--ny", 8);
    let ninst: usize = a.num("--instances", 6);
    let nrel: usize = a.num("--relations", 14);
    let nv = nx + ny;
    let xs: Vec<usize> = (0..nx).collect();
    let all: Vec<usize> = (0..nv).collect();
    let ident: Vec<usize> = (0..nv).collect();
    let ones = vec![1u32; nv];
    // y block first (0), x block second (1)
    let blk_yx: Vec<u8> = (0..nv).map(|v| if v >= nx { 0 } else { 1 }).collect();
    let blk_xy: Vec<u8> = (0..nv).map(|v| if v >= nx { 1 } else { 0 }).collect();
    let mut all_ok = true;
    let mut min_ratio = f64::INFINITY;
    for k in 0..ninst {
        let mut rng = Rng(0xC0DE_0005_0000_0000 ^ k as u64);
        let mut eqs: Vec<Poly> = Vec::new();
        for j in 0..ny {
            let mut q = rand_poly(&mut rng, &xs, 8, true);
            q = padd(&q, &vec![1u64 << (nx + j)]);
            eqs.push(q);
        }
        for _ in 0..nrel {
            eqs.push(rand_poly(&mut rng, &all, 10, true));
        }
        eqs.retain(|p| !p.is_empty());
        let brute = brute_solutions(&eqs, nv);
        let o1 = Order::new("blkYX", &ident, &ones, &blk_yx);
        let o2 = Order::new("blkXY", &ident, &ones, &blk_xy);
        let lim = Limits { deadline: None, max_cols: 1 << 22, vm_cap: 0 };
        let j1 = run_job(&eqs, &o1, &lim);
        let j2 = run_job(&eqs, &o2, &lim);
        let ok1 = (brute == 0 && j1.status == Status::Refuted) || (brute > 0 && j1.status == Status::Gb && standard_monomials(&j1.lm, nv) == brute);
        let ok2 = (brute == 0 && j2.status == Status::Refuted) || (brute > 0 && j2.status == Status::Gb && standard_monomials(&j2.lm, nv) == brute);
        let (t1, t2) = (j1.t.touches().max(1) as f64, j2.t.touches().max(1) as f64);
        let ratio = (t1 / t2).max(t2 / t1);
        min_ratio = min_ratio.min(ratio);
        println!(
            "{{\"sens_instance\":{k},\"solutions\":{brute},\"blkYX_status\":\"{}\",\"blkXY_status\":\"{}\",\"blkYX_touches\":{},\"blkXY_touches\":{},\"blkYX_d_mat\":{},\"blkXY_d_mat\":{},\"ratio\":{ratio:.3},\"agree_with_brute_force\":{}}}",
            j1.status.name(), j2.status.name(), j1.t.touches(), j2.t.touches(), j1.t.d_mat, j2.t.d_mat, ok1 && ok2
        );
        if !(ok1 && ok2) {
            all_ok = false;
        }
    }
    println!("sensitivity: smallest block-order touches ratio over {ninst} planted instances = {min_ratio:.3}");
    if !all_ok {
        println!("SENS FAILED: a run disagrees with brute force");
        std::process::exit(2);
    }
    if min_ratio < 1.5 {
        println!("SENS FAILED: the instrument does not see a 1.5x order effect on every planted instance");
        std::process::exit(2);
    }
    println!("SENS OK");
}

// ---------------------------------------------------------------- summary by the frozen rule

#[derive(Clone)]
struct Rec {
    cell: String,
    target: usize,
    arm: String,
    status: String,
    d_mat: f64,
    touches: f64,
    rc_max: f64,
    ms: f64,
}

fn jnum_f(line: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == 'E' || c == '+')).unwrap_or(rest.len());
    rest[..end].parse().ok()
}
fn jstr_f(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":\"");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    Some(rest[..rest.find('"')?].to_string())
}

fn parse_rec(line: &str) -> Option<Rec> {
    if !line.starts_with("{\"cell\"") {
        return None;
    }
    Some(Rec {
        cell: jstr_f(line, "cell")?,
        target: jnum_f(line, "target")? as usize,
        arm: jstr_f(line, "arm")?,
        status: jstr_f(line, "status")?,
        d_mat: jnum_f(line, "d_mat").unwrap_or(0.0),
        touches: jnum_f(line, "touches").unwrap_or(0.0),
        rc_max: jnum_f(line, "rc_max").unwrap_or(0.0),
        ms: jnum_f(line, "ms").unwrap_or(0.0),
    })
}

fn gmean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}

/// `summarize --dir <raw dir> [--cells a,b,c] [--min-targets 4]`: read every
/// `*.stdout.log` in the directory, pair each arm with grevlex target by target
/// and apply the frozen rule of PROTOCOL.md.  Prints; decides nothing else.
fn mode_summarize(a: &Args) {
    let dir = a.flag("--dir").expect("--dir");
    let cells: Vec<String> = a
        .flag("--cells")
        .unwrap_or_else(|| "c17-4-3,c19-4-3,c13-3-4,c15-3-4,c17-4-4,c31-15-2".into())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let min_t: usize = a.num("--min-targets", 4);
    let min_arm: usize = a.num("--min-arm-targets", 2);
    let mut recs: Vec<Rec> = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".stdout.log"))
        .collect();
    files.sort();
    for f in &files {
        for line in std::fs::read_to_string(f).unwrap_or_default().lines() {
            if let Some(r) = parse_rec(line) {
                recs.push(r);
            }
        }
    }
    println!("read {} records from {} files", recs.len(), files.len());
    let get = |cell: &str, arm: &str, t: usize| -> Option<&Rec> {
        recs.iter().find(|r| r.cell == cell && r.arm == arm && r.target == t)
    };
    let fixed = ["blkLI", "blkIL", "wdeg", "mindeg", "tw"];
    let desc = ["mindeg-rev", "tw-rev"];
    let rands: Vec<String> = (0..8).map(|d| format!("rand{d}")).collect();
    // per cell, per arm: (n, GM touches ratio, GM rc ratio, fraction D lower)
    struct Stat {
        n: usize,
        ratio: f64,
        rc_ratio: f64,
        dfrac: f64,
        rmin: f64,
        rmax: f64,
        done: bool,
        capped: usize,
    }
    let stat = |cell: &str, arm: &str, base: &[usize]| -> Stat {
        let mut ratios = Vec::new();
        let mut rcs = Vec::new();
        let mut dl = 0usize;
        let mut done = !base.is_empty();
        let mut capped = 0usize;
        for &t in base {
            match (get(cell, "grevlex", t), get(cell, arm, t)) {
                (Some(g), Some(r)) if g.status == "refuted" && r.status == "refuted" => {
                    ratios.push(g.touches / r.touches.max(1.0));
                    rcs.push(g.rc_max / r.rc_max.max(1.0));
                    if r.d_mat <= g.d_mat - 1.0 {
                        dl += 1;
                    }
                }
                (Some(g), Some(r)) if g.status == "refuted" && (r.status == "timelimit" || r.status == "memcap" || r.status == "colscap") => {
                    capped += 1;
                    done = false;
                }
                _ => done = false,
            }
        }
        let n = ratios.len();
        Stat {
            n,
            ratio: gmean(&ratios),
            rc_ratio: gmean(&rcs),
            dfrac: if n > 0 { dl as f64 / n as f64 } else { 0.0 },
            rmin: ratios.iter().cloned().fold(f64::INFINITY, f64::min),
            rmax: ratios.iter().cloned().fold(0.0, f64::max),
            done,
            capped,
        }
    };
    // arm -> list of (cell, Stat, band_max_ratio, band_max_dfrac, band_complete)
    let mut qual_const: HashMap<String, Vec<String>> = HashMap::new();
    let mut qual_const_mat: HashMap<String, Vec<String>> = HashMap::new();
    let mut qual_prim: HashMap<String, Vec<String>> = HashMap::new();
    let mut worse: HashMap<String, usize> = HashMap::new();
    let mut countable_cells = 0usize;
    for cell in &cells {
        let base: Vec<usize> = {
            let mut v: Vec<usize> = recs
                .iter()
                .filter(|r| &r.cell == cell && r.arm == "grevlex" && r.status == "refuted")
                .map(|r| r.target)
                .collect();
            v.sort();
            v.dedup();
            v
        };
        println!("\n== cell {cell}: grevlex refuted on {} targets {:?}", base.len(), base);
        if let Some(g) = recs.iter().find(|r| &r.cell == cell && r.arm == "grevlex") {
            let _ = g;
        }
        let gt: Vec<f64> = base.iter().filter_map(|&t| get(cell, "grevlex", t)).map(|r| r.touches).collect();
        let gd: Vec<f64> = base.iter().filter_map(|&t| get(cell, "grevlex", t)).map(|r| r.d_mat).collect();
        let gms: f64 = base.iter().filter_map(|&t| get(cell, "grevlex", t)).map(|r| r.ms).sum();
        println!("   grevlex: GM touches {:.4e}, D_mat {:?}, wall {:.1}s", gmean(&gt), gd, gms / 1000.0);
        let rstats: Vec<Stat> = rands.iter().map(|r| stat(cell, r, &base)).collect();
        let band_complete = base.len() >= min_t && rstats.iter().all(|s| s.done && s.n == base.len());
        let band_max = rstats.iter().map(|s| s.ratio).fold(0.0, f64::max);
        let band_min = rstats.iter().map(|s| s.ratio).fold(f64::INFINITY, f64::min);
        let band_dmax = rstats.iter().map(|s| s.dfrac).fold(0.0, f64::max);
        println!(
            "   random band (8 draws, all {} targets): touches ratio [{:.3}, {:.3}], D-drop fraction max {:.2}, complete {}",
            base.len(), band_min, band_max, band_dmax, band_complete
        );
        if band_complete {
            countable_cells += 1;
        }
        for arm in fixed.iter().chain(desc.iter()) {
            let eligible = fixed.contains(arm);
            // the arm's own target set: the grevlex-refuted targets it has a record for
            let arm_t: Vec<usize> = base
                .iter()
                .cloned()
                .filter(|&t| {
                    get(cell, arm, t)
                        .map_or(false, |r| matches!(r.status.as_str(), "refuted" | "timelimit" | "memcap" | "colscap"))
                })
                .collect();
            let s = stat(cell, arm, &arm_t);
            // the random band is recomputed on the same targets, so the comparison is paired
            let rs: Vec<Stat> = rands.iter().map(|r| stat(cell, r, &arm_t)).collect();
            let arm_band_ok = arm_t.len() >= min_arm && rs.iter().all(|x| x.done && x.n == arm_t.len());
            let a_max = rs.iter().map(|x| x.ratio).fold(0.0, f64::max);
            let a_dmax = rs.iter().map(|x| x.dfrac).fold(0.0, f64::max);
            let ok_cell = eligible && arm_band_ok && s.done && s.n == arm_t.len();
            println!(
                "   {:<10} targets {:?} touches ratio {:.3} (per-target {:.3}..{:.3}) rc ratio {:.3} D-drop frac {:.2} | band max on these targets {:.3}, D-drop {:.2} | done {} capped {}{}",
                arm, arm_t, s.ratio, s.rmin, s.rmax, s.rc_ratio, s.dfrac, a_max, a_dmax, s.done, s.capped,
                if eligible { "" } else { "  [descriptive]" }
            );
            if eligible && arm_band_ok && s.capped > 0 {
                // the arm hit its time or memory cap where grevlex finished: counted as worse
                *worse.entry(arm.to_string()).or_default() += 1;
            }
            if ok_cell {
                if s.ratio >= 1.5 && s.ratio > a_max {
                    qual_const.entry(arm.to_string()).or_default().push(cell.clone());
                    if s.rc_ratio >= 1.0 / 1.5 {
                        qual_const_mat.entry(arm.to_string()).or_default().push(cell.clone());
                    }
                }
                if s.dfrac >= 0.75 && s.dfrac > a_dmax {
                    qual_prim.entry(arm.to_string()).or_default().push(cell.clone());
                }
                if s.ratio <= 1.0 / 1.5 {
                    *worse.entry(arm.to_string()).or_default() += 1;
                }
            }
        }
    }
    println!("\n== every record (cell, target, arm, status, D_mat, touches, rc_max, wall ms)");
    let mut sorted = recs.clone();
    sorted.sort_by(|x, y| (x.cell.clone(), x.target, x.arm.clone()).cmp(&(y.cell.clone(), y.target, y.arm.clone())));
    for r in &sorted {
        println!("RAW {} t{} {} {} D={} touches={:.0} rc_max={:.0} ms={:.0}", r.cell, r.target, r.arm, r.status, r.d_mat, r.touches, r.rc_max, r.ms);
    }
    println!("\n== frozen rule");
    let mut adv: Vec<String> = Vec::new();
    let mut trade: Vec<String> = Vec::new();
    for arm in fixed.iter() {
        let qc = qual_const.get(*arm).map(|v| v.len()).unwrap_or(0);
        let qm = qual_const_mat.get(*arm).map(|v| v.len()).unwrap_or(0);
        let qp = qual_prim.get(*arm).map(|v| v.len()).unwrap_or(0);
        println!(
            "RULE {arm}: constant cells {qc}/6 {:?}; of them with matrix <= 1.5x of grevlex {qm}; primitive cells {qp}/6 {:?}; touches >= 1.5x worse on {} cells",
            qual_const.get(*arm).cloned().unwrap_or_default(),
            qual_prim.get(*arm).cloned().unwrap_or_default(),
            worse.get(*arm).copied().unwrap_or(0)
        );
        if qp >= 4 {
            adv.push(format!("{arm} (primitive)"));
        }
        if qc >= 4 {
            if qm >= 4 {
                adv.push(format!("{arm} (constant)"));
            } else {
                trade.push(format!("{arm} (constant touches, larger matrix)"));
            }
        }
    }
    println!("countable cells (grevlex, and the 8 random draws, finished >= {min_t} common targets): {countable_cells}/6");
    let all_worse = fixed.iter().all(|a| worse.get(*a).copied().unwrap_or(0) >= 4);
    if countable_cells < 4 {
        println!("VERDICT: INCONCLUSIVE (fewer than 4 of 6 cells countable)");
    } else if !adv.is_empty() {
        println!("VERDICT: ADVANCES {:?}", adv);
    } else if !trade.is_empty() {
        println!("VERDICT: TRADES {:?}", trade);
    } else if all_worse {
        println!("VERDICT: REGRESSES (every structure-aware arm is >= 1.5x worse than grevlex on >= 4 cells)");
    } else {
        println!("VERDICT: MATCHES (no arm clears the frozen rule)");
    }
}
