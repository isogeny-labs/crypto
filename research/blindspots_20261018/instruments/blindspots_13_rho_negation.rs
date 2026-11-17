//! Blind-spot item 13 instrument: the negation-map shortfall of Pollard rho
//! (hypothesis H-RHO-bc6ea6) and the ECC2K-130 completion plan.
//!
//! std-only, one file.  Build:
//!   rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native \
//!         -o b13 examples/blindspots_13_rho_negation.rs
//!
//! Part A.  One r-adding walk engine (Teske), generic over a counted group, run
//! on the plain points (the baseline) and on the classes {P, -P} under eleven
//! escape rules.  Every group addition and doubling is counted, every
//! unpriced native operation (partition hash, canonicalisation, cycle
//! comparison, distinguished-point table probe) is counted separately, and the
//! summary prices both from the raw counts.  The groups are
//!   * `ZGroup`    Z/N with the discrete logarithm as the point (sweep cells);
//!   * `TableZ`    Z/N whose keys and signs come from a real prime-field curve
//!                 (the agreement check against the next one);
//!   * `Curve`     a real short-Weierstrass curve over F_p with affine
//!                 arithmetic whose multiplications, squarings and inversions
//!                 are tallied by the arithmetic itself.
//! Subcommands: selftest | sweep | ctl | summary | plan.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::time::Instant;

// ---------------------------------------------------------------- utilities

#[inline]
fn mix(z0: u64) -> u64 {
    let mut z = z0.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(mix(seed))
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix(self.0)
    }
    fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[inline]
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, m);
        }
        b = mulmod(b, b, m);
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'bases: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn prev_prime(mut n: u64) -> u64 {
    n -= 1;
    while !is_prime(n) {
        n -= 1;
    }
    n
}

fn invmod(a: u64, m: u64) -> Option<u64> {
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if r0 != 1 {
        return None;
    }
    Some(t0.rem_euclid(m as i128) as u64)
}

#[inline]
fn negm(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}

fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(|s| s.as_str())
}

fn arg_u64(args: &[String], name: &str, default: u64) -> u64 {
    arg(args, name).map(|s| s.parse().unwrap_or_else(|_| die(&format!("bad {name}")))).unwrap_or(default)
}

fn arg_f64(args: &[String], name: &str, default: f64) -> f64 {
    arg(args, name).map(|s| s.parse().unwrap_or_else(|_| die(&format!("bad {name}")))).unwrap_or(default)
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(2)
}

// ------------------------------------------------------------ counted groups

/// Group operations tallied by a group's `add` and `dbl`.  `fm`, `fs` and `fi`
/// are field multiplications, squarings and inversions, tallied only by the
/// real curve (zero for the Z/N groups, whose field work is derived from
/// `adds` and `dbls` with the same per-operation constants).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Tally {
    adds: u64,
    dbls: u64,
    fm: u64,
    fs: u64,
    fi: u64,
}

impl Tally {
    fn merge(&mut self, o: &Tally) {
        self.adds += o.adds;
        self.dbls += o.dbls;
        self.fm += o.fm;
        self.fs += o.fs;
        self.fi += o.fi;
    }
}

/// Field operations of one affine addition and one affine doubling, the
/// repository's ledger convention (`PrimeCurve::FIELD_OPS_PER_ADD`,
/// `FIELD_OPS_PER_DOUBLE` in `src/cryptanalysis/ic_boundary.rs`).
const ADD_MSI: (u64, u64, u64) = (2, 1, 1);
const DBL_MSI: (u64, u64, u64) = (5, 2, 1);

trait Grp {
    type P: Copy + Eq + std::fmt::Debug;
    fn is_id(&self, p: Self::P) -> bool;
    /// `p + q`; `p == q` is a doubling, `p == -q` the identity (no field work).
    fn add(&self, p: Self::P, q: Self::P, t: &mut Tally) -> Self::P;
    fn dbl(&self, p: Self::P, t: &mut Tally) -> Self::P;
    fn neg(&self, p: Self::P) -> Self::P;
    /// True when `p` is the non-canonical one of `{p, -p}`.
    fn flipped(&self, p: Self::P) -> bool;
    /// A unique 64-bit name of the point; canonical representatives order by it.
    fn key(&self, p: Self::P) -> u64;
    /// `[a]G + [b]Q` by one joint double-and-add (Shamir), counted.
    fn lincomb(&self, a: u64, b: u64, t: &mut Tally) -> Self::P;
    /// Does `[d]G == Q`?  The double-and-add is counted.
    fn verify(&self, d: u64, t: &mut Tally) -> bool;
}

fn joint_counts(a: u64, b: u64) -> (u64, u64) {
    let v = a | b;
    if v == 0 {
        return (0, 0);
    }
    let top = 64 - v.leading_zeros();
    let low = v & ((1u64 << (top - 1)) - 1);
    ((top - 1) as u64, low.count_ones() as u64)
}

fn mul_counts(k: u64) -> (u64, u64) {
    if k == 0 {
        return (0, 0);
    }
    let bits = 64 - k.leading_zeros();
    ((bits - 1) as u64, (k.count_ones() - 1) as u64)
}

// -- Z/N with the logarithm as the point (sweep cells) --

struct ZGroup {
    n: u64,
    half: u64,
    d: u64,
}

impl Grp for ZGroup {
    type P = u64;
    #[inline]
    fn is_id(&self, p: u64) -> bool {
        p == 0
    }
    #[inline]
    fn add(&self, p: u64, q: u64, t: &mut Tally) -> u64 {
        if p == 0 {
            return q;
        }
        if q == 0 {
            return p;
        }
        if p == q {
            t.dbls += 1;
        } else if p + q == self.n {
            return 0;
        } else {
            t.adds += 1;
        }
        let s = p + q;
        if s >= self.n {
            s - self.n
        } else {
            s
        }
    }
    #[inline]
    fn dbl(&self, p: u64, t: &mut Tally) -> u64 {
        if p == 0 {
            return 0;
        }
        t.dbls += 1;
        let s = p + p;
        if s >= self.n {
            s - self.n
        } else {
            s
        }
    }
    #[inline]
    fn neg(&self, p: u64) -> u64 {
        negm(p, self.n)
    }
    #[inline]
    fn flipped(&self, p: u64) -> bool {
        p > self.half
    }
    #[inline]
    fn key(&self, p: u64) -> u64 {
        p
    }
    fn lincomb(&self, a: u64, b: u64, t: &mut Tally) -> u64 {
        let (d, s) = joint_counts(a, b);
        t.dbls += d;
        t.adds += s;
        ((a as u128 + b as u128 * self.d as u128) % self.n as u128) as u64
    }
    fn verify(&self, d: u64, t: &mut Tally) -> bool {
        let (dd, s) = mul_counts(d);
        t.dbls += dd;
        t.adds += s;
        d % self.n == self.d
    }
}

// -- a real short-Weierstrass curve y^2 = x^3 + a x + b over F_p, p = 3 mod 4 --

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: u64,
    y: u64,
    inf: bool,
}
const INF: Pt = Pt { x: 0, y: 0, inf: true };

#[derive(Clone)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    g: Pt,
    q: Pt,
    gq: Pt,
}

impl Curve {
    fn half(&self) -> u64 {
        (self.p - 1) / 2
    }
    fn raw_add(&self, p1: Pt, p2: Pt, t: &mut Tally) -> Pt {
        let p = self.p;
        if p1.inf {
            return p2;
        }
        if p2.inf {
            return p1;
        }
        if p1.x == p2.x {
            if (p1.y + p2.y) % p == 0 {
                return INF;
            }
            return self.raw_dbl(p1, t);
        }
        t.adds += 1;
        let dx = (p2.x + p - p1.x) % p;
        let inv = invmod(dx, p).expect("distinct abscissae");
        t.fi += 1;
        let dy = (p2.y + p - p1.y) % p;
        let lam = mulmod(dy, inv, p);
        t.fm += 1;
        let lam2 = mulmod(lam, lam, p);
        t.fs += 1;
        let x3 = (lam2 + 2 * p - p1.x - p2.x) % p;
        let dx3 = (p1.x + p - x3) % p;
        let ld = mulmod(lam, dx3, p);
        t.fm += 1;
        let y3 = (ld + p - p1.y) % p;
        Pt { x: x3, y: y3, inf: false }
    }
    fn raw_dbl(&self, p1: Pt, t: &mut Tally) -> Pt {
        let p = self.p;
        if p1.inf || p1.y == 0 {
            return INF;
        }
        t.dbls += 1;
        let x2 = mulmod(p1.x, p1.x, p);
        t.fs += 1;
        let three_x2 = mulmod(3, x2, p);
        t.fm += 1;
        let num = (three_x2 + self.a) % p;
        let two_y = mulmod(2, p1.y, p);
        t.fm += 1;
        let inv = invmod(two_y, p).expect("odd order point");
        t.fi += 1;
        let lam = mulmod(num, inv, p);
        t.fm += 1;
        let lam2 = mulmod(lam, lam, p);
        t.fs += 1;
        let two_x = mulmod(2, p1.x, p);
        t.fm += 1;
        let x3 = (lam2 + p - two_x) % p;
        let dx3 = (p1.x + p - x3) % p;
        let ld = mulmod(lam, dx3, p);
        t.fm += 1;
        let y3 = (ld + p - p1.y) % p;
        Pt { x: x3, y: y3, inf: false }
    }
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (mulmod(mulmod(x, x, p), x, p) + mulmod(self.a, x, p) + self.b) % p
    }
    fn mul_raw(&self, pt: Pt, k: u64) -> Pt {
        let mut t = Tally::default();
        if k == 0 {
            return INF;
        }
        let bits = 64 - k.leading_zeros();
        let mut acc = pt;
        for i in (0..bits - 1).rev() {
            acc = self.raw_dbl(acc, &mut t);
            if (k >> i) & 1 == 1 {
                acc = self.raw_add(acc, pt, &mut t);
            }
        }
        acc
    }
}

impl Grp for Curve {
    type P = Pt;
    fn is_id(&self, p: Pt) -> bool {
        p.inf
    }
    fn add(&self, p: Pt, q: Pt, t: &mut Tally) -> Pt {
        self.raw_add(p, q, t)
    }
    fn dbl(&self, p: Pt, t: &mut Tally) -> Pt {
        self.raw_dbl(p, t)
    }
    fn neg(&self, p: Pt) -> Pt {
        if p.inf {
            p
        } else {
            Pt { x: p.x, y: (self.p - p.y) % self.p, inf: false }
        }
    }
    fn flipped(&self, p: Pt) -> bool {
        p.y > self.half()
    }
    fn key(&self, p: Pt) -> u64 {
        (p.x << 1) | (p.y > self.half()) as u64
    }
    fn lincomb(&self, a: u64, b: u64, t: &mut Tally) -> Pt {
        let v = a | b;
        if v == 0 {
            return INF;
        }
        let top = 64 - v.leading_zeros();
        let pick = |i: u32| match ((a >> i) & 1, (b >> i) & 1) {
            (1, 0) => Some(self.g),
            (0, 1) => Some(self.q),
            (1, 1) => Some(self.gq),
            _ => None,
        };
        let mut acc = pick(top - 1).unwrap();
        for i in (0..top - 1).rev() {
            acc = self.raw_dbl(acc, t);
            if let Some(m) = pick(i) {
                acc = self.raw_add(acc, m, t);
            }
        }
        acc
    }
    fn verify(&self, d: u64, t: &mut Tally) -> bool {
        if d == 0 {
            return false;
        }
        let bits = 64 - d.leading_zeros();
        let mut acc = self.g;
        for i in (0..bits - 1).rev() {
            acc = self.raw_dbl(acc, t);
            if (d >> i) & 1 == 1 {
                acc = self.raw_add(acc, self.g, t);
            }
        }
        acc == self.q
    }
}

