//! F6: physics-style solvers (random sampling, greedy, simulated annealing,
//! parallel tempering) on the m = 2 Semaev-S3 decomposition landscape.
//! Standalone, std-only tool implementing PROTOCOL.md (frozen 2026-11-10).
//! Curve generator: std reimplementation of the recipe of
//! `src/cryptanalysis/prime_fast.rs::find_a3_curve` (a = -3, p = 3 mod 4 the
//! largest prime below 2^n, smallest b with prime order, order by BSGS on a
//! point inside the Hasse interval).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// ------------------------------------------------------------ number theory

fn mulm(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, m);
        }
        b = mulm(b, b, m);
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut d = 5u64;
    while d * d <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d += 6;
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

fn ceil_sqrt(n: u64) -> u64 {
    let r = isqrt(n);
    if r * r == n {
        r
    } else {
        r + 1
    }
}

/// Tonelli-Shanks square root modulo an odd prime (any root), None for a non-residue.
fn sqrt_mod(n: u64, p: u64) -> Option<u64> {
    let n = n % p;
    if n == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(n);
    }
    if powmod(n, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(powmod(n, (p + 1) / 4, p));
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(n, q, p);
    let mut r = powmod(n, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0u32;
        let mut t2 = t;
        while t2 != 1 {
            t2 = mulm(t2, t2, p);
            i += 1;
        }
        let b = powmod(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

// FNV-1a seeded SplitMix64
struct Rng(u64);
impl Rng {
    fn from_seed_str(s: &str) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in s.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        Rng(h)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// uniform in [0, n)
    fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
    fn f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}


// SHA-256
fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    let bitlen = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(t1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = t1.wrapping_add(t2);
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    let mut s = String::new();
    for x in h {
        write!(s, "{:08x}", x).unwrap();
    }
    s
}


#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Int(i128),
    F(f64),
    S(String),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

fn jo(items: Vec<(&str, J)>) -> J {
    J::O(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
fn ji<T: Into<i128>>(x: T) -> J {
    J::Int(x.into())
}
fn js(s: &str) -> J {
    J::S(s.to_string())
}

impl J {
    fn scalar(&self) -> bool {
        !matches!(self, J::A(_) | J::O(_))
    }
    fn render(&self, out: &mut String, ind: usize) {
        match self {
            J::Null => out.push_str("null"),
            J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            J::Int(i) => write!(out, "{}", i).unwrap(),
            J::F(f) => {
                if f.is_finite() {
                    write!(out, "{:e}", f).unwrap()
                } else {
                    out.push_str("null")
                }
            }
            J::S(s) => {
                out.push('"');
                for ch in s.chars() {
                    match ch {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            J::A(v) => {
                if v.iter().all(|x| x.scalar()) {
                    out.push('[');
                    for (i, x) in v.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        x.render(out, ind);
                    }
                    out.push(']');
                } else {
                    out.push_str("[\n");
                    for (i, x) in v.iter().enumerate() {
                        out.push_str(&" ".repeat(ind + 2));
                        x.render(out, ind + 2);
                        if i + 1 < v.len() {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    out.push_str(&" ".repeat(ind));
                    out.push(']');
                }
            }
            J::O(v) => {
                out.push_str("{\n");
                for (i, (k, x)) in v.iter().enumerate() {
                    out.push_str(&" ".repeat(ind + 2));
                    J::S(k.clone()).render(out, ind);
                    out.push_str(": ");
                    x.render(out, ind + 2);
                    if i + 1 < v.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                out.push_str(&" ".repeat(ind));
                out.push('}');
            }
        }
    }
}


fn par_map<T: Sync, R: Send>(items: &[T], threads: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= items.len() {
                    break;
                }
                let r = f(&items[i]);
                out.lock().unwrap()[i] = Some(r);
            });
        }
    });
    out.into_inner().unwrap().into_iter().map(|x| x.unwrap()).collect()
}



fn jf(x: f64) -> J {
    J::F(x)
}
fn jopt(x: Option<f64>) -> J {
    match x {
        Some(v) => J::F(v),
        None => J::Null,
    }
}
fn jb(b: bool) -> J {
    J::Bool(b)
}

// ------------------------------------------------------------------ curve

type Pt = Option<(u64, u64)>;

#[derive(Clone, Debug)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (mulm(mulm(x, x, p), x, p) + mulm(self.a, x, p) + self.b) % p
    }
    fn lift_x(&self, x: u64) -> Option<u64> {
        sqrt_mod(self.rhs(x), self.p)
    }
    fn neg(&self, q: Pt) -> Pt {
        q.map(|(x, y)| (x, (self.p - y) % self.p))
    }
    #[allow(dead_code)]
    fn on_curve(&self, q: Pt) -> bool {
        match q {
            None => true,
            Some((x, y)) => mulm(y, y, self.p) == self.rhs(x),
        }
    }
    fn add(&self, q1: Pt, q2: Pt) -> Pt {
        let p = self.p;
        let (x1, y1) = match q1 {
            None => return q2,
            Some(v) => v,
        };
        let (x2, y2) = match q2 {
            None => return q1,
            Some(v) => v,
        };
        let lam = if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            let num = (3 * mulm(x1, x1, p) + self.a) % p;
            let den = (2 * y1) % p;
            mulm(num, powmod(den, p - 2, p), p)
        } else {
            let num = (y2 + p - y1) % p;
            let den = (x2 + p - x1) % p;
            mulm(num, powmod(den, p - 2, p), p)
        };
        let x3 = (mulm(lam, lam, p) + 2 * p - x1 - x2) % p;
        let y3 = (mulm(lam, (x1 + p - x3) % p, p) + p - y1) % p;
        Some((x3, y3))
    }
    fn mul(&self, q: Pt, mut k: u64) -> Pt {
        let mut acc: Pt = None;
        let mut base = q;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
}

/// Group order of the curve from a point g of large order: the unique N in the
/// Hasse interval with [N]g = O (BSGS); None when ambiguous.
fn order_bsgs(c: &Curve, g: (u64, u64)) -> Option<u64> {
    let p = c.p;
    let s = isqrt(p) + 1;
    let lo = p + 1 - 2 * s;
    hi_check(p, s);
    let width = 4 * s + 1;
    let m = ceil_sqrt(width).max(2);
    const O: (u64, u64) = (u64::MAX, 0);
    let mut baby: HashMap<(u64, u64), u64> = HashMap::with_capacity(m as usize * 2);
    let mut cur: Pt = None;
    for j in 0..m {
        baby.entry(cur.unwrap_or(O)).or_insert(j);
        cur = c.add(cur, Some(g));
    }
    let mg = c.mul(Some(g), m);
    let mut t = c.mul(Some(g), lo);
    let mut cand: Vec<u64> = vec![];
    for i in 0..=(width / m + 1) {
        let key = match t {
            None => O,
            Some((x, y)) => (x, (p - y) % p),
        };
        if let Some(&j) = baby.get(&key) {
            cand.push(lo + i * m + j);
        }
        t = c.add(t, mg);
    }
    cand.sort_unstable();
    cand.dedup();
    let hi = p + 1 + 2 * s;
    cand.retain(|&nn| nn >= lo && nn <= hi && c.mul(Some(g), nn).is_none());
    if cand.len() == 1 {
        Some(cand[0])
    } else {
        None
    }
}

fn hi_check(p: u64, _s: u64) {
    assert!(p >= 100 && p < (1u64 << 32), "prime size outside tool range");
}

#[derive(Clone, Debug)]
struct CurveInfo {
    n: u32,
    curve: Curve,
    order: u64,
    gx: u64,
    gy: u64,
    b_tried: u64,
}

fn find_a3_curve(n: u32) -> Result<CurveInfo, String> {
    let mut p = (1u64 << n) - 1;
    while !(p % 4 == 3 && is_prime(p)) {
        p -= 1;
    }
    let a = p - 3;
    for b in 1u64..1_000_000 {
        let disc = (4 * mulm(mulm(a, a, p), a, p) + 27 * mulm(b, b, p)) % p;
        if disc == 0 {
            continue;
        }
        let c = Curve { p, a, b };
        let Some((gx, gy)) = (1u64..10_000).find_map(|x| c.lift_x(x).map(|y| (x, y))) else {
            continue;
        };
        let Some(order) = order_bsgs(&c, (gx, gy)) else {
            continue;
        };
        if !is_prime(order) {
            continue;
        }
        if c.mul(Some((gx, gy)), order).is_some() {
            continue;
        }
        return Ok(CurveInfo { n, curve: c, order, gx, gy, b_tried: b });
    }
    Err(format!("no a=-3 prime-order curve found for n={}", n))
}

/// Committed ladder rungs (src/cryptanalysis/research_bench.rs `bench_curves_a_minus_3`,
/// P-256-class entries): (bits, p, b, order).
const KNOWN_RUNGS: [(u32, u64, u64, u64); 4] = [
    (16, 65_519, 76, 65_447),
    (20, 1_048_571, 44, 1_048_189),
    (24, 16_777_199, 31, 16_782_299),
    (28, 268_435_399, 3, 268_407_199),
];

fn known_rung(ci: &CurveInfo) -> Option<bool> {
    KNOWN_RUNGS
        .iter()
        .find(|r| r.0 == ci.n)
        .map(|r| r.1 == ci.curve.p && r.2 == ci.curve.b && r.3 == ci.order)
}

// ------------------------------------------------------------------ Semaev S3

/// S3(x1,x2,x3) = (x1-x2)^2 x3^2 - 2((x1+x2)(x1x2+a)+2b) x3 + ((x1x2-a)^2 - 4b(x1+x2)) mod p.
#[allow(dead_code)]
fn s3(p: u64, a: u64, b: u64, x1: u64, x2: u64, x3: u64) -> u64 {
    let (x1, x2, x3) = (x1 % p, x2 % p, x3 % p);
    let s = (x1 + x2) % p;
    let m = mulm(x1, x2, p);
    let d = (x1 + p - x2) % p;
    let lead = mulm(mulm(d, d, p), mulm(x3, x3, p), p);
    let t = (mulm(s, (m + a) % p, p) + 2 * b) % p;
    let mid = mulm(2 * t % p, x3, p);
    let e = (m + p - a) % p;
    let cst = (mulm(e, e, p) + p - mulm(4 * b % p, s, p)) % p;
    (lead + p - mid + cst) % p
}

/// Fast cost oracle with x3 = x_R fixed (p < 2^32 so products fit in u64).
#[derive(Clone, Copy)]
struct Cost {
    p: u64,
    a: u64,
    b: u64,
    xr: u64,
    xr2: u64,
}
impl Cost {
    fn new(p: u64, a: u64, b: u64, xr: u64) -> Cost {
        assert!(p < (1u64 << 32));
        Cost { p, a, b, xr, xr2: xr * xr % p }
    }
    #[inline]
    fn c(&self, x1: u64, x2: u64) -> u64 {
        let p = self.p;
        let s = (x1 + x2) % p;
        let m = x1 * x2 % p;
        let d = (x1 + p - x2) % p;
        let a2 = d * d % p;
        let t = (s * ((m + self.a) % p) % p + 2 * self.b) % p;
        let bc = 2 * t % p * self.xr % p;
        let e = (m + p - self.a) % p;
        let cc = (e * e % p + p - 4 * self.b % p * s % p) % p;
        let v = (a2 * self.xr2 % p + p - bc + cc) % p;
        v.min(p - v)
    }
}

// ------------------------------------------------------------------ instances

#[derive(Clone, Debug)]
struct Inst {
    n: u32,
    i: usize,
    x1: u64,
    x2: u64,
    xr: u64,
    yr: u64,
}

fn box_b(p: u64) -> u64 {
    ceil_sqrt(p)
}

fn gen_instance(ci: &CurveInfo, n: u32, i: usize) -> Inst {
    let c = &ci.curve;
    let bb = box_b(c.p);
    let mut rng = Rng::from_seed_str(&format!("F6/inst/{}/{}", n, i));
    loop {
        let x1 = 1 + rng.below(bb);
        let x2 = 1 + rng.below(bb);
        let (Some(y1), Some(y2)) = (c.lift_x(x1), c.lift_x(x2)) else {
            continue;
        };
        let y1 = if rng.next() & 1 == 1 { (c.p - y1) % c.p } else { y1 };
        let y2 = if rng.next() & 1 == 1 { (c.p - y2) % c.p } else { y2 };
        if let Some((xr, yr)) = c.add(Some((x1, y1)), Some((x2, y2))) {
            return Inst { n, i, x1, x2, xr, yr };
        }
    }
}

/// Lift x1, x2 and check P1 +- P2 = +-R.
fn verify(c: &Curve, x1: u64, x2: u64, xr: u64, yr: u64) -> bool {
    let (Some(y1), Some(y2)) = (c.lift_x(x1), c.lift_x(x2)) else {
        return false;
    };
    let p1 = Some((x1, y1));
    let p2 = Some((x2, y2));
    for sum in [c.add(p1, p2), c.add(p1, c.neg(p2))] {
        if let Some((x, y)) = sum {
            if x == xr && (y == yr || y == (c.p - yr) % c.p) {
                return true;
            }
        }
    }
    false
}

// ------------------------------------------------------------------ root-solving enumeration

struct RootEnum {
    first: Option<(u64, u64)>,
    first_evals: u64, // number of x1 values (quadratic solves) up to and including the first solution
    solutions: Vec<(u64, u64)>, // all verified ordered states in the box
    inconsistent: u64, // quadratic roots in the box with c != 0 (must stay 0)
}

fn root_enum(c: &Curve, bb: u64, inst: &Inst) -> RootEnum {
    let p = c.p;
    let cost = Cost::new(p, c.a, c.b, inst.xr);
    let mut out = RootEnum { first: None, first_evals: 0, solutions: vec![], inconsistent: 0 };
    for x1 in 1..=bb {
        // S3(x1, w, xR) as a quadratic in w: A w^2 - 2 Bh w + C
        let u = x1 % p;
        let v = inst.xr % p;
        let amp = {
            let d = (u + p - v) % p;
            mulm(d, d, p)
        };
        let uv = mulm(u, v, p);
        let bh = (mulm((u + v) % p, (uv + c.a) % p, p) + 2 * c.b) % p;
        let e = (uv + p - c.a) % p;
        let cst = (mulm(e, e, p) + p - mulm(4 * c.b % p, (u + v) % p, p)) % p;
        let mut roots: Vec<u64> = vec![];
        if amp == 0 {
            if bh != 0 {
                roots.push(mulm(cst, powmod(2 * bh % p, p - 2, p), p));
            }
        } else {
            let disc = (mulm(bh, bh, p) + p - mulm(amp, cst, p)) % p;
            if let Some(s) = sqrt_mod(disc, p) {
                let inv = powmod(amp, p - 2, p);
                roots.push(mulm((bh + s) % p, inv, p));
                if s != 0 {
                    roots.push(mulm((bh + p - s) % p, inv, p));
                }
            }
        }
        for w in roots {
            if w >= 1 && w <= bb {
                if cost.c(x1, w) != 0 {
                    out.inconsistent += 1;
                    continue;
                }
                if verify(c, x1, w, inst.xr, inst.yr) {
                    out.solutions.push((x1, w));
                    if out.first.is_none() {
                        out.first = Some((x1, w));
                        out.first_evals = x1;
                    }
                }
            }
        }
    }
    out
}

// ------------------------------------------------------------------ search space and oracle

struct Space {
    bb: u64,
    k: u32,
}
impl Space {
    fn new(bb: u64) -> Space {
        let mut k = 0;
        while (1u64 << k) < bb {
            k += 1;
        }
        Space { bb, k }
    }
    fn n_moves(&self) -> usize {
        2 * (self.k as usize + 2)
    }
    fn random_state(&self, rng: &mut Rng) -> (u64, u64) {
        (rng.below(self.bb), rng.below(self.bb))
    }
    /// Move m: coordinate m/(k+2); op < k flips a bit of the Gray word, k is +1, k+1 is -1.
    fn neighbor(&self, st: (u64, u64), m: usize) -> Option<(u64, u64)> {
        let nm = self.k as usize + 2;
        let coord = m / nm;
        let op = m % nm;
        let i = if coord == 0 { st.0 } else { st.1 };
        let ni = if op < self.k as usize {
            gray_dec((i ^ (i >> 1)) ^ (1u64 << op))
        } else if op == self.k as usize {
            i + 1
        } else {
            if i == 0 {
                return None;
            }
            i - 1
        };
        if ni >= self.bb {
            return None;
        }
        Some(if coord == 0 { (ni, st.1) } else { (st.0, ni) })
    }
    fn random_neighbor(&self, st: (u64, u64), rng: &mut Rng) -> (u64, u64) {
        loop {
            let m = rng.below(self.n_moves() as u64) as usize;
            if let Some(s) = self.neighbor(st, m) {
                return s;
            }
        }
    }
    /// single bit flip of the Gray word of a random coordinate (landscape walks)
    fn random_bitflip(&self, st: (u64, u64), rng: &mut Rng) -> (u64, u64) {
        loop {
            let coord = rng.below(2) as usize;
            let bit = rng.below(self.k as u64) as usize;
            if let Some(s) = self.neighbor(st, coord * (self.k as usize + 2) + bit) {
                return s;
            }
        }
    }
}

fn gray_dec(mut g: u64) -> u64 {
    let mut n = 0;
    while g != 0 {
        n ^= g;
        g >>= 1;
    }
    n
}

struct Oracle<'a> {
    cost: Cost,
    curve: &'a Curve,
    yr: u64,
    evals: u64,
    cap: u64,
    found: Option<(u64, u64)>,
    found_at: u64,
    spurious: u64,
}
impl<'a> Oracle<'a> {
    fn new(curve: &'a Curve, inst: &Inst, cap: u64) -> Oracle<'a> {
        Oracle {
            cost: Cost::new(curve.p, curve.a, curve.b, inst.xr),
            curve,
            yr: inst.yr,
            evals: 0,
            cap,
            found: None,
            found_at: 0,
            spurious: 0,
        }
    }
    /// one cost evaluation at index state (i1, i2) = abscissae (i1+1, i2+1)
    #[inline]
    fn eval(&mut self, i1: u64, i2: u64) -> u64 {
        self.evals += 1;
        let (x1, x2) = (i1 + 1, i2 + 1);
        let c = self.cost.c(x1, x2);
        if c == 0 && self.found.is_none() {
            if verify(self.curve, x1, x2, self.cost.xr, self.yr) {
                self.found = Some((x1, x2));
                self.found_at = self.evals;
            } else {
                self.spurious += 1;
            }
        }
        c
    }
    #[inline]
    fn done(&self) -> bool {
        self.found.is_some() || self.evals >= self.cap
    }
}

// ------------------------------------------------------------------ solvers

const SOLVERS: [&str; 6] = ["uniform", "greedy", "sa_B", "sa_4B", "sa_16B", "pt"];

fn solve_uniform(o: &mut Oracle, sp: &Space, rng: &mut Rng) {
    while !o.done() {
        let s = sp.random_state(rng);
        o.eval(s.0, s.1);
    }
}

fn solve_greedy(o: &mut Oracle, sp: &Space, rng: &mut Rng) {
    while !o.done() {
        let mut st = sp.random_state(rng);
        let mut cur = o.eval(st.0, st.1);
        loop {
            if o.done() {
                return;
            }
            let mut best = cur;
            let mut best_st = st;
            for m in 0..sp.n_moves() {
                if let Some(nb) = sp.neighbor(st, m) {
                    let c = o.eval(nb.0, nb.1);
                    if c < best {
                        best = c;
                        best_st = nb;
                    }
                    if o.done() {
                        return;
                    }
                }
            }
            if best < cur {
                cur = best;
                st = best_st;
            } else {
                break; // local minimum: restart
            }
        }
    }
}

fn metropolis(rng: &mut Rng, delta: f64, t: f64) -> bool {
    delta <= 0.0 || rng.f64() < (-delta / t).exp()
}

fn solve_sa(o: &mut Oracle, sp: &Space, rng: &mut Rng, steps: u64, p: u64) {
    let t0 = p as f64 / 4.0;
    let t1 = 1.0f64;
    let ratio = (t1 / t0).powf(1.0 / (steps.max(2) - 1) as f64);
    while !o.done() {
        let mut st = sp.random_state(rng);
        let mut cur = o.eval(st.0, st.1) as f64;
        let mut t = t0;
        for _ in 0..steps {
            if o.done() {
                return;
            }
            let nb = sp.random_neighbor(st, rng);
            let c = o.eval(nb.0, nb.1) as f64;
            if metropolis(rng, c - cur, t) {
                st = nb;
                cur = c;
            }
            t *= ratio;
        }
    }
}

fn solve_pt(o: &mut Oracle, sp: &Space, rng: &mut Rng, p: u64) {
    const R: usize = 8;
    let t_lo = 1.0f64;
    let t_hi = p as f64 / 4.0;
    let temps: Vec<f64> = (0..R).map(|j| t_lo * (t_hi / t_lo).powf(j as f64 / (R - 1) as f64)).collect();
    let mut st: Vec<(u64, u64)> = vec![];
    let mut en: Vec<f64> = vec![];
    for _ in 0..R {
        let s = sp.random_state(rng);
        let c = o.eval(s.0, s.1) as f64;
        st.push(s);
        en.push(c);
        if o.done() {
            return;
        }
    }
    let mut sweep = 0u64;
    while !o.done() {
        for j in 0..R {
            let nb = sp.random_neighbor(st[j], rng);
            let c = o.eval(nb.0, nb.1) as f64;
            if metropolis(rng, c - en[j], temps[j]) {
                st[j] = nb;
                en[j] = c;
            }
            if o.done() {
                return;
            }
        }
        sweep += 1;
        if sweep % 10 == 0 {
            let parity = ((sweep / 10) % 2) as usize;
            let mut j = parity;
            while j + 1 < R {
                let arg = (1.0 / temps[j] - 1.0 / temps[j + 1]) * (en[j] - en[j + 1]);
                if arg >= 0.0 || rng.f64() < arg.exp() {
                    st.swap(j, j + 1);
                    en.swap(j, j + 1);
                }
                j += 2;
            }
        }
    }
}

fn run_solver(name: &str, o: &mut Oracle, sp: &Space, rng: &mut Rng, p: u64, bb: u64) {
    match name {
        "uniform" => solve_uniform(o, sp, rng),
        "greedy" => solve_greedy(o, sp, rng),
        "sa_B" => solve_sa(o, sp, rng, bb, p),
        "sa_4B" => solve_sa(o, sp, rng, 4 * bb, p),
        "sa_16B" => solve_sa(o, sp, rng, 16 * bb, p),
        "pt" => solve_pt(o, sp, rng, p),
        _ => panic!("unknown solver {}", name),
    }
}

struct Row {
    n: u32,
    i: usize,
    solver: String,
    evals: u64,
    censored: bool,
    solved: bool,
    found: Option<(u64, u64)>,
    planted_state: bool,
    spurious: u64,
}

fn run_row(ci: &CurveInfo, inst: &Inst, solver: &str) -> Row {
    let c = &ci.curve;
    let bb = box_b(c.p);
    let sp = Space::new(bb);
    let cap = (50 * bb * bb).min(2_000_000_000);
    let mut o = Oracle::new(c, inst, cap);
    let mut rng = Rng::from_seed_str(&format!("F6/solver/{}/{}/{}", solver, inst.n, inst.i));
    run_solver(solver, &mut o, &sp, &mut rng, c.p, bb);
    let solved = o.found.is_some();
    let planted = o
        .found
        .map(|(a, b)| (a, b) == (inst.x1, inst.x2) || (a, b) == (inst.x2, inst.x1))
        .unwrap_or(false);
    Row {
        n: inst.n,
        i: inst.i,
        solver: solver.to_string(),
        evals: if solved { o.found_at } else { cap },
        censored: !solved,
        solved,
        found: o.found,
        planted_state: planted,
        spurious: o.spurious,
    }
}

// ------------------------------------------------------------------ landscape statistic

const LAGS: usize = 8;
const WALKS: usize = 32;
const WALK_STEPS: usize = 1000;

fn acf(c: &[f64]) -> Option<[f64; LAGS]> {
    let nn = c.len();
    let mu = c.iter().sum::<f64>() / nn as f64;
    let den: f64 = c.iter().map(|x| (x - mu) * (x - mu)).sum();
    if den <= 0.0 {
        return None;
    }
    let mut r = [0.0; LAGS];
    for k in 1..=LAGS {
        let mut s = 0.0;
        for t in 0..nn - k {
            s += (c[t] - mu) * (c[t + k] - mu);
        }
        r[k - 1] = s / den;
    }
    Some(r)
}

/// (walk acfs, random-landscape acfs), 32 each
fn landscape_task(ci: &CurveInfo, inst: &Inst) -> (Vec<[f64; LAGS]>, Vec<[f64; LAGS]>) {
    let c = &ci.curve;
    let bb = box_b(c.p);
    let sp = Space::new(bb);
    let cost = Cost::new(c.p, c.a, c.b, inst.xr);
    let mut rw = Rng::from_seed_str(&format!("F6/walk/{}/{}", inst.n, inst.i));
    let mut rr = Rng::from_seed_str(&format!("F6/rand/{}/{}", inst.n, inst.i));
    let (mut wa, mut ra) = (vec![], vec![]);
    for _ in 0..WALKS {
        let mut st = sp.random_state(&mut rw);
        let mut seq = vec![cost.c(st.0 + 1, st.1 + 1) as f64];
        for _ in 0..WALK_STEPS {
            st = sp.random_bitflip(st, &mut rw);
            seq.push(cost.c(st.0 + 1, st.1 + 1) as f64);
        }
        if let Some(r) = acf(&seq) {
            wa.push(r);
        }
        let mut seq2 = vec![];
        for _ in 0..=WALK_STEPS {
            let s = sp.random_state(&mut rr);
            seq2.push(cost.c(s.0 + 1, s.1 + 1) as f64);
        }
        if let Some(r) = acf(&seq2) {
            ra.push(r);
        }
    }
    (wa, ra)
}

// ------------------------------------------------------------------ statistics

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return 0.0;
    }
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}
fn median_sorted(v: &[f64]) -> f64 {
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}
fn percentile(sorted: &[f64], q: f64) -> f64 {
    let pos = q * (sorted.len() - 1) as f64;
    let (lo, hi) = (pos.floor() as usize, pos.ceil() as usize);
    sorted[lo] + (sorted[hi] - sorted[lo]) * (pos - lo as f64)
}

/// median of the pooled values, defined (exactly) only when fewer than half are censored
fn median_valid(vals: &[(f64, bool)]) -> Option<f64> {
    let cens = vals.iter().filter(|v| v.1).count();
    if cens * 2 >= vals.len() {
        return None;
    }
    let mut v: Vec<f64> = vals.iter().map(|x| x.0).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Some(median_sorted(&v))
}

fn ols_slope(pts: &[(f64, f64)]) -> Option<f64> {
    if pts.len() < 2 {
        return None;
    }
    let mx = pts.iter().map(|p| p.0).sum::<f64>() / pts.len() as f64;
    let my = pts.iter().map(|p| p.1).sum::<f64>() / pts.len() as f64;
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx) * (p.0 - mx)).sum();
    if sxx <= 0.0 {
        return None;
    }
    Some(pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum::<f64>() / sxx)
}

