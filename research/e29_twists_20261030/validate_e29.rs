//! E29 validation (independent validator, 2026-10-31).
//!
//! An independent recompute of the headline numbers of
//! `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.31 (E29; pre-registration §9.27) from the raw records in the repository,
//! and a leaf-by-leaf comparison of a fresh replay with the committed records.
//! Standard library only.  The JSON parser, SHA-256 and the leaf comparator are
//! copied from the E27 validator's `research/e27_walk_20261030/validate_e27.rs`
//! (validator code); everything else is new.  It shares no code with
//! `examples/e29_twist.rs`, `examples/e29_tables.rs` or `crypto_lib`.
//!
//! ```text
//! rustc -O --edition 2021 validate_e29.rs -o validate_e29
//! validate_e29 compare     <repo> <evidence-dir>   # -> replay/replay_comparison.json
//! validate_e29 report      <repo> <evidence-dir>   # -> validator-report.json
//! validate_e29 certificate <repo> <evidence-dir>   # -> replay-certificate.json
//! ```

#[allow(unused_imports)]
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

#[allow(dead_code)]
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

const E29: &str = "experiments/34_twist24_stage_r";
const E18: &str = "experiments/26_quartic_folds_psi.json";
const GAE: f64 = 97.0;
const C4: f64 = 1.213e12;
const WEIGHTS: [(&str, f64); 3] = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)];
const KILL_PS: [u64; 8] = [241, 337, 541, 601, 733, 769, 997, 1069];
const A_PS: [u64; 3] = [241, 337, 541];

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
        }
    }
    fn walk_muls(&self) -> f64 {
        (self.muls - self.vmuls) as f64
    }
}

