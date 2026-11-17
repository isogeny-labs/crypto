//! Independent native check of the logarithm recovered by `ic price` on rule v3
//! k0n53/T01 (research/ic_aimed_gather_20261023/aa2_20261029).
//!
//! Written by the validator; supersedes verify_scalars.py (kept as historical
//! evidence).  One file, standard library only, no repository code:
//!
//!     rustc -O -o verify_scalars verify_scalars.rs
//!     ./verify_scalars --registry docs/curves/registry.json \
//!         --slug icv1-f2m53-tm56619371-dac20a85 \
//!         --params research/ic_tool_program/rule/v3/runs/k0n53/T01.params.json \
//!         --target-record research/ic_tool_program/rule/v3/runs/k0n53/T01-R1.price.json \
//!         --recovered <ic price --json outputs...>
//!
//! Where each value comes from:
//!   registry (curves[slug]): modulus (params.modulus and the representation's
//!     modulus_exponents), #E (order), trace, r (subgroup_order), cofactor, G,
//!     the model coefficients;
//!   params: n (curve.degree), a (curve.curve_a), targets[0].public_hash_seed;
//!   target record: the Q, G and modulus low terms an earlier `ic price` printed
//!     (compared with only; Q is derived here from the seed);
//!   recovered: each output's status, all_verified, curve, n, a, r, recovered[0].
//!
//! Checks: BLAKE3 self-test; the three modulus encodings agree; the modulus is
//! irreducible (Rabin); #E and the trace from the Frobenius trace recurrence
//! equal the registry; r is prime (deterministic Miller-Rabin, r < 3.3e24) and
//! r * cofactor = #E; G on E, G != O, r*G = O (so ord G = r); one root lambda of
//! l^2 - t1*l + 2 = 0 mod r has Frobenius(G) = lambda*G; Q derived from the seed
//! by the workflow's public hash-to-curve (BLAKE3 over a 62-byte message, the
//! low n bits of digest[0..8] as x, the lift chosen by digest[8] & 1 among the
//! lifts sorted by (x+1, y), times the cofactor, counter incremented until the
//! result is not O) equals the recorded Q, is on E and r*Q = O; every recovered
//! d satisfies 1 <= d < r and d*G == Q; negative controls: (d+1)*G != Q and
//! seed+1 hashes to a different point on E.
//!
//! Prints a JSON report; exits 0 only if every check passes.

use std::collections::BTreeMap;
use std::process::exit;

