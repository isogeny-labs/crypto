//! F4: SAT (CaDiCaL) on the bit-level x-only Montgomery ladder: conflict-count scaling and
//! clause reuse.  Standalone tool implementing PROTOCOL.md (frozen 2026-11-10).
//!
//! Encoding summary (see also the JSON header "encoding" field):
//!  * k = nk = ceil(log2 r) free Boolean variables, constraint k < r (carry-out of k + (2^nk - r)).
//!  * Ladder over ALL nk bits MSB->LSB starting from (R0,R1) = (O,G) with O = (1:0).  The
//!    differential-add and doubling formulas are valid at O (checked algebraically and by the unit
//!    tests: leading zero bits keep (O,G)), so every k in [0, r) is representable with no
//!    leading-bit constraint.  Conditional swaps are mux gates on b_i xor b_{i-1}.
//!  * Field ops on n-bit words.  Multiplication: schoolbook AND/ripple-adder product (2n bits),
//!    reduction by free quotient bits q (n bits): prod + q*c = q*2^n + rem with c = 2^n - p
//!    (algebraically prod = q*p + rem), rem < p via carry-out of rem + c.  Add/sub: one
//!    conditional correction (mux).
//!  * Final: X = x_Q * Z (mod p) and Z != 0.  x_Q bits are free variables (assumptions in REUSE,
//!    unit clauses in FRESH), so both arms load the identical CNF.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use cadical_sys::{CaDiCal, Status};

// ------------------------------------------------------------------ utils

fn modinv(a: u64, m: u64) -> Option<u64> {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (m as i128, (a % m) as i128);
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
        t += m as i128;
    }
    Some(t as u64)
}

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
    let mut x = limit - 1;
    while !is_prime(x) {
        x -= 1;
    }
    x
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

fn jf(x: f64) -> J {
    if x.is_finite() {
        J::F(x)
    } else {
        J::Null
    }
}
fn jopt(x: Option<f64>) -> J {
    match x {
        Some(v) => jf(v),
        None => J::Null,
    }
}
fn ju(x: u64) -> J {
    J::Int(x as i128)
}

// ------------------------------------------------------------------ curve

type Pt = Option<(u64, u64)>;

#[derive(Clone, Debug)]
struct Curve {
    n: u32,
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    g: (u64, u64),
    nk: usize,
}

impl Curve {
    fn add(&self, l: Pt, r: Pt) -> Pt {
        let p = self.p;
        let (a, b) = match (l, r) {
            (None, r) => return r,
            (l, None) => return l,
            (Some(a), Some(b)) => (a, b),
        };
        let (x1, y1) = a;
        let (x2, y2) = b;
        if x1 == x2 && (y1 + y2) % p == 0 {
            return None;
        }
        let slope = if a == b {
            (3 * x1 % p * x1 + self.a) % p * modinv(2 * y1 % p, p).unwrap() % p
        } else {
            (y2 + p - y1) % p * modinv((x2 + p - x1) % p, p).unwrap() % p
        };
        let x3 = (slope * slope % p + 2 * p - x1 - x2) % p;
        let y3 = (slope * ((x1 + p - x3) % p) % p + p - y1) % p;
        Some((x3, y3))
    }
    fn mul(&self, mut s: u64, pt: Pt) -> Pt {
        let mut out = None;
        let mut addend = pt;
        while s > 0 {
            if s & 1 == 1 {
                out = self.add(out, addend);
            }
            addend = self.add(addend, addend);
            s >>= 1;
        }
        out
    }
    fn xq(&self, k: u64) -> Option<u64> {
        self.mul(k, Some(self.g)).map(|(x, _)| x)
    }
}

fn rhs_of(p: u64, a: u64, b: u64, x: u64) -> u64 {
    (x * x % p * x % p + a * x % p + b) % p
}

/// Prime-order curve search: p = largest prime below 2^n, (a,b) drawn from the SplitMix64 stream
/// of seed "F4/curve/<n>", first draw with nonzero discriminant, prime group order r != p.
fn find_curve(n: u32) -> Curve {
    let p = prime_below(1u64 << n);
    let mut sqrt = vec![0u32; p as usize];
    for y in 1..p {
        let v = (y * y % p) as usize;
        if sqrt[v] == 0 {
            sqrt[v] = y as u32;
        }
    }
    let mut rng = Rng::from_seed_str(&format!("F4/curve/{}", n));
    loop {
        let a = rng.next() % p;
        let b = rng.next() % p;
        let disc = (4 * (a * a % p * a % p) + 27 * (b * b % p)) % p;
        if disc == 0 {
            continue;
        }
        let mut cnt = 1u64;
        let mut gen: Option<(u64, u64)> = None;
        for x in 0..p {
            let v = rhs_of(p, a, b, x);
            if v == 0 {
                cnt += 1;
                if gen.is_none() {
                    gen = Some((x, 0));
                }
            } else if sqrt[v as usize] != 0 {
                cnt += 2;
                if gen.is_none() {
                    gen = Some((x, sqrt[v as usize] as u64));
                }
            }
        }
        if cnt == p || !is_prime(cnt) {
            continue;
        }
        let nk = 64 - (cnt - 1).leading_zeros() as usize; // ceil(log2 r) for r not a power of two
        return Curve { n, p, a, b, r: cnt, g: gen.unwrap(), nk };
    }
}

// ------------------------------------------------------------------ CNF builder

const T: i32 = 1; // variable 1 is the constant TRUE
const F: i32 = -1;

type Word = Vec<i32>;
type St = (Word, Word); // (X, Z)

struct Cnf {
    nv: i32,
    cl: Vec<i32>, // flat, 0-terminated
    ncl: usize,
    and_cache: HashMap<(i32, i32), i32>,
    n: usize,
    p: u64,
    c: u64, // 2^n - p
}