fn walks(j: &J) -> Vec<Walk> {
    j.arr().iter().map(Walk::of).collect()
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// A figure as printed: `1.74·10^7`, `0.45`, `5519`, `3.0300e6`.
fn printed(s: &str) -> (f64, f64) {
    let t = s.trim().trim_matches('`').replace(',', "");
    let (mant, exp) = if let Some((m, e)) = t.split_once('e') {
        (m.to_string(), e.parse::<i32>().unwrap())
    } else if let Some((m, e)) = t.split_once("·10") {
        (m.to_string(), e.trim_start_matches('^').parse::<i32>().unwrap())
    } else if let Some(e) = t.strip_prefix("2^") {
        return (e.parse::<f64>().unwrap(), 0.05);
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
    ["kill", "stage_r_p241", "stage_r_rest", "stage_a_p241", "stage_a_p337", "stage_a_p541"]
        .iter()
        .map(|n| (n.to_string(), format!("{E29}/{n}.json"), format!("{}/replay/{n}.json", ctx.ev)))
        .collect()
}

fn compare(ctx: &Ctx) -> J {
    let mut runs = Vec::new();
    let (mut all_c, mut all_i, mut all_ok) = (0u64, 0u64, true);
    for (name, committed, replay) in replay_files(ctx) {
        let (a, ha) = ctx.read(&committed);
        let (b, hb) = ctx.read(&replay);
        let mut d = LeafDiff::default();
        leaves("$", &a, &b, &mut d);
        let ok = d.mismatches.is_empty() && d.only_a.is_empty() && d.only_b.is_empty();
        let trip = |v: &Vec<(String, J, J)>| {
            J::Arr(v.iter().map(|(p, x, y)| obj(vec![("path", js(p)), ("committed", x.clone()), ("replay", y.clone())])).collect())
        };
        all_ok &= ok;
        all_c += d.compared;
        all_i += d.identical;
        runs.push(obj(vec![
            ("run", js(&name)),
            ("committed", obj(vec![("path", js(&committed)), ("sha256", js(&ha))])),
            ("replay", obj(vec![("path", js(&replay)), ("sha256", js(&hb))])),
            ("fields_compared", ju(d.compared)),
            ("fields_identical", ju(d.identical)),
            ("mismatches", trip(&d.mismatches)),
            ("only_in_committed", J::Arr(d.only_a.iter().map(|p| js(p)).collect())),
            ("only_in_replay", J::Arr(d.only_b.iter().map(|p| js(p)).collect())),
            ("timing_not_compared", trip(&d.timing)),
            ("identical", jb(ok)),
        ]));
    }
    obj(vec![
        ("schema", js("e29-replay-comparison/v1")),
        ("tool", js(&format!("{}/validate_e29.rs compare", ctx.ev))),
        ("rule", js("every leaf of every committed record against the replay: integers and floats by their text (equal text, or equal value with the same integer/float form; integers compared exactly as i128), strings and booleans exactly; wall_ms is wall-clock and listed, not compared")),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_identical", ju(runs.iter().filter(|r| r.get("identical").b()).count() as u64)),
        ("fields_compared", ju(all_c)),
        ("fields_identical", ju(all_i)),
        ("all_identical", jb(all_ok)),
        ("runs", J::Arr(runs)),
    ])
}

fn comparator_controls(ctx: &Ctx) -> J {
    let rel = format!("{E29}/stage_r_p241.json");
    let (a, _) = ctx.read(&rel);
    let (t, _) = ctx.text(&rel);
    let mut d = LeafDiff::default();
    leaves("$", &a, &a, &mut d);
    let mut all = d.mismatches.is_empty() && d.compared > 0;
    let mut out = vec![obj(vec![("control", js("record against itself")), ("fields_compared", ju(d.compared)), ("mismatches", ju(d.mismatches.len() as u64)), ("holds", jb(all))])];
    for (name, from, to) in [
        ("steps 15654 -> 15655", "\"steps\": 15654", "\"steps\": 15655"),
        ("visit_digest + 1 (beyond f64 precision)", "12106413775656540146", "12106413775656540147"),
        ("correct true -> false", "\"correct\": true", "\"correct\": false"),
    ] {
        assert!(t.contains(from), "anchor {from}");
        let b = parse(&t.replacen(from, to, 1));
        let mut d = LeafDiff::default();
        leaves("$", &a, &b, &mut d);
        let caught = d.mismatches.len() == 1;
        all &= caught;
        out.push(obj(vec![("control", js(name)), ("mismatches", ju(d.mismatches.len() as u64)), ("expected", js("exactly 1")), ("holds", jb(caught))]));
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
        self.items.push(obj(vec![("id", js(id)), ("check", js(what)), ("holds", jb(holds)), ("detail", js(&detail))]));
    }
}

struct RRow {
    p: u64,
    kara: bool,
    n: f64,
    m6: f64,
    s6: f64,
    m24: f64,
    s24: f64,
}

fn report(ctx: &Ctx) -> J {
    let mut raw: Vec<J> = Vec::new();
    let pin = |rel: &str, h: &str, raw: &mut Vec<J>| raw.push(obj(vec![("path", js(&format!("../../{rel}"))), ("sha256", js(h))]));
    let mut ck = Checks { items: Vec::new(), bad: Vec::new() };
    let mut hl = Headlines { items: Vec::new(), bad: Vec::new() };
    let (tables, ht) = ctx.text(&format!("{E29}/tables.md"));
    pin(&format!("{E29}/tables.md"), &ht, &mut raw);

    // ---------------- kill test ----------------
    let (kill, hk) = ctx.read(&format!("{E29}/kill.json"));
    pin(&format!("{E29}/kill.json"), &hk, &mut raw);
    let (e18, h18) = ctx.read(E18);
    pin(E18, &h18, &mut raw);
    let b0_claim = [3u64, 3, 3, 9, 9, 2, 17, 3];
    let mut kill_ok = true;
    let mut kill_detail = Vec::new();
    for (i, k) in kill.arr().iter().enumerate() {
        let (p, n) = (k.get("p").u(), k.get("n").u());
        let e18n = e18.arr().iter().find(|r| r.get("p").u() == p).map(|r| r.get("n").u());
        let ok = p == KILL_PS[i]
            && k.get("psi_p_on_curve").b()
            && k.get("lambda_p_found").b()
            && k.get("order_lambda_p").u() == 24
            && k.get("order_chi6_b").u() == 6
            && k.get("order_lambda_omega").u() == 3
            && k.get("group_order").u() == 24
            && !k.get("u2_in_fp").b()
            && k.get("canonical_b0").u() == b0_claim[i]
            && k.get("n_mod_24").u() == n % 24
            && k.get("n_mod_96").u() == n % 96
            && n % 24 == 1
            && (n % 96 == 1 || n % 96 == 49)
            && e18n == Some(n);
        kill_ok &= ok;
        kill_detail.push(format!("p={p}: {ok}"));
    }
    ck.add("K.1", "Kill test (the binary's checks on E18's own models, replayed): on all 8 curves psi_p maps E to E, lambda_p found of order 24, ord chi6(b) = 6 (so ord lambda_p = 4 ord chi6(b)), ord lambda_omega = 3, |<lambda_p, lambda_omega, -1>| = 24, u^2 not in F_p, canonical b0 = 3,3,3,9,9,2,17,3; n mod 24 and mod 96 recomputed from n (1, and 1 or 49); n equals E18's recorded n", kill_ok && kill.arr().len() == 8, kill_detail.join("; "));
    let (tc, htc) = ctx.read(&format!("{}/replay/twist_check.json", ctx.ev));
    raw.push(obj(vec![("path", js("replay/twist_check.json")), ("sha256", js(&htc))]));
    let (_, hti) = ctx.text(&format!("{}/replay/twist_input.txt", ctx.ev));
    raw.push(obj(vec![("path", js("replay/twist_input.txt")), ("sha256", js(&hti))]));
    let tc_ok = tc.arr().len() == 11
        && tc.arr().iter().all(|r| r.get("ok").b() && r.get("group_order").u() == 24)
        && kill.arr().iter().all(|k| {
            tc.arr().iter().any(|r| r.get("p").u() == k.get("p").u() && r.get("n").u() == k.get("n").u() && r.get("b0").u() == k.get("canonical_b0").u())
        });
    ck.add("K.2", "Validator's own F_p4 arithmetic on the 8 canonical models y^2 = x^3 + b0 t^3 (replay/twist_check.rs): a point P (own Tonelli-Shanks, and Stage A's generator for p = 241, 337, 541) with [n]P = O, n prime in the Hasse interval (#E = n, as E18's), psi(x, y) = (t^(1-p) x^p, t^(3(1-p)/2) y^p) on E with psi(P) = [lambda]P, lambda of order 8 (psi^4 = -1 for this normalisation, N(u) = -1), lambda_omega of order 3, lambda * lambda_omega of order 24 and |<lambda, lambda_omega, -1>| = 24 on every curve: the kill criterion's group, independently. E18's own models' coefficients are not in the records, so ord lambda_p = 24 for E18's normalisation (u = b^(-(p-1)/6)) rests on the replayed binary check (K.1); it differs from this psi by an automorphism", tc_ok, format!("{} of 11 lines ok", tc.arr().iter().filter(|r| r.get("ok").b()).count()));
    for (i, row) in md_table(&tables, "## Kill test").iter().enumerate() {
        let (v, _) = printed(&row[9]);
        hl.add(&format!("canonical_b0_p{}", KILL_PS[i]), "experiments/34_twist24_stage_r/tables.md, kill test", &row[9], v, 0.0, kill.arr()[i].get("canonical_b0").f(), "least b0 with #(y^2 = x^3 + b0 t^3) = n (binary), and own check K.2");
    }
    hl.add("kill_group_order_24_curves", "note §9.31 (\"on all eight curves ... |<lambda_p, lambda_omega, -1>| = 24\")", "8", 8.0, 0.0, tc.arr().iter().filter(|r| !r.get("point_from_stage_a").b() && r.get("group_order").u() == 24).count() as f64, "curves with group order 24 in the validator's own check");

    // ---------------- Stage R ----------------
    let mut rrows: Vec<RRow> = Vec::new();
    let (mut wtot, mut wok, mut ident) = (0u64, 0u64, true);
    for f in ["stage_r_p241", "stage_r_rest"] {
        let rel = format!("{E29}/{f}.json");
        let (j, h) = ctx.read(&rel);
        pin(&rel, &h, &mut raw);
        for row in j.arr() {
            let runs = row.get("runs").arr();
            let ws: Vec<(Vec<Walk>, Vec<Walk>)> = runs.iter().map(|r| (walks(r.get("order6")), walks(r.get("order24")))).collect();
            // the two arithmetics walk the same walks
            for o in 0..2 {
                let (a, b) = if o == 0 { (&ws[0].0, &ws[1].0) } else { (&ws[0].1, &ws[1].1) };
                ident &= a.len() == 16 && b.len() == 16;
                for (x, y) in a.iter().zip(b.iter()) {
                    ident &= x.seed == y.seed && x.steps == y.steps && x.gops == y.gops && x.rec == y.rec && x.esc == y.esc && x.fold == y.fold;
                }
            }
            for (r, (w6, w24)) in runs.iter().zip(&ws) {
                for w in w6.iter().chain(w24.iter()) {
                    wtot += 1;
                    if w.correct && w.rec.is_some() {
                        wok += 1;
                    }
                }
                rrows.push(RRow {
                    p: r.get("p").u(),
                    kara: r.get("karatsuba").b(),
                    n: r.get("n").f(),
                    m6: mean(&w6.iter().map(|w| w.walk_muls()).collect::<Vec<_>>()),
                    s6: mean(&w6.iter().map(|w| w.steps as f64).collect::<Vec<_>>()),
                    m24: mean(&w24.iter().map(|w| w.walk_muls()).collect::<Vec<_>>()),
                    s24: mean(&w24.iter().map(|w| w.steps as f64).collect::<Vec<_>>()),
                });
            }
        }
    }
    ck.add("R.1", "Stage R: 8 curves x 2 arithmetics x {order 6, order 24} x 16 walks; every walk record correct (512 records = 256 walks, each priced in both arithmetics)", wok == wtot && wtot == 512 && rrows.len() == 16, format!("{wok} of {wtot} records"));
    ck.add("R.2", "Stage R: schoolbook and Karatsuba records are the same walks (seed, steps, group ops, recovered, cycle escapes)", ident, format!("{ident}"));
    let md_r = md_table(&tables, "## Stage R");
    let mut r_out = Vec::new();
    for (row, md) in rrows.iter().zip(&md_r) {
        assert_eq!(md[0].parse::<u64>().unwrap(), row.p);
        let tag = format!("p{}_{}", row.p, if row.kara { "karatsuba" } else { "schoolbook" });
        let src = "experiments/34_twist24_stage_r/tables.md, Stage R";
        let sq = row.n.sqrt();
        // mean steps are printed rounded to an integer ({:.0}): half a unit is 0.5
        for (name, cell, val, def) in [("steps_order6", &md[4], row.s6, "mean steps, 16 order-6 walks (printed rounded)"), ("steps_order24", &md[5], row.s24, "mean steps, 16 order-24 walks (printed rounded)")] {
            let (v, _) = printed(cell);
            hl.add(&format!("{name}_{tag}"), src, cell, v, 0.5, val, def);
        }
        let mut add = |name: &str, cell: &str, val: f64, def: &str| {
            let (v, h) = printed(cell);
            hl.add(&format!("{name}_{tag}"), src, cell, v, h, val, def);
        };
        add("steps_ratio", &md[6], row.s6 / row.s24, "order 6 / order 24");
        add("muls_order6", &md[7], row.m6, "mean fp_muls - verify_fp_muls, order 6");
        add("muls_order24", &md[8], row.m24, "mean fp_muls - verify_fp_muls, order 24");
        add("muls_ratio", &md[9], row.m6 / row.m24, "order 6 / order 24");
        let ps: Vec<&str> = md[10].split(" / ").collect();
        add("per_step_order6", ps[0], row.m6 / row.s6, "F_p mults per step, order 6");
        add("per_step_order24", ps[1], row.m24 / row.s24, "F_p mults per step, order 24");
        let sr: Vec<&str> = md[11].split(" / ").collect();
        add("srho_order6", sr[0], row.m6 / sq, "F_p mults per sqrt(n), order 6");
        add("srho_order24", sr[1], row.m24 / sq, "F_p mults per sqrt(n), order 24");
        let band: Vec<J> = WEIGHTS
            .iter()
            .map(|w| {
                let c6 = row.m6 + w.1 * GAE * row.s6;
                let c24 = row.m24 + w.1 * GAE * row.s24;
                obj(vec![("weight", js(w.0)), ("order6", jf(c6)), ("order24", jf(c24)), ("order6_over_order24", jf(c6 / c24))])
            })
            .collect();
        r_out.push(obj(vec![("p", ju(row.p)), ("karatsuba", jb(row.kara)), ("steps_ratio", jf(row.s6 / row.s24)), ("muls_ratio", jf(row.m6 / row.m24)), ("per_step_6", jf(row.m6 / row.s6)), ("per_step_24", jf(row.m24 / row.s24)), ("band", J::Arr(band))]));
    }
    // pooled and ranges
    let sch: Vec<&RRow> = rrows.iter().filter(|r| !r.kara).collect();
    let kar: Vec<&RRow> = rrows.iter().filter(|r| r.kara).collect();
    let pooled = |v: &Vec<&RRow>, f6: &dyn Fn(&RRow) -> f64, f24: &dyn Fn(&RRow) -> f64| {
        v.iter().map(|r| f6(r) / r.n.sqrt()).sum::<f64>() / v.iter().map(|r| f24(r) / r.n.sqrt()).sum::<f64>()
    };
    let st_pooled = pooled(&sch, &|r| r.s6, &|r| r.s24);
    let mu_s = pooled(&sch, &|r| r.m6, &|r| r.m24);
    let mu_k = pooled(&kar, &|r| r.m6, &|r| r.m24);
    let srho6 = mean(&sch.iter().map(|r| r.m6 / r.n.sqrt()).collect::<Vec<_>>());
    let srho24 = mean(&sch.iter().map(|r| r.m24 / r.n.sqrt()).collect::<Vec<_>>());
    let (slo, shi) = minmax(&sch.iter().map(|r| r.s6 / r.s24).collect::<Vec<_>>());
    let (mslo, mshi) = minmax(&sch.iter().map(|r| r.m6 / r.m24).collect::<Vec<_>>());
    let (mklo, mkhi) = minmax(&kar.iter().map(|r| r.m6 / r.m24).collect::<Vec<_>>());
    let (p6lo, p6hi) = minmax(&sch.iter().map(|r| r.m6 / r.s6).collect::<Vec<_>>());
    let (p24lo, p24hi) = minmax(&sch.iter().map(|r| r.m24 / r.s24).collect::<Vec<_>>());
    let n = "note §9.31 Stage R";
    hl.add("steps_ratio_min", n, "1.29", 1.29, 0.005, slo, "least order-6/order-24 steps per curve");
    hl.add("steps_ratio_max", n, "2.10", 2.10, 0.005, shi, "largest");
    hl.add("steps_ratio_pooled", n, "1.701", 1.701, 0.0005, st_pooled, "sum over curves of mean steps / sqrt(n), order 6 over order 24");
    hl.add("per_step_order24_min", n, "191", 191.0, 0.5, p24lo, "least F_p mults per step, order 24, schoolbook");
    hl.add("per_step_order24_max", n, "210", 210.0, 0.5, p24hi, "largest");
    hl.add("per_step_order6_min", n, "109", 109.0, 0.5, p6lo, "least F_p mults per step, order 6, schoolbook");
    hl.add("per_step_order6_max", n, "122", 122.0, 0.5, p6hi, "largest");
    hl.add("muls_ratio_schoolbook_min", n, "0.743", 0.743, 0.0005, mslo, "least order-6/order-24 F_p cost per curve");
    hl.add("muls_ratio_schoolbook_max", n, "1.202", 1.202, 0.0005, mshi, "largest");
    hl.add("muls_ratio_schoolbook_pooled", n, "0.962", 0.962, 0.0005, mu_s, "pooled per sqrt(n)");
    hl.add("muls_ratio_karatsuba_min", n, "0.684", 0.684, 0.0005, mklo, "least, Karatsuba");
    hl.add("muls_ratio_karatsuba_max", n, "1.107", 1.107, 0.0005, mkhi, "largest, Karatsuba");
    hl.add("muls_ratio_karatsuba_pooled", n, "0.884", 0.884, 0.0005, mu_k, "pooled per sqrt(n), Karatsuba");
    hl.add("srho_per_sqrt_n_order6", n, "52.9", 52.9, 0.05, srho6, "mean over the 8 curves of F_p mults / sqrt(n), order 6, schoolbook");
    hl.add("srho_per_sqrt_n_order24", n, "55.0", 55.0, 0.05, srho24, "same, order 24");
    // E18's psi walk
    let e18_srho = mean(
        &e18.arr()
            .iter()
            .map(|r| {
                let w: Vec<Walk> = walks(r.get("rho")).into_iter().filter(|w| w.fold == "Psi").collect();
                mean(&w.iter().map(|w| w.walk_muls()).collect::<Vec<_>>()) / r.get("n").f().sqrt()
            })
            .collect::<Vec<_>>(),
    );
    hl.add("e18_psi_walk_srho", "note §9.31 (\"E18's psi walk measured 54.1\")", "54.1", 54.1, 0.05, e18_srho, "mean over E18's 8 curves of its Psi walks' mean fp_muls / sqrt(n) (E18 records carry no verify_fp_muls)");
    hl.add("walks_correct", "note §9.31 (\"all 256 walks correct\")", "256", 256.0, 0.0, (wok / 2) as f64, "walks correct (each walk recorded in two arithmetics)");
    // the band between the two walks (§5a)
    let mut rank = Vec::new();
    for (name, v) in [("schoolbook", &sch), ("Karatsuba", &kar)] {
        for w in WEIGHTS {
            let pr = pooled(v, &|r| r.m6 + w.1 * GAE * r.s6, &|r| r.m24 + w.1 * GAE * r.s24);
            let per: Vec<f64> = v.iter().map(|r| (r.m6 + w.1 * GAE * r.s6) / (r.m24 + w.1 * GAE * r.s24)).collect();
            let cheaper24 = per.iter().filter(|&&x| x > 1.0).count();
            rank.push(obj(vec![("arithmetic", js(name)), ("weight", js(w.0)), ("pooled_order6_over_order24", jf(pr)), ("curves_where_order24_is_cheaper", ju(cheaper24 as u64)), ("per_curve", J::Arr(per.iter().map(|&x| jf(x)).collect()))]));
        }
    }
    let flips = rank.iter().any(|r| r.get("pooled_order6_over_order24").f() > 1.0) && rank.iter().any(|r| r.get("pooled_order6_over_order24").f() < 1.0);

    // ---------------- Stage A ----------------
    let md_a = md_table(&tables, "## Stage A");
    let mut a_out = Vec::new();
    let mut a_ok = true;
    let mut cols4 = true;
    let mut la_r = Vec::new();
    let mut r24 = Vec::new();
    let mut nstar = Vec::new();
    let mut nstar_cubic = Vec::new();
    let mut mi = 0;
    for p in A_PS {
        let rel = format!("{E29}/stage_a_p{p}.json");
        let (j, h) = ctx.read(&rel);
        pin(&rel, &h, &mut raw);
        let rep = j.arr()[0].get("report");
        let nn = rep.get("n").f();
        let sq = nn.sqrt();
        let ws = walks(rep.get("rho"));
        let d = rep.get("known_scalar").u();
        a_ok &= ws.len() == 48 && ws.iter().all(|w| w.correct && w.rec == Some(d));
        let m6 = mean(&ws.iter().filter(|w| w.fold == "Psi").map(|w| w.walk_muls()).collect::<Vec<_>>());
        let m24 = mean(&ws.iter().filter(|w| w.fold == "Twist24").map(|w| w.walk_muls()).collect::<Vec<_>>());
        let mut arms = Vec::new();
        for arm in ["flat", "cubic", "twist24"] {
            let a = rep.get(arm);
            a_ok &= a.get("solved").b() && a.get("correct").b() && a.get("recovered").u() == d;
            let la = 16.0 * a.get("la_ops").f();
            let rel_ = a.get("relations").f();
            let res = a.get("residuals_at_solve").f();
            let y = res / rel_;
            let cols = a.get("columns").f();
            let ns = |m: f64| {
                let r = la / m;
                if r >= 1.0 {
                    f64::NAN
                } else {
                    4.0 * (y * (cols / p as f64) * C4 / ((m / sq) * (1.0 - r))).log2()
                }
            };
            let md = &md_a[mi];
            mi += 1;
            assert_eq!(md[3], arm);
            let tag = format!("p{p}_{arm}");
            let src = "experiments/34_twist24_stage_r/tables.md, Stage A";
            let mut add = |name: &str, cell: &str, val: f64, def: &str| {
                if cell.contains("NaN") {
                    return;
                }
                let (v, h) = printed(cell);
                hl.add(&format!("{name}_{tag}"), src, cell, v, h, val, def);
            };
            add("cols", &md[4], cols, "columns");
            add("residuals", &md[6], res, "residuals_at_solve");
            add("y", &md[7], y, "residuals / relations");
            add("la16", &md[8], la, "16 * la_ops");
            add("r_order6", &md[9], la / m6, "16 LA / mean order-6 walk");
            add("r_order24", &md[10], la / m24, "16 LA / mean order-24 walk");
            add("nstar_order6_log2", &md[12], ns(m6), "4 log2(y (cols/p) C4 / ((S_rho/sqrt n)(1 - r))), order-6 walk");
            add("nstar_order24_log2", &md[13], ns(m24), "same, order-24 walk");
            arms.push(obj(vec![("arm", js(arm)), ("cols", jf(cols)), ("y", jf(y)), ("la16", jf(la)), ("r6", jf(la / m6)), ("r24", jf(la / m24)), ("nstar6_log2", if ns(m6).is_nan() { J::Null } else { jf(ns(m6)) }), ("nstar24_log2", if ns(m24).is_nan() { J::Null } else { jf(ns(m24)) })]));
            if arm == "twist24" {
                r24.push((p, la / m24));
                nstar.push((p, ns(m6), ns(m24)));
            }
            if arm == "cubic" {
                nstar_cubic.push((p, ns(m6)));
            }
        }
        let c = rep.get("cubic");
        let t = rep.get("twist24");
        cols4 &= c.get("columns").u() == 4 * t.get("columns").u();
        la_r.push((p, c.get("la_ops").f() / t.get("la_ops").f()));
        a_out.push(obj(vec![("p", ju(p)), ("log", ju(d)), ("walk_order6", jf(m6)), ("walk_order24", jf(m24)), ("arms", J::Arr(arms))]));
    }
    ck.add("A.1", "Stage A: flat, cubic and twist24 arms solved and correct with the known logarithm on all 3 curves; all 48 walks per curve correct", a_ok, format!("{a_ok}"));
    ck.add("A.2", "Stage A: cubic / twist24 columns = 4.00 on every curve", cols4, format!("{cols4}"));
    let (llo, lhi) = minmax(&la_r.iter().map(|x| x.1).collect::<Vec<_>>());
    let (rlo, rhi) = minmax(&r24.iter().map(|x| x.1).collect::<Vec<_>>());
    let all_n: Vec<f64> = nstar.iter().flat_map(|x| [x.1, x.2]).collect();
    let (nlo, nhi) = minmax(&all_n);
    let (clo, chi) = minmax(&nstar_cubic.iter().map(|x| x.1).collect::<Vec<_>>());
    let n = "note §9.31 Stage A";
    hl.add("la_ratio_min", n, "13.3", 13.3, 0.05, llo, "least la_ops cubic / twist24");
    hl.add("la_ratio_max", n, "15.2", 15.2, 0.05, lhi, "largest");
    hl.add("r24_min", n, "0.0058", 0.0058, 0.00005, rlo, "least 16 LA twist24 / order-24 walk");
    hl.add("r24_max", n, "0.0103", 0.0103, 0.00005, rhi, "largest");
    hl.add("nstar_twist24_min_log2", "note §9.31 G2.109 (\"2^135.0-2^142.2\")", "135.0", 135.0, 0.05, nlo, "least log2 n* of the twist24 arm over both walks");
    hl.add("nstar_twist24_max_log2", "note §9.31 G2.109", "142.2", 142.2, 0.05, nhi, "largest");
    hl.add("nstar_cubic_min_log2", "task statement / §9.31 (\"E18's 2^146-2^152\")", "146", 146.0, 0.5, clo, "least log2 n* of the cubic (E18 psi fold) arm, order-6 walk");
    hl.add("nstar_cubic_max_log2", "task statement / §9.31", "152", 152.0, 0.5, chi, "largest");
    hl.add("gap_to_e24_min_bits", "note §9.31 G2.109 (\"2^44.9-2^52.1 above E24's 2^90.1\")", "44.9", 44.9, 0.05, nlo - 90.1, "least twist24 log2 n* minus E24's 90.1 (cited, not recomputed)");
    hl.add("gap_to_e24_max_bits", "note §9.31 G2.109", "52.1", 52.1, 0.05, nhi - 90.1, "largest");

    let verdicts = J::Arr(vec![
        obj(vec![("id", js("G2.104")), ("note", js("holds (not killed)")), ("recomputed", js("replayed binary checks on E18's models (K.1) and the validator's own group computation on the canonical models (K.2): |<lambda_p, lambda_omega, -1>| = 24 on all 8"))]),
        obj(vec![("id", js("G2.105")), ("note", js("holds pooled (at the band's edge), falsified per curve")), ("recomputed", js(&format!("pooled {st_pooled:.4} in [1.7, 2.3]; per curve {slo:.3}-{shi:.3}, {} of 8 below 1.7", sch.iter().filter(|r| r.s6 / r.s24 < 1.7).count())))]),
        obj(vec![("id", js("G2.106")), ("note", js("falsified")), ("recomputed", js(&format!("order-6/order-24 F_p cost {mslo:.3}-{mshi:.3} (schoolbook), {mklo:.3}-{mkhi:.3} (Karatsuba), outside [1.3, 2.0] on every curve")))]),
        obj(vec![("id", js("G2.107")), ("note", js("the producer's reading holds at 0 GAE only")), ("recomputed", js("at 0 GAE the order-6 walk is cheaper pooled (0.962 / 0.884); at 1 GAE (one native operation per step) the order-24 walk is cheaper pooled and on 7 of 8 curves, so under §5a the two walks are unresolved across the band (see walk_band_5a); \"no correction of E18's psi rows at the pooled level\" holds only at 0 GAE"))]),
        obj(vec![("id", js("G2.108")), ("note", js("holds except r_24, falsified low at p = 241 and 541")), ("recomputed", js(&format!("cols 4.00 on 3/3; LA ratio {llo:.2}-{lhi:.2} in [10, 22]; r_24 {}; logs on 9/9 arms", r24.iter().map(|x| format!("p={} {:.5}", x.0, x.1)).collect::<Vec<_>>().join(", "))))]),
        obj(vec![("id", js("G2.109")), ("note", js("holds at p = 241, falsified low at 337 and 541; the >= 2^45 clause misses by 0.1 bit at 337")), ("recomputed", js(&nstar.iter().map(|x| format!("p={}: 2^{:.2} / 2^{:.2}", x.0, x.1, x.2)).collect::<Vec<_>>().join("; ")))]),
    ]);
    let errata = J::Arr(vec![
        obj(vec![("id", js("E1")), ("claim", js("§9.31 G2.107 / What it says: \"the matched reference stays the order-6 walk (cheaper pooled)\"; \"E18's psi-arm distances stand\"")), ("recomputed", js(&format!("true at 0 GAE only. Pricing one native operation per walk step (the repository's convention, E27) at 0, 0.1 and 1 GAE: pooled order-6/order-24 cost {} (schoolbook) and {} (Karatsuba); at 1 GAE the order-24 walk is cheaper on 7 of 8 curves", rank.iter().filter(|r| r.get("arithmetic").s() == "schoolbook").map(|r| format!("{} at {}", { let v = r.get("pooled_order6_over_order24").f(); format!("{v:.3}") }, r.get("weight").s())).collect::<Vec<_>>().join(", "), rank.iter().filter(|r| r.get("arithmetic").s() == "Karatsuba").map(|r| format!("{} at {}", { let v = r.get("pooled_order6_over_order24").f(); format!("{v:.3}") }, r.get("weight").s())).collect::<Vec<_>>().join(", ")))), ("effect", js("AGENTS.md §5a: the order of the two walks changes inside the band, so neither is the stronger reference across it; the order-24 walk also has more unemitted per-step work (12 candidate comparisons against fewer), which the band does not price, so the ranking is unresolved, not reversed. E18's psi-arm distances are lower bounds at 1 GAE"))]),
    ]);
    let limitations = J::Arr(vec![
        js("No measured end-to-end index-calculus S exists for E29: Stage A uses E18's exact meet-in-the-middle oracle (its pair-table lookups unpriced) and the S5 solve was not run (no Stage B, by registration). The handover n* is §11.16's model with §11.17's C4 = 1.213e12 (cited, not recomputed) and each arm's own y, cols/p, r and walk; y rests on 9-89 relations. E24's 2^90.1 is cited from the note, not recomputed here."),
        js("The validation gate cannot show a modelled handover as a measured row: this report's status is fail for that reason alone; every recomputed number and check is listed."),
        js("Kill test: the replayed binary checks E18's own models; the validator's independent check is on the canonical models (E18's coefficients b are not in the records), which verifies the isomorphism-invariant group |<lambda_p, lambda_omega, -1>| = 24, not ord lambda_p = 24 in E18's normalisation."),
        js("Operation counts only. The replay ran locally in the validator's 4-vCPU container, not on isolab-eks; no wall time is compared or claimed."),
    ]);
    let walk_band = obj(vec![
        ("definition", js("walk cost(w) = mean F_p multiplications (fp_muls - verify_fp_muls) + w * 97 * mean steps; pooled ratio = sum over curves of cost/sqrt(n), order 6 over order 24")),
        ("rows", J::Arr(rank)),
        ("order_changes_inside_band", jb(flips)),
    ]);
    obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js("fail")),
        ("independent", jb(true)),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E29; this recompute is its own standard-library Rust program (validate_e29.rs: JSON parser, SHA-256 and leaf comparator taken from the E27 validator's validate_e27.rs, the rest new) and shares no code with examples/e29_twist.rs, examples/e29_tables.rs, crypto_lib or experiments/34_twist24_stage_r/summary.json")),
        ("date", js("2026-10-31")),
        ("row", js("E29 (GLV note §9.27 pre-registration, §9.31 results): order-24 twist action on E18's j = 0 curves. Kill test (8 curves), Stage R (8 curves, order-6 and order-24 walks, two arithmetics) and Stage A (3 curves, flat / cubic / twist24 arms with E18's exact oracle) are measurements; the handover n* is a model. Evidence directory research/e29_twists_20261030")),
        ("script", js("research/e29_twists_20261030/validate_e29.rs (rustc -O --edition 2021; `validate_e29 report <repo> research/e29_twists_20261030`)")),
        ("status_rule", js("pass iff (a) every check holds, (b) every headline agrees with its printed figure within half a unit of the last printed digit, and (c) the row has a measured end-to-end index-calculus cost S to set against a matched rho. E29 has no measured S (c): the index-calculus side is a modelled handover. Status is fail for the gate; the measured parts are verified (replay-certificate.json)")),
        ("gate_refusal", js("model-only: no measured end-to-end S exists for E29 (the handover is §11.16's model; Stage A's decompositions come from an unpriced exact oracle; no S5 solve was run), and Stage R is a rho-against-rho measurement with no index-calculus arm")),
        ("numbers_verified", jb(ck.bad.is_empty() && hl.bad.is_empty())),
        ("raw_data", J::Arr(raw)),
        ("check_failures", J::Arr(ck.bad.iter().map(|s| js(s)).collect())),
        ("headline_disagreements", J::Arr(hl.bad.iter().map(|s| js(s)).collect())),
        ("checks", J::Arr(ck.items)),
        ("headline", J::Arr(hl.items)),
        ("stage_r", J::Arr(r_out)),
        ("walk_band_5a", walk_band),
        ("stage_a", J::Arr(a_out)),
        ("verdicts_recomputed", verdicts),
        ("errata", errata),
        ("pre_registration", js("§9.27 committed e772a783c (2026-10-30T19:00:23Z) before E29's code 0e4fccce1 (21:47:35Z) and records 22f560ff3 (21:52:49Z); its text at HEAD equals the registered text (one trailing blank line)")),
        ("limitations", limitations),
        ("comparator_controls", comparator_controls(ctx)),
    ])
}