// ------------------------------------------------------------------ JSON

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, k: &str) -> Option<&Json> {
        match self {
            Json::Obj(v) => v.iter().find(|(kk, _)| kk == k).map(|(_, x)| x),
            _ => None,
        }
    }
    fn at(&self, i: usize) -> Option<&Json> {
        match self {
            Json::Arr(v) => v.get(i),
            _ => None,
        }
    }
    fn path(&self, p: &[&str]) -> Option<&Json> {
        let mut cur = self;
        for k in p {
            cur = match k.parse::<usize>() {
                Ok(i) if matches!(cur, Json::Arr(_)) => cur.at(i)?,
                _ => cur.get(k)?,
            };
        }
        Some(cur)
    }
    fn str(&self) -> Option<&str> {
        if let Json::Str(s) = self { Some(s) } else { None }
    }
    fn arr(&self) -> Option<&Vec<Json>> {
        if let Json::Arr(v) = self { Some(v) } else { None }
    }
    fn u128(&self) -> Option<u128> {
        match self {
            Json::Num(s) => s.parse().ok(),
            Json::Str(s) => parse_int(s),
            _ => None,
        }
    }
    fn i128(&self) -> Option<i128> {
        match self {
            Json::Num(s) | Json::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn bool(&self) -> Option<bool> {
        if let Json::Bool(b) = self { Some(*b) } else { None }
    }
}

fn parse_int(s: &str) -> Option<u128> {
    if let Some(h) = s.strip_prefix("0x") {
        u128::from_str_radix(h, 16).ok()
    } else {
        s.parse().ok()
    }
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }
    fn err<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("JSON: {m} at byte {}", self.i))
    }
    fn expect(&mut self, c: u8) -> Result<(), String> {
        self.ws();
        if self.b.get(self.i) == Some(&c) {
            self.i += 1;
            Ok(())
        } else {
            self.err(&format!("expected '{}'", c as char))
        }
    }
    fn lit(&mut self, s: &str, v: Json) -> Result<Json, String> {
        if self.b[self.i..].starts_with(s.as_bytes()) {
            self.i += s.len();
            Ok(v)
        } else {
            self.err("bad literal")
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let h = self.b.get(self.i..self.i + 4).ok_or("JSON: short \\u escape")?;
        self.i += 4;
        u32::from_str_radix(std::str::from_utf8(h).map_err(|e| e.to_string())?, 16).map_err(|e| e.to_string())
    }
    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i).ok_or("JSON: unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or("JSON: bad escape")?;
                    self.i += 1;
                    match e {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp) && self.b[self.i..].starts_with(b"\\u") {
                                self.i += 2;
                                let lo = self.hex4()?;
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            let ch = char::from_u32(cp).unwrap_or('\u{FFFD}');
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return self.err("bad escape"),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.b.get(self.i) {
            Some(b'{') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Json::Obj(v));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.expect(b':')?;
                    let x = self.value()?;
                    v.push((k, x));
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(Json::Obj(v));
                        }
                        _ => return self.err("expected ',' or '}'"),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Json::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(Json::Arr(v));
                        }
                        _ => return self.err("expected ',' or ']'"),
                    }
                }
            }
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.lit("true", Json::Bool(true)),
            Some(b'f') => self.lit("false", Json::Bool(false)),
            Some(b'n') => self.lit("null", Json::Null),
            Some(_) => {
                let s = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                if s == self.i {
                    return self.err("unexpected byte");
                }
                Ok(Json::Num(String::from_utf8_lossy(&self.b[s..self.i]).into_owned()))
            }
            None => self.err("unexpected end"),
        }
    }
}

fn load(path: &str) -> Json {
    let b = std::fs::read(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut p = Parser { b: &b, i: 0 };
    let v = p.value().unwrap_or_else(|e| die(&format!("{path}: {e}")));
    p.ws();
    if p.i != b.len() {
        die(&format!("{path}: trailing data"));
    }
    v
}

fn die(m: &str) -> ! {
    eprintln!("verify_scalars: {m}");
    exit(2)
}

fn jstr(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

// ------------------------------------------------------------------ BLAKE3 (one chunk)

const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];
const PERM: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const CHUNK_START: u32 = 1;
const CHUNK_END: u32 = 2;
const ROOT: u32 = 8;

fn g(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    s[a] = s[a].wrapping_add(s[b]).wrapping_add(mx);
    s[d] = (s[d] ^ s[a]).rotate_right(16);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] = (s[b] ^ s[c]).rotate_right(12);
    s[a] = s[a].wrapping_add(s[b]).wrapping_add(my);
    s[d] = (s[d] ^ s[a]).rotate_right(8);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] = (s[b] ^ s[c]).rotate_right(7);
}

fn compress(cv: &[u32; 8], block: &[u32; 16], counter: u64, len: u32, flags: u32) -> [u32; 16] {
    let mut s = [
        cv[0], cv[1], cv[2], cv[3], cv[4], cv[5], cv[6], cv[7], IV[0], IV[1], IV[2], IV[3],
        counter as u32, (counter >> 32) as u32, len, flags,
    ];
    let mut m = *block;
    for r in 0..7 {
        g(&mut s, 0, 4, 8, 12, m[0], m[1]);
        g(&mut s, 1, 5, 9, 13, m[2], m[3]);
        g(&mut s, 2, 6, 10, 14, m[4], m[5]);
        g(&mut s, 3, 7, 11, 15, m[6], m[7]);
        g(&mut s, 0, 5, 10, 15, m[8], m[9]);
        g(&mut s, 1, 6, 11, 12, m[10], m[11]);
        g(&mut s, 2, 7, 8, 13, m[12], m[13]);
        g(&mut s, 3, 4, 9, 14, m[14], m[15]);
        if r < 6 {
            let mut p = [0u32; 16];
            for i in 0..16 {
                p[i] = m[PERM[i]];
            }
            m = p;
        }
    }
    for i in 0..8 {
        s[i] ^= s[i + 8];
        s[i + 8] ^= cv[i];
    }
    s
}

