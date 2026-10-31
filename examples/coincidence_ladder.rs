//! LEAD-ICP-coincidence-harvest-structured-base: the break-even growth ladder.
//!
//! Measures kappa(N), the ratio of the exact number of nontrivial pair-sum
//! coincidences of a structured symmetric base to the mean over matched
//! uniformly random symmetric bases of the same size, on prime-order curves
//! E/F_p of `bits` bits, and the slope d ln kappa / d ln N across rungs.
//! A coincidence is a pair of unordered pairs {P, Q} != {R, S} of base points
//! with P + Q = R + S, where P != Q and P != -Q. Everything is exact; nothing
//! is sampled. Standard library only; p < 2^32.
//!
//! ```text
//! rustc -O coincidence_ladder.rs -o coincidence_ladder
//! coincidence_ladder selftest
//! coincidence_ladder census --bits 24 --block 8100000 --n30 400 --n35 40 \
//!     --threads 8 --verify-every 19 --out main-24.tsv
//! coincidence_ladder design pilot-16.tsv pilot-20.tsv ...
//! coincidence_ladder analyze main-16.tsv main-20.tsv ...
//! coincidence_ladder stage1 cells.tsv
//! ```

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const THETAS: [f64; 2] = [0.30, 0.35];
const NULLS: usize = 20;
const GAMMAS: [f64; 3] = [0.0, 1.0 / 6.0, 0.2];
const BOOT: usize = 2000;
const CHUNK: usize = 4096;
/// Target relative standard error of kappa per cell (PROTOCOL.md section 5).
const REL_TARGET: f64 = 0.02;
/// A slope whose lower 95% bound is above 0 and whose upper bound reaches this
/// is ALIVE: the weakest break-even exponent, delta*(h = 4).
/// The break-even exponents delta*(h) = 1/(2h-1) for h = 2, 3, 4.
const DELTA_STAR: [f64; 3] = [1.0 / 3.0, 0.2, 1.0 / 7.0];

// ---------------------------------------------------------------- randomness

/// splitmix64 seeded by FNV-1a of a label string, so every base is a pure
/// function of its labels and nothing depends on thread order.
struct Rng(u64);

impl Rng {
    fn from_labels(labels: &str) -> Self {
        let mut h = 0xcbf2_9ce4_8422_2325_u64;
        for byte in labels.bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        Rng(h)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        ((u128::from(self.next_u64()) * u128::from(n)) >> 64) as u64
    }

    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn normal(&mut self) -> f64 {
        let u = self.unit().max(1e-300);
        let v = self.unit();
        (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos()
    }
}

// ------------------------------------------------------------ number theory

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}

fn powmod(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut acc = 1 % p;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mulmod(acc, base, p);
        }
        base = mulmod(base, base, p);
        exp >>= 1;
    }
    acc
}

fn invmod(a: u64, p: u64) -> u64 {
    powmod(a, p - 2, p)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13] {
        if n == q {
            return true;
        }
        if n.is_multiple_of(q) {
            return false;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }
    let mul = |a: u64, b: u64| (u128::from(a) * u128::from(b) % u128::from(n)) as u64;
    'bases: for a in [2u64, 3, 5, 7, 11, 13] {
        let mut x = 1u64;
        let mut base = a % n;
        let mut e = d;
        while e > 0 {
            if e & 1 == 1 {
                x = mul(x, base);
            }
            base = mul(base, base);
            e >>= 1;
        }
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul(x, x);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

/// Square root modulo an odd prime (Tonelli-Shanks); `None` for a non-residue.
fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(powmod(a, p.div_ceil(4), p));
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, q.div_ceil(2), p);
    while t != 1 {
        let mut i = 0u32;
        let mut tt = t;
        while tt != 1 {
            tt = mulmod(tt, tt, p);
            i += 1;
        }
        let b = powmod(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        r = mulmod(r, b, p);
    }
    Some(r)
}

// -------------------------------------------------------------------- curves

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (mulmod(mulmod(x, x, p), x, p) + mulmod(self.a, x, p) + self.b) % p
    }

    fn on_curve(&self, q: Pt) -> bool {
        mulmod(q.y, q.y, self.p) == self.rhs(q.x)
    }

    fn add(&self, u: Option<Pt>, v: Option<Pt>) -> Option<Pt> {
        let p = self.p;
        let (u, v) = match (u, v) {
            (None, w) | (w, None) => return w,
            (Some(u), Some(v)) => (u, v),
        };
        let lambda = if u.x == v.x {
            if (u.y + v.y) % p == 0 {
                return None;
            }
            let num = (3 * mulmod(u.x, u.x, p) + self.a) % p;
            mulmod(num, invmod(2 * u.y % p, p), p)
        } else {
            mulmod((v.y + p - u.y) % p, invmod((v.x + p - u.x) % p, p), p)
        };
        let x3 = (mulmod(lambda, lambda, p) + 2 * p - u.x - v.x) % p;
        let y3 = (mulmod(lambda, (u.x + p - x3) % p, p) + p - u.y) % p;
        Some(Pt { x: x3, y: y3 })
    }

    fn mul(&self, mut k: u64, q: Pt) -> Option<Pt> {
        let mut acc = None;
        let mut base = Some(q);
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }

    /// A random point with a random sign.
    fn random_point(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.below(self.p);
            let r = self.rhs(x);
            if r == 0 {
                continue;
            }
            if let Some(y) = sqrt_mod(r, self.p) {
                let y = if rng.next_u64() & 1 == 1 {
                    y
                } else {
                    self.p - y
                };
                return Pt { x, y };
            }
        }
    }
}

struct Instance {
    curve: Curve,
    order: u64,
    gen: Pt,
}

