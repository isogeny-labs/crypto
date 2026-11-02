//! E32 validation (independent validator, 2026-11-01).
//!
//! An independent recompute of the headline numbers of the E32 handover ladder
//! (`research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.34 pre-registration, §9.36 results) from the raw records in
//! `experiments/36_handover_ladder/`, a leaf-by-leaf comparison of a fresh
//! replay with the committed records, and the gate files.  Standard library
//! only: its own JSON parser and writer, SHA-256, random generator
//! (xoshiro256**), least squares and bootstrap.  It shares no code with
//! `examples/e32_ladder.rs`, `examples/e32_tables.rs` or `crypto_lib`, and its
//! resampling uses its own generator and seeds (not E32's SplitMix64, seed 32).
//!
//! ```text
//! rustc -O --edition 2021 validate_e32.rs -o validate_e32
//! validate_e32 compare     <repo> <evidence-dir>   # -> replay/replay_comparison.json
//! validate_e32 report      <repo> <evidence-dir>   # -> validator-report.json
//! validate_e32 certificate <repo> <evidence-dir>   # -> replay-certificate.json
//! validate_e32 gate        <repo> <evidence-dir>   # -> VALIDATION.json
//! ```
//!
//! `<evidence-dir>` is relative to `<repo>` (here
//! `research/e32_handover_ladder_20261031`).

use std::fs;
use std::process::Command;

// ---------------------------------------------------------------- JSON ----

#[derive(Clone, Debug, PartialEq)]
enum J {
    Null,
    Bool(bool),
    /// A number kept as its source text (64-bit integers and floats survive).
    Num(String),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

static JNULL: J = J::Null;

struct Rd<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Rd<'a> {
    fn fail<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("{m} at byte {}", self.i))
    }
    fn skip(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn eat(&mut self, c: u8) -> Result<(), String> {
        if self.b.get(self.i) == Some(&c) {
            self.i += 1;
            Ok(())
        } else {
            self.fail(&format!("expected '{}'", c as char))
        }
    }
    fn word(&mut self, w: &str, v: J) -> Result<J, String> {
        if self.b[self.i..].starts_with(w.as_bytes()) {
            self.i += w.len();
            Ok(v)
        } else {
            self.fail("bad literal")
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        if self.i + 4 > self.b.len() {
            return self.fail("short \\u escape");
        }
        let s = std::str::from_utf8(&self.b[self.i..self.i + 4]).map_err(|e| e.to_string())?;
        let v = u32::from_str_radix(s, 16).map_err(|e| e.to_string())?;
        self.i += 4;
        Ok(v)
    }
    fn string(&mut self) -> Result<String, String> {
        self.eat(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = match self.b.get(self.i) {
                Some(c) => *c,
                None => return self.fail("unterminated string"),
            };
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(out).map_err(|e| e.to_string()),
                b'\\' => {
                    let e = match self.b.get(self.i) {
                        Some(e) => *e,
                        None => return self.fail("bad escape"),
                    };
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
                            let hi = self.hex4()?;
                            let cp = if (0xD800..0xDC00).contains(&hi) {
                                self.eat(b'\\')?;
                                self.eat(b'u')?;
                                let lo = self.hex4()?;
                                0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)
                            } else {
                                hi
                            };
                            let ch = match char::from_u32(cp) {
                                Some(ch) => ch,
                                None => return self.fail("bad code point"),
                            };
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return self.fail("unknown escape"),
                    }
                }
                _ => out.push(c),
            }
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.skip();
        match self.b.get(self.i) {
            None => self.fail("unexpected end"),
            Some(b'{') => {
                self.i += 1;
                let mut kv = Vec::new();
                self.skip();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(J::Obj(kv));
                }
                loop {
                    self.skip();
                    let k = self.string()?;
                    self.skip();
                    self.eat(b':')?;
                    let v = self.value()?;
                    kv.push((k, v));
                    self.skip();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(J::Obj(kv));
                        }
                        _ => return self.fail("expected ',' or '}'"),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut a = Vec::new();
                self.skip();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(J::Arr(a));
                }
                loop {
                    a.push(self.value()?);
                    self.skip();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(J::Arr(a));
                        }
                        _ => return self.fail("expected ',' or ']'"),
                    }
                }
            }
            Some(b'"') => Ok(J::Str(self.string()?)),
            Some(b't') => self.word("true", J::Bool(true)),
            Some(b'f') => self.word("false", J::Bool(false)),
            Some(b'n') => self.word("null", J::Null),
            Some(_) => {
                let st = self.i;
                while self.i < self.b.len()
                    && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                {
                    self.i += 1;
                }
                if st == self.i {
                    return self.fail("unexpected character");
                }
                let t = std::str::from_utf8(&self.b[st..self.i]).unwrap().to_string();
                if t.parse::<f64>().is_err() {
                    return self.fail("bad number");
                }
                Ok(J::Num(t))
            }
        }
    }
}

fn parse(text: &str) -> Result<J, String> {
    let mut r = Rd { b: text.as_bytes(), i: 0 };
    let v = r.value()?;
    r.skip();
    if r.i != r.b.len() {
        return r.fail("trailing data");
    }
    Ok(v)
}

