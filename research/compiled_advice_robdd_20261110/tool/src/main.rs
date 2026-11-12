//! F5: is the decomposable set compressible? ROBDD size of X2 / X_E against random sets.
//! Standalone, std-only tool implementing PROTOCOL.md (frozen 2026-11-10).
//! Deviation record: see `header.notes` in the JSON output.

#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Write as _;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const SOURCE: &str = include_str!("main.rs");

// ------------------------------------------------------------------ utils (ported)

fn modinv(a: u128, m: u128) -> Option<u128> {
    // extended Euclid in i64 (m < 2^62)
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (m as i64, (a % m) as i64);
    while nr != 0 {
        let q = r / nr;
        let x = t - q * nt;
        t = nt;
        nt = x;
        let y = r - q * nr;
        r = nr;
        nr = y;
    }
    if r != 1 {
        return None;
    }
    if t < 0 {
        t += m as i64;
    }
    Some(t as u128)
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = ((r as u128 * b as u128) % m as u128) as u64;
        }
        b = ((b as u128 * b as u128) % m as u128) as u64;
        e >>= 1;
    }
    r
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

fn seq_sha256(seq: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(seq.len() * 4);
    for x in seq {
        bytes.extend_from_slice(&x.to_le_bytes());
    }
    sha256(&bytes)
}

// ------------------------------------------------------------------ JSON

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

// ------------------------------------------------------------------ curve


fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn prime_below(limit: u64) -> u64 {
    let mut q = limit - 1;
    while !is_prime(q) {
        q -= 1;
    }
    q
}

fn jf(x: f64) -> J {
    J::F(x)
}
fn jsv(v: &[u64]) -> J {
    J::A(v.iter().map(|&x| J::Int(x as i128)).collect())
}

// ------------------------------------------------------------------ curve

fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(powmod(a, (p + 1) / 4, p));
    }
    // Tonelli-Shanks
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = ((tt as u128 * tt as u128) % p as u128) as u64;
            i += 1;
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = ((b as u128 * b as u128) % p as u128) as u64;
        }
        m = i;
        c = ((b as u128 * b as u128) % p as u128) as u64;
        t = ((t as u128 * c as u128) % p as u128) as u64;
        r = ((r as u128 * b as u128) % p as u128) as u64;
    }
    Some(r)
}

/// Bitset of nonzero squares modulo p.
fn square_bitset(p: u64) -> Vec<u64> {
    let mut bs = vec![0u64; (p as usize + 63) / 64];
    for y in 1..p {
        let s = (y * y % p) as usize;
        bs[s >> 6] |= 1u64 << (s & 63);
    }
    bs
}
fn is_sq(bs: &[u64], v: u64) -> bool {
    (bs[(v >> 6) as usize] >> (v & 63)) & 1 == 1
}

fn rhs(p: u64, a: u64, b: u64, x: u64) -> u64 {
    let x2 = x * x % p;
    let x3 = x2 * x % p;
    (x3 + a * x % p + b) % p
}

#[derive(Clone, Debug)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    tries: u64,
}

fn find_curve(n: u32, sq: &[u64], p: u64) -> Result<Curve, String> {
    let mut rng = Rng::from_seed_str(&format!("F5/curve/{}", n));
    let mut tries = 0u64;
    loop {
        tries += 1;
        if tries > 5000 {
            return Err("curve search exhausted".into());
        }
        let a = rng.next() % p;
        let b = rng.next() % p;
        let disc = (4 * (a as u128).pow(3) + 27 * (b as u128).pow(2)) % p as u128;
        if disc == 0 {
            continue;
        }
        let mut s: i64 = 0;
        for x in 0..p {
            let r = rhs(p, a, b, x);
            if r != 0 {
                s += if is_sq(sq, r) { 1 } else { -1 };
            }
        }
        let order = (p as i64 + 1 + s) as u64;
        if order != p && is_prime(order) {
            return Ok(Curve { p, a, b, order, tries });
        }
    }
}

type Pt = Option<(u64, u64)>;

fn add_pt(c: &Curve, p1: Pt, p2: Pt) -> Pt {
    let p = c.p;
    let (x1, y1) = p1?;
    let (x2, y2) = match p2 {
        Some(q) => q,
        None => return p1,
    };
    let lam = if x1 == x2 {
        if (y1 + y2) % p == 0 {
            return None;
        }
        let num = (3 * (x1 * x1 % p) + c.a) % p;
        let den = 2 * y1 % p;
        num * modinv(den as u128, p as u128).unwrap() as u64 % p
    } else {
        let num = (y2 + p - y1) % p;
        let den = (x2 + p - x1) % p;
        num * modinv(den as u128, p as u128).unwrap() as u64 % p
    };
    let x3 = (lam * lam % p + 2 * p - x1 - x2) % p;
    let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
    Some((x3, y3))
}
fn neg_pt(c: &Curve, q: Pt) -> Pt {
    q.map(|(x, y)| (x, (c.p - y) % c.p))
}

