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

const ARM_NAMES: [&str; 11] = [
    "base", "none", "dbl", "min2", "la", "per16", "per32", "per64", "laper16", "laper32", "laper64",
];

fn arm_of(name: &str) -> Option<Arm> {
    let (neg, rule) = match name {
        "base" | "baseb" => (false, Rule::None),
        "none" => (true, Rule::None),
        "dbl" => (true, Rule::Dbl),
        "min2" => (true, Rule::Min2),
        "la" => (true, Rule::La),
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

    fn walk(&self, st: &mut St, sx: G::P, sa: u64, sb: u64) -> End<G::P> {
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
                    st.rec.c2_trials += 1;
                }
                if ev && fresh {
                    st.rec.c2_entries += 1;
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

fn rec_json(bits: u32, r: usize, arm: &str, t: u64, rec: &Rec) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "{{\"kind\":\"rec\",\"bits\":{bits},\"r\":{r},\"arm\":\"{arm}\",\"t\":{t},\
\"solved\":{},\"cens\":{},\"found\":{},\
\"setup_a\":{},\"setup_d\":{},\"start_a\":{},\"walk_a\":{},\"walk_d\":{},\"la_a\":{},\"la_d\":{},\
\"esc_a\":{},\"esc_d\":{},\"ver_a\":{},\"ver_d\":{},\
\"hash\":{},\"canon\":{},\"cmp\":{},\"lookup\":{},\"walks\":{},\"dps\":{},\"capped\":{},\"lost\":{},\
\"steps\":{},\"escapes\":{},\"c2e\":{},\"c2t\":{},\"useless\":{},\"bad\":{}}}",
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
                if name != "none" && !r1.solved {
                    solved_all = false;
                    let _ = write!(detail, " [{name} r={r} t={t}: unsolved]");
                }
            }
        }
    }
    check(
        "agreement",
        all_equal,
        format!("{runs} runs (11 arms x 2 r x 4 targets) identical in every counter, DP digest and recovered log; field ledger = constants x ops{detail}"),
    );
    check("recovery", solved_all, "every arm except the no-escape control recovered the planted log at N~2^20".into());

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
        if name != "none" && name != "dbl" && name != "min2" && name != "la" && solved < 5 {
            non_none_ok = false;
        }
        if bad > 0 {
            ok.set(false);
        }
    }
    check("positive control (absorption)", absorbed >= 5, format!("no-escape arm censored on {absorbed}/6 targets at r=16"));
    check("positive control (baseline)", base_ok >= 5, format!("baseline solved {base_ok}/6"));
    check("positive control (detecting arms)", non_none_ok, "periodic and look-ahead+periodic arms solved at least 5 of 6".into());

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
                writeln!(f, "{}", rec_json(bits, r, name, t, &rec)).unwrap();
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

const GAE_FREE: [f64; 3] = [0.0, 0.1, 1.0];

#[derive(Clone, Copy)]
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