fn blake3_one_chunk(data: &[u8]) -> [u8; 32] {
    assert!(data.len() <= 1024, "single chunk only");
    let nblocks = if data.is_empty() { 1 } else { (data.len() + 63) / 64 };
    let mut cv = IV;
    for i in 0..nblocks {
        let blk = &data[(i * 64).min(data.len())..((i + 1) * 64).min(data.len())];
        let mut buf = [0u8; 64];
        buf[..blk.len()].copy_from_slice(blk);
        let mut w = [0u32; 16];
        for j in 0..16 {
            w[j] = u32::from_le_bytes([buf[4 * j], buf[4 * j + 1], buf[4 * j + 2], buf[4 * j + 3]]);
        }
        let mut flags = if i == 0 { CHUNK_START } else { 0 };
        if i == nblocks - 1 {
            flags |= CHUNK_END | ROOT;
        }
        let out = compress(&cv, &w, 0, blk.len() as u32, flags);
        cv.copy_from_slice(&out[..8]);
    }
    let mut d = [0u8; 32];
    for j in 0..8 {
        d[4 * j..4 * j + 4].copy_from_slice(&cv[j].to_le_bytes());
    }
    d
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

// ------------------------------------------------------------------ GF(2)[x], GF(2^n)

fn deg(a: u128) -> i32 {
    127 - a.leading_zeros() as i32
}

fn clmul(a: u128, b: u128) -> u128 {
    assert!(a == 0 || b == 0 || deg(a) + deg(b) < 128, "clmul overflow");
    let (mut a, mut b, mut r) = (a, b, 0u128);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    r
}

fn pdivmod(a: u128, b: u128) -> (u128, u128) {
    let (mut q, mut r) = (0u128, a);
    let db = deg(b);
    while r != 0 && deg(r) >= db {
        let sh = deg(r) - db;
        q ^= 1u128 << sh;
        r ^= b << sh;
    }
    (q, r)
}

fn pgcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = pdivmod(a, b).1;
        a = b;
        b = r;
    }
    a
}

#[derive(Clone, Copy)]
struct Field {
    f: u128,
    n: u32,
}

impl Field {
    fn red(&self, x: u128) -> u64 {
        pdivmod(x, self.f).1 as u64
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.red(clmul(a as u128, b as u128))
    }
    fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of 0");
        let (mut r0, mut r1, mut s0, mut s1) = (self.f, a as u128, 0u128, 1u128);
        while r1 != 0 {
            let (q, r) = pdivmod(r0, r1);
            r0 = r1;
            r1 = r;
            let ns = s0 ^ clmul(q, s1);
            s0 = s1;
            s1 = ns;
        }
        assert!(r0 == 1, "not invertible: modulus reducible");
        self.red(s0)
    }
    fn frob(&self, mut a: u64, k: u32) -> u64 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }
    fn trace(&self, a: u64) -> u64 {
        let (mut t, mut v) = (0u64, a);
        for _ in 0..self.n {
            t ^= v;
            v = self.sqr(v);
        }
        t
    }
    fn half_trace(&self, c: u64) -> u64 {
        let (mut h, mut v) = (0u64, c);
        for _ in 0..=((self.n - 1) / 2) {
            h ^= v;
            v = self.sqr(self.sqr(v));
        }
        h
    }
    fn irreducible(&self) -> bool {
        // Rabin: x^(2^n) = x mod f, and gcd(x^(2^(n/p)) - x, f) = 1 for every prime p | n.
        let x = 2u64;
        if self.frob(x, self.n) != x {
            return false;
        }
        for p in small_prime_factors(self.n as u64) {
            let h = self.frob(x, self.n / p as u32) ^ x;
            if pgcd(self.f, h as u128) != 1 {
                return false;
            }
        }
        true
    }
}

// ------------------------------------------------------------------ integers

fn small_prime_factors(mut m: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut p = 2u64;
    while p * p <= m {
        if m % p == 0 {
            out.push(p);
            while m % p == 0 {
                m /= p;
            }
        }
        p += 1;
    }
    if m > 1 {
        out.push(m);
    }
    out
}

fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    // operands < m < 2^64 here, so the product fits u128
    assert!(m < (1u128 << 64));
    (a % m) * (b % m) % m
}

fn powmod(mut b: u128, mut e: u128, m: u128) -> u128 {
    let mut r = 1u128 % m;
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

fn is_prime(m: u128) -> bool {
    assert!(m < 3_317_044_064_679_887_385_961_981u128);
    if m < 2 {
        return false;
    }
    let bases = [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    for &p in &bases {
        if m % p == 0 {
            return m == p;
        }
    }
    let (mut d, mut s) = (m - 1, 0u32);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for &a in &bases {
        let mut x = powmod(a, d, m);
        if x == 1 || x == m - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, m);
            if x == m - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn sqrt_mod(a: u128, p: u128) -> Option<u128> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u128;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, powmod(z, q, p), powmod(a, q, p), powmod(a, (q + 1) / 2, p));
    while t != 1 {
        let (mut i, mut t2) = (0u32, t);
        while t2 != 1 {
            t2 = mulmod(t2, t2, p);
            i += 1;
        }
        let b = powmod(c, 1u128 << (m - i - 1), p);
        m = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        r = mulmod(r, b, p);
    }
    Some(r)
}

// ------------------------------------------------------------------ the curve

type Pt = Option<(u64, u64)>; // None = O

struct Curve {
    f: Field,
    a: u64,
}

impl Curve {
    fn on(&self, p: Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let x2 = self.f.sqr(x);
                (self.f.sqr(y) ^ self.f.mul(x, y)) == (self.f.mul(x2, x) ^ if self.a == 1 { x2 } else { 0 } ^ 1)
            }
        }
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = &self.f;
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        if x1 == x2 {
            if y2 == x1 ^ y1 {
                return None;
            }
            let lam = x1 ^ f.mul(y1, f.inv(x1));
            let x3 = f.sqr(lam) ^ lam ^ self.a;
            return Some((x3, f.sqr(x1) ^ f.mul(lam ^ 1, x3)));
        }
        let lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.sqr(lam) ^ lam ^ x1 ^ x2 ^ self.a;
        Some((x3, f.mul(lam, x1 ^ x3) ^ x3 ^ y1))
    }
    fn mul(&self, mut k: u128, p: Pt) -> Pt {
        let (mut r, mut a) = (None, p);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, a);
            }
            a = self.add(a, a);
            k >>= 1;
        }
        r
    }
    /// (#E, trace) from s_0 = 2, s_1 = t1, s_k = t1 s_{k-1} - 2 s_{k-2}.
    fn order(&self) -> (u128, i128) {
        let t1: i128 = if self.a == 0 { -1 } else { 1 };
        let (mut sp, mut s) = (2i128, t1);
        for _ in 1..self.f.n {
            let nx = t1 * s - 2 * sp;
            sp = s;
            s = nx;
        }
        (((1i128 << self.f.n) + 1 - s) as u128, s)
    }
    fn lifts(&self, x: u64) -> Vec<(u64, u64)> {
        let f = &self.f;
        if x == 0 {
            return vec![(0, f.frob(1, f.n - 1))];
        }
        let c = x ^ self.a ^ f.sqr(f.inv(x)); // u^2 + u = x + a + 1/x^2, y = x u
        if f.trace(c) != 0 {
            return vec![];
        }
        let y = f.mul(x, f.half_trace(c));
        vec![(x, y), (x, y ^ x)]
    }
}

struct Target {
    q: (u64, u64),
    counter: u64,
    digest: [u8; 32],
    msg_len: usize,
    in_subgroup: bool,
}