fn icbrt_ceil(p: u64) -> u64 {
    let mut b = 1u64;
    while b * b * b < p {
        b += 1;
    }
    b
}

fn small_base(c: &Curve) -> (u64, Vec<(u64, u64)>) {
    let b = icbrt_ceil(c.p);
    let mut f = vec![];
    for x in 1..=b {
        let r = rhs(c.p, c.a, c.b, x);
        if r == 0 {
            continue;
        }
        if let Some(y) = sqrt_mod(r, c.p) {
            f.push((x, y));
            f.push((x, c.p - y));
        }
    }
    (b, f)
}

/// X2 sorted-deduped, plus raw event count.
fn two_sums(c: &Curve, f: &[(u64, u64)]) -> (Vec<u32>, u64) {
    let mut xs: Vec<u32> = vec![];
    for i in 0..f.len() {
        for j in i..f.len() {
            let (a, b) = (Some(f[i]), Some(f[j]));
            if let Some((x, _)) = add_pt(c, a, b) {
                xs.push(x as u32);
            }
            if let Some((x, _)) = add_pt(c, a, neg_pt(c, b)) {
                xs.push(x as u32);
            }
        }
    }
    let raw = xs.len() as u64;
    xs.sort_unstable();
    xs.dedup();
    (xs, raw)
}

fn legendre_set(c: &Curve, sq: &[u64]) -> Vec<u32> {
    let mut v = Vec::with_capacity(c.p as usize / 2 + 1024);
    for x in 0..c.p {
        let r = rhs(c.p, c.a, c.b, x);
        if r != 0 && is_sq(sq, r) {
            v.push(x as u32);
        }
    }
    v
}

/// Knuth Algorithm S: sorted uniform k-subset of [0, p).
fn random_subset(p: u64, k: usize, seed: &str) -> Vec<u32> {
    let mut rng = Rng::from_seed_str(seed);
    let mut out = Vec::with_capacity(k);
    for i in 0..p {
        let need = (k - out.len()) as u64;
        if need == 0 {
            break;
        }
        let remain = p - i;
        let r = ((rng.next() as u128 * remain as u128) >> 64) as u64;
        if r < need {
            out.push(i as u32);
        }
    }
    out
}

// ------------------------------------------------------------------ ROBDD manager

#[derive(Default, Clone, Copy)]
struct IdHasher(u64);
impl Hasher for IdHasher {
    fn write(&mut self, b: &[u8]) {
        for &x in b {
            self.0 = (self.0.rotate_left(5) ^ x as u64).wrapping_mul(0x517cc1b727220a95);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517cc1b727220a95);
    }
    fn finish(&self) -> u64 {
        self.0 ^ (self.0 >> 32)
    }
}
type Tab = HashMap<u64, u32, BuildHasherDefault<IdHasher>>;

fn key(lo: u32, hi: u32) -> u64 {
    ((lo as u64) << 32) | hi as u64
}

#[derive(Default, Clone, Copy, Debug)]
struct Ops {
    mk_calls: u64,
    reductions: u64,
    hits: u64,
    creations: u64,
    swaps: u64,
}

struct Mgr {
    n: usize,
    lo: Vec<u32>,
    hi: Vec<u32>,
    lvl: Vec<u32>,
    rc: Vec<u32>,
    tabs: Vec<Tab>,
    free: Vec<u32>,
    live: usize,
    order: Vec<usize>, // level -> variable (bit index of x)
    ops: Ops,
}

impl Mgr {
    fn new(n: usize, order: Vec<usize>) -> Mgr {
        assert_eq!(order.len(), n);
        let mut m = Mgr {
            n,
            lo: vec![0, 1],
            hi: vec![0, 1],
            lvl: vec![n as u32, n as u32],
            rc: vec![u32::MAX / 2, u32::MAX / 2],
            tabs: (0..n).map(|_| Tab::default()).collect(),
            free: vec![],
            live: 0,
            order,
            ops: Ops::default(),
        };
        m.lo.reserve(1 << 16);
        m
    }

    fn inc(&mut self, id: u32) {
        if id >= 2 {
            self.rc[id as usize] += 1;
        }
    }
    fn deref(&mut self, id: u32) {
        if id < 2 {
            return;
        }
        let i = id as usize;
        self.rc[i] -= 1;
        if self.rc[i] == 0 {
            let (l, h, v) = (self.lo[i], self.hi[i], self.lvl[i] as usize);
            self.tabs[v].remove(&key(l, h));
            self.free.push(id);
            self.live -= 1;
            self.deref(l);
            self.deref(h);
        }
    }