type Key = (u32, usize, String);

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
                    let key = (bits, row.f["r"] as usize, row.s["arm"].clone());
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
    let mut sizes: Vec<u32> = recs.keys().map(|k| k.0).collect();
    sizes.sort();
    sizes.dedup();
    let mut md = String::new();
    let mut js = String::new();
    let mut rng = Rng::new(0x5EED_13);
    let _ = writeln!(md, "# Item 13 part A summary (mechanical read-out of the frozen rule)\n");
    let _ = writeln!(md, "Files read: {files}. Bootstrap resamples: {reps}. Gain = mean cost of the baseline / mean cost of the arm, paired by target, 95% percentile interval. A dagger marks a cell where the arm left a target unsolved at the budget cap (its cost is then a lower bound, its gain an upper bound).\n");
    let _ = writeln!(js, "{{\"cells\":[");
    let mut first_js = true;
    let mut reach_cells: Vec<String> = Vec::new();
    let mut point_reach: Vec<String> = Vec::new();
    let mut loss_cells: HashMap<u32, Vec<String>> = HashMap::new();
    let mut gate_rows: Vec<String> = Vec::new();
    let mut best: HashMap<(u32, String), (f64, f64, f64, usize)> = HashMap::new();
    let mut rate_rows: Vec<String> = Vec::new();
    let mut baseline_gate_ok = true;
    let mut none_absorbed_ok = true;
    for &bits in &sizes {
        let n = prev_prime(1u64 << bits);
        let sqrt_n = (n as f64).sqrt();
        let theta = theta_of(bits);
        let mut rs: Vec<usize> = recs.keys().filter(|k| k.0 == bits).map(|k| k.1).collect();
        rs.sort();
        rs.dedup();
        let _ = writeln!(md, "\n## 2^{bits} (N = {n}, distinguished-point bits {theta}, walk cap {})\n", 20 * (1u64 << theta) + 64);
        // baseline reproduction gate: pooled over r
        let mut pool: Vec<f64> = Vec::new();
        for &r in &rs {
            if let Some(v) = recs.get(&(bits, r, "base".to_string())) {
                pool.extend(v.iter().map(|x| walk_ops(&x.1) / sqrt_n));
            }
        }
        if !pool.is_empty() {
            let m = mean(&pool);
            let ok = (1.10..=1.45).contains(&m);
            baseline_gate_ok &= ok;
            gate_rows.push(format!("2^{bits}: pooled baseline walk ops / sqrt(N) = {m:.3} over {} targets (band [1.10, 1.45]; sqrt(pi/2) = 1.253; the KN-TECH-006 figure 0.886 is sqrt(pi/4), the ideal negation cost): {}", pool.len(), if ok { "PASS" } else { "FAIL" }));
        }
        for &r in &rs {
            let Some(base) = recs.get(&(bits, r, "base".to_string())) else { continue };
            let _ = writeln!(md, "\n### 2^{bits}, r = {r} ({} baseline targets)\n", base.len());
            let _ = writeln!(md, "| arm | solved | walk ops / sqrt N | gain JAC30 nu=0 walk-only [95%] | gain JAC30 nu=0 fully charged [95%] | AFF3 nu=0 | AFF30 nu=0 | JAC30 nu=0.1 | JAC30 nu=1 | AFF3 nu=0.1 | AFF3 nu=1 |");
            let _ = writeln!(md, "|:--|--:|--:|:--|:--|--:|--:|--:|--:|--:|--:|");
            for name in ARM_NAMES.iter().skip(1) {
                let Some(armv) = recs.get(&(bits, r, name.to_string())) else { continue };
                let n_t = base.len().min(armv.len());
                let col = |v: &Vec<(u64, Row)>, m: &Model, nu: f64, su: bool| -> Vec<f64> { v[..n_t].iter().map(|x| cost(&x.1, m, nu, su)).collect() };
                let solved = armv.iter().filter(|x| x.1.f["solved"] > 0.5).count();
                let dag = if solved < armv.len() { "†" } else { "" };
                let wo = mean(&armv.iter().map(|x| walk_ops(&x.1) / sqrt_n).collect::<Vec<_>>());
                let g_walk = paired_ratio(&col(base, &MODELS[0], 0.0, false), &col(armv, &MODELS[0], 0.0, false), &mut rng, reps);
                let g_full = paired_ratio(&col(base, &MODELS[0], 0.0, true), &col(armv, &MODELS[0], 0.0, true), &mut rng, reps);
                let pt = |m: &Model, nu: f64| mean(&col(base, m, nu, false)) / mean(&col(armv, m, nu, false));
                let (a3_0, a30_0) = (pt(&MODELS[1], 0.0), pt(&MODELS[2], 0.0));
                let (j_01, j_1) = (pt(&MODELS[0], 0.1), pt(&MODELS[0], 1.0));
                let (a3_01, a3_1) = (pt(&MODELS[1], 0.1), pt(&MODELS[1], 1.0));
                let _ = writeln!(
                    md,
                    "| {name} | {solved}/{}{dag} | {wo:.3} | {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}] | {a3_0:.3} | {a30_0:.3} | {j_01:.3} | {j_1:.3} | {a3_01:.3} | {a3_1:.3} |",
                    armv.len(),
                    g_walk.0, g_walk.1, g_walk.2, g_full.0, g_full.1, g_full.2
                );
                if !first_js {
                    js.push_str(",\n");
                }
                first_js = false;
                let _ = write!(js, "{{\"bits\":{bits},\"r\":{r},\"arm\":\"{name}\",\"targets\":{},\"solved\":{solved},\"walk_ops_over_sqrtN\":{wo:.6},\"gain_jac30_nu0_walk\":[{:.6},{:.6},{:.6}],\"gain_jac30_nu0_full\":[{:.6},{:.6},{:.6}],\"gain_aff3_nu0\":{a3_0:.6},\"gain_aff30_nu0\":{a30_0:.6},\"gain_jac30_nu01\":{j_01:.6},\"gain_jac30_nu1\":{j_1:.6},\"gain_aff3_nu01\":{a3_01:.6},\"gain_aff3_nu1\":{a3_1:.6}}}", armv.len(), g_walk.0, g_walk.1, g_walk.2, g_full.0, g_full.1, g_full.2);
                if *name != "none" {
                    let tag = format!("2^{bits} {name} r={r}: {:.3} [{:.3}, {:.3}]", g_walk.0, g_walk.1, g_walk.2);
                    if g_walk.1 >= 1.35 {
                        reach_cells.push(tag.clone());
                    }
                    if g_walk.0 >= 1.35 {
                        point_reach.push(tag);
                    }
                    if g_full.2 < 1.0 {
                        loss_cells.entry(bits).or_default().push(format!("{name} r={r}: {:.3} [{:.3}, {:.3}]", g_full.0, g_full.1, g_full.2));
                    }
                    let e = best.entry((bits, name.to_string())).or_insert((0.0, 0.0, 0.0, 0));
                    if g_walk.0 > e.0 {
                        *e = (g_walk.0, g_walk.1, g_walk.2, r);
                    }
                }
                // cycle-entry rate
                if *name == "la" || *name == "min2" || *name == "dbl" {
                    let e: f64 = armv.iter().map(|x| x.1.f["c2e"]).sum();
                    let t: f64 = armv.iter().map(|x| x.1.f["c2t"]).sum();
                    let pred = 1.0 / (2.0 * r as f64);
                    let rate = e / t;
                    rate_rows.push(format!("| 2^{bits} | {r} | {name} | {rate:.5} | {pred:.5} | {:.3} | {} |", rate / pred, if (0.5..=2.0).contains(&(rate / pred)) { "within factor 2" } else { "OUTSIDE" }));
                }
                if *name == "none" {
                    let cap: f64 = armv.iter().map(|x| x.1.f["capped"]).sum();
                    let w: f64 = armv.iter().map(|x| x.1.f["walks"]).sum();
                    let pred = 1.0 - (-(2f64.powi(theta as i32)) / (2.0 * r as f64)).exp();
                    let frac = cap / w;
                    let ok = frac >= 0.5 * pred;
                    if r == 16 {
                        none_absorbed_ok &= ok;
                    }
                    gate_rows.push(format!("2^{bits} r={r}: no-escape arm capped-walk fraction {frac:.3} (model 1-exp(-2^theta/(2r)) = {pred:.3}), targets solved {solved}/{}: {}", armv.len(), if ok { "absorbed as modelled" } else { "NOT ABSORBED" }));
                }
            }
            // A/A negative control
            if let Some(bb) = recs.get(&(bits, r, "baseb".to_string())) {
                let a: Vec<f64> = base.iter().map(|x| cost(&x.1, &MODELS[0], 0.0, false)).collect();
                let b: Vec<f64> = bb.iter().map(|x| cost(&x.1, &MODELS[0], 0.0, false)).collect();
                let g = unpaired_ratio(&a, &b, &mut rng, reps);
                gate_rows.push(format!("2^{bits} r={r}: A/A baseline ratio {:.3} [{:.3}, {:.3}] ({} vs {} targets): {}", g.0, g.1, g.2, a.len(), b.len(), if g.1 <= 1.0 && g.2 >= 1.0 { "interval covers 1" } else { "INTERVAL EXCLUDES 1" }));
            }
        }
    }
    let _ = writeln!(js, "\n]}}");
    // ideal-map control
    for &bits in &sizes {
        if let (Some(p), Some(c)) = (ctls.get(&(bits, "ctl_pts".to_string())), ctls.get(&(bits, "ctl_cls".to_string()))) {
            let x: Vec<f64> = p.iter().map(|v| v.1).collect();
            let y: Vec<f64> = c.iter().map(|v| v.1).collect();
            let g = unpaired_ratio(&x, &y, &mut rng, reps);
            let ok = g.1 <= std::f64::consts::SQRT_2 && g.2 >= std::f64::consts::SQRT_2;
            gate_rows.push(format!("2^{bits}: ideal random-map control, points/classes steps ratio {:.3} [{:.3}, {:.3}] ({} targets), sqrt2 = 1.414: {}", g.0, g.1, g.2, x.len(), if ok { "interval covers sqrt2" } else { "INTERVAL EXCLUDES sqrt2" }));
        }
    }
    let _ = writeln!(md, "\n## Instrument gates and controls\n");
    for r in &gate_rows {
        let _ = writeln!(md, "- {r}");
    }
    let _ = writeln!(md, "\n## Per-step fruitless-cycle entry rate (prediction 3)\n\n| size | r | arm | measured per-step rate | 1/(2r) | ratio | verdict |\n|:--|--:|:--|--:|--:|--:|:--|");
    for r in &rate_rows {
        let _ = writeln!(md, "{r}");
    }
    let _ = writeln!(md, "\n## Best r per rule (walk-only JAC30 nu=0 point estimate)\n\n| size | rule | best r | gain [95%] |\n|:--|:--|--:|:--|");
    let mut bk: Vec<_> = best.iter().collect();
    bk.sort_by(|a, b| a.0.cmp(b.0));
    for ((bits, name), v) in bk {
        let _ = writeln!(md, "| 2^{bits} | {name} | {} | {:.3} [{:.3}, {:.3}] |", v.3, v.0, v.1, v.2);
    }
    let _ = writeln!(md, "\n## Frozen rule, read mechanically\n");
    let _ = writeln!(md, "- Instrument gate (baseline within band at every size): {}", if baseline_gate_ok { "PASS" } else { "FAIL, no comparison may be reported" });
    let _ = writeln!(md, "- No-escape control absorbed at r = 16 at every size: {}", if none_absorbed_ok { "yes" } else { "NO (falsification condition 2)" });
    let _ = writeln!(md, "- Cells whose point estimate (walk-only, JAC30, nu = 0) is >= 1.35: {}", if point_reach.is_empty() { "none".to_string() } else { point_reach.join("; ") });
    let _ = writeln!(md, "- Cells whose 95% lower bound is >= 1.35: {}", if reach_cells.is_empty() { "none".to_string() } else { reach_cells.join("; ") });
    let mut loss_sizes: Vec<_> = loss_cells.keys().copied().collect();
    loss_sizes.sort();
    for s in &sizes {
        let v = loss_cells.get(s);
        let _ = writeln!(md, "- 2^{s}: cells with a clear net loss (fully charged JAC30 nu = 0, 95% upper bound < 1.0): {}", v.map(|v| v.join("; ")).unwrap_or_else(|| "none".into()));
    }
    let all_three = sizes.len() >= 3 && sizes.iter().all(|s| loss_cells.contains_key(s));
    let _ = writeln!(md, "- A setting with a clear net loss at all three sizes: {}", if all_three { "yes" } else { "no; the exact words for the tested range are: no sign change inside the tested range" });
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
    let dp34_log2 = 25.2f64;
    let dp_bytes = 32.0f64;
    struct Gpu {
        name: &'static str,
        rate: f64,
        src: &'static str,
        same_fn: bool,
    }
    let gpus = [
        Gpu { name: "RTX PRO 6000, sigma-table kernel (campaign iteration function)", rate: 15.884369e9, src: "ecc2k130/benchmarks/sigma-table/headline/result.json candidate median; ecc2k130/benchmarks/modal-sigma-table-live-20261015/README.md", same_fn: true },
        Gpu { name: "RTX PRO 6000, table walk (different iteration function, pre-v3 revision)", rate: 20.078e9, src: "ecc2k130/ONE-BLOCK-GEOMETRY.md (20.078 B/s median)", same_fn: false },
        Gpu { name: "RTX PRO 4500 (AWS g7.2xlarge), promoted x-only bridge map", rate: 12.499203e9, src: "ecc2k130/README.md (12.499203 B/s paired median)", same_fn: false },
    ];
    struct Price {
        name: &'static str,
        usd_h: f64,
        src: &'static str,
    }
    let prices = [
        Price { name: "AWS g7e.2xlarge spot, us-west-2", usd_h: 1.2128, src: "ecc2k130/RTX-PRO4500.md, observed 2026-09-16" },
        Price { name: "AWS g7.2xlarge spot, us-west-2", usd_h: 0.5705, src: "ecc2k130/RTX-PRO4500.md, observed 2026-09-16" },
        Price { name: "g7e spot per GPU-hour", usd_h: 1.31, src: "ecc2k130/FPGA-CEILING.md section 5 (prices dated 2026-09-11)" },
        Price { name: "g7e.48xlarge on-demand per GPU-hour", usd_h: 4.14, src: "ecc2k130/FPGA-CEILING.md section 5" },
        Price { name: "Modal list, RTX PRO 6000", usd_h: 3.03, src: "ecc2k130/B200.md" },
    ];
    let mut md = String::new();
    let mut js = String::new();
    let _ = writeln!(md, "# Item 13 part B: costed plan for completing ECC2K-130\n");
    let _ = writeln!(md, "## Measured CPU rate of the current `ecc2k130/` client\n");
    let _ = writeln!(md, "{} repetitions of `ecc2k130-cpu --curve 131 --bench --threads 1`, one thread each, on this host (shared 4-vCPU container, isolation level L0, load recorded in raw/): {}.", rates.len(), rates.iter().map(|r| format!("{r:.3}")).collect::<Vec<_>>().join(", "));
    let _ = writeln!(md, "\nMedian **{:.3} M iterations/s per core**, min {:.3}, max {:.3}. Committed single-core figures for comparison (ecc2k130/README.md Status): 12.6 M/s per AVX-512 core, 15.7 M/s per M4 Pro core.\n", med / 1e6, lo / 1e6, hi / 1e6);
    let _ = writeln!(md, "## Work\n");
    let _ = writeln!(md, "Expected iterations to solve: 2^{w_log2} = {w:.4e} (ecc2k130/README.md; measured for this walk 2^60.91 to 2^60.92 on completed trails, ecc2k130/WALK-CONSTANT.md). Iterations per distinguished point: 2^{dp32_log2} at weight 32 (ecc2k130/benchmarks/dp-interval/README.md, 132 workers), 2^{dp34_log2} at weight 34. Record: {dp_bytes} bytes per point.\n");
    let dps32 = w / 2f64.powf(dp32_log2);
    let dps34 = w / 2f64.powf(dp34_log2);
    let _ = writeln!(md, "| distinguished-point weight | points to collect | storage |\n|:--|--:|--:|");
    let _ = writeln!(md, "| 32 | {dps32:.3e} (2^{:.2}) | {:.1} GB |", dps32.log2(), dps32 * dp_bytes / 1e9);
    let _ = writeln!(md, "| 34 | {dps34:.3e} (2^{:.2}) | {:.2} TB |", dps34.log2(), dps34 * dp_bytes / 1e12);
    let _ = writeln!(md, "\nThe in-memory collision table needs the key (at least 8 bytes) of every point: {:.0} GB at weight 32.\n", dps32 * 8.0 / 1e9);
    let _ = writeln!(md, "## CPU alone\n");
    let cpu_h = w / med / 3600.0;
    let _ = writeln!(md, "{cpu_h:.3e} core-hours = {:.0} core-years at the measured median rate.\n", cpu_h / 8766.0);
    let _ = writeln!(js, "{{\"cpu_rate_median\":{med:.1},\"cpu_rate_min\":{lo:.1},\"cpu_rate_max\":{hi:.1},\"cpu_reps\":{},\"work_iterations\":{w:.6e},\"dp32_points\":{dps32:.6e},\"dp34_points\":{dps34:.6e},\"cpu_core_hours\":{cpu_h:.6e},\"gpu_rows\":[", rates.len());
    let _ = writeln!(md, "## GPU hours and cost\n");
    let _ = writeln!(md, "GPU-hours = 2^60.9 / (rate x 3600). Cost = GPU-hours x price. A price is an input from a committed observation; no live quote was available to this run.\n");
    let _ = writeln!(md, "| GPU row | rate (B it/s) | same iteration function as the CPU client and the corpus | GPU-hours | CPU core-hours per GPU-hour | break-even CPU core-hour price at the cheapest listed GPU price |\n|:--|--:|:--|--:|--:|--:|");
    let cheapest = prices.iter().map(|p| p.usd_h).fold(f64::MAX, f64::min);
    for (i, g) in gpus.iter().enumerate() {
        let gh = w / g.rate / 3600.0;
        let be = cheapest * med / g.rate;
        let _ = writeln!(md, "| {} | {:.3} | {} | {gh:.0} | {:.0} | ${be:.5} |", g.name, g.rate / 1e9, if g.same_fn { "yes" } else { "no (separate corpus or revision; not substitutable)" }, g.rate / med);
        let _ = write!(js, "{}{{\"name\":\"{}\",\"rate\":{:.1},\"gpu_hours\":{gh:.1},\"src\":\"{}\"}}", if i > 0 { "," } else { "" }, g.name, g.rate, g.src);
    }
    let _ = writeln!(js, "],\"costs\":[");
    let _ = writeln!(md, "\n| GPU row | price row | $/h | total cost (USD) |\n|:--|:--|--:|--:|");
    let mut first = true;
    for g in &gpus {
        let gh = w / g.rate / 3600.0;
        for p in &prices {
            let _ = writeln!(md, "| {} | {} | {:.4} | {:.0} |", g.name, p.name, p.usd_h, gh * p.usd_h);
            let _ = write!(js, "{}{{\"gpu\":\"{}\",\"price\":\"{}\",\"usd_h\":{},\"usd\":{:.1},\"price_src\":\"{}\"}}", if first { "" } else { "," }, g.name, p.name, p.usd_h, gh * p.usd_h, p.src);
            first = false;
        }
    }
    let _ = writeln!(js, "]}}");
    let _ = writeln!(md, "\nSources of the rates and prices:");
    for g in &gpus {
        let _ = writeln!(md, "- {}: {}", g.name, g.src);
    }
    for p in &prices {
        let _ = writeln!(md, "- {}: {}", p.name, p.src);
    }
    let _ = writeln!(md, "\n## Wall clock\n\n| GPUs (sigma-table, 15.884 B/s each) | days |\n|--:|--:|");
    for fleet in [100.0f64, 1000.0, 10000.0] {
        let _ = writeln!(md, "| {fleet:.0} | {:.1} |", w / (gpus[0].rate * fleet) / 86400.0);
    }
    let _ = writeln!(md, "\n## What this plan claims and does not\n\nIt claims the CPU rate measured above and arithmetic on the committed GPU rates, work constants and price observations listed with their sources. It claims no GPU rate measured here (no GPU exists on this host), no live price, no campaign progress credit, and nothing about the collision-finding cost beyond the storage line. The weight-34 interval is the committed 2^25.2; the weight-32 interval is the measured 2^28.41.");
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