/// A prime-order curve with p = 3 mod 4 near 2^bits, found by exhaustive
/// point counting (small `bits` only), and a generator.
fn find_curve(bits: u32, seed: u64) -> Curve {
    let mut rng = Rng::new(seed);
    loop {
        let mut p = (1u64 << (bits - 1)) + rng.below(1u64 << (bits - 1));
        p |= 3;
        if !is_prime(p) || p % 4 != 3 {
            continue;
        }
        let a = rng.below(p);
        let b = 1 + rng.below(p - 1);
        if (4 * mulmod(mulmod(a, a, p), a, p) + 27 * mulmod(b, b, p)) % p == 0 {
            continue;
        }
        let mut c = Curve { p, a, b, n: 0, g: INF, q: INF, gq: INF };
        let mut count = 1u64;
        for x in 0..p {
            let r = c.rhs(x);
            if r == 0 {
                count += 1;
            } else if powmod(r, (p - 1) / 2, p) == 1 {
                count += 2;
            }
        }
        if !is_prime(count) || count == p {
            continue;
        }
        c.n = count;
        loop {
            let x = rng.below(p);
            let r = c.rhs(x);
            let y = powmod(r, (p + 1) / 4, p);
            if mulmod(y, y, p) == r && y != 0 {
                c.g = Pt { x, y, inf: false };
                break;
            }
        }
        return c;
    }
}

/// Z/N whose keys and signs are those of the real curve's multiples of G.
struct TableZ {
    z: ZGroup,
    xs: Vec<u32>,
    ys: Vec<u32>,
    half_p: u32,
}

impl TableZ {
    fn new(c: &Curve, d: u64) -> TableZ {
        let n = c.n as usize;
        let mut xs = vec![0u32; n];
        let mut ys = vec![0u32; n];
        let mut t = Tally::default();
        let mut acc = c.g;
        for e in 1..n {
            xs[e] = acc.x as u32;
            ys[e] = acc.y as u32;
            acc = c.raw_add(acc, c.g, &mut t);
        }
        TableZ { z: ZGroup { n: c.n, half: c.n / 2, d }, xs, ys, half_p: c.half() as u32 }
    }
}

impl Grp for TableZ {
    type P = u64;
    fn is_id(&self, p: u64) -> bool {
        p == 0
    }
    fn add(&self, p: u64, q: u64, t: &mut Tally) -> u64 {
        self.z.add(p, q, t)
    }
    fn dbl(&self, p: u64, t: &mut Tally) -> u64 {
        self.z.dbl(p, t)
    }
    fn neg(&self, p: u64) -> u64 {
        self.z.neg(p)
    }
    fn flipped(&self, p: u64) -> bool {
        self.ys[p as usize] > self.half_p
    }
    fn key(&self, p: u64) -> u64 {
        ((self.xs[p as usize] as u64) << 1) | (self.ys[p as usize] > self.half_p) as u64
    }
    fn lincomb(&self, a: u64, b: u64, t: &mut Tally) -> u64 {
        self.z.lincomb(a, b, t)
    }
    fn verify(&self, d: u64, t: &mut Tally) -> bool {
        self.z.verify(d, t)
    }
}

// ------------------------------------------------------------------ the arms

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rule {
    /// No detection or escape.
    None,
    /// Detect a 2-cycle by comparing with the point two steps back; escape by
    /// doubling the repeated point (history dependent).
    Dbl,
    /// The same detection; escape by doubling the smaller of the two cycle
    /// members (Bos-Kleinjung-Lenstra min-of-2; memoryless).
    Min2,
    /// Wiener-Zuccherato look-ahead: refuse a jump whose image would take the
    /// same jump back; no other detection.
    La,
    /// Running minimum over windows of k steps; two equal consecutive minima
    /// are a cycle, escaped by doubling the minimum.
    Per(u32),
    /// Look-ahead plus periodic detection.
    LaPer(u32),
}

#[derive(Clone, Copy, Debug)]
struct Arm {
    neg: bool,
    rule: Rule,
}

const ARM_NAMES: [&str; 14] = [
    "base", "none", "dbl", "min2", "la", "per2", "per4", "per8", "per16", "per32", "per64", "laper16", "laper32", "laper64",
];

/// Collision-preserving escape rules: two walks that fall into the same cycle
/// leave it at the same point (a function of the cycle's content, not of how
/// each walk entered it), so distinguished-point collisions survive the escape
/// (H-RHO-bc6ea6 memorylessness filter, read as that property).  `dbl` doubles
/// the point at which the walk happened to be, which depends on the entry
/// phase: it is measured and reported but never counted as a winner.
fn admissible(name: &str) -> bool {
    !matches!(name, "dbl" | "none" | "base" | "baseb")
}

fn arm_of(name: &str) -> Option<Arm> {
    let (neg, rule) = match name {
        "base" | "baseb" => (false, Rule::None),
        "none" => (true, Rule::None),
        "dbl" => (true, Rule::Dbl),
        "min2" => (true, Rule::Min2),
        "la" => (true, Rule::La),
        "per2" => (true, Rule::Per(2)),
        "per4" => (true, Rule::Per(4)),
        "per8" => (true, Rule::Per(8)),
        "per16" => (true, Rule::Per(16)),
        "per32" => (true, Rule::Per(32)),
        "per64" => (true, Rule::Per(64)),
        "laper16" => (true, Rule::LaPer(16)),
        "laper32" => (true, Rule::LaPer(32)),
        "laper64" => (true, Rule::LaPer(64)),
        _ => return None,
    };
    Some(Arm { neg, rule })
}

// ------------------------------------------------------------------- records

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct Rec {
    setup: Tally,
    start: Tally,
    walk: Tally,
    la: Tally,
    esc: Tally,
    ver: Tally,
    hash: u64,
    canon: u64,
    cmp: u64,
    lookup: u64,
    walks: u64,
    dps: u64,
    capped: u64,
    lost: u64,
    steps: u64,
    escapes: u64,
    c2_entries: u64,
    c2_trials: u64,
    /// The same two counts over every step, trapped walks included.
    c2_entries_all: u64,
    c2_trials_all: u64,
    useless: u64,
    bad: u64,
    solved: bool,
    censored: bool,
    found: u64,
    /// Order-sensitive digest of the distinguished points reached.
    dp_digest: u64,
}

impl Rec {
    fn total(&self) -> Tally {
        let mut t = Tally::default();
        for x in [&self.setup, &self.start, &self.walk, &self.la, &self.esc, &self.ver] {
            t.merge(x);
        }
        t
    }
}

enum End<P> {
    Dp(P, u64, u64),
    Capped,
    Lost,
    Out,
}

struct Ctx<'a, G: Grp> {
    g: &'a G,
    n: u64,
    r: usize,
    dpmask: u64,
    salt: u64,
    neg: bool,
    rule: Rule,
    walk_cap: u64,
    cap_ops: u64,
    jumps: Vec<(G::P, u64, u64)>,
}

struct St {
    rec: Rec,
    ops: u64,
}

impl<'a, G: Grp> Ctx<'a, G> {
    #[inline]
    fn idx(&self, h: u64) -> usize {
        (((h >> 32) * self.r as u64) >> 32) as usize
    }
    #[inline]
    fn is_dp(&self, h: u64) -> bool {
        h & self.dpmask == 0
    }
    fn la(&self) -> bool {
        matches!(self.rule, Rule::La | Rule::LaPer(_))
    }
    fn period(&self) -> u32 {
        match self.rule {
            Rule::Per(k) | Rule::LaPer(k) => k,
            _ => 0,
        }
    }

    /// Canonicalise a raw point with its coefficients and hash it.
    #[inline]
    fn arrive(&self, st: &mut St, z: G::P, za: u64, zb: u64) -> (G::P, u64, u64, u64) {
        let g = self.g;
        let (x, a, b) = if self.neg {
            st.rec.canon += 1;
            if g.flipped(z) {
                (g.neg(z), negm(za, self.n), negm(zb, self.n))
            } else {
                (z, za, zb)
            }
        } else {
            (z, za, zb)
        };
        st.rec.hash += 1;
        let h = mix(g.key(x) ^ self.salt);
        (x, a, b, h)
    }

    /// Leave a detected cycle by doubling `m`; `None` when that is the identity.
    fn escape(&self, st: &mut St, m: (G::P, u64, u64)) -> Option<(G::P, u64, u64, u64)> {
        let mut t = Tally::default();
        let z = self.g.dbl(m.0, &mut t);
        st.rec.esc.merge(&t);
        st.ops += 1;
        st.rec.escapes += 1;
        if self.g.is_id(z) {
            return None;
        }
        let n = self.n;
        let za = (m.1 + m.1) % n;
        let zb = (m.2 + m.2) % n;
        Some(self.arrive(st, z, za, zb))
    }

    /// One walk.  The fruitless-cycle entry counts of a walk are credited to
    /// the record only when the walk ends at a distinguished point: steps spent
    /// inside a longer trapped cycle until the cap are not draws of the
    /// per-step 2-cycle probability that HEUR-RHO-1 states.
    fn walk(&self, st: &mut St, sx: G::P, sa: u64, sb: u64) -> End<G::P> {
        let (mut ce, mut ct) = (0u64, 0u64);
        let end = self.walk_inner(st, sx, sa, sb, &mut ce, &mut ct);
        st.rec.c2_entries_all += ce;
        st.rec.c2_trials_all += ct;
        if matches!(end, End::Dp(..)) {
            st.rec.c2_entries += ce;
            st.rec.c2_trials += ct;
        }
        end
    }