impl Cnf {
    fn new(n: usize, p: u64) -> Cnf {
        let mut s = Cnf { nv: 1, cl: vec![1, 0], ncl: 1, and_cache: HashMap::new(), n, p, c: (1u64 << n) - p };
        s.nv = 1;
        s
    }
    fn fresh(&mut self) -> i32 {
        self.nv += 1;
        self.nv
    }
    fn fresh_word(&mut self, w: usize) -> Word {
        (0..w).map(|_| self.fresh()).collect()
    }
    fn clause(&mut self, lits: &[i32]) {
        let mut v: Vec<i32> = Vec::with_capacity(lits.len());
        for &l in lits {
            if l == T || v.contains(&-l) {
                return;
            }
            if l == F || v.contains(&l) {
                continue;
            }
            v.push(l);
        }
        assert!(!v.is_empty(), "empty clause generated (formula trivially UNSAT)");
        self.cl.extend_from_slice(&v);
        self.cl.push(0);
        self.ncl += 1;
    }
    fn and(&mut self, a: i32, b: i32) -> i32 {
        if a == F || b == F || a == -b {
            return F;
        }
        if a == T || a == b {
            return b;
        }
        if b == T {
            return a;
        }
        let key = (a.min(b), a.max(b));
        if let Some(&v) = self.and_cache.get(&key) {
            return v;
        }
        let v = self.fresh();
        self.clause(&[-v, a]);
        self.clause(&[-v, b]);
        self.clause(&[v, -a, -b]);
        self.and_cache.insert(key, v);
        v
    }
    fn or(&mut self, a: i32, b: i32) -> i32 {
        -self.and(-a, -b)
    }
    fn xor(&mut self, a: i32, b: i32) -> i32 {
        if a == F {
            return b;
        }
        if b == F {
            return a;
        }
        if a == T {
            return -b;
        }
        if b == T {
            return -a;
        }
        if a == b {
            return F;
        }
        if a == -b {
            return T;
        }
        let v = self.fresh();
        self.clause(&[-v, a, b]);
        self.clause(&[-v, -a, -b]);
        self.clause(&[v, -a, b]);
        self.clause(&[v, a, -b]);
        v
    }
    fn xor3(&mut self, a: i32, b: i32, c: i32) -> i32 {
        for (x, y, z) in [(a, b, c), (b, a, c), (c, a, b)] {
            if x == F {
                return self.xor(y, z);
            }
            if x == T {
                return -self.xor(y, z);
            }
        }
        if a == b {
            return c;
        }
        if a == -b {
            return -c;
        }
        if a == c {
            return b;
        }
        if a == -c {
            return -b;
        }
        if b == c {
            return a;
        }
        if b == -c {
            return -a;
        }
        let v = self.fresh();
        for m in 0..8u32 {
            let (ba, bb, bc) = (m & 1 == 1, m & 2 != 0, m & 4 != 0);
            let par = ba ^ bb ^ bc;
            let la = if ba { -a } else { a };
            let lb = if bb { -b } else { b };
            let lc = if bc { -c } else { c };
            self.clause(&[la, lb, lc, if par { v } else { -v }]);
        }
        v
    }
    fn maj(&mut self, a: i32, b: i32, c: i32) -> i32 {
        for (x, y, z) in [(a, b, c), (b, a, c), (c, a, b)] {
            if x == F {
                return self.and(y, z);
            }
            if x == T {
                return self.or(y, z);
            }
        }
        if a == b {
            return a;
        }
        if a == -b {
            return c;
        }
        if a == c {
            return a;
        }
        if a == -c {
            return b;
        }
        if b == c {
            return b;
        }
        if b == -c {
            return a;
        }
        let v = self.fresh();
        self.clause(&[-a, -b, v]);
        self.clause(&[-a, -c, v]);
        self.clause(&[-b, -c, v]);
        self.clause(&[a, b, -v]);
        self.clause(&[a, c, -v]);
        self.clause(&[b, c, -v]);
        v
    }
    /// s ? a1 : a0
    fn mux(&mut self, s: i32, a1: i32, a0: i32) -> i32 {
        if s == T {
            return a1;
        }
        if s == F || a1 == a0 {
            return a0;
        }
        if a1 == T && a0 == F {
            return s;
        }
        if a1 == F && a0 == T {
            return -s;
        }
        if a0 == F {
            return self.and(s, a1);
        }
        if a1 == F {
            return self.and(-s, a0);
        }
        if a0 == T {
            return -self.and(s, -a1);
        }
        if a1 == T {
            return -self.and(-s, -a0);
        }
        let v = self.fresh();
        self.clause(&[-s, -a1, v]);
        self.clause(&[-s, a1, -v]);
        self.clause(&[s, -a0, v]);
        self.clause(&[s, a0, -v]);
        self.clause(&[-a1, -a0, v]);
        self.clause(&[a1, a0, -v]);
        v
    }