/// A prime-order curve of `bits` bits. The order is proved, not assumed: the
/// Hasse interval holds exactly one multiple of the order of a random point,
/// it is prime and it exceeds half the interval's top, so it is `#E`.
fn find_curve(bits: u32, rng: &mut Rng) -> Instance {
    assert!((8..=32).contains(&bits));
    loop {
        let p = loop {
            let cand = (rng.next_u64() & ((1u64 << bits) - 1)) | (1u64 << (bits - 1)) | 1;
            if is_prime(cand) {
                break cand;
            }
        };
        for _ in 0..64 {
            let a = rng.below(p);
            let b = 1 + rng.below(p - 1);
            let disc = (4 * mulmod(mulmod(a, a, p), a, p) + 27 * mulmod(b, b, p)) % p;
            if disc == 0 {
                continue;
            }
            let curve = Curve { p, a, b };
            let gen = curve.random_point(rng);
            if let Some(order) = prime_order_of(&curve, gen) {
                return Instance { curve, order, gen };
            }
        }
    }
}

/// `Some(n)` iff `n` is the unique multiple of `ord(g)` in the Hasse interval
/// and is prime; then `#E = n`.
fn prime_order_of(curve: &Curve, g: Pt) -> Option<u64> {
    let p = curve.p;
    let s = isqrt(p) + 1;
    let lo = p + 1 - 2 * s;
    let hi = p + 1 + 2 * s;
    let m = isqrt(hi - lo + 1) + 1;
    let mut baby: HashMap<(u64, u64), u64> = HashMap::new();
    let mut cur = Some(g);
    for j in 1..m {
        let q = cur?;
        baby.insert((q.x, q.y), j);
        cur = curve.add(cur, Some(g));
    }
    let step = curve.mul(m, g);
    let mut giant = curve.mul(lo, g);
    let mut hits: Vec<u64> = Vec::new();
    for i in 0..=((hi - lo) / m + 1) {
        let base = lo + i * m;
        match giant {
            None => hits.push(base),
            Some(q) => {
                if let Some(j) = baby.get(&(q.x, (p - q.y) % p)) {
                    hits.push(base + j);
                }
            }
        }
        giant = curve.add(giant, step);
    }
    hits.retain(|n| (lo..=hi).contains(n));
    hits.sort_unstable();
    hits.dedup();
    if hits.len() != 1 {
        return None;
    }
    let n = hits[0];
    if is_prime(n) && curve.mul(n, g).is_none() {
        Some(n)
    } else {
        None
    }
}

// ----------------------------------------------------------------- counting

/// All unordered pair sums P + Q (P != Q, x(P) != x(Q)) as exact keys x*p + y,
/// computed with chunked simultaneous inversion.
fn pair_sum_keys(pts: &[Pt], p: u64) -> Vec<u64> {
    let n = pts.len();
    let mut keys = Vec::with_capacity(n * n.saturating_sub(1) / 2);
    let mut idx: Vec<(Pt, Pt)> = Vec::with_capacity(CHUNK);
    let flush = |idx: &mut Vec<(Pt, Pt)>, keys: &mut Vec<u64>| {
        if idx.is_empty() {
            return;
        }
        let mut prefix = Vec::with_capacity(idx.len());
        let mut acc = 1u64;
        for (u, v) in idx.iter() {
            acc = mulmod(acc, (v.x + p - u.x) % p, p);
            prefix.push(acc);
        }
        let mut inv = invmod(acc, p);
        for (k, (u, v)) in idx.iter().enumerate().rev() {
            let d = (v.x + p - u.x) % p;
            let inv_k = if k == 0 {
                inv
            } else {
                mulmod(inv, prefix[k - 1], p)
            };
            inv = mulmod(inv, d, p);
            let lambda = mulmod((v.y + p - u.y) % p, inv_k, p);
            let x3 = (mulmod(lambda, lambda, p) + 2 * p - u.x - v.x) % p;
            let y3 = (mulmod(lambda, (u.x + p - x3) % p, p) + p - u.y) % p;
            keys.push(x3 * p + y3);
        }
        idx.clear();
    };
    for (i, u) in pts.iter().enumerate() {
        for v in pts.iter().skip(i + 1) {
            if u.x == v.x {
                continue;
            }
            idx.push((*u, *v));
            if idx.len() == CHUNK {
                flush(&mut idx, &mut keys);
            }
        }
    }
    flush(&mut idx, &mut keys);
    keys
}

/// C = sum over sums s of binom(m_s, 2), from a multiset of keys.
fn coincidences(mut keys: Vec<u64>) -> u64 {
    keys.sort_unstable();
    let mut total = 0u64;
    let mut i = 0;
    while i < keys.len() {
        let mut j = i + 1;
        while j < keys.len() && keys[j] == keys[i] {
            j += 1;
        }
        let run = (j - i) as u64;
        total += run * (run - 1) / 2;
        i = j;
    }
    total
}

fn count_ec(pts: &[Pt], p: u64) -> u64 {
    coincidences(pair_sum_keys(pts, p))
}

/// The same count by one chord addition per pair (no batching) and a hash
/// map: an independent path used to audit `count_ec` on a sample of jobs.
fn count_ec_slow(pts: &[Pt], curve: &Curve) -> u64 {
    let mut counts: HashMap<(u64, u64), u64> = HashMap::new();
    for (i, u) in pts.iter().enumerate() {
        for v in pts.iter().skip(i + 1) {
            if u.x == v.x {
                continue;
            }
            let s = curve.add(Some(*u), Some(*v)).expect("distinct x");
            *counts.entry((s.x, s.y)).or_insert(0) += 1;
        }
    }
    counts.values().map(|r| r * (r - 1) / 2).sum()
}

/// Pair-sum coincidences of a symmetric set of residues {+-u} modulo n.
fn count_zn(units: &[u64], n: u64) -> u64 {
    let mut set: Vec<u64> = Vec::with_capacity(2 * units.len());
    for &u in units {
        set.push(u);
        set.push(n - u);
    }
    let mut keys = Vec::new();
    for (i, &u) in set.iter().enumerate() {
        for &v in set.iter().skip(i + 1) {
            if u + v == n {
                continue;
            }
            keys.push((u + v) % n);
        }
    }
    coincidences(keys)
}

// -------------------------------------------------------------------- bases

struct Builder<'a> {
    curve: &'a Curve,
    seen: HashSet<u64>,
    pts: Vec<Pt>,
    target: usize,
}

impl<'a> Builder<'a> {
    fn new(curve: &'a Curve, target: usize) -> Self {
        Builder {
            curve,
            seen: HashSet::new(),
            pts: Vec::with_capacity(target),
            target,
        }
    }