    fn walk_inner(&self, st: &mut St, sx: G::P, sa: u64, sb: u64, ce: &mut u64, ct: &mut u64) -> End<G::P> {
        let g = self.g;
        let n = self.n;
        let r = self.r;
        let la = self.la();
        let period = self.period();
        let (mut x, mut a, mut b, mut h) = self.arrive(st, sx, sa, sb);
        if self.is_dp(h) {
            return End::Dp(x, a, b);
        }
        let mut len = 0u64;
        let mut have_prev = false;
        let mut prev_key = 0u64;
        let mut prev_ev = false;
        let mut wmin_key = u64::MAX;
        let mut wmin = (x, a, b);
        let mut pmin_key = u64::MAX;
        let mut wcount = 0u32;
        loop {
            if st.ops >= self.cap_ops {
                return End::Out;
            }
            if len >= self.walk_cap {
                st.rec.capped += 1;
                return End::Capped;
            }
            let j0 = self.idx(h);
            let mut tried = 0usize;
            let (y, ya, yb, hy);
            loop {
                let j = (j0 + tried) % r;
                let (m, al, be) = self.jumps[j];
                let mut tt = Tally::default();
                let s = g.add(x, m, &mut tt);
                st.ops += 1;
                if g.is_id(s) {
                    st.rec.walk.merge(&tt);
                    st.rec.lost += 1;
                    return End::Lost;
                }
                let mut sa2 = a + al;
                if sa2 >= n {
                    sa2 -= n;
                }
                let mut sb2 = b + be;
                if sb2 >= n {
                    sb2 -= n;
                }
                let fl = self.neg && g.flipped(s);
                let (yy, aa, bb, hh) = self.arrive(st, s, sa2, sb2);
                // The next step from `yy` would use jump `j` and return to -x.
                let ev = self.neg && fl && self.idx(hh) == j;
                let fresh = !prev_ev;
                if fresh {
                    *ct += 1;
                }
                if ev && fresh {
                    *ce += 1;
                }
                if la {
                    st.rec.cmp += fl as u64;
                    if ev && tried + 1 < r {
                        st.rec.la.merge(&tt);
                        tried += 1;
                        continue;
                    }
                }
                prev_ev = ev;
                st.rec.walk.merge(&tt);
                st.rec.steps += 1;
                y = yy;
                ya = aa;
                yb = bb;
                hy = hh;
                break;
            }
            len += 1;
            if self.is_dp(hy) {
                return End::Dp(y, ya, yb);
            }
            let ky = g.key(y);
            match self.rule {
                Rule::Dbl | Rule::Min2 => {
                    st.rec.cmp += 1;
                    if have_prev && ky == prev_key {
                        let m = if self.rule == Rule::Min2 {
                            st.rec.cmp += 1;
                            if g.key(x) < ky {
                                (x, a, b)
                            } else {
                                (y, ya, yb)
                            }
                        } else {
                            (y, ya, yb)
                        };
                        match self.escape(st, m) {
                            None => {
                                st.rec.lost += 1;
                                return End::Lost;
                            }
                            Some((ex, ea, eb, eh)) => {
                                x = ex;
                                a = ea;
                                b = eb;
                                h = eh;
                                have_prev = false;
                                prev_ev = false;
                                if self.is_dp(h) {
                                    return End::Dp(x, a, b);
                                }
                                continue;
                            }
                        }
                    }
                }
                Rule::Per(_) | Rule::LaPer(_) => {
                    st.rec.cmp += 1;
                    if ky < wmin_key {
                        wmin_key = ky;
                        wmin = (y, ya, yb);
                    }
                    wcount += 1;
                    if wcount == period {
                        st.rec.cmp += 1;
                        if wmin_key == pmin_key {
                            match self.escape(st, wmin) {
                                None => {
                                    st.rec.lost += 1;
                                    return End::Lost;
                                }
                                Some((ex, ea, eb, eh)) => {
                                    x = ex;
                                    a = ea;
                                    b = eb;
                                    h = eh;
                                    pmin_key = u64::MAX;
                                    wmin_key = u64::MAX;
                                    wcount = 0;
                                    have_prev = false;
                                    prev_ev = false;
                                    if self.is_dp(h) {
                                        return End::Dp(x, a, b);
                                    }
                                    continue;
                                }
                            }
                        } else {
                            pmin_key = wmin_key;
                            wmin_key = u64::MAX;
                            wcount = 0;
                        }
                    }
                }
                Rule::None | Rule::La => {}
            }
            have_prev = true;
            prev_key = g.key(x);
            x = y;
            a = ya;
            b = yb;
            h = hy;
        }
    }
}

struct Cfg {
    n: u64,
    r: usize,
    theta: u32,
    cap_ops: u64,
}

fn run_target<G: Grp>(g: &G, cfg: &Cfg, arm: Arm, tseed: u64, d: u64) -> Rec {
    let n = cfg.n;
    let r = cfg.r;
    let mut st = St { rec: Rec::default(), ops: 0 };
    let mut rng = Rng::new(mix(tseed) ^ ((r as u64) << 40) ^ 0x7AB1E);
    let salt = rng.next();
    // Set-up: G + Q once, the jump table, the start stride.
    st.rec.setup.adds += 1;
    st.rec.setup.fm += ADD_MSI.0;
    st.rec.setup.fs += ADD_MSI.1;
    st.rec.setup.fi += ADD_MSI.2;
    let mut jumps = Vec::with_capacity(r);
    for _ in 0..r {
        let al = 1 + rng.below(n - 1);
        let be = 1 + rng.below(n - 1);
        let pt = g.lincomb(al, be, &mut st.rec.setup);
        jumps.push((pt, al, be));
    }
    let (mut ta, mut tb) = (1 + rng.below(n - 1), 1 + rng.below(n - 1));
    let mut stride = g.lincomb(ta, tb, &mut st.rec.setup);
    while g.is_id(stride) {
        ta = 1 + rng.below(n - 1);
        tb = 1 + rng.below(n - 1);
        stride = g.lincomb(ta, tb, &mut st.rec.setup);
    }
    let ctx = Ctx {
        g,
        n,
        r,
        dpmask: (1u64 << cfg.theta) - 1,
        salt,
        neg: arm.neg,
        rule: arm.rule,
        walk_cap: 20 * (1u64 << cfg.theta) + 64,
        cap_ops: cfg.cap_ops,
        jumps,
    };
    let mut table: HashMap<u64, (u64, u64)> = HashMap::with_capacity(1 << 12);
    let (mut sx, mut sa, mut sb) = (stride, ta, tb);
    let mut digest = 0u64;
    let mut walks = 0u64;
    loop {
        if walks > 0 {
            sx = g.add(sx, stride, &mut st.rec.start);
            st.ops += 1;
            sa = (sa + ta) % n;
            sb = (sb + tb) % n;
        }
        walks += 1;
        if st.ops >= cfg.cap_ops {
            st.rec.censored = true;
            break;
        }
        if g.is_id(sx) {
            continue;
        }
        match ctx.walk(&mut st, sx, sa, sb) {
            End::Out => {
                st.rec.censored = true;
                break;
            }
            End::Capped | End::Lost => continue,
            End::Dp(x, a, b) => {
                st.rec.dps += 1;
                st.rec.lookup += 1;
                let key = g.key(x);
                digest = mix(digest ^ key) ^ walks;
                match table.get(&key) {
                    Some(&(a2, b2)) => {
                        if b2 != b {
                            let num = (a + n - a2) % n;
                            let den = (b2 + n - b) % n;
                            let inv = invmod(den, n).expect("prime order");
                            let dc = mulmod(num, inv, n);
                            if g.verify(dc, &mut st.rec.ver) {
                                st.rec.solved = true;
                                st.rec.found = dc;
                                if dc != d {
                                    st.rec.bad += 1;
                                }
                                break;
                            }
                            st.rec.bad += 1;
                        }
                        st.rec.useless += 1;
                    }
                    None => {
                        table.insert(key, (a, b));
                    }
                }
            }
        }
    }
    st.rec.walks = walks;
    st.rec.dp_digest = digest;
    st.rec
}

fn pick_d(tseed: u64, n: u64) -> u64 {
    1 + Rng::new(mix(tseed) ^ 0xD1D1).below(n - 1)
}

fn tseed_of(bits: u32, t: u64, offset: u64) -> u64 {
    mix(0x13_0000_0000 ^ ((bits as u64) << 48) ^ t.wrapping_add(offset))
}

fn cap_ops_of(n: u64, mult: f64) -> u64 {
    (mult * (std::f64::consts::PI * n as f64 / 2.0).sqrt()) as u64
}

// ---------------------------------------------------------------- JSON lines

fn rec_json(bits: u32, theta: u32, r: usize, arm: &str, t: u64, rec: &Rec) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "{{\"kind\":\"rec\",\"bits\":{bits},\"theta\":{theta},\"r\":{r},\"arm\":\"{arm}\",\"t\":{t},\
\"solved\":{},\"cens\":{},\"found\":{},\
\"setup_a\":{},\"setup_d\":{},\"start_a\":{},\"walk_a\":{},\"walk_d\":{},\"la_a\":{},\"la_d\":{},\
\"esc_a\":{},\"esc_d\":{},\"ver_a\":{},\"ver_d\":{},\
\"hash\":{},\"canon\":{},\"cmp\":{},\"lookup\":{},\"walks\":{},\"dps\":{},\"capped\":{},\"lost\":{},\
\"steps\":{},\"escapes\":{},\"c2e\":{},\"c2t\":{},\"c2ea\":{},\"c2ta\":{},\"useless\":{},\"bad\":{}}}",
        rec.solved as u8,
        rec.censored as u8,
        rec.found,
        rec.setup.adds,
        rec.setup.dbls,
        rec.start.adds,
        rec.walk.adds,
        rec.walk.dbls,
        rec.la.adds,
        rec.la.dbls,
        rec.esc.adds,
        rec.esc.dbls,
        rec.ver.adds,
        rec.ver.dbls,
        rec.hash,
        rec.canon,
        rec.cmp,
        rec.lookup,
        rec.walks,
        rec.dps,
        rec.capped,
        rec.lost,
        rec.steps,
        rec.escapes,
        rec.c2_entries,
        rec.c2_trials,
        rec.c2_entries_all,
        rec.c2_trials_all,
        rec.useless,
        rec.bad
    );
    s
}

// ------------------------------------------------------- ideal random-map control

/// Steps until the first distinguished-point collision between two different
/// walks of a random map on `n` points (or on its `(n+1)/2` classes), with the
/// sweep's walk length, cap and start protocol.  No group, no coefficients.
fn ctl_run(n: u64, quotient: bool, theta: u32, tseed: u64) -> (u64, bool) {
    let salt1 = mix(tseed ^ 0xC71);
    let salt2 = mix(tseed ^ 0xC72);
    let mask = (1u64 << theta) - 1;
    let walk_cap = 20 * (1u64 << theta) + 64;
    let mut rng = Rng::new(tseed ^ 0xC73);
    let canon = |v: u64| if quotient && v > n / 2 { n - v } else { v };
    let mut table: HashMap<u64, u64> = HashMap::with_capacity(1 << 12);
    let mut steps = 0u64;
    let limit = 64 * ((std::f64::consts::PI * n as f64 / 2.0).sqrt() as u64 + 1);
    let mut walk_id = 0u64;
    while steps < limit {
        walk_id += 1;
        let mut x = canon(rng.below(n));
        let mut len = 0u64;
        loop {
            x = canon(((mix(x ^ salt1) as u128 * n as u128) >> 64) as u64);
            steps += 1;
            len += 1;
            if mix(x ^ salt2) & mask == 0 {
                match table.get(&x) {
                    Some(&w) if w != walk_id => return (steps, true),
                    Some(_) => {}
                    None => {
                        table.insert(x, walk_id);
                    }
                }
                break;
            }
            if len >= walk_cap {
                break;
            }
        }
    }
    (steps, false)
}