    // ---- words (LSB first)
    fn cbits(c: u64, w: usize) -> Word {
        (0..w).map(|i| if (c >> i) & 1 == 1 { T } else { F }).collect()
    }
    fn add_words(&mut self, a: &[i32], b: &[i32], cin: i32) -> (Word, i32) {
        let w = a.len().max(b.len());
        let mut carry = cin;
        let mut out = Vec::with_capacity(w);
        for i in 0..w {
            let x = *a.get(i).unwrap_or(&F);
            let y = *b.get(i).unwrap_or(&F);
            out.push(self.xor3(x, y, carry));
            carry = self.maj(x, y, carry);
        }
        (out, carry)
    }
    fn const_val(w: &[i32]) -> Option<u64> {
        let mut v = 0u64;
        for (i, &l) in w.iter().enumerate() {
            if l == T {
                v |= 1 << i;
            } else if l != F {
                return None;
            }
        }
        Some(v)
    }
    /// a*b mod p for a,b < p (n-bit words).
    fn mulmod(&mut self, a: &[i32], b: &[i32]) -> Word {
        let n = self.n;
        let (ca, cb) = (Self::const_val(a), Self::const_val(b));
        if let (Some(x), Some(y)) = (ca, cb) {
            return Self::cbits((x as u128 * y as u128 % self.p as u128) as u64, n);
        }
        if ca == Some(0) || cb == Some(0) {
            return vec![F; n];
        }
        if ca == Some(1) {
            return b.to_vec();
        }
        if cb == Some(1) {
            return a.to_vec();
        }
        let mut acc: Word = vec![F; 2 * n];
        for i in 0..n {
            let row: Word = (0..n).map(|j| self.and(a[j], b[i])).collect();
            let (sum, carry) = self.add_words(&acc[i..i + n], &row, F);
            for j in 0..n {
                acc[i + j] = sum[j];
            }
            acc[i + n] = carry;
        }
        // reduction
        let q = self.fresh_word(n);
        let w = 2 * n + 1;
        let mut qc: Word = vec![F; w];
        for s in 0..64 {
            if (self.c >> s) & 1 == 1 {
                let shifted: Word = (0..w).map(|i| if i >= s && i - s < n { q[i - s] } else { F }).collect();
                qc = self.add_words(&qc, &shifted, F).0;
            }
        }
        let mut accw = acc.clone();
        accw.push(F);
        let (t, _) = self.add_words(&accw, &qc, F);
        for i in 0..n {
            // upper half of T equals q
            self.clause(&[-t[n + i], q[i]]);
            self.clause(&[t[n + i], -q[i]]);
        }
        self.clause(&[-t[2 * n]]);
        let rem: Word = t[..n].to_vec();
        // rem < p  <=>  rem + c does not carry out of n bits
        let (_, cy) = self.add_words(&rem, &Self::cbits(self.c, n), F);
        self.clause(&[-cy]);
        rem
    }
    fn mulc(&mut self, a: &[i32], k: u64) -> Word {
        let kb = Self::cbits(k % self.p, self.n);
        self.mulmod(a, &kb)
    }
    fn mod_add(&mut self, a: &[i32], b: &[i32]) -> Word {
        let n = self.n;
        let (mut s, cy) = self.add_words(a, b, F);
        s.push(cy);
        let cc = Self::cbits(self.c, n + 1);
        let (t, _) = self.add_words(&s, &cc, F);
        let ge = t[n];
        (0..n).map(|i| self.mux(ge, t[i], s[i])).collect()
    }
    fn mod_sub(&mut self, a: &[i32], b: &[i32]) -> Word {
        let n = self.n;
        let nb: Word = b.iter().map(|&l| -l).collect();
        let (d, cout) = self.add_words(a, &nb, T);
        let pb = Self::cbits(self.p, n);
        let (e, _) = self.add_words(&d, &pb, F);
        (0..n).map(|i| self.mux(cout, d[i], e[i])).collect()
    }
    fn cswap(&mut self, s: i32, x: &St, y: &St) -> (St, St) {
        let mx = |me: &mut Cnf, u: &Word, v: &Word| -> (Word, Word) {
            let nu: Word = (0..u.len()).map(|i| me.mux(s, v[i], u[i])).collect();
            let nv: Word = (0..u.len()).map(|i| me.mux(s, u[i], v[i])).collect();
            (nu, nv)
        };
        let (x0, y0) = mx(self, &x.0, &y.0);
        let (x1, y1) = mx(self, &x.1, &y.1);
        ((x0, x1), (y0, y1))
    }
}

struct Ladder {
    cnf: Cnf,
    kv: Word,
    xqv: Word,
}

fn ladder_add(c: &mut Cnf, curve: &Curve, a: &St, b: &St) -> St {
    let (p, ca, cb, xg) = (curve.p, curve.a, curve.b, curve.g.0);
    let m1 = c.mulmod(&a.0, &b.1);
    let m2 = c.mulmod(&b.0, &a.1);
    let m3 = c.mulmod(&a.0, &b.0);
    let m4 = c.mulmod(&a.1, &b.1);
    let m5 = c.mulc(&m4, ca);
    let u = c.mod_add(&m3, &m5);
    let v = c.mod_add(&m1, &m2);
    let w = c.mod_sub(&m1, &m2);
    let t1 = c.mulmod(&u, &v);
    let t1d = c.mod_add(&t1, &t1);
    let zz = c.mulmod(&m4, &m4);
    let t2 = c.mulc(&zz, 4 * cb % p);
    let w2 = c.mulmod(&w, &w);
    let t3 = c.mulc(&w2, xg);
    let s = c.mod_add(&t1d, &t2);
    let x3 = c.mod_sub(&s, &t3);
    (x3, w2)
}

fn ladder_dbl(c: &mut Cnf, curve: &Curve, a: &St) -> St {
    let (p, ca, cb) = (curve.p, curve.a, curve.b);
    let (x, z) = (&a.0, &a.1);
    let s1 = c.mulmod(x, x);
    let s2 = c.mulmod(z, z);
    let s3 = c.mulc(&s2, ca);
    let d = c.mod_sub(&s1, &s3);
    let d2 = c.mulmod(&d, &d);
    let z3 = c.mulmod(&s2, z);
    let xz = c.mulmod(x, &z3);
    let t = c.mulc(&xz, 8 * cb % p);
    let x4 = c.mod_sub(&d2, &t);
    let s13 = c.mod_add(&s1, &s3);
    let i1 = c.mulmod(x, &s13);
    let i2 = c.mulc(&z3, cb);
    let inner = c.mod_add(&i1, &i2);
    let zi = c.mulmod(z, &inner);
    let z2 = c.mod_add(&zi, &zi);
    let z4 = c.mod_add(&z2, &z2);
    (x4, z4)
}