    /// Consumes one reference to `lo` and one to `hi`; returns one owned reference.
    fn mk(&mut self, level: usize, lo: u32, hi: u32) -> u32 {
        self.ops.mk_calls += 1;
        if lo == hi {
            self.ops.reductions += 1;
            if lo >= 2 {
                self.rc[lo as usize] -= 1;
            }
            return lo;
        }
        let k = key(lo, hi);
        if let Some(&id) = self.tabs[level].get(&k) {
            self.ops.hits += 1;
            self.rc[id as usize] += 1;
            self.deref(lo);
            self.deref(hi);
            return id;
        }
        self.ops.creations += 1;
        let id = if let Some(id) = self.free.pop() {
            let i = id as usize;
            self.lo[i] = lo;
            self.hi[i] = hi;
            self.lvl[i] = level as u32;
            self.rc[i] = 1;
            id
        } else {
            self.lo.push(lo);
            self.hi.push(hi);
            self.lvl.push(level as u32);
            self.rc.push(1);
            (self.lo.len() - 1) as u32
        };
        self.tabs[level].insert(k, id);
        self.live += 1;
        id
    }

    /// Build from a sorted slice of permuted keys (variable at level l is bit n-1-l of the key).
    fn build(&mut self, elems: &[u32], level: usize) -> u32 {
        if elems.is_empty() {
            return 0;
        }
        if level == self.n {
            return 1;
        }
        let bit = self.n - 1 - level;
        let split = elems.partition_point(|&e| (e >> bit) & 1 == 0);
        let lo = self.build(&elems[..split], level + 1);
        let hi = self.build(&elems[split..], level + 1);
        self.mk(level, lo, hi)
    }

    fn eval(&self, root: u32, x: u32) -> bool {
        let mut id = root;
        while id >= 2 {
            let l = self.lvl[id as usize] as usize;
            id = if (x >> self.order[l]) & 1 == 1 { self.hi[id as usize] } else { self.lo[id as usize] };
        }
        id == 1
    }

    fn count_sat(&self, root: u32) -> u64 {
        let mut memo = vec![u64::MAX; self.lo.len()];
        fn rec(m: &Mgr, id: u32, memo: &mut Vec<u64>) -> u64 {
            if id < 2 {
                return id as u64;
            }
            if memo[id as usize] != u64::MAX {
                return memo[id as usize];
            }
            let l = m.lvl[id as usize] as usize;
            let mut tot = 0;
            for c in [m.lo[id as usize], m.hi[id as usize]] {
                let lc = m.lvl[c as usize] as usize;
                tot += rec(m, c, memo) << (lc - l - 1);
            }
            memo[id as usize] = tot;
            tot
        }
        let lr = self.lvl[root as usize] as usize;
        rec(self, root, &mut memo) << lr
    }

    fn reachable(&self, root: u32) -> usize {
        let mut seen = vec![false; self.lo.len()];
        let mut stack = vec![root];
        let mut c = 0;
        while let Some(id) = stack.pop() {
            if id < 2 || seen[id as usize] {
                continue;
            }
            seen[id as usize] = true;
            c += 1;
            stack.push(self.lo[id as usize]);
            stack.push(self.hi[id as usize]);
        }
        c
    }

    fn enumerate(&self, root: u32) -> Vec<u32> {
        fn en(m: &Mgr, id: u32, l: usize, acc: u32, out: &mut Vec<u32>) {
            if id == 0 {
                return;
            }
            if l == m.n {
                out.push(acc);
                return;
            }
            let bit = 1u32 << m.order[l];
            if id == 1 || m.lvl[id as usize] as usize > l {
                en(m, id, l + 1, acc, out);
                en(m, id, l + 1, acc | bit, out);
            } else {
                en(m, m.lo[id as usize], l + 1, acc, out);
                en(m, m.hi[id as usize], l + 1, acc | bit, out);
            }
        }
        let mut out = vec![];
        en(self, root, 0, 0, &mut out);
        out
    }