    fn full(&self) -> bool {
        self.pts.len() >= self.target
    }

    /// Both lifts of x when x is new and f(x) is a nonzero square.
    fn try_x(&mut self, x: u64) {
        if self.full() || !self.seen.insert(x) {
            return;
        }
        let r = self.curve.rhs(x);
        if r == 0 {
            return;
        }
        if let Some(y) = sqrt_mod(r, self.curve.p) {
            self.pts.push(Pt { x, y });
            self.pts.push(Pt {
                x,
                y: self.curve.p - y,
            });
        }
    }

    fn try_point(&mut self, q: Pt) -> bool {
        if self.full() || !self.seen.insert(q.x) {
            return false;
        }
        self.pts.push(q);
        self.pts.push(Pt {
            x: q.x,
            y: self.curve.p - q.y,
        });
        true
    }

    fn fill_random(&mut self, rng: &mut Rng) {
        while !self.full() {
            let x = rng.below(self.curve.p);
            self.try_x(x);
        }
    }
}

fn base_size(bits: u32, theta: f64) -> usize {
    2 * ((f64::from(bits) * theta).exp2() / 2.0).round() as usize
}

fn null_base(curve: &Curve, rng: &mut Rng, b: usize) -> Vec<Pt> {
    let mut bd = Builder::new(curve, b);
    bd.fill_random(rng);
    bd.pts
}

fn window_base(curve: &Curve, rng: &mut Rng, b: usize) -> Vec<Pt> {
    let mut bd = Builder::new(curve, b);
    let x0 = rng.below(curve.p);
    let mut t = 0u64;
    while !bd.full() {
        bd.try_x((x0 + t) % curve.p);
        t += 1;
    }
    bd.pts
}

/// Rank-d label GAP x0 + a + c*w1 (+ e*w2), box side ceil(b^(1/d)) (at least
/// 2), innermost index fastest, outermost unbounded.
fn gap_base(curve: &Curve, rng: &mut Rng, b: usize, d: u32) -> Vec<Pt> {
    let p = curve.p;
    let mut bd = Builder::new(curve, b);
    let x0 = rng.below(p);
    let w1 = 1 + rng.below(p - 1);
    let w2 = 1 + rng.below(p - 1);
    let side = ((b as f64).powf(1.0 / f64::from(d)).ceil() as u64).max(2);
    let mut outer = 0u64;
    while !bd.full() {
        if d == 2 {
            for a in 0..side {
                bd.try_x((x0 + a + mulmod(outer % p, w1, p)) % p);
            }
        } else {
            for c in 0..side {
                for a in 0..side {
                    let lab = x0 + a + mulmod(c, w1, p) + mulmod(outer % p, w2, p);
                    bd.try_x(lab % p);
                }
            }
        }
        outer += 1;
    }
    bd.pts
}

/// The integer pair-sum coincidences of {+-1, ..., +-c0}: one row of the
/// weighted plant, counted with no group arithmetic.
fn formal_row(c0: u64) -> u64 {
    let mut set: Vec<i64> = Vec::new();
    for c in 1..=c0 as i64 {
        set.push(c);
        set.push(-c);
    }
    let mut keys = Vec::new();
    for (i, &u) in set.iter().enumerate() {
        for &v in set.iter().skip(i + 1) {
            if u + v != 0 {
                keys.push((u + v + 2 * c0 as i64) as u64);
            }
        }
    }
    coincidences(keys)
}

/// The weighted plant {+-c P_i : 1 <= c <= c0}, m = b / (2 c0) rows, topped up
/// with random points; `None` when m = 0. Returns the points and the formal
/// coincidence count of the rows.
fn weighted_base(curve: &Curve, rng: &mut Rng, b: usize, c0: u64) -> Option<(Vec<Pt>, u64, u64)> {
    let m = b / (2 * c0 as usize);
    if m == 0 {
        return None;
    }
    let mut bd = Builder::new(curve, b);
    for _ in 0..m {
        let gen = curve.random_point(rng);
        let mut q = gen;
        for c in 1..=c0 {
            if c > 1 {
                q = curve
                    .add(Some(q), Some(gen))
                    .expect("small multiple is finite");
            }
            bd.try_point(q);
        }
    }
    bd.fill_random(rng);
    Some((bd.pts, m as u64 * formal_row(c0), m as u64))
}

fn zn_interval(b: usize, n: u64) -> Vec<u64> {
    (1..=(b / 2) as u64).filter(|&k| 2 * k < n).collect()
}

fn zn_null(rng: &mut Rng, b: usize, n: u64) -> Vec<u64> {
    let mut seen = HashSet::new();
    let mut units = Vec::new();
    while units.len() < b / 2 {
        let u = 1 + rng.below((n - 1) / 2);
        if seen.insert(u) {
            units.push(u);
        }
    }
    units
}

// ------------------------------------------------------------------- census

struct Rec {
    bits: u32,
    theta: usize,
    n: u64,
    null: [u64; NULLS],
    win: u64,
    gap: [u64; 2],
    pl: [u64; 3],
    plf: [u64; 3],
    plm: [u64; 3],
    zn: u64,
    znnull: [u64; NULLS],
}