fn build_ladder(curve: &Curve) -> Ladder {
    let n = curve.n as usize;
    let nk = curve.nk;
    let mut c = Cnf::new(n, curve.p);
    let kv = c.fresh_word(nk);
    let xqv = c.fresh_word(n);
    // k < r  <=>  k + (2^nk - r) does not carry out of nk bits
    let cr = Cnf::cbits((1u64 << nk) - curve.r, nk);
    let (_, cy) = c.add_words(&kv, &cr, F);
    c.clause(&[-cy]);
    let mut a: St = (Cnf::cbits(1, n), Cnf::cbits(0, n)); // O = (1:0)
    let mut b: St = (Cnf::cbits(curve.g.0, n), Cnf::cbits(1, n)); // G
    let mut prev = F;
    for i in (0..nk).rev() {
        let bit = kv[i];
        let s = c.xor(bit, prev);
        let (a2, b2) = c.cswap(s, &a, &b);
        prev = bit;
        let nb = ladder_add(&mut c, curve, &a2, &b2);
        let na = ladder_dbl(&mut c, curve, &a2);
        a = na;
        b = nb;
    }
    let (a, _b) = c.cswap(prev, &a, &b);
    let (xr, zr) = a;
    let zl: Vec<i32> = zr.clone();
    c.clause(&zl); // Z != 0
    let m = c.mulmod(&xqv, &zr);
    for i in 0..n {
        c.clause(&[-m[i], xr[i]]);
        c.clause(&[m[i], -xr[i]]);
    }
    Ladder { cnf: c, kv, xqv }
}

// ------------------------------------------------------------------ solving

fn load(cnf: &Cnf) -> CaDiCal {
    let mut s = CaDiCal::new();
    let mut cur: Vec<i32> = Vec::new();
    for &l in &cnf.cl {
        if l == 0 {
            s.clause6(&cur);
            cur.clear();
        } else {
            cur.push(l);
        }
    }
    s
}

fn stats(s: &CaDiCal) -> (i64, i64, i64) {
    (
        s.get_statistic_value("conflicts"),
        s.get_statistic_value("decisions"),
        s.get_statistic_value("propagations"),
    )
}

#[derive(Clone, Debug)]
struct Row {
    n: u32,
    arm: &'static str,
    target: usize,
    order: usize,
    k: u64,
    xq: u64,
    found: bool,
    verified: bool,
    decoded_k: Option<u64>,
    conflicts: u64,
    decisions: u64,
    propagations: u64,
    censored: bool,
    wall_ms: f64,
}

fn solve_target(
    s: &mut CaDiCal,
    lad: &Ladder,
    curve: &Curve,
    arm: &'static str,
    target: usize,
    order: usize,
    k: u64,
    budget: i32,
) -> Result<Row, String> {
    let xq = curve.xq(k).ok_or("k*G = O")?;
    let (c0, d0, p0) = stats(s);
    s.limit("conflicts", budget);
    let t0 = Instant::now();
    for i in 0..curve.n as usize {
        let lit = if (xq >> i) & 1 == 1 { lad.xqv[i] } else { -lad.xqv[i] };
        if arm == "REUSE" {
            s.assume(lit);
        } else {
            s.clause6(&[lit]);
        }
    }
    let st = s.solve();
    let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    let (c1, d1, p1) = stats(s);
    let mut row = Row {
        n: curve.n,
        arm,
        target,
        order,
        k,
        xq,
        found: false,
        verified: false,
        decoded_k: None,
        conflicts: (c1 - c0) as u64,
        decisions: (d1 - d0) as u64,
        propagations: (p1 - p0) as u64,
        censored: false,
        wall_ms,
    };
    eprintln!(
        "  n={} {} target={} status={:?} conflicts={} wall_ms={:.0}",
        curve.n, arm, target, st, row.conflicts, wall_ms
    );
    match st {
        Status::UNKNOWN => row.censored = true,
        Status::UNSATISFIABLE => {
            return Err(format!("FATAL: UNSAT on planted instance n={} arm={} target={} k={}", curve.n, arm, target, k))
        }
        Status::SATISFIABLE => {
            let mut kd = 0u64;
            for i in 0..curve.nk {
                if s.val(lad.kv[i]) > 0 {
                    kd |= 1 << i;
                }
            }
            row.found = true;
            row.decoded_k = Some(kd);
            row.verified = kd >= 1 && kd < curve.r && curve.xq(kd) == Some(xq);
            if !row.verified {
                return Err(format!("FATAL: decoded k={} does not satisfy x([k]G)=x_Q (n={} arm={} target={})", kd, curve.n, arm, target));
            }
        }
    }
    Ok(row)
}

struct NInfo {
    curve: Curve,
    vars: i32,
    clauses: usize,
    build_ms: f64,
}

fn targets_for(curve: &Curve, targets: usize) -> Vec<u64> {
    (0..targets)
        .map(|i| {
            let mut rng = Rng::from_seed_str(&format!("F4/target/{}/{}", curve.n, i));
            1 + rng.next() % (curve.r - 1)
        })
        .collect()
}

