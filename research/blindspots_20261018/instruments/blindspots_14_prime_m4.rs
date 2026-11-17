//! **Blind-spot item 14: first prime-field arity m = 4 cost cells**
//! (`research/blindspots_20261018/14-prime-m4-cells`).
//!
//! The file depends on nothing but `std`, so `rustc` builds it on its own (the
//! `crypto_lib` library does not compile at the branch base).  Build:
//!
//! ```sh
//! rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native \
//!     -o <bin> examples/blindspots_14_prime_m4.rs
//! <bin> selftest [--deep]
//! <bin> find-curve --bits B --seed S
//! <bin> tdec --bits B --arms a,b --targets N --out FILE [--bcap-madds M] [--bx K]
//! <bin> fullic --bits B --arm a|b --runs N --out FILE [--bx K]
//! <bin> rho --bits B --runs N --out FILE
//! <bin> reach --out FILE
//! <bin> summarize --dir DIR --out FILE
//! <bin> reuse --files F1,F2,... --out FILE
//! ```
//!
//! What it measures (see `PROTOCOL.md`): the cost of one m = 4 decomposition
//! attempt (`R = P1 + P2 + P3 + P4` over the small-x base `{P : x(P) <= B}`) on a
//! prime-field curve with
//!
//! * arm (a): a table of all signed triple sums, probed once per signed base
//!   point (the generic enumeration / meet-in-the-middle reference);
//! * arm (b): Faugere's F4 over `F_p` on the Semaev polynomial `S5(x1..x4, xR)`
//!   together with the base membership polynomials `L(x_i) = prod (X - x)`;
//! * arm (d): the determinant condition of the Coppersmith / Jochemsz-May
//!   lattice for `S3`, `S4`, `S5` (an upper bound on the reach, no LLL loss);
//!
//! plus a matched Pollard rho (counted in the same group-addition unit) and the
//! committed PKM tower rows (arm (c)) copied from their sources by `reuse`.
//!
//! Unit.  One group addition (doubling included) is one group-addition
//! equivalent (GAE).  Everything else is a *native* operation, counted by name
//! and priced by the reader at 0, 0.1 and 1 GAE (AGENTS.md section 5a).

#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::unusual_byte_groupings,
    clippy::unnecessary_cast,
    clippy::collapsible_if,
    clippy::useless_vec,
    clippy::unnecessary_sort_by
)]

use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use std::sync::atomic::{AtomicU64, Ordering as AO};
use std::time::Instant;

// ───────────────────────── counters ─────────────────────────

const NC: usize = 16;
const GADD: usize = 0; // group additions and doublings (1 GAE each)
const LOOKUP: usize = 1; // hash-table probes
const INSERT: usize = 2; // hash-table inserts
const LEG: usize = 3; // Legendre symbols (modular exponentiations)
const SQRT: usize = 4; // modular square roots
const ROWOP: usize = 5; // modular multiply-adds in the relation matrix
const MADD: usize = 6; // F_p multiply-adds in F4 elimination
const HASHT: usize = 7; // distinguished-point tests (rho)
const FMUL: usize = 8; // field multiplications inside group additions
const FSQR: usize = 9;
const FINV: usize = 10;
const BUILD: usize = 11; // F_p multiplications building S5 / reducing it (arm b setup)
const NAMES: [&str; NC] = [
    "gadd", "lookup", "insert", "legendre", "sqrt", "rowop", "madd", "dptest", "fmul", "fsqr",
    "finv", "build", "x12", "x13", "x14", "x15",
];
static CT: [AtomicU64; NC] = [const { AtomicU64::new(0) }; NC];

#[inline]
fn bump(i: usize, n: u64) {
    CT[i].fetch_add(n, AO::Relaxed);
}
fn snap() -> [u64; NC] {
    let mut s = [0u64; NC];
    for (i, c) in CT.iter().enumerate() {
        s[i] = c.load(AO::Relaxed);
    }
    s
}
fn dsnap(a: &[u64; NC], b: &[u64; NC]) -> [u64; NC] {
    let mut d = [0u64; NC];
    for i in 0..NC {
        d[i] = b[i] - a[i];
    }
    d
}
fn counts_json(d: &[u64; NC]) -> String {
    let mut s = String::from("{");
    let mut first = true;
    for i in 0..NC {
        if NAMES[i].starts_with('x') && d[i] == 0 {
            continue;
        }
        if !first {
            s.push(',');
        }
        first = false;
        s.push_str(&format!("\"{}\":{}", NAMES[i], d[i]));
    }
    s.push('}');
    s
}
/// Native operations the reader prices at `w` GAE each.
fn native_total(d: &[u64; NC]) -> u64 {
    d[LOOKUP] + d[INSERT] + d[LEG] + d[SQRT] + d[ROWOP] + d[MADD] + d[HASHT] + d[BUILD]
}