fn public_hash_target(e: &Curve, n: u32, a: u8, seed: u64, cofactor: u128, r: u128) -> Target {
    let mask = (1u64 << n) - 1;
    for counter in 0u64..1_000_000 {
        let mut msg: Vec<u8> = b"ic-workflow-public-target-v1\0".to_vec();
        msg.extend_from_slice(&n.to_le_bytes()); // n: u32
        msg.push(a); // a: u8
        msg.extend_from_slice(&1u32.to_le_bytes()); // k: u32 = 1
        msg.extend_from_slice(&1u64.to_le_bytes()); // b_index: u64 = 1
        msg.extend_from_slice(&seed.to_le_bytes());
        msg.extend_from_slice(&counter.to_le_bytes());
        let d = blake3_one_chunk(&msg);
        let mut w = [0u8; 8];
        w.copy_from_slice(&d[..8]);
        let x = u64::from_le_bytes(w) & mask;
        let mut l = e.lifts(x);
        if l.is_empty() {
            continue;
        }
        l.sort_by_key(|p| (p.0 as u128 + 1, p.1));
        let raw = l[(d[8] & 1) as usize % l.len()];
        let t = e.mul(cofactor, Some(raw));
        if let Some(q) = t {
            return Target { q, counter, digest: d, msg_len: msg.len(), in_subgroup: e.mul(r, t).is_none() };
        }
    }
    die("hash-to-curve cap exhausted")
}

// ------------------------------------------------------------------ driver