fn run_n(n: u32, targets: usize, budget: i32) -> Result<(NInfo, Vec<Row>), String> {
    let curve = find_curve(n);
    let t0 = Instant::now();
    let lad = build_ladder(&curve);
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    let info = NInfo { curve: curve.clone(), vars: lad.cnf.nv, clauses: lad.cnf.ncl, build_ms };
    let ks = targets_for(&curve, targets);
    let mut rows = Vec::new();
    // FRESH
    for (i, &k) in ks.iter().enumerate() {
        let mut s = load(&lad.cnf);
        rows.push(solve_target(&mut s, &lad, &curve, "FRESH", i, i, k, budget)?);
    }
    // REUSE: seeded random order, one incremental solver
    let mut order: Vec<usize> = (0..targets).collect();
    let mut rng = Rng::from_seed_str(&format!("F4/order/{}", n));
    for i in (1..order.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        order.swap(i, j);
    }
    let mut s = load(&lad.cnf);
    for (pos, &i) in order.iter().enumerate() {
        rows.push(solve_target(&mut s, &lad, &curve, "REUSE", i, pos, ks[i], budget)?);
    }
    Ok((info, rows))
}

// ------------------------------------------------------------------ statistics

/// Median with censored values ordered as +infinity; None if a middle element is censored.
fn median_c(vals: &[(u64, bool)]) -> Option<f64> {
    let mut v: Vec<u64> = vals.iter().map(|&(c, cens)| if cens { u64::MAX } else { c }).collect();
    v.sort();
    let m = v.len();
    if m == 0 {
        return None;
    }
    let (lo, hi) = (v[(m - 1) / 2], v[m / 2]);
    if lo == u64::MAX || hi == u64::MAX {
        None
    } else {
        Some((lo as f64 + hi as f64) / 2.0)
    }
}

fn lsq(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let m = xs.len() as f64;
    if xs.len() < 2 {
        return None;
    }
    let mx = xs.iter().sum::<f64>() / m;
    let my = ys.iter().sum::<f64>() / m;
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    if sxx == 0.0 {
        return None;
    }
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let s = sxy / sxx;
    Some((s, my - s * mx))
}

struct Fit {
    ns: Vec<u32>,
    alpha: Option<f64>,
    intercept: Option<f64>,
    lo: Option<f64>,
    hi: Option<f64>,
}

const BOOT: usize = 1000;

/// per-n conflicts of one arm; fits on the n at which every target finished.
fn fit_arm(arm: &str, data: &BTreeMap<u32, Vec<(u64, bool)>>) -> Fit {
    let ns: Vec<u32> = data.iter().filter(|(_, v)| !v.is_empty() && v.iter().all(|x| !x.1)).map(|(n, _)| *n).collect();
    let xs: Vec<f64> = ns.iter().map(|&n| n as f64).collect();
    let lg = |m: f64| m.max(1.0).log2();
    let ys: Vec<f64> = ns.iter().map(|n| lg(median_c(&data[n]).unwrap())).collect();
    let (alpha, intercept) = match lsq(&xs, &ys) {
        Some((s, c)) => (Some(s), Some(c)),
        None => (None, None),
    };
    let (mut lo, mut hi) = (None, None);
    if alpha.is_some() {
        let mut rng = Rng::from_seed_str(&format!("F4/boot/{}", arm));
        let mut slopes = Vec::with_capacity(BOOT);
        for _ in 0..BOOT {
            let ysb: Vec<f64> = ns
                .iter()
                .map(|n| {
                    let v = &data[n];
                    let rs: Vec<(u64, bool)> = (0..v.len()).map(|_| v[(rng.next() % v.len() as u64) as usize]).collect();
                    lg(median_c(&rs).unwrap())
                })
                .collect();
            if let Some((s, _)) = lsq(&xs, &ysb) {
                slopes.push(s);
            }
        }
        slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if !slopes.is_empty() {
            let m = slopes.len();
            lo = Some(slopes[((0.025 * m as f64) as usize).min(m - 1)]);
            hi = Some(slopes[((0.975 * m as f64) as usize).min(m - 1)]);
        }
    }
    Fit { ns, alpha, intercept, lo, hi }
}

fn fit_json(f: &Fit) -> J {
    jo(vec![
        ("uncensored_n", J::A(f.ns.iter().map(|&n| ji(n)).collect())),
        ("alpha", jopt(f.alpha)),
        ("intercept", jopt(f.intercept)),
        ("ci95_lo", jopt(f.lo)),
        ("ci95_hi", jopt(f.hi)),
        ("bootstrap_resamples", ji(BOOT as u64)),
    ])
}

// ------------------------------------------------------------------ CLI / main

struct Args {
    out: String,
    ns: Vec<u32>,
    targets: usize,
    budget: i32,
    quick: bool,
    threads: usize,
    protocol: String,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        out: "results/run.json".into(),
        ns: vec![10, 12, 14, 16, 18, 20],
        targets: 20,
        budget: 10_000_000,
        quick: false,
        threads: 1,
        protocol: concat!(env!("CARGO_MANIFEST_DIR"), "/../PROTOCOL.md").into(),
    };
    let (mut n_set, mut t_set) = (false, false);
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < argv.len() {
        let key = argv[i].as_str();
        let mut val = || -> Result<String, String> {
            i += 1;
            argv.get(i).cloned().ok_or(format!("missing value for {}", key))
        };
        match key {
            "--out" => a.out = val()?,
            "--n" => {
                a.ns = val()?.split(',').map(|s| s.trim().parse::<u32>().map_err(|e| e.to_string())).collect::<Result<_, _>>()?;
                n_set = true;
            }
            "--targets" => {
                a.targets = val()?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                t_set = true;
            }
            "--budget" => {
                let b: f64 = val()?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                if b < 1.0 || b > i32::MAX as f64 {
                    return Err("budget must be in [1, 2^31-1] (CaDiCaL limit is i32)".into());
                }
                a.budget = b as i32;
            }
            "--threads" => a.threads = val()?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            "--protocol" => a.protocol = val()?,
            "--quick" => a.quick = true,
            "-h" | "--help" => {
                println!("usage: sat_ladder_scaling [--out results/run.json] [--n 10,12,..] [--targets 20] [--budget 10000000] [--quick] [--threads 1] [--protocol PATH]");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {}", other)),
        }
        i += 1;
    }
    if a.quick {
        if !n_set {
            a.ns = vec![10, 12];
        }
        if !t_set {
            a.targets = 5;
        }
    }
    for &n in &a.ns {
        if !(4..=24).contains(&n) {
            return Err("n must be in 4..=24".into());
        }
    }
    if a.targets == 0 || a.threads == 0 {
        return Err("targets and threads must be >= 1".into());
    }
    Ok(a)
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(2);
        }
    };
    std::process::exit(run(&args));
}