fn certificate(ctx: &Ctx) -> J {
    let (cmp, hcmp) = ctx.read(&format!("{}/replay/replay_comparison.json", ctx.ev));
    let (build, hbuild) = ctx.read(&format!("{}/replay/build.json", ctx.ev));
    let mut runs = Vec::new();
    let (mut wtot, mut wok, mut arms_ok, mut kill_ok) = (0u64, 0u64, 0u64, true);
    for r in cmp.get("runs").arr() {
        let name = r.get("run").s().to_string();
        let (rep, _) = ctx.read(r.get("replay").get("path").s());
        let mut entry = vec![
            ("run", js(&name)),
            ("committed", r.get("committed").clone()),
            ("replay_output", r.get("replay").clone()),
            ("fields_compared", r.get("fields_compared").clone()),
            ("fields_identical", r.get("fields_identical").clone()),
            ("mismatches", r.get("mismatches").clone()),
            ("matched", r.get("identical").clone()),
        ];
        if name == "kill" {
            for k in rep.arr() {
                kill_ok &= k.get("group_order").u() == 24 && k.get("psi_p_on_curve").b() && k.get("lambda_p_found").b();
            }
            entry.push(("scalars", obj(vec![("curves", ju(rep.arr().len() as u64)), ("group_order_24_and_psi_on_curve_all", jb(kill_ok))])));
        } else if name.starts_with("stage_r") {
            for row in rep.arr() {
                for run in row.get("runs").arr() {
                    for w in walks(run.get("order6")).iter().chain(walks(run.get("order24")).iter()) {
                        wtot += 1;
                        if w.correct {
                            wok += 1;
                        }
                    }
                }
            }
        } else {
            let x = rep.arr()[0].get("report");
            let d = x.get("known_scalar").u();
            for arm in ["flat", "cubic", "twist24"] {
                let a = x.get(arm);
                if a.get("solved").b() && a.get("correct").b() && a.get("recovered").u() == d {
                    arms_ok += 1;
                }
            }
            for w in walks(x.get("rho")) {
                wtot += 1;
                if w.correct && w.rec == Some(d) {
                    wok += 1;
                }
            }
            entry.push(("timing_not_compared", r.get("timing_not_compared").clone()));
        }
        runs.push(obj(entry));
    }
    let scalars = kill_ok && wok == wtot && wtot == 512 + 144 && arms_ok == 9;
    let all_identical = cmp.get("all_identical").b();
    obj(vec![
        ("schema", js("replay-certificate/v1")),
        ("status", js(if all_identical && scalars { "verified" } else { "failed" })),
        ("independent", jb(true)),
        ("scalars_verified", jb(scalars)),
        ("date", js("2026-10-31")),
        ("row", js("E29 (GLV note §9.27/§9.31), the measured parts: kill test (8 curves), Stage R (8 curves x {order 6, order 24} x 16 walks x 2 arithmetics) and Stage A (p = 241, 337, 541; flat, cubic and twist24 arms, 48 walks each); evidence directory research/e29_twists_20261030. The handover n* is a model: recomputed in validator-report.json, not replayed")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E29: fresh build from an export of the recorded binary commit in a fresh target directory, fresh runs, own comparison (validate_e29.rs compare)")),
        ("recorded_binary", js("0e4fccce1ad1baf7ace18a550483771f02315838 (experiments/34_twist24_stage_r/binary_commit.txt)")),
        ("local_replay_note", js("Replayed locally in the validator's 4-vCPU container, not on isolab-eks: the replay admits operation counts only; no wall time is compared or claimed (wall_ms listed, not compared).")),
        ("build", build),
        ("build_record", obj(vec![("path", js("replay/build.json")), ("sha256", js(&hbuild))])),
        ("comparison", obj(vec![("path", js("replay/replay_comparison.json")), ("sha256", js(&hcmp)), ("rule", cmp.get("rule").clone())])),
        ("runs_total", cmp.get("runs_total").clone()),
        ("runs_matched", cmp.get("runs_identical").clone()),
        ("fields_compared", cmp.get("fields_compared").clone()),
        ("fields_identical", cmp.get("fields_identical").clone()),
        ("coverage", js("full: every recorded run replayed (kill test, both Stage R runs, the three Stage A runs); no sampling")),
        ("scalar_verification", js("Every replayed walk record (512 in Stage R, 144 in Stage A) reports correct = true (the binary sets it only when the recovered logarithm equals the planted one and [d]G = Q is checked); every Stage A arm (9) is solved and correct and recovers the known scalar; the kill test reports a group of order 24 with psi_p on the curve on all 8 curves, which the validator's own arithmetic confirms on the canonical models (replay/twist_check.json).")),
        ("runs", J::Arr(runs)),
    ])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: validate_e29 compare|report|certificate <repo> <evidence-dir>");
        std::process::exit(2);
    }
    let ctx = Ctx { repo: args[2].clone(), ev: args[3].clone() };
    let out = match args[1].as_str() {
        "compare" => compare(&ctx),
        "report" => report(&ctx),
        "certificate" => certificate(&ctx),
        _ => std::process::exit(2),
    };
    print!("{}", out.dump());
}