fn census_one(
    bits: u32,
    block: u64,
    c: usize,
    thetas: [bool; 2],
    verify: bool,
) -> Result<Vec<String>, String> {
    let mut rng = Rng::from_labels(&format!("pch20261029|curve|{block}|{bits}|{c}"));
    let inst = find_curve(bits, &mut rng);
    let curve = inst.curve;
    if !curve.on_curve(inst.gen) || inst.curve.mul(inst.order, inst.gen).is_some() {
        return Err(format!(
            "bits {bits} block {block} curve {c}: generator or order check failed"
        ));
    }
    let mut lines = Vec::new();
    for (ti, &theta) in THETAS.iter().enumerate() {
        if !thetas[ti] {
            continue;
        }
        let b = base_size(bits, theta);
        let label =
            |kind: &str, k: usize| format!("pch20261029|{kind}|{block}|{bits}|{c}|{ti}|{k}");
        let audit = |pts: &[Pt], what: &str, got: u64| -> Result<(), String> {
            if verify && count_ec_slow(pts, &curve) != got {
                return Err(format!(
                    "recount mismatch: bits {bits} curve {c} theta {ti} {what}"
                ));
            }
            Ok(())
        };
        let mut null = [0u64; NULLS];
        for (k, slot) in null.iter_mut().enumerate() {
            let pts = null_base(&curve, &mut Rng::from_labels(&label("null", k)), b);
            *slot = count_ec(&pts, curve.p);
            if k < 2 {
                audit(&pts, "null", *slot)?;
            }
        }
        let win_pts = window_base(&curve, &mut Rng::from_labels(&label("window", 0)), b);
        let win = count_ec(&win_pts, curve.p);
        audit(&win_pts, "window", win)?;
        let mut gap = [0u64; 2];
        for (g, slot) in gap.iter_mut().enumerate() {
            let pts = gap_base(
                &curve,
                &mut Rng::from_labels(&label("gap", g)),
                b,
                g as u32 + 2,
            );
            *slot = count_ec(&pts, curve.p);
            audit(&pts, "gap", *slot)?;
        }
        let (mut pl, mut plf, mut plm) = ([0u64; 3], [0u64; 3], [0u64; 3]);
        for (g, &gamma) in GAMMAS.iter().enumerate() {
            let c0 = ((inst.order as f64).powf(gamma).ceil() as u64).max(1);
            let mut prng = Rng::from_labels(&label("plant", g));
            if let Some((pts, formal, m)) = weighted_base(&curve, &mut prng, b, c0) {
                pl[g] = count_ec(&pts, curve.p);
                audit(&pts, "plant", pl[g])?;
                plf[g] = formal;
                plm[g] = m;
            }
        }
        let zn = count_zn(&zn_interval(b, inst.order), inst.order);
        let mut znnull = [0u64; NULLS];
        for (k, slot) in znnull.iter_mut().enumerate() {
            let units = zn_null(&mut Rng::from_labels(&label("znnull", k)), b, inst.order);
            *slot = count_zn(&units, inst.order);
        }
        let mut line = format!(
            "{bits} {ti} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            inst.order,
            win,
            gap[0],
            gap[1],
            pl[0],
            pl[1],
            pl[2],
            plf[0],
            plf[1],
            plf[2],
            plm[0],
            plm[1],
            plm[2],
            zn
        );
        for v in null.iter().chain(znnull.iter()) {
            let _ = write!(line, " {v}");
        }
        lines.push(format!("{c} {line}"));
    }
    Ok(lines)
}

