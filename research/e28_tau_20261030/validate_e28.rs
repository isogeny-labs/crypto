//! E28 validation (independent validator, 2026-10-31).
//!
//! An independent recompute of the headline numbers of
//! `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.30 (E28; pre-registration §9.26) from the raw records in the repository,
//! and a leaf-by-leaf comparison of a fresh replay with the committed records.
//! Standard library only.  The JSON parser, SHA-256 and the leaf comparator are
//! copied from the E27 validator's `research/e27_walk_20261030/validate_e27.rs`
//! (validator code); everything else is new.  It shares no code with
//! `examples/e28_tau.rs`, `examples/e28_tables.rs` or `crypto_lib`.
//!
//! ```text
//! rustc -O --edition 2021 validate_e28.rs -o validate_e28
//! validate_e28 compare     <repo> <evidence-dir>   # -> replay/replay_comparison.json
//! validate_e28 report      <repo> <evidence-dir>   # -> validator-report.json
//! validate_e28 certificate <repo> <evidence-dir>   # -> replay-certificate.json
//! ```
//!
//! `<evidence-dir>` is relative to `<repo>` (here `research/e28_tau_20261030`).

use std::collections::BTreeSet;
use std::fs;

// ---------------------------------------------------------------- JSON ----

#[derive(Clone, Debug, PartialEq)]
enum J {
    Null,
    Bool(bool),
    /// A number kept as its text, so 64-bit digests and exact floats survive.
    Num(String),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

static NULL: J = J::Null;

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }
    fn expect(&mut self, c: u8) -> Result<(), String> {
        if self.peek() == Some(c) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected '{}' at byte {}", c as char, self.i))
        }
    }
    fn lit(&mut self, word: &str, v: J) -> Result<J, String> {
        if self.s[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(format!("bad literal at byte {}", self.i))
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let h = std::str::from_utf8(&self.s[self.i..self.i + 4]).map_err(|e| e.to_string())?;
        self.i += 4;
        u32::from_str_radix(h, 16).map_err(|e| e.to_string())
    }
    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = self.peek().ok_or("unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = self.peek().ok_or("bad escape")?;
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
                            if (0xD800..0xDC00).contains(&cp) {
                                self.expect(b'\\')?;
                                self.expect(b'u')?;
                                let lo = self.hex4()?;
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                            }
                            let ch = char::from_u32(cp).ok_or("bad code point")?;
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return Err(format!("bad escape at byte {}", self.i)),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        match self.peek() {
            Some(b'{') => {
                self.i += 1;
                let mut o = Vec::new();
                self.ws();
                if self.peek() == Some(b'}') {
                    self.i += 1;
                    return Ok(J::Obj(o));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    self.expect(b':')?;
                    let v = self.value()?;
                    o.push((k, v));
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(J::Obj(o));
                        }
                        _ => return Err(format!("bad object at byte {}", self.i)),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut a = Vec::new();
                self.ws();
                if self.peek() == Some(b']') {
                    self.i += 1;
                    return Ok(J::Arr(a));
                }
                loop {
                    a.push(self.value()?);
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(J::Arr(a));
                        }
                        _ => return Err(format!("bad array at byte {}", self.i)),
                    }
                }
            }
            Some(b'"') => Ok(J::Str(self.string()?)),
            Some(b't') => self.lit("true", J::Bool(true)),
            Some(b'f') => self.lit("false", J::Bool(false)),
            Some(b'n') => self.lit("null", J::Null),
            Some(c) if c == b'-' || c.is_ascii_digit() => {
                let st = self.i;
                while self.i < self.s.len()
                    && matches!(
                        self.s[self.i],
                        b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E'
                    )
                {
                    self.i += 1;
                }
                Ok(J::Num(
                    String::from_utf8(self.s[st..self.i].to_vec()).unwrap(),
                ))
            }
            _ => Err(format!("unexpected byte at {}", self.i)),
        }
    }
}

fn parse(text: &str) -> J {
    let mut p = Parser {
        s: text.as_bytes(),
        i: 0,
    };
    let v = p
        .value()
        .unwrap_or_else(|e| panic!("JSON parse error: {e}"));
    p.ws();
    assert_eq!(p.i, text.len(), "trailing bytes after JSON value");
    v
}

impl J {
    fn get(&self, k: &str) -> &J {
        match self {
            J::Obj(o) => o
                .iter()
                .find(|(kk, _)| kk == k)
                .map(|(_, v)| v)
                .unwrap_or(&NULL),
            _ => &NULL,
        }
    }
    fn arr(&self) -> &[J] {
        match self {
            J::Arr(a) => a,
            _ => panic!("not an array: {self:?}"),
        }
    }
    fn u(&self) -> u64 {
        match self {
            J::Num(s) => s
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("not a u64: {s}")),
            _ => panic!("not a number: {self:?}"),
        }
    }
    fn f(&self) -> f64 {
        match self {
            J::Num(s) => s.parse::<f64>().unwrap(),
            _ => panic!("not a number: {self:?}"),
        }
    }
    fn b(&self) -> bool {
        match self {
            J::Bool(b) => *b,
            _ => panic!("not a bool: {self:?}"),
        }
    }
    fn s(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => panic!("not a string: {self:?}"),
        }
    }
    fn opt_u(&self) -> Option<u64> {
        match self {
            J::Null => None,
            _ => Some(self.u()),
        }
    }
    fn write(&self, out: &mut String, ind: usize) {
        let pad = |n: usize| " ".repeat(n);
        match self {
            J::Null => out.push_str("null"),
            J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            J::Num(s) => out.push_str(s),
            J::Str(s) => esc(s, out),
            J::Arr(a) if a.is_empty() => out.push_str("[]"),
            J::Obj(o) if o.is_empty() => out.push_str("{}"),
            J::Arr(a) => {
                out.push_str("[\n");
                for (k, v) in a.iter().enumerate() {
                    out.push_str(&pad(ind + 1));
                    v.write(out, ind + 1);
                    out.push_str(if k + 1 < a.len() { ",\n" } else { "\n" });
                }
                out.push_str(&pad(ind));
                out.push(']');
            }
            J::Obj(o) => {
                out.push_str("{\n");
                for (k, (key, v)) in o.iter().enumerate() {
                    out.push_str(&pad(ind + 1));
                    esc(key, out);
                    out.push_str(": ");
                    v.write(out, ind + 1);
                    out.push_str(if k + 1 < o.len() { ",\n" } else { "\n" });
                }
                out.push_str(&pad(ind));
                out.push('}');
            }
        }
    }
    fn dump(&self) -> String {
        let mut s = String::new();
        self.write(&mut s, 0);
        s.push('\n');
        s
    }
}