impl J {
    fn get(&self, k: &str) -> &J {
        if let J::Obj(kv) = self {
            for (a, v) in kv {
                if a == k {
                    return v;
                }
            }
        }
        &JNULL
    }
    fn at(&self, i: usize) -> &J {
        match self {
            J::Arr(v) if i < v.len() => &v[i],
            _ => &JNULL,
        }
    }
    fn arr(&self) -> &[J] {
        match self {
            J::Arr(v) => v,
            _ => &[],
        }
    }
    fn has(&self, k: &str) -> bool {
        matches!(self, J::Obj(kv) if kv.iter().any(|(a, _)| a == k))
    }
    fn f(&self) -> f64 {
        match self {
            J::Num(s) => s.parse::<f64>().unwrap_or(f64::NAN),
            _ => f64::NAN,
        }
    }
    fn u(&self) -> u64 {
        match self {
            J::Num(s) => match s.parse::<u64>() {
                Ok(v) => v,
                Err(_) => {
                    let x: f64 = s.parse().unwrap_or(f64::NAN);
                    if x.is_finite() && x >= 0.0 && x.fract() == 0.0 && x < 9.0e15 {
                        x as u64
                    } else {
                        panic!("not a non-negative integer: {s}")
                    }
                }
            },
            other => panic!("not a number: {other:?}"),
        }
    }
    fn s(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => "",
        }
    }
    fn bo(&self) -> Option<bool> {
        match self {
            J::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

fn esc(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_json(v: &J, ind: usize, out: &mut String) {
    let pad = |n: usize, out: &mut String| {
        for _ in 0..n {
            out.push(' ');
        }
    };
    match v {
        J::Null => out.push_str("null"),
        J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        J::Num(s) => out.push_str(s),
        J::Str(s) => esc(s, out),
        J::Arr(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (k, x) in a.iter().enumerate() {
                pad(ind + 1, out);
                write_json(x, ind + 1, out);
                if k + 1 < a.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            pad(ind, out);
            out.push(']');
        }
        J::Obj(kv) => {
            if kv.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push_str("{\n");
            for (k, (key, x)) in kv.iter().enumerate() {
                pad(ind + 1, out);
                esc(key, out);
                out.push_str(": ");
                write_json(x, ind + 1, out);
                if k + 1 < kv.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            pad(ind, out);
            out.push('}');
        }
    }
}

fn dump(v: &J) -> String {
    let mut s = String::new();
    write_json(v, 0, &mut s);
    s.push('\n');
    s
}

fn ju(v: u64) -> J {
    J::Num(v.to_string())
}
fn jf(v: f64) -> J {
    if v.is_finite() {
        let s = format!("{v}");
        J::Num(s)
    } else {
        J::Null
    }
}
fn js(v: &str) -> J {
    J::Str(v.to_string())
}
fn jb(v: bool) -> J {
    J::Bool(v)
}
fn obj(kv: Vec<(&str, J)>) -> J {
    J::Obj(kv.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
fn jfs(v: &[f64]) -> J {
    J::Arr(v.iter().map(|x| jf(*x)).collect())
}

// ------------------------------------------------------------- SHA-256 ----

fn sha256(data: &[u8]) -> [u8; 32] {
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
    let mut m = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bits.to_be_bytes());
    for ch in m.chunks(64) {
        let mut w = [0u32; 64];
        for t in 0..16 {
            w[t] = u32::from_be_bytes([ch[4 * t], ch[4 * t + 1], ch[4 * t + 2], ch[4 * t + 3]]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16].wrapping_add(s0).wrapping_add(w[t - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for t in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let chf = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(chf).wrapping_add(K[t]).wrapping_add(w[t]);
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
        for k in 0..8 {
            h[k] = h[k].wrapping_add(v[k]);
        }
    }
    let mut out = [0u8; 32];
    for k in 0..8 {
        out[4 * k..4 * k + 4].copy_from_slice(&h[k].to_be_bytes());
    }
    out
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn sha_file(path: &str) -> String {
    hex(&sha256(&fs::read(path).unwrap_or_else(|e| panic!("read {path}: {e}"))))
}

// ------------------------------------------------- random (xoshiro256**) ----

struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// State = SHA-256 of a label (so every stream is named, not tuned).
    fn named(label: &str) -> Rng {
        let h = sha256(label.as_bytes());
        let mut s = [0u64; 4];
        for k in 0..4 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&h[8 * k..8 * k + 8]);
            s[k] = u64::from_be_bytes(b);
        }
        if s == [0; 4] {
            s[0] = 1;
        }
        Rng { s }
    }
    fn next(&mut self) -> u64 {
        let r = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        r
    }
    /// Uniform in [0, m), unbiased (reject the low 2^64 mod m values).
    fn below(&mut self, m: u64) -> u64 {
        let t = 0u64.wrapping_sub(m) % m;
        loop {
            let r = self.next();
            if r >= t {
                return r % m;
            }
        }
    }
}

// ------------------------------------------------------------ helpers ----

const SIZES: [u64; 11] = [101, 149, 197, 269, 397, 557, 773, 1093, 1549, 2213, 3089];
const E25_SIZES: [u64; 5] = [101, 149, 197, 269, 397];
const GAE: f64 = 97.0;
const WEIGHTS: [(&str, f64); 3] = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)];
const EXP: &str = "experiments/36_handover_ladder";
const E27: &str = "experiments/32_walk_karatsuba";
const NOTE: &str = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md";
const PREREG: &str = "246460f094e98e51da97a7ae325f1d1ee2b049e2";
const RESULTS: &str = "8afc92781bc7f194282e0f3c4b11b81f07132481";
const BINARY_COMMIT: &str = "e757710521b28668c834d227588389abff164fc7";
const BOOT_R: usize = 10_000;
const BOOT_B: usize = 20;

struct Ctx {
    repo: String,
    ev: String,
}

impl Ctx {
    fn path(&self, rel: &str) -> String {
        format!("{}/{}", self.repo, rel)
    }
    fn load(&self, rel: &str) -> J {
        let p = self.path(rel);
        let t = fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {p}: {e}"));
        parse(&t).unwrap_or_else(|e| panic!("parse {p}: {e}"))
    }
    fn exists(&self, rel: &str) -> bool {
        fs::metadata(self.path(rel)).is_ok()
    }
    fn git(&self, args: &[&str]) -> (bool, String) {
        let o = Command::new("git").arg("-C").arg(&self.repo).args(args).output();
        match o {
            Ok(o) => (o.status.success(), String::from_utf8_lossy(&o.stdout).trim_end().to_string()),
            Err(e) => (false, e.to_string()),
        }
    }
    /// A path relative to the evidence directory (as the gate wants).
    fn from_ev(&self, rel: &str) -> String {
        let depth = self.ev.trim_end_matches('/').split('/').count();
        format!("{}{}", "../".repeat(depth), rel)
    }
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    }
}
fn vmin(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}
fn vmax(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}
fn rel(a: f64, b: f64) -> f64 {
    if a == b {
        0.0
    } else {
        (a - b).abs() / a.abs().max(b.abs())
    }
}
/// Percentile, linear interpolation between order statistics (type 7).
fn pct(sorted: &[f64], q: f64) -> f64 {
    let h = (sorted.len() - 1) as f64 * q;
    let lo = h.floor() as usize;
    let fr = h - lo as f64;
    if lo + 1 >= sorted.len() || fr == 0.0 {
        return sorted[lo];
    }
    let (a, b) = (sorted[lo], sorted[lo + 1]);
    if b.is_infinite() {
        return b;
    }
    a + fr * (b - a)
}
fn epoch(ts: &str) -> i64 {
    let n = |a: usize, b: usize| ts[a..b].parse::<i64>().unwrap();
    let (y, m, d) = (n(0, 4), n(5, 7), n(8, 10));
    let (y2, m2) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = y2.div_euclid(400);
    let yoe = y2 - era * 400;
    let doy = (153 * m2 + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    days * 86400 + n(11, 13) * 3600 + n(14, 16) * 60 + n(17, 19)
}
fn section(text: &str, head: &str) -> String {
    let mut out = String::new();
    let mut on = false;
    for line in text.split_inclusive('\n') {
        if line.starts_with(head) {
            on = true;
        } else if on && line.starts_with("### ") {
            break;
        }
        if on {
            out.push_str(line);
        }
    }
    out
}

// ------------------------------------------------------------ records ----

#[derive(Clone)]
struct Curve {
    p: u64,
    seed: u64,
    n: u64,
    log2n: f64,
    x: f64,
    s: u64,
    e: u64,
    rho: f64,
    steps: f64,
    residuals: u64,
    relations: u64,
    cols: u64,
    cmed: f64,
    y: f64,
    epr: f64,
    solve_share: f64,
    weil_solve_share: f64,
    la_share: f64,
    steps_sqrt_n: f64,
    per_step: f64,
    sg: [f64; 3],
    rg: [f64; 3],
    d: [f64; 3],
    recovered: u64,
    checks: Vec<(String, bool)>,
}

fn ic_rel(p: u64, s: u64) -> String {
    format!("{EXP}/ic_p{p}s{s}.json")
}
fn walk_rel(p: u64, s: u64) -> String {
    format!("{EXP}/walk_p{p}s{s}.json")
}

/// Mean over the 16 walks of (fp_muls - verify_fp_muls) and of steps, exact sums.
fn walk_means(w: &J) -> (u64, u64, usize) {
    let mut cost = 0u64;
    let mut steps = 0u64;
    for r in w.get("runs").arr() {
        cost += r.get("fp_muls").u() - r.get("verify_fp_muls").u();
        steps += r.get("steps").u();
    }
    (cost, steps, w.get("runs").arr().len())
}

fn curve(ctx: &Ctx, p: u64, seed: u64) -> Curve {
    let ic = ctx.load(&ic_rel(p, seed));
    let w = ctx.load(&walk_rel(p, seed));
    let mut ck: Vec<(String, bool)> = Vec::new();
    let mut c = |name: &str, ok: bool| ck.push((name.to_string(), ok));
    let n = ic.get("n").u();
    let s = ic.get("total_muls").u();
    let e = ic.get("macaulay_entries").u();
    let comps = [
        "base_muls", "check_muls", "generation_muls", "la_fp_muls", "lift_muls",
        "precompute_muls", "solve_muls", "weil_muls",
    ];
    let sum: u64 = comps.iter().map(|k| ic.get(k).u()).sum();
    c("S = sum of its eight charged phases", sum == s);
    c("ic p, seed match the file name", ic.get("p").u() == p && ic.get("seed").u() == seed);
    c("walk p, seed match the file name", w.get("p").u() == p && w.get("seed").u() == seed);
    c("ic arm B, chain, Karatsuba, psi-folded base, matched fold Gls",
        ic.get("arm").s() == "B" && ic.get("solver").s() == "chain"
            && ic.get("karatsuba").bo() == Some(true) && ic.get("fp4_product").s() == "karatsuba"
            && ic.get("folded").get("arm").s() == "v4-psi-folded" && ic.get("matched_fold").s() == "Gls");
    c("walk is E27 walk (iii): fold Gls, Karatsuba + batched, pool 16, 16 walks",
        w.get("fold").s() == "Gls" && w.get("arithmetic").s().contains("(iii)")
            && w.get("pool").u() == 16 && w.get("walks").u() == 16 && w.get("runs").arr().len() == 16
            && w.get("batch").get("pool").u() == 16);
    let seeds_ok = w.get("runs").arr().iter().enumerate().all(|(k, r)| r.get("seed").u() == 1000 * seed + k as u64);
    c("walk seeds are 1000*seed + k, k < 16", seeds_ok);
    c("cofactor h = 16 in both records", ic.get("cofactor").u() == 16 && w.get("cofactor").u() == 16);
    c("same n in both records", w.get("n").u() == n);
    let folded = ic.get("folded");
    c("IC solved and correct (logarithm checked in the binary)",
        folded.get("solved").bo() == Some(true) && folded.get("correct").bo() == Some(true));
    c("every kept row re-added (rows_readded = relations, 0 failures)",
        ic.get("rows_readd_failures").u() == 0 && ic.get("rows_readded").u() == folded.get("relations").u());
    c("check_failures = 0", ic.get("check_failures").u() == 0);
    c("residual cap 40,000,000 not reached", ic.get("residuals").u() < 40_000_000);
    let e25 = E25_SIZES.contains(&p) && seed <= 2;
    let rr = ic.get("rho").arr().len();
    c("pipeline walks as registered (48 at E25 curves, 3 elsewhere)", rr == if e25 { 48 } else { 3 });
    let recovered = folded.get("recovered").u();
    let walks_ok = w.get("runs").arr().iter().all(|r| r.get("correct").bo() == Some(true));
    let walks_same = w.get("runs").arr().iter().all(|r| r.get("recovered").u() == recovered);
    c("all 16 walks correct (all_correct and per walk)", walks_ok && w.get("all_correct").bo() == Some(true));
    c("every walk recovers the IC's logarithm (two algorithms agree)", walks_same);
    let (cost, steps_sum, k) = walk_means(&w);
    let rho = cost as f64 / k as f64;
    let steps = steps_sum as f64 / k as f64;
    c("rho_iii_mean_fp_muls = mean(fp_muls - verify_fp_muls)", w.get("rho_iii_mean_fp_muls").f() == rho);
    c("mean_steps = mean(steps)", w.get("mean_steps").f() == steps);
    let log2n = (n as f64).log2();
    let mut sg = [0.0; 3];
    let mut rg = [0.0; 3];
    let mut d = [0.0; 3];
    for (k, (_, g)) in WEIGHTS.iter().enumerate() {
        sg[k] = s as f64 + GAE * g * e as f64;
        rg[k] = rho + GAE * g * steps;
        d[k] = sg[k] / rg[k];
    }
    let residuals = ic.get("residuals").u();
    let relations = folded.get("relations").u();
    Curve {
        p,
        seed,
        n,
        log2n,
        x: log2n - 32.0,
        s,
        e,
        rho,
        steps,
        residuals,
        relations,
        cols: folded.get("columns").u(),
        cmed: ic.get("call").get("median").f(),
        y: residuals as f64 / relations as f64,
        epr: e as f64 / residuals as f64,
        solve_share: ic.get("solve_muls").u() as f64 / s as f64,
        weil_solve_share: (ic.get("solve_muls").u() + ic.get("weil_muls").u()) as f64 / s as f64,
        la_share: ic.get("la_fp_muls").u() as f64 / s as f64,
        steps_sqrt_n: steps / (n as f64).sqrt(),
        per_step: rho / steps,
        sg,
        rg,
        d,
        recovered,
        checks: ck,
    }
}

// ---------------------------------------------------------------- fits ----

#[derive(Clone, Copy)]
struct Fit {
    alpha: f64,
    beta: f64,
    se: f64,
    nstar: f64,
}

fn ols(pts: &[(f64, f64)]) -> Fit {
    let n = pts.len() as f64;
    let mx = pts.iter().map(|p| p.0).sum::<f64>() / n;
    let my = pts.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx) * (p.0 - mx)).sum();
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let beta = sxy / sxx;
    let alpha = my - beta * mx;
    let rss: f64 = pts.iter().map(|p| (p.1 - alpha - beta * p.0).powi(2)).sum();
    let se = (rss / (n - 2.0) / sxx).sqrt();
    let nstar = if beta < 0.0 { 32.0 - alpha / beta } else { f64::INFINITY };
    Fit { alpha, beta, se, nstar }
}

fn theory(pts: &[(f64, f64)]) -> Fit {
    let alpha = pts.iter().map(|p| p.1 + p.0 / 4.0).sum::<f64>() / pts.len() as f64;
    Fit { alpha, beta: -0.25, se: 0.0, nstar: 32.0 + 4.0 * alpha }
}

#[derive(Clone, Copy, PartialEq)]
enum Model {
    M1,
    M1All,
    M2,
}

impl Model {
    fn name(self) -> &'static str {
        match self {
            Model::M1 => "M1",
            Model::M1All => "M1_all",
            Model::M2 => "M2",
        }
    }
    fn pmin(self) -> u64 {
        if self == Model::M1All {
            0
        } else {
            269
        }
    }
    fn fit(self, pts: &[(f64, f64)]) -> Fit {
        if self == Model::M2 {
            theory(pts)
        } else {
            ols(pts)
        }
    }
}

const MODELS: [Model; 3] = [Model::M1, Model::M1All, Model::M2];

struct Boot {
    /// Per batch: [alpha_lo, alpha_hi, beta_lo, beta_hi, nstar_lo, nstar_hi].
    batches: Vec<[f64; 6]>,
    frac_beta_nonneg: f64,
    replicates: usize,
}

fn bootstrap(curves: &[Curve], model: Model, gk: usize, label: &str) -> Boot {
    let strata: Vec<Vec<(f64, f64)>> = SIZES
        .iter()
        .filter(|p| **p >= model.pmin())
        .map(|p| {
            curves.iter().filter(|c| c.p == *p).map(|c| (c.x, c.d[gk].log2())).collect()
        })
        .collect();
    let mut batches = Vec::new();
    let mut nonneg = 0usize;
    for b in 0..BOOT_B {
        let mut rng = Rng::named(&format!("{label}:batch{b}"));
        let mut al = Vec::with_capacity(BOOT_R);
        let mut be = Vec::with_capacity(BOOT_R);
        let mut ns = Vec::with_capacity(BOOT_R);
        let mut pts = Vec::new();
        for _ in 0..BOOT_R {
            pts.clear();
            for st in &strata {
                for _ in 0..st.len() {
                    pts.push(st[rng.below(st.len() as u64) as usize]);
                }
            }
            let f = model.fit(&pts);
            if f.beta >= 0.0 {
                nonneg += 1;
            }
            al.push(f.alpha);
            be.push(f.beta);
            ns.push(f.nstar);
        }
        for v in [&mut al, &mut be, &mut ns] {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        batches.push([
            pct(&al, 0.025),
            pct(&al, 0.975),
            pct(&be, 0.025),
            pct(&be, 0.975),
            pct(&ns, 0.025),
            pct(&ns, 0.975),
        ]);
    }
    Boot { batches, frac_beta_nonneg: nonneg as f64 / (BOOT_B * BOOT_R) as f64, replicates: BOOT_R }
}

// ----------------------------------------------------------- headlines ----

struct Heads {
    v: Vec<J>,
    bad: Vec<String>,
}

/// Half a unit of the last printed digit of `printed` (e.g. "2.07e4" -> 50).
fn half_unit(printed: &str) -> f64 {
    let t = printed.trim_start_matches('-');
    let (mant, ex) = match t.find(['e', 'E']) {
        Some(k) => (&t[..k], t[k + 1..].parse::<i32>().unwrap()),
        None => (t, 0),
    };
    let dec = mant.find('.').map(|k| mant.len() - k - 1).unwrap_or(0) as i32;
    0.5 * 10f64.powi(ex - dec)
}

impl Heads {
    fn push(&mut self, name: &str, source: &str, printed: &str, claimed: f64, recomputed: f64, abs_tol: f64, definition: &str) {
        let ok = (claimed - recomputed).abs() <= abs_tol * (1.0 + 1e-12) + 1e-300;
        let scale = claimed.abs().max(recomputed.abs()).max(1e-300);
        let rel_tol = if abs_tol == 0.0 { 1e-15 } else { abs_tol / scale * (1.0 + 1e-9) };
        if !ok {
            self.bad.push(format!("{name}: claimed {claimed} ({printed}), recomputed {recomputed}"));
        }
        self.v.push(obj(vec![
            ("name", js(name)),
            ("source", js(source)),
            ("printed", js(printed)),
            ("claimed", jf(claimed)),
            ("recomputed", jf(recomputed)),
            ("rel_tol", jf(rel_tol)),
            ("agrees", jb(ok)),
            ("definition", js(definition)),
        ]));
    }
    /// A printed figure: agrees within half a unit of its last digit (plus `extra`).
    fn printed(&mut self, name: &str, source: &str, printed: &str, recomputed: f64, extra: f64, definition: &str) {
        let claimed: f64 = printed.parse().unwrap();
        self.push(name, source, printed, claimed, recomputed, half_unit(printed) + extra, definition);
    }
    /// A full-precision recorded value: agrees within a relative 1e-9.
    fn exact(&mut self, name: &str, source: &str, claimed: f64, recomputed: f64, definition: &str) {
        let tol = 1e-9 * claimed.abs().max(recomputed.abs());
        self.push(name, source, &format!("{claimed}"), claimed, recomputed, tol, definition);
    }
}

// ---------------------------------------------------------- comparison ----

struct Diff {
    compared: usize,
    identical: usize,
    mismatches: Vec<J>,
    timing: Vec<J>,
}

fn is_timing(key: &str) -> bool {
    matches!(key, "wall_ms" | "solve_ms" | "wall_s")
}

fn num_same(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let fa = a.contains(['.', 'e', 'E']);
    let fb = b.contains(['.', 'e', 'E']);
    fa == fb && a.parse::<f64>().ok() == b.parse::<f64>().ok()
}

/// Every leaf of `a` (the committed record) against `b`; keys in `skip` and
/// timing keys are not compared (timing keys are listed).
fn leaves(path: &str, a: &J, b: &J, skip: &[&str], d: &mut Diff) {
    match a {
        J::Obj(kv) if !kv.is_empty() => {
            for (k, v) in kv {
                let p = format!("{path}.{k}");
                if is_timing(k) {
                    d.timing.push(obj(vec![("path", js(&p)), ("committed", v.clone()), ("replay", b.get(k).clone())]));
                    continue;
                }
                if skip.contains(&k.as_str()) {
                    continue;
                }
                if !b.has(k) {
                    d.compared += 1;
                    d.mismatches.push(obj(vec![("path", js(&p)), ("committed", v.clone()), ("other", js("<missing>"))]));
                    continue;
                }
                leaves(&p, v, b.get(k), skip, d);
            }
        }
        J::Arr(xs) if !xs.is_empty() => {
            let ys = b.arr();
            for (i, x) in xs.iter().enumerate() {
                let p = format!("{path}[{i}]");
                if i >= ys.len() {
                    d.compared += 1;
                    d.mismatches.push(obj(vec![("path", js(&p)), ("committed", x.clone()), ("other", js("<missing>"))]));
                } else {
                    leaves(&p, x, &ys[i], skip, d);
                }
            }
            if ys.len() > xs.len() {
                d.compared += 1;
                d.mismatches.push(obj(vec![("path", js(path)), ("committed", ju(xs.len() as u64)), ("other", ju(ys.len() as u64))]));
            }
        }
        _ => {
            d.compared += 1;
            let same = match (a, b) {
                (J::Num(x), J::Num(y)) => num_same(x, y),
                _ => a == b,
            };
            if same {
                d.identical += 1;
            } else if d.mismatches.len() < 50 {
                d.mismatches.push(obj(vec![("path", js(path)), ("committed", a.clone()), ("other", b.clone())]));
            } else {
                d.mismatches.push(js("..."));
            }
        }
    }
}

fn new_diff() -> Diff {
    Diff { compared: 0, identical: 0, mismatches: Vec::new(), timing: Vec::new() }
}

fn selection(ctx: &Ctx) -> (String, Vec<(u64, u64)>) {
    let nj = ctx.load(&format!("{}/replay/nonce.json", ctx.ev));
    let nonce = nj.get("nonce").s().to_string();
    let sel = SIZES
        .iter()
        .map(|p| {
            let h = sha256(format!("{nonce}:{p}").as_bytes());
            let mut b = [0u8; 8];
            b.copy_from_slice(&h[..8]);
            (*p, 1 + u64::from_be_bytes(b) % 3)
        })
        .collect();
    (nonce, sel)
}

fn compare(ctx: &Ctx) -> J {
    let (nonce, sel) = selection(ctx);
    let mut runs = Vec::new();
    let (mut tc, mut ti, mut nmatch, mut ntot) = (0usize, 0usize, 0usize, 0usize);
    let mut scalars_ok = true;
    // Required: the nonce-selected IC run of every size and every walk pool.
    // Extension: the other IC runs (phase 2), compared when present.
    let mut plan: Vec<(String, u64, u64, bool)> = Vec::new();
    for (p, s) in &sel {
        for t in 1..=3 {
            plan.push(("ic".into(), *p, t, t == *s));
        }
        for w in 1..=3 {
            plan.push(("walk".into(), *p, w, true));
        }
    }
    let mut missing = Vec::new();
    let (mut req, mut req_ok, mut ext, mut ext_ok) = (0usize, 0usize, 0usize, 0usize);
    for (kind, p, s, required) in &plan {
        let name = format!("{kind}_p{p}s{s}.json");
        let rrel = format!("{}/replay/runs/{name}", ctx.ev);
        let crel = format!("{EXP}/{name}");
        if *required {
            req += 1;
        }
        if !ctx.exists(&rrel) {
            if *required {
                missing.push(js(&name));
            }
            continue;
        }
        if !*required {
            ext += 1;
        }
        ntot += 1;
        let a = ctx.load(&crel);
        let b = ctx.load(&rrel);
        let mut d = new_diff();
        leaves("$", &a, &b, &[], &mut d);
        // Leaves only in the replay would be extra fields: count them as mismatches.
        let mut back = new_diff();
        leaves("$", &b, &a, &[], &mut back);
        let extra = back.mismatches.iter().filter(|m| m.get("other").s() == "<missing>").count();
        let matched = d.compared == d.identical && extra == 0;
        tc += d.compared;
        ti += d.identical;
        if matched {
            nmatch += 1;
            if *required {
                req_ok += 1;
            } else {
                ext_ok += 1;
            }
        }
        let ic_c = ctx.load(&ic_rel(*p, *s));
        let mut sc = Vec::new();
        if kind == "ic" {
            let f = b.get("folded");
            let ok = f.get("solved").bo() == Some(true) && f.get("correct").bo() == Some(true)
                && f.get("recovered").u() == ic_c.get("folded").get("recovered").u();
            let wc = ctx.load(&walk_rel(*p, *s));
            let walks_agree = wc.get("runs").arr().iter().all(|r| r.get("recovered").u() == f.get("recovered").u());
            scalars_ok &= ok && walks_agree;
            sc.push(("ic_solved", jb(f.get("solved").bo() == Some(true))));
            sc.push(("ic_correct", jb(f.get("correct").bo() == Some(true))));
            sc.push(("ic_recovered", ju(f.get("recovered").u())));
            sc.push(("equals_committed_walks_logarithm", jb(walks_agree)));
            sc.push(("total_muls", ju(b.get("total_muls").u())));
            sc.push(("macaulay_entries", ju(b.get("macaulay_entries").u())));
        } else {
            let rs = b.get("runs").arr();
            let all = rs.iter().all(|r| r.get("correct").bo() == Some(true));
            let lg = ic_c.get("folded").get("recovered").u();
            let same = rs.iter().all(|r| r.get("recovered").u() == lg);
            scalars_ok &= all && same && rs.len() == 16;
            let (cost, st, k) = walk_means(&b);
            sc.push(("walks", ju(rs.len() as u64)));
            sc.push(("all_correct", jb(all)));
            sc.push(("recover_the_committed_ic_logarithm", jb(same)));
            sc.push(("rho_iii_mean_fp_muls", jf(cost as f64 / k as f64)));
            sc.push(("mean_steps", jf(st as f64 / k as f64)));
        }
        runs.push(obj(vec![
            ("run", js(&format!("{kind}_p{p}s{s}"))),
            ("sample", js(if *required { "required (nonce selection or walk pool)" } else { "extension (phase 2)" })),
            ("committed", obj(vec![("path", js(&ctx.from_ev(&crel))), ("sha256", js(&sha_file(&ctx.path(&crel))))])),
            ("replay_output", obj(vec![("path", js(&format!("replay/runs/{name}"))), ("sha256", js(&sha_file(&ctx.path(&rrel))))])),
            ("fields_compared", ju(d.compared as u64)),
            ("fields_identical", ju(d.identical as u64)),
            ("fields_only_in_replay", ju(extra as u64)),
            ("mismatches", J::Arr(d.mismatches)),
            ("matched", jb(matched)),
            ("scalars", obj(sc)),
            ("timing_not_compared", J::Arr(d.timing)),
        ]));
    }
    obj(vec![
        ("schema", js("e32-replay-comparison/v1")),
        ("rule", js("every leaf of every committed record against the replay output: numbers by their text (equal text, or equal value in the same integer/float form), strings, booleans and nulls exactly, arrays by length and position, objects by key in both directions; wall_ms, solve_ms (ic) and wall_s (walk) are wall-clock and listed, not compared")),
        ("nonce", js(&nonce)),
        ("selection", J::Arr(sel.iter().map(|(p, s)| obj(vec![("p", ju(*p)), ("ic_seed", ju(*s))])).collect())),
        ("recorded_runs", ju(plan.len() as u64)),
        ("required_runs", ju(req as u64)),
        ("required_matched", ju(req_ok as u64)),
        ("required_missing", J::Arr(missing)),
        ("extension_runs_present", ju(ext as u64)),
        ("extension_matched", ju(ext_ok as u64)),
        ("runs_present", ju(ntot as u64)),
        ("runs_matched", ju(nmatch as u64)),
        ("fields_compared", ju(tc as u64)),
        ("fields_identical", ju(ti as u64)),
        ("scalars_verified", jb(scalars_ok)),
        ("runs", J::Arr(runs)),
    ])
}

// -------------------------------------------------------------- report ----

fn report(ctx: &Ctx) -> J {
    let mut checks: Vec<J> = Vec::new();
    let mut fails: Vec<String> = Vec::new();
    let mut check = |name: &str, ok: bool, detail: String| {
        if !ok {
            fails.push(format!("{name}: {detail}"));
        }
        checks.push(obj(vec![("check", js(name)), ("holds", jb(ok)), ("detail", js(&detail))]));
    };
    let mut h = Heads { v: Vec::new(), bad: Vec::new() };
    let src = "note §9.36";

    // --- self test of the SHA-256 against the FIPS 180-2 vector.
    check("own SHA-256 reproduces the FIPS 180-2 'abc' vector",
        hex(&sha256(b"abc")) == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "sha256(\"abc\")".into());

    // --- raw data presence and hashes.
    let mut raw = Vec::new();
    let mut curves = Vec::new();
    for p in SIZES {
        for s in 1..=3 {
            for rel_ in [ic_rel(p, s), walk_rel(p, s)] {
                raw.push(obj(vec![("path", js(&ctx.from_ev(&rel_))), ("sha256", js(&sha_file(&ctx.path(&rel_))))]));
            }
            curves.push(curve(ctx, p, s));
        }
    }
    for extra in [
        format!("{EXP}/ic_p4337s1.json"),
        format!("{EXP}/run.log"),
        format!("{EXP}/run_ladder.sh"),
        format!("{EXP}/binary_commit.txt"),
        format!("{EXP}/summary.json"),
        format!("{EXP}/fit.json"),
        format!("{EXP}/tables.md"),
        format!("{E27}/stage_w.json"),
        format!("{E27}/summary.json"),
    ] {
        raw.push(obj(vec![("path", js(&ctx.from_ev(&extra))), ("sha256", js(&sha_file(&ctx.path(&extra))))]));
    }
    for p in E25_SIZES {
        for s in 1..=2 {
            let r = format!("{E27}/e2e_chain_karatsuba_p{p}s{s}.json");
            raw.push(obj(vec![("path", js(&ctx.from_ev(&r))), ("sha256", js(&sha_file(&ctx.path(&r))))]));
        }
    }
    let (clean, st) = ctx.git(&["status", "--porcelain", "--", EXP]);
    check("raw records committed and unmodified (git status of experiments/36_handover_ladder empty)",
        clean && st.is_empty(), if st.is_empty() { "clean".into() } else { st });
    let mut odd = Vec::new();
    for e in fs::read_dir(ctx.path(EXP)).unwrap() {
        let n = e.unwrap().file_name().to_string_lossy().to_string();
        if n.ends_with(".censored") || n.ends_with(".nocurve") || n.ends_with(".part") {
            odd.push(n);
        }
    }
    check("no censored, no-curve or partial record (every seed 1-3 gave a curve; no replacement)",
        odd.is_empty(), format!("{odd:?}"));

    // --- per-curve record checks.
    let mut per_curve_fail = Vec::new();
    for c in &curves {
        for (name, ok) in &c.checks {
            if !ok {
                per_curve_fail.push(format!("p={} s={}: {name}", c.p, c.seed));
            }
        }
    }
    let names: Vec<String> = curves[0].checks.iter().map(|x| x.0.clone()).collect();
    check(&format!("33 paired curves x {} record checks each", names.len()),
        per_curve_fail.is_empty() && curves.len() == 33,
        if per_curve_fail.is_empty() { names.join("; ") } else { per_curve_fail.join("; ") });
    let walks_total: usize = curves.len() * 16;

    // --- p = 4337: unpaired and excluded.
    let i4 = ctx.load(&format!("{EXP}/ic_p4337s1.json"));
    let i4ok = i4.get("folded").get("correct").bo() == Some(true) && i4.get("p").u() == 4337;
    let no_walk = !ctx.exists(&format!("{EXP}/walk_p4337s1.json")) && ctx.exists(&format!("{EXP}/walk_p4337s1.oom"));
    let summ = ctx.load(&format!("{EXP}/summary.json"));
    let fitj = ctx.load(&format!("{EXP}/fit.json"));
    let sum_has_4337 = summ.get("curves").arr().iter().any(|c| c.get("p").u() == 4337)
        || summ.get("per_size").arr().iter().any(|c| c.get("p").u() == 4337);
    let mut fit_has_4337 = false;
    for (g, _) in WEIGHTS {
        for m in MODELS {
            fit_has_4337 |= fitj.get(g).get(m.name()).get("sizes").arr().iter().any(|x| x.u() == 4337);
        }
    }
    check("p = 4337: IC record complete, walk pool absent (OOM marker), excluded from summary.json and every fit; this recompute uses the 33 paired curves only",
        i4ok && no_walk && !sum_has_4337 && !fit_has_4337 && curves.iter().all(|c| c.p != 4337),
        format!("ic correct {i4ok}, walk absent with .oom {no_walk}, in summary {sum_has_4337}, in fit sizes {fit_has_4337}"));
    let y4 = i4.get("residuals").u() as f64 / i4.get("folded").get("relations").u() as f64;
    h.printed("p=4337 S (unpaired)", src, "2.484e11", i4.get("total_muls").u() as f64, 0.0, "total_muls of ic_p4337s1.json");
    h.printed("p=4337 y (unpaired)", src, "1252", y4, 0.0, "residuals / relations");
    h.printed("p=4337 columns (unpaired)", src, "282", i4.get("folded").get("columns").u() as f64, 0.0, "folded.columns");

    // --- j-invariant probe: is <±psi> the largest fold on every instance?
    let probe_rel = format!("{}/replay/j_invariant_probe.json", ctx.ev);
    if ctx.exists(&probe_rel) {
        let pr = ctx.load(&probe_rel);
        let mut bad = Vec::new();
        let mut count = 0usize;
        for e in pr.arr() {
            let (p, s) = (e.get("p").u(), e.get("seed").u());
            count += 1;
            let ic = ctx.load(&ic_rel(p, s));
            let j_ok = e.get("j_in_fp").bo() == Some(false) && e.get("j_in_fp2").bo() == Some(true)
                && e.get("j_is_0").bo() == Some(false) && e.get("j_is_1728").bo() == Some(false);
            let inst_ok = e.get("n").u() == ic.get("n").u() && e.get("group_order").u() == ic.get("group_order").u()
                && e.get("cofactor").u() == 16 && e.get("planted_d").u() == ic.get("folded").get("recovered").u();
            if !(j_ok && inst_ok) {
                bad.push(format!("p={p} s={s}: j {j_ok}, instance {inst_ok}"));
            }
        }
        check("j-invariant probe (replay/j_invariant_probe.json; the validator's example calling the library's generator and F_p4 arithmetic): on all 34 instances j(E) lies in F_p2, not in F_p, and is neither 0 nor 1728, so Aut(E) = {±1} and <±psi> is the largest fold; n, #E, h = 16 and the planted d equal the records, and d is the recovered logarithm",
            bad.is_empty() && count == 34, if bad.is_empty() { format!("{count} instances") } else { bad.join("; ") });
    } else {
        check("j-invariant probe present", false, "replay/j_invariant_probe.json missing".into());
    }

    // --- pre-registration before the first run record.
    let (_, ct) = ctx.git(&["log", "-1", "--format=%ct", PREREG]);
    let prereg_t: i64 = ct.parse().unwrap_or(i64::MAX);
    let runlog = fs::read_to_string(ctx.path(&format!("{EXP}/run.log"))).unwrap();
    let first = runlog.lines().next().unwrap_or("");
    let first_t = epoch(&first[..20]);
    check("pre-registration commit 246460f094 predates the first run (run.log line 1)",
        prereg_t < first_t,
        format!("commit time {} (unix {prereg_t}) < first run {} (unix {first_t}), {} s earlier", ctx.git(&["log", "-1", "--format=%cI", PREREG]).1, &first[..20], first_t - prereg_t));
    let (_, tree) = ctx.git(&["ls-tree", "--name-only", PREREG, &format!("{EXP}/")]);
    let tree_files: Vec<&str> = tree.lines().collect();
    check("no run record exists in the pre-registration commit's tree",
        tree_files.iter().all(|f| f.ends_with("binary_commit.txt") || f.ends_with("run_ladder.py")),
        format!("{tree_files:?}"));
    let (anc1, _) = ctx.git(&["merge-base", "--is-ancestor", PREREG, RESULTS]);
    let (anc2, _) = ctx.git(&["merge-base", "--is-ancestor", RESULTS, "HEAD"]);
    check("pre-registration is an ancestor of the results commit 8afc92781b, which is in HEAD", anc1 && anc2, format!("{anc1} {anc2}"));
    let (_, a) = ctx.git(&["show", &format!("{PREREG}:{NOTE}")]);
    let (_, b) = ctx.git(&["show", &format!("HEAD:{NOTE}")]);
    let (sa, sb) = (section(&a, "### 9.34 "), section(&b, "### 9.34 "));
    check("§9.34 (frozen predictions G2.114-G2.123) byte-identical at the pre-registration and at HEAD",
        !sa.is_empty() && sa == sb, format!("{} bytes, sha256 {}", sa.len(), hex(&sha256(sa.as_bytes()))));
    let (_, l1) = ctx.git(&["rev-parse", &format!("{PREREG}:examples/e32_ladder.rs")]);
    let (_, l2) = ctx.git(&["rev-parse", "HEAD:examples/e32_ladder.rs"]);
    check("examples/e32_ladder.rs unchanged since the pre-registration", l1 == l2 && !l1.is_empty(), format!("blob {l1}"));
    let (bc_ok, _) = ctx.git(&["cat-file", "-e", &format!("{BINARY_COMMIT}^{{commit}}")]);
    let bct = fs::read_to_string(ctx.path(&format!("{EXP}/binary_commit.txt"))).unwrap();
    check("recorded binary commit resolves (binary_commit.txt prints it with one digit dropped: e75771052b28668c… for e757710521b28668c…; the 9-digit prefix e75771052 is unambiguous)",
        bc_ok && bct.contains("e75771052"), "erratum E1".into());

    // --- E27 identity (G2.114).
    let e27w = ctx.load(&format!("{E27}/stage_w.json"));
    let e27s = ctx.load(&format!("{E27}/summary.json"));
    let mut id_rows = Vec::new();
    let (mut idc, mut idi, mut wc, mut wi, mut ref_exact) = (0usize, 0usize, 0usize, 0usize, 0usize);
    for p in E25_SIZES {
        for s in 1..=2u64 {
            let e2e = ctx.load(&format!("{E27}/e2e_chain_karatsuba_p{p}s{s}.json"));
            let armb = e2e.at(1);
            let ours = ctx.load(&ic_rel(p, s));
            let mut d = new_diff();
            leaves("$", armb, &ours, &["experiment"], &mut d);
            idc += d.compared;
            idi += d.identical;
            let w = ctx.load(&walk_rel(p, s));
            let cur = e27w.get("curves").arr().iter().find(|c| c.get("p").u() == p && c.get("seed").u() == s).unwrap();
            let g = cur.get("folds").get("Gls");
            let theirs = g.get("walks").get("batched").arr();
            let mine = w.get("runs").arr();
            let (mut c, mut ok) = (0usize, 0usize);
            for (k, t) in theirs.iter().enumerate() {
                let m = mine.get(k).unwrap_or(&JNULL);
                for key in ["seed", "steps", "group_ops", "fp_muls", "verify_fp_muls", "verify_group_ops", "cycle_escapes", "recovered"] {
                    c += 1;
                    let same = match (t.get(key), m.get(key)) {
                        (J::Num(x), J::Num(y)) => x.parse::<u64>().is_ok() && x.parse::<u64>().ok() == y.parse::<u64>().ok(),
                        _ => false,
                    };
                    if same {
                        ok += 1;
                    }
                }
                c += 2;
                if t.get("correct") == m.get("correct") {
                    ok += 1;
                }
                let dig = u64::from_str_radix(m.get("visit_digest").s(), 16).ok();
                if dig == Some(t.get("visit_digest").u()) {
                    ok += 1;
                }
            }
            for key in ["batch_muls", "filler_walks", "inversions", "pool", "recorded_batch_muls", "recorded_inversions", "rounds"] {
                c += 1;
                if g.get("batch").get(key).u() == w.get("batch").get(key).u() {
                    ok += 1;
                }
            }
            wc += c;
            wi += ok;
            let row = e27s.get("rows").arr().iter().find(|r| r.get("p").u() == p && r.get("seed").u() == s && r.get("arm").s() == "B").unwrap();
            let (cost, st, k) = walk_means(&w);
            let rho_ok = row.get("rho_e27").f() == cost as f64 / k as f64 && row.get("walk_steps").f() == st as f64 / k as f64
                && g.get("mean_fp_muls").get("batched").f() == cost as f64 / k as f64;
            if rho_ok {
                ref_exact += 1;
            }
            id_rows.push(obj(vec![
                ("p", ju(p)), ("seed", ju(s)),
                ("ic_fields_compared", ju(d.compared as u64)), ("ic_fields_identical", ju(d.identical as u64)),
                ("ic_mismatches", J::Arr(d.mismatches)),
                ("walk_fields_compared", ju(c as u64)), ("walk_fields_identical", ju(ok as u64)),
                ("rho_e27_and_walk_steps_exact", jb(rho_ok)),
            ]));
        }
    }
    check("G2.114 identity: E32's ten E25-curve IC records equal E27's arm-B records leaf by leaf (wall_ms, solve_ms and the runner label `experiment` aside)",
        idc == idi && idc > 0, format!("{idi}/{idc} fields"));
    check("G2.114 identity: E32's walk pools on the ten E25 curves equal E27's walk (iii) records walk by walk (16 walks x 10 fields incl. visit digest) and pool by pool (7 batch fields)",
        wc == wi && wc > 0, format!("{wi}/{wc} fields"));
    check("G2.114 identity: rho_e27 and walk_steps of E27's summary reproduced exactly on the ten curves", ref_exact == 10, format!("{ref_exact}/10"));
    h.exact("G2.114 IC fields compared (6,040)", src, 6040.0, idc as f64, "leaves of E27's arm-B records minus wall_ms, solve_ms and `experiment`, ten curves");
    h.exact("G2.114 IC fields identical (6,040)", src, 6040.0, idi as f64, "of those, equal in E32's record");

    // --- recompute against summary.json (full precision).
    let mut maxrel: f64 = 0.0;
    let mut sum_bad = Vec::new();
    for c in &curves {
        let r = summ.get("curves").arr().iter().find(|x| x.get("p").u() == c.p && x.get("seed").u() == c.seed);
        let Some(r) = r else {
            sum_bad.push(format!("p={} s={} missing", c.p, c.seed));
            continue;
        };
        // summary.json's `solve_share` is (weil_muls + solve_muls) / S (erratum E2).
        let pairs = [
            ("S", c.s as f64), ("E", c.e as f64), ("rho", c.rho), ("steps", c.steps), ("y", c.y),
            ("C_median", c.cmed), ("entries_per_residual", c.epr), ("solve_share", c.weil_solve_share),
            ("la_share", c.la_share), ("log2n", c.log2n), ("steps_per_sqrt_n", c.steps_sqrt_n),
            ("muls_per_step", c.per_step), ("cols", c.cols as f64),
        ];
        for (k, v) in pairs {
            let e = rel(r.get(k).f(), v);
            maxrel = maxrel.max(e);
            if !(e <= 1e-12) {
                sum_bad.push(format!("p={} s={} {k}: {} vs {v}", c.p, c.seed, r.get(k).f()));
            }
        }
        for (k, (g, _)) in WEIGHTS.iter().enumerate() {
            let e = rel(r.get("D").get(g).f(), c.d[k]);
            maxrel = maxrel.max(e);
            if !(e <= 1e-12) {
                sum_bad.push(format!("p={} s={} D({g})", c.p, c.seed));
            }
        }
    }
    check("summary.json per-curve values (S, E, rho, steps, D(g) at 3 weights, y, C, E/residual, (Weil + solve) share [its `solve_share`, erratum E2], LA share, log2 n, steps/sqrt n, muls/step, columns) equal the recompute",
        sum_bad.is_empty() && summ.get("curves").arr().len() == 33,
        if sum_bad.is_empty() { format!("33 curves x 16 values, max relative difference {maxrel:.2e}") } else { sum_bad.join("; ") });

    // --- per size.
    let mut per_size = Vec::new();
    let mut med_d: Vec<[f64; 3]> = Vec::new();
    let note_table: [(u64, &str, &str, &str, &str, &str, &str, &str); 11] = [
        (101, "22.63", "1366", "2.07e4", "1.71e4", "3.93e4", "7.21e4", "3.83e5"),
        (149, "24.88", "4272", "4.03e4", "8.26e3", "1.52e5", "1.38e5", "6.59e5"),
        (197, "26.49", "969", "1.49e4", "1.07e4", "3.49e4", "5.09e4", "2.38e5"),
        (269, "28.29", "1918", "1.75e4", "8.83e3", "2.75e4", "5.90e4", "2.61e5"),
        (397, "30.53", "1349", "8.75e3", "6.39e3", "9.72e3", "2.91e4", "1.24e5"),
        (557, "32.49", "1180", "6.46e3", "4.98e3", "7.43e3", "2.13e4", "8.94e4"),
        (773, "34.38", "2268", "7.63e3", "6.52e3", "1.15e4", "2.50e4", "1.04e5"),
        (1093, "36.38", "1563", "4.37e3", "3.67e3", "5.06e3", "1.42e4", "5.84e4"),
        (1549, "38.39", "1392", "2.37e3", "2.07e3", "3.72e3", "7.66e3", "3.14e4"),
        (2213, "40.45", "1369", "1.61e3", "1.34e3", "2.05e3", "5.15e3", "2.10e4"),
        (3089, "42.37", "1496", "1.54e3", "1.14e3", "1.70e3", "4.93e3", "2.01e4"),
    ];
    let mut ps_bad = Vec::new();
    for (k, p) in SIZES.iter().enumerate() {
        let cs: Vec<&Curve> = curves.iter().filter(|c| c.p == *p).collect();
        let col = |f: &dyn Fn(&Curve) -> f64| -> Vec<f64> { cs.iter().map(|c| f(*c)).collect() };
        let l2 = mean(&col(&|c| c.log2n));
        let ymed = median(&col(&|c| c.y));
        let mut md = [0.0; 3];
        let mut sgm = [0.0; 3];
        let mut rgm = [0.0; 3];
        let mut dj = Vec::new();
        for (gk, (g, _)) in WEIGHTS.iter().enumerate() {
            let v = col(&|c| c.d[gk]);
            md[gk] = median(&v);
            sgm[gk] = median(&col(&|c| c.sg[gk]));
            rgm[gk] = median(&col(&|c| c.rg[gk]));
            dj.push((*g, obj(vec![("median", jf(md[gk])), ("min", jf(vmin(&v))), ("max", jf(vmax(&v))),
                ("median_S_g", jf(sgm[gk])), ("median_rho_g", jf(rgm[gk]))])));
            let sp = summ.get("per_size").arr().iter().find(|x| x.get("p").u() == *p).unwrap_or(&JNULL);
            let key = format!("D_{g}");
            for (stat, val) in [("median", md[gk]), ("min", vmin(&v)), ("max", vmax(&v))] {
                if !(rel(sp.get(&key).get(stat).f(), val) <= 1e-12) {
                    ps_bad.push(format!("p={p} {key}.{stat}"));
                }
            }
        }
        med_d.push(md);
        let t = note_table[k];
        assert_eq!(t.0, *p);
        let d0 = col(&|c| c.d[0]);
        h.printed(&format!("p={p} log2 n"), src, t.1, l2, 0.0, "mean over the size's three curves of log2 n");
        h.printed(&format!("p={p} y median"), src, t.2, ymed, 0.0, "median over 3 curves of residuals / relations");
        h.printed(&format!("p={p} D(0) median"), src, t.3, md[0], 0.0, "median over 3 curves of S / rho");
        h.printed(&format!("p={p} D(0) min"), src, t.4, vmin(&d0), 0.0, "min over 3 curves");
        h.printed(&format!("p={p} D(0) max"), src, t.5, vmax(&d0), 0.0, "max over 3 curves");
        h.printed(&format!("p={p} D(0.1) median"), src, t.6, md[1], 0.0, "median of (S + 9.7 E) / (rho + 9.7 steps)");
        h.printed(&format!("p={p} D(1) median"), src, t.7, md[2], 0.0, "median of (S + 97 E) / (rho + 97 steps)");
        per_size.push(obj(vec![
            ("p", ju(*p)), ("curves", ju(cs.len() as u64)), ("log2n_mean", jf(l2)), ("y_median", jf(ymed)),
            ("C_median_median", jf(median(&col(&|c| c.cmed)))),
            ("muls_per_step_median", jf(median(&col(&|c| c.per_step)))),
            ("D", J::Obj(dj.into_iter().map(|(g, v)| (g.to_string(), v)).collect())),
        ]));
    }
    check("summary.json per-size D medians, minima and maxima equal the recompute", ps_bad.is_empty(), format!("{ps_bad:?}"));

    // --- other printed figures of §9.36.
    let at = |pmin: u64| -> Vec<&Curve> { curves.iter().filter(|c| c.p >= pmin).collect() };
    let fold = |cs: &[&Curve], f: &dyn Fn(&Curve) -> f64| -> (f64, f64) {
        let v: Vec<f64> = cs.iter().map(|c| f(*c)).collect();
        (vmin(&v), vmax(&v))
    };
    let all = at(0);
    let c101: Vec<&Curve> = curves.iter().filter(|c| c.p == 101).collect();
    let c3089: Vec<&Curve> = curves.iter().filter(|c| c.p == 3089).collect();
    let med_of = |cs: &[&Curve], f: &dyn Fn(&Curve) -> f64| -> f64 { median(&cs.iter().map(|c| f(*c)).collect::<Vec<_>>()) };
    h.printed("C median at p=101", src, "5.99e5", med_of(&c101, &|c| c.cmed), 0.0, "median over 3 curves of call.median");
    h.printed("C median at p=3089", src, "6.65e5", med_of(&c3089, &|c| c.cmed), 0.0, "median over 3 curves of call.median");
    let (lo, hi) = fold(&all, &|c| c.epr);
    h.printed("Macaulay entries per residual, min (all curves)", src, "1.76e5", lo, 0.0, "E / residuals");
    h.printed("Macaulay entries per residual, max (all curves)", src, "1.82e5", hi, 0.0, "E / residuals");
    let (lo, hi) = fold(&all, &|c| 100.0 * c.weil_solve_share);
    h.printed("'solve' share of S, min (%)", src, "98.3", lo, 0.0, "(weil_muls + solve_muls) / S, which the note labels 'solve' (erratum E2; the solve alone is 97.69-98.45 %)");
    h.printed("'solve' share of S, max (%)", src, "99.1", hi, 0.0, "(weil_muls + solve_muls) / S (erratum E2)");
    let (_, la_hi) = fold(&all, &|c| 100.0 * c.la_share);
    h.printed("linear algebra share of S, max (%), prose", src, "0.006", la_hi, 0.0, "la_fp_muls / S");
    let (lo, hi) = fold(&c101, &|c| c.per_step);
    h.printed("walk F_p mults per step at p=101, min", src, "175", lo, 0.0, "rho / steps");
    h.printed("walk F_p mults per step at p=101, max", src, "229", hi, 0.0, "rho / steps");
    h.printed("walk F_p mults per step at p=3089 (median)", src, "95.9", med_of(&c3089, &|c| c.per_step), 0.0, "rho / steps");
    h.exact("paired curves", src, 33.0, curves.len() as f64, "ic + walk records present");
    h.exact("walk-(iii) walks", src, 528.0, walks_total as f64, "16 per paired curve");
    h.printed("ladder top log2 n (p=3089)", src, "42.37", med_of(&c3089, &|c| c.log2n), 0.0, "log2 n at p = 3089 (median of 3 curves)");

    // --- fits.
    let mut fits_j = Vec::new();
    let mut point: Vec<Vec<Fit>> = Vec::new();
    let mut boots: Vec<Vec<Boot>> = Vec::new();
    let (nonce, _) = selection(ctx);
    let note_fits: [(&str, Model, &str, &str, &str, &str, &str, &str); 9] = [
        ("0", Model::M1, "-0.243", "-0.279", "-0.205", "85.5", "79.2", "94.3"),
        ("0", Model::M1All, "-0.232", "-0.272", "-0.191", "87.7", "80.2", "98.6"),
        ("0", Model::M2, "", "", "", "84.0", "83.4", "84.6"),
        ("0.1", Model::M1, "-0.248", "-0.284", "-0.211", "91.2", "84.4", "100.7"),
        ("0.1", Model::M1All, "-0.239", "-0.278", "-0.197", "93.5", "85.4", "105.4"),
        // The note prints 90.9 for this point estimate; fit.json has 90.847 (erratum E4).
        ("0.1", Model::M2, "", "", "", "", "90.3", "91.4"),
        ("1", Model::M1, "-0.256", "-0.292", "-0.219", "97.6", "90.1", "107.9"),
        ("1", Model::M1All, "-0.256", "-0.295", "-0.216", "97.7", "89.6", "108.9"),
        ("1", Model::M2, "", "", "", "99.1", "98.5", "99.6"),
    ];
    let mut boot_rows = Vec::new();
    let mut boot_bad = Vec::new();
    let mut frac_all: f64 = 0.0;
    for (gk, (g, _)) in WEIGHTS.iter().enumerate() {
        let mut pf = Vec::new();
        let mut bs = Vec::new();
        let mut gj = Vec::new();
        for m in MODELS {
            let pts: Vec<(f64, f64)> = curves.iter().filter(|c| c.p >= m.pmin()).map(|c| (c.x, c.d[gk].log2())).collect();
            let f = m.fit(&pts);
            let label = format!("e32-validator-bootstrap:{nonce}:{g}:{}", m.name());
            let b = bootstrap(&curves, m, gk, &label);
            frac_all = frac_all.max(b.frac_beta_nonneg);
            let claimed = fitj.get(g).get(m.name());
            // point estimates against fit.json (deterministic).
            let tag = format!("{} {g} GAE", m.name());
            h.exact(&format!("{tag} alpha"), "fit.json", claimed.get("alpha").f(), f.alpha, "OLS intercept of log2 D on log2 n - 32 (M2: mean of L + x/4)");
            h.exact(&format!("{tag} log2 n*"), "fit.json", claimed.get("log2_nstar").f(), f.nstar, "32 - alpha/beta (M2: 32 + 4 alpha)");
            h.exact(&format!("{tag} curves"), "fit.json", claimed.get("n_curves").f(), pts.len() as f64, "curves in the fit");
            if m != Model::M2 {
                h.exact(&format!("{tag} beta"), "fit.json", claimed.get("beta").f(), f.beta, "OLS slope");
                h.exact(&format!("{tag} beta OLS se"), "fit.json", claimed.get("beta_se").f(), f.se, "sqrt(RSS/(n-2)/Sxx)");
                h.exact(&format!("{tag} beta OLS 95% lo"), "fit.json", claimed.get("beta_ols_ci95").at(0).f(), f.beta - 1.96 * f.se, "beta - 1.96 se");
                h.exact(&format!("{tag} beta OLS 95% hi"), "fit.json", claimed.get("beta_ols_ci95").at(1).f(), f.beta + 1.96 * f.se, "beta + 1.96 se");
            }
            // bootstrap endpoints: own resampling against fit.json within resampling noise.
            let keys = [("alpha", 0usize), ("beta", 2), ("log2_nstar", 4)];
            let mut ends = Vec::new();
            for (key, off) in keys {
                if m == Model::M2 && key == "beta" {
                    continue;
                }
                for side in 0..2 {
                    let vals: Vec<f64> = b.batches.iter().map(|x| x[off + side]).collect();
                    let (mu, sig) = (mean(&vals), sd(&vals));
                    let tol = 4.0 * sig * (1.0 + 1.0 / BOOT_B as f64).sqrt();
                    let cl = claimed.get("ci95").get(key).at(side).f();
                    let z = (cl - mu) / (sig * (1.0 + 1.0 / BOOT_B as f64).sqrt());
                    let ok = (cl - mu).abs() <= tol;
                    if !ok {
                        boot_bad.push(format!("{tag} {key}[{side}] claimed {cl}, own {mu} +- {sig}"));
                    }
                    ends.push(obj(vec![
                        ("quantity", js(key)), ("end", js(if side == 0 { "2.5%" } else { "97.5%" })),
                        ("claimed_fit_json", jf(cl)), ("own_mean_of_batches", jf(mu)), ("own_batch_sd", jf(sig)),
                        ("z", jf(z)), ("tolerance", jf(tol)), ("agrees", jb(ok)),
                    ]));
                }
            }
            // printed figures of the note.
            let nf = note_fits.iter().find(|x| x.0 == *g && x.1 == m).unwrap();
            let mc = |off: usize| -> (f64, f64) {
                let vals: Vec<f64> = b.batches.iter().map(|x| x[off]).collect();
                (mean(&vals), 4.0 * sd(&vals) * (1.0 + 1.0 / BOOT_B as f64).sqrt())
            };
            if m != Model::M2 {
                h.printed(&format!("{tag} beta (printed)"), src, nf.2, f.beta, 0.0, "OLS slope");
                let (mu, tol) = mc(2);
                h.printed(&format!("{tag} beta 95% lo (printed)"), src, nf.3, mu, tol, "own stratified cluster bootstrap, mean of 20 x 10,000; tolerance half a printed unit + 4 sigma resampling noise");
                let (mu, tol) = mc(3);
                h.printed(&format!("{tag} beta 95% hi (printed)"), src, nf.4, mu, tol, "as above");
            }
            if !nf.5.is_empty() {
                h.printed(&format!("{tag} log2 n* (printed)"), src, nf.5, f.nstar, 0.0, "point estimate");
            }
            let (mu, tol) = mc(4);
            h.printed(&format!("{tag} log2 n* 95% lo (printed)"), src, nf.6, mu, tol, "own bootstrap, as above");
            let (mu, tol) = mc(5);
            h.printed(&format!("{tag} log2 n* 95% hi (printed)"), src, nf.7, mu, tol, "own bootstrap, as above");
            boot_rows.push(obj(vec![("weight", js(g)), ("model", js(m.name())), ("endpoints", J::Arr(ends)),
                ("frac_beta_nonneg", jf(b.frac_beta_nonneg)), ("replicates_per_batch", ju(b.replicates as u64)), ("batches", ju(BOOT_B as u64))]));
            let mean_end = |off: usize| mean(&b.batches.iter().map(|x| x[off]).collect::<Vec<_>>());
            gj.push((m.name().to_string(), obj(vec![
                ("curves", ju(pts.len() as u64)), ("alpha", jf(f.alpha)), ("beta", jf(f.beta)), ("beta_se", jf(f.se)),
                ("beta_ols_ci95", jfs(&[f.beta - 1.96 * f.se, f.beta + 1.96 * f.se])), ("log2_nstar", jf(f.nstar)),
                ("bootstrap_ci95_mean_of_batches", obj(vec![
                    ("alpha", jfs(&[mean_end(0), mean_end(1)])), ("beta", jfs(&[mean_end(2), mean_end(3)])),
                    ("log2_nstar", jfs(&[mean_end(4), mean_end(5)])),
                ])),
            ])));
            pf.push(f);
            bs.push(b);
        }
        fits_j.push((g.to_string(), J::Obj(gj)));
        point.push(pf);
        boots.push(bs);
    }
    check("bootstrap: every 95% endpoint of fit.json (alpha, beta, log2 n*; 3 weights x 3 models) agrees with this validator's own resampling within 4 sigma of resampling noise",
        boot_bad.is_empty(), if boot_bad.is_empty() { "48 endpoints".into() } else { boot_bad.join("; ") });
    let claimed_frac: f64 = (0..3).map(|k| MODELS.iter().map(|m| fitj.get(WEIGHTS[k].0).get(m.name()).get("frac_beta_nonneg").f()).fold(0.0, f64::max)).fold(0.0, f64::max);
    check("no bootstrap replicate with beta >= 0 (claimed: none)", frac_all == 0.0 && claimed_frac == 0.0,
        format!("own max fraction {frac_all}, fit.json max {claimed_frac}"));

    // --- yield exponent and LA extrapolation.
    let ypts: Vec<(f64, f64)> = curves.iter().map(|c| ((c.p as f64).ln(), c.y.ln())).collect();
    let yf = ols(&ypts);
    h.printed("yield exponent of y in p", src, "-0.066", yf.beta, 0.0, "OLS slope of ln y on ln p, 33 curves");
    h.printed("yield exponent 95% lo", src, "-0.285", yf.beta - 1.96 * yf.se, 0.0, "beta - 1.96 se");
    h.printed("yield exponent 95% hi", src, "0.154", yf.beta + 1.96 * yf.se, 0.0, "beta + 1.96 se");
    let la_from = mean(&c3089.iter().map(|c| c.la_share).collect::<Vec<_>>());
    let m2n = point[0][2].nstar;
    let pstar = 2.0 * 2f64.powf(m2n / 4.0);
    let r0 = la_from / (1.0 - la_from) * pstar / 3089.0;
    let share_star = r0 / (1.0 + r0);
    h.printed("LA share at p=3089 (%, mean of 3)", src, "0.0044", 100.0 * la_from, 0.0, "la_fp_muls / S");
    h.printed("p at M2's n* (0 GAE)", src, "4.2e6", pstar, 0.0, "p = 2 n^(1/4) (n = #E/16, #E ~ p^4)");
    h.printed("LA share extrapolated to M2's n* (%)", src, "5.6", 100.0 * share_star, 0.0, "LA/rest ratio scaled by p*/3089 (LA ~ p^2, rest ~ p), share = r/(1+r)");

    // --- G2.114-G2.123 verdicts.
    let mut v = Vec::new();
    let mut vbad = Vec::new();
    let mut verdict = |id: &str, claimed: &str, recomputed: String, agrees: bool| {
        if !agrees {
            vbad.push(id.to_string());
        }
        v.push(obj(vec![("id", js(id)), ("claimed", js(claimed)), ("recomputed", js(&recomputed)), ("agrees", jb(agrees))]));
    };
    let all_ok = curves.iter().all(|c| c.checks.iter().all(|x| x.1));
    verdict("G2.114", "holds", format!("IC {idi}/{idc} fields; walks {wi}/{wc} fields; rho and steps exact on {ref_exact}/10; every log, row and walk correct at every size: {all_ok}"),
        idi == idc && wi == wc && ref_exact == 10 && all_ok);
    let big = at(557);
    let (clo, chi) = fold(&big, &|c| c.cols as f64 / c.p as f64);
    let h16 = curves.iter().all(|c| c.checks.iter().any(|x| x.0.starts_with("cofactor") && x.1));
    let g115 = h16 && curves.len() == 33 && clo >= 0.04 && chi <= 0.10;
    verdict("G2.115", "holds (for the sizes reached)", format!("33/33 seeds gave h = 16 curves: {h16}; columns/p at p >= 557 in [{clo:.4}, {chi:.4}]"), g115);
    // The note prints columns/p 0.057-0.068 here: erratum E3 (recomputed range below).
    let cmin = big.iter().min_by(|a, b| (a.cols as f64 / a.p as f64).partial_cmp(&(b.cols as f64 / b.p as f64)).unwrap()).unwrap();
    let cmax = big.iter().max_by(|a, b| (a.cols as f64 / a.p as f64).partial_cmp(&(b.cols as f64 / b.p as f64)).unwrap()).unwrap();
    let (cmlo, cmhi) = fold(&big, &|c| c.cmed);
    let (eplo, ephi) = fold(&big, &|c| c.epr);
    let g116 = cmlo >= 6.2e5 && cmhi <= 6.9e5 && eplo >= 1.70e5 && ephi <= 1.85e5;
    verdict("G2.116", "holds", format!("C median [{cmlo:.0}, {cmhi:.0}], E/residual [{eplo:.0}, {ephi:.0}] at p >= 557"), g116);
    h.printed("G2.116 C median min (p>=557)", src, "6.39e5", cmlo, 0.0, "call.median");
    h.printed("G2.116 C median max (p>=557)", src, "6.67e5", cmhi, 0.0, "call.median");
    h.printed("G2.116 E/residual min (p>=557)", src, "1.762e5", eplo, 0.0, "E / residuals");
    h.printed("G2.116 E/residual max (p>=557)", src, "1.770e5", ephi, 0.0, "E / residuals");
    let (swlo, _) = fold(&all, &|c| c.weil_solve_share);
    let (solo, _) = fold(&all, &|c| c.solve_share);
    let (_, lahi) = fold(&all, &|c| c.la_share);
    verdict("G2.117", "holds", format!("(Weil + solve)/S >= {:.4} %, solve alone >= {:.4} %; LA <= {:.5} %", 100.0 * swlo, 100.0 * solo, 100.0 * lahi),
        swlo >= 0.98 && lahi <= 0.001);
    h.printed("G2.117 'solve' share min (%)", src, "98.28", 100.0 * swlo, 0.0, "(weil_muls + solve_muls) / S, the registered quantity, labelled 'solve' in the note (erratum E2)");
    h.printed("G2.117 LA share max (%)", src, "0.0057", 100.0 * lahi, 0.0, "la_fp_muls / S");
    let ymeds: Vec<f64> = SIZES.iter().map(|p| median(&curves.iter().filter(|c| c.p == *p).map(|c| c.y).collect::<Vec<_>>())).collect();
    verdict("G2.118", "holds", format!("exponent {:.4} in [-0.5, 0.5]; per-size median y [{:.1}, {:.1}] in [400, 6000]", yf.beta, vmin(&ymeds), vmax(&ymeds)),
        yf.beta.abs() <= 0.5 && vmin(&ymeds) >= 400.0 && vmax(&ymeds) <= 6000.0);
    h.printed("G2.118 per-size median y min", src, "969", vmin(&ymeds), 0.0, "median of residuals/relations");
    h.printed("G2.118 per-size median y max", src, "4272", vmax(&ymeds), 0.0, "median of residuals/relations");
    let (snlo, snhi) = fold(&all, &|c| c.steps_sqrt_n);
    let top = at(1549);
    let (pslo, pshi) = fold(&top, &|c| c.per_step);
    let step_meds: Vec<f64> = SIZES.iter().map(|p| median(&curves.iter().filter(|c| c.p == *p).map(|c| c.per_step).collect::<Vec<_>>())).collect();
    let falling = step_meds.windows(2).all(|w| w[1] < w[0]);
    let steps_hold = snlo >= 0.4 && snhi <= 0.9;
    let band_hold = pslo >= 66.0 && pshi <= 80.0;
    verdict("G2.119", "steps hold; per step falsified high",
        format!("steps/sqrt n [{snlo:.3}, {snhi:.3}] (band [0.4, 0.9]: {steps_hold}); per step at p >= 1549 [{pslo:.2}, {pshi:.2}] (band [66, 80]: {band_hold}); per-size median per step strictly falling: {falling}"),
        steps_hold && !band_hold && pslo > 80.0 && falling);
    h.printed("G2.119 steps/sqrt n min", src, "0.468", snlo, 0.0, "mean steps / sqrt n");
    h.printed("G2.119 steps/sqrt n max", src, "0.787", snhi, 0.0, "mean steps / sqrt n");
    h.printed("G2.119 per step min (p>=1549)", src, "95.9", pslo, 0.0, "rho / steps");
    h.printed("G2.119 per step max (p>=1549)", src, "96.5", pshi, 0.0, "rho / steps");
    let ups: Vec<u64> = (1..SIZES.len()).filter(|k| med_d[*k][0] >= med_d[*k - 1][0]).map(|k| SIZES[k]).collect();
    let below = curves.iter().filter(|c| c.d.iter().any(|d| *d <= 1.0)).count();
    let top_med = med_d[10][0];
    let level = (3e2..=5e3).contains(&top_med);
    verdict("G2.120", "level and rho ahead hold; falls with n holds as a trend, falsified read as monotone (3 of 10 steps)",
        format!("largest-size median D(0) {top_med:.1} in [3e2, 5e3]: {level}; rows with D <= 1 at any weight: {below}; per-size median D(0) rises at {} of 10 steps (p = {:?})", ups.len(), ups),
        level && below == 0 && ups == vec![149, 269, 773]);
    h.printed("G2.120 rows at or below rho, any weight", src, "0", below as f64, 0.0, "count of D(g) <= 1");
    h.exact("G2.120 non-monotone steps", src, 3.0, ups.len() as f64, "per-size median D(0) not below the previous size's");
    let f0 = &point[0];
    let b0 = &boots[0][0];
    let ci = |b: &Boot, off: usize| mean(&b.batches.iter().map(|x| x[off]).collect::<Vec<_>>());
    let (blo, bhi) = (ci(b0, 2), ci(b0, 3));
    verdict("G2.121", "holds", format!("beta {:.4} in [-0.30, -0.15]; own 95% [{blo:.4}, {bhi:.4}] contains -0.25", f0[0].beta),
        (-0.30..=-0.15).contains(&f0[0].beta) && blo <= -0.25 && bhi >= -0.25);
    let width = |b: &Boot| -> (f64, f64) {
        let w: Vec<f64> = b.batches.iter().map(|x| x[5] - x[4]).collect();
        (mean(&w), 4.0 * sd(&w) * (1.0 + 1.0 / BOOT_B as f64).sqrt())
    };
    let (w1, w1tol) = width(b0);
    let b2 = &boots[0][2];
    let (w2, w2tol) = width(b2);
    verdict("G2.122", "holds", format!("M1 log2 n* {:.2} in [74, 96], interval {w1:.2} bits (> 8); M2 {:.2} in [76, 90], interval {w2:.2} bits (<= 6)", f0[0].nstar, f0[2].nstar),
        (74.0..=96.0).contains(&f0[0].nstar) && w1 > 8.0 && (76.0..=90.0).contains(&f0[2].nstar) && w2 <= 6.0);
    h.printed("G2.122 M1 interval width (bits)", src, "15.1", w1, w1tol, "own bootstrap 97.5% - 2.5% of log2 n*, 0 GAE; tolerance half a printed unit + 4 sigma resampling noise");
    h.printed("G2.122 M2 interval width (bits)", src, "1.1", w2, w2tol, "own bootstrap, 0 GAE; as above");
    let s1 = point[1][2].nstar - f0[2].nstar;
    let s2 = point[2][2].nstar - f0[2].nstar;
    let ordered = curves.iter().filter(|c| c.d[0] < c.d[1] && c.d[1] < c.d[2]).count();
    verdict("G2.123", "holds", format!("M2 shift +{s1:.2} bits at 0.1 GAE (band [5, 9]), +{s2:.2} at 1 GAE (band [11, 17]); D(0) < D(0.1) < D(1) on {ordered}/33 rows"),
        (5.0..=9.0).contains(&s1) && (11.0..=17.0).contains(&s2) && ordered == 33);
    h.printed("G2.123 M2 shift at 0.1 GAE (bits)", src, "6.8", s1, 0.0, "M2 log2 n*(0.1) - log2 n*(0)");
    h.printed("G2.123 M2 shift at 1 GAE (bits)", src, "15.1", s2, 0.0, "M2 log2 n*(1) - log2 n*(0)");
    h.exact("G2.123 rows ordered D(0)<D(0.1)<D(1)", src, 33.0, ordered as f64, "count");
    let best = curves.iter().min_by(|a, b| a.d[0].partial_cmp(&b.d[0]).unwrap()).unwrap();
    h.printed("best row D(0)", src, "1.14e3", best.d[0], 0.0, "min over 33 curves of S / rho");
    h.exact("best row p", src, 3089.0, best.p as f64, "its size");
    h.exact("best row seed", src, 2.0, best.seed as f64, "its seed");
    h.printed("largest size median D(0.1)", src, "4.9e3", med_d[10][1], 0.0, "p = 3089");
    h.printed("largest size median D(1)", src, "2.0e4", med_d[10][2], 0.0, "p = 3089");
    h.printed("bits of n from the top rung to M1's n* (0 GAE)", src, "43", f0[0].nstar - med_of(&c3089, &|c| c.log2n), 0.0, "log2 n* - 42.37");
    check("every recomputed G2.114-G2.123 verdict equals the note's", vbad.is_empty(), format!("{vbad:?}"));

    // --- §5a band and order.
    let rho_ahead = curves.iter().all(|c| (0..3).all(|k| c.rg[k] < c.sg[k]));
    check("rho below IC on all 33 paired rows at 0, 0.1 and 1 GAE (order holds across the band)", rho_ahead, format!("min D over rows and weights {:.1}", curves.iter().map(|c| vmin(&c.d)).fold(f64::INFINITY, f64::min)));
    let band = |k: usize| obj(vec![
        ("ic", jf(median(&c3089.iter().map(|c| c.sg[k]).collect::<Vec<_>>()))),
        ("rho", jf(median(&c3089.iter().map(|c| c.rg[k]).collect::<Vec<_>>()))),
        ("median_D", jf(med_d[10][k])),
    ]);
    let gae_band = obj(vec![
        ("definition", js("medians over the three curves of the ladder's top rung (p = 3089, log2 n = 42.37) of S(g) = S + g * 97 * Macaulay entries (IC: arm B, solver A, Karatsuba) and rho(g) = rho + g * 97 * steps (E27 walk (iii), mean of 16 walks, [d]G = Q check excluded), g = 0, 0.1, 1; median_D is the median of the per-curve ratio (the note's figure), not the ratio of the medians. Per-size medians for every rung are in per_size[].D.")),
        ("values", obj(vec![("0", band(0)), ("0.1", band(1)), ("1", band(2))])),
        ("order_rho_below_ic_on_all_33_rows_all_weights", jb(rho_ahead)),
    ]);

    // --- curves table.
    let cj: Vec<J> = curves.iter().map(|c| obj(vec![
        ("p", ju(c.p)), ("seed", ju(c.seed)), ("n", ju(c.n)), ("log2n", jf(c.log2n)), ("S", ju(c.s)), ("E", ju(c.e)),
        ("rho", jf(c.rho)), ("steps", jf(c.steps)), ("D", obj(vec![("0", jf(c.d[0])), ("0.1", jf(c.d[1])), ("1", jf(c.d[2]))])),
        ("y", jf(c.y)), ("columns", ju(c.cols)), ("relations", ju(c.relations)), ("residuals", ju(c.residuals)),
        ("C_median", jf(c.cmed)), ("entries_per_residual", jf(c.epr)), ("solve_share", jf(c.solve_share)),
        ("weil_plus_solve_share", jf(c.weil_solve_share)), ("la_share", jf(c.la_share)),
        ("steps_per_sqrt_n", jf(c.steps_sqrt_n)), ("muls_per_step", jf(c.per_step)), ("logarithm", ju(c.recovered)),
    ])).collect();

    // --- replay summary (if the comparison exists).
    let cmp_rel = format!("{}/replay/replay_comparison.json", ctx.ev);
    let mut scope_text = "Replay comparison missing.".to_string();
    if ctx.exists(&cmp_rel) {
        let cmp = ctx.load(&cmp_rel);
        scope_text = if cmp.get("runs_present").u() == cmp.get("recorded_runs").u() {
            "Replay scope: full. All 33 index-calculus runs and all 33 walk pools (528 walks) were re-executed with the validator's fresh build; the nonce sample (one IC curve per size) and every walk pool ran first, the other 22 IC runs as phase 2. The p = 4337 IC record (unpaired) was not replayed.".to_string()
        } else {
            format!("Replay scope: one nonce-chosen index-calculus run per size (11 of 33), {} further IC runs, and all 33 walk pools (528 walks); the other IC records are recomputed from their raw records, not re-executed. Ten of the 33 IC records sit on E25's curves and equal E27's records leaf by leaf, which E27's validator replayed with an independent build of af2504ef1 (research/e27_walk_20261030/replay-certificate.json).", cmp.get("extension_runs_present").u())
        };
        let ok = cmp.get("required_matched").u() == cmp.get("required_runs").u()
            && cmp.get("runs_matched").u() == cmp.get("runs_present").u()
            && cmp.get("fields_compared").u() == cmp.get("fields_identical").u()
            && cmp.get("scalars_verified").bo() == Some(true);
        check("replay: every required run (nonce-selected IC per size, every walk pool) and every extension run replayed and identical on every non-timing leaf, scalars verified",
            ok, format!("required {}/{}, extension {}/{}, all {}/{} of {} recorded runs, {}/{} fields",
                cmp.get("required_matched").u(), cmp.get("required_runs").u(), cmp.get("extension_matched").u(), cmp.get("extension_runs_present").u(),
                cmp.get("runs_matched").u(), cmp.get("runs_present").u(), cmp.get("recorded_runs").u(), cmp.get("fields_identical").u(), cmp.get("fields_compared").u()));
    } else {
        check("replay comparison present", false, "replay/replay_comparison.json missing".into());
    }

    let status = fails.is_empty() && h.bad.is_empty();
    let mut lim: Vec<J> = vec![js(&scope_text)];
    lim.extend([
        "Local 4-vCPU container, one core (taskset -c 3), RAYON_NUM_THREADS=4 as the producer; not isolab-eks (unreachable). Operation counts only: no wall time is compared or claimed.",
        "The fit is an extrapolation: log2 n* lies 37-66 bits beyond the top rung (2^42.37); the recompute confirms the arithmetic and the resampling, not the model's validity beyond the measured range. The LA term (~p^2) is not modelled; its extrapolated share is a sensitivity.",
        "Bootstrap intervals are compared within resampling noise (own xoshiro256** streams named by the replay nonce, 20 batches x 10,000 replicates, 4 sigma); E32's own generator (SplitMix64, seed 32) is not re-run, so its exact endpoints are not reproduced digit for digit.",
        "strongest_available rests on the E25/E27 argument (Aut(E) = {±1}, psi the only Frobenius-type endomorphism, <±psi> the largest fold) and on this validator's j-invariant probe of all 34 instances (replay/j_invariant_probe.json); the probe calls the library's generator and F_p4 arithmetic, so it is a consistency check, not an independent implementation.",
        "p = 4337 is outside the measurement: its IC run completed but its walk pool was OOM-killed (memory cgroup); it is excluded from every fit and the gate band.",
        "Unit: F_p multiplications counted by the library's counters; the §5a band prices one native operation per Macaulay entry (IC) or walk step (rho) at 97 F_p multiplications per GAE.",
        "Not a measured bound in the sense of the autoresearcher ledger (no sealed ECBND1h record): E32 claims a distance measurement and a labelled extrapolation, not an improvement; this gate admits it as evidence only.",
    ].iter().map(|s| js(s)));
    obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js(if status { "pass" } else { "fail" })),
        ("independent", jb(true)),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E32; this recompute is its own standard-library Rust program (validate_e32.rs: own JSON parser, SHA-256, xoshiro256** resampling, OLS), sharing no code with examples/e32_ladder.rs, examples/e32_tables.rs or crypto_lib")),
        ("date", js("2026-11-01")),
        ("row", js("E32 (GLV note §9.34 pre-registration at 246460f094, §9.36 results at 8afc92781b): the handover ladder, arm B with E31 solver A and Karatsuba products against E27's walk (iii), 11 sizes p = 101-3089, 3 curves each, 33 paired curves, 528 walks; fitted crossover order at 0, 0.1, 1 GAE")),
        ("script", js("research/e32_handover_ladder_20261031/validate_e32.rs (rustc -O --edition 2021; `validate_e32 report . research/e32_handover_ladder_20261031`)")),
        ("status_rule", js("pass iff every check holds and every headline agrees: printed figures within half a unit of their last printed digit (bootstrap endpoints: plus 4 sigma of resampling noise), full-precision values of fit.json within a relative 1e-9, counts exactly")),
        ("raw_data", J::Arr(raw)),
        ("headline", J::Arr(h.v)),
        ("headline_disagreements", J::Arr(h.bad.iter().map(|s| js(s)).collect())),
        ("checks", J::Arr(checks)),
        ("check_failures", J::Arr(fails.iter().map(|s| js(s)).collect())),
        ("verdicts_recomputed", J::Arr(v)),
        ("errata", J::Arr(vec![
            obj(vec![("id", js("E1")), ("claim", js("experiments/36_handover_ladder/binary_commit.txt: 'Build-input commit: e75771052b28668c834d227588389abff164fc7'")),
                ("recomputed", js("the commit is e757710521b28668c834d227588389abff164fc7 (39 hex digits printed; the tenth digit '1' dropped); the 9-digit prefix e75771052 the note cites is unambiguous")),
                ("effect", js("none on any number; the record is a typo"))]),
            obj(vec![("id", js("E2")), ("claim", js("§9.36: 'the solve is 98.3-99.1 % of S' and G2.117 'solve >= 98.28 %'; summary.json/tables.md `solve_share` / 'solve %'")),
                ("recomputed", js(&format!("these figures are (weil_muls + solve_muls) / S: {:.4}-{:.4} %; the solve alone is {:.4}-{:.4} % of S", 100.0 * swlo, 100.0 * fold(&all, &|c| c.weil_solve_share).1, 100.0 * solo, 100.0 * fold(&all, &|c| c.solve_share).1))),
                ("effect", js("none on G2.117: its registered quantity is (Weil + solve) / S >= 98 %, which is what the figures measure; only the label 'solve' is wrong"))]),
            obj(vec![("id", js("E3")), ("claim", js("§9.36 G2.115: 'columns / p 0.057-0.068 at p >= 557'")),
                ("recomputed", js(&format!("{:.4} (p = {} seed {}: {} / {}) to {:.4} (p = {} seed {}: {} / {}); folded.columns in summary.json equals the records",
                    cmin.cols as f64 / cmin.p as f64, cmin.p, cmin.seed, cmin.cols, cmin.p, cmax.cols as f64 / cmax.p as f64, cmax.p, cmax.seed, cmax.cols, cmax.p))),
                ("effect", js("none on G2.115: the band [0.04, 0.10] holds on every curve with p >= 557; the printed range is wrong at both ends"))]),
            obj(vec![("id", js("E4")), ("claim", js("§9.36 fits table: M2 at 0.1 GAE, log2 n* = 90.9")),
                ("recomputed", js(&format!("{:.4} (fit.json 90.84743389821227), i.e. 90.8 to one decimal; the printed 90.9 is consistent with a double rounding 90.847 -> 90.85 -> 90.9", point[1][2].nstar))),
                ("effect", js("none: G2.123's shift (+6.8 bits) and the M2 interval [90.3, 91.4] are computed from the unrounded value; M2 is a sensitivity row"))]),
        ])),
        ("limitations", J::Arr(lim)),
        ("heuristic_validation_checks", J::Arr(vec![
            obj(vec![("check", js("pre-registered prediction")), ("holds", jb(true)),
                ("detail", js("the theory slope -1/4 (S ~ p, rho ~ p^2), the fit models M1/M1-all/M2, the bootstrap (10,000 replicates, stratified by size) and the bands G2.114-G2.123 are in §9.34, committed at 246460f094 before the first run (checks above); §9.34 is byte-identical at HEAD"))]),
            obj(vec![("check", js("sample integrity")), ("holds", jb(true)),
                ("detail", js("sizes, seeds (1-3 per size, replacement rule, none used), rho-run counts and the stopping rule are in §9.34 and run_ladder.sh; every distance, median, fit and interval is recomputed here from the 66 raw records"))]),
            obj(vec![("check", js("correspondence validity")), ("holds", jb(true)),
                ("detail", js("no substitute sampler: curves are E25's GLS generator itself; the scope is E25's GLS twists (h = 16, p = 1 mod 4) and nothing is transferred to other curve families"))]),
            obj(vec![("check", js("scale binding")), ("holds", jb(true)),
                ("detail", js("measured p = 101-3089, log2 n = 22.6-42.4; the fitted log2 n* (85.5 [79.2, 94.3] at 0 GAE) lies 37-52 bits beyond the top rung and is labelled an extrapolation in §9.34 and §9.36 (AGENTS.md §6); the measured statement is n* > 2^42.4"))]),
            obj(vec![("check", js("null-object control (inventor protocol §3)")), ("holds", jb(true)),
                ("detail", js("not applicable: no correlation, bias or excess against a null is claimed; the slope is a cost trend compared with a pre-registered theory slope, and the bootstrap shows no replicate with beta >= 0"))]),
        ])),
        ("cost_model_checks", J::Arr(vec![
            obj(vec![("check", js("unit declared")), ("holds", jb(true)),
                ("detail", js("F_p multiplications from the library's counters on both arms; the §5a band adds 97 g per Macaulay entry (IC) or walk step (rho)"))]),
            obj(vec![("check", js("optimistic assumptions flagged")), ("holds", jb(true)),
                ("detail", js("the walk's [d]G = Q check is excluded (E25's convention, stated); unpriced native work is priced by the band, not assumed free; the LA's p^2 term is not modelled (sensitivity reported, ~5.6 % at M2's n*)"))]),
            obj(vec![("check", js("memory reported alongside cost")), ("holds", jb(false)),
                ("detail", js("limitation, not a failure of this gate: neither arm's memory is recorded; the only datum is the p = 4337 16-walk pool killed at 4.8 GB, which is why that rung is unpaired. Any extrapolation to 2^85 says nothing about memory"))]),
            obj(vec![("check", js("cost bookkeeping (per-attempt x inverse success)")), ("holds", jb(true)),
                ("detail", js("success probability 1 in every recorded run (every IC log recovered, 528/528 walks correct); S is the whole run's count including reruns and fallbacks, rho the mean over 16 walks of the full walk cost; nothing is scaled by an assumed success rate"))]),
        ])),
        ("identity_e27", J::Arr(id_rows)),
        ("curves", J::Arr(cj)),
        ("per_size", J::Arr(per_size)),
        ("fits", J::Obj(fits_j)),
        ("bootstrap", obj(vec![
            ("method", js("stratified cluster bootstrap over curves (resample each size's curves with replacement), 20 independent batches of 10,000 replicates; streams xoshiro256** seeded by SHA-256 of 'e32-validator-bootstrap:<nonce>:<weight>:<model>:batch<b>'; percentiles by linear interpolation; replicates with beta >= 0 count as n* = infinity (none occurred)")),
            ("rows", J::Arr(boot_rows)),
        ])),
        ("yield_exponent", obj(vec![("slope", jf(yf.beta)), ("se", jf(yf.se)), ("ci95", jfs(&[yf.beta - 1.96 * yf.se, yf.beta + 1.96 * yf.se]))])),
        ("la_extrapolation", obj(vec![("share_at_3089_mean", jf(la_from)), ("p_star", jf(pstar)), ("share_at_nstar", jf(share_star))])),
        ("gae_band", gae_band),
    ])
}