/// One line per (curve, theta): the curve index, then 16 numbers, then the
/// 20 null counts and the 20 Z/N null counts.
fn parse_recs(text: &str) -> Vec<Rec> {
    let mut out = Vec::new();
    for line in text.lines() {
        let f: Vec<u64> = line
            .split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect();
        if f.len() != 1 + 16 + 2 * NULLS {
            continue;
        }
        let f = &f[1..];
        let mut null = [0u64; NULLS];
        let mut znnull = [0u64; NULLS];
        null.copy_from_slice(&f[16..16 + NULLS]);
        znnull.copy_from_slice(&f[16 + NULLS..16 + 2 * NULLS]);
        out.push(Rec {
            bits: f[0] as u32,
            theta: f[1] as usize,
            n: f[2],
            win: f[3],
            gap: [f[4], f[5]],
            pl: [f[6], f[7], f[8]],
            plf: [f[9], f[10], f[11]],
            plm: [f[12], f[13], f[14]],
            zn: f[15],
            null,
            znnull,
        });
    }
    out
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn cmd_census(args: &[String]) -> Result<(), String> {
    let bits: u32 = arg(args, "--bits")
        .ok_or("--bits")?
        .parse()
        .map_err(|_| "--bits")?;
    let block: u64 = arg(args, "--block")
        .ok_or("--block")?
        .parse()
        .map_err(|_| "--block")?;
    let n30: usize = arg(args, "--n30")
        .unwrap_or("0")
        .parse()
        .map_err(|_| "--n30")?;
    let n35: usize = arg(args, "--n35")
        .unwrap_or("0")
        .parse()
        .map_err(|_| "--n35")?;
    let threads: usize = arg(args, "--threads")
        .unwrap_or("1")
        .parse()
        .map_err(|_| "--threads")?;
    let every: usize = arg(args, "--verify-every")
        .unwrap_or("0")
        .parse()
        .map_err(|_| "--verify-every")?;
    let out = arg(args, "--out").ok_or("--out")?;
    let total = n30.max(n35);
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<(usize, Vec<String>)>> = Mutex::new(Vec::new());
    let failure: Mutex<Option<String>> = Mutex::new(None);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let c = next.fetch_add(1, Ordering::Relaxed);
                if c >= total {
                    return;
                }
                let verify = every > 0 && c.is_multiple_of(every);
                match census_one(bits, block, c, [c < n30, c < n35], verify) {
                    Ok(lines) => done.lock().expect("lock").push((c, lines)),
                    Err(e) => {
                        *failure.lock().expect("lock") = Some(e);
                        next.store(total, Ordering::Relaxed);
                        return;
                    }
                }
            });
        }
    });
    if let Some(e) = failure.into_inner().expect("lock") {
        return Err(e);
    }
    let mut done = done.into_inner().expect("lock");
    done.sort_by_key(|(c, _)| *c);
    let mut file = std::fs::File::create(out).map_err(|e| e.to_string())?;
    for (_, lines) in done {
        for l in lines {
            writeln!(file, "{l}").map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

// ------------------------------------------------------------------- design

/// Curve cap per rung (compute budget; recorded in design.json).
fn curve_cap(bits: u32) -> usize {
    match bits {
        ..=16 => 30_000,
        17..=20 => 12_000,
        21..=24 => 5000,
        25..=28 => 2500,
        _ => 1200,
    }
}

fn null_stats(r: &Rec) -> (f64, f64) {
    mean_var(&r.null)
}

fn mean_var(xs: &[u64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let m = xs.iter().map(|&v| v as f64).sum::<f64>() / n;
    let v = xs.iter().map(|&v| (v as f64 - m).powi(2)).sum::<f64>() / (n - 1.0);
    (m, v)
}

fn load(paths: &[String]) -> Result<Vec<Rec>, String> {
    let mut all = Vec::new();
    for p in paths {
        let text = std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"))?;
        all.extend(parse_recs(&text));
    }
    Ok(all)
}

/// `n = clamp(ceil(1.05 V1 / (0.03^2 M1^2)), 40, cap)` per (bits, theta), from
/// the pilot's per-curve null mean M1 and variance V1.
fn cmd_design(paths: &[String]) -> Result<(), String> {
    let recs = load(paths)?;
    let mut out = String::from("[\n");
    let mut keys: Vec<(u32, usize)> = recs.iter().map(|r| (r.bits, r.theta)).collect();
    keys.sort_unstable();
    keys.dedup();
    for (k, &(bits, ti)) in keys.iter().enumerate() {
        let sel: Vec<&Rec> = recs
            .iter()
            .filter(|r| r.bits == bits && r.theta == ti)
            .collect();
        let m1 = sel.iter().map(|r| null_stats(r).0).sum::<f64>() / sel.len() as f64;
        let v1 = sel.iter().map(|r| null_stats(r).1).sum::<f64>() / sel.len() as f64;
        let rule = (1.05 * v1 / (REL_TARGET.powi(2) * m1 * m1)).ceil() as usize;
        let cap = curve_cap(bits);
        let n = rule.clamp(40, cap);
        let rel = (1.05 * v1 / (n as f64 * m1 * m1)).sqrt();
        let _ = writeln!(
            out,
            " {{\"bits\":{bits},\"theta\":{},\"b\":{},\"pilot_curves\":{},\"m1\":{m1:.4},\"v1\":{v1:.4},\"n_rule\":{rule},\"cap\":{cap},\"n_main\":{n},\"predicted_rel_sd\":{rel:.4}}}{}",
            THETAS[ti],
            base_size(bits, THETAS[ti]),
            sel.len(),
            if k + 1 < keys.len() { "," } else { "" }
        );
    }
    out.push(']');
    println!("{out}");
    Ok(())
}

// ----------------------------------------------------------------- analysis

const GEOMS: [&str; 8] = [
    "window",
    "gap2",
    "gap3",
    "plant_g0",
    "plant_g1_6",
    "plant_g1_5",
    "zn_interval",
    "null_cal",
];

/// (structured count, null mean, null variance, null replicates, formal)
type GeomVal = (f64, f64, f64, f64, f64);

fn geom_value(r: &Rec, g: usize) -> Option<GeomVal> {
    let (m, v) = null_stats(r);
    match g {
        0 => Some((r.win as f64, m, v, NULLS as f64, 0.0)),
        1 | 2 => Some((r.gap[g - 1] as f64, m, v, NULLS as f64, 0.0)),
        3..=5 => (r.plm[g - 3] > 0)
            .then(|| (r.pl[g - 3] as f64, m, v, NULLS as f64, r.plf[g - 3] as f64)),
        6 => {
            let (zm, zv) = mean_var(&r.znnull);
            Some((r.zn as f64, zm, zv, NULLS as f64, 0.0))
        }
        _ => {
            let (cm, cv) = mean_var(&r.null[1..]);
            Some((r.null[0] as f64, cm, cv, (NULLS - 1) as f64, 0.0))
        }
    }
}

fn ols(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    sxy / sxx
}

fn pct(sorted: &[f64], q: f64) -> f64 {
    let i = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[i]
}

struct Cell {
    bits: u32,
    n: usize,
    s: f64,
    m: f64,
    v: f64,
    f: f64,
    kappa: f64,
    z: f64,
    ci: (f64, f64),
    mean_ln_r: f64,
}

fn json_f(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.6}")
    } else {
        "null".to_string()
    }
}

fn class_of(lo: f64, hi: f64) -> &'static str {
    let crit = DELTA_STAR[2];
    if hi >= crit && lo > 0.0 {
        "ALIVE"
    } else if hi >= crit {
        "inconclusive_wide"
    } else if lo > 0.0 {
        "positive_subcritical"
    } else if hi < 0.0 {
        "negative"
    } else {
        "null"
    }
}

/// The rungs PROTOCOL.md section 4 registers, and the fewest curves a cell may
/// have (the pilot size, below which no cell is sized).
const REGISTERED_RUNGS: [u32; 5] = [16, 20, 24, 28, 32];
const MIN_CURVES: usize = 40;

/// Refuse any input that is not a complete ladder: every registered rung, both
/// theta values and at least `min_curves` curves in every (rung, theta) cell.
fn validate(recs: &[Rec], rungs: &[u32], min_curves: usize) -> Result<(), String> {
    if let Some(r) = recs.iter().find(|r| r.theta >= THETAS.len()) {
        return Err(format!(
            "record with theta index {} (bits {})",
            r.theta, r.bits
        ));
    }
    let mut seen: Vec<u32> = recs.iter().map(|r| r.bits).collect();
    seen.sort_unstable();
    seen.dedup();
    if seen != rungs {
        return Err(format!(
            "rungs present {seen:?}, registered {rungs:?}: no reading is made"
        ));
    }
    for &b in rungs {
        for (ti, theta) in THETAS.iter().enumerate() {
            let n = recs.iter().filter(|r| r.bits == b && r.theta == ti).count();
            if n < min_curves {
                return Err(format!(
                    "bits {b} theta {theta}: {n} curves, at least {min_curves} required: no reading is made"
                ));
            }
        }
    }
    Ok(())
}

