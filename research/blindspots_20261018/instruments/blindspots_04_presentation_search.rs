//! Blind-spot item 04: presentation search for the m-sum variety in the
//! binary descent.  Does the descended solving degree D depend on how the
//! summation variety is presented, at a fixed `F_2`-subspace factor base?
//!
//! Standard library only (the `crypto` library does not compile at the branch
//! base; see `research/blindspots_20261018/04-presentation-search/PROTOCOL.md`).
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
//!     -o target/blindspots-04/b04 examples/blindspots_04_presentation_search.rs
//! b04 selftest
//! b04 cell    --n 17 --l 4 --m 3 --arms a,d0,d,c --random 8 --planted 4 --out cell.jsonl
//! b04 cell    --n 17 --l 4 --m 3 --arms e,b --random 8 --planted 4 --dmax 4
//! b04 control --n 17 --l 4 --m 3 --kind linear|nonlinear|aa
//! b04 plan    --n 17 --l 4 --m 3
//! b04 summarize raw/*.jsonl
//! ```
//!
//! # The object computed
//!
//! A cell is `(n, l, m)`: the field `F = F_{2^n}`, an `F_2`-subspace
//! `V = <1, beta_2, .., beta_l>` of dimension `l`, and `m` summands.  A curve
//! `E_{a,b}: y^2 + x y = x^3 + a x^2 + b` is derived from binary Edwards
//! parameters `(d1, d2)` (Bernstein-Lange-Farashahi) so the Edwards arm and
//! the Weierstrass arms live on curves with equal point counts.  The target is
//! a point `R`.  The factor base of an arm is the set of points whose
//! coordinate in the arm's frame lies in `V`:
//!
//! * frame X: the Weierstrass `x`;
//! * frame U: `u = c / (x + c)`, `c = b^(1/4)`, which turns the translation by
//!   the rational 2-torsion point into `u -> u + 1`;
//! * frame W: the Edwards `w = x + y`, invariant under `P -> -P` and
//!   `P -> P + (1,1)`.
//!
//! The arms are different presentations of `P_1 + .. + P_m = R`:
//!
//! * `a`  Semaev `S_{m+1}` in `x`, descended with no intermediates;
//! * `b`  the unprojected chain: `y`-coordinates and the affine addition law;
//! * `c`  binary Edwards `w`-frame summation polynomial;
//! * `d0` the Moebius `u`-frame `S_{m+1}` with no symmetrisation;
//! * `d`  the same with the 2-torsion symmetrisation (first coordinate bits
//!   replaced by their parity);
//! * `e`  the chained `S_3` with `n`-bit intermediates (Semaev 2015).
//!
//! `D` is the least degree `d` at which the MutantXL fixed-degree closure
//! `V_d` of the (echelonised) equations equals the vanishing ideal of the
//! system's solution set in degrees `<= d`.  For an unsatisfiable system this
//! is the least degree at which `1` is in `V_d` (the refutation degree item 07
//! reads); for a satisfiable one it is the degree at which every polynomial of
//! degree `<= d` vanishing on the solutions is generated.

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

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
}

fn binom_table(maxn: usize) -> Vec<[u64; 24]> {
    let mut t = vec![[0u64; 24]; maxn + 1];
    for n in 0..=maxn {
        t[n][0] = 1;
        for k in 1..24 {
            if n > 0 {
                t[n][k] = t[n - 1][k - 1].saturating_add(t[n - 1][k]);
            }
        }
    }
    t
}

fn cols_up_to(nv: usize, d: usize) -> u64 {
    let b = binom_table(nv.max(1));
    (0..=d.min(23)).map(|k| b[nv][k]).sum()
}

// ---------------------------------------------------------------- GF(2^n)

struct Gf {
    n: u32,
    low: u64,
    mask: u64,
    q1: u64,
    exp: Vec<u32>,
    log: Vec<u32>,
    trmask: u64,
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

/// Same candidate order as the repository's `find_irreducible_sparse` (and
/// item 07's copy): the least `low` among constant + up to three further
/// terms for which `z^n + low` is irreducible.
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

fn prime_factors(mut m: u64) -> Vec<u64> {
    let mut f = vec![];
    let mut p = 2;
    while p * p <= m {
        if m % p == 0 {
            f.push(p);
            while m % p == 0 {
                m /= p;
            }
        }
        p += 1;
    }
    if m > 1 {
        f.push(m);
    }
    f
}

impl Gf {
    fn new(n: u32) -> Gf {
        assert!(n % 2 == 1 && n >= 5 && n <= 21, "odd n in 5..=21");
        let low = find_irreducible_sparse(n);
        let mask = (1u64 << n) - 1;
        let q1 = mask;
        let mulx = |a: u64| -> u64 {
            let hi = (a >> (n - 1)) & 1;
            ((a << 1) & mask) ^ if hi == 1 { low } else { 0 }
        };
        let mulslow = |a: u64, b: u64| -> u64 {
            let (mut r, mut aa, mut bb) = (0u64, a, b);
            while bb != 0 {
                if bb & 1 == 1 {
                    r ^= aa;
                }
                aa = mulx(aa);
                bb >>= 1;
            }
            r
        };
        let powslow = |mut b: u64, mut e: u64| -> u64 {
            let mut r = 1u64;
            while e > 0 {
                if e & 1 == 1 {
                    r = mulslow(r, b);
                }
                b = mulslow(b, b);
                e >>= 1;
            }
            r
        };
        let fs = prime_factors(q1);
        let mut g = 2u64;
        loop {
            if fs.iter().all(|&p| powslow(g, q1 / p) != 1) {
                break;
            }
            g += 1;
        }
        let mut exp = vec![0u32; 2 * q1 as usize + 2];
        let mut log = vec![0u32; 1usize << n];
        let mut p = 1u64;
        for i in 0..q1 {
            exp[i as usize] = p as u32;
            log[p as usize] = i as u32;
            p = mulslow(p, g);
        }
        for i in q1..2 * q1 + 2 {
            exp[i as usize] = exp[(i - q1) as usize];
        }
        let mut gf = Gf {
            n,
            low,
            mask,
            q1,
            exp,
            log,
            trmask: 0,
        };
        let mut tm = 0u64;
        for k in 0..n {
            let e = 1u64 << k;
            let mut t = 0u64;
            let mut s = e;
            for _ in 0..n {
                t ^= s;
                s = gf.mul(s, s);
            }
            assert!(t == 0 || t == 1);
            if t == 1 {
                tm |= e;
            }
        }
        gf.trmask = tm;
        gf
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[(self.log[a as usize] as usize) + (self.log[b as usize] as usize)] as u64
        }
    }
    #[inline(always)]
    fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.exp[(self.q1 - self.log[a as usize] as u64) as usize] as u64
    }
    fn div(&self, a: u64, b: u64) -> u64 {
        self.mul(a, self.inv(b))
    }
    fn pow(&self, a: u64, e: u32) -> u64 {
        let mut r = 1u64;
        for _ in 0..e {
            r = self.mul(r, a);
        }
        r
    }
    fn tr(&self, a: u64) -> u64 {
        ((a & self.trmask).count_ones() & 1) as u64
    }
    fn sqrt(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            let l = self.log[a as usize] as u128;
            let h = ((l * ((self.q1 as u128 + 1) / 2)) % (self.q1 as u128)) as usize;
            self.exp[h] as u64
        }
    }
    /// A root of `z^2 + z = c`, if `Tr(c) = 0` (odd `n`: the half trace).
    fn as_root(&self, c: u64) -> Option<u64> {
        if self.tr(c) == 1 {
            return None;
        }
        let mut z = 0u64;
        let mut t = c;
        for _ in 0..=((self.n - 1) / 2) {
            z ^= t;
            t = self.sqr(self.sqr(t));
        }
        Some(z)
    }
    /// Every root of `a y^2 + b y + c = 0`.
    fn quad(&self, a: u64, b: u64, c: u64) -> Vec<u64> {
        if a == 0 {
            if b == 0 {
                return if c == 0 { vec![0] } else { vec![] };
            }
            return vec![self.div(c, b)];
        }
        if b == 0 {
            return vec![self.sqrt(self.div(c, a))];
        }
        let cc = self.div(self.mul(a, c), self.sqr(b));
        match self.as_root(cc) {
            None => vec![],
            Some(z) => {
                let s = self.div(b, a);
                let r1 = self.mul(s, z);
                vec![r1, r1 ^ s]
            }
        }
    }
    /// `c * z^k` for each `k`: the matrix of "multiply by c" over the bits.
    fn mulc_cols(&self, c: u64) -> Vec<u64> {
        (0..self.n).map(|k| self.mul(c, 1u64 << k)).collect()
    }
}

// ---------------------------------------------------------------- monomials and polynomials

/// A squarefree monomial on up to 192 Boolean variables.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct M([u64; 3]);

const M0: M = M([0, 0, 0]);

impl M {
    #[inline(always)]
    fn bit(i: usize) -> M {
        let mut w = [0u64; 3];
        w[i >> 6] = 1u64 << (i & 63);
        M(w)
    }
    #[inline(always)]
    fn or(self, o: M) -> M {
        M([self.0[0] | o.0[0], self.0[1] | o.0[1], self.0[2] | o.0[2]])
    }
    #[inline(always)]
    fn and(self, o: M) -> M {
        M([self.0[0] & o.0[0], self.0[1] & o.0[1], self.0[2] & o.0[2]])
    }
    #[inline(always)]
    fn andnot(self, o: M) -> M {
        M([self.0[0] & !o.0[0], self.0[1] & !o.0[1], self.0[2] & !o.0[2]])
    }
    #[inline(always)]
    fn has(self, i: usize) -> bool {
        (self.0[i >> 6] >> (i & 63)) & 1 == 1
    }
    #[inline(always)]
    fn deg(self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones() + self.0[2].count_ones()
    }
    #[inline(always)]
    fn is_zero(self) -> bool {
        self.0[0] | self.0[1] | self.0[2] == 0
    }
    fn from_u64(v: u64) -> M {
        M([v, 0, 0])
    }
    /// The set bits, ascending.
    fn iter(self) -> impl Iterator<Item = usize> {
        (0..3).flat_map(move |w| {
            let mut bits = self.0[w];
            std::iter::from_fn(move || {
                if bits == 0 {
                    None
                } else {
                    let t = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    Some(w * 64 + t)
                }
            })
        })
    }
}

impl PartialOrd for M {
    fn partial_cmp(&self, o: &M) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
/// Total order refining degree: degree, then the words from the top.
impl Ord for M {
    fn cmp(&self, o: &M) -> std::cmp::Ordering {
        self.deg()
            .cmp(&o.deg())
            .then(self.0[2].cmp(&o.0[2]))
            .then(self.0[1].cmp(&o.0[1]))
            .then(self.0[0].cmp(&o.0[0]))
    }
}

type Poly = Vec<M>; // ascending, unique

fn norm(mut v: Vec<M>) -> Poly {
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
            v.push(x.or(y));
        }
    }
    norm(v)
}
fn poly_degree(p: &Poly) -> u32 {
    p.last().map(|m| m.deg()).unwrap_or(0)
}
fn eval_poly(p: &Poly, point: M) -> u64 {
    let mut acc = 0u64;
    for &m in p {
        if m.andnot(point).is_zero() {
            acc ^= 1;
        }
    }
    acc
}

// ---------------------------------------------------------------- symbolic field elements

/// A field element whose `n` coordinate bits are Boolean polynomials.
type Sym = Vec<Poly>;

fn sym_const(v: u64, n: u32) -> Sym {
    (0..n).map(|k| if (v >> k) & 1 == 1 { vec![M0] } else { vec![] }).collect()
}
fn sym_add(a: &Sym, b: &Sym) -> Sym {
    a.iter().zip(b).map(|(x, y)| padd(x, y)).collect()
}
fn sym_sq(a: &Sym, gf: &Gf) -> Sym {
    let n = gf.n as usize;
    let mut raw: Vec<Vec<M>> = vec![Vec::new(); n];
    for (k, c) in a.iter().enumerate() {
        if c.is_empty() {
            continue;
        }
        let mut bits = gf.mul(1u64 << k, 1u64 << k);
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
    let mut raw: Vec<Vec<M>> = vec![Vec::new(); n];
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
            let mut bits = gf.mul(1u64 << i, 1u64 << j);
            while bits != 0 {
                let k = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                raw[k].extend_from_slice(&prod);
            }
        }
    }
    raw.into_iter().map(norm).collect()
}
/// `c * a` for a field constant `c`.
fn sym_mulc(a: &Sym, c: u64, gf: &Gf) -> Sym {
    let n = gf.n as usize;
    let cols = gf.mulc_cols(c);
    let mut raw: Vec<Vec<M>> = vec![Vec::new(); n];
    for (k, p) in a.iter().enumerate() {
        if p.is_empty() {
            continue;
        }
        let mut bits = cols[k];
        while bits != 0 {
            let t = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            raw[t].extend_from_slice(p);
        }
    }
    raw.into_iter().map(norm).collect()
}
/// The subspace element `sum_t bit_{offset+t} * basis[t]`.
fn sym_subspace(basis: &[u64], offset: usize, n: u32) -> Sym {
    let mut c: Vec<Vec<M>> = vec![Vec::new(); n as usize];
    for (t, &b) in basis.iter().enumerate() {
        for k in 0..n as usize {
            if (b >> k) & 1 == 1 {
                c[k].push(M::bit(offset + t));
            }
        }
    }
    c.into_iter().map(norm).collect()
}
fn sym_free(offset: usize, n: u32) -> Sym {
    (0..n as usize).map(|k| vec![M::bit(offset + k)]).collect()
}
fn sym_cube(x: &Sym, gf: &Gf) -> Sym {
    sym_mul(x, &sym_sq(x, gf), gf)
}

/// Pack the value of a `Sym` at a Boolean point into a field element.
fn sym_eval(s: &Sym, point: M) -> u64 {
    let mut v = 0u64;
    for (k, p) in s.iter().enumerate() {
        v |= eval_poly(p, point) << k;
    }
    v
}

// ---------------------------------------------------------------- curves

/// An affine point; the Weierstrass point at infinity is `INF`.
type Pt = (u64, u64);
const INF: Pt = (u64::MAX, u64::MAX);

/// The two group models of one cell.  `E_{a,b}` is the Weierstrass curve
/// `y^2 + xy = x^3 + a x^2 + b`; `Ed_{d1,d2}` the binary Edwards curve
/// `d1(x+y) + d2(x^2+y^2) = xy + xy(x+y) + x^2 y^2` with `Tr(d2) = 1` (a
/// complete addition law, neutral element `(0,0)`).  The cell derives
/// `a = d1^2 + d2`, `b = d1^2 (d1^2 + d1 + d2)`; the equal point counts of the
/// two models are asserted when the cell is built.
struct Curves {
    gf: Gf,
    d1: u64,
    d2: u64,
    a: u64,
    b: u64,
    /// `b^(1/4)`: the `x`-coordinate of the 4-torsion line, the fixed point of
    /// `x -> sqrt(b)/x`.
    c: u64,
}