struct Fit {
    slope: Option<f64>,
    ci: Option<(f64, f64)>,
    json: J,
}

/// `data`: per n: (n, B, per-instance (evals, censored)). Slope of ln(median) on ln(B)
/// over the n whose median is uncensored; bootstrap CI over instances.
fn fit(solver: &str, set: &str, data: &[(u32, f64, Vec<(f64, bool)>)], reps: usize) -> Fit {
    let mut pts = vec![];
    let mut jpts = vec![];
    let mut used: Vec<usize> = vec![];
    for (idx, (n, bb, vals)) in data.iter().enumerate() {
        let cens = vals.iter().filter(|v| v.1).count();
        let med = median_valid(vals);
        jpts.push(jo(vec![
            ("n", ji(*n)),
            ("B", jf(*bb)),
            ("median_evals", jopt(med)),
            ("censored", ji(cens as u32)),
            ("instances", ji(vals.len() as u32)),
        ]));
        if let Some(m) = med {
            pts.push((bb.ln(), m.ln()));
            used.push(idx);
        }
    }
    let slope = ols_slope(&pts);
    let mut ci = None;
    if slope.is_some() {
        let mut rng = Rng::from_seed_str(&format!("F6/boot/{}/{}", solver, set));
        let mut slopes = vec![];
        for _ in 0..reps {
            let mut bp = vec![];
            for &idx in &used {
                let vals = &data[idx].2;
                let rs: Vec<(f64, bool)> = (0..vals.len()).map(|_| vals[rng.below(vals.len() as u64) as usize]).collect();
                if let Some(m) = median_valid(&rs) {
                    bp.push((data[idx].1.ln(), m.ln()));
                }
            }
            if let Some(s) = ols_slope(&bp) {
                slopes.push(s);
            }
        }
        if slopes.len() >= reps / 2 {
            slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());
            ci = Some((percentile(&slopes, 0.025), percentile(&slopes, 0.975)));
        }
    }
    let json = jo(vec![
        ("solver", js(solver)),
        ("set", js(set)),
        ("ns_used", J::A(used.iter().map(|&i| ji(data[i].0)).collect())),
        ("slope", jopt(slope)),
        ("ci95_lo", jopt(ci.map(|c| c.0))),
        ("ci95_hi", jopt(ci.map(|c| c.1))),
        ("bootstrap_reps", ji(reps as u32)),
        ("points", J::A(jpts)),
    ]);
    Fit { slope, ci, json }
}