fn cmd_analyze(paths: &[String]) -> Result<(), String> {
    let recs = load(paths)?;
    validate(&recs, &REGISTERED_RUNGS, MIN_CURVES)?;
    let mut bits: Vec<u32> = recs.iter().map(|r| r.bits).collect();
    bits.sort_unstable();
    bits.dedup();
    let mut out = String::from("{\n \"rungs\":[");
    let _ = write!(
        out,
        "{}],\n \"thetas\":{:?},\n \"boot\":{BOOT},\n \"series\":[\n",
        bits.iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(","),
        THETAS
    );
    let mut first = true;
    let mut summary: Vec<Summary> = Vec::new();
    for (ti, &theta) in THETAS.iter().enumerate() {
        let rungs: Vec<Vec<&Rec>> = bits
            .iter()
            .map(|&b| {
                recs.iter()
                    .filter(|r| r.bits == b && r.theta == ti)
                    .collect()
            })
            .collect();
        let ln_r: Vec<Vec<f64>> = rungs
            .iter()
            .map(|rs| rs.iter().map(|r| (r.n as f64).ln()).collect())
            .collect();
        // per geometry, per rung, per curve values
        let vals: Vec<Option<Vec<Vec<GeomVal>>>> = (0..GEOMS.len())
            .map(|g| {
                rungs
                    .iter()
                    .map(|rs| {
                        rs.iter()
                            .map(|r| geom_value(r, g))
                            .collect::<Option<Vec<_>>>()
                    })
                    .collect::<Option<Vec<_>>>()
            })
            .collect();
        let mut rng = Rng::from_labels(&format!("pch20261029|boot|{ti}"));
        let mut kap_draws: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); bits.len()]; GEOMS.len()];
        let mut slope_draws: Vec<Vec<f64>> = vec![Vec::new(); GEOMS.len()];
        for _ in 0..BOOT {
            let mut sums = vec![vec![(0.0f64, 0.0f64); bits.len()]; GEOMS.len()];
            let mut xs = vec![0.0f64; bits.len()];
            for (i, rs) in rungs.iter().enumerate() {
                let n = rs.len();
                let pick: Vec<usize> = (0..n).map(|_| rng.below(n as u64) as usize).collect();
                xs[i] = pick.iter().map(|&k| ln_r[i][k]).sum::<f64>() / n as f64;
                for (g, v) in vals.iter().enumerate() {
                    if let Some(v) = v {
                        let (s, m) = pick
                            .iter()
                            .fold((0.0, 0.0), |a, &k| (a.0 + v[i][k].0, a.1 + v[i][k].1));
                        sums[g][i] = (s, m);
                    }
                }
            }
            for g in 0..GEOMS.len() {
                if vals[g].is_none() || sums[g].iter().any(|&(s, _)| s <= 0.0) {
                    continue;
                }
                let ys: Vec<f64> = sums[g].iter().map(|&(s, m)| (s / m).ln()).collect();
                for (i, &(s, m)) in sums[g].iter().enumerate() {
                    kap_draws[g][i].push(s / m);
                }
                slope_draws[g].push(ols(&xs, &ys));
            }
        }
        for (g, name) in GEOMS.iter().enumerate() {
            let Some(v) = &vals[g] else {
                return Err(format!(
                    "theta {theta} geometry {name}: series infeasible or incomplete: no reading is made"
                ));
            };
            let mut cells = Vec::new();
            for (i, rs) in rungs.iter().enumerate() {
                let (s, m, vv, f) = v[i].iter().fold((0.0, 0.0, 0.0, 0.0), |a, t| {
                    (a.0 + t.0, a.1 + t.1, a.2 + t.2, a.3 + t.4)
                });
                let fac = 1.0 + 1.0 / v[i][0].3;
                let mut d = kap_draws[g][i].clone();
                d.sort_by(f64::total_cmp);
                cells.push(Cell {
                    bits: bits[i],
                    n: rs.len(),
                    s,
                    m,
                    v: vv,
                    f,
                    kappa: s / m,
                    z: (s - m) / (vv * fac).sqrt(),
                    ci: (pct(&d, 0.025), pct(&d, 0.975)),
                    mean_ln_r: ln_r[i].iter().sum::<f64>() / ln_r[i].len() as f64,
                });
            }
            let xs: Vec<f64> = cells.iter().map(|c| c.mean_ln_r).collect();
            let ys: Vec<f64> = cells.iter().map(|c| c.kappa.ln()).collect();
            let slope = ols(&xs, &ys);
            let mut sd = slope_draws[g].clone();
            sd.sort_by(f64::total_cmp);
            let (lo, hi) = (pct(&sd, 0.025), pct(&sd, 0.975));
            let pred = (3..=5).contains(&g).then(|| {
                let yp: Vec<f64> = cells.iter().map(|c| (1.0 + c.f / c.m).ln()).collect();
                ols(&xs, &yp)
            });
            summary.push(Summary {
                theta,
                geom: g,
                slope,
                lo,
                hi,
                pred,
            });
            if !first {
                out.push_str(",\n");
            }
            first = false;
            let _ = write!(
                out,
                "  {{\"theta\":{},\"geometry\":\"{name}\",\"slope\":{},\"slope_ci95\":[{},{}],\"class\":\"{}\",\"slope_formal_pred\":{},\"b\":{},\"cells\":[",
                theta,
                json_f(slope),
                json_f(lo),
                json_f(hi),
                class_of(lo, hi),
                pred.map_or("null".to_string(), json_f),
                base_size(bits[0], theta)
            );
            for (i, c) in cells.iter().enumerate() {
                let _ = write!(
                    out,
                    "{}{{\"bits\":{},\"b\":{},\"curves\":{},\"S\":{},\"M\":{:.3},\"V\":{:.3},\"kappa\":{},\"ci95\":[{},{}],\"z\":{},\"kappa_formal_pred\":{}}}",
                    if i > 0 { "," } else { "" },
                    c.bits,
                    base_size(c.bits, theta),
                    c.n,
                    c.s,
                    c.m,
                    c.v,
                    json_f(c.kappa),
                    json_f(c.ci.0),
                    json_f(c.ci.1),
                    json_f(c.z),
                    if (3..=5).contains(&g) { json_f(1.0 + c.f / c.m) } else { "null".to_string() }
                );
            }
            out.push_str("]}");
        }
    }
    out.push_str("\n ],\n");
    out.push_str(&gates(&summary)?);
    out.push_str("\n}");
    println!("{out}");
    Ok(())
}

struct Summary {
    theta: f64,
    geom: usize,
    slope: f64,
    lo: f64,
    hi: f64,
    pred: Option<f64>,
}