    /// Exchange the variables at levels k and k+1 in place (roots keep their ids).
    fn swap(&mut self, k: usize) {
        assert!(k + 1 < self.n);
        self.ops.swaps += 1;
        let old_k = std::mem::take(&mut self.tabs[k]);
        let old_k1 = std::mem::take(&mut self.tabs[k + 1]);
        let kk = (k + 1) as u32;
        let mut dep: Vec<(u32, [u32; 4])> = vec![];
        let mut indep: Vec<(u64, u32)> = vec![];
        for (&kv, &f) in old_k.iter() {
            let (a, b) = (self.lo[f as usize], self.hi[f as usize]);
            let da = self.lvl[a as usize] == kk;
            let db = self.lvl[b as usize] == kk;
            if !da && !db {
                indep.push((kv, f));
            } else {
                let (a0, a1) = if da { (self.lo[a as usize], self.hi[a as usize]) } else { (a, a) };
                let (b0, b1) = if db { (self.lo[b as usize], self.hi[b as usize]) } else { (b, b) };
                dep.push((f, [a0, a1, b0, b1]));
            }
        }
        for (_, &g) in old_k1.iter() {
            self.lvl[g as usize] = k as u32;
        }
        self.tabs[k] = old_k1;
        let mut t1 = Tab::default();
        t1.reserve(indep.len() + dep.len());
        for (kv, f) in indep {
            self.lvl[f as usize] = kk;
            t1.insert(kv, f);
        }
        self.tabs[k + 1] = t1;
        let mut newch = Vec::with_capacity(dep.len());
        for &(_, [f00, f01, f10, f11]) in &dep {
            self.inc(f00);
            self.inc(f10);
            let nlo = self.mk(k + 1, f00, f10);
            self.inc(f01);
            self.inc(f11);
            let nhi = self.mk(k + 1, f01, f11);
            newch.push((nlo, nhi));
        }
        for (&(f, _), &(nlo, nhi)) in dep.iter().zip(newch.iter()) {
            let (oa, ob) = (self.lo[f as usize], self.hi[f as usize]);
            self.deref(oa);
            self.deref(ob);
            self.lo[f as usize] = nlo;
            self.hi[f as usize] = nhi;
            self.lvl[f as usize] = k as u32;
            self.tabs[k].insert(key(nlo, nhi), f);
        }
        self.order.swap(k, k + 1);
    }

    /// One pass of Rudell sifting with a swap budget. Returns (start size, end size).
    fn sift(&mut self, budget: u64) -> (usize, usize) {
        let n = self.n;
        let start = self.live;
        if n < 2 {
            return (start, start);
        }
        let mut vars: Vec<(usize, usize)> =
            (0..n).map(|l| (self.tabs[l].len(), self.order[l])).collect();
        vars.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let base = self.ops.swaps;
        for &(_, v) in &vars {
            if self.ops.swaps - base >= budget {
                break;
            }
            let mut pos = self.order.iter().position(|&x| x == v).unwrap();
            let mut best = self.live;
            let mut bpos = pos;
            let ends = if pos * 2 >= n - 1 { [n - 1, 0] } else { [0, n - 1] };
            'outer: for &end in &ends {
                while pos != end {
                    if self.ops.swaps - base >= budget {
                        break 'outer;
                    }
                    if pos < end {
                        self.swap(pos);
                        pos += 1;
                    } else {
                        self.swap(pos - 1);
                        pos -= 1;
                    }
                    if self.live < best {
                        best = self.live;
                        bpos = pos;
                    }
                    if self.live as f64 > 1.2 * best as f64 {
                        break;
                    }
                }
            }
            while pos != bpos {
                if pos < bpos {
                    self.swap(pos);
                    pos += 1;
                } else {
                    self.swap(pos - 1);
                    pos -= 1;
                }
            }
        }
        (start, self.live)
    }
}

fn permute_key(x: u32, order: &[usize]) -> u32 {
    let n = order.len();
    let mut y = 0u32;
    for l in 0..n {
        if (x >> order[l]) & 1 == 1 {
            y |= 1 << (n - 1 - l);
        }
    }
    y
}

fn build_mgr(n: usize, elems: &[u32], order: &[usize]) -> (Mgr, u32) {
    let mut ys: Vec<u32> = elems.iter().map(|&x| permute_key(x, order)).collect();
    ys.sort_unstable();
    let mut m = Mgr::new(n, order.to_vec());
    let root = m.build(&ys, 0);
    (m, root)
}

const ENUM_LIMIT: usize = 1 << 20;