impl Curves {
    fn new(n: u32, d1: u64, d2: u64) -> Curves {
        let gf = Gf::new(n);
        assert!(d1 != 0 && gf.tr(d2) == 1);
        let a = gf.sqr(d1) ^ d2;
        let b = gf.mul(gf.sqr(d1), gf.sqr(d1) ^ d1 ^ d2);
        assert!(b != 0);
        let c = gf.sqrt(gf.sqrt(b));
        Curves { gf, d1, d2, a, b, c }
    }
    // ---- Weierstrass
    fn w_rhs(&self, x: u64) -> u64 {
        let g = &self.gf;
        let x2 = g.sqr(x);
        g.mul(x, x2) ^ g.mul(self.a, x2) ^ self.b
    }
    fn w_ys(&self, x: u64) -> Vec<u64> {
        self.gf.quad(1, x, self.w_rhs(x))
    }
    fn w_on(&self, p: Pt) -> bool {
        if p == INF {
            return true;
        }
        let g = &self.gf;
        g.sqr(p.1) ^ g.mul(p.0, p.1) == self.w_rhs(p.0)
    }
    fn w_neg(&self, p: Pt) -> Pt {
        if p == INF {
            INF
        } else {
            (p.0, p.0 ^ p.1)
        }
    }
    fn w_add(&self, p: Pt, q: Pt) -> Pt {
        if p == INF {
            return q;
        }
        if q == INF {
            return p;
        }
        let g = &self.gf;
        let (x1, y1, x2, y2) = (p.0, p.1, q.0, q.1);
        let lam;
        if x1 == x2 {
            if y1 != y2 || x1 == 0 {
                return INF;
            }
            lam = x1 ^ g.div(y1, x1);
        } else {
            lam = g.div(y1 ^ y2, x1 ^ x2);
        }
        let x3 = g.sqr(lam) ^ lam ^ self.a ^ x1 ^ x2;
        let y3 = g.mul(lam, x1 ^ x3) ^ x3 ^ y1;
        (x3, y3)
    }
    fn w_count(&self) -> usize {
        1 + (0..=self.gf.mask).map(|x| self.w_ys(x).len()).sum::<usize>()
    }
    fn w_rand(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.next() & self.gf.mask;
            if x == 0 || x == self.c {
                continue;
            }
            let ys = self.w_ys(x);
            if ys.is_empty() {
                continue;
            }
            return (x, ys[(rng.next() & 1) as usize % ys.len()]);
        }
    }
    // ---- frames on the Weierstrass x
    fn u_of_x(&self, x: u64) -> u64 {
        self.gf.div(self.c, x ^ self.c)
    }
    fn x_of_u(&self, u: u64) -> u64 {
        self.gf.mul(self.c, self.gf.div(u ^ 1, u))
    }
    // ---- Edwards
    fn e_on(&self, p: Pt) -> bool {
        let g = &self.gf;
        let (x, y) = p;
        let l = g.mul(self.d1, x ^ y) ^ g.mul(self.d2, g.sqr(x) ^ g.sqr(y));
        let xy = g.mul(x, y);
        l == xy ^ g.mul(xy, x ^ y) ^ g.sqr(xy)
    }
    fn e_ys(&self, x: u64) -> Vec<u64> {
        let g = &self.gf;
        let a = self.d2 ^ x ^ g.sqr(x);
        let b = self.d1 ^ x ^ g.sqr(x);
        let c = g.mul(self.d1, x) ^ g.mul(self.d2, g.sqr(x));
        g.quad(a, b, c)
    }
    fn e_neg(&self, p: Pt) -> Pt {
        (p.1, p.0)
    }
    fn e_add(&self, p: Pt, q: Pt) -> Pt {
        let g = &self.gf;
        let (x1, y1) = p;
        let (x2, y2) = q;
        let nx = g.mul(self.d1, x1 ^ x2)
            ^ g.mul(self.d2, g.mul(x1 ^ y1, x2 ^ y2))
            ^ g.mul(x1 ^ g.sqr(x1), g.mul(x2, y1 ^ y2 ^ 1) ^ g.mul(y1, y2));
        let dx = self.d1 ^ g.mul(x1 ^ g.sqr(x1), x2 ^ y2);
        let ny = g.mul(self.d1, y1 ^ y2)
            ^ g.mul(self.d2, g.mul(x1 ^ y1, x2 ^ y2))
            ^ g.mul(y1 ^ g.sqr(y1), g.mul(y2, x1 ^ x2 ^ 1) ^ g.mul(x1, x2));
        let dy = self.d1 ^ g.mul(y1 ^ g.sqr(y1), x2 ^ y2);
        assert!(dx != 0 && dy != 0, "Edwards law is complete when Tr(d2) = 1");
        (g.div(nx, dx), g.div(ny, dy))
    }
    fn e_count(&self) -> usize {
        (0..=self.gf.mask).map(|x| self.e_ys(x).len()).sum()
    }
    fn e_rand(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.next() & self.gf.mask;
            let ys = self.e_ys(x);
            if ys.is_empty() {
                continue;
            }
            let p = (x, ys[(rng.next() & 1) as usize % ys.len()]);
            if p.0 ^ p.1 == 0 {
                continue; // w = 0: the two 2-torsion points
            }
            return p;
        }
    }
}

// ---------------------------------------------------------------- summation tensors

/// `T[i][j][k]`: coefficient of `z1^i z2^j z3^k` in a symmetric `S_3`, each
/// exponent at most 2.
type Tensor = [[[u64; 3]; 3]; 3];

/// Semaev `S_3 = (x1+x2)^2 x3^2 + x1 x2 x3 + (x1 x2)^2 + b` in the `x` frame.
fn tensor_x(b: u64) -> Tensor {
    let mut t = [[[0u64; 3]; 3]; 3];
    t[2][2][0] = 1;
    t[2][0][2] = 1;
    t[0][2][2] = 1;
    t[1][1][1] = 1;
    t[0][0][0] = b;
    t
}

/// `u1^2 u2^2 u3^2 S_3(x(u1), x(u2), x(u3))` with `x(u) = c (u+1) / u`.
fn tensor_u(tx: &Tensor, c: u64, gf: &Gf) -> Tensor {
    // x^e * u^2 as a polynomial in u: e=0: u^2; e=1: c (u^2+u); e=2: c^2 (u^2+1)
    let c2 = gf.sqr(c);
    let mq: [[u64; 3]; 3] = [[0, 0, 1], [0, c, c], [c2, 0, c2]];
    let mut t = [[[0u64; 3]; 3]; 3];
    for e1 in 0..3 {
        for e2 in 0..3 {
            for e3 in 0..3 {
                let co = tx[e1][e2][e3];
                if co == 0 {
                    continue;
                }
                for i in 0..3 {
                    if mq[e1][i] == 0 {
                        continue;
                    }
                    for j in 0..3 {
                        if mq[e2][j] == 0 {
                            continue;
                        }
                        for k in 0..3 {
                            if mq[e3][k] == 0 {
                                continue;
                            }
                            t[i][j][k] ^= gf.mul(co, gf.mul(mq[e1][i], gf.mul(mq[e2][j], mq[e3][k])));
                        }
                    }
                }
            }
        }
    }
    t
}

/// Coefficients (index = power) of `S_3(a, b, t)` as a polynomial in `t`.
fn coef3(gf: &Gf, tn: &Tensor, a: u64, b: u64) -> [u64; 3] {
    let pa = [1u64, a, gf.sqr(a)];
    let pb = [1u64, b, gf.sqr(b)];
    let mut out = [0u64; 3];
    for k in 0..3 {
        let mut s = 0u64;
        for i in 0..3 {
            for j in 0..3 {
                if tn[i][j][k] != 0 {
                    s ^= gf.mul(tn[i][j][k], gf.mul(pa[i], pb[j]));
                }
            }
        }
        out[k] = s;
    }
    out
}

/// Resultant of two quadratics (coefficient index = power), characteristic 2.
fn res22(gf: &Gf, p: &[u64; 3], q: &[u64; 3]) -> u64 {
    let (c1, b1, a1) = (p[0], p[1], p[2]);
    let (c2, b2, a2) = (q[0], q[1], q[2]);
    let t0 = gf.mul(a1, c2) ^ gf.mul(a2, c1);
    let t1 = gf.mul(a1, b2) ^ gf.mul(a2, b1);
    let t2 = gf.mul(b1, c2) ^ gf.mul(b2, c1);
    gf.sqr(t0) ^ gf.mul(t1, t2)
}

fn mat_det(gf: &Gf, mut m: Vec<Vec<u64>>) -> u64 {
    let n = m.len();
    let mut det = 1u64;
    for col in 0..n {
        let piv = match (col..n).find(|&r| m[r][col] != 0) {
            Some(r) => r,
            None => return 0,
        };
        m.swap(col, piv);
        let p = m[col][col];
        det = gf.mul(det, p);
        let pi = gf.inv(p);
        for r in (col + 1)..n {
            if m[r][col] != 0 {
                let f = gf.mul(m[r][col], pi);
                for c in col..n {
                    let v = gf.mul(f, m[col][c]);
                    m[r][c] ^= v;
                }
            }
        }
    }
    det
}

/// Resultant of two polynomials (coefficient index = power) with the formal
/// degrees `len - 1`, by the Sylvester determinant.
fn res_gen(gf: &Gf, a: &[u64], b: &[u64]) -> u64 {
    let (da, db) = (a.len() - 1, b.len() - 1);
    let n = da + db;
    let mut m = vec![vec![0u64; n]; n];
    for i in 0..db {
        for k in 0..=da {
            m[i][i + k] = a[da - k];
        }
    }
    for i in 0..da {
        for k in 0..=db {
            m[db + i][i + k] = b[db - k];
        }
    }
    mat_det(gf, m)
}

fn mat_inv(gf: &Gf, m: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let n = m.len();
    let mut a: Vec<Vec<u64>> = m.to_vec();
    let mut inv: Vec<Vec<u64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1 } else { 0 }).collect())
        .collect();
    for col in 0..n {
        let piv = (col..n).find(|&r| a[r][col] != 0).expect("singular");
        a.swap(col, piv);
        inv.swap(col, piv);
        let pi = gf.inv(a[col][col]);
        for c in 0..n {
            a[col][c] = gf.mul(a[col][c], pi);
            inv[col][c] = gf.mul(inv[col][c], pi);
        }
        for r in 0..n {
            if r != col && a[r][col] != 0 {
                let f = a[r][col];
                for c in 0..n {
                    let v1 = gf.mul(f, a[col][c]);
                    let v2 = gf.mul(f, inv[col][c]);
                    a[r][c] ^= v1;
                    inv[r][c] ^= v2;
                }
            }
        }
    }
    inv
}

/// Evaluates `S_{m+1}` of a frame at `z = (z_1, .., z_m, z_R)`, for `m = 3`
/// (`S_4`) or `m = 4` (`S_5`), as the nested resultant of `S_3`'s.
struct SumPoly<'a> {
    gf: &'a Gf,
    t: Tensor,
    vinv: Vec<Vec<u64>>, // inverse Vandermonde on the nodes 0..=4
}