/// The calibration gates and the registered reading (PROTOCOL.md section 7).
fn gates(s: &[Summary]) -> Result<String, String> {
    let of = |g: usize| s.iter().filter(move |x| x.geom == g);
    let null_like: Vec<&Summary> = of(3).chain(of(7)).collect();
    let null_excl = null_like
        .iter()
        .filter(|x| x.lo > 0.0 || x.hi < 0.0)
        .count();
    let g1 = null_excl <= 1;
    let zn_err: Vec<f64> = of(6).map(|x| (x.slope - (1.0 - x.theta)).abs()).collect();
    let g2 = !zn_err.is_empty() && zn_err.iter().all(|e| *e <= 0.05);
    let pl_err: Vec<f64> = of(4)
        .chain(of(5))
        .filter_map(|x| x.pred.map(|p| (x.slope - p).abs()))
        .collect();
    let g3 = !pl_err.is_empty() && pl_err.iter().all(|e| *e <= 0.03);
    let real: Vec<&Summary> = of(0).chain(of(1)).chain(of(2)).collect();
    // PROTOCOL.md section 7: six real slopes, four null-type, two Z/N, four plant.
    let counts = [
        ("real-geometry", real.len(), 6),
        ("null-type", null_like.len(), 4),
        ("Z/N", zn_err.len(), 2),
        ("plant", pl_err.len(), 4),
    ];
    for (what, got, want) in counts {
        if got != want {
            return Err(format!(
                "{what} slopes: {got} present, {want} required: no reading is made"
            ));
        }
    }
    let alive = real.iter().any(|x| class_of(x.lo, x.hi) == "ALIVE");
    let sub = !real.is_empty() && real.iter().all(|x| x.hi < DELTA_STAR[2]);
    let max_hi = real.iter().map(|x| x.hi).fold(f64::NEG_INFINITY, f64::max);
    let reading = if !(g1 && g2 && g3) {
        "GATES_FAILED_NO_READING"
    } else if alive {
        "ALIVE"
    } else if sub {
        "SUB_BREAK_EVEN_AT_SCALE"
    } else {
        "INCONCLUSIVE"
    };
    Ok(format!(
        " \"gates\":{{\"g1_null_slopes_excluding_zero\":{null_excl},\"g1_of\":{},\"g1_pass\":{g1},\"g2_zn_slope_errors\":{:?},\"g2_pass\":{g2},\"g3_plant_slope_errors\":{:?},\"g3_pass\":{g3}}},\n \"real_geometry_max_upper\":{},\n \"delta_star\":{:?},\n \"reading\":\"{reading}\"",
        null_like.len(),
        zn_err,
        pl_err,
        json_f(max_hi),
        DELTA_STAR
    ))
}

// ------------------------------------------------------------------- stage 1

type Group = ((String, String), Vec<(f64, f64, f64)>);

/// Slope of ln kappa on ln N from recorded cells (`theta d bits kappa lo hi`),
/// with a parametric bootstrap from each cell's recorded 95% interval.
fn cmd_stage1(path: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut groups: Vec<Group> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() != 6 {
            continue;
        }
        let key = (f[0].to_string(), f[1].to_string());
        let bits: f64 = f[2].parse().map_err(|_| "bits")?;
        let k: f64 = f[3].parse().map_err(|_| "kappa")?;
        let lo: f64 = f[4].parse().map_err(|_| "lo")?;
        let hi: f64 = f[5].parse().map_err(|_| "hi")?;
        let sigma = (hi.ln() - lo.ln()) / (2.0 * 1.96);
        match groups.iter_mut().find(|(g, _)| *g == key) {
            Some((_, v)) => v.push((bits * std::f64::consts::LN_2, k.ln(), sigma)),
            None => groups.push((key, vec![(bits * std::f64::consts::LN_2, k.ln(), sigma)])),
        }
    }
    let mut rng = Rng::from_labels("pch20261029|stage1");
    println!("[");
    let mut first = true;
    for ((theta, d), pts) in groups.iter().filter(|(_, p)| p.len() >= 3) {
        let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
        let slope = ols(&xs, &ys);
        let mut draws: Vec<f64> = (0..10_000)
            .map(|_| {
                let yb: Vec<f64> = pts.iter().map(|p| p.1 + p.2 * rng.normal()).collect();
                ols(&xs, &yb)
            })
            .collect();
        draws.sort_by(f64::total_cmp);
        println!(
            "{}{{\"theta\":{theta},\"d\":{d},\"points\":{},\"slope\":{},\"ci95\":[{},{}]}}",
            if first { " " } else { "," },
            pts.len(),
            json_f(slope),
            json_f(pct(&draws, 0.025)),
            json_f(pct(&draws, 0.975))
        );
        first = false;
    }
    println!("]");
    Ok(())
}

// ------------------------------------------------------------------ selftest

fn brute_order(curve: &Curve) -> u64 {
    let p = curve.p;
    let mut n = 1u64;
    for x in 0..p {
        let r = curve.rhs(x);
        n += if r == 0 {
            1
        } else if powmod(r, (p - 1) / 2, p) == 1 {
            2
        } else {
            0
        };
    }
    n
}

fn check(ok: bool, what: &str, fails: &mut Vec<String>) {
    println!("{} {what}", if ok { "ok  " } else { "FAIL" });
    if !ok {
        fails.push(what.to_string());
    }
}