fn verify(m: &Mgr, root: u32, elems: &[u32]) -> Result<(), String> {
    let cnt = m.count_sat(root);
    if cnt != elems.len() as u64 {
        return Err(format!("count mismatch: dag {} vs set {}", cnt, elems.len()));
    }
    for &x in elems {
        if !m.eval(root, x) {
            return Err(format!("member {} not accepted", x));
        }
    }
    let reach = m.reachable(root);
    if reach != m.live {
        return Err(format!("live {} != reachable {}", m.live, reach));
    }
    if elems.len() <= ENUM_LIMIT {
        let mut e = m.enumerate(root);
        e.sort_unstable();
        if e != elems {
            return Err("enumeration differs from input set".into());
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct OrderRes {
    name: &'static str,
    nodes: u64,
    start_nodes: u64,
    ops: Ops,
    order: Vec<usize>,
    err: Option<String>,
}

/// Builds msb and lsb ROBDDs and sifts each; every ROBDD is verified.
fn analyze(n: usize, elems: &[u32], budget: u64) -> Vec<OrderRes> {
    let msb: Vec<usize> = (0..n).map(|l| n - 1 - l).collect();
    let lsb: Vec<usize> = (0..n).collect();
    let mut out = vec![];
    for (fixed, sifted, order) in [("msb", "sift_msb", msb), ("lsb", "sift_lsb", lsb)] {
        let (mut m, root) = build_mgr(n, elems, &order);
        let err = verify(&m, root, elems).err();
        out.push(OrderRes {
            name: fixed,
            nodes: m.live as u64,
            start_nodes: m.live as u64,
            ops: m.ops,
            order: m.order.clone(),
            err,
        });
        let before = m.ops;
        let (s0, s1) = m.sift(budget);
        let err = verify(&m, root, elems).err();
        let mut so = m.ops;
        so.mk_calls -= before.mk_calls;
        so.reductions -= before.reductions;
        so.hits -= before.hits;
        so.creations -= before.creations;
        so.swaps -= before.swaps;
        out.push(OrderRes {
            name: sifted,
            nodes: s1 as u64,
            start_nodes: s0 as u64,
            ops: so,
            order: m.order.clone(),
            err,
        });
    }
    out
}

// ------------------------------------------------------------------ stats and JSON

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let sd = if v.len() > 1 { (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0)).sqrt() } else { 0.0 };
    (m, sd)
}

fn ops_json(o: &Ops) -> J {
    jo(vec![
        ("unique_table_ops", ji(o.mk_calls as i64)),
        ("hits", ji(o.hits as i64)),
        ("creations", ji(o.creations as i64)),
        ("reductions", ji(o.reductions as i64)),
        ("swaps", ji(o.swaps as i64)),
    ])
}

fn order_json(r: &OrderRes) -> J {
    jo(vec![
        ("nodes", ji(r.nodes as i64)),
        ("start_nodes", ji(r.start_nodes as i64)),
        ("ops", ops_json(&r.ops)),
        ("final_order_level_to_bit", J::A(r.order.iter().map(|&x| ji(x as i64)).collect())),
        ("verified", J::Bool(r.err.is_none())),
    ])
}

fn peak_rss_kb() -> i64 {
    if let Ok(s) = std::fs::read_to_string("/proc/self/status") {
        for l in s.lines() {
            if let Some(r) = l.strip_prefix("VmHWM:") {
                return r.trim().trim_end_matches("kB").trim().parse().unwrap_or(-1);
            }
        }
    }
    -1
}

const ORDERS: [&str; 4] = ["msb", "lsb", "sift_msb", "sift_lsb"];

struct SetRow {
    n: usize,
    set: &'static str,
    size: usize,
    real: Vec<OrderRes>,
    rand: Vec<Vec<OrderRes>>,
}

fn par_jobs<R: Send>(count: usize, threads: usize, f: impl Fn(usize) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let res: Mutex<Vec<Option<R>>> = Mutex::new((0..count).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1).min(count.max(1)) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= count {
                    break;
                }
                let r = f(i);
                res.lock().unwrap()[i] = Some(r);
            });
        }
    });
    res.into_inner().unwrap().into_iter().map(|x| x.unwrap()).collect()
}

fn row_json(r: &SetRow) -> J {
    let mut orders = vec![];
    let mut ratios = vec![];
    let mut p1 = vec![];
    for (oi, name) in ORDERS.iter().enumerate() {
        orders.push((name.to_string(), order_json(&r.real[oi])));
        if !r.rand.is_empty() {
            let counts: Vec<f64> = r.rand.iter().map(|x| x[oi].nodes as f64).collect();
            let (m, sd) = mean_sd(&counts);
            let ratio = r.real[oi].nodes as f64 / m;
            let z = if sd > 0.0 { (r.real[oi].nodes as f64 - m) / sd } else { f64::NAN };
            let mn = counts.iter().cloned().fold(f64::INFINITY, f64::min);
            let mx = counts.iter().cloned().fold(0.0, f64::max);
            let rops: Vec<f64> = r.rand.iter().map(|x| x[oi].ops.mk_calls as f64).collect();
            ratios.push((
                name.to_string(),
                jo(vec![
                    ("rand_nodes", J::A(counts.iter().map(|&c| ji(c as i64)).collect())),
                    ("rand_mean", jf(m)),
                    ("rand_sd", jf(sd)),
                    ("rand_min", jf(mn)),
                    ("rand_max", jf(mx)),
                    ("rand_mean_unique_table_ops", jf(mean_sd(&rops).0)),
                    ("ratio_x_over_mean_rand", jf(ratio)),
                    ("ratio_sd_from_rand_spread", jf(ratio * sd / m)),
                    ("z_score", jf(z)),
                ]),
            ));
            p1.push((name.to_string(), J::Bool((0.9..=1.1).contains(&ratio))));
        }
    }
    let mut v = vec![
        ("n", ji(r.n as i64)),
        ("set", js(r.set)),
        ("size", ji(r.size as i64)),
        ("orders", J::O(orders)),
    ];
    if !r.rand.is_empty() {
        v.push(("rand_count", ji(r.rand.len() as i64)));
        v.push(("vs_rand", J::O(ratios)));
        v.push(("p1_ratio_in_0.9_1.1", J::O(p1)));
    } else if r.set == "INT" {
        let p2: Vec<(String, J)> = ORDERS
            .iter()
            .enumerate()
            .map(|(i, nm)| (nm.to_string(), J::Bool(r.real[i].nodes <= 2 * r.n as u64)))
            .collect();
        v.push(("p2_nodes_le_2n", J::O(p2)));
    }
    jo(v)
}