fn fmt_opt(x: Option<f64>) -> String {
    x.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "cens".into())
}

fn run(args: &Args) -> i32 {
    let results: Mutex<Vec<(usize, Result<(NInfo, Vec<Row>), String>)>> = Mutex::new(Vec::new());
    let next = AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..args.threads.min(args.ns.len()) {
            sc.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::SeqCst);
                if j >= args.ns.len() {
                    break;
                }
                let n = args.ns[j];
                eprintln!("[n={}] building and solving ...", n);
                let r = run_n(n, args.targets, args.budget);
                eprintln!("[n={}] done", n);
                results.lock().unwrap().push((j, r));
            });
        }
    });
    let mut res = results.into_inner().unwrap();
    res.sort_by_key(|x| x.0);
    let mut infos: Vec<NInfo> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();
    for (_, r) in res {
        match r {
            Ok((i, rs)) => {
                infos.push(i);
                rows.extend(rs);
            }
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    }

    let src = include_bytes!("main.rs");
    let src_sha = sha256(src);
    let proto_sha = std::fs::read(&args.protocol).map(|b| sha256(&b)).unwrap_or_else(|_| "unavailable".into());

    let mut data: BTreeMap<&str, BTreeMap<u32, Vec<(u64, bool)>>> = BTreeMap::new();
    for r in &rows {
        data.entry(r.arm).or_default().entry(r.n).or_default().push((r.conflicts, r.censored));
    }
    let empty = BTreeMap::new();
    let fit_f = fit_arm("FRESH", data.get("FRESH").unwrap_or(&empty));
    let fit_r = fit_arm("REUSE", data.get("REUSE").unwrap_or(&empty));

    // per-n table
    let mut table = Vec::new();
    let mut ratios: Vec<(u32, Option<f64>)> = Vec::new();
    println!(
        "{:>3} {:>9} {:>10} | {:>12} {:>5} | {:>12} {:>5} | {:>7}",
        "n", "vars", "clauses", "FRESH med", "cens", "REUSE med", "cens", "ratio"
    );
    for inf in &infos {
        let n = inf.curve.n;
        let f = &data["FRESH"][&n];
        let r = &data["REUSE"][&n];
        let (mf, mr) = (median_c(f), median_c(r));
        let ratio = match (mf, mr) {
            (Some(a), Some(b)) if a > 0.0 => Some(b / a),
            _ => None,
        };
        ratios.push((n, ratio));
        let (cf, cr) = (f.iter().filter(|x| x.1).count(), r.iter().filter(|x| x.1).count());
        println!(
            "{:>3} {:>9} {:>10} | {:>12} {:>5} | {:>12} {:>5} | {:>7}",
            n,
            inf.vars,
            inf.clauses,
            fmt_opt(mf),
            cf,
            fmt_opt(mr),
            cr,
            ratio.map(|v| format!("{:.3}", v)).unwrap_or_else(|| "n/a".into())
        );
        table.push(jo(vec![
            ("n", ji(n)),
            ("vars", ji(inf.vars)),
            ("clauses", ji(inf.clauses as u64)),
            ("fresh_median_conflicts", jopt(mf)),
            ("fresh_censored", ji(cf as u64)),
            ("reuse_median_conflicts", jopt(mr)),
            ("reuse_censored", ji(cr as u64)),
            ("reuse_over_fresh_median", jopt(ratio)),
        ]));
    }
    let fa = |x: Option<f64>| x.map(|v| format!("{:.3}", v)).unwrap_or_else(|| "n/a".into());
    println!(
        "alpha FRESH = {} [{}, {}] over n={:?};  alpha REUSE = {} [{}, {}] over n={:?}",
        fa(fit_f.alpha),
        fa(fit_f.lo),
        fa(fit_f.hi),
        fit_f.ns,
        fa(fit_r.alpha),
        fa(fit_r.lo),
        fa(fit_r.hi),
        fit_r.ns
    );

    // predictions / decision rules
    let p3 = rows.iter().all(|r| {
        r.censored || (r.found && r.verified && r.decoded_k.map(|d| d == r.k || d == infos.iter().find(|i| i.curve.n == r.n).unwrap().curve.r - r.k).unwrap_or(false))
    });
    let uncens_ns = fit_f.ns.len();
    let instrument_failure = !p3 || uncens_ns < 3;
    let p1 = fit_f.alpha.map(|a| (0.85..=1.15).contains(&a));
    let p2_vals: Vec<f64> = ratios.iter().filter_map(|x| x.1).collect();
    let p2 = if p2_vals.is_empty() { None } else { Some(p2_vals.iter().all(|&v| v >= 0.5)) };
    let big_unc = |f: &Fit| f.ns.iter().any(|&n| n == 18 || n == 20);
    let ci_alive = |f: &Fit| f.hi.map(|h| h < 0.6).unwrap_or(false) && big_unc(f);
    let mut consecutive = false;
    for w in ratios.windows(2) {
        if let (Some(a), Some(b)) = (w[0].1, w[1].1) {
            if a < 0.2 && b < 0.2 {
                consecutive = true;
            }
        }
    }
    let lead_alive = !instrument_failure && (ci_alive(&fit_f) || ci_alive(&fit_r) || consecutive);
    let verdict = if instrument_failure {
        "instrument failure (rule 1)"
    } else if lead_alive {
        "lead alive (rule 2)"
    } else {
        "closed (rule 3)"
    };
    println!("P1={:?} P2={:?} P3={} verdict: {}{}", p1, p2, p3, verdict, if args.quick { " (quick run: not the frozen configuration, not decisive)" } else { "" });

    let row_json: Vec<J> = rows
        .iter()
        .map(|r| {
            jo(vec![
                ("n", ji(r.n)),
                ("arm", js(r.arm)),
                ("target", ji(r.target as u64)),
                ("solve_order", ji(r.order as u64)),
                ("k", ju(r.k)),
                ("x_q", ju(r.xq)),
                ("found", J::Bool(r.found)),
                ("verified", J::Bool(r.verified)),
                ("decoded_k", r.decoded_k.map(ju).unwrap_or(J::Null)),
                ("conflicts", ju(r.conflicts)),
                ("decisions", ju(r.decisions)),
                ("propagations", ju(r.propagations)),
                ("censored", J::Bool(r.censored)),
                ("wall_ms", jf(r.wall_ms)),
            ])
        })
        .collect();
    let curves: Vec<J> = infos
        .iter()
        .map(|i| {
            let c = &i.curve;
            jo(vec![
                ("n", ji(c.n)),
                ("p", ju(c.p)),
                ("a", ju(c.a)),
                ("b", ju(c.b)),
                ("r", ju(c.r)),
                ("G", J::A(vec![ju(c.g.0), ju(c.g.1)])),
                ("nk", ji(c.nk as u64)),
                ("vars", ji(i.vars)),
                ("clauses", ji(i.clauses as u64)),
                ("cnf_build_ms", jf(i.build_ms)),
            ])
        })
        .collect();
    let doc = jo(vec![
        (
            "header",
            jo(vec![
                ("study", js("F4 sat_ladder_scaling")),
                ("solver", js("CaDiCaL")),
                ("solver_version", js(&CaDiCal::version())),
                ("solver_signature", js(&CaDiCal::signature())),
                ("solver_crate", js("cadical-sys 0.8.0 (vendored CaDiCaL C++ source, built via cc)")),
                ("fallback_solver_used", J::Bool(false)),
                ("solver_options", js("CaDiCaL defaults; only per-solve limit('conflicts', budget)")),
                (
                    "stat_definitions",
                    js("conflicts, decisions: CaDiCaL stats.conflicts/stats.decisions, delta per solve call; propagations: stats.propagations.search (search-loop propagations only, excludes probing/vivification), delta per solve call"),
                ),
                (
                    "encoding",
                    js("k = nk=ceil(log2 r) free bits with k<r comparator; ladder over all nk bits from (O=(1:0),G), xor-swap muxes; schoolbook multiplier, reduction prod + q*c = q*2^n + rem (c=2^n-p), rem<p; X = x_Q*Z mod p, Z!=0; x_Q bits free vars (assumptions in REUSE, units in FRESH); ~20 modular mults per ladder step"),
                ),
                (
                    "x_only_note",
                    js("x-only ladder cannot distinguish k from r-k; a model is accepted iff x([k']G)=x_Q (verified); P3 counts k' in {k, r-k} as the true k"),
                ),
                ("protocol_sha256", js(&proto_sha)),
                ("source_sha256", js(&src_sha)),
                ("ns", J::A(args.ns.iter().map(|&n| ji(n)).collect())),
                ("targets", ji(args.targets as u64)),
                ("conflict_budget", ji(args.budget)),
                ("quick", J::Bool(args.quick)),
                ("threads", ji(args.threads as u64)),
                ("censoring_rules", js("fit uses n where every target of the arm finished; ratio uses medians with censored ordered as +inf (null if a middle element is censored); nothing is imputed")),
            ]),
        ),
        ("curves", J::A(curves)),
        ("rows", J::A(row_json)),
        ("fits", jo(vec![("FRESH", fit_json(&fit_f)), ("REUSE", fit_json(&fit_r))])),
        (
            "summary",
            jo(vec![
                ("per_n", J::A(table)),
                ("P1_alpha_in_0.85_1.15", p1.map(J::Bool).unwrap_or(J::Null)),
                ("P2_all_ratios_ge_0.5", p2.map(J::Bool).unwrap_or(J::Null)),
                ("P3_all_models_decode", J::Bool(p3)),
                ("instrument_failure", J::Bool(instrument_failure)),
                ("lead_alive", J::Bool(lead_alive)),
                ("verdict", js(verdict)),
            ]),
        ),
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
    eprintln!("wrote {}", args.out);
    0
}

// ------------------------------------------------------------------ tests

#[cfg(test)]
mod tests {
    use super::*;

    fn sat_with(cnf: &Cnf, s: &mut CaDiCal, units: &[i32]) -> Status {
        let _ = cnf;
        for &u in units {
            s.assume(u);
        }
        s.solve()
    }
    fn word_units(w: &[i32], v: u64) -> Vec<i32> {
        w.iter().enumerate().map(|(i, &l)| if (v >> i) & 1 == 1 { l } else { -l }).collect()
    }
    fn read(s: &mut CaDiCal, w: &[i32]) -> u64 {
        let mut v = 0;
        for (i, &l) in w.iter().enumerate() {
            let b = if l == T {
                true
            } else if l == F {
                false
            } else {
                s.val(l) > 0
            };
            if b {
                v |= 1 << i;
            }
        }
        v
    }

    #[test]
    fn sha256_known() {
        assert_eq!(sha256(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn field_circuits_random_inputs() {
        for n in [10usize, 12] {
            let p = prime_below(1u64 << n);
            let mut c = Cnf::new(n, p);
            let a = c.fresh_word(n);
            let b = c.fresh_word(n);
            let (sum, cy) = c.add_words(&a, &b, F);
            let mm = c.mulmod(&a, &b);
            let ma = c.mod_add(&a, &b);
            let ms = c.mod_sub(&a, &b);
            let mc = c.mulc(&a, 12345);
            let mut s = load(&c);
            let mut rng = Rng::from_seed_str(&format!("test/field/{}", n));
            for it in 0..40 {
                let (x, y) = if it == 0 {
                    (p - 1, p - 1)
                } else if it == 1 {
                    (0, p - 1)
                } else {
                    (rng.next() % p, rng.next() % p)
                };
                let mut u = word_units(&a, x);
                u.extend(word_units(&b, y));
                assert_eq!(sat_with(&c, &mut s, &u), Status::SATISFIABLE);
                assert_eq!(read(&mut s, &sum) + ((read(&mut s, &[cy]) as u64) << n), x + y);
                assert_eq!(read(&mut s, &mm), (x as u128 * y as u128 % p as u128) as u64, "mulmod {} {}", x, y);
                assert_eq!(read(&mut s, &ma), (x + y) % p);
                assert_eq!(read(&mut s, &ms), (x + p - y) % p);
                assert_eq!(read(&mut s, &mc), (x as u128 * (12345 % p) as u128 % p as u128) as u64);
            }
        }
    }

    #[test]
    fn ladder_fixed_k_reproduces_x_of_kg() {
        let curve = find_curve(10);
        let lad = build_ladder(&curve);
        let mut s = load(&lad.cnf);
        let mut ks = vec![1u64, 2, 3, curve.r - 1, curve.r - 2, (curve.r - 1) / 2, (curve.r + 1) / 2];
        let mut rng = Rng::from_seed_str("test/ladder");
        for _ in 0..12 {
            ks.push(1 + rng.next() % (curve.r - 1));
        }
        for &k in &ks {
            let units = word_units(&lad.kv, k);
            assert_eq!(sat_with(&lad.cnf, &mut s, &units), Status::SATISFIABLE, "k={}", k);
            let x = read(&mut s, &lad.xqv);
            assert_eq!(Some(x), curve.xq(k), "k={}", k);
        }
        // k = 0 (Z = 0) and k >= r are excluded
        for &k in &[0u64, curve.r, (1u64 << curve.nk) - 1] {
            if k < (1u64 << curve.nk) {
                let units = word_units(&lad.kv, k);
                assert_eq!(sat_with(&lad.cnf, &mut s, &units), Status::UNSATISFIABLE, "k={}", k);
            }
        }
    }

    fn planted(n: u32) {
        let (info, rows) = run_n(n, 3, 10_000_000).unwrap();
        assert!(info.vars > 0);
        for r in &rows {
            assert!(r.found && r.verified && !r.censored, "{:?}", r);
            let d = r.decoded_k.unwrap();
            assert!(d == r.k || d == info.curve.r - r.k);
        }
        assert_eq!(rows.iter().filter(|r| r.arm == "FRESH").count(), 3);
        assert_eq!(rows.iter().filter(|r| r.arm == "REUSE").count(), 3);
    }
    /// Full-problem planted recovery in both arms at sizes where CaDiCaL finishes (n = 4..6).
    #[test]
    fn planted_k_recovered_small_n() {
        for n in [4u32, 5] {
            planted(n);
        }
    }
    #[test]
    #[ignore = "n=6 FRESH needs ~3e5 conflicts (about 1 min)"]
    fn planted_k_recovered_n6() {
        planted(6);
    }

    /// Hinted recovery at n = 10 and 12 (the full instance is censored at 1e7 conflicts): the top
    /// nk-2 bits of k are given as units (FRESH) / assumptions (REUSE); the solver must find the
    /// low 2 bits, the model is decoded and verified natively, in both arms.
    fn hinted(n: u32) {
        let curve = find_curve(n);
        let lad = build_ladder(&curve);
        let ks = targets_for(&curve, 2);
        let hint = |k: u64| -> Vec<i32> {
            (2..curve.nk).map(|i| if (k >> i) & 1 == 1 { lad.kv[i] } else { -lad.kv[i] }).collect()
        };
        let check = |s: &mut CaDiCal, k: u64| {
            let kd = read(s, &lad.kv);
            assert!(kd == k || kd == curve.r - k);
            assert_eq!(curve.xq(kd), curve.xq(k));
        };
        for arm in ["FRESH", "REUSE"] {
            let mut s = load(&lad.cnf);
            for &k in &ks {
                if arm == "FRESH" {
                    s = load(&lad.cnf);
                }
                let xq = curve.xq(k).unwrap();
                let mut lits = word_units(&lad.xqv, xq);
                lits.extend(hint(k));
                for l in lits {
                    if arm == "FRESH" {
                        s.clause6(&[l]);
                    } else {
                        s.assume(l);
                    }
                }
                s.limit("conflicts", 3_000_000);
                assert_eq!(s.solve(), Status::SATISFIABLE, "n={} arm={} k={}", n, arm, k);
                check(&mut s, k);
            }
        }
    }
    #[test]
    fn planted_k_recovered_hinted_n10() {
        hinted(10);
    }
    #[test]
    fn planted_k_recovered_hinted_n12() {
        hinted(12);
    }

    #[test]
    fn tiny_budget_is_censored() {
        let (_, rows) = run_n(12, 2, 1).unwrap();
        assert!(rows.iter().any(|r| r.censored));
    }
}