fn esc(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
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

fn ju(v: u64) -> J {
    J::Num(v.to_string())
}
fn jf(v: f64) -> J {
    assert!(v.is_finite(), "non-finite number");
    J::Num(format!("{v}"))
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

// ------------------------------------------------------------- SHA-256 ----

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
    let bitlen = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for t in 0..16 {
            w[t] = u32::from_be_bytes([
                chunk[4 * t],
                chunk[4 * t + 1],
                chunk[4 * t + 2],
                chunk[4 * t + 3],
            ]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }
        let mut v = h;
        for t in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[t])
                .wrapping_add(w[t]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [
                t1.wrapping_add(t2),
                v[0],
                v[1],
                v[2],
                v[3].wrapping_add(t1),
                v[4],
                v[5],
                v[6],
            ];
        }
        for k in 0..8 {
            h[k] = h[k].wrapping_add(v[k]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}


fn minmax(v: &[f64]) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &x in v {
        lo = lo.min(x);
        hi = hi.max(x);
    }
    (lo, hi)
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

/// A printed figure with `dec` decimals in its mantissa agrees when it is the
/// recomputed value rounded to that precision; `rel_tol` is half a unit of the
/// last printed digit relative to the printed value.
struct Headlines {
    items: Vec<J>,
    bad: Vec<String>,
}

impl Headlines {
    fn add(
        &mut self,
        name: &str,
        source: &str,
        printed: &str,
        claimed: f64,
        half_unit: f64,
        recomputed: f64,
        def: &str,
    ) {
        let rel_tol = if half_unit == 0.0 {
            1e-12
        } else {
            half_unit / claimed.abs() * (1.0 + 1e-9)
        };
        let scale = claimed.abs().max(recomputed.abs()).max(1e-300);
        let agrees = (claimed - recomputed).abs() <= rel_tol * scale;
        if !agrees {
            self.bad.push(format!(
                "{name}: printed {printed}, recomputed {recomputed}"
            ));
        }
        self.items.push(obj(vec![
            ("name", js(name)),
            ("source", js(source)),
            ("printed", js(printed)),
            ("claimed", jf(claimed)),
            ("recomputed", jf(recomputed)),
            ("rel_tol", jf(rel_tol)),
            ("agrees", jb(agrees)),
            ("definition", js(def)),
        ]));
    }
}

// --------------------------------------------------------- comparisons ----

const TIMING: [&str; 3] = ["wall_ms", "solve_ms", "ms"];

/// Every leaf of `a` against `b`: (compared, mismatches, timing, only_a, only_b).
fn leaves(path: &str, a: &J, b: &J, out: &mut LeafDiff) {
    match (a, b) {
        (J::Obj(oa), J::Obj(ob)) => {
            for (k, va) in oa {
                let p = format!("{path}.{k}");
                match ob.iter().find(|(kb, _)| kb == k) {
                    Some((_, vb)) => leaves(&p, va, vb, out),
                    None => out.only_a.push(p),
                }
            }
            for (k, _) in ob {
                if !oa.iter().any(|(ka, _)| ka == k) {
                    out.only_b.push(format!("{path}.{k}"));
                }
            }
        }
        (J::Arr(xa), J::Arr(xb)) => {
            if xa.len() != xb.len() {
                out.mismatches.push((
                    format!("{path}.length"),
                    ju(xa.len() as u64),
                    ju(xb.len() as u64),
                ));
            }
            for (k, (va, vb)) in xa.iter().zip(xb).enumerate() {
                leaves(&format!("{path}[{k}]"), va, vb, out);
            }
        }
        _ => {
            let key = path.rsplit('.').next().unwrap_or("");
            if TIMING.contains(&key) {
                out.timing.push((path.to_string(), a.clone(), b.clone()));
                return;
            }
            out.compared += 1;
            let same = match (a, b) {
                (J::Num(x), J::Num(y)) => num_eq(x, y),
                _ => a == b,
            };
            if same {
                out.identical += 1;
            } else {
                out.mismatches
                    .push((path.to_string(), a.clone(), b.clone()));
            }
        }
    }
}

/// Two JSON numbers are equal when their text is; otherwise integers are
/// compared exactly (as i128: 64-bit digests do not survive an f64) and only
/// two non-integers by value.  An integer never equals a float's text.
fn num_eq(x: &str, y: &str) -> bool {
    if x == y {
        return true;
    }
    let is_int = |s: &str| !s.contains(['.', 'e', 'E']);
    match (is_int(x), is_int(y)) {
        (true, true) => match (x.parse::<i128>(), y.parse::<i128>()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        },
        (false, false) => matches!((x.parse::<f64>(), y.parse::<f64>()), (Ok(a), Ok(b)) if a == b),
        _ => false,
    }
}

#[derive(Default)]
struct LeafDiff {
    compared: u64,
    identical: u64,
    mismatches: Vec<(String, J, J)>,
    timing: Vec<(String, J, J)>,
    only_a: Vec<String>,
    only_b: Vec<String>,
}

/// Paths with array indices replaced by `*`, for grouping.
#[allow(dead_code)]
fn generic(path: &str) -> String {
    let mut out = String::new();
    let mut in_idx = false;
    for c in path.chars() {
        match c {
            '[' => {
                in_idx = true;
                out.push_str("[*]");
            }
            ']' => in_idx = false,
            _ if in_idx => {}
            _ => out.push(c),
        }
    }
    out
}

// ------------------------------------------------------------ the data ----

const PS: [u64; 5] = [53, 101, 149, 197, 269];
const SEEDS: [u64; 2] = [1, 2];
const GAE: f64 = 97.0;
const WEIGHTS: [(&str, f64); 3] = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)];
const E28: &str = "experiments/33_tau_stage_a";
const E19: &str = "experiments/27_quartic_pipeline_d4";
const E31: &str = "experiments/31_solve_cost";

struct Ctx {
    repo: String,
    ev: String,
}

impl Ctx {
    fn read(&self, rel: &str) -> (J, String) {
        let path = format!("{}/{}", self.repo, rel);
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let h = sha256(&bytes);
        (parse(std::str::from_utf8(&bytes).unwrap()), h)
    }
    fn text(&self, rel: &str) -> (String, String) {
        let path = format!("{}/{}", self.repo, rel);
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let h = sha256(&bytes);
        (String::from_utf8(bytes).unwrap(), h)
    }
}

fn first(j: &J) -> &J {
    match j {
        J::Arr(a) => &a[0],
        _ => j,
    }
}

#[derive(Clone, Debug)]
struct Walk {
    fold: String,
    seed: u64,
    steps: u64,
    gops: u64,
    muls: u64,
    vmuls: u64,
    esc: u64,
    correct: bool,
    rec: Option<u64>,
    digest: Option<u64>,
}

impl Walk {
    fn of(j: &J) -> Walk {
        Walk {
            fold: j.get("fold").s().to_string(),
            seed: j.get("seed").u(),
            steps: j.get("steps").u(),
            gops: j.get("group_ops").u(),
            muls: j.get("fp_muls").u(),
            vmuls: j.get("verify_fp_muls").opt_u().unwrap_or(0),
            esc: j.get("cycle_escapes").u(),
            correct: j.get("correct").b(),
            rec: j.get("recovered").opt_u(),
            digest: j.get("visit_digest").opt_u(),
        }
    }
    /// E25's convention: the walk's `[d]G = Q` check is not charged.
    fn walk_muls(&self) -> u64 {
        self.muls - self.vmuls
    }
}

fn walks(j: &J) -> Vec<Walk> {
    j.arr().iter().map(Walk::of).collect()
}

fn mean_of(ws: &[&Walk], f: impl Fn(&Walk) -> f64) -> f64 {
    ws.iter().map(|w| f(w)).sum::<f64>() / ws.len() as f64
}

fn isqrt(v: u128) -> u128 {
    let mut r = (v as f64).sqrt() as u128;
    while r * r > v {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= v {
        r += 1;
    }
    r
}

fn is_square(v: i128) -> bool {
    v >= 0 && {
        let r = isqrt(v as u128);
        r * r == v as u128
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

/// Independent number theory on (p, n, h) alone: is `#E = h·n` the order of
/// a GLS twist (quadratic twist over F_{p⁴} of a curve E'/F_{p²} of trace t₂,
/// `#E = p⁴ + 1 + t₂² − 2p²`), and does the trace rule out the cases in which
/// a fold larger than ⟨±ψ⟩ would exist (j ∈ F_p, j = 0, j = 1728)?
fn gls_order_checks(p: u64, n: u64, h: u64) -> J {
    let (p, n, h) = (p as i128, n as i128, h as i128);
    let order = h * n;
    let p2 = p * p;
    let p4 = p2 * p2;
    let t2sq = order - p4 - 1 + 2 * p2;
    let gls = is_square(t2sq);
    let t2 = if gls { isqrt(t2sq as u128) as i128 } else { -1 };
    let base_change_form = is_square(2 * p2 + p4 + 1 - order);
    let hasse = (order - p4 - 1).abs() <= 2 * p2;
    // j ∈ F_p (Aut = ±1): t₂ = ±(t² − 2p), t² ≤ 4p.
    let j_in_fp = gls
        && [2 * p + t2, 2 * p - t2]
            .iter()
            .any(|&v| v >= 0 && v <= 4 * p && is_square(v));
    let ordinary = gls && t2 % p != 0;
    let disc = 4 * p2 - t2 * t2; // −(t₂² − 4p²) > 0 for ordinary
    let cm_qi = gls && is_square(disc);
    let cm_qw = gls && disc % 3 == 0 && is_square(disc / 3);
    let n_prime = is_prime(n as u64);
    let structure_ok = gls && hasse && n_prime && ordinary && !cm_qi && !cm_qw;
    obj(vec![
        ("p", ju(p as u64)),
        ("n", ju(n as u64)),
        ("h", ju(h as u64)),
        ("n_prime", jb(n_prime)),
        ("hasse", jb(hasse)),
        ("gls_twist_order_shape", jb(gls)),
        ("t2", J::Num(t2.to_string())),
        ("untwisted_base_change_shape", jb(base_change_form)),
        ("j_in_fp_possible", jb(j_in_fp)),
        ("ordinary", jb(ordinary)),
        ("cm_by_q_i_possible", jb(cm_qi)),
        ("cm_by_q_sqrt_minus3_possible", jb(cm_qw)),
        ("structure_ok", jb(structure_ok)),
        ("order_screen_rules_out_j_in_fp", jb(gls && !j_in_fp)),
    ])
}

/// A figure as printed: `1.74·10^7`, `0.45`, `5519`, `1.36·10⁹`.  Returns the
/// value and half a unit of its last printed digit (0 for an integer).
fn printed(s: &str) -> (f64, f64) {
    let t = s.trim().trim_matches('`').replace(',', "");
    let sup = |c: char| match c {
        '⁰' => Some('0'),
        '¹' => Some('1'),
        '²' => Some('2'),
        '³' => Some('3'),
        '⁴' => Some('4'),
        '⁵' => Some('5'),
        '⁶' => Some('6'),
        '⁷' => Some('7'),
        '⁸' => Some('8'),
        '⁹' => Some('9'),
        '⁻' => Some('-'),
        _ => None,
    };
    let (mant, exp) = if let Some((m, e)) = t.split_once("·10") {
        let e: String = e.trim_start_matches('^').chars().map(|c| sup(c).unwrap_or(c)).collect();
        (m.to_string(), e.parse::<i32>().unwrap_or_else(|_| panic!("bad exponent in {s}")))
    } else {
        (t.clone(), 0)
    };
    let v: f64 = mant.parse().unwrap_or_else(|_| panic!("not a number: {s}"));
    let dec = mant.split_once('.').map(|(_, d)| d.len() as i32).unwrap_or(0);
    let half = if dec == 0 && exp == 0 { 0.0 } else { 0.5 * 10f64.powi(exp - dec) };
    (v * 10f64.powi(exp), half)
}

fn cells(line: &str) -> Vec<String> {
    let t = line.trim().trim_start_matches('|').trim_end_matches('|');
    t.split('|').map(|c| c.trim().to_string()).collect()
}

/// The rows of the markdown table under the heading that starts with `head`.
fn md_table(text: &str, head: &str) -> Vec<Vec<String>> {
    let mut on = false;
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.starts_with("## ") {
            on = line.starts_with(head);
            continue;
        }
        if on && line.starts_with('|') && !line.starts_with("|--") && !line.starts_with("| p ") {
            rows.push(cells(line));
        }
    }
    rows
}

// --------------------------------------------------------- comparisons ----

fn replay_files(ctx: &Ctx) -> Vec<(String, String, String)> {
    let mut files = vec![(
        "stage0".to_string(),
        format!("{E28}/stage0.json"),
        format!("{}/replay/stage0.json", ctx.ev),
    )];
    for p in PS {
        for s in SEEDS {
            files.push((
                format!("stage_a_p{p}s{s}"),
                format!("{E28}/stage_a_p{p}s{s}.json"),
                format!("{}/replay/stage_a_p{p}s{s}.json", ctx.ev),
            ));
        }
    }
    files.push((
        "control_decomposed".to_string(),
        format!("{E28}/control_decomposed.json"),
        format!("{}/replay/control_decomposed.json", ctx.ev),
    ));
    files
}

fn diff_entry(name: &str, committed: &str, ha: &str, replay: &str, hb: &str, d: &LeafDiff) -> J {
    let ok = d.mismatches.is_empty() && d.only_a.is_empty() && d.only_b.is_empty();
    let trip = |v: &Vec<(String, J, J)>| {
        J::Arr(
            v.iter()
                .map(|(p, x, y)| obj(vec![("path", js(p)), ("committed", x.clone()), ("replay", y.clone())]))
                .collect(),
        )
    };
    obj(vec![
        ("run", js(name)),
        ("committed", obj(vec![("path", js(committed)), ("sha256", js(ha))])),
        ("replay", obj(vec![("path", js(replay)), ("sha256", js(hb))])),
        ("fields_compared", ju(d.compared)),
        ("fields_identical", ju(d.identical)),
        ("mismatches", trip(&d.mismatches)),
        ("only_in_committed", J::Arr(d.only_a.iter().map(|p| js(p)).collect())),
        ("only_in_replay", J::Arr(d.only_b.iter().map(|p| js(p)).collect())),
        ("timing_not_compared", trip(&d.timing)),
        ("identical", jb(ok)),
    ])
}

fn compare(ctx: &Ctx) -> J {
    let mut runs = Vec::new();
    let (mut all_c, mut all_i, mut all_ok) = (0u64, 0u64, true);
    for (name, committed, replay) in replay_files(ctx) {
        let (a, ha) = ctx.read(&committed);
        let (b, hb) = ctx.read(&replay);
        let mut d = LeafDiff::default();
        leaves("$", &a, &b, &mut d);
        let e = diff_entry(&name, &committed, &ha, &replay, &hb, &d);
        all_ok &= e.get("identical").b();
        all_c += d.compared;
        all_i += d.identical;
        runs.push(e);
    }
    obj(vec![
        ("schema", js("e28-replay-comparison/v1")),
        ("tool", js(&format!("{}/validate_e28.rs compare", ctx.ev))),
        ("rule", js("every leaf of every committed record against the replay: integers and floats by their text (equal text, or equal value with the same integer/float form; integers compared exactly as i128), strings and booleans exactly; wall_ms (Stage A) and ms (each solve-cost control call) are wall-clock and listed, not compared")),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_identical", ju(runs.iter().filter(|r| r.get("identical").b()).count() as u64)),
        ("fields_compared", ju(all_c)),
        ("fields_identical", ju(all_i)),
        ("all_identical", jb(all_ok)),
        ("runs", J::Arr(runs)),
    ])
}

/// The comparator's own controls: a committed record against itself (all
/// identical) and against copies with one integer, one float, one 64-bit
/// digest and one boolean changed (each must be caught).
fn comparator_controls(ctx: &Ctx) -> J {
    let (a, _) = ctx.read(&format!("{E28}/stage_a_p53s1.json"));
    let mut d = LeafDiff::default();
    leaves("$", &a, &a, &mut d);
    let self_ok = d.mismatches.is_empty() && d.compared > 0;
    let mut out = vec![obj(vec![
        ("control", js("record against itself")),
        ("fields_compared", ju(d.compared)),
        ("mismatches", ju(d.mismatches.len() as u64)),
        ("expected", js("0 mismatches")),
        ("holds", jb(self_ok)),
    ])];
    let perturb = |text: &str, from: &str, to: &str| -> J {
        assert!(text.contains(from), "control anchor {from} missing");
        parse(&text.replacen(from, to, 1))
    };
    let (t, _) = ctx.text(&format!("{E28}/stage_a_p53s1.json"));
    let cases = [
        ("residuals_at_solve 5519 -> 5520", "\"residuals_at_solve\": 5519", "\"residuals_at_solve\": 5520"),
        ("generation_muls_per_target last digit", "6534.790948275862", "6534.790948275863"),
        ("visit_digest + 1 (beyond f64 precision)", "1871561872930114859", "1871561872930114860"),
        ("tau.correct true -> false", "\"correct\": true", "\"correct\": false"),
    ];
    let mut all = self_ok;
    for (name, from, to) in cases {
        let b = perturb(&t, from, to);
        let mut d = LeafDiff::default();
        leaves("$", &a, &b, &mut d);
        let caught = d.mismatches.len() == 1;
        all &= caught;
        out.push(obj(vec![
            ("control", js(name)),
            ("mismatches", ju(d.mismatches.len() as u64)),
            ("expected", js("exactly 1 mismatch")),
            ("holds", jb(caught)),
        ]));
    }
    obj(vec![("all_hold", jb(all)), ("cases", J::Arr(out))])
}

// ----------------------------------------------------------- recompute ----

struct Checks {
    items: Vec<J>,
    bad: Vec<String>,
}

impl Checks {
    fn add(&mut self, id: &str, what: &str, holds: bool, detail: String) {
        if !holds {
            self.bad.push(format!("{id}: {what}: {detail}"));
        }
        self.items.push(obj(vec![
            ("id", js(id)),
            ("check", js(what)),
            ("holds", jb(holds)),
            ("detail", js(&detail)),
        ]));
    }
}

struct Row {
    p: u64,
    s: u64,
    n: u64,
    rt: f64,
    rp: f64,
    relt: f64,
    relp: f64,
    lat: f64,
    lap: f64,
    g: f64,
    c_med: f64,
    c_mean: f64,
    rho_g_i: f64,
    rho_n_i: f64,
    rho_g_iii: f64,
    rho_n_iii: f64,
    steps_g: f64,
    steps_n: f64,
    e19_ratio: f64,
    e19_entries: f64,
}

impl Row {
    fn s_model(&self, r: f64, la: f64, c: f64, w: f64) -> f64 {
        r * c + r * self.g + 16.0 * la + w * GAE * r * self.e19_entries
    }
    fn d_psi(&self, w: f64) -> f64 {
        self.s_model(self.rp, self.lap, self.c_med, w) / (self.rho_g_i + w * GAE * self.steps_g)
    }
    fn d_psi_iii(&self, w: f64) -> f64 {
        self.s_model(self.rp, self.lap, self.c_med, w) / (self.rho_g_iii + w * GAE * self.steps_g)
    }
    fn d_tau(&self, w: f64) -> f64 {
        self.s_model(self.rt, self.lat, self.c_med, w) / (self.rho_n_i + w * GAE * self.steps_n)
    }
}


fn report(ctx: &Ctx) -> J {
    let mut raw: Vec<J> = Vec::new();
    let pin = |rel: &str, h: &str, raw: &mut Vec<J>| {
        raw.push(obj(vec![("path", js(&format!("../../{rel}"))), ("sha256", js(h))]));
    };
    let mut ck = Checks { items: Vec::new(), bad: Vec::new() };
    let mut hl = Headlines { items: Vec::new(), bad: Vec::new() };
    let (tables, h_tables) = ctx.text(&format!("{E28}/tables.md"));
    pin(&format!("{E28}/tables.md"), &h_tables, &mut raw);

    // ---------------- Stage 0 ----------------
    let (s0, h0) = ctx.read(&format!("{E28}/stage0.json"));
    pin(&format!("{E28}/stage0.json"), &h0, &mut raw);
    let rows0 = s0.arr();
    let flags = [
        "j_in_fp2",
        "psi_squared_is_minus_one",
        "psi_g_is_lambda_g",
        "lambda_squared_is_minus_one",
        "psi_fixes_t",
        "line_checks",
    ];
    let mut nt0 = Vec::new();
    let (mut all_flags, mut h4, mut cols_ok, mut pts_ok, mut dens_ok, mut nt_ok) = (true, 0u64, true, true, true, true);
    let mut dens_by_p: Vec<(u64, f64)> = Vec::new();
    for r in rows0 {
        assert!(matches!(r.get("error"), J::Null), "stage 0 error row");
        let p = r.get("p").u();
        for f in flags {
            all_flags &= r.get(f).b();
        }
        if r.get("cofactor").u() == 4 {
            h4 += 1;
        }
        let (b, ct, cp) = (r.get("base_points").u(), r.get("cols_tau").u(), r.get("cols_tau_psi").u());
        cols_ok &= ct == 2 * cp && b == 2 * ct;
        pts_ok &= r.get("points_per_column_tau").f() == 2.0 && r.get("points_per_column_tau_psi").f() == 4.0;
        dens_ok &= (r.get("density").f() - b as f64 / (p - 5) as f64).abs() < 1e-12;
        let nt = gls_order_checks(p, r.get("n").u(), r.get("cofactor").u());
        nt_ok &= nt.get("structure_ok").b() && !nt.get("untwisted_base_change_shape").b();
        nt0.push(nt);
    }
    for p in PS {
        let (sb, sd): (u64, u64) = rows0
            .iter()
            .filter(|r| r.get("p").u() == p)
            .fold((0, 0), |(a, b), r| (a + r.get("base_points").u(), b + p - 5));
        dens_by_p.push((p, sb as f64 / sd as f64));
    }
    ck.add("S0.1", "Stage 0: 40 curves (8 seeds x 5 sizes), no error row", rows0.len() == 40, format!("{} rows", rows0.len()));
    ck.add("S0.2", "Stage 0: psi^2 = -1, psi(G) = [lambda]G, lambda^2 = -1, psi(T) = T, j in F_p2 and the line checks all true on every curve (the binary's own checks, replayed)", all_flags, format!("all flags true: {all_flags}"));
    ck.add("S0.3", "Stage 0: h = 4 on all 40", h4 == 40, format!("{h4} of 40"));
    ck.add("S0.4", "Stage 0: cols_tau = 2 cols_tau_psi and base = 2 cols_tau on every curve (2 / 4 abscissae a column; 4 / 8 points with ±)", cols_ok && pts_ok, format!("cols {cols_ok}, points per column {pts_ok}"));
    ck.add("S0.5", "Stage 0: density = base / (p - 5) on every curve", dens_ok, format!("{dens_ok}"));
    // The decisive j check: replay/curve_check.json (own F_p4 arithmetic on the dumped coefficients).
    let (cc, hcc) = ctx.read(&format!("{}/replay/curve_check.json", ctx.ev));
    let cc_rows = cc.arr();
    let cc_match = cc_rows.len() == 40
        && rows0.iter().all(|r| {
            cc_rows.iter().any(|c| {
                c.get("p").u() == r.get("p").u() && c.get("seed").u() == r.get("seed").u() && c.get("n").u() == r.get("n").u() && c.get("h").u() == r.get("cofactor").u()
            })
        });
    let cc_ok = cc_rows.iter().all(|c| c.get("ok").b() && !c.get("j_in_fp").b() && c.get("j_in_fp2").b());
    let screen_hits = nt0.iter().filter(|x| !x.get("order_screen_rules_out_j_in_fp").b()).count();
    ck.add("S0.6", "Validator's own number theory on (p, n, h) of all 40 curves: n prime, h n within Hasse and of GLS-twist shape (p^4 + 1 + t2^2 - 2p^2 with t2^2 a square; the untwisted shape is not), ordinary, no CM by Q(i) or Q(sqrt -3) (so j is neither 1728 nor 0)", nt_ok, format!("{} of 40; the order screen alone (no t with t^2 = 2p ± t2) rules out j in F_p on {} of 40, the other {} are decided by S0.7", nt0.iter().filter(|x| x.get("structure_ok").b()).count(), 40 - screen_hits, screen_hits));
    ck.add("S0.7", "Validator's own F_p4 arithmetic on the 40 curves' coefficients (replay/curve_check.rs on replay/params.txt, sha256 recorded): G on the curve, [n]G = O, n prime, h n in the Hasse interval (so #E = 4n), j = 1728*4a^3/(4a^3+27b^2) in F_p2 but not in F_p, psi(G) = [lambda]G, psi^2(G) = -G, lambda^2 = -1 mod n; (p, seed, n, h) equal the Stage 0 records. With S0.6: Aut(E) = ±1 and psi is the only Frobenius-type endomorphism over F_p4, so <±psi> is the largest fold and the <±psi> walk the strongest rho available", cc_ok && cc_match, format!("{} of {} curves ok; records match: {cc_match}; curve_check.json sha256 {hcc}", cc_rows.iter().filter(|c| c.get("ok").b()).count(), cc_rows.len()));
    let (dlo, dhi) = minmax(&dens_by_p.iter().map(|x| x.1).collect::<Vec<_>>());
    // tables.md Stage 0
    for row in md_table(&tables, "## Stage 0") {
        let p: u64 = row[0].parse().unwrap();
        let d = dens_by_p.iter().find(|x| x.0 == p).unwrap().1;
        let (v, h) = printed(&row[5]);
        hl.add(&format!("stage0_density_pooled_p{p}"), "experiments/33_tau_stage_a/tables.md, Stage 0", &row[5], v, h, d, "sum of base points / sum of (p - 5) over the 8 seeds");
    }
    hl.add("stage0_density_pooled_min", "note §9.30 Stage 0 (\"density pooled 0.50-0.52 per size\")", "0.50", 0.50, 0.005, dlo, "least pooled density over the five sizes");
    hl.add("stage0_density_pooled_max", "note §9.30 Stage 0", "0.52", 0.52, 0.005, dhi, "largest pooled density over the five sizes");
    hl.add("stage0_curves_h4", "note §9.30 Stage 0 (\"h = 4 on all 40\")", "40", 40.0, 0.0, h4 as f64, "curves with cofactor 4");

    // ---------------- E19 and E31 references ----------------
    let mut e19 = Vec::new(); // (p, s, ratio, ratio_recomputed, entries/residual, call median, call mean, total, residuals, la, gen)
    for p in PS {
        for s in SEEDS {
            let rel = format!("{E19}/p{p}s{s}.json");
            let (j, h) = ctx.read(&rel);
            pin(&rel, &h, &mut raw);
            let r = first(&j);
            let tot = r.get("total_muls").f();
            let rho = r.get("rho_matched_muls").f();
            e19.push((
                p,
                s,
                r.get("ratio").f(),
                tot / rho,
                r.get("macaulay_entries").f() / r.get("residuals").f(),
                r.get("call").get("median").f(),
                r.get("call").get("mean").f(),
                tot,
                r.get("folded").get("residuals_at_solve").f(),
                r.get("folded").get("la_ops").f(),
                r.get("generation_muls").f(),
            ));
        }
    }
    let e19_ratio_ok = e19.iter().all(|x| ((x.2 - x.3) / x.3).abs() < 1e-12);
    ck.add("REF.1", "E19's recorded ratio equals its total_muls / rho_matched_muls on all 10 rows", e19_ratio_ok, format!("{e19_ratio_ok}"));
    let mut e31 = Vec::new(); // (p, s, arm, ratio)
    let mut e31_ok = true;
    for p in [101u64, 149, 197, 269, 397] {
        for s in SEEDS {
            let rel = format!("{E31}/e2e_chain_p{p}s{s}.json");
            let (j, h) = ctx.read(&rel);
            pin(&rel, &h, &mut raw);
            for r in j.arr() {
                let rc = r.get("total_muls").f() / r.get("rho_matched_muls").f();
                e31_ok &= ((rc - r.get("ratio").f()) / rc).abs() < 1e-12 && r.get("solver").s() == "chain";
                e31.push((p, s, r.get("arm").s().to_string(), rc));
            }
        }
    }
    ck.add("REF.2", "E31 solver A (chain) records: 20 rows, ratio = total_muls / rho_matched_muls", e31_ok && e31.len() == 20, format!("{} rows", e31.len()));
    let (c19lo, c19hi) = minmax(&e19.iter().map(|x| x.5).collect::<Vec<_>>());
    hl.add("e19_call_median_min", "note §9.26 (E19's C \"1.37-1.41·10⁹\")", "1.37·10⁹", 1.37e9, 0.005e9, c19lo, "least E19 per-call median (call.median) over its 10 rows");
    hl.add("e19_call_median_max", "note §9.26", "1.41·10⁹", 1.41e9, 0.005e9, c19hi, "largest E19 per-call median over its 10 rows");

    // ---------------- Stage A ----------------
    let (cd, hcd) = ctx.read(&format!("{E28}/control_decomposed.json"));
    pin(&format!("{E28}/control_decomposed.json"), &hcd, &mut raw);
    let mut rows: Vec<Row> = Vec::new();
    let mut stage_a_out = Vec::new();
    let (mut walks_total, mut walks_correct) = (0u64, 0u64);
    let (mut logs_ok, mut ident_ok, mut cols2, mut med8, mut ctrl_ok, mut supp_ok, mut hist_ok) =
        (true, true, true, true, true, true, true);
    let (mut sum_rt, mut sum_rp, mut sum_relt, mut sum_relp) = (0f64, 0f64, 0f64, 0f64);
    let mut c_uni = Vec::new();
    let mut c_dec = Vec::new();
    let mut supp_decs = BTreeSet::new();
    let mut within30 = Vec::new();
    let mut nt_a = Vec::new();
    for p in PS {
        for s in SEEDS {
            let rel = format!("{E28}/stage_a_p{p}s{s}.json");
            let (j, h) = ctx.read(&rel);
            pin(&rel, &h, &mut raw);
            let r = first(&j);
            assert!(matches!(r.get("error"), J::Null));
            let (tau, tp) = (r.get("tau"), r.get("tau_psi"));
            nt_a.push(gls_order_checks(p, r.get("n").u(), r.get("cofactor").u()));
            cols2 &= tau.get("columns").u() == 2 * tp.get("columns").u();
            // decompositions per decomposed target
            let hist: Vec<u64> = r.get("decompositions_per_target").arr().iter().map(|x| x.u()).collect();
            let mut dv = Vec::new();
            for (k, &c) in hist.iter().enumerate().skip(1) {
                for _ in 0..c {
                    dv.push(k as f64);
                }
            }
            med8 &= median(&dv) == 8.0;
            hist_ok &= hist.iter().sum::<u64>() == r.get("residuals").u()
                && hist[1..].iter().sum::<u64>() == r.get("targets_decomposed").u()
                && (hist[9] > 0
                    || hist.iter().enumerate().map(|(k, &c)| k as u64 * c).sum::<u64>() == r.get("decompositions").u());
            // logarithms and walks
            let sb = walks(r.get("rho_schoolbook"));
            let e27 = walks(r.get("rho_e27"));
            let d = tau.get("recovered").u();
            let mut ok = tau.get("solved").b() && tau.get("correct").b() && tp.get("solved").b() && tp.get("correct").b()
                && tp.get("recovered").u() == d;
            for w in sb.iter().chain(&e27) {
                walks_total += 1;
                if w.correct {
                    walks_correct += 1;
                }
                ok &= w.rec == Some(d);
            }
            logs_ok &= ok;
            for fold in ["Gls", "Negation"] {
                let a: Vec<&Walk> = sb.iter().filter(|w| w.fold == fold).collect();
                let b: Vec<&Walk> = e27.iter().filter(|w| w.fold == fold).collect();
                ident_ok &= a.len() == 16 && b.len() == 16;
                for k in 0..16u64 {
                    let x = a.iter().find(|w| w.seed == s * 1000 + k);
                    let y = b.iter().find(|w| w.seed == s * 1000 + k);
                    ident_ok &= match (x, y) {
                        (Some(x), Some(y)) => {
                            x.steps == y.steps && x.gops == y.gops && x.rec == y.rec && x.digest == y.digest && x.esc == y.esc
                        }
                        _ => false,
                    };
                }
            }
            let g_i: Vec<&Walk> = sb.iter().filter(|w| w.fold == "Gls").collect();
            let n_i: Vec<&Walk> = sb.iter().filter(|w| w.fold == "Negation").collect();
            let g_iii: Vec<&Walk> = e27.iter().filter(|w| w.fold == "Gls").collect();
            let n_iii: Vec<&Walk> = e27.iter().filter(|w| w.fold == "Negation").collect();
            // registered control
            let calls = r.get("control_calls").arr();
            let cm: Vec<f64> = calls.iter().map(|c| c.get("call_muls").f()).collect();
            ctrl_ok &= calls.len() == 3
                && calls.iter().all(|c| {
                    c.get("solved").b() && c.get("agrees").b() && c.get("decompositions").u() == 0 && c.get("mitm_decompositions").u() == 0
                });
            c_uni.push(median(&cm));
            // supplementary control
            let sc = cd.arr().iter().find(|x| x.get("p").u() == p && x.get("seed").u() == s).expect("supplementary row");
            let scalls = sc.get("calls").arr();
            let sm: Vec<f64> = scalls.iter().map(|c| c.get("call_muls").f()).collect();
            supp_ok &= scalls.len() == 3
                && scalls.iter().all(|c| {
                    c.get("solved").b()
                        && c.get("agrees").b()
                        && c.get("decompositions").u() == c.get("mitm_decompositions").u()
                        && c.get("mitm_decompositions").u() > 0
                });
            for c in scalls {
                supp_decs.insert(c.get("decompositions").u());
            }
            c_dec.push(median(&sm));
            // E19 at the same p
            let e19p: Vec<_> = e19.iter().filter(|x| x.0 == p).collect();
            let e19_ratio = e19p.iter().map(|x| x.2).sum::<f64>() / e19p.len() as f64;
            let e19_entries = e19p.iter().map(|x| x.4).sum::<f64>() / e19p.len() as f64;
            let e19_cmed = e19p.iter().map(|x| x.5).sum::<f64>() / e19p.len() as f64;
            within30.push((p, s, median(&cm) / e19_cmed, median(&sm) / e19_cmed));
            let (rt, rp) = (tau.get("residuals_at_solve").f(), tp.get("residuals_at_solve").f());
            let (relt, relp) = (tau.get("relations").f(), tp.get("relations").f());
            sum_rt += rt;
            sum_rp += rp;
            sum_relt += relt;
            sum_relp += relp;
            let row = Row {
                p,
                s,
                n: r.get("n").u(),
                rt,
                rp,
                relt,
                relp,
                lat: tau.get("la_ops").f(),
                lap: tp.get("la_ops").f(),
                g: r.get("generation_muls_per_target").f(),
                c_med: median(&cm),
                c_mean: cm.iter().sum::<f64>() / cm.len() as f64,
                rho_g_i: mean_of(&g_i, |w| w.walk_muls() as f64),
                rho_n_i: mean_of(&n_i, |w| w.walk_muls() as f64),
                rho_g_iii: mean_of(&g_iii, |w| w.walk_muls() as f64),
                rho_n_iii: mean_of(&n_iii, |w| w.walk_muls() as f64),
                steps_g: mean_of(&g_i, |w| w.steps as f64),
                steps_n: mean_of(&n_i, |w| w.steps as f64),
                e19_ratio,
                e19_entries,
            };
            stage_a_out.push(obj(vec![
                ("p", ju(p)),
                ("seed", ju(s)),
                ("n", ju(row.n)),
                ("log", ju(d)),
                ("cols_tau", ju(tau.get("columns").u())),
                ("cols_tau_psi", ju(tp.get("columns").u())),
                ("residuals_tau", jf(rt)),
                ("residuals_tau_psi", jf(rp)),
                ("residuals_ratio", jf(rt / rp)),
                ("y_tau", jf(rt / relt)),
                ("y_tau_psi", jf(rp / relp)),
                ("decs_per_decomposed_target_median", jf(median(&dv))),
                ("generation_muls_per_target", jf(row.g)),
                ("la_ops_tau", jf(row.lat)),
                ("la_ops_tau_psi", jf(row.lap)),
                ("control_c_median_uniform", jf(row.c_med)),
                ("control_c_mean_uniform", jf(row.c_mean)),
                ("control_c_median_decomposed", jf(median(&sm))),
                ("rho_psi_i", jf(row.rho_g_i)),
                ("rho_neg_i", jf(row.rho_n_i)),
                ("rho_psi_iii", jf(row.rho_g_iii)),
                ("rho_neg_iii", jf(row.rho_n_iii)),
                ("steps_psi", jf(row.steps_g)),
                ("steps_neg", jf(row.steps_n)),
                ("e19_entries_per_residual_same_p", jf(e19_entries)),
                ("e19_ratio_same_p_mean_of_seeds", jf(e19_ratio)),
            ]));
            rows.push(row);
        }
    }
    let nt_a_ok = nt_a.iter().all(|x| x.get("structure_ok").b())
        && rows.iter().all(|r| cc_rows.iter().any(|c| c.get("p").u() == r.p && c.get("seed").u() == r.s && c.get("n").u() == r.n && c.get("ok").b() && !c.get("j_in_fp").b()));
    ck.add("A.1", "Stage A: cols tau / tau-psi = 2.00 on every curve", cols2, format!("{cols2}"));
    ck.add("A.2", "Stage A: decompositions per decomposed target, median 8 on every curve (from the recorded histogram)", med8, format!("{med8}"));
    ck.add("A.3", "Stage A: histogram consistent with residuals, targets_decomposed and decompositions", hist_ok, format!("{hist_ok}"));
    ck.add("A.4", "Stage A: both arms solved and correct (the binary checks the Wiedemann logarithm against the planted d and [x]G = Q) and recover the same d as all 64 walks of the curve, on all 10 curves", logs_ok, format!("{logs_ok}"));
    ck.add("A.5", "Stage A: all 640 walks correct (10 curves x {<±psi>, negation} x 16 x {E25 schoolbook serial, E27 Karatsuba batched})", walks_correct == walks_total && walks_total == 640, format!("{walks_correct} of {walks_total}"));
    ck.add("A.6", "Stage A: the E25-arithmetic and E27-arithmetic walks are the same walks (steps, group ops, recovered d, visit digest, cycle escapes equal per seed and fold)", ident_ok, format!("{ident_ok}"));
    ck.add("A.7", "Registered control: 3 uniform targets per curve, 30 calls, every call solved, 0 solver and 0 oracle decompositions (agreement 0 = 0: a negative control only)", ctrl_ok, format!("{ctrl_ok}"));
    ck.add("A.8", "Supplementary control (added after the run, disclosed in §9.30): 3 oracle-decomposable targets per curve, 30 calls, every call solved and its decomposition set equal to the oracle's (agrees = solved && sets equal)", supp_ok, format!("{supp_ok}; decomposition counts seen {:?}", supp_decs));
    ck.add("A.9", "The 10 Stage A curves are Stage 0's seeds 1-2 (same p, seed, n) and pass S0.6 and S0.7: the <±psi> walk is the strongest rho available on every Stage A curve", nt_a_ok, format!("{nt_a_ok}"));
    let w30 = within30.iter().all(|x| (x.2 - 1.0).abs() <= 0.30 && (x.3 - 1.0).abs() <= 0.30);
    let (w30lo, w30hi) = minmax(&within30.iter().flat_map(|x| [x.2, x.3]).collect::<Vec<_>>());
    ck.add("A.10", "G2.102: every control's median C within ±30 % of E19's per-call median at the same p (mean of E19's two seeds)", w30, format!("C / C_E19 in [{w30lo:.4}, {w30hi:.4}]"));

    // Stage A headlines from tables.md, and pooled/range claims from the note.
    for (row, md) in rows.iter().zip(md_table(&tables, "## Stage A (fold")) {
        assert_eq!(md[0].parse::<u64>().unwrap(), row.p);
        assert_eq!(md[1].parse::<u64>().unwrap(), row.s);
        let tag = format!("p{}s{}", row.p, row.s);
        let src = "experiments/33_tau_stage_a/tables.md, Stage A";
        let add = |hl: &mut Headlines, name: &str, cell: &str, val: f64, def: &str| {
            let (v, h) = printed(cell);
            hl.add(&format!("{name}_{tag}"), src, cell, v, h, val, def);
        };
        add(&mut hl, "residuals_tau", &md[6], row.rt, "tau.residuals_at_solve");
        add(&mut hl, "residuals_tau_psi", &md[7], row.rp, "tau_psi.residuals_at_solve");
        add(&mut hl, "y_tau", &md[8], row.rt / row.relt, "residuals_at_solve / relations, tau arm");
        add(&mut hl, "y_tau_psi", &md[9], row.rp / row.relp, "residuals_at_solve / relations, tau-psi arm");
        add(&mut hl, "residuals_ratio", &md[10], row.rt / row.rp, "R tau / R tau-psi");
        add(&mut hl, "control_c_median_uniform", &md[12], row.c_med, "median call_muls of the 3 registered (uniform) control calls");
        let supp_cell = md[14].split(" (").next().unwrap().to_string();
        let i = rows.iter().position(|x| x.p == row.p && x.s == row.s).unwrap();
        add(&mut hl, "control_c_median_decomposed", &supp_cell, c_dec[i], "median call_muls of the 3 supplementary (decomposable) control calls");
    }
    let rlist: Vec<f64> = rows.iter().map(|r| r.rt / r.rp).collect();
    let (rlo, rhi) = minmax(&rlist);
    hl.add("residuals_ratio_per_curve_min", "note §9.30 G2.101 (\"per curve the residuals ratio is 1.36-2.56\")", "1.36", 1.36, 0.005, rlo, "least R tau / R tau-psi");
    hl.add("residuals_ratio_per_curve_max", "note §9.30 G2.101", "2.56", 2.56, 0.005, rhi, "largest R tau / R tau-psi");
    hl.add("residuals_tau_pooled", "note §9.30 (\"residuals 40,749 / 23,345\")", "40,749", 40749.0, 0.0, sum_rt, "sum of R tau");
    hl.add("residuals_tau_psi_pooled", "note §9.30", "23,345", 23345.0, 0.0, sum_rp, "sum of R tau-psi");
    hl.add("residuals_ratio_pooled", "note §9.30 (\"(1.75)\"; G2.101 band [1.6, 2.4])", "1.75", 1.75, 0.005, sum_rt / sum_rp, "sum R tau / sum R tau-psi");
    hl.add("y_tau_pooled", "note §9.30 (\"y 98.4 / 110.1\")", "98.4", 98.4, 0.05, sum_rt / sum_relt, "sum R / sum relations, tau");
    hl.add("y_tau_psi_pooled", "note §9.30", "110.1", 110.1, 0.05, sum_rp / sum_relp, "sum R / sum relations, tau-psi");
    hl.add("y_ratio_pooled", "note §9.30 (\"(1.12)\"; G2.101 band [0.8, 1.25])", "1.12", 1.12, 0.005, (sum_rp / sum_relp) / (sum_rt / sum_relt), "pooled y tau-psi / y tau");
    hl.add("walks_correct", "note §9.30 (\"all 640 walks correct\")", "640", 640.0, 0.0, walks_correct as f64, "walks with correct = true");
    let (culo, cuhi) = minmax(&c_uni);
    let (cdlo, cdhi) = minmax(&c_dec);
    hl.add("control_c_uniform_min", "note §9.30 G2.102 (\"1.36-1.44·10⁹ (uniform)\")", "1.36·10⁹", 1.36e9, 0.005e9, culo, "least per-curve median, registered control");
    hl.add("control_c_uniform_max", "note §9.30 G2.102", "1.44·10⁹", 1.44e9, 0.005e9, cuhi, "largest per-curve median, registered control");
    hl.add("control_c_decomposed_min", "note §9.30 G2.102 (\"1.37-1.48·10⁹ (decomposed)\")", "1.37·10⁹", 1.37e9, 0.005e9, cdlo, "least per-curve median, supplementary control");
    hl.add("control_c_decomposed_max", "note §9.30 G2.102", "1.48·10⁹", 1.48e9, 0.005e9, cdhi, "largest per-curve median, supplementary control");

    // ---------------- Stage A2 (model) ----------------
    let mut a2_out = Vec::new();
    let mut d0 = Vec::new();
    let mut d01 = Vec::new();
    let mut d1 = Vec::new();
    let mut d3 = Vec::new();
    let mut tau_over = Vec::new();
    let mut e19_over = Vec::new();
    for (row, md) in rows.iter().zip(md_table(&tables, "## Stage A2")) {
        assert_eq!(md[0].parse::<u64>().unwrap(), row.p);
        let tag = format!("p{}s{}", row.p, row.s);
        let src = "experiments/33_tau_stage_a/tables.md, Stage A2";
        let sp = row.s_model(row.rp, row.lap, row.c_med, 0.0);
        let st = row.s_model(row.rt, row.lat, row.c_med, 0.0);
        let dd: Vec<f64> = WEIGHTS.iter().map(|w| row.d_psi(w.1)).collect();
        let band: Vec<String> = md[4].split(" / ").map(|x| x.to_string()).collect();
        let add = |hl: &mut Headlines, name: &str, cell: &str, val: f64, def: &str| {
            let (v, h) = printed(cell);
            hl.add(&format!("{name}_{tag}"), src, cell, v, h, val, def);
        };
        add(&mut hl, "s_model_tau_psi", &md[2], sp, "R tau-psi * C + R tau-psi * g + 16 LA tau-psi (C the median of the 3 registered control calls)");
        add(&mut hl, "rho_psi_i", &md[3], row.rho_g_i, "mean of the 16 <±psi> walks, E25 arithmetic, fp_muls - verify_fp_muls");
        add(&mut hl, "dist_tau_psi_0gae", &band[0], dd[0], "S_model / rho_psi_i");
        add(&mut hl, "dist_tau_psi_0.1gae", &band[1], dd[1], "(S_model + 0.1*97*R*E19 entries per residual) / (rho + 0.1*97*mean steps)");
        add(&mut hl, "dist_tau_psi_1gae", &band[2], dd[2], "as above at 1 GAE");
        add(&mut hl, "s_model_tau", &md[5], st, "the same model on the tau arm");
        add(&mut hl, "rho_neg_i", &md[6], row.rho_n_i, "mean of the 16 negation walks, E25 arithmetic");
        add(&mut hl, "dist_tau_0gae", &md[7], row.d_tau(0.0), "S_model tau / rho_neg_i");
        add(&mut hl, "e19_ratio_same_p", &md[8], row.e19_ratio, "mean over E19's two seeds at p of total_muls / rho_matched_muls");
        add(&mut hl, "e19_over_e28", &md[9], row.e19_ratio / dd[0], "E19 measured S/rho / E28 modelled S/rho (0 GAE)");
        add(&mut hl, "dist_tau_psi_iii_0gae", &md[10], row.d_psi_iii(0.0), "S_model / rho_psi_iii (E27's walk (iii): Karatsuba, batched)");
        d0.push(dd[0]);
        d01.push(dd[1]);
        d1.push(dd[2]);
        d3.push(row.d_psi_iii(0.0));
        tau_over.push(row.d_tau(0.0) / dd[0]);
        e19_over.push(row.e19_ratio / dd[0]);
        let band3: Vec<J> = WEIGHTS
            .iter()
            .map(|w| {
                let ic = row.s_model(row.rp, row.lap, row.c_med, w.1);
                let rho = row.rho_g_iii + w.1 * GAE * row.steps_g;
                obj(vec![("weight", js(w.0)), ("ic", jf(ic)), ("rho", jf(rho)), ("ic_over_rho", jf(ic / rho))])
            })
            .collect();
        let s_mean = row.s_model(row.rp, row.lap, row.c_mean, 0.0);
        a2_out.push(obj(vec![
            ("p", ju(row.p)),
            ("seed", ju(row.s)),
            ("s_model_tau_psi", jf(sp)),
            ("s_model_tau", jf(st)),
            ("dist_tau_psi_walk_i", J::Arr(dd.iter().map(|&x| jf(x)).collect())),
            ("dist_tau_negwalk_i_0gae", jf(row.d_tau(0.0))),
            ("dist_tau_psi_walk_iii_band", J::Arr(band3)),
            ("e19_over_e28", jf(row.e19_ratio / dd[0])),
            ("sensitivity_c_mean_of_3_calls", jf(row.c_mean)),
            ("sensitivity_dist_walk_i_0gae_with_c_mean", jf(s_mean / row.rho_g_i)),
            ("sensitivity_e19_over_e28_with_c_mean", jf(row.e19_ratio / (s_mean / row.rho_g_i))),
        ]));
    }
    let r = |v: &Vec<f64>| minmax(v);
    let (a, b) = r(&d0);
    hl.add("dist_walk_i_0gae_min", "note §9.30 A2 (\"6.99·10⁵-1.74·10⁷x ... at 0 GAE\")", "6.99·10⁵", 6.99e5, 0.005e5, a, "least modelled S/rho, tau-psi vs <±psi> walk (i)");
    hl.add("dist_walk_i_0gae_max", "note §9.30 A2", "1.74·10⁷", 1.74e7, 0.005e7, b, "largest");
    let (a, b) = r(&d01);
    hl.add("dist_walk_i_0.1gae_min", "note §9.30 A2 (\"7.32·10⁵-1.92·10⁷ at 0.1\")", "7.32·10⁵", 7.32e5, 0.005e5, a, "least at 0.1 GAE");
    hl.add("dist_walk_i_0.1gae_max", "note §9.30 A2", "1.92·10⁷", 1.92e7, 0.005e7, b, "largest at 0.1 GAE");
    let (a, b) = r(&d1);
    hl.add("dist_walk_i_1gae_min", "note §9.30 A2 (\"9.09·10⁵-3.19·10⁷ at 1\")", "9.09·10⁵", 9.09e5, 0.005e5, a, "least at 1 GAE");
    hl.add("dist_walk_i_1gae_max", "note §9.30 A2", "3.19·10⁷", 3.19e7, 0.005e7, b, "largest at 1 GAE");
    let (a, b) = r(&d3);
    hl.add("dist_walk_iii_0gae_min", "note §9.30 A2 (\"against E27's walk (iii) 1.02·10⁶-2.55·10⁷\"; distance remaining \">= 1.0·10⁶x\")", "1.02·10⁶", 1.02e6, 0.005e6, a, "least modelled S/rho against walk (iii)");
    hl.add("dist_walk_iii_0gae_max", "note §9.30 A2", "2.55·10⁷", 2.55e7, 0.005e7, b, "largest against walk (iii)");
    let (a, b) = r(&tau_over);
    hl.add("tau_vs_taupsi_min", "note §9.30 A2 (\"1.30-2.82x below the tau arm against the negation walk\")", "1.30", 1.30, 0.005, a, "(S tau / rho neg) / (S tau-psi / rho psi), per curve");
    hl.add("tau_vs_taupsi_max", "note §9.30 A2", "2.82", 2.82, 0.005, b, "largest");
    let (a, b) = r(&e19_over);
    hl.add("e19_over_e28_min", "note §9.30 A2 / G2.103 (\"0.45-6.98x per curve\")", "0.45", 0.45, 0.005, a, "least E19 S/rho_neg / E28 S_model/rho_psi");
    hl.add("e19_over_e28_max", "note §9.30 A2", "6.98", 6.98, 0.005, b, "largest");
    let by_size_claim = [(53u64, "1.77"), (101, "4.29"), (149, "2.39"), (197, "1.83"), (269, "3.65")];
    let mut by_size = Vec::new();
    for (p, c) in by_size_claim {
        let v: Vec<f64> = rows.iter().zip(&e19_over).filter(|(x, _)| x.p == p).map(|(_, &v)| v).collect();
        let gm = (v[0] * v[1]).sqrt();
        by_size.push((p, gm));
        let (cv, h) = printed(c);
        hl.add(&format!("e19_over_e28_by_size_p{p}"), "note §9.30 A2 (\"by size (geometric mean of the two seeds) 1.77, 4.29, 2.39, 1.83, 3.65\")", c, cv, h, gm, "geometric mean of the two seeds' E19/E28");
    }
    // E31 A comparison
    let mut e31_cmp = Vec::new();
    for (p, c, half) in [(101u64, Some("16.6"), 0.05), (149, Some("3.3"), 0.05), (197, Some("52"), 0.5), (269, Some("21"), 0.5)] {
        let best = rows.iter().zip(&d0).filter(|(x, _)| x.p == p).map(|(_, &v)| v).fold(f64::INFINITY, f64::min);
        let worst = e31.iter().filter(|x| x.0 == p).map(|x| x.3).fold(0f64, f64::max);
        let f = best / worst;
        e31_cmp.push(obj(vec![("p", ju(p)), ("e28_best_row", jf(best)), ("e31_a_worst_row", jf(worst)), ("factor", jf(f)), ("ten_x_holds", jb(f >= 10.0))]));
        if let Some(c) = c {
            let (cv, _) = printed(c);
            let h = half;
            hl.add(&format!("e28_best_over_e31a_worst_p{p}"), "note §9.30 G2.103 (\">= 16.6x at p = 101, >= 52x at 197, >= 21x at 269, only 3.3x at 149\")", c, cv, h, f, "least E28 tau-psi S_model/rho (walk i) at p over the largest E31 solver-A S/rho at p (both arms, both seeds)");
        }
        if p == 149 {
            hl.add("e31a_worst_p149", "note §9.30 G2.103 (\"E31 A's worst row at p = 149 (2.79·10⁵)\")", "2.79·10⁵", 2.79e5, 0.005e5, worst, "largest E31 solver-A S/rho at p = 149");
        }
    }
    let med28 = median(&d0);
    let med31 = median(&e31.iter().map(|x| x.3).collect::<Vec<_>>());
    let med31_matched = median(&e31.iter().filter(|x| x.0 != 397).map(|x| x.3).collect::<Vec<_>>());
    hl.add("dist_remaining_model_walk_i", "note §9.30 (\"Distance remaining (model): >= 7·10⁵x the matched rho in E25's arithmetic\")", "7·10⁵", 7e5, 0.5e5, a_min(&d0), "least modelled S/rho against walk (i); printed with one significant figure, the exact least is 6.99·10⁵");

    // ---------------- model fidelity on E19's measured rows ----------------
    let mut fid = Vec::new();
    for x in &e19 {
        // x: (p, s, ratio, ratio_rc, entries/res, cmed, cmean, total, R, la, gen_total)
        let model_med = x.8 * x.5 + x.10 + 16.0 * x.9;
        let model_mean = x.8 * x.6 + x.10 + 16.0 * x.9;
        fid.push((x.0, x.1, x.7 / model_med, x.7 / model_mean));
    }
    let (flo, fhi) = minmax(&fid.iter().map(|x| x.2).collect::<Vec<_>>());
    let (fmlo, fmhi) = minmax(&fid.iter().map(|x| x.3).collect::<Vec<_>>());
    ck.add("M.1", "Model fidelity (validator's own control, not a registered check): E28's A2 formula with the median call applied to E19's own measured rows (R = residuals_at_solve, C = call.median, generation and LA as recorded) against E19's measured total_muls", true, format!("measured / modelled = {flo:.4}-{fhi:.4} (median C); {fmlo:.4}-{fmhi:.4} with the mean call: the median-based model under-prices E19's measured S by up to {:.0} %", (fhi - 1.0) * 100.0));

    // ---------------- §5a band for the gate (strongest walk) ----------------
    let band_med: Vec<J> = WEIGHTS
        .iter()
        .map(|w| {
            let ic: Vec<f64> = rows.iter().map(|r| r.s_model(r.rp, r.lap, r.c_med, w.1)).collect();
            let rho: Vec<f64> = rows.iter().map(|r| r.rho_g_iii + w.1 * GAE * r.steps_g).collect();
            let ratio: Vec<f64> = rows.iter().map(|r| r.d_psi_iii(w.1)).collect();
            let (lo, hi) = minmax(&ratio);
            obj(vec![
                ("weight", js(w.0)),
                ("ic_median", jf(median(&ic))),
                ("rho_median", jf(median(&rho))),
                ("ic_over_rho_min", jf(lo)),
                ("ic_over_rho_max", jf(hi)),
                ("rho_below_ic_on_all_10_rows", jb(lo > 1.0)),
            ])
        })
        .collect();
    let order_holds = band_med.iter().all(|b| b.get("rho_below_ic_on_all_10_rows").b());

    // The registered control's expected number of decomposable targets.
    let mut expected_decomposable = 0f64;
    for p in PS {
        for s in SEEDS {
            let (j, _) = ctx.read(&format!("{E28}/stage_a_p{p}s{s}.json"));
            let r = first(&j);
            expected_decomposable += 3.0 * r.get("targets_decomposed").f() / r.get("residuals").f();
        }
    }
    // Model variant priced at the mean call (sensitivity, not a registered quantity).
    let mut by_size_mean = Vec::new();
    for p in PS {
        let v: Vec<f64> = rows
            .iter()
            .filter(|x| x.p == p)
            .map(|x| x.e19_ratio / (x.s_model(x.rp, x.lap, x.c_mean, 0.0) / x.rho_g_i))
            .collect();
        by_size_mean.push((p, (v[0] * v[1]).sqrt()));
    }
    let by_size_rounded: Vec<(u64, f64)> = PS
        .iter()
        .map(|&p| {
            let v: Vec<f64> = rows.iter().zip(&e19_over).filter(|(x, _)| x.p == p).map(|(_, &v)| (v * 100.0).round() / 100.0).collect();
            (p, (v[0] * v[1]).sqrt())
        })
        .collect();
    let arm_med = |a: &str| median(&e31.iter().filter(|x| x.2 == a).map(|x| x.3).collect::<Vec<_>>());
    let (pc, hpc) = ctx.text(&format!("{}/replay/params.txt", ctx.ev));
    let _ = pc;
    raw.push(obj(vec![("path", js("replay/params.txt")), ("sha256", js(&hpc))]));
    raw.push(obj(vec![("path", js("replay/curve_check.json")), ("sha256", js(&hcc))]));
    let f101 = by_size.iter().find(|x| x.0 == 101).unwrap().1;
    let fm101 = by_size_mean.iter().find(|x| x.0 == 101).unwrap().1;
    let p149 = e31_cmp_factor(&rows, &d0, &e31, 149);
    let errata = J::Arr(vec![
        obj(vec![("id", js("E1")), ("claim", js("§9.30 A2: \"by size (geometric mean of the two seeds) 1.77, 4.29, 2.39, 1.83, 3.65\"")), ("recomputed", js(&format!("geometric mean of the two seeds' E19/E28: {}; the printed figures are the geometric means of the 2-decimal per-curve values ({})", by_size.iter().map(|x| format!("{:.4}", x.1)).collect::<Vec<_>>().join(", "), by_size_rounded.iter().map(|x| format!("{:.4}", x.1)).collect::<Vec<_>>().join(", ")))), ("effect", js("none on any verdict: p = 53 reads 1.78 and p = 149 reads 2.40; both stay inside [1.4, 4.0], and p = 101 stays above 4.0 (4.287)"))]),
        obj(vec![("id", js("E2")), ("claim", js("§9.30 G2.103: \">= 52x at 197\"")), ("recomputed", js(&format!("{:.4}", e31_cmp_factor(&rows, &d0, &e31, 197)))), ("effect", js("none: the >= 10x clause holds at p = 197 either way; the bound should read >= 51.6x"))]),
        obj(vec![("id", js("E3")), ("claim", js("§9.30: \"Distance remaining (model): >= 7·10⁵x the matched rho in E25's arithmetic\"")), ("recomputed", js(&format!("least modelled S/rho against walk (i) = {:.4e}", a_min(&d0)))), ("effect", js("rounding only: the least row is 6.99·10⁵, so >= 7·10⁵ is not strictly true; >= 1.0·10⁶ against walk (iii) holds (1.018·10⁶)"))]),
        obj(vec![("id", js("E4")), ("claim", js("§9.30 G2.103: \"holds ... by a factor ≈ 100 between medians\"")), ("recomputed", js(&format!("median E28 tau-psi S_model/rho (walk i) {:.4e}; against the median of all 20 E31 solver-A rows {:.1}x, of the 16 at matched sizes {:.1}x, of arm A {:.1}x, of arm B {:.1}x", med28, med28 / med31, med28 / med31_matched, med28 / arm_med("A"), med28 / arm_med("B")))), ("effect", js("none on a verdict; \"≈ 100\" holds as an order of magnitude (60-112 by pooling); the note does not say which rows the median is over"))]),
        obj(vec![("id", js("E5")), ("claim", js("§9.26/§9.30 Stage A2: S_model prices every residual at the control's median call C")), ("recomputed", js(&format!("E19's measured S is priced at the mean call (reruns and high-degree tails); the same formula with the median call under-prices E19's own measured rows by {flo:.3}-{fhi:.3}x (with the mean call {fmlo:.3}-{fmhi:.3}x). With E28's 3-call mean instead of the median, E19/E28 by size becomes {} (p = 101: {fm101:.3} against {f101:.3})", by_size_mean.iter().map(|x| format!("{:.3}", x.1)).collect::<Vec<_>>().join(", ")))), ("effect", js("a bias of the registered model in E28's favour (its S_model is optimistic), not an arithmetic error: G2.103's per-curve and p = 101 falsifications and the >= 10x failure at p = 149 stand under either pricing; the registered quantity (median) is what is verified"))]),
    ]);
    let verdicts = J::Arr(vec![
        obj(vec![("id", js("G2.100")), ("note", js("holds")), ("recomputed", js(&format!("40 curves, every psi/lambda/T/line check true, h = 4 on 40, density pooled {dlo:.3}-{dhi:.3} in [0.3, 0.7], 2/4 abscissae (4/8 points with ±) a column; own arithmetic confirms j in F_p2 \\ F_p on all 40")))]),
        obj(vec![("id", js("G2.101")), ("note", js("holds (pooled, as registered)")), ("recomputed", js(&format!("cols 2.00 on 10/10; decompositions per decomposed target median 8 on 10/10; y ratio pooled {:.4} in [0.8, 1.25]; residuals ratio pooled {:.4} in [1.6, 2.4] (per curve {rlo:.3}-{rhi:.3}); logs on 20/20 arms", (sum_rp / sum_relp) / (sum_rt / sum_relt), sum_rt / sum_rp)))]),
        obj(vec![("id", js("G2.102")), ("note", js("holds, with the disclosed deviation")), ("recomputed", js(&format!("registered: 30/30 calls solved, 0 = 0 decompositions (negative control only); supplementary: 30/30 solved with the oracle's exact set; C / C_E19 in [{w30lo:.3}, {w30hi:.3}] within ±30 %")))]),
        obj(vec![("id", js("G2.103")), ("note", js("falsified")), ("recomputed", js(&format!("per curve E19/E28 {:.3}-{:.3} (outside [1.4, 4.0] on several curves); by size p = 101 {f101:.3} > 4.0; best E28 row / worst E31 A row at p = 149 = {p149:.3} < 10", a_min(&e19_over), e19_over.iter().cloned().fold(0f64, f64::max))))]),
    ]);
    let deviation = obj(vec![
        ("disclosed", jb(true)),
        ("where", js("§9.30 \"One deviation, recorded\" and G2.102's verdict (\"holds, with the deviation above\"); tables.md lists both controls side by side; the supplementary control's code (--stage c) is in 0e4fccce1, committed after the registered run (dfd1e7a39) and before its own run")),
        ("why_the_registered_control_was_vacuous", js(&format!("3 uniform targets per curve against decomposition rates of targets_decomposed / residuals: the expected number of decomposable targets in the 30 registered draws was {expected_decomposable:.3}, so 0 was the likely outcome by design; the registered check was a negative control (30/30 calls returned no decomposition where the oracle had none), not an agreement test"))),
        ("adequacy", js("Adequate for G2.102's agreement clause: 30 calls on targets the exact oracle decomposes (3 per curve, all 10 curves, families of 2 and 8 decompositions), every one solved and returning exactly the oracle's set, plus the registered 30 as negative controls. Selecting targets by the oracle does not bias an exact-set comparison. Limits: 3 positive calls per curve is a small sample, so it shows agreement on these 30 calls, not in general; the model's C comes from the registered (uniform) calls, as registered, and the decomposable-target calls cost 1.37-1.48·10⁹, slightly more")),
        ("verdict", js("disclosed and adequate, with the sample-size caveat")),
    ]);
    let limitations = J::Arr(vec![
        js("S is a model (Stage A2): no end-to-end run with the D4 solve exists (no Stage B, by registration). The validation gate cannot show a modelled S as a measured row; this report's status is fail for that reason alone."),
        js("The model prices every residual at the median of 3 control calls; E19's measured S is priced at the mean call. On E19's own rows the median-based formula under-prices the measured S (E5), so S_model is optimistic for E28."),
        js("The band's native operations for the IC side are E19's Macaulay entries per residual at the same p (borrowed: E28 ran no Macaulay solve on its residuals); the rho side's are the walk's steps. GAE = 97 F_p multiplications."),
        js("The comparison with E19 is across different curves at the same p (E19's curves vs E28's GLS twists), as registered."),
        js("Operation counts only. The replay ran locally in the validator's 4-vCPU container, not on isolab-eks; no wall time is compared or claimed."),
        js("Memory is not reported by the producer or recomputed here (op-count study)."),
    ]);
    let cost_model = J::Arr(vec![
        obj(vec![("check", js("unit declared")), ("holds", jb(true)), ("detail", js("F_p multiplications; LA as mod-n multiplications at 16 F_p multiplications each; generation measured per target; GAE = 97"))]),
        obj(vec![("check", js("optimistic assumptions flagged")), ("holds", jb(false)), ("detail", js("the median-call pricing is not flagged as optimistic in §9.30 (E5)"))]),
        obj(vec![("check", js("total = per-attempt cost x attempts")), ("holds", jb(true)), ("detail", js("attempts are the measured residuals to the log R (not an inverse probability), so no success probability is assumed; the per-attempt cost is the median, see E5"))]),
        obj(vec![("check", js("memory reported alongside time")), ("holds", jb(false)), ("detail", js("not reported; op counts only"))]),
    ]);
    let status_ok_numbers = ck.bad.is_empty() && hl.bad.is_empty();
    obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js("fail")),
        ("independent", jb(true)),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E28; this recompute is its own standard-library Rust program (validate_e28.rs: JSON parser, SHA-256 and leaf comparator taken from the E27 validator's validate_e27.rs, the rest new) and shares no code with examples/e28_tau.rs, examples/e28_tables.rs, crypto_lib or experiments/33_tau_stage_a/summary.json")),
        ("date", js("2026-10-31")),
        ("row", js("E28 (GLV note §9.26 pre-registration, §9.30 results): GLS psi fold on E19's single-tau_T line. Stage 0 (40 curves), Stage A (10 curves, tau and tau-psi arms on one exact-oracle stream, 640 walks, the registered solve-cost control) and the supplementary control are measurements; Stage A2 (S_model) is a model. Evidence directory research/e28_tau_20261030")),
        ("script", js("research/e28_tau_20261030/validate_e28.rs (rustc -O --edition 2021; `validate_e28 report <repo> research/e28_tau_20261030`)")),
        ("status_rule", js("pass iff (a) every check holds, (b) every headline agrees with its printed figure within half a unit of the last printed digit (exact for counts), and (c) the row has a measured end-to-end index-calculus cost S, which the validation gate needs to show it against a matched rho. E28 meets (a) and (b) and fails (c): its S is Stage A2's model (R x C + R x g + 16 LA), no end-to-end run with the D4 solve was made (no Stage B, by registration). Status is therefore fail for the gate; the measured parts are verified (replay-certificate.json)")),
        ("gate_refusal", js("model-only S: no measured end-to-end S exists for E28 (Stage A2 is a model by registration, §9.26; §9.30 labels it so). Validation of the measured parts and of the model's arithmetic does not make the model a measurement.")),
        ("numbers_verified", jb(status_ok_numbers)),
        ("raw_data", J::Arr(raw)),
        ("check_failures", J::Arr(ck.bad.iter().map(|s| js(s)).collect())),
        ("headline_disagreements", J::Arr(hl.bad.iter().map(|s| js(s)).collect())),
        ("checks", J::Arr(ck.items)),
        ("headline", J::Arr(hl.items)),
        ("number_theory_stage0", J::Arr(nt0)),
        ("number_theory_stage_a", J::Arr(nt_a)),
        ("stage_a", J::Arr(stage_a_out)),
        ("stage_a2_model", J::Arr(a2_out)),
        ("e19_by_size_over_e28", J::Arr(by_size.iter().map(|(p, v)| obj(vec![("p", ju(*p)), ("geometric_mean", jf(*v))])).collect())),
        ("e31_comparison", J::Arr(e31_cmp)),
        ("medians", obj(vec![("e28_tau_psi_walk_i_0gae", jf(med28)), ("e31_a_all_20_rows", jf(med31)), ("e31_a_matched_sizes_16_rows", jf(med31_matched)), ("ratio_all", jf(med28 / med31)), ("ratio_matched_sizes", jf(med28 / med31_matched))])),
        ("model_fidelity_e19", J::Arr(fid.iter().map(|x| obj(vec![("p", ju(x.0)), ("seed", ju(x.1)), ("measured_over_model_median_c", jf(x.2)), ("measured_over_model_mean_c", jf(x.3))])).collect())),
        ("controls_g2_102_within30", J::Arr(within30.iter().map(|x| obj(vec![("p", ju(x.0)), ("seed", ju(x.1)), ("uniform_over_e19", jf(x.2)), ("decomposed_over_e19", jf(x.3))])).collect())),
        ("gae_band_walk_iii", obj(vec![("definition", js("IC: S_model(w) = R C + R g + 16 LA + w * 97 * R * (E19's Macaulay entries per residual at the same p), the producer's band, one native operation per Macaulay entry; rho: E27's <±psi> walk (iii) (Karatsuba products, batched inversions, the cheapest arithmetic built; the walk is the same walk as (i), checked in A.6) + w * 97 * mean steps. GAE = 97 F_p multiplications (E25's affine addition), as E27's validation. Medians over the 10 curves.")), ("weights", J::Arr(band_med)), ("order_holds_across_band", jb(order_holds))])),
        ("measured_vs_modelled", obj(vec![
            ("measured", js("Stage 0 (40 curves: psi, lambda, T and line checks, h, density, columns); Stage A (10 curves: columns, residuals to the logarithm R, relations, y, decompositions per target, generation cost g, LA, both logarithms, 640 walks in two arithmetics); the registered solve-cost control (30 weighted-D4 calls on uniform targets: C, agreement 0 = 0) and the supplementary control (30 calls on decomposable targets: C, exact agreement). All replayed.")),
            ("modelled", js("Stage A2: S_model = R C + R g + 16 LA for each arm, S_model / rho at 0, 0.1 and 1 GAE (IC native operations borrowed from E19's Macaulay entries per residual), E19/E28, and every comparison with E31 A built on them (G2.103). Recomputed from the measured inputs above, not replayed: no end-to-end run with the D4 solve exists.")),
        ])),
        ("errata", errata),
        ("verdicts_recomputed", verdicts),
        ("registered_control_deviation", deviation),
        ("cost_model_checks", cost_model),
        ("pre_registration", js("§9.26 committed 4845b32c7 (2026-10-30T18:44:54Z) before E28's code 789dc2ff6 (18:58:13Z) and records dfd1e7a39 (21:38:33Z); its text at HEAD f23c53871 equals the registered text (git diff of the section: one trailing blank line)")),
        ("limitations", limitations),
        ("comparator_controls", comparator_controls(ctx)),
    ])
}

fn e31_cmp_factor(rows: &[Row], d0: &[f64], e31: &[(u64, u64, String, f64)], p: u64) -> f64 {
    let best = rows.iter().zip(d0).filter(|(x, _)| x.p == p).map(|(_, &v)| v).fold(f64::INFINITY, f64::min);
    let worst = e31.iter().filter(|x| x.0 == p).map(|x| x.3).fold(0f64, f64::max);
    best / worst
}

fn a_min(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}

// --------------------------------------------------------- certificate ----

fn certificate(ctx: &Ctx) -> J {
    let (cmp, hcmp) = ctx.read(&format!("{}/replay/replay_comparison.json", ctx.ev));
    let (build, hbuild) = ctx.read(&format!("{}/replay/build.json", ctx.ev));
    let mut runs = Vec::new();
    let (mut walks_total, mut walks_ok, mut arms_ok, mut arms_total) = (0u64, 0u64, 0u64, 0u64);
    let mut s0_checks = true;
    let mut supp_ok = true;
    for r in cmp.get("runs").arr() {
        let name = r.get("run").s().to_string();
        let rel = r.get("replay").get("path").s().to_string();
        let (rep, _) = ctx.read(&rel);
        let mut entry = vec![
            ("run", js(&name)),
            ("committed", r.get("committed").clone()),
            ("replay_output", r.get("replay").clone()),
            ("fields_compared", r.get("fields_compared").clone()),
            ("fields_identical", r.get("fields_identical").clone()),
            ("mismatches", r.get("mismatches").clone()),
            ("matched", r.get("identical").clone()),
        ];
        if name.starts_with("stage_a_") {
            let x = first(&rep);
            let d = x.get("tau").get("recovered").u();
            let mut wc = 0u64;
            let ws: Vec<Walk> = walks(x.get("rho_schoolbook")).into_iter().chain(walks(x.get("rho_e27"))).collect();
            for w in &ws {
                walks_total += 1;
                if w.correct && w.rec == Some(d) {
                    walks_ok += 1;
                    wc += 1;
                }
            }
            for arm in ["tau", "tau_psi"] {
                arms_total += 1;
                let a = x.get(arm);
                if a.get("solved").b() && a.get("correct").b() && a.get("recovered").u() == d {
                    arms_ok += 1;
                }
            }
            entry.push(("scalars", obj(vec![
                ("log", ju(d)),
                ("tau_solved_correct", jb(x.get("tau").get("solved").b() && x.get("tau").get("correct").b())),
                ("tau_psi_solved_correct", jb(x.get("tau_psi").get("solved").b() && x.get("tau_psi").get("correct").b())),
                ("walks_correct_and_equal_to_log", ju(wc)),
                ("walks", ju(ws.len() as u64)),
            ])));
            entry.push(("timing_not_compared", r.get("timing_not_compared").clone()));
        } else if name == "stage0" {
            for x in rep.arr() {
                for f in ["j_in_fp2", "psi_squared_is_minus_one", "psi_g_is_lambda_g", "lambda_squared_is_minus_one", "psi_fixes_t", "line_checks"] {
                    s0_checks &= x.get(f).b();
                }
            }
            entry.push(("scalars", obj(vec![("curves", ju(rep.arr().len() as u64)), ("all_psi_and_line_checks_true", jb(s0_checks))])));
        } else {
            let mut n = 0u64;
            for x in rep.arr() {
                for c in x.get("calls").arr() {
                    n += 1;
                    supp_ok &= c.get("solved").b() && c.get("agrees").b();
                }
            }
            entry.push(("scalars", obj(vec![("calls", ju(n)), ("all_solved_and_equal_to_oracle", jb(supp_ok && n == 30))])));
            entry.push(("timing_not_compared", r.get("timing_not_compared").clone()));
        }
        runs.push(obj(entry));
    }
    let all_identical = cmp.get("all_identical").b();
    let scalars = walks_ok == walks_total && walks_total == 640 && arms_ok == arms_total && arms_total == 20 && s0_checks && supp_ok;
    obj(vec![
        ("schema", js("replay-certificate/v1")),
        ("status", js(if all_identical && scalars { "verified" } else { "failed" })),
        ("independent", jb(true)),
        ("scalars_verified", jb(scalars)),
        ("date", js("2026-10-31")),
        ("row", js("E28 (GLV note §9.26/§9.30), the measured parts: Stage 0 (40 curves), Stage A (10 curves x {tau, tau-psi} arms on one exact-oracle stream, 3 registered solve-cost control calls and 64 walks per curve) and the supplementary control (30 calls on decomposable targets); evidence directory research/e28_tau_20261030. Stage A2 is a model and is not a run: it is recomputed in validator-report.json, not replayed")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E28: fresh builds from exports of the recorded binary commits in a fresh target directory, fresh runs, own comparison (validate_e28.rs compare)")),
        ("recorded_binary", js("789dc2ff62a8a6d9497a93542e3e59e07cf2c862 for Stage 0, Stage A and the registered control (experiments/33_tau_stage_a/binary_commit.txt); 0e4fccce1ad1baf7ace18a550483771f02315838 for the supplementary control (--stage c exists only from that commit, §9.30)")),
        ("local_replay_note", js("Replayed locally in the validator's 4-vCPU container, not on isolab-eks: the replay admits operation counts only; no wall time is compared or claimed (wall_ms and ms are listed, not compared).")),
        ("build", build),
        ("build_record", obj(vec![("path", js("replay/build.json")), ("sha256", js(&hbuild))])),
        ("comparison", obj(vec![("path", js("replay/replay_comparison.json")), ("sha256", js(&hcmp)), ("rule", cmp.get("rule").clone())])),
        ("runs_total", cmp.get("runs_total").clone()),
        ("runs_matched", cmp.get("runs_identical").clone()),
        ("fields_compared", cmp.get("fields_compared").clone()),
        ("fields_identical", cmp.get("fields_identical").clone()),
        ("coverage", js("full: every recorded run replayed (Stage 0's one run over 40 curves, all ten Stage A runs, the supplementary control's one run over 10 curves); no sampling")),
        ("scalar_verification", js("Every replayed walk (640) reports correct = true, which the binary sets only when the collision-recovered logarithm equals the planted d and [d]G = Q is checked on the curve, and recovers the same d as both index-calculus arms of its curve; every arm (20) reports solved and correct (Wiedemann logarithm checked against the planted d in the binary). Stage 0's psi, lambda, T and line checks are true on all 40 replayed curves. Every one of the 30 supplementary control calls is solved and returns exactly the oracle's decomposition set.")),
        ("runs", J::Arr(runs)),
    ])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: validate_e28 compare|report|certificate <repo> <evidence-dir>");
        std::process::exit(2);
    }
    let ctx = Ctx { repo: args[2].clone(), ev: args[3].clone() };
    let out = match args[1].as_str() {
        "compare" => compare(&ctx),
        "report" => report(&ctx),
        "certificate" => certificate(&ctx),
        _ => {
            eprintln!("unknown command");
            std::process::exit(2);
        }
    };
    print!("{}", out.dump());
}