// ------------------------------------------------------------------ CLI

struct Args {
    out: String,
    ns: Vec<u32>,
    rand: usize,
    quick: bool,
    threads: usize,
    protocol: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        out: "results/run.json".into(),
        ns: vec![16, 18, 20, 22, 24],
        rand: 10,
        quick: false,
        threads: 1,
        protocol: None,
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let (mut n_given, mut r_given) = (false, false);
    while i < argv.len() {
        let val = |i: &mut usize| -> Result<String, String> {
            *i += 1;
            argv.get(*i).cloned().ok_or(format!("{} needs a value", argv[*i - 1]))
        };
        match argv[i].as_str() {
            "--out" => a.out = val(&mut i)?,
            "--n" => {
                a.ns = val(&mut i)?.split(',').map(|s| s.parse::<u32>().map_err(|_| "bad --n".to_string())).collect::<Result<_, _>>()?;
                n_given = true;
            }
            "--rand" => {
                a.rand = val(&mut i)?.parse().map_err(|_| "bad --rand")?;
                r_given = true;
            }
            "--threads" => a.threads = val(&mut i)?.parse().map_err(|_| "bad --threads")?,
            "--protocol" => a.protocol = Some(val(&mut i)?),
            "--quick" => a.quick = true,
            x => return Err(format!("unknown argument {}", x)),
        }
        i += 1;
    }
    if a.quick {
        if !n_given {
            a.ns = vec![16];
        }
        if !r_given {
            a.rand = 3;
        }
    }
    for &n in &a.ns {
        if !(8..=28).contains(&n) {
            return Err(format!("n={} out of range 8..=28", n));
        }
    }
    if a.rand == 0 {
        return Err("--rand must be >= 1".into());
    }
    Ok(a)
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
    let mut fatal: Vec<String> = vec![];
    let proto_path = {
        let mut c: Vec<String> = vec![];
        if let Some(p) = &args.protocol {
            c.push(p.clone());
        }
        c.push(format!("{}/../PROTOCOL.md", env!("CARGO_MANIFEST_DIR")));
        c.push("../PROTOCOL.md".into());
        c.into_iter().find(|p| std::path::Path::new(p).exists())
    };
    let protocol_sha = match &proto_path {
        Some(p) => sha256(&std::fs::read(p).unwrap()),
        None => {
            fatal.push("PROTOCOL.md not found".into());
            "missing".into()
        }
    };
    let source_sha = sha256(SOURCE.as_bytes());
    println!("protocol sha256 {}\nsource   sha256 {}", protocol_sha, source_sha);

    let mut rows: Vec<SetRow> = vec![];
    let mut curves: Vec<J> = vec![];
    for &n32 in &args.ns {
        let n = n32 as usize;
        let p = prime_below(1u64 << n32);
        let sq = square_bitset(p);
        let c = match find_curve(n32, &sq, p) {
            Ok(c) => c,
            Err(e) => {
                fatal.push(format!("n={}: {}", n, e));
                continue;
            }
        };
        let (b, f) = small_base(&c);
        let (x2, raw) = two_sums(&c, &f);
        let xe = legendre_set(&c, &sq);
        drop(sq);
        let int: Vec<u32> = (0..x2.len() as u32).collect();
        let budget = 2 * (n as u64) * (n as u64);
        println!("n={} p={} a={} b={} N={} B={} |F|={} |X2|={} raw={} coll={} |XE|={}", n, p, c.a, c.b, c.order, b, f.len(), x2.len(), raw, raw - x2.len() as u64, xe.len());
        curves.push(jo(vec![
            ("n", ji(n as i64)),
            ("p", ji(p as i64)),
            ("a", ji(c.a as i64)),
            ("b", ji(c.b as i64)),
            ("group_order", ji(c.order as i64)),
            ("curve_candidates_tried", ji(c.tries as i64)),
            ("B", ji(b as i64)),
            ("F_size", ji(f.len() as i64)),
            ("x2_raw_events", ji(raw as i64)),
            ("x2_size", ji(x2.len() as i64)),
            ("x2_collisions", ji((raw - x2.len() as u64) as i64)),
            ("xe_size", ji(xe.len() as i64)),
            ("sift_swap_budget", ji(budget as i64)),
        ]));
        for (set, elems) in [("X2", &x2), ("XE", &xe), ("INT", &int)] {
            let real = analyze(n, elems, budget);
            let rand = if set == "INT" {
                vec![]
            } else {
                par_jobs(args.rand, args.threads, |j| {
                    let r = random_subset(p, elems.len(), &format!("F5/rand/{}/{}/{}", n, set, j));
                    analyze(n, &r, budget)
                })
            };
            let row = SetRow { n, set, size: elems.len(), real, rand };
            for r in row.real.iter().chain(row.rand.iter().flatten()) {
                if let Some(e) = &r.err {
                    fatal.push(format!("n={} {} {}: verification failed: {}", n, set, r.name, e));
                }
            }
            if set == "INT" {
                for r in &row.real {
                    if r.nodes > 2 * n as u64 {
                        fatal.push(format!("n={} INT {}: {} nodes > 2n (P2 fails)", n, r.name, r.nodes));
                    }
                }
            }
            // stdout line
            let nodes: Vec<String> = row.real.iter().map(|r| r.nodes.to_string()).collect();
            let mut line = format!("n={:<2} {:<3} |X|={:<8} nodes[msb lsb sm sl]={}", n, set, row.size, nodes.join(" "));
            if !row.rand.is_empty() {
                let rs: Vec<String> = (0..4)
                    .map(|oi| {
                        let cs: Vec<f64> = row.rand.iter().map(|x| x[oi].nodes as f64).collect();
                        let (m, sd) = mean_sd(&cs);
                        format!("{:.3}({:.3})", row.real[oi].nodes as f64 / m, sd / m)
                    })
                    .collect();
                line += &format!("  ratio(relsd)={}", rs.join(" "));
            }
            println!("{}", line);
            rows.push(row);
        }
    }