// ───────────────────────── rng and number theory ─────────────────────────

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
    fn below(&mut self, n: u64) -> u64 {
        // rejection sampling for an unbiased draw
        let lim = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next();
            if v < lim {
                return v % n;
            }
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    // deterministic Miller-Rabin for n < 2^64
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let mm = |a: u64, b: u64| ((a as u128 * b as u128) % n as u128) as u64;
    let pw = |mut b: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mm(r, b);
            }
            b = mm(b, b);
            e >>= 1;
        }
        r
    };
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = pw(a % n, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mm(x, x);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// `F_p` for `p < 2^32`, `p = 3 mod 4` for the square root.
#[derive(Clone, Copy)]
struct Fp {
    p: u64,
}
impl Fp {
    #[inline]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        a * b % self.p
    }
    fn pow(&self, mut b: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    /// Inverse by the extended Euclid algorithm; `a != 0`.
    fn inv(&self, a: u64) -> u64 {
        let (mut r0, mut r1) = (self.p as i64, a as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        debug_assert_eq!(r0, 1);
        if t0 < 0 {
            (t0 + self.p as i64) as u64
        } else {
            t0 as u64
        }
    }
    /// Legendre symbol by Euler's criterion (counted).
    fn legendre(&self, a: u64) -> i32 {
        bump(LEG, 1);
        if a == 0 {
            return 0;
        }
        if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    /// Square root for `p = 3 mod 4` (counted); `a` must be a residue.
    fn sqrt(&self, a: u64) -> u64 {
        bump(SQRT, 1);
        self.pow(a, (self.p + 1) / 4)
    }
}

// ───────────────────────── the curve ─────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: u64,
    y: u64,
    inf: bool,
}
const INF: Pt = Pt {
    x: 0,
    y: 0,
    inf: true,
};
/// Partition of the r-adding walk: a multiplicative hash of the abscissa.
fn part(x: u64) -> usize {
    ((x.wrapping_mul(0x9E37_79B1) >> 9) % 16) as usize
}
fn key(q: &Pt) -> u64 {
    if q.inf {
        u64::MAX
    } else {
        (q.x << 32) | q.y
    }
}

#[derive(Clone, Copy)]
struct Curve {
    f: Fp,
    a: u64,
    b: u64,
}
impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let f = &self.f;
        let x2 = f.mul(x, x);
        f.add(f.add(f.mul(x2, x), f.mul(self.a, x)), self.b)
    }
    fn neg(&self, q: Pt) -> Pt {
        if q.inf {
            q
        } else {
            Pt {
                x: q.x,
                y: self.f.neg(q.y),
                inf: false,
            }
        }
    }
    /// One group addition (doubling and `P + (-P)` included); counted as one GAE
    /// whenever both operands are finite.
    fn add(&self, p: Pt, q: Pt) -> Pt {
        if p.inf {
            return q;
        }
        if q.inf {
            return p;
        }
        let f = &self.f;
        bump(GADD, 1);
        let lam;
        if p.x == q.x {
            if f.add(p.y, q.y) == 0 {
                return INF;
            }
            let x2 = f.mul(p.x, p.x);
            let num = f.add(f.add(f.add(x2, f.add(x2, x2)), 0), self.a);
            lam = f.mul(num, f.inv(f.add(p.y, p.y)));
            bump(FMUL, 2);
            bump(FSQR, 2);
            bump(FINV, 1);
        } else {
            lam = f.mul(f.sub(q.y, p.y), f.inv(f.sub(q.x, p.x)));
            bump(FMUL, 2);
            bump(FSQR, 1);
            bump(FINV, 1);
        }
        let x3 = f.sub(f.sub(f.mul(lam, lam), p.x), q.x);
        let y3 = f.sub(f.mul(lam, f.sub(p.x, x3)), p.y);
        Pt {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    fn mul(&self, mut k: u64, p: Pt) -> Pt {
        let mut acc = INF;
        let mut base = p;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            k >>= 1;
            if k > 0 {
                base = self.add(base, base);
            }
        }
        acc
    }
    fn on_curve(&self, q: Pt) -> bool {
        q.inf || self.f.mul(q.y, q.y) == self.rhs(q.x)
    }
    /// A point with abscissa `x`, canonical (smaller) ordinate, if one exists (counted).
    fn lift(&self, x: u64) -> Option<Pt> {
        let r = self.rhs(x);
        if self.f.legendre(r) < 0 {
            return None;
        }
        let y = self.f.sqrt(r);
        let y = y.min(self.f.neg(y));
        Some(Pt { x, y, inf: false })
    }
}

/// `#E(F_p)` by a full character sum with incremental differences (uncounted setup).
fn count_points(p: u64, a: u64, b: u64) -> u64 {
    let words = (p as usize) / 64 + 1;
    let mut sq = vec![0u64; words];
    let (mut y2, mut dy) = (0u64, 1u64); // y^2, 2y+1
    for _ in 0..p {
        sq[(y2 >> 6) as usize] |= 1 << (y2 & 63);
        y2 += dy;
        if y2 >= p {
            y2 -= p;
        }
        dy += 2;
        if dy >= p {
            dy -= p;
        }
    }
    let f = Fp { p };
    // rhs(x) = x^3 + a x + b; d1(x) = rhs(x+1) - rhs(x) = 3x^2 + 3x + 1 + a; d2(x) = d1(x+1) - d1(x) = 6x + 6
    let mut rhs = b % p;
    let mut d1 = f.add(1, a % p);
    let mut d2 = 6 % p;
    let mut sum: i64 = 0;
    for _ in 0..p {
        if rhs != 0 {
            if (sq[(rhs >> 6) as usize] >> (rhs & 63)) & 1 == 1 {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
        rhs = f.add(rhs, d1);
        d1 = f.add(d1, d2);
        d2 = f.add(d2, 6 % p);
    }
    (p as i64 + 1 + sum) as u64
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    bits: u32,
    p: u64,
    a: u64,
    b: u64,
    n: u64, // prime group order
    gx: u64,
    gy: u64,
}
impl Cell {
    fn curve(&self) -> Curve {
        Curve {
            f: Fp { p: self.p },
            a: self.a,
            b: self.b,
        }
    }
    fn g(&self) -> Pt {
        Pt {
            x: self.gx,
            y: self.gy,
            inf: false,
        }
    }
}

/// Frozen instances (`find-curve --bits B --seed 20261018`), each of prime order.
/// Recounted by `selftest`.
const CELLS: [Cell; 4] = [
    Cell {
        bits: 16,
        p: 48479,
        a: 25392,
        b: 20877,
        n: 48383,
        gx: 3,
        gy: 19437,
    },
    Cell {
        bits: 20,
        p: 661603,
        a: 422772,
        b: 346249,
        n: 660917,
        gx: 9,
        gy: 268107,
    },
    Cell {
        bits: 24,
        p: 10539323,
        a: 7836717,
        b: 8828358,
        n: 10542893,
        gx: 3,
        gy: 2623236,
    },
    Cell {
        bits: 28,
        p: 248685287,
        a: 49054657,
        b: 177851102,
        n: 248685841,
        gx: 1,
        gy: 65256793,
    },
];

fn cell_for(bits: u32) -> Cell {
    for c in CELLS.iter() {
        if c.bits == bits {
            return *c;
        }
    }
    panic!("no frozen cell for {bits} bits");
}

fn find_curve(bits: u32, seed: u64) -> Cell {
    let mut rng = Rng(seed ^ ((bits as u64) << 40));
    loop {
        let p = loop {
            let c = (rng.next() & ((1u64 << bits) - 1)) | (1u64 << (bits - 1)) | 3;
            if c % 4 == 3 && is_prime(c) {
                break c;
            }
        };
        for _ in 0..64 {
            let a = rng.below(p);
            let b = 1 + rng.below(p - 1);
            let f = Fp { p };
            // non-singular
            let disc = f.add(f.mul(4, f.mul(a, f.mul(a, a))), f.mul(27, f.mul(b, b)));
            if disc == 0 {
                continue;
            }
            let n = count_points(p, a, b);
            if is_prime(n) {
                let cv = Curve { f, a, b };
                // smallest x with a point
                let mut x = 1;
                let g = loop {
                    let r = cv.rhs(x);
                    if r != 0 && cv.f.pow(r, (p - 1) / 2) == 1 {
                        let y = cv.f.pow(r, (p + 1) / 4);
                        break Pt {
                            x,
                            y: y.min(p - y),
                            inf: false,
                        };
                    }
                    x += 1;
                };
                return Cell {
                    bits,
                    p,
                    a,
                    b,
                    n,
                    gx: g.x,
                    gy: g.y,
                };
            }
        }
    }
}

// ───────────────────────── small helpers ─────────────────────────

fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}
fn arg_u64(args: &[String], name: &str, default: u64) -> u64 {
    arg(args, name)
        .map(|s| s.parse().expect("number"))
        .unwrap_or(default)
}
fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}
struct Out {
    f: Option<std::fs::File>,
}
impl Out {
    fn new(path: Option<&str>) -> Self {
        Out {
            f: path.map(|p| std::fs::File::create(p).expect("create out")),
        }
    }
    fn line(&mut self, s: &str) {
        println!("{s}");
        if let Some(f) = self.f.as_mut() {
            writeln!(f, "{s}").unwrap();
        }
    }
}

// ───────────────────────── the small-x base ─────────────────────────

/// `F = {P : x(P) in [1, B]}`, both signs.  Signed index `2c` is `+P_c`, `2c+1` is `-P_c`.
struct Base {
    xs: Vec<u64>, // valid abscissae, ascending
    pts: Vec<Pt>, // signed points
    bound: u64,
}
impl Base {
    /// The base of the first `k` abscissae (in increasing order) that lie on the curve.
    fn build_k(cv: &Curve, k: usize) -> Base {
        let mut xs = Vec::new();
        let mut pts = Vec::new();
        let mut x = 1u64;
        while xs.len() < k {
            if let Some(pt) = cv.lift(x) {
                xs.push(x);
                pts.push(pt);
                pts.push(cv.neg(pt));
            }
            x += 1;
        }
        Base {
            xs,
            pts,
            bound: x - 1,
        }
    }
    fn nsigned(&self) -> usize {
        self.pts.len()
    }
}

/// Number of valid abscissae of the base of a cell: the smallest `k` with `2k >= p^(1/4)`, so
/// `|F| = 2k` signed points, the fixed point of `B = (p T)^(1/5)` with `T = |F|`.
fn cell_bx(c: &Cell) -> usize {
    let r = (c.p as f64).powf(0.25);
    (r / 2.0).ceil() as usize
}

// ───────────────────────── arm (a): triple table ─────────────────────────

struct TripleTable {
    map: HashMap<u64, [u16; 3]>,
}
impl TripleTable {
    fn build(cv: &Curve, base: &Base) -> TripleTable {
        let f = base.nsigned();
        let mut map: HashMap<u64, [u16; 3]> = HashMap::with_capacity(f * f * f / 6 + 16);
        for i in 0..f {
            for j in i..f {
                let s2 = cv.add(base.pts[i], base.pts[j]);
                for k in j..f {
                    let s3 = cv.add(s2, base.pts[k]);
                    bump(INSERT, 1);
                    map.entry(key(&s3))
                        .or_insert([i as u16, j as u16, k as u16]);
                }
            }
        }
        TripleTable { map }
    }
    /// One attempt: for each signed base point `P`, probe `R - P`; the first hit wins.
    fn decompose(&self, cv: &Curve, base: &Base, r: Pt) -> Option<[usize; 4]> {
        for i in 0..base.nsigned() {
            let q = cv.add(r, cv.neg(base.pts[i]));
            bump(LOOKUP, 1);
            if let Some(t) = self.map.get(&key(&q)) {
                return Some([i, t[0] as usize, t[1] as usize, t[2] as usize]);
            }
        }
        None
    }
}

/// Independent cross-check: a pair table, `R - (Pi + Pj)` probed against all pair sums.
struct PairTable {
    map: HashMap<u64, Vec<(u16, u16)>>,
    sums: Vec<(u16, u16, Pt)>,
}
impl PairTable {
    fn build(cv: &Curve, base: &Base) -> PairTable {
        let f = base.nsigned();
        let mut map: HashMap<u64, Vec<(u16, u16)>> = HashMap::new();
        let mut sums = Vec::new();
        for i in 0..f {
            for j in i..f {
                let s = cv.add(base.pts[i], base.pts[j]);
                map.entry(key(&s)).or_default().push((i as u16, j as u16));
                sums.push((i as u16, j as u16, s));
            }
        }
        PairTable { map, sums }
    }
    fn decompositions(&self, cv: &Curve, r: Pt) -> Vec<[usize; 4]> {
        let mut out = Vec::new();
        for &(i, j, s) in &self.sums {
            let q = cv.add(r, cv.neg(s));
            if let Some(v) = self.map.get(&key(&q)) {
                for &(k, l) in v {
                    out.push([i as usize, j as usize, k as usize, l as usize]);
                }
            }
        }
        out
    }
}

fn verify_decomposition(cv: &Curve, base: &Base, d: &[usize; 4], r: Pt) -> bool {
    let mut s = INF;
    for &i in d {
        s = cv.add(s, base.pts[i]);
    }
    s == r
}

/// Ordered `x`-tuples over the valid abscissae with `e1 P1 + e2 P2 + e3 P3 + e4 P4 = +-R`
/// for some signs: the zero set of `S5(., ., ., ., xR)` on the grid (uncounted control).
fn brute_grid_roots(cv: &Curve, base: &Base, r: Pt) -> u64 {
    let b = base.xs.len();
    let negr = cv.neg(r);
    let mut count = 0u64;
    for c1 in 0..b {
        for c2 in 0..b {
            let s12 = [
                cv.add(base.pts[2 * c1], base.pts[2 * c2]),
                cv.add(base.pts[2 * c1], base.pts[2 * c2 + 1]),
            ];
            for c3 in 0..b {
                let mut s123 = [INF; 4];
                for (t, s) in s123.iter_mut().enumerate() {
                    *s = cv.add(s12[t & 1], base.pts[2 * c3 + (t >> 1)]);
                }
                for c4 in 0..b {
                    let mut hit = false;
                    'o: for s in s123.iter() {
                        for e4 in 0..2 {
                            let s4 = cv.add(*s, base.pts[2 * c4 + e4]);
                            if s4 == r || s4 == negr {
                                hit = true;
                                break 'o;
                            }
                        }
                    }
                    if hit {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

// ───────────────────────── the relation matrix ─────────────────────────

struct Matrix {
    n: u64,
    cols: usize,                          // unknowns: b_x base logs and d
    pivots: Vec<Option<(Vec<u64>, u64)>>, // pivot row per column: (coeffs normalised to 1 at the pivot, rhs)
    rank: usize,
}
impl Matrix {
    fn new(n: u64, cols: usize) -> Self {
        Matrix {
            n,
            cols,
            pivots: vec![None; cols],
            rank: 0,
        }
    }
    fn modinv(&self, a: u64) -> u64 {
        Fp { p: self.n }.inv(a)
    }
    /// Reduce a row and keep it if independent.  Counted: one `rowop` per modular multiply-add.
    fn add_row(&mut self, mut row: Vec<u64>, mut rhs: u64) -> bool {
        let n = self.n;
        for c in 0..self.cols {
            if row[c] == 0 {
                continue;
            }
            if let Some((pr, prhs)) = &self.pivots[c] {
                let fct = row[c];
                for k in c..self.cols {
                    if pr[k] != 0 {
                        row[k] = (row[k] + n - fct * pr[k] % n) % n;
                    }
                }
                rhs = (rhs + n - fct * prhs % n) % n;
                bump(ROWOP, (self.cols - c + 1) as u64);
            } else {
                let inv = self.modinv(row[c]);
                for k in c..self.cols {
                    row[k] = row[k] * inv % n;
                }
                rhs = rhs * inv % n;
                bump(ROWOP, (self.cols - c + 1) as u64);
                self.pivots[c] = Some((row, rhs));
                self.rank += 1;
                return true;
            }
        }
        false
    }
    /// Back substitution; returns the solution vector.
    fn solve(&self) -> Option<Vec<u64>> {
        if self.rank < self.cols {
            return None;
        }
        let n = self.n;
        let mut sol = vec![0u64; self.cols];
        for c in (0..self.cols).rev() {
            let (pr, rhs) = self.pivots[c].as_ref()?;
            let mut v = *rhs;
            for k in c + 1..self.cols {
                v = (v + n - pr[k] * sol[k] % n) % n;
            }
            bump(ROWOP, (self.cols - c) as u64);
            sol[c] = v;
        }
        Some(sol)
    }
}

// ───────────────────────── Semaev S5 and the F4 engine (arm b) ─────────────────────────

mod s5 {
    use super::*;

    /// Coefficients of S3(x1, x2, z) in z: [c0, c1, c2].
    pub fn s3_in_z(f: &Fp, a: u64, b: u64, x1: u64, x2: u64) -> [u64; 3] {
        let d = f.sub(x1, x2);
        let c2 = f.mul(d, d);
        let s = f.add(x1, x2);
        let m = f.mul(x1, x2);
        let c1 = f.neg(f.mul(2, f.add(f.mul(s, f.add(m, a)), f.mul(2, b))));
        let t = f.sub(m, a);
        let c0 = f.sub(f.mul(t, t), f.mul(4, f.mul(b, s)));
        [c0, c1, c2]
    }

    fn psub(f: &Fp, p: &[u64], q: &[u64]) -> Vec<u64> {
        let n = p.len().max(q.len());
        (0..n)
            .map(|i| f.sub(*p.get(i).unwrap_or(&0), *q.get(i).unwrap_or(&0)))
            .collect()
    }
    fn pmul(f: &Fp, p: &[u64], q: &[u64]) -> Vec<u64> {
        let mut r = vec![0u64; p.len() + q.len() - 1];
        for (i, &u) in p.iter().enumerate() {
            for (j, &v) in q.iter().enumerate() {
                r[i + j] = f.add(r[i + j], f.mul(u, v));
            }
        }
        r
    }

    /// S4(x1, x2, x3, X) as a polynomial in X of formal degree 4: Res_Y(S3(x1,x2,Y), S3(x3,X,Y)).
    pub fn s4_in_x(f: &Fp, a: u64, b: u64, x1: u64, x2: u64, x3: u64) -> Vec<u64> {
        let [f0, f1, f2] = s3_in_z(f, a, b, x1, x2);
        // S3(x3, X, Y) in Y, coefficients polynomials in X
        // g2 = (x3 - X)^2 ; g1 = -2((x3 + X)(x3 X + a) + 2b) ; g0 = (x3 X - a)^2 - 4b(x3 + X)
        let x32 = f.mul(x3, x3);
        let g2 = vec![x32, f.neg(f.mul(2, x3)), 1];
        let t = vec![f.mul(a, x3), f.add(x32, a), x3]; // (x3+X)(x3 X + a)
        let g1: Vec<u64> = vec![
            f.neg(f.mul(2, f.add(t[0], f.mul(2, b)))),
            f.neg(f.mul(2, t[1])),
            f.neg(f.mul(2, t[2])),
        ];
        let g0 = vec![
            f.sub(f.mul(a, a), f.mul(4, f.mul(b, x3))),
            f.sub(f.neg(f.mul(2, f.mul(a, x3))), f.mul(4, b)),
            x32,
        ];
        let sc = |p: &[u64], c: u64| p.iter().map(|&v| f.mul(v, c)).collect::<Vec<u64>>();
        // Res = (f2 g0 - f0 g2)^2 - (f2 g1 - f1 g2)(f1 g0 - f0 g1)
        let u = psub(f, &sc(&g0, f2), &sc(&g2, f0));
        let v = psub(f, &sc(&g1, f2), &sc(&g2, f1));
        let w = psub(f, &sc(&g0, f1), &sc(&g1, f0));
        let mut r = psub(f, &pmul(f, &u, &u), &pmul(f, &v, &w));
        r.resize(5, 0);
        r
    }

    /// Determinant of an `n x n` matrix mod p (Gaussian elimination).
    fn det(f: &Fp, mut m: Vec<Vec<u64>>) -> u64 {
        let n = m.len();
        let mut d = 1u64;
        for c in 0..n {
            let mut piv = None;
            for r in c..n {
                if m[r][c] != 0 {
                    piv = Some(r);
                    break;
                }
            }
            let Some(r) = piv else { return 0 };
            if r != c {
                m.swap(r, c);
                d = f.neg(d);
            }
            d = f.mul(d, m[c][c]);
            let inv = f.inv(m[c][c]);
            for r2 in c + 1..n {
                if m[r2][c] != 0 {
                    let fct = f.mul(m[r2][c], inv);
                    for k in c..n {
                        let t = f.mul(fct, m[c][k]);
                        m[r2][k] = f.sub(m[r2][k], t);
                    }
                }
            }
        }
        d
    }

    /// S5(x1..x5) = Res_X(S4(x1,x2,x3,X), S3(x4,x5,X)), formal degrees (4, 2).
    pub fn s5_eval(f: &Fp, a: u64, b: u64, x: [u64; 5]) -> u64 {
        let g = s4_in_x(f, a, b, x[0], x[1], x[2]); // g[0..=4]
        let h = s3_in_z(f, a, b, x[3], x[4]); // h[0..=2]
                                              // Sylvester matrix, 6 x 6: 2 rows of g (descending coefficients), 4 rows of h.
        let gd = [g[4], g[3], g[2], g[1], g[0]];
        let hd = [h[2], h[1], h[0]];
        let mut m = vec![vec![0u64; 6]; 6];
        for r in 0..2 {
            for (k, &v) in gd.iter().enumerate() {
                m[r][r + k] = v;
            }
        }
        for r in 0..4 {
            for (k, &v) in hd.iter().enumerate() {
                m[2 + r][r + k] = v;
            }
        }
        det(f, m)
    }

    pub const D: usize = 9; // nodes per variable (degree 8)

    /// All 9^5 coefficients of S5 in x1..x5, index ((((e1*9+e2)*9+e3)*9+e4)*9+e5).
    pub fn s5_poly(f: &Fp, a: u64, b: u64) -> Vec<u64> {
        let total = D.pow(5);
        // counted as native work: ~200 field multiplications per resultant evaluation (estimate),
        // plus 5 axes x 9^5 x 9 multiplications of the interpolation
        bump(BUILD, total as u64 * 200 + 5 * total as u64 * 9);
        let mut v = vec![0u64; total];
        let mut idx = 0usize;
        for i1 in 0..D {
            for i2 in 0..D {
                for i3 in 0..D {
                    for i4 in 0..D {
                        for i5 in 0..D {
                            v[idx] = s5_eval(
                                f,
                                a,
                                b,
                                [i1 as u64, i2 as u64, i3 as u64, i4 as u64, i5 as u64],
                            );
                            idx += 1;
                        }
                    }
                }
            }
        }
        // inverse Vandermonde on nodes 0..8
        let mut vm = vec![vec![0u64; 2 * D]; D];
        for i in 0..D {
            let mut pw = 1u64;
            for j in 0..D {
                vm[i][j] = pw;
                pw = f.mul(pw, i as u64);
            }
            vm[i][D + i] = 1;
        }
        for c in 0..D {
            let r = (c..D)
                .find(|&r| vm[r][c] != 0)
                .expect("vandermonde invertible (p > 8)");
            vm.swap(r, c);
            let inv = f.inv(vm[c][c]);
            for k in 0..2 * D {
                vm[c][k] = f.mul(vm[c][k], inv);
            }
            for r2 in 0..D {
                if r2 != c && vm[r2][c] != 0 {
                    let fct = vm[r2][c];
                    for k in 0..2 * D {
                        let t = f.mul(fct, vm[c][k]);
                        vm[r2][k] = f.sub(vm[r2][k], t);
                    }
                }
            }
        }
        let vinv: Vec<Vec<u64>> = (0..D).map(|i| vm[i][D..].to_vec()).collect();
        // apply along each of the 5 axes: coefficient_j = sum_i vinv[j][i] * value_i
        let mut stride = 1usize;
        for _axis in 0..5 {
            let mut out = vec![0u64; total];
            for base in 0..total {
                let e = (base / stride) % D;
                if e != 0 {
                    continue;
                }
                for j in 0..D {
                    let mut acc = 0u64;
                    for i in 0..D {
                        acc = f.add(acc, f.mul(vinv[j][i], v[base + i * stride]));
                    }
                    out[base + j * stride] = acc;
                }
            }
            v = out;
            stride *= D;
        }
        v
    }

    /// Evaluate the stored polynomial at a point (for controls).
    pub fn poly_eval(f: &Fp, c: &[u64], x: [u64; 5]) -> u64 {
        let mut pw = [[1u64; D]; 5];
        for t in 0..5 {
            for e in 1..D {
                pw[t][e] = f.mul(pw[t][e - 1], x[t]);
            }
        }
        let mut acc = 0u64;
        let mut idx = 0usize;
        for e1 in 0..D {
            for e2 in 0..D {
                for e3 in 0..D {
                    for e4 in 0..D {
                        for e5 in 0..D {
                            let co = c[idx];
                            idx += 1;
                            if co == 0 {
                                continue;
                            }
                            let m = f.mul(
                                f.mul(f.mul(pw[0][e1], pw[1][e2]), f.mul(pw[2][e3], pw[3][e4])),
                                pw[4][e5],
                            );
                            acc = f.add(acc, f.mul(co, m));
                        }
                    }
                }
            }
        }
        acc
    }

    /// Specialise `x5 = xr`: 9^4 coefficients in x1..x4, index (((e1*9+e2)*9+e3)*9+e4).
    pub fn specialise(f: &Fp, c: &[u64], xr: u64) -> Vec<u64> {
        let mut pw = [1u64; D];
        for e in 1..D {
            pw[e] = f.mul(pw[e - 1], xr);
        }
        let n4 = D.pow(4);
        let mut out = vec![0u64; n4];
        for i in 0..n4 {
            let mut acc = 0u64;
            for e5 in 0..D {
                acc = f.add(acc, f.mul(c[i * D + e5], pw[e5]));
            }
            out[i] = acc;
        }
        bump(BUILD, (n4 * D) as u64);
        out
    }
}

// ----- F4 over F_p in four variables -----

mod f4 {
    use super::*;

    pub type Mono = u32; // e1 | e2<<8 | e3<<16 | e4<<24, each exponent < 128
    pub type Poly = Vec<(Mono, u32)>; // leading term first (grevlex)

    #[inline]
    pub fn exps(m: Mono) -> [u32; 4] {
        [m & 255, (m >> 8) & 255, (m >> 16) & 255, m >> 24]
    }
    #[inline]
    pub fn deg(m: Mono) -> u32 {
        let e = exps(m);
        e[0] + e[1] + e[2] + e[3]
    }
    /// Larger key = larger monomial in grevlex with x1 > x2 > x3 > x4.
    #[inline]
    pub fn gkey(m: Mono) -> u64 {
        let e = exps(m);
        ((e[0] + e[1] + e[2] + e[3]) as u64) << 32
            | ((255 - e[3]) as u64) << 24
            | ((255 - e[2]) as u64) << 16
            | ((255 - e[1]) as u64) << 8
            | (255 - e[0]) as u64
    }
    #[inline]
    pub fn divides(a: Mono, b: Mono) -> bool {
        const H: u32 = 0x8080_8080;
        ((b | H).wrapping_sub(a)) & H == H
    }
    #[inline]
    pub fn lcm(a: Mono, b: Mono) -> Mono {
        let (x, y) = (exps(a), exps(b));
        (x[0].max(y[0])) | (x[1].max(y[1])) << 8 | (x[2].max(y[2])) << 16 | (x[3].max(y[3])) << 24
    }
    pub fn coprime(a: Mono, b: Mono) -> bool {
        let (x, y) = (exps(a), exps(b));
        (0..4).all(|i| x[i] == 0 || y[i] == 0)
    }

    #[derive(Clone, Copy)]
    pub struct Bar {
        pub p: u64,
        m: u64,
    }
    impl Bar {
        pub fn new(p: u64) -> Bar {
            Bar {
                p,
                m: ((1u128 << 64) / p as u128) as u64,
            }
        }
        #[inline]
        pub fn red(&self, x: u64) -> u64 {
            let q = ((x as u128 * self.m as u128) >> 64) as u64;
            let r = x - q * self.p;
            if r >= self.p {
                r - self.p
            } else {
                r
            }
        }
    }

    pub fn monic(f: &Fp, mut p: Poly) -> Poly {
        if p.is_empty() {
            return p;
        }
        let inv = f.inv(p[0].1 as u64);
        for t in p.iter_mut() {
            t.1 = f.mul(t.1 as u64, inv) as u32;
        }
        p
    }
    pub fn sort_poly(p: &mut Poly) {
        p.sort_by(|a, b| gkey(b.0).cmp(&gkey(a.0)));
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Status {
        Done,
        Unit,
        Budget(&'static str),
    }
    pub struct Report {
        pub status: Status,
        pub steps: u32,
        pub max_deg: u32,
        pub madds: u64,
        pub max_rows: usize,
        pub max_cols: usize,
        pub peak_nnz: usize,
        pub basis_len: usize,
        pub zero_rows: u64,
        pub lms: Vec<Mono>,
        pub basis: Vec<Poly>,
        pub last_deg: u32,
    }

    pub struct Limits {
        pub madds: u64,
        pub nnz: usize,
        pub deadline: Option<Instant>,
    }

    struct Pair {
        i: usize,
        j: usize,
        l: Mono,
    }

    /// Gebauer-Moller update: add the pairs of the new element `h` (the last of `lms`),
    /// dropping those the product, chain (M) and B/F criteria discard.
    fn update(lms: &[Mono], pairs: &mut Vec<Pair>, h: usize) {
        let lh = lms[h];
        let mut c: Vec<(usize, Mono)> = (0..h).map(|g| (g, lcm(lh, lms[g]))).collect();
        let mut d: Vec<(usize, Mono)> = Vec::new();
        while let Some((g, l)) = c.pop() {
            if coprime(lh, lms[g])
                || (!c.iter().any(|&(_, l2)| divides(l2, l))
                    && !d.iter().any(|&(_, l2)| divides(l2, l)))
            {
                d.push((g, l));
            }
        }
        pairs.retain(|pr| {
            !(divides(lh, pr.l) && lcm(lms[pr.i], lh) != pr.l && lcm(lh, lms[pr.j]) != pr.l)
        });
        for (g, l) in d {
            if !coprime(lh, lms[g]) {
                pairs.push(Pair { i: g, j: h, l });
            }
        }
    }

    /// F4, normal selection strategy.  `input` are polynomials over F_p (not necessarily monic).
    pub fn run(p: u64, input: Vec<Poly>, lim: &Limits) -> Report {
        let f = Fp { p };
        let bar = Bar::new(p);
        let mut basis: Vec<Poly> = Vec::new();
        let mut lms: Vec<Mono> = Vec::new();
        let mut pairs: Vec<Pair> = Vec::new();
        let mut rep = Report {
            status: Status::Done,
            steps: 0,
            max_deg: 0,
            madds: 0,
            max_rows: 0,
            max_cols: 0,
            peak_nnz: 0,
            basis_len: 0,
            zero_rows: 0,
            lms: Vec::new(),
            basis: Vec::new(),
            last_deg: 0,
        };
        for mut q in input {
            sort_poly(&mut q);
            if q.is_empty() {
                continue;
            }
            let q = monic(&f, q);
            if q[0].0 == 0 {
                rep.status = Status::Unit;
                rep.basis = vec![q];
                rep.lms = vec![0];
                rep.basis_len = 1;
                return rep;
            }
            lms.push(q[0].0);
            basis.push(q);
            let h = basis.len() - 1;
            update(&lms, &mut pairs, h);
        }
        loop {
            if pairs.is_empty() {
                break;
            }
            let d = pairs.iter().map(|pr| deg(pr.l)).min().unwrap();
            rep.max_deg = rep.max_deg.max(d);
            rep.last_deg = d;
            let (sel, rest): (Vec<Pair>, Vec<Pair>) =
                pairs.drain(..).partition(|pr| deg(pr.l) == d);
            pairs = rest;
            rep.steps += 1;
            // rows: (multiplier, basis index)
            let mut row_spec: Vec<(Mono, usize)> = Vec::new();
            let mut seen_spec: HashSet<(Mono, usize)> = HashSet::new();
            for pr in &sel {
                for &g in &[pr.i, pr.j] {
                    let mult = pr.l - lms[g];
                    if seen_spec.insert((mult, g)) {
                        row_spec.push((mult, g));
                    }
                }
            }
            let n_pair_rows = row_spec.len();
            let mut lead_set: HashSet<Mono> = row_spec.iter().map(|&(m, g)| m + lms[g]).collect();
            let mut mono_seen: HashSet<Mono> = HashSet::new();
            let mut queue: Vec<Mono> = Vec::new();
            for &(m, g) in &row_spec {
                for t in &basis[g] {
                    let mm = t.0 + m;
                    if mono_seen.insert(mm) {
                        queue.push(mm);
                    }
                }
            }
            while let Some(m) = queue.pop() {
                if lead_set.contains(&m) {
                    continue;
                }
                if let Some(g) = (0..lms.len()).find(|&g| divides(lms[g], m)) {
                    let mult = m - lms[g];
                    lead_set.insert(m);
                    row_spec.push((mult, g));
                    for t in &basis[g] {
                        let mm = t.0 + mult;
                        if mono_seen.insert(mm) {
                            queue.push(mm);
                        }
                    }
                }
            }
            // columns
            let mut cols: Vec<Mono> = mono_seen.into_iter().collect();
            cols.sort_by(|a, b| gkey(*b).cmp(&gkey(*a)));
            let colidx: HashMap<Mono, u32> = cols
                .iter()
                .enumerate()
                .map(|(i, &m)| (m, i as u32))
                .collect();
            let ncols = cols.len();
            rep.max_cols = rep.max_cols.max(ncols);
            rep.max_rows = rep.max_rows.max(row_spec.len());
            // build sparse rows
            let build = |mult: Mono, g: usize| -> Vec<(u32, u32)> {
                basis[g]
                    .iter()
                    .map(|t| (colidx[&(t.0 + mult)], t.1))
                    .collect()
            };
            let mut pivots: Vec<Option<Vec<(u32, u32)>>> = vec![None; ncols];
            let mut nnz = 0usize;
            let mut todo: Vec<Vec<(u32, u32)>> = Vec::new();
            // reducers (indices >= n_pair_rows) first, then pair rows
            for &(m, g) in row_spec[n_pair_rows..].iter() {
                let r = build(m, g);
                nnz += r.len();
                let c = r[0].0 as usize;
                pivots[c] = Some(r);
            }
            for &(m, g) in row_spec[..n_pair_rows].iter() {
                let r = build(m, g);
                let c = r[0].0 as usize;
                if pivots[c].is_none() {
                    nnz += r.len();
                    pivots[c] = Some(r);
                } else {
                    todo.push(r);
                }
            }
            let mut acc = vec![0u64; ncols];
            let mut new_elems: Vec<Poly> = Vec::new();
            let mut budget_hit: Option<&'static str> = None;
            for (ri, row) in todo.into_iter().enumerate() {
                if ri % 32 == 0 {
                    if rep.madds > lim.madds {
                        budget_hit = Some("madds");
                        break;
                    }
                    if let Some(dl) = lim.deadline {
                        if Instant::now() > dl {
                            budget_hit = Some("deadline");
                            break;
                        }
                    }
                }
                for &(c, v) in &row {
                    acc[c as usize] = v as u64;
                }
                let mut c = row[0].0 as usize;
                let mut found: Option<usize> = None;
                while c < ncols {
                    let v = acc[c];
                    if v != 0 {
                        if let Some(pr) = &pivots[c] {
                            // pivot rows are monic: acc -= v * pr
                            let negv = p - v;
                            for &(k, w) in pr.iter() {
                                let kk = k as usize;
                                acc[kk] = bar.red(acc[kk] + negv * w as u64);
                            }
                            rep.madds += pr.len() as u64;
                        } else {
                            found = Some(c);
                            break;
                        }
                    }
                    c += 1;
                }
                match found {
                    None => {
                        rep.zero_rows += 1;
                        // acc is all zero from c.. (reduced); nothing to clear beyond what was touched
                        for x in acc.iter_mut() {
                            *x = 0;
                        }
                    }
                    Some(c) => {
                        let inv = f.inv(acc[c]);
                        let mut nr: Vec<(u32, u32)> = Vec::new();
                        for k in c..ncols {
                            if acc[k] != 0 {
                                nr.push((k as u32, bar.red(acc[k] * inv) as u32));
                                acc[k] = 0;
                            }
                        }
                        rep.madds += nr.len() as u64;
                        for x in acc.iter_mut() {
                            *x = 0;
                        }
                        nnz += nr.len();
                        let poly: Poly = nr.iter().map(|&(k, v)| (cols[k as usize], v)).collect();
                        pivots[c] = Some(nr);
                        new_elems.push(poly);
                        if nnz > lim.nnz {
                            budget_hit = Some("nnz");
                            break;
                        }
                    }
                }
            }
            rep.peak_nnz = rep.peak_nnz.max(nnz);
            // adopt new elements even on a budget stop (they are valid ideal members)
            for q in new_elems {
                if q[0].0 == 0 {
                    rep.status = Status::Unit;
                    rep.basis = vec![q];
                    rep.lms = vec![0];
                    rep.basis_len = 1;
                    return rep;
                }
                lms.push(q[0].0);
                basis.push(q);
                let h = basis.len() - 1;
                update(&lms, &mut pairs, h);
            }
            if let Some(why) = budget_hit {
                rep.status = Status::Budget(why);
                break;
            }
            if rep.madds > lim.madds {
                rep.status = Status::Budget("madds");
                break;
            }
        }
        rep.basis_len = basis.len();
        rep.lms = lms;
        rep.basis = basis;
        rep
    }

    /// Number of standard monomials in the box `e_i < b` (the quotient dimension when
    /// `x_i^b` is in the ideal).
    pub fn quotient_dim(lms: &[Mono], b: u32) -> u64 {
        let mut count = 0u64;
        for e1 in 0..b {
            for e2 in 0..b {
                for e3 in 0..b {
                    for e4 in 0..b {
                        let m = e1 | e2 << 8 | e3 << 16 | e4 << 24;
                        if !lms.iter().any(|&l| divides(l, m)) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn eval(f: &Fp, q: &Poly, x: [u64; 4]) -> u64 {
        let mut acc = 0u64;
        for &(m, c) in q {
            let e = exps(m);
            let mut t = c as u64;
            for i in 0..4 {
                t = f.mul(t, f.pow(x[i], e[i] as u64));
            }
            acc = f.add(acc, t);
        }
        acc
    }
}

/// The F4 input for one target: `S5(., ., ., ., xR)` reduced modulo the membership polynomials,
/// together with `L(x_i)`.
fn build_f4_input(f: &Fp, s5c: &[u64], base: &Base, xr: u64) -> Vec<f4::Poly> {
    let b = base.xs.len();
    let d = s5::D;
    let mut c = s5::specialise(f, s5c, xr);
    // L(X) = prod (X - x), degree b, coefficients l[0..=b]
    let mut l = vec![1u64];
    for &x in &base.xs {
        let mut nl = vec![0u64; l.len() + 1];
        for (i, &v) in l.iter().enumerate() {
            nl[i + 1] = f.add(nl[i + 1], v);
            nl[i] = f.sub(nl[i], f.mul(v, x));
        }
        l = nl;
    }
    if b < d {
        // reduce each axis modulo L: x^e = x^(e-b) x^b, x^b = -sum_{j<b} l_j x^j
        let mut stride = 1usize;
        for _axis in 0..4 {
            let mut cnt = 0u64;
            for start in 0..c.len() {
                if (start / stride) % d != 0 {
                    continue;
                }
                for e in (b..d).rev() {
                    let co = c[start + e * stride];
                    if co == 0 {
                        continue;
                    }
                    c[start + e * stride] = 0;
                    for j in 0..b {
                        let idx = start + (e - b + j) * stride;
                        c[idx] = f.sub(c[idx], f.mul(co, l[j]));
                        cnt += 1;
                    }
                }
            }
            bump(BUILD, cnt);
            stride *= d;
        }
    }
    let mut main: f4::Poly = Vec::new();
    for e1 in 0..d {
        for e2 in 0..d {
            for e3 in 0..d {
                for e4 in 0..d {
                    let v = c[((e1 * d + e2) * d + e3) * d + e4];
                    if v != 0 {
                        main.push(((e1 | e2 << 8 | e3 << 16 | e4 << 24) as u32, v as u32));
                    }
                }
            }
        }
    }
    let mut input = vec![main];
    for var in 0..4 {
        let mut q: f4::Poly = Vec::new();
        for (e, &co) in l.iter().enumerate() {
            if co != 0 {
                q.push(((e as u32) << (8 * var), co as u32));
            }
        }
        input.push(q);
    }
    input
}

struct BOutcome {
    status: String,
    dim: Option<u64>,
    steps: u32,
    max_deg: u32,
    madds: u64,
    max_rows: usize,
    max_cols: usize,
    peak_nnz: usize,
    zero_rows: u64,
    ms: f64,
    planted_ok: Option<bool>,
}

fn run_arm_b(
    f: &Fp,
    s5c: &[u64],
    base: &Base,
    xr: u64,
    lim: &f4::Limits,
    planted: Option<[u64; 4]>,
) -> BOutcome {
    let t0 = Instant::now();
    let input = build_f4_input(f, s5c, base, xr);
    let rep = f4::run(f.p, input, lim);
    bump(MADD, rep.madds);
    let b = base.xs.len() as u32;
    let (status, dim) = match &rep.status {
        f4::Status::Unit => ("done".to_string(), Some(0u64)),
        f4::Status::Done => ("done".to_string(), Some(f4::quotient_dim(&rep.lms, b))),
        f4::Status::Budget(w) => (format!("budget-{w}"), None),
    };
    let planted_ok = planted.map(|x| {
        // every basis element vanishes at the planted point
        rep.status == f4::Status::Done && rep.basis.iter().all(|q| f4::eval(f, q, x) == 0)
    });
    BOutcome {
        status,
        dim,
        steps: rep.steps,
        max_deg: rep.max_deg,
        madds: rep.madds,
        max_rows: rep.max_rows,
        max_cols: rep.max_cols,
        peak_nnz: rep.peak_nnz,
        zero_rows: rep.zero_rows,
        ms: t0.elapsed().as_secs_f64() * 1e3,
        planted_ok,
    }
}

// ───────────────────────── targets ─────────────────────────

/// A random target `R = [k]G` whose abscissa is not in the base (a base point is a 1-decomposition).
fn random_target(cell: &Cell, base: &Base, rng: &mut Rng) -> Pt {
    let cv = cell.curve();
    loop {
        let k = 1 + rng.below(cell.n - 1);
        let r = cv.mul(k, cell.g());
        if !r.inf && !(r.x >= 1 && r.x <= base.bound && base.xs.binary_search(&r.x).is_ok()) {
            return r;
        }
    }
}

/// A planted target `R = e1 P1 + .. + e4 P4` for random distinct-x base points.
fn planted_target(cv: &Curve, base: &Base, rng: &mut Rng) -> (Pt, [usize; 4]) {
    loop {
        let b = base.xs.len();
        assert!(b >= 3, "base too small for a planted target");
        let mut idx = [0usize; 4];
        let mut cs = [0usize; 4];
        // with fewer than four abscissae the first point is used twice
        let distinct = b.min(4);
        for t in 0..distinct {
            loop {
                let c = rng.below(b as u64) as usize;
                if !cs[..t].contains(&c) {
                    cs[t] = c;
                    break;
                }
            }
            idx[t] = 2 * cs[t] + rng.below(2) as usize;
        }
        if distinct < 4 {
            idx[3] = idx[0];
        }
        let mut s = INF;
        for &i in &idx {
            s = cv.add(s, base.pts[i]);
        }
        if !s.inf && !(s.x >= 1 && s.x <= base.bound && base.xs.binary_search(&s.x).is_ok()) {
            return (s, idx);
        }
    }
}

/// The r-adding walk's jump table: 16 jumps `T_j = alpha_j G + beta_j Q` whose scalars have four
/// random bits set each, built from the doubling chains of `G` and `Q` (about `2 bits + 16 * 8`
/// counted additions).
fn make_jumps(cv: &Curve, cell: &Cell, q: Pt, rng: &mut Rng) -> Vec<(u64, u64, Pt)> {
    let n = cell.n;
    let bits = cell.bits as usize;
    let mut gp = Vec::with_capacity(bits);
    let mut qp = Vec::with_capacity(bits);
    let (mut x, mut y) = (cell.g(), q);
    for _ in 0..bits {
        gp.push(x);
        qp.push(y);
        x = cv.add(x, x);
        y = cv.add(y, y);
    }
    let mut out = Vec::with_capacity(16);
    for _ in 0..16 {
        let pick = |rng: &mut Rng| {
            let mut set = [usize::MAX; 4];
            for t in 0..4 {
                loop {
                    let i = rng.below(bits as u64) as usize;
                    if !set[..t].contains(&i) {
                        set[t] = i;
                        break;
                    }
                }
            }
            set
        };
        let (sa, sb) = (pick(rng), pick(rng));
        let (mut al, mut be, mut t) = (0u64, 0u64, INF);
        for &i in &sa {
            al += 1u64 << i;
            t = cv.add(t, gp[i]);
        }
        for &i in &sb {
            be += 1u64 << i;
            t = cv.add(t, qp[i]);
        }
        out.push((al % n, be % n, t));
    }
    out
}

// ───────────────────────── rho (the matched reference) ─────────────────────────

struct RhoResult {
    setup_gadd: u64,
    ok: bool,
    counts: [u64; NC],
    walks: u64,
    ms: f64,
}

/// Pollard rho, r-adding walk (32 jumps), van Oorschot-Wiener distinguished points, one thread.
/// Counted in the same unit: every walk step is one group addition, every step one DP test, every
/// stored or probed DP one table operation.
fn rho_run(cell: &Cell, d: u64, seed: u64) -> RhoResult {
    let cv = cell.curve();
    let g = cell.g();
    let q = cv.mul(d, g); // the instance, not charged
    let n = cell.n;
    let t0 = Instant::now();
    let s0 = snap();
    let mut rng = Rng(seed);
    const J: usize = 16;
    let jumps = make_jumps(&cv, cell, q, &mut rng);
    // start points: a chain of additions of random jump-like points
    let theta = ((cell.bits / 4).max(2)) as u32;
    let mask = (1u64 << theta) - 1;
    let cap = 20 * (1u64 << theta) + 64;
    let mut table: HashMap<u64, (u64, u64, u64)> = HashMap::new();
    let (mut a, mut b) = (1 + rng.below(n - 1), 1 + rng.below(n - 1));
    let mut r = cv.add(cv.mul(a, g), cv.mul(b, q));
    let setup_gadd = dsnap(&s0, &snap())[GADD];
    let mut walks = 0u64;
    let mut found = None;
    'outer: loop {
        // fresh start: add a random jump
        let j = rng.below(J as u64) as usize;
        r = cv.add(r, jumps[j].2);
        a = (a + jumps[j].0) % n;
        b = (b + jumps[j].1) % n;
        let j2 = rng.below(J as u64) as usize;
        r = cv.add(r, jumps[j2].2);
        a = (a + jumps[j2].0) % n;
        b = (b + jumps[j2].1) % n;
        walks += 1;
        let (mut wa, mut wb, mut wr) = (a, b, r);
        let mut steps = 0u64;
        loop {
            if wr.inf {
                break;
            }
            let j = part(wr.x);
            wr = cv.add(wr, jumps[j].2);
            wa = (wa + jumps[j].0) % n;
            wb = (wb + jumps[j].1) % n;
            steps += 1;
            bump(HASHT, 1);
            if !wr.inf && wr.x & mask == 0 {
                bump(LOOKUP, 1);
                if let Some(&(pa, pb, py)) = table.get(&wr.x) {
                    // same x: wr = +-point; a + b d = +- (pa + pb d)
                    let same = py == wr.y;
                    let (num, den) = if same {
                        ((pa + n - wa) % n, (wb + n - pb) % n)
                    } else {
                        ((n - (wa + pa) % n) % n, (wb + pb) % n)
                    };
                    if den != 0 {
                        let cand = num * Fp { p: n }.inv(den) % n;
                        if cv.mul(cand, g) == q {
                            found = Some(cand);
                            break 'outer;
                        }
                    }
                } else {
                    bump(INSERT, 1);
                    table.insert(wr.x, (wa, wb, wr.y));
                }
                break;
            }
            if steps > cap {
                break;
            }
        }
        // continue the chain of starting points from the end of the walk
        if !wr.inf && steps <= cap {
            // keep (a, b, r) as the chain state; do not reuse the DP (avoids merging into it)
        }
        if walks > 50_000_000 {
            break;
        }
    }
    let counts = dsnap(&s0, &snap());
    RhoResult {
        setup_gadd,
        ok: found == Some(d),
        counts,
        walks,
        ms: t0.elapsed().as_secs_f64() * 1e3,
    }
}

// ───────────────────────── full index calculus ─────────────────────────

struct IcResult {
    ok: bool,
    attempts: u64,
    relations: u64,
    rank: usize,
    cols: usize,
    setup: [u64; NC],
    online: [u64; NC],
    verify: [u64; NC],
    ms: f64,
    hits: u64,
    note: String,
}

fn sign_of(i: usize) -> i64 {
    if i % 2 == 0 {
        1
    } else {
        -1
    }
}

/// One complete IC run with arm `arm` ('a' triple table, 'b' F4) over a base of `bxk` abscissae.
#[allow(clippy::too_many_arguments)]
fn full_ic(
    cell: &Cell,
    bxk: usize,
    arm: char,
    planted: u64,
    seed: u64,
    s5c: Option<&[u64]>,
    lim: &f4::Limits,
    max_attempts: u64,
) -> IcResult {
    let cv = cell.curve();
    let g = cell.g();
    let n = cell.n;
    let f = cv.f;
    let q = cv.mul(planted, g); // instance, uncharged
    let t0 = Instant::now();
    let mut rng = Rng(seed);
    // ---- setup (everything target-independent, and the walk's jump table)
    let s_setup0 = snap();
    let base = Base::build_k(&cv, bxk);
    let bx = base.xs.len();
    let cols = bx + 1;
    let table = if arm == 'a' {
        Some(TripleTable::build(&cv, &base))
    } else {
        None
    };
    const J: usize = 16;
    let jumps = make_jumps(&cv, cell, q, &mut rng);
    let (mut a, mut b) = (1 + rng.below(n - 1), 1 + rng.below(n - 1));
    let mut r = cv.add(cv.mul(a, g), cv.mul(b, q));
    let setup = dsnap(&s_setup0, &snap());
    // ---- online: relation phase
    let s_on0 = snap();
    let mut m = Matrix::new(n, cols);
    let (mut attempts, mut relations, mut hits) = (0u64, 0u64, 0u64);
    let mut note = String::new();
    while m.rank < cols && attempts < max_attempts {
        // next target: one group addition of a uniformly chosen jump (a deterministic jump choice
        // would enter a cycle)
        let j = rng.below(J as u64) as usize;
        r = cv.add(r, jumps[j].2);
        a = (a + jumps[j].0) % n;
        b = (b + jumps[j].1) % n;
        attempts += 1;
        if r.inf {
            continue;
        }
        let dec: Option<[usize; 4]> = match arm {
            'a' => table.as_ref().unwrap().decompose(&cv, &base, r),
            _ => {
                // arm b: F4 decides; roots are extracted on the grid only when the quotient is not 0
                let out = run_arm_b_extract(&f, s5c.unwrap(), &base, &cv, r, lim);
                match out {
                    Ok(v) => v,
                    Err(why) => {
                        note = why;
                        None
                    }
                }
            }
        };
        if let Some(dd) = dec {
            if !verify_decomposition(&cv, &base, &dd, r) {
                note = "decomposition failed verification".into();
                continue;
            }
            hits += 1;
            // row: sum_i s_i l_{x_i} - b d = a   (mod n)
            let mut row = vec![0u64; cols];
            for &i in &dd {
                let c = i / 2;
                let sgn = sign_of(i);
                row[c] = if sgn > 0 {
                    (row[c] + 1) % n
                } else {
                    (row[c] + n - 1) % n
                };
            }
            row[bx] = (n - b) % n;
            if m.add_row(row, a % n) {
                relations += 1;
            }
        }
    }
    let online = dsnap(&s_on0, &snap());
    // ---- solve and verify
    let s_v0 = snap();
    let mut ok = false;
    if let Some(sol) = m.solve() {
        let d = sol[bx];
        ok = cv.mul(d, g) == q && d == planted;
    }
    let verify = dsnap(&s_v0, &snap());
    IcResult {
        ok,
        attempts,
        relations,
        rank: m.rank,
        cols,
        setup,
        online,
        verify,
        ms: t0.elapsed().as_secs_f64() * 1e3,
        hits,
        note,
    }
}

/// Arm (b) inside the IC loop: run F4; if the quotient is non-trivial, find a grid root by
/// evaluating the basis on the grid (charged as `build` multiplications) and lift to signed points
/// (charged group additions).  `Ok(None)` for a refutation, `Err` if F4 stopped on a limit.
fn run_arm_b_extract(
    f: &Fp,
    s5c: &[u64],
    base: &Base,
    cv: &Curve,
    r: Pt,
    lim: &f4::Limits,
) -> Result<Option<[usize; 4]>, String> {
    let input = build_f4_input(f, s5c, base, r.x);
    let rep = f4::run(f.p, input, lim);
    bump(MADD, rep.madds);
    match rep.status {
        f4::Status::Unit => Ok(None),
        f4::Status::Budget(w) => Err(format!("f4 stopped: {w}")),
        f4::Status::Done => {
            let b = base.xs.len();
            if f4::quotient_dim(&rep.lms, b as u32) == 0 {
                return Ok(None);
            }
            // roots on the grid: evaluate the (small) basis
            let mut evals = 0u64;
            for c1 in 0..b {
                for c2 in c1..b {
                    for c3 in c2..b {
                        for c4 in c3..b {
                            let xs = [base.xs[c1], base.xs[c2], base.xs[c3], base.xs[c4]];
                            let mut zero = true;
                            for q in &rep.basis {
                                evals += (q.len() * 4) as u64;
                                if f4::eval(f, q, xs) != 0 {
                                    zero = false;
                                    break;
                                }
                            }
                            if zero {
                                bump(BUILD, evals);
                                // lift: signs on the points of columns c1..c4, e1 = +
                                let cs = [c1, c2, c3, c4];
                                for mask in 0..8u32 {
                                    let mut idx = [0usize; 4];
                                    for t in 0..4 {
                                        let neg =
                                            if t == 0 { 0 } else { (mask >> (t - 1)) & 1 } as usize;
                                        idx[t] = 2 * cs[t] + neg;
                                    }
                                    let mut s = INF;
                                    for &i in &idx {
                                        s = cv.add(s, base.pts[i]);
                                    }
                                    if s == r {
                                        return Ok(Some(idx));
                                    }
                                    if s == cv.neg(r) {
                                        let mut idn = idx;
                                        for v in idn.iter_mut() {
                                            *v ^= 1;
                                        }
                                        return Ok(Some(idn));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            bump(BUILD, evals);
            Err("quotient non-trivial but no sorted-tuple root lifts".into())
        }
    }
}

// ───────────────────────── arm (d): the lattice reach ─────────────────────────

/// Ideal determinant condition of the Jochemsz-May lattice of level `l` for a polynomial with
/// per-variable degree `d` in `m` variables: `delta_max = sum k / sum |alpha|`.
fn reach_row(m: usize, d: u64, l: u64) -> (u64, f64, f64, f64) {
    // monomials alpha in [0, d*l]^m ; k = min(l, min_i floor(alpha_i / d))
    let side = d * l + 1;
    let dim = side.pow(m as u32);
    // by symmetry: sum over alpha of k and of |alpha|
    let mut sum_k = 0f64;
    let mut sum_a = 0f64;
    let mut idx = vec![0u64; m];
    let mut count = 0u64;
    loop {
        let mut k = l;
        let mut s = 0u64;
        for &a in &idx {
            k = k.min(a / d);
            s += a;
        }
        sum_k += k as f64;
        sum_a += s as f64;
        count += 1;
        // increment
        let mut pos = 0;
        loop {
            if pos == m {
                debug_assert_eq!(count, dim);
                return (dim, sum_k, sum_a, sum_k / sum_a);
            }
            idx[pos] += 1;
            if idx[pos] < side {
                break;
            }
            idx[pos] = 0;
            pos += 1;
        }
    }
}

fn cmd_reach(args: &[String]) {
    let mut out = Out::new(arg(args, "--out"));
    for (m, d, lmax) in [(2usize, 2u64, 40u64), (3, 4, 12), (4, 8, 6)] {
        for l in 1..=lmax {
            let (dim, sk, sa, delta) = reach_row(m, d, l);
            let asym = 2.0 / (m as f64 * (m as f64 + 1.0) * d as f64);
            let mut line = format!(
                "{{\"kind\":\"reach\",\"m\":{m},\"summands\":{m},\"poly\":\"S{}\",\"d\":{d},\"level\":{l},\"dim\":{dim},\"sum_k\":{sk},\"sum_alpha\":{sa},\"delta_max\":{delta:.8},\"delta_asymptote\":{asym:.8}",
                m + 1
            );
            // per-cell B_max = floor(p^delta) and the log2 p needed for B >= 2 and for B = p^(1/4)
            line.push_str(",\"bmax\":{");
            let mut first = true;
            for c in [16u32, 20, 24, 28] {
                let p = 2f64.powi(c as i32);
                if !first {
                    line.push(',');
                }
                first = false;
                line.push_str(&format!("\"{}\":{:.6}", c, p.powf(delta)));
            }
            line.push_str(&format!("}},\"log2p_for_b2\":{:.1}}}", 1.0 / delta));
            out.line(&line);
        }
    }
}

// ───────────────────────── reuse: the committed PKM rows (arm c) ─────────────────────────

fn json_field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let pat = format!("\"{name}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim_matches('"'))
}

fn cmd_reuse(args: &[String]) {
    let mut out = Out::new(arg(args, "--out"));
    let list = arg(args, "--files").expect("--files f1,f2,...");
    let files: Vec<&str> = list.split(',').collect();
    for f in files {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("read {f}: {e}"));
        for line in text.lines() {
            if json_field(line, "m") != Some("4") {
                continue;
            }
            let g = |k: &str| json_field(line, k).unwrap_or("null");
            out.line(&format!(
                "{{\"kind\":\"reused-pkm\",\"source\":\"{f}\",\"family\":\"{}\",\"m\":4,\"N\":{},\"t\":{},\"p\":{},\"target_index\":{},\"muladds\":{},\"degree\":{},\"steps\":{},\"max_cols\":{},\"inconsistent\":{},\"timed_out\":{},\"ms\":{}}}",
                g("kind"), g("N"), g("t"), g("p"), g("target_index"), g("muladds"), g("solving_degree_max"),
                g("steps"), g("max_cols"), g("inconsistent"), g("timed_out"), g("ms")
            ));
        }
    }
}

// ───────────────────────── commands: tdec ─────────────────────────

fn bx_for(cell: &Cell, args: &[String]) -> usize {
    match arg(args, "--bx") {
        Some(v) => v.parse().expect("--bx"),
        None => cell_bx(cell),
    }
}

fn cmd_tdec(args: &[String]) {
    let bits = arg_u64(args, "--bits", 16) as u32;
    let cell = cell_for(bits);
    let cv = cell.curve();
    let f = cv.f;
    let bxk = bx_for(&cell, args);
    let targets = arg_u64(args, "--targets", 16);
    let planted_n = arg_u64(args, "--planted", 0);
    let seed = arg_u64(args, "--seed", 20261018);
    let arms = arg(args, "--arms").unwrap_or("a");
    let check = flag(args, "--check");
    let t0i = arg_u64(args, "--t0", 0) as usize;
    let bcap = arg_u64(args, "--bcap-madds", 2_000_000_000);
    let bdead = arg_u64(args, "--bdeadline-s", 120);
    let bnnz = arg_u64(args, "--bcap-nnz", 120_000_000) as usize;
    let mut out = Out::new(arg(args, "--out"));
    let mut rng = Rng(seed ^ (bits as u64) << 32 ^ bxk as u64);
    let base = Base::build_k(&cv, bxk);
    let bound = base.bound;
    let bx = base.xs.len();
    out.line(&format!(
        "{{\"kind\":\"cell\",\"bits\":{bits},\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"B\":{bound},\"bx\":{bx},\"signed\":{},\"seed\":{seed}}}",
        cell.p, cell.a, cell.b, cell.n, base.nsigned()
    ));
    // targets shared by all arms (paired)
    let mut tg: Vec<(Pt, Option<[usize; 4]>, &str)> = Vec::new();
    for _ in 0..targets {
        tg.push((random_target(&cell, &base, &mut rng), None, "random"));
    }
    for _ in 0..planted_n {
        let (r, idx) = planted_target(&cv, &base, &mut rng);
        tg.push((r, Some(idx), "planted"));
    }
    let pairt = if check {
        Some(PairTable::build(&cv, &base))
    } else {
        None
    };
    let t1i = arg_u64(args, "--t1", tg.len() as u64) as usize;
    for arm in arms.split(',') {
        match arm {
            "a" => {
                let s0 = snap();
                let t0 = Instant::now();
                let table = TripleTable::build(&cv, &base);
                let setup = dsnap(&s0, &snap());
                out.line(&format!(
                    "{{\"kind\":\"setup\",\"arm\":\"a\",\"bits\":{bits},\"B\":{bound},\"bx\":{bx},\"entries\":{},\"counts\":{},\"ms\":{:.3}}}",
                    table.map.len(), counts_json(&setup), t0.elapsed().as_secs_f64() * 1e3
                ));
                for (ti, (r, pl, kind)) in tg.iter().enumerate() {
                    if ti < t0i || ti >= t1i {
                        continue;
                    }
                    let s0 = snap();
                    let t0 = Instant::now();
                    let res = table.decompose(&cv, &base, *r);
                    let d = dsnap(&s0, &snap());
                    let ms = t0.elapsed().as_secs_f64() * 1e3;
                    let valid = res.map(|dd| verify_decomposition(&cv, &base, &dd, *r));
                    let cross = pairt
                        .as_ref()
                        .map(|pt| !pt.decompositions(&cv, *r).is_empty());
                    out.line(&format!(
                        "{{\"kind\":\"tdec\",\"arm\":\"a\",\"bits\":{bits},\"B\":{bound},\"bx\":{bx},\"target\":{ti},\"tkind\":\"{kind}\",\"hit\":{},\"valid\":{},\"crosscheck_decomposable\":{},\"planted\":{},\"counts\":{},\"native\":{},\"ms\":{ms:.4}}}",
                        res.is_some(),
                        valid.map(|v| v.to_string()).unwrap_or("null".into()),
                        cross.map(|v| v.to_string()).unwrap_or("null".into()),
                        pl.is_some(),
                        counts_json(&d),
                        native_total(&d)
                    ));
                }
            }
            "b" => {
                let t0 = Instant::now();
                let s0 = snap();
                let s5c = s5::s5_poly(&f, cell.a, cell.b);
                let setup = dsnap(&s0, &snap());
                out.line(&format!(
                    "{{\"kind\":\"setup\",\"arm\":\"b\",\"bits\":{bits},\"B\":{bound},\"bx\":{bx},\"counts\":{},\"ms\":{:.3},\"note\":\"S5 interpolated on 9^5 nodes; evaluation by resultants is uncounted setup\"}}",
                    counts_json(&setup), t0.elapsed().as_secs_f64() * 1e3
                ));
                for (ti, (r, pl, kind)) in tg.iter().enumerate() {
                    if ti < t0i || ti >= t1i {
                        continue;
                    }
                    let lim = f4::Limits {
                        madds: bcap,
                        nnz: bnnz,
                        deadline: Some(Instant::now() + std::time::Duration::from_secs(bdead)),
                    };
                    let planted_x = pl.map(|idx| {
                        let mut x = [0u64; 4];
                        for t in 0..4 {
                            x[t] = base.pts[idx[t]].x;
                        }
                        x
                    });
                    let s0 = snap();
                    let o = run_arm_b(&f, &s5c, &base, r.x, &lim, planted_x);
                    let d = dsnap(&s0, &snap());
                    let brute = if check && bx <= 12 && o.dim.is_some() {
                        Some(brute_grid_roots(&cv, &base, *r))
                    } else {
                        None
                    };
                    out.line(&format!(
                        "{{\"kind\":\"tdec\",\"arm\":\"b\",\"bits\":{bits},\"B\":{bound},\"bx\":{bx},\"target\":{ti},\"tkind\":\"{kind}\",\"status\":\"{}\",\"dim\":{},\"brute_roots\":{},\"planted_vanishes\":{},\"steps\":{},\"max_degree\":{},\"max_rows\":{},\"max_cols\":{},\"peak_nnz\":{},\"zero_rows\":{},\"counts\":{},\"native\":{},\"ms\":{:.3}}}",
                        o.status,
                        o.dim.map(|v| v.to_string()).unwrap_or("null".into()),
                        brute.map(|v| v.to_string()).unwrap_or("null".into()),
                        o.planted_ok.map(|v| v.to_string()).unwrap_or("null".into()),
                        o.steps, o.max_deg, o.max_rows, o.max_cols, o.peak_nnz, o.zero_rows,
                        counts_json(&d), native_total(&d), o.ms
                    ));
                }
            }
            other => panic!("unknown arm {other}"),
        }
    }
}

fn cmd_fullic(args: &[String]) {
    let bits = arg_u64(args, "--bits", 16) as u32;
    let cell = cell_for(bits);
    let cv = cell.curve();
    let bxk = bx_for(&cell, args);
    let arm = arg(args, "--arm").unwrap_or("a").chars().next().unwrap();
    let runs = arg_u64(args, "--runs", 16);
    let run0 = arg_u64(args, "--run0", 0);
    let seed = arg_u64(args, "--seed", 20261018);
    let max_att = arg_u64(args, "--max-attempts", 5_000_000);
    let bcap = arg_u64(args, "--bcap-madds", 2_000_000_000);
    let bdead = arg_u64(args, "--bdeadline-s", 120);
    let mut out = Out::new(arg(args, "--out"));
    let s5c = if arm == 'b' {
        Some(s5::s5_poly(&cv.f, cell.a, cell.b))
    } else {
        None
    };
    for run in run0..run0 + runs {
        let planted =
            1 + Rng(seed ^ ((bits as u64) << 32) ^ 0xF011_1C ^ run.wrapping_mul(0x9E37_79B9))
                .below(cell.n - 1);
        let lim = f4::Limits {
            madds: bcap,
            nnz: 120_000_000,
            deadline: Some(Instant::now() + std::time::Duration::from_secs(bdead)),
        };
        let res = full_ic(
            &cell,
            bxk,
            arm,
            planted,
            seed + run * 7919 + 1,
            s5c.as_deref(),
            &lim,
            max_att,
        );
        let tot = |a: &[u64; NC], b: &[u64; NC], c: &[u64; NC]| {
            let mut t = [0u64; NC];
            for i in 0..NC {
                t[i] = a[i] + b[i] + c[i];
            }
            t
        };
        let all = tot(&res.setup, &res.online, &res.verify);
        out.line(&format!(
            "{{\"kind\":\"fullic\",\"arm\":\"{arm}\",\"bits\":{bits},\"bx\":{},\"run\":{run},\"ok\":{},\"attempts\":{},\"hits\":{},\"relations\":{},\"rank\":{},\"cols\":{},\"setup\":{},\"online\":{},\"verify\":{},\"total\":{},\"gae_total\":{},\"native_total\":{},\"n\":{},\"ms\":{:.1},\"note\":\"{}\"}}",
            res.cols - 1, res.ok, res.attempts, res.hits, res.relations, res.rank, res.cols,
            counts_json(&res.setup), counts_json(&res.online), counts_json(&res.verify), counts_json(&all),
            all[GADD], native_total(&all), cell.n, res.ms, res.note
        ));
    }
}

fn cmd_rho(args: &[String]) {
    let bits = arg_u64(args, "--bits", 16) as u32;
    let cell = cell_for(bits);
    let runs = arg_u64(args, "--runs", 16);
    let seed = arg_u64(args, "--seed", 20261018);
    let mut out = Out::new(arg(args, "--out"));
    for run in 0..runs {
        let d = 1 + Rng(seed ^ ((bits as u64) << 32) ^ 0x4410 ^ run.wrapping_mul(0x9E37_79B9))
            .below(cell.n - 1);
        let res = rho_run(&cell, d, seed + run * 104729 + 3);
        out.line(&format!(
            "{{\"kind\":\"rho\",\"bits\":{bits},\"run\":{run},\"ok\":{},\"setup_gadd\":{},\"walks\":{},\"counts\":{},\"gae\":{},\"native\":{},\"n\":{},\"sqrt_pi_n_over_2\":{:.1},\"ms\":{:.1}}}",
            res.ok, res.setup_gadd, res.walks, counts_json(&res.counts), res.counts[GADD], native_total(&res.counts), cell.n,
            (std::f64::consts::PI * cell.n as f64 / 2.0).sqrt(), res.ms
        ));
    }
}

fn cmd_find_curve(args: &[String]) {
    let bits = arg_u64(args, "--bits", 16) as u32;
    let seed = arg_u64(args, "--seed", 20261018);
    let t0 = Instant::now();
    let c = find_curve(bits, seed);
    println!(
        "    Cell {{ bits: {}, p: {}, a: {}, b: {}, n: {}, gx: {}, gy: {} }}, // {:.1}s",
        c.bits,
        c.p,
        c.a,
        c.b,
        c.n,
        c.gx,
        c.gy,
        t0.elapsed().as_secs_f64()
    );
}

// ───────────────────────── summarize ─────────────────────────

fn jnum(line: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    rest[..end].trim().parse().ok()
}
fn jstr<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":\"");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    Some(&rest[..rest.find('"')?])
}
fn jsec<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":{{");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    Some(&rest[..rest.find('}')?])
}
fn jbool(line: &str, key: &str) -> Option<bool> {
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
fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        f64::NAN
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}
/// OLS slope of y on x with a 95% t interval (None below three points).
fn ols(xs: &[f64], ys: &[f64]) -> Option<(f64, f64, f64)> {
    let n = xs.len();
    if n < 3 {
        return None;
    }
    let (mx, my) = (mean(xs), mean(ys));
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = sxy / sxx;
    let icpt = my - slope * mx;
    let sse: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (y - icpt - slope * x).powi(2))
        .sum();
    let se = (sse / (n as f64 - 2.0) / sxx).sqrt();
    const T: [f64; 10] = [
        12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228,
    ];
    let t = if n - 2 <= 10 { T[n - 3] } else { 2.0 };
    Some((slope, slope - t * se, slope + t * se))
}

fn cmd_summarize(args: &[String]) {
    let dir = arg(args, "--dir").unwrap_or("research/blindspots_20261018/14-prime-m4-cells/raw");
    let mut out = Out::new(arg(args, "--out"));
    let mut lines: Vec<String> = Vec::new();
    let mut names: Vec<_> = std::fs::read_dir(dir)
        .expect("read dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    names.sort();
    for pth in names {
        if pth.extension().map(|e| e == "jsonl").unwrap_or(false) {
            let t = std::fs::read_to_string(&pth).unwrap();
            for l in t.lines() {
                if l.starts_with('{') {
                    lines.push(l.to_string());
                }
            }
        }
    }
    let ws = [0.0, 0.1, 1.0];
    let cellp = |bits: u32| cell_for(bits);
    out.line("# Item 14 summary (generated by `summarize`; numbers only, no verdict)\n");
    // ---- rho
    out.line("## Matched rho (measured, same unit)\n");
    out.line("| bits | runs | ok | mean GAE | mean native | mean setup GAE | sqrt(pi n/2) | GAE / sqrt(pi n/2) | rho(w=0) | rho(w=0.1) | rho(w=1) |");
    out.line("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut rho_w: HashMap<u32, [f64; 3]> = HashMap::new();
    for bits in [16u32, 20, 24, 28] {
        let r: Vec<&String> = lines
            .iter()
            .filter(|l| jstr(l, "kind") == Some("rho") && jnum(l, "bits") == Some(bits as f64))
            .collect();
        if r.is_empty() {
            continue;
        }
        let gae: Vec<f64> = r.iter().map(|l| jnum(l, "gae").unwrap()).collect();
        let nat: Vec<f64> = r.iter().map(|l| jnum(l, "native").unwrap()).collect();
        let setup: Vec<f64> = r
            .iter()
            .map(|l| jnum(l, "setup_gadd").unwrap_or(0.0))
            .collect();
        let ok = r.iter().filter(|l| jbool(l, "ok") == Some(true)).count();
        let sq = (std::f64::consts::PI * cellp(bits).n as f64 / 2.0).sqrt();
        let rw = [0, 1, 2].map(|i| mean(&gae) + ws[i] * mean(&nat));
        rho_w.insert(bits, rw);
        out.line(&format!("| {bits} | {} | {ok} | {:.1} | {:.1} | {:.1} | {:.1} | {:.3} | {:.1} | {:.1} | {:.1} |", r.len(), mean(&gae), mean(&nat), mean(&setup), sq, mean(&gae) / sq, rw[0], rw[1], rw[2]));
    }
    // ---- arm a tdec
    out.line("\n## Arm (a): T_dec per attempt (online, one oracle call on a random target)\n");
    out.line("| bits | p | bx | signed | targets | hit rate | planted found | cross-check mismatches | mean GAE | mean lookups | T(0) | T(0.1) | T(1) | table entries | table GAE | table inserts |");
    out.line("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut fit_a: Vec<(f64, [f64; 3])> = Vec::new();
    let mut keys: Vec<(u32, u32)> = Vec::new();
    for l in &lines {
        if jstr(l, "kind") == Some("tdec") && jstr(l, "arm") == Some("a") {
            let k = (
                jnum(l, "bits").unwrap() as u32,
                jnum(l, "bx").unwrap() as u32,
            );
            if !keys.contains(&k) {
                keys.push(k);
            }
        }
    }
    keys.sort();
    for (bits, bx) in keys {
        let sel = |tk: &str| -> Vec<&String> {
            lines
                .iter()
                .filter(|l| {
                    jstr(l, "kind") == Some("tdec")
                        && jstr(l, "arm") == Some("a")
                        && jnum(l, "bits") == Some(bits as f64)
                        && jnum(l, "bx") == Some(bx as f64)
                        && jstr(l, "tkind") == Some(tk)
                })
                .collect()
        };
        let rnd = sel("random");
        let pl = sel("planted");
        let gadd: Vec<f64> = rnd
            .iter()
            .map(|l| jnum(jsec(l, "counts").unwrap(), "gadd").unwrap())
            .collect();
        let look: Vec<f64> = rnd
            .iter()
            .map(|l| jnum(jsec(l, "counts").unwrap(), "lookup").unwrap())
            .collect();
        let nat: Vec<f64> = rnd.iter().map(|l| jnum(l, "native").unwrap()).collect();
        let hit = rnd.iter().filter(|l| jbool(l, "hit") == Some(true)).count();
        let found = pl.iter().filter(|l| jbool(l, "hit") == Some(true)).count();
        let mism = rnd
            .iter()
            .chain(pl.iter())
            .filter(
                |l| match (jbool(l, "hit"), jbool(l, "crosscheck_decomposable")) {
                    (Some(a), Some(b)) => a != b,
                    _ => false,
                },
            )
            .count();
        let setup = lines.iter().find(|l| {
            jstr(l, "kind") == Some("setup")
                && jstr(l, "arm") == Some("a")
                && jnum(l, "bits") == Some(bits as f64)
                && jnum(l, "bx") == Some(bx as f64)
        });
        let (ent, tg, ti) = match setup {
            Some(l) => (
                jnum(l, "entries").unwrap_or(f64::NAN),
                jnum(jsec(l, "counts").unwrap(), "gadd").unwrap_or(f64::NAN),
                jnum(jsec(l, "counts").unwrap(), "insert").unwrap_or(f64::NAN),
            ),
            None => (f64::NAN, f64::NAN, f64::NAN),
        };
        let t = [0, 1, 2].map(|i| mean(&gadd) + ws[i] * mean(&nat));
        let cell = cellp(bits);
        let prune = pl.len();
        let _ = prune;
        out.line(&format!("| {bits} | {} | {bx} | {} | {} | {:.3} | {}/{} | {mism} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {ent} | {tg} | {ti} |", cell.p, 2 * bx, rnd.len(), hit as f64 / rnd.len().max(1) as f64, found, pl.len(), mean(&gadd), mean(&look), t[0], t[1], t[2]));
        if bx as usize == cell_bx(&cell) {
            fit_a.push(((cell.p as f64).log2(), t));
        }
    }
    out.line(
        "\nFit of log2 T_dec(a) on log2 p over the design cells (OLS slope, 95% t interval). Arm (a) is the reference: its base is |F| = p^(1/4) up to rounding by construction, so this slope checks the base sizes; it is not an arm verdict (Decision rule 1):\n",
    );
    for (i, w) in ws.iter().enumerate() {
        let xs: Vec<f64> = fit_a.iter().map(|v| v.0).collect();
        let ys: Vec<f64> = fit_a.iter().map(|v| v.1[i].log2()).collect();
        match ols(&xs, &ys) {
            Some((m, lo, hi)) => out.line(&format!(
                "- w = {w}: exponent {m:.4} [{lo:.4}, {hi:.4}] over {} cells (threshold 0.25)",
                xs.len()
            )),
            None => out.line(&format!("- w = {w}: fewer than three cells")),
        }
    }
    // ---- full IC
    for arm in ["a", "b"] {
        out.line(&format!("\n## Full IC to completion, arm ({arm})\n"));
        out.line("| bits | bx | runs | ok | mean attempts | hit rate | mean GAE | mean native | S(0) | S(0.1) | S(1) | rho(0) | rho(0.1) | rho(1) | S(0)/rho(0) | S(0.1)/rho(0.1) | S(1)/rho(1) | mean setup GAE | mean online GAE |");
        out.line("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
        let mut ks: Vec<(u32, u32)> = Vec::new();
        for l in &lines {
            if jstr(l, "kind") == Some("fullic") && jstr(l, "arm") == Some(arm) {
                let k = (
                    jnum(l, "bits").unwrap() as u32,
                    jnum(l, "bx").unwrap() as u32,
                );
                if !ks.contains(&k) {
                    ks.push(k);
                }
            }
        }
        ks.sort();
        for (bits, bx) in ks {
            let r: Vec<&String> = lines
                .iter()
                .filter(|l| {
                    jstr(l, "kind") == Some("fullic")
                        && jstr(l, "arm") == Some(arm)
                        && jnum(l, "bits") == Some(bits as f64)
                        && jnum(l, "bx") == Some(bx as f64)
                })
                .collect();
            let ok = r.iter().filter(|l| jbool(l, "ok") == Some(true)).count();
            let att: Vec<f64> = r.iter().map(|l| jnum(l, "attempts").unwrap()).collect();
            let hits: f64 = r.iter().map(|l| jnum(l, "hits").unwrap()).sum();
            let gae: Vec<f64> = r.iter().map(|l| jnum(l, "gae_total").unwrap()).collect();
            let nat: Vec<f64> = r.iter().map(|l| jnum(l, "native_total").unwrap()).collect();
            let sg: Vec<f64> = r
                .iter()
                .map(|l| jnum(jsec(l, "setup").unwrap(), "gadd").unwrap())
                .collect();
            let og: Vec<f64> = r
                .iter()
                .map(|l| jnum(jsec(l, "online").unwrap(), "gadd").unwrap())
                .collect();
            let sw = [0, 1, 2].map(|i| mean(&gae) + ws[i] * mean(&nat));
            let rw = rho_w.get(&bits).copied().unwrap_or([f64::NAN; 3]);
            out.line(&format!("| {bits} | {bx} | {} | {ok} | {:.1} | {:.4} | {:.1} | {:.3e} | {:.1} | {:.1} | {:.3e} | {:.1} | {:.1} | {:.1} | {:.2} | {:.2} | {:.3e} | {:.1} | {:.1} |", r.len(), mean(&att), hits / att.iter().sum::<f64>(), mean(&gae), mean(&nat), sw[0], sw[1], sw[2], rw[0], rw[1], rw[2], sw[0] / rw[0], sw[1] / rw[1], sw[2] / rw[2], mean(&sg), mean(&og)));
        }
    }
    // ---- arm b tdec
    out.line("\n## Arm (b): F4 on S5 with the membership polynomials\n");
    out.line("| bits | bx | targets | done | budget | dim==brute (checked) | planted vanishes | mean madds (done) | min..max madds (done) | mean steps | max degree | max cols | max rows | peak nnz | zero rows | mean ms (done) | T(0.1) = 0.1 x native | T(1) = native |");
    out.line("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut kb: Vec<(u32, u32)> = Vec::new();
    for l in &lines {
        if jstr(l, "kind") == Some("tdec") && jstr(l, "arm") == Some("b") {
            let k = (
                jnum(l, "bits").unwrap() as u32,
                jnum(l, "bx").unwrap() as u32,
            );
            if !kb.contains(&k) {
                kb.push(k);
            }
        }
    }
    kb.sort();
    let mut ladder: Vec<(f64, f64)> = Vec::new();
    for (bits, bx) in kb {
        let r: Vec<&String> = lines
            .iter()
            .filter(|l| {
                jstr(l, "kind") == Some("tdec")
                    && jstr(l, "arm") == Some("b")
                    && jnum(l, "bits") == Some(bits as f64)
                    && jnum(l, "bx") == Some(bx as f64)
            })
            .collect();
        let done: Vec<&&String> = r
            .iter()
            .filter(|l| jstr(l, "status") == Some("done"))
            .collect();
        let madds: Vec<f64> = done
            .iter()
            .map(|l| jnum(jsec(l, "counts").unwrap(), "madd").unwrap())
            .collect();
        let natv: Vec<f64> = done.iter().map(|l| jnum(l, "native").unwrap()).collect();
        let ms: Vec<f64> = done.iter().map(|l| jnum(l, "ms").unwrap()).collect();
        let mut agree = 0;
        let mut checked = 0;
        for l in &r {
            if let (Some(d), Some(b)) = (jnum(l, "dim"), jnum(l, "brute_roots")) {
                checked += 1;
                if d == b {
                    agree += 1;
                }
            }
        }
        let pv_t = r
            .iter()
            .filter(|l| jbool(l, "planted_vanishes") == Some(true))
            .count();
        let pv_n = r
            .iter()
            .filter(|l| jbool(l, "planted_vanishes").is_some())
            .count();
        let mx = |k: &str| r.iter().filter_map(|l| jnum(l, k)).fold(0.0f64, f64::max);
        let st: Vec<f64> = r.iter().filter_map(|l| jnum(l, "steps")).collect();
        let zr: Vec<f64> = r.iter().filter_map(|l| jnum(l, "zero_rows")).collect();
        let (lo, hi) = (
            madds.iter().cloned().fold(f64::INFINITY, f64::min),
            madds.iter().cloned().fold(0.0f64, f64::max),
        );
        out.line(&format!("| {bits} | {bx} | {} | {} | {} | {agree}/{checked} | {pv_t}/{pv_n} | {:.4e} | {:.4e}..{:.4e} | {:.1} | {} | {} | {} | {} | {:.0} | {:.0} | {:.4e} | {:.4e} |", r.len(), done.len(), r.len() - done.len(), mean(&madds), lo, hi, mean(&st), mx("max_degree"), mx("max_cols"), mx("max_rows"), mx("peak_nnz"), mean(&zr), mean(&ms), 0.1 * mean(&natv), mean(&natv)));
        if bits == 16 && !madds.is_empty() && r.len() == done.len() {
            ladder.push((((2 * bx) as f64).log2(), mean(&madds).log2()));
        }
    }
    let (lx, ly): (Vec<f64>, Vec<f64>) = ladder.iter().cloned().unzip();
    match ols(&lx, &ly) {
        Some((m, lo, hi)) => out.line(&format!("\nLadder at 16 bits (finished cells only): log2 madds on log2 |F|: slope {m:.3} [{lo:.3}, {hi:.3}] over {} points. T_dec(b) grows as |F|^slope; with |F| = p^(1/4) the p-exponent is slope/4 (threshold 0.25, i.e. slope 1). At w = 0 the cost is zero GAE (F4 performs no group operation), so the exponent is undefined there.", lx.len())),
        None => out.line("\nLadder: fewer than three finished cells."),
    }
    // ---- arm c
    out.line("\n## Arm (c): committed PKM Kummer m = 4 rows (reused, not re-measured)\n");
    out.line("| source | N | t | |V| = 2^t | p | muladds | degree | steps | max cols |");
    out.line("|--|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut cx = Vec::new();
    let mut cy = Vec::new();
    for l in lines
        .iter()
        .filter(|l| jstr(l, "kind") == Some("reused-pkm"))
    {
        let src = jstr(l, "source").unwrap();
        let t = jnum(l, "t").unwrap();
        out.line(&format!(
            "| {src} | {} | {t} | {} | {} | {:.4e} | {} | {} | {} |",
            jnum(l, "N").unwrap(),
            2f64.powf(t),
            jnum(l, "p").unwrap(),
            jnum(l, "muladds").unwrap_or(f64::NAN),
            jnum(l, "degree").unwrap_or(f64::NAN),
            jnum(l, "steps").unwrap_or(f64::NAN),
            jnum(l, "max_cols").unwrap_or(f64::NAN)
        ));
        if src.contains("round2") || src.contains("round3") {
            if jnum(l, "target_index") == Some(0.0) {
                cx.push(t); // log2 |V| = t
                cy.push(jnum(l, "muladds").unwrap().log2());
            }
        }
    }
    match ols(&cx, &cy) {
        Some((m, lo, hi)) => out.line(&format!("\nPKM (round 2/3 engine, target 0): log2 muladds on log2 |V|: slope {m:.3} [{lo:.3}, {hi:.3}] over {} points (|V| = 4, 8, 16).", cx.len())),
        None => out.line("\nPKM slope needs three points."),
    }
    // ---- oracle-phase model rows
    out.line("\n## Oracle-phase cost rows (MODEL from measured components; not end-to-end measurements)\n");
    out.line("relation-phase oracle cost = attempts x T_dec, with attempts = mean attempts to full rank measured by the arm (a) full IC at the same (bits, bx); the setup, matrix and verification costs are left out, so each row is a lower bound on that arm's whole S.\n");
    out.line("| bits | bx | arm | attempts | T_dec native (madds) | status | oracle cost w=0 (GAE, attempts) | oracle cost w=0.1 | oracle cost w=1 | rho(0.1) | rho(1) |");
    out.line("|--:|--:|--|--:|--:|--|--:|--:|--:|--:|--:|");
    for (bits, bx) in [
        (16u32, 8u32),
        (20, 15),
        (24, 29),
        (28, 63),
        (16, 3),
        (16, 4),
        (16, 5),
        (16, 6),
        (16, 7),
    ] {
        let fa: Vec<&String> = lines
            .iter()
            .filter(|l| {
                jstr(l, "kind") == Some("fullic")
                    && jstr(l, "arm") == Some("a")
                    && jnum(l, "bits") == Some(bits as f64)
                    && jnum(l, "bx") == Some(bx as f64)
            })
            .collect();
        let rb: Vec<&String> = lines
            .iter()
            .filter(|l| {
                jstr(l, "kind") == Some("tdec")
                    && jstr(l, "arm") == Some("b")
                    && jnum(l, "bits") == Some(bits as f64)
                    && jnum(l, "bx") == Some(bx as f64)
            })
            .collect();
        if fa.is_empty() || rb.is_empty() {
            continue;
        }
        let att = mean(
            &fa.iter()
                .map(|l| jnum(l, "attempts").unwrap())
                .collect::<Vec<_>>(),
        );
        let done = rb.iter().all(|l| jstr(l, "status") == Some("done"));
        let madds = mean(
            &rb.iter()
                .map(|l| jnum(jsec(l, "counts").unwrap(), "madd").unwrap())
                .collect::<Vec<_>>(),
        );
        let status = if done {
            "measured"
        } else {
            "LOWER BOUND (F4 stopped at its cap)"
        };
        let rw = rho_w.get(&bits).copied().unwrap_or([f64::NAN; 3]);
        out.line(&format!("| {bits} | {bx} | b | {att:.1} | {madds:.4e} | {status} | {:.1} | {:.4e} | {:.4e} | {:.1} | {:.4e} |", att, att * madds * 0.1 + att, att * madds + att, rw[1], rw[2]));
    }
    out.line("\n(Arm (c) at its reachable |V| = 4, 8, 16 against p = 2013265921: the relation hit probability is at most C(|F|+3, 4)/n with |F| the signed base points on the curve, so attempts >= (|F|/2 + 1) / hit probability; with muladds from the table above this is a lower bound on the arm's oracle phase. The rho reference at that p is 1.2533 sqrt(p).)");
    let p1 = 2013265921f64;
    out.line("\n| t | |V| | signed points (at most) | hit prob (at most) | attempts (at least) | muladds per attempt | oracle phase muladds (at least) | rho GAE |");
    out.line("|--:|--:|--:|--:|--:|--:|--:|");
    for l in lines.iter().filter(|l| {
        jstr(l, "kind") == Some("reused-pkm")
            && jnum(l, "target_index") == Some(0.0)
            && jstr(l, "source")
                .map(|s| s.contains("round2") || s.contains("round3"))
                .unwrap_or(false)
    }) {
        let t = jnum(l, "t").unwrap();
        let f = 2f64.powf(t);
        let k = f + 3.0;
        let c4 = k * (k - 1.0) * (k - 2.0) * (k - 3.0) / 24.0;
        let hp = c4 / p1;
        let att = (f / 2.0 + 1.0) / hp;
        let m = jnum(l, "muladds").unwrap_or(f64::NAN);
        out.line(&format!(
            "| {t} | {f} | {f} | {hp:.3e} | {att:.3e} | {m:.4e} | {:.3e} | {:.1} |",
            att * m,
            1.2533 * p1.sqrt()
        ));
    }
    // ---- reach
    out.line("\n## Arm (d): ideal determinant reach\n");
    out.line("| poly | d | level | dim | delta_max | asymptote | B_max at 16 | 20 | 24 | 28 bits | log2 p for B >= 2 |");
    out.line("|--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    for l in lines.iter().filter(|l| jstr(l, "kind") == Some("reach")) {
        let lvl = jnum(l, "level").unwrap();
        let m = jnum(l, "m").unwrap();
        if !(lvl <= 3.0
            || (m == 4.0 && lvl >= 5.0)
            || (m == 3.0 && lvl == 12.0)
            || (m == 2.0 && lvl == 40.0))
        {
            continue;
        }
        let b = jsec(l, "bmax").unwrap();
        out.line(&format!(
            "| {} | {} | {} | {} | {:.6} | {:.6} | {:.4} | {:.4} | {:.4} | {:.4} | {:.1} |",
            jstr(l, "poly").unwrap(),
            jnum(l, "d").unwrap(),
            lvl,
            jnum(l, "dim").unwrap(),
            jnum(l, "delta_max").unwrap(),
            jnum(l, "delta_asymptote").unwrap(),
            jnum(b, "16").unwrap(),
            jnum(b, "20").unwrap(),
            jnum(b, "24").unwrap(),
            jnum(b, "28").unwrap(),
            jnum(l, "log2p_for_b2").unwrap()
        ));
    }
}

// ───────────────────────── selftest ─────────────────────────

macro_rules! check {
    ($ok:expr, $($msg:tt)*) => {
        if !$ok {
            println!("SELFTEST FAIL: {}", format!($($msg)*));
            std::process::exit(1);
        }
    };
}

fn cmd_selftest(args: &[String]) {
    let deep = flag(args, "--deep");
    let t0 = Instant::now();
    // number theory
    check!(
        is_prime(2013265921) && !is_prime(2013265923) && is_prime(65521),
        "is_prime"
    );
    let f = Fp { p: 65519 };
    let tp = 65519u64;
    check!(is_prime(tp) && tp % 4 == 3, "test prime");
    check!(f.mul(f.inv(12345), 12345) == 1, "inv");
    check!(
        {
            let r = f.sqrt(f.mul(777, 777));
            r == 777 || r == f.neg(777)
        },
        "sqrt"
    );
    println!("selftest: number theory ok");
    // cells: recount, prime order, generator
    for c in CELLS.iter() {
        check!(c.p > 0, "cell {} not frozen", c.bits);
        check!(
            c.p >> (c.bits - 1) == 1 && c.p % 4 == 3 && is_prime(c.p),
            "cell {} p",
            c.bits
        );
        check!(is_prime(c.n), "cell {} n prime", c.bits);
        let cv = c.curve();
        check!(cv.on_curve(c.g()), "cell {} G on curve", c.bits);
        if c.bits <= 24 || deep {
            check!(
                count_points(c.p, c.a, c.b) == c.n,
                "cell {} recount",
                c.bits
            );
        }
        check!(cv.mul(c.n, c.g()).inf, "cell {} n G = O", c.bits);
        check!(!cv.mul(c.n - 1, c.g()).inf, "cell {} (n-1) G != O", c.bits);
        println!(
            "selftest: cell {} ok (p={}, n={}, bx={})",
            c.bits,
            c.p,
            c.n,
            cell_bx(c)
        );
    }
    // S3 and S5 controls on the 16-bit cell
    let cell = cell_for(16);
    let cv = cell.curve();
    let f = cv.f;
    let mut rng = Rng(99);
    let rand_pt = |rng: &mut Rng| loop {
        let x = 1 + rng.below(cell.p - 1);
        if let Some(pt) = cv.lift(x) {
            break if rng.below(2) == 0 { pt } else { cv.neg(pt) };
        }
    };
    for _ in 0..50 {
        let (p1, p2) = (rand_pt(&mut rng), rand_pt(&mut rng));
        let r = cv.add(p1, p2);
        if r.inf {
            continue;
        }
        let s = s5::s3_in_z(&f, cell.a, cell.b, p1.x, p2.x);
        let v = f.add(f.add(s[0], f.mul(s[1], r.x)), f.mul(s[2], f.mul(r.x, r.x)));
        check!(v == 0, "S3 vanishes on a sum");
    }
    println!("selftest: S3 ok");
    let t1 = Instant::now();
    let s5c = s5::s5_poly(&f, cell.a, cell.b);
    println!(
        "selftest: S5 interpolated in {:.2}s",
        t1.elapsed().as_secs_f64()
    );
    for _ in 0..40 {
        let ps = [
            rand_pt(&mut rng),
            rand_pt(&mut rng),
            rand_pt(&mut rng),
            rand_pt(&mut rng),
        ];
        let mut s = INF;
        for p in &ps {
            s = cv.add(s, *p);
        }
        if s.inf {
            continue;
        }
        let xs = [ps[0].x, ps[1].x, ps[2].x, ps[3].x, s.x];
        check!(
            s5::s5_eval(&f, cell.a, cell.b, xs) == 0,
            "S5 direct eval vanishes on a 4-sum"
        );
        check!(
            s5::poly_eval(&f, &s5c, xs) == 0,
            "S5 interpolated vanishes on a 4-sum"
        );
        let mut ys = xs;
        ys[4] = f.add(ys[4], 1);
        let a = s5::s5_eval(&f, cell.a, cell.b, ys);
        check!(
            a == s5::poly_eval(&f, &s5c, ys),
            "interpolated S5 equals direct S5 off the variety"
        );
        check!(
            a != 0,
            "S5 does not vanish off the variety (probability 1/p of a false alarm)"
        );
    }
    println!("selftest: S5 ok (vanishes on planted sums, interpolation exact)");
    // arm (a) vs the pair-table cross-check, planted and random targets, three cells
    for bits in [16u32, 20] {
        let cell = cell_for(bits);
        let cv = cell.curve();
        let base = Base::build_k(&cv, cell_bx(&cell));
        let table = TripleTable::build(&cv, &base);
        let pairt = PairTable::build(&cv, &base);
        let mut rng = Rng(5 + bits as u64);
        let (mut agree, mut hits) = (0, 0);
        for _ in 0..200 {
            let r = random_target(&cell, &base, &mut rng);
            let a = table.decompose(&cv, &base, r);
            let b = pairt.decompositions(&cv, r);
            check!(
                a.is_some() == !b.is_empty(),
                "arm a vs pair table agree on decomposability"
            );
            if let Some(dd) = a {
                check!(
                    verify_decomposition(&cv, &base, &dd, r),
                    "decomposition verifies"
                );
                hits += 1;
            }
            agree += 1;
        }
        for _ in 0..20 {
            let (r, _) = planted_target(&cv, &base, &mut rng);
            let a = table.decompose(&cv, &base, r);
            check!(
                a.is_some(),
                "positive control: planted target found by arm a"
            );
            check!(
                verify_decomposition(&cv, &base, &a.unwrap(), r),
                "planted decomposition verifies"
            );
        }
        println!("selftest: arm a, {} bits: {agree} targets agree with the pair table ({hits} decomposable); 20 planted found", bits);
    }
    // arm (b): F4 vs brute-force root count, small bases on the 16-bit cell
    for bound in [3u64, 4, 5, 6] {
        let base = Base::build_k(&cv, bound as usize);
        let bx = base.xs.len();
        if bx < 4 {
            continue;
        }
        let mut rng = Rng(1000 + bound);
        for t in 0..6 {
            let kind = t % 3; // 0 planted with distinct x, 1 random, 2 planted with a repeated point
            let planted = kind != 1;
            let (r, pl) = if kind == 0 {
                let (r, idx) = planted_target(&cv, &base, &mut rng);
                (r, Some(idx))
            } else if kind == 2 {
                let (r, idx) = loop {
                    let c: Vec<usize> = (0..3).map(|_| rng.below(bx as u64) as usize).collect();
                    if c[0] == c[1] || c[0] == c[2] || c[1] == c[2] {
                        continue;
                    }
                    let i0 = 2 * c[0] + rng.below(2) as usize;
                    let idx = [
                        i0,
                        i0,
                        2 * c[1] + rng.below(2) as usize,
                        2 * c[2] + rng.below(2) as usize,
                    ];
                    let mut s = INF;
                    for &i in &idx {
                        s = cv.add(s, base.pts[i]);
                    }
                    if !s.inf && base.xs.binary_search(&s.x).is_err() {
                        break (s, idx);
                    }
                };
                (r, Some(idx))
            } else {
                (random_target(&cell, &base, &mut rng), None)
            };
            let lim = f4::Limits {
                madds: 4_000_000_000,
                nnz: 100_000_000,
                deadline: Some(Instant::now() + std::time::Duration::from_secs(60)),
            };
            let px = pl.map(|idx| {
                [
                    base.pts[idx[0]].x,
                    base.pts[idx[1]].x,
                    base.pts[idx[2]].x,
                    base.pts[idx[3]].x,
                ]
            });
            let o = run_arm_b(&f, &s5c, &base, r.x, &lim, px);
            if o.dim.is_none() {
                println!("selftest: arm b bx={bx}: {} (skipped comparison)", o.status);
                continue;
            }
            let brute = brute_grid_roots(&cv, &base, r);
            check!(o.dim == Some(brute), "F4 quotient dimension {:?} equals brute-force root count {} (bx={bx}, planted={planted})", o.dim, brute);
            if planted {
                check!(brute >= 1, "positive control has a root");
                check!(
                    o.planted_ok == Some(true),
                    "planted tuple is in the variety"
                );
            }
            println!("selftest: arm b bx={bx} planted={planted}: dim {:?} = brute {} (madds {}, steps {}, {:.0} ms)", o.dim, brute, o.madds, o.steps, o.ms);
        }
    }
    // matrix and full IC (arm a) at 16 bits, rho
    {
        let cell = cell_for(16);
        let lim = f4::Limits {
            madds: u64::MAX,
            nnz: usize::MAX,
            deadline: None,
        };
        for run in 0..3u64 {
            let planted = 12345 + 7777 * run;
            let res = full_ic(
                &cell,
                cell_bx(&cell),
                'a',
                planted,
                31 + run,
                None,
                &lim,
                1_000_000,
            );
            check!(res.ok, "full IC (arm a) recovers the planted logarithm: run {run}, attempts {}, rank {}/{}", res.attempts, res.rank, res.cols);
            println!(
                "selftest: full IC arm a, 16 bits, run {run}: ok, {} attempts, {} hits, rank {}",
                res.attempts, res.hits, res.rank
            );
        }
        for run in 0..3u64 {
            let r = rho_run(&cell, 4321 + 999 * run, 77 + run);
            check!(r.ok, "rho recovers the planted logarithm");
            println!(
                "selftest: rho 16 bits run {run}: ok, {} adds, {} walks",
                r.counts[GADD], r.walks
            );
        }
    }
    // reach control: S3 levels
    {
        let (_, _, _, d1) = reach_row(2, 2, 1);
        let (_, _, _, d2) = reach_row(2, 2, 2);
        let (_, _, _, d3) = reach_row(2, 2, 3);
        check!((d1 - 1.0 / 18.0).abs() < 1e-12, "S3 level 1 reach is 1/18");
        check!((d2 - 0.1).abs() < 1e-12, "S3 level 2 reach is 1/10");
        check!((d3 - 0.119).abs() < 0.001, "S3 level 3 reach is 0.119");
        println!("selftest: reach control ok ({d1:.5}, {d2:.5}, {d3:.5})");
    }
    println!("SELFTEST OK ({:.1}s)", t0.elapsed().as_secs_f64());
}

// ───────────────────────── main ─────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");
    match cmd {
        "find-curve" => cmd_find_curve(&args),
        "selftest" => cmd_selftest(&args),
        "tdec" => cmd_tdec(&args),
        "fullic" => cmd_fullic(&args),
        "rho" => cmd_rho(&args),
        "reach" => cmd_reach(&args),
        "reuse" => cmd_reuse(&args),
        "summarize" => cmd_summarize(&args),
        _ => {
            eprintln!("usage: see the file header");
            std::process::exit(2);
        }
    }
}