// ------------------------------------------------------------------ selftest

fn theta_of(bits: u32) -> u32 {
    bits / 4
}

fn selftest() -> bool {
    let ok = std::cell::Cell::new(true);
    let check = |name: &str, pass: bool, detail: String| {
        println!("{} {name}: {detail}", if pass { "PASS" } else { "FAIL" });
        if !pass {
            ok.set(false);
        }
    };
    // 1. number theory
    check("is_prime", is_prime(1_099_511_627_689) && !is_prime(1_099_511_627_691), "40-bit".into());
    check("invmod", invmod(12345, 1_099_511_627_689).map(|i| mulmod(i, 12345, 1_099_511_627_689)) == Some(1), String::new());

    // 2. real curve: counted field work equals the per-operation constants, and
    //    the Z/N group's analytic joint and verification counts equal the real ones.
    let bits = 20u32;
    let mut c = find_curve(bits, 0x13A);
    let d = pick_d(77, c.n);
    c.q = c.mul_raw(c.g, d);
    let mut tt = Tally::default();
    c.gq = c.raw_add(c.g, c.q, &mut tt);
    check(
        "curve",
        c.n > (1 << (bits - 1)) && is_prime(c.n) && c.mul_raw(c.g, c.n).inf,
        format!("p={} N={} (prime, [N]G=O)", c.p, c.n),
    );
    let z = ZGroup { n: c.n, half: c.n / 2, d };
    let mut agree = true;
    let mut rng = Rng::new(5);
    for _ in 0..200 {
        let (a, b) = (1 + rng.below(c.n - 1), 1 + rng.below(c.n - 1));
        let (mut t1, mut t2) = (Tally::default(), Tally::default());
        let pt = c.lincomb(a, b, &mut t1);
        let e = z.lincomb(a, b, &mut t2);
        let want = c.mul_raw(c.g, e);
        agree &= pt == want && (t1.adds, t1.dbls) == (t2.adds, t2.dbls);
        agree &= t1.fm == ADD_MSI.0 * t1.adds + DBL_MSI.0 * t1.dbls
            && t1.fs == ADD_MSI.1 * t1.adds + DBL_MSI.1 * t1.dbls
            && t1.fi == ADD_MSI.2 * t1.adds + DBL_MSI.2 * t1.dbls;
        let (mut v1, mut v2) = (Tally::default(), Tally::default());
        agree &= c.verify(d, &mut v1) && z.verify(d, &mut v2) && (v1.adds, v1.dbls) == (v2.adds, v2.dbls);
    }
    check("counts", agree, "200 random joint multiplications and verifications: points, adds, doubles, field ledger".into());

    // 3. agreement of the two implementations on frozen seeds: the walk engine on
    //    the real curve and on the Z/N table built from it, every arm.
    let tab = TableZ::new(&c, d);
    let cfg_r = |r: usize| Cfg { n: c.n, r, theta: theta_of(bits), cap_ops: cap_ops_of(c.n, 6.0) };
    let mut all_equal = true;
    let mut solved_all = true;
    let mut runs = 0;
    let mut detail = String::new();
    for name in ARM_NAMES {
        let arm = arm_of(name).unwrap();
        for r in [16usize, 64] {
            for t in 0..4u64 {
                let ts = tseed_of(bits, t, 0);
                let dd = pick_d(ts, c.n);
                let mut real = c.clone();
                real.q = real.mul_raw(real.g, dd);
                let mut t0 = Tally::default();
                real.gq = real.raw_add(real.g, real.q, &mut t0);
                let tz = TableZ { z: ZGroup { n: c.n, half: c.n / 2, d: dd }, xs: tab.xs.clone(), ys: tab.ys.clone(), half_p: tab.half_p };
                let cfg = cfg_r(r);
                let r1 = run_target(&real, &cfg, arm, ts, dd);
                let r2 = run_target(&tz, &cfg, arm, ts, dd);
                runs += 1;
                let mut a = r1.clone();
                let mut b = r2.clone();
                // field ledger of the real run against the derived one
                let tot = r1.total();
                let f_ok = tot.fm == ADD_MSI.0 * tot.adds + DBL_MSI.0 * tot.dbls
                    && tot.fs == ADD_MSI.1 * tot.adds + DBL_MSI.1 * tot.dbls
                    && tot.fi == ADD_MSI.2 * tot.adds + DBL_MSI.2 * tot.dbls;
                // the Z group does not tally field work; compare group counts
                for rr in [&mut a, &mut b] {
                    for tl in [&mut rr.setup, &mut rr.start, &mut rr.walk, &mut rr.la, &mut rr.esc, &mut rr.ver] {
                        tl.fm = 0;
                        tl.fs = 0;
                        tl.fi = 0;
                    }
                }
                if a != b || !f_ok {
                    if std::env::var("B13_DEBUG").is_ok() {
                        eprintln!("DIFF {name} r={r} t={t} f_ok={f_ok}\n real {a:?}\n tabz {b:?}");
                    }
                    all_equal = false;
                    let _ = write!(detail, " [{name} r={r} t={t}: differ]");
                }
                if arm.neg && r1.bad > 0 || r1.bad > 0 {
                    all_equal = false;
                }
                if matches!(name, "base" | "min2" | "la" | "laper16" | "laper32" | "laper64") && !r1.solved {
                    solved_all = false;
                    let _ = write!(detail, " [{name} r={r} t={t}: unsolved]");
                }
            }
        }
    }
    check(
        "agreement",
        all_equal,
        format!("{runs} runs ({} arms x 2 r x 4 targets) identical in every counter, DP digest and recovered log; field ledger = constants x ops{detail}", ARM_NAMES.len()),
    );
    check("recovery", solved_all, "baseline, min-of-2, look-ahead and the three look-ahead+periodic arms recovered the planted log on every target at N~2^20 (no run returned a wrong log)".into());

    // 4. smoke at the smallest sweep cell: 2^40, r = 16
    let bits = 40u32;
    let n = prev_prime(1u64 << bits);
    let cfg = Cfg { n, r: 16, theta: theta_of(bits), cap_ops: cap_ops_of(n, 2.0) };
    println!("smoke cell 2^40 (N={n}), r=16, 6 targets, cap 2.0 x sqrt(pi N/2)");
    let sqrt_n = (n as f64).sqrt();
    let mut absorbed = 0;
    let mut base_ok = 0;
    let mut non_none_ok = true;
    for name in ARM_NAMES {
        let arm = arm_of(name).unwrap();
        let (mut solved, mut cens, mut ops, mut bad) = (0, 0, 0u64, 0u64);
        for t in 0..6u64 {
            let ts = tseed_of(bits, t, 0);
            let dd = pick_d(ts, n);
            let g = ZGroup { n, half: n / 2, d: dd };
            let rec = run_target(&g, &cfg, arm, ts, dd);
            solved += rec.solved as u32;
            cens += rec.censored as u32;
            ops += rec.walk.adds + rec.walk.dbls + rec.la.adds + rec.esc.adds + rec.esc.dbls + rec.start.adds;
            bad += rec.bad;
        }
        println!("  {name:8} solved {solved}/6 censored {cens}/6 mean ops/sqrt(N) {:.3} bad {bad}", ops as f64 / 6.0 / sqrt_n);
        if name == "none" {
            absorbed = cens;
        }
        if name == "base" {
            base_ok = solved;
        }
        if matches!(name, "per4" | "per8" | "per16" | "laper16" | "laper32" | "laper64") && solved < 5 {
            non_none_ok = false;
        }
        if bad > 0 {
            ok.set(false);
        }
    }
    check("positive control (absorption)", absorbed >= 5, format!("no-escape arm censored on {absorbed}/6 targets at r=16"));
    check("positive control (baseline)", base_ok >= 5, format!("baseline solved {base_ok}/6"));
    check("positive control (detecting arms)", non_none_ok, "windows of 4, 8, 16 and the three look-ahead+periodic arms solved at least 5 of 6 at r=16 (a window of k sees only cycles of length <= k, so the window of 2 is a 2-cycle-only rule like min-of-2 and dbl, which longer fruitless cycles absorb at r=16; windows of 32 and 64 lose to re-entry; those are not asserted, they are the effect under measurement)".into());
    // 4b. the same cell at r = 2048, where fruitless cycles are rare enough for 2-cycle rules
    let cfg2 = Cfg { n, r: 2048, theta: theta_of(bits), cap_ops: cap_ops_of(n, 2.0) };
    let mut r2048_ok = true;
    for name in ["base", "dbl", "min2", "la", "per16", "laper16"] {
        let arm = arm_of(name).unwrap();
        let mut solved = 0;
        for t in 0..6u64 {
            let ts = tseed_of(bits, t, 0);
            let dd = pick_d(ts, n);
            let g = ZGroup { n, half: n / 2, d: dd };
            let rec = run_target(&g, &cfg2, arm, ts, dd);
            solved += rec.solved as u32;
            if rec.bad > 0 {
                ok.set(false);
            }
        }
        println!("  r=2048 {name:8} solved {solved}/6");
        if solved < 5 {
            r2048_ok = false;
        }
    }
    check("positive control (r = 2048)", r2048_ok, "baseline, doubling escape, min-of-2, look-ahead, window 16 and look-ahead+16 solved at least 5 of 6 at r=2048".into());

    // 5. ideal random-map control: the detector sees sqrt(2)
    let n30 = prev_prime(1u64 << 30);
    let th = theta_of(30);
    let (mut sp, mut sc) = (0u64, 0u64);
    let k = 200u64;
    for t in 0..k {
        sp += ctl_run(n30, false, th, tseed_of(30, t, 0)).0;
        sc += ctl_run(n30, true, th, tseed_of(30, t, 0)).0;
    }
    let ratio = sp as f64 / sc as f64;
    check("ideal-map control", (1.2..1.65).contains(&ratio), format!("points/classes mean steps = {ratio:.3} over {k} targets at 2^30 (sqrt2 = 1.414)"));
    ok.get()
}

// --------------------------------------------------------------------- sweep

fn cmd_sweep(args: &[String]) {
    let bits = arg_u64(args, "--bits", 40) as u32;
    let t0 = arg_u64(args, "--t0", 0);
    let t1 = arg_u64(args, "--t1", 64);
    let cap_mult = arg_f64(args, "--cap-mult", 2.0);
    let max_seconds = arg_f64(args, "--max-seconds", 1e9);
    let out = arg(args, "--out").unwrap_or_else(|| die("--out required"));
    let arms: Vec<&str> = arg(args, "--arms").unwrap_or("base").split(',').collect();
    let rs: Vec<usize> = arg(args, "--rs").unwrap_or("16").split(',').map(|s| s.parse().unwrap_or_else(|_| die("bad --rs"))).collect();
    let offset = arg_u64(args, "--seed-offset", 0);
    let n = prev_prime(1u64 << bits);
    let theta = arg_u64(args, "--theta", theta_of(bits) as u64) as u32;
    let mut f = std::io::BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap_or_else(|e| die(&format!("{out}: {e}"))));
    let start = Instant::now();
    let mut lines = 0u64;
    for t in t0..t1 {
        let ts = tseed_of(bits, t, offset);
        let d = pick_d(ts, n);
        let g = ZGroup { n, half: n / 2, d };
        for &r in &rs {
            let cfg = Cfg { n, r, theta, cap_ops: cap_ops_of(n, cap_mult) };
            for name in &arms {
                let arm = arm_of(name).unwrap_or_else(|| die(&format!("unknown arm {name}")));
                let rec = run_target(&g, &cfg, arm, ts, d);
                writeln!(f, "{}", rec_json(bits, theta, r, name, t, &rec)).unwrap();
                lines += 1;
            }
        }
        f.flush().unwrap();
        if start.elapsed().as_secs_f64() > max_seconds {
            eprintln!("time guard: stopped after target {t} ({lines} records, {:.1}s)", start.elapsed().as_secs_f64());
            std::process::exit(4);
        }
    }
    f.flush().unwrap();
    eprintln!("sweep bits={bits} targets {t0}..{t1} arms {arms:?} rs {rs:?}: {lines} records in {:.1}s", start.elapsed().as_secs_f64());
}