// --------------------------------------------------------- certificate ----

fn certificate(ctx: &Ctx) -> J {
    let cmp_rel = format!("{}/replay/replay_comparison.json", ctx.ev);
    let build_rel = format!("{}/replay/build.json", ctx.ev);
    let cmp = ctx.load(&cmp_rel);
    let build = ctx.load(&build_rel);
    let all = cmp.get("required_matched").u() == cmp.get("required_runs").u()
        && cmp.get("required_missing").arr().is_empty()
        && cmp.get("runs_matched").u() == cmp.get("runs_present").u()
        && cmp.get("fields_compared").u() == cmp.get("fields_identical").u();
    let full = cmp.get("runs_present").u() == cmp.get("recorded_runs").u();
    let coverage = if full {
        "full: every recorded run replayed, all 33 index-calculus runs and all 33 walk pools (528 walks). The nonce sample (replay/nonce.json, replay/selection.txt: one IC curve per size, seeds 3,1,1,3,3,3,1,3,1,2,2 for p = 101...3089, drawn after the results commit and before any replay) and every walk pool ran first (replay.sh); the other 22 IC runs followed as phase 2 (replay_rest.sh)".to_string()
    } else {
        format!("sampled for the index calculus, full for the walks: at each of the 11 sizes the IC run of one curve chosen by a nonce drawn after the results commit and before any replay (replay/nonce.json, replay/selection.txt; seeds 3,1,1,3,3,3,1,3,1,2,2 for p = 101...3089, including the heavy-tail p = 149 seed 1 and the best row p = 3089 seed 2), the 16-walk pool of all 33 curves (528 walks), and {} further IC runs (phase 2). The remaining IC records are recomputed from raw records in validator-report.json; ten of the 33 equal E27's arm-B records leaf by leaf, which E27's validator replayed independently.", cmp.get("extension_runs_present").u())
    };
    let sc = cmp.get("scalars_verified").bo() == Some(true);
    let runs: Vec<J> = cmp.get("runs").arr().iter().map(|r| obj(vec![
        ("run", r.get("run").clone()), ("committed", r.get("committed").clone()), ("replay_output", r.get("replay_output").clone()),
        ("fields_compared", r.get("fields_compared").clone()), ("fields_identical", r.get("fields_identical").clone()),
        ("matched", r.get("matched").clone()), ("scalars", r.get("scalars").clone()),
    ])).collect();
    obj(vec![
        ("schema", js("replay-certificate/v1")),
        ("status", js(if all && sc { "verified" } else { "failed" })),
        ("independent", jb(true)),
        ("scalars_verified", jb(sc)),
        ("date", js("2026-11-01")),
        ("row", js("E32 (GLV note §9.34/§9.36): the handover ladder, e32_ladder ic (arm B, solver chain, Karatsuba) and e32_ladder walk (E27 walk (iii)) on 11 sizes p = 101-3089")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E32: fresh build of the recorded binary commit from a git export in a fresh tmpfs target directory; nothing reused from the producer's target/ or its pinned binaries")),
        ("recorded_binary", js("e757710521b28668c834d227588389abff164fc7 (experiments/36_handover_ladder/binary_commit.txt) plus examples/e32_ladder.rs from the pre-registration commit 246460f094 (sha256 a093a902...ff756, as recorded)")),
        ("build", build.clone()),
        ("build_record", obj(vec![("path", js("replay/build.json")), ("sha256", js(&sha_file(&ctx.path(&build_rel))))])),
        ("comparison", obj(vec![("path", js("replay/replay_comparison.json")), ("sha256", js(&sha_file(&ctx.path(&cmp_rel)))), ("rule", cmp.get("rule").clone())])),
        ("runs_recorded", cmp.get("recorded_runs").clone()),
        ("runs_total", cmp.get("runs_present").clone()),
        ("runs_matched", cmp.get("runs_matched").clone()),
        ("required_runs", cmp.get("required_runs").clone()),
        ("required_matched", cmp.get("required_matched").clone()),
        ("fields_compared", cmp.get("fields_compared").clone()),
        ("fields_identical", cmp.get("fields_identical").clone()),
        ("coverage", js(&coverage)),
        ("scalar_verification", js("Every replayed IC run reports solved and correct (the binary checks the Wiedemann logarithm against the planted d and [x]G = Q) and its recovered logarithm equals that of the committed record and of every committed walk of the curve; every replayed walk (528) reports correct and recovers the committed IC logarithm: two algorithms agree on every curve.")),
        ("runs", J::Arr(runs)),
    ])
}

fn gate(ctx: &Ctx) -> J {
    let rep_rel = format!("{}/validator-report.json", ctx.ev);
    let cert_rel = format!("{}/replay-certificate.json", ctx.ev);
    let rep = ctx.load(&rep_rel);
    let vals = rep.get("gae_band").get("values");
    let band = J::Obj(WEIGHTS.iter().map(|(g, _)| (g.to_string(), obj(vec![("ic", vals.get(g).get("ic").clone()), ("rho", vals.get(g).get("rho").clone())]))).collect());
    let order = rep.get("gae_band").get("order_rho_below_ic_on_all_33_rows_all_weights").bo() == Some(true);
    let protocol = format!(
        "Named by the protocol (note §9.34, pre-registered at 246460f094 before any E32 run): E27's walk (iii), the matched rho of the line since §9.25/§9.29: on each GLS curve (E25's generator, h = 16) the <±psi> walk, psi the GLS endomorphism (psi^2 = -1), a class {{±P, ±psi P}} of 4 points, Karatsuba F_p4 products and the norm inversions of a 16-walk lockstep pool batched by Montgomery's trick (fillers from 1,000,000*seed + 500,000), 16 walks per curve with seeds 1000*seed + k, cost per walk without the [d]G = Q check (E25's and E27's convention), rho = mean of the 16. Strongest available: on these curves j(E) lies in F_p2 but not in F_p and is neither 0 nor 1728 (validator's probe on all 34 instances, replay/j_invariant_probe.json), so Aut(E) = {{±1}} and psi is the only Frobenius-type endomorphism over F_p4: <±psi> is the largest fold, and walk (iii) is the cheapest arithmetic implemented for it (E27: 1.456-1.463x below walk (i)). Unpriced native work (AGENTS.md §5a): the IC leaves its Macaulay-matrix entry traffic (macaulay_entries) and its F_p additions and row scans unpriced, the walks each step's hash lookup/insert and partition hash (steps) and the batch bookkeeping. Band: S(g) = F_p multiplications + g * 97 * native operations, one per Macaulay entry or walk step, GAE = 97 F_p multiplications (E25's affine addition, the registered primary), g = 0, 0.1, 1. gae_band values: medians over the three curves of the ladder's top rung (p = 3089, log2 n = 42.37), IC arm B (solver A, Karatsuba) against walk (iii): median ratio D = {:.0} (0), {:.0} (0.1), {:.0} (1 GAE). Walk (iii) is below the IC on every one of the 33 paired rows (p = 101-3089) at every weight; the per-curve ratio is {:.3e}-{:.3e} (0), so the order rho < IC holds across the band. p = 4337 (IC complete, walk pool OOM-killed) is unpaired and outside the band.",
        vals.get("0").get("median_D").f(), vals.get("0.1").get("median_D").f(), vals.get("1").get("median_D").f(),
        rep.get("curves").arr().iter().map(|c| c.get("D").get("0").f()).fold(f64::INFINITY, f64::min),
        rep.get("curves").arr().iter().map(|c| c.get("D").get("0").f()).fold(0.0, f64::max));
    obj(vec![
        ("schema", js("validation-gate/v1")),
        ("replay_certificate", obj(vec![("path", js("replay-certificate.json")), ("sha256", js(&sha_file(&ctx.path(&cert_rel))))])),
        ("validator_report", obj(vec![("path", js("validator-report.json")), ("sha256", js(&sha_file(&ctx.path(&rep_rel))))])),
        ("rho_reference", obj(vec![
            ("kind", js("negation_frobenius")),
            ("protocol", js(&protocol)),
            ("unit", js("F_p multiplications, both arms counted by the same Fp4 counters of one build: an F_p4 product 13 (Karatsuba tower) on both sides; the IC's F_p4 inversion 40, the walk's 24 + its round's batch share; the solve's F_p multiplications counted directly, the Wiedemann matrix at 16 per multiplication mod n; walk set-up included, the walk's [d]G = Q check excluded (E25's convention)")),
            ("binary_or_process", js("One binary, examples/e32_ladder, built fresh by the validator from the recorded commit e757710521 with examples/e32_ladder.rs from 246460f094 (replay/build.json): mode `ic` runs run_k4_gls_pipeline_with (arm B, V4Solver::Chain, Karatsuba) and mode `walk` runs rho4_folded_batched (Gls fold, Karatsuba, pool 16) on the same instance (generate_gls_klein_instance(p, seed, 16): n, cofactor and the logarithm agree between the two records of every curve) with the same crypto_lib counters. The producer's records (pinned binaries from the same commit) and this replay agree on every non-timing leaf of every replayed run.")),
            ("matches_protocol", jb(true)),
            ("strongest_available", jb(true)),
            ("same_unit", jb(true)),
            ("same_binary_or_process", jb(true)),
            ("gae_band", band),
            ("order_holds_across_band", jb(order)),
        ])),
    ])
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 4 {
        eprintln!("usage: validate_e32 compare|report|certificate|gate <repo> <evidence-dir>");
        std::process::exit(2);
    }
    let ctx = Ctx { repo: a[2].trim_end_matches('/').to_string(), ev: a[3].trim_end_matches('/').to_string() };
    let (out, v) = match a[1].as_str() {
        "compare" => (format!("{}/replay/replay_comparison.json", ctx.ev), compare(&ctx)),
        "report" => (format!("{}/validator-report.json", ctx.ev), report(&ctx)),
        "certificate" => (format!("{}/replay-certificate.json", ctx.ev), certificate(&ctx)),
        "gate" => (format!("{}/VALIDATION.json", ctx.ev), gate(&ctx)),
        m => panic!("unknown mode {m}"),
    };
    fs::write(ctx.path(&out), dump(&v)).unwrap();
    let status = v.get("status").s().to_string();
    println!("{} -> {} {}", a[1], out, status);
    if a[1] == "report" {
        for x in v.get("check_failures").arr() {
            println!("CHECK FAIL: {}", x.s());
        }
        for x in v.get("headline_disagreements").arr() {
            println!("HEADLINE: {}", x.s());
        }
    }
}