// ------------------------------------------------------------------ CLI / driver

struct Args {
    out: String,
    ns: Vec<u32>,
    quick: bool,
    instances: usize,
    reps: usize,
    protocol: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut out = "results/run.json".to_string();
    let mut ns: Option<Vec<u32>> = None;
    let mut quick = false;
    let mut instances: Option<usize> = None;
    let mut reps = 1000usize;
    let mut protocol = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--out" => {
                i += 1;
                out = argv.get(i).ok_or("--out needs a value")?.clone();
            }
            "--n" => {
                let mut v = vec![];
                while i + 1 < argv.len() && !argv[i + 1].starts_with("--") {
                    i += 1;
                    for tok in argv[i].split(',').filter(|t| !t.is_empty()) {
                        v.push(tok.parse::<u32>().map_err(|_| format!("bad --n value {}", tok))?);
                    }
                }
                if v.is_empty() {
                    return Err("--n needs values".into());
                }
                ns = Some(v);
            }
            "--quick" => quick = true,
            "--instances" => {
                i += 1;
                instances = Some(argv.get(i).ok_or("--instances needs a value")?.parse().map_err(|_| "bad --instances")?);
            }
            "--boot" => {
                i += 1;
                reps = argv.get(i).ok_or("--boot needs a value")?.parse().map_err(|_| "bad --boot")?;
            }
            "--protocol" => {
                i += 1;
                protocol = Some(argv.get(i).ok_or("--protocol needs a value")?.clone());
            }
            x => return Err(format!("unknown argument {}", x)),
        }
        i += 1;
    }
    let ns = ns.unwrap_or_else(|| if quick { vec![12, 16] } else { vec![12, 16, 20, 24, 28] });
    for &n in &ns {
        if !(10..=30).contains(&n) {
            return Err(format!("n={} outside supported range 10..30", n));
        }
    }
    let instances = instances.unwrap_or(if quick { 5 } else { 20 });
    Ok(Args { out, ns, quick, instances, reps, protocol })
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(64);
        }
    };
    std::process::exit(run(&args));
}