    // summary
    let mut summary: Vec<(String, J)> = vec![];
    let int_ok = !rows.iter().any(|r| r.set == "INT" && r.real.iter().any(|x| x.nodes > 2 * r.n as u64));
    let verif_ok = fatal.is_empty();
    let mut alive_any = false;
    let mut per_set: Vec<(String, J)> = vec![];
    for set in ["X2", "XE"] {
        let mut per_order: Vec<(String, J)> = vec![];
        for (oi, name) in ORDERS.iter().enumerate() {
            let mut ratios: Vec<(usize, f64)> = vec![];
            for r in rows.iter().filter(|r| r.set == set && !r.rand.is_empty()) {
                let cs: Vec<f64> = r.rand.iter().map(|x| x[oi].nodes as f64).collect();
                ratios.push((r.n, r.real[oi].nodes as f64 / mean_sd(&cs).0));
            }
            let p1 = !ratios.is_empty() && ratios.iter().all(|&(_, x)| (0.9..=1.1).contains(&x));
            let le09 = !ratios.is_empty() && ratios.iter().all(|&(_, x)| x <= 0.9);
            let dec = ratios.len() >= 2 && ratios.windows(2).all(|w| w[1].1 < w[0].1);
            let alive = le09 && dec;
            alive_any |= alive;
            per_order.push((
                name.to_string(),
                jo(vec![
                    ("ratios_by_n", J::A(ratios.iter().map(|&(n, x)| jo(vec![("n", ji(n as i64)), ("ratio", jf(x))])).collect())),
                    ("p1_holds_all_n", J::Bool(p1)),
                    ("ratio_le_0.9_all_n", J::Bool(le09)),
                    ("strictly_decreasing_in_n", J::Bool(dec)),
                    ("lead_alive_rule2", J::Bool(alive)),
                ]),
            ));
        }
        per_set.push((set.to_string(), J::O(per_order)));
    }
    let decision = if !int_ok || !verif_ok {
        "instrument_failure"
    } else if alive_any {
        "lead_alive_candidate"
    } else {
        "closed"
    };
    summary.push(("per_set_order".into(), J::O(per_set)));
    summary.push(("p2_all_interval_le_2n".into(), J::Bool(int_ok)));
    summary.push(("all_verifications_ok".into(), J::Bool(verif_ok)));
    summary.push(("decision_rule_outcome".into(), js(decision)));
    summary.push(("peak_rss_kb".into(), ji(peak_rss_kb())));
    println!("decision: {}  peak_rss_kb={}", decision, peak_rss_kb());

    let notes = "Order columns: msb, lsb (fixed), sift_msb / sift_lsb = one budgeted Rudell sifting pass started from msb / lsb. \
Sifting budget = 2*n^2 adjacent swaps (stops starting new variables once exhausted; 1.2x max growth abort per direction). \
No random-order fallback used. Each ROBDD verified: DAG satisfying count == |X|, every input element accepted, live nodes == reachable nodes, \
and full enumeration == input for |X| <= 2^20. X2 raw events = non-infinity values of x(P1+P2), x(P1-P2) over i<=j in F (F is sign-symmetric, so many events coincide by symmetry); collisions = raw - |X2|.";
    let header = jo(vec![
        ("study", js("compiled_advice_robdd_20261110")),
        ("protocol_sha256", js(&protocol_sha)),
        ("source_sha256", js(&source_sha)),
        ("ns", J::A(args.ns.iter().map(|&n| ji(n as i64)).collect())),
        ("rand_per_set", ji(args.rand as i64)),
        ("quick", J::Bool(args.quick)),
        ("notes", js(notes)),
    ]);
    let doc = jo(vec![
        ("header", header),
        ("curves", J::A(curves)),
        ("rows", J::A(rows.iter().map(row_json).collect())),
        ("fatal", J::A(fatal.iter().map(|s| js(s)).collect())),
        ("summary", J::O(summary)),
    ]);
    let mut s = String::new();
    doc.render(&mut s, 0);
    s.push('\n');
    if let Some(parent) = std::path::Path::new(&args.out).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if let Err(e) = std::fs::write(&args.out, s) {
        eprintln!("cannot write {}: {}", args.out, e);
        return 1;
    }
    println!("wrote {}", args.out);
    if fatal.is_empty() {
        0
    } else {
        for f in &fatal {
            eprintln!("FATAL: {}", f);
        }
        2
    }
}