fn cmd_selftest() -> Result<(), String> {
    let mut fails = Vec::new();
    check(
        is_prime(4_294_967_291)
            && !is_prime(4_294_967_297)
            && is_prime(2_957_076_383)
            && !is_prime(1),
        "is_prime",
        &mut fails,
    );
    let mut good = true;
    for p in [41u64, 101, 1009, 65_537, 1_000_000_009, 4_294_967_291] {
        if !is_prime(p) {
            continue;
        }
        for a in 1..40u64 {
            let s = sqrt_mod(a * a % p, p);
            good &= s.is_some_and(|r| r * r % p == a * a % p);
        }
        good &= invmod(7, p) * 7 % p == 1;
    }
    check(
        good,
        "sqrt_mod (both p mod 4 classes) and invmod",
        &mut fails,
    );
    // curves: the proved order equals the brute-force count
    let mut curves = Vec::new();
    let mut ok = true;
    for (bits, tries) in [(10u32, 6), (12, 6), (14, 4)] {
        for t in 0..tries {
            let mut rng = Rng::from_labels(&format!("selftest|curve|{bits}|{t}"));
            let inst = find_curve(bits, &mut rng);
            ok &= brute_order(&inst.curve) == inst.order
                && is_prime(inst.order)
                && inst.curve.on_curve(inst.gen)
                && inst.curve.mul(inst.order, inst.gen).is_none();
            curves.push(inst);
        }
    }
    check(
        ok,
        "find_curve: proved order == brute-force point count, prime, N*G = O (16 curves)",
        &mut fails,
    );
    // counters: batched sorted count == unbatched hash-map count, over every base kind
    let mut ok = true;
    let mut kinds = 0;
    for (ci, inst) in curves.iter().enumerate().filter(|(i, _)| i % 3 == 0) {
        let curve = inst.curve;
        let b = 2 * ((curve.p as f64).sqrt() as usize / 2).clamp(6, 60) - 2 * (ci % 2);
        let mut rng = Rng::from_labels(&format!("selftest|base|{ci}"));
        let mut sets: Vec<Vec<Pt>> = vec![
            null_base(&curve, &mut rng, b),
            window_base(&curve, &mut rng, b),
        ];
        sets.push(gap_base(&curve, &mut rng, b, 2));
        sets.push(gap_base(&curve, &mut rng, b, 3));
        if let Some((pts, _, _)) = weighted_base(&curve, &mut rng, b, 3) {
            sets.push(pts);
        }
        for pts in &sets {
            kinds += 1;
            ok &= pts.len() == b
                && pts.iter().all(|q| curve.on_curve(*q))
                && pts.iter().map(|q| q.x).collect::<HashSet<_>>().len() == b / 2
                && count_ec(pts, curve.p) == count_ec_slow(pts, &curve);
        }
    }
    check(
        ok,
        &format!("count_ec == count_ec_slow, exact size, on-curve, distinct x ({kinds} bases)"),
        &mut fails,
    );
    // chunk boundary: more pairs than CHUNK
    let inst = &curves[curves.len() - 1];
    let mut rng = Rng::from_labels("selftest|big");
    let pts = null_base(&inst.curve, &mut rng, 130);
    check(
        pts.len() * (pts.len() - 1) / 2 > CHUNK
            && count_ec(&pts, inst.curve.p) == count_ec_slow(&pts, &inst.curve),
        "count_ec across a CHUNK boundary",
        &mut fails,
    );
    // Z/N counter against a naive double loop
    let n = 10_007u64;
    let units: Vec<u64> = (1..=9).collect();
    let mut set: Vec<u64> = units.iter().flat_map(|&u| [u, n - u]).collect();
    set.sort_unstable();
    let mut naive = 0u64;
    let pairs: Vec<(usize, usize)> = (0..set.len())
        .flat_map(|i| (i + 1..set.len()).map(move |j| (i, j)))
        .filter(|&(i, j)| set[i] + set[j] != n)
        .collect();
    for (a, &(i, j)) in pairs.iter().enumerate() {
        for &(k, l) in pairs.iter().skip(a + 1) {
            if (set[i] + set[j]) % n == (set[k] + set[l]) % n {
                naive += 1;
            }
        }
    }
    check(
        count_zn(&units, n) == naive && naive > 0,
        "count_zn == naive pair-of-pairs count",
        &mut fails,
    );
    check(
        formal_row(1) == 0 && formal_row(2) == 0 && formal_row(3) == 2,
        "formal_row hand values (0, 0, 2)",
        &mut fails,
    );
    check(
        base_size(16, 0.30) == 28 && base_size(20, 0.30) == 64 && base_size(18, 0.35) == 78,
        "base_size matches EXP-PFDR-ca5e01 cells b16-t0.30 (28), b20-t0.30 (64)",
        &mut fails,
    );
    // plant: the group count contains the formal rows
    let mut ok = true;
    for inst in curves.iter().filter(|i| i.curve.p > 3000) {
        let curve = inst.curve;
        let mut rng = Rng::from_labels(&format!("selftest|plant|{}", curve.p));
        if let Some((pts, formal, m)) = weighted_base(&curve, &mut rng, 60, 4) {
            ok &= m == 7 && count_ec(&pts, curve.p) >= formal && formal > 0;
        }
    }
    check(
        ok,
        "weighted plant: group coincidences >= formal row coincidences",
        &mut fails,
    );
    // analyze refuses an incomplete ladder
    let mut text = String::new();
    for bits in [10u32, 12] {
        for c in 0..3 {
            for line in census_one(bits, 7, c, [true, true], false)? {
                text.push_str(&line);
                text.push('\n');
            }
        }
    }
    let full = parse_recs(&text);
    let theta0: Vec<Rec> = parse_recs(&text)
        .into_iter()
        .filter(|r| r.theta == 0)
        .collect();
    let no12: Vec<Rec> = parse_recs(&text)
        .into_iter()
        .filter(|r| r.bits == 10)
        .collect();
    check(
        validate(&full, &[10, 12], 3).is_ok()
            && validate(&theta0, &[10, 12], 3).is_err()
            && validate(&no12, &[10, 12], 3).is_err()
            && validate(&full, &[10, 12], 4).is_err()
            && validate(&full, &[10, 12, 14], 3).is_err(),
        "validate refuses a missing theta, rung or curve count",
        &mut fails,
    );
    // determinism of a whole job
    let a = census_one(12, 1, 3, [true, true], true)?;
    let b = census_one(12, 1, 3, [true, true], true)?;
    check(
        a == b && !a.is_empty(),
        "census_one is a pure function of its labels",
        &mut fails,
    );
    if fails.is_empty() {
        println!("selftest passed");
        Ok(())
    } else {
        Err(format!("{} selftest checks failed", fails.len()))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("selftest") => cmd_selftest(),
        Some("census") => cmd_census(&args[1..]),
        Some("design") => cmd_design(&args[1..]),
        Some("analyze") => cmd_analyze(&args[1..]),
        Some("stage1") => args
            .get(1)
            .ok_or("stage1 <cells.tsv>".to_string())
            .and_then(|p| cmd_stage1(p)),
        _ => Err(
            "usage: coincidence_ladder selftest | census | design | analyze | stage1".to_string(),
        ),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