fn run(args: &Args) -> i32 {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let proto_path = {
        let mut cands: Vec<String> = vec![];
        if let Some(p) = &args.protocol {
            cands.push(p.clone());
        }
        cands.push(format!("{}/../PROTOCOL.md", env!("CARGO_MANIFEST_DIR")));
        cands.push("../PROTOCOL.md".into());
        cands.push("PROTOCOL.md".into());
        cands.into_iter().find(|p| std::path::Path::new(p).exists())
    };
    let protocol_sha = match &proto_path {
        Some(p) => sha256(&std::fs::read(p).unwrap()),
        None => "unavailable".to_string(),
    };
    let source_sha = sha256(include_bytes!("main.rs"));
    let solvers: Vec<&str> = if args.quick { vec!["uniform", "sa_4B"] } else { SOLVERS.to_vec() };

    // curves
    let mut curves: Vec<CurveInfo> = vec![];
    for &n in &args.ns {
        match find_a3_curve(n) {
            Ok(ci) => curves.push(ci),
            Err(e) => {
                eprintln!("fatal: {}", e);
                return 2;
            }
        }
    }
    // instances
    let insts: Vec<Vec<Inst>> = curves.iter().map(|ci| (0..args.instances).map(|i| gen_instance(ci, ci.n, i)).collect()).collect();

    // root-solving enumeration (measured baseline), also counts all box solutions
    let re_jobs: Vec<(usize, usize)> = (0..curves.len()).flat_map(|c| (0..args.instances).map(move |i| (c, i))).collect();
    let re_res = par_map(&re_jobs, threads, |&(c, i)| {
        let ci = &curves[c];
        root_enum(&ci.curve, box_b(ci.curve.p), &insts[c][i])
    });

    // solver rows
    let n_solvers = solvers.len();
    let jobs: Vec<(usize, usize, usize)> = (0..curves.len())
        .flat_map(|c| (0..args.instances).flat_map(move |i| (0..n_solvers).map(move |s| (c, i, s))))
        .collect();
    let rows = par_map(&jobs, threads, |&(c, i, s)| run_row(&curves[c], &insts[c][i], solvers[s]));

    // landscape (n <= 20 only)
    let ls_jobs: Vec<(usize, usize)> = re_jobs.iter().cloned().filter(|&(c, _)| curves[c].n <= 20).collect();
    let ls_res = par_map(&ls_jobs, threads, |&(c, i)| landscape_task(&curves[c], &insts[c][i]));

    // ---- verification audit (independent re-check of every reported solution)
    let mut verification_failures = 0usize;
    let mut instrument_errors: Vec<String> = vec![];
    for r in &rows {
        if let Some((x1, x2)) = r.found {
            let ci = curves.iter().position(|ci| ci.n == r.n).unwrap();
            let inst = &insts[ci][r.i];
            if !verify(&curves[ci].curve, x1, x2, inst.xr, inst.yr) {
                verification_failures += 1;
            }
        }
    }
    for (k, &(c, i)) in re_jobs.iter().enumerate() {
        let re = &re_res[k];
        let inst = &insts[c][i];
        if re.inconsistent > 0 {
            instrument_errors.push(format!("n={} i={}: {} quadratic roots with c != 0", curves[c].n, i, re.inconsistent));
        }
        if !re.solutions.contains(&(inst.x1, inst.x2)) {
            instrument_errors.push(format!("n={} i={}: root enumeration missed the planted state", curves[c].n, i));
        }
    }

    // ---- aggregates
    let mut data: HashMap<(String, u32), Vec<(f64, bool)>> = HashMap::new();
    for r in &rows {
        data.entry((r.solver.clone(), r.n)).or_default().push((r.evals as f64, r.censored));
    }
    for (k, &(c, _i)) in re_jobs.iter().enumerate() {
        data.entry(("root_enum".to_string(), curves[c].n)).or_default().push((re_res[k].first_evals as f64, false));
    }
    let mut all_solvers: Vec<String> = solvers.iter().map(|s| s.to_string()).collect();
    all_solvers.push("root_enum".to_string());

    let mut fits_json = vec![];
    let mut fit_all: HashMap<String, Fit> = HashMap::new();
    let mut fit_20: HashMap<String, Fit> = HashMap::new();
    for s in &all_solvers {
        for (set, min_n) in [("all", 0u32), ("n_ge_20", 20u32)] {
            let d: Vec<(u32, f64, Vec<(f64, bool)>)> = curves
                .iter()
                .filter(|ci| ci.n >= min_n)
                .map(|ci| (ci.n, box_b(ci.curve.p) as f64, data[&(s.clone(), ci.n)].clone()))
                .collect();
            let f = fit(s, set, &d, args.reps);
            fits_json.push(f.json.clone());
            if set == "all" {
                fit_all.insert(s.clone(), f);
            } else {
                fit_20.insert(s.clone(), f);
            }
        }
    }

    // landscape aggregation
    let mut land_json = vec![];
    let mut lag1_fail_sizes = 0usize;
    let mut lag1_rows: Vec<(u32, f64, f64, f64, bool)> = vec![];
    let mut p2_all_in_band = true;
    let ns_land: Vec<u32> = curves.iter().map(|c| c.n).filter(|&n| n <= 20).collect();
    for &n in &ns_land {
        let mut w: Vec<[f64; LAGS]> = vec![];
        let mut rr: Vec<[f64; LAGS]> = vec![];
        for (k, &(c, _)) in ls_jobs.iter().enumerate() {
            if curves[c].n == n {
                w.extend(ls_res[k].0.iter().cloned());
                rr.extend(ls_res[k].1.iter().cloned());
            }
        }
        let mut lag_json = vec![];
        for l in 0..LAGS {
            let wv: Vec<f64> = w.iter().map(|x| x[l]).collect();
            let rv: Vec<f64> = rr.iter().map(|x| x[l]).collect();
            let (wm, ws, rm, rs) = (mean(&wv), sd(&wv), mean(&rv), sd(&rv));
            let (lo, hi) = (rm - 3.0 * rs, rm + 3.0 * rs);
            let in_band = wm >= lo && wm <= hi;
            let se = (ws * ws / wv.len() as f64 + rs * rs / rv.len() as f64).sqrt();
            let z = if se > 0.0 { (wm - rm) / se } else { 0.0 };
            if !in_band {
                p2_all_in_band = false;
            }
            if l == 0 {
                if !in_band {
                    lag1_fail_sizes += 1;
                }
                lag1_rows.push((n, wm, rm, rs, in_band));
            }
            lag_json.push(jo(vec![
                ("lag", ji((l + 1) as u32)),
                ("walk_mean", jf(wm)),
                ("walk_sd", jf(ws)),
                ("rand_mean", jf(rm)),
                ("rand_sd", jf(rs)),
                ("band_lo", jf(lo)),
                ("band_hi", jf(hi)),
                ("in_band", jb(in_band)),
                ("z_of_mean_difference_advisory", jf(z)),
                ("walks", ji(wv.len() as u32)),
            ]));
        }
        land_json.push(jo(vec![("n", ji(n)), ("lags", J::A(lag_json))]));
    }

    // decisions (conventions documented in the header; PROTOCOL.md gives no numeric tolerances)
    let uniform_slope = fit_all.get("uniform").and_then(|f| f.slope);
    let p3_ok = uniform_slope.map(|e| (e - 2.0).abs() <= 0.25);
    let mut p1_json = vec![];
    let mut p1_all: Option<bool> = Some(true);
    let mut lead_alive = false;
    let mut lead_evaluable = true;
    for s in &solvers {
        let e = fit_all[*s].slope;
        let ok = e.map(|e| e >= 1.8);
        if ok == Some(false) {
            p1_all = Some(false);
        } else if ok.is_none() && p1_all == Some(true) {
            p1_all = None;
        }
        p1_json.push(jo(vec![("solver", js(s)), ("slope_all", jopt(e)), ("e_ge_1.8", ok.map(jb).unwrap_or(J::Null))]));
        match fit_20[*s].ci {
            Some((_, hi)) => {
                if hi < 1.0 {
                    lead_alive = true;
                }
            }
            None => lead_evaluable = false,
        }
    }
    let p2_lead = lag1_fail_sizes >= 3;
    let instrument_failure = p3_ok == Some(false) || verification_failures > 0 || !instrument_errors.is_empty();
    let verdict = if instrument_failure {
        "instrument_failure"
    } else if lead_alive || p2_lead {
        "lead_alive"
    } else if lead_evaluable {
        "closed"
    } else {
        "undetermined_insufficient_uncensored_n_ge_20"
    };

    // baselines and medians per n
    let mut base_json = vec![];
    for ci in &curves {
        let bb = box_b(ci.curve.p);
        let mut meds = vec![];
        for s in &all_solvers {
            let v = &data[&(s.clone(), ci.n)];
            let cens = v.iter().filter(|x| x.1).count();
            meds.push(jo(vec![
                ("solver", js(s)),
                ("median_evals", jopt(median_valid(v))),
                ("censored", ji(cens as u32)),
            ]));
        }
        base_json.push(jo(vec![
            ("n", ji(ci.n)),
            ("B", ji(bb)),
            ("naive_B2", jf((bb as f64) * (bb as f64))),
            ("root_B", jf(bb as f64)),
            ("cap_50B2", jf(50.0 * (bb as f64) * (bb as f64))),
            ("medians", J::A(meds)),
        ]));
    }

    // ---- JSON
    let curves_json: Vec<J> = curves
        .iter()
        .map(|ci| {
            jo(vec![
                ("n", ji(ci.n)),
                ("p", ji(ci.curve.p)),
                ("a", ji(ci.curve.a)),
                ("b", ji(ci.curve.b)),
                ("order", ji(ci.order)),
                ("G", J::A(vec![ji(ci.gx), ji(ci.gy)])),
                ("B", ji(box_b(ci.curve.p))),
                ("b_candidates_tried", ji(ci.b_tried)),
                ("p_mod_4", ji(ci.curve.p % 4)),
                ("matches_committed_rung", match known_rung(ci) { Some(b) => jb(b), None => J::Null }),
            ])
        })
        .collect();
    let inst_json: Vec<J> = re_jobs
        .iter()
        .enumerate()
        .map(|(k, &(c, i))| {
            let inst = &insts[c][i];
            let re = &re_res[k];
            jo(vec![
                ("n", ji(inst.n)),
                ("instance", ji(i as u32)),
                ("x1", ji(inst.x1)),
                ("x2", ji(inst.x2)),
                ("xR", ji(inst.xr)),
                ("yR", ji(inst.yr)),
                ("box_solutions_ordered", ji(re.solutions.len() as u32)),
                ("root_enum_first", match re.first { Some((a, b)) => J::A(vec![ji(a), ji(b)]), None => J::Null }),
                ("root_enum_evals", ji(re.first_evals)),
            ])
        })
        .collect();
    let mut rows_json: Vec<J> = rows
        .iter()
        .map(|r| {
            jo(vec![
                ("n", ji(r.n)),
                ("instance", ji(r.i as u32)),
                ("solver", js(&r.solver)),
                ("evals", ji(r.evals)),
                ("censored", jb(r.censored)),
                ("solved", jb(r.solved)),
                ("found", match r.found { Some((a, b)) => J::A(vec![ji(a), ji(b)]), None => J::Null }),
                ("is_planted_state", jb(r.planted_state)),
                ("spurious_zero_cost_states", ji(r.spurious)),
            ])
        })
        .collect();
    for (k, &(c, i)) in re_jobs.iter().enumerate() {
        let re = &re_res[k];
        rows_json.push(jo(vec![
            ("n", ji(curves[c].n)),
            ("instance", ji(i as u32)),
            ("solver", js("root_enum")),
            ("evals", ji(re.first_evals)),
            ("censored", jb(false)),
            ("solved", jb(re.first.is_some())),
            ("found", match re.first { Some((a, b)) => J::A(vec![ji(a), ji(b)]), None => J::Null }),
            ("is_planted_state", jb(re.first.map(|f| f == (insts[c][i].x1, insts[c][i].x2) || f == (insts[c][i].x2, insts[c][i].x1)).unwrap_or(false))),
            ("spurious_zero_cost_states", ji(0u32)),
        ]));
    }
    let header = jo(vec![
        ("study", js("semaev_qubo_annealing_20261110 (F6)")),
        ("protocol_sha256", js(&protocol_sha)),
        ("tool_source_sha256", js(&source_sha)),
        ("quick", jb(args.quick)),
        ("ns", J::A(args.ns.iter().map(|&n| ji(n)).collect())),
        ("instances_per_n", ji(args.instances as u32)),
        ("solvers", J::A(solvers.iter().map(|s| js(s)).collect())),
        ("baseline_solver", js("root_enum")),
        ("eval_cap", js("50*B^2 cost evaluations (censored at the cap)")),
        ("bootstrap_reps", ji(args.reps as u32)),
        ("threads", ji(threads as u32)),
        ("seed_scheme", js("FNV-1a-seeded SplitMix64: F6/inst/<n>/<i>, F6/solver/<name>/<n>/<i>, F6/walk/<n>/<i>, F6/rand/<n>/<i>, F6/boot/<solver>/<set>")),
        ("conventions", js("fit uses n whose median is exact (censored < half); P3 holds if |e_uniform - 2| <= 0.25; P2 in-band = walk mean within random mean +- 3 sd over walks; root_enum evals = x1 values tried (one quadratic solve each); solution = c=0 AND lifted P1 +- P2 = +-R")),
    ]);
    let summary = jo(vec![
        ("verdict", js(verdict)),
        ("instrument_failure", jb(instrument_failure)),
        ("verification_failures", ji(verification_failures as u32)),
        ("instrument_errors", J::A(instrument_errors.iter().map(|s| js(s)).collect())),
        ("P1_e_ge_1.8_per_solver", J::A(p1_json)),
        ("P1_all", match p1_all { Some(b) => jb(b), None => J::Null }),
        ("P2_lag1_fail_sizes", ji(lag1_fail_sizes as u32)),
        ("P2_all_lags_in_band", jb(p2_all_in_band)),
        ("P3_uniform_slope", jopt(uniform_slope)),
        ("P3_ok", match p3_ok { Some(b) => jb(b), None => J::Null }),
        ("lead_alive_solver_ci_hi_lt_1_on_n_ge_20", jb(lead_alive)),
        ("lead_rule_evaluable", jb(lead_evaluable)),
        ("baselines_per_n", J::A(base_json)),
    ]);
    let doc = jo(vec![
        ("header", header),
        ("curves", J::A(curves_json)),
        ("instances", J::A(inst_json)),
        ("rows", J::A(rows_json)),
        ("fits", J::A(fits_json)),
        ("landscape", J::A(land_json)),
        ("summary", summary),
    ]);
    let mut text = String::new();
    doc.render(&mut text, 0);
    text.push('\n');
    if let Some(parent) = std::path::Path::new(&args.out).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if let Err(e) = std::fs::write(&args.out, text) {
        eprintln!("cannot write {}: {}", args.out, e);
        return 3;
    }

    // ---- stdout table
    println!("F6 semaev_qubo_annealing  protocol_sha256 {}", protocol_sha);
    println!("tool_source_sha256 {}", source_sha);
    for ci in &curves {
        println!(
            "n={:<2} p={} a=-3 b={} order={} G=({},{}) B={} rung_match={:?}",
            ci.n, ci.curve.p, ci.curve.b, ci.order, ci.gx, ci.gy, box_b(ci.curve.p), known_rung(ci)
        );
    }
    println!("\nmedian cost evaluations to first c=0 (censored count in brackets; cap 50 B^2)");
    print!("{:>3} {:>7} {:>11} {:>9}", "n", "B", "B^2", "boxsol");
    for s in &all_solvers {
        print!(" {:>13}", s);
    }
    println!();
    for (ci_idx, ci) in curves.iter().enumerate() {
        let bb = box_b(ci.curve.p);
        let avg_sol: f64 = {
            let v: Vec<f64> = re_jobs
                .iter()
                .enumerate()
                .filter(|(_, &(c, _))| c == ci_idx)
                .map(|(k, _)| re_res[k].solutions.len() as f64)
                .collect();
            mean(&v)
        };
        print!("{:>3} {:>7} {:>11} {:>9.2}", ci.n, bb, bb * bb, avg_sol);
        for s in &all_solvers {
            let v = &data[&(s.clone(), ci.n)];
            let cens = v.iter().filter(|x| x.1).count();
            let m = median_valid(v);
            let cell = match m {
                Some(m) => format!("{:.0}[{}]", m, cens),
                None => format!(">cap[{}]", cens),
            };
            print!(" {:>13}", cell);
        }
        println!();
    }
    println!("\nexponent e: median evals ~ B^e (ln-ln OLS), bootstrap 95% CI over instances");
    for s in &all_solvers {
        for (label, f) in [("all", &fit_all[s]), ("n>=20", &fit_20[s])] {
            let sl = f.slope.map(|x| format!("{:.3}", x)).unwrap_or("n/a".into());
            let ci = f.ci.map(|c| format!("[{:.3}, {:.3}]", c.0, c.1)).unwrap_or("n/a".into());
            println!("  {:<10} {:<6} e = {:>7}  CI {}", s, label, sl, ci);
        }
    }
    println!("\nlandscape lag-1 autocorrelation (bit-flip walks vs random landscape; band = random mean +- 3 sd)");
    for (n, wm, rm, rs, inb) in &lag1_rows {
        println!("  n={:<2} walk {:+.4}  rand {:+.4} (sd {:.4})  in_band={}", n, wm, rm, rs, inb);
    }
    println!(
        "\nverification_failures={} instrument_errors={} P3_uniform_slope={} verdict={}",
        verification_failures,
        instrument_errors.len(),
        uniform_slope.map(|x| format!("{:.3}", x)).unwrap_or("n/a".into()),
        verdict
    );
    println!("wrote {}", args.out);
    if instrument_failure {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn curve(n: u32) -> CurveInfo {
        find_a3_curve(n).unwrap()
    }

    #[test]
    fn sha256_vector() {
        assert_eq!(sha256(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn tonelli_shanks() {
        let mut rng = Rng::from_seed_str("ts");
        for p in [13u64, 17, 41, 65537, 1_000_003, 998_244_353, 268_435_399] {
            for _ in 0..200 {
                let x = rng.below(p);
                let sq = mulm(x, x, p);
                let r = sqrt_mod(sq, p).unwrap();
                assert_eq!(mulm(r, r, p), sq);
            }
            // non-residue detection
            let mut nr = 2;
            while powmod(nr, (p - 1) / 2, p) == 1 {
                nr += 1;
            }
            assert!(sqrt_mod(nr, p).is_none());
        }
    }

    #[test]
    fn gray_roundtrip_and_neighbors() {
        for i in 0..5000u64 {
            assert_eq!(gray_dec(i ^ (i >> 1)), i);
        }
        let sp = Space::new(100);
        assert_eq!(sp.k, 7);
        let mut rng = Rng::from_seed_str("gray");
        for _ in 0..2000 {
            let st = sp.random_state(&mut rng);
            let nb = sp.random_neighbor(st, &mut rng);
            assert!(nb.0 < 100 && nb.1 < 100 && nb != st);
            let nb = sp.random_bitflip(st, &mut rng);
            assert!(nb.0 < 100 && nb.1 < 100 && nb != st);
        }
    }

    #[test]
    fn curve_order_matches_exhaustive_count() {
        for n in [12u32, 16] {
            let ci = curve(n);
            let c = &ci.curve;
            let mut cnt = 1u64; // infinity
            for x in 0..c.p {
                let r = c.rhs(x);
                cnt += if r == 0 { 1 } else if powmod(r, (c.p - 1) / 2, c.p) == 1 { 2 } else { 0 };
            }
            assert_eq!(cnt, ci.order, "n={}", n);
            assert!(is_prime(ci.order));
            assert_eq!(c.p % 4, 3);
            assert!(c.on_curve(Some((ci.gx, ci.gy))));
        }
    }

    #[test]
    fn generator_reproduces_committed_rungs() {
        for n in [16u32, 20, 24, 28] {
            let ci = curve(n);
            assert_eq!(known_rung(&ci), Some(true), "n={} p={} b={} order={}", n, ci.curve.p, ci.curve.b, ci.order);
            assert_eq!(ci.curve.a, ci.curve.p - 3);
        }
        let c28 = curve(28);
        assert_eq!((c28.gx, c28.order), (1, 268_407_199));
    }

    #[test]
    fn s3_vanishes_on_sums_both_signs() {
        let ci = curve(16);
        let c = &ci.curve;
        let mut rng = Rng::from_seed_str("s3test");
        let mut checked = 0;
        let mut nonzero_random = 0;
        while checked < 300 {
            let x1 = 1 + rng.below(c.p - 1);
            let x2 = if checked % 10 == 0 { x1 } else { 1 + rng.below(c.p - 1) };
            let (Some(y1), Some(y2)) = (c.lift_x(x1), c.lift_x(x2)) else { continue };
            let p1 = Some((x1, y1));
            let p2 = Some((x2, y2));
            for sum in [c.add(p1, p2), c.add(p1, c.neg(p2))] {
                if let Some((x3, _)) = sum {
                    assert_eq!(s3(c.p, c.a, c.b, x1, x2, x3), 0, "x1={} x2={} x3={}", x1, x2, x3);
                    // fast oracle agrees with the reference formula
                    let cost = Cost::new(c.p, c.a, c.b, x3);
                    assert_eq!(cost.c(x1, x2), 0);
                    // symmetry
                    assert_eq!(s3(c.p, c.a, c.b, x1, x3, x2), 0);
                    let r = 1 + rng.below(c.p - 1);
                    let v = s3(c.p, c.a, c.b, x1, x2, r);
                    assert_eq!(Cost::new(c.p, c.a, c.b, r).c(x1, x2), v.min(c.p - v));
                    if v != 0 {
                        nonzero_random += 1;
                    }
                }
            }
            checked += 1;
        }
        assert!(nonzero_random > 400);
    }

    #[test]
    fn planted_instance_zero_cost_and_root_enum() {
        for n in [12u32, 16] {
            let ci = curve(n);
            let bb = box_b(ci.curve.p);
            for i in 0..8 {
                let inst = gen_instance(&ci, n, i);
                assert!(inst.x1 >= 1 && inst.x1 <= bb && inst.x2 >= 1 && inst.x2 <= bb);
                let cost = Cost::new(ci.curve.p, ci.curve.a, ci.curve.b, inst.xr);
                assert_eq!(cost.c(inst.x1, inst.x2), 0);
                assert!(verify(&ci.curve, inst.x1, inst.x2, inst.xr, inst.yr));
                let re = root_enum(&ci.curve, bb, &inst);
                assert_eq!(re.inconsistent, 0);
                assert!(re.solutions.contains(&(inst.x1, inst.x2)), "n={} i={}", n, i);
                assert!(re.solutions.contains(&(inst.x2, inst.x1)));
                assert!(re.first.is_some() && re.first_evals <= inst.x1.min(inst.x2));
                // brute force the whole box at n=12: same solution set
                if n == 12 {
                    let mut brute = vec![];
                    for a in 1..=bb {
                        for b in 1..=bb {
                            if cost.c(a, b) == 0 && verify(&ci.curve, a, b, inst.xr, inst.yr) {
                                brute.push((a, b));
                            }
                        }
                    }
                    let mut s = re.solutions.clone();
                    s.sort();
                    brute.sort();
                    assert_eq!(s, brute);
                }
            }
        }
    }

    #[test]
    fn all_solvers_find_verified_solutions_at_n12() {
        let ci = curve(12);
        let inst = gen_instance(&ci, 12, 0);
        for s in SOLVERS {
            let row = run_row(&ci, &inst, s);
            assert!(row.solved && !row.censored, "{}", s);
            let (x1, x2) = row.found.unwrap();
            assert!(verify(&ci.curve, x1, x2, inst.xr, inst.yr));
        }
    }
}