// ------------------------------------------------------------------ tests

#[cfg(test)]
mod tests {
    use super::*;

    fn orders(n: usize) -> Vec<Vec<usize>> {
        vec![(0..n).map(|l| n - 1 - l).collect(), (0..n).collect()]
    }

    #[test]
    fn interval_has_at_most_2n_nodes() {
        let n = 12;
        for m in [1u32, 2, 3, 5, 100, 1000, 2047, 2048, 3000, 4095] {
            let e: Vec<u32> = (0..m).collect();
            for o in orders(n) {
                let (mg, root) = build_mgr(n, &e, &o);
                assert!(mg.live <= 2 * n, "m={} nodes={}", m, mg.live);
                verify(&mg, root, &e).unwrap();
            }
        }
    }

    #[test]
    fn random_small_set_verifies_and_rebuild_invariant() {
        let n = 12;
        let p = 4093u64;
        for j in 0..5 {
            let e = random_subset(p, 700 + 100 * j, &format!("test/{}", j));
            for o in orders(n) {
                let (mg, root) = build_mgr(n, &e, &o);
                verify(&mg, root, &e).unwrap();
                let (mg2, r2) = build_mgr(n, &e, &o);
                verify(&mg2, r2, &e).unwrap();
                assert_eq!(mg.live, mg2.live);
            }
        }
    }

    #[test]
    fn sifting_is_correct_and_matches_rebuild() {
        let n = 12;
        for j in 0..6 {
            let e = random_subset(4093, 300 + 150 * j, &format!("sift/{}", j));
            for o in orders(n) {
                let (mut mg, root) = build_mgr(n, &e, &o);
                let before = mg.live;
                let (s0, s1) = mg.sift(10_000);
                assert_eq!(s0, before);
                assert!(s1 <= s0);
                verify(&mg, root, &e).unwrap();
                let order = mg.order.clone();
                let (mg2, r2) = build_mgr(n, &e, &order);
                verify(&mg2, r2, &e).unwrap();
                assert_eq!(mg2.live, mg.live, "swap node count != rebuild count under final order");
            }
        }
        // structured set: parity-like function benefits from sifting but must stay correct
        let e: Vec<u32> = (0..4096u32).filter(|x| ((x & 0x3f) ^ (x >> 6)).count_ones() % 2 == 1).collect();
        let (mut mg, root) = build_mgr(12, &e, &orders(12)[0]);
        mg.sift(10_000);
        verify(&mg, root, &e).unwrap();
    }

    #[test]
    fn x2_membership_spot_checks_n16() {
        let n = 16u32;
        let p = prime_below(1 << n);
        let sq = square_bitset(p);
        let c = find_curve(n, &sq, p).unwrap();
        assert!(is_prime(c.order) && c.order != p);
        let (_, f) = small_base(&c);
        let (x2, raw) = two_sums(&c, &f);
        assert!(raw >= x2.len() as u64);
        let inx2 = |x: u32| x2.binary_search(&x).is_ok();
        // independent check: x in X2 iff exists Q with x(Q)=x and P in F with Q-P in F
        let inf = |q: Pt| -> bool {
            match q {
                Some((x, _)) => f.iter().any(|&(fx, _)| fx == x),
                None => false,
            }
        };
        let brute = |x: u64| -> bool {
            let r = rhs(p, c.a, c.b, x);
            if r == 0 {
                return false;
            }
            let y = match sqrt_mod(r, p) {
                Some(y) => y,
                None => return false,
            };
            let q = Some((x, y));
            f.iter().any(|&pp| inf(add_pt(&c, q, neg_pt(&c, Some(pp)))))
        };
        for &x in x2.iter().step_by(7) {
            assert!(brute(x as u64), "x={} in X2 but brute says no", x);
        }
        let mut rng = Rng::from_seed_str("test/x2");
        for _ in 0..3000 {
            let x = rng.next() % p;
            assert_eq!(inx2(x as u32), brute(x), "x={}", x);
        }
    }
}