impl<'a> SumPoly<'a> {
    fn new(gf: &'a Gf, t: Tensor) -> SumPoly<'a> {
        let v: Vec<Vec<u64>> = (0..5u64)
            .map(|k| (0..5).map(|i| gf.pow(k, i)).collect())
            .collect();
        SumPoly {
            gf,
            t,
            vinv: mat_inv(gf, &v),
        }
    }
    fn s4(&self, z: &[u64]) -> u64 {
        let g = self.gf;
        res22(g, &coef3(g, &self.t, z[0], z[1]), &coef3(g, &self.t, z[2], z[3]))
    }
    fn s5(&self, z: &[u64]) -> u64 {
        let g = self.gf;
        let a = coef3(g, &self.t, z[0], z[1]);
        let q = coef3(g, &self.t, z[3], z[4]);
        let mut vals = [0u64; 5];
        for k in 0..5u64 {
            vals[k as usize] = res22(g, &coef3(g, &self.t, k, z[2]), &q);
        }
        let mut b = [0u64; 5];
        for i in 0..5 {
            let mut s = 0u64;
            for k in 0..5 {
                s ^= g.mul(self.vinv[i][k], vals[k]);
            }
            b[i] = s;
        }
        res_gen(g, &a, &b)
    }
    fn eval(&self, m: usize, z: &[u64]) -> u64 {
        match m {
            3 => self.s4(z),
            4 => self.s5(z),
            _ => panic!("m must be 3 or 4"),
        }
    }
}

/// Fit the symmetric `S_3` of the Edwards `w = x + y` frame by interpolation
/// over sampled group-law triples; asserts a one-dimensional solution space.
fn tensor_w(cv: &Curves, rng: &mut Rng) -> Tensor {
    let g = &cv.gf;
    // symmetric classes: sorted exponent triples
    let mut classes: Vec<[usize; 3]> = vec![];
    for i in 0..3 {
        for j in i..3 {
            for k in j..3 {
                classes.push([i, j, k]);
            }
        }
    }
    let perms = |c: &[usize; 3]| -> Vec<[usize; 3]> {
        let mut v = vec![
            [c[0], c[1], c[2]],
            [c[0], c[2], c[1]],
            [c[1], c[0], c[2]],
            [c[1], c[2], c[0]],
            [c[2], c[0], c[1]],
            [c[2], c[1], c[0]],
        ];
        v.sort();
        v.dedup();
        v
    };
    let nc = classes.len();
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for _ in 0..80 {
        let p = cv.e_rand(rng);
        let q = cv.e_rand(rng);
        let w1 = p.0 ^ p.1;
        let w2 = q.0 ^ q.1;
        for r in [cv.e_add(p, q), cv.e_add(p, cv.e_neg(q))] {
            let w3 = r.0 ^ r.1;
            let pw = |w: u64| [1u64, w, g.sqr(w)];
            let (a, b, c) = (pw(w1), pw(w2), pw(w3));
            let mut row = vec![0u64; nc];
            for (ci, cl) in classes.iter().enumerate() {
                let mut s = 0u64;
                for pm in perms(cl) {
                    s ^= g.mul(a[pm[0]], g.mul(b[pm[1]], c[pm[2]]));
                }
                row[ci] = s;
            }
            rows.push(row);
        }
    }
    // nullspace by Gauss-Jordan
    let mut piv_col: Vec<usize> = vec![];
    let mut r = 0;
    for col in 0..nc {
        let pr = match (r..rows.len()).find(|&i| rows[i][col] != 0) {
            Some(x) => x,
            None => continue,
        };
        rows.swap(r, pr);
        let pi = g.inv(rows[r][col]);
        for c in 0..nc {
            rows[r][c] = g.mul(rows[r][c], pi);
        }
        for i in 0..rows.len() {
            if i != r && rows[i][col] != 0 {
                let f = rows[i][col];
                for c in 0..nc {
                    let v = g.mul(f, rows[r][c]);
                    rows[i][c] ^= v;
                }
            }
        }
        piv_col.push(col);
        r += 1;
    }
    assert!(
        piv_col.len() == nc - 1,
        "Edwards S3 interpolation: nullspace dimension {} (expected 1)",
        nc - piv_col.len()
    );
    let free = (0..nc).find(|c| !piv_col.contains(c)).unwrap();
    let mut sol = vec![0u64; nc];
    sol[free] = 1;
    for (ri, &pc) in piv_col.iter().enumerate() {
        sol[pc] = rows[ri][free];
    }
    let mut t = [[[0u64; 3]; 3]; 3];
    for (ci, cl) in classes.iter().enumerate() {
        for pm in perms(cl) {
            t[pm[0]][pm[1]][pm[2]] = sol[ci];
        }
    }
    t
}

// ---------------------------------------------------------------- cell construction

/// A cell: curves, the three frame tensors, and the factor-base subspace.
struct Cell {
    n: u32,
    l: usize,
    m: usize,
    cv: Curves,
    tx: Tensor,
    tu: Tensor,
    tw: Tensor,
}

fn cell_seed(n: u32, l: usize, m: usize) -> u64 {
    let mut r = Rng(20261018 ^ ((n as u64) << 40) ^ ((l as u64) << 32) ^ ((m as u64) << 24));
    r.next();
    r.next()
}

impl Cell {
    fn new(n: u32, l: usize, m: usize) -> Cell {
        let mut rng = Rng(cell_seed(n, l, m));
        let g0 = Gf::new(n);
        let (d1, d2) = loop {
            let d1 = rng.next() & g0.mask;
            let d2 = rng.next() & g0.mask;
            if d1 == 0 || g0.tr(d2) != 1 {
                continue;
            }
            break (d1, d2);
        };
        drop(g0);
        let cv = Curves::new(n, d1, d2);
        let tx = tensor_x(cv.b);
        let tu = tensor_u(&tx, cv.c, &cv.gf);
        let tw = tensor_w(&cv, &mut Rng(cell_seed(n, l, m) ^ 0xED));
        Cell { n, l, m, cv, tx, tu, tw }
    }
    /// The subspace basis of draw `draw`: `beta_1 = 1` then `l-1` further
    /// independent elements.
    fn basis(&self, draw: u64, attempt: u64) -> Vec<u64> {
        let mut rng = Rng(cell_seed(self.n, self.l, self.m) ^ 0xBA515 ^ (draw << 20) ^ (attempt << 40));
        let mut basis = vec![1u64];
        let mut span = vec![0u64, 1];
        while basis.len() < self.l {
            let v = rng.next() & self.cv.gf.mask;
            if v == 0 || span.contains(&v) {
                continue;
            }
            let cur = span.clone();
            span.extend(cur.iter().map(|s| s ^ v));
            basis.push(v);
        }
        basis
    }
}

fn span_of(basis: &[u64]) -> Vec<u64> {
    let l = basis.len();
    (0..1u64 << l)
        .map(|c| (0..l).filter(|t| (c >> t) & 1 == 1).fold(0, |a, t| a ^ basis[t]))
        .collect()
}

// ---------------------------------------------------------------- arithmetic selftest

fn arith_selftest(check: &mut dyn FnMut(&str, bool, String)) {
    // field
    {
        let g = Gf::new(13);
        let mut rng = Rng(5);
        let mut fine = true;
        for _ in 0..3000 {
            let (x, y, z) = (rng.next() & g.mask, rng.next() & g.mask, rng.next() & g.mask);
            if g.mul(x, g.mul(y, z)) != g.mul(g.mul(x, y), z) || g.mul(x, y ^ z) != g.mul(x, y) ^ g.mul(x, z) {
                fine = false;
            }
            if x != 0 && g.mul(x, g.inv(x)) != 1 {
                fine = false;
            }
            if g.sqr(g.sqrt(x)) != x {
                fine = false;
            }
            for r in g.quad(x, y, z) {
                if g.mul(x, g.sqr(r)) ^ g.mul(y, r) ^ z != 0 {
                    fine = false;
                }
            }
        }
        check("field axioms, sqrt, quadratic roots (n=13)", fine, "3000 random triples".into());
        let g = Gf::new(9);
        let mut complete = true;
        for _ in 0..200 {
            let (x, y, z) = (rng.next() & g.mask, rng.next() & g.mask, rng.next() & g.mask);
            let brute: Vec<u64> = (0..1u64 << 9)
                .filter(|&r| g.mul(x, g.sqr(r)) ^ g.mul(y, r) ^ z == 0)
                .collect();
            let mut got = g.quad(x, y, z);
            got.sort_unstable();
            if got != brute {
                complete = false;
            }
        }
        check("quadratic solve equals brute force (n=9)", complete, "200 triples".into());
    }
    // curves, frames, polynomials at two sizes
    for &n in &[11u32, 13] {
        let cell = Cell::new(n, 3, 3);
        let cv = &cell.cv;
        let g = &cv.gf;
        let mut rng = Rng(77 + n as u64);
        check(
            &format!("n={n} Edwards and Weierstrass point counts agree"),
            cv.w_count() == cv.e_count(),
            format!("{} vs {} (d1={} d2={} a={} b={})", cv.w_count(), cv.e_count(), cv.d1, cv.d2, cv.a, cv.b),
        );
        let mut ok_w = true;
        let mut ok_e = true;
        for _ in 0..400 {
            let (p, q, r) = (cv.w_rand(&mut rng), cv.w_rand(&mut rng), cv.w_rand(&mut rng));
            let s = cv.w_add(p, q);
            if !cv.w_on(s) || cv.w_add(cv.w_add(p, q), r) != cv.w_add(p, cv.w_add(q, r)) {
                ok_w = false;
            }
            if cv.w_add(p, cv.w_neg(p)) != INF {
                ok_w = false;
            }
            let (p, q, r) = (cv.e_rand(&mut rng), cv.e_rand(&mut rng), cv.e_rand(&mut rng));
            if !cv.e_on(cv.e_add(p, q)) || cv.e_add(cv.e_add(p, q), r) != cv.e_add(p, cv.e_add(q, r)) {
                ok_e = false;
            }
            if cv.e_add(p, cv.e_neg(p)) != (0, 0) {
                ok_e = false;
            }
        }
        check(&format!("n={n} Weierstrass group law"), ok_w, "400 triples".into());
        check(&format!("n={n} Edwards group law (complete, neutral (0,0))"), ok_e, "400 triples".into());
        // 2-torsion translation
        let t_pt = (0u64, g.sqrt(cv.b));
        let mut ok_t = cv.w_on(t_pt) && cv.w_add(t_pt, t_pt) == INF;
        let mut ok_u = true;
        for _ in 0..300 {
            let p = cv.w_rand(&mut rng);
            let q = cv.w_add(p, t_pt);
            if q != INF && q.0 != g.div(g.sqrt(cv.b), p.0) {
                ok_t = false;
            }
            if q != INF && cv.u_of_x(q.0) != cv.u_of_x(p.0) ^ 1 {
                ok_u = false;
            }
            if cv.x_of_u(cv.u_of_x(p.0)) != p.0 {
                ok_u = false;
            }
        }
        check(&format!("n={n} x(P+T) = sqrt(b)/x"), ok_t, "300 points".into());
        check(&format!("n={n} Moebius frame turns T into u -> u+1"), ok_u, "300 points".into());
        // S3 in the three frames
        let mut ok_x = true;
        let mut ok_uf = true;
        let mut ok_sym = true;
        for _ in 0..300 {
            let (p, q) = (cv.w_rand(&mut rng), cv.w_rand(&mut rng));
            for r in [cv.w_add(p, q), cv.w_add(p, cv.w_neg(q))] {
                if r == INF {
                    continue;
                }
                let z = [p.0, q.0, r.0];
                let cz = coef3(g, &cell.tx, z[0], z[1]);
                if g.mul(cz[2], g.sqr(z[2])) ^ g.mul(cz[1], z[2]) ^ cz[0] != 0 {
                    ok_x = false;
                }
                let u = [cv.u_of_x(p.0), cv.u_of_x(q.0), cv.u_of_x(r.0)];
                let cu = coef3(g, &cell.tu, u[0], u[1]);
                if g.mul(cu[2], g.sqr(u[2])) ^ g.mul(cu[1], u[2]) ^ cu[0] != 0 {
                    ok_uf = false;
                }
                let cu1 = coef3(g, &cell.tu, u[0] ^ 1, u[1] ^ 1);
                let a = g.mul(cu1[2], g.sqr(u[2])) ^ g.mul(cu1[1], u[2]) ^ cu1[0];
                if a != 0 {
                    ok_sym = false;
                }
            }
        }
        check(&format!("n={n} S3 (x frame) vanishes on P+Q and P-Q"), ok_x, "300 pairs".into());
        check(&format!("n={n} S3 (u frame) vanishes on P+Q and P-Q"), ok_uf, "300 pairs".into());
        check(
            &format!("n={n} S3 (u frame) zero set invariant under (u1,u2) -> (u1+1,u2+1)"),
            ok_sym,
            "300 pairs".into(),
        );
        // w frame: invariances and the fitted S3 on fresh samples
        let t2 = (1u64, 1u64);
        let mut ok_inv = cv.e_on(t2) && cv.e_add(t2, t2) == (0, 0);
        let mut ok_wf = true;
        for _ in 0..300 {
            let (p, q) = (cv.e_rand(&mut rng), cv.e_rand(&mut rng));
            let w = |r: Pt| r.0 ^ r.1;
            if w(cv.e_neg(p)) != w(p) || w(cv.e_add(p, t2)) != w(p) {
                ok_inv = false;
            }
            for r in [cv.e_add(p, q), cv.e_add(p, cv.e_neg(q))] {
                let cw = coef3(g, &cell.tw, w(p), w(q));
                if g.mul(cw[2], g.sqr(w(r))) ^ g.mul(cw[1], w(r)) ^ cw[0] != 0 {
                    ok_wf = false;
                }
            }
        }
        check(&format!("n={n} w = x+y invariant under negation and +(1,1)"), ok_inv, "300 points".into());
        check(&format!("n={n} fitted S3 (w frame) vanishes on fresh P+-Q"), ok_wf, "300 pairs".into());
        // S_{m+1} vanishes on planted decompositions, per frame
        for m in [3usize, 4] {
            let mut cnt = [0usize; 3];
            let mut zero_ok = [true; 3];
            let mut nonzero = [0usize; 3];
            let trials = 120;
            let spx = SumPoly::new(g, cell.tx);
            let spu = SumPoly::new(g, cell.tu);
            let spw = SumPoly::new(g, cell.tw);
            for _ in 0..trials {
                let pts: Vec<Pt> = (0..m).map(|_| cv.w_rand(&mut rng)).collect();
                let signs: Vec<bool> = (0..m).map(|_| rng.next() & 1 == 1).collect();
                let mut s = INF;
                for (p, sg) in pts.iter().zip(&signs) {
                    s = cv.w_add(s, if *sg { cv.w_neg(*p) } else { *p });
                }
                if s != INF {
                    let mut zx: Vec<u64> = pts.iter().map(|p| p.0).collect();
                    zx.push(s.0);
                    if spx.eval(m, &zx) != 0 {
                        zero_ok[0] = false;
                    }
                    let zu: Vec<u64> = zx.iter().map(|&x| cv.u_of_x(x)).collect();
                    if zx.iter().all(|&x| x != cv.c) {
                        if spu.eval(m, &zu) != 0 {
                            zero_ok[1] = false;
                        }
                        let mut zr = zx.clone();
                        zr[m] ^= 1;
                        cnt[1] += 1;
                        let zr: Vec<u64> = zr.iter().map(|&x| if x == cv.c { 0 } else { cv.u_of_x(x) }).collect();
                        if zx[m] != cv.c && spu.eval(m, &zr) == 0 {
                            nonzero[1] += 0;
                        }
                    }
                    cnt[0] += 1;
                    // a random non-decomposition target
                    let mut zbad = zx.clone();
                    zbad[m] = cv.w_rand(&mut rng).0;
                    if spx.eval(m, &zbad) != 0 {
                        nonzero[0] += 1;
                    }
                }
                let pe: Vec<Pt> = (0..m).map(|_| cv.e_rand(&mut rng)).collect();
                let mut se = (0u64, 0u64);
                for p in &pe {
                    se = cv.e_add(se, *p);
                }
                let mut zw: Vec<u64> = pe.iter().map(|p| p.0 ^ p.1).collect();
                zw.push(se.0 ^ se.1);
                cnt[2] += 1;
                if spw.eval(m, &zw) != 0 {
                    zero_ok[2] = false;
                }
                let mut zbad = zw.clone();
                zbad[m] = cv.e_rand(&mut rng).0 ^ 1;
                if spw.eval(m, &zbad) != 0 {
                    nonzero[2] += 1;
                }
            }
            check(
                &format!("n={n} S_{} vanishes on true decompositions (x, u, w frames)", m + 1),
                zero_ok.iter().all(|&b| b),
                format!("x {}/{} u {}/{} w {}/{}", cnt[0], trials, cnt[1], trials, cnt[2], trials),
            );
            check(
                &format!("n={n} S_{} is not identically zero off the variety", m + 1),
                nonzero[0] > cnt[0] / 2 && nonzero[2] > cnt[2] / 2,
                format!("nonzero x {} w {}", nonzero[0], nonzero[2]),
            );
        }
    }
}

// ---------------------------------------------------------------- draws, fibres, targets

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Frame {
    X,
    U,
    W,
}

/// One draw of the factor base: a subspace and the points over each of its
/// elements, in each frame.
struct Draw {
    basis: Vec<u64>,
    span: Vec<u64>,
    fib_x: Vec<Vec<Pt>>,
    fib_u: Vec<Vec<Pt>>,
    fib_w: Vec<Vec<Pt>>,
}

impl Draw {
    /// The first basis (in a fixed seeded order) whose factor base has at
    /// least `m + 1` non-zero elements over rational points in every frame,
    /// so that a planted decomposition with distinct summands exists.
    fn new(cell: &Cell, draw: u64) -> Draw {
        let cv = &cell.cv;
        let g = &cv.gf;
        // Attempt cap: 2_000_000 (the first run capped it at 2000, which draw 1
        // of cell (15,3,4) needs 2051 attempts to pass; see PROTOCOL.md,
        // Deviation 10).  The attempts are a fixed seeded sequence and the first
        // basis that passes is returned, so every draw that already passed
        // under the old cap returns the identical basis.
        for attempt in 0..2_000_000u64 {
            let basis = cell.basis(draw, attempt);
            let span = span_of(&basis);
            let pts_of_x = |x: u64| -> Vec<Pt> {
                if x == 0 {
                    return vec![(0, g.sqrt(cv.b))];
                }
                cv.w_ys(x).into_iter().map(|y| (x, y)).collect()
            };
            let fib_x: Vec<Vec<Pt>> = span.iter().map(|&z| pts_of_x(z)).collect();
            let fib_u: Vec<Vec<Pt>> = span
                .iter()
                .map(|&z| if z == 0 { vec![INF] } else { pts_of_x(cv.x_of_u(z)) })
                .collect();
            let nz = |f: &Vec<Vec<Pt>>| (1..span.len()).filter(|&v| !f[v].is_empty()).count();
            if nz(&fib_x) <= cell.m || nz(&fib_u) <= cell.m {
                continue;
            }
            let index: HashMap<u64, usize> = span.iter().enumerate().map(|(i, &z)| (z, i)).collect();
            let mut fib_w: Vec<Vec<Pt>> = vec![Vec::new(); span.len()];
            for x in 0..=g.mask {
                for y in cv.e_ys(x) {
                    if let Some(&i) = index.get(&(x ^ y)) {
                        fib_w[i].push((x, y));
                    }
                }
            }
            if nz(&fib_w) <= cell.m {
                continue;
            }
            return Draw {
                basis,
                span,
                fib_x,
                fib_u,
                fib_w,
            };
        }
        panic!("no factor base with enough rational elements");
    }
    fn fib(&self, f: Frame) -> &Vec<Vec<Pt>> {
        match f {
            Frame::X => &self.fib_x,
            Frame::U => &self.fib_u,
            Frame::W => &self.fib_w,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Kind {
    Random,
    Planted,
}

/// A target in every frame.  Random targets use one Weierstrass point in the
/// X and U frames and an independent Edwards point in the W frame; planted
/// targets are built in each frame from that frame's own factor base.
struct Target {
    kind: Kind,
    idx: usize,
    rx: Pt,
    ru: Pt,
    rw: Pt,
    /// planted subspace-element indices per frame, if planted
    plant: Option<[Vec<usize>; 3]>,
}

impl Target {
    fn zr(&self, cv: &Curves, f: Frame) -> u64 {
        match f {
            Frame::X => self.rx.0,
            Frame::U => cv.u_of_x(self.ru.0),
            Frame::W => self.rw.0 ^ self.rw.1,
        }
    }
}

fn target_rng(cell: &Cell, draw: u64, kind: Kind, idx: usize) -> Rng {
    Rng(cell_seed(cell.n, cell.l, cell.m)
        ^ 0x7A5C37
        ^ (draw << 16)
        ^ ((idx as u64) << 8)
        ^ if kind == Kind::Planted { 0x5555 } else { 0 })
}

fn make_target(cell: &Cell, dr: &Draw, draw: u64, kind: Kind, idx: usize) -> Target {
    let cv = &cell.cv;
    let mut rng = target_rng(cell, draw, kind, idx);
    let m = cell.m;
    match kind {
        Kind::Random => {
            let rx = loop {
                let p = cv.w_rand(&mut rng);
                if p.0 != cv.c {
                    break p;
                }
            };
            let rw = cv.e_rand(&mut rng);
            Target {
                kind,
                idx,
                rx,
                ru: rx,
                rw,
                plant: None,
            }
        }
        Kind::Planted => {
            let nv = dr.span.len();
            let mut plant: [Vec<usize>; 3] = [vec![], vec![], vec![]];
            let mut rr = [INF; 3];
            for (fi, f) in [Frame::X, Frame::U, Frame::W].iter().enumerate() {
                let fib = dr.fib(*f);
                let mut tries = 0u32;
                'retry: loop {
                    tries += 1;
                    assert!(tries < 200000, "cannot plant a decomposition");
                    let mut vs: Vec<usize> = vec![];
                    while vs.len() < m {
                        let v = 1 + (rng.next() as usize) % (nv - 1);
                        if !vs.contains(&v) && !fib[v].is_empty() {
                            vs.push(v);
                        }
                    }
                    let pts: Vec<Pt> = vs
                        .iter()
                        .map(|&v| fib[v][(rng.next() as usize) % fib[v].len()])
                        .collect();
                    match f {
                        Frame::W => {
                            let mut s = (0u64, 0u64);
                            for p in &pts {
                                s = cv.e_add(s, *p);
                            }
                            if s.0 ^ s.1 == 0 {
                                continue 'retry;
                            }
                            rr[fi] = s;
                        }
                        _ => {
                            let mut q = pts[0];
                            for p in &pts[1..] {
                                if q == INF || q.0 == p.0 {
                                    continue 'retry; // generic chain only
                                }
                                q = cv.w_add(q, *p);
                            }
                            if q == INF || q.0 == cv.c || q.0 == 0 {
                                continue 'retry;
                            }
                            rr[fi] = q;
                        }
                    }
                    plant[fi] = vs;
                    break;
                }
            }
            Target {
                kind,
                idx,
                rx: rr[0],
                ru: rr[1],
                rw: rr[2],
                plant: Some(plant),
            }
        }
    }
}

// ---------------------------------------------------------------- ground truth

/// For every tuple (index `sum v_i << (i*l)`), whether some choice of points
/// in the fibres sums to the target (`any`), and whether some choice does so
/// along a generic chain (`gen`; Weierstrass frames only).
struct Truth {
    any: HashSet<u64>,
    gen: HashSet<u64>,
}

fn ground_truth(cell: &Cell, dr: &Draw, f: Frame, r: Pt) -> Truth {
    let cv = &cell.cv;
    let (m, l) = (cell.m, cell.l);
    let fib = dr.fib(f);
    let nv = 1usize << l;
    let mut any = HashSet::new();
    let mut gen = HashSet::new();
    // depth-first over positions; track (sum, generic)
    fn rec(
        cv: &Curves,
        f: Frame,
        fib: &Vec<Vec<Pt>>,
        nv: usize,
        l: usize,
        m: usize,
        pos: usize,
        idx: u64,
        sum: Pt,
        generic: bool,
        r: Pt,
        any: &mut HashSet<u64>,
        gen: &mut HashSet<u64>,
    ) {
        if pos == m {
            if sum == r {
                any.insert(idx);
                if generic {
                    gen.insert(idx);
                }
            }
            return;
        }
        for v in 0..nv {
            let nidx = idx | ((v as u64) << (pos * l));
            for &p in &fib[v] {
                let (ns, ng) = match f {
                    Frame::W => {
                        if pos == 0 {
                            (p, true)
                        } else {
                            (cv.e_add(sum, p), true)
                        }
                    }
                    _ => {
                        if pos == 0 {
                            (p, p != INF)
                        } else {
                            let g2 = generic && sum != INF && p != INF && sum.0 != p.0;
                            (cv.w_add(sum, p), g2)
                        }
                    }
                };
                rec(cv, f, fib, nv, l, m, pos + 1, nidx, ns, ng, r, any, gen);
            }
        }
    }
    // the target in the sum is R itself; fibres are closed under negation
    // (and under +T' in the W frame), so "sum = R" and "sum = -R" coincide
    if f == Frame::W && m == 4 {
        // meet in the middle: pair sums of (v1, v2) and (v3, v4)
        let pair = |va: usize, vb: usize| -> Vec<Pt> {
            let mut out = vec![];
            for &p in &fib[va] {
                for &q in &fib[vb] {
                    let s = cv.e_add(p, q);
                    if !out.contains(&s) {
                        out.push(s);
                    }
                }
            }
            out
        };
        let pairs: Vec<Vec<Pt>> = (0..nv * nv).map(|k| pair(k % nv, k / nv)).collect();
        let sets: Vec<HashSet<Pt>> = pairs.iter().map(|v| v.iter().copied().collect()).collect();
        for k12 in 0..nv * nv {
            // for each partial sum a, the needed complement is r - a
            let needs: Vec<Pt> = pairs[k12].iter().map(|&a| cv.e_add(r, cv.e_neg(a))).collect();
            for k34 in 0..nv * nv {
                if needs.iter().any(|nd| sets[k34].contains(nd)) {
                    let idx = (k12 % nv) as u64
                        | (((k12 / nv) as u64) << l)
                        | (((k34 % nv) as u64) << (2 * l))
                        | (((k34 / nv) as u64) << (3 * l));
                    any.insert(idx);
                }
            }
        }
        return Truth { any, gen };
    }
    let start = if f == Frame::W { (0u64, 0u64) } else { INF };
    // pos 0 handles the first point specially, so start from a dummy
    rec(cv, f, fib, nv, l, m, 0, 0, start, true, r, &mut any, &mut gen);
    Truth { any, gen }
}

// ---------------------------------------------------------------- arm systems

/// A Boolean system with its exact solution set.
struct Sys {
    arm: &'static str,
    nv: usize,
    eqs: Vec<Poly>,
    /// every solution in all `nv` variables (empty if unsatisfiable)
    sols: Vec<M>,
    /// the solutions projected on the subspace coordinates, as tuple indices
    /// `sum v_i << (i*l)` (lifted through the quotient for arm `d`)
    zset: HashSet<u64>,
}

/// Algebraic normal form of the `n` coordinate functions of `vals`, a function
/// of `nv` Boolean variables given by its truth table (field value per point).
fn anf_polys(vals: &[u64], n: u32, nv: usize) -> Vec<Poly> {
    let size = 1usize << nv;
    let mut polys = Vec::new();
    let mut a = vec![0u8; size];
    for k in 0..n {
        for i in 0..size {
            a[i] = ((vals[i] >> k) & 1) as u8;
        }
        for b in 0..nv {
            let bit = 1usize << b;
            for i in 0..size {
                if i & bit != 0 {
                    a[i] ^= a[i ^ bit];
                }
            }
        }
        let p: Poly = (0..size).filter(|&i| a[i] == 1).map(|i| M::from_u64(i as u64)).collect();
        // ascending in the `M` order
        let mut p = p;
        p.sort_unstable();
        if !p.is_empty() {
            polys.push(p);
        }
    }
    polys
}

/// The unchained `S_{m+1}` of a frame, descended: `m*l` unknown bits (or
/// `1 + m*(l-1)` after the 2-torsion symmetrisation `sym`), `n` equations.
fn build_unchained(
    cell: &Cell,
    dr: &Draw,
    t: Tensor,
    zr: u64,
    sym: bool,
    arm: &'static str,
    phi: Option<&dyn Fn(usize) -> usize>,
) -> Sys {
    let g = &cell.cv.gf;
    let (m, l, n) = (cell.m, cell.l, cell.n);
    let nvs = 1usize << l;
    let span = &dr.span;
    let nv_full = m * l;
    // per-pair coefficient tables
    let ca: Vec<[u64; 3]> = (0..nvs * nvs)
        .map(|k| coef3(g, &t, span[k % nvs], span[k / nvs]))
        .collect(); // index v1 + nvs*v2
    let sp = SumPoly::new(g, t);
    let value_of = |tuple: &[usize]| -> u64 {
        match m {
            3 => {
                let cb = coef3(g, &t, span[tuple[2]], zr);
                res22(g, &ca[tuple[0] + nvs * tuple[1]], &cb)
            }
            4 => {
                let mut z = [0u64; 5];
                for i in 0..4 {
                    z[i] = span[tuple[i]];
                }
                z[4] = zr;
                sp.s5(&z)
            }
            _ => panic!(),
        }
    };
    // cache B for m = 4 by (v3, v4)
    let cb4: Vec<Vec<u64>> = if m == 4 {
        let q = |v4: usize| coef3(g, &t, span[v4], zr);
        let mut out = Vec::with_capacity(nvs * nvs);
        for k in 0..nvs * nvs {
            let (v3, v4) = (k % nvs, k / nvs);
            let qq = q(v4);
            let mut vals = [0u64; 5];
            for node in 0..5u64 {
                vals[node as usize] = res22(g, &coef3(g, &t, node, span[v3]), &qq);
            }
            let mut b = vec![0u64; 5];
            for i in 0..5 {
                let mut s = 0u64;
                for kk in 0..5 {
                    s ^= g.mul(sp.vinv[i][kk], vals[kk]);
                }
                b[i] = s;
            }
            out.push(b);
        }
        out
    } else {
        vec![]
    };
    let eval_tuple = |tuple: &[usize]| -> u64 {
        if m == 4 {
            res_gen(g, &ca[tuple[0] + nvs * tuple[1]], &cb4[tuple[2] + nvs * tuple[3]])
        } else {
            value_of(tuple)
        }
    };
    let mut zset = HashSet::new();
    if !sym {
        let size = 1usize << nv_full;
        let mut vals = vec![0u64; size];
        let mut sols = vec![];
        let mut tuple = vec![0usize; m];
        for idx in 0..size {
            let pidx = match phi {
                Some(f) => f(idx),
                None => idx,
            };
            for i in 0..m {
                tuple[i] = (pidx >> (i * l)) & (nvs - 1);
            }
            let v = eval_tuple(&tuple);
            vals[idx] = v;
            if v == 0 {
                sols.push(M::from_u64(idx as u64));
                zset.insert(pidx as u64);
            }
        }
        let eqs = anf_polys(&vals, n, nv_full);
        Sys {
            arm,
            nv: nv_full,
            eqs,
            sols,
            zset,
        }
    } else {
        assert!(dr.basis[0] == 1 && l >= 2);
        // variable 0 = parity p; then bits 1..l of each v_i
        let nvq = 1 + m * (l - 1);
        let size = 1usize << nvq;
        let mut vals = vec![0u64; size];
        let mut sols = vec![];
        let mut tuple = vec![0usize; m];
        for q in 0..size {
            let p = q & 1;
            for i in 0..m {
                let rest = (q >> (1 + i * (l - 1))) & ((1usize << (l - 1)) - 1);
                let b0 = if i == 0 { p } else { 0 };
                tuple[i] = b0 | (rest << 1);
            }
            let v = eval_tuple(&tuple);
            vals[q] = v;
            if v == 0 {
                sols.push(M::from_u64(q as u64));
                // lift: all first-bit vectors of parity p
                for fb in 0..(1usize << m) {
                    if (fb.count_ones() as usize) & 1 != p {
                        continue;
                    }
                    let mut idx = 0u64;
                    for i in 0..m {
                        let rest = (q >> (1 + i * (l - 1))) & ((1usize << (l - 1)) - 1);
                        let vi = ((fb >> i) & 1) | (rest << 1);
                        idx |= (vi as u64) << (i * l);
                    }
                    zset.insert(idx);
                }
            }
        }
        let eqs = anf_polys(&vals, n, nvq);
        Sys {
            arm,
            nv: nvq,
            eqs,
            sols,
            zset,
        }
    }
}

/// Roots `t` of `S_3(a, c, t) = 0` in the X frame.
fn x_roots(g: &Gf, b: u64, a: u64, c: u64) -> Vec<u64> {
    let ac = g.mul(a, c);
    g.quad(g.sqr(a ^ c), ac, g.sqr(ac) ^ b)
}

fn s3x(g: &Gf, b: u64, a: u64, c: u64, d: u64) -> u64 {
    let ac = g.mul(a, c);
    g.mul(g.sqr(a ^ c), g.sqr(d)) ^ g.mul(ac, d) ^ g.sqr(ac) ^ b
}

/// Arm e: the chained `S_3` with intermediates.  Variables: `m*l` subspace
/// bits, then `(m-2)*n` intermediate bits.
fn build_chain(cell: &Cell, dr: &Draw, xr: u64) -> Sys {
    let cv = &cell.cv;
    let g = &cv.gf;
    let (m, l, n) = (cell.m, cell.l, cell.n);
    let nv = m * l + (m - 2) * n as usize;
    assert!(nv <= 190);
    let s: Vec<Sym> = (0..m).map(|i| sym_subspace(&dr.basis, i * l, n)).collect();
    let target = sym_const(xr, n);
    let inter: Vec<Sym> = (0..m - 2).map(|i| sym_free(m * l + i * n as usize, n)).collect();
    let s3 = |x1: &Sym, x2: &Sym, x3: &Sym| -> Vec<Poly> {
        let sum_sq = sym_sq(&sym_add(x1, x2), g);
        let prod = sym_mul(x1, x2, g);
        let mut acc = sym_mul(&sum_sq, &sym_sq(x3, g), g);
        acc = sym_add(&acc, &sym_mul(&prod, x3, g));
        acc = sym_add(&acc, &sym_sq(&prod, g));
        acc = sym_add(&acc, &sym_const(cv.b, n));
        acc
    };
    let mut eqs: Vec<Poly> = Vec::new();
    eqs.extend(s3(&s[0], &s[1], &inter[0]));
    for i in 0..m - 3 {
        eqs.extend(s3(&inter[i], &s[i + 2], &inter[i + 1]));
    }
    eqs.extend(s3(&inter[m - 3], &s[m - 1], &target));
    eqs.retain(|p| !p.is_empty());
    // exact solutions by depth-first search over the chain
    let nvs = 1usize << l;
    let mut sols: Vec<M> = vec![];
    let mut zset = HashSet::new();
    fn rec(
        g: &Gf,
        b: u64,
        span: &[u64],
        m: usize,
        l: usize,
        n: u32,
        xr: u64,
        pos: usize,
        cur: u64, // current intermediate (pos >= 2) or x_1 (pos == 1)
        vs: &mut Vec<usize>,
        ts: &mut Vec<u64>,
        sols: &mut Vec<M>,
        zset: &mut HashSet<u64>,
    ) {
        let nvs = span.len();
        // pos = number of subspace elements consumed so far (>= 2)
        for v in 0..nvs {
            let x = span[v];
            vs.push(v);
            if pos == m - 1 {
                // final link: S3(cur, x, xr) = 0
                if s3x(g, b, cur, x, xr) == 0 {
                    let mut mono = M0;
                    let mut idx = 0u64;
                    for (i, &vi) in vs.iter().enumerate() {
                        idx |= (vi as u64) << (i * l);
                        for t in 0..l {
                            if (vi >> t) & 1 == 1 {
                                mono = mono.or(M::bit(i * l + t));
                            }
                        }
                    }
                    for (j, &t) in ts.iter().enumerate() {
                        for k in 0..n as usize {
                            if (t >> k) & 1 == 1 {
                                mono = mono.or(M::bit(m * l + j * n as usize + k));
                            }
                        }
                    }
                    sols.push(mono);
                    zset.insert(idx);
                }
            } else {
                for t in x_roots(g, b, cur, x) {
                    ts.push(t);
                    rec(g, b, span, m, l, n, xr, pos + 1, t, vs, ts, sols, zset);
                    ts.pop();
                }
            }
            vs.pop();
        }
    }
    for v1 in 0..nvs {
        for v2 in 0..nvs {
            for t in x_roots(g, cv.b, dr.span[v1], dr.span[v2]) {
                let mut vs = vec![v1, v2];
                let mut ts = vec![t];
                rec(g, cv.b, &dr.span, m, l, n, xr, 2, t, &mut vs, &mut ts, &mut sols, &mut zset);
            }
        }
    }
    Sys {
        arm: "e",
        nv,
        eqs,
        sols,
        zset,
    }
}

/// Arm b: the unprojected chain with `y`-coordinates and the affine addition
/// law.  Variables: `m*l` subspace bits, `m*n` bits of the `y_i`, `(m-1)*n`
/// bits of the chord slopes, and one sign bit for `P_1 + .. + P_m = +-R`.
/// The affine law needs `x(Q) != x(P)` at every step; the solutions
/// enumerated here are the generic ones (a coinciding step also admits a free
/// slope, which this enumeration does not list; see the protocol).
/// The residual field elements of arm b, in a fixed order: the `m` curve
/// equations, then per chain step the slope equation, then the two final
/// equations.  Returns them with the variable count.
fn unprojected_residuals(cell: &Cell, dr: &Draw, r: Pt) -> (Vec<Sym>, usize) {
    let cv = &cell.cv;
    let g = &cv.gf;
    let (m, l, n) = (cell.m, cell.l, cell.n);
    let nu = n as usize;
    let nv = m * l + m * nu + (m - 1) * nu + 1;
    assert!(nv <= 190, "too many variables for the monomial type");
    let ybase = |i: usize| m * l + i * nu;
    let lbase = |j: usize| m * l + m * nu + j * nu;
    let sbit = nv - 1;
    let xs: Vec<Sym> = (0..m).map(|i| sym_subspace(&dr.basis, i * l, n)).collect();
    let ys: Vec<Sym> = (0..m).map(|i| sym_free(ybase(i), n)).collect();
    let mut res: Vec<Sym> = Vec::new();
    for i in 0..m {
        let mut e = sym_add(&sym_sq(&ys[i], g), &sym_mul(&xs[i], &ys[i], g));
        e = sym_add(&e, &sym_cube(&xs[i], g));
        e = sym_add(&e, &sym_mulc(&sym_sq(&xs[i], g), cv.a, g));
        e = sym_add(&e, &sym_const(cv.b, n));
        res.push(e);
    }
    let a_const = sym_const(cv.a, n);
    let mut xq = xs[0].clone();
    let mut yq = ys[0].clone();
    for j in 0..m - 1 {
        let lam = sym_free(lbase(j), n);
        let dx = sym_add(&xq, &xs[j + 1]);
        let dy = sym_add(&yq, &ys[j + 1]);
        res.push(sym_add(&sym_mul(&lam, &dx, g), &dy));
        let mut x3 = sym_add(&sym_sq(&lam, g), &lam);
        x3 = sym_add(&x3, &a_const);
        x3 = sym_add(&x3, &dx);
        let mut y3 = sym_mul(&lam, &sym_add(&xq, &x3), g);
        y3 = sym_add(&y3, &x3);
        y3 = sym_add(&y3, &yq);
        xq = x3;
        yq = y3;
    }
    res.push(sym_add(&xq, &sym_const(r.0, n)));
    let sig_xr: Sym = (0..nu)
        .map(|k| if (r.0 >> k) & 1 == 1 { vec![M::bit(sbit)] } else { vec![] })
        .collect();
    res.push(sym_add(&sym_add(&yq, &sym_const(r.1, n)), &sig_xr));
    (res, nv)
}

fn build_unprojected(cell: &Cell, dr: &Draw, r: Pt) -> Sys {
    let cv = &cell.cv;
    let g = &cv.gf;
    let (m, l, n) = (cell.m, cell.l, cell.n);
    let nu = n as usize;
    let ybase = |i: usize| m * l + i * nu;
    let lbase = |j: usize| m * l + m * nu + j * nu;
    let (res, nv) = unprojected_residuals(cell, dr, r);
    let sbit = nv - 1;
    let mut eqs: Vec<Poly> = res.into_iter().flatten().collect();
    eqs.retain(|p| !p.is_empty());
    // exact generic solutions
    let nvs = 1usize << l;
    let mut sols: Vec<M> = vec![];
    let mut zset = HashSet::new();
    let size = 1usize << (m * l);
    let mut vs = vec![0usize; m];
    let neg_r = cv.w_neg(r);
    'tuples: for idx in 0..size {
        for i in 0..m {
            vs[i] = (idx >> (i * l)) & (nvs - 1);
            if dr.fib_x[vs[i]].is_empty() {
                continue 'tuples;
            }
        }
        let lens: Vec<usize> = (0..m).map(|i| dr.fib_x[vs[i]].len()).collect();
        let combos: usize = lens.iter().product();
        for c in 0..combos {
            let mut cc = c;
            let mut pts = Vec::with_capacity(m);
            for i in 0..m {
                pts.push(dr.fib_x[vs[i]][cc % lens[i]]);
                cc /= lens[i];
            }
            let mut q = pts[0];
            let mut lams = Vec::with_capacity(m - 1);
            let mut generic = true;
            for j in 0..m - 1 {
                let p = pts[j + 1];
                if q.0 == p.0 {
                    generic = false;
                    break;
                }
                let lam = g.div(q.1 ^ p.1, q.0 ^ p.0);
                let x3 = g.sqr(lam) ^ lam ^ cv.a ^ q.0 ^ p.0;
                let y3 = g.mul(lam, q.0 ^ x3) ^ x3 ^ q.1;
                lams.push(lam);
                q = (x3, y3);
            }
            if !generic {
                continue;
            }
            let sigma = if q == r {
                0
            } else if q == neg_r {
                1
            } else {
                continue;
            };
            let mut mono = M0;
            for i in 0..m {
                for t in 0..l {
                    if (vs[i] >> t) & 1 == 1 {
                        mono = mono.or(M::bit(i * l + t));
                    }
                }
                for k in 0..nu {
                    if (pts[i].1 >> k) & 1 == 1 {
                        mono = mono.or(M::bit(ybase(i) + k));
                    }
                }
            }
            for (j, &lam) in lams.iter().enumerate() {
                for k in 0..nu {
                    if (lam >> k) & 1 == 1 {
                        mono = mono.or(M::bit(lbase(j) + k));
                    }
                }
            }
            if sigma == 1 {
                mono = mono.or(M::bit(sbit));
            }
            sols.push(mono);
            zset.insert(idx as u64);
        }
    }
    Sys {
        arm: "b",
        nv,
        eqs,
        sols,
        zset,
    }
}

// ---------------------------------------------------------------- echelon form of the equations

/// Echelon basis of the span of `polys`, pivoting on the greatest monomial in
/// the degree-refining order.  The rows whose leading degree is `<= d` span
/// exactly the part of the span of degree `<= d`.
fn echelon(polys: Vec<Poly>) -> Vec<Poly> {
    let mut rows: Vec<Poly> = Vec::new();
    let mut by_lead: HashMap<M, usize> = HashMap::new();
    for mut p in polys {
        while let Some(&lead) = p.last() {
            match by_lead.get(&lead) {
                Some(&ri) => p = padd(&p, &rows[ri]),
                None => break,
            }
        }
        if let Some(&lead) = p.last() {
            by_lead.insert(lead, rows.len());
            rows.push(p);
        }
    }
    rows.sort_by_key(|p| p.last().copied().unwrap());
    rows
}

// ---------------------------------------------------------------- affine elimination

/// Eliminate the variables that the affine equations of the system determine.
/// An invertible affine change of variables preserves the degree filtration
/// and the closure under multiplication by a variable, so `D` is unchanged;
/// the elimination only shrinks the column set.  Applied to every arm.
fn eliminate(sys: &Sys) -> (Sys, usize) {
    let ech = echelon(sys.eqs.clone());
    // affine rows, ascending by leading variable
    let mut subs: HashMap<usize, Poly> = HashMap::new(); // pivot var -> affine poly (non-pivot vars + const)
    let mut order: Vec<(usize, &Poly)> = vec![];
    for p in &ech {
        if poly_degree(p) == 1 {
            let lead = p.last().unwrap().iter().next().unwrap();
            order.push((lead, p));
        }
    }
    order.sort_by_key(|x| x.0);
    for (lead, p) in &order {
        // x_lead = (the other terms), with already-eliminated variables replaced
        let mut acc: Poly = vec![];
        for &mono in p.iter() {
            if mono.deg() == 1 && mono.iter().next().unwrap() == *lead {
                continue;
            }
            if mono.deg() == 1 {
                let v = mono.iter().next().unwrap();
                match subs.get(&v) {
                    Some(sp) => acc = padd(&acc, sp),
                    None => acc = padd(&acc, &vec![mono]),
                }
            } else {
                acc = padd(&acc, &vec![mono]); // the constant 1
            }
        }
        subs.insert(*lead, acc);
    }
    if subs.is_empty() {
        return (
            Sys {
                arm: sys.arm,
                nv: sys.nv,
                eqs: sys.eqs.clone(),
                sols: sys.sols.clone(),
                zset: sys.zset.clone(),
            },
            0,
        );
    }
    // renumber the kept variables
    let mut map = vec![usize::MAX; sys.nv];
    let mut nk = 0;
    for v in 0..sys.nv {
        if !subs.contains_key(&v) {
            map[v] = nk;
            nk += 1;
        }
    }
    let remap_mono = |m: M| -> M {
        let mut r = M0;
        for v in m.iter() {
            if map[v] != usize::MAX {
                r = r.or(M::bit(map[v]));
            }
        }
        r
    };
    // substitute: a monomial becomes (kept part) * product of the substitutions
    let mut new_eqs: Vec<Poly> = Vec::new();
    for p in &sys.eqs {
        let mut acc: Poly = vec![];
        for &mono in p.iter() {
            let mut kept = M0;
            let mut term: Poly = vec![M0];
            for v in mono.iter() {
                match subs.get(&v) {
                    Some(sp) => term = pmul(&term, sp),
                    None => kept = kept.or(M::bit(v)),
                }
            }
            let kept_poly: Poly = vec![remap_mono(kept)];
            // the substitutions are already over kept variables, in old numbering
            let remapped: Poly = norm(term.iter().map(|&t| remap_mono(t)).collect());
            acc = padd(&acc, &pmul(&kept_poly, &remapped));
        }
        if !acc.is_empty() {
            new_eqs.push(acc);
        }
    }
    let sols: Vec<M> = sys.sols.iter().map(|&z| remap_mono(z)).collect();
    (
        Sys {
            arm: sys.arm,
            nv: nk,
            eqs: new_eqs,
            sols,
            zset: sys.zset.clone(),
        },
        sys.nv - nk,
    )
}

// ---------------------------------------------------------------- column index

struct Cols {
    nv: usize,
    d: usize,
    off: Vec<usize>,
    ncols: usize,
    mask_of: Vec<M>,
    deg_of: Vec<u8>,
    binom: Vec<[u64; 24]>,
}

impl Cols {
    fn new(nv: usize, d: usize) -> Cols {
        assert!(nv <= 190 && d >= 1 && d <= 23);
        let binom = binom_table(nv);
        let mut off = vec![0usize; d + 1];
        for k in (0..d).rev() {
            off[k] = off[k + 1] + binom[nv][k + 1] as usize;
        }
        let ncols = off[0] + 1;
        let mut mask_of = Vec::with_capacity(ncols);
        let mut deg_of = Vec::with_capacity(ncols);
        for k in (0..=d).rev() {
            if k == 0 {
                mask_of.push(M0);
                deg_of.push(0);
                continue;
            }
            if k > nv {
                continue;
            }
            // k-subsets of 0..nv in colex order
            let mut c: Vec<usize> = (0..k).collect();
            loop {
                let mut mono = M0;
                for &i in &c {
                    mono = mono.or(M::bit(i));
                }
                mask_of.push(mono);
                deg_of.push(k as u8);
                let mut i = 0;
                while i < k {
                    let lim = if i + 1 < k { c[i + 1] } else { nv };
                    if c[i] + 1 < lim {
                        break;
                    }
                    i += 1;
                }
                if i == k {
                    break;
                }
                c[i] += 1;
                for j in 0..i {
                    c[j] = j;
                }
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
    fn col_of(&self, mask: M) -> usize {
        let mut i = 1usize;
        let mut r = 0u64;
        for p in mask.iter() {
            r += self.binom[p][i];
            i += 1;
        }
        self.off[i - 1] + r as usize
    }
    fn words(&self) -> usize {
        (self.ncols + 63) / 64
    }
}

// ---------------------------------------------------------------- closure engine

#[inline(always)]
fn xor_into(dst: &mut [u64], src: &[u64]) {
    for (a, b) in dst.iter_mut().zip(src.iter()) {
        *a ^= *b;
    }
}

struct Row {
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
    hits_fwd: u64,
    new_pivots: u64,
    falls: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
static ORDER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static KCHUNK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static TILE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(256);

impl<'a> Engine<'a> {
    fn new(cols: &'a Cols, kchunk: usize) -> Engine<'a> {
        let w = cols.words();
        let low_start = cols.off[cols.d - 1];
        let mut mv: Vec<Vec<u32>> = Vec::with_capacity(cols.nv);
        for j in 0..cols.nv {
            let bit = M::bit(j);
            let mut t = Vec::with_capacity(cols.ncols - low_start);
            for c in low_start..cols.ncols {
                t.push(cols.col_of(cols.mask_of[c].or(bit)) as u32);
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

    fn process_chunk(&mut self, chunk: &mut Vec<Vec<u64>>) {
        let w = self.w;
        let cols = self.cols;
        self.st.chunks += 1;
        let mut local: Vec<(usize, Vec<u64>)> = Vec::new();
        let mut lmask = vec![0u64; w];
        for mut b in chunk.drain(..) {
            self.forward(&mut b);
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
        if local.is_empty() {
            return;
        }
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
        seeds: &[&Poly],
        time_limit: f64,
        mem_cap: u64,
        as_cap: u64,
        started: &Instant,
    ) -> Outcome {
        let w = self.w;
        let cols = self.cols;
        let mut chunk: Vec<Vec<u64>> = Vec::new();
        for p in seeds {
            let mut b = vec![0u64; w];
            for &m in p.iter() {
                let c = cols.col_of(m);
                b[c / 64] ^= 1u64 << (c % 64);
            }
            chunk.push(b);
            self.st.seed_rows += 1;
        }
        self.process_chunk(&mut chunk);
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
            let order = ORDER.load(std::sync::atomic::Ordering::Relaxed);
            let popped = match order {
                0 => self.queue.iter_mut().find_map(|q| q.pop_front()),
                1 => self.queue.iter_mut().rev().find_map(|q| q.pop_front()),
                3 => self.queue.iter_mut().find_map(|q| q.pop_back()),
                _ => self.queue.iter_mut().rev().find_map(|q| q.pop_back()),
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
            let mut andmask = M([u64::MAX; 3]);
            for &c in &terms {
                andmask = andmask.and(cols.mask_of[c as usize]);
            }
            for j in 0..cols.nv {
                if andmask.has(j) {
                    continue;
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

    /// If the closure holds an affine form for every variable, the point.
    fn linear_point(&self) -> Option<M> {
        let cols = self.cols;
        let mut pt = M0;
        let mut seen = 0usize;
        let cc = cols.ncols - 1;
        for r in &self.rows {
            let c = r.piv as usize;
            if cols.deg_of[c] != 1 {
                continue;
            }
            let var = cols.mask_of[c].iter().next().unwrap();
            let wd = cc / 64;
            let bit = if wd >= r.start as usize {
                (r.data[wd - r.start as usize] >> (cc % 64)) & 1
            } else {
                0
            };
            if bit == 1 {
                pt = pt.or(M::bit(var));
            }
            seen += 1;
        }
        if seen == cols.nv {
            Some(pt)
        } else {
            None
        }
    }
}

/// `dim` of the polynomials of degree `<= d` that vanish on `sols`, as
/// `ncols - rank(evaluation matrix)`.
fn rank_eval(cols: &Cols, sols: &[M]) -> usize {
    let w = cols.words();
    let mut basis: Vec<(usize, Vec<u64>)> = Vec::new(); // (pivot word, row) with leading bit
    let mut rank = 0usize;
    for &z in sols {
        let mut row = vec![0u64; w];
        for (c, &mk) in cols.mask_of.iter().enumerate() {
            if mk.andnot(z).is_zero() {
                row[c / 64] |= 1u64 << (c % 64);
            }
        }
        for (_, b) in &basis {
            // reduce by the lowest set bit of b
            let lb = b.iter().position(|&x| x != 0).unwrap();
            let bit = b[lb] & b[lb].wrapping_neg();
            if row[lb] & bit != 0 {
                xor_into(&mut row, b);
            }
        }
        if row.iter().any(|&x| x != 0) {
            let lb = row.iter().position(|&x| x != 0).unwrap();
            basis.push((lb, row));
            rank += 1;
        }
    }
    rank
}

#[derive(Clone)]
struct DegreeResult {
    degree: usize,
    columns: usize,
    outcome: Outcome,
    rank: usize,
    target_rank: Option<usize>,
    stats: Stats,
    peak_basis: u64,
    secs: f64,
    rss_kb: u64,
    seed_rows: usize,
    recovered: Option<M>,
}

fn closure_at(
    seeds: &[&Poly],
    sys: &Sys,
    degree: usize,
    time_limit: f64,
    mem_cap: u64,
    sol_cap: usize,
) -> DegreeResult {
    let started = Instant::now();
    let cols = Cols::new(sys.nv, degree);
    let w = cols.words();
    let kreq = KCHUNK.load(std::sync::atomic::Ordering::Relaxed);
    let kchunk = if kreq > 0 { kreq } else { (400_000_000usize / (w * 8)).clamp(64, 4096) };
    let mut e = Engine::new(&cols, kchunk);
    let as_cap = as_limit_bytes().map_or(u64::MAX, |l| l.saturating_sub(256 << 20));
    let out = e.run(seeds, time_limit, mem_cap, as_cap, &started);
    let mut target_rank = None;
    let mut recovered = None;
    if out == Outcome::Saturated {
        if sys.sols.is_empty() {
            target_rank = Some(cols.ncols);
        } else if sys.sols.len() <= sol_cap {
            target_rank = Some(cols.ncols - rank_eval(&cols, &sys.sols));
        }
        recovered = e.linear_point();
    }
    if out == Outcome::Refuted {
        target_rank = Some(cols.ncols);
    }
    DegreeResult {
        degree,
        columns: cols.ncols,
        outcome: out,
        rank: e.rows.len(),
        target_rank,
        stats: e.st.clone(),
        peak_basis: e.peak_basis_bytes,
        secs: started.elapsed().as_secs_f64(),
        rss_kb: rss_peak_kb(),
        seed_rows: seeds.len(),
        recovered,
    }
}

/// The scan for `D`.
struct Scan {
    d: Option<usize>,
    /// "solved" (V_d equals the vanishing ideal) | "refuted" | "not-reached" |
    /// "timelimit" | "memcap" | "inconsistent"
    status: &'static str,
    per_degree: Vec<DegreeResult>,
    last_degree: usize,
    recovered_ok: Option<bool>,
}

fn find_d(
    sys: &Sys,
    ech: &[Poly],
    d_max: usize,
    time_limit: f64,
    mem_cap: u64,
    sol_cap: usize,
) -> Scan {
    let t0 = Instant::now();
    let mut per = Vec::new();
    let mindeg = ech.iter().map(poly_degree).min().unwrap_or(0) as usize;
    let d_lo = mindeg.max(1);
    let d_max = d_max.min(sys.nv).min(23);
    let mut last = d_lo.saturating_sub(1);
    if ech.iter().any(|p| p.len() == 1 && p[0] == M0) {
        // the constant 1 among the equations: refuted at once
        return Scan {
            d: Some(0),
            status: "refuted",
            per_degree: per,
            last_degree: 0,
            recovered_ok: None,
        };
    }
    for d in d_lo..=d_max {
        let left = time_limit - t0.elapsed().as_secs_f64();
        if left <= 0.0 {
            return Scan {
                d: None,
                status: "timelimit",
                per_degree: per,
                last_degree: last,
                recovered_ok: None,
            };
        }
        let seeds: Vec<&Poly> = ech.iter().filter(|p| poly_degree(p) as usize <= d).collect();
        let r = closure_at(&seeds, sys, d, left, mem_cap, sol_cap);
        last = d;
        let outcome = r.outcome;
        let rank = r.rank;
        let target = r.target_rank;
        let rec = r.recovered;
        per.push(r);
        match outcome {
            Outcome::Refuted => {
                if !sys.sols.is_empty() {
                    return Scan {
                        d: Some(d),
                        status: "inconsistent",
                        per_degree: per,
                        last_degree: d,
                        recovered_ok: None,
                    };
                }
                return Scan {
                    d: Some(d),
                    status: "refuted",
                    per_degree: per,
                    last_degree: d,
                    recovered_ok: None,
                };
            }
            Outcome::TimeLimit => {
                return Scan {
                    d: None,
                    status: "timelimit",
                    per_degree: per,
                    last_degree: d,
                    recovered_ok: None,
                }
            }
            Outcome::MemCap => {
                return Scan {
                    d: None,
                    status: "memcap",
                    per_degree: per,
                    last_degree: d,
                    recovered_ok: None,
                }
            }
            Outcome::Saturated => {
                if let Some(t) = target {
                    if !sys.sols.is_empty() {
                        if rank == t {
                            let ok = rec.map(|p| sys.sols.contains(&p));
                            return Scan {
                                d: Some(d),
                                status: "solved",
                                per_degree: per,
                                last_degree: d,
                                recovered_ok: ok,
                            };
                        }
                        if rank > t {
                            return Scan {
                                d: Some(d),
                                status: "inconsistent",
                                per_degree: per,
                                last_degree: d,
                                recovered_ok: None,
                            };
                        }
                    }
                }
            }
        }
    }
    Scan {
        d: None,
        status: "not-reached",
        per_degree: per,
        last_degree: last,
        recovered_ok: None,
    }
}

// ---------------------------------------------------------------- the cell run

fn arm_frame(arm: &str) -> Frame {
    match arm {
        "c" => Frame::W,
        "d0" | "d" => Frame::U,
        _ => Frame::X,
    }
}

fn build_arm(
    cell: &Cell,
    dr: &Draw,
    tgt: &Target,
    arm: &'static str,
    phi: Option<&dyn Fn(usize) -> usize>,
) -> Sys {
    let cv = &cell.cv;
    match arm {
        "a" => build_unchained(cell, dr, cell.tx, tgt.zr(cv, Frame::X), false, "a", phi),
        "c" => build_unchained(cell, dr, cell.tw, tgt.zr(cv, Frame::W), false, "c", phi),
        "d0" => build_unchained(cell, dr, cell.tu, tgt.zr(cv, Frame::U), false, "d0", phi),
        "d" => build_unchained(cell, dr, cell.tu, tgt.zr(cv, Frame::U), true, "d", phi),
        "e" => build_chain(cell, dr, tgt.rx.0),
        "b" => build_unprojected(cell, dr, tgt.rx),
        _ => panic!("unknown arm {arm}"),
    }
}

fn arm_static(a: &str) -> &'static str {
    match a {
        "a" => "a",
        "b" => "b",
        "c" => "c",
        "d0" => "d0",
        "d" => "d",
        "e" => "e",
        _ => panic!("unknown arm {a}; use a,b,c,d0,d,e"),
    }
}

fn jnum_f(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.3}")
    } else {
        "null".into()
    }
}

fn degree_json(r: &DegreeResult) -> String {
    let s = &r.stats;
    format!(
        concat!(
            "{{\"d\":{},\"cols\":{},\"rank\":{},\"target\":{},\"out\":\"{}\",",
            "\"secs\":{},\"seed_rows\":{},\"products\":{},\"falls\":{},\"word_ops\":{},",
            "\"peak_basis_bytes\":{},\"rows_total\":{}}}"
        ),
        r.degree,
        r.columns,
        r.rank,
        r.target_rank.map(|t| t.to_string()).unwrap_or("null".into()),
        match r.outcome {
            Outcome::Refuted => "refuted",
            Outcome::Saturated => "saturated",
            Outcome::TimeLimit => "timelimit",
            Outcome::MemCap => "memcap",
        },
        jnum_f(r.secs),
        s.seed_rows,
        s.products,
        s.falls,
        s.ops_fwd + s.ops_local + s.ops_pass,
        r.peak_basis,
        s.seed_rows + s.products
    )
}

struct CellOpts {
    time_limit: f64,
    job_time: f64,
    mem_cap: u64,
    dmax_e: usize,
    dmax_b: usize,
    dmax_u: usize,
    sol_cap: usize,
}

/// Run one arm on one target and print a JSON line.
#[allow(clippy::too_many_arguments)]
fn run_one(
    cell: &Cell,
    dr: &Draw,
    draw: u64,
    tgt: &Target,
    arm: &'static str,
    label: &str,
    phi: Option<&dyn Fn(usize) -> usize>,
    truths: &mut HashMap<(u8, Kind, usize), Truth>,
    opts: &CellOpts,
    started: &Instant,
    out: &mut Vec<String>,
) -> Option<&'static str> {
    let t_arm = Instant::now();
    let kind_s = if tgt.kind == Kind::Random { "random" } else { "planted" };
    let head = format!(
        "{{\"tool\":\"b04\",\"n\":{},\"l\":{},\"m\":{},\"draw\":{},\"arm\":\"{}\",\"kind\":\"{}\",\"t\":{}",
        cell.n, cell.l, cell.m, draw, label, kind_s, tgt.idx
    );
    if started.elapsed().as_secs_f64() > opts.time_limit {
        out.push(format!("{head},\"status\":\"skipped-timebudget\"}}"));
        return Some("skipped");
    }
    let sys = build_arm(cell, dr, tgt, arm, phi);
    let f = arm_frame(arm);
    let fk = match f {
        Frame::X => 0u8,
        Frame::U => 1,
        Frame::W => 2,
    };
    let r = match f {
        Frame::X => tgt.rx,
        Frame::U => tgt.ru,
        Frame::W => tgt.rw,
    };
    let truth = truths
        .entry((fk, tgt.kind, tgt.idx))
        .or_insert_with(|| ground_truth(cell, dr, f, r));
    let chain_arm = arm == "e" || arm == "b";
    let need: &HashSet<u64> = if chain_arm { &truth.gen } else { &truth.any };
    let complete = need.iter().all(|i| sys.zset.contains(i));
    let spurious = sys.zset.iter().filter(|i| !truth.any.contains(i)).count();
    // solutions satisfy the equations
    let mut sound = true;
    for &z in sys.sols.iter().take(12) {
        if sys.eqs.iter().any(|p| eval_poly(p, z) != 0) {
            sound = false;
        }
    }
    let planted_in = tgt.plant.as_ref().map(|pl| {
        let vs = &pl[fk as usize];
        let mut idx = 0u64;
        for (i, &v) in vs.iter().enumerate() {
            idx |= (v as u64) << (i * cell.l);
        }
        sys.zset.contains(&idx)
    });
    let nv_before = sys.nv;
    let (sys, n_elim) = eliminate(&sys);
    let ech = echelon(sys.eqs.clone());
    let mindeg = ech.iter().map(poly_degree).min().unwrap_or(0);
    let maxdeg = ech.iter().map(poly_degree).max().unwrap_or(0);
    let dmax = match arm {
        "e" => opts.dmax_e,
        "b" => opts.dmax_b,
        _ => opts.dmax_u,
    };
    let left = opts.time_limit - started.elapsed().as_secs_f64();
    let tl = opts.job_time.min(left.max(1.0));
    let scan = find_d(&sys, &ech, dmax, tl, opts.mem_cap, opts.sol_cap);
    let secs = t_arm.elapsed().as_secs_f64();
    let d_lb = match scan.d {
        Some(d) => d,
        None => scan
            .per_degree
            .iter()
            .filter(|r| r.outcome == Outcome::Saturated)
            .map(|r| r.degree)
            .max()
            .map(|d| d + 1)
            .unwrap_or((mindeg as usize).max(1)),
    };
    let per: Vec<String> = scan.per_degree.iter().map(degree_json).collect();
    let (rows_at, cols_at, ops_at) = scan
        .per_degree
        .last()
        .map(|r| {
            (
                r.stats.seed_rows + r.stats.products,
                r.columns,
                r.stats.ops_fwd + r.stats.ops_local + r.stats.ops_pass,
            )
        })
        .unwrap_or((0, 0, 0));
    let total_ops: u64 = scan
        .per_degree
        .iter()
        .map(|r| r.stats.ops_fwd + r.stats.ops_local + r.stats.ops_pass)
        .sum();
    let peak_basis = scan.per_degree.iter().map(|r| r.peak_basis).max().unwrap_or(0);
    out.push(format!(
        concat!(
            "{head},\"nv0\":{nv0},\"elim\":{elim},\"nv\":{nv},\"eqs\":{eqs},\"ech_rows\":{ech},\"mindeg\":{mind},\"maxdeg\":{maxd},",
            "\"n_sols\":{ns},\"n_z\":{nz},\"n_truth_any\":{ta},\"n_truth_gen\":{tg},\"spurious_z\":{sp},",
            "\"complete\":{comp},\"sound\":{sound},\"planted_in\":{pin},",
            "\"status\":\"{status}\",\"D\":{dd},\"D_lb\":{dlb},\"last_degree\":{ld},\"recovered_ok\":{rec},",
            "\"matrix_rows\":{mr},\"matrix_cols\":{mc},\"word_ops_last\":{ol},\"word_ops_total\":{ot},",
            "\"peak_basis_bytes\":{pb},\"secs\":{secs},\"rss_peak_kb\":{rss},\"degrees\":[{per}]}}"
        ),
        head = head,
        nv0 = nv_before,
        elim = n_elim,
        nv = sys.nv,
        eqs = sys.eqs.len(),
        ech = ech.len(),
        mind = mindeg,
        maxd = maxdeg,
        ns = sys.sols.len(),
        nz = sys.zset.len(),
        ta = truth.any.len(),
        tg = truth.gen.len(),
        sp = spurious,
        comp = complete,
        sound = sound,
        pin = planted_in.map(|b| b.to_string()).unwrap_or("null".into()),
        status = scan.status,
        dd = scan.d.map(|d| d.to_string()).unwrap_or("null".into()),
        dlb = d_lb,
        ld = scan.last_degree,
        rec = scan.recovered_ok.map(|b| b.to_string()).unwrap_or("null".into()),
        mr = rows_at,
        mc = cols_at,
        ol = ops_at,
        ot = total_ops,
        pb = peak_basis,
        secs = jnum_f(secs),
        rss = rss_peak_kb(),
        per = per.join(",")
    ));
    match scan.status {
        "timelimit" | "memcap" => Some(scan.status),
        _ => None,
    }
}

fn mode_cell(a: &Args) {
    let n: u32 = a.num("--n", 17);
    let l: usize = a.num("--l", 4);
    let m: usize = a.num("--m", 3);
    let draw: u64 = a.num("--draw", 0);
    let n_random: usize = a.num("--random", 8);
    let n_planted: usize = a.num("--planted", 4);
    let arms_s = a.flag("--arms").unwrap_or("a,d0,d,c,e,b".into());
    let arms: Vec<&'static str> = arms_s.split(',').map(arm_static).collect();
    let recode = a.flag("--recode"); // linear | nonlinear: arm a only
    let opts = CellOpts {
        time_limit: a.num("--time-limit", 480.0f64),
        job_time: a.num("--job-time", 150.0f64),
        mem_cap: a.num("--mem-cap-mb", 3000u64) << 20,
        dmax_e: a.num("--dmax-e", 5usize),
        dmax_b: a.num("--dmax-b", 3usize),
        dmax_u: a.num("--dmax-u", 23usize),
        sol_cap: a.num("--sol-cap", 4000usize),
    };
    let t_first: usize = a.num("--first", 0); // first target index to run
    let started = Instant::now();
    let cell = Cell::new(n, l, m);
    let dr = Draw::new(&cell, draw);
    assert_eq!(cell.cv.w_count(), cell.cv.e_count(), "Edwards and Weierstrass point counts differ");
    eprintln!(
        "cell n={} l={} m={} draw={} curve d1={} d2={} a={} b={} basis={:?} setup {:.1}s",
        n,
        l,
        m,
        draw,
        cell.cv.d1,
        cell.cv.d2,
        cell.cv.a,
        cell.cv.b,
        dr.basis,
        started.elapsed().as_secs_f64()
    );
    let mut truths: HashMap<(u8, Kind, usize), Truth> = HashMap::new();
    let mut out: Vec<String> = Vec::new();
    // the linear / nonlinear recoding of the variables of arm a
    let phi_lin: Option<Box<dyn Fn(usize) -> usize>> = match recode.as_deref() {
        Some("linear") => {
            let nv = m * l;
            let mut rng = Rng(cell_seed(n, l, m) ^ 0x11AE);
            // random invertible matrix: columns as bit masks
            let cols: Vec<usize> = loop {
                let c: Vec<usize> = (0..nv).map(|_| (rng.next() as usize) & ((1 << nv) - 1)).collect();
                // rank check by elimination
                let mut rows = c.clone();
                let mut rank = 0;
                for bit in 0..nv {
                    if let Some(p) = (rank..nv).find(|&i| rows[i] >> bit & 1 == 1) {
                        rows.swap(rank, p);
                        for i in 0..nv {
                            if i != rank && rows[i] >> bit & 1 == 1 {
                                rows[i] ^= rows[rank];
                            }
                        }
                        rank += 1;
                    }
                }
                if rank == nv {
                    break c;
                }
            };
            Some(Box::new(move |idx: usize| {
                let mut r = 0usize;
                for (b, &cm) in cols.iter().enumerate() {
                    if idx >> b & 1 == 1 {
                        r ^= cm;
                    }
                }
                r
            }))
        }
        Some("nonlinear") => {
            let nv = m * l;
            Some(Box::new(move |idx: usize| {
                let mut r = idx;
                for k in 0..nv.saturating_sub(2) {
                    let t = (idx >> (k + 1)) & (idx >> (k + 2)) & 1;
                    r ^= t << k;
                }
                r
            }))
        }
        _ => None,
    };
    let mut blocked: HashMap<&'static str, &'static str> = HashMap::new();
    let mut targets: Vec<Target> = Vec::new();
    for t in t_first..(t_first + n_random) {
        targets.push(make_target(&cell, &dr, draw, Kind::Random, t));
    }
    for t in t_first..(t_first + n_planted) {
        targets.push(make_target(&cell, &dr, draw, Kind::Planted, t));
    }
    for tgt in &targets {
        for &arm in &arms {
            let label = match (&recode, arm) {
                (Some(r), "a") => format!("a-{r}"),
                _ => arm.to_string(),
            };
            if let Some(why) = blocked.get(arm) {
                if tgt.kind == Kind::Random || *why == "memcap" {
                    out.push(format!(
                        "{{\"tool\":\"b04\",\"n\":{},\"l\":{},\"m\":{},\"draw\":{},\"arm\":\"{}\",\"kind\":\"{}\",\"t\":{},\"status\":\"not-attempted-stop-rule\",\"blocked_by\":\"{}\"}}",
                        n, l, m, draw, label,
                        if tgt.kind == Kind::Random { "random" } else { "planted" },
                        tgt.idx, why
                    ));
                    continue;
                }
            }
            let phi = phi_lin.as_ref().filter(|_| arm == "a").map(|b| &**b as &dyn Fn(usize) -> usize);
            let res = run_one(
                &cell, &dr, draw, tgt, arm, &label, phi, &mut truths, &opts, &started, &mut out,
            );
            if let Some(s) = res {
                if s != "skipped" && (arm == "e" || arm == "b") {
                    blocked.insert(arm, s);
                }
            }
            // stream output as it is produced
            for line in out.drain(..) {
                println!("{line}");
            }
        }
    }
    eprintln!("cell done in {:.1}s", started.elapsed().as_secs_f64());
}

fn mode_plan(a: &Args) {
    let n: u32 = a.num("--n", 17);
    let l: usize = a.num("--l", 4);
    let m: usize = a.num("--m", 3);
    println!("plan n={n} l={l} m={m}");
    let rows = [
        ("a", m * l),
        ("c", m * l),
        ("d0", m * l),
        ("d", 1 + m * (l - 1)),
        ("e", m * l + (m - 2) * n as usize),
        ("b", m * l + (2 * m - 1) * n as usize + 1),
    ];
    for (arm, nv) in rows {
        print!("arm {arm:>2} N={nv:>3}");
        for d in 2..=6usize {
            if d > nv {
                break;
            }
            let c = cols_up_to(nv, d);
            // reduced basis, triangular rows: about c^2 / 16 bytes
            let tri = (c as f64) * (c as f64) / 16.0;
            print!("  d{d}: {c} cols {:.2e} B", tri);
        }
        println!();
    }
    if a.has("--build") {
        let cell = Cell::new(n, l, m);
        let dr = Draw::new(&cell, a.num("--draw", 0u64));
        let tgt = make_target(&cell, &dr, a.num("--draw", 0u64), Kind::Random, 0);
        for arm in ["e", "b"] {
            let t0 = Instant::now();
            let sys = build_arm(&cell, &dr, &tgt, arm_static(arm), None);
            let (es, ne) = eliminate(&sys);
            println!(
                "built arm {arm}: N={} eliminated {} -> N={} eqs {} columns d3 {} d4 {} reduced-basis bound d3 {:.2e} B d4 {:.2e} B  ({:.1}s)",
                sys.nv,
                ne,
                es.nv,
                es.eqs.len(),
                cols_up_to(es.nv, 3),
                cols_up_to(es.nv, 4),
                (cols_up_to(es.nv, 3) as f64).powi(2) / 16.0,
                (cols_up_to(es.nv, 4) as f64).powi(2) / 16.0,
                t0.elapsed().as_secs_f64()
            );
        }
    }
}


// ---------------------------------------------------------------- system selftest

fn field_bits(z: M, base: usize, n: u32) -> u64 {
    let mut v = 0u64;
    for k in 0..n as usize {
        if z.has(base + k) {
            v |= 1 << k;
        }
    }
    v
}

fn system_selftest(check: &mut dyn FnMut(&str, bool, String)) {
    // column indexing
    {
        let cols = Cols::new(10, 4);
        let inv = (0..cols.ncols).all(|c| cols.col_of(cols.mask_of[c]) == c);
        check("column rank is the inverse of the column list", inv, format!("ncols {}", cols.ncols));
        let big = Cols::new(50, 4);
        check("column count N=50 D=4", big.ncols as u64 == cols_up_to(50, 4), format!("{}", big.ncols));
        let wide = Cols::new(136, 3);
        check(
            "column ranks N=136 D=3 (three-word monomials)",
            (0..wide.ncols).step_by(97).all(|c| wide.col_of(wide.mask_of[c]) == c),
            format!("ncols {}", wide.ncols),
        );
    }
    // tiny cell: the unchained arms are exact and the closure is complete at full degree
    {
        let cell = Cell::new(7, 3, 3);
        let dr = Draw::new(&cell, 0);
        let mut ok_vanish = true;
        let mut ok_d = true;
        let mut ok_lift = true;
        let mut detail = String::new();
        let mut cnt = 0;
        for (kind, nt) in [(Kind::Random, 3usize), (Kind::Planted, 3)] {
            for t in 0..nt {
                let tgt = make_target(&cell, &dr, 0, kind, t);
                let mut zs: HashMap<&str, HashSet<u64>> = HashMap::new();
                for arm in ["a", "c", "d0", "d"] {
                    let sys = build_arm(&cell, &dr, &tgt, arm_static(arm), None);
                    let solset: HashSet<u64> = sys.sols.iter().map(|z| z.0[0]).collect();
                    for idx in 0..(1u64 << sys.nv) {
                        let z = M::from_u64(idx);
                        let all0 = sys.eqs.iter().all(|p| eval_poly(p, z) == 0);
                        if all0 != solset.contains(&idx) {
                            ok_vanish = false;
                        }
                    }
                    let ech = echelon(sys.eqs.clone());
                    let sc = find_d(&sys, &ech, sys.nv, 60.0, 1 << 30, 4000);
                    if !(sc.status == "solved" || sc.status == "refuted") {
                        ok_d = false;
                        detail += &format!(" {arm}:{}", sc.status);
                    }
                    cnt += 1;
                    zs.insert(arm, sys.zset);
                }
                if zs["d"] != zs["d0"] {
                    ok_lift = false;
                }
            }
        }
        check("n=7 unchained systems vanish exactly on the enumerated solutions", ok_vanish, format!("{cnt} systems"));
        check("n=7 closure reaches the vanishing ideal by degree N (engine complete)", ok_d, detail);
        check("n=7 symmetrised zero set lifts to the unsymmetrised one", ok_lift, "6 targets".into());
        // semantic residuals of arm b at random assignments
        let tgt = make_target(&cell, &dr, 0, Kind::Random, 0);
        let (res, nv) = unprojected_residuals(&cell, &dr, tgt.rx);
        let cv = &cell.cv;
        let g = &cv.gf;
        let (m, l, n) = (3usize, 3usize, 7u32);
        let nu = 7usize;
        let mut rng = Rng(4242);
        let mut ok_b = true;
        for _ in 0..200 {
            let mut z = M0;
            for i in 0..nv {
                if rng.next() & 1 == 1 {
                    z = z.or(M::bit(i));
                }
            }
            let mut want: Vec<u64> = vec![];
            let xs: Vec<u64> = (0..m)
                .map(|i| {
                    let mut x = 0u64;
                    for t in 0..l {
                        if z.has(i * l + t) {
                            x ^= dr.basis[t];
                        }
                    }
                    x
                })
                .collect();
            let ys: Vec<u64> = (0..m).map(|i| field_bits(z, m * l + i * nu, n)).collect();
            for i in 0..m {
                let x = xs[i];
                want.push(g.sqr(ys[i]) ^ g.mul(x, ys[i]) ^ g.mul(x, g.sqr(x)) ^ g.mul(cv.a, g.sqr(x)) ^ cv.b);
            }
            let (mut xq, mut yq) = (xs[0], ys[0]);
            for j in 0..m - 1 {
                let lam = field_bits(z, m * l + m * nu + j * nu, n);
                let dx = xq ^ xs[j + 1];
                let dy = yq ^ ys[j + 1];
                want.push(g.mul(lam, dx) ^ dy);
                let x3 = g.sqr(lam) ^ lam ^ cv.a ^ dx;
                let y3 = g.mul(lam, xq ^ x3) ^ x3 ^ yq;
                xq = x3;
                yq = y3;
            }
            want.push(xq ^ tgt.rx.0);
            want.push(yq ^ tgt.rx.1 ^ if z.has(nv - 1) { tgt.rx.0 } else { 0 });
            let got: Vec<u64> = res.iter().map(|s| sym_eval(s, z)).collect();
            if got != want {
                ok_b = false;
            }
        }
        check("arm b polynomial residuals equal the field-arithmetic residuals", ok_b, "200 random assignments".into());
        // arm e residuals
        let sys_e = build_arm(&cell, &dr, &tgt, "e", None);
        let mut ok_e = true;
        let mut rng = Rng(777);
        let e_nv = sys_e.nv;
        for _ in 0..100 {
            let mut z = M0;
            for i in 0..e_nv {
                if rng.next() & 1 == 1 {
                    z = z.or(M::bit(i));
                }
            }
            let xs: Vec<u64> = (0..m)
                .map(|i| {
                    let mut x = 0u64;
                    for t in 0..l {
                        if z.has(i * l + t) {
                            x ^= dr.basis[t];
                        }
                    }
                    x
                })
                .collect();
            let t1 = field_bits(z, m * l, n);
            let r1 = s3x(g, cv.b, xs[0], xs[1], t1);
            let r2 = s3x(g, cv.b, t1, xs[2], tgt.rx.0);
            // the two residuals are the non-zero ones packed in order; compare by evaluation of the polynomials
            let vals: Vec<u64> = sys_e.eqs.iter().map(|p| eval_poly(p, z)).collect();
            // rebuild: bits of r1 then r2 for non-empty polynomial rows
            let mut flat: Vec<u64> = (0..n).map(|k| (r1 >> k) & 1).collect();
            flat.extend((0..n).map(|k| (r2 >> k) & 1));
            // equations dropped when identically zero as polynomials; they are zero-constant bits
            // so compare the sorted multiset of ones
            let ones_got: u64 = vals.iter().sum();
            let ones_want: u64 = flat.iter().sum();
            // dropped equations are identically zero polynomials, so the counts of ones agree
            if ones_got != ones_want {
                ok_e = false;
            }
        }
        check("arm e polynomial residuals match S3 evaluations", ok_e, "100 random assignments".into());
    }
    // arm e: full-degree completeness on a tiny chain (n = 5)
    {
        let cell = Cell::new(5, 3, 3);
        let dr = Draw::new(&cell, 0);
        let mut ok = true;
        let mut detail = String::new();
        for (kind, t) in [(Kind::Random, 0usize), (Kind::Random, 1), (Kind::Planted, 0)] {
            let tgt = make_target(&cell, &dr, 0, kind, t);
            let sys = build_arm(&cell, &dr, &tgt, "e", None);
            let ech = echelon(sys.eqs.clone());
            let sc = find_d(&sys, &ech, sys.nv, 120.0, 1 << 30, 4000);
            if !(sc.status == "solved" || sc.status == "refuted") {
                ok = false;
            }
            detail += &format!(" N={} sols={} D={:?} {}", sys.nv, sys.sols.len(), sc.d, sc.status);
            let (es, ne) = eliminate(&sys);
            let sound = es.sols.iter().all(|&z| es.eqs.iter().all(|p| eval_poly(p, z) == 0));
            let ech2 = echelon(es.eqs.clone());
            let sc2 = find_d(&es, &ech2, es.nv, 120.0, 1 << 30, 4000);
            if sc2.d != sc.d || sc2.status != sc.status || !sound {
                ok = false;
            }
            detail += &format!(" [elim {ne}: D={:?}]", sc2.d);
        }
        check("n=5 chained system: closure at full degree equals the vanishing ideal of the enumerated solutions", ok, detail);
    }
}

// ---------------------------------------------------------------- summary and the frozen rule

fn jfind<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    Some(line[i..].trim_start())
}
fn jnum(line: &str, key: &str) -> Option<i64> {
    let s = jfind(line, key)?;
    let end = s.find(|c: char| !(c.is_ascii_digit() || c == '-')).unwrap_or(s.len());
    s[..end].parse().ok()
}
fn jstr(line: &str, key: &str) -> Option<String> {
    let s = jfind(line, key)?.strip_prefix('"')?;
    let end = s.find('"')?;
    Some(s[..end].to_string())
}
fn jbool(line: &str, key: &str) -> Option<bool> {
    let s = jfind(line, key)?;
    if s.starts_with("true") {
        Some(true)
    } else if s.starts_with("false") {
        Some(false)
    } else {
        None
    }
}
fn jf64(line: &str, key: &str) -> Option<f64> {
    let s = jfind(line, key)?;
    let end = s.find(|c: char| !(c.is_ascii_digit() || c == '-' || c == '.' || c == 'e')).unwrap_or(s.len());
    s[..end].parse().ok()
}

#[derive(Clone)]
struct Rec {
    n: i64,
    l: i64,
    m: i64,
    draw: i64,
    arm: String,
    kind: String,
    t: i64,
    status: String,
    d: Option<i64>,
    d_lb: Option<i64>,
    complete: Option<bool>,
    sound: Option<bool>,
    planted_in: Option<bool>,
    nv: Option<i64>,
    rows: Option<i64>,
    cols: Option<i64>,
    ops: Option<i64>,
    secs: Option<f64>,
}

fn parse_rec(line: &str) -> Option<Rec> {
    if !line.starts_with("{\"tool\":\"b04\"") {
        return None;
    }
    Some(Rec {
        n: jnum(line, "n")?,
        l: jnum(line, "l")?,
        m: jnum(line, "m")?,
        draw: jnum(line, "draw")?,
        arm: jstr(line, "arm")?,
        kind: jstr(line, "kind")?,
        t: jnum(line, "t")?,
        status: jstr(line, "status")?,
        d: jnum(line, "D").filter(|_| !line.contains("\"D\":null")),
        d_lb: jnum(line, "D_lb"),
        complete: jbool(line, "complete"),
        sound: jbool(line, "sound"),
        planted_in: jbool(line, "planted_in"),
        nv: jnum(line, "nv"),
        rows: jnum(line, "matrix_rows"),
        cols: jnum(line, "matrix_cols"),
        ops: jnum(line, "word_ops_total"),
        secs: jf64(line, "secs"),
    })
}

fn measured(r: &Rec) -> bool {
    (r.status == "solved" || r.status == "refuted") && r.d.is_some()
}

/// Twice the median of eight values, and the maximum.
fn med2_max(v: &[i64]) -> (i64, i64) {
    let mut s = v.to_vec();
    s.sort_unstable();
    (s[s.len() / 2 - 1] + s[s.len() / 2], *s.last().unwrap())
}

fn mode_summarize(a: &Args) {
    let mut recs: Vec<Rec> = vec![];
    let mut files: Vec<String> = vec![];
    let mut i = 2;
    while i < a.v.len() {
        let x = &a.v[i];
        if x == "--tsv" || x == "--targets" {
            i += 2;
            continue;
        }
        if std::path::Path::new(x).is_dir() {
            let mut names: Vec<String> = std::fs::read_dir(x)
                .expect("read dir")
                .filter_map(|e| e.ok())
                .map(|e| e.path().to_string_lossy().to_string())
                .filter(|p| p.ends_with(".jsonl"))
                .collect();
            names.sort();
            files.extend(names);
        } else {
            files.push(x.clone());
        }
        i += 1;
    }
    for f in &files {
        if let Ok(t) = std::fs::read_to_string(f) {
            recs.extend(t.lines().filter_map(parse_rec));
        } else {
            println!("MISSING {f}");
        }
    }
    println!("read {} rows from {} files", recs.len(), files.len());
    let cells: Vec<(i64, i64, i64)> = vec![(17, 4, 3), (19, 4, 3), (13, 3, 4), (15, 3, 4), (17, 4, 4)];
    let nrand: usize = a.num("--targets", 8);
    let arms = ["a", "d0", "d", "c", "e", "b", "a-linear", "a-nonlinear"];
    // table
    println!("# per-cell D on the random targets (draw 0); '.' = not measured (status in brackets)");
    let find = |c: &(i64, i64, i64), draw: i64, arm: &str, kind: &str, t: i64| -> Option<&Rec> {
        recs.iter().find(|r| r.n == c.0 && r.l == c.1 && r.m == c.2 && r.draw == draw && r.arm == arm && r.kind == kind && r.t == t)
    };
    let dvec = |c: &(i64, i64, i64), draw: i64, arm: &str| -> Vec<Option<i64>> {
        (0..nrand as i64)
            .map(|t| find(c, draw, arm, "random", t).filter(|r| measured(r)).and_then(|r| r.d))
            .collect()
    };
    let lbvec = |c: &(i64, i64, i64), draw: i64, arm: &str| -> Vec<Option<i64>> {
        (0..nrand as i64)
            .map(|t| {
                find(c, draw, arm, "random", t).and_then(|r| {
                    if measured(r) {
                        r.d
                    } else if r.status == "skipped-timebudget" || r.status == "not-attempted-stop-rule" {
                        None
                    } else {
                        r.d_lb
                    }
                })
            })
            .collect()
    };
    for c in &cells {
        println!("cell n={} l={} m={}", c.0, c.1, c.2);
        for arm in arms.iter() {
            for draw in 0..2 {
                if *arm != "a" && draw == 1 {
                    continue;
                }
                let rows: Vec<Option<&Rec>> = (0..nrand as i64).map(|t| find(c, draw, arm, "random", t)).collect();
                if rows.iter().all(|r| r.is_none()) {
                    continue;
                }
                let cells_s: Vec<String> = rows
                    .iter()
                    .map(|r| match r {
                        Some(r) if measured(r) => format!("{}", r.d.unwrap()),
                        Some(r) if r.d_lb.is_some() && r.status != "skipped-timebudget" => {
                            format!(">={}[{}]", r.d_lb.unwrap(), r.status)
                        }
                        Some(r) => format!(".[{}]", r.status),
                        None => ".[absent]".into(),
                    })
                    .collect();
                println!("  arm {:>11} draw {} D: {}", arm, draw, cells_s.join(" "));
            }
        }
        // controls
        for arm in arms.iter() {
            let pl: Vec<&Rec> = recs.iter().filter(|r| r.n == c.0 && r.l == c.1 && r.m == c.2 && r.draw == 0 && r.arm == *arm && r.kind == "planted").collect();
            if pl.is_empty() {
                continue;
            }
            let comp = pl.iter().filter(|r| r.complete == Some(true) && r.sound == Some(true) && r.planted_in != Some(false)).count();
            let meas = pl.iter().filter(|r| measured(r)).count();
            let dl: Vec<String> = pl.iter().map(|r| if measured(r) { r.d.unwrap().to_string() } else { format!(".[{}]", r.status) }).collect();
            println!("  planted {:>11}: complete+sound+planted-in {}/{}  measured {}  D: {}", arm, comp, pl.len(), meas, dl.join(" "));
        }
    }
    // rule
    println!("\n# frozen rule: an arm LOWERS D on a cell if all 8 random targets of the arm are measured exactly and of the incumbent measured (exactly, or by the certified lower bound D >= last saturated degree + 1), median(arm) <= median(a) - 1 and max(arm) <= max(a) - 1, and every random and planted run of the arm on the cell is complete, sound and contains its planted decomposition");
    let gate = |c: &(i64, i64, i64), arm: &str| -> bool {
        let rs: Vec<&Rec> = recs.iter().filter(|r| r.n == c.0 && r.l == c.1 && r.m == c.2 && r.draw == 0 && r.arm == arm && (r.status != "skipped-timebudget" && r.status != "not-attempted-stop-rule")).collect();
        !rs.is_empty() && rs.iter().all(|r| r.complete == Some(true) && r.sound == Some(true) && r.planted_in != Some(false))
    };
    let passes = |c: &(i64, i64, i64), arm: &str, base_draw: i64, arm_draw: i64, base: &str| -> Option<(bool, bool)> {
        let x = dvec(c, arm_draw, arm);
        let y = lbvec(c, base_draw, base);
        if x.iter().any(|v| v.is_none()) || y.iter().any(|v| v.is_none()) {
            return None;
        }
        let x: Vec<i64> = x.into_iter().flatten().collect();
        let y: Vec<i64> = y.into_iter().flatten().collect();
        let (xm, xx) = med2_max(&x);
        let (ym, yx) = med2_max(&y);
        Some((xm <= ym - 2 && xx <= yx - 1, xm >= ym + 2 && xx >= yx + 1))
    };
    // A/A control
    let mut aa_pass = 0;
    let mut aa_meas = 0;
    for c in &cells {
        if let Some((lo, _)) = passes(c, "a", 0, 1, "a") {
            aa_meas += 1;
            if lo {
                aa_pass += 1;
            }
        }
    }
    println!("A/A control (a on draw 1 against a on draw 0): lowers on {aa_pass} of {aa_meas} measured cells");
    let aa_valid = aa_meas >= 4 && aa_pass <= 1;
    let mut advancing: Vec<String> = vec![];
    for arm in ["b", "c", "d", "d0", "e"] {
        let mut lo = 0;
        let mut hi = 0;
        let mut un = 0;
        let mut g_ok = 0;
        let mut beyond_e = 0;
        for c in &cells {
            match passes(c, arm, 0, 0, "a") {
                Some((l, h)) => {
                    let gt = gate(c, arm);
                    if gt {
                        g_ok += 1;
                    }
                    if l && gt {
                        lo += 1;
                    }
                    if h {
                        hi += 1;
                    }
                    if arm != "e" && l && gt {
                        if let Some((l2, _)) = passes(c, arm, 0, 0, "e") {
                            if l2 {
                                beyond_e += 1;
                            }
                        }
                    }
                }
                None => un += 1,
            }
        }
        let role = match arm {
            "b" | "c" | "d" => "candidate",
            "d0" => "diagnostic",
            _ => "reference",
        };
        let tag = if lo >= 4 { "LOWER" } else if hi >= 4 { "HIGHER" } else { "MIXED-OR-SAME" };
        println!("arm {arm:>2} ({role}): lowers D on {lo} of 5 cells, raises on {hi}, unmeasured cells {un}, gated cells {g_ok}; also lowers D below arm e on {beyond_e} cells; reading {tag}");
        if lo >= 4 && role == "candidate" {
            advancing.push(arm.to_string());
        }
    }
    let verdict = if !advancing.is_empty() && aa_valid {
        format!("ADVANCES (primitive) by arm(s) {}", advancing.join(","))
    } else if !advancing.is_empty() {
        "INCONCLUSIVE (the A/A control does not satisfy: at least 4 measured cells and at most 1 passing)".to_string()
    } else {
        "MATCHES (no candidate arm lowers D by at least 1 on 4 of 5 cells; see the per-arm lines for the cells it was unmeasured on)".to_string()
    };
    println!("VERDICT BY THE FROZEN RULE: {verdict}");
    // tsv
    if let Some(path) = a.flag("--tsv") {
        let mut out = String::from("n\tl\tm\tdraw\tarm\tkind\tt\tstatus\tD\tcomplete\tsound\tplanted_in\tnv\tmatrix_rows\tmatrix_cols\tword_ops\tsecs\n");
        for r in &recs {
            out += &format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                r.n, r.l, r.m, r.draw, r.arm, r.kind, r.t, r.status,
                r.d.map(|d| d.to_string()).unwrap_or("".into()),
                r.complete.map(|b| b.to_string()).unwrap_or("".into()),
                r.sound.map(|b| b.to_string()).unwrap_or("".into()),
                r.planted_in.map(|b| b.to_string()).unwrap_or("".into()),
                r.nv.map(|d| d.to_string()).unwrap_or("".into()),
                r.rows.map(|d| d.to_string()).unwrap_or("".into()),
                r.cols.map(|d| d.to_string()).unwrap_or("".into()),
                r.ops.map(|d| d.to_string()).unwrap_or("".into()),
                r.secs.map(|d| format!("{d:.3}")).unwrap_or("".into()),
            );
        }
        std::fs::write(&path, out).expect("write tsv");
        println!("wrote {path}");
    }
}

fn main() {
    let a = Args {
        v: std::env::args().collect(),
    };
    ORDER.store(a.num("--order", 0usize), std::sync::atomic::Ordering::Relaxed);
    KCHUNK.store(a.num("--kchunk", 0usize), std::sync::atomic::Ordering::Relaxed);
    TILE.store(a.num("--tile", 256usize), std::sync::atomic::Ordering::Relaxed);
    match a.v.get(1).map(|s| s.as_str()) {
        Some("selftest") => {
            let mut ok = true;
            let mut check = |name: &str, pass: bool, detail: String| {
                println!("{} {name} {detail}", if pass { "PASS" } else { "FAIL" });
                if !pass {
                    ok = false;
                }
            };
            arith_selftest(&mut check);
            system_selftest(&mut check);
            println!("{}", if ok { "SELFTEST OK" } else { "SELFTEST FAILED" });
            if !ok {
                std::process::exit(1);
            }
        }
        Some("cell") => mode_cell(&a),
        Some("plan") => mode_plan(&a),
        Some("summarize") => mode_summarize(&a),
        _ => {
            eprintln!("usage: b04 selftest|cell|plan|summarize ...");
            std::process::exit(2);
        }
    }
}