fn cmd_ctl(args: &[String]) {
    let bits = arg_u64(args, "--bits", 40) as u32;
    let targets = arg_u64(args, "--targets", 64);
    let out = arg(args, "--out").unwrap_or_else(|| die("--out required"));
    let n = prev_prime(1u64 << bits);
    let theta = theta_of(bits);
    let mut f = std::io::BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap_or_else(|e| die(&format!("{out}: {e}"))));
    let start = Instant::now();
    for t in 0..targets {
        for (name, q) in [("ctl_pts", false), ("ctl_cls", true)] {
            let (steps, found) = ctl_run(n, q, theta, tseed_of(bits, t, 0));
            writeln!(f, "{{\"kind\":\"ctl\",\"bits\":{bits},\"arm\":\"{name}\",\"t\":{t},\"steps\":{steps},\"found\":{}}}", found as u8).unwrap();
        }
    }
    f.flush().unwrap();
    eprintln!("ctl bits={bits}: {targets} targets x 2 arms in {:.1}s", start.elapsed().as_secs_f64());
}

// ------------------------------------------------------------------- summary

#[derive(Clone, Default)]
struct Row {
    f: HashMap<String, f64>,
    s: HashMap<String, String>,
}

fn parse_line(line: &str) -> Option<Row> {
    let line = line.trim();
    if !line.starts_with('{') {
        return None;
    }
    let b = line.as_bytes();
    let mut row = Row::default();
    let mut i = 1;
    while i < b.len() {
        if b[i] == b'"' {
            let j = line[i + 1..].find('"')? + i + 1;
            let key = &line[i + 1..j];
            let mut k = j + 1;
            while k < b.len() && (b[k] == b':' || b[k] == b' ') {
                k += 1;
            }
            if b[k] == b'"' {
                let e = line[k + 1..].find('"')? + k + 1;
                row.s.insert(key.to_string(), line[k + 1..e].to_string());
                i = e + 1;
            } else {
                let mut e = k;
                while e < b.len() && b[e] != b',' && b[e] != b'}' {
                    e += 1;
                }
                row.f.insert(key.to_string(), line[k..e].trim().parse().ok()?);
                i = e;
            }
        } else {
            i += 1;
        }
    }
    Some(row)
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Model {
    name: &'static str,
    add: f64,
    dbl: f64,
    /// field multiplications per arrived point (hash event), and extra per canonicalisation.
    per_hash: f64,
    per_canon: f64,
    gae: f64,
}

/// Pricing models, all in field multiplications (a squaring is priced as a
/// multiplication; an inversion as kappa multiplications).
const MODELS: [Model; 3] = [
    // Jacobian mixed addition 8M+3S, doubling 4M+6S, inversion 30M; every arrived
    // point is normalised to affine x (I+S+M) and, under negation, to affine y (+2M).
    Model { name: "JAC30", add: 11.0, dbl: 10.0, per_hash: 32.0, per_canon: 2.0, gae: 43.0 },
    // Affine, inversion priced as 3M (Montgomery batch).
    Model { name: "AFF3", add: 6.0, dbl: 10.0, per_hash: 0.0, per_canon: 0.0, gae: 6.0 },
    // Affine, inversion priced as 30M (one inversion per operation).
    Model { name: "AFF30", add: 33.0, dbl: 37.0, per_hash: 0.0, per_canon: 0.0, gae: 33.0 },
];

/// Per-target cost vector under (model, weight of native work, include set-up).
fn cost(r: &Row, m: &Model, nu: f64, with_setup: bool) -> f64 {
    let g = |k: &str| *r.f.get(k).unwrap_or(&0.0);
    let adds = g("walk_a") + g("la_a") + g("esc_a") + g("start_a") + g("ver_a") + if with_setup { g("setup_a") } else { 0.0 };
    let dbls = g("walk_d") + g("la_d") + g("esc_d") + g("ver_d") + if with_setup { g("setup_d") } else { 0.0 };
    let field = adds * m.add + dbls * m.dbl + g("hash") * m.per_hash + g("canon") * m.per_canon;
    let native = g("hash") + g("canon") + g("cmp") + g("lookup");
    field + nu * m.gae * native
}

fn walk_ops(r: &Row) -> f64 {
    let g = |k: &str| *r.f.get(k).unwrap_or(&0.0);
    g("walk_a") + g("walk_d") + g("la_a") + g("la_d") + g("esc_a") + g("esc_d") + g("start_a")
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// Ratio of means with a paired bootstrap interval over targets.
fn paired_ratio(base: &[f64], arm: &[f64], rng: &mut Rng, reps: usize) -> (f64, f64, f64) {
    let n = base.len().min(arm.len());
    let point = mean(&base[..n]) / mean(&arm[..n]);
    let mut v = Vec::with_capacity(reps);
    for _ in 0..reps {
        let (mut sb, mut sa) = (0.0, 0.0);
        for _ in 0..n {
            let i = rng.below(n as u64) as usize;
            sb += base[i];
            sa += arm[i];
        }
        v.push(sb / sa);
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (point, v[(reps as f64 * 0.025) as usize], v[(reps as f64 * 0.975) as usize - 1])
}

fn unpaired_ratio(x: &[f64], y: &[f64], rng: &mut Rng, reps: usize) -> (f64, f64, f64) {
    let point = mean(x) / mean(y);
    let mut v = Vec::with_capacity(reps);
    for _ in 0..reps {
        let (mut sx, mut sy) = (0.0, 0.0);
        for _ in 0..x.len() {
            sx += x[rng.below(x.len() as u64) as usize];
        }
        for _ in 0..y.len() {
            sy += y[rng.below(y.len() as u64) as usize];
        }
        v.push((sx / x.len() as f64) / (sy / y.len() as f64));
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (point, v[(reps as f64 * 0.025) as usize], v[(reps as f64 * 0.975) as usize - 1])
}

type Key = (u32, u32, usize, String);

/// One (size, theta, r, rule) cell read out of the records.
#[allow(dead_code)]
struct Cell {
    bits: u32,
    theta: u32,
    r: usize,
    arm: String,
    targets: usize,
    solved: usize,
    wo: f64,
    wo_max: f64,
    g_walk: (f64, f64, f64),
    g_full: (f64, f64, f64),
    band: Vec<f64>,
}

impl Cell {
    fn band_min(&self) -> f64 {
        self.band.iter().cloned().fold(f64::MAX, f64::min)
    }
    fn band_max(&self) -> f64 {
        self.band.iter().cloned().fold(f64::MIN, f64::max)
    }
    fn tag(&self) -> String {
        format!("{} r={} (2^{}): {:.3} [{:.3}, {:.3}], band {:.3}..{:.3}", self.arm, self.r, self.bits, self.g_full.0, self.g_full.1, self.g_full.2, self.band_min(), self.band_max())
    }
}

/// Pricing band: three models times nu in {0, 0.1, 1}, fully charged, as point gains.
fn band_points(base: &[(u64, Row)], arm: &[(u64, Row)], n_t: usize) -> Vec<f64> {
    let mut v = Vec::new();
    for m in MODELS.iter() {
        for nu in [0.0, 0.1, 1.0] {
            let b = mean(&base[..n_t].iter().map(|x| cost(&x.1, m, nu, true)).collect::<Vec<_>>());
            let a = mean(&arm[..n_t].iter().map(|x| cost(&x.1, m, nu, true)).collect::<Vec<_>>());
            v.push(b / a);
        }
    }
    v
}

fn cmd_summary(args: &[String]) {
    let dir = arg(args, "--dir").unwrap_or_else(|| die("--dir required"));
    let out_md = arg(args, "--out-md").unwrap_or_else(|| die("--out-md required"));
    let out_json = arg(args, "--out-json").unwrap_or_else(|| die("--out-json required"));
    let reps = arg_u64(args, "--boot", 4000) as usize;
    let mut recs: HashMap<Key, Vec<(u64, Row)>> = HashMap::new();
    let mut ctls: HashMap<(u32, String), Vec<(u64, f64)>> = HashMap::new();
    let mut files = 0;
    let mut names: Vec<_> = std::fs::read_dir(dir).unwrap_or_else(|e| die(&format!("{dir}: {e}"))).filter_map(|e| e.ok()).map(|e| e.path()).collect();
    names.sort();
    for p in names {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if !(name.starts_with("sweep-") || name.starts_with("ctl-")) || !name.ends_with(".jsonl") {
            continue;
        }
        files += 1;
        for line in std::fs::read_to_string(&p).unwrap().lines() {
            let Some(row) = parse_line(line) else { continue };
            let bits = row.f["bits"] as u32;
            let t = row.f["t"] as u64;
            match row.s.get("kind").map(|s| s.as_str()) {
                Some("rec") => {
                    let key = (bits, row.f["theta"] as u32, row.f["r"] as usize, row.s["arm"].clone());
                    recs.entry(key).or_default().push((t, row));
                }
                Some("ctl") => ctls.entry((bits, row.s["arm"].clone())).or_default().push((t, row.f["steps"])),
                _ => {}
            }
        }
    }
    for v in recs.values_mut() {
        v.sort_by_key(|x| x.0);
        v.dedup_by_key(|x| x.0);
    }
    for v in ctls.values_mut() {
        v.sort_by_key(|x| x.0 as u64);
        v.dedup_by_key(|x| x.0);
    }
    let mut sections: Vec<(u32, u32)> = recs.keys().map(|k| (k.0, k.1)).collect();
    sections.sort();
    sections.dedup();
    let mut md = String::new();
    let mut js = String::new();
    let mut rng = Rng::new(0x5EED_13);
    let _ = writeln!(md, "# Item 13 part A summary (mechanical read-out of the frozen rule)\n");
    let _ = writeln!(md, "Files read: {files}. Bootstrap resamples: {reps}. Gain = mean cost of the baseline / mean cost of the rule, paired by target, 95% percentile interval, fully charged (set-up and verification included) under the JAC30 model with native work at weight 0 unless a column says otherwise. `band` is the minimum and maximum point gain over the nine pricing points (JAC30, AFF3, AFF30 times native-work weights 0, 0.1, 1 group-addition equivalents). `adm` marks collision-preserving escape rules, the only ones counted as winners. A dagger marks a cell where the rule left a target unsolved at the budget cap (its cost is then a lower bound, its gain an upper bound). Sections whose theta is not bits/4 are extensions and do not enter the frozen rule.\n");
    let _ = writeln!(js, "{{\"cells\":[");
    let mut first_js = true;
    let mut cells: Vec<Cell> = Vec::new();
    let mut gate_rows: Vec<String> = Vec::new();
    let mut rate_rows: Vec<String> = Vec::new();
    let mut rate_by: HashMap<(String, usize), Vec<(u32, f64)>> = HashMap::new();
    let mut baseline_gate_ok = true;
    let mut baseline_gate_seen = false;
    let mut none_absorbed_ok = true;
    let mut none_seen = false;
    let mut aa_total = 0usize;
    let mut aa_cover = 0usize;
    let mut ctl_ok = true;
    for &(bits, theta) in &sections {
        let default = theta == theta_of(bits);
        let n = prev_prime(1u64 << bits);
        let sqrt_n = (n as f64).sqrt();
        let mut rs: Vec<usize> = recs.keys().filter(|k| k.0 == bits && k.1 == theta).map(|k| k.2).collect();
        rs.sort();
        rs.dedup();
        let _ = writeln!(md, "\n## 2^{bits}, theta = {theta}{} (N = {n}, walk cap {})\n", if default { "" } else { " (extension)" }, 20 * (1u64 << theta) + 64);
        if default {
            // C1: the baseline, pooled over the near-ideal r, against the ideal random-map control
            let mut pool: Vec<f64> = Vec::new();
            for &r in &rs {
                if r < 128 {
                    continue;
                }
                for nm in ["base", "baseb"] {
                    if let Some(v) = recs.get(&(bits, theta, r, nm.to_string())) {
                        pool.extend(v.iter().map(|x| walk_ops(&x.1) / sqrt_n));
                    }
                }
            }
            if !pool.is_empty() {
                baseline_gate_seen = true;
                let m = mean(&pool);
                match ctls.get(&(bits, "ctl_pts".to_string())) {
                    Some(c) => {
                        let cv: Vec<f64> = c.iter().map(|v| v.1 / sqrt_n).collect();
                        let g = unpaired_ratio(&pool, &cv, &mut rng, reps);
                        let ok = g.2 >= 0.92 && g.1 <= 1.08;
                        baseline_gate_ok &= ok;
                        gate_rows.push(format!("2^{bits}: baseline (r >= 128, base and baseb pooled, {} targets) mean walk ops / sqrt(N) = {m:.3}; ideal random-map control (ctl_pts, {} targets) mean = {:.3}; ratio {:.3} [{:.3}, {:.3}] must overlap the band [0.92, 1.08]; sqrt(pi/2) = 1.253 for reference (the KN-TECH-006 figure 0.886 is sqrt(pi/4), the ideal negation cost, not a plain-walk cost): {}", pool.len(), cv.len(), mean(&cv), g.0, g.1, g.2, if ok { "PASS" } else { "FAIL" }));
                    }
                    None => {
                        baseline_gate_ok = false;
                        gate_rows.push(format!("2^{bits}: baseline mean walk ops / sqrt(N) = {m:.3}; no ideal random-map control records for this size: FAIL"));
                    }
                }
            }
        }
        for &r in &rs {
            let Some(base) = recs.get(&(bits, theta, r, "base".to_string())) else { continue };
            let _ = writeln!(md, "\n### 2^{bits}, theta = {theta}, r = {r} ({} baseline targets)\n", base.len());
            let _ = writeln!(md, "| rule | adm | solved | mean ops / sqrt N | max ops / sqrt N | gain walk-only [95%] | gain fully charged [95%] | band min..max | AFF3 nu=0 | JAC30 nu=0.1 | JAC30 nu=1 |");
            let _ = writeln!(md, "|:--|:-:|--:|--:|--:|:--|:--|:--|--:|--:|--:|");
            for name in ARM_NAMES.iter().skip(1) {
                let Some(armv) = recs.get(&(bits, theta, r, name.to_string())) else { continue };
                let n_t = base.len().min(armv.len());
                let col = |v: &Vec<(u64, Row)>, m: &Model, nu: f64, su: bool| -> Vec<f64> { v[..n_t].iter().map(|x| cost(&x.1, m, nu, su)).collect() };
                let solved = armv.iter().filter(|x| x.1.f["solved"] > 0.5).count();
                let dag = if solved < armv.len() { "†" } else { "" };
                let wov: Vec<f64> = armv.iter().map(|x| walk_ops(&x.1) / sqrt_n).collect();
                let wo = mean(&wov);
                let wo_max = wov.iter().cloned().fold(0.0, f64::max);
                let g_walk = paired_ratio(&col(base, &MODELS[0], 0.0, false), &col(armv, &MODELS[0], 0.0, false), &mut rng, reps);
                let g_full = paired_ratio(&col(base, &MODELS[0], 0.0, true), &col(armv, &MODELS[0], 0.0, true), &mut rng, reps);
                let band = band_points(base, armv, n_t);
                let pt = |m: &Model, nu: f64| mean(&col(base, m, nu, true)) / mean(&col(armv, m, nu, true));
                let (a3_0, j_01, j_1) = (pt(&MODELS[1], 0.0), pt(&MODELS[0], 0.1), pt(&MODELS[0], 1.0));
                let cell = Cell { bits, theta, r, arm: name.to_string(), targets: armv.len(), solved, wo, wo_max, g_walk, g_full, band };
                let _ = writeln!(
                    md,
                    "| {name} | {} | {solved}/{}{dag} | {wo:.3} | {wo_max:.3} | {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}] | {:.3}..{:.3} | {a3_0:.3} | {j_01:.3} | {j_1:.3} |",
                    if admissible(name) { "y" } else { "n" },
                    armv.len(),
                    g_walk.0, g_walk.1, g_walk.2, g_full.0, g_full.1, g_full.2, cell.band_min(), cell.band_max()
                );
                if !first_js {
                    js.push_str(",\n");
                }
                first_js = false;
                let bandj: Vec<String> = cell.band.iter().map(|x| format!("{x:.6}")).collect();
                let _ = write!(js, "{{\"bits\":{bits},\"theta\":{theta},\"r\":{r},\"arm\":\"{name}\",\"admissible\":{},\"targets\":{},\"solved\":{solved},\"ops_over_sqrtN\":{wo:.6},\"max_ops_over_sqrtN\":{wo_max:.6},\"gain_walk\":[{:.6},{:.6},{:.6}],\"gain_full\":[{:.6},{:.6},{:.6}],\"band_order\":\"JAC30,AFF3,AFF30 x nu 0,0.1,1\",\"band\":[{}]}}", admissible(name), armv.len(), g_walk.0, g_walk.1, g_walk.2, g_full.0, g_full.1, g_full.2, bandj.join(","));
                cells.push(cell);
                if default && ["la", "min2", "dbl", "per16"].contains(name) {
                    let e: f64 = armv.iter().map(|x| x.1.f["c2e"]).sum();
                    let t: f64 = armv.iter().map(|x| x.1.f["c2t"]).sum();
                    let ea: f64 = armv.iter().map(|x| x.1.f.get("c2ea").copied().unwrap_or(0.0)).sum();
                    let ta: f64 = armv.iter().map(|x| x.1.f.get("c2ta").copied().unwrap_or(0.0)).sum();
                    if t > 0.0 {
                        let pred = 1.0 / (2.0 * r as f64);
                        let rate = e / t;
                        rate_by.entry((name.to_string(), r)).or_default().push((bits, rate));
                        rate_rows.push(format!("| 2^{bits} | {r} | {name} | {rate:.5} | {pred:.5} | {:.3} | {e} / {t} | {} | {:.3} |", rate / pred, if (0.5..=2.0).contains(&(rate / pred)) { "within factor 2" } else { "OUTSIDE" }, if ta > 0.0 { ea / ta / pred } else { 0.0 }));
                    }
                }
                if default && *name == "none" {
                    let cap: f64 = armv.iter().map(|x| x.1.f["capped"]).sum();
                    let w: f64 = armv.iter().map(|x| x.1.f["walks"]).sum();
                    let pred = 1.0 - (-(2f64.powi(theta as i32)) / (2.0 * r as f64)).exp();
                    let frac = cap / w;
                    let ok = frac >= 0.5 * pred;
                    if r == 16 {
                        none_seen = true;
                        none_absorbed_ok &= ok && solved == 0;
                    }
                    gate_rows.push(format!("2^{bits} r={r}: no-escape rule capped-walk fraction {frac:.3} (model 1-exp(-2^theta/(2r)) = {pred:.3}), targets solved {solved}/{}: {}", armv.len(), if ok { "absorbed as modelled" } else { "NOT ABSORBED" }));
                }
            }
            // A/A negative control
            if default {
                if let Some(bb) = recs.get(&(bits, theta, r, "baseb".to_string())) {
                    let a: Vec<f64> = base.iter().map(|x| cost(&x.1, &MODELS[0], 0.0, false)).collect();
                    let b: Vec<f64> = bb.iter().map(|x| cost(&x.1, &MODELS[0], 0.0, false)).collect();
                    let g = unpaired_ratio(&a, &b, &mut rng, reps);
                    let ok = g.1 <= 1.0 && g.2 >= 1.0;
                    aa_total += 1;
                    aa_cover += ok as usize;
                    gate_rows.push(format!("2^{bits} r={r}: A/A baseline ratio {:.3} [{:.3}, {:.3}] ({} vs {} targets): {}", g.0, g.1, g.2, a.len(), b.len(), if ok { "interval covers 1" } else { "INTERVAL EXCLUDES 1" }));
                }
            }
        }
    }
    let _ = writeln!(js, "\n]}}");
    // ideal-map control
    let mut ctl_sizes: Vec<u32> = ctls.keys().map(|k| k.0).collect();
    ctl_sizes.sort();
    ctl_sizes.dedup();
    for &bits in &ctl_sizes {
        if let (Some(p), Some(c)) = (ctls.get(&(bits, "ctl_pts".to_string())), ctls.get(&(bits, "ctl_cls".to_string()))) {
            let x: Vec<f64> = p.iter().map(|v| v.1).collect();
            let y: Vec<f64> = c.iter().map(|v| v.1).collect();
            let g = unpaired_ratio(&x, &y, &mut rng, reps);
            let ok = g.1 <= std::f64::consts::SQRT_2 && g.2 >= std::f64::consts::SQRT_2;
            ctl_ok &= ok;
            gate_rows.push(format!("2^{bits}: ideal random-map control, points/classes steps ratio {:.3} [{:.3}, {:.3}] ({} targets), sqrt2 = 1.414: {}", g.0, g.1, g.2, x.len(), if ok { "interval covers sqrt2" } else { "INTERVAL EXCLUDES sqrt2" }));
        }
    }
    let _ = writeln!(md, "\n## Instrument gates and controls\n");
    for r in &gate_rows {
        let _ = writeln!(md, "- {r}");
    }
    let _ = writeln!(md, "\n## Per-step fruitless-cycle entry rate (prediction 3, HEUR-RHO-1)\n\nThe primary estimator counts entries and draws over walks that ended at a distinguished point (steps spent inside a longer trapped cycle are not draws of the 2-cycle probability). The last column is the same ratio over every step, trapped walks included.\n\n| size | r | rule | measured per-step rate | 1/(2r) | ratio | entries / draws | verdict | ratio, all steps |\n|:--|--:|:--|--:|--:|--:|:--|:--|--:|");
    for r in &rate_rows {
        let _ = writeln!(md, "{r}");
    }
    let mut rate_keys: Vec<_> = rate_by.keys().cloned().collect();
    rate_keys.sort();
    let _ = writeln!(md, "\nSpread of the measured rate across sizes (max / min over the sizes present), the N-dependence check of prediction 3:\n");
    for k in &rate_keys {
        let v = &rate_by[k];
        if v.len() >= 2 {
            let mx = v.iter().map(|x| x.1).fold(0.0, f64::max);
            let mn = v.iter().map(|x| x.1).fold(f64::MAX, f64::min);
            let _ = writeln!(md, "- {} r={}: {:.3}", k.0, k.1, mx / mn);
        }
    }
    // size dependence (HEUR-RHO-2)
    let _ = writeln!(md, "\n## Size dependence of the fully charged gain (HEUR-RHO-2)\n\nA rule/r pair is flagged when the 95% intervals of its smallest and largest size are disjoint.\n\n| rule | r | gains by size (point) | flag |\n|:--|--:|:--|:--|");
    let mut keys: Vec<(String, usize)> = cells.iter().filter(|c| c.theta == theta_of(c.bits)).map(|c| (c.arm.clone(), c.r)).collect();
    keys.sort();
    keys.dedup();
    for (arm, r) in &keys {
        let mut v: Vec<&Cell> = cells.iter().filter(|c| c.theta == theta_of(c.bits) && &c.arm == arm && c.r == *r).collect();
        v.sort_by_key(|c| c.bits);
        if v.len() >= 2 {
            let (lo, hi) = (v[0], v[v.len() - 1]);
            let disjoint = lo.g_full.2 < hi.g_full.1 || hi.g_full.2 < lo.g_full.1;
            let _ = writeln!(md, "| {arm} | {r} | {} | {} |", v.iter().map(|c| format!("2^{}: {:.3}", c.bits, c.g_full.0)).collect::<Vec<_>>().join(", "), if disjoint { "intervals disjoint" } else { "overlap" });
        }
    }
    // frozen rule
    let def: Vec<&Cell> = cells.iter().filter(|c| c.theta == theta_of(c.bits)).collect();
    let mut sizes: Vec<u32> = def.iter().map(|c| c.bits).collect();
    sizes.sort();
    sizes.dedup();
    let _ = writeln!(md, "\n## Best cell per size and rule (fully charged JAC30, native work 0; collision-preserving rules marked adm)\n\n| size | rule | adm | best r | gain [95%] | band min..max |\n|:--|:--|:-:|--:|:--|:--|");
    for &bits in &sizes {
        for name in ARM_NAMES.iter().skip(1) {
            let mut best: Option<&Cell> = None;
            for c in def.iter().filter(|c| c.bits == bits && c.arm == *name) {
                if best.map(|b| c.g_full.0 > b.g_full.0).unwrap_or(true) {
                    best = Some(c);
                }
            }
            if let Some(c) = best {
                let _ = writeln!(md, "| 2^{bits} | {name} | {} | {} | {:.3} [{:.3}, {:.3}] | {:.3}..{:.3} |", if admissible(name) { "y" } else { "n" }, c.r, c.g_full.0, c.g_full.1, c.g_full.2, c.band_min(), c.band_max());
            }
        }
    }
    let _ = writeln!(md, "\n## Frozen rule, read mechanically\n");
    let ctl_present = !ctl_sizes.is_empty();
    let _ = writeln!(md, "- Instrument gate C1 (baseline against the ideal random-map control at every size): {}", if baseline_gate_seen && baseline_gate_ok { "PASS" } else { "FAIL or absent, no comparison may be reported" });
    let _ = writeln!(md, "- Ideal random-map control sees sqrt2 at every size: {}", if ctl_present && ctl_ok { "yes" } else { "NO or absent" });
    let _ = writeln!(md, "- A/A negative control (baseline against a second independent baseline sample): the 95% interval covers 1 in {aa_cover} of {aa_total} cells; the gate passes with at most one miss (a 95% interval misses about one cell in twenty by chance): {}", if aa_total > 0 && aa_total - aa_cover <= 1 { "PASS" } else { "FAIL" });
    let _ = writeln!(md, "- No-escape control absorbed at r = 16 at every size (no target solved, capped-walk fraction as modelled): {}", if none_seen && none_absorbed_ok { "yes" } else { "NO or absent (falsification condition 2)" });
    let adm: Vec<&&Cell> = def.iter().filter(|c| admissible(&c.arm)).collect();
    let mut fav_all = true;
    let mut rob_all = true;
    for &bits in &sizes {
        let sz: Vec<&&&Cell> = adm.iter().filter(|c| c.bits == bits).collect();
        let fav: Vec<String> = sz.iter().filter(|c| c.band_max() >= 1.35).map(|c| c.tag()).collect();
        let rob: Vec<String> = sz.iter().filter(|c| c.band_min() >= 1.35 && c.g_full.1 >= 1.35 && c.solved == c.targets).map(|c| c.tag()).collect();
        let loss: Vec<String> = sz.iter().filter(|c| c.g_full.2 < 1.0).map(|c| c.tag()).collect();
        let lossr: Vec<String> = sz.iter().filter(|c| c.g_full.2 < 1.0 && c.band_max() < 1.0).map(|c| c.tag()).collect();
        let top = sz.iter().max_by(|a, b| a.g_full.0.partial_cmp(&b.g_full.0).unwrap()).map(|c| c.tag()).unwrap_or_default();
        let nonadm: Vec<String> = def.iter().filter(|c| c.bits == bits && !admissible(&c.arm) && c.arm != "none" && c.band_max() >= 1.35).map(|c| c.tag()).collect();
        fav_all &= !fav.is_empty();
        rob_all &= !rob.is_empty();
        let j = |v: &Vec<String>| if v.is_empty() { "none".to_string() } else { v.join("; ") };
        let _ = writeln!(md, "- 2^{bits}: best collision-preserving cell: {top}");
        let _ = writeln!(md, "  - cells whose point gain reaches 1.35 under the most favourable of the nine pricing points: {}", j(&fav));
        let _ = writeln!(md, "  - cells whose gain reaches 1.35 at every pricing point and whose 95% lower bound is >= 1.35: {}", j(&rob));
        let _ = writeln!(md, "  - cells with a clear net loss (JAC30, native 0, 95% upper bound < 1.0): {}", j(&loss));
        let _ = writeln!(md, "  - cells with a clear net loss at every pricing point as well: {}", j(&lossr));
        let _ = writeln!(md, "  - non-collision-preserving rules (reported, not counted) at or above 1.35 under the favourable pricing: {}", j(&nonadm));
    }
    let loss_every = |robust: bool| -> Vec<String> {
        let mut out = Vec::new();
        let mut ks: Vec<(String, usize)> = adm.iter().map(|c| (c.arm.clone(), c.r)).collect();
        ks.sort();
        ks.dedup();
        for (arm, r) in ks {
            let ok = !sizes.is_empty() && sizes.iter().all(|&b| adm.iter().any(|c| c.bits == b && c.arm == arm && c.r == r && c.g_full.2 < 1.0 && (!robust || c.band_max() < 1.0)));
            if ok {
                out.push(format!("{arm} r={r}"));
            }
        }
        out
    };
    let la = loss_every(false);
    let lr = loss_every(true);
    let _ = writeln!(md, "- Settings (rule, r) with a clear net loss at every size: {}", if la.is_empty() { "none; the exact words for the tested range are: no sign change inside the tested range".to_string() } else { la.join("; ") });
    let _ = writeln!(md, "- The same, also at every pricing point: {}", if lr.is_empty() { "none".to_string() } else { lr.join("; ") });
    let _ = writeln!(md, "- At every size some collision-preserving cell reaches 1.35 under favourable pricing: {}", if fav_all && !sizes.is_empty() { "yes" } else { "no" });
    let _ = writeln!(md, "- At every size some collision-preserving cell robustly reaches 1.35: {}", if rob_all && !sizes.is_empty() { "yes" } else { "no" });
    let _ = writeln!(md, "\nThe verdict words (SUPPORTED, REFUTED, INCONCLUSIVE; ADVANCES, TRADES, MATCHES, REGRESSES at the constant level) are assigned from these lines by the assess stage under PROTOCOL.md section \"Decision rule\".");
    std::fs::write(out_md, md).unwrap();
    std::fs::write(out_json, js).unwrap();
    println!("wrote {out_md} and {out_json}");
}

// ---------------------------------------------------------------------- plan

fn parse_rates(path: &str) -> Vec<f64> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    text.lines()
        .filter_map(|l| {
            let l = l.trim();
            let rest = l.strip_prefix("finished:")?;
            rest.trim().split_whitespace().next()?.parse::<f64>().ok()
        })
        .collect()
}

fn cmd_plan(args: &[String]) {
    let logs: Vec<&str> = arg(args, "--cpu-logs").unwrap_or_else(|| die("--cpu-logs a,b,c required")).split(',').collect();
    let out_md = arg(args, "--out-md").unwrap_or_else(|| die("--out-md required"));
    let out_json = arg(args, "--out-json").unwrap_or_else(|| die("--out-json required"));
    let mut rates: Vec<f64> = Vec::new();
    for l in &logs {
        rates.extend(parse_rates(l));
    }
    if rates.is_empty() {
        die("no 'finished: X M it/s' line in the CPU logs");
    }
    let mut sorted = rates.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let med = sorted[sorted.len() / 2] * 1e6;
    let (lo, hi) = (sorted[0] * 1e6, sorted[sorted.len() - 1] * 1e6);
    // Committed inputs.  Each carries its source.
    let w_log2 = 60.9f64;
    let w = 2f64.powf(w_log2);
    let dp32_log2 = 28.41f64;
    let dp34_log2 = 25.27f64;
    let dp_bytes = 32.0f64;
    struct Gpu {
        name: &'static str,
        rate: f64,
        /// Iterations per solve of this walk relative to the sigma walk's 2^60.9.
        work_factor: f64,
        src: &'static str,
        /// Indexes into `prices` that name this GPU part.
        price_ix: &'static [usize],
    }
    struct Price {
        name: &'static str,
        usd_h: f64,
        src: &'static str,
    }
    let prices = [
        Price { name: "AWS g7e.2xlarge spot, us-west-2 (RTX PRO 6000)", usd_h: 1.2128, src: "ecc2k130/RTX-PRO4500.md, observed 2026-09-16" },
        Price { name: "AWS g7e spot per GPU-hour (RTX PRO 6000)", usd_h: 1.31, src: "ecc2k130/FPGA-CEILING.md section 5 (prices dated 2026-09-11)" },
        Price { name: "AWS g7e.48xlarge on-demand per GPU-hour (RTX PRO 6000)", usd_h: 4.14, src: "ecc2k130/FPGA-CEILING.md section 5" },
        Price { name: "Modal list, RTX PRO 6000", usd_h: 3.03, src: "ecc2k130/B200.md" },
        Price { name: "AWS g7.2xlarge spot, us-west-2 (RTX PRO 4500)", usd_h: 0.5705, src: "ecc2k130/RTX-PRO4500.md, observed 2026-09-16" },
    ];
    let gpus = [
        Gpu {
            name: "RTX PRO 6000, sigma walk with the shared-memory sigma-table kernel (the campaign's shipping iteration function)",
            rate: 15.884369e9,
            work_factor: 1.0,
            src: "ecc2k130/benchmarks/sigma-table/headline/result.json candidate_median_million_per_second 15884.369; ecc2k130/README.md",
            price_ix: &[0, 1, 2, 3],
        },
        Gpu {
            name: "RTX PRO 4500, promoted x-only two-bridge map (a different iteration function; work factor 1.006696 from its frozen 2,000-trial collision study)",
            rate: 12.499203e9,
            work_factor: 1.006696,
            src: "ecc2k130/README.md (12.499203 B/s paired median; 1.006696 mean-work ratio, benchmarks/g7-12b-xonly-doubling-bridge)",
            price_ix: &[4],
        },
    ];
    let mut md = String::new();
    let mut js = String::new();
    let _ = writeln!(md, "# Item 13 part B: costed plan for completing ECC2K-130\n");
    let _ = writeln!(md, "## Measured CPU rate of the current `ecc2k130/` client\n");
    let _ = writeln!(md, "{} repetitions of `ecc2k130-cpu --curve 131 --bench --threads 1`, one thread each, built from the committed `ecc2k130/src/main.cu` by the card, on this host (shared 4-vCPU container, isolation level L0, load recorded in raw/): {} M iterations/s.", rates.len(), rates.iter().map(|r| format!("{r:.3}")).collect::<Vec<_>>().join(", "));
    let _ = writeln!(md, "\nMedian **{:.3} M iterations/s per core**, min {:.3}, max {:.3}. Committed single-core figures for comparison (ecc2k130/README.md Status): 12.6 M/s per AVX-512 core, 15.7 M/s per M4 Pro core. This is the shipping sigma walk's CPU rate; it is not a GPU rate and not a prediction of one.\n", med / 1e6, lo / 1e6, hi / 1e6);
    let _ = writeln!(md, "## Work\n");
    let _ = writeln!(md, "Expected iterations to solve: 2^{w_log2} = {w:.4e} (ecc2k130/README.md; measured for the sigma walk 2^60.91 to 2^60.92 on completed trails, ecc2k130/WALK-CONSTANT.md). Iterations per distinguished point: 2^{dp32_log2} at weight 32 (the live bucket, ecc2k130/benchmarks/dp-interval/README.md, 132 workers), 2^{dp34_log2} at weight 34 (Bailey et al., same file). Record: {dp_bytes} bytes per point (ecc2k130 `--dp-file`).\n");
    let dps32 = w / 2f64.powf(dp32_log2);
    let dps34 = w / 2f64.powf(dp34_log2);
    let _ = writeln!(md, "| distinguished-point weight | points to collect (expected) | storage at 32 bytes |\n|:--|--:|--:|");
    let _ = writeln!(md, "| 32 | {dps32:.3e} (2^{:.2}) | {:.1} GB |", dps32.log2(), dps32 * dp_bytes / 1e9);
    let _ = writeln!(md, "| 34 | {dps34:.3e} (2^{:.2}) | {:.2} TB |", dps34.log2(), dps34 * dp_bytes / 1e12);
    let _ = writeln!(md, "\nAn in-memory collision table needs at least the 8-byte key of every point: {:.0} GB at weight 32, {:.0} GB at weight 34. A sorted-run or disk-backed table with 32-byte records needs the storage column.\n", dps32 * 8.0 / 1e9, dps34 * 8.0 / 1e9);
    let _ = writeln!(md, "## CPU alone\n");
    let cpu_h = w / med / 3600.0;
    let _ = writeln!(md, "{cpu_h:.3e} core-hours = {:.3e} core-years at the measured median rate.\n", cpu_h / 8766.0);
    let _ = writeln!(js, "{{\"cpu_rate_median\":{med:.1},\"cpu_rate_min\":{lo:.1},\"cpu_rate_max\":{hi:.1},\"cpu_reps\":{},\"work_iterations\":{w:.6e},\"dp32_points\":{dps32:.6e},\"dp34_points\":{dps34:.6e},\"dp32_bytes\":{:.6e},\"dp34_bytes\":{:.6e},\"cpu_core_hours\":{cpu_h:.6e},\"gpu_rows\":[", rates.len(), dps32 * dp_bytes, dps34 * dp_bytes);
    let _ = writeln!(md, "## GPU hours and cost (expected work)\n");
    let _ = writeln!(md, "GPU-hours = work factor x 2^60.9 / (rate x 3600). Cost = GPU-hours x the price of the same GPU part. Every price is an input from a committed observation with its date; no live quote was available to this run. Rates are the committed GPU measurements, not measured here (no GPU exists on this host).\n");
    let _ = writeln!(md, "| GPU row | rate (B it/s) | work factor | GPU-hours | CPU core-hours per GPU-hour | cheapest listed price ($/h) | break-even CPU core-hour price |\n|:--|--:|--:|--:|--:|--:|--:|");
    for (i, g) in gpus.iter().enumerate() {
        let gh = g.work_factor * w / g.rate / 3600.0;
        let cheapest = g.price_ix.iter().map(|&k| prices[k].usd_h).fold(f64::MAX, f64::min);
        let be = cheapest * med / g.rate;
        let _ = writeln!(md, "| {} | {:.6} | {:.6} | {gh:.0} | {:.0} | {cheapest:.4} | ${be:.5} |", g.name, g.rate / 1e9, g.work_factor, g.rate / med);
        let _ = write!(js, "{}{{\"name\":\"{}\",\"rate\":{:.1},\"work_factor\":{},\"gpu_hours\":{gh:.1},\"src\":\"{}\"}}", if i > 0 { "," } else { "" }, g.name, g.rate, g.work_factor, g.src);
    }
    let _ = writeln!(js, "],\"costs\":[");
    let _ = writeln!(md, "\n| GPU row | price row | $/h | total cost, expected work (USD) |\n|:--|:--|--:|--:|");
    let mut first = true;
    for g in &gpus {
        let gh = g.work_factor * w / g.rate / 3600.0;
        for &k in g.price_ix {
            let p = &prices[k];
            let _ = writeln!(md, "| {} | {} | {:.4} | {:.0} |", g.name.split(',').next().unwrap(), p.name, p.usd_h, gh * p.usd_h);
            let _ = write!(js, "{}{{\"gpu\":\"{}\",\"price\":\"{}\",\"usd_h\":{},\"usd\":{:.1},\"price_src\":\"{}\"}}", if first { "" } else { "," }, g.name, p.name, p.usd_h, gh * p.usd_h, p.src);
            first = false;
        }
    }
    let _ = writeln!(js, "],\"quantiles\":[");
    // Rayleigh model of the first-collision work: quantile q of the work in units of its mean.
    let _ = writeln!(md, "\n## Work to complete at a stated probability (modelled, not measured)\n\nThe expected work is a mean; the first-collision work of a random map follows approximately a Rayleigh law, whose p-quantile in units of the mean is sqrt(-2 ln(1-p)) / sqrt(pi/2). This is a model (the classical random-map law the corpus assumes), not a measurement of this walk's tail.\n\n| probability of having solved | work, multiple of the mean | iterations | sigma-table GPU-hours (RTX PRO 6000) | cost at the lowest RTX PRO 6000 price (USD) |\n|--:|--:|--:|--:|--:|");
    let g0 = &gpus[0];
    let low0 = g0.price_ix.iter().map(|&k| prices[k].usd_h).fold(f64::MAX, f64::min);
    for (i, p) in [0.5f64, 0.9, 0.99].iter().enumerate() {
        let k = (-2.0 * (1.0 - p).ln()).sqrt() / (std::f64::consts::PI / 2.0).sqrt();
        let gh = k * w / g0.rate / 3600.0;
        let _ = writeln!(md, "| {p} | {k:.3} | {:.3e} | {gh:.0} | {:.0} |", k * w, gh * low0);
        let _ = write!(js, "{}{{\"p\":{p},\"multiple_of_mean\":{k:.6},\"gpu_hours\":{gh:.1},\"usd\":{:.1}}}", if i > 0 { "," } else { "" }, gh * low0);
    }
    let _ = writeln!(js, "]}}");
    let _ = writeln!(md, "\nSources of the rates and prices:");
    for g in &gpus {
        let _ = writeln!(md, "- {}: {}", g.name, g.src);
    }
    for p in &prices {
        let _ = writeln!(md, "- {}: {}", p.name, p.src);
    }
    let _ = writeln!(md, "\nNot used as a completion rate: the 20.078 B/s table-walk figure (ecc2k130/ONE-BLOCK-GEOMETRY.md) was measured under the superseded v2 cycle rule and does not transfer to the v3 table walk, which measures about 5.1 B/s (ecc2k130/README.md); the sigma walk is the faster one.");
    let _ = writeln!(md, "\n## Wall clock\n\n| GPUs (sigma-table, 15.884 B/s each) | days at expected work |\n|--:|--:|");
    for fleet in [100.0f64, 1000.0, 10000.0] {
        let _ = writeln!(md, "| {fleet:.0} | {:.1} |", w / (gpus[0].rate * fleet) / 86400.0);
    }
    let _ = writeln!(md, "\n## What this plan claims and does not\n\nIt claims the CPU rate measured above and arithmetic on the committed GPU rates, work constants, record size and price observations listed with their sources. It claims no GPU rate measured here, no live price, no campaign progress credit (distinguished points already collected are not subtracted), and nothing about the collision-finding cost beyond the storage lines. The weight-34 interval is the committed 2^25.27 and the weight-32 interval the measured 2^28.41. The Rayleigh quantiles are a model. Network, orchestration, preemption and data-transfer costs are not priced.");
    std::fs::write(out_md, md).unwrap();
    std::fs::write(out_json, js).unwrap();
    println!("wrote {out_md} and {out_json}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("selftest") => {
            let t = Instant::now();
            let ok = selftest();
            println!("selftest {} in {:.1}s", if ok { "OK" } else { "FAILED" }, t.elapsed().as_secs_f64());
            std::process::exit(if ok { 0 } else { 1 });
        }
        Some("sweep") => cmd_sweep(&args),
        Some("ctl") => cmd_ctl(&args),
        Some("summary") => cmd_summary(&args),
        Some("plan") => cmd_plan(&args),
        _ => die("usage: b13 selftest | sweep | ctl | summary | plan"),
    }
}

#[allow(dead_code)]
fn _unused(r: &mut Rng) -> f64 {
    r.unit()
}