fn need<T>(v: Option<T>, what: &str) -> T {
    v.unwrap_or_else(|| die(&format!("missing or malformed: {what}")))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut opt: BTreeMap<String, String> = BTreeMap::new();
    let mut recovered: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--registry" | "--slug" | "--params" | "--target-record" => {
                opt.insert(args[i].clone(), need(args.get(i + 1).cloned(), &args[i]));
                i += 2;
            }
            "--recovered" => {
                i += 1;
                while i < args.len() && !args[i].starts_with("--") {
                    recovered.push(args[i].clone());
                    i += 1;
                }
            }
            o => die(&format!("unknown argument {o}")),
        }
    }
    for k in ["--registry", "--slug", "--params", "--target-record"] {
        if !opt.contains_key(k) {
            die(&format!("{k} is required"));
        }
    }
    if recovered.is_empty() {
        die("--recovered needs at least one file");
    }
    let slug = opt["--slug"].clone();
    let mut checks: Vec<(String, bool)> = Vec::new();
    let mut ck = |k: &str, v: bool| checks.push((k.to_string(), v));

    ck(
        "blake3_selftest",
        hex(&blake3_one_chunk(b"")) == "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
            && hex(&blake3_one_chunk(b"abc")) == "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
    );

    let reg = load(&opt["--registry"]);
    let entries: Vec<&Json> = need(reg.get("curves").and_then(|c| c.arr()), "registry curves")
        .iter()
        .filter(|c| c.get("slug").and_then(|s| s.str()) == Some(slug.as_str()))
        .collect();
    if entries.len() != 1 {
        die(&format!("registry has {} entries for {slug}", entries.len()));
    }
    let ent = entries[0];
    let params = load(&opt["--params"]);
    let trec = load(&opt["--target-record"]);

    let n = need(params.path(&["curve", "degree"]).and_then(|v| v.u128()), "params curve.degree") as u32;
    let a = need(params.path(&["curve", "curve_a"]).and_then(|v| v.u128()), "params curve.curve_a") as u8;
    let seed = need(params.path(&["targets", "0", "public_hash_seed"]).and_then(|v| v.u128()), "public_hash_seed") as u64;
    let f = need(ent.path(&["params", "modulus"]).and_then(|v| v.u128()), "registry params.modulus");
    let rp = need(ent.path(&["representations", "0"]), "registry representation");
    let f_exp: u128 = need(rp.path(&["field", "modulus_exponents"]).and_then(|v| v.arr()), "modulus_exponents")
        .iter()
        .map(|e| 1u128 << need(e.u128(), "exponent"))
        .fold(0, |x, y| x ^ y);
    let hashed = need(trec.path(&["target", "hashed"]), "target.hashed");
    let rec_deg = need(hashed.path(&["curve", "degree"]).and_then(|v| v.u128()), "hashed degree") as u32;
    let f_rec: u128 = need(hashed.path(&["curve", "irreducible_low_terms"]).and_then(|v| v.arr()), "low terms")
        .iter()
        .map(|e| 1u128 << need(e.u128(), "low term"))
        .fold(1u128 << rec_deg, |x, y| x ^ y);
    let r = need(rp.path(&["curve", "subgroup_order"]).and_then(|v| v.u128()), "subgroup_order");
    let cof = need(rp.path(&["curve", "cofactor"]).and_then(|v| v.u128()), "cofactor");
    let gx = need(rp.path(&["curve", "generator", "0"]).and_then(|v| v.u128()), "G.x") as u64;
    let gy = need(rp.path(&["curve", "generator", "1"]).and_then(|v| v.u128()), "G.y") as u64;
    let grx = need(hashed.path(&["curve", "generator", "0"]).and_then(|v| v.u128()), "record G.x") as u64;
    let gry = need(hashed.path(&["curve", "generator", "1"]).and_then(|v| v.u128()), "record G.y") as u64;
    let qrx = need(trec.path(&["target", "point", "0"]).and_then(|v| v.u128()), "record Q.x") as u64;
    let qry = need(trec.path(&["target", "point", "1"]).and_then(|v| v.u128()), "record Q.y") as u64;
    let coeffs: Vec<u128> = need(rp.path(&["curve", "coefficients"]).and_then(|v| v.arr()), "coefficients")
        .iter()
        .map(|c| need(c.u128(), "coefficient"))
        .collect();
    let reg_order = need(ent.get("order").and_then(|v| v.u128()), "registry order");
    let reg_trace = need(ent.get("trace").and_then(|v| v.i128()), "registry trace");

    ck("modulus_encodings_agree", f == f_exp && f == f_rec);
    ck(
        "degree_agrees",
        deg(f) as u32 == n && Some(n as u128) == ent.path(&["params", "n"]).and_then(|v| v.u128()) && rec_deg == n,
    );
    ck(
        "a_agrees",
        Some(a as u128) == ent.path(&["params", "a"]).and_then(|v| v.u128())
            && Some(a as u128) == hashed.path(&["curve", "curve_a"]).and_then(|v| v.u128()),
    );
    ck("registry_b_is_1", coeffs == vec![1, a as u128, 0, 0, 1]);
    ck("generator_agrees_with_target_record", (gx, gy) == (grx, gry));
    ck(
        "subgroup_order_agrees_with_target_record",
        Some(r) == hashed.path(&["curve", "subgroup_order"]).and_then(|v| v.u128()),
    );
    ck(
        "target_seed_agrees",
        Some(seed as u128) == trec.path(&["target", "record", "public_hash_seed"]).and_then(|v| v.u128()),
    );

    let fld = Field { f, n };
    let e = Curve { f: fld, a: a as u64 };
    ck("modulus_irreducible", fld.irreducible());
    let (order, s_n) = e.order();
    let trace = s_n; // 2^n + 1 - #E
    ck("order_matches_registry", order == reg_order);
    ck("trace_matches_registry", trace == reg_trace);
    ck("r_prime", is_prime(r));
    ck("r_times_cofactor_is_order", r * cof == order);
    let gpt: Pt = Some((gx, gy));
    ck("G_on_curve", e.on(gpt));
    ck("G_exact_order_r", gpt.is_some() && e.mul(r, gpt).is_none());
    // Frobenius eigenvalue: lambda^2 - t1 lambda + 2 = 0 mod r
    let t1: i128 = if a == 0 { -1 } else { 1 };
    let disc = ((t1 * t1 - 8).rem_euclid(r as i128)) as u128;
    let sq = need(sqrt_mod(disc, r), "sqrt of t1^2 - 8 mod r");
    let inv2 = powmod(2, r - 2, r);
    let t1m = (t1.rem_euclid(r as i128)) as u128;
    let roots = [mulmod((t1m + sq) % r, inv2, r), mulmod((t1m + r - sq) % r, inv2, r)];
    let frob_g: Pt = Some((fld.sqr(gx), fld.sqr(gy)));
    let lam_ok: Vec<u128> = roots.iter().copied().filter(|&l| e.mul(l, gpt) == frob_g).collect();
    ck("frobenius_eigenvalue_found", lam_ok.len() == 1);

    let tgt = public_hash_target(&e, n, a, seed, cof, r);
    let qpt: Pt = Some(tgt.q);
    ck("Q_derived_equals_recorded", tgt.q == (qrx, qry));
    ck(
        "Q_counter_equals_recorded",
        Some(tgt.counter as u128) == trec.path(&["target", "record", "public_hash_counter"]).and_then(|v| v.u128()),
    );
    ck("Q_on_curve", e.on(qpt));
    ck("Q_in_subgroup", tgt.in_subgroup);

    let mut ds: Vec<u128> = Vec::new();
    let mut meta_bad: Vec<String> = Vec::new();
    for p in &recovered {
        let d = load(p);
        let rec = d.get("recovered").and_then(|v| v.arr());
        let ok = d.get("status").and_then(|v| v.str()) == Some("complete")
            && d.get("all_verified").and_then(|v| v.bool()) == Some(true)
            && d.get("curve").and_then(|v| v.str()) == Some(slug.as_str())
            && d.get("n").and_then(|v| v.u128()) == Some(n as u128)
            && d.get("a").and_then(|v| v.u128()) == Some(a as u128)
            && d.get("r").and_then(|v| v.u128()) == Some(r)
            && rec.map(|v| v.len()) == Some(1);
        if ok {
            ds.push(need(rec.and_then(|v| v[0].u128()), "recovered[0]"));
        } else {
            meta_bad.push(p.clone());
        }
    }
    ds.sort();
    ds.dedup();
    ck("all_recovered_records_meta_ok", meta_bad.is_empty());
    ck("one_distinct_recovered_scalar", ds.len() == 1);
    let scal: Vec<(u128, bool, bool)> = ds.iter().map(|&d| (d, 1 <= d && d < r, e.mul(d, gpt) == qpt)).collect();
    ck("every_scalar_dG_equals_Q", !scal.is_empty() && scal.iter().all(|s| s.1 && s.2));
    if let Some(&(d, _, _)) = scal.first() {
        ck("negative_control_d_plus_1_rejected", e.mul(d + 1, gpt) != qpt);
    }
    let other = public_hash_target(&e, n, a, seed + 1, cof, r).q;
    ck("negative_control_seed_plus_1_gives_other_Q", other != tgt.q && e.on(Some(other)));

    let all_ok = checks.iter().all(|c| c.1);
    let mut o = String::from("{\n");
    o += &format!(" \"tool\": \"verify_scalars.rs\",\n \"slug\": {},\n", jstr(&slug));
    o += &format!(
        " \"curve\": {{\"n\": {n}, \"a\": {a}, \"modulus_hex\": \"{f:#x}\", \"order\": {order}, \"trace\": {trace}, \"subgroup_order\": {r}, \"cofactor\": {cof}, \"generator\": [\"{gx:#x}\", \"{gy:#x}\"], \"frobenius_lambda\": {}}},\n",
        lam_ok.first().map(|l| l.to_string()).unwrap_or("null".into())
    );
    o += &format!(
        " \"target\": {{\"public_hash_seed\": {seed}, \"counter\": {}, \"hash_message_len\": {}, \"digest\": \"{}\", \"Q_dec\": [\"{}\", \"{}\"], \"Q_recorded_dec\": [\"{qrx}\", \"{qry}\"]}},\n",
        tgt.counter, tgt.msg_len, hex(&tgt.digest), tgt.q.0, tgt.q.1
    );
    o += &format!(" \"recovered_records\": {},\n \"records_failing_meta\": [{}],\n", recovered.len(),
        meta_bad.iter().map(|s| jstr(s)).collect::<Vec<_>>().join(", "));
    o += &format!(
        " \"scalars\": [{}],\n",
        scal.iter().map(|s| format!("{{\"d\": {}, \"in_range\": {}, \"dG_equals_Q\": {}}}", s.0, s.1, s.2)).collect::<Vec<_>>().join(", ")
    );
    o += " \"checks\": {\n";
    o += &checks.iter().map(|(k, v)| format!("  {}: {v}", jstr(k))).collect::<Vec<_>>().join(",\n");
    o += &format!("\n }},\n \"all_ok\": {all_ok}\n}}\n");
    print!("{o}");
    exit(if all_ok { 0 } else { 1 });
}
